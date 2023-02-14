use std::{
    ops::{
        Add, BitAnd, BitOr, BitXor, Deref, DerefMut, Div, Index, IndexMut, Mul, Not, Rem, Shl, Shr,
        Sub,
    },
    slice::SliceIndex,
};

#[macro_export]
macro_rules! b {
    [$T:ty] => {
        Box<$T>
    };
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct U8x4([u8; 4]);

impl U8x4 {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self([a, b, c, d])
    }

    pub fn from_u16(l: u16, r: u16) -> Self {
        let ll = ((l >> 8) & 0xff) as u8;
        let lr = (l & 0xff) as u8;
        let rl = ((r >> 8) & 0xff) as u8;
        let rr = (r & 0xff) as u8;

        Self([ll, lr, rl, rr])
    }

    pub fn from_u32(x: u32) -> Self {
        let l = ((x >> 16) & 0xffff) as u16;
        let r = (x & 0xffff) as u16;
        Self::from_u16(l, r)
    }

    pub fn as_u8(&self) -> &[u8; 4] {
        &self.0
    }

    pub fn as_u8_mut(&mut self) -> &mut [u8; 4] {
        &mut self.0
    }

    pub fn as_u16(&self) -> [u16; 2] {
        let left = ((self.0[0] as u16) << 8) | (self.0[1] as u16);
        let right = ((self.0[2] as u16) << 8) | (self.0[3] as u16);
        [left, right]
    }

    pub fn as_u32(&self) -> u32 {
        let [ll, lr, rl, rr] = self.0;
        ((ll as u32) << 24) | ((lr as u32) << 16) | ((rl as u32) << 8) | (rr as u32)
    }
}

impl<Idx> Index<Idx> for U8x4
where
    Idx: SliceIndex<[u8]>,
{
    type Output = Idx::Output;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<Idx> IndexMut<Idx> for U8x4
where
    Idx: SliceIndex<[u8]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Deref for U8x4 {
    type Target = [u8; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for U8x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq<u32> for U8x4 {
    fn eq(&self, other: &u32) -> bool {
        &self.as_u32() == other
    }
}

impl PartialOrd<u32> for U8x4 {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.as_u32().partial_cmp(other)
    }
}

impl From<u32> for U8x4 {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

macro_rules! impl_both {
    ($trait:ident, $name:ident) => {
        impl $trait for U8x4 {
            type Output = U8x4;
            fn $name(self, rhs: U8x4) -> Self::Output {
                Self::from_u32($trait::$name(self.as_u32(), rhs.as_u32()))
            }
        }

        impl $trait<u32> for U8x4 {
            type Output = U8x4;
            fn $name(self, rhs: u32) -> Self::Output {
                Self::from_u32($trait::$name(self.as_u32(), rhs))
            }
        }

        impl $trait<U8x4> for u32 {
            type Output = u32;
            fn $name(self, rhs: U8x4) -> Self::Output {
                $trait::$name(self, rhs.as_u32())
            }
        }
    };
}

impl_both!(Add, add);
impl_both!(Sub, sub);
impl_both!(Mul, mul);
impl_both!(Div, div);
impl_both!(Rem, rem);
impl_both!(BitAnd, bitand);
impl_both!(BitOr, bitor);
impl_both!(BitXor, bitxor);
impl_both!(Shl, shl);
impl_both!(Shr, shr);

impl Not for U8x4 {
    type Output = U8x4;
    fn not(self) -> Self::Output {
        Self::from_u32(!self.as_u32())
    }
}
