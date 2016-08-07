// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

//! Provides POSIX style substition.
//!
//! I saying POSIX style like `${var}` or `$var` formatted strings.
//! > Main refering souce is Bash parameter substition.

/// Parse src and substitute found variables with result of `mapfn`.
///
/// # Examples
///
/// ```
/// use varsun::posix::substitute;
///
/// let src = "foo is $foo.";
/// let res = substitute(src, |name: &str| -> Option<String> {
///     match name {
///         "foo" => Some("FOO!!".to_string()),
///         _     => None,      // If None returns, variable replaces with "" (empty string).
///     }
/// });
/// ```
pub fn substitute<F>(src: &str, mapfn: F) -> String where F: Fn(&str) -> Option<String> {
    let mut dst = String::new();
    let mut chs = src.chars();

    // Temporaries.
    let mut varname = String::new();
    let mut started = false;
    let mut escaped = false;
    let mut bracket = false;

    while let Some(ch) = chs.next() {
        if started {
            if varname.is_empty() && ch == '{' {
                // Open bracket.
                bracket = true;
                continue;
            }

            // Get varname
            if bracket {
                if ch == '}' {
                    // Close bracket.
                    bracket = false;
                    started = false;

                    // Call mapping-function.
                    if let Some(val) = mapfn(varname.as_str()) {
                        // OK
                        dst.push_str(val.as_str());
                    }

                    // Reset varname.
                    varname.clear();
                } else {
                    // Inside of brackets, allow any characters.
                    // FIXME Incompatible with POSIX.
                    varname.push(ch);
                }
                continue;
            } else {
                // Check character range.
                if (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z') || (ch == '_') {
                    // Alphabets or Underbar.
                    varname.push(ch);
                    continue;
                } else if !varname.is_empty() && (ch >= '0' && ch <= '9') {
                    // ch is not first, allow numbers.
                    varname.push(ch);
                    continue;
                } else {
                    // ch cannot use for varname.
                    // (end of varname)
                    started = false;

                    // Call mapping-function.
                    if let Some(val) = mapfn(varname.as_str()) {
                        // OK
                        dst.push_str(val.as_str());
                    }

                    // Reset varname.
                    varname.clear();

                    // Pass throught to Check blocks.
                    // Why?
                    //   if ch is '$', this ch is start point of next variable.
                }
            }
        }

        // ---- Check Blocks ----

        // Check escape-seq.
        if !escaped && ch == '\\' {
            escaped = true;
            continue;
        }

        // Push to dest if escaped.
        if escaped {
            escaped = false;
            dst.push(ch);
            continue;
        }

        // Start of placeholder?
        if ch == '$' {
            started = true;
        } else {
            dst.push(ch);
        }
    }

    // varname still alive, replace of remove varname block.
    if !varname.is_empty() {
        if !bracket {
            // Replace mapped value.
            if let Some(val) = mapfn(varname.as_str()) {
                dst.push_str(val.as_str());
            }
        }
        varname.clear();

        // Opened bracket not closed, removed it.
    }

    return dst;
}

/// Substitute environment variable by POSIX format.
pub fn substenvar(src: &str) -> String {
    return self::substitute(src, super::envar);
}

#[cfg(test)]
mod tests {
    fn mapfn(name: &str) -> Option<String> {
        match name {
            "foo" => Some("foo!!".to_string()),
            "bar" => Some("!bar!".to_string()),
            "baz" => Some("-baz-".to_string()),
            _     => Some("(　・ω・)?".to_string()),
        }
    }

    #[test]
    fn substitute_basic() {
        assert_eq!("foo!!", super::substitute("$foo", mapfn));
        assert_eq!("!bar!", super::substitute("${bar}", mapfn));
        assert_eq!("-baz-", super::substitute("$baz", mapfn));
        assert_eq!("foo is foo!!", super::substitute("foo is $foo", mapfn));
        assert_eq!("!bar! not (　・ω・)?", super::substitute("${bar} not $foobar", mapfn));
        assert_eq!("$foo is foo!!", super::substitute("\\$foo is $foo", mapfn));
        assert_eq!("foo!!!bar!-baz-", super::substitute("$foo${bar}$baz", mapfn));
    }

    #[test]
    fn substitute_escape() {
        assert_eq!("foo!!", super::substitute("$foo", mapfn));
        assert_eq!("$foo", super::substitute("\\$foo", mapfn));
        assert_eq!("\\foo!!", super::substitute("\\\\$foo", mapfn));
        assert_eq!("\\${foo}", super::substitute("\\\\\\${foo}", mapfn));
    }

    //#[bench]
    //fn substitute_bench(b: &mut Bencher) {
    //    b.iter(|| super::substitute("$foo${bar}$baz", mapfn));
    //}

    #[test]
    fn substenvar_basic() {
        ::std::env::set_var("FOO", "foo, foo!");
        ::std::env::set_var("BAR", "foobar");

        assert_eq!("foo, foo!", super::substenvar("$FOO"));
        assert_eq!("foobar says 'foo, foo!'", super::substenvar("${BAR} says '$FOO'"));
        assert_eq!("", super::substenvar("$BAZ"));
        assert_eq!("foobar provided by $BAR", super::substenvar("$BAR provided by \\$BAR"));
    }
}
