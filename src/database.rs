use sled::{Db, Error};
use serde::{Serialize, Deserialize};
use std::path::Path;

/// Инициализация базы данных
pub fn init_db<P: AsRef<Path>>(path: P) -> Result<Db, Error> {
    Db::open(path)
}

/// Запись данных в базу
pub fn write_db<T: Serialize>(
    db: &Db,
    key: &[u8],
    value: &T
) -> Result<(), Error> {
    let serialized = bincode::serialize(value)?;
    db.insert(key, serialized)?;
    Ok(())
}

/// Чтение данных из базы
pub fn read_db<T: DeserializeOwned>(
    db: &Db,
    key: &[u8]
) -> Result<Option<T>, Error> {
    if let Some(bytes) = db.get(key)? {
        Ok(Some(bincode::deserialize(&bytes)?))
    } else {
        Ok(None)
    }
}
