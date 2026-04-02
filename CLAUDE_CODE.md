# Claude Code 开发配置

本文档记录 MovieMaster 项目的 Claude Code 开发环境配置。

## 插件配置

### 已安装插件

| 插件名称 | 来源 | 用途 |
|---------|------|------|
| claude-hud | jarrodwatts/claude-hud | Claude Code HUD 增强 |

### 插件安装记录

**安装日期**: 2026-03-31

**安装方式**: 手动安装（自动安装遇到跨盘符错误）

```bash
# 步骤1: 清理旧文件
rm -rf "$HOME/.claude/plugins/cache/claude-hud"
rm -rf "$HOME/.claude/plugins/claude-hud"

# 步骤2: 克隆仓库
git clone https://github.com/jarrodwatts/claude-hud.git \
  "$HOME/.claude/plugins/claude-hud"

# 步骤3: 安装依赖
cd "$HOME/.claude/plugins/claude-hud"
npm install

# 步骤4: 构建
npm run build
```

**初始化配置**:
```bash
/claude-hud:setup
```

## Claude Code 设置

项目级设置文件位于：`.claude/settings.local.json`

当前配置允许以下命令自动执行：
- `cargo test:*` - Rust 测试
- `cargo check:*` - Rust 检查
- `cargo build:*` - Rust 构建
- `npm run build` - 前端构建
- `npm run tauri dev:*` - Tauri 开发模式
- `npm run tauri build:*` - Tauri 构建

## 常用命令

### 开发
```bash
# 启动开发服务器
npm run tauri dev

# 构建生产版本
npm run tauri build

# 检查 Rust 代码
cargo check

# 运行 Rust 测试
cargo test
```

### 项目检查
```bash
# 查看项目结构
find src -type f \( -name '*.ts' -o -name '*.tsx' \)

# 列出 Rust 源文件
find src-tauri/src -name '*.rs' -type f
```

## 项目结构速览

```
moviemaster/
├── src/                    # React 前端
├── src-tauri/src/          # Rust 后端
├── .claude/                # Claude Code 配置
│   └── settings.local.json
└── CLAUDE_CODE.md          # 本文档
```

## 注意事项

1. 插件安装如果遇到 "Unknown skill" 错误，可能是插件版本兼容性问题
2. 所有 Bash 命令权限都在 `settings.local.json` 中配置
3. 项目使用 Tauri 2.x + React + TypeScript + Rust 技术栈

---

**最后更新**: 2026-03-31
