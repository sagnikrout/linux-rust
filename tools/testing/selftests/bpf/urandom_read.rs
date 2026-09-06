//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/urandom_read.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


pub const _SDT_HAS_SEMAPHORES: c_int = 1;

pub const SHARED: c_int = 1;

pub const BUF_SIZE: c_int = 256;
// defined in urandom_read_aux.c
    void urand_read_without_sema(int iter_num, int iter_cnt, int read_sz);
// these are coming from urandom_read_lib{1,2}.c
    void urandlib_read_with_sema(int iter_num, int iter_cnt, int read_sz);
    void urandlib_read_without_sema(int iter_num, int iter_cnt, int read_sz);
    int urandlib_api(void);
    COMPAT_VERSION(urandlib_api_old, urandlib_api, LIBURANDOM_READ_1.0.0)
    int urandlib_api_old(void);
    int urandlib_api_sameoffset(void);
    unsigned short urand_read_with_sema_semaphore SEC(".probes");
#[no_mangle]
unsafe extern "C" fn urandom_read(fd: c_int, count: c_int) -> noinline void {
    static noinline void urandom_read(int fd, int count)
    {
    char buf[BUF_SIZE];
    int i;
    for (i = 0; i < count; ++i) {
    read(fd, buf, BUF_SIZE);
// trigger USDTs defined in executable itself
    urand_read_without_sema(i, count, BUF_SIZE);
    STAP_PROBE3(urand, read_with_sema, i, count, BUF_SIZE);
// trigger USDTs defined in shared lib
    urandlib_read_without_sema(i, count, BUF_SIZE);
    urandlib_read_with_sema(i, count, BUF_SIZE);
    }
    }
    static volatile bool parent_ready;
#[no_mangle]
unsafe extern "C" fn handle_sigpipe(sig: c_int) {
    static void handle_sigpipe(int sig)
    {
    parent_ready = true;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut fd: c_int = open("/dev/urandom", O_RDONLY);
    let mut count: c_int = 4;
    let mut report_pid: bool = false;
    if (fd < 0)
    return 1;
    if (argc >= 2)
    count = atoi(argv[1]);
    if (argc >= 3) {
    report_pid = true;
// install SIGPIPE handler to catch when parent closes their
// end of the pipe (on the other side of our stdout)
//
    signal(SIGPIPE, handle_sigpipe);
    }
// report PID and wait for parent process to send us "signal" by
// closing stdout
//
    if (report_pid) {
    while (!parent_ready) {
    fprintf(stdout, "%d\n", getpid());
    fflush(stdout);
    }
// at this point stdout is closed, parent process knows our
// PID and is ready to trace us
//
    }
    urandom_read(fd, count);
    urandlib_api();
    urandlib_api_old();
    urandlib_api_sameoffset();
    close(fd);
    return 0;
    }
