    use pizza_builder::ingredient::Ingredient;
    use pizza_builder::pizza::Pizza;
    use pizza_builder::menu::Menu;
    use pizza_builder::pizza_builder::PizzaBuilder;

    #[test]
    fn test_create_ingredient() {
        let cheese = Ingredient::new("Cheese", 1.50);
        assert_eq!(cheese.get_name(), "Cheese");
        assert_eq!(cheese.get_cost(), 1.50);
    }

    #[test]
    fn test_add_ingredient() {
        let mut pizza = Pizza::new();
        let cheese = Ingredient::new("Cheese", 1.50);
        pizza.add_ingredient(cheese);
        assert_eq!(pizza.get_ingredients().len(), 1);
        assert_eq!(pizza.get_total_cost(), 1.50);
    }

    #[test]
    fn test_build_pizza() {
        let pizza = PizzaBuilder::new()
            .add_ingredient(Ingredient::new("Cheese", 1.50))
            .add_ingredient(Ingredient::new("Tomato", 0.75))
            .build();

        assert_eq!(pizza.get_ingredients().len(), 2);
        assert_eq!(pizza.get_total_cost(), 2.25);
    }

    #[test]
    fn test_load_ingredients() {
        let menu = Menu::new("src/data/ingredients.txt");
        assert!(menu.get_ingredients().len() > 0);
    }
