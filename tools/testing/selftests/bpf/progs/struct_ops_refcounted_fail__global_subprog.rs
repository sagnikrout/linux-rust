//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_refcounted_fail__global_subprog.c
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


    char _license[] SEC("license") = "GPL";
    extern void bpf_task_release(struct task_struct *p) __ksym;
#[no_mangle]
pub unsafe extern "C" fn subprog_release(__arg_ctx: *mut *mut __u64 ctx) -> __noinline int {
    __noinline int subprog_release(__u64 *ctx __arg_ctx)
    {
    struct task_struct *task = (struct task_struct *)ctx[1];
    let mut dummy: c_int = (int)ctx[0];
    bpf_task_release(task);
    return dummy + 1;
    }
// Test that the verifier rejects a program that contains a global
// subprogram with referenced kptr arguments
//
    SEC("struct_ops/test_refcounted")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("Validating subprog_release() func#1...")
    __msg("invalid bpf_context access off=8. Reference may already be released")
#[no_mangle]
pub unsafe extern "C" fn refcounted_fail__global_subprog(ctx: *mut c_ulonglong) -> c_int {
    int refcounted_fail__global_subprog(unsigned long long *ctx)
    {
    struct task_struct *task = (struct task_struct *)ctx[1];
    bpf_task_release(task);
    return subprog_release(ctx);
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_ops testmod_ref_acquire = {
    .test_refcounted = (void *)refcounted_fail__global_subprog,
    };
