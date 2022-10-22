
pub mod event;

use graphics_binding::*;
use graphics_binding::sdl::event::*;
use graphics_binding::sdl::video::*;
use graphics_binding::sdl::init::*;

use crate::event::*;

pub fn run( /*event_processor : fn(Event, &mut T)*/ ) {

    use std::mem::MaybeUninit;
    use std::ffi::*;

    unsafe {

        SDL_Init(SDL_INIT_EVENTS | SDL_INIT_VIDEO);// TODO check result

        let title = &CString::new("the title").expect("CString::new failed");
        let window = SDL_CreateWindow(title.as_ptr(), 100, 100, 1000, 1000, SDL_WINDOW_RESIZABLE);
        loop {
            let mut event : MaybeUninit<SDL_Event> = MaybeUninit::uninit();
            let mut event_ptr = event.as_mut_ptr();
            let poll_result = SDL_PollEvent(event_ptr);
            // TODO check result
            if !event_ptr.is_null() {
                let e = event.assume_init();
                println!("event type = {}", e.event_type);
                match e.event_type {
                    SDL_QUIT => break,
                    _ => { },
                }
            }
            use std::{thread, time};
            let x = time::Duration::from_millis(100);
            thread::sleep(x);
        }
        SDL_DestroyWindow(window);

        SDL_Quit();

    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        run();
    }
}