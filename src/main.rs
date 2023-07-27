use clap::{Parser, ValueEnum};
use clap_stdin::MaybeStdin;
use dotenvy::dotenv;
use llmify::{
    clients::OpenAIClient,
    models::{LanguageModel, OpenAILanguageModel},
    prompts::{Prompt, SUMMARIZE},
};

#[derive(Debug, Clone, Parser, ValueEnum)]
enum Mode {
    Summarize,
    QA,
    Continue,
    Custom,
}

#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Cli {
    #[arg(value_enum)]
    task_type: Mode,
    // This is the main task to be performed
    task: MaybeStdin<String>,

    #[clap(short, long, default_value = "gpt-3.5-turbo")]
    model: String,
    #[clap(short, long, default_value = "0.7")]
    temperature: f32,
    #[clap(short, long)]
    custom_instructions: Option<String>,
}

fn main() {
    let args = Cli::parse();
    dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = OpenAIClient::new(&api_key);
    let language_model = OpenAILanguageModel::new(&client, args.model.parse().unwrap());

    match args.task_type {
        Mode::Summarize => {
            let prompt = SUMMARIZE;
            let task = args.task;
            let custom_instructions = args.custom_instructions;
            let formatted_prompt = match custom_instructions {
                Some(instructions) => {
                    Prompt::with_custom_instructions(prompt, &task, &instructions)
                }
                None => Prompt::new(prompt, &task),
            };
            dbg!(&formatted_prompt);
            let response = language_model.generate(&formatted_prompt);
            println!("{}", response);
        }
        _ => {
            unimplemented!();
        }
    }
}
