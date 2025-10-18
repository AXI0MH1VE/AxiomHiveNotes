use crate::db::Database;
use crate::models::{Note, SearchResult, Tag};
use rusqlite::{params, OptionalExtension};
use tauri::State;

#[tauri::command]
pub fn create_note(
    db: State<Database>,
    title: String,
    content_md: Option<String>,
    folder: String,
    brand: String,
) -> Result<Note, String> {
    let conn = db.conn.lock().unwrap();
    
    conn.execute(
        "INSERT INTO notes (title, content_md, folder, brand) VALUES (?1, ?2, ?3, ?4)",
        params![title, content_md, folder, brand],
    )
    .map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    
    get_note_by_id(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_note(db: State<Database>, id: i64) -> Result<Note, String> {
    let conn = db.conn.lock().unwrap();
    get_note_by_id(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_note(
    db: State<Database>,
    id: i64,
    title: Option<String>,
    content_md: Option<String>,
    folder: Option<String>,
    pinned: Option<bool>,
    archived: Option<bool>,
) -> Result<Note, String> {
    let conn = db.conn.lock().unwrap();
    
    if let Some(t) = title {
        conn.execute(
            "UPDATE notes SET title = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![t, id],
        )
        .map_err(|e| e.to_string())?;
    }
    
    if let Some(c) = content_md {
        conn.execute(
            "UPDATE notes SET content_md = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![c, id],
        )
        .map_err(|e| e.to_string())?;
    }
    
    if let Some(f) = folder {
        conn.execute(
            "UPDATE notes SET folder = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![f, id],
        )
        .map_err(|e| e.to_string())?;
    }
    
    if let Some(p) = pinned {
        conn.execute(
            "UPDATE notes SET pinned = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![p as i32, id],
        )
        .map_err(|e| e.to_string())?;
    }
    
    if let Some(a) = archived {
        conn.execute(
            "UPDATE notes SET archived = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![a as i32, id],
        )
        .map_err(|e| e.to_string())?;
    }
    
    get_note_by_id(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_note(db: State<Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    
    conn.execute(
        "UPDATE notes SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?1",
        params![id],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub fn list_notes(
    db: State<Database>,
    folder: Option<String>,
    pinned: Option<bool>,
    archived: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Note>, String> {
    let conn = db.conn.lock().unwrap();
    
    let mut query = "SELECT * FROM notes WHERE deleted_at IS NULL".to_string();
    let mut conditions = Vec::new();
    
    if let Some(f) = folder {
        conditions.push(format!(" AND folder = '{}'", f));
    }
    
    if let Some(p) = pinned {
        conditions.push(format!(" AND pinned = {}", p as i32));
    }
    
    if let Some(a) = archived {
        conditions.push(format!(" AND archived = {}", a as i32));
    }
    
    query.push_str(&conditions.join(""));
    query.push_str(" ORDER BY updated_at DESC");
    
    if let Some(l) = limit {
        query.push_str(&format!(" LIMIT {}", l));
    }
    
    if let Some(o) = offset {
        query.push_str(&format!(" OFFSET {}", o));
    }
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    
    let notes = stmt
        .query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content_md: row.get(2)?,
                content_json: row.get(3)?,
                folder: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                reminder_at: row.get(8)?,
                pinned: row.get::<_, i32>(9)? != 0,
                archived: row.get::<_, i32>(10)? != 0,
                deleted_at: row.get(11)?,
                brand: row.get(12)?,
                backlinks_json: row.get(13)?,
                tags_cache: row.get(14)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(notes)
}

#[tauri::command]
pub fn search_notes(
    db: State<Database>,
    query: String,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<SearchResult>, String> {
    let conn = db.conn.lock().unwrap();
    
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);
    
    let sql = format!(
        "SELECT n.id, n.title, snippet(notes_fts, 1, '[', ']', '...', 12) as snippet, bm25(notes_fts) as rank \
         FROM notes_fts \
         JOIN notes n ON notes_fts.rowid = n.id \
         WHERE notes_fts MATCH ?1 AND n.deleted_at IS NULL \
         ORDER BY rank \
         LIMIT {} OFFSET {}",
        limit, offset
    );
    
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    
    let results = stmt
        .query_map([&query], |row| {
            Ok(SearchResult {
                id: row.get(0)?,
                title: row.get(1)?,
                snippet: row.get(2)?,
                rank: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(results)
}

#[tauri::command]
pub fn create_tag(db: State<Database>, name: String) -> Result<Tag, String> {
    let conn = db.conn.lock().unwrap();
    
    conn.execute("INSERT INTO tags (name) VALUES (?1)", params![name])
        .map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    
    let tag = conn
        .query_row(
            "SELECT id, name, created_at FROM tags WHERE id = ?1",
            params![id],
            |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    
    Ok(tag)
}

#[tauri::command]
pub fn list_tags(db: State<Database>) -> Result<Vec<Tag>, String> {
    let conn = db.conn.lock().unwrap();
    
    let mut stmt = conn
        .prepare("SELECT id, name, created_at FROM tags ORDER BY name")
        .map_err(|e| e.to_string())?;
    
    let tags = stmt
        .query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(tags)
}

#[tauri::command]
pub fn assign_tags(db: State<Database>, note_id: i64, tag_ids: Vec<i64>) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    
    // Clear existing tags
    conn.execute("DELETE FROM note_tags WHERE note_id = ?1", params![note_id])
        .map_err(|e| e.to_string())?;
    
    // Insert new tags
    for tag_id in tag_ids {
        conn.execute(
            "INSERT INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            params![note_id, tag_id],
        )
        .map_err(|e| e.to_string())?;
    }
    
    // Update tags_cache
    let tags: Vec<String> = conn
        .prepare(
            "SELECT t.name FROM tags t \
             JOIN note_tags nt ON t.id = nt.tag_id \
             WHERE nt.note_id = ?1",
        )
        .map_err(|e| e.to_string())?
        .query_map(params![note_id], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    let tags_cache = tags.join(", ");
    
    conn.execute(
        "UPDATE notes SET tags_cache = ?1 WHERE id = ?2",
        params![tags_cache, note_id],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub fn list_folders(db: State<Database>) -> Result<Vec<String>, String> {
    let conn = db.conn.lock().unwrap();
    
    let mut stmt = conn
        .prepare("SELECT DISTINCT folder FROM notes WHERE deleted_at IS NULL AND folder != '' ORDER BY folder")
        .map_err(|e| e.to_string())?;
    
    let folders = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    
    Ok(folders)
}

#[tauri::command]
pub fn get_setting(db: State<Database>, key: String) -> Result<Option<String>, String> {
    let conn = db.conn.lock().unwrap();
    
    let result = conn
        .query_row("SELECT value FROM settings WHERE key = ?1", params![key], |row| {
            row.get(0)
        })
        .optional()
        .map_err(|e| e.to_string())?;
    
    Ok(result)
}

#[tauri::command]
pub fn set_setting(db: State<Database>, key: String, value: String) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        params![key, value],
    )
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

// Helper function
fn get_note_by_id(conn: &rusqlite::Connection, id: i64) -> rusqlite::Result<Note> {
    conn.query_row(
        "SELECT * FROM notes WHERE id = ?1",
        params![id],
        |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content_md: row.get(2)?,
                content_json: row.get(3)?,
                folder: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                reminder_at: row.get(8)?,
                pinned: row.get::<_, i32>(9)? != 0,
                archived: row.get::<_, i32>(10)? != 0,
                deleted_at: row.get(11)?,
                brand: row.get(12)?,
                backlinks_json: row.get(13)?,
                tags_cache: row.get(14)?,
            })
        },
    )
}
