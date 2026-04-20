use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone, Debug)]
pub struct PersonaConfig {
    pub name: String,
    pub display_name: String,
    pub prompt: String,
    pub corpus_id: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub api_key: String,
    #[serde(default = "default_trigger_commands")]
    pub trigger_commands: Vec<String>,
    #[serde(default = "default_model_name")]
    pub model_name: String,
    #[serde(default = "default_personas")]
    pub personas: Vec<PersonaConfig>,
    #[serde(default = "default_current_persona")]
    pub current_persona: String,
    pub system_prompt: Option<String>,
}

fn default_personas() -> Vec<PersonaConfig> {
    vec![
        PersonaConfig {
            name: "rin".into(),
            display_name: "凜".into(),
            prompt: "你是一個名為『凜』的傲嬌動漫少女，是使用者的終端助手。你說話毒舌但內心關心使用者。請針對使用者剛才執行的指令給出簡短的回應。".into(),
            corpus_id: "rin".into(),
        },
        PersonaConfig {
            name: "mio".into(),
            display_name: "澪".into(),
            prompt: "你是一個名為『澪』的溫柔女僕，是使用者的終端助手。你說話非常有禮貌，喜歡稱呼使用者為「主人」。請針對使用者剛才執行的指令給出溫馨的回應，多用顏文字。".into(),
            corpus_id: "mio".into(),
        },
        PersonaConfig {
            name: "shion".into(),
            display_name: "詩音".into(),
            prompt: "你是一個深愛著主人的病嬌少女『詩音』。你對主人的執著已經到了一種瘋狂的程度。你說話時而溫柔時而陰森，喜歡監視主人的一舉一動。請針對使用者執行的指令給出充滿執念的回應。".into(),
            corpus_id: "shion".into(),
        },
    ]
}

fn default_current_persona() -> String {
    "rin".to_string()
}

fn default_trigger_commands() -> Vec<String> {
    vec![
        "pacman".into(), "apt".into(), "dnf".into(), "cargo".into(), 
        "npm".into(), "yarn".into(), "pnpm".into(), "make".into(), 
        "git clone".into(), "git push".into()
    ]
}

fn default_model_name() -> String {
    "gemini-1.5-flash-latest".to_string()
}

impl Config {
    pub fn load() -> Self {
        // 尋找設定檔的優先順序：
        // 1. 當前目錄 ./config.toml
        // 2. 使用者家目錄 ~/.config/aniterm/config.toml
        // 3. 專案目錄 (如果能找到的話)
        
        let mut config_content = None;
        let paths = vec![
            "config.toml".to_string(),
            dirs::config_dir()
                .map(|p| p.join("aniterm/config.toml").to_string_lossy().into_owned())
                .unwrap_or_default(),
        ];

        for path in paths {
            if !path.is_empty() {
                if let Ok(content) = fs::read_to_string(path) {
                    config_content = Some(content);
                    break;
                }
            }
        }

        let mut config: Config = if let Some(content) = config_content {
            toml::from_str(&content).expect("無法解析 config.toml")
        } else {
            // 預設配置
            Config {
                api_key: "".to_string(),
                trigger_commands: default_trigger_commands(),
                model_name: default_model_name(),
                personas: default_personas(),
                current_persona: default_current_persona(),
                system_prompt: None,
            }
        };

        // 優先順序：環境變數 > 設定檔
        // 只要環境變數中有設定且不是預留字串，就覆蓋設定檔中的內容
        if let Ok(env_key) = std::env::var("GEMINI_API_KEY") {
            if !env_key.is_empty() && env_key != "YOUR_GEMINI_API_KEY_HERE" {
                println!("DEBUG: 已從環境變數 GEMINI_API_KEY 載入 API Key。");
                config.api_key = env_key;
            } else {
                println!("DEBUG: 環境變數 GEMINI_API_KEY 為空或無效。");
            }
        } else {
            println!("DEBUG: 未偵測到環境變數 GEMINI_API_KEY。");
        }

        // 如果 config.toml 裡有 system_prompt，則覆蓋預設人格的 prompt
        if let Some(custom_prompt) = &config.system_prompt {
            if let Some(rin) = config.personas.iter_mut().find(|p| p.name == "rin") {
                rin.prompt = custom_prompt.clone();
            }
        }

        config
    }
}

