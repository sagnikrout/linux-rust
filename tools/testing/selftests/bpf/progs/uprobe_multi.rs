//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/uprobe_multi.c
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
    let mut uprobe_multi_func_1_result: __u64 = 0;
    let mut uprobe_multi_func_2_result: __u64 = 0;
    let mut uprobe_multi_func_3_result: __u64 = 0;
    let mut uretprobe_multi_func_1_result: __u64 = 0;
    let mut uretprobe_multi_func_2_result: __u64 = 0;
    let mut uretprobe_multi_func_3_result: __u64 = 0;
    let mut uprobe_multi_sleep_result: __u64 = 0;
    let mut pid: c_int = 0;
    let mut child_pid: c_int = 0;
    let mut child_tid: c_int = 0;
    let mut child_pid_usdt: c_int = 0;
    let mut child_tid_usdt: c_int = 0;
    let mut expect_pid: c_int = 0;
    let mut bad_pid_seen: bool = false;
    let mut bad_pid_seen_usdt: bool = false;
    let mut test_cookie: bool = false;
    void *user_ptr = 0;
#[no_mangle]
unsafe extern "C" fn verify_sleepable_user_copy() -> __always_inline bool {
    static __always_inline bool verify_sleepable_user_copy(void)
    {
    char data[9];
    bpf_copy_from_user(data, sizeof(data), user_ptr);
    return bpf_strncmp(data, sizeof(data), "test_data") == 0;
    }
#[no_mangle]
unsafe extern "C" fn uprobe_multi_check(ctx: *mut c_void, is_return: bool, is_sleep: bool) {
    static void uprobe_multi_check(void *ctx, bool is_return, bool is_sleep)
    {
    let mut cur_pid_tgid: __u64 = bpf_get_current_pid_tgid();
    __u32 cur_pid;
    cur_pid = cur_pid_tgid >> 32;
    if (pid && cur_pid != pid)
    return;
    if (expect_pid && cur_pid != expect_pid)
    bad_pid_seen = true;
    child_pid = cur_pid_tgid >> 32;
    child_tid = (__u32)cur_pid_tgid;
    let mut cookie: __u64 = test_cookie ? bpf_get_attach_cookie(ctx) : 0;
    let mut addr: __u64 = bpf_get_func_ip(ctx);

    if (addr == __addr &&				\
    (!test_cookie || (cookie == __cookie)))	\
    __var += 1;				\
    })
    if (is_return) {
    SET(uretprobe_multi_func_1_result, uprobe_multi_func_1_addr, 2);
    SET(uretprobe_multi_func_2_result, uprobe_multi_func_2_addr, 3);
    SET(uretprobe_multi_func_3_result, uprobe_multi_func_3_addr, 1);
    } else {
    SET(uprobe_multi_func_1_result, uprobe_multi_func_1_addr, 3);
    SET(uprobe_multi_func_2_result, uprobe_multi_func_2_addr, 1);
    SET(uprobe_multi_func_3_result, uprobe_multi_func_3_addr, 2);
    }

    if (is_sleep && verify_sleepable_user_copy())
    uprobe_multi_sleep_result += 1;
    }
    SEC("uprobe.multi//proc/self/exe:uprobe_multi_func_*")
#[no_mangle]
pub unsafe extern "C" fn uprobe(ctx: *mut pt_regs) -> c_int {
    int uprobe(struct pt_regs *ctx)
    {
    uprobe_multi_check(ctx, false, false);
    return 0;
    }
    SEC("uretprobe.multi//proc/self/exe:uprobe_multi_func_*")
#[no_mangle]
pub unsafe extern "C" fn uretprobe(ctx: *mut pt_regs) -> c_int {
    int uretprobe(struct pt_regs *ctx)
    {
    uprobe_multi_check(ctx, true, false);
    return 0;
    }
    SEC("uprobe.multi.s//proc/self/exe:uprobe_multi_func_*")
#[no_mangle]
pub unsafe extern "C" fn uprobe_sleep(ctx: *mut pt_regs) -> c_int {
    int uprobe_sleep(struct pt_regs *ctx)
    {
    uprobe_multi_check(ctx, false, true);
    return 0;
    }
    SEC("uretprobe.multi.s//proc/self/exe:uprobe_multi_func_*")
#[no_mangle]
pub unsafe extern "C" fn uretprobe_sleep(ctx: *mut pt_regs) -> c_int {
    int uretprobe_sleep(struct pt_regs *ctx)
    {
    uprobe_multi_check(ctx, true, true);
    return 0;
    }
    SEC("uprobe.multi//proc/self/exe:uprobe_multi_func_*")
#[no_mangle]
pub unsafe extern "C" fn uprobe_extra(ctx: *mut pt_regs) -> c_int {
    int uprobe_extra(struct pt_regs *ctx)
    {
// we need this one just to mix PID-filtered and global uprobes
    return 0;
    }
    SEC("usdt")
#[no_mangle]
pub unsafe extern "C" fn usdt_pid(ctx: *mut pt_regs) -> c_int {
    int usdt_pid(struct pt_regs *ctx)
    {
    let mut cur_pid_tgid: __u64 = bpf_get_current_pid_tgid();
    __u32 cur_pid;
    cur_pid = cur_pid_tgid >> 32;
    if (pid && cur_pid != pid)
    return 0;
    if (expect_pid && cur_pid != expect_pid)
    bad_pid_seen_usdt = true;
    child_pid_usdt = cur_pid_tgid >> 32;
    child_tid_usdt = (__u32)cur_pid_tgid;
    return 0;
    }
    SEC("usdt")
#[no_mangle]
pub unsafe extern "C" fn usdt_extra(ctx: *mut pt_regs) -> c_int {
    int usdt_extra(struct pt_regs *ctx)
    {
// we need this one just to mix PID-filtered and global USDT probes
    return 0;
    }
