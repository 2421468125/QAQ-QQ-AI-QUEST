# 项目层级结构

[Cargo.toml](Cargo.toml) <p style="font-size:12px">配置文件</p>

## src

- [config.rs](src/config.rs)<p style="font-size:12px">存储常量</p>
- [db.rs](src/db.rs)<p style="font-size:12px">对数据库的直接操作</p>
- [handlers.rs](src/handlers.rs)<p style="font-size:12px">路由请求处理</p>
- [lib.rs](src/lib.rs)<p style="font-size:12px">模块声明</p>
- [main.rs](src/main.rs)<p style="font-size:12px">程序入口</p>
- [models.rs](src/models.rs)<p style="font-size:12px">数据模型</p>
- [pipeline.rs](src/pipeline.rs)<p style="font-size:12px">消息流水线</p>
- [routes.rs](src/routes.rs)<p style="font-size:12px">配置路由</p>
- [services.rs](src/services.rs)<p style="font-size:12px">API 请求</p>
- ll_one_bot <p style="font-size:12px">与 LLOneBot 相关代码</p>
- - [interface.rs](src/ll_one_bot/interface.rs) <p style="font-size:12px">接口</p>

## fore-end/src

- [App.vue](fore-end/src/App.vue)<p style="font-size:12px">前端网页</p>
- [utils.js](fore-end/src/utils.js)<p style="font-size:12px">axois 相关代码</p>
- components<p style="font-size:12px">组件文件夹</p>
- - [Parameter.vue](fore-end/src/components/Parameter.vue)<p style="font-size:12px">修改参数组件</p>
