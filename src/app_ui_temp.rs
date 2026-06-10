use makepad_widgets::*;

live_design! {
    use makepad_widgets::theme_desktop_dark::*;

    App = {{App}} {
        ui: <Root> {
            main_window: <Window> {
                show_bg: true,
                draw_bg: {
                    color: #0
                }
                window: {inner_size: vec2(800., 600.), title: "PinPonCAD"},
                caption_bar = {
                    caption_label = {
                        label = {text: "PinPonCAD"}
                    }
                }
                body = <View> {
                    flow: Down, spacing: 20, padding: 20
                    label_text = <Label> {
                        draw_text: { color: #0f0 },
                        text: "PinPonCAD test app"
                    }
                    button_1 = <Button> {
                        draw_bg: { color: #00f },
                        text: "Click me"
                    }
                }
            }
        }
    }
}
