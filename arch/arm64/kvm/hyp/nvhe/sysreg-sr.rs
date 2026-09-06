//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/nvhe/sysreg-sr.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012-2015 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

//
// Non-VHE: Both host and guest must save everything.
//
#[no_mangle]
pub unsafe extern "C" fn __sysreg_save_state_nvhe(ctxt: *mut kvm_cpu_context) {
    void __sysreg_save_state_nvhe(struct kvm_cpu_context *ctxt)
    {
    __sysreg_save_el1_state(ctxt);
    __sysreg_save_common_state(ctxt);
    __sysreg_save_user_state(ctxt);
    __sysreg_save_el2_return_state(ctxt);
    }
#[no_mangle]
pub unsafe extern "C" fn __sysreg_restore_state_nvhe(ctxt: *mut kvm_cpu_context) {
    void __sysreg_restore_state_nvhe(struct kvm_cpu_context *ctxt)
    {
    let mut midr: u64 = ctxt_midr_el1(ctxt);
    __sysreg_restore_el1_state(ctxt, midr, ctxt_sys_reg(ctxt, MPIDR_EL1));
    __sysreg_restore_common_state(ctxt);
    __sysreg_restore_user_state(ctxt);
    __sysreg_restore_el2_return_state(ctxt);
    }
