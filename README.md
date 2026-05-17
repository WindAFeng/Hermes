# Hermes（赫尔墨斯）

> 一个基于 Rust + Tokio + Actix-web 构建的高性能、多协议数据库网关与中间件

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange?logo=rust)](https://www.rust-lang.org/)[![Actix Web](https://img.shields.io/badge/Actix_Web-4.x-blue?logo=rust)](https://actix.rs/)[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE.txt)

---

## 📌 项目简介

**Hermes** 是一款专为现代应用设计的数据库网关中间件，支持通过 **Socket** 和 **WebSocket** 协议接收外部请求，并以 **JSON 指令格式** 对接多种主流数据库（MySQL、PostgreSQL、Redis、MongoDB等），为外部操作数据库提供统一的数据操作接口。项目同时内置了智能缓存机制、优先级写入队列和高并发处理能力，适用于需要高效数据交互与统一管理的微服务架构或后端系统。

---

## ✨ 核心特性

- ✅ **多协议通信**：支持 TCP Socket 与 WebSocket 双通道接入
- ✅ **统一 JSON 指令**：所有数据库操作均通过结构化 JSON 指令完成
- ✅ **数据库支持**：
    - MySQL(当前版本未实现)
    - PostgreSQL(当前版本未实现)
    - Redis(当前版本部分实现)
    - MongoDB(当前版本未实现)
- ✅ **智能缓存层**：
    - 查询结果自动缓存（默认 TTL = 5 分钟，可通过Config.toml配置）
    - 写入时自动更新或失效相关缓存
    - 支持按“重要程度”设定写入优先级，动态调整写入队列顺序
    - 支持根据数据调用间隔动态将数据存入缓存的数据预判系统 (该条目未实现)
- ✅ **高并发 & 异步**：基于 `Tokio` 运行，充分利用 Rust 的异步与零成本抽象优势
- ✅ **Web 控制台(当前版本未实现)**：配套前端（Node.js + Vue.js + Naive UI）提供可视化终端，用于调试与监控
- ✅ **Windows客户端控制台(当前版本未实现):**基于**C#**+**WPF**开发的windows监控客户端
- ✅**数据网(当前版本未实现)：**基于数据调动次数实现的动态数据预热系统

---

### 支持的指令

| 指令     | 功能说明                                               |
| -------- | ------------------------------------------------------ |
| `Add`    | 插入新数据（单条或多条），仅当数据不存在时生效         |
| `Get`    | 批量查询数据，支持复杂条件与迭代返回（List 格式）      |
| `Update` | 更新已有数据，支持字段追加、修改、自增/自减            |
| `Delete` | 批量删除，支持复杂条件表达式                           |
| `Clear`  | **手动清除** Hermes 内存缓存（全局或指定键）           |
| `Set`    | **手动设置** 缓存内容（用于预热或调试）                |
| `Use`    | 调用数据库特有功能（如 Redis Pub/Sub、MySQL 触发器等） |
| `Config` | **热更新**部分配置项（如缓存 TTL、日志级别等）         |

## 🚀 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/WindAFeng/Hermes.git
cd hermes
```

### 2. 配置环境

复制示例配置并按需修改：

```bash
cp example.config.toml Config.toml
```

关键配置项包括：
- 数据库连接字符串
- 缓存 TTL（秒）
- 监听端口（HTTP/WebSocket/Socket）
- 日志级别

### 3. 启动后端服务

```bash
cargo run --release
```

### 4. 启动 Web 控制台（可选）

```bash
cd web-console
npm install
npm run dev
```

访问 `http://localhost:3000` 即可使用图形化终端。

---

## 📡 通信协议

- **Socket**
- **Websocket**

- **HTTP**

---

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE.txt)。

---

## 👥 贡献与反馈

欢迎提交 Issue 或 Pull Request！
作为开发者，你的建议对我非常重要 ❤️

