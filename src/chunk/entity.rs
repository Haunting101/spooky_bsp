use derive_new::new;
use std::io::Read;

use crate::{Decode, Matrix, Str};

#[derive(new, Clone, Debug)]
pub struct Entity {
    pub entity_type: u32,
    pub matrix: Matrix,
    pub action_points_count: i32,
    pub name: String,
}

impl Decode for Entity {
    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self> {
        let entity_type = u32::decode(reader, ())?;
        let matrix = Matrix::decode(reader, ())?;
        let action_points_count = i32::decode(reader, ())?;
        let category_name = Str::<u8>::decode(reader, ())?;

        assert!(category_name.len() <= 50);

        Ok(Self::new(
            entity_type,
            matrix,
            action_points_count,
            category_name,
        ))
    }
}
