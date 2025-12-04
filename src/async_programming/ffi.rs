pub const EPOLL_CTRL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1;
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
unsafe extern "C" {
    pub fn epoll_create(size: i32) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *const Event) -> i32;
    pub fn epoll_wait(epfd: i32, events: *const Event, maxevents: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
// 保持和原始C 结构体一致的内存布局（https://elixir.bootlin.com/linux/v6.18/source/include/uapi/linux/eventpoll.h#L83）
#[repr(C, packed)]
pub struct Event {
    pub(crate) events: u32,
    // token to identify the event source
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}