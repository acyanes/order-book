enum Order {
    Buy(BuyOrder)
    Sell(SellOrder)
}

struct MatchingEngine {
    order: Order
}

impl MatchingEngine {

}
