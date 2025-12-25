CREATE TABLE IF NOT EXISTS drops (
    id TEXT PRIMARY KEY,
    filename TEXT NOT NULL,
    encrypted_path TEXT NOT NULL,
    
    mime_type TEXT,
    size_bytes INTEGER NOT NULL,
    nonce BLOB NOT NULL,
    
    is_text BOOLEAN DEFAULT 0,
    
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    
    max_downloads INTEGER,
    download_count INTEGER DEFAULT 0,
    
    burn_after_read BOOLEAN DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_expires_at ON drops(expires_at);
CREATE INDEX IF NOT EXISTS idx_created_at ON drops(created_at);