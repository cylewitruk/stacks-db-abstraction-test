use rusqlite::Connection;

use crate::db::{
    DbConnection, DbConnectionGuard, DbError, DbTransaction, DbTransactionGuard, Result,
    TransactionalDb,
};

pub struct SQLiteDbImpl {
    pub params: SQLiteDbParams,
    pub conn: Connection,
}

#[derive(Clone)]
pub struct SQLiteDbParams {
    pub uri: String,
}

impl DbConnection for SQLiteDbImpl {
    type Params = SQLiteDbParams;
    fn establish(params: SQLiteDbParams) -> Result<DbConnectionGuard<Self>>
    where
        Self: DbConnection,
    {
        let conn =
            Connection::open(params.uri.clone()).map_err(|e| DbError::Connection(e.to_string()))?;

        let db = SQLiteDbImpl {
            params: params.clone(),
            conn,
        };

        Ok(DbConnectionGuard::new(db))
    }
}

impl TransactionalDb for SQLiteDbImpl {
    type TxType<'conn> = SQLiteDbTransactionImpl<'conn> where Self: 'conn;

    fn transaction<'tx>(&mut self) -> Result<DbTransactionGuard<Self::TxType<'_>>> {
        let inner_tx = self
            .conn
            .transaction()
            .expect("failed to begin transaction");

        let tx = SQLiteDbTransactionImpl { tx: inner_tx };

        Ok(DbTransactionGuard::new(tx))
    }
}

pub struct SQLiteDbTransactionImpl<'conn> {
    tx: rusqlite::Transaction<'conn>,
}

impl<'conn> DbTransaction<'conn> for SQLiteDbTransactionImpl<'conn> {
    fn commit(self) -> Result<()> {
        self.tx.commit().map_err(|e| DbError::Commit(e.to_string()))
    }

    fn rollback(self) -> Result<()> {
        self.tx
            .rollback()
            .map_err(|e| DbError::Rollback(e.to_string()))
    }
}
