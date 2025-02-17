use uuid::Uuid;

#[derive(Debug)]
pub struct Item {
    pub id: Uuid,
    pub quantity: usize,
}
