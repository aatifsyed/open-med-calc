use anyhow::anyhow;
use boa_interner::ToIndentedString as _;
use open_med_survey::InputSchema::*;

fn main() -> anyhow::Result<()> {
    for conditionality in open_med_survey::scraped()
        .flat_map(|root| root.props.page_props.calc.input_schema)
        .filter_map(|it| match it {
            Dropdown { conditionality, .. }
            | Radio { conditionality, .. }
            | Toggle { conditionality, .. }
            | Textbox { conditionality, .. } => match conditionality {
                Some(s) if s.is_empty() => None,
                None => None,
                Some(s) => Some(s),
            },
            Visual { .. } | Subheading { .. } => None,
        })
    {
        let mut interner = boa_interner::Interner::new();
        let script = boa_parser::Parser::new(boa_engine::Source::from_bytes(&conditionality))
            .parse_script(&mut interner)
            .map_err(|_| anyhow!("couldn't parse js"))?;

        println!("{}", script.to_indented_string(&interner, 0).trim());
    }
    Ok(())
}

#[test]
fn test() {
    let mut ctx = boa_engine::Context::default();
    let script =
        boa_engine::Script::parse(boa_engine::Source::from_bytes("age == 1"), None, &mut ctx)
            .unwrap();

    ctx.register_global_property(
        "age",
        boa_engine::JsValue::Integer(0),
        boa_engine::property::Attribute::READONLY,
    )
    .unwrap();

    let res = script.evaluate(&mut ctx).unwrap();

    dbg!(res);
}
