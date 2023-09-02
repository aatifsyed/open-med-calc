use std::{collections::HashMap, fmt::Display, hash::Hash};

use anyhow::{bail, ensure, Context as _};
use either::Either;
pub use ident::Ident;
use itertools::Itertools as _;
use select::{document::Document, predicate::Name};
use serde_json::Number;
use url::Url;

use crate::deser::{Calc, Measurement, PageProps};
use crate::deser::{
    InputSchema::{Dropdown, Radio, Subheading, Textbox, Toggle, Visual},
    InputSchemaOption,
};

mod ident {
    use std::{fmt, str::FromStr};

    #[derive(Hash, Eq, PartialEq)]
    pub struct Ident(String);

    #[derive(Debug, thiserror::Error)]
    #[error("must be a non-empty ascii string with which starts with a letter")]
    pub struct NotAnIdent;

    impl fmt::Display for Ident {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl TryFrom<String> for Ident {
        type Error = NotAnIdent;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            match is_ident(value.as_str()) {
                true => Ok(Self(value)),
                false => Err(NotAnIdent),
            }
        }
    }

    impl FromStr for Ident {
        type Err = NotAnIdent;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match is_ident(s) {
                true => Ok(Self(String::from(s))),
                false => Err(NotAnIdent),
            }
        }
    }

    /// `s` is:
    /// - non-empty
    /// - ascii
    /// - follows unicode ident rules
    fn is_ident(s: &str) -> bool {
        if !s.is_ascii() || s.is_empty() {
            return false;
        };
        for (ix, ch) in s.chars().enumerate() {
            let is_ok = match ix {
                0 => unicode_ident::is_xid_start(ch),
                _ => unicode_ident::is_xid_continue(ch),
            };
            if !is_ok {
                return false;
            }
        }
        true
    }

    #[test]
    fn all_input_schema_names_are_idents() {
        crate::test::all()
            .flat_map(|root| root.props.page_props.calc.input_schema)
            .flat_map(|it| it.name().map(|it| it.parse::<Ident>()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
    }
}

#[test]
fn test() {
    let (passed, mut failed) = crate::test::all()
        .map(|root| root.props.page_props)
        .map(|it| {
            let slug = it.calc.slug.clone();
            match Form::try_from(it) {
                Ok(_) => Ok(()),
                Err(e) => Err((slug, e)),
            }
        })
        .partition_result::<Vec<_>, Vec<_>, _, _>();
    failed.sort_by_key(|(_, reason)| reason.to_string());
    println!("{} passed, {} failed", passed.len(), failed.len());
    if !failed.is_empty() {
        println!("failures:");
        for (test, reason) in failed {
            println!("{test}\n\t{reason:#}")
        }
        panic!("failed");
    }
}

pub struct Form {
    pub slug: String,
    pub items: Vec<Either<Markup, Input>>,
}

impl TryFrom<PageProps> for Form {
    type Error = anyhow::Error;

    fn try_from(value: PageProps) -> Result<Self, Self::Error> {
        let PageProps {
            calc:
                Calc {
                    // full_title_en,
                    // short_title_en,
                    // medium_description_en,
                    // short_description_en,
                    // before_use,
                    // instructions_en,
                    slug,
                    input_schema,
                    ..
                },
            measurements,
            ..
        } = value;
        ensure_unique(measurements.iter().map(|it| &it.unit))?;
        let measurements_by_unit = measurements
            .into_iter()
            .map(|it| (it.unit.clone(), it))
            .collect::<HashMap<_, _>>();
        let items = input_schema
            .into_iter()
            .map(|it| match it {
                Dropdown {
                    conditionality,
                    default,
                    label_en,
                    name,
                    optional,
                    options,
                    show_points,
                    tips_en,
                }
                | Radio {
                    conditionality,
                    default,
                    label_en,
                    name,
                    optional,
                    options,
                    show_points,
                    tips_en,
                }
                | Toggle {
                    conditionality,
                    default,
                    label_en,
                    name,
                    optional,
                    options,
                    show_points,
                    tips_en,
                } => {
                    ensure!(
                        conditionality.is_none() || conditionality.is_some_and(|s| s.is_empty())
                    );
                    let choices = options
                        .into_iter()
                        .map(|InputSchemaOption { label, value }| Choice {
                            description: label,
                            weight: value,
                        })
                        .collect::<Vec<_>>();

                    Ok(Either::Right(Input {
                        ty: InputType::Choices { choices },
                        ident: name.parse()?,
                    }))
                }
                Subheading {
                    subheading,
                    subheading_instructions,
                } => Ok(Either::Left(Markup::Subheading {
                    title: subheading.as_deref().and_then(none_if_empty),
                    description: subheading_instructions.as_deref().and_then(none_if_empty),
                })),
                Textbox {
                    conditionality,
                    default,
                    label_en,
                    name,
                    optional,
                    show_points,
                    tips_en,
                    unit,
                } => {
                    let Measurement {
                        conversion,
                        created_at,
                        error_max,
                        error_max_si,
                        error_max_us,
                        error_min,
                        error_min_si,
                        error_min_us,
                        name: measurement_name,
                        normal_max_si,
                        normal_max_us,
                        normal_min_si,
                        normal_min_us,
                        published_at,
                        unit,
                        units_si,
                        units_us,
                        updated_at,
                        warn_max,
                        warn_max_si,
                        warn_max_us,
                        warn_min,
                        warn_min_si,
                        warn_min_us,
                    } = measurements_by_unit
                        .get(&unit)
                        .context(format!("undefined unit: {unit}"))?;

                    Ok(Either::Right(Input {
                        ty: InputType::Number {
                            unit: NumberUnit {
                                us: none_if_empty(units_us),
                                si: none_if_empty(units_si),
                                name: none_if_empty(measurement_name),
                                id: none_if_empty(unit).context(format!("invalid unit: {unit}"))?,
                            },
                            max: error_max.clone(),
                            min: error_min.clone(),
                        },
                        ident: name.parse()?,
                    }))
                }
                Visual { visual } => {
                    let document = Document::from(visual.as_str());
                    let images = document.find(Name("img")).collect::<Vec<_>>();
                    ensure!(images.len() == 1);
                    ensure!(unique(images[0].attrs().filter(|(k, _)| *k == "src")));
                    let url = images[0]
                        .attr("src")
                        .context("<img> must have a src attribute")?
                        .parse()
                        .context("<img> src attribute must be a url")?;
                    Ok(Either::Left(Markup::Image { url }))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        let input_idents = items.iter().filter_map(|it| match it {
            Either::Right(Input { ident, .. }) => Some(ident),
            _ => None,
        });
        ensure!(unique(input_idents));
        Ok(Self { slug, items })
    }
}

fn ensure_unique<T: Eq + Hash + Display>(items: impl IntoIterator<Item = T>) -> anyhow::Result<()> {
    let duplicate_keys = items
        .into_iter()
        .counts()
        .into_iter()
        .filter_map(|(k, count)| match count > 1 {
            true => Some(k),
            false => None,
        })
        .collect::<Vec<_>>();
    match duplicate_keys.is_empty() {
        true => Ok(()),
        false => bail!("duplicate items: [{}]", duplicate_keys.iter().join(", ")),
    }
}

fn unique<T: Eq + Hash>(items: impl IntoIterator<Item = T>) -> bool {
    let counts = items.into_iter().counts();
    for (_, count) in counts {
        if count != 1 {
            return false;
        }
    }
    true
}

fn none_if_empty(s: &str) -> Option<String> {
    let s = s.trim();
    match s.is_empty() {
        false => Some(String::from(s)),
        true => None,
    }
}

pub struct Choice {
    pub description: String,
    pub weight: Number,
}

pub struct NumberUnit {
    /// "Ethanol (ETOH)"
    /// "Length"
    pub name: Option<String>,
    /// "etoh"
    pub id: String,
    /// "mg/dL"
    pub us: Option<String>,
    /// "mmol/L"
    pub si: Option<String>,
}

pub enum Markup {
    Subheading {
        title: Option<String>,
        description: Option<String>,
    },
    Image {
        url: Url,
    },
}

pub struct Input {
    pub ty: InputType,
    pub ident: Ident,
}

pub enum InputType {
    Choices {
        /// Some forms repeat the descriptions:
        /// ```text
        /// - Good
        /// - (More severe)
        /// - (More severe)
        /// - Bad
        /// ```
        ///
        /// Some forms repeat the weights:
        /// ```text
        /// - 0kg-5kg (0)
        /// - 5kg-10kg (0)
        /// - 10kg-15kg (1)
        /// ```
        choices: Vec<Choice>,
    },
    Number {
        unit: NumberUnit,
        max: Number,
        min: Number,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use itertools::Itertools as _;

    #[test]
    fn number_of_options_by_type() {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum Type {
            Dropdown,
            Radio,
            Toggle,
        }
        for ((ty, n_options), count) in all()
            .flat_map(|root| root.props.page_props.calc.input_schema)
            .filter_map(|it| match it {
                Dropdown { options, .. } => Some((Type::Dropdown, options.len())),
                Radio { options, .. } => Some((Type::Radio, options.len())),
                Toggle { options, .. } => Some((Type::Toggle, options.len())),
                Subheading { .. } | Textbox { .. } | Visual { .. } => None,
            })
            .counts()
        {
            println!("{ty:?} with {n_options}\t{count}")
        }
    }

    #[test]
    fn all_visual_ones() {
        for visual in all()
            .flat_map(|root| root.props.page_props.calc.input_schema)
            .filter_map(|it| match it {
                Visual { visual } => Some(visual),
                _ => None,
            })
        {
            println!("{visual}")
        }
    }

    #[test]
    fn calc_type() {
        let counts = all()
            .map(|root| root.props.page_props.calc.calc_type)
            .counts();
        dbg!(counts);
    }
}
