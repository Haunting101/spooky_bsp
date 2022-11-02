use crate::{Decode, Matrix};

#[derive(Clone, Debug, Decode)]
pub struct Entity {
    pub entity_type: u32,
    pub matrix: Matrix,
    pub action_points_count: i32,
    pub name: String,
}
