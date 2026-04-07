#!/usr/bin/env bash
set -e

echo "Setting up RemitX..."

cp .env.example .env

echo "Installing API dependencies..."
cd apps/api && npm install && cd ../..

echo "Installing frontend dependencies..."
cd apps/frontend && npm install && cd ../..

echo "Generating Prisma client..."
cd apps/api && npx prisma generate && cd ../..

echo ""
echo "Setup complete. Run: docker-compose up --build"
