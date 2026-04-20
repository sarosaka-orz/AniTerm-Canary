use colored::*;
use sysinfo::System;
use chrono::Local;
use unicode_width::UnicodeWidthStr;

pub fn print_fetch(persona_name: &str) {
    let mut sys = System::new_all();
    sys.refresh_all();

    // 獲取系統資訊
    let os = System::name().unwrap_or_else(|| "Unknown".to_string());
    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let uptime = System::uptime(); // 秒
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    
    let cpu_name = sys.global_cpu_info().brand().trim().to_string();
    let total_mem = sys.total_memory() / 1024 / 1024; // MB
    let used_mem = sys.used_memory() / 1024 / 1024; // MB
    
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 獲取更多系統資訊
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_swap = sys.total_swap() / 1024 / 1024;
    let used_swap = sys.used_swap() / 1024 / 1024;
    let hostname = System::host_name().unwrap_or_else(|| "localhost".to_string());

    // 二次元化的標籤 (加入日文元素)
    let labels = [
        ("ユーザー (User)", whoami::username()),
        ("ホスト (Host)", hostname),
        ("世界線 (Worldline)", os),
        ("核 (Kernel)", kernel),
        ("生存時間 (Uptime)", format!("{}d {}h {}m", days, hours, minutes)),
        ("魔力源 (CPU)", format!("{}({:.1}%)", cpu_name, cpu_usage)),
        ("記憶の池 (Memory)", format!("{} / {} MB ({:.1}%)", used_mem, total_mem, (used_mem as f64 / total_mem as f64) * 100.0)),
        ("交換領域 (Swap)", format!("{} / {} MB", used_swap, total_swap)),
        ("現在の刻 (Time)", time),
    ];

    // 穩定的 ASCII Art (避免顏文字導致壓縮問題)
    let persona_header = format!("       [ {} SYSTEM ]        ", persona_name.to_uppercase());
    let ascii = vec![
        r#"      |\      _,,,---,,_     "#,
        r#"      /,`.-'`'    -.  ;-;;,_ "#,
        r#"     |,4-  ) )-,_. ,\ (  `'-."#,
        r#"    '---''(_/--'  `-'\_)     "#,
        r#"                             "#,
        &persona_header,
        r#"                             "#,
    ];

    let phrases = [
        "今日も一日、がんばるぞい！",
        "今日はいい天気ですね、マスター。",
        "主人、お帰りなさいませ！",
        "適度に休憩も必要ですよ、マスター。",
    ];
    let phrase = phrases[Local::now().timestamp() as usize % phrases.len()];

    println!("\n  {}", format!("--- 終端の守護者：{} 啟動中 ---", persona_name).magenta().bold());
    
    for (i, line) in ascii.iter().enumerate() {
        let info = if i < labels.len() {
            let (key, val) = &labels[i];
            let key_display = format!("{}：", key);
            let width = key_display.width();
            let padding = " ".repeat(28usize.saturating_sub(width));
            format!("{}{}{}", key.cyan().bold(), padding, val.white())
        } else {
            "".to_string()
        };
        
        let visual_width = line.width();
        let padding_size = 32usize.saturating_sub(visual_width);
        let padded_line = format!("{}{}", line, " ".repeat(padding_size));
        
        println!("  {}    {}", padded_line.magenta(), info);
    }
    
    println!("\n  {}\n", phrase.yellow().italic());
}

// 簡單的 whoami 替代方案，因為我們不想引入太多依賴
mod whoami {
    pub fn username() -> String {
        std::env::var("USER").unwrap_or_else(|_| "Adventurer".to_string())
    }
}

