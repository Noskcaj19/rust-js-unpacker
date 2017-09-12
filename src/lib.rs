extern crate regex;

#[cfg(test)]
mod tests {
    use super::*;
    static test: &'static str = "eval(function(p,a,c,k,e,r){e=String;if(!''.replace(/^/,String)){while(c--)r[c]=k[c]||c;k=[function(e){return r[e]}];e=function(){return'\\w+'};c=1};while(c--)if(k[c])p=p.replace(new RegExp('\\b'+e(c)+'\\b','g'),k[c]);return p}('1 0=2;3(0)',4,4,'x|var|5|alert'.split('|'),0,{}))";
    #[test]
    fn check_valid() {
        assert!(detect(test));
    }
}
