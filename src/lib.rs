extern crate regex;

use regex::{Captures, Regex, RegexBuilder};

fn unbase(base: u8, string: &str) -> u64 {
    match base {
        n @ 2..=36 => u64::from_str_radix(string, n as u32).unwrap(),
        _ => unimplemented!(),
    }
}

fn filter_args(source: &str) -> Option<(&str, Vec<&str>, u32, u32)> {
    let regexes = [
        RegexBuilder::new(r"}\('(.*)', *(\d+), *(\d+), *'(.*)'\.split\('\|'\), *(\d+), *(.*)\)\)")
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        RegexBuilder::new(r"}\('(.*)', *(\d+), *(\d+), *'(.*)'\.split\('\|'\)")
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
    ];
    if let Some(regex) = regexes.get(0) {
        let args = regex.captures(source).unwrap();
        // There must be a better way im not thinking of
        let arg1 = if let Some(arg) = args.get(1) {
            arg.as_str()
        } else {
            return None;
        };
        let args2: Vec<&str> = if let Some(args) = args.get(4) {
            args.as_str().split('|').collect()
        } else {
            return None;
        };
        let arg3 = if let Some(arg) = args.get(2).map(|arg| arg.as_str()) {
            if let Ok(arg) = arg.parse::<u32>() {
                arg
            } else {
                return None;
            }
        } else {
            return None;
        };
        let arg4 = if let Some(arg) = args.get(3).map(|arg| arg.as_str()) {
            if let Ok(arg) = arg.parse::<u32>() {
                arg
            } else {
                return None;
            }
        } else {
            return None;
        };
        return Some((arg1, args2, arg3, arg4));
    }
    None
}

/// Detects whether `source` is P.A.C.K.E.R. coded.
pub fn detect(source: &str) -> bool {
    source
        .replace(' ', "")
        .starts_with("eval(function(p,a,c,k,e,")
}

/// Unpacks P.A.C.K.E.R. packed js code.
pub fn unpack(source: &str) -> Option<String> {
    let (payload, symtab, radix, count) = filter_args(source)?;
    if count != symtab.len() as u32 {
        return None;
    }
    let char_regex = Regex::new(r"\b\w+\b").unwrap();
    let source = char_regex.replace_all(payload, |x: &Captures| {
        // let cap = x[0]; //.to_string();
        let cap = if let Some(x) = x.get(0) {
            x.as_str()
        } else {
            ""
        };
        let sym = if let Some(sym) = symtab.get(unbase(radix as u8, cap) as usize) {
            sym
        } else {
            cap
        };
        sym.to_string()
    });
    Some(source.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &'static str = "eval(function(p,a,c,k,e,r){e=String;if(!''\
    .replace(/^/,String)){while(c--)r[c]=k[c]||c;k=[function(e){return r[e]}];e=\
    function(){return'\\w+'};c=1};while(c--)if(k[c])p=p.replace(new RegExp('\\b'+\
    e(c)+'\\b','g'),k[c]);return p}('1 0=2;3(0)',4,4,'x|var|5|alert'.split('|'),0,{}))";
    #[test]
    fn check_valid() {
        assert!(detect(TEST_DATA));
    }
    #[test]
    fn extract_args() {
        let (payload, symtab, radix, count) = filter_args(TEST_DATA).unwrap();
        assert_eq!(payload, "1 0=2;3(0)");
        assert_eq!(symtab, ["x", "var", "5", "alert"]);
        assert_eq!(radix, 4);
        assert_eq!(count, 4);
    }
    #[test]
    fn unpack_code() {
        assert_eq!(unpack(TEST_DATA).unwrap(), "var x=5;alert(x)");
    }
}
