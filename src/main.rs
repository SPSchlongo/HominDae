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
        price : str,
        change : str,
  	changePer : str,
  	volume : str,
  	ytd : str,
  	ytdPer : str,
        open : str,
        high : str,
        close : str,
        tick : str,
        graph[389] : str,
}       // 389 minutres in Trading day, one price for each minute





impl stock {
        fn fromTick (tick :&str) ->  stock {
                if str.len() > 9 {
                        stock::emtpy();       
                }
                let URL = generateURL(&tick);
                let data = webRequest(&URL);
                parseStock(&data)
        }
        fn search (term :&str) -> stock[] {
                let URL = generateURL(&str);
                let data = webRequest(&URL);
              
                
        }
        fn empty -> stock {
                let x[389] = "";
                
                stock { name : ""
                        price  : ""
                        change : ""
                        changePer : ""
                        volume : ""
                        ytd : ""
                        ytdPer : ""
                        open : ""
                        high :""
                        close : ""
                        tick   : ""
                        graph[389] : ""
                }
        }
}
fn parseStock(stockData :&str) -> stock {
	let mut tempStock: stock = stock::empty();
	let mut data = ""
	let mut vari = "";
	let mut inXML = false;
	let mut inData = false;
	for chr in stockData.chars() {
		if chr == "<" && inData == true {
			match vari {
				"Name" => tempstock.name = data,
				"Symbol" => tempstock.tick = data
				"LastPrice" => tempstock.tick = data,
				"Change" => tempstock.change = data,
				"ChangePercent" => tempstock.changePer = data,
				"Volume" => tempstock.volume = data,
				"ChangeYTD" => tempstock.ytd = data,
				"ChangePercentYTD" => tempstock.ytdPer = data,
				"High" => tempstock.high = data,
				"Low" => tempstock.low = data,
				"Open" => tempstock.open = data
			}
			inData = false;
			continue;
			data += char;
		}
		else if chr == ">" {
			if inXML != true {
				inXML = true;
				continue;
			}
			inData = true;
			vari = "";
			continue;
		}
		else if inData == false && inXML == true {
			vari += chr;
		}
		else if inData == true && chr != "<" && chr != "Q" {
			data += chr;
		}
		if chr == "Q" && inData == false{
			break;
		}// Q is only in closing xml tag.
	}
	tempstock;
}
			
		
