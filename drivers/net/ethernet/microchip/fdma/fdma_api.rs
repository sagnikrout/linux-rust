//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/fdma/fdma_api.h
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


// SPDX-License-Identifier: GPL-2.0+

// This provides a common set of functions and data structures for interacting
// with the Frame DMA engine on multiple Microchip switchcores.
//
// Frame DMA DCB format:
//
// +---------------------------+
// |         Next Ptr          |
// +---------------------------+
// |   Reserved  |    Info     |
// +---------------------------+
// |         Data0 Ptr         |
// +---------------------------+
// |   Reserved  |    Status0  |
// +---------------------------+
// |         Data1 Ptr         |
// +---------------------------+
// |   Reserved  |    Status1  |
// +---------------------------+
// |         Data2 Ptr         |
// +---------------------------+
// |   Reserved  |    Status2  |
// |-------------|-------------|
// |                           |
// |---------------------------|
// |         Data14 Ptr        |
// +-------------|-------------+
// |   Reserved  |    Status14 |
// +-------------|-------------+
//
// The data pointers points to the actual frame data to be received or sent. The
// addresses of the data pointers can, as of writing, be either a: DMA address,
// physical address or mapped address.
//

pub const FDMA_DCB_INVALID_DATA: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdma_db {
    pub dataptr: u64,
    pub status: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdma_dcb {
    pub nextptr: u64,
    pub info: u64,
    pub db: [fdma_db; FDMA_DB_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdma_ops {
// User-provided callback to set the dataptr
    pub ptr): *mut *mut *mut int (dataptr_cb)(struct fdma fdma, int dcb_idx, int db_idx, u64,
// User-provided callback to set the nextptr
    pub ptr): *mut *mut *mut int (nextptr_cb)(struct fdma fdma, int dcb_idx, u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdma {
    pub priv: *mut c_void,
// Virtual addresses
    pub dcbs: *mut fdma_dcb,
    pub last_dcb: *mut fdma_dcb,
// DMA address
    pub dma: dma_addr_t,
// Size of DCB + DB memory
    pub size: c_int,
// Indexes used to access the next-to-be-used DCB or DB
    pub db_index: c_int,
    pub dcb_index: c_int,
// Number of DCB's and DB's
    pub n_dcbs: u32,
    pub n_dbs: u32,
// Size of DB's
    pub db_size: u32,
// Channel id this FDMA object operates on
    pub channel_id: u32,
    pub ops: fdma_ops,
}

// Advance the DCB index and wrap if required.
// Advance the DB index.
// Reset the db index to zero.
// Check if a DCB can be reused in case of multiple DB's per DCB.
// Check if the FDMA has marked this DB as done.
// Get the length of a DB.
extern "C" {
    pub fn FDMA_DCB_STATUS_BLOCKL(_arg: db->status) -> return;
}
// Set the length of a DB.
// Get a DB by index.
// Get the next DB.
extern "C" {
    pub fn fdma_db_get(_arg: fdma, _arg: fdma->dcb_index, _arg: fdma->db_index) -> return;
}
// Get a DCB by index.
// Get the next DCB.
extern "C" {
    pub fn fdma_dcb_get(_arg: fdma, _arg: fdma->dcb_index) -> return;
}
// Check if the FDMA has frames ready for extraction.
extern "C" {
    pub fn fdma_db_is_done(_arg: fdma_db_next_get(fdma)) -> return;
}
// Get a nextptr by index
// nextptr = fdma->dma + (sizeof(struct fdma_dcb) * dcb_idx);
// Get the DMA address of a dataptr, by index. This function is only applicable
// if the dataptr addresses and DCB's are in contiguous memory and the driver
// supports XDP.
//
// Get the virtual address of a dataptr, by index. This function is only
// applicable if the dataptr addresses and DCB's are in contiguous memory and
// the driver supports XDP.
//
// Check if this DCB is the last used DCB.
extern "C" {
    pub fn fdma_dcbs_init(fdma: *mut fdma, info: u64, status: u64) -> c_int;
}
extern "C" {
    pub fn fdma_db_add(fdma: *mut fdma, dcb_idx: c_int, db_idx: c_int, status: u64) -> c_int;
}
extern "C" {
    pub fn fdma_dcb_add(fdma: *mut fdma, dcb_idx: c_int, info: u64, status: u64) -> c_int;
}
extern "C" {
    pub fn fdma_alloc_coherent(dev: *mut device, fdma: *mut fdma) -> c_int;
}
extern "C" {
    pub fn fdma_alloc_phys(fdma: *mut fdma) -> c_int;
}
extern "C" {
    pub fn fdma_free_coherent(dev: *mut device, fdma: *mut fdma);
}
extern "C" {
    pub fn fdma_free_phys(fdma: *mut fdma);
}
extern "C" {
    pub fn fdma_get_size(fdma: *mut fdma) -> u32;
}
extern "C" {
    pub fn fdma_get_size_contiguous(fdma: *mut fdma) -> u32;
}
