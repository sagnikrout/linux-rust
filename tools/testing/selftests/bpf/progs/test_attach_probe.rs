//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_attach_probe.c
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
// Copyright (c) 2017 Facebook

    let mut dynamic_sz: u32 = 1;
    let mut kprobe2_res: c_int = 0;
    let mut kretprobe2_res: c_int = 0;
    let mut uprobe_byname_res: c_int = 0;
    let mut uretprobe_byname_res: c_int = 0;
    let mut uprobe_byname2_res: c_int = 0;
    let mut uretprobe_byname2_res: c_int = 0;
    let mut uprobe_byname3_sleepable_res: c_int = 0;
    let mut uprobe_byname3_str_sleepable_res: c_int = 0;
    let mut uprobe_byname3_res: c_int = 0;
    let mut uretprobe_byname3_sleepable_res: c_int = 0;
    let mut uretprobe_byname3_str_sleepable_res: c_int = 0;
    let mut uretprobe_byname3_res: c_int = 0;
    void *user_ptr = 0;
    int bpf_copy_from_user_str(void *dst, u32, const void *, u64) __weak __ksym;
    SEC("ksyscall/nanosleep")
#[no_mangle]
pub unsafe extern "C" fn BPF_KSYSCALL(_arg: handle_kprobe_auto, req: *mut __kernel_timespec, rem: *mut __kernel_timespec) -> c_int {
    int BPF_KSYSCALL(handle_kprobe_auto, struct __kernel_timespec *req, struct __kernel_timespec *rem)
    {
    kprobe2_res = 11;
    return 0;
    }
    SEC("kretsyscall/nanosleep")
#[no_mangle]
pub unsafe extern "C" fn BPF_KRETPROBE(_arg: handle_kretprobe_auto, ret: c_int) -> c_int {
    int BPF_KRETPROBE(handle_kretprobe_auto, int ret)
    {
    kretprobe2_res = 22;
    return ret;
    }
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uprobe_ref_ctr(ctx: *mut pt_regs) -> c_int {
    int handle_uprobe_ref_ctr(struct pt_regs *ctx)
    {
    return 0;
    }
    SEC("uretprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uretprobe_ref_ctr(ctx: *mut pt_regs) -> c_int {
    int handle_uretprobe_ref_ctr(struct pt_regs *ctx)
    {
    return 0;
    }
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn handle_uprobe_byname(ctx: *mut pt_regs) -> c_int {
    int handle_uprobe_byname(struct pt_regs *ctx)
    {
    uprobe_byname_res = 5;
    return 0;
    }
// use auto-attach format for section definition.
    SEC("uretprobe//proc/self/exe:trigger_func2")
#[no_mangle]
pub unsafe extern "C" fn handle_uretprobe_byname(ctx: *mut pt_regs) -> c_int {
    int handle_uretprobe_byname(struct pt_regs *ctx)
    {
    uretprobe_byname_res = 6;
    return 0;
    }
    SEC("uprobe")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: handle_uprobe_byname2, pathname: *const c_char, mode: *const c_char) -> c_int {
    int BPF_UPROBE(handle_uprobe_byname2, const char *pathname, const char *mode)
    {
    char mode_buf[2] = {};
// verify fopen mode
    bpf_probe_read_user(mode_buf, sizeof(mode_buf), mode);
    if (mode_buf[0] == 'r' && mode_buf[1] == 0)
    uprobe_byname2_res = 7;
    return 0;
    }
    SEC("uretprobe")
#[no_mangle]
pub unsafe extern "C" fn BPF_URETPROBE(_arg: handle_uretprobe_byname2, ret: *mut c_void) -> c_int {
    int BPF_URETPROBE(handle_uretprobe_byname2, void *ret)
    {
    uretprobe_byname2_res = 8;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn verify_sleepable_user_copy() -> __always_inline bool {
    static __always_inline bool verify_sleepable_user_copy(void)
    {
    char data[9];
    bpf_copy_from_user(data, sizeof(data), user_ptr);
    return bpf_strncmp(data, sizeof(data), "test_data") == 0;
    }
#[no_mangle]
unsafe extern "C" fn verify_sleepable_user_copy_str() -> __always_inline bool {
    static __always_inline bool verify_sleepable_user_copy_str(void)
    {
    int ret;
    char data_long[20];
    char data_long_pad[20];
    char data_long_err[20];
    char data_short[4];
    char data_short_pad[4];
    ret = bpf_copy_from_user_str(data_short, sizeof(data_short), user_ptr, 0);
    if (bpf_strncmp(data_short, 4, "tes\0") != 0 || ret != 4)
    return false;
    ret = bpf_copy_from_user_str(data_short_pad, sizeof(data_short_pad), user_ptr, BPF_F_PAD_ZEROS);
    if (bpf_strncmp(data_short, 4, "tes\0") != 0 || ret != 4)
    return false;
// Make sure this passes the verifier
    ret = bpf_copy_from_user_str(data_long, dynamic_sz & sizeof(data_long), user_ptr, 0);
    if (ret != 0)
    return false;
    ret = bpf_copy_from_user_str(data_long, sizeof(data_long), user_ptr, 0);
    if (bpf_strncmp(data_long, 10, "test_data\0") != 0 || ret != 10)
    return false;
    ret = bpf_copy_from_user_str(data_long_pad, sizeof(data_long_pad), user_ptr, BPF_F_PAD_ZEROS);
    if (bpf_strncmp(data_long_pad, 10, "test_data\0") != 0 || ret != 10 || data_long_pad[19] != '\0')
    return false;
    ret = bpf_copy_from_user_str(data_long_err, sizeof(data_long_err), (void *)data_long, BPF_F_PAD_ZEROS);
    if (ret > 0 || data_long_err[19] != '\0')
    return false;
    ret = bpf_copy_from_user_str(data_long, sizeof(data_long), user_ptr, 2);
    if (ret != -EINVAL)
    return false;
    return true;
    }
    SEC("uprobe.s//proc/self/exe:trigger_func3")
#[no_mangle]
pub unsafe extern "C" fn handle_uprobe_byname3_sleepable(ctx: *mut pt_regs) -> c_int {
    int handle_uprobe_byname3_sleepable(struct pt_regs *ctx)
    {
    if (verify_sleepable_user_copy())
    uprobe_byname3_sleepable_res = 9;
    if (verify_sleepable_user_copy_str())
    uprobe_byname3_str_sleepable_res = 10;
    return 0;
    }
//
// same target as the uprobe.s above to force sleepable and non-sleepable
// programs in the same bpf_prog_array
//
    SEC("uprobe//proc/self/exe:trigger_func3")
#[no_mangle]
pub unsafe extern "C" fn handle_uprobe_byname3(ctx: *mut pt_regs) -> c_int {
    int handle_uprobe_byname3(struct pt_regs *ctx)
    {
    uprobe_byname3_res = 11;
    return 0;
    }
    SEC("uretprobe.s//proc/self/exe:trigger_func3")
#[no_mangle]
pub unsafe extern "C" fn handle_uretprobe_byname3_sleepable(ctx: *mut pt_regs) -> c_int {
    int handle_uretprobe_byname3_sleepable(struct pt_regs *ctx)
    {
    if (verify_sleepable_user_copy())
    uretprobe_byname3_sleepable_res = 12;
    if (verify_sleepable_user_copy_str())
    uretprobe_byname3_str_sleepable_res = 13;
    return 0;
    }
    SEC("uretprobe//proc/self/exe:trigger_func3")
#[no_mangle]
pub unsafe extern "C" fn handle_uretprobe_byname3(ctx: *mut pt_regs) -> c_int {
    int handle_uretprobe_byname3(struct pt_regs *ctx)
    {
    uretprobe_byname3_res = 14;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
