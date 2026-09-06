//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/page_64_types.h
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

pub const KASAN_STACK_ORDER: c_int = 1;

pub const KASAN_STACK_ORDER: c_int = 0;

//
// The index for the tss.ist[] array. The hardware limit is 7 entries.
//
pub const IST_INDEX_DF: c_int = 0;
pub const IST_INDEX_NMI: c_int = 1;
pub const IST_INDEX_DB: c_int = 2;
pub const IST_INDEX_MCE: c_int = 3;
pub const IST_INDEX_VC: c_int = 4;
//
// Set __PAGE_OFFSET to the most negative possible address +
// PGDIR_SIZE*17 (pgd slot 273).
//
// The gap is to allow a space for LDT remap for PTI (1 pgd slot) and space for
// a hypervisor (16 slots). Choosing 16 slots for a hypervisor is arbitrary,
// but it's what Xen requires.
//

// See Documentation/arch/x86/x86_64/mm.rst for a description of the memory map.
pub const __PHYSICAL_MASK_SHIFT: c_int = 52;

// This decides where the kernel will search for a free chunk of vm
// space during mmap's.
//

//
// In spite of the name, KERNEL_IMAGE_SIZE is a limit on the maximum virtual
// address for the kernel image, rather than the limit on the size itself.
// This can be at most 1 GiB, due to the fixmap living in the next 1 GiB (see
// level2_kernel_pgt in arch/x86/kernel/head_64.S).
//
// On KASLR use 1 GiB by default, leaving 1 GiB for modules once the
// page tables are fully set up.
//
// If KASLR is disabled we can shrink it to 0.5 GiB and increase the size
// of the modules area to 1.5 GiB.
//

