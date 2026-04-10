# Rust Auth Template

Servidor de autenticacion centralizado construido con Axum. Emite JWTs firmados con RS256 y gestiona acceso a multiples sistemas mediante RBAC (roles por sistema).

## Stack

- **Axum 0.8** — HTTP framework
- **SQLx** — PostgreSQL async con migraciones automaticas
- **jsonwebtoken** — JWT RS256 sign/verify
- **Argon2** — hashing de passwords
- **Lettre** — envio de correos (recovery)
- **Tower** — middleware (CORS, rate limiting, tracing)

## Estructura

```
src/
├── config.rs              # Variables de entorno
├── state.rs               # AppState (DB pool, JWT keys)
├── errors.rs              # Manejo de errores centralizado
├── main.rs                # Entrypoint
├── lib.rs                 # create_app()
├── dto/                   # Request/Response DTOs
│   ├── auth_dto.rs
│   └── user_dto.rs
├── middleware/
│   ├── auth.rs            # require_auth (valida JWT)
│   ├── claims.rs          # Claims struct + encode/decode
│   └── role.rs            # require_admin
├── models/
│   ├── user.rs
│   └── rbac.rs            # System, Role, UserSystemRole
├── routes/
│   ├── auth/
│   │   ├── auth_routes.rs # login, register, refresh, me
│   │   ├── recovery_routes.rs # request-reset, reset-password
│   │   └── jwks.rs        # /.well-known/jwks.json
│   ├── users.rs           # CRUD usuarios (admin)
│   └── health.rs
├── services/
│   ├── auth/
│   │   ├── auth_service.rs
│   │   └── recovery_service.rs
│   └── user_service.rs
├── store/
│   ├── user_store.rs
│   ├── rbac_store.rs      # roles_by_user, assign_role
│   └── seeder.rs          # Seed admin al arrancar
└── utils/
    ├── password.rs
    └── mail.rs
```

## Setup

### 1. Clonar y configurar

```bash
cp .env.example .env
# Editar .env con tus valores de DB, mail, etc.
```

### 2. Generar llaves RSA

```bash
chmod +x keys_generate.sh
./keys_generate.sh rsa .secrets
```

Esto crea `.secrets/private.pem` y `.secrets/public.pem`.

### 3. Crear base de datos

Necesitas PostgreSQL. Las migraciones corren automaticamente al arrancar.

### 4. Registrar sistemas y roles

Edita `migrations/003_seed_rbac_example.sql` descomentando y adaptando los inserts, o ejecuta SQL directamente:

```sql
-- Registrar un sistema
INSERT INTO systems (key, name, description)
VALUES ('my_system', 'Mi Sistema', 'Descripcion');

-- Crear roles
INSERT INTO roles (system_id, key, name) VALUES
    ((SELECT id FROM systems WHERE key = 'my_system'), 'admin',  'Administrador'),
    ((SELECT id FROM systems WHERE key = 'my_system'), 'editor', 'Editor'),
    ((SELECT id FROM systems WHERE key = 'my_system'), 'viewer', 'Visor');
```

### 5. Ejecutar

```bash
cargo run
```

El servidor arranca en `APP_HOST:APP_PORT` (default `127.0.0.1:3000`).

Si `ADMIN_EMAIL` y `ADMIN_PASSWORD` estan configurados en `.env`, se crea el usuario admin y se le asigna rol `admin` en todos los sistemas activos.

## API

### Publicas

| Metodo | Ruta | Descripcion |
|--------|------|-------------|
| POST | `/auth/register` | Registro de usuario |
| POST | `/auth/login` | Login, retorna JWT |
| POST | `/auth/request-reset` | Solicitar reset de password |
| POST | `/auth/validate-token` | Validar token de reset |
| POST | `/auth/reset-password` | Resetear password |
| GET | `/.well-known/jwks.json` | Llave publica (JWKS) |
| GET | `/health` | Health check |

### Protegidas (requieren JWT)

| Metodo | Ruta | Descripcion |
|--------|------|-------------|
| POST | `/auth/refresh` | Renovar JWT |
| POST | `/auth/me` | Info del usuario + roles |

### Admin (requieren JWT + rol admin)

| Metodo | Ruta | Descripcion |
|--------|------|-------------|
| GET | `/users` | Listar usuarios |
| POST | `/users` | Crear usuario |
| GET | `/users/{id}` | Obtener usuario |
| PUT | `/users/{id}` | Actualizar usuario |
| DELETE | `/users/{id}` | Eliminar usuario |

## JWT

El token emitido tiene esta estructura:

```json
{
  "iss": "https://auth.tudominio.com",
  "aud": ["system_a", "system_b"],
  "sub": "uuid-del-usuario",
  "roles": {
    "system_a": ["admin", "editor"],
    "system_b": ["viewer"]
  },
  "exp": 1234567890,
  "iat": 1234567890
}
```

- **iss** — Identidad del auth server
- **aud** — Sistemas donde el usuario tiene acceso (derivado de sus roles en la DB)
- **sub** — UUID del usuario
- **roles** — Mapa de roles por sistema

## RBAC

```
systems       →  Catalogo de sistemas/aplicaciones
roles         →  Roles por sistema (admin, editor, viewer, etc.)
user_roles    →  Asignacion de roles a usuarios
```

El auth server emite el JWT con la informacion de roles. Cada microservicio:

1. Valida la firma con la llave publica (obtenida via JWKS)
2. Verifica que su `system_key` este en `aud`
3. Lee los roles del claim `roles[su_sistema]`
4. Aplica su propia logica de permisos segun el rol

## Migraciones

| Archivo | Contenido |
|---------|-----------|
| `001_create_users.sql` | Tabla de usuarios |
| `002_create_rbac.sql` | Tablas systems, roles, user_roles |
| `003_seed_rbac_example.sql` | Ejemplo comentado para registrar sistemas y roles |

Las migraciones se ejecutan automaticamente al arrancar via `sqlx::migrate!()`.

## Variables de entorno

Ver `.env.example` para la lista completa. Las principales:

| Variable | Requerida | Descripcion |
|----------|-----------|-------------|
| `JWT_ISSUER` | Si | Identidad del auth server |
| `JWT_EXPIRATION_SECONDS` | No | Duracion del token (default: 3600) |
| `JWT_PRIVATE_KEY_PATH` | Si* | Ruta a la llave privada RSA |
| `JWT_PUBLIC_KEY_PATH` | Si* | Ruta a la llave publica RSA |
| `DATABASE_HOST` | Si | Host de PostgreSQL |
| `DATABASE_NAME` | Si | Nombre de la base de datos |
| `ADMIN_EMAIL` | No | Email del admin seed |
| `ADMIN_PASSWORD` | No | Password del admin seed |
| `ALLOW_REGISTRATION` | No | Habilitar registro publico (default: true) |

*En produccion se pueden usar AWS Secrets Manager en lugar de archivos locales (`AWS_JWT_PRIVATE_KEY_SECRET`, `AWS_JWT_PUBLIC_KEY_SECRET`).
