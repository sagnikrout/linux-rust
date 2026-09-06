//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_jit_inline.c
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

    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __jited("	addq	%gs:{{.*}}, %rax")
    __arch_arm64
    __jited("	mrs	x8, SP_EL0")
    __arch_riscv64
    __jited("	mv	a5, tp")
    __arch_loongarch
    __jited("	move	$a5, $tp")
#[no_mangle]
pub unsafe extern "C" fn inline_bpf_get_current_task() -> c_int {
    int inline_bpf_get_current_task(void)
    {
    bpf_get_current_task();
    return 0;
    }
    SEC("fentry/bpf_fentry_test2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_loongarch
    __jited("	ld.wu	$a5, $tp, 16")
#[no_mangle]
pub unsafe extern "C" fn inline_bpf_get_smp_processor_id() -> c_int {
    int inline_bpf_get_smp_processor_id(void)
    {
    bpf_get_smp_processor_id();
    return 0;
    }
    char _license[] SEC("license") = "GPL";
