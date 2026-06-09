use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

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

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
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

// CADViewport3D

#[derive(Live, Widget)]
pub struct CadViewport3D {
    #[deref]
    view: View,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[rust]
    camera_zoom: f32,
    #[rust]
    camera_rotation: Vec2,
    #[rust]
    pan_offset: Vec3,
}

impl Widget for CadViewport3D {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl LiveHook for CadViewport3D {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.camera_zoom = 50.0;
        self.camera_rotation = vec2(-30.0, 45.0);
        self.pan_offset = vec3(0.0, 0.0, 0.0);
    }
}

// main
fn main() {
    // call the generated `app_main()` to start the app/event loop
    app_main();
}

// Register the app type for the generated `app_main()` function.
app_main!(App);
