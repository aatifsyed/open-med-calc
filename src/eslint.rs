use std::{collections::HashSet, io::Write as _, path::PathBuf, process::Stdio};

use anyhow::Context as _;
use either::Either;
use serde::{Deserialize, Serialize};

pub fn undefined_vars(source: &str) -> anyhow::Result<HashSet<String>> {
    let mut child = std::process::Command::new("eslint")
        .args([
            "--no-color",
            "--no-eslintrc",
            "--stdin",
            "--exit-on-fatal-error",
            "--format=json",
            r#"--parser-options={ "ecmaVersion": 6 }"#, // allow `const a = ...;`
            r#"--rule={ "no-undef":             ["error"] }"#, // var x = y;
            r#"--rule={ "no-use-before-define": ["error"] }"#, // var x = parseFloat(x);
        ])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("couldn't exec eslint - is it installed?")?;
    let output = child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(source.as_bytes())
        .and(child.wait_with_output())
        .context("couldn't communicate with eslint")?;
    match output.status.code() {
        Some(0) => Ok(HashSet::new()),
        Some(1) => {
            let linted_files = serde_json::from_slice::<Vec<LintedFile>>(&output.stdout)
                .context("couldn't parse elsint output")?;
            assert_eq!(linted_files.len(), 1);
            let mut vars = HashSet::new();
            for LintMessage { rule_id, message } in &linted_files[0].messages {
                let var_name = match rule_id.as_str() {
                    "no-undef" => {
                        let (_all, var_name) =
                            lazy_regex::regex_captures!(r"'(.*)' is not defined.", message)
                                .expect("message for this lint should be stable");
                        var_name
                    }
                    "no-use-before-define" => {
                        let (_all, var_name) = lazy_regex::regex_captures!(
                            r"'(.*)' was used before it was defined.",
                            message
                        )
                        .expect("message for this lint should be stable");
                        var_name
                    }
                    other => panic!("unexpected rule id: {other}"),
                };
                vars.insert(String::from(var_name));
            }
            Ok(vars)
        }
        Some(2) | None => {
            let message = String::from_utf8_lossy(&output.stderr).into_owned();
            Err(anyhow::Error::msg(message).context(format!(
                "eslint exited abnormally ({})",
                output
                    .status
                    .code()
                    .map(|it| format!("code {it}"))
                    .unwrap_or(String::from("killed"))
            )))
        }
        Some(_) => unreachable!(),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LintedFile {
    file_path: PathBuf,
    messages: Vec<LintMessage>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LintMessage {
    rule_id: String,
    message: String,
}

#[test]
fn test() {
    use rayon::prelude::*;
    crate::normalised().par_bridge().for_each(|it| {
        let source = it.equation_logic.into_parts().2;
        let vars_required_in_script = undefined_vars(&source).unwrap();
        let vars_defined_in_schema = it
            .items
            .into_iter()
            .filter_map(Either::right)
            .map(|it| String::from(it.ident))
            .collect::<HashSet<String>>();

        let _satisfied = vars_required_in_script.intersection(&vars_defined_in_schema);
        let undefined = vars_required_in_script
            .difference(&vars_defined_in_schema)
            .filter(|it| {
                !matches!(
                    it.as_str(),
                    "UOMSYSTEM" | "webLanguage" | "mini_msg" | "msg" | "calc_output"
                )
            })
            .collect::<Vec<_>>();
        let _spare = vars_defined_in_schema
            .difference(&vars_required_in_script)
            .collect::<Vec<_>>();

        let slug = it.slug;

        if !undefined.is_empty() {
            println!("{slug}: undefined: {undefined:?}")
        }
        // if !spare.is_empty() {
        //     println!("{slug}:     spare: {spare:?}")
        // }
    });
}
