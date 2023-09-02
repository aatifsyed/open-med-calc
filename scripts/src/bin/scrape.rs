use std::path::Path;

use anyhow::ensure;
use futures::{
    stream::{self, TryStreamExt as _},
    StreamExt,
};
use select::{document::Document, predicate::Attr};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let client = &client;

    let directory = Path::new("./scraped");

    tokio::fs::remove_dir_all(directory).await?;
    tokio::fs::create_dir_all(directory).await?;

    let root = get_next_js_payload::<Root>(client, "http://mdcalc.com").await?;
    tokio::fs::write(
        directory.join("popular-calcs.json"),
        serde_json::to_string_pretty(&root.props.page_props.popular_calcs)?,
    )
    .await?;
    let all_calcs = root.props.page_props.all_calcs;
    println!("downloading {} pages...", all_calcs.len());

    let n = stream::iter(all_calcs)
        .map(|Calc { id, slug }| async move {
            println!("{id}\t{slug}");
            let json = get_next_js_payload::<serde_json::Value>(
                client,
                format!("http://mdcalc.com/calc/{id}/{slug}"),
            )
            .await?;
            tokio::fs::write(
                directory.clone().join(format!("{id}-{slug}.json")),
                format!("{json:#}"),
            )
            .await?;
            anyhow::Ok(())
        })
        .buffer_unordered(5)
        .try_collect::<Vec<_>>()
        .await?
        .len();

    println!("downloaded {} pages.", n);
    Ok(())
}

/// <https://mdcalc.com> uses [next.js](https://nextjs.org/).
/// This means it serves files which look like this:
/// ```html
/// <html>
/// ...
/// <script id="__NEXT_DATA__">
/// ... // a bunch of JSON
/// </script>
/// </html>
/// ```
/// This function extracts the JSON
async fn get_next_js_payload<T>(
    client: &reqwest::Client,
    url: impl reqwest::IntoUrl,
) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let html = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let searcher = Document::from(html.as_str());
    let matching_nodes = searcher
        .find(Attr("id", "__NEXT_DATA__"))
        .collect::<Vec<_>>();
    ensure!(matching_nodes.len() == 1);
    let payload = serde_json::from_str(&matching_nodes[0].text())?;
    Ok(payload)
}

#[derive(Serialize, Deserialize)]
struct Root {
    props: Props,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Props {
    page_props: PageProps,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageProps {
    all_calcs: Vec<Calc>,
    popular_calcs: Vec<Calc>,
}

#[derive(Serialize, Deserialize)]
struct Calc {
    id: u16,
    slug: String,
}
