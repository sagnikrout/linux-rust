//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/surface/aggregator/ssh_parser.h
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
//
// SSH message parser.
//
// Copyright (C) 2019-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

//
// struct sshp_buf - Parser buffer for SSH messages.
// @ptr: Pointer to the beginning of the buffer.
// @len: Number of bytes used in the buffer.
// @cap: Maximum capacity of the buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sshp_buf {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

//
// sshp_buf_init() - Initialize a SSH parser buffer.
// @buf: The buffer to initialize.
// @ptr: The memory backing the buffer.
// @cap: The length of the memory backing the buffer, i.e. its capacity.
//
// Initializes the buffer with the given memory as backing and set its used
// length to zero.
//
// sshp_buf_alloc() - Allocate and initialize a SSH parser buffer.
// @buf:   The buffer to initialize/allocate to.
// @cap:   The desired capacity of the buffer.
// @flags: The flags used for allocating the memory.
//
// Allocates @cap bytes and initializes the provided buffer struct with the
// allocated memory.
//
// Return: Returns zero on success and %-ENOMEM if allocation failed.
//
// sshp_buf_free() - Free a SSH parser buffer.
// @buf: The buffer to free.
//
// Frees a SSH parser buffer by freeing the memory backing it and then
// resetting its pointer to %NULL and length and capacity to zero. Intended to
// free a buffer previously allocated with sshp_buf_alloc().
//
// sshp_buf_drop() - Drop data from the beginning of the buffer.
// @buf: The buffer to drop data from.
// @n:   The number of bytes to drop.
//
// Drops the first @n bytes from the buffer. Re-aligns any remaining data to
// the beginning of the buffer.
//
// sshp_buf_read_from_fifo() - Transfer data from a fifo to the buffer.
// @buf:  The buffer to write the data into.
// @fifo: The fifo to read the data from.
//
// Transfers the data contained in the fifo to the buffer, removing it from
// the fifo. This function will try to transfer as much data as possible,
// limited either by the remaining space in the buffer or by the number of
// bytes available in the fifo.
//
// Return: Returns the number of bytes transferred.
//
// sshp_buf_span_from() - Initialize a span from the given buffer and offset.
// @buf:    The buffer to create the span from.
// @offset: The offset in the buffer at which the span should start.
// @span:   The span to initialize (output).
//
// Initializes the provided span to point to the memory at the given offset in
// the buffer, with the length of the span being capped by the number of bytes
// used in the buffer after the offset (i.e. bytes remaining after the
// offset).
//
// Warning: This function does not validate that @offset is less than or equal
// to the number of bytes used in the buffer or the buffer capacity. This must
// be guaranteed by the caller.
//
extern "C" {
    pub fn sshp_find_syn(src: *const ssam_span, rem: *mut ssam_span) -> bool;
}
