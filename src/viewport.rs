use makepad_widgets::*;

#[derive(Live, LiveHook)]
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

impl LiveRegister for CadViewport3D {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl WidgetNode for CadViewport3D {
    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        self.view.uid_to_widget(uid)
    }

    fn find_widgets(&self, path: &[LiveId], cache: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cache, results)
    }

    fn area(&self) -> Area {
        self.view.area()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx)
    }
}

impl Widget for CadViewport3D {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
