// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:
//
// ## 名前の由来
// varsun=VARiable SUbstitioN という適当な造語です。
//
// ## 読み方
// `ヴァルサン` または `バルサン` です。
// > 虫退治のアレと同じ読み方か
//

//! Shell-like variable substition functions.
//!
//! Basic usages are:
//!
//! - Simple templating.
//! - Expand envrionment variables.
//!

pub mod posix;
pub mod mswin;

/// Parse src string and replace found variables with `mapfn` result.
///
/// This function is an proxy function. Call `substitute` function in `mswin` module if platform is
/// windows, otherwise call `posix` implemntation.
pub fn substitute<F>(src: &str, mapfn: F) -> String where F: Fn(&str) -> Option<String> {
    if cfg!(windows) {
        return self::mswin::substitute(src, mapfn);
    } else {
        return self::posix::substitute(src, mapfn);
    }
}

/// Parse src string and replace variables with Environment Variable.
pub fn substenvar(src: &str) -> String {
    return self::substitute(src, self::envar);
}

fn envar(name: &str) -> Option<String> {
    match ::std::env::var(name) {
        Ok(val) => Some(val),
        Err(_)  => None,
    }
}
