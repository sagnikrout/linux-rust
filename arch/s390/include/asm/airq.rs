//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/airq.h
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
// Copyright IBM Corp. 2002, 2007
// Author(s): Ingo Adlung <adlung@de.ibm.com>
// Cornelia Huck <cornelia.huck@de.ibm.com>
// Arnd Bergmann <arndb@de.ibm.com>
// Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airq_struct {
    pub /: *mut *mut hlist_node list; / Handler queueing.,
    pub tpi_info): *mut *mut *mut void (handler)(struct airq_struct airq, struct tpi_info,
    pub /: *mut *mut *mut u8 lsi_ptr; / Local-Summary-Indicator pointer,
    pub /: *mut *mut u8 isc; / Interrupt-subclass,
    pub flags: u8,
}

pub const AIRQ_PTR_ALLOCATED: c_uint = 0x01;
extern "C" {
    pub fn register_adapter_interrupt(airq: *mut airq_struct) -> c_int;
}
extern "C" {
    pub fn unregister_adapter_interrupt(airq: *mut airq_struct);
}
// Adapter interrupt bit vector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airq_iv {
    pub /: *mut *mut *mut unsigned long vector; / Adapter interrupt bit vector,
    pub /: *mut *mut dma_addr_t vector_dma; / Adapter interrupt bit vector dma,
    pub /: *mut *mut *mut unsigned long avail; / Allocation bit mask for the bit vector,
    pub /: *mut *mut *mut unsigned long bitlock; / Lock bit mask for the bit vector,
    pub /: *mut *mut *mut unsigned long ptr; / Pointer associated with each bit,
    pub /: *mut *mut *mut unsigned int data; / 32 bit value associated with each bit,
    pub /: *mut *mut unsigned long bits; / Number of bits in the vector,
    pub /: *mut *mut unsigned long end; / Number of highest allocated bit + 1,
    pub /: *mut *mut unsigned long flags; / Allocation flags,
    pub /: *mut *mut spinlock_t lock; / Lock to protect alloc & free,
}

extern "C" {
    pub fn airq_iv_release(iv: *mut airq_iv);
}
extern "C" {
    pub fn airq_iv_alloc(iv: *mut airq_iv, num: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn airq_iv_free(iv: *mut airq_iv, bit: c_ulong, num: c_ulong);
}
extern "C" {
    pub fn airq_iv_alloc(_arg: iv, _arg: 1) -> return;
}
