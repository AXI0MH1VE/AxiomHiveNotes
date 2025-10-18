-- Notes table with support for encryption, folders, branding, and soft delete
CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL DEFAULT '',
    content_md TEXT,
    content_json TEXT,
    ydoc BLOB,
    folder TEXT DEFAULT '',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reminder_at DATETIME,
    pinned INTEGER NOT NULL DEFAULT 0,
    archived INTEGER NOT NULL DEFAULT 0,
    deleted_at DATETIME,
    brand TEXT NOT NULL DEFAULT '@AxiomHive',
    backlinks_json TEXT DEFAULT '[]',
    tags_cache TEXT DEFAULT ''
);

-- Tags table
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Note-Tags junction table
CREATE TABLE note_tags (
    note_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (note_id, tag_id),
    FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Settings table for app configuration
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- FTS5 virtual table for full-text search with Porter tokenizer
CREATE VIRTUAL TABLE notes_fts USING fts5(
    title,
    content,
    tags,
    content=notes,
    content_rowid=id,
    tokenize='porter'
);

-- Trigger to keep FTS index in sync on INSERT
CREATE TRIGGER notes_fts_insert AFTER INSERT ON notes BEGIN
    INSERT INTO notes_fts(rowid, title, content, tags)
    VALUES (
        new.id,
        new.title,
        COALESCE(new.content_md, ''),
        COALESCE(new.tags_cache, '')
    );
END;

-- Trigger to keep FTS index in sync on UPDATE
CREATE TRIGGER notes_fts_update AFTER UPDATE ON notes BEGIN
    UPDATE notes_fts
    SET title = new.title,
        content = COALESCE(new.content_md, ''),
        tags = COALESCE(new.tags_cache, '')
    WHERE rowid = new.id;
END;

-- Trigger to keep FTS index in sync on DELETE
CREATE TRIGGER notes_fts_delete AFTER DELETE ON notes BEGIN
    DELETE FROM notes_fts WHERE rowid = old.id;
END;

-- Indices for common queries
CREATE INDEX idx_notes_created_at ON notes(created_at DESC);
CREATE INDEX idx_notes_updated_at ON notes(updated_at DESC);
CREATE INDEX idx_notes_folder ON notes(folder);
CREATE INDEX idx_notes_pinned ON notes(pinned);
CREATE INDEX idx_notes_archived ON notes(archived);
CREATE INDEX idx_notes_deleted_at ON notes(deleted_at);
CREATE INDEX idx_notes_reminder_at ON notes(reminder_at);
CREATE INDEX idx_notes_brand ON notes(brand);

-- Insert default settings
INSERT INTO settings (key, value) VALUES ('theme', 'dark');
INSERT INTO settings (key, value) VALUES ('encryption_enabled', '0');
INSERT INTO settings (key, value) VALUES ('telemetry_enabled', '0');
INSERT INTO settings (key, value) VALUES ('brand', '@AxiomHive');
