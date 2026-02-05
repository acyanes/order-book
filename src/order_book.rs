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
}

#[derive(Debug)]
struct BuyOrder {
    order: Order,
    order_type: OrderType,
}

#[derive(Debug)]
struct SellOrder {
    order: Order
}


#[derive(Debug)]
struct OrderBook {
    bids: Vec<BuyOrder>,
    asks: Vec<SellOrder>,
    limit_orders: Vec<Order>,
    market_orders: Vec<Order>,
}
