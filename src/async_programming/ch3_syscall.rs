use std::{arch::asm, io};

// unix, mac都适合写出这样的syscall 代码，但windows 据说规则不稳定，比如write的编号会变
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let msg_len = message.len();

    unsafe {
        asm!(
            "mov rax, 1",    // syscall number for write
            "mov rdi, 1",    // file descriptor 1 is stdout
            "syscall",         // invoke operating system to do the write
            in("rsi") msg_ptr,
            in("rdx") msg_len,
            out("rax") _,  // we don't care about the return value
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _
        );
    }
}

#[cfg(target_family = "unix")]
#[link(name = "c")]
unsafe extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

fn syscall_ffi(msg: String) -> io::Result<()> {
    let msg_ptr = msg.as_ptr();
    let len = msg.len();
    let res = unsafe { write(1, msg_ptr, len) };
    if res == -1 {
        // This function reads the value of errno for the target platform
        // (e.g. GetLastError on Windows) and will return a corresponding instance of Error for the error code.
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

pub fn t3_main() {
    let message = String::from("Hello from syscall!\n");
    //syscall(message);
    syscall_ffi(message).unwrap();
}