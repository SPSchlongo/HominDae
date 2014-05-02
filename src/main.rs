export crate HominDae_Trader;


use wxRust::*;
use rust-html;

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

