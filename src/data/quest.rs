use table;

#[derive(Debug)]
enum QuestSignificant {
    Normal,
    Key,
    Urgent,
}

#[derive(Debug)]
enum QuestCategory {
    Village(i32),
    Guild(i32),
    Special(i32),
}

#[derive(Debug)]
enum QuestType {
    Gathering, // 0, Green
    Slaying, // 1, Orange
    Hunting, // 2, Red
    Capture, // 3, White
    Special, // 4, Purple
    Endurance, // 5, Blue
    Monathon, // 6, Blue
}

#[derive(Debug)]
pub struct Quest {
    id: i32,
    dex_id: i32,
    name: String,
    significant: QuestSignificant,
    category: QuestCategory,
    quest_type: QuestType,
    location: table::Ref,
    fee: i32,
    time: i32,
}

impl Quest {
    pub fn new(name: String) -> Quest {
        Quest {
            id: 0,
            dex_id: 0,
            name: name,
            significant: QuestSignificant::Normal,
            category: QuestCategory::Village(3),
            quest_type: QuestType::Hunting,
            location: table::Ref::Id(12),
            fee: 10,
            time: 50,
        }
    }
}
