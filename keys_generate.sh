#!/bin/bash

# ---------------------------------------------
# Generador de claves JWT (RSA o ECDSA)
# Uso: ./gen_keys.sh [rsa|ec] [directorio]
# Ejemplo: ./gen_keys.sh rsa .secrets
#          ./gen_keys.sh ec  .secrets
# ---------------------------------------------

set -e  # detener si cualquier comando falla

ALGO=${1:-rsa}       # rsa o ec, default rsa
DIR=${2:-.secrets}  # directorio destino, default .secrets

PRIVATE="$DIR/private.pem"
PUBLIC="$DIR/public.pem"

# colores
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Generando claves JWT ($ALGO)...${NC}"

# crear directorio si no existe
mkdir -p "$DIR"
chmod 700 "$DIR"

case $ALGO in
    rsa)
        echo "Algoritmo: RS256 (RSA 2048 bits)"
        openssl genrsa -out "$PRIVATE" 2048
        openssl rsa -in "$PRIVATE" -pubout -out "$PUBLIC"
        ;;
    ec)
        echo "Algoritmo: ES256 (ECDSA P-256)"
        openssl ecparam -name prime256v1 -genkey -noout -out "$PRIVATE"
        openssl ec -in "$PRIVATE" -pubout -out "$PUBLIC"
        ;;
    *)
        echo -e "${RED}Algoritmo inválido. Usa: rsa o ec${NC}"
        exit 1
        ;;
esac

# permisos estrictos
chmod 600 "$PRIVATE"
chmod 644 "$PUBLIC"

# verificar que el par coincide
echo ""
echo "Verificando par de claves..."

if [ "$ALGO" = "rsa" ]; then
    HASH_PRIVATE=$(openssl rsa -in "$PRIVATE" -pubout 2>/dev/null | openssl md5 | awk '{print $2}')
    HASH_PUBLIC=$(openssl md5 < "$PUBLIC" | awk '{print $2}')
else
    HASH_PRIVATE=$(openssl ec -in "$PRIVATE" -pubout 2>/dev/null | openssl md5 | awk '{print $2}')
    HASH_PUBLIC=$(openssl md5 < "$PUBLIC" | awk '{print $2}')
fi

if [ "$HASH_PRIVATE" = "$HASH_PUBLIC" ]; then
    echo -e "${GREEN}Par de claves válido${NC}"
else
    echo -e "${RED}Error: las claves no coinciden${NC}"
    exit 1
fi

# agregar al .gitignore si existe
if [ -f ".gitignore" ]; then
    if ! grep -q "*.pem" .gitignore; then
        echo "*.pem" >> .gitignore
        echo -e "${GREEN}.gitignore actualizado${NC}"
    else
        echo ".gitignore ya contiene *.pem"
    fi
fi

# resumen
echo ""
echo -e "${GREEN}Claves generadas correctamente${NC}"
echo "-------------------------------------"
echo "Privada : $PRIVATE"
echo "Pública : $PUBLIC"
echo "Directorio : $DIR (chmod 700)"
echo "-------------------------------------"
echo -e "${YELLOW}IMPORTANTE: nunca commitees $PRIVATE${NC}"