PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS workspaces (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

CREATE TABLE IF NOT EXISTS workspace_members (
  workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
  email TEXT NOT NULL,
  role TEXT NOT NULL DEFAULT 'writer' CHECK (role IN ('owner','editor','writer','reader')),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  PRIMARY KEY (workspace_id, email)
);

CREATE TABLE IF NOT EXISTS documents (
  id TEXT PRIMARY KEY,
  workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
  title TEXT NOT NULL DEFAULT 'Untitled document',
  status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft','in_review','final','archived')),
  created_by TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  version INTEGER NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_documents_workspace_updated
  ON documents(workspace_id, updated_at DESC);

CREATE TABLE IF NOT EXISTS sentence_blocks (
  id TEXT PRIMARY KEY,
  document_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
  position INTEGER NOT NULL,
  body TEXT NOT NULL DEFAULT '',
  claim_type TEXT CHECK (claim_type IN ('descriptive','normative','analytical','causal','inferential','uncertain')),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  UNIQUE(document_id, position)
);
CREATE INDEX IF NOT EXISTS idx_sentence_blocks_document_position
  ON sentence_blocks(document_id, position);

CREATE TABLE IF NOT EXISTS sources (
  id TEXT PRIMARY KEY,
  workspace_id TEXT NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  authors TEXT NOT NULL DEFAULT '',
  publication_year INTEGER,
  publisher TEXT NOT NULL DEFAULT '',
  url TEXT NOT NULL DEFAULT '',
  doi TEXT NOT NULL DEFAULT '',
  source_type TEXT NOT NULL DEFAULT 'web' CHECK (source_type IN ('web','journal','book','report','dataset','other')),
  notes TEXT NOT NULL DEFAULT '',
  file_key TEXT,
  created_by TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE INDEX IF NOT EXISTS idx_sources_workspace_title
  ON sources(workspace_id, title);

CREATE TABLE IF NOT EXISTS citations (
  id TEXT PRIMARY KEY,
  sentence_id TEXT NOT NULL REFERENCES sentence_blocks(id) ON DELETE CASCADE,
  source_id TEXT NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
  locator TEXT NOT NULL DEFAULT '',
  quoted_text TEXT NOT NULL DEFAULT '',
  note TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  UNIQUE(sentence_id, source_id, locator)
);
CREATE INDEX IF NOT EXISTS idx_citations_sentence ON citations(sentence_id);
CREATE INDEX IF NOT EXISTS idx_citations_source ON citations(source_id);

CREATE TABLE IF NOT EXISTS argument_reviews (
  id TEXT PRIMARY KEY,
  document_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
  sentence_id TEXT REFERENCES sentence_blocks(id) ON DELETE CASCADE,
  review_kind TEXT NOT NULL CHECK (review_kind IN ('claim_type','evidence','logic','causality','consistency','structure')),
  severity TEXT NOT NULL DEFAULT 'note' CHECK (severity IN ('note','check','important')),
  finding TEXT NOT NULL,
  rationale TEXT NOT NULL DEFAULT '',
  suggestion TEXT NOT NULL DEFAULT '',
  model_name TEXT NOT NULL DEFAULT 'rules-v1',
  dismissed INTEGER NOT NULL DEFAULT 0 CHECK (dismissed IN (0,1)),
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE INDEX IF NOT EXISTS idx_argument_reviews_document
  ON argument_reviews(document_id, dismissed, created_at DESC);

CREATE TABLE IF NOT EXISTS document_versions (
  id TEXT PRIMARY KEY,
  document_id TEXT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
  version INTEGER NOT NULL,
  snapshot_json TEXT NOT NULL,
  created_by TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
  UNIQUE(document_id, version)
);
