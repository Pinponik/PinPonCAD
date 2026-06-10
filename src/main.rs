use makepad_widgets::*;
use pinponcad::app::App;

// Register the app type for the generated `app_main()` function.
app_main!(App);

fn main() {
    // call the generated `app_main()` to start the app/event loop
    app_main();
}
