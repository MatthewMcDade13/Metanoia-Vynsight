// use core::slice::SlicePattern;
use std::{path::{PathBuf, Path}, fmt::{Display, Write}};
use std::io::{Error, ErrorKind};

use actix::{Actor, Context};
use lmdb::{Environment, EnvironmentFlags, DatabaseFlags, WriteFlags, RoTransaction, RwTransaction, Transaction, Database};
use serde::{Serialize, Deserialize};

use crate::{common::{DynResult, from_cbor, into_cbor}, error::db::TransactionError};

pub enum TransactionHandle<'a> {
    Rw(RwTransaction<'a>),
    Ro(RoTransaction<'a>),
}


impl<'a> TransactionHandle<'a> {
    pub fn put(&mut self, db: &Database, key: &str, value: &[u8]) -> DynResult<()> {
        match self {
            TransactionHandle::Rw(txn) => {
                txn.put(*db, &key.as_bytes(), &value, WriteFlags::empty())?;
                Ok(())
            },
            TransactionHandle::Ro(_) => { 
                let err = TransactionError("Tried to write to a ReadOnly Transaction".into());
                Err(Box::new(err))
             },
        }
    }

    pub fn get(&self, db: &Database, key: &str) -> DynResult<&[u8]> {
        match self {
            TransactionHandle::Rw(txn) => {
                let result = txn.get(*db, &key.as_bytes())?;
                Ok(result)
            },
            TransactionHandle::Ro(txn) => {
                let result = txn.get(*db, &key.as_bytes())?;
                Ok(result)
            },
        }
    }

    pub fn commit(self) -> DynResult<()> {
        match self {
            TransactionHandle::Rw(txn) => { txn.commit()?; Ok(()) },
            TransactionHandle::Ro(txn) => { txn.commit()?; Ok(()) },
        }
    }
}

pub struct MetaDB {
    
    env: lmdb::Environment,
    db: lmdb::Database,

}

 
impl MetaDB {

    const DB_DEFAULT_DIR: &'static str = "./data/";

    pub fn new(dbname: &str) -> lmdb::Result<Self> {
        Self::with_dir(dbname, Self::DB_DEFAULT_DIR)
    }

    pub fn with_dir(dbname: &str, db_dir: &str) -> lmdb::Result<Self> {
        let mut db_path: PathBuf = PathBuf::from(db_dir);
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

    pub fn begin_rw_txn(&self) -> DynResult<TransactionHandle> {
        use TransactionHandle::Rw;
        let txn = Rw(self.env.begin_rw_txn()?);
        Ok(txn)
    }


    pub fn begin_ro_txn(&self) -> DynResult<TransactionHandle> {
        use TransactionHandle::Ro;
        let txn = Ro(self.env.begin_ro_txn()?);
        Ok(txn)
    }

    pub fn txn_write<'a, T: 'a>(&self, txn: &mut TransactionHandle, key: &str, value: &T) -> DynResult<()> 
     where T: Serialize + Deserialize<'a> {
        let value_buffer = into_cbor(value)?;
        let value_buffer = value_buffer.as_slice();

        txn.put(&self.db, key, &value_buffer)?;
        Ok(())
    }

    pub fn txn_read<'a, T: 'a>(&self, txn: &TransactionHandle, key: &str) -> DynResult<T> 
     where T: Serialize + Deserialize<'a> {
        let value_bytes = txn.get(&self.db, key)?;

        let value = from_cbor::<T>(value_bytes)?;

        Ok(value)
    }

    pub fn write<'a, T: 'a>(&self, key: &str, value: &T) -> DynResult<()>
        where T: Serialize + Deserialize<'a> {
        
        let mut txn = self.begin_rw_txn()?;
        self.txn_write(&mut txn, key, value)?;
        txn.commit()?;

        Ok(())/*  */
    }

    pub fn read<'b, T: 'b>(&self, key: &str) -> DynResult<T>
        where T: Deserialize<'b> {

        let txn = self.begin_ro_txn()?;
        let value_bytes = self.txn_read(&txn, key)?;
        txn.commit()?;

        let value = from_cbor(value_bytes)?;

        Ok(value)
    }
    
}

impl Actor for MetaDB where Self: Sized + Unpin + 'static {
    type Context = Context<Self>;

}

