pub use serde::{Deserialize, Serialize};
pub use serde_json::Number;
pub use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Citation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    pub text: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct About {
    pub evidence_based_medicine_en: String,
    pub formula_en: String,
    pub more_info_en: String,
    // Original/Primary Reference
    // Other References
    // Validation
    // Validations
    // Clinical Practice Guidelines
    // Manufacturer Website
    // Outcomes
    pub references_list: HashMap<String, Vec<Citation>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentContributor {
    pub expert_name: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qa_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HowToUse {
    pub pearls_pitfalls_en: String,
    pub use_case_en: String,
    pub why_use_en: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NextSteps {
    pub advice_en: String,
    pub critical_actions_en: String,
    pub management_en: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guideline {
    #[serde(rename = "CalculatorId", skip_serializing_if = "Option::is_none")]
    pub calculator_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    pub society: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingFull {
    #[serde(rename = "CalculatorId")]
    pub calculator_id: i64,
    pub clinical: i64,
    pub created_at: String,
    pub evidence: i64,
    pub id: i64,
    pub popularity: i64,
    pub updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(Number),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RatingShort {
    pub clinical: StringOrNumber,
    pub evidence: StringOrNumber,
    pub popularity: StringOrNumber,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MdCalcRating {
    Full(Vec<RatingFull>),
    Short(RatingShort),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interest {
    #[serde(rename = "CalculatorId", skip_serializing_if = "Option::is_none")]
    pub calculator_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartnerContact {
    #[serde(rename = "CalculatorId", skip_serializing_if = "Option::is_none")]
    pub calculator_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    pub society: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teaching {
    #[serde(rename = "CalculatorId", skip_serializing_if = "Option::is_none")]
    pub calculator_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdmTool {
    #[serde(rename = "CalculatorId")]
    pub calculator_id: i64,
    pub created_at: String,
    pub description: String,
    pub id: i64,
    pub link: String,
    pub order: i64,
    pub text: String,
    pub updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guidelines: Option<Vec<Guideline>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<Interest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdcalc_rating: Option<MdCalcRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_contact: Option<Vec<PartnerContact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdm_tool: Option<Vec<SdmTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teaching: Option<Vec<Teaching>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<()>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    pub how_to_use: HowToUse,
    pub next_steps: NextSteps,
    pub about: About,
    pub contributor: ContentContributor,
    pub creator: Vec<Creator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resources: Option<RelatedResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewers: Option<Vec<String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputSchemaOption {
    pub label: String,
    pub value: Number,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum InputSchema {
    Dropdown {
        #[serde(skip_serializing_if = "Option::is_none")]
        conditionality: Option<String>,
        // this is actuall always None
        // but have as a field for destructuring
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<Number>,
        label_en: String,
        name: String,
        optional: bool,
        options: Vec<InputSchemaOption>,
        show_points: bool,
        // this is actuall always Some(...)
        // but store as an option for destructuring
        #[serde(skip_serializing_if = "Option::is_none")]
        tips_en: Option<String>,
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
pub struct Log {
    pub message: String,
    pub time: String,
    pub user: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Seo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords_en: Option<String>,
    pub meta_description_en: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_use: Option<String>,
    pub calc_type: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeEndDate")]
    pub cme_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeLastReviewed")]
    pub cme_last_reviewed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeReleaseDate")]
    pub cme_release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeVersion")]
    pub cme_version: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub diagnostic_criteria: bool,
    pub disabled: bool,
    pub disabled_reason: (),
    pub dosing: bool,
    pub equation_logic: String,
    pub equation_logic_text: String,
    pub favorite_id: i64,
    pub full_title_en: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_en: Option<String>,
    #[serde(rename = "isVisibleInListView")]
    pub is_visible_in_list_view: bool,
    pub logic_language: String,
    pub md5: String,
    pub medium_description_en: String,
    #[serde(rename = "publishedAt", skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    pub replacement_calc_id: (),
    pub short_description_en: String,
    pub short_title_en: String,
    pub slug: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "versionNumber")]
    pub version_number: i64,
    pub vuid: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub created_at: String,
    pub id: i64,
    pub name: String,
    pub published_at: (),
    pub updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Calc {
    pub favorite_id: i64,
    // "diagnostic_criteria": 89,
    // "calculator": 496,
    // "": 113,
    pub calc_type: String,
    pub dosing: bool,
    pub full_title_en: String,
    pub short_title_en: String,
    pub medium_description_en: String,
    pub short_description_en: String,
    pub before_use: String,
    pub instructions_en: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "isVisibleInListView"
    )]
    pub is_visible_in_list_view: Option<bool>,
    pub purpose_en: Vec<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at2: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub disease_en: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    pub specialty_en: Vec<String>,
    pub chief_complaint_en: Vec<String>,
    pub system_en: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<Version>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "versionNumber")]
    pub version_number: Option<i64>,
    pub search_abbreviation_en: Vec<String>,
    pub slug: String,
    pub seo: Seo,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none", rename = "publishedAt")]
    pub published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeEndDate")]
    pub cme_end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeLastReviewed")]
    pub cme_last_reviewed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeReleaseDate")]
    pub cme_release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cmeVersion")]
    pub cme_version: Option<String>,
    pub cme_status: String,
    pub equation_logic_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
    pub search_id: String,
    pub md5: String,
    pub input_schema: Vec<InputSchema>,
    pub logs: Vec<Log>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equation_logic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorFull {
    #[serde(rename = "about_en", skip_serializing_if = "Option::is_none")]
    pub about_en: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deceased: Option<bool>,
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    #[serde(rename = "photo_en", skip_serializing_if = "Option::is_none")]
    pub photo_en: Option<String>,
    pub published_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubmed_link: Option<String>,
    pub signed_c_o_i: bool,
    pub updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CmeFaq {
    pub answer: String,
    pub created_at: String,
    pub published_at: String,
    pub question: String,
    pub r#type: String,
    pub updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Faq {
    pub cme_faq: CmeFaq,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadConfig {
    pub canonical_url: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    pub title: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    pub conversion: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub error_max: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_max_si: Option<Number>,
    pub error_max_us: Number,
    pub error_min: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_min_si: Option<Number>,
    pub error_min_us: Number,
    pub name: String,
    pub normal_max_si: Number,
    pub normal_max_us: Number,
    pub normal_min_si: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_min_us: Option<Number>,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub unit: String,
    pub units_si: String,
    pub units_us: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub warn_max: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn_max_si: Option<Number>,
    pub warn_max_us: Number,
    pub warn_min: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn_min_si: Option<Number>,
    pub warn_min_us: Number,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrI64 {
    String(String),
    I64(i64),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedCalc {
    #[serde(rename = "calcId")]
    pub calc_id: StringOrI64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_title_en: Option<String>,
    pub slug: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Society {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_link: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootContributor {
    #[serde(rename = "contributors_page")]
    pub contributors_page: bool,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub first_name: String,
    pub has_disclosure: Vec<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,
    pub last_name: String,
    pub name: String,
    pub published_at: String,
    pub signed_c_o_i: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub target: String,
    pub updated_at: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageProps {
    // CANONICAL_URL_ROOT
    // RECAPTCHA_PUBLIC_KEY
    // STRAPI_TAG
    // STRIPE_PUB_KEY
    pub envs: HashMap<String, String>,
    pub calc: Calc,
    pub contributors: Vec<RootContributor>,
    pub creators: Vec<CreatorFull>,
    pub faqs: Vec<Faq>,
    pub head_config: HeadConfig,
    pub rel_calcs: Vec<RelatedCalc>,
    pub is_c_m_e_calc: bool,
    pub measurements: Vec<Measurement>,
    pub societies: HashMap<String, Society>,
    // [ "whenToUseViewed", "howToUseViewed" ]
    pub valid_sections: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Props {
    #[serde(rename = "__N_SSP")]
    pub _n_ssp: bool,
    pub page_props: PageProps,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub slug: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub props: Props,
    pub page: String,
    pub query: Query,
    pub build_id: String,
    pub is_fallback: bool,
    pub gssp: bool,
    pub script_loader: Vec<()>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::test::*;

    use InputSchema::{Dropdown, Radio, Subheading, Textbox, Toggle, Visual};

    impl InputSchema {
        /// Non-empty ident, unique to all [`InputSchema`]s in a [`Calc`].
        pub fn name(&self) -> Option<&str> {
            match self {
                Dropdown { name, .. }
                | Radio { name, .. }
                | Textbox { name, .. }
                | Toggle { name, .. } => Some(name.as_str()),
                Visual { .. } | Subheading { .. } => None,
            }
        }
    }

    #[test]
    fn all_measurement_units_are_used() {
        for Root {
            props:
                Props {
                    page_props:
                        PageProps {
                            calc: Calc { input_schema, .. },
                            measurements,
                            ..
                        },
                    ..
                },
            ..
        } in all()
        {
            let units_in_schema = input_schema
                .iter()
                .filter_map(|it| match it {
                    Textbox { unit, .. } => Some(unit.as_str()),
                    _ => None,
                })
                .collect::<HashSet<_>>();
            let defined_units = measurements
                .iter()
                .map(|it| it.unit.as_str())
                .collect::<HashSet<_>>();
            pretty_assertions::assert_eq!(units_in_schema, defined_units);
        }
    }

    #[test]
    fn input_schema_names_are_unique_in_calcs() {
        for root in all() {
            let mut unique = HashSet::new();
            for name in root
                .props
                .page_props
                .calc
                .input_schema
                .iter()
                .filter_map(InputSchema::name)
            {
                let is_unique = unique.insert(name);
                if !is_unique {
                    panic!("duplicate key: {}", name)
                }
            }
        }
    }

    /// Check that we capture all the information
    #[test]
    fn round_trip_all() {
        for file in include_dir::include_dir!("$CARGO_MANIFEST_DIR/scraped").files() {
            print!("{}...", file.path().display());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            test_roundtrip::<Root>(&[
                serde_json::from_slice(file.contents()).expect("invalid json")
            ]);
            println!("ok.");
        }
    }

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
                Err(e) => {
                    panic!(
                        "Failed to serialize a {} from json: {e}",
                        std::any::type_name::<T>()
                    )
                }
            }
        }
    }

    fn redact_nulls_in_objects(value: serde_json::Value) -> serde_json::Value {
        pub use serde_json::Value::{Array, Bool, Null, Number, Object, String};
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
}
