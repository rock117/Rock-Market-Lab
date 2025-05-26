use std::fmt::Debug;
use derive_more::Display;

#[derive(Debug, Eq, PartialEq, Clone, Display)]
pub enum AllSingle<T> where T: Debug + Clone + Eq + PartialEq {
    All,
    Single(T),
}