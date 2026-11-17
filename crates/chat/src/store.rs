// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Chat's own state, kept beside the sessions the host stores: which chats
//! are pinned. Sessions, titles and history live in the host; only what
//! Chat alone means is stored here.

use futures::FutureExt as _;
use futures::future::LocalBoxFuture;
use std::cell::RefCell;
use std::collections::BTreeSet;

/// Chat's schema, declared in its manifest.
pub const SCHEMA: &str = "chat";

/// Statements the SQL store sends, one per call.
pub mod sql {
    /// Creates the pins table once.
    pub const CREATE: &str = "create table if not exists pins (session text primary key, pinned_at timestamptz not null default now())";
    /// The pinned sessions, oldest pin first.
    pub const LOAD: &str = "select session from pins order by pinned_at";
    /// Pins a session.
    pub const PIN: &str = "insert into pins (session) values ($1) on conflict (session) do nothing";
    /// Unpins a session.
    pub const UNPIN: &str = "delete from pins where session = $1";
}

/// A store's answer; the error is readable.
pub type StoreFuture<T> = LocalBoxFuture<'static, Result<T, String>>;

/// Where Chat keeps its own state.
pub trait Store {
    /// The pinned sessions, in pin order.
    fn pins(&self) -> StoreFuture<Vec<String>>;
    /// Pin or unpin a session.
    fn set_pinned(&self, session: &str, pinned: bool) -> StoreFuture<()>;
}

/// A store in memory, for tests and the dev window.
#[derive(Default)]
pub struct MemoryStore {
    pins: RefCell<Vec<String>>,
    /// Why writes fail, while they do.
    refusing: RefCell<Option<String>>,
}

impl MemoryStore {
    /// Make every write fail with `reason` until [`Self::accept_writes`].
    pub fn refuse_writes(&self, reason: &str) {
        *self.refusing.borrow_mut() = Some(reason.to_owned());
    }

    /// Let writes through again.
    pub fn accept_writes(&self) {
        *self.refusing.borrow_mut() = None;
    }
}

impl Store for MemoryStore {
    fn pins(&self) -> StoreFuture<Vec<String>> {
        futures::future::ready(Ok(self.pins.borrow().clone())).boxed_local()
    }

    fn set_pinned(&self, session: &str, pinned: bool) -> StoreFuture<()> {
        if let Some(reason) = self.refusing.borrow().clone() {
            return futures::future::ready(Err(reason)).boxed_local();
        }
        let mut pins = self.pins.borrow_mut();
        pins.retain(|pin| pin != session);
        if pinned {
            pins.push(session.to_owned());
        }
        futures::future::ready(Ok(())).boxed_local()
    }
}

/// The store on the SDK's `sql` interface, in Chat's own schema.
pub struct SqlStore {
    sql: isonomic_client::Sql,
}

impl SqlStore {
    /// A store over the app's scoped `sql` client.
    pub fn new(sql: isonomic_client::Sql) -> Self {
        Self { sql }
    }
}

impl Store for SqlStore {
    fn pins(&self) -> StoreFuture<Vec<String>> {
        let sql = self.sql.clone();
        async move {
            sql.execute(sql::CREATE, Vec::new())
                .await
                .map_err(|e| e.to_string())?;
            let rows = sql
                .query(sql::LOAD, Vec::new())
                .await
                .map_err(|e| e.to_string())?;
            let mut seen = BTreeSet::new();
            Ok((0..rows.rows.len())
                .filter_map(|row| match rows.get(row, "session") {
                    Some(isonomic_proto::SqlValue::Text(id)) => Some(id.clone()),
                    _ => None,
                })
                .filter(|id| seen.insert(id.clone()))
                .collect())
        }
        .boxed_local()
    }

    fn set_pinned(&self, session: &str, pinned: bool) -> StoreFuture<()> {
        let (sql, session) = (self.sql.clone(), session.to_owned());
        async move {
            let statement = if pinned { sql::PIN } else { sql::UNPIN };
            sql.execute(statement, vec![isonomic_proto::SqlValue::Text(session)])
                .await
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
        .boxed_local()
    }
}
