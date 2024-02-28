use std::fmt::format;

use anyhow::{Error, Result};
use postgres::{types::Type, Transaction};

use crate::{entities::Product, repository::Repository};

pub struct ProductRepository<'a, 'b>(pub &'a mut Transaction<'b>);

impl Repository<Product, i32, u64> for ProductRepository<'_, '_> {
    fn select_all(&mut self) -> Result<Vec<Product>> {
        let mut products = Vec::<Product>::new();
        let rows = self.0.query("SELECT id, name, price FROM products", &[])?;
        for row in rows.iter() {
            products.push(Product::new(
                row.get("id"),
                row.get("name"),
                row.get("price"),
                row.get("category_id"),
                None,
            ));
        }
        Ok(products)
    }

    fn select_by_id(&mut self, id: i32) -> Result<Product> {
        let sql = "SELECT id, name, price FROM products WHERE id = $1";
        let stmt = self.0.prepare_typed(sql, &[Type::INT4])?;
        let result = self.0.query_opt(&stmt, &[&id])?;
        match result {
            Some(row) => Ok(Product::new(
                row.get("id"),
                row.get("name"),
                row.get("price"),
                row.get("category_id"),
                None,
            )),

            None => Err(Error::msg(format!(
                "指定された値: {}に該当するレコードが存在しません。",
                id,
            ))),
        }
    }

    fn insert(&mut self, row: Product) -> Result<u64> {
        todo!()
    }

    fn update_by_id(&mut self, id: i32) -> Result<u64> {
        todo!()
    }

    fn delete_by_id(&mut self, id: i32) -> Result<u64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connect::PostgresClient;
    use crate::params::ConnectionParams;
    use crate::transaction::TransactionUtil;

    #[test]
    fn select_by_id() -> Result<()> {
        let params = ConnectionParams::new(
            "postgres".to_string(),
            5432,
            "mydatabase".to_string(),
            "myuser".to_string(),
            "mypassword".to_string(),
        );
        let mut client = PostgresClient::connect(&params)?;
        let mut transaction = TransactionUtil::start(&mut client, true)?;
        let mut product_repository = ProductRepository(&mut transaction);
        let result = product_repository.select_by_id(1);
        match result {
            Ok(product) => {
                println!("select_by_id ok: {:?}", product);
            }
            Err(e) => {
                panic!("select_by_id error: {:?}", e);
            }
        }

        // set unexesting id
        let result = product_repository.select_by_id(100);
        match result {
            Ok(_) => {
                panic!("select_by_id error: not found");
            }
            Err(e) => {
                println!("select_by_id ok: {:?}", e);
            }
        }

        Ok(())
    }
}
