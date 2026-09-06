//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/get_func_ip_test.c
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
    extern int bpf_fentry_test1(int a) __ksym;
    extern int bpf_modify_return_test(int a, int *b) __ksym;
    extern const void bpf_fentry_test2 __ksym;
    extern const void bpf_fentry_test3 __ksym;
    extern const void bpf_fentry_test4 __ksym;
    extern bool CONFIG_X86_KERNEL_IBT __kconfig __weak;
// This function is here to have CONFIG_X86_KERNEL_IBT
// used and added to object BTF.
//
#[no_mangle]
pub unsafe extern "C" fn unused() -> c_int {
    int unused(void)
    {
    return CONFIG_X86_KERNEL_IBT ? 0 : 1;
    }
    let mut test1_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test1, a: c_int) -> c_int {
    int BPF_PROG(test1, int a)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    test1_result = (const void *) addr == &bpf_fentry_test1;
    return 0;
    }
    let mut test2_result: __u64 = 0;
    SEC("fexit/bpf_fentry_test2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test2, a: c_int) -> c_int {
    int BPF_PROG(test2, int a)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    test2_result = (const void *) addr == &bpf_fentry_test2;
    return 0;
    }
    let mut test3_result: __u64 = 0;
    SEC("kprobe/bpf_fentry_test3")
#[no_mangle]
pub unsafe extern "C" fn test3(ctx: *mut pt_regs) -> c_int {
    int test3(struct pt_regs *ctx)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    test3_result = (const void *) addr == &bpf_fentry_test3;
    return 0;
    }
    let mut test4_result: __u64 = 0;
    SEC("kretprobe/bpf_fentry_test4")
#[no_mangle]
pub unsafe extern "C" fn BPF_KRETPROBE(_arg: test4) -> c_int {
    int BPF_KRETPROBE(test4)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    test4_result = (const void *) addr == &bpf_fentry_test4;
    return 0;
    }
    let mut test5_result: __u64 = 0;
    SEC("fmod_ret/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test5, a: c_int, b: *mut c_int, ret: c_int) -> c_int {
    int BPF_PROG(test5, int a, int *b, int ret)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    test5_result = (const void *) addr == &bpf_modify_return_test;
    return ret;
    }
    let mut test6_result: __u64 = 0;
    SEC("?kprobe")
#[no_mangle]
pub unsafe extern "C" fn test6(ctx: *mut pt_regs) -> c_int {
    int test6(struct pt_regs *ctx)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    test6_result = (const void *) addr == 0;
    return 0;
    }
    unsigned long uprobe_trigger;
    let mut test7_result: __u64 = 0;
    SEC("uprobe//proc/self/exe:uprobe_trigger")
#[no_mangle]
pub unsafe extern "C" fn BPF_UPROBE(_arg: test7) -> c_int {
    int BPF_UPROBE(test7)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    test7_result = (const void *) addr == (const void *) uprobe_trigger;
    return 0;
    }
    let mut test8_result: __u64 = 0;
    SEC("uretprobe//proc/self/exe:uprobe_trigger")
#[no_mangle]
pub unsafe extern "C" fn BPF_URETPROBE(_arg: test8, ret: c_int) -> c_int {
    int BPF_URETPROBE(test8, int ret)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    test8_result = (const void *) addr == (const void *) uprobe_trigger;
    return 0;
    }
