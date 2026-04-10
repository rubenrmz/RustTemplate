-- ============================================================
-- Seed: Ejemplo de RBAC
-- Reemplaza 'my_system' y roles según tu proyecto.
-- ============================================================

-- Registrar un sistema
-- INSERT INTO systems (key, name, description) VALUES
--     ('my_system', 'Mi Sistema', 'Descripción del sistema');

-- Crear roles para ese sistema
-- INSERT INTO roles (system_id, key, name, description) VALUES
--     ((SELECT id FROM systems WHERE key = 'my_system'), 'admin',    'Administrador', 'Acceso total'),
--     ((SELECT id FROM systems WHERE key = 'my_system'), 'editor',   'Editor',        'Lectura y escritura'),
--     ((SELECT id FROM systems WHERE key = 'my_system'), 'viewer',   'Visor',         'Solo lectura');

-- Asignar un rol a un usuario
-- INSERT INTO user_roles (user_id, role_id)
-- SELECT u.id, r.id
-- FROM users u, roles r
-- JOIN systems s ON s.id = r.system_id
-- WHERE u.email = 'admin@example.com'
--   AND s.key = 'my_system'
--   AND r.key = 'admin'
-- ON CONFLICT DO NOTHING;
