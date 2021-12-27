use std::rc::Rc;

/**
* public struct with public members
*/
pub struct TutorialPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/**
* Tuple struct, public
*/
pub struct TutorialTupleStruct (pub i32, pub i32, pub i32);

/**
* Enum
*/
pub enum TutorialEnum {
    RED,
    BLUE,
    GREEN,
    DEFAULT
}

/**
More complex enum
*/
pub enum TutorialEnumExpression {
    ADD(i32, i32),
    OR(bool, bool),
}

/**
Option has Some or None as a value, similar to optional in C++
As there is no nullptr in Rust (except for Raw ptrs) this is commonly used to represent this case
RefCount ptr is required, as struct cannot contain itself as a member (and also data structure wise
a ptr is wise in this case)
*/
pub struct TutorialNode {
    pub parent: Option<Rc<TutorialNode>>,
    pub value: i32
}