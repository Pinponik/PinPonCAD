use makepad_widgets::*;

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

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

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(button_1)).clicked(&actions) {
            println!("Button clicked!");
            let button_1 = self.ui.button(id!(button_1));
            button_1.set_text(cx, "Hello, World!");
            button_1.redraw(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);

        if let Event::Draw(draw_event) = event {
            let mut cx_draw = CxDraw::new(cx, draw_event);
            let mut cx2d = Cx2d::new(&mut cx_draw);
            let mut scope = Scope::empty();
            self.ui.draw_all(&mut cx2d, &mut scope);
            return;
        }

        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
