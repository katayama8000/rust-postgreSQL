use lombok::*;

#[derive(Getter, GetterMut, Setter, NoArgsConstructor, AllArgsConstructor, ToString, Clone)]
pub struct ProductCategory {
    id: i32,
    name: String,
    product: Option<Vec<Product>>,
}

#[derive(Getter, GetterMut, Setter, NoArgsConstructor, AllArgsConstructor, ToString, Clone)]
pub struct Product {
    id: i32,
    name: String,
    price: i32,
    category_id: i32,
    product_category: Option<ProductCategory>,
}
