use std::rc::Rc;
use std::cell::RefCell;

use crate::{
    TransactionalDb, Result, FromDbConnection, DbConnectionGuard
};

pub trait SortitionDb
{
    fn do_some_mut_thing(&mut self);
    fn do_some_immut_thing(&self);
}

pub struct SortitionDbImpl<DB> 
where
    DB: TransactionalDb
{
    conn: Rc<RefCell<DB>>
}

impl<DB> FromDbConnection<DB> for SortitionDbImpl<DB>
where
    DB: TransactionalDb
{
    fn from_db(db: &DbConnectionGuard<DB>) -> Result<Self> where Self: Sized {
        Ok(Self {
            conn: Rc::clone(&db)
        })
    }
}

impl<DB> SortitionDb for SortitionDbImpl<DB>
where
    DB: TransactionalDb
{
    fn do_some_mut_thing(&mut self) {
        let mut conn = self.conn.borrow_mut();
        let tx = conn.transaction().unwrap();

        todo!()
    }

    fn do_some_immut_thing(&self) {
        todo!()
    }
}

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

pub trait DbInit<Params> {
    fn init(params: Params) -> Result<Rc<RefCell<Self>>> where Self: Sized;
}