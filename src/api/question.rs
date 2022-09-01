use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeetCodeQuestion {
    pub data: Data,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub question: Question,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question_id: String,
    pub question_frontend_id: String,
    pub bound_topic_id: Value,
    pub title: String,
    pub title_slug: String,
    pub content: String,
    pub translated_title: Value,
    pub translated_content: Value,
    pub is_paid_only: bool,
    pub difficulty: String,
    pub likes: i64,
    pub dislikes: i64,
    pub is_liked: Value,
    pub similar_questions: String,
    pub example_testcases: String,
    pub category_title: String,
    pub contributors: Vec<Value>,
    pub topic_tags: Vec<TopicTag>,
    pub company_tag_stats: Value,
    pub code_snippets: Vec<CodeSnippet>,
    pub stats: String,
    pub hints: Vec<Value>,
    pub solution: Solution,
    pub status: Value,
    pub sample_test_case: String,
    pub meta_data: String,
    pub judger_available: bool,
    pub judge_type: String,
    pub mysql_schemas: Vec<Value>,
    pub enable_run_code: bool,
    pub enable_test_mode: bool,
    pub enable_debugger: bool,
    pub env_info: String,
    pub library_url: Value,
    pub admin_url: Value,
    pub challenge_question: Value,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicTag {
    pub name: String,
    pub slug: String,
    pub translated_name: Value,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSnippet {
    pub lang: String,
    pub lang_slug: String,
    pub code: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    pub id: String,
    pub can_see_detail: bool,
    pub paid_only: bool,
    pub has_video_solution: bool,
    pub paid_only_video: bool,
    #[serde(rename = "__typename")]
    pub typename: String,
}
