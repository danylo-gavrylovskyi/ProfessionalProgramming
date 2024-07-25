trait PriceInfo {
    fn get_coefficient(&self) -> f64;
    fn get_base_price(&self) -> f64;
}

struct PricePresenter<const PRIORITY: usize>;

impl<const PRIORITY: usize> PricePresenter<PRIORITY> {
    fn print_total_price<T1, T2>(obj1: &T1, obj2: &T2)
    where
        T1: PriceInfo,
        T2: PriceInfo,
    {
        let total_price = (PRIORITY as f64) * obj1.get_coefficient() * obj1.get_base_price()
            + obj2.get_coefficient() * obj2.get_base_price();
        println!("Total Price: {}", total_price);
    }
}

struct Milk;

impl PriceInfo for Milk {
    fn get_coefficient(&self) -> f64 {
        1.2
    }
    fn get_base_price(&self) -> f64 {
        2.5
    }
}

struct Cookies;

impl PriceInfo for Cookies {
    fn get_coefficient(&self) -> f64 {
        0.8
    }
    fn get_base_price(&self) -> f64 {
        1.0
    }
}

struct Pineapple;

impl PriceInfo for Pineapple {
    fn get_coefficient(&self) -> f64 {
        1.5
    }
    fn get_base_price(&self) -> f64 {
        3.0
    }
}

fn main() {
    let milk = Milk;
    let cookies = Cookies;
    let pineapple = Pineapple;

    PricePresenter::<3>::print_total_price(&milk, &cookies);
    PricePresenter::<2>::print_total_price(&cookies, &pineapple);
}
