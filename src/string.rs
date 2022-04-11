use std::{
    fmt::Debug,
    io::Read,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::Decode;

pub struct Str<T, const NULL_TERMINATED: bool = false>(pub(crate) String, PhantomData<T>);

impl<T, const NULL_TERMINATED: bool> Deref for Str<T, NULL_TERMINATED> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const NULL_TERMINATED: bool> DerefMut for Str<T, NULL_TERMINATED> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Decode, const NULL_TERMINATED: bool> Decode for Str<T, NULL_TERMINATED>
where
    <T as Decode>::Output: Default + Debug + PartialEq + TryInto<u8>,
    <<T as Decode>::Output as TryInto<u8>>::Error: Debug,
{
    type Output = String;

    fn decode(reader: &mut impl Read, _state: ()) -> eyre::Result<Self::Output> {
        let mut length = i32::decode(reader, ())?;

        assert!(length >= 0);

        if length > 0 {
            if NULL_TERMINATED {
                length -= 1;
            }

            let mut characters = Vec::with_capacity(length as usize);

            for _ in 0..length {
                characters.push(T::decode(reader, ())?.try_into().unwrap() as char);
            }

            if NULL_TERMINATED {
                // Null terminator
                assert_eq!(T::Output::default(), T::decode(reader, ())?);
            }

            Ok(characters.into_iter().collect::<String>())
        } else {
            Ok(String::new())
        }
    }
}
