extern crate smoke_test;
use smoke_test::echo_server::*;

static IP: &str = "0.0.0.0";
const PORT: u32 = 8000;

fn main() {
    run_server(IP, PORT);
}
