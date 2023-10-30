use druid::widget::{Button, Flex};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};
use screenshots::Screen;

fn build_ui() -> impl Widget<u32> {
    // Un semplice bottone
    let button = Button::new("Ciao, mondo!").on_click(|_ctx, _data, _env| screenshot());

    // Aggiungi il bottone a un layout
    Flex::column().with_child(button)
}

fn screenshot() {
    let screens = Screen::all().unwrap();

    let screen = screens[0].clone();

    println!("capturer: {:?}", screen);
    let image = screen.capture().unwrap();
    image
        .save(format!("target/prova.png"))
        .unwrap();
}

fn main() -> Result<(), PlatformError> {
    // Descrizione della finestra principale
    let main_window = WindowDesc::new(build_ui()).title("Applicazione Druid di Base");

    // Esegui l'applicazione
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(0)
}
