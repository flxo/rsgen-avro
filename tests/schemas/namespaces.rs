pub mod com_example_a {

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct TypeA {
    pub value: String,
}
}

pub mod com_example_b {

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct TypeB {
    pub ref_to_a: super::com_example_a::TypeA,
}
}

pub mod com_example_c {

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct TypeC {
    pub ref_to_a: super::com_example_a::TypeA,
    pub ref_to_b: super::com_example_b::TypeB,
}
}
