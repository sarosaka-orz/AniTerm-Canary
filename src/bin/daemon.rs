use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use std::path::Path;
use aniterm::ipc::{IpcRequest, IpcResponse, RequestType};
use aniterm::corpus::CorpusEngine;
use aniterm::config::Config;
use aniterm::environment::{self, TimePeriod};
use serde_json::json;
use rand::Rng;

async fn call_gemini(api_key: &str, model_name: &str, system_prompt: &str, user_input: &str, system_info: Option<&str>) -> Option<String> {
    if api_key.is_empty() || api_key == "YOUR_GEMINI_API_KEY_HERE" {
        eprintln!("AI 失敗：未設定有效的 API Key。");
        return None;
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;

    let model = if model_name.is_empty() { "gemini-1.5-flash-latest" } else { model_name };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model,
        api_key
    );

    let env_context = system_info.unwrap_or("無環境資訊");

    let payload = json!({
        "contents": [{
            "parts": [{
                "text": format!(
                    "{}\n\n【目前環境】\n{}\n\n【重要指令】\n1. 絕對不要重複使用者的指令內容。\n2. 針對使用者執行的指令「{}」給出一段傲嬌、毒舌但帶有關心的吐槽。\n3. 如果環境資訊顯示系統負載過高或時間太晚，請加入相關的吐槽。\n4. 如果今天是節日（環境資訊中的「節日」不為「無」），請在吐槽中「不經意地」流露出一點點真誠的節日祝福或關心，但要表現得像是順便說的，隨後馬上恢復傲嬌態度。\n5. 回應必須是純文字，長度在 40 字以內。\n回應：", 
                    system_prompt, 
                    env_context,
                    user_input
                )
            }]
        }],
        "generationConfig": {
            "maxOutputTokens": 1024,
            "temperature": 0.9
        }
    });

    let res = match client.post(url).json(&payload).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("AI 失敗：網路請求錯誤或超時 - {}", e);
            return None;
        }
    };

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        eprintln!("AI 失敗：API 回傳錯誤狀態 {} - {}", status, body);
        return None;
    }

    let json: serde_json::Value = match res.json().await {
        Ok(j) => j,
        Err(e) => {
            eprintln!("AI 失敗：無法解析 JSON - {}", e);
            return None;
        }
    };
    
    json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(|s| s.trim().to_string())
}

async fn call_gemini_chat(api_key: &str, model_name: &str, system_prompt: &str, user_input: &str, system_info: &str) -> Option<String> {
    if api_key.is_empty() || api_key == "YOUR_GEMINI_API_KEY_HERE" {
        return None;
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .ok()?;

    let model = if model_name.is_empty() { "gemini-1.5-flash-latest" } else { model_name };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model,
        api_key
    );

    let payload = json!({
        "contents": [{
            "parts": [{
                "text": format!(
                    "{}\n\n【目前環境】\n{}\n\n【重要指令】\n1. 使用者正在與你直接對話。\n2. 請根據你的性格設定給出回應。\n3. 如果環境資訊顯示特殊情況（如深夜、高負載），可以在對話中提及。\n4. 回應必須是純文字，長度建議在 100 字以內，不要太過冗長。\n使用者說：{}", 
                    system_prompt, 
                    system_info,
                    user_input
                )
            }]
        }],
        "generationConfig": {
            "maxOutputTokens": 1024,
            "temperature": 0.8
        }
    });

    let res = match client.post(url).json(&payload).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("AI 失敗：網路請求錯誤或超時 - {}", e);
            return None;
        }
    };

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        eprintln!("AI 失敗：API 回傳錯誤狀態 {} - {}", status, body);
        return None;
    }

    let json: serde_json::Value = match res.json().await {
        Ok(j) => j,
        Err(e) => {
            eprintln!("AI 失敗：無法解析 JSON - {}", e);
            return None;
        }
    };
    
    json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(|s| s.trim().to_string())
}

async fn call_gemini_greeting(api_key: &str, model_name: &str, system_prompt: &str, system_info: &str) -> Option<String> {
    if api_key.is_empty() || api_key == "YOUR_GEMINI_API_KEY_HERE" {
        return None;
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .ok()?;

    let model = if model_name.is_empty() { "gemini-1.5-flash-latest" } else { model_name };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model,
        api_key
    );

    let payload = json!({
        "contents": [{
            "parts": [{
                "text": format!(
                    "{}\n\n【目前環境】\n{}\n\n【重要指令】\n1. 使用者剛啟動了系統資訊查看 (fetch)。\n2. 請根據目前的時段、天氣、節日，給出一句向用戶打招呼或是加油鼓氣的話。\n3. 保持傲嬌性格。如果今天是節日，請在打招呼時稍微流露出真實的祝福（真心話），但要表現得很彆扭，像是被強迫說的一樣。\n4. 回應必須是純文字，長度在 50 字以內。\n回應：", 
                    system_prompt, 
                    system_info
                )
            }]
        }],
        "generationConfig": {
            "maxOutputTokens": 1024,
            "temperature": 0.9
        }
    });

    let res = match client.post(url).json(&payload).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Greeting AI 失敗：{}", e);
            return None;
        }
    };

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        eprintln!("Greeting AI 失敗：{} - {}", status, body);
        return None;
    }

    let json: serde_json::Value = res.json().await.ok()?;
    json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(|s| s.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = "/tmp/aniterm.sock";
    let corpus_engine = std::sync::Arc::new(CorpusEngine::new());
    let config = std::sync::Arc::new(Config::load());

    if Path::new(socket_path).exists() {
        fs::remove_file(socket_path)?;
    }

    println!("--- AniTerm 守護進程 (Daemon) 啟動中 ---");
    println!("正在獲取環境資訊 (天氣、節日)...");
    let static_env = std::sync::Arc::new(environment::fetch_static_env().await);
    println!("環境資訊獲取完成：{}", static_env.weather);
    if let Some(h) = &static_env.holiday {
        println!("今日節日：{}", h);
    }

    println!("正在監聽 IPC 通訊：{}", socket_path);
    if config.api_key.is_empty() || config.api_key == "YOUR_GEMINI_API_KEY_HERE" {
        println!("提示：未偵測到有效的 API Key，將僅使用本地語料庫。");
    } else {
        println!("Gemini AI 已就緒 (模型: {})。", config.model_name);
    }

    let active_persona_name = std::sync::Arc::new(tokio::sync::RwLock::new(config.current_persona.clone()));

    let listener = UnixListener::bind(socket_path)?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let engine = corpus_engine.clone();
        let cfg = config.clone();
        let s_env = static_env.clone();
        let active_persona = active_persona_name.clone();
        
        tokio::spawn(async move {
            let mut buffer = vec![0; 16384]; 
            let n = match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => n,
                _ => return,
            };

            let req_str = String::from_utf8_lossy(&buffer[..n]);
            let req: IpcRequest = match serde_json::from_str(&req_str) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("IPC 解析錯誤: {} | 原始資料: {}", e, req_str);
                    return;
                }
            };

            match req.request_type {
                RequestType::SwitchPersona => {
                    let persona_name = req.command.trim().to_lowercase();
                    if cfg.personas.iter().any(|p| p.name == persona_name) {
                        let mut active = active_persona.write().await;
                        *active = persona_name.clone();
                        
                        let response = IpcResponse {
                            message: format!("切換至人格：{}", persona_name),
                            persona_name: "系統".to_string(),
                        };
                        let res_str = serde_json::to_string(&response).unwrap();
                        let _ = socket.write_all(res_str.as_bytes()).await;
                    } else {
                        let response = IpcResponse {
                            message: format!("錯誤：找不到人格 「{}」", persona_name),
                            persona_name: "系統".to_string(),
                        };
                        let res_str = serde_json::to_string(&response).unwrap();
                        let _ = socket.write_all(res_str.as_bytes()).await;
                    }
                },
                RequestType::GetStatus => {
                    let persona_name = active_persona.read().await;
                    let persona = cfg.personas.iter().find(|p| p.name == *persona_name).unwrap();
                    
                    let response = IpcResponse {
                        message: "OK".to_string(),
                        persona_name: persona.display_name.clone(),
                    };
                    let res_str = serde_json::to_string(&response).unwrap();
                    let _ = socket.write_all(res_str.as_bytes()).await;
                },
                RequestType::Chat => {
                    let persona_name = active_persona.read().await;
                    let persona = cfg.personas.iter().find(|p| p.name == *persona_name).unwrap();
                    
                    let mut env_info = req.system_info.unwrap_or_else(|| "無環境資訊".to_string());
                    let holiday = req.force_holiday.as_deref().or(s_env.holiday.as_deref()).unwrap_or("無");
                    env_info = format!("{}, 天氣={}, 節日={}", env_info, s_env.weather, holiday);

                    let chat_msg = call_gemini_chat(&cfg.api_key, &cfg.model_name, &persona.prompt, &req.command, &env_info).await
                        .unwrap_or_else(|| "（似乎在發呆，沒有回應...）".to_string());

                    let response = IpcResponse {
                        message: chat_msg,
                        persona_name: persona.display_name.clone(),
                    };
                    let res_str = serde_json::to_string(&response).unwrap();
                    let _ = socket.write_all(res_str.as_bytes()).await;
                },
                RequestType::FetchGreeting => {
                    let persona_name = active_persona.read().await;
                    let persona = cfg.personas.iter().find(|p| p.name == *persona_name).unwrap();
                    
                    let mut env_info = req.system_info.unwrap_or_else(|| "無環境資訊".to_string());
                    
                    let holiday = req.force_holiday.as_deref().or(s_env.holiday.as_deref()).unwrap_or("無");
                    env_info = format!("{}, 天氣={}, 節日={}", env_info, s_env.weather, holiday);
                    
                    let greeting = call_gemini_greeting(&cfg.api_key, &cfg.model_name, &persona.prompt, &env_info).await
                        .unwrap_or_else(|| engine.get_greeting(&persona.corpus_id, "fetch_fallback"));
                    
                    let response = IpcResponse {
                        message: greeting,
                        persona_name: persona.display_name.clone(),
                    };
                    let res_str = serde_json::to_string(&response).unwrap();
                    let _ = socket.write_all(res_str.as_bytes()).await;
                },
                RequestType::Command => {
                    let cmd_trimmed = req.command.trim();
                    if cmd_trimmed.is_empty() || cmd_trimmed.starts_with("aniterm") {
                        let response = IpcResponse {
                            message: "".to_string(),
                            persona_name: "".to_string(),
                        };
                        let res_str = serde_json::to_string(&response).unwrap();
                        let _ = socket.write_all(res_str.as_bytes()).await;
                        return;
                    }

                    let persona_name = active_persona.read().await;
                    let persona = cfg.personas.iter().find(|p| p.name == *persona_name).unwrap();
                    
                    println!("收到指令：{}", req.command);
                    
                    let mut response_msg = None;
                    
                    let (trigger_env, trigger_weather) = {
                        let mut rng = rand::thread_rng();
                        (req.force_env || rng.gen_bool(0.1), rng.gen_bool(0.3))
                    };
                    
                    if trigger_env {
                        let snapshot = environment::collect_snapshot();
                        
                        if let Some(music) = &snapshot.music {
                            if environment::is_kawaii_bass(music) {
                                response_msg = Some(engine.get_music_response(&persona.corpus_id, "kawaii_bass", music));
                            }
                        }
                        
                        if response_msg.is_none() {
                            if snapshot.cpu_usage > 80.0 {
                                response_msg = Some(engine.get_system_response(&persona.corpus_id, "high_cpu"));
                            } else if snapshot.mem_usage_percent > 90.0 {
                                response_msg = Some(engine.get_system_response(&persona.corpus_id, "high_mem"));
                            }
                        }

                        if response_msg.is_none() {
                            let period_key = match snapshot.time_period {
                                TimePeriod::Morning => "morning",
                                TimePeriod::Noon => "noon",
                                TimePeriod::Afternoon => "afternoon",
                                TimePeriod::Evening => "evening",
                                TimePeriod::LateNight => "late_night",
                            };
                            response_msg = Some(engine.get_time_response(&persona.corpus_id, period_key));
                        }
                        
                        if trigger_weather {
                            if s_env.weather.contains("Rain") || s_env.weather.contains("雨") {
                                response_msg = Some(engine.get_weather_response(&persona.corpus_id, "rain"));
                            }
                        }
                    }

                    if response_msg.is_none() {
                        let should_trigger_ai = cfg.trigger_commands.iter().any(|cmd| req.command.contains(cmd));
                        if should_trigger_ai {
                            println!("觸發 AI 回應...");
                            let mut env_info = req.system_info.unwrap_or_else(|| "無環境資訊".to_string());
                            let holiday = req.force_holiday.as_deref().or(s_env.holiday.as_deref()).unwrap_or("無");
                            env_info = format!("{}, 天氣={}, 節日={}", env_info, s_env.weather, holiday);
                            
                            response_msg = call_gemini(&cfg.api_key, &cfg.model_name, &persona.prompt, &req.command, Some(&env_info)).await;
                        }
                    }

                    let final_msg = response_msg.unwrap_or_else(|| {
                        engine.get_response(&persona.corpus_id, &req.command)
                    });

                    let response = IpcResponse {
                        message: final_msg,
                        persona_name: persona.display_name.clone(),
                    };
                    
                    let res_str = serde_json::to_string(&response).unwrap();
                    let _ = socket.write_all(res_str.as_bytes()).await;
                }
            }
        });
    }
}
