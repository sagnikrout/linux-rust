//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vm_sockets.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// VMware vSockets Driver
//
// Copyright (C) 2007-2013 VMware, Inc. All rights reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation version 2 and no later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//

// Option name for STREAM socket buffer size.  Use as the option name in
// setsockopt(3) or getsockopt(3) to set or get an unsigned long long that
// specifies the size of the buffer underlying a vSockets STREAM socket.
// Value is clamped to the MIN and MAX.
//
pub const SO_VM_SOCKETS_BUFFER_SIZE: c_int = 0;
// Option name for STREAM socket minimum buffer size.  Use as the option name
// in setsockopt(3) or getsockopt(3) to set or get an unsigned long long that
// specifies the minimum size allowed for the buffer underlying a vSockets
// STREAM socket.
//
pub const SO_VM_SOCKETS_BUFFER_MIN_SIZE: c_int = 1;
// Option name for STREAM socket maximum buffer size.  Use as the option name
// in setsockopt(3) or getsockopt(3) to set or get an unsigned long long
// that specifies the maximum size allowed for the buffer underlying a
// vSockets STREAM socket.
//
pub const SO_VM_SOCKETS_BUFFER_MAX_SIZE: c_int = 2;
// Option name for socket peer's host-specific VM ID.  Use as the option name
// in getsockopt(3) to get a host-specific identifier for the peer endpoint's
// VM.  The identifier is a signed integer.
// Only available for hypervisor endpoints.
//
pub const SO_VM_SOCKETS_PEER_HOST_VM_ID: c_int = 3;
// Option name for determining if a socket is trusted.  Use as the option name
// in getsockopt(3) to determine if a socket is trusted.  The value is a
// signed integer.
//
pub const SO_VM_SOCKETS_TRUSTED: c_int = 5;
// Option name for STREAM socket connection timeout.  Use as the option name
// in setsockopt(3) or getsockopt(3) to set or get the connection
// timeout for a STREAM socket.
//
pub const SO_VM_SOCKETS_CONNECT_TIMEOUT_OLD: c_int = 6;
// Option name for using non-blocking send/receive.  Use as the option name
// for setsockopt(3) or getsockopt(3) to set or get the non-blocking
// transmit/receive flag for a STREAM socket.  This flag determines whether
// send() and recv() can be called in non-blocking contexts for the given
// socket.  The value is a signed integer.
//
// This option is only relevant to kernel endpoints, where descheduling the
// thread of execution is not allowed, for example, while holding a spinlock.
// It is not to be confused with conventional non-blocking socket operations.
//
// Only available for hypervisor endpoints.
//
pub const SO_VM_SOCKETS_NONBLOCK_TXRX: c_int = 7;
pub const SO_VM_SOCKETS_CONNECT_TIMEOUT_NEW: c_int = 8;

// The vSocket equivalent of INADDR_ANY.  This works for the svm_cid field of
// sockaddr_vm and indicates the context ID of the current endpoint.
//

// Bind to any available port.  Works for the svm_port field of
// sockaddr_vm.
//

// Use this as the destination CID in an address when referring to the
// hypervisor.  VMCI relies on it being 0, but this would be useful for other
// transports too.
//
pub const VMADDR_CID_HYPERVISOR: c_int = 0;
// Use this as the destination CID in an address when referring to the
// local communication (loopback).
// (This was VMADDR_CID_RESERVED, but even VMCI doesn't use it anymore,
// it was a legacy value from an older release).
//
pub const VMADDR_CID_LOCAL: c_int = 1;
// Use this as the destination CID in an address when referring to the host
// (any process other than the hypervisor).  VMCI relies on it being 2, but
// this would be useful for other transports too.
//
pub const VMADDR_CID_HOST: c_int = 2;
// The current default use case for the vsock channel is the following:
// local vsock communication between guest and host and nested VMs setup.
// In addition to this, implicitly, the vsock packets are forwarded to the host
// if no host->guest vsock transport is set.
//
// Set this flag value in the sockaddr_vm corresponding field if the vsock
// packets need to be always forwarded to the host. Using this behavior,
// vsock communication between sibling VMs can be setup.
//
// This way can explicitly distinguish between vsock channels created for
// different use cases, such as nested VMs (or local communication between
// guest and host) and sibling VMs.
//
// The flag can be set in the connect logic in the user space application flow.
// In the listen logic (from kernel space) the flag is set on the remote peer
// address. This happens for an incoming connection when it is routed from the
// host and comes from the guest (local CID and remote CID > VMADDR_CID_HOST).
//
pub const VMADDR_FLAG_TO_HOST: c_uint = 0x01;
// Invalid vSockets version.

// The epoch (first) component of the vSockets version.  A single byte
// representing the epoch component of the vSockets version.
//

// The major (second) component of the vSockets version.   A single byte
// representing the major component of the vSockets version.  Typically
// changes for every major release of a product.
//

// The minor (third) component of the vSockets version.  Two bytes representing
// the minor component of the vSockets version.
//

// Address structure for vSockets.   The address family should be set to
// AF_VSOCK.  The structure members should all align on their natural
// boundaries without resorting to compiler packing directives.  The total size
// of this structure should be exactly the same as that of struct sockaddr.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_vm {
    pub svm_family: __kernel_sa_family_t,
    pub svm_reserved1: c_ushort,
    pub svm_port: c_uint,
    pub svm_cid: c_uint,
    pub svm_flags: __u8,
}

// MSG_ZEROCOPY notifications are encoded in the standard error format,
// sock_extended_err. See Documentation/networking/msg_zerocopy.rst in
// kernel source tree for more details.
//
// 'cmsg_level' field value of 'struct cmsghdr' for notification parsing
// when MSG_ZEROCOPY flag is used on transmissions.
//
pub const SOL_VSOCK: c_int = 287;
// 'cmsg_type' field value of 'struct cmsghdr' for notification parsing
// when MSG_ZEROCOPY flag is used on transmissions.
//
pub const VSOCK_RECVERR: c_int = 1;
