use futures::StreamExt;
use telegram_bot::*;
use tokio;

const OPENAI_API_KEY: &str = "YOUR_API_KEY";

async fn generate_text(api: Api, message: Message) {
    let text = message.text.unwrap();

    // Call the OpenAI API to generate text. Dont forget for cargo file!
    let response = openai::CompletionBuilder::default()
        .engine("davinci")
        .prompt(text)
        .max_tokens(1024)
        .n(1)
        .temperature(0.5)
        .build()
        .unwrap()
        .send(OPENAI_API_KEY)
        .await
        .unwrap();

    // Extract the generated text from the API response
    let generated_text = response.choices[0].text.clone();

    // Send the generated text back to the user
    api.send(SendMessage::new(message.chat.id(), generated_text))
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), telegram_bot::Error> {
    // Replace YOUR_BOT_TOKEN with your actual bot token
    let api = Api::new("YOUR_BOT_TOKEN");

    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        // If the update contains a new message
        if let Some(message) = update.message {
            tokio::spawn(generate_text(api.clone(), message));
        }
    }

    Ok(())
}