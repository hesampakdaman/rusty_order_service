use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineItem {
    pub id: Uuid,
    pub quantity: usize,
}

impl LineItem {
    pub fn new(id: Uuid, quantity: usize) -> Self  {
        LineItem { id, quantity }
    }
}
