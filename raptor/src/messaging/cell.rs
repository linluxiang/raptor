pub struct Cell {
    id: String,
}

pub struct Message {
    address: String,
}

pub trait CellBehavior {
    fn on_start(&self);
    fn on_stop(&self);
}
