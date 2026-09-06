//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/kvm_para.h
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

extern "C" {
    pub fn kvm_check_and_clear_guest_paused() -> bool;
}

// For KVM hypercalls, a three-byte sequence of either the vmcall or the vmmcall
// instruction.  The hypervisor may replace it with something else but only the
// instructions are guaranteed to be supported.
//
// Up to four arguments may be passed in rbx, rcx, rdx, and rsi respectively.
// The hypercall number should be placed in rax and the return value will be
// placed in rax.  No other registers will be clobbered unless explicitly
// noted by the particular hypercall.
//
extern "C" {
    pub fn tdx_kvm_hypercall(_arg: nr, _arg: 0, _arg: 0, _arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn tdx_kvm_hypercall(_arg: nr, _arg: p1, _arg: 0, _arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn tdx_kvm_hypercall(_arg: nr, _arg: p1, _arg: p2, _arg: 0, _arg: 0) -> return;
}
extern "C" {
    pub fn tdx_kvm_hypercall(_arg: nr, _arg: p1, _arg: p2, _arg: p3, _arg: 0) -> return;
}
extern "C" {
    pub fn tdx_kvm_hypercall(_arg: nr, _arg: p1, _arg: p2, _arg: p3, _arg: p4) -> return;
}

extern "C" {
    pub fn kvmclock_init();
}
extern "C" {
    pub fn kvmclock_disable();
}
extern "C" {
    pub fn kvm_para_available() -> bool;
}
extern "C" {
    pub fn kvm_arch_para_features() -> c_uint;
}
extern "C" {
    pub fn kvm_arch_para_hints() -> c_uint;
}
extern "C" {
    pub fn kvm_async_pf_task_wait_schedule(token: u32);
}
extern "C" {
    pub fn kvm_read_and_reset_apf_flags() -> u32;
}
extern "C" {
    pub fn __kvm_handle_async_pf(regs: *mut pt_regs, token: u32) -> bool;
}
extern "C" {
    pub fn __kvm_handle_async_pf(_arg: regs, _arg: token) -> return;
}

extern "C" {
    pub fn kvm_spinlock_init() -> void __init;
}

