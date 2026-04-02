# MovieMaster 项目开发总结

## 一、项目概述

### 1.1 项目背景

MovieMaster（影视管家）原版是一款使用 桌面 Delphi 开发 Windows应用程序，主要用于管理本地影视文件。项目始于 2016 年，历经多次迭代更新至 v0.27 版本。原版软件功能丰富但受限于 Delphi 技术和旧代码架构，维护和扩展存在一定困难。

### 1.2 重构目标

本次重构旨在使用现代化的技术栈重新实现该应用，核心目标包括：

1. **技术升级** - 采用 Tauri 2.x + React + Rust 的现代化技术栈
2. **性能提升** - 利用 Rust 语言的高性能特性提升应用响应速度
3. **跨平台支持** - 为未来支持 macOS 和 Linux 做准备
4. **代码可维护性** - 采用模块化架构，便于后续维护和扩展
5. **SMB 网络支持** - 新增对 NAS 和网络共享目录的支持

### 1.3 技术选型

| 层级 | 技术方案 | 版本 | 选择理由 |
|------|----------|------|----------|
| 前端框架 | React | 18+ | 成熟的组件化 UI 框架，生态丰富 |
| UI 组件 | 自定义 + Radix UI | 最新 | 无障碍支持，可定制性强 |
| 样式方案 | Tailwind CSS | 4.x | 原子化 CSS 框架，开发效率高 |
| 桌面运行时 | Tauri | 2.x | Rust 后端，性能优异，包体积小 |
| 后端语言 | Rust | 1.85 | 内存安全，高性能 |
| 数据库 | SQLite | 3.x | 嵌入式，无需额外服务 |
| HTTP 客户端 | reqwest | 0.12 | 成熟的异步 HTTP 库 |
| 状态管理 | Zustand | 5.x | 轻量级，API 简洁 |
| 文件遍历 | walkdir | 2.x | 跨平台目录遍历 |

---

## 二、系统架构

### 2.1 整体架构

项目采用前后端分离架构，通过 Tauri IPC 通信：

```
┌─────────────────────────────────────────────────────────────┐
│                      React Frontend                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Pages     │  │ Components  │  │   Stores    │        │
│  │  Dashboard  │  │    UI       │  │ movieStore  │        │
│  │             │  │  button     │  │  dirStore   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│                           │                                 │
│                    Tauri IPC                                │
└───────────────────────────┼─────────────────────────────────┘
                            │
┌───────────────────────────┼─────────────────────────────────┐
│                      Rust Backend                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  Commands   │  │  Services   │  │   Models    │        │
│  │ directory   │  │   douban    │  │   Movie     │        │
│  │  movie      │  │     nfo     │  │  Directory  │        │
│  │   smb       │  │    hash     │  │   SMBConn   │        │
│  │  settings   │  │  scanner    │  │             │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│                           │                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │    DB       │  │    FS       │  │   HTTP      │        │
│  │  rusqlite   │  │   std::fs   │  │  reqwest    │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 前端架构

前端使用 React + TypeScript 开发，采用组件化设计：

```
src/
├── components/           # React 组件
│   ├── ui/              # 基础 UI 组件
│   │   ├── button.tsx
│   │   ├── input.tsx
│   │   ├── card.tsx
│   │   ├── dialog.tsx
│   │   └── tabs.tsx
│   └── layout/          # 布局组件
│       └── sidebar.tsx
├── pages/               # 页面组件
│   └── Dashboard.tsx    # 主仪表盘页面
├── stores/              # Zustand 状态管理
│   ├── movieStore.ts    # 影片状态
│   └── directoryStore.ts # 目录状态
├── hooks/               # 自定义 Hooks
├── lib/                 # 工具函数
│   ├── api.ts           # Tauri IPC 封装
│   └── utils.ts         # 通用工具
└── types/               # TypeScript 类型定义
```

### 2.3 后端架构

Rust 后端按职责分层组织：

```
src-tauri/src/
├── lib.rs               # 应用入口
├── main.rs              # 主函数
├── commands/            # Tauri 命令层
│   ├── mod.rs
│   ├── directory.rs    # 目录管理命令
│   ├── movie.rs        # 影片管理命令
│   ├── settings.rs     # 设置命令
│   └── smb.rs          # SMB 连接命令
├── services/           # 业务逻辑层
│   ├── mod.rs
│   ├── douban.rs       # 豆瓣 API 服务（框架）
│   ├── nfo.rs          # NFO 文件生成
│   └── hash.rs         # OpenSubtitles Hash 计算
├── models/             # 数据模型
│   └── mod.rs
├── scanner/            # 文件扫描
│   └── mod.rs
└── db/                 # 数据库层
    └── mod.rs
```

### 2.4 数据库设计

使用 SQLite 数据库，主要表结构如下：

```sql
-- 影视目录表
CREATE TABLE directories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    name TEXT,
    path_type TEXT DEFAULT 'local',    -- 'local' | 'smb'
    smb_connection_id TEXT,
    enabled INTEGER DEFAULT 1,
    created_at TEXT DEFAULT (datetime('now'))
);

-- 影片表
CREATE TABLE movies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    directory_id INTEGER REFERENCES directories(id),
    filename TEXT NOT NULL,
    path TEXT NOT NULL,
    cnname TEXT,           -- 中文名
    cnoname TEXT,          -- 英文名
    year TEXT,             -- 年份
    countries TEXT,        -- 国家
    douban_id TEXT,        -- 豆瓣 ID
    imdb_id TEXT,          -- IMDB ID
    poster_path TEXT,      -- 海报路径
    fanart_path TEXT,      -- 背景图路径
    description TEXT,     -- 简介
    douban_rating REAL,   -- 豆瓣评分
    imdb_rating REAL,      -- IMDB 评分
    video_type TEXT DEFAULT 'movie',  -- 'movie' | 'tv'
    season TEXT,           -- 季号
    episode TEXT,          -- 集号
    file_size INTEGER,     -- 文件大小
    file_hash TEXT,        -- OpenSubtitles Hash
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- SMB 连接表
CREATE TABLE smb_connections (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    host TEXT NOT NULL,
    share TEXT NOT NULL,
    username TEXT,
    password TEXT,
    domain TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- 字幕表
CREATE TABLE subtitles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    movie_id INTEGER REFERENCES movies(id),
    language TEXT,
    format TEXT,
    filename TEXT,
    path TEXT,
    download_url TEXT,
    file_hash TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

-- 设置表
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT
);
```

---

## 三、功能实现

### 3.1 核心功能

| 功能 | 实现状态 | 说明 |
|------|----------|------|
| 目录管理 | ✅ 完成 | 添加/删除/启用/禁用目录 |
| 文件扫描 | ✅ 完成 | 递归扫描，支持多种视频格式 |
| 文件名解析 | ✅ 完成 | 智能解析电影/电视剧 |
| SQLite 存储 | ✅ 完成 | 持久化存储 |
| 豆瓣信息获取 | ⚠️ 框架 | 集成 TMDB API 作为备选 |
| 海报下载 | ✅ 完成 | 异步下载到本地缓存 |
| NFO 生成 | ✅ 完成 | 生成 Kodi 兼容 NFO 文件 |
| SMB 连接管理 | ⚠️ 框架 | 数据库支持，需实际 SMB 库 |
| 字幕下载 | 📋 待开发 | OpenSubtitles Hash 已实现 |

### 3.2 文件名解析

文件名解析器支持多种格式：

**电视剧格式：**
- `Title.S01E01.1080p.WEB-DL.x264.mkv`
- `Title.1x01.720p.mkv`
- `Title - Season 1 Episode 01.mkv`

**电影格式：**
- `Title.2023.1080p.BluRay.x264.mkv`
- `Title (2023) 1080p.mkv`

### 3.3 NFO 生成

生成符合 Kodi 规范的 NFO 文件：

**电影 NFO：**
```xml
<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<movie>
  <title>电影标题</title>
  <originaltitle>Original Title</originaltitle>
  <year>2023</year>
  <rating>8.5</rating>
  <plot>剧情简介...</plot>
  <country>中国</country>
  <genre>动作</genre>
  <id>tt1234567</id>
</movie>
```

**电视剧 NFO：**
```xml
<?xml version="1.0" encoding="utf-8" standalone="yes"?>
<tvshow>
  <title>电视剧标题</title>
  <season>1</season>
  <episode>01</episode>
  <plot>剧情简介...</plot>
</tvshow>
```

---

## 四、构建产物

### 4.1 编译结果

```
moviemaster/src-tauri/target/release/
├── moviemaster.exe              # 主程序 (~15 MB)
└── bundle/
    ├── msi/
    │   └── MovieMaster_0.1.0_x64_en-US.msi
    └── nsis/
        └── MovieMaster_0.1.0_x64-setup.exe  (~3.6 MB)
```

### 4.2 依赖包体积

| 依赖 | 大小 | 说明 |
|------|------|------|
| Tauri 运行时 | ~5 MB | 基础运行时 |
| WebView2 | ~3 MB | Windows Edge 组件 |
| SQLite | ~2 MB | 嵌入式数据库 |
| reqwest | ~1 MB | HTTP 客户端 |
| 其他 Rust crates | ~4 MB | 各种工具库 |

---

## 五、开发过程

### 5.1 关键决策

1. **技术栈选择**
   - 选择 Tauri 而非 Electron：更小的包体积，更好的性能
   - 选择 Tailwind CSS：开发效率高，易于维护

2. **数据库设计**
   - 采用 SQLite：无需额外服务，便携性好
   - 使用文件路径作为唯一标识：便于跨设备迁移

3. **模块划分**
   - 前端使用 Zustand：轻量级，不需要 Redux 的复杂性
   - 后端按命令/服务/模型分层：职责清晰

### 5.2 遇到的问题及解决方案

1. **Rust 版本兼容性**
   - 问题：`time` crate 需要 Rust 1.88，但环境是 1.85
   - 解决：降级 `chrono` 到 0.4.38 版本

2. **模块命名冲突**
   - 问题：`scanner::parse_filename` 位置错误
   - 解决：将函数移到 `services` 模块并正确导出

3. **库名变更**
   - 问题：`tauri_app_lib` 改为 `moviemaster_lib`
   - 解决：更新 `main.rs` 中的引用

### 5.3 待优化项

1. **豆瓣 API**：需要稳定的 API 方案
2. **SMB 支持**：需要集成实际 SMB 库
3. **字幕下载**：需要完善 OpenSubtitles API 集成
4. **错误处理**：需要更完善的异常处理
5. **日志系统**：需要结构化日志

---

## 六、项目结构

```
moviemaster/
├── src/                          # React 前端
│   ├── components/               # UI 组件
│   │   ├── ui/                   # 基础组件
│   │   │   ├── button.tsx
│   │   │   ├── input.tsx
│   │   │   ├── card.tsx
│   │   │   ├── dialog.tsx
│   │   │   └── tabs.tsx
│   │   └── layout/               # 布局组件
│   │       └── sidebar.tsx
│   ├── pages/                    # 页面
│   │   └── Dashboard.tsx
│   ├── stores/                   # 状态管理
│   │   ├── movieStore.ts
│   │   └── directoryStore.ts
│   ├── lib/                      # 工具函数
│   │   ├── api.ts
│   │   └── utils.ts
│   ├── App.tsx
│   ├── main.tsx
│   └── index.css
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── commands/             # Tauri 命令
│   │   │   ├── directory.rs
│   │   │   ├── movie.rs
│   │   │   ├── settings.rs
│   │   │   └── smb.rs
│   │   ├── services/             # 业务服务
│   │   │   └── mod.rs
│   │   ├── models/
│   │   │   └── mod.rs
│   │   ├── scanner/
│   │   │   └── mod.rs
│   │   ├── db/
│   │   │   └── mod.rs
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/                         # 文档
├── package.json
├── README.md
└── CHANGELOG.md
```

---

## 七、后续开发计划

### 7.1 短期目标

1. **完善豆瓣 API**：集成稳定的豆瓣数据源
2. **SMB 实际支持**：集成 `smb` crate 实现网络邻居访问
3. **字幕下载**：实现 OpenSubtitles API 完整集成
4. **UI 优化**：添加右键菜单、批量操作等功能

### 7.2 中期目标

1. **数据迁移工具**：支持从原版 Delphi 导入数据
2. **多语言支持**：添加英文界面
3. **主题切换**：支持亮色/暗色主题
4. **云同步**：可选的云端数据备份

### 7.3 长期目标

1. **跨平台**：支持 macOS 和 Linux
2. **插件系统**：支持第三方扩展
3. **智能推荐**：基于用户库的影片推荐

---

## 八、总结

MovieMaster 项目使用现代技术栈成功实现了原版软件的核心功能，并在此基础上进行了架构优化。重构后的应用具备以下优势：

1. **性能优异**：Rust 后端确保文件扫描和数据处理的高效性
2. **体积小巧**：可执行文件仅约 15 MB，安装包约 3.6 MB
3. **代码现代**：使用 TypeScript 和 Rust 的最新特性
4. **架构清晰**：模块化设计便于维护和扩展

项目已具备基本可用性，后续将继续完善功能，特别是豆瓣信息获取、SMB 网络支持和字幕下载等核心功能。

---

**项目状态**: 🟢 开发中  
**最后更新**: 2026-02-24  
**版本**: 0.1.0
