use std::{rc::Rc, cell::RefCell, ops::Deref, marker::PhantomData};

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
    pub db: Rc<RefCell<DB>>,
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