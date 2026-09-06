//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/powernv/powernv.h
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
// There's various hacks scattered throughout the generic powerpc arch code
// that needs to call into powernv platform stuff. The prototypes for those
// functions are in asm/powernv.h
//

extern "C" {
    pub fn pnv_smp_init();
}

extern "C" {
    pub fn pnv_pci_init();
}
extern "C" {
    pub fn pnv_pci_shutdown();
}

extern "C" {
    pub fn pnv_get_supported_cpuidle_states() -> u32;
}
extern "C" {
    pub fn pnv_lpc_init();
}
extern "C" {
    pub fn opal_handle_events();
}
extern "C" {
    pub fn opal_have_pending_events() -> bool;
}
extern "C" {
    pub fn opal_event_shutdown();
}
extern "C" {
    pub fn cpu_core_split_required() -> bool;
}
extern "C" {
    pub fn memcons_copy(mc: *mut memcons, to: *mut c_char, pos: loff_t, count: usize) -> isize;
}
extern "C" {
    pub fn memcons_get_size(mc: *mut memcons) -> u32 __init;
}
extern "C" {
    pub fn memcons_init(node: *mut device_node, mc_prop_name: *const c_char) -> *mut memcons __init;
}
extern "C" {
    pub fn pnv_rng_init();
}
