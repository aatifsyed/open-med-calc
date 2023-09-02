use either::Either;
use html::forms::Form;

use crate::norm::{Choice, Input, InputType, Markup, NormalisedCalc, NumberUnit};
use itertools::EitherOrBoth;

impl From<NormalisedCalc> for Form {
    fn from(value: NormalisedCalc) -> Self {
        let NormalisedCalc { slug: _, items } = value;
        Form::builder()
            .unordered_list(|ul| {
                for item in items {
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
                            title,
                            ty,
                            required,
                            default,
                            ident,
                        }) => match ty {
                            InputType::Choices { choices } => li.text(title).unordered_list(|ul| {
                                for (
                                    i,
                                    Choice {
                                        description,
                                        weight,
                                    },
                                ) in choices.into_iter().enumerate()
                                {
                                    let choice_id = format!("{ident}_{i}");
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
                                unit: NumberUnit { name, id, us, si },
                                max,
                                min,
                            } => li
                                .text(title)
                                .input(|input| {
                                    input
                                        .type_("number")
                                        .min(min.to_string())
                                        .max(max.to_string())
                                        .id("TODO")
                                        .required(required.to_string())
                                })
                                .label(|label| label.for_("TODO").text("TODO")),
                        },
                    });
                }
                ul
            })
            .build()
    }
}
