# Vue 3 + TypeScript + Vite

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

Learn more about the recommended Project Setup and IDE Support in the [Vue Docs TypeScript Guide](https://vuejs.org/guide/typescript/overview.html#project-setup).

启动命令

```bash
cargo tauri dev
```

对应内容
【1.文本数据转为JSON数据 of "Rust 实战：电影信息维护（命令行版）"】
https://www.bilibili.com/video/BV1spJVzCEv8?vd_source=815da4d219cd869eab3c60217436c2aa

【3.实现CRUD of "Rust 实战：电影信息维护（命令行版）"】
https://www.bilibili.com/video/BV1NwjtzCE7W?vd_source=815da4d219cd869eab3c60217436c2aa



【25、练习4：进军Web开发1-Axum初体验】
https://www.bilibili.com/video/BV1YpJWzWEjk?vd_source=815da4d219cd869eab3c60217436c2aa


使用 sea-orm-cli 从数据库初始化实体
```bash
sea-orm-cli generate entity --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --date-time-crate chrono -o ./src/entity
```
https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/


