use graphics_binding::sdl::event::*;
use graphics_binding::sdl::window::*;
use graphics_binding::sdl::keyboard::*;

#[derive(Debug)]
pub struct Timestamp(pub u32);

#[derive(Debug)]
pub enum Window {
    Shown(Timestamp),
    Hidden(Timestamp),
    Exposed(Timestamp),
    Moved { timestamp : Timestamp, x : i32, y : i32 },
    Resized { timestamp : Timestamp, width : i32, height : i32 },
    SizeChanged { timestamp : Timestamp, width : i32, height : i32 },
    Minimized(Timestamp),
    Maximized(Timestamp),
    Restored(Timestamp),
    Enter(Timestamp),
    Leave(Timestamp),
    FocusGained(Timestamp),
    FocusLost(Timestamp),
    Close(Timestamp),
    TakeFocus(Timestamp),
    HitTest(Timestamp),
    IccProfChanged(Timestamp),
    DisplayChanged(Timestamp),
}

#[derive(Debug)]

#[derive(Debug)]
pub enum Event {
    Window(Window),
    Quit,
    Ignore,
}

pub fn convert(event : SDL_Event) -> Event {
    unsafe {
        match event.event_type {
            SDL_WINDOWEVENT => {
                let timestamp = Timestamp(event.window.timestamp);
                let window = match event.window.event {
                    SDL_WINDOWEVENT_SHOWN => Window::Shown(timestamp),
                    SDL_WINDOWEVENT_HIDDEN => Window::Hidden(timestamp),
                    SDL_WINDOWEVENT_EXPOSED => Window::Exposed(timestamp),
                    SDL_WINDOWEVENT_MOVED => Window::Moved { timestamp, x: event.window.data_1, y: event.window.data_2 },
                    SDL_WINDOWEVENT_RESIZED => Window::Resized { timestamp, width: event.window.data_1, height: event.window.data_2 },
                    SDL_WINDOWEVENT_SIZE_CHANGED => Window::SizeChanged { timestamp, width: event.window.data_1, height: event.window.data_2 },
                    SDL_WINDOWEVENT_MINIMIZED => Window::Minimized(timestamp),
                    SDL_WINDOWEVENT_MAXIMIZED => Window::Maximized(timestamp),
                    SDL_WINDOWEVENT_RESTORED => Window::Restored(timestamp),
                    SDL_WINDOWEVENT_ENTER => Window::Enter(timestamp),
                    SDL_WINDOWEVENT_LEAVE => Window::Leave(timestamp),
                    SDL_WINDOWEVENT_FOCUS_GAINED => Window::FocusGained(timestamp),
                    SDL_WINDOWEVENT_FOCUS_LOST => Window::FocusLost(timestamp),
                    SDL_WINDOWEVENT_HIT_TEST => Window::HitTest(timestamp),
                    SDL_WINDOWEVENT_ICCPROF_CHANGED => Window::IccProfChanged(timestamp),
                    SDL_WINDOWEVENT_DISPLAY_CHANGED => Window::DisplayChanged(timestamp),
                    _ => { return Event::Ignore },
                };
                Event::Window(window)
            },
            SDL_QUIT => Event::Quit,
            _ => Event::Ignore,
        }
    }
}