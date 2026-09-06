//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/spec-ctrl.h
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

//
// On VMENTER we must preserve whatever view of the SPEC_CTRL MSR
// the guest has, while on VMEXIT we restore the host view. This
// would be easier if SPEC_CTRL were architecturally maskable or
// shadowable for guests but this is not (currently) the case.
// Takes the guest view of SPEC_CTRL MSR as a parameter and also
// the guest's version of VIRT_SPEC_CTRL, if emulated.
//
extern "C" {
    pub fn x86_virt_spec_ctrl(guest_virt_spec_ctrl: u64, guest: bool);
}
//
// x86_spec_ctrl_set_guest - Set speculation control registers for the guest
// @guest_virt_spec_ctrl:	The guest controlled bits of MSR_VIRT_SPEC_CTRL
// (may get translated to MSR_AMD64_LS_CFG bits)
//
// Avoids writing to the MSR if the content/bits are the same
//
// x86_spec_ctrl_restore_host - Restore host speculation control registers
// @guest_virt_spec_ctrl:	The guest controlled bits of MSR_VIRT_SPEC_CTRL
// (may get translated to MSR_AMD64_LS_CFG bits)
//
// Avoids writing to the MSR if the content/bits are the same
//
// AMD specific Speculative Store Bypass MSR data
//
// This can be used in noinstr functions & should only be called in bare
// metal context.
//

extern "C" {
    pub fn speculative_store_bypass_ht_init();
}

extern "C" {
    pub fn speculation_ctrl_update(tif: c_ulong);
}
extern "C" {
    pub fn speculation_ctrl_update_current();
}
