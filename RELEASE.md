# MovieMaster-X v0.1.0 发布说明

## 下载地址

### Windows 安装包
- **NSIS 安装程序** (推荐): `release/MovieMaster_0.1.0_x64-setup.exe` (4.6 MB)
  - 双击运行，按向导安装
  - 支持自动创建桌面快捷方式
  
- **MSI 安装包**: `release/MovieMaster_0.1.0_x64_en-US.msi` (7.7 MB)
  - 企业部署推荐
  - 支持静默安装

### 便携版
- **独立可执行文件**: `release/moviemaster.exe` (20 MB)
  - 无需安装，解压即用
  - 适合U盘携带

## 系统要求
- Windows 10/11 64位
- 内存: 4GB+
- 硬盘: 50MB 可用空间

## 首次使用

### 1. 启动应用
- 安装版：从开始菜单或桌面快捷方式启动
- 便携版：双击 moviemaster.exe

### 2. 添加影视目录
1. 点击左侧「目录管理」
2. 点击「添加目录」按钮
3. 输入本地影视文件夹路径（如 `D:\Movies`）
4. 点击「扫描」开始索引

### 3. 配置 AI 助手（可选）
1. 打开「设置」→「AI 大语言模型配置」
2. 配置本地 LLM 服务地址（vLLM/Ollama）
3. 点击「测试连接」验证
4. 启用 AI 助手悬浮窗口

### 4. 配置 PT 搜索（可选）
1. 安装浏览器插件 PT-Depiler
2. 在设置中配置 HTTP 桥接地址
3. 在网页上选中影视名称即可搜索

## 主要功能

### AI Agent 智能助手
- 自然语言交互管理影视库
- 智能推荐和分类
- 自我学习用户偏好

### 媒体库管理
- 自动扫描本地/NAS 影视文件
- TMDB 元数据自动获取
- 海报封面下载与显示
- 重复文件检测与清理

### 下载集成
- PT 站点资源搜索
- 一键推送到 qBittorrent
- 下载状态实时监控

### 元数据工具
- 生成 Kodi 兼容的 NFO 文件
- 字幕文件管理
- 批量重命名

## 技术栈
- **前端**: React 19 + TypeScript + Tailwind CSS 4
- **后端**: Tauri 2.x + Rust
- **数据库**: SQLite
- **AI**: Hermes Agent 架构 + 本地 LLM

## 更新日志

### v0.1.0 (2025-04-02)
- 首次发布
- AI Agent 智能助手（基于 Hermes Agent 架构）
- PT-Depiler 浏览器插件集成
- qBittorrent 远程下载控制
- 智能重复文件检测
- TMDB 元数据集成
- SMB 网络共享支持
- 海报封面显示

## 反馈与支持
- GitHub Issues: https://github.com/91zgaoge/moviemaster-X/issues
- 提交 Bug 报告或功能建议

## 许可证
MIT License
