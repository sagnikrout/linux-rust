//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/apple/rtkit.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple RTKit IPC Library
// Copyright (C) The Asahi Linux Contributors
//
// Apple's SoCs come with various co-processors running their RTKit operating
// system. This protocol library is used by client drivers to use the
// features provided by them.
//

//
// Struct to represent implementation-specific RTKit operations.
//
// @buffer:    Shared memory buffer allocated inside normal RAM.
// @iomem:     Shared memory buffer controlled by the co-processors.
// @size:      Size of the shared memory buffer.
// @iova:      Device VA of shared memory buffer.
// @is_mapped: Shared memory buffer is managed by the co-processor.
// @private:   Private data pointer for the parent driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_rtkit_shmem {
    pub buffer: *mut c_void,
    pub iomem: *mut void __iomem,
    pub size: usize,
    pub iova: dma_addr_t,
    pub is_mapped: bool,
    pub private: *mut c_void,
}

//
// Struct to represent implementation-specific RTKit operations.
//
// @crashed:       Called when the co-processor has crashed. Runs in process
// context.
// @recv_message:  Function called when a message from RTKit is received
// on a non-system endpoint. Called from a worker thread.
// @recv_message_early:
// Like recv_message, but called from atomic context. It
// should return true if it handled the message. If it
// returns false, the message will be passed on to the
// worker thread.
// @shmem_setup:   Setup shared memory buffer. If bfr.is_iomem is true the
// buffer is managed by the co-processor and needs to be mapped.
// Otherwise the buffer is managed by Linux and needs to be
// allocated. If not specified dma_alloc_coherent is used.
// Called in process context.
// @shmem_destroy: Undo the shared memory buffer setup in shmem_setup. If not
// specified dma_free_coherent is used. Called in process
// context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_rtkit_ops {
    pub crashlog_size): *const *const *const *const void (crashed)(void cookie, void crashlog, size_t,
    pub message): *mut *mut *mut void (recv_message)(void cookie, u8 endpoint, u64,
    pub message): *mut *mut *mut bool (recv_message_early)(void cookie, u8 endpoint, u64,
    pub bfr): *mut *mut *mut int (shmem_setup)(void cookie, struct apple_rtkit_shmem,
    pub bfr): *mut *mut *mut void (shmem_destroy)(void cookie, struct apple_rtkit_shmem,
}

//
// Initializes the internal state required to handle RTKit. This
// should usually be called within _probe.
//
// @dev:         Pointer to the device node this coprocessor is associated with
// @cookie:      opaque cookie passed to all functions defined in rtkit_ops
// @mbox_name:   mailbox name used to communicate with the co-processor
// @mbox_idx:    mailbox index to be used if mbox_name is NULL
// @ops:         pointer to rtkit_ops to be used for this co-processor
//
// Non-devm version of devm_apple_rtkit_init. Must be freed with
// apple_rtkit_free.
//
// @dev:         Pointer to the device node this coprocessor is associated with
// @cookie:      opaque cookie passed to all functions defined in rtkit_ops
// @mbox_name:   mailbox name used to communicate with the co-processor
// @mbox_idx:    mailbox index to be used if mbox_name is NULL
// @ops:         pointer to rtkit_ops to be used for this co-processor
//
// Free an instance of apple_rtkit.
//
extern "C" {
    pub fn apple_rtkit_free(rtk: *mut apple_rtkit);
}
//
// Reinitialize internal structures. Must only be called with the co-processor
// is held in reset.
//
extern "C" {
    pub fn apple_rtkit_reinit(rtk: *mut apple_rtkit) -> c_int;
}
//
// Handle RTKit's boot process. Should be called after the CPU of the
// co-processor has been started.
//
extern "C" {
    pub fn apple_rtkit_boot(rtk: *mut apple_rtkit) -> c_int;
}
//
// Quiesce the co-processor.
//
extern "C" {
    pub fn apple_rtkit_quiesce(rtk: *mut apple_rtkit) -> c_int;
}
//
// Wake the co-processor up from hibernation mode.
//
extern "C" {
    pub fn apple_rtkit_wake(rtk: *mut apple_rtkit) -> c_int;
}
//
// Shutdown the co-processor
//
extern "C" {
    pub fn apple_rtkit_shutdown(rtk: *mut apple_rtkit) -> c_int;
}
//
// Put the co-processor into the lowest power state. Note that it usually
// is not possible to recover from this state without a full SoC reset.
//
extern "C" {
    pub fn apple_rtkit_poweroff(rtk: *mut apple_rtkit) -> c_int;
}
//
// Put the co-processor into idle mode
//
extern "C" {
    pub fn apple_rtkit_idle(rtk: *mut apple_rtkit) -> c_int;
}
//
// Checks if RTKit is running and ready to handle messages.
//
extern "C" {
    pub fn apple_rtkit_is_running(rtk: *mut apple_rtkit) -> bool;
}
//
// Checks if RTKit has crashed.
//
extern "C" {
    pub fn apple_rtkit_is_crashed(rtk: *mut apple_rtkit) -> bool;
}
//
// Starts an endpoint. Must be called after boot but before any messages can be
// sent or received from that endpoint.
//
extern "C" {
    pub fn apple_rtkit_start_ep(rtk: *mut apple_rtkit, endpoint: u8) -> c_int;
}
//
// Send a message to the given endpoint.
//
// @rtk:            RTKit reference
// @ep:             target endpoint
// @message:        message to be sent
// @completeion:    will be completed once the message has been submitted
// to the hardware FIFO. Can be NULL.
// @atomic:         if set to true this function can be called from atomic
// context.
//
// Process incoming messages in atomic context.
// This only guarantees that messages arrive as far as the recv_message_early
// callback; drivers expecting to handle incoming messages synchronously
// by calling this function must do it that way.
// Will return 1 if some data was processed, 0 if none was, or a
// negative error code on failure.
//
// @rtk:            RTKit reference
//
extern "C" {
    pub fn apple_rtkit_poll(rtk: *mut apple_rtkit) -> c_int;
}
