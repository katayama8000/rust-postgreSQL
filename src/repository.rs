use anyhow::Result;

pub trait Repository<T, PK, UPD> {
    fn select_all(&mut self) -> Result<Vec<T>>;
    fn select_by_id(&mut self, id: PK) -> Result<T>;
    fn insert(&mut self, row: T) -> Result<UPD>;
    fn update_by_id(&mut self, id: PK) -> Result<UPD>;
    fn delete_by_id(&mut self, id: PK) -> Result<UPD>;
}
