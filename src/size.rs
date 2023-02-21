static mut WIDTH: isize = 320;
static mut HEIGHT: isize = 240;

pub unsafe fn set_size(width: isize, height: isize) {
    WIDTH = width;
    HEIGHT = height;
}

pub fn get_size() -> (isize, isize) {
    unsafe {
        (WIDTH, HEIGHT)
    }
}