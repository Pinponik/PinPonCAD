use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                window: {inner_size: vec2(800., 600.)}, // Opcjonalnie: rozmiar okna
                body = <View> {
                    flow: Down, spacing: 20, padding: 20
                    label = <Label> {
                        draw_text: { color: #0f0 }, // Zielony tekst, by był widoczny
                        text: "PinPonCAD"
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

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // 1. Obsługa rysowania okna (Kluczowy brakujący krok!)
        if let Event::Draw(draw_event) = event {
            let mut cx_draw = CxDraw::new(cx, draw_event);
            let mut cx2d = Cx2d::new(&mut cx_draw);
            let mut scope = Scope::empty();
            self.ui.draw_all(&mut cx2d, &mut scope);
            return;
        }

        // 2. Obsługa pozostałych zdarzeń
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

fn main() {
    app_main!(App);
    // call the generated `app_main()` to start the app/event loop
    app_main();
}
