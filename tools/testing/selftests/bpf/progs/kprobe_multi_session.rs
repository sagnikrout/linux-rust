//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kprobe_multi_session.c
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
    extern const void bpf_fentry_test1 __ksym;
    extern const void bpf_fentry_test2 __ksym;
    extern const void bpf_fentry_test3 __ksym;
    extern const void bpf_fentry_test4 __ksym;
    extern const void bpf_fentry_test5 __ksym;
    extern const void bpf_fentry_test6 __ksym;
    extern const void bpf_fentry_test7 __ksym;
    extern const void bpf_fentry_test8 __ksym;
    let mut pid: c_int = 0;
    __u64 kprobe_session_result[8];
#[no_mangle]
unsafe extern "C" fn session_check(ctx: *mut c_void) -> c_int {
    static int session_check(void *ctx)
    {
    unsigned int i;
    __u64 addr;
    const void *kfuncs[] = {
    &bpf_fentry_test1,
    &bpf_fentry_test2,
    &bpf_fentry_test3,
    &bpf_fentry_test4,
    &bpf_fentry_test5,
    &bpf_fentry_test6,
    &bpf_fentry_test7,
    &bpf_fentry_test8,
    };
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 1;
    addr = bpf_get_func_ip(ctx);
    for (i = 0; i < ARRAY_SIZE(kfuncs); i++) {
    if (kfuncs[i] == (void *) addr) {
    kprobe_session_result[i]++;
    break;
    }
    }
//
// Force probes for function bpf_fentry_test[5-8] not to
// install and execute the return probe
//
    if (((const void *) addr == &bpf_fentry_test5) ||
    ((const void *) addr == &bpf_fentry_test6) ||
    ((const void *) addr == &bpf_fentry_test7) ||
    ((const void *) addr == &bpf_fentry_test8))
    return 1;
    return 0;
    }
//
// No tests in here, just to trigger 'bpf_fentry_test*'
// through tracing test_run
//
    SEC("fentry/bpf_modify_return_test")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trigger) -> c_int {
    int BPF_PROG(trigger)
    {
    return 0;
    }
    SEC("kprobe.session/bpf_fentry_test*")
#[no_mangle]
pub unsafe extern "C" fn test_kprobe(ctx: *mut pt_regs) -> c_int {
    int test_kprobe(struct pt_regs *ctx)
    {
    return session_check(ctx);
    }
//
// Exact function name (no wildcards) - exercises the fast syms[] path
// in bpf_program__attach_kprobe_multi_opts() which bypasses kallsyms parsing.
//
    SEC("kprobe.session/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn test_kprobe_syms(ctx: *mut pt_regs) -> c_int {
    int test_kprobe_syms(struct pt_regs *ctx)
    {
    return session_check(ctx);
    }
