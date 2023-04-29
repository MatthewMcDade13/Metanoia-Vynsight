use std::{path::{PathBuf, Path}, fmt::Display};

use lmdb::{Environment, EnvironmentFlags, DatabaseFlags, WriteFlags, RoTransaction, RwTransaction, Transaction};
use serde::{Serialize, Deserialize};

use crate::common::{DynResult, from_cbor, into_cbor};

pub struct MetaDB {
    
    env: lmdb::Environment,
    db: lmdb::Database,

}

 
impl MetaDB {
    pub fn new(dbname: &str) -> lmdb::Result<Self> {
        let mut db_path: PathBuf = PathBuf::from("./data/");
        db_path.push(dbname);

        let env = {
            let env_result = Environment::new()
                .set_max_dbs(1)
                .set_flags(EnvironmentFlags::NO_TLS)
                .open(db_path.as_path());

            match env_result {
                Ok(env) => env,
                Err(err) => { return Err(err); }
            }
        };

        let db = {
            let db_result = env.create_db(None, DatabaseFlags::default());

            match db_result {
                Ok(db) => db,
                Err(err) => { return Err(err); }
            }
        };

        Ok(Self { env, db })
    }

    pub fn begin_rw_txn(&mut self) -> DynResult<RwTransaction> {
        self.begin_rw_txn()
    }

    pub fn end_rw_txn(txn: RwTransaction) -> DynResult<()> {
        txn.commit()?;
        Ok(())
    }

    pub fn txn_write<'a, T: 'a>(&mut self, txn: &mut RwTransaction, key: &str, value: &T) -> DynResult<()> 
     where T: Serialize + Deserialize<'a> {

        Ok(())
    }

    pub fn write<'a, T: 'a>(&mut self, key: &str, value: &T) -> Result<(), Box<dyn std::error::Error>>
        where T: Serialize + Deserialize<'a> {
        let mut txn = self.env.begin_rw_txn()?;
        let value_buffer = into_cbor(value)?;
        let value_buffer = value_buffer.as_slice();

        let key = key.as_bytes();
        txn.put(self.db, &key, &value_buffer, WriteFlags::empty())?;
        Transaction::commit(txn)?;

        Ok(())
    }

    // pub fn write_many<'a, T: 'a>(&mut self, vals: &Vec<(String, T)>) -> DynResult<()>
    //     where T: Serialize + Deserialize<'a> {
    //         let mut txn = self.env.begin_rw_txn()?;
            
    //     }

    pub fn read<'a, T: 'a>(&self, key: &str) -> Result<T, Box<dyn std::error::Error>>
        where T: Deserialize<'a> {

        let txn = self.env.begin_ro_txn()?;
        let value_bytes = txn.get(self.db, &key.as_bytes())?;

        let value = from_cbor(value_bytes)?;

        txn.commit()?;
        Ok(value)
    }
    
}

type Reason = String;
#[derive(Debug)]
pub struct DBSerializeError(pub Reason);

impl Display for DBSerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DBSerializeError {}
