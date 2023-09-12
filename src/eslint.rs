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
            r#"--rule={ "no-undef": ["error"] }"#,
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
            Ok(linted_files[0].undefined_vars())
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

impl LintedFile {
    fn undefined_vars(&self) -> HashSet<String> {
        self.messages
            .iter()
            .filter(|it| it.rule_id == "no-undef")
            .map(|it| {
                let (_all, var_name) =
                    lazy_regex::regex_captures!(r"'(.*)' is not defined.", &it.message)
                        .expect("message for this lint should be stable");
                String::from(var_name)
            })
            .collect()
    }
}

#[test]
fn test() {
    for root in crate::normalised() {
        let source = root.equation_logic.into_parts().2;
        let vars_required_in_script = undefined_vars(&source).unwrap();
        let vars_defined_in_schema = root
            .items
            .into_iter()
            .filter_map(Either::right)
            .map(|it| String::from(it.ident))
            .collect::<HashSet<String>>();

        println!("{}", root.slug);
        println!("\tvars_required_in_script: {:?}", vars_required_in_script);
        println!("\tvars_defined_in_schema: {:?}", vars_defined_in_schema);
        // println!("{}", root.slug);
        // println!(
        //     "\tsatisfied: {:?}",
        //     vars_required_in_script.intersection(&vars_defined_in_input)
        // );
        // println!(
        //     "\tundefined: {:?}",
        //     vars_required_in_script.difference(&vars_defined_in_input)
        // );
        // println!(
        //     "\tspare: {:?}",
        //     vars_defined_in_input.difference(&vars_required_in_script)
        // )
    }
}
