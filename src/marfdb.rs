use std::{cell::RefCell, rc::Rc};

use crate::db::{TransactionalDb, FromDbConnection, DbConnectionGuard, Result};

pub trait MarfTrieDb {
    fn do_something_else_immut(&self);
    fn do_something_mut(&mut self);
}

pub struct MarfTrieDbImpl<DB> 
where
    DB: TransactionalDb
{
    conn: Rc<RefCell<DB>>
}

impl<DB> FromDbConnection<DB> for MarfTrieDbImpl<DB>
where
    DB: TransactionalDb
{
    fn from_db(db: &DbConnectionGuard<DB>) -> Result<Self> where Self: Sized {
        Ok(Self {
            conn: db.db.clone()
        })
    }
}

impl<DB> MarfTrieDb for MarfTrieDbImpl<DB> 
where
    DB: TransactionalDb
{
    fn do_something_else_immut(&self) {
        let mut conn = self.conn.borrow_mut();
        let tx = conn.transaction().unwrap();

        todo!()
    }

    fn do_something_mut(&mut self) {
        todo!()
    }
}