use clipboard::{ClipboardContext, ClipboardProvider};
use fltk::{app, input, prelude::*, window::{self}};
use fltk::button::Button;
use tokio::runtime::Builder;

mod uuid;
mod properties;
mod base_properties;
mod http_utils;

fn main() {
    app::background(255, 255, 255 );
    app::foreground(0, 0, 0 );
    let mut a = app::App::default();
    open_window();
    a.set_scheme(app::Scheme::Gtk);
    a.run().unwrap();
}

fn open_window() {
    // 创建并启动 Tokio 运行时
    let rt = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut wind = window::Window::new(100, 100, 400, 300, "mc小工具");
    wind.begin();
    let mut username = input::Input::new(100, 100, 250, 40, "用户名");
    username.set_value("wuqiyang101");
    let mut button = Button::new(100, 170, 200, 40, "查询");
    button.set_callback(move |b| {
        b.set_label("查询中...");
        // 获取输入框中的用户名
        let username = username.value();
        rt.block_on(async {
            // 使用get_uuid函数获取指定用户名的UUID
            let uuid = http_utils::get_uuid(username).await.expect("常见错误");
            // 使用get_skin函数获取指定UUID的皮肤和披风的URL
            let (skin, cape) = http_utils::get_skin_and_cape_url(uuid).await.expect("常见错误");
            let mut wind = window::Window::new(100, 100, 400, 300, "mc小工具");
            wind.begin();
            let mut skin_url = input::Input::new(100, 10, 200, 20, "皮肤URL");
            skin_url.set_value(&skin);
            let mut cape_url = input::Input::new(100, 40, 200, 20, "披风URL");
            cape_url.set_value(&cape);
            // 一键复制
            let mut copy_button = Button::new(100, 70, 200, 30, "复制皮肤URL");
            copy_button.set_callback(move |_b| {
                // 使用fltk库复制文本到剪贴板
                // 创建剪贴板上下文
                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                clipboard.set_contents(skin.clone()).unwrap()
            });
            let mut copy_button2 = Button::new(100, 110, 200, 30, "复制披风URL");
            copy_button2.set_callback(move |_b| {
                // 使用fltk库复制文本到剪贴板
                // 创建剪贴板上下文
                let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                clipboard.set_contents(cape.clone()).unwrap()
            });
            wind.end();
            wind.show();
            b.set_label("查询");
        });
    });
    wind.end();
    wind.show();
}

