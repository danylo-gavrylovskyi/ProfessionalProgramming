#include <print>
#include <concepts>

template<typename T>
concept HasPriceInfo = requires(T obj) {
    { obj.getCoefficient() } -> std::convertible_to<double>;
    { obj.getBasePrice() } -> std::convertible_to<double>;
};

template<size_t PRIORITY>
class PricePresenter {
public:
    template<HasPriceInfo T1, HasPriceInfo T2>
    void printTotalPrice(const T1& obj1, const T2& obj2) {
        double totalPrice = PRIORITY * obj1.getCoefficient() * obj1.getBasePrice()
                           + obj2.getCoefficient() * obj2.getBasePrice();
        std::println("Total Price: {}", totalPrice);
    }
};

class Milk {
public:
    double getCoefficient() const { return 1.2; }
    double getBasePrice() const { return 2.5; }
};

class Cookies {
public:
    double getCoefficient() const { return 0.8; }
    double getBasePrice() const { return 1.0; }
};

class Pineapple {
public:
    double getCoefficient() const { return 1.5; }
    double getBasePrice() const { return 3.0; }
};

int main() {
    PricePresenter<3> presenter1;
    PricePresenter<2> presenter2;

    Milk milk;
    Cookies cookies;
    Pineapple pineapple;

    presenter1.printTotalPrice(milk, cookies);
    presenter2.printTotalPrice(cookies, pineapple);

    return 0;
}
