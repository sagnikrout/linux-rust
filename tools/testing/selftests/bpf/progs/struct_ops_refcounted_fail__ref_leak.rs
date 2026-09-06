//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_refcounted_fail__ref_leak.c
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
// Test that the verifier rejects a program that acquires a referenced
// kptr through context without releasing the reference
//
    SEC("struct_ops/test_refcounted")
#[no_mangle]
pub unsafe extern "C" fn __msg(alloc_insn=0": "Unreleased reference id=1) -> __failure {
    __failure __msg("Unreleased reference id=1 alloc_insn=0")
    int BPF_PROG(refcounted_fail__ref_leak, int dummy,
    struct task_struct *task)
    {
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_ops testmod_ref_acquire = {
    .test_refcounted = (void *)refcounted_fail__ref_leak,
    };
