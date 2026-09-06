//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/linked_funcs2.c
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
// Copyright (c) 2021 Facebook

// weak and shared between both files
    const volatile int my_tid __weak;
    long syscall_id __weak;
    int output_val2;
    int output_ctx2;
    int output_weak2; /* should stay zero */
// same "subprog" name in all files, but it's ok because they all are static
#[no_mangle]
unsafe extern "C" fn subprog(x: c_int) -> __noinline int {
    static __noinline int subprog(int x)
    {
// but different formula
    return x * 2;
    }
// Global functions can't be void
#[no_mangle]
pub unsafe extern "C" fn set_output_val2(x: c_int) -> c_int {
    int set_output_val2(int x)
    {
    output_val2 = 2 * x + 2 * subprog(x);
    return 2 * x;
    }
// This function can't be verified as global, as it assumes raw_tp/sys_enter
// context and accesses syscall id (second argument). So we mark it as
// __hidden, so that libbpf will mark it as static in the final object file,
// right before verifying it in the kernel.
//
// But we don't mark it as __hidden here, rather at extern site. __hidden is
// "contaminating" visibility, so it will get propagated from either extern or
// actual definition (including from the losing __weak definition).
//
#[no_mangle]
pub unsafe extern "C" fn set_output_ctx2(ctx: *mut __u64) {
    void set_output_ctx2(__u64 *ctx)
    {
    output_ctx2 = ctx[1]; /* long id, same as in BPF_PROG below */
    }
// this weak instance should lose, because it will be processed second
#[no_mangle]
pub unsafe extern "C" fn set_output_weak(x: c_int) -> __weak int {
    __weak int set_output_weak(int x)
    {
    static volatile int whatever;
// make sure we use CO-RE relocations in a weak function, this used to
// cause problems for BPF static linker
//
    whatever = 2 * bpf_core_type_size(struct task_struct);
    __sink(whatever);
    output_weak2 = x;
    return 2 * x;
    }
    extern int set_output_val1(int x);
// here we'll force set_output_ctx1() to be __hidden in the final obj file
    __hidden extern void set_output_ctx1(__u64 *ctx);
    void *bpf_cast_to_kern_ctx(void *obj) __ksym;
    SEC("?raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handler2, regs: *mut pt_regs, id: c_long) -> c_int {
    int BPF_PROG(handler2, struct pt_regs *regs, long id)
    {
    static volatile int whatever;
    if (my_tid != (s32)bpf_get_current_pid_tgid() || id != syscall_id)
    return 0;
// make sure we have CO-RE relocations in main program
    whatever = bpf_core_type_size(struct task_struct);
    __sink(whatever);
    set_output_val1(2000);
    set_output_ctx1(ctx); /* ctx definition is hidden in BPF_PROG macro */
// keep input value the same across both files to avoid dependency on
// handler call order; differentiate by output_weak1 vs output_weak2.
//
    set_output_weak(42);
    return 0;
    }
// Generate BTF FUNC record and test linking with duplicate extern functions
#[no_mangle]
pub unsafe extern "C" fn kfunc_gen2() {
    void kfunc_gen2(void)
    {
    bpf_cast_to_kern_ctx(0);
    }
    char LICENSE[] SEC("license") = "GPL";
