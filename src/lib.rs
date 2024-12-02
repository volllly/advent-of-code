use std::{fmt::Debug, str::FromStr};

use chumsky::prelude::*;

pub mod template;

pub fn int<T>() -> impl Parser<char, T, Error = Simple<char>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    text::int::<char, Simple<_>>(10).try_map(|s: String, span| {
        s.parse::<T>()
            .map_err(|e| Simple::custom(span, format!("{:?}", e)))
    })
}
