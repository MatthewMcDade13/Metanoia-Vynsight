use std::path::PathBuf;

use lmdb::Environment;

pub struct MetaDB {
    
    env: lmdb::Environment,
    db: lmdb::Database,

}

 
impl MetaDB {
    pub fn new_open(path: &str) -> std::io::Result<Self> {
        let path_buf = PathBuf::from(path);
        let full_path = path_buf.canonicalize().unwrap();
        let db_path = full_path.as_path();
        let env = Environment::new().open(db_path).unwrap();
    
        let db = env.open_db(None).unwrap();
        Ok(Self { env, db })
    }
}
