use std::{collections::HashMap, fmt::Display, hash::Hash};

use anyhow::{anyhow, bail, ensure, Context as _};
use either::Either;
pub use ident::Ident;
use itertools::{EitherOrBoth, Itertools as _};
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

    #[derive(Hash, Eq, PartialEq, Clone)]
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

    impl From<Ident> for String {
        fn from(value: Ident) -> Self {
            let Ident(s) = value;
            s
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
        crate::raw()
            .flat_map(|root| root.props.page_props.calc.input_schema)
            .flat_map(|it| it.name().map(|it| it.parse::<Ident>()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
    }
}

pub struct NormalisedCalc {
    pub slug: String,
    pub items: Vec<Either<Markup, Input>>,
    pub equation_logic: boa_engine::Script,
}

impl NormalisedCalc {
    pub fn new(page_props: PageProps, context: &mut boa_engine::Context) -> anyhow::Result<Self> {
        let PageProps {
            calc:
                Calc {
                    slug,
                    input_schema,
                    equation_logic_text,
                    ..
                },
            measurements,
            ..
        } = page_props;

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
                    let choices = options
                        .into_iter()
                        .map(|InputSchemaOption { label, value }| Choice {
                            description: label,
                            weight: value,
                        })
                        .collect::<Vec<_>>();

                    ensure!(!choices.is_empty());

                    Ok(Either::Right(Input {
                        conditionality: conditionality
                            .map(|it| parse_js(&it, context))
                            .transpose()?,
                        title: label_en,
                        default,
                        required: !optional,
                        ty: InputType::Choices { choices },
                        ident: name.parse()?,
                    }))
                }
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
                        conditionality: conditionality
                            .map(|it| parse_js(&it, context))
                            .transpose()?,
                        title: label_en,
                        default,
                        required: !optional,
                        ty: InputType::Number {
                            unit: NumberUnit {
                                us_and_si_units: none_if_empty(units_us)
                                    .and_then(|us| none_if_empty(units_si).map(|si| (us, si))),
                                name: measurement_name.clone(),
                                id: none_if_empty(unit).context(format!("invalid unit: {unit}"))?,
                            },
                            max: error_max.clone(),
                            min: error_min.clone(),
                        },
                        ident: name.parse()?,
                    }))
                }
                Subheading {
                    subheading,
                    subheading_instructions,
                } => {
                    let title_and_instructions = match (
                        subheading.as_deref().and_then(none_if_empty),
                        subheading_instructions.as_deref().and_then(none_if_empty),
                    ) {
                        (None, None) => bail!("subheading cannot be empty"),
                        (None, Some(instructions)) => EitherOrBoth::Right(instructions),
                        (Some(title), None) => EitherOrBoth::Left(title),
                        (Some(title), Some(instructions)) => {
                            EitherOrBoth::Both(title, instructions)
                        }
                    };
                    Ok(Either::Left(Markup::Subheading {
                        title_and_instructions,
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
        Ok(Self {
            slug,
            items,
            equation_logic: {
                if equation_logic_text.trim().is_empty() {
                    bail!("no equation logic text")
                };
                parse_js(&equation_logic_text, context)?
            },
        })
    }
}

fn parse_js(source: &str, context: &mut boa_engine::Context) -> anyhow::Result<boa_engine::Script> {
    boa_engine::Script::parse(boa_engine::Source::from_bytes(source), None, context)
        .map_err(|e| anyhow!("failed to parse javascript: {e}"))
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

#[derive(Clone)]
pub struct Choice {
    pub description: String,
    pub weight: Number,
}

#[derive(Clone)]
pub struct NumberUnit {
    /// "Ethanol (ETOH)"
    /// "Length"
    /// ""
    pub name: String,
    /// "etoh"
    pub id: String,
    /// ("mg/dL", "mmol/L")
    pub us_and_si_units: Option<(String, String)>,
}

#[derive(Clone)]
pub enum Markup {
    Subheading {
        title_and_instructions: EitherOrBoth<String, String>,
    },
    Image {
        url: Url,
    },
}

#[derive(Clone)]
pub struct Input {
    /// markup like `<p>Age</p>`
    pub title: String,
    pub ty: InputType,
    pub conditionality: Option<boa_engine::Script>,
    pub required: bool,
    pub default: Option<Number>,
    pub ident: Ident,
}

#[derive(Clone)]
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
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn num_choices() {
        let histogram = crate::raw()
            .map(|root| root.props.page_props)
            .map(|it| NormalisedCalc::new(it, &mut boa_engine::Context::default()))
            .filter_map(Result::ok)
            .flat_map(|form| form.items)
            .flat_map(Either::right)
            .flat_map(|input| match input.ty {
                InputType::Choices { choices } => Some(choices.len()),
                InputType::Number { .. } => None,
            })
            .counts();
        dbg!(BTreeMap::from_iter(histogram));
    }

    #[test]
    fn normalise_all() {
        let skip_slug = ["covid-19-inpatient-risk-calculator-circ"];
        let (passed, failed) = crate::raw()
            .map(|root| root.props.page_props)
            .filter(|it| !skip_slug.contains(&it.calc.slug.as_str()))
            .map(|it| {
                NormalisedCalc::new(it.clone(), &mut boa_engine::Context::default())
                    .context(format!("failed to normalise {}", it.calc.slug))
            })
            .partition_result::<Vec<_>, Vec<_>, _, _>();
        println!(
            "{} passed, {} failed ({} skipped)",
            passed.len(),
            failed.len(),
            skip_slug.len()
        );
        if !failed.is_empty() {
            for f in failed.iter() {
                println!("{f:#}");
            }
            panic!("{} failed", failed.len());
        }
    }
}
