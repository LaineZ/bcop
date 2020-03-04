use crate::bop_core;
use crate::bop_core::playback;
use crate::model::album::Album;
use bytes::Bytes;

use druid::widget::{Align, Button, Flex, Label, Padding, WidgetExt};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc};

pub async fn gui_mode(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
    Ok(())
}

fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text)
        .padding(5.0)
        .center();
    let button = Button::new("increment", |_ctx, data, _env| *data += 1)
        .padding(5.0);

    let mut col = Flex::column();
    col.add_child(label, 1.0);
    col.add_child(button, 1.0);
    col
}