//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/uv/uv.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uv_system_type {

    static inline int uv(int uvtype)
    {
// uv(0) is "any"
    if (uvtype >= 0 && uvtype <= 30)
    return 1 << uvtype;
    return 1;
    }

    extern unsigned long uv_systab_phys;

    extern enum uv_system_type get_uv_system_type(void);
    static inline bool is_early_uv_system(void)
    {
    return uv_systab_phys && uv_systab_phys != EFI_INVALID_TABLE_ADDR;
    }
    extern int is_uv_system(void);
    extern int is_uv_hubbed(int uvtype);
    extern void uv_cpu_init(void);
    extern void uv_nmi_init(void);
    extern void uv_system_init(void);

    static inline enum uv_system_type get_uv_system_type(void) { return UV_NONE; }
    static inline bool is_early_uv_system(void)	{ return 0; }
    static inline int is_uv_system(void)	{ return 0; }
    static inline int is_uv_hubbed(int uv)	{ return 0; }
    static inline void uv_cpu_init(void)	{ }
    static inline void uv_system_init(void)	{ }

