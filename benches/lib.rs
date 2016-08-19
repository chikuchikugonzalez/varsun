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
fn posix_nothing(b: &mut Bencher) {
    let src = "foo bar baz";
    b.iter(|| {
        varsun::posix::substitute(src, mapfn);
    });
}

#[bench]
fn posix_simple(b: &mut Bencher) {
    let src = "${foo}";
    b.iter(|| {
        varsun::posix::substitute(src, mapfn);
    });
}

#[bench]
fn posix_once(b: &mut Bencher) {
    let src = "foo, ${bar} and baz.";
    b.iter(|| {
        varsun::posix::substitute(src, mapfn);
    });
}

#[bench]
fn posix_twice(b: &mut Bencher) {
    let src = "foo is ${foo}, bar is $bar.";
    b.iter(|| {
        varsun::posix::substitute(src, mapfn);
    });
}

#[bench]
fn mswin_nothing(b: &mut Bencher) {
    let src = "foo bar baz";
    b.iter(|| {
        varsun::mswin::substitute(src, mapfn);
    });
}

#[bench]
fn mswin_simple(b: &mut Bencher) {
    let src = "%foo%";
    b.iter(|| {
        varsun::mswin::substitute(src, mapfn);
    });
}

#[bench]
fn mswin_once(b: &mut Bencher) {
    let src = "foo, %bar% and baz.";
    b.iter(|| {
        varsun::mswin::substitute(src, mapfn);
    });
}

#[bench]
fn mswin_twice(b: &mut Bencher) {
    let src = "foo is %foo%, baz is %baz%.";
    b.iter(|| {
        varsun::mswin::substitute(src, mapfn);
    });
}
