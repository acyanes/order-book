use std::vec::Vec;

/*
* What is a order book?
* real time list of all pending buy and sell orders for a
* specific financial asset
* Bids - buyers desired prices and quantities -> listed high to low
* Asks - sellers desired prices and quantities -> listed low to high
*
* Types of orders
* Limit orders - Orders to buy or sell at a specific price or better
* forming the core of the order book.
*
* Market orders - Orders to execute immediately at the best available
* price, bypassing the list.
*
*
* When an order arrives, the orderbook is responsible for matching
* Matching - Bid => is there a sell that matches the bid?
* --- if yes, execute the trade
* --- if no, the bid "rests" waiting for a future sell order to match
*
* Selling => the reverse happens
*
* Order Types
* LimitOrder -> I want 10 shares @ 50 or less
* If no match exists, rest
* Stays until cancelled, expired, or matched
*
*
*
* MarketOrder -> I want 10 shares as soon as possible at whatever price
* Match immediately against rest orders
* Never rests, if unfilled cancel or reject
* 
*
* Price time priority
* If multiple orders exist at the same price, FIFO to fufill
*
*
*/

#[derive(Debug)]
enum OrderType {
    LimitOrder,
    MakertOrder
}

#[derive(Debug)]
struct Order {
    asset: String,
    price: f32,
    quantity:f32,
    order_type: OrderType,
}


#[derive(Debug)]
struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>,
    limit_orders: Vec<Order>,
    market_orders: Vec<Order>,
}

fn main() {
    let mut order_book = OrderBook {
        bids: Vec::new(),
        asks: Vec::new(),
        limit_orders: Vec::new(),
        market_orders: Vec::new(),
    };

    // put in a bid - bid assumes that an asset exists to be purchased, but
    // does not guarantee an immediate seller
    order_book.bids.push(Order {
        asset: "PLTR".to_string(),
        price: 100.0,
        quantity: 100.5,
        order_type: OrderType::LimitOrder
    });

    println!("Debug print: {:?}", order_book.bids);


    // put in a ask
}



// TODO - this is the happy case, what if the orders arent perfect ie bid is 100, there an ask with
// [50, 50] -> do they get 50 then 50, or do they get the 50 then sit back in the queue for the
// next 50?




/*The Matching Process

  When a bid (buy order) comes in:

  1. Check the asks (sell side) - Is there a seller willing to sell at or below the buyer's price?
  2. If yes → Execute a trade. The order is "filled" (fully or partially)
  3. If no → The bid "rests" on the book, waiting for a future sell order to match it

  The reverse happens for asks.

  Two Order Types

  Limit orders - "I want to buy 10 shares at $50 or less"
  - If no match exists, the order rests on the book
  - It stays there until matched, cancelled, or expires

  Market orders - "I want to buy 10 shares right now at whatever price"
  - These execute immediately against resting orders
  - They never rest on the book (if unfilled, they typically cancel or reject)

  Price-Time Priority

  When multiple orders exist at the same price, they're filled in the order they arrived (FIFO). This is why you often see a queue at each price level.

  The Book Structure

  ASKS (sell side) - sorted lowest first
    $52.00: [100 shares, 50 shares]   ← best ask
    $53.00: [200 shares]

  --- spread ($1) ---

  BIDS (buy side) - sorted highest first
    $51.00: [150 shares, 75 shares]   ← best bid
    $50.00: [300 shares]

  Example Flow

  1. New buy limit order arrives: "Buy 100 @ $52.50"
  2. Orderbook checks asks: best ask is $52.00
  3. $52.00 ≤ $52.50, so it matches
  4. Trade executes: buyer gets 100 shares at $52.00
  5. The ask order at $52.00 is reduced by 100 (or removed if fully filled)
  6. The incoming order is fully filled, so it doesn't rest

  If the buy order was instead "Buy 100 @ $50.50", no match exists (best ask is $52), so it rests on the bid side at $50.50.
*/
