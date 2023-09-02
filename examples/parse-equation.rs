use std::{fs, path::Path};

use anyhow::anyhow;
use boa_interner::ToIndentedString;

fn main() -> anyhow::Result<()> {
    let directory = Path::new("target/open-med-calc/js");
    fs::remove_dir_all(directory)?;
    fs::create_dir_all(directory)?;
    for root in open_med_survey::scraped() {
        let calc = root.props.page_props.calc;
        println!("{}", calc.slug);

        let mut interner = boa_interner::Interner::new();
        let script =
            boa_parser::Parser::new(boa_engine::Source::from_bytes(&calc.equation_logic_text))
                .parse_script(&mut interner)
                .map_err(|_| anyhow!("couldn't parse js"))?;

        fs::write(
            directory.join(calc.slug).with_extension("js"),
            script.to_indented_string(&interner, 0),
        )?;
    }
    Ok(())
}
