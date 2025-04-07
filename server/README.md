# GanttAi Server

这是 GanttAi 项目的服务器端实现，使用 Rust 语言开发，基于 Axum 框架构建。

## 技术栈

- **Web 框架**: Axum 0.7
- **数据库**: PostgreSQL + Diesel ORM
- **异步运行时**: Tokio
- **序列化**: Serde
- **API 文档**: Utoipa (Swagger UI)
- **日志系统**: Tracing
- **配置管理**: config

## 项目结构

```
src/
├── api/        # API 路由和处理器
├── core/       # 核心业务逻辑
├── db/         # 数据库相关代码
├── models/     # 数据模型定义
├── services/   # 业务服务层
└── utils/      # 工具函数
```

## 主要功能模块

### API 层

- RESTful API 接口
- 请求验证和错误处理
- API 文档自动生成

### 数据层

- PostgreSQL 数据库集成
- 异步数据库操作
- 数据模型定义和迁移

### 业务逻辑

- 甘特图项目管理
- 任务调度和依赖管理
- 用户认证和授权

### 基础设施

- 配置管理（环境变量、配置文件）
- 日志系统
- 错误处理
- 中间件（CORS、认证等）

## 开发指南

### 环境要求

- Rust 2021 edition
- PostgreSQL 数据库
- 环境变量配置（参考 .env 文件）

### 运行项目

#### 安装 postgresql 数据库

Ubuntu:

```bash
sudo apt-get update
sudo apt-get install postgresql
```

macOS:

```bash
brew install postgresql
```

#### 构建并运行项目：

```bash
# 安装依赖
cargo build

# 运行开发服务器
cargo run

# 运行测试
cargo test
```

### 数据库迁移

```bash
# 创建迁移
diesel migration generate <migration_name>

# 运行迁移
diesel migration run

# 回滚迁移
diesel migration revert
```

## API 文档

启动服务器后，访问 `/swagger-ui` 路径可以查看完整的 API 文档。

## 日志系统

使用 tracing 进行日志记录，支持不同级别的日志输出：

- ERROR: 错误信息
- WARN: 警告信息
- INFO: 一般信息
- DEBUG: 调试信息
- TRACE: 详细跟踪信息

## 配置管理

项目使用 `.env` 文件进行环境配置，主要配置项包括：

- 数据库连接信息
- 服务器端口
- 日志级别
- 其他环境特定配置

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

MIT License
