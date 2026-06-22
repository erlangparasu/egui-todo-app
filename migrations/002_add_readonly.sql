-- 002: Add readonly field
ALTER TABLE todos ADD COLUMN readonly INTEGER DEFAULT 0;