use std::{io::{self, Result}, net::TcpStream, os::fd::AsRawFd};
use super::ffi;

type Events = Vec<ffi::Event>;

pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        let res = unsafe { ffi::epoll_create(1) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self {
            registry: Registry { raw_fd: res },
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let epoll_fd = self.registry.raw_fd;
        let timeout = timeout.unwrap_or(-1); // -1 means wait indefinitely
        let max_events = events.capacity() as i32;
        let res = unsafe {
            ffi::epoll_wait(
                epoll_fd,
                // 传 as_ptr 逻辑上不对，可能有UB
                events.as_mut_ptr(),
                //events.as_ptr(),
                max_events,
                timeout,
            )
        };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        unsafe {
            // 访问的内存不是在rust中初使化的
            events.set_len(res as usize);
        }
        Ok(())
    }
}

pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    // 暂未为source使用泛型
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        let event = ffi::Event {
            events: interests as u32,
            epoll_data: token,
        };
        let op = ffi::EPOLL_CTRL_ADD;
        let res = unsafe {
            ffi::epoll_ctl(self.raw_fd, op, source.as_raw_fd(), &event)
        };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe { ffi::close(self.raw_fd) };
        if res < 0 {
            let err = io::Error::last_os_error();
            eprintln!("Failed to close epoll fd {}: {}", self.raw_fd, err);
        }
    }
}