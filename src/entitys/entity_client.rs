use uuid::Uuid;

#[derive(Debug)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub balance: f32,
}


