//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/page_32_types.h
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
// This handles the memory map.
//
// A __PAGE_OFFSET of 0xC0000000 means that the kernel has
// a virtual address space of one gigabyte, which limits the
// amount of physical memory you can use to about 950MB.
//
// If you want more physical memory than this then see the CONFIG_VMSPLIT_2G
// and CONFIG_HIGHMEM4G options in the kernel configuration.
//

pub const THREAD_SIZE_ORDER: c_int = 1;

pub const N_EXCEPTION_STACKS: c_int = 1;

//
// This is beyond the 44 bit limit imposed by the 32bit long pfns,
// but we need the full mask to make sure inverted PROT_NONE
// entries have all the host bits set in a guest.
// The real limit is still 44 bits.
//
pub const __PHYSICAL_MASK_SHIFT: c_int = 52;
pub const __VIRTUAL_MASK_SHIFT: c_int = 32;

pub const __PHYSICAL_MASK_SHIFT: c_int = 32;
pub const __VIRTUAL_MASK_SHIFT: c_int = 32;

//
// User space process size: 3GB (default).
//

//
// In spite of the name, KERNEL_IMAGE_SIZE is a limit on the maximum virtual
// address for the kernel image, rather than the limit on the size itself. On
// 32-bit, this is not a strict limit, but this value is used to limit the
// link-time virtual address range of the kernel, and by KASLR to limit the
// randomized address from which the kernel is executed. A relocatable kernel
// can be loaded somewhat higher than KERNEL_IMAGE_SIZE as long as enough space
// remains for the vmalloc area.
//

//
// This much address space is reserved for vmalloc() and iomap()
// as well as fixmap mappings.
//
extern "C" {
    pub fn find_low_pfn_range();
}

