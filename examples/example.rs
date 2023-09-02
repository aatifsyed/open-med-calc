#![recursion_limit = "512"]

use std::fmt;
use std::path::Path;

#[path = "../src/deser.rs"]
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
                default,
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
}

// #[test]
fn main() {
    for file in include_dir::include_dir!("$CARGO_MANIFEST_DIR/scraped").files() {
        let calc = serde_json::from_slice::<Root>(file.contents())
            .expect("invalid json")
            .props
            .page_props
            .calc;
        std::fs::write(
            Path::new("html")
                .join(calc.slug.as_str())
                .with_extension("html"),
            calc.render_html().to_string(),
        )
        .unwrap();
    }
}
