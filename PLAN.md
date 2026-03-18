# Claude Code Message History Viewer

## 概述

将 `~/.claude/history.jsonl` 可视化为可搜索、可筛选的 Web 界面。

- **后端**: Rust (Axum)
- **前端**: SolidJS + TanStack Virtual (虚拟滚动长列表)
- **构建**: Vite

## 数据结构

```jsonl
{
  "display": "用户输入的消息文本",
  "pastedContents": {},
  "timestamp": 1770455022368,    // Unix ms
  "project": "/Users/s/test/...", // 项目路径
  "sessionId": "uuid"
}
```

当前数据规模: ~711 条记录, 172K, 47 个项目, 262 个会话。

## 前端选型: SolidJS

| 候选 | 首屏速度 | 长列表性能 | 包大小 |
|------|----------|-----------|--------|
| React | 中 | 需 react-window | ~40KB |
| Vue | 中 | 需 vue-virtual-scroller | ~33KB |
| **SolidJS** | **快** | **@tanstack/solid-virtual** | **~7KB** |
| Svelte | 快 | 生态弱 | ~2KB |

SolidJS 无 VDOM，细粒度响应式更新，配合 TanStack Virtual 做虚拟滚动，是长列表场景最优解。

## 架构

```
┌─────────────┐     HTTP/JSON     ┌──────────────┐
│  SolidJS    │  ◄──────────────► │  Axum Server │
│  + Vite     │   GET /api/...    │  (Rust)      │
│  虚拟滚动   │                    │  读 JSONL    │
└─────────────┘                    └──────────────┘
```

开发时 Vite dev server 代理 API 到 Axum；生产时 Axum 静态服务前端产物。

## API 设计

### `GET /api/messages`

查询参数:
- `project` - 按项目路径筛选
- `session` - 按 sessionId 筛选
- `q` - 全文搜索 display 字段
- `from` / `to` - 时间范围 (Unix ms)
- `offset` / `limit` - 分页 (默认 0/100)

响应:
```json
{
  "total": 711,
  "messages": [
    {
      "display": "...",
      "timestamp": 1770455022368,
      "project": "...",
      "sessionId": "..."
    }
  ]
}
```

### `GET /api/projects`

返回去重的项目列表及每个项目的消息数。

```json
[
  { "path": "/Users/s/test/claude-code", "count": 42 }
]
```

### `GET /api/sessions`

查询参数: `project` (可选)

返回会话列表 (sessionId, 首条消息时间, 消息数, 项目路径)。

### `GET /api/stats`

汇总统计: 总消息数、项目数、会话数、日期范围、每日消息数分布。

## 实现阶段

### Phase 1: Rust 后端骨架

1. `cargo init` 创建项目
2. 依赖: `axum`, `tokio`, `serde`, `serde_json`, `chrono`, `tower-http` (CORS + static)
3. 启动时读取 JSONL 到内存 `Vec<Message>` (172K 完全可以全量加载)
4. 实现 4 个 API endpoint
5. 错误处理: 跳过解析失败的行 (如 git conflict markers)

文件结构:
```
src/
  main.rs        — 启动 Axum server, 路由挂载
  models.rs      — Message, ProjectInfo, SessionInfo, Stats 结构体
  store.rs       — MessageStore: 加载 JSONL, 查询/筛选/统计方法
  handlers.rs    — API handler 函数
  error.rs       — 错误类型
Cargo.toml
```

### Phase 2: SolidJS 前端骨架

1. `pnpm create vite frontend --template solid-ts`
2. 依赖: `@tanstack/solid-virtual`, `solid-icons`, `dayjs`
3. Vite 配置代理 `/api` → `http://localhost:3001`

文件结构:
```
frontend/
  src/
    App.tsx           — 布局: 侧栏 + 主区域
    api/client.ts     — fetch wrapper
    components/
      MessageList.tsx — 虚拟滚动消息列表 (核心)
      MessageItem.tsx — 单条消息卡片
      Sidebar.tsx     — 项目列表 + 会话列表
      SearchBar.tsx   — 搜索 + 时间筛选
      StatsPanel.tsx  — 顶部统计卡片
    stores/
      filters.ts     — 响应式筛选状态 (createSignal)
  index.html
  vite.config.ts
```

### Phase 3: 核心交互

1. **虚拟滚动列表**: TanStack Virtual 渲染可见区域，支撑万级消息无卡顿
2. **侧栏筛选**: 点击项目/会话自动过滤
3. **搜索**: 输入框 debounce 300ms → 调用 API
4. **时间范围**: 日期选择器筛选
5. **统计面板**: 总消息数、项目数、会话数

### Phase 4: 打磨

1. 消息高亮搜索关键词
2. 按日期分组显示
3. 暗色主题
4. Axum 生产模式: 内嵌前端静态文件 (rust-embed 或 include_dir)
5. 一键启动脚本

## 技术决策

| 决策 | 选择 | 理由 |
|------|------|------|
| 数据存储 | 内存 | 172K 文件全量加载，无需数据库 |
| 前端框架 | SolidJS | 无 VDOM，细粒度更新，启动快 |
| 虚拟滚动 | TanStack Virtual | 框架无关，SolidJS 官方适配 |
| HTTP 框架 | Axum | Tokio 生态，性能好，API 简洁 |
| 前端构建 | Vite | 开发体验好，HMR 快 |
| 包管理 | pnpm | 磁盘效率高 |

## 启动方式

```bash
# 开发
cargo run &                          # 后端 :3001
cd frontend && pnpm dev              # 前端 :5173 (代理 API)

# 生产
cd frontend && pnpm build
cargo run --release                  # 同时服务 API + 静态文件于 :3001
```
