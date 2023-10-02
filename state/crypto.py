"""
# Startegy patterns
 - create context class
 - create startegy abstract class
 - create concrete startegies class
"""


from abc import ABC, abstractmethod
from dataclasses import dataclass

class Seller(ABC):
    """seller Base State abstract class"""
    @abstractmethod
    def sell_coin(self) -> None:
        """sell crypto"""


class FullSeller(Seller):
    """full coin seller: concrete state"""
    def sell_coin(self) -> None:
        print("Sell Full Coin")


class HalfSeller(Seller):
    """full coin seller: concrete state"""
    def sell_coin(self) -> None:
        print("Sell Half Coin")


@dataclass(frozen=False)
class Trader:
    """Trader: context"""
    seller: Seller

    def sell_coin(self) -> None:
        self.seller.sell_coin()

    def set_seller(self, seller: Seller) -> None:
        self.seller = seller


def main() -> None:
    full_seller = FullSeller()
    half_seller = HalfSeller()

    trader = Trader(seller=full_seller)
    
    trader.sell_coin()

    trader.set_seller(half_seller)

    trader.sell_coin()


if __name__ == "__main__":
    main()
