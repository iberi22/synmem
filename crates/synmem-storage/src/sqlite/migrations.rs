//! Database migrations

use rusqlite::Connection;

/// Initialize the database schema
pub fn init_schema(conn: &Connection) -> rusqlite::Result<()> {
    // Create sessions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_active_at TEXT NOT NULL,
            is_active INTEGER NOT NULL DEFAULT 1,
            item_count INTEGER NOT NULL DEFAULT 0,
            metadata TEXT
        )",
        [],
    )?;

    // Create memories table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS memories (
            id TEXT PRIMARY KEY,
            content_type TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            source_url TEXT,
            source TEXT NOT NULL,
            tags TEXT,
            session_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            metadata TEXT,
            FOREIGN KEY (session_id) REFERENCES sessions(id)
        )",
        [],
    )?;

    // Create FTS5 virtual table for full-text search on memories
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS memories_fts USING fts5(
            title,
            content,
            source,
            tags,
            content=memories,
            content_rowid=rowid
        )",
        [],
    )?;

    // Create triggers to keep FTS index in sync
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS memories_ai AFTER INSERT ON memories BEGIN
            INSERT INTO memories_fts(rowid, title, content, source, tags)
            VALUES (NEW.rowid, NEW.title, NEW.content, NEW.source, NEW.tags);
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS memories_ad AFTER DELETE ON memories BEGIN
            INSERT INTO memories_fts(memories_fts, rowid, title, content, source, tags)
            VALUES ('delete', OLD.rowid, OLD.title, OLD.content, OLD.source, OLD.tags);
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS memories_au AFTER UPDATE ON memories BEGIN
            INSERT INTO memories_fts(memories_fts, rowid, title, content, source, tags)
            VALUES ('delete', OLD.rowid, OLD.title, OLD.content, OLD.source, OLD.tags);
            INSERT INTO memories_fts(rowid, title, content, source, tags)
            VALUES (NEW.rowid, NEW.title, NEW.content, NEW.source, NEW.tags);
        END",
        [],
    )?;

    // Create saved_contexts table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS saved_contexts (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            content TEXT NOT NULL,
            tags TEXT,
            created_at TEXT NOT NULL,
            session_id TEXT,
            metadata TEXT,
            FOREIGN KEY (session_id) REFERENCES sessions(id)
        )",
        [],
    )?;

    // Create chat_contexts table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chat_contexts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            source TEXT NOT NULL,
            messages TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            url TEXT,
            session_id TEXT,
            FOREIGN KEY (session_id) REFERENCES sessions(id)
        )",
        [],
    )?;

    // Create indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_memories_session ON memories(session_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_memories_created ON memories(created_at DESC)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_memories_content_type ON memories(content_type)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_memories_source ON memories(source)",
        [],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_schema() {
        let conn = Connection::open_in_memory().unwrap();
        init_schema(&conn).unwrap();

        // Verify tables exist
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='memories'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }
}
