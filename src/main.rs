use std::env;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;
use sysinfo::{System, Disks};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Comandos:")]
enum Command {
    #[command(description = "Métricas completas do servidor")]
    Stats,
    #[command(description = "Uso de CPU")]
    Cpu,
    #[command(description = "Uso de memória RAM")]
    Mem,
    #[command(description = "Uso de disco")]
    Disk,
    #[command(description = "Tempo de funcionamento")]
    Uptime,
    #[command(description = "Status do VSCode Tunnel")]
    Tunnel,
}

fn format_bytes(bytes: u64) -> String {
    let kb = bytes as f64 / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;
    if gb >= 1.0 {
        format!("{:.1} GB", gb)
    } else if mb >= 1.0 {
        format!("{:.1} MB", mb)
    } else {
        format!("{:.0} KB", kb)
    }
}

fn get_tunnel_status() -> String {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("ps aux | grep -v grep | grep 'code tunnel' || echo 'OFF'")
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.trim().contains("code tunnel") {
                "🟢 ON - Tunnel ativo".to_string()
            } else {
                "🔴 OFF - Tunnel não está rodando".to_string()
            }
        }
        Err(_) => "⚠️ Erro ao verificar tunnel".to_string(),
    }
}

async fn handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Stats => {
            let mut sys = System::new_all();
            sys.refresh_all();

            let cpu = sys.global_cpu_usage();
            let mem_used = sys.used_memory();
            let mem_total = sys.total_memory();
            let mem_percent = (mem_used as f64 / mem_total as f64 * 100.0) as u32;

            let disks = Disks::new_with_refreshed_list();
            let disk_info: Vec<String> = disks.iter().map(|d| {
                let total = d.total_space();
                let available = d.available_space();
                let used = total.saturating_sub(available);
                let percent = if total > 0 {
                    (used as f64 / total as f64 * 100.0) as u32
                } else {
                    0
                };
                format!("📦 {}: {} / {} ({}%)",
                    d.mount_point().display(),
                    format_bytes(used),
                    format_bytes(total),
                    percent)
            }).collect();

            let uptime = System::uptime();
            let days = uptime / 86400;
            let hours = (uptime % 86400) / 3600;
            let mins = (uptime % 3600) / 60;

            let tunnel = get_tunnel_status();

            let stats = format!(
                "📊 *STATS*\n\n\
                🖥️ *CPU:* {:.1}%\n\n\
                🧠 *MEM:* {} / {} ({}%)\n\n\
                💾 *DISCO:*\n{}\n\n\
                ⏱️ *UPTIME:* {}d {}h {}m\n\n\
                🔗 *TUNNEL:* {}",
                cpu, format_bytes(mem_used), format_bytes(mem_total), mem_percent,
                disk_info.join("\n"),
                days, hours, mins,
                tunnel
            );

            bot.send_message(msg.chat.id, stats).parse_mode(ParseMode::MarkdownV2).await?;
        }

        Command::Cpu => {
            let mut sys = System::new_all();
            sys.refresh_cpu_all();

            let cpu = sys.global_cpu_usage();
            let load_avg = std::fs::read_to_string("/proc/loadavg")
                .map(|l| {
                    let parts: Vec<&str> = l.split_whitespace().take(3).collect();
                    parts.join(" ")
                })
                .unwrap_or_else(|_| "N/A".to_string());

            let msg_text = format!("🖥️ *CPU*\n\n📈 Uso: *{:.1}%*\n\n📉 Load avg (1m, 5m, 15m): *{}*", cpu, load_avg);
            bot.send_message(msg.chat.id, msg_text).parse_mode(ParseMode::MarkdownV2).await?;
        }

        Command::Mem => {
            let mut sys = System::new_all();
            sys.refresh_memory();

            let total = sys.total_memory();
            let used = sys.used_memory();
            let swap = sys.total_swap();
            let swap_used = sys.used_swap();
            let percent = (used as f64 / total as f64 * 100.0) as u32;

            let msg_text = format!(
                "🧠 *MEMORY*\n\n\
                📊 RAM: {} / {} ({}%)\n\n\
                💽 SWAP: {} / {}",
                format_bytes(used), format_bytes(total), percent,
                format_bytes(swap_used), format_bytes(swap)
            );
            bot.send_message(msg.chat.id, msg_text).parse_mode(ParseMode::MarkdownV2).await?;
        }

        Command::Disk => {
            let disks = Disks::new_with_refreshed_list();

            if disks.is_empty() {
                bot.send_message(msg.chat.id, "💾 Nenhum disco encontrado").await?;
            } else {
                let info: Vec<String> = disks.iter().map(|d| {
                    let total = d.total_space();
                    let available = d.available_space();
                    let used = total.saturating_sub(available);
                    let percent = if total > 0 {
                        (used as f64 / total as f64 * 100.0) as u32
                    } else {
                        0
                    };
                    format!(
                        "📦 *{}*\n   {} / {} ({}%)\n   Livre: {}",
                        d.mount_point().display(),
                        format_bytes(used),
                        format_bytes(total),
                        percent,
                        format_bytes(available)
                    )
                }).collect();

                let msg_text = format!("💾 *DISK*\n\n{}", info.join("\n\n"));
                bot.send_message(msg.chat.id, msg_text).parse_mode(ParseMode::MarkdownV2).await?;
            }
        }

        Command::Uptime => {
            let uptime = System::uptime();
            let days = uptime / 86400;
            let hours = (uptime % 86400) / 3600;
            let mins = (uptime % 3600) / 60;
            let secs = uptime % 60;

            let boot_time = System::boot_time();
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let msg_text = format!(
                "⏱️ *UPTIME*\n\n\
                ⌛ Tempo ligado: *{}d {}h {}m {}s*\n\n\
                🕐 Boot: <t:{}:R>\n\
                🕓 Agora: <t:{}:R>",
                days, hours, mins, secs,
                boot_time, now
            );
            bot.send_message(msg.chat.id, msg_text).parse_mode(ParseMode::Html).await?;
        }

        Command::Tunnel => {
            let status = get_tunnel_status();
            bot.send_message(msg.chat.id, status).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set");
    let chat_id: i64 = env::var("CHAT_ID").expect("CHAT_ID not set").parse()
        .expect("CHAT_ID must be a number");

    let bot = Bot::new(token);

    bot.send_message(ChatId(chat_id), "🟢 Servidor ligou — Bot ativo!")
        .await
        .expect("Failed to send startup message");

    println!("Dockode bot iniciado. Aguardando comandos...");

    Command::repl(bot, handler).await;
}