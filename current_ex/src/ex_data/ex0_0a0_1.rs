pub struct Quest {
    id: u32,
    is_done: bool,
    title: String,
    desc: String,
}

impl Quest {
    pub fn new(id: u32, title: &str, desc: &str) -> Self {
        Quest {
            id,
            is_done: false,
            title: String::from(title),
            desc: String::from(desc),
        }
    }
	pub fn id(&self) -> u32{
		self.id
	}
	pub fn set_id(&mut self, new_id:u32){
		self.id = new_id;
	}
	pub fn is_done(&self) -> bool{
		self.is_done
	}
	pub fn set_is_done(&mut self, new_is_done:bool){
		self.is_done = new_is_done;
	}
	pub fn title(&self) -> &str{
		&self.title
	}
	pub fn set_title(&mut self, new_title:String){
		self.title = new_title;
	}
	pub fn desc(&self) -> &str{
		&self.desc
	}
	pub fn set_desc(&mut self, new_desc:String){
		self.desc = new_desc;
	}
}

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

pub fn ex0_func1() {
    let my_quest: Quest = Quest::new(01,"","");
    println!(
        "The quest ID is {}, and status is {} \n\n",
        my_quest.id(), my_quest.is_done()
    );
}

pub fn ex0_func2() {
    let my_quest: Quest = Quest::new(02,"Hog killer","kill 15 hogs in the forest");
    println!(
        "{}: \"{}\"\nis done? {}\nid:{} \n\n",
        my_quest.title(), my_quest.desc(), my_quest.is_done(), my_quest.id()
    );
}

pub fn ex0_func3() {
    let my_quest1: Quest = Quest::new(03,"mushroom gather","Gather 15 mushroom in the forest");
    let my_quest2: Quest = Quest::new(04,"Hog tamer","tame 15 hogs in the forest");
    let mut log: QuestLog = QuestLog::new();
    log.add_quest(my_quest1);
    log.add_quest(my_quest2);
    for quest in log.self_iter() {
        println!(
            "{}: \"{}\"\nis done? {}\nid:{}",
            quest.title(), quest.desc(), quest.is_done(), quest.id()
        );
    }
}
