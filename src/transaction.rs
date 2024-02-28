use anyhow::Result;
use postgres::Transaction;

pub struct TransactionUtil;
impl TransactionUtil {
    pub fn start<'a>(client: &'a mut postgres::Client, mode: bool) -> Result<Transaction<'a>> {
        let transaction = client.build_transaction().read_only(mode).start()?;
        Ok(transaction)
    }

    pub fn commit(transaction: Transaction) -> Result<()> {
        transaction.commit()?;
        Ok(())
    }

    pub fn rollback(transaction: Transaction) -> Result<()> {
        transaction.rollback()?;
        Ok(())
    }
}
