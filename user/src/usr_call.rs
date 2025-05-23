use crate::syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn getchar() -> u8 {
    let mut buf: [u8; 1] = [0u8];
    sys_read(0, &mut buf);
    buf[0]
}
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
}
pub fn yield_() -> isize {
    sys_yield()
}
pub fn get_time() -> isize {
    sys_get_time()
}

pub fn fork() -> isize {
    sys_fork()
}
pub fn exec(path: &str, args: &[*const u8], envp: &[*const u8]) -> isize {
    sys_exec(path, args, envp)
}
pub fn wait(exit_code: &mut i32) -> isize {
    sys_waitpid(-1, exit_code as *mut _)
}

pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    sys_waitpid(pid as isize, exit_code as *mut _)
}
pub fn sleep(period_ms: usize) {
    let start = sys_get_time();
    while sys_get_time() < start + period_ms as isize {
        sys_yield();
    }
}

pub fn shutdown() -> isize {
    sys_shutdown()
}
