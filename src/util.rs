// Copyright 2016 Google Inc. All Rights Reserved.
//
// Licensed under the MIT License, <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use futures::{Future, Poll};
use futures::stream::Stream;
use std::fmt;
use std::error::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A bottom type that impls `Error`, `Serialize`, and `Deserialize`. It is impossible to
/// instantiate this type.
#[derive(Debug)]
pub struct Never(!);

impl Error for Never {
    fn description(&self) -> &str {
        match self.0 {
            // TODO(tikue): remove when https://github.com/rust-lang/rust/issues/12609 lands
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Never {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            // TODO(tikue): remove when https://github.com/rust-lang/rust/issues/12609 lands
            _ => unreachable!(),
        }
    }
}

impl Future for Never {
    type Item = Never;
    type Error = Never;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0 {
            // TODO(tikue): remove when https://github.com/rust-lang/rust/issues/12609 lands
            _ => unreachable!(),
        }
    }
}

impl Stream for Never {
    type Item = Never;
    type Error = Never;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.0 {
            // TODO(tikue): remove when https://github.com/rust-lang/rust/issues/12609 lands
            _ => unreachable!(),
        }
    }
}

impl Serialize for Never {
    fn serialize<S>(&self, _: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        match self.0 {
            // TODO(tikue): remove when https://github.com/rust-lang/rust/issues/12609 lands
            _ => unreachable!(),
        }
    }
}

// Please don't try to deserialize this. :(
impl Deserialize for Never {
    fn deserialize<D>(_: &mut D) -> Result<Self, D::Error> 
        where D: Deserializer
    {
        panic!("Never cannot be instantiated!");
    }
}

/// A `String` that impls `std::error::Error`. Useful for quick-and-dirty error propagation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Message(pub String);

impl Error for Message {
    fn description(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<S: Into<String>> From<S> for Message {
    fn from(s: S) -> Self {
        Message(s.into())
    }
}
