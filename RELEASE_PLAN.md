# Release Plan — cc-msg-history-viewer

## 版本策略

采用语义化版本 (SemVer)：`MAJOR.MINOR.PATCH`

| 类型 | 规则 |
|------|------|
| MAJOR | 破坏性 API 变更或数据格式不兼容 |
| MINOR | 新功能，向后兼容 |
| PATCH | Bug 修复，向后兼容 |

**0.x.x 阶段**：快速迭代，API 不保证稳定。

---

## 发布里程碑

### v0.1.0 — Backend Foundation ✅
> **目标**: Rust 后端骨架可运行

**功能范围**:
- [ ] Axum HTTP 服务器 (`:3001`)
- [ ] 启动时读取 `~/.claude/history.jsonl` 到内存
- [ ] `GET /api/messages` — 分页 + 筛选 + 全文搜索
- [ ] `GET /api/projects` — 项目列表
- [ ] `GET /api/sessions` — 会话列表
- [ ] `GET /api/stats` — 汇总统计

**发布产物**: Linux x86_64 二进制（开发用）

---

### v0.2.0 — Frontend Skeleton
> **目标**: 前端能连通后端，基础 UI 可用

**功能范围**:
- [ ] SolidJS + Vite 项目初始化
- [ ] API client (`src/api/client.ts`)
- [ ] 消息列表（虚拟滚动，TanStack Virtual）
- [ ] 侧栏项目/会话列表
- [ ] 基础搜索框（debounce 300ms）

**发布产物**: 压缩包（binary + `frontend/dist/`）

---

### v0.3.0 — Core Interactions
> **目标**: 核心交互完整，日常可用

**功能范围**:
- [ ] 搜索关键词高亮
- [ ] 时间范围筛选（日期选择器）
- [ ] 按日期分组显示
- [ ] 统计面板（消息数/项目数/会话数）
- [ ] 暗色主题

**发布产物**: 压缩包（binary + `frontend/dist/`）

---

### v1.0.0 — Single Binary Release
> **目标**: 单二进制文件，开箱即用

**功能范围**:
- [ ] `rust-embed` 内嵌前端静态资源
- [ ] 一个可执行文件同时提供 API + 静态资源
- [ ] 自定义 history 文件路径（`--history-file`）
- [ ] 自定义端口（`--port`，默认 3001）
- [ ] 文件变更自动重载（inotify/FSEvents）

**发布产物**:
| 平台 | 文件名 |
|------|--------|
| Linux x86_64 | `cc-msg-viewer-linux-x86_64` |
| Linux ARM64  | `cc-msg-viewer-linux-aarch64` |
| macOS x86_64 | `cc-msg-viewer-macos-x86_64` |
| macOS ARM64  | `cc-msg-viewer-macos-aarch64` |

---

### v1.x.x — Enhancement Backlog

- [ ] 消息导出（JSON / CSV）
- [ ] 多 history 文件支持（多用户/多机器合并）
- [ ] 会话时间线视图
- [ ] Web 端分享会话链接（permalink）

---

## 发布流程

```
1. 在 main 分支完成功能开发 + 测试通过
2. 更新 Cargo.toml 中的 version 字段
3. 更新 CHANGELOG.md（可选）
4. git tag v<VERSION>
5. git push origin v<VERSION>
6. GitHub Actions 自动触发 release workflow
7. 检查 GitHub Releases 页面，确认产物正确
8. 补充 Release Notes（或使用 workflow 自动生成）
```

---

## 分支策略

```
main          — 稳定分支，所有 release 打 tag 于此
feat/*        — 功能开发分支
fix/*         — Bug 修复分支
```

所有 PR 必须通过 CI (`ci.yml`) 才能合并到 `main`。
