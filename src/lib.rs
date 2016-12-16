
#![crate_name = "tlib_core"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![doc(html_logo_url = "http://tajoy.net/img/logo.jpg",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "http://tajoy.net/",
       html_playground_url = "https://play.rust-lang.org/",
       test(attr(allow(unused_variables), deny(warnings))))]


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("it_works");
    }
}
