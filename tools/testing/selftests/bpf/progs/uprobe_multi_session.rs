//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uprobe_multi_session.c
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

    char _license[] SEC("license") = "GPL";
    let mut uprobe_multi_func_1_addr: __u64 = 0;
    let mut uprobe_multi_func_2_addr: __u64 = 0;
    let mut uprobe_multi_func_3_addr: __u64 = 0;
    __u64 uprobe_session_result[3] = {};
    let mut uprobe_multi_sleep_result: __u64 = 0;
    void *user_ptr = 0;
    let mut pid: c_int = 0;
#[no_mangle]
unsafe extern "C" fn uprobe_multi_check(ctx: *mut c_void, is_return: bool) -> c_int {
    static int uprobe_multi_check(void *ctx, bool is_return)
    {
    const __u64 funcs[] = {
    uprobe_multi_func_1_addr,
    uprobe_multi_func_2_addr,
    uprobe_multi_func_3_addr,
    };
    unsigned int i;
    __u64 addr;
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 1;
    addr = bpf_get_func_ip(ctx);
    for (i = 0; i < ARRAY_SIZE(funcs); i++) {
    if (funcs[i] == addr) {
    uprobe_session_result[i]++;
    break;
    }
    }
// only uprobe_multi_func_2 executes return probe
    if ((addr == uprobe_multi_func_1_addr) ||
    (addr == uprobe_multi_func_3_addr))
    return 1;
    return 0;
    }
    SEC("uprobe.session//proc/self/exe:uprobe_multi_func_*")
#[no_mangle]
pub unsafe extern "C" fn uprobe(ctx: *mut pt_regs) -> c_int {
    int uprobe(struct pt_regs *ctx)
    {
    return uprobe_multi_check(ctx, bpf_session_is_return(ctx));
    }
#[no_mangle]
unsafe extern "C" fn verify_sleepable_user_copy() -> __always_inline bool {
    static __always_inline bool verify_sleepable_user_copy(void)
    {
    char data[9];
    bpf_copy_from_user(data, sizeof(data), user_ptr);
    return bpf_strncmp(data, sizeof(data), "test_data") == 0;
    }
    SEC("uprobe.session.s//proc/self/exe:uprobe_multi_func_*")
#[no_mangle]
pub unsafe extern "C" fn uprobe_sleepable(ctx: *mut pt_regs) -> c_int {
    int uprobe_sleepable(struct pt_regs *ctx)
    {
    if (verify_sleepable_user_copy())
    uprobe_multi_sleep_result++;
    return uprobe_multi_check(ctx, bpf_session_is_return(ctx));
    }
