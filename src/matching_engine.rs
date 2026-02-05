enum Order {
    Buy(BuyOrder),
    Sell(SellOrder)
}

struct MatchingEngine {
    order_book: OrderBook
}

impl MatchingEngine {
    fn run(&self) {
        // this needs to run the entire engine
    }
    fn process(&self, order: Order) {
        // process buy or sell
    }
}
