export crate HominDae_Trader;


use wxRust::*;
use rust-html;
#![crate_id = "client"]

extern crate http;
use http::client::RequestWriter;
use http::method::Get;
use http::headers::HeaderEnum;
use std::os;
use std::str;
use std::io::println;

fn main() {
    println!("started");
    format!("{}", Get);
    let args = os::args();
    if args.len() != 4 {
 	println!("Usage: {} tick spacing url", args[0]);
    }
     else {
    let url = parse_url(args[1],args[2],args[3]);
		make_and_print_request(url);


            return;
        }
    }


fn parse_url(tick:&str,spacing:&str,days: &str) -> ~str {

	let x = "http://www.google.com/finance/getprices?q=" + tick + "&i=" + spacing + "&p=" + days + "d";
	println!("{}",x);
	x
	//make_and_print_request(url);
}

fn make_and_print_request(url: &str) {
    let request: RequestWriter = RequestWriter::new(Get, from_str(url).expect("Invalid URL :-("))
                                           .unwrap();
    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_request) => fail!("This example can progress no further with no response :-("),
    };

    let body = match response.read_to_end() {
        Ok(body) => body,
        Err(err) => fail!("Reading response failed: {}", err),
    };
    println(str::from_utf8(body.as_slice()).expect("Uh oh, response wasn't UTF-8"));
}

struct stock {
        name : str
        price : float32
        change : float32
        open : float32
        close: float32
        tick : string
        graph[389] : float 32
}       // 389 minutres in Trading day, one price for each minute





impl stock
        fn fromTick (tick :&str) ->  stock {
                if str.len() > 9 {
                        stock::error         
                        }
                getStock(&tick);
        }
        fn search (term :&str) -> stock[] {
                findStock(&term) 
        }
        fn error -> stock {
                let x[389] = 0;
                
                stock { name : "Not Found"
                        price  : 0.00
                        change : 0.00
                        tick   : "N/A"
                        graph[389] : x
                }
        }
}


