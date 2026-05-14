use std::env;

fn main() {
    let token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set");
    let chat_id = env::var("CHAT_ID").expect("CHAT_ID not set");
    let args: Vec<String> = env::args().collect();
    let message = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Mensagem vazia".to_string()
    };

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let params = [("chat_id", chat_id.as_str()), ("text", message.as_str())];

    let client = reqwest::blocking::Client::new();
    match client.post(&url).form(&params).send() {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("OK: mensagem enviada");
            } else {
                println!("ERRO: {}", resp.text().unwrap_or_default());
            }
        }
        Err(e) => println!("ERRO: {}", e),
    }
}
