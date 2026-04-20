use clap::{Parser};
use aniterm::fastfetch;
use aniterm::ipc::{IpcRequest, IpcResponse, RequestType};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use colored::*;

#[derive(Parser)]
#[command(name = "aniterm")]
#[command(about = "AniTerm: Anime AI Terminal Companion", long_about = None)]
struct Cli {
    #[arg(long, help = "顯示二次元系統資訊")]
    fetch: bool,

    #[arg(long, value_name = "COMMAND", help = "測試 AI 回應")]
    test: Option<String>,

    #[arg(long, help = "強制觸發環境吐槽 (僅用於 --test)", default_value_t = false)]
    force_env: bool,

    #[arg(long, help = "強制指定節日進行測試 (例如: --force-holiday 'Christmas')")]
    force_holiday: Option<String>,

    #[arg(long, help = "進入與當前人格的對話模式")]
    chat: bool,

    #[arg(long, help = "Shell Hook 模式 (內部使用)", hide = true)]
    hook: bool,

    #[arg(long, value_name = "PERSONA", help = "切換當前人格")]
    persona: Option<String>,

    #[arg(last = true, help = "指令與輸出")]
    args: Vec<String>,
}

use textwrap::Options;
use unicode_width::UnicodeWidthStr;

fn print_bubble(name: &str, message: &str) {
    let name_tag = format!(" {} ", name);
    let name_len = name_tag.width();
    
    // 處理文字換行
    let wrap_options = Options::new(50);
    let wrapped = textwrap::fill(message, wrap_options);
    let lines: Vec<&str> = wrapped.lines().collect();
    
    // 計算內部寬度 (不含邊框)
    let max_line_width = lines.iter()
        .map(|l| l.width())
        .max()
        .unwrap_or(0);
    
    // 總內部寬度 w 必須至少能容納名字和最長行，且左右各留一個空格
    let w = max_line_width.max(name_len).max(10) + 2;

    // 頂部邊框與名字
    // 格式:   ╭[名牌]──────╮
    print!("\n  {}", "╭".magenta());
    print!("{}", name_tag.on_magenta().white().bold());
    println!("{}{}", "─".repeat(w - name_len).magenta(), "╮".magenta());

    // 內容行
    // 格式:   │ 內容       │
    for line in lines {
        let line_w = line.width();
        let padding = w - line_w - 2; 
        println!("  {} {} {}{} ", "│".magenta(), line.white(), " ".repeat(padding), "│".magenta());
    }

    // 底部邊框
    // 格式:   ╰────────────╯
    println!("  {}{}{}", "╰".magenta(), "─".repeat(w).magenta(), "╯".magenta());
}

use aniterm::environment;

fn send_to_daemon(request_type: RequestType, command: &str, output: &str, force_env: bool, force_holiday: Option<String>) -> Option<IpcResponse> {
    let socket_path = "/tmp/aniterm.sock";
    let mut stream = UnixStream::connect(socket_path).ok()?;
    
    // 收集環境快照
    let snapshot = environment::collect_snapshot();
    let env_info = format!("時間={}, CPU={:.1}%, MEM={:.1}%", 
        snapshot.current_time, snapshot.cpu_usage, snapshot.mem_usage_percent);
    
    let req = IpcRequest {
        request_type,
        command: command.to_string(),
        output: output.to_string(),
        system_info: Some(env_info),
        force_env,
        force_holiday,
    };
    
    let req_str = serde_json::to_string(&req).ok()?;
    stream.write_all(req_str.as_bytes()).ok()?;
    
    // 提醒：我們不關閉寫入端，因為守護進程只讀取一次
    
    let mut res_str = String::new();
    stream.read_to_string(&mut res_str).ok()?;
    
    if res_str.is_empty() {
        return None;
    }
    
    serde_json::from_str(&res_str).ok()
}

fn main() {
    let cli = Cli::parse();

    if let Some(persona_name) = cli.persona {
        if let Some(res) = send_to_daemon(RequestType::SwitchPersona, &persona_name, "", false, None) {
            println!("{}", res.message.green());
        } else {
            println!("{}", "錯誤：無法連接到 anitermd 守護進程。".red());
        }
        return;
    }

    if cli.chat {
        // 先獲取當前人格名稱
        let persona_display_name = if let Some(res) = send_to_daemon(RequestType::GetStatus, "", "", false, None) {
             res.persona_name
        } else {
            "凜".to_string()
        };

        println!("{} (輸入 'exit' 或 'quit' 退出對話)", format!("--- 與 {} 的對話模式 ---", persona_display_name).cyan().bold());
        
        loop {
            print!("{} ", "➤".cyan().bold());
            std::io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_err() {
                break;
            }
            
            let input = input.trim();
            if input.is_empty() { continue; }
            if input == "exit" || input == "quit" { break; }
            
            print!("  {} ", format!("{} is thinking...", persona_display_name).yellow().italic());
            std::io::stdout().flush().unwrap();

            if let Some(res) = send_to_daemon(RequestType::Chat, input, "", false, None) {
                print!("\r\x1B[K"); 
                print_bubble(&res.persona_name, &res.message);
            } else {
                println!("\r\x1B[K{}", "錯誤：無法連接到守護進程。".red());
                break;
            }
        }
        println!("{}", "對話結束。".cyan());
        return;
    }

    if cli.fetch {
        // 先獲取當前人格名稱
        let persona_display_name = if let Some(res) = send_to_daemon(RequestType::GetStatus, "", "", false, None) {
             res.persona_name
        } else {
            "凜".to_string()
        };

        fastfetch::print_fetch(&persona_display_name);
        print!("  {} ", format!("{} is preparing...", persona_display_name).yellow().italic());
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        
        if let Some(res) = send_to_daemon(RequestType::FetchGreeting, "fetch", "", false, cli.force_holiday) {
            print!("\r\x1B[K"); 
            print_bubble(&res.persona_name, &res.message);
        } else {
            println!("\r\x1B[K{}", format!("  ({}似乎還在睡覺...)", persona_display_name).black().bold());
        }
        return;
    }

    if cli.hook {
        if !cli.args.is_empty() {
            let cmd = &cli.args[0];
            let output = if cli.args.len() >= 2 { &cli.args[1] } else { "" };
            if let Some(res) = send_to_daemon(RequestType::Command, cmd, output, false, None) {
                print_bubble(&res.persona_name, &res.message);
            }
        }
        return;
    }

    if let Some(cmd) = cli.test {
        if let Some(res) = send_to_daemon(RequestType::Command, &cmd, "Test output", cli.force_env, cli.force_holiday) {
            print_bubble(&res.persona_name, &res.message);
        } else {
            println!("{}", "錯誤：無法連接到 anitermd 守護進程。請先啟動它！".red());
        }
        return;
    }

    println!("歡迎使用 AniTerm！輸入 'aniterm --help' 查看更多選項。");
}
