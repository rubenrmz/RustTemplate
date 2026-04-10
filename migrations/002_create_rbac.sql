-- ============================================================
-- RBAC: Sistemas y Roles (permisos los resuelve cada microservicio)
-- ============================================================

-- Sistemas/aplicaciones que protege este auth server
CREATE TABLE IF NOT EXISTS systems (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key         VARCHAR(50)  NOT NULL UNIQUE,
    name        VARCHAR(100) NOT NULL,
    description TEXT,
    active      BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Roles por sistema
CREATE TABLE IF NOT EXISTS roles (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id   UUID         NOT NULL REFERENCES systems(id) ON DELETE CASCADE,
    key         VARCHAR(50)  NOT NULL,
    name        VARCHAR(100) NOT NULL,
    description TEXT,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    UNIQUE(system_id, key)
);

-- Roles asignados a un usuario
CREATE TABLE IF NOT EXISTS user_roles (
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id    UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- Indices
CREATE INDEX idx_roles_system     ON roles(system_id);
CREATE INDEX idx_user_roles_user  ON user_roles(user_id);
CREATE INDEX idx_user_roles_role  ON user_roles(role_id);

-- Quitar el campo role de users (ahora se maneja por tablas)
ALTER TABLE users DROP COLUMN IF EXISTS role;
