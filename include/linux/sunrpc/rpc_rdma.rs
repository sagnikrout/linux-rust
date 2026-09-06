//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/rpc_rdma.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2015-2017 Oracle. All rights reserved.
// Copyright (c) 2003-2007 Network Appliance, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the BSD-type
// license below:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//
// Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials provided
// with the distribution.
//
// Neither the name of the Network Appliance, Inc. nor the names of
// its contributors may be used to endorse or promote products
// derived from this software without specific prior written
// permission.
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

pub const RPCRDMA_VERSION: c_int = 1;

//
// XDR sizes, in quads
//
// Smallest RPC/RDMA header: rm_xid through rm_type, then rm_nochunks
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpcrdma_errcode {
    ERR_VERS = 1,
    ERR_CHUNK = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpcrdma_proc {
    RDMA_MSG = 0,		/* An RPC call or reply msg */
    RDMA_NOMSG = 1,		/* An RPC call or reply msg - separate body */
    RDMA_MSGP = 2,		/* An RPC call or reply msg with padding */
    RDMA_DONE = 3,		/* Client signals reply completion */
    RDMA_ERROR = 4		/* An RPC RDMA encoding error */
}

//
// Private extension to RPC-over-RDMA Version One.
// Message passed during RDMA-CM connection set-up.
//
// Add new fields at the end, and don't permute existing
// fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcrdma_connect_private {
    pub cp_magic: __be32,
    pub cp_version: u8,
    pub cp_flags: u8,
    pub cp_send_size: u8,
    pub cp_recv_size: u8,
    pub __packed: },

}

//
// xdr_encode_rdma_segment - Encode contents of an RDMA segment
// @p: Pointer into a send buffer
// @handle: The RDMA handle to encode
// @length: The RDMA length to encode
// @offset: The RDMA offset to encode
//
// Return value:
// Pointer to the XDR position that follows the encoded RDMA segment
//
// p++ = cpu_to_be32(handle);
// p++ = cpu_to_be32(length);
extern "C" {
    pub fn xdr_encode_hyper(_arg: p, _arg: offset) -> return;
}
//
// xdr_encode_read_segment - Encode contents of a Read segment
// @p: Pointer into a send buffer
// @position: The position to encode
// @handle: The RDMA handle to encode
// @length: The RDMA length to encode
// @offset: The RDMA offset to encode
//
// Return value:
// Pointer to the XDR position that follows the encoded Read segment
//
// p++ = cpu_to_be32(position);
extern "C" {
    pub fn xdr_encode_rdma_segment(_arg: p, _arg: handle, _arg: length, _arg: offset) -> return;
}
//
// xdr_decode_rdma_segment - Decode contents of an RDMA segment
// @p: Pointer to the undecoded RDMA segment
// @handle: Upon return, the RDMA handle
// @length: Upon return, the RDMA length
// @offset: Upon return, the RDMA offset
//
// Return value:
// Pointer to the XDR item that follows the RDMA segment
//
// handle = be32_to_cpup(p++);
// length = be32_to_cpup(p++);
extern "C" {
    pub fn xdr_decode_hyper(_arg: p, _arg: offset) -> return;
}
//
// xdr_decode_read_segment - Decode contents of a Read segment
// @p: Pointer to the undecoded Read segment
// @position: Upon return, the segment's position
// @handle: Upon return, the RDMA handle
// @length: Upon return, the RDMA length
// @offset: Upon return, the RDMA offset
//
// Return value:
// Pointer to the XDR item that follows the Read segment
//
// position = be32_to_cpup(p++);
extern "C" {
    pub fn xdr_decode_rdma_segment(_arg: p, _arg: handle, _arg: length, _arg: offset) -> return;
}
