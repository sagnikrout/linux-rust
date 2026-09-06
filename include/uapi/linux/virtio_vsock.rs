//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_vsock.h
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
// This header, excluding the #ifdef __KERNEL__ part, is BSD licensed so
// anyone can use the definitions to implement compatible drivers/servers:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of IBM nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS ``AS IS''
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//
// Copyright (C) Red Hat, Inc., 2013-2015
// Copyright (C) Asias He <asias@redhat.com>, 2013
// Copyright (C) Stefan Hajnoczi <stefanha@redhat.com>, 2015
//

// The feature bitmap for virtio vsock

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_vsock_config {
    pub guest_cid: __le64,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_vsock_event_id {
    VIRTIO_VSOCK_EVENT_TRANSPORT_RESET = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_vsock_event {
    pub id: __le32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_vsock_hdr {
    pub src_cid: __le64,
    pub dst_cid: __le64,
    pub src_port: __le32,
    pub dst_port: __le32,
    pub len: __le32,
    pub /: *mut *mut __le16 type; / enum virtio_vsock_type,
    pub /: *mut *mut __le16 op; / enum virtio_vsock_op,
    pub flags: __le32,
    pub buf_alloc: __le32,
    pub fwd_cnt: __le32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_vsock_type {
    VIRTIO_VSOCK_TYPE_STREAM = 1,
    VIRTIO_VSOCK_TYPE_SEQPACKET = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_vsock_op {
    VIRTIO_VSOCK_OP_INVALID = 0,

// Connect operations
    VIRTIO_VSOCK_OP_REQUEST = 1,
    VIRTIO_VSOCK_OP_RESPONSE = 2,
    VIRTIO_VSOCK_OP_RST = 3,
    VIRTIO_VSOCK_OP_SHUTDOWN = 4,

// To send payload
    VIRTIO_VSOCK_OP_RW = 5,

// Tell the peer our credit info
    VIRTIO_VSOCK_OP_CREDIT_UPDATE = 6,
// Request the peer to send the credit info to us
    VIRTIO_VSOCK_OP_CREDIT_REQUEST = 7,
}

// VIRTIO_VSOCK_OP_SHUTDOWN flags values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_vsock_shutdown {
    VIRTIO_VSOCK_SHUTDOWN_RCV = 1,
    VIRTIO_VSOCK_SHUTDOWN_SEND = 2,
}

// VIRTIO_VSOCK_OP_RW flags values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_vsock_rw {
    VIRTIO_VSOCK_SEQ_EOM = 1,
    VIRTIO_VSOCK_SEQ_EOR = 2,
}
