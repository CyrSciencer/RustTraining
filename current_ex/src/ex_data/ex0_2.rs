use crate::ex_data::ex0_0a0_1::Quest;

pub struct QuestLog {
    list: Vec<Quest>,
}

impl QuestLog {
    pub fn new() -> Self {
        return QuestLog { list: Vec::new()}
    }
	pub fn add_quest(&mut self, added_quest:Quest){
		self.list.push(added_quest);
	}
	pub fn self_iter(&self) -> std::slice::Iter<'_, Quest> {
        return self.list.iter();
    }
	pub fn print_all(&self){
		for quest in &self.list{
			let mut status: String = String::from("Pending");
			if let true = quest.is_done(){
				status = String::from("Complete");
			}
			println!("[ID: {}] {} (Status: {})",quest.id(), quest.title(), status);
		}
	}
	pub fn print_completed(&self){
		for quest in &self.list{
			let mut status: String = String::from("Pending");
			if let true = quest.is_done(){
				status = String::from("Complete");
				println!("[ID: {}] {} (Status: {})",quest.id(), quest.title(), status);
			}
		}
	}
}

pub fn ex0_3Func0(){
	let my_quest1: Quest = Quest::new(0,"mushroom gather","Gather 15 mushroom in the forest");
    let my_quest2: Quest = Quest::new(1,"Hog tamer","tame 15 hogs in the forest");
    let mut log: QuestLog = QuestLog::new();
    log.add_quest(my_quest1);
    log.add_quest(my_quest2);
    log.print_all();
}

pub fn ex0_3Func1(){
	let mut my_quest1: Quest = Quest::new(0,"mushroom gather","Gather 15 mushroom in the forest");
    let my_quest2: Quest = Quest::new(1,"Hog tamer","tame 15 hogs in the forest");
	my_quest1.set_is_done(true);
    let mut log: QuestLog = QuestLog::new();
    log.add_quest(my_quest1);
    log.add_quest(my_quest2);
    log.print_completed();
}
