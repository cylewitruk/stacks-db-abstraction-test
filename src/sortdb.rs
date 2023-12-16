use std::cell::RefCell;
use std::rc::Rc;

use crate::db::{DbConnectionGuard, DbTransaction, FromDbConnection, Result, TransactionalDb};

pub trait SortitionDb {
    fn do_some_mut_thing(&mut self);
    fn do_some_immut_thing(&self);
}

pub struct SortitionDbImpl<DB>
where
    DB: TransactionalDb,
{
    conn: Rc<RefCell<DB>>,
}

impl<DB> FromDbConnection<DB> for SortitionDbImpl<DB>
where
    DB: TransactionalDb,
{
    fn from_db(db: &DbConnectionGuard<DB>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            conn: Rc::clone(db),
        })
    }
}

impl<DB> SortitionDb for SortitionDbImpl<DB>
where
    DB: TransactionalDb,
{
    fn do_some_mut_thing(&mut self) {
        let mut conn = self.conn.borrow_mut();
        let tx = conn.transaction().unwrap();

        eprintln!("sortdb: do_some_mut_thing");

        tx.commit().unwrap();
    }

    fn do_some_immut_thing(&self) {
        eprintln!("sortdb: do_some_immut_thing");
    }
}
