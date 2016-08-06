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
