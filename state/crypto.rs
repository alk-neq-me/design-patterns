#![allow(unused_variables)]

/**
 * Base State Seller
 **/
trait Seller {
  fn sell_coin(&self);
}


/**
 * Concrete States
 */
struct FullSeller;

impl Seller for FullSeller {
  fn sell_coin(&self) {
    println!("full seller");
  }
}


struct HalfSeller;

impl Seller for HalfSeller {
  fn sell_coin(&self) {
    println!("half seller");
  }
}


/**
 * Context
 */
struct Trader {
  // Why Box, to set_seller dynamic Own type
  seller: Box<dyn Seller>
}


impl Trader{
  fn sell_coin(&self) {
    self.seller.sell_coin();
  }

  fn set_seller(&mut self, seller: Box<dyn Seller>) {
    self.seller = seller;
  }
}


fn main() {
  let full_seller = FullSeller;
  let half_seller = HalfSeller;

  let mut trader = Trader {
    seller: Box::new(half_seller)
  };

  trader.sell_coin();

  trader.set_seller(Box::new(full_seller));

  trader.sell_coin();
}
