use chrono::Local;
use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use toml_edit::DocumentMut;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(index = 1)]
    title_slug: Option<String>,

    #[arg(index = 2)]
    language_file_ext: Option<String>,
}

#[derive(Deserialize)]
struct QuestionData {
    question: Question,
}

#[derive(Deserialize)]
struct Question {
    title: String,
    #[serde(rename = "questionFrontendId")]
    frontend_id: String,
    difficulty: QuestionDifficulty,
}

#[derive(Deserialize)]
struct QuestionContentData {
    question: QuestionContent,
}

#[derive(Deserialize)]
struct QuestionContent {
    content: String,
}

#[derive(Deserialize)]
struct QuestionTopicData {
    question: QuestionTopicTags,
}

#[derive(Deserialize)]
struct QuestionTopicTags {
    #[serde(rename = "topicTags")]
    topic_tags: Vec<QuestionTopicTag>,
}

#[derive(Deserialize)]
struct QuestionTopicTag {
    name: String,
}

#[derive(Serialize)]
struct QuestionAttributes {
    difficulty: QuestionDifficulty,
    topics: Vec<String>,
}

#[derive(Deserialize)]
struct QuestionBoilerplateData {
    question: QuestionBoilerplate,
}

#[derive(Deserialize)]
struct QuestionBoilerplate {
    #[serde(rename = "codeSnippets")]
    code_snippets: Vec<CodeSnippet>,
}

#[derive(Deserialize)]
struct CodeSnippet {
    lang: String,
    #[serde(rename = "langSlug")]
    lang_slug: String,
    /// code is returned from Leetcode API as string representation
    code: String,
}

#[derive(Serialize)]
struct AttemptAttributes {
    success: bool,
    perceived_trickiness: u8,
    attempt_start_time: toml_edit::Datetime,
    attempt_end_time: toml_edit::Datetime,
    reflections: String,
}

#[derive(Serialize, Deserialize)]
enum QuestionDifficulty {
    Easy,
    Medium,
    Hard,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_pandoc()?;
    let cli = Cli::parse();

    if let (Some(title_slug), Some(language_file_ext)) = (cli.title_slug, cli.language_file_ext) {
        log_practice(title_slug, validate_lang_file_ext(language_file_ext)?).await?;
    } else {
        eprintln!("Usage: `grindset <problem_title_slug e.g. two-sum> <language file extention WITHOUT dot; e.g. `py`, `js`, `cpp`, `rs`>`");
    }
    Ok(())
}

#[derive(Debug)]
enum LeetcodeSupportedLang {
    Go,
    Java,
    /// NOTE: we are pretending only python3 exists
    Python3,
    JavaScript,
    TypeScript,
    Cpp,
    C,
    Swift,
    Kotlin,
    Dart,
    Ruby,
    Scala,
    Rust,
    Racket,
    Erlang,
    Elixir,
}
impl TryFrom<&str> for LeetcodeSupportedLang {
    type Error = Box<dyn std::error::Error>;
    /// Expects file extensions (without the dot) of supported programming languages
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cpp" => Ok(Self::Cpp),
            "java" => Ok(Self::Java),
            "py" => Ok(Self::Python3),
            "c" => Ok(Self::C),
            "js" => Ok(Self::JavaScript),
            "ts" => Ok(Self::TypeScript),
            "swift" => Ok(Self::Swift),
            "kt" => Ok(Self::Kotlin),
            "dart" => Ok(Self::Dart),
            "go" => Ok(Self::Go),
            "rb" => Ok(Self::Ruby),
            "scala" => Ok(Self::Scala),
            "rs" => Ok(Self::Rust),
            "rkt" => Ok(Self::Racket),
            "erl" => Ok(Self::Erlang),
            "ex" | "exs" => Ok(Self::Elixir),
            unsupp_ext => Err(format!("Unsupported file extension: {}", unsupp_ext).into()),
        }
    }
}

impl LeetcodeSupportedLang {
    fn leetcode_slug(&self) -> &str {
        match self {
            LeetcodeSupportedLang::Go => "golang",
            LeetcodeSupportedLang::Java => "java",
            LeetcodeSupportedLang::Python3 => "python3",
            LeetcodeSupportedLang::JavaScript => "javascript",
            LeetcodeSupportedLang::TypeScript => "typescript",
            LeetcodeSupportedLang::Cpp => "cpp",
            LeetcodeSupportedLang::C => "c",
            LeetcodeSupportedLang::Swift => "swift",
            LeetcodeSupportedLang::Kotlin => "kotlin",
            LeetcodeSupportedLang::Dart => "dart",
            LeetcodeSupportedLang::Ruby => "ruby",
            LeetcodeSupportedLang::Scala => "scala",
            LeetcodeSupportedLang::Rust => "rust",
            LeetcodeSupportedLang::Racket => "racket",
            LeetcodeSupportedLang::Erlang => "erlang",
            LeetcodeSupportedLang::Elixir => "elixir",
        }
    }
}

/// Validates and normalizes a language file extension by removing the leading dot if present.
/// Returns an error if the resulting language code is empty.
fn validate_lang_file_ext(parsed: String) -> Result<String, Box<dyn std::error::Error>> {
    let lang_code: String = parsed.trim_matches(|c| c == ' ' || c == '.').into();
    match &lang_code.is_empty() {
        true => Err("Language code is empty after trimming".into()),
        false => Ok(lang_code),
    }
}

fn check_pandoc() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("pandoc").arg("--version").output()?;

    if !output.status.success() {
        return Err("Pandoc is not installed or not in PATH".into());
    }

    eprintln!(
        "Pandoc version: {}",
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or("Unknown")
    );
    Ok(())
}

async fn log_practice(
    title_slug: String,
    language_file_ext: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let question_data = fetch_question_data(&title_slug).await?;

    let question_id = question_data.question.frontend_id.clone();
    let question_name = question_data.question.title.clone();
    let question_folder_name = format!("{}_{}", question_id, question_name.replace(" ", "_"));
    let language_folder = Path::new(&language_file_ext);
    let full_question_folder_path = language_folder.join(&question_folder_name);
    fs::create_dir_all(&full_question_folder_path)?;

    // Create description.md
    let question_content_markdown =
        html_to_markdown(&fetch_question_content(&title_slug).await?.question.content)?;
    let question_content_file_path = full_question_folder_path.join("question.md");
    let mut question_content_file = File::create(question_content_file_path)?;
    question_content_file.write_all(question_content_markdown.as_bytes())?;

    // Create question_attributes.toml
    let question_topics: Vec<String> = fetch_question_topics(&title_slug)
        .await?
        .question
        .topic_tags
        .into_iter()
        .map(|tag| tag.name)
        .collect();
    let attributes = QuestionAttributes {
        difficulty: question_data.question.difficulty,
        topics: question_topics,
    };
    let attributes_toml = toml::to_string(&attributes)?;
    let attributes_path = full_question_folder_path.join("question_attributes.toml");
    let mut attributes_file = File::create(attributes_path)?;
    attributes_file.write_all(attributes_toml.as_bytes())?;

    // Create attempt folder and files
    let attempt_datetime = Local::now();
    let attempt_folder =
        full_question_folder_path.join(attempt_datetime.format("%Y%m%d_%H%M%S").to_string());
    fs::create_dir_all(&attempt_folder)?;

    let attempt_file_path = attempt_folder.join(format!("attempt.{}", language_file_ext));
    let mut attempt_file = File::create(attempt_file_path)?;
    let question_boilerplate = fetch_question_boilerplate(&title_slug).await?;
    let language = LeetcodeSupportedLang::try_from(language_file_ext.borrow())?;
    let boilerplate_for_lang = question_boilerplate
        .question
        .code_snippets
        .into_iter()
        .find(|snippet| snippet.lang_slug == language.leetcode_slug())
        .map(|found| found.code);
    if let Some(code) = boilerplate_for_lang {
        attempt_file.write_all(code.as_bytes())?;
    } else {
        eprintln!("No boilerplate for language: {:#?}", language);
    };

    let toml_datetime = toml_edit::Datetime::from_str(&attempt_datetime.to_rfc3339())?;
    let attempt_attributes = AttemptAttributes {
        success: false,
        perceived_trickiness: 1,
        attempt_start_time: toml_datetime,
        attempt_end_time: toml_datetime,
        reflections: "".to_string(),
    };
    let mut attempt_attributes_toml_document =
        toml::to_string(&attempt_attributes)?.parse::<DocumentMut>()?;

    // Add comment explaining how to fill in `perceived_trickiness`
    attempt_attributes_toml_document
        .get_mut("perceived_trickiness")
        .expect("`perceived_trickiness` should be in the toml document.")
        .as_value_mut()
        .expect("should exist.")
        .decor_mut()
        .set_suffix(" # 7-point scale: 1 is brain-dead, 7 is diabolical");

    // Serialize `reflections as a multiline string`
    attempt_attributes_toml_document
        .get_mut("reflections")
        .expect("`reflections` should be in the toml document.")
        .as_value_mut()
        .expect("should exist.")
        .decor_mut()
        .set_prefix(" \"\"");

    attempt_attributes_toml_document
        .get_mut("reflections")
        .expect("`reflections` should be in the toml document.")
        .as_value_mut()
        .expect("should exist.")
        .decor_mut()
        .set_suffix("\"\" # This is multiline!");

    let attempt_attributes_path = attempt_folder.join("attempt_attributes.toml");
    let mut attempt_attributes_file = File::create(attempt_attributes_path)?;
    attempt_attributes_file.write_all(attempt_attributes_toml_document.to_string().as_bytes())?;

    eprintln!("All done. Grindset time! ");
    println!("{:#?}", attempt_folder);
    Ok(())
}

fn html_to_markdown(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create a temporary file to store the HTML content
    let mut temp_file = tempfile::NamedTempFile::new()?;
    temp_file.write_all(html.as_bytes())?;
    let temp_path = temp_file.path().to_str().ok_or("Invalid temp file path")?;

    // Run pandoc command
    let output = Command::new("pandoc")
        .arg("-f")
        .arg("html")
        .arg("-t")
        .arg("markdown")
        .arg("-o")
        .arg("-")
        .arg(temp_path)
        .output()?;

    if !output.status.success() {
        return Err(format!("Pandoc error: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

async fn execute_graphql_query<T: serde::de::DeserializeOwned>(
    query_string: &str,
    variables: serde_json::Value,
) -> Result<T, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .post("https://leetcode.com/graphql")
        .json(&serde_json::json!({
            "query": query_string,
            "variables": variables
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let question_data: T = serde_json::from_value(response["data"].clone())?;
    Ok(question_data)
}

async fn fetch_question_topics(
    title_slug: &str,
) -> Result<QuestionTopicData, Box<dyn std::error::Error>> {
    execute_graphql_query(
        r#"
        query singleQuestionTopicTags($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                topicTags {
                name
                }
            }
        }
    "#,
        serde_json::json!({
            "titleSlug": title_slug
        }),
    )
    .await
}

async fn fetch_question_boilerplate(
    title_slug: &str,
) -> Result<QuestionBoilerplateData, Box<dyn std::error::Error>> {
    execute_graphql_query(
        r#"
        query questionEditorData($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                codeSnippets {
                lang
                langSlug
                code
                }
            }
        }
    "#,
        serde_json::json!({
            "titleSlug": title_slug
        }),
    )
    .await
}

async fn fetch_question_content(
    title_slug: &str,
) -> Result<QuestionContentData, Box<dyn std::error::Error>> {
    execute_graphql_query(
        r#"
        query questionContent($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                content
            }
        }
    "#,
        serde_json::json!({
            "titleSlug": title_slug
        }),
    )
    .await
}
async fn fetch_question_data(title_slug: &str) -> Result<QuestionData, Box<dyn std::error::Error>> {
    execute_graphql_query(
        r#"
        query questionTitle($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
            questionFrontendId
            title
            difficulty
            }
        }
    "#,
        serde_json::json!({
            "titleSlug": title_slug
        }),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_extensions() {
        assert_eq!(
            validate_lang_file_ext("js".to_string()).unwrap(),
            "js".to_string()
        );
        assert_eq!(
            validate_lang_file_ext("py".to_string()).unwrap(),
            "py".to_string()
        );
        assert_eq!(
            validate_lang_file_ext(".js".to_string()).unwrap(),
            "js".to_string()
        );
        assert_eq!(
            validate_lang_file_ext(".ts".to_string()).unwrap(),
            "ts".to_string()
        );
        assert_eq!(
            validate_lang_file_ext("   ..ts".to_string()).unwrap(),
            "ts".to_string()
        );
        assert_eq!(
            validate_lang_file_ext(".cpp".to_string()).unwrap(),
            "cpp".to_string()
        );
        assert_eq!(
            validate_lang_file_ext("...cpp".to_string()).unwrap(),
            "cpp".to_string()
        );
        assert_eq!(
            validate_lang_file_ext("...  cpp...".to_string()).unwrap(),
            "cpp".to_string()
        );
    }

    #[test]
    fn test_invalid_extensions() {
        assert!(validate_lang_file_ext(".".to_string()).is_err());
        assert!(validate_lang_file_ext("...".to_string()).is_err());
        assert!(validate_lang_file_ext("".to_string()).is_err());
    }

    #[test]
    fn test_error_message() {
        let error = validate_lang_file_ext("".to_string()).unwrap_err();
        assert_eq!(error.to_string(), "Language code is empty after trimming");
    }
}
