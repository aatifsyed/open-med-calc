#![recursion_limit = "1000000"]

pub(crate) mod deser;
pub(crate) mod norm;
pub(crate) mod render;

pub use norm::NormalisedCalc;

pub fn library() -> impl Iterator<Item = NormalisedCalc> {
    scraped()
        .map(|root| root.props.page_props)
        .map(NormalisedCalc::try_from)
        .flat_map(Result::ok)
}

fn scraped() -> impl Iterator<Item = deser::Root> {
    use include_dir::{include_dir, Dir};
    static DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/scraped/calc");
    DIR.files()
        .map(include_dir::File::contents)
        .map(serde_json::from_slice)
        .map(Result::unwrap)
}
