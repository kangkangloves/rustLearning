#![allow(non_snake_case)]
// 导入warp库中的Filter和Reply trait
use warp::{Filter, Reply};

// 使用tokio宏标记main函数，以便它可以在异步上下文中运行
#[tokio::main]
async fn main() {
    // 创建一个新的路由，当请求路径为/时，返回一个包含“Hello, World!”文本的响应
    let routes = warp::path::end().map(|| "Hello, World!");

    // 启动一个新的HTTP服务器，监听本地主机的端口3000，并运行路由
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}