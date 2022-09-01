use leetcode_rs::api::fetch::fetch_question;

use std::{env, fs::OpenOptions, io::Write};

static TEST_TEMPLATE: &str = "#[test]\nfn test1() {\n\ttodo!()\n}";
static MAIN_TEMPLATE: &str = "#[allow(unused)]\nstruct Solution {}";

#[tokio::main]
async fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let name = args.next().unwrap();
    let question = fetch_question(&name).await.unwrap();
    let question_id = question.question_id.parse::<u16>().unwrap();

    let question_path_name = format!("s_{:04}_{}", question_id, name.replace('-', "_"));

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("src/solutions/{question_path_name}.rs"))
        .unwrap();

    let snippet = question
        .code_snippets
        .iter()
        .find(|e| e.lang_slug == "rust")
        .unwrap()
        .code
        .replace("        ", "\t\ttodo!()")
        .replace("impl", "#[allow(unused)]\nimpl");

    write!(
        file,
        "{}\n\n{}\n\n{}",
        MAIN_TEMPLATE, snippet, TEST_TEMPLATE
    )
    .unwrap();

    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open("src/solutions/mod.rs")
        .unwrap();

    writeln!(file, "mod {};", question_path_name).unwrap();
}
