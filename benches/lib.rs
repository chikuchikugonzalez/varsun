// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:
#![feature(test)]

extern crate test;
extern crate varsun;

use test::Bencher;

fn mapfn(name: &str) -> Option<String> {
    match name {
        "foo" => Some("foo!!".to_string()),
        "bar" => Some("!bar!".to_string()),
        "baz" => Some("-baz-".to_string()),
        _     => Some("(　・ω・)?".to_string()),
    }
}

#[bench]
fn posix_basic(b: &mut Bencher) {
    let src = "${foo}";
    b.iter(|| {
        varsun::posix::substitute(src, mapfn);
    });
}

#[bench]
fn mswin_basic(b: &mut Bencher) {
    let src = "%foo%";
    b.iter(|| {
        varsun::mswin::substitute(src, mapfn);
    });
}
