use db::{DbConnection, FromDbConnection, Result};
use marfdb::{MarfTrieDb, MarfTrieDbImpl};
use sortdb::{SortitionDb, SortitionDbImpl};
use sqlite::{SQLiteDbImpl, SQLiteDbParams};

mod db;
mod marfdb;
mod sortdb;
mod sqlite;

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
        uri: ":memory:".to_string(),
    };

    // Establish the connection to the database. [SQLiteDbImpl] implements [DbConnection].
    let db = SQLiteDbImpl::establish(connection_params)?;

    // Create our sortition DB from the DB connection. [SortitionDbImpl] implements
    // [SortitionDb] and [FromDbConnection].
    let mut sortdb = SortitionDbImpl::from_db(&db)?;

    // Create our marf trie DB from the DB connection. [MarfTrieDbImpl] implements
    // [MarfTrieDb] and [FromDbConnection].
    let mut marf_trie_db = MarfTrieDbImpl::from_db(&db)?;

    // Do some stuff with the different DB's. Note that the same underlying [Connection]
    // is used, with synchronousy guarded by the DB impl via Rc<RefCell<>>.
    sortdb.do_some_mut_thing();
    marf_trie_db.do_something_else_immut();
    sortdb.do_some_immut_thing();
    marf_trie_db.do_something_mut();

    Ok(())
}
