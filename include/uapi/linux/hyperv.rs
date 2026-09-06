//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/hyperv.h
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
// Copyright (c) 2011, Microsoft Corporation.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms and conditions of the GNU General Public License,
// version 2, as published by the Free Software Foundation.
//
// This program is distributed in the hope it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the Free Software Foundation, Inc., 59 Temple
// Place - Suite 330, Boston, MA 02111-1307 USA.
//
// Authors:
// Haiyang Zhang <haiyangz@microsoft.com>
// Hank Janssen  <hjanssen@microsoft.com>
// K. Y. Srinivasan <kys@microsoft.com>
//

//
// Framework version for util services.
//
pub const UTIL_FW_MINOR: c_int = 0;
pub const UTIL_WS2K8_FW_MAJOR: c_int = 1;

pub const UTIL_FW_MAJOR: c_int = 3;

//
// Implementation of host controlled snapshot of the guest.
//
pub const VSS_OP_REGISTER: c_int = 128;
//
pub const VSS_OP_REGISTER1: c_int = 129;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_vss_op {
    VSS_OP_CREATE = 0,
    VSS_OP_DELETE,
    VSS_OP_HOT_BACKUP,
    VSS_OP_GET_DM_INFO,
    VSS_OP_BU_COMPLETE,
//
// Following operations are only supported with IC version >= 5.0
//
    VSS_OP_FREEZE, /* Freeze the file systems in the VM */
    VSS_OP_THAW, /* Unfreeze the file systems */
    VSS_OP_AUTO_RECOVER,
    VSS_OP_COUNT /* Number of operations, must be last */
}

//
// Header for all VSS messages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vss_hdr {
    pub operation: __u8,
    pub reserved: [__u8; 7],
    pub __attribute__((packed)): },
//
// Flag values for the hv_vss_check_feature. Linux supports only
// one value.
//
pub const VSS_HBU_NO_AUTO_RECOVERY: c_uint = 0x00000005;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vss_check_feature {
    pub flags: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vss_check_dm_info {
    pub flags: __u32,
    pub __attribute__((packed)): },
//
// struct hv_vss_msg encodes the fields that the Linux VSS
// driver accesses. However, FREEZE messages from Hyper-V contain
// additional LUN information that Linux doesn't use and are not
// represented in struct hv_vss_msg. A received FREEZE message may
// be as large as 6,260 bytes, so the driver must allocate at least
// that much space, not sizeof(struct hv_vss_msg). Other messages
// such as AUTO_RECOVER may be as large as 12,500 bytes. However,
// because the Linux VSS driver responds that it doesn't support
// auto-recovery, it should not receive such messages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vss_msg {
    pub vss_hdr: hv_vss_hdr,
    pub error: c_int,
}

//
// Implementation of a host to guest copy facility.
//
pub const FCOPY_VERSION_0: c_int = 0;
pub const FCOPY_VERSION_1: c_int = 1;

pub const W_MAX_PATH: c_int = 260;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_fcopy_op {
    START_FILE_COPY = 0,
    WRITE_TO_FILE,
    COMPLETE_FCOPY,
    CANCEL_FCOPY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_fcopy_hdr {
    pub operation: __u32,
    pub /: *mut *mut __u8 service_id0[16]; / currently unused,
    pub /: *mut *mut __u8 service_id1[16]; / currently unused,
    pub __attribute__((packed)): },
pub const OVER_WRITE: c_uint = 0x1;
pub const CREATE_PATH: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_start_fcopy {
    pub hdr: hv_fcopy_hdr,
    pub file_name: [__u16; W_MAX_PATH],
    pub path_name: [__u16; W_MAX_PATH],
    pub copy_flags: __u32,
    pub file_size: __u64,
    pub __attribute__((packed)): },
//
// The file is chunked into fragments.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_do_fcopy {
    pub hdr: hv_fcopy_hdr,
    pub pad: __u32,
    pub offset: __u64,
    pub size: __u32,
    pub data: [__u8; DATA_FRAGMENT],
    pub __attribute__((packed)): },
//
// An implementation of HyperV key value pair (KVP) functionality for Linux.
//
// Copyright (C) 2010, Novell, Inc.
// Author : K. Y. Srinivasan <ksrinivasan@novell.com>
//
// Maximum value size - used for both key names and value data, and includes
// any applicable NULL terminators.
//
// Note:  This limit is somewhat arbitrary, but falls easily within what is
// supported for all native guests (back to Win 2000) and what is reasonable
// for the IC KVP exchange functionality.  Note that Windows Me/98/95 are
// limited to 255 character key names.
//
// MSDN recommends not storing data values larger than 2048 bytes in the
// registry.
//
// Note:  This value is used in defining the KVP exchange message - this value
// cannot be modified without affecting the message size and compatibility.
//
// bytes, including any null terminators
//

//
// Maximum key size - the registry limit for the length of an entry name
// is 256 characters, including the null terminator
//

//
// In Linux, we implement the KVP functionality in two components:
// 1) The kernel component which is packaged as part of the hv_utils driver
// is responsible for communicating with the host and responsible for
// implementing the host/guest protocol. 2) A user level daemon that is
// responsible for data gathering.
//
// Host/Guest Protocol: The host iterates over an index and expects the guest
// to assign a key name to the index and also return the value corresponding to
// the key. The host will have atmost one KVP transaction outstanding at any
// given point in time. The host side iteration stops when the guest returns
// an error. Microsoft has specified the following mapping of key names to
// host specified index:
//
// Index		Key Name
// 0		FullyQualifiedDomainName
// 1		IntegrationServicesVersion
// 2		NetworkAddressIPv4
// 3		NetworkAddressIPv6
// 4		OSBuildNumber
// 5		OSName
// 6		OSMajorVersion
// 7		OSMinorVersion
// 8		OSVersion
// 9		ProcessorArchitecture
//
// The Windows host expects the Key Name and Key Value to be encoded in utf16.
//
// Guest Kernel/KVP Daemon Protocol: As noted earlier, we implement all of the
// data gathering functionality in a user mode daemon. The user level daemon
// is also responsible for binding the key name to the index as well. The
// kernel and user-level daemon communicate using a connector channel.
//
// The user mode component first registers with the
// kernel component. Subsequently, the kernel component requests, data
// for the specified keys. In response to this message the user mode component
// fills in the value corresponding to the specified key. We overload the
// sequence field in the cn_msg header to define our KVP message types.
//
// The kernel component simply acts as a conduit for communication between the
// Windows host and the user-level daemon. The kernel component passes up the
// index received from the Host to the user-level daemon. If the index is
// valid (supported), the corresponding key as well as its
// value (both are strings) is returned. If the index is invalid
// (not supported), a NULL key string is returned.
//
// Registry value types.
//
pub const REG_SZ: c_int = 1;
pub const REG_U32: c_int = 4;
pub const REG_U64: c_int = 8;
//
// As we look at expanding the KVP functionality to include
// IP injection functionality, we need to maintain binary
// compatibility with older daemons.
//
// The KVP opcodes are defined by the host and it was unfortunate
// that I chose to treat the registration operation as part of the
// KVP operations defined by the host.
// Here is the level of compatibility
// (between the user level daemon and the kernel KVP driver) that we
// will implement:
//
// An older daemon will always be supported on a newer driver.
// A given user level daemon will require a minimal version of the
// kernel driver.
// If we cannot handle the version differences, we will fail gracefully
// (this can happen when we have a user level daemon that is more
// advanced than the KVP driver.
//
// We will use values used in this handshake for determining if we have
// workable user level daemon and the kernel driver. We begin by taking the
// registration opcode out of the KVP opcode namespace. We will however,
// maintain compatibility with the existing user-level daemon code.
//
// Daemon code not supporting IP injection (legacy daemon).
//
pub const KVP_OP_REGISTER: c_int = 4;
//
// Daemon code supporting IP injection.
// The KVP opcode field is used to communicate the
// registration information; so define a namespace that
// will be distinct from the host defined KVP opcode.
//
pub const KVP_OP_REGISTER1: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_kvp_exchg_op {
    KVP_OP_GET = 0,
    KVP_OP_SET,
    KVP_OP_DELETE,
    KVP_OP_ENUMERATE,
    KVP_OP_GET_IP_INFO,
    KVP_OP_SET_IP_INFO,
    KVP_OP_COUNT /* Number of operations, must be last. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_kvp_exchg_pool {
    KVP_POOL_EXTERNAL = 0,
    KVP_POOL_GUEST,
    KVP_POOL_AUTO,
    KVP_POOL_AUTO_EXTERNAL,
    KVP_POOL_AUTO_INTERNAL,
    KVP_POOL_COUNT /* Number of pools, must be last. */
}

//
// Some Hyper-V status codes.
//
pub const HV_S_OK: c_uint = 0x00000000;
pub const HV_E_FAIL: c_uint = 0x80004005;
pub const HV_S_CONT: c_uint = 0x80070103;
pub const HV_ERROR_NOT_SUPPORTED: c_uint = 0x80070032;
pub const HV_ERROR_MACHINE_LOCKED: c_uint = 0x800704F7;
pub const HV_ERROR_DEVICE_NOT_CONNECTED: c_uint = 0x8007048F;
pub const HV_INVALIDARG: c_uint = 0x80070057;
pub const HV_GUID_NOTFOUND: c_uint = 0x80041002;
pub const HV_ERROR_ALREADY_EXISTS: c_uint = 0x80070050;
pub const HV_ERROR_DISK_FULL: c_uint = 0x80070070;
pub const ADDR_FAMILY_NONE: c_uint = 0x00;
pub const ADDR_FAMILY_IPV4: c_uint = 0x01;
pub const ADDR_FAMILY_IPV6: c_uint = 0x02;
pub const MAX_ADAPTER_ID_SIZE: c_int = 128;
pub const MAX_IP_ADDR_SIZE: c_int = 1024;
pub const MAX_GATEWAY_SIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_ipaddr_value {
    pub adapter_id: [__u16; MAX_ADAPTER_ID_SIZE],
    pub addr_family: __u8,
    pub dhcp_enabled: __u8,
    pub ip_addr: [__u16; MAX_IP_ADDR_SIZE],
    pub sub_net: [__u16; MAX_IP_ADDR_SIZE],
    pub gate_way: [__u16; MAX_GATEWAY_SIZE],
    pub dns_addr: [__u16; MAX_IP_ADDR_SIZE],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_hdr {
    pub operation: __u8,
    pub pool: __u8,
    pub pad: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_exchg_msg_value {
    pub value_type: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub key: [__u8; HV_KVP_EXCHANGE_MAX_KEY_SIZE],
    pub value: [__u8; HV_KVP_EXCHANGE_MAX_VALUE_SIZE],
    pub value_u32: __u32,
    pub value_u64: __u64,
    pub __attribute__((packed)): },
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg_enumerate {
    pub index: __u32,
    pub data: hv_kvp_exchg_msg_value,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg_get {
    pub data: hv_kvp_exchg_msg_value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg_set {
    pub data: hv_kvp_exchg_msg_value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg_delete {
    pub key_size: __u32,
    pub key: [__u8; HV_KVP_EXCHANGE_MAX_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_register {
    pub version: [__u8; HV_KVP_EXCHANGE_MAX_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_msg {
    pub kvp_hdr: hv_kvp_hdr,
    pub error: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_kvp_ip_msg {
    pub operation: __u8,
    pub pool: __u8,
    pub kvp_ip_val: hv_kvp_ipaddr_value,
    pub __attribute__((packed)): },
