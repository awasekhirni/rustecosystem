mod products;

use products::{
    discount::strategy::{CategoryDiscount, DiscountStrategy, PercentageDiscount},
    product::Product,
    repository::{InMemoryProductRepository, ProductRepository},
    service::ProductService,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create repository
    let mut repository = InMemoryProductRepository::new();

    // Demonstrate different discount strategies
    demo_discount_strategies(&mut repository)?;

    Ok(())
}

fn demo_discount_strategies(repository: &mut impl ProductRepository) -> Result<(), Box<dyn Error>> {
    println!("=== No Discount Strategy ===");
    let no_discount = products::discount::strategy::NoDiscount;
    let mut service = ProductService::new(repository, no_discount);
    demo_service_operations(&mut service)?;

    println!("\n=== Percentage Discount Strategy (10% off) ===");
    let percentage_discount = PercentageDiscount::new(10.0);
    let mut service = ProductService::new(repository, percentage_discount);
    demo_service_operations(&mut service)?;

    println!("\n=== Category Discount Strategy (20% off Electronics) ===");
    let category_discount = CategoryDiscount::new("Electronics".to_string(), 20.0);
    let mut service = ProductService::new(repository, category_discount);
    demo_service_operations(&mut service)?;

    Ok(())
}

fn demo_service_operations<R: ProductRepository, D: DiscountStrategy>(
    service: &mut ProductService<R, D>,
) -> Result<(), Box<dyn Error>> {
    // Add products
    service.add_product(Product::new(
        1,
        "Laptop".to_string(),
        1000.0,
        "Electronics".to_string(),
    ))?;
    service.add_product(Product::new(
        2,
        "T-Shirt".to_string(),
        25.0,
        "Clothing".to_string(),
    ))?;

    // List products
    println!("All products:");
    for product in service.list_products() {
        println!(
            "ID: {}, Name: {}, Price: ${:.2}, Category: {}",
            product.id, product.name, product.price, product.category
        );
    }

    // Get single product
    if let Some(product) = service.get_product(1) {
        println!("\nProduct with ID 1:");
        println!(
            "ID: {}, Name: {}, Price: ${:.2}, Category: {}",
            product.id, product.name, product.price, product.category
        );
    }

    // Remove a product
    service.remove_product(2)?;
    println!("\nAfter removing product with ID 2:");
    for product in service.list_products() {
        println!(
            "ID: {}, Name: {}, Price: ${:.2}, Category: {}",
            product.id, product.name, product.price, product.category
        );
    }

    Ok(())
}
