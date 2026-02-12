use crate::models::{Bundle, Item, ProgressStats};
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Criar tabelas
        conn.execute(
            "CREATE TABLE IF NOT EXISTS bundles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                room TEXT NOT NULL,
                required_items INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id TEXT PRIMARY KEY,
                bundle_id TEXT NOT NULL,
                name TEXT NOT NULL,
                status TEXT DEFAULT 'missing',
                quality TEXT,
                FOREIGN KEY (bundle_id) REFERENCES bundles(id)
            )",
            [],
        )?;

        // Índices
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_items_bundle ON items(bundle_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_items_status ON items(status)",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn get_all_bundles_with_items(&self) -> Result<Vec<Bundle>> {
        let mut bundles = self.get_all_bundles()?;

        for bundle in &mut bundles {
            let items = self.get_items_for_bundle(&bundle.id)?;
            bundle.items = Some(items);
        }

        Ok(bundles)
    }

    pub fn get_all_bundles(&self) -> Result<Vec<Bundle>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, room, required_items FROM bundles ORDER BY room, name")?;

        let bundles = stmt
            .query_map([], |row| {
                Ok(Bundle {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    room: row.get(2)?,
                    required_items: row.get(3)?,
                    items: None,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(bundles)
    }

    fn get_items_for_bundle(&self, bundle_id: &str) -> Result<Vec<Item>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, bundle_id, name, status, quality
             FROM items
             WHERE bundle_id = ?1
             ORDER BY name",
        )?;

        let items = stmt
            .query_map(params![bundle_id], |row| {
                Ok(Item {
                    id: row.get(0)?,
                    bundle_id: row.get(1)?,
                    name: row.get(2)?,
                    status: row.get(3)?,
                    quality: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    pub fn update_item_status(&self, item_id: &str, status: &str) -> Result<()> {
        if !matches!(status, "missing" | "collected" | "delivered") {
            return Err(rusqlite::Error::InvalidParameterName(format!(
                "Invalid status: {}",
                status
            )));
        }

        self.conn.execute(
            "UPDATE items SET status = ?1 WHERE id = ?2",
            params![status, item_id],
        )?;

        Ok(())
    }

    pub fn get_progress_stats(&self) -> Result<ProgressStats> {
        let total_items: i32 = self
            .conn
            .query_row("SELECT COUNT(*) FROM items", [], |row| row.get(0))?;

        let collected_items: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM items WHERE status = 'collected'",
            [],
            |row| row.get(0),
        )?;

        let delivered_items: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM items WHERE status = 'delivered'",
            [],
            |row| row.get(0),
        )?;

        let total_bundles: i32 =
            self.conn
                .query_row("SELECT COUNT(*) FROM bundles", [], |row| row.get(0))?;

        let bundles_completed: i32 = self.conn.query_row(
            "SELECT COUNT(DISTINCT bundle_id)
             FROM items
             WHERE bundle_id NOT IN (
                 SELECT DISTINCT bundle_id
                 FROM items
                 WHERE status != 'delivered'
             )",
            [],
            |row| row.get(0),
        )?;

        let progress_percentage = if total_items > 0 {
            (delivered_items as f32 / total_items as f32) * 100.0
        } else {
            0.0
        };

        Ok(ProgressStats {
            total_items,
            collected_items,
            delivered_items,
            progress_percentage,
            bundles_completed,
            total_bundles,
        })
    }

    pub fn insert_bundle(&self, bundle: &Bundle) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO bundles (id, name, room, required_items)
             VALUES (?1, ?2, ?3, ?4)",
            params![bundle.id, bundle.name, bundle.room, bundle.required_items],
        )?;
        Ok(())
    }

    pub fn insert_item(&self, item: &Item) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO items (id, bundle_id, name, status, quality)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                item.id,
                item.bundle_id,
                item.name,
                item.status,
                item.quality
            ],
        )?;
        Ok(())
    }
}
