pub enum GameItemKind {
    WoodenLog,
}

pub struct GameItemPile {
    pub kind: GameItemKind,
    pub amount: u32,
}
