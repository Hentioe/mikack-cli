use crate::{checker, errors::*};
use std::process::Command;

pub fn read_output(code: &str) -> Result<String> {
    if !check_node() {
        return Err(err_msg("please install Node.js: https://nodejs.org"));
    }
    let output = Command::new("node").arg("-e").arg(code).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(err_msg(format!(
            "javascript execution failed, {}",
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

fn check_node() -> bool {
    checker::exec_succeed("node", &["-v"])
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_read_output() {
        let code = r#"
        console.log('Hello world!');
        "#;
        let output = read_output(code).unwrap();
        assert_eq!(output, "Hello world!");
    }

    #[test]
    fn test_read_from_json() {
        let code_s = r#"
var arr_img = new Array();
var page = '';
eval(function (p, a, c, k, e, d) {
    e = function (c) {
        return (c < a ? '' : e(parseInt(c / a))) + ((c = c % a) > 35 ? String.fromCharCode(c + 29) : c.toString(36))
    };
    if (!''.replace(/^/, String)) {
        while (c--) {
            d[e(c)] = k[c] || e(c)
        }
        k = [function (e) {
            return d[e]
        }];
        e = function () {
            return '\\w+'
        };
        c = 1
    };
    while (c--) {
        if (k[c]) {
            p = p.replace(new RegExp('\\b' + e(c) + '\\b', 'g'), k[c])
        }
    }
    return p
}('q m=m=\'["l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/C.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/B.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/A.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/z.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/D.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/E-7.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/I.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/H.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/G.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/F.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/y.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/x.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/p.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/o.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/n.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/r.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/s.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/w.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/v.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/u.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/t.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/J.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/V.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/10.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/Z.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/Y.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/X.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/11.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/12.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/15.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/13.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/K.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/16.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/14.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/W.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/O.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/N.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/M.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/L.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/P.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/Q.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/U.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/T.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/S.e","l\\/%2%1%h%2%1%8%0%6%a%0%5%3%4%9%k%0%b%i\\/%0%j%g%f%c%d\\/R.e"]\';', 62, 69, 'E7|B5|E6|84|E4|9A|8C||AA|B8|AB|94|AF|9D|jpg|E8|AC01|81|9F|AC|80||pages|016|015|014|var|017|018|022|021|020|019|013|012|004|003|002|001|005|006|011|010|009|008|023|033|040|039|038|037|041|042|046|045|044|043|024|036|028|027|026|025|029|030|032|035|031|034'.split('|'), 0, {}))

;
var g_comic_name = "流浪猫的一生";
var g_chapter_name = "第01话";
var g_comic_url = "liulangmaodeyisheng/";
var g_chapter_url = "liulangmaodeyisheng/81737.shtml";
var g_current_page = 1;
var g_max_pic_count = 45;
var g_page_base = '';
var g_comic_id = res_id = '46127';
var g_chapter_id = chapter_id = '81737';
var g_comic_code = '80c6cb4e1d21d07b03a4d05a33019f94';
var arr_pages = eval(pages);
var next_chapter_pages = '["l\/流浪猫的一生\/第02话\/001.jpg","l\/流浪猫的一生\/第02话\/003.jpg","l\/流浪猫的一生\/第02话\/004.jpg"]';
var arr_nextchapter_pages = eval(next_chapter_pages);
var final_page_url = "/liulangmaodeyisheng/jump.shtml?46127_81737&e4086e592c810f77058e22638c2ba5c8";
var sns_sys_id = '46127_81737';
var sns_view_point_token = 'e4086e592c810f77058e22638c2ba5c8';
var is_hot_comic = false;
var is_fast_comic = true;
var server_name = 0;
var page_site_root = '/';
var res_type = 1;

// Output code
console.log(JSON.stringify({pages: eval(pages), name: `${g_comic_name} ${g_chapter_name}`}));
        "#;

        let output = read_output(code_s).unwrap();
        let v: Value = serde_json::from_str(&output).unwrap();
        let name = v["name"].as_str().unwrap().to_string();
        let pages = v["pages"].as_array().unwrap();
        assert_eq!("流浪猫的一生 第01话", &name);
        assert_eq!(45, pages.len());
    }
}
