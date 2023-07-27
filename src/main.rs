use clap::{Parser, Subcommand};
use clap_stdin::MaybeStdin;
use dotenvy::dotenv;
use llmify::{
    clients::OpenAIClient,
    models::{LanguageModel, OpenAILanguageModel},
    prompts::{Prompt, QUERY, SUMMARIZE},
};

#[derive(Debug, Parser, Clone)]
struct TaskArgs {
    task: MaybeStdin<String>,
}

#[derive(Debug, Parser, Clone)]
struct QAArgs {
    #[clap(short, long)]
    question: String,
    task: MaybeStdin<String>,
}

#[derive(Debug, Clone, Subcommand)]
enum Mode {
    Summarize(TaskArgs),
    Ask(QAArgs),
}

#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Cli {
    #[clap(subcommand)]
    task_type: Mode,
    // This is the main task to be performed
    #[clap(short, long, default_value = "gpt-3.5-turbo")]
    model: String,
    #[clap(short, long, default_value = "0.7")]
    temperature: f32,
    #[clap(short, long)]
    custom_instructions: Option<String>,
}

fn main() {
    let command = Cli::parse();
    dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = OpenAIClient::new(&api_key);
    let language_model = OpenAILanguageModel::new(&client, command.model.parse().unwrap());

    match command.task_type {
        Mode::Summarize(sargs) => {
            let response = process_task(
                &language_model,
                SUMMARIZE,
                &sargs.task,
                &command.custom_instructions,
            );
            println!("{}", response);
        }
        Mode::Ask(qa_args) => {
            let task = format!("QUESTION: {}\nINPUT: {}", qa_args.question, qa_args.task);
            let response =
                process_task(&language_model, QUERY, &task, &command.custom_instructions);
            println!("{}", response);
        }
    }
}

fn process_task<'a, T: LanguageModel<Prompt>>(
    model: &'a T,
    prompt: &'a str,
    task: &'a str,
    custom_instructions: &Option<String>,
) -> String {
    let formatted_prompt = custom_instructions.as_ref().map_or_else(
        || Prompt::new(prompt, task),
        |instructions| Prompt::with_custom_instructions(prompt, task, instructions),
    );
    model.generate(formatted_prompt)
}
