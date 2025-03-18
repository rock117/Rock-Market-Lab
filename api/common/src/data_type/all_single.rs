use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AllSingle<T> where T: Debug + Clone + Eq + PartialEq {
    All,
    Single(T),
}