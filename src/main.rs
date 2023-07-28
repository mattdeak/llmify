use clap::{Parser, Subcommand, ValueEnum};
use clap_stdin::MaybeStdin;
use dotenvy::dotenv;
use llmify::{
    clients::OpenAIClient,
    models::{openai::OpenAILanguageModel, LanguageModel},
    prompts::{
        instructions::BehaviourInstructions,
        prompt::Prompt,
        templates::{InstructionSelector, ASK_QUESTION, QUERY_DATA, SUMMARIZE},
    },
};

#[derive(Debug, Parser, Clone)]
struct TaskArgs {
    task: MaybeStdin<String>,
}

#[derive(Debug, Parser, Clone)]
struct QueryArgs {
    #[clap(short, long)]
    question: String,
    task: MaybeStdin<String>,
}

#[derive(Debug, Parser, Clone)]
struct QuestionArgs {
    question: String,
}

#[derive(Debug, Clone, Subcommand)]
enum Mode {
    Summarize(TaskArgs),
    Query(QueryArgs),
    Ask(QuestionArgs),
}

#[derive(Debug, Clone, Parser, ValueEnum)]
enum Behaviour {
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
    behaviour_instructions: Option<Behaviour>,

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

    let formatter = match command.behaviour_instructions {
        Some(Behaviour::TOT) => Some(InstructionSelector::tree_of_thoughts()),
        Some(Behaviour::SelfCritique) => Some(InstructionSelector::self_critique()),
        None => None,
    };
    println!("Using formatter: {:?}", formatter);

    match command.task_type {
        Mode::Summarize(sargs) => {
            let response = process_input(&language_model, SUMMARIZE, &sargs.task, &formatter);
            println!("{}", response);
        }
        Mode::Query(qa_args) => {
            let task = format!("QUESTION: {}\nINPUT: {}", qa_args.question, qa_args.task);
            let response = process_input(&language_model, QUERY_DATA, &task, &formatter);
            println!("{}", response);
        }
        Mode::Ask(q_args) => {
            let response =
                process_input(&language_model, ASK_QUESTION, &q_args.question, &formatter);
            println!("{}", response);
        }
    }
}

fn process_input<'a, T: LanguageModel>(
    model: &'a T,
    prompt: &'a str,
    input: &'a str,
    behaviour_instructions: &Option<BehaviourInstructions>,
) -> String {
    let mut prompt_builder = Prompt::new().with_prefix(prompt).with_task(input);

    if let Some(instructions) = behaviour_instructions {
        prompt_builder = prompt_builder.with_behaviour_instructions(instructions);
    }
    let final_prompt = prompt_builder.build().unwrap();

    model.generate(final_prompt).unwrap()
}
