use crate::entity::Entity;

#[derive(PartialEq,Clone,Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub width:  i32,
    pub height: i32,
}

pub struct Board {
    pub size:       Size,
    pub scale:      i32,
    pub entity_map: Vec<Entity>,
}

pub trait Movable {
    fn move_dir(
        &mut self,
        steps:      i32,
        direction:  Direction,
        board:      &Board
    ) -> ();
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
