#![recursion_limit = "1000000"]

use std::path::Path;

use html::forms::Form;

fn main() -> anyhow::Result<()> {
    let target_dir = &Path::new("target/open-med-calc/html");
    println!("rendering to {}", target_dir.display());
    std::fs::remove_dir_all(target_dir)?;
    std::fs::create_dir_all(target_dir)?;
    for n in open_med_survey::library() {
        println!("{}", n.slug);
        std::fs::write(
            target_dir.join(n.slug.as_str()).with_extension("html"),
            Form::from(n).to_string(),
        )?;
    }
    Ok(())
}
