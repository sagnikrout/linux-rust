//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mce/inject-ra-err.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

    static bool faulted;
#[no_mangle]
unsafe extern "C" fn sigbus_handler(n: c_int, info: *mut siginfo_t, ctxt_v: *mut c_void) {
    static void sigbus_handler(int n, siginfo_t *info, void *ctxt_v)
    {
    ucontext_t *ctxt = (ucontext_t *)ctxt_v;
    struct pt_regs *regs = ctxt.uc_mcontext.regs;
    faulted = true;
    regs.nip += 4;
    }
#[no_mangle]
unsafe extern "C" fn test_ra_error() -> c_int {
    static int test_ra_error(void)
    {
    struct vas_tx_win_open_attr attr;
    int fd, *paste_addr;
    char *devname = "/dev/crypto/nx-gzip";
    struct sigaction act = {
    .sa_sigaction = sigbus_handler,
    .sa_flags = SA_SIGINFO,
    };
    memset(&attr, 0, sizeof(attr));
    attr.version = 1;
    attr.vas_id = 0;
    SKIP_IF(access(devname, F_OK));
    fd = open(devname, O_RDWR);
    FAIL_IF(fd < 0);
    FAIL_IF(ioctl(fd, VAS_TX_WIN_OPEN, &attr) < 0);
    FAIL_IF(sigaction(SIGBUS, &act, core::ptr::null_mut()) != 0);
    paste_addr = mmap(core::ptr::null_mut(), 4096, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0ULL);
// The following assignment triggers exception
    mb();
// paste_addr = 1;
    mb();
    FAIL_IF(!faulted);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_ra_error, "inject-ra-err");
    }
