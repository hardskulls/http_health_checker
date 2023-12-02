use reqwest::StatusCode;
use std::io::{stdout, Stdout, Write};
use std::thread;
use std::time::Duration;

pub fn check_url(url: &str) -> String {
    let resp = reqwest::blocking::get(url);
    match resp.unwrap().status() {
        StatusCode::OK => "Result: OK(200)\n".to_string(),
        st_code => format!("Result: ERR({})\n", st_code.as_u16()),
    }
}

pub fn print_check_res(out: &mut Stdout, url: &str, greet: &str) {
    out.write_all(greet.as_bytes()).ok();
    out.flush().ok();

    let res = format!("{}\n", check_url(url));
    out.write_all(res.as_bytes()).ok();
}

pub fn check_url_in_loop(url: &str, timeout: Duration) {
    let mut out = stdout();
    let greet = format!("Checking '{url}'. ");

    loop {
        print_check_res(&mut out, url, &greet);
        thread::sleep(timeout);
    }
}
