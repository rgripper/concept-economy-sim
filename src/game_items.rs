#[derive(Debug)]
pub enum GameItemKind {
    WoodenLog,
}

#[derive(Debug)]
pub struct GameItemPile {
    pub kind: GameItemKind,
    pub amount: u32,
}
