
pub mod event;

use crate::event::*;

pub fn run<T>( event_processor : fn(Event, &mut T) ) {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {

    }
}