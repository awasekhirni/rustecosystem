//base trait for printable descriptions

trait Describable {
    fn describe(&self) {
        println!("This is a generic product!");
    }
}

//Product trait inherits from Describable
trait Product: Describable {
    fn name(&self) -> &str; //type string
    fn price(&self) -> f64; //type float 64 bit

    //default method implementation
    fn label(&self) -> String {
        format!("{}-${:.2}", self.name(), self.price())
    }
}

//Product types

struct Book {
    title: String,
    price: f64,
    author: String,
}

struct Electronics {
    model: String,
    price: f64,
    brand: String,
}

//implement Describable
impl Describable for Book {
    fn describe(&self) {
        println!(
            "'{}' by {}.priced at ${:.2}",
            self.title, self.author, self.price
        );
    }
}

impl Describable for Electronics {
    fn describe(&self) {
        println!("{} {} for ${:.2}", self.brand, self.model, self.price);
    }
}

//Implement Product

impl Product for Book {
    fn name(&self) -> &str {
        &self.title
    }

    fn price(&self) -> f64 {
        self.price
    }
}

impl Product for Electronics {
    fn name(&self) -> &str {
        &self.model
    }
    fn price(&self) -> f64 {
        self.price
    }
}

//Enum for dynamic creation of products
enum ProductType {
    Book(String, f64, String),
    Electronics(String, f64, String),
}

//Factory function
fn product_factory(product_type: ProductType) -> Box<dyn Product> {
    match product_type {
        ProductType::Book(title, price, author) => Box::new(Book {
            title,
            price,
            author,
        }),
        ProductType::Electronics(model, price, brand) => Box::new(Electronics {
            model,
            price,
            brand,
        }),
    }
}

//Generic Function
fn display_product_info<T: Product>(product: &T) {
    product.describe();
    println!("Label:{}", product.label());
}

fn main() {
    //using trait object
    let products: Vec<Box<dyn Product>> = vec![
        product_factory(ProductType::Book(
            "Exploratory GIR".into(),
            189.99,
            "Awase Khirni Syed".into(),
        )),
        product_factory(ProductType::Electronics(
            "Sony Z1".into(),
            999.18,
            "Sony Corporation".into(),
        )),
    ];

    println!("--using trait object---");
    for product in &products {
        println!("{}", product.label());
    }

    println!("\n--Using generics with trait bounds--");
    let my_book = Book {
        title: "Startup Valuation".into(),
        price: 89.99,
        author: "Aswath Damodaran".into(),
    };
    display_product_info(&my_book);
}
