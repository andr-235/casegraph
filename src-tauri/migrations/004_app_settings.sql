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
    ('docx_default_template', 'analytical-report', 'string', 'docx', 'Шаблон DOCX по умолчанию'),
    ('integrity_check_on_startup', 'false', 'boolean', 'integrity', 'Проверка целостности при запуске'),
    ('viewer_can_export_docx', 'false', 'boolean', 'docx', 'Разрешить наблюдателю экспорт в DOCX'),
    ('analyst_can_create_backup', 'false', 'boolean', 'backup', 'Разрешить аналитику создавать бэкапы'),
    ('audit_strict_mode', 'true', 'boolean', 'audit', 'Строгий режим логирования аудита');
