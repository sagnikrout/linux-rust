//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/ipmi/ipmi_plat_data.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Generic code to add IPMI platform devices.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipmi_plat_interface_type {

    struct ipmi_plat_data {
    enum ipmi_plat_interface_type iftype;
    unsigned int type; /* si_type for si, SI_INVALID for others */
    unsigned int space; /* addr_space for si, intf# for ssif. */
    unsigned long addr;
    unsigned int regspacing;
    unsigned int regsize;
    unsigned int regshift;
    unsigned int irq;
    unsigned int slave_addr;
    enum ipmi_addr_src addr_source;
}
