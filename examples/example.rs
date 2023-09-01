use std::fmt;

#[path = "../src/model.rs"]
mod model;

impl model::InputSchema {
    fn render(&self) {
        use dialoguer::{Input, MultiSelect, Select};
        use model::{
            InputSchema::{Dropdown, Radio, Subheading, Textbox, Toggle, Visual},
            InputSchemaOption,
        };
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

fn main() {
    use model::{About, Calc, Content, HowToUse, NextSteps, Root};
    for file in include_dir::include_dir!("$CARGO_MANIFEST_DIR/scraped").files() {
        let calc = serde_json::from_slice::<Root>(file.contents())
            .expect("invalid json")
            .props
            .page_props
            .calc;
        println!("{:#?}", CalcInfo(&calc));
        for input_schema in calc.input_schema {
            println!("{:#?}", InputSchemaInfo(&input_schema));
            input_schema.render();
        }
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
        use model::{About, Calc, Content, HowToUse, NextSteps};
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
        use model::InputSchema::{Dropdown, Radio, Subheading, Textbox, Toggle, Visual};
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
