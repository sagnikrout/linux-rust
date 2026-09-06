//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/scif_ioctl.h
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
// Intel MIC Platform Software Stack (MPSS)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// GPL LICENSE SUMMARY
//
// Copyright(c) 2014 Intel Corporation.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// BSD LICENSE
//
// Copyright(c) 2014 Intel Corporation.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// * Neither the name of Intel Corporation nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Intel SCIF driver.
//
// -----------------------------------------
// SCIF IOCTL interface information
// -----------------------------------------
//

//
// struct scif_port_id - SCIF port information
// @node:	node on which port resides
// @port:	local port number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scif_port_id {
    pub node: __u16,
    pub port: __u16,
}

//
// struct scifioctl_connect - used for SCIF_CONNECT IOCTL
// @self:	used to read back the assigned port_id
// @peer:	destination node and port to connect to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scifioctl_connect {
    pub self: scif_port_id,
    pub peer: scif_port_id,
}

//
// struct scifioctl_accept - used for SCIF_ACCEPTREQ IOCTL
// @flags:	flags
// @peer:	global id of peer endpoint
// @endpt:	new connected endpoint descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scifioctl_accept {
    pub flags: __s32,
    pub peer: scif_port_id,
    pub endpt: __u64,
}

//
// struct scifioctl_msg - used for SCIF_SEND/SCIF_RECV IOCTL
// @msg:	message buffer address
// @len:	message length
// @flags:	flags
// @out_len:	number of bytes sent/received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scifioctl_msg {
    pub msg: __u64,
    pub len: __s32,
    pub flags: __s32,
    pub out_len: __s32,
}

//
// struct scifioctl_reg - used for SCIF_REG IOCTL
// @addr:	starting virtual address
// @len:	length of range
// @offset:	offset of window
// @prot:	read/write protection
// @flags:	flags
// @out_offset:	offset returned
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scifioctl_reg {
    pub addr: __u64,
    pub len: __u64,
    pub offset: __s64,
    pub prot: __s32,
    pub flags: __s32,
    pub out_offset: __s64,
}

//
// struct scifioctl_unreg - used for SCIF_UNREG IOCTL
// @offset:	start of range to unregister
// @len:	length of range to unregister
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scifioctl_unreg {
    pub offset: __s64,
    pub len: __u64,
}

//
// struct scifioctl_copy - used for SCIF DMA copy IOCTLs
//
// @loffset:	offset in local registered address space to/from
// which to copy
// @len:	length of range to copy
// @roffset:	offset in remote registered address space to/from
// which to copy
// @addr:	user virtual address to/from which to copy
// @flags:	flags
//
// This structure is used for SCIF_READFROM, SCIF_WRITETO, SCIF_VREADFROM
// and SCIF_VREADFROM IOCTL's.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scifioctl_copy {
    pub loffset: __s64,
    pub len: __u64,
    pub roffset: __s64,
    pub addr: __u64,
    pub flags: __s32,
}

//
// struct scifioctl_fence_mark  - used for SCIF_FENCE_MARK IOCTL
// @flags:	flags
// @mark:	fence handle which is a pointer to a __s32
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scifioctl_fence_mark {
    pub flags: __s32,
    pub mark: __u64,
}

//
// struct scifioctl_fence_signal - used for SCIF_FENCE_SIGNAL IOCTL
// @loff:	local offset
// @lval:	value to write to loffset
// @roff:	remote offset
// @rval:	value to write to roffset
// @flags:	flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scifioctl_fence_signal {
    pub loff: __s64,
    pub lval: __u64,
    pub roff: __s64,
    pub rval: __u64,
    pub flags: __s32,
}

//
// struct scifioctl_node_ids - used for SCIF_GET_NODEIDS IOCTL
// @nodes:	pointer to an array of node_ids
// @self:	ID of the current node
// @len:	length of array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scifioctl_node_ids {
    pub nodes: __u64,
    pub self: __u64,
    pub len: __s32,
}

