//! File and filesystem-related syscalls

use crate::batch::{APP_BASE_ADDRESS, USER_STACK_SIZE};
use core::arch::asm;

const FD_STDOUT: usize = 1;

unsafe fn r_sp() -> usize {
    let mut sp: usize;
    asm!("mv {}, sp", out(reg) sp);
    sp
}

unsafe fn stack_range() -> (usize, usize) {
    let sp = r_sp();
    let top = (sp + USER_STACK_SIZE - 1) & (!(USER_STACK_SIZE - 1));
    (top - USER_STACK_SIZE, top)
}

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    unsafe {
        let (bottom, top) = stack_range();
        if (buf as usize) <= APP_BASE_ADDRESS
            && ((buf as usize) < bottom || (buf as usize) + len > top)
        {
            return -1;
        }
    }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            return -1;
        }
    }
}
