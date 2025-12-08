@echo off
echo 正在启动 Rock Market Lab 前端开发服务器...
echo.

REM 检查是否安装了 Node.js
node --version >nul 2>&1
if errorlevel 1 (
    echo 错误: 未找到 Node.js，请先安装 Node.js
    pause
    exit /b 1
)

REM 检查是否安装了依赖
if not exist "node_modules" (
    echo 正在安装依赖...
    npm install
    if errorlevel 1 (
        echo 依赖安装失败
        pause
        exit /b 1
    )
)

echo 启动开发服务器...
echo 应用将在 http://localhost:3000 启动
echo 按 Ctrl+C 停止服务器
echo.

npm run dev
