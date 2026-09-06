//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/dma-types.h
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
// typedef dma32_t
// Contains a 31 bit absolute address to a DMA capable piece of storage.
//
// For CIO, DMA addresses are always absolute addresses. These addresses tend
// to be used in architectured memory blocks (like ORB, IDAW, MIDAW). Under
// certain circumstances 31 bit wide addresses must be used because the
// address must fit in 31 bits.
//
// This type is to be used when such fields can be modelled as 32 bit wide.
//
pub type dma32_t = u32 ;
//
// typedef dma64_t
// Contains a 64 bit absolute address to a DMA capable piece of storage.
//
// For CIO, DMA addresses are always absolute addresses. These addresses tend
// to be used in architectured memory blocks (like ORB, IDAW, MIDAW).
//
// This type is to be used to model such 64 bit wide fields.
//
pub type dma64_t = u64 ;
//
// Although DMA addresses should be obtained using the DMA API, in cases when
// it is known that the first argument holds a virtual address that points to
// DMA-able 31 bit addressable storage, then this function can be safely used.
//
extern "C" {
    pub fn __va(long)addr: ( unsigned) -> return;
}
//
// Although DMA addresses should be obtained using the DMA API, in cases when
// it is known that the first argument holds a virtual address that points to
// DMA-able storage, then this function can be safely used.
//
extern "C" {
    pub fn __va(long)addr: ( unsigned) -> return;
}
