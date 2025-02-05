pub mod openai;


pub enum Role {
    System,
    User,
}

pub struct Message {
    pub role: Role,
    pub content: String,
}