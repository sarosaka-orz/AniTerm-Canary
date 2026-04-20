use serde::Deserialize;
use std::collections::HashMap;
use rand::seq::SliceRandom;

#[derive(Deserialize, Debug, Clone)]
pub struct PersonaCorpus {
    pub responses: HashMap<String, Vec<String>>,
    pub time: HashMap<String, Vec<String>>,
    pub system: HashMap<String, Vec<String>>,
    pub weather: HashMap<String, Vec<String>>,
    pub music: HashMap<String, Vec<String>>,
    pub greetings: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct CorpusRoot {
    pub personas: HashMap<String, PersonaCorpus>,
}

pub struct CorpusEngine {
    root: CorpusRoot,
}

impl CorpusEngine {
    pub fn new() -> Self {
        let content = include_str!("../resources/corpus.toml");
        let root: CorpusRoot = toml::from_str(content).expect("無法解析 corpus.toml");
        Self { root }
    }

    fn get_persona_corpus(&self, corpus_id: &str) -> &PersonaCorpus {
        self.root.personas.get(corpus_id).expect(&format!("找不到語料庫 ID: {}", corpus_id))
    }

    fn pick_random(&self, list: Option<&Vec<String>>) -> Option<String> {
        let mut rng = rand::thread_rng();
        list.and_then(|l| l.choose(&mut rng).cloned())
    }

    pub fn get_response(&self, corpus_id: &str, command: &str) -> String {
        let mut rng = rand::thread_rng();
        let corpus = self.get_persona_corpus(corpus_id);
        
        for (key, list) in &corpus.responses {
            if key != "default" && command.contains(key) {
                return list.choose(&mut rng).cloned().unwrap_or_default();
            }
        }

        let default_list = corpus.responses.get("default").expect("corpus.toml 缺少 default 欄位");
        let template = default_list.choose(&mut rng).cloned().unwrap_or_default();
        template.replace("{}", command)
    }

    pub fn get_time_response(&self, corpus_id: &str, period: &str) -> String {
        let corpus = self.get_persona_corpus(corpus_id);
        self.pick_random(corpus.time.get(period))
            .unwrap_or_else(|| "時間過得真快呢。".to_string())
    }

    pub fn get_system_response(&self, corpus_id: &str, key: &str) -> String {
        let corpus = self.get_persona_corpus(corpus_id);
        self.pick_random(corpus.system.get(key))
            .unwrap_or_else(|| "系統好像有點累了。".to_string())
    }

    pub fn get_weather_response(&self, corpus_id: &str, key: &str) -> String {
        let corpus = self.get_persona_corpus(corpus_id);
        self.pick_random(corpus.weather.get(key))
            .unwrap_or_else(|| "天氣真不錯呢。".to_string())
    }

    pub fn get_music_response(&self, corpus_id: &str, key: &str, music_name: &str) -> String {
        let corpus = self.get_persona_corpus(corpus_id);
        let template = self.pick_random(corpus.music.get(key))
            .unwrap_or_else(|| "這首歌...還行吧。".to_string());
        template.replace("{}", music_name)
    }

    pub fn get_greeting(&self, corpus_id: &str, key: &str) -> String {
        let corpus = self.get_persona_corpus(corpus_id);
        self.pick_random(corpus.greetings.get(key))
            .unwrap_or_else(|| "你好。".to_string())
    }
}
