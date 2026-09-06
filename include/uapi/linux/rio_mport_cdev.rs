//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/rio_mport_cdev.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// Copyright (c) 2015-2016, Integrated Device Technology Inc.
// Copyright (c) 2015, Prodrive Technologies
// Copyright (c) 2015, Texas Instruments Incorporated
// Copyright (c) 2015, RapidIO Trade Association
// All rights reserved.
//
// This software is available to you under a choice of one of two licenses.
// You may choose to be licensed under the terms of the GNU General Public
// License(GPL) Version 2, or the BSD-3 Clause license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
// ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_mport_maint_io {
    pub /: *mut *mut __u16 rioid; / destID of remote device,
    pub /: *mut *mut __u8 hopcount; / hopcount to remote device,
    pub pad0: [__u8; 5],
    pub /: *mut *mut __u32 offset; / offset in register space,
    pub /: *mut *mut __u32 length; / length in bytes,
    pub /: *mut *mut __u64 buffer; / pointer to data buffer,
}

//
// Definitions for RapidIO data transfers:
// - memory mapped (MAPPED)
// - packet generation from memory (TRANSFER)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_mport_properties {
    pub hdid: __u16,
    pub /: *mut *mut __u8 id; / Physical port ID,
    pub index: __u8,
    pub flags: __u32,
    pub /: *mut *mut __u32 sys_size; / Default addressing size,
    pub port_ok: __u8,
    pub link_speed: __u8,
    pub link_width: __u8,
    pub pad0: __u8,
    pub dma_max_sge: __u32,
    pub dma_max_size: __u32,
    pub dma_align: __u32,
    pub /: *mut *mut __u32 transfer_mode; / Default transfer mode,
    pub /: *mut *mut __u32 cap_sys_size; / Capable system sizes,
    pub /: *mut *mut __u32 cap_addr_size; / Capable addressing sizes,
    pub /: *mut *mut __u32 cap_transfer_mode; / Capable transfer modes,
    pub /: *mut *mut __u32 cap_mport; / Mport capabilities,
}

//
// Definitions for RapidIO events;
// - incoming port-writes
// - incoming doorbells
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_doorbell {
    pub rioid: __u16,
    pub payload: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_doorbell_filter {
    pub /: *mut *mut __u16 rioid; / Use RIO_INVALID_DESTID to match all ids,
    pub low: __u16,
    pub high: __u16,
    pub pad0: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_portwrite {
    pub payload: [__u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_pw_filter {
    pub mask: __u32,
    pub low: __u32,
    pub high: __u32,
    pub pad0: __u32,
}

// RapidIO base address for inbound requests set to value defined below
// indicates that no specific RIO-to-local address translation is requested
// and driver should use direct (one-to-one) address mapping.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_mmap {
    pub rioid: __u16,
    pub pad0: [__u16; 3],
    pub rio_addr: __u64,
    pub length: __u64,
    pub handle: __u64,
    pub address: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_dma_mem {
    pub /: *mut *mut __u64 length; / length of DMA memory,
    pub /: *mut *mut __u64 dma_handle; / handle associated with this memory,
    pub address: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_event {
    pub /: *mut *mut __u32 header; / event type RIO_DOORBELL or RIO_PORTWRITE,
    pub /: *mut *mut rio_doorbell doorbell; / header for RIO_DOORBELL,
    pub /: *mut *mut rio_portwrite portwrite; / header for RIO_PORTWRITE,
    pub u: },
    pub pad0: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_transfer_sync {
    RIO_TRANSFER_SYNC,	/* synchronous transfer */
    RIO_TRANSFER_ASYNC,	/* asynchronous transfer */
    RIO_TRANSFER_FAF,	/* fire-and-forget transfer */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_transfer_dir {
    RIO_TRANSFER_DIR_READ,	/* Read operation */
    RIO_TRANSFER_DIR_WRITE,	/* Write operation */
}

//
// RapidIO data exchange transactions are lists of individual transfers. Each
// transfer exchanges data between two RapidIO devices by remote direct memory
// access and has its own completion code.
//
// The RapidIO specification defines four types of data exchange requests:
// NREAD, NWRITE, SWRITE and NWRITE_R. The RapidIO DMA channel interface allows
// to specify the required type of write operation or combination of them when
// only the last data packet requires response.
//
// NREAD:    read up to 256 bytes from remote device memory into local memory
// NWRITE:   write up to 256 bytes from local memory to remote device memory
// without confirmation
// SWRITE:   as NWRITE, but all addresses and payloads must be 64-bit aligned
// NWRITE_R: as NWRITE, but expect acknowledgment from remote device.
//
// The default exchange is chosen from NREAD and any of the WRITE modes as the
// driver sees fit. For write requests the user can explicitly choose between
// any of the write modes for each transaction.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_exchange {
    RIO_EXCHANGE_DEFAULT,	/* Default method */
    RIO_EXCHANGE_NWRITE,	/* All packets using NWRITE */
    RIO_EXCHANGE_SWRITE,	/* All packets using SWRITE */
    RIO_EXCHANGE_NWRITE_R,	/* Last packet NWRITE_R, others NWRITE */
    RIO_EXCHANGE_SWRITE_R,	/* Last packet NWRITE_R, others SWRITE */
    RIO_EXCHANGE_NWRITE_R_ALL, /* All packets using NWRITE_R */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_transfer_io {
    pub /: *mut *mut __u64 rio_addr; / Address in target's RIO mem space,
    pub loc_addr: __u64,
    pub handle: __u64,
    pub /: *mut *mut __u64 offset; / Offset in buffer,
    pub /: *mut *mut __u64 length; / Length in bytes,
    pub /: *mut *mut __u16 rioid; / Target destID,
    pub /: *mut *mut __u16 method; / Data exchange method, one of rio_exchange enum,
    pub /: *mut *mut __u32 completion_code; / Completion code for this transfer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_transaction {
    pub /: *mut *mut __u64 block; / Pointer to array of <count> transfers,
    pub /: *mut *mut __u32 count; / Number of transfers,
    pub /: *mut *mut __u32 transfer_mode; / Data transfer mode,
    pub /: *mut *mut __u16 sync; / Synch method, one of rio_transfer_sync enum,
    pub /: *mut *mut __u16 dir; / Transfer direction, one of rio_transfer_dir enum,
    pub pad0: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_async_tx_wait {
    pub /: *mut *mut __u32 token; / DMA transaction ID token,
    pub /: *mut *mut __u32 timeout; / Wait timeout in msec, if 0 use default TO,
}

pub const RIO_MAX_DEVNAME_SZ: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_rdev_info {
    pub destid: __u16,
    pub hopcount: __u8,
    pub pad0: __u8,
    pub comptag: __u32,
    pub 1]: char name[RIO_MAX_DEVNAME_SZ +,
}

// Driver IOCTL codes

