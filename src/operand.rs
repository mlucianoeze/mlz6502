#[derive(Clone, Copy)]
pub struct Val(pub u8);

#[derive(Clone, Copy)]
pub struct Addr(pub u16);

#[derive(Clone, Copy)]
pub struct Rel(pub i8);

#[derive(Clone, Copy)]
pub struct Acc;

#[derive(Clone, Copy)]
pub struct Imp;

pub trait Operand {}
impl Operand for Val {}
impl Operand for Addr {}
impl Operand for Rel {}
impl Operand for Acc {}
impl Operand for Imp {}
