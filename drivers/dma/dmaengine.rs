//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/dmaengine.h
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
// The contents of this file are private to DMA engine drivers, and is not
// part of the API to be used by DMA engine users.
//

//
// dma_cookie_init - initialize the cookies for a DMA channel
// @chan: dma channel to initialize
//
// dma_cookie_assign - assign a DMA engine cookie to the descriptor
// @tx: descriptor needing cookie
//
// Assign a unique non-zero per-channel cookie to the descriptor.
// Note: caller is expected to hold a lock to prevent concurrency.
//
// dma_cookie_complete - complete a descriptor
// @tx: descriptor to complete
//
// Mark this descriptor complete by updating the channels completed
// cookie marker.  Zero the descriptors cookie to prevent accidental
// repeated completions.
//
// Note: caller is expected to hold a lock to prevent concurrency.
//
// dma_cookie_status - report cookie status
// @chan: dma channel
// @cookie: cookie we are interested in
// @state: dma_tx_state structure to return last/used cookies
//
// Report the status of the cookie, filling in the state structure if
// non-NULL.  No locking is required.
//
extern "C" {
    pub fn dma_async_is_complete(_arg: cookie, _arg: complete, _arg: used) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmaengine_desc_callback {
    pub callback: dma_async_tx_callback,
    pub callback_result: dma_async_tx_callback_result,
    pub callback_param: *mut c_void,
}

//
// dmaengine_desc_get_callback - get the passed in callback function
// @tx: tx descriptor
// @cb: temp struct to hold the callback info
//
// Fill the passed in cb struct with what's available in the passed in
// tx descriptor struct
// No locking is required.
//
// dmaengine_desc_callback_invoke - call the callback function in cb struct
// @cb: temp struct that is holding the callback info
// @result: transaction result
//
// Call the callback function provided in the cb struct with the parameter
// in the cb struct.
// Locking is dependent on the driver.
//
// dmaengine_desc_get_callback_invoke - get the callback in tx descriptor and
// then immediately call the callback.
// @tx: dma async tx descriptor
// @result: transaction result
//
// Call dmaengine_desc_get_callback() and dmaengine_desc_callback_invoke()
// in a single function since no work is necessary in between for the driver.
// Locking is dependent on the driver.
//
// dmaengine_desc_callback_valid - verify the callback is valid in cb
// @cb: callback info struct
//
// Return a bool that verifies whether callback in cb is valid or not.
// No locking is required.
//

