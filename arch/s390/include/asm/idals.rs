//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/idals.h
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
// Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
// Bugreports.to..: <Linux390@de.ibm.com>
// Copyright IBM Corp. 2000
//
// History of changes
// 07/24/00 new file
// 05/04/02 code restructuring.
//

pub const IDA_SIZE_SHIFT: c_int = 12;

pub const IDA_2K_SIZE_SHIFT: c_int = 11;

//
// Test if an address/length pair needs an idal list.
//
// Return the number of idal words needed for an address/length pair.
//
// Return the number of 2K IDA words needed for an address/length pair.
//
// Create the list of idal words for an address/length pair.
//
// idaws++ = paddr;
//
// Sets the address of the data in CCW.
// If necessary it allocates an IDAL and sets the appropriate flags.
//
// Releases any allocated IDAL related to the CCW.
//
// Idal buffer extension
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idal_buffer {
    pub size: usize,
    pub page_order: usize,
    pub data: [dma64_t; ],
}

//
// Allocate an idal buffer
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
//
// Free an idal buffer.
//
// Allocate an array of IDAL buffers to cover a total data size of @size. The
// resulting array is null-terminated.
//
// The amount of individual IDAL buffers is determined based on @size.
// Each IDAL buffer can have a maximum size of @CCW_MAX_BYTE_COUNT.
//
// Determine size for the current idal buffer
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
//
// Free array of IDAL buffers
//
// ibs = NULL;
//
// Determine size of IDAL buffer array
//
// Determine total data size covered by IDAL buffer array
//
// Test if a idal list is really needed.
//
extern "C" {
    pub fn idal_is_needed(_arg: dma64_to_virt(ib->data[0]), _arg: ib->size) -> return;
}
//
// Set channel data address to idal buffer.
//
// Setup idals
//
// No idals needed - use direct addressing. Convert from
// dma64_t to virt and then to dma32_t only because of type
// checking. The physical address is known to be below 2GB.
//
// Copy count bytes from an idal buffer to user memory
//
extern "C" {
    pub fn copy_to_user(_arg: to, _arg: vaddr, _arg: count) -> return;
}
//
// Copy count bytes from user memory to an idal buffer
//
extern "C" {
    pub fn copy_from_user(_arg: vaddr, _arg: from, _arg: count) -> return;
}
