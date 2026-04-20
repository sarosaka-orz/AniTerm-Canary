use sysinfo::System;
use chrono::{Local, Datelike, Timelike};
use std::process::Command;

pub struct EnvironmentSnapshot {
    pub cpu_usage: f32,
    pub mem_usage_percent: f64,
    pub current_time: String,
    pub hostname: String,
    pub load_avg: f64,
    pub time_period: TimePeriod,
    pub music: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum TimePeriod {
    Morning,   // 06:00 - 11:59
    Noon,      // 12:00 - 13:59
    Afternoon, // 14:00 - 17:59
    Evening,   // 18:00 - 23:59
    LateNight, // 00:00 - 05:59
}

impl TimePeriod {
    pub fn get_current() -> Self {
        let hour = Local::now().hour();
        match hour {
            6..=11 => TimePeriod::Morning,
            12..=13 => TimePeriod::Noon,
            14..=17 => TimePeriod::Afternoon,
            18..=23 => TimePeriod::Evening,
            _ => TimePeriod::LateNight,
        }
    }

    pub fn to_chinese(&self) -> &'static str {
        match self {
            TimePeriod::Morning => "早晨",
            TimePeriod::Noon => "中午",
            TimePeriod::Afternoon => "下午",
            TimePeriod::Evening => "夜晚",
            TimePeriod::LateNight => "深夜",
        }
    }
}

pub struct StaticEnv {
    pub weather: String,
    pub holiday: Option<String>,
}

pub async fn fetch_static_env() -> StaticEnv {
    let weather = fetch_weather().await.unwrap_or_else(|_| "未知天氣".to_string());
    let holiday = fetch_holiday().await.ok();
    StaticEnv { weather, holiday }
}

async fn fetch_weather() -> Result<String, Box<dyn std::error::Error>> {
    // 使用 wttr.in 獲取簡短天氣
    let client = reqwest::Client::new();
    let res = client.get("https://wttr.in?format=3").send().await?.text().await?;
    Ok(res.trim().to_string())
}

async fn fetch_holiday() -> Result<String, Box<dyn std::error::Error>> {
    // 使用 Nager.Date API 獲取今日節日 (假設在台灣/香港/中國區域，這裡預設用 CN)
    let now = Local::now();
    let url = format!("https://date.nager.at/api/v3/PublicHolidays/{}/CN", now.year());
    let res: serde_json::Value = reqwest::get(url).await?.json().await?;
    
    if let Some(holidays) = res.as_array() {
        for h in holidays {
            if let Some(date) = h["date"].as_str() {
                if date == now.format("%Y-%m-%d").to_string() {
                    return Ok(h["localName"].as_str().unwrap_or("節日").to_string());
                }
            }
        }
    }
    Err("今日無節日".into())
}

pub fn get_current_music() -> Option<String> {
    let output = Command::new("playerctl")
        .arg("metadata")
        .arg("--format")
        .arg("{{artist}} - {{title}}")
        .output()
        .ok()?;
    
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !s.is_empty() {
            return Some(s);
        }
    }
    None
}

pub fn is_kawaii_bass(music: &str) -> bool {
    let music_lower = music.to_lowercase();
    let db = vec![
        "snail's house", "ujico", "yunomi", "kotori", "fusq", "moeshop", 
        "dark cat", "kawaii bass", "future core"
    ];
    db.iter().any(|&artist| music_lower.contains(artist))
}

pub fn collect_snapshot() -> EnvironmentSnapshot {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let mem_usage_percent = (used_mem as f64 / total_mem as f64) * 100.0;
    
    let load_avg = System::load_average().one;
    let hostname = System::host_name().unwrap_or_else(|| "localhost".to_string());
    let current_time = Local::now().format("%H:%M:%S").to_string();
    let time_period = TimePeriod::get_current();
    let music = get_current_music();

    EnvironmentSnapshot {
        cpu_usage,
        mem_usage_percent,
        current_time,
        hostname,
        load_avg,
        time_period,
        music,
    }
}

pub fn format_for_ai(snapshot: &EnvironmentSnapshot, static_env: &StaticEnv) -> String {
    let music_info = snapshot.music.as_deref().unwrap_or("無");
    let holiday_info = static_env.holiday.as_deref().unwrap_or("無");
    
    format!(
        "系統狀態：時間={}, 時段={}, 天氣={}, 節日={}, 主機={}, CPU負載={:.1}%, 記憶體使用率={:.1}%, 平均負載={:.2}, 正在播放={}",
        snapshot.current_time,
        snapshot.time_period.to_chinese(),
        static_env.weather,
        holiday_info,
        snapshot.hostname,
        snapshot.cpu_usage,
        snapshot.mem_usage_percent,
        snapshot.load_avg,
        music_info
    )
}
