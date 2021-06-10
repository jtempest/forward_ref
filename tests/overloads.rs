use core::ops::{Add, AddAssign, Neg};
use forward_ref::{forward_ref_binop, forward_ref_op_assign, forward_ref_unop};

#[derive(Clone, Copy, Debug, PartialEq)]
struct MyInt(i32);

impl Neg for MyInt {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

forward_ref_unop!(impl Neg, neg for MyInt);

impl Add for MyInt {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<i32> for MyInt {
    type Output = Self;

    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<MyInt> for i32 {
    type Output = MyInt;

    #[inline]
    fn add(self, rhs: MyInt) -> Self::Output {
        MyInt(self + rhs.0)
    }
}

forward_ref_binop!(impl Add, add for MyInt, MyInt);
forward_ref_binop!(impl Add, add for MyInt, i32);
forward_ref_binop!(impl Add, add for i32, MyInt);

impl AddAssign for MyInt {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl AddAssign<i32> for MyInt {
    #[inline]
    fn add_assign(&mut self, rhs: i32) {
        self.0.add_assign(rhs);
    }
}

impl AddAssign<MyInt> for i32 {
    #[inline]
    fn add_assign(&mut self, rhs: MyInt) {
        self.add_assign(rhs.0);
    }
}

forward_ref_op_assign!(impl AddAssign, add_assign for MyInt, MyInt);
forward_ref_op_assign!(impl AddAssign, add_assign for MyInt, i32);
forward_ref_op_assign!(impl AddAssign, add_assign for i32, MyInt);

#[test]
fn unop_self() {
    let a = MyInt(1);
    assert_eq!(-a, MyInt(-1));
    assert_eq!(-&a, MyInt(-1));
}

#[test]
fn binop() {
    let a = MyInt(1);
    let b = MyInt(2);
    assert_eq!(a + b, MyInt(3));
    assert_eq!(&a + b, MyInt(3));
    assert_eq!(a + &b, MyInt(3));
    assert_eq!(&a + &b, MyInt(3));

    let a = MyInt(1);
    let b = 2;
    assert_eq!(a + b, MyInt(3));
    assert_eq!(&a + b, MyInt(3));
    assert_eq!(a + &b, MyInt(3));
    assert_eq!(&a + &b, MyInt(3));

    let a = 1;
    let b = MyInt(2);
    assert_eq!(a + b, MyInt(3));
    assert_eq!(&a + b, MyInt(3));
    assert_eq!(a + &b, MyInt(3));
    assert_eq!(&a + &b, MyInt(3));
}

#[test]
fn opassign() {
    let mut a = MyInt(1);
    let b = MyInt(2);
    a += b;
    assert_eq!(a, MyInt(3));
    a += &b;
    assert_eq!(a, MyInt(5));

    let mut a = MyInt(1);
    let b = 2;
    a += b;
    assert_eq!(a, MyInt(3));
    a += &b;
    assert_eq!(a, MyInt(5));

    let mut a = 1;
    let b = MyInt(2);
    a += b;
    assert_eq!(a, 3);
    a += &b;
    assert_eq!(a, 5);
}
