@echo off

echo Building frontend...
cd frontend
call npm install
call npm run build

echo Building backend with embedded frontend...
cd ..\backend
cargo build --release

echo Build complete! Binary is at: backend\target\release\voxicraft-auth-backend.exe
