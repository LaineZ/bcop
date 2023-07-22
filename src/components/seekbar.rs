use dioxus::prelude::*;


#[derive(Props)]
pub struct SeekbarProps<'a> {
    pub min: u32,
    pub max: u32,
    pub on_value_change: EventHandler<'a, u32>,
}

pub fn seekbar<'a>(cx: Scope<'a, SeekbarProps<'a>>) -> Element<'a> {
    let value = use_state(cx, || 0 as u32);

    cx.render(rsx!( div {
        class: "seekbar",
        style: "background: linear-gradient(
            to right, 
            var(--fg),
            var(--fg) {value}%,
            var(--bg1) {value}%,
            var(--bg1)
          );",
        input {
            r#type: "range",
            min: 0,
            max: 320,
            oninput: move |e| {
                value.set((e.value.parse::<f32>().unwrap_or_default() / 320.0 * 100.0) as u32);
                cx.props.on_value_change.call(*value.get());
            },
            onchange: move |e| {
                value.set((e.value.parse::<f32>().unwrap_or_default() / 320.0 * 100.0) as u32);
                cx.props.on_value_change.call(*value.get());
            },
        }
    }))
}