pub trait True {}
pub trait False {}

pub struct If<const V: bool>;
impl True for If<true> {}
impl False for If<false> {}
