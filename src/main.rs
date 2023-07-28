use clap::{Parser, Subcommand, ValueEnum};
use clap_stdin::MaybeStdin;
use dotenvy::dotenv;
use llmify::{
    clients::OpenAIClient,
    models::{openai::OpenAILanguageModel, LanguageModel},
    prompts::{
        instructions::BehaviourInstructions,
        prompt::Prompt,
        templates::{InstructionSelector, QUERY, SUMMARIZE},
    },
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

#[derive(Debug, Clone, Parser, ValueEnum)]
enum FormatInstruction {
    TOT,
    SelfCritique,
}

#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Cli {
    // This is the main task to be performed
    #[clap(short, long, default_value = "gpt-3.5-turbo")]
    model: String,
    #[clap(short, long, default_value = "0.7")]
    temperature: f32,
    #[clap(short, long)]
    format_instructions: Option<FormatInstruction>,

    #[clap(subcommand)]
    task_type: Mode,
}

fn main() {
    let command = Cli::parse();
    dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = OpenAIClient::new(&api_key);
    let language_model = OpenAILanguageModel::new(&client, command.model.parse().unwrap());
    println!("Using model: {}", command.model);

    let formatter = match command.format_instructions {
        Some(FormatInstruction::TOT) => Some(InstructionSelector::tree_of_thoughts()),
        Some(FormatInstruction::SelfCritique) => Some(InstructionSelector::self_critique()),
        None => None,
    };
    println!("Using formatter: {:?}", formatter);

    match command.task_type {
        Mode::Summarize(sargs) => {
            let response = process_task(&language_model, SUMMARIZE, &sargs.task, &formatter);
            println!("{}", response);
        }
        Mode::Ask(qa_args) => {
            let task = format!("QUESTION: {}\nINPUT: {}", qa_args.question, qa_args.task);
            let response = process_task(&language_model, QUERY, &task, &formatter);
            println!("{}", response);
        }
    }
}

fn process_task<'a, T: LanguageModel>(
    model: &'a T,
    prompt: &'a str,
    task: &'a str,
    format_instructions: &Option<BehaviourInstructions>,
) -> String {
    let mut prompt_builder = Prompt::new().with_prefix(prompt).with_task(task);

    if let Some(instructions) = format_instructions {
        prompt_builder = prompt_builder.with_format_instructions(instructions);
    }

    let final_prompt = prompt_builder.build().unwrap();
    dbg!(&final_prompt);

    model.generate(final_prompt).unwrap()
}
