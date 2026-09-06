//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_probe_user.c
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_pro_bss {
    pub old: sockaddr_in,
    pub test_pid: __u32,
}

    struct test_pro_bss bss;
#[no_mangle]
unsafe extern "C" fn handle_sys_connect_common(uservaddr: *mut sockaddr_in) -> c_int {
    static int handle_sys_connect_common(struct sockaddr_in *uservaddr)
    {
    struct sockaddr_in new;
    let mut cur: __u32 = bpf_get_current_pid_tgid() >> 32;
    if (bss.test_pid && cur != bss.test_pid)
    return 0;
    bpf_probe_read_user(&bss.old, sizeof(bss.old), uservaddr);
    __builtin_memset(&new, 0xab, sizeof(new));
    bpf_probe_write_user(uservaddr, &new, sizeof(new));
    return 0;
    }
    SEC("ksyscall/connect")
    int BPF_KSYSCALL(handle_sys_connect, int fd, struct sockaddr_in *uservaddr,
    int addrlen)
    {
    return handle_sys_connect_common(uservaddr);
    }

pub const SYS_CONNECT: c_int = 3;

    SEC("ksyscall/socketcall")
#[no_mangle]
pub unsafe extern "C" fn BPF_KSYSCALL(_arg: handle_sys_socketcall, call: c_int, args: *mut c_ulong) -> c_int {
    int BPF_KSYSCALL(handle_sys_socketcall, int call, unsigned long *args)
    {
    if (call == SYS_CONNECT) {
    struct sockaddr_in *uservaddr;
    bpf_probe_read_user(&uservaddr, sizeof(uservaddr), &args[1]);
    return handle_sys_connect_common(uservaddr);
    }
    return 0;
    }

    char _license[] SEC("license") = "GPL";
