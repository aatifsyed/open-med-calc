use anyhow::anyhow;
use itertools::Itertools as _;
use open_med_survey::InputSchema::*;

fn main() -> anyhow::Result<()> {
    for root in open_med_survey::raw() {
        let calc = root.props.page_props.calc;
        let source = calc.equation_logic_text.as_str();
        println!(
            "{}\t{}\t{} ({})\t{}",
            parse_boa(source).icon(),
            parse_deno(source).icon(),
            parse_rslint(source).icon(),
            parse_rslint_lossy(source).icon(),
            calc.slug
        );
        for conditionality in calc.input_schema.iter().filter_map(|it| match it {
            Dropdown { conditionality, .. }
            | Radio { conditionality, .. }
            | Toggle { conditionality, .. }
            | Textbox { conditionality, .. } => match conditionality {
                Some(s) if s.is_empty() => None,
                None => None,
                Some(s) => Some(s.as_str()),
            },
            Visual { .. } | Subheading { .. } => None,
        }) {
            println!(
                "{}\t{}\t{} ({})",
                parse_boa(conditionality).icon(),
                parse_deno(conditionality).icon(),
                parse_rslint(conditionality).icon(),
                parse_rslint_lossy(conditionality).icon(),
            );
        }
    }
    Ok(())
}

fn parse_boa(source: &str) -> anyhow::Result<(boa_ast::Script, boa_interner::Interner)> {
    let mut interner = boa_interner::Interner::new();
    let script = boa_parser::Parser::new(boa_engine::Source::from_bytes(source))
        .parse_script(&mut interner)
        .map_err(|e| anyhow!("{e}").context("couldn't parse javascript"))?;
    Ok((script, interner))
}

fn parse_rslint(source: &str) -> anyhow::Result<rslint_parser::ast::Script> {
    rslint_parser::parse_text(source, 0).ok().map_err(|diags| {
        anyhow!("{}", diags.into_iter().map(|it| it.title).join("\n")).context("parsing js failed")
    })
}

fn parse_rslint_lossy(source: &str) -> anyhow::Result<rslint_parser::ast::Script> {
    rslint_parser::parse_text_lossy(source, 0)
        .ok()
        .map_err(|diags| {
            anyhow!("{}", diags.into_iter().map(|it| it.title).join("\n"))
                .context("parsing js failed")
        })
}

fn parse_deno(source: &str) -> anyhow::Result<deno_ast::ParsedSource> {
    let script = deno_ast::parse_script(deno_ast::ParseParams {
        specifier: String::from("<anon>"),
        text_info: deno_ast::SourceTextInfo::new(source.into()),
        media_type: deno_ast::MediaType::Cjs,
        capture_tokens: false,
        scope_analysis: false,
        maybe_syntax: None,
    })?;
    Ok(script)
}

trait Icon<T, E>: Into<Result<T, E>> {
    fn icon(self) -> &'static str
    where
        Self: Sized,
    {
        match self.into() {
            Ok(_) => "✅",
            Err(_) => "❌",
        }
    }
}

impl<S, T, E> Icon<T, E> for S where S: Into<Result<T, E>> {}
