use std::{ops::Deref, marker::PhantomData, cell::RefCell, rc::Rc};

use marfdb::{MarfTrieDbImpl, MarfTrieDb};
use sortdb::{SortitionDb, SortitionDbImpl};
use sqlite::{SQLiteDbImpl, SQLiteDbParams};

mod sortdb;
mod marfdb;
mod sqlite;

/// Type to simplify error handling in the DB impls.
pub type Result<T> = std::result::Result<T, DbError>;

/// Base trait for database implementations which can be opened/connected to.
/// This trait is used to abstract over the underlying database implementation, 
/// and provides a single method to establish a connection to the database.
pub trait DbConnection: Sized {
    type Params: Clone;
    fn establish(params: Self::Params) -> Result<DbConnectionGuard<Self>>;
}

/// Wrapper around a database connection which stores the connection in an
/// Rc<RefCell<>>. This is used as a return type for the [DbConnection::establish]
/// method, and is used to abstract over the [DbConnection] implementation providing
/// for multiple ownership. This is useful for when multiple databases are "marf-ed",
pub struct DbConnectionGuard<DB>
where
    DB: DbConnection
{
    db: Rc<RefCell<DB>>,
}

/// Implementation of [DbConnectionGuard] which wraps a [DbConnection] in a
/// [DbConnectionGuard] to allow for the passing of a single DB implementation
/// into multiple database implementations, such as a SortitionDB + MarfTrieDB.
impl<DB> DbConnectionGuard<DB>
where
    DB: DbConnection 
{
    pub fn new(db: DB) -> Self {
        Self {
            db: Rc::new(RefCell::new(db))
        }
    }
}

/// Implementation of [Deref] for [DbConnectionGuard] which helps keep the code clean
/// from Rc<RefCell<>> boilerplate.
impl<DB> Deref for DbConnectionGuard<DB>
where
    DB: DbConnection
{
    type Target = Rc<RefCell<DB>>;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

/// Trait for database implementations which support transactions.
pub trait TransactionalDb
where
    Self: DbConnection
{
    type TxType<'conn>: DbTransaction<'conn> where Self: 'conn;
    
    fn transaction<'conn, 'tx>(
        &'conn mut self
    ) -> Result<DbTransactionGuard<Self::TxType<'conn>>>;
}

/// Trait for database transactions.
pub trait DbTransaction<'conn> {
    fn commit(self) -> Result<()>;
    fn rollback(self) -> Result<()>;
}

/// Wrapper around a database transaction which stores the transaction.
pub struct DbTransactionGuard<'conn, TxType>
where
    TxType: DbTransaction<'conn>,
 {
    tx: TxType,
    _phantom: PhantomData<&'conn ()>,
}

/// Implementation of [DbTransactionGuard] which wraps a [DbTransaction] in a
/// [DbTransactionGuard] to allow for the passing of a single DB transaction.
impl<'conn, TxType> DbTransactionGuard<'conn, TxType>
where
    TxType: DbTransaction<'conn>
{
    pub fn new(tx: TxType) -> Self {
        Self {
            tx,
            _phantom: PhantomData,
        }
    }
}

impl<'conn, TxType> Deref for DbTransactionGuard<'conn, TxType>
where
    TxType: DbTransaction<'conn>,
{
    type Target = TxType;

    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}

impl<'conn, TxType> DbTransaction<'conn> for DbTransactionGuard<'conn, TxType> 
where
    TxType: DbTransaction<'conn>,
{
    fn commit(self) -> Result<()> {
        self.tx.commit()
    }

    fn rollback(self) -> Result<()> {
        self.tx.rollback()
    }
}

pub trait FromDbConnection<DB>
where
    DB: DbConnection
{
    fn from_db(db: &DbConnectionGuard<DB>) -> Result<Self> where Self: Sized;
}

#[derive(Debug)]
pub enum DbError {
    Database(String),
    Commit(String),
    Rollback(String),
    Transaction(String),
    Connection(String),
    Other(String),
}

/*impl<'conn> SQLiteDbImpl<'conn> {
    pub fn from_connection(conn: &'conn mut Connection) -> Result<DbConnectionGuard<Self>> {
        Ok(DbConnectionGuard::new(Self { conn }))
    }
}*/

fn main() -> Result<()> {
    // This code demonstrates an internal wrapper for an sqlite db which implements internal 
    // traits [DbConnection] + [TransactionalDb].
    //
    // This uses the Rc<RefCell<>> pattern to help with keeping the code + 
    // lifetimes simpler. Synchronization logic is then responsible and isolated to
    // the impl of the [TransactionalDb] + [DbConnection] traits.
    //
    // This is due to the fact that multiple databases are "marf-ed", i.e. use the
    // MARF as an extension to purpose-specific database. Instead of passing around
    // a raw [`Connection`] object with complex lifetimes, we can embrace multiple
    // ownership. In addition, by providing an abstraction over raw DB's makes it
    // easier to swap out the implementation of some or all of the DB's in the future.

    // Define our connection parameters. [SQLiteDbParams] implements [DbConnection::Params].
    let connection_params = SQLiteDbParams {
        uri: ":memory:".to_string()
    };

    // Establish the connection to the database. [SQLiteDbImpl] implements [DbConnection].
    let db = SQLiteDbImpl::establish(connection_params)?;
    
    // Create our sortition DB from the DB connection. [SortitionDbImpl] implements
    // [SortitionDb] and [FromDbConnection].
    let mut sortdb = 
        SortitionDbImpl::from_db(&db)?;

    // Create our marf trie DB from the DB connection. [MarfTrieDbImpl] implements
    // [MarfTrieDb] and [FromDbConnection].
    let mut marf_trie_db =
        MarfTrieDbImpl::from_db(&db)?;

    // Do some stuff with the different DB's. Note that the same underlying [Connection]
    // is used, with synchronousy guarded by the DB impl via Rc<RefCell<>>.
    sortdb.do_some_mut_thing();
    marf_trie_db.do_something_else_immut();
    sortdb.do_some_immut_thing();
    marf_trie_db.do_something_mut();

    Ok(())
}