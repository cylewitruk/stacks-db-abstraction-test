use std::{cell::RefCell, rc::Rc};

use crate::db::{DbConnectionGuard, DbTransaction, FromDbConnection, Result, TransactionalDb};

/// Trait which defines, as an example, "MARF" database operations.
pub trait MarfTrieDb {
    fn do_something_else_immut(&self);
    fn do_something_mut(&mut self);
}

/// An example implementation of a DB struct.
pub struct MarfTrieDbImpl<DB>
where
    DB: TransactionalDb,
{
    conn: Rc<RefCell<DB>>,
}

/// Implementation of [FromDbConnection] which allows for the creation of this
/// database implementation from a [DbConnectionGuard].
impl<DB> FromDbConnection<DB> for MarfTrieDbImpl<DB>
where
    DB: TransactionalDb,
{
    fn from_db(db: &DbConnectionGuard<DB>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            conn: db.db.clone(),
        })
    }
}

/// Implementation of [MarfTrieDb] for [MarfTrieDbImpl].
impl<DB> MarfTrieDb for MarfTrieDbImpl<DB>
where
    DB: TransactionalDb,
{
    fn do_something_else_immut(&self) {
        let mut conn = self.conn.borrow_mut();
        let tx = conn.transaction().unwrap();

        eprintln!("marfdb do_something_else_immut");

        tx.commit().unwrap();
    }

    fn do_something_mut(&mut self) {
        eprintln!("marfdb do_something_mut");
    }
}
