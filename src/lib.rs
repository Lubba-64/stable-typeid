use std::fmt::Display;

pub use stable_typeid_macro::*;

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

impl StableID for () {
    const _STABLE_ID: &'static StableId = &StableId(0);
}

impl StableID for bool {
    const _STABLE_ID: &'static StableId = &StableId(1);
}
impl StableID for char {
    const _STABLE_ID: &'static StableId = &StableId(2);
}
impl StableID for u8 {
    const _STABLE_ID: &'static StableId = &StableId(3);
}
impl StableID for u16 {
    const _STABLE_ID: &'static StableId = &StableId(4);
}
impl StableID for u32 {
    const _STABLE_ID: &'static StableId = &StableId(5);
}
impl StableID for u64 {
    const _STABLE_ID: &'static StableId = &StableId(6);
}
impl StableID for u128 {
    const _STABLE_ID: &'static StableId = &StableId(7);
}
impl StableID for usize {
    const _STABLE_ID: &'static StableId = &StableId(8);
}
impl StableID for i8 {
    const _STABLE_ID: &'static StableId = &StableId(9);
}
impl StableID for i16 {
    const _STABLE_ID: &'static StableId = &StableId(10);
}
impl StableID for i32 {
    const _STABLE_ID: &'static StableId = &StableId(11);
}
impl StableID for i64 {
    const _STABLE_ID: &'static StableId = &StableId(12);
}
impl StableID for i128 {
    const _STABLE_ID: &'static StableId = &StableId(13);
}
impl StableID for isize {
    const _STABLE_ID: &'static StableId = &StableId(14);
}
impl StableID for f32 {
    const _STABLE_ID: &'static StableId = &StableId(15);
}
impl StableID for f64 {
    const _STABLE_ID: &'static StableId = &StableId(16);
}
impl StableID for String {
    const _STABLE_ID: &'static StableId = &StableId(17);
}
impl StableID for &str {
    const _STABLE_ID: &'static StableId = &StableId(18);
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
