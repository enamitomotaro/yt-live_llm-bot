#[derive(Clone)]
pub enum Role {
    User,
    Bot,
}

#[derive(Clone)]
pub struct Message {
    pub role: Role,
    pub text: String,
}
