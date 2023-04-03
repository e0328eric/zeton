use std::ffi;

pub extern "C" fn sigchld(_unused: ffi::c_int) {
    if unsafe { libc::signal(libc::SIGCHLD, sigchld as usize) == libc::SIG_ERR } {
        panic!("can't install SIGCHLD handler:");
    }

    while unsafe { libc::waitpid(-1, std::ptr::null_mut(), libc::WNOHANG) } > 0 {}
}
