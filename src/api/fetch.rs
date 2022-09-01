use super::question::{LeetCodeQuestion, Question};
use reqwest::{Client, Error, Method};
use serde_json::json;

static LEETCODE_GRAPHQL_ENDPOINT: &str = "https://leetcode.com/graphql";

pub async fn fetch_question(name: &str) -> Result<Question, Error> {
    Ok(Client::new()
        .request(Method::POST, LEETCODE_GRAPHQL_ENDPOINT)
        .json(&json!({
            "operationName":"questionData",
            "variables":{"titleSlug": name},
            "query":"query questionData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    boundTopicId\n    title\n    titleSlug\n    content\n    translatedTitle\n    translatedContent\n    isPaidOnly\n    difficulty\n    likes\n    dislikes\n    isLiked\n    similarQuestions\n    exampleTestcases\n    categoryTitle\n    contributors {\n      username\n      profileUrl\n      avatarUrl\n      __typename\n    }\n    topicTags {\n      name\n      slug\n      translatedName\n      __typename\n    }\n    companyTagStats\n    codeSnippets {\n      lang\n      langSlug\n      code\n      __typename\n    }\n    stats\n    hints\n    solution {\n      id\n      canSeeDetail\n      paidOnly\n      hasVideoSolution\n      paidOnlyVideo\n      __typename\n    }\n    status\n    sampleTestCase\n    metaData\n    judgerAvailable\n    judgeType\n    mysqlSchemas\n    enableRunCode\n    enableTestMode\n    enableDebugger\n    envInfo\n    libraryUrl\n    adminUrl\n    challengeQuestion {\n      id\n      date\n      incompleteChallengeCount\n      streakCount\n      type\n      __typename\n    }\n    __typename\n  }\n}\n"
        }))
        .send()
        .await?
        .json::<LeetCodeQuestion>()
        .await?.data.question
    )
}
