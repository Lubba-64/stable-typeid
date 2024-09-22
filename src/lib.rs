#![feature(const_type_name)]
use const_fnv1a_hash::fnv1a_hash_64;
use impl_stable_id_macro::*;
pub use stable_typeid_macro::*;
use std::{any::type_name, fmt::Display};

pub trait StableAny: 'static {
    fn stable_id(&self) -> &'static StableId;
}

pub trait StableAnyTrait {
    fn is<T>(&self) -> bool
    where
        T: StableID;
    fn downcast_ref_unchecked<N>(&self) -> &N;
    fn downcast_ref<N>(&self) -> Option<&N>
    where
        N: StableID;
    fn downcast_mut_unchecked<N>(&mut self) -> &mut N;
    fn downcast_mut<N>(&mut self) -> Option<&mut N>
    where
        N: StableID;
}

impl StableAnyTrait for dyn StableAny {
    fn is<T>(&self) -> bool
    where
        T: StableID,
    {
        T::_STABLE_ID == self.stable_id()
    }
    fn downcast_ref_unchecked<N>(&self) -> &N {
        unsafe { &*(self as *const Self as *const N) }
    }
    fn downcast_ref<N>(&self) -> Option<&N>
    where
        N: StableID,
    {
        if self.is::<N>() {
            Some(self.downcast_ref_unchecked())
        } else {
            None
        }
    }
    fn downcast_mut_unchecked<N>(&mut self) -> &mut N {
        unsafe { &mut *(self as *mut Self as *mut N) }
    }
    fn downcast_mut<N>(&mut self) -> Option<&mut N>
    where
        N: StableID,
    {
        if self.is::<N>() {
            Some(self.downcast_mut_unchecked())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Hash, Eq)]
pub struct StableId(pub u64);
impl PartialEq for StableId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
pub trait StableID {
    const _STABLE_ID: &'static StableId;
}
impl Display for StableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ID({})", self.0))
    }
}

pub fn get_pkg_name() -> String {
    let env = std::env::var("CARGO_PKG_NAME");
    env.unwrap_or("".to_string())
}

macro_rules! impl_with_type_name {
    ($name:ident) => {
        impl StableAny for $name {
            fn stable_id(&self) -> &'static StableId
            where
                Self: Sized,
            {
                $name::_STABLE_ID
            }
        }

        impl StableID for $name {
            const _STABLE_ID: &'static StableId =
                &StableId(fnv1a_hash_64(type_name::<$name>().as_bytes(), None));
        }
    };
}

macro_rules! impl_with_type_name_tuple {
    (()) => {
        impl StableID for () {
            const _STABLE_ID: &'static StableId =
                &StableId(fnv1a_hash_64(type_name::<()>().as_bytes(), None));
        }
    };

    ($param:ident) => {
        impl <$param: StableID> StableID for ($param,) {
            const _STABLE_ID: &'static StableId =
                &StableId(fnv1a_hash_64(type_name::<($param,)>().as_bytes(), None));
        }
    };

    ($last:ident $(,$param:ident)*) => {
        impl <$($param: StableID,)* $last: StableID> StableID for ($($param,)* $last) {
            const _STABLE_ID: &'static StableId =
                &StableId(fnv1a_hash_64(type_name::<($($param,)*)>().as_bytes(), None));
        }
    };
}

impl_with_type_name!(bool);
impl_with_type_name!(char);
impl_with_type_name!(u8);
impl_with_type_name!(u16);
impl_with_type_name!(u32);
impl_with_type_name!(u64);
impl_with_type_name!(u128);
impl_with_type_name!(usize);
impl_with_type_name!(i8);
impl_with_type_name!(i16);
impl_with_type_name!(i32);
impl_with_type_name!(i64);
impl_with_type_name!(i128);
impl_with_type_name!(isize);
impl_with_type_name!(f32);
impl_with_type_name!(f64);
impl_with_type_name!(String);
impl_with_type_name_tuple!(());
impl_with_type_name_tuple!(T);
impl_with_type_name_tuple!(T, T1);
impl_with_type_name_tuple!(T, T1, T2);
impl_with_type_name_tuple!(T, T1, T2, T3);
impl_with_type_name_tuple!(T, T1, T2, T3, T4);
impl_with_type_name_tuple!(T, T1, T2, T3, T4, T5);
impl_with_type_name_tuple!(T, T1, T2, T3, T4, T5, T6);
impl_with_type_name_tuple!(T, T1, T2, T3, T4, T5, T6, T7);
impl_with_type_name_tuple!(T, T1, T2, T3, T4, T5, T6, T7, T8);
impl_with_type_name_tuple!(T, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_with_type_name_tuple!(T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_with_type_name_tuple!(T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
stable_id_impl!(std::collections::HashMap<K, V>);
stable_id_impl!(std::vec::Vec<T>);
stable_id_impl!(Option<T>);

impl<T, const N: usize> StableID for [T; N] {
    const _STABLE_ID: &'static StableId =
        &StableId(fnv1a_hash_64(type_name::<T>().as_bytes(), None) ^ N as u64);
}

#[cfg(test)]
mod tests {
    use crate as stable_typeid;
    use stable_typeid::*;
    #[test]
    fn main_test() {
        let any = MyStruct {
            anything: "Hello TypeId".to_string(),
        };

        foo(&any);
    }
    fn foo(any: &dyn StableAny) {
        if let Some(my_struct) = any.downcast_ref::<MyStruct>() {
            println!("{} {}", my_struct.anything, MyStruct::_STABLE_ID);
        }
    }
    #[derive(StableID)]
    struct MyStruct {
        anything: String,
    }
}
