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

impl FullSeller {
  fn new() -> FullSeller {
    FullSeller
  }
}

impl Seller for FullSeller {
  fn sell_coin(&self) {
    println!("full seller");
  }
}


struct HalfSeller;

impl HalfSeller {
  fn new() -> HalfSeller {
    HalfSeller
  }
}

impl Seller for HalfSeller {
  fn sell_coin(&self) {
    println!("half seller");
  }
}


/**
 * Context
 */
struct Trader<'a> {
  seller: &'a dyn Seller
}


impl<'a> Trader<'a> {
  fn new(seller: &'a dyn Seller) -> Trader {
    Trader {
      seller
    }
  }

  fn sell_coin(&self) {
    self.seller.sell_coin();
  }

  fn set_seller(&mut self, seller: &'a dyn Seller) {
    self.seller = seller;
  }
}


fn main() {
  let full_seller = FullSeller::new();
  let half_seller = HalfSeller::new();

  let mut trader = Trader::new(&half_seller);

  trader.sell_coin();

  trader.set_seller(&full_seller);

  trader.sell_coin();
}
