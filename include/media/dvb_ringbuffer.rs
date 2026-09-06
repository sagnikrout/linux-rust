//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/dvb_ringbuffer.h
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
// dvb_ringbuffer.h: ring buffer implementation for the dvb driver
//
// Copyright (C) 2003 Oliver Endriss
// Copyright (C) 2004 Andrew de Quincey
//
// based on code originally found in av7110.c & dvb_ci.c:
// Copyright (C) 1999-2003 Ralph Metzler & Marcus Metzler
// for convergence integrated media GmbH
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//

//
// struct dvb_ringbuffer - Describes a ring buffer used at DVB framework
//
// @data: Area were the ringbuffer data is written
// @size: size of the ringbuffer
// @pread: next position to read
// @pwrite: next position to write
// @error: used by ringbuffer clients to indicate that an error happened.
// @queue: Wait queue used by ringbuffer clients to indicate when buffer
// was filled
// @lock: Spinlock used to protect the ringbuffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_ringbuffer {
    pub data: *mut u8,
    pub size: isize,
    pub pread: isize,
    pub pwrite: isize,
    pub error: c_int,
    pub queue: wait_queue_head_t,
    pub lock: spinlock_t,
}

pub const DVB_RINGBUFFER_PKTHDRSIZE: c_int = 3;
//
// dvb_ringbuffer_init - initialize ring buffer, lock and queue
//
// @rbuf: pointer to struct dvb_ringbuffer
// @data: pointer to the buffer where the data will be stored
// @len: bytes from ring buffer into @buf
//
// dvb_ringbuffer_empty - test whether buffer is empty
//
// @rbuf: pointer to struct dvb_ringbuffer
//
extern "C" {
    pub fn dvb_ringbuffer_empty(rbuf: *mut dvb_ringbuffer) -> c_int;
}
//
// dvb_ringbuffer_free - returns the number of free bytes in the buffer
//
// @rbuf: pointer to struct dvb_ringbuffer
//
// Return: number of free bytes in the buffer
//
extern "C" {
    pub fn dvb_ringbuffer_free(rbuf: *mut dvb_ringbuffer) -> isize;
}
//
// dvb_ringbuffer_avail - returns the number of bytes waiting in the buffer
//
// @rbuf: pointer to struct dvb_ringbuffer
//
// Return: number of bytes waiting in the buffer
//
extern "C" {
    pub fn dvb_ringbuffer_avail(rbuf: *mut dvb_ringbuffer) -> isize;
}
//
// dvb_ringbuffer_reset - resets the ringbuffer to initial state
//
// @rbuf: pointer to struct dvb_ringbuffer
//
// Resets the read and write pointers to zero and flush the buffer.
//
// This counts as a read and write operation
//
extern "C" {
    pub fn dvb_ringbuffer_reset(rbuf: *mut dvb_ringbuffer);
}
//
// read routines & macros
//
// dvb_ringbuffer_flush - flush buffer
//
// @rbuf: pointer to struct dvb_ringbuffer
//
extern "C" {
    pub fn dvb_ringbuffer_flush(rbuf: *mut dvb_ringbuffer);
}
//
// dvb_ringbuffer_flush_spinlock_wakeup- flush buffer protected by spinlock
// and wake-up waiting task(s)
//
// @rbuf: pointer to struct dvb_ringbuffer
//
extern "C" {
    pub fn dvb_ringbuffer_flush_spinlock_wakeup(rbuf: *mut dvb_ringbuffer);
}
//
// DVB_RINGBUFFER_PEEK - peek at byte @offs in the buffer
//
// @rbuf: pointer to struct dvb_ringbuffer
// @offs: offset inside the ringbuffer
//

//
// DVB_RINGBUFFER_SKIP - advance read ptr by @num bytes
//
// @rbuf: pointer to struct dvb_ringbuffer
// @num: number of bytes to advance
//

//
// dvb_ringbuffer_read_user - Reads a buffer into a user pointer
//
// @rbuf: pointer to struct dvb_ringbuffer
// @buf: pointer to the buffer where the data will be stored
// @len: bytes from ring buffer into @buf
//
// This variant assumes that the buffer is a memory at the userspace. So,
// it will internally call copy_to_user().
//
// Return: number of bytes transferred or -EFAULT
//
// dvb_ringbuffer_read - Reads a buffer into a pointer
//
// @rbuf: pointer to struct dvb_ringbuffer
// @buf: pointer to the buffer where the data will be stored
// @len: bytes from ring buffer into @buf
//
// This variant assumes that the buffer is a memory at the Kernel space
//
// Return: number of bytes transferred or -EFAULT
//
// write routines & macros
//
// DVB_RINGBUFFER_WRITE_BYTE - write single byte to ring buffer
//
// @rbuf: pointer to struct dvb_ringbuffer
// @byte: byte to write
//

//
// dvb_ringbuffer_write - Writes a buffer into the ringbuffer
//
// @rbuf: pointer to struct dvb_ringbuffer
// @buf: pointer to the buffer where the data will be read
// @len: bytes from ring buffer into @buf
//
// This variant assumes that the buffer is a memory at the Kernel space
//
// return: number of bytes transferred or -EFAULT
//
// dvb_ringbuffer_write_user - Writes a buffer received via a user pointer
//
// @rbuf: pointer to struct dvb_ringbuffer
// @buf: pointer to the buffer where the data will be read
// @len: bytes from ring buffer into @buf
//
// This variant assumes that the buffer is a memory at the userspace. So,
// it will internally call copy_from_user().
//
// Return: number of bytes transferred or -EFAULT
//
// dvb_ringbuffer_pkt_write - Write a packet into the ringbuffer.
//
// @rbuf: Ringbuffer to write to.
// @buf: Buffer to write.
// @len: Length of buffer (currently limited to 65535 bytes max).
//
// Return: Number of bytes written, or -EFAULT, -ENOMEM, -EINVAL.
//
// dvb_ringbuffer_pkt_read_user - Read from a packet in the ringbuffer.
//
// @rbuf: Ringbuffer concerned.
// @idx: Packet index as returned by dvb_ringbuffer_pkt_next().
// @offset: Offset into packet to read from.
// @buf: Destination buffer for data.
// @len: Size of destination buffer.
//
// Return: Number of bytes read, or -EFAULT.
//
// .. note::
//
// unlike dvb_ringbuffer_read(), this does **NOT** update the read pointer
// in the ringbuffer. You must use dvb_ringbuffer_pkt_dispose() to mark a
// packet as no longer required.
//
// dvb_ringbuffer_pkt_read - Read from a packet in the ringbuffer.
// Note: unlike dvb_ringbuffer_read_user(), this DOES update the read pointer
// in the ringbuffer.
//
// @rbuf: Ringbuffer concerned.
// @idx: Packet index as returned by dvb_ringbuffer_pkt_next().
// @offset: Offset into packet to read from.
// @buf: Destination buffer for data.
// @len: Size of destination buffer.
//
// Return: Number of bytes read, or -EFAULT.
//
// dvb_ringbuffer_pkt_dispose - Dispose of a packet in the ring buffer.
//
// @rbuf: Ring buffer concerned.
// @idx: Packet index as returned by dvb_ringbuffer_pkt_next().
//
extern "C" {
    pub fn dvb_ringbuffer_pkt_dispose(rbuf: *mut dvb_ringbuffer, idx: usize);
}
//
// dvb_ringbuffer_pkt_next - Get the index of the next packet in a ringbuffer.
//
// @rbuf: Ringbuffer concerned.
// @idx: Previous packet index, or -1 to return the first packet index.
// @pktlen: On success, will be updated to contain the length of the packet
// in bytes.
// returns Packet index (if >=0), or -1 if no packets available.
//
