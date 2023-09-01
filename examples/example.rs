#![recursion_limit = "512"]

use std::fmt;
use std::path::Path;

#[path = "../src/model.rs"]
mod model;

use model::InputSchema::{self, Dropdown, Radio, Subheading, Textbox, Toggle, Visual};
use model::{About, Calc, Content, HowToUse, NextSteps, Root};

use crate::model::InputSchemaOption;

impl model::Calc {
    fn render_html(&self) -> html::forms::Form {
        html::forms::Form::builder()
            .method("post")
            .action("/handler")
            .heading_1(|h1| h1.text(self.full_title_en.clone()))
            .unordered_list(|ul| {
                for input_schema in &self.input_schema {
                    ul.list_item(|li| input_schema.render_to_html(li));
                }
                ul
            })
            .build()
    }
}

impl model::InputSchema {
    fn render_to_html<'a>(
        &self,
        li: &'a mut html::text_content::builders::ListItemBuilder,
    ) -> &'a mut html::text_content::builders::ListItemBuilder {
        match self {
            Dropdown {
                conditionality,
                label_en,
                name,
                optional,
                options,
                show_points,
                tips_en,
            } => li.select(|select| {
                select.required(!optional);
                for InputSchemaOption { label, value } in options {
                    select.option(|option| option.text(label.to_string()).value(value.to_string()));
                }
                select
            }),
            Radio {
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
            } => li.text(label_en.to_string()).unordered_list(|ul| {
                // without this, all items appear on one line
                for InputSchemaOption { label, value } in options {
                    ul.list_item(|li| {
                        li.input(|input| {
                            input
                                .type_("radio")
                                .id(label.to_string()) // unique to option
                                .value(value.to_string()) // unique to option
                                .name(name.to_string()) // unique to group of options
                                .required(match optional {
                                    true => "true",
                                    false => "false",
                                })
                        })
                        .label(|label_builder| {
                            label_builder
                                .for_(label.to_string()) // unique to option
                                .text(label.to_string()) // unique to option
                        })
                    });
                }
                ul
            }),
            Subheading {
                subheading,
                subheading_instructions,
            } => {
                if let Some(subheading) = subheading {
                    li.heading_2(|h2| h2.text(subheading.to_string()));
                }
                if let Some(instructions) = subheading_instructions {
                    li.paragraph(|p| p.text(instructions.to_string()));
                }
                li
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
            } => li
                .input(|input| {
                    input
                        .type_("text")
                        .id(name.to_string())
                        .required(match optional {
                            true => "true",
                            false => "false",
                        })
                })
                .label(|label| label.for_(name.to_string()).text(label_en.to_string())),
            Visual { visual } => li.image(|img| img.src(visual.to_string())),
        }
    }

    fn render_terminal(&self) {
        use dialoguer::{Input, Select};

        match self {
            Dropdown {
                conditionality: _,
                label_en: _,
                name: _,
                optional: _,
                options,
                show_points: _,
                tips_en: _,
            }
            | Radio {
                conditionality: _,
                default: _,
                label_en: _,
                name: _,
                optional: _,
                options,
                show_points: _,
                tips_en: _,
            }
            | Toggle {
                conditionality: _,
                default: _,
                label_en: _,
                name: _,
                optional: _,
                options,
                show_points: _,
                tips_en: _,
            } => {
                let select = Select::new()
                    .items(
                        options
                            .iter()
                            .map(|it| it.label.as_str())
                            .collect::<Vec<_>>()
                            .as_slice(),
                    )
                    .interact();
                dbg!(select);
            }
            Subheading {
                subheading,
                subheading_instructions,
            } => {}
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
                let input = Input::<String>::new().interact();
                dbg!(input);
            }
            Visual { visual } => {}
        }
    }
}

// #[test]
fn main() {
    for schema in include_dir::include_dir!("$CARGO_MANIFEST_DIR/scraped")
        .files()
        .map(include_dir::File::contents)
        .map(serde_json::from_slice::<Root>)
        .map(Result::unwrap)
        .flat_map(|root| root.props.page_props.calc.input_schema)
    {
        // if let Dropdown {
        //     conditionality,
        //     label_en,
        //     name,
        //     optional,
        //     options,
        //     show_points,
        //     tips_en,
        // } = schema
        // {
        //     println!("name = {name}\tlabel_en = {label_en}");
        //     for InputSchemaOption { label, value } in options {
        //         println!("\tlabel = {label}\tvalue = {value}");
        //     }
        // }
        // if let Textbox {
        //     conditionality,
        //     default,
        //     label_en,
        //     name,
        //     optional,
        //     show_points,
        //     tips_en,
        //     unit,
        // } = schema
        // {
        //     println!("name = {name}\tlabel_en = {label_en}");
        // }
        // if let Toggle {
        //     conditionality,
        //     default,
        //     label_en,
        //     name,
        //     optional,
        //     options,
        //     show_points,
        //     tips_en,
        // } = schema
        // {
        //     println!("name = {name}\tlabel_en = {label_en}");
        //     for InputSchemaOption { label, value } in options {
        //         println!("\tlabel = {label}\tvalue = {value}");
        //     }
        // }
        match schema {
            Dropdown { conditionality, .. }
            | Radio { conditionality, .. }
            | Textbox { conditionality, .. }
            | Toggle { conditionality, .. } => match conditionality.as_deref() {
                Some("") | None => println!("<none>"),
                Some(thing) => println!("{thing}"),
            },
            Visual { .. } | Subheading { .. } => {}
        }
    }
}

#[test]
fn test() {
    for file in include_dir::include_dir!("$CARGO_MANIFEST_DIR/scraped").files() {
        let calc = serde_json::from_slice::<Root>(file.contents())
            .expect("invalid json")
            .props
            .page_props
            .calc;
        // println!("{:#?}", CalcInfo(&calc));
        // for input_schema in calc.input_schema {
        //     println!("{:#?}", InputSchemaInfo(&input_schema));
        //     input_schema.render_terminal();
        // }
        std::fs::write(
            Path::new("html")
                .join(calc.slug.as_str())
                .with_extension("html"),
            calc.render_html().to_string(),
        )
        .unwrap();
    }
}

struct IgnoreAlternate<T>(pub T);
impl<T: fmt::Debug> fmt::Debug for IgnoreAlternate<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

macro_rules! debug_fields {
    ( $debug_struct:ident {
        $($ident:ident $(@$frag:ident)?),* $(,)?

    }) => {
        $(
            debug_fields!($debug_struct @field $ident $(@$frag)?);
        )*
    };
    ($debug_struct:ident @field $ident:ident @string) => {
        if !$ident.is_empty() {
            $debug_struct.field(stringify!($ident), $ident);
        }
    };
    ($debug_struct:ident @field $ident:ident @vec_string) => {
        let $ident =
        $ident.iter().filter(|it|!it.is_empty()).collect::<Vec<_>>();
        if !$ident.is_empty() {
            $debug_struct.field(stringify!($ident), &IgnoreAlternate($ident));
        }
    };
    ($debug_struct:ident @field $ident:ident @option_vec_tag) => {
        let $ident = $ident.clone().unwrap_or_default();
        let $ident = $ident.iter().map(|it|it.name.as_str()).collect::<Vec<_>>();
        if !$ident.is_empty() {
            $debug_struct.field(stringify!($ident), &$ident);
        }
    };
    ($debug_struct:ident @field $ident:ident @hashmap_string) => {
        let $ident = $ident.keys().collect::<Vec<_>>();
        if !$ident.is_empty() {
            $debug_struct.field(stringify!($ident), &IgnoreAlternate($ident));
        }
    };

    ($debug_struct:ident @field $ident:ident @option) => {
        if let Some($ident) = $ident {
            $debug_struct.field(stringify!($ident), &$ident);
        }
    };
    ($debug_struct:ident @field $ident:ident) => {
        $debug_struct.field(stringify!($ident), $ident);
    };
}

struct CalcInfo<'a>(&'a model::Calc);
struct InputSchemaInfo<'a>(&'a model::InputSchema);

impl fmt::Debug for CalcInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Calc {
            calc_type,
            dosing,
            full_title_en,
            short_title_en,
            medium_description_en,
            short_description_en,
            before_use,
            instructions_en,
            purpose_en,
            disease_en,
            specialty_en,
            chief_complaint_en,
            system_en,
            tags,
            search_abbreviation_en,
            slug,
            content:
                Content {
                    how_to_use:
                        HowToUse {
                            pearls_pitfalls_en,
                            use_case_en,
                            why_use_en,
                        },
                    next_steps:
                        NextSteps {
                            advice_en,
                            critical_actions_en,
                            management_en,
                        },
                    about:
                        About {
                            evidence_based_medicine_en,
                            formula_en,
                            more_info_en,
                            references_list,
                        },
                    ..
                },
            equation_logic_text,
            logic_language,
            ..
        } = self.0;

        let mut d = f.debug_struct("CalcInfo");
        debug_fields!( d {
                calc_type @string,
                dosing,
                full_title_en @string,
                short_title_en @string,
                medium_description_en @string,
                short_description_en @string,
                before_use @string,
                instructions_en @string,
                purpose_en @vec_string,
                disease_en @vec_string,
                specialty_en @vec_string,
                chief_complaint_en @vec_string,
                system_en @vec_string,
                tags @option_vec_tag,
                search_abbreviation_en @vec_string,
                slug @string,
                pearls_pitfalls_en @string,
                use_case_en @string,
                why_use_en @string,
                advice_en @string,
                critical_actions_en @string,
                management_en @string,
                evidence_based_medicine_en @string,
                formula_en @string,
                more_info_en @string,
                references_list @hashmap_string,
                equation_logic_text @string,
                logic_language @option,
        });

        d.finish_non_exhaustive()
    }
}

impl fmt::Debug for InputSchemaInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Dropdown {
                conditionality,
                label_en,
                name,
                optional,
                options: _,
                show_points,
                tips_en,
            } => {
                let mut d = f.debug_struct("Dropdown");
                debug_fields!(d {
                        conditionality @option,
                        label_en @string,
                        name @string,
                        optional,
                        show_points,
                        tips_en @string,
                });
                d.finish_non_exhaustive()
            }
            Radio {
                conditionality,
                default: _,
                label_en,
                name,
                optional,
                options: _,
                show_points,
                tips_en,
            } => {
                let mut d = f.debug_struct("Radio");
                debug_fields!(d {
                        conditionality @option,
                        label_en @string,
                        name @string,
                        optional,
                        show_points,
                        tips_en @option,
                });
                d.finish_non_exhaustive()
            }
            Subheading {
                subheading,
                subheading_instructions,
            } => {
                let mut d = f.debug_struct("Subheading");
                debug_fields!(d {
                    subheading @option,
                    subheading_instructions @option,
                });
                d.finish_non_exhaustive()
            }
            Textbox {
                conditionality,
                default: _,
                label_en,
                name,
                optional,
                show_points,
                tips_en,
                unit,
            } => {
                let mut d = f.debug_struct("Textbox");
                debug_fields!(d {
                        conditionality @option,
                        label_en @string,
                        name @string,
                        optional,
                        show_points @option,
                        tips_en @option,
                        unit @string,
                });
                d.finish_non_exhaustive()
            }
            Toggle {
                conditionality,
                default: _,
                label_en,
                name,
                optional,
                options: _,
                show_points,
                tips_en,
            } => {
                let mut d = f.debug_struct("Toggle");
                debug_fields!(d {
                        conditionality @option,
                        label_en @string,
                        name @string,
                        optional,
                        show_points,
                        tips_en @option,
                });
                d.finish_non_exhaustive()
            }
            Visual { visual: _ } => f.debug_struct("Visual").finish_non_exhaustive(),
        }
    }
}
