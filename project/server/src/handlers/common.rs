use schema::common::*;
use schema::backend::*;


pub struct Auth<T> {
    input: T,
    token: Token,
}