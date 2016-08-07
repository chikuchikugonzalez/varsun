// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

//! Provides MS-Windows style substition.
//!
//! MS-Winodws style substition is `%var%` format strings.
//! You can see on COMMAND PROMPT (not PowerShell).

/// Parse src and substitute found variables with result of `mapfn`.
///
/// # Examples
///
/// ```
/// use varsun::mswin::substitute;
///
/// let src = "foo is %foo%.";
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

    // Temporary variables.
    let mut varname = String::new();
    let mut started = false;

    // Check each characters.
    while let Some(ch) = chs.next() {
        if ch == '%' {
            if started {
                // Reach end of varname section.

                // Call mapping-function.
                if let Some(val) = mapfn(varname.as_str()) {
                    // Push raw value.
                    dst.push_str(val.as_str());

                    // Leave varname section.
                    started = false;
                } else {
                    // Push Back Variable.
                    dst.push('%');
                    dst.push_str(varname.as_str());
                }

                // Reset varname.
                varname.clear();
            } else {
                // Enter varname section.
                started = true;

                // Continue to Next.
                continue;
            }
        } else {
            if started {
                // Part of varname.
                varname.push(ch);
            } else {
                // Part of text.
                dst.push(ch);
            }
        }
    }

    // Push Back vaname if cursor placed in varname section yet.
    if started {
        dst.push('%');
        dst.push_str(varname.as_str());

        // Reset
        varname.clear();
    }

    return dst;
}

/// Substitute environment variable by Windows format.
pub fn substenvar(src: &str) -> String {
    return self::substitute(src, super::envar);
}

#[cfg(test)]
mod tests {
    fn mapfn(name: &str) -> Option<String> {
        match name {
            "foo" => Some("foo!!".to_string()),
            "bar" => Some("!bar!".to_string()),
            "baz" => Some("%baz%".to_string()),
            _     => Some("(　・ω・)?".to_string()),
        }
    }

    #[test]
    fn substitute_basic() {
        assert_eq!("foo!!", super::substitute("%foo%", mapfn));
        assert_eq!("!bar!", super::substitute("%bar%", mapfn));
        assert_eq!("%baz%", super::substitute("%baz%", mapfn));
        assert_eq!("foo is foo!!", super::substitute("foo is %foo%", mapfn));
        assert_eq!("!bar! not (　・ω・)?", super::substitute("%bar% not %foobar%", mapfn));
        assert_eq!("foo!!!bar!%baz%", super::substitute("%foo%%bar%%baz%", mapfn));
    }

    //#[bench]
    //fn substitute_bench(b: &mut Bencher) {
    //    b.iter(|| super::substitute("%foo%%bar%%baz%", mapfn));
    //}

    #[test]
    fn substenvar_basic() {
        ::std::env::set_var("FOO", "foo, foo!");
        ::std::env::set_var("BAR", "foobar");

        assert_eq!("foo, foo!", super::substenvar("%FOO%"));
        assert_eq!("foobar says 'foo, foo!'", super::substenvar("%BAR% says '%FOO%'"));
        assert_eq!("%BAZ%", super::substenvar("%BAZ%"));
    }

}
