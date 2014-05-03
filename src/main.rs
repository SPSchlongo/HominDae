export crate HominDae_Trader;


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

/*
 *&q is ticker input
 *&p is number of days
 *&i is the price every i seconds
 */
fn generateURL(tick:&str) -> str {
	"http://www.google.com/finance/getprices?q=" + tick + "&i=" + "60" + "&p=" + "10" + "d";
}

fn webRequest(url: &str) -> ~str {
	let request: RequestWriter = RequestWriter::new(Get, from_str(url).expect("Invalid URL :-(")).unwrap();
	let mut response = match request.read_response() {
		Ok(response) => response,
		Err(_request) => fail!("This example can progress no further with no response :-("),
	};

	let body = match response.read_to_end() {
		Ok(body) => body,
		Err(err) => fail!("Reading response failed: {}", err),
	};
	str::from_utf8(body.as_slice()).expect("Uh oh, response wasn't UTF-8")
}

struct stock {
        name : str,
        price : float,
        change : float,
  	changePer : float
  	volume : int
  	ytd : float
  	ytdPer : float
        open : float,
        high : float
        close : float
        tick : str,
        graph[389] : float 32
}       // 389 minutres in Trading day, one price for each minute





impl stock {
        fn fromTick (tick :&str) ->  stock {
                if str.len() > 9 {
                        stock::error         
                }
                let URL = generateURL(&tick);
                let data = webRequest(&URL);
        }
        fn search (term :&str) -> stock[] {
                let URL = generateURL(&str);
                let data = webRequest(&URL);
                
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


