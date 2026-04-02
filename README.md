# MovieMaster-X 影视管家

<p align="center">
  <img src="./docs/logo.png" alt="MovieMaster Logo" width="128" height="128" />
</p>

<p align="center">
  <a href="https://github.com/91zgaoge/moviemaster-X/releases">
    <img src="https://img.shields.io/github/v/release/91zgaoge/moviemaster-X?include_prereleases&label=latest" alt="Latest Release" />
  </a>
  <a href="https://github.com/91zgaoge/moviemaster-X/stargazers">
    <img src="https://img.shields.io/github/stars/91zgaoge/moviemaster-X" alt="Stars" />
  </a>
  <a href="./LICENSE">
    <img src="https://img.shields.io/github/license/91zgaoge/moviemaster-X" alt="License" />
  </a>
</p>

> 🤖 AI驱动的本地影视文件智能管理工具 - 基于 Hermes Agent 架构的自我进化系统

## 📖 简介

MovieMaster-X（影视管家X版）是一款**AI驱动的本地影视文件智能管理工具**，基于 Hermes Agent 架构实现自我学习和进化能力。

### 核心能力

🤖 **AI Agent 智能助手** - 基于 Nous Research Hermes Agent 架构
- 自我学习进化：技能自动创建、性能优化
- 双轨记忆系统：永久存档 + 动态状态
- 本地 LLM 支持：兼容 vLLM/Ollama，支持 API Key 认证

🔍 **PT 站点集成** - 配合 PT-Depiler 浏览器插件
- HTTP 桥接搜索 PT 站点影视资源
- 一键推送到 qBittorrent 远程下载

🎬 **智能媒体库管理**
- 自动扫描本地/NAS 影视文件
- TMDB 元数据获取与海报下载
- 重复文件检测与批量处理
- 智能更新：关联季/集自动同步

## ✨ 核心特性

### 🤖 AI Agent 智能助手
- **Hermes Agent 架构** - 基于 Nous Research 的 Hermes Agent 设计
- **自我学习进化** - 支持技能创建、进化和性能优化
- **双轨记忆系统** - Frozen Memory (永久存档) + Live Memory (动态状态)
- **本地 LLM 支持** - 兼容 vLLM/Ollama，支持 Qwen2.5-32B 等模型
- **API Key 认证** - 支持 OpenAI 兼容的 API 认证
- **橙色主题界面** - 独特的 AI 助手悬浮聊天窗口

### 🔍 PT 站点集成
- **PT-Depiler 浏览器插件** - 通过 HTTP 桥接搜索影视资源
- **智能搜索** - 支持电影/剧集名称、年份、季集号搜索
- **一键推送 qBittorrent** - 搜索结果直接发送到下载器

### ⬇️ 下载管理
- **qBittorrent 远程控制** - Web API 集成
- **下载状态监控** - 实时查看下载进度
- **自动分类** - 根据电影类型自动整理下载文件

### 🎬 媒体库管理
- **智能扫描** - 自动识别视频文件并提取元数据
- **TMDB 集成** - 自动获取电影信息、海报、评分
- **重复文件检测** - 基于文件哈希快速找出重复文件
- **智能更新** - 更新一部影片时自动更新相关季/版本
- **NFO 生成** - 导出标准格式影视信息文件
- **海报管理** - 自动下载和缓存海报封面

### 🌐 多协议支持
- **SMB 协议** - 支持网络共享文件夹扫描
- **本地存储** - 本地磁盘影视库管理
- **字幕管理** - 字幕文件关联和管理

### 用户体验
- 🚀 轻量级 - 可执行文件仅约 15MB
- 🔄 快速响应 - Rust 后端确保高性能
- 💾 便携数据 - SQLite 数据库，随应用移动
- 🖥️ 现代 UI - 简洁直观的界面设计

## 🛠️ 技术架构

### 前端技术栈
| 技术 | 版本 | 用途 |
|------|------|------|
| React | 19.x | UI 框架 |
| TypeScript | 5.8 | 类型安全 |
| Vite | 7.x | 构建工具 |
| Tailwind CSS | 4.x | 样式方案 |
| Radix UI | latest | 基础组件 |
| Zustand | 5.x | 状态管理 |
| Lucide React | latest | 图标库 |

### 后端技术栈
| 技术 | 版本 | 用途 |
|------|------|------|
| Tauri | 2.x | 桌面框架 |
| Rust | 1.70+ | 后端语言 |
| Tokio | latest | 异步运行时 |
| SQLite | latest | 数据库 |
| Reqwest | latest | HTTP 客户端 |
| Walkdir | latest | 文件扫描 |

## 🚀 快速开始

### 前置要求

- Node.js 18+
- Rust 1.70+
- Windows 10/11 (主要目标平台)

### 开发环境

```bash
# 克隆项目
git clone https://github.com/moviemaster/moviemaster.git
cd moviemaster

# 安装依赖
npm install

# 启动开发服务器
npm run tauri dev
```

### 构建发布

```bash
# 构建生产版本
npm run tauri build

# 构建产物位于
# - 可执行文件: src-tauri/target/release/moviemaster.exe
# - 安装包: src-tauri/target/release/bundle/
```

## 📖 使用指南

### 添加影视目录

1. 点击左侧菜单「目录管理」
2. 点击「添加目录」按钮
3. 输入目录路径（如 `D:\Movies`）
4. 点击「扫描」按钮开始扫描

### 批量更新信息

在目录管理页面，点击目录旁边的扫描按钮，后台会自动：
- 扫描目录中的视频文件
- 解析文件名提取影片信息
- 计算 OpenSubtitles Hash（用于字幕下载）

### 生成 NFO 文件

选择已获取信息的影片，右键选择「生成 NFO」，将在视频同目录下生成：
- `movie.nfo` 或 `tvshow.nfo`
- `poster.jpg` (海报)
- `fanart.jpg` (背景图)

## 🗂️ 项目结构

```
moviemaster/
├── src/                          # 前端源码
│   ├── components/               # React 组件
│   │   ├── AIAssistant.tsx       # AI 助手界面（橙色主题）
│   │   ├── LLMSettings.tsx       # LLM 配置（支持 API Key）
│   │   ├── PTDepilerSettings.tsx # PT 插件设置
│   │   ├── QBittorrentSettings.tsx # 下载器设置
│   │   ├── DuplicateManager.tsx  # 重复文件管理
│   │   ├── MovieDetailDialog.tsx # 影片详情
│   │   └── ui/                   # 基础 UI 组件
│   ├── pages/                    # 页面
│   │   └── Dashboard.tsx         # 主控制台
│   ├── stores/                   # 状态管理
│   │   ├── movieStore.ts         # 影片状态
│   │   └── directoryStore.ts     # 目录状态
│   └── lib/                      # 工具库
│       └── api.ts                # API 接口
├── src-tauri/                    # 后端源码
│   ├── src/
│   │   ├── agent/                # AI Agent 系统
│   │   │   ├── mod.rs            # Agent 管理器
│   │   │   ├── llm.rs            # LLM 客户端
│   │   │   ├── memory.rs         # 双轨记忆系统
│   │   │   ├── skills.rs         # 技能注册表
│   │   │   └── system_prompt.md  # 系统提示词
│   │   ├── commands/             # Tauri 命令
│   │   │   ├── agent.rs          # AI 相关命令
│   │   │   ├── movie.rs          # 影片管理
│   │   │   ├── pt_depiler.rs     # PT 搜索
│   │   │   ├── qbittorrent.rs    # 下载控制
│   │   │   └── duplicate.rs      # 重复检测
│   │   ├── services/             # 业务服务
│   │   │   ├── tmdb.rs           # TMDB API
│   │   │   ├── nfo.rs            # NFO 生成
│   │   │   └── subtitle.rs       # 字幕管理
│   │   ├── db/mod.rs             # 数据库
│   │   └── main.rs               # 入口
│   └── Cargo.toml                # Rust 依赖
├── README.md                     # 本文件
└── package.json                  # NPM 配置
```

## 🔧 配置说明

### Tauri 配置 (tauri.conf.json)

```json
{
  "productName": "MovieMaster",
  "version": "0.1.0",
  "identifier": "com.moviemaster.app",
  "app": {
    "windows": [{
      "title": "影视管家 MovieMaster",
      "width": 1280,
      "height": 800
    }]
  }
}
```

### 数据库结构

主要表结构：
- `directories` - 影视目录
- `movies` - 影片信息
- `subtitles` - 字幕记录
- `smb_connections` - SMB 连接配置
- `settings` - 应用设置

## 🤖 AI Agent 使用指南

### LLM 配置
1. 打开"设置" → "AI 大语言模型配置"
2. 配置 LLM API 地址（如 http://localhost:8000/v1）
3. 输入模型名称（如 Qwen2.5-32B）
4. 如需认证，填写 API Key
5. 点击"测试连接"验证配置

### 可用技能
- **Movie Search** - 搜索本地影视库
- **PT Site Search** - PT 站点资源搜索
- **qBittorrent Control** - 远程下载管理
- **Duplicate Detection** - 重复文件检测
- **Smart Update** - 智能元数据更新

## 🔍 PT-Depiler 集成

### 配置步骤
1. 安装浏览器插件 PT-Depiler
2. 在 MovieMaster 设置中配置 HTTP 桥接地址
3. 插件会自动连接并搜索选中的影视名称

### 搜索流程
1. 在浏览器中选择影视名称
2. 点击插件图标发送搜索请求
3. MovieMaster 接收请求并搜索 PT 站点
4. 结果返回给浏览器插件显示

## 🤝 贡献指南

欢迎提交 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 📄 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](./LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Hermes Agent](https://github.com/NousResearch/hermes-agent) - AI Agent 架构参考
- [PT-Depiler](https://github.com/pt-plugins/PT-depiler) - PT 站点搜索插件
- [TMDB](https://www.themoviedb.org/) - 影视元数据源
- [React](https://react.dev/) - 前端框架
- [Kodi](https://kodi.tv/) - NFO 规范参考

---

<p align="center">
  Made with ❤️ by MovieMaster-X Team
</p>
