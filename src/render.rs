use either::Either;
use html::forms::Form;

use crate::norm::{Choice, Input, InputType, Markup, NormalisedCalc, NumberUnit};
use itertools::EitherOrBoth;

impl From<NormalisedCalc> for Form {
    fn from(value: NormalisedCalc) -> Self {
        let NormalisedCalc { items, .. } = value;
        Form::builder()
            .unordered_list(|ul| {
                for (item_ix, item) in itertools::enumerate(items) {
                    ul.list_item(|li| match item {
                        Either::Left(Markup::Image { url }) => {
                            li.image(|img| img.src(url.to_string()))
                        }
                        Either::Left(Markup::Subheading {
                            title_and_instructions,
                        }) => match title_and_instructions {
                            EitherOrBoth::Both(title, instructions) => li
                                .heading_1(|h1| h1.text(title))
                                .paragraph(|p| p.text(instructions)),
                            EitherOrBoth::Left(title) => li.heading_1(|h1| h1.text(title)),
                            EitherOrBoth::Right(instructions) => {
                                li.paragraph(|p| p.text(instructions))
                            }
                        },
                        Either::Right(Input {
                            conditionality: _todo,
                            title,
                            ty,
                            required,
                            default,
                            ident,
                        }) => match ty {
                            InputType::Choices { choices } => li.text(title).unordered_list(|ul| {
                                for (
                                    choice_ix,
                                    Choice {
                                        description,
                                        weight,
                                    },
                                ) in itertools::enumerate(choices)
                                {
                                    let choice_id = format!("{ident}_{choice_ix}");
                                    ul.list_item(|li| {
                                        li.input(|input| {
                                            input
                                                .type_("radio")
                                                .id(choice_id.clone()) // unique to choice
                                                .value(weight.to_string()) // unique to choice
                                                .name(ident.to_string()) // unique to group of choices
                                                .required(required.to_string())
                                        })
                                        .label(
                                            |label_builder| {
                                                label_builder
                                                    .for_(choice_id) // unique to choice
                                                    .text(description) // unique to choice
                                            },
                                        )
                                    });
                                }
                                ul
                            }),
                            InputType::Number {
                                unit:
                                    NumberUnit {
                                        name,
                                        id,
                                        us_and_si_units,
                                    },
                                max,
                                min,
                            } => li
                                .input(|input| {
                                    input
                                        .type_("number")
                                        .min(min.to_string())
                                        .max(max.to_string())
                                        .id(item_ix.to_string())
                                        .required(required.to_string())
                                })
                                .label(|label| label.for_(item_ix.to_string()).text(name)),
                        },
                    });
                }
                ul
            })
            .build()
    }
}
