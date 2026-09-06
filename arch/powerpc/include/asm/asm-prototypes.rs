//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/asm-prototypes.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// This file is for C prototypes of asm symbols that are EXPORTed.
// It allows the modversions logic to see their prototype and
// generate proper CRCs for them.
//
// Copyright 2016, Daniel Axtens, IBM Corporation.
//

// Ultravisor

extern "C" {
    pub fn ucall_norets(opcode: c_ulong, ...) -> c_long;
}

// OPAL
// misc runtime
extern "C" {
    pub fn enable_machine_check();
}
extern "C" {
    pub fn __bswapdi2(_arg: u64) -> u64;
}
extern "C" {
    pub fn __lshrdi3(_arg: i64, _arg: c_int) -> i64;
}
extern "C" {
    pub fn __ashldi3(_arg: i64, _arg: c_int) -> i64;
}
extern "C" {
    pub fn __ashrdi3(_arg: i64, _arg: c_int) -> i64;
}
extern "C" {
    pub fn __cmpdi2(_arg: i64, _arg: i64) -> c_int;
}
extern "C" {
    pub fn __ucmpdi2(_arg: u64, _arg: u64) -> c_int;
}
// tracing
extern "C" {
    pub fn _mcount();
}
// Transaction memory related
extern "C" {
    pub fn tm_enable();
}
extern "C" {
    pub fn tm_disable();
}
extern "C" {
    pub fn tm_abort(cause: u8);
}
extern "C" {
    pub fn _kvmppc_restore_tm_pr(vcpu: *mut kvm_vcpu, guest_msr: u64);
}
extern "C" {
    pub fn _kvmppc_save_tm_pr(vcpu: *mut kvm_vcpu, guest_msr: u64);
}

extern "C" {
    pub fn kvmppc_save_tm_hv(vcpu: *mut kvm_vcpu, msr: u64, preserve_nv: bool);
}
extern "C" {
    pub fn kvmppc_restore_tm_hv(vcpu: *mut kvm_vcpu, msr: u64, preserve_nv: bool);
}

extern "C" {
    pub fn kvmppc_p9_enter_guest(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_h_set_dabr(vcpu: *mut kvm_vcpu, dabr: c_ulong) -> c_long;
}
