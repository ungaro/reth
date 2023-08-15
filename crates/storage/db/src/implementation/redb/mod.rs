use std::{marker::PhantomData, sync::Arc};

use crate::{
    cursor::{DbCursorRO, DbCursorRW},
    database::{Database, DatabaseGAT},
    table::{DupSort, Table, TableImporter},
    transaction::{DbTx, DbTxGAT, DbTxMut, DbTxMutGAT},
};
use reth_interfaces::db::DatabaseError;

// todo: only possible to get stats via writetransaction????

// todo: move stuff to modules

// todo: cursors operate on tables
// - ro table: ReadOnlyTable<K, V>
// - ro multimap: ReadOnlyMultimapTable<K, V>
// - rw table: Table<K, V>
// - rw multimap: MultimapTable<K, V>
pub struct Cursor<'tx, T> {
    _todo: &'tx T,
}

impl<'tx, T: Table> DbCursorRO<'tx, T> for Cursor<'tx, T> {
    fn first(&mut self) -> crate::common::PairResult<T> {
        todo!()
    }

    fn seek_exact(&mut self, key: <T as Table>::Key) -> crate::common::PairResult<T> {
        todo!()
    }

    fn seek(&mut self, key: <T as Table>::Key) -> crate::common::PairResult<T> {
        todo!()
    }

    fn next(&mut self) -> crate::common::PairResult<T> {
        todo!()
    }

    fn prev(&mut self) -> crate::common::PairResult<T> {
        todo!()
    }

    fn last(&mut self) -> crate::common::PairResult<T> {
        todo!()
    }

    fn current(&mut self) -> crate::common::PairResult<T> {
        todo!()
    }

    fn walk<'cursor>(
        &'cursor mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Result<crate::cursor::Walker<'cursor, 'tx, T, Self>, DatabaseError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn walk_range<'cursor>(
        &'cursor mut self,
        range: impl std::ops::RangeBounds<<T as Table>::Key>,
    ) -> Result<crate::cursor::RangeWalker<'cursor, 'tx, T, Self>, DatabaseError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn walk_back<'cursor>(
        &'cursor mut self,
        start_key: Option<<T as Table>::Key>,
    ) -> Result<crate::cursor::ReverseWalker<'cursor, 'tx, T, Self>, DatabaseError>
    where
        Self: Sized,
    {
        todo!()
    }
}
impl<'tx, T: Table> DbCursorRW<'tx, T> for Cursor<'tx, T> {
    fn upsert(
        &mut self,
        key: <T as Table>::Key,
        value: <T as Table>::Value,
    ) -> Result<(), DatabaseError> {
        todo!()
    }

    fn insert(
        &mut self,
        key: <T as Table>::Key,
        value: <T as Table>::Value,
    ) -> Result<(), DatabaseError> {
        todo!()
    }

    fn append(
        &mut self,
        key: <T as Table>::Key,
        value: <T as Table>::Value,
    ) -> Result<(), DatabaseError> {
        todo!()
    }

    fn delete_current(&mut self) -> Result<(), DatabaseError> {
        todo!()
    }
}

// todo: verify that the tx is *aborted* on drop (as is the case w mdbx)
pub struct Wt<'a>(&'a redb::WriteTransaction<'a>);

impl<'a> std::fmt::Debug for Wt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Wt").finish()
    }
}

impl<'a> DbTxGAT<'a> for Wt<'_> {
    type Cursor<T: Table> = Cursor<'a, T>;
    type DupCursor<T: DupSort> = Cursor<'a, T>;
}

impl<'a> DbTxMutGAT<'a> for Wt<'_> {
    type CursorMut<T: Table> = Cursor<'a, T>;
    type DupCursorMut<T: DupSort> = Cursor<'a, T>;
}

// todo: autoimpl
impl<'a> TableImporter<'a> for Wt<'a> {}

impl<'a> DbTx<'a> for Wt<'a> {
    fn get<T: Table>(&self, key: T::Key) -> Result<Option<T::Value>, DatabaseError> {
        todo!()
    }

    fn commit(self) -> Result<bool, DatabaseError> {
        // noop
    }

    fn drop(self) {
        // noop
    }

    fn cursor_read<T: Table>(&self) -> Result<<Self as DbTxGAT<'_>>::Cursor<T>, DatabaseError> {
        todo!()
    }

    fn cursor_dup_read<T: DupSort>(
        &self,
    ) -> Result<<Self as DbTxGAT<'_>>::DupCursor<T>, DatabaseError> {
        todo!()
    }

    fn entries<T: Table>(&self) -> Result<usize, DatabaseError> {
        todo!()
    }
}

impl<'a> DbTxMut<'_> for Wt<'a> {
    fn put<T: Table>(&self, key: T::Key, value: T::Value) -> Result<(), DatabaseError> {
        todo!()
    }

    fn delete<T: Table>(
        &self,
        key: T::Key,
        value: Option<T::Value>,
    ) -> Result<bool, DatabaseError> {
        todo!()
    }

    fn clear<T: Table>(&self) -> Result<(), DatabaseError> {
        todo!()
    }

    fn cursor_write<T: Table>(
        &self,
    ) -> Result<<Self as DbTxMutGAT<'_>>::CursorMut<T>, DatabaseError> {
        todo!()
    }

    fn cursor_dup_write<T: DupSort>(
        &self,
    ) -> Result<<Self as DbTxMutGAT<'_>>::DupCursorMut<T>, DatabaseError> {
        todo!()
    }
}

///

pub struct Rt<'a>(redb::ReadTransaction<'a>);

impl<'a> std::fmt::Debug for Rt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rt").finish()
    }
}

impl<'a> DbTxGAT<'a> for Rt<'_> {
    type Cursor<T: Table> = Cursor<'a, T>;
    type DupCursor<T: DupSort> = Cursor<'a, T>;
}

impl<'a> DbTx<'a> for Rt<'a> {
    fn get<T: Table>(&self, key: T::Key) -> Result<Option<T::Value>, DatabaseError> {
        // todo: is it ok to cache table handles? (check type size and side effects)
        todo!()
    }

    fn commit(self) -> Result<bool, DatabaseError> {
        // todo err
        self.commit().unwrap();
        // todo: what is the bool
        Ok(true)
    }

    fn drop(self) {
        // todo: this can fail - why?
        self.abort().unwrap();
    }

    fn cursor_read<T: Table>(&self) -> Result<<Self as DbTxGAT<'_>>::Cursor<T>, DatabaseError> {
        todo!()
    }

    fn cursor_dup_read<T: DupSort>(
        &self,
    ) -> Result<<Self as DbTxGAT<'_>>::DupCursor<T>, DatabaseError> {
        todo!()
    }

    fn entries<T: Table>(&self) -> Result<usize, DatabaseError> {
        todo!()
    }
}

// todo: can we simplify this gat shenanigans if we don't use mdbx anymore? :thinking:
impl<'a> DatabaseGAT<'a> for redb::Database {
    type TX = Rt<'a>;
    type TXMut = Wt<'a>;
}

impl Database for redb::Database {
    fn tx(&self) -> Result<<Self as DatabaseGAT<'_>>::TX, DatabaseError> {
        // todo: fix err
        Ok(Rt(self.begin_read().map_err(|_| DatabaseError::DecodeError)?))
    }

    fn tx_mut(&self) -> Result<<Self as DatabaseGAT<'_>>::TXMut, DatabaseError> {
        // todo: fix err
        Ok(Wt(&self.begin_write().map_err(|_| DatabaseError::DecodeError)?))
    }
}
