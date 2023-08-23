use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Citation {
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    text: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct About {
    evidence_based_medicine_en: String,
    formula_en: String,
    more_info_en: String,
    // Original/Primary Reference
    // Other References
    // Validation
    // Validations
    // Clinical Practice Guidelines
    // Manufacturer Website
    // Outcomes
    references_list: HashMap<String, Vec<Citation>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ContentContributor {
    expert_name: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    approved: Option<bool>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    qa_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HowToUse {
    pearls_pitfalls_en: String,
    use_case_en: String,
    why_use_en: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct NextSteps {
    advice_en: String,
    critical_actions_en: String,
    management_en: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Guideline {
    #[serde(rename = "CalculatorId", skip_serializing_if = "Option::is_none")]
    calculator_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<i64>,
    society: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RatingFull {
    #[serde(rename = "CalculatorId")]
    calculator_id: i64,
    clinical: i64,
    created_at: String,
    evidence: i64,
    id: i64,
    popularity: i64,
    updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum StringOrNumber {
    String(String),
    Number(Number),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RatingShort {
    clinical: StringOrNumber,
    evidence: StringOrNumber,
    popularity: StringOrNumber,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum MdCalcRating {
    Full(Vec<RatingFull>),
    Short(RatingShort),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Interest {
    #[serde(rename = "CalculatorId", skip_serializing_if = "Option::is_none")]
    calculator_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<i64>,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartnerContact {
    #[serde(rename = "CalculatorId", skip_serializing_if = "Option::is_none")]
    calculator_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<i64>,
    society: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Teaching {
    #[serde(rename = "CalculatorId", skip_serializing_if = "Option::is_none")]
    calculator_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<i64>,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SdmTool {
    #[serde(rename = "CalculatorId")]
    calculator_id: i64,
    created_at: String,
    description: String,
    id: i64,
    link: String,
    order: i64,
    text: String,
    updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RelatedResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    guidelines: Option<Vec<Guideline>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interests: Option<Vec<Interest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mdcalc_rating: Option<MdCalcRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partner_contact: Option<Vec<PartnerContact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sdm_tool: Option<Vec<SdmTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    teaching: Option<Vec<Teaching>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    videos: Option<Vec<()>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Content {
    how_to_use: HowToUse,
    next_steps: NextSteps,
    about: About,
    contributor: ContentContributor,
    creator: Vec<Creator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_resources: Option<RelatedResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reviewer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reviewers: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct InputSchemaOption {
    label: String,
    value: Number,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum InputSchema {
    Dropdown {
        #[serde(skip_serializing_if = "Option::is_none")]
        conditionality: Option<String>,
        label_en: String,
        name: String,
        optional: bool,
        options: Vec<InputSchemaOption>,
        show_points: bool,
        tips_en: String,
    },
    Radio {
        #[serde(skip_serializing_if = "Option::is_none")]
        conditionality: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<Number>,
        label_en: String,
        name: String,
        optional: bool,
        options: Vec<InputSchemaOption>,
        show_points: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        tips_en: Option<String>,
    },
    Subheading {
        #[serde(skip_serializing_if = "Option::is_none")]
        subheading: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subheading_instructions: Option<String>,
    },
    Textbox {
        #[serde(skip_serializing_if = "Option::is_none")]
        conditionality: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<Number>,
        label_en: String,
        name: String,
        optional: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_points: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tips_en: Option<String>,
        unit: String,
    },
    Toggle {
        #[serde(skip_serializing_if = "Option::is_none")]
        conditionality: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<Number>,
        label_en: String,
        name: String,
        optional: bool,
        options: Vec<InputSchemaOption>,
        show_points: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        tips_en: Option<String>,
    },
    Visual {
        visual: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Log {
    message: String,
    time: String,
    user: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Seo {
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords_en: Option<String>,
    meta_description_en: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Version {
    #[serde(skip_serializing_if = "Option::is_none")]
    before_use: Option<String>,
    calc_type: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeEndDate")]
    cme_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeLastReviewed")]
    cme_last_reviewed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeReleaseDate")]
    cme_release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeVersion")]
    cme_version: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: String,
    diagnostic_criteria: bool,
    disabled: bool,
    disabled_reason: (),
    dosing: bool,
    equation_logic: String,
    equation_logic_text: String,
    favorite_id: i64,
    full_title_en: String,
    id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions_en: Option<String>,
    #[serde(rename = "isVisibleInListView")]
    is_visible_in_list_view: bool,
    logic_language: String,
    md5: String,
    medium_description_en: String,
    #[serde(rename = "publishedAt", skip_serializing_if = "Option::is_none")]
    published_at: Option<String>,
    replacement_calc_id: (),
    short_description_en: String,
    short_title_en: String,
    slug: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    #[serde(rename = "versionNumber")]
    version_number: i64,
    vuid: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tag {
    created_at: String,
    id: i64,
    name: String,
    published_at: (),
    updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Calc {
    favorite_id: i64,
    calc_type: String,
    dosing: bool,
    full_title_en: String,
    short_title_en: String,
    medium_description_en: String,
    short_description_en: String,
    before_use: String,
    instructions_en: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "isVisibleInListView"
    )]
    is_visible_in_list_view: Option<bool>,
    purpose_en: Vec<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    updated_at2: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    created_at2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    disease_en: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_reason: Option<String>,
    specialty_en: Vec<String>,
    chief_complaint_en: Vec<String>,
    system_en: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versions: Option<Vec<Version>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "versionNumber")]
    version_number: Option<i64>,
    search_abbreviation_en: Vec<String>,
    slug: String,
    seo: Seo,
    content: Content,
    #[serde(skip_serializing_if = "Option::is_none", rename = "publishedAt")]
    published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeEndDate")]
    cme_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeLastReviewed")]
    cme_last_reviewed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeReleaseDate")]
    cme_release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeVersion")]
    cme_version: Option<String>,
    cme_status: String,
    equation_logic_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    logic_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_manual: Option<bool>,
    search_id: String,
    md5: String,
    input_schema: Vec<InputSchema>,
    logs: Vec<Log>,
    #[serde(skip_serializing_if = "Option::is_none")]
    equation_logic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatorFull {
    #[serde(rename = "about_en", skip_serializing_if = "Option::is_none")]
    about_en: Option<String>,
    created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    deceased: Option<bool>,
    first_name: String,
    last_name: String,
    name: String,
    #[serde(rename = "photo_en", skip_serializing_if = "Option::is_none")]
    photo_en: Option<String>,
    published_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubmed_link: Option<String>,
    signed_c_o_i: bool,
    updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CmeFaq {
    answer: String,
    created_at: String,
    published_at: String,
    question: String,
    r#type: String,
    updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Faq {
    cme_faq: CmeFaq,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HeadConfig {
    canonical_url: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<String>,
    title: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Measurement {
    conversion: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    error_max: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_max_si: Option<Number>,
    error_max_us: Number,
    error_min: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_min_si: Option<Number>,
    error_min_us: Number,
    name: String,
    normal_max_si: Number,
    normal_max_us: Number,
    normal_min_si: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    normal_min_us: Option<Number>,
    #[serde(rename = "publishedAt")]
    published_at: String,
    unit: String,
    units_si: String,
    units_us: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    warn_max: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    warn_max_si: Option<Number>,
    warn_max_us: Number,
    warn_min: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    warn_min_si: Option<Number>,
    warn_min_us: Number,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum StringOrI64 {
    String(String),
    I64(i64),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RelatedCalc {
    #[serde(rename = "calcId")]
    calc_id: StringOrI64,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_title_en: Option<String>,
    slug: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Society {
    #[serde(skip_serializing_if = "Option::is_none")]
    abbreviation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    img: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_link: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RootContributor {
    #[serde(rename = "contributors_page")]
    contributors_page: bool,
    created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    first_name: String,
    has_disclosure: Vec<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    img: Option<String>,
    last_name: String,
    name: String,
    published_at: String,
    signed_c_o_i: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    target: String,
    updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageProps {
    // CANONICAL_URL_ROOT
    // RECAPTCHA_PUBLIC_KEY
    // STRAPI_TAG
    // STRIPE_PUB_KEY
    envs: HashMap<String, String>,
    calc: Calc,
    contributors: Vec<RootContributor>,
    creators: Vec<CreatorFull>,
    faqs: Vec<Faq>,
    head_config: HeadConfig,
    rel_calcs: Vec<RelatedCalc>,
    is_c_m_e_calc: bool,
    measurements: Vec<Measurement>,
    societies: HashMap<String, Society>,
    valid_sections: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Props {
    #[serde(rename = "__N_SSP")]
    _n_ssp: bool,
    page_props: PageProps,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Query {
    slug: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    props: Props,
    page: String,
    query: Query,
    build_id: String,
    is_fallback: bool,
    gssp: bool,
    script_loader: Vec<()>,
}

#[test]
fn all() {
    for file in include_dir::include_dir!("$CARGO_MANIFEST_DIR/scraped").files() {
        print!("{}...", file.path().display());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        test_roundtrip::<Root>(&[serde_json::from_slice(file.contents()).expect("invalid json")]);
        println!("ok.");
    }
}
#[cfg(test)]
fn test_roundtrip<T: Serialize + serde::de::DeserializeOwned>(samples: &[serde_json::Value]) {
    for sample in samples {
        match serde_path_to_error::deserialize::<_, T>(sample.clone()) {
            Ok(deser) => {
                let round_tripped = serde_json::to_value(deser).unwrap();
                pretty_assertions::assert_str_eq!(
                    redact_nulls_in_objects(round_tripped).to_string(),
                    redact_nulls_in_objects(sample.clone()).to_string(),
                    "Round trip failed (roundtripped != sample)"
                );
            }
            Err(e) => panic!(
                "Failed to serialize a {} from json: {e}",
                std::any::type_name::<T>()
            ),
        }
    }
}

#[cfg(test)]
fn redact_nulls_in_objects(value: serde_json::Value) -> serde_json::Value {
    use serde_json::Value::{Array, Bool, Null, Number, Object, String};
    match value {
        leaf @ (Null | Bool(_) | Number(_) | String(_)) => leaf,
        Array(array) => Array(array.into_iter().map(redact_nulls_in_objects).collect()),
        Object(object) => Object(
            object
                .into_iter()
                .map(|(k, v)| (k, redact_nulls_in_objects(v)))
                .filter(|(_k, v)| !matches!(v, Null))
                .collect(),
        ),
    }
}
