pub(crate) mod deser;
pub(crate) mod norm;

#[cfg(test)]
mod test {
    use super::*;

    pub fn all() -> impl Iterator<Item = deser::Root> {
        use include_dir::{include_dir, Dir};
        static DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/scraped");
        DIR.files()
            .map(include_dir::File::contents)
            .map(serde_json::from_slice)
            .map(Result::unwrap)
    }
}
