CREATE TABLE IF NOT EXISTS app_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    value_type TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    is_system INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO app_settings (key, value, value_type, category, description) VALUES
    ('docx.default_template', 'analytical-report', 'string', 'docx', 'Шаблон DOCX по умолчанию'),
    ('docx.default_export_dir', '', 'string', 'docx', 'Папка экспорта DOCX по умолчанию'),
    ('docx.include_materials_table', 'true', 'boolean', 'docx', 'Включать таблицу материалов в DOCX'),
    ('docx.include_sha256_table', 'true', 'boolean', 'docx', 'Включать таблицу SHA-256 в DOCX'),
    ('backup.default_dir', '', 'string', 'backup', 'Папка резервного копирования по умолчанию'),
    ('backup.safety_before_restore', 'true', 'boolean', 'backup', 'Создавать бэкап безопасности перед восстановлением'),
    ('backup.verify_after_create', 'true', 'boolean', 'backup', 'Проверять бэкап после создания'),
    ('integrity.warn_before_docx_export', 'true', 'boolean', 'integrity', 'Предупреждать перед экспортом DOCX'),
    ('integrity.warn_before_backup', 'true', 'boolean', 'integrity', 'Предупреждать перед резервным копированием'),
    ('access.viewer_can_export_docx', 'false', 'boolean', 'access', 'Разрешить наблюдателю экспорт в DOCX'),
    ('access.analyst_can_create_backup', 'false', 'boolean', 'access', 'Разрешить аналитику создавать бэкапы');
