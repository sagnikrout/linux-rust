//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/physaddr.c
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

#[no_mangle]
pub unsafe extern "C" fn __virt_to_phys(x: c_ulong) -> phys_addr_t {
    phys_addr_t __virt_to_phys(unsigned long x)
    {
    WARN(!__is_lm_address(__tag_reset(x)),
    "virt_to_phys used for non-linear address: %p (%pS)\n",
    (void *)x,
    (void *)x);
    return __virt_to_phys_nodebug(x);
    }
    EXPORT_SYMBOL(__virt_to_phys);
#[no_mangle]
pub unsafe extern "C" fn __phys_addr_symbol(x: c_ulong) -> phys_addr_t {
    phys_addr_t __phys_addr_symbol(unsigned long x)
    {
//
// This is bounds checking against the kernel image only.
// __pa_symbol should only be used on kernel symbol addresses.
//
    VIRTUAL_BUG_ON(x < (unsigned long) KERNEL_START ||
    x > (unsigned long) KERNEL_END);
    return __pa_symbol_nodebug(x);
    }
    EXPORT_SYMBOL(__phys_addr_symbol);
