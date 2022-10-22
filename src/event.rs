
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
}

#[derive(Debug)]
pub enum Event {
    Window(Window),
}