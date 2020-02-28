/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use hyper::method::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use uuid::Uuid;

pub trait Operation {
    type PathParams: for<'a> TupleRef<'a>;
    type QueryParams: UrlEncode;
    type Body: Serialize;
    type Output: for<'de> Deserialize<'de>;

    fn method() -> Method;
    fn path(p: <Self::PathParams as TupleRef>::Ref, q: &Self::QueryParams) -> String;

    fn to_body(body: &Self::Body) -> Option<serde_json::Value> {
        Some(serde_json::to_value(body).expect("serialize to value"))
    }
}

pub trait UrlEncode {
    fn url_encode(&self, m: &mut HashMap<&'static str, String>);

    fn encode(&self) -> String {
        let mut m = HashMap::new();
        self.url_encode(&mut m);
        let mut output = String::with_capacity(64);
        for (i, (k, v)) in m.into_iter().enumerate() {
            if i > 0 {
                output.push('&');
            }
            write!(&mut output, "{}={}", k, v).unwrap(); // FIXME: formurlencode
        }
        output
    }
}

impl UrlEncode for () {
    fn url_encode(&self, _m: &mut HashMap<&'static str, String>) {}
}

pub trait TupleRef<'a> {
    type Ref: 'a;
}

impl<'a> TupleRef<'a> for Uuid {
    type Ref = &'a Uuid;
}

impl<'a> TupleRef<'a> for String {
    type Ref = &'a String;
}

impl<'a> TupleRef<'a> for () {
    type Ref = ();
}

impl<'a, T1: 'a> TupleRef<'a> for (T1,) {
    type Ref = (&'a T1,);
}

impl<'a, T1: 'a, T2: 'a> TupleRef<'a> for (T1, T2) {
    type Ref = (&'a T1, &'a T2);
}

impl<'a, T1: 'a, T2: 'a, T3: 'a> TupleRef<'a> for (T1, T2, T3) {
    type Ref = (&'a T1, &'a T2, &'a T3);
}
