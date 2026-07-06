use serde_derive;
use serde_json;
use serde;
// use std::error::Error;

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
struct Quest {
    id: u32,
    title: String,
    desc: String,
    is_done: bool,
}

impl Quest {
    pub fn new(id: u32, title: &str, desc: &str) -> Self {
        return Quest {
            id,
            title: String::from(title),
            desc: String::from(desc),
            is_done: false,
        };
    }
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug, Default)]
pub struct QuestLog {
    list: Vec<Quest>,
}

impl QuestLog {
    pub fn new() -> Self {
        QuestLog { list: Vec::new()}
    }
	pub fn add_quest(&mut self, added_quest:Quest){
		println!("Added quest node: {}", added_quest.title);
		self.list.push(added_quest);
	}
	pub fn self_iter(&self) -> std::slice::Iter<'_, Quest> {
        self.list.iter()
    }
}
fn save_to_json<T: serde::Serialize>(path: &str, data: &T) -> std::io::Result<()> {
    let json_data = serde_json::to_string_pretty(data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
	std::fs::write(path, json_data)
}

fn load_or_default<T: serde::de::DeserializeOwned + Default>(path: &str) -> T {
    let content = match std::fs::read_to_string(path) {
        Ok(data) => data,
        Err(_) => return T::default(),
    };
    match serde_json::from_str(&content) {
        Ok(data) => data,
        Err(_) => T::default(),
    }
}
fn main5() {
	let the_quest = Quest::new(1, "first", "testing");
	let the_quest_bis = Quest::new(9, "test", "testing 3");
    let the_quest2 = Quest::new(2, "sec", "testing 2");
	let mut log = QuestLog::new();
	let mut log2 = QuestLog::new();

	log.add_quest(the_quest);
	log.add_quest(the_quest2);
	let _ = save_to_json("quests.json", &log);
    println!("Hello, world!");
	log2 = load_or_default("quest.json");
	println!("{:?}", log2);
	log2 = load_or_default("quests.json");
	print!("{:?}", log2);
}
