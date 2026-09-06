//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/suspend_32.h
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
// Copyright 2001-2002 Pavel Machek <pavel@suse.cz>
// Based on code
// Copyright 2001 Patrick Mochel <mochel@osdl.org>
//

// image of the saved processor state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saved_context {
    pub cr4: unsigned long cr0, cr2, cr3,,
    pub misc_enable: u64,
    pub saved_msrs: saved_msrs,
    pub gdt_desc: desc_ptr,
    pub idt: desc_ptr,
    pub ldt: u16,
    pub tss: u16,
    pub tr: c_ulong,
    pub safety: c_ulong,
    pub return_address: c_ulong,
//
// On x86_32, all segment registers except gs are saved at kernel
// entry in pt_regs.
//
    pub gs: u16,
    pub misc_enable_saved: bool,
    pub __attribute__((packed)): },
// routines for saving/restoring kernel state
    pub core_restore_code: [extern char; ],
    pub restore_registers: [extern char; ],