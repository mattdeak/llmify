mod models;
mod prompts;
use clap::{Parser, ValueEnum};
use prompts::format_prompt;

#[derive(Debug, Clone, Parser, ValueEnum)]
enum Mode {
    Summarize,
    QA,
    Continue,
    Custom,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
enum ModelChoice {
    Gpt3Turbo,
    Gpt3Turbo16k,
    Gpt4,
}

#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Cli {
    #[arg(value_enum)]
    task_type: Mode,
    // This is the main task to be performed
    task: String,

    #[clap(long, default_value = "gpt3-turbo")]
    model: ModelChoice,
    #[clap(long, default_value = "0.7")]
    temperature: f32,
    custom_instructions: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.task_type {
        Mode::Summarize => {
            let prompt = prompts::SUMMARIZE;
            let task = args.task;
            let custom_instructions = args.custom_instructions;
            let formatted_prompt = format_prompt(prompt, &task, &custom_instructions);
            println!("{}", formatted_prompt);
        }
        _ => {
            unimplemented!();
        }
    }
}
