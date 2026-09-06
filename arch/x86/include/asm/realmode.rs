//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/realmode.h
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
// Flag bit definitions for use with the flags field of the trampoline header
// in the CONFIG_X86_64 variant.
//
pub const TH_FLAGS_SME_ACTIVE_BIT: c_int = 0;

// This must match data at realmode/rm/header.S
#[repr(C)]
#[derive(Copy, Clone)]
pub struct real_mode_header {
    pub text_start: u32,
    pub ro_end: u32,
// SMP trampoline
    pub trampoline_start: u32,
    pub trampoline_header: u32,

    pub sev_es_trampoline_start: u32,

    pub trampoline_start64: u32,
    pub trampoline_pgd: u32,

// ACPI S3 wakeup

    pub wakeup_start: u32,
    pub wakeup_header: u32,

// APM/BIOS reboot
    pub machine_real_restart_asm: u32,

    pub machine_real_restart_seg: u32,

}

// This must match data at realmode/rm/trampoline_{32,64}.S
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trampoline_header {

    pub start: u32,
    pub gdt_pad: u16,
    pub gdt_limit: u16,
    pub gdt_base: u32,

    pub start: u64,
    pub efer: u64,
    pub cr4: u32,
    pub flags: u32,
    pub lock: u32,

}

extern "C" {
    pub fn ALIGN(real_mode_blob: real_mode_blob_end -, _arg: PAGE_SIZE) -> return;
}
extern "C" {
    pub fn reserve_real_mode();
}
extern "C" {
    pub fn load_trampoline_pgtable();
}
extern "C" {
    pub fn init_real_mode();
}

