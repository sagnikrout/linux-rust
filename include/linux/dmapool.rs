//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dmapool.h
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


//
// include/linux/dmapool.h
//
// Allocation pools for DMAable (coherent) memory.
//
// This file is licensed under  the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

extern "C" {
    pub fn dma_pool_destroy(pool: *mut dma_pool);
}
extern "C" {
    pub fn dma_pool_free(pool: *mut dma_pool, vaddr: *mut c_void, addr: dma_addr_t);
}
//
// Managed DMA pool
//
extern "C" {
    pub fn dmam_pool_destroy(pool: *mut dma_pool);
}

//
// dma_pool_zalloc - Get a zero-initialized block of DMA coherent memory.
// @pool: dma pool that will produce the block
// @mem_flags: GFP_* bitmask
// @handle: pointer to dma address of block
//
// Same as dma_pool_alloc(), but the returned memory is zeroed.
//
extern "C" {
    pub fn dma_pool_alloc(_arg: pool, __GFP_ZERO: mem_flags |, _arg: handle) -> return;
}
