#!/bin/bash

echo "Building frontend..."
cd frontend
npm install
npm run build

echo "Building backend with embedded frontend..."
cd ../backend
cargo build --release

echo "Build complete! Binary is at: backend/target/release/minecraft-auth-backend"
