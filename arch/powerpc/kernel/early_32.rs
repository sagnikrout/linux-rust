//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/early_32.c
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
// Early init before relocation
//

//
// We're called here very early in the boot.
//
// Note that the kernel may be running at an address which is different
// from the address that it was linked at, so we must use RELOC/PTRRELOC
// to access static data (including strings).  -- paulus
//
#[no_mangle]
pub unsafe extern "C" fn early_init(dt_ptr: c_ulong) -> notrace unsigned long __init {
    notrace unsigned long __init early_init(unsigned long dt_ptr)
    {
    unsigned long kva, offset = reloc_offset();
    kva = *PTRRELOC(&kernstart_virt_addr);
// First zero the BSS
    if (kva == KERNELBASE)
    memset(PTRRELOC(&__bss_start), 0, __bss_stop - __bss_start);
//
// Identify the CPU type and fix up code sections
// that depend on which cpu we have.
//
    identify_cpu(offset, mfspr(SPRN_PVR));
    apply_feature_fixups();
    return kva + offset;
    }
