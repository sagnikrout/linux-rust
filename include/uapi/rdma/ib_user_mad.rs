//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/rdma/ib_user_mad.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB)
//
// Copyright (c) 2004 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Voltaire, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// Increment this value if any changes that break userspace ABI
// compatibility are made.
//
pub const IB_USER_MAD_ABI_VERSION: c_int = 5;
//
// Make sure that all structs defined in this file remain laid out so
// that they pack the same way on 32-bit and 64-bit architectures (to
// avoid incompatibility between 32-bit userspace and 64-bit kernels).
//
// struct ib_user_mad_hdr_old - Old version of MAD packet header without pkey_index
// @id: ID of agent MAD received with/to be sent with
// @status: 0 on successful receive, ETIMEDOUT if no response
// received (transaction ID in data[] will be set to TID of original
// request) (ignored on send)
// @timeout_ms: Milliseconds to wait for response (unset on receive)
// @retries: Number of automatic retries to attempt
// @qpn: Remote QP number received from/to be sent to
// @qkey: Remote Q_Key to be sent with (unset on receive)
// @lid: Remote lid received from/to be sent to
// @sl: Service level received with/to be sent with
// @path_bits: Local path bits received with/to be sent with
// @grh_present: If set, GRH was received/should be sent
// @gid_index: Local GID index to send with (unset on receive)
// @hop_limit: Hop limit in GRH
// @traffic_class: Traffic class in GRH
// @gid: Remote GID in GRH
// @flow_label: Flow label in GRH
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_user_mad_hdr_old {
    pub id: __u32,
    pub status: __u32,
    pub timeout_ms: __u32,
    pub retries: __u32,
    pub length: __u32,
    pub qpn: __be32,
    pub qkey: __be32,
    pub lid: __be16,
    pub sl: __u8,
    pub path_bits: __u8,
    pub grh_present: __u8,
    pub gid_index: __u8,
    pub hop_limit: __u8,
    pub traffic_class: __u8,
    pub gid: [__u8; 16],
    pub flow_label: __be32,
}

//
// struct ib_user_mad_hdr - MAD packet header
// This layout allows specifying/receiving the P_Key index.  To use
// this capability, an application must call the
// IB_USER_MAD_ENABLE_PKEY ioctl on the user MAD file handle before
// any other actions with the file handle.
// @id: ID of agent MAD received with/to be sent with
// @status: 0 on successful receive, ETIMEDOUT if no response
// received (transaction ID in data[] will be set to TID of original
// request) (ignored on send)
// @timeout_ms: Milliseconds to wait for response (unset on receive)
// @retries: Number of automatic retries to attempt
// @qpn: Remote QP number received from/to be sent to
// @qkey: Remote Q_Key to be sent with (unset on receive)
// @lid: Remote lid received from/to be sent to
// @sl: Service level received with/to be sent with
// @path_bits: Local path bits received with/to be sent with
// @grh_present: If set, GRH was received/should be sent
// @gid_index: Local GID index to send with (unset on receive)
// @hop_limit: Hop limit in GRH
// @traffic_class: Traffic class in GRH
// @gid: Remote GID in GRH
// @flow_label: Flow label in GRH
// @pkey_index: P_Key index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_user_mad_hdr {
    pub id: __u32,
    pub status: __u32,
    pub timeout_ms: __u32,
    pub retries: __u32,
    pub length: __u32,
    pub qpn: __be32,
    pub qkey: __be32,
    pub lid: __be16,
    pub sl: __u8,
    pub path_bits: __u8,
    pub grh_present: __u8,
    pub gid_index: __u8,
    pub hop_limit: __u8,
    pub traffic_class: __u8,
    pub gid: [__u8; 16],
    pub flow_label: __be32,
    pub pkey_index: __u16,
    pub reserved: [__u8; 6],
}

//
// struct ib_user_mad - MAD packet
// @hdr: MAD packet header
// @data: Contents of MAD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_user_mad {
    pub hdr: ib_user_mad_hdr,
    pub data: [__aligned_u64; ],
}

//
// Earlier versions of this interface definition declared the
// method_mask[] member as an array of __u32 but treated it as a
// bitmap made up of longs in the kernel.  This ambiguity meant that
// 32-bit big-endian applications that can run on both 32-bit and
// 64-bit kernels had no consistent ABI to rely on, and 64-bit
// big-endian applications that treated method_mask as being made up
// of 32-bit words would have their bitmap misinterpreted.
//
// To clear up this confusion, we change the declaration of
// method_mask[] to use unsigned long and handle the conversion from
// 32-bit userspace to 64-bit kernel for big-endian systems in the
// compat_ioctl method.  Unfortunately, to keep the structure layout
// the same, we need the method_mask[] array to be aligned only to 4
// bytes even when long is 64 bits, which forces us into this ugly
// typedef.
//

//
// struct ib_user_mad_reg_req - MAD registration request
// @id: Set by the kernel; used to identify agent in future requests.
// @qpn: Queue pair number; must be 0 or 1.
// @method_mask: The caller will receive unsolicited MADs for any method
// where @method_mask = 1.
// @mgmt_class: Indicates which management class of MADs should be receive
// by the caller.  This field is only required if the user wishes to
// receive unsolicited MADs, otherwise it should be 0.
// @mgmt_class_version: Indicates which version of MADs for the given
// management class to receive.
// @oui: Indicates IEEE OUI when mgmt_class is a vendor class
// in the range from 0x30 to 0x4f. Otherwise not used.
// @rmpp_version: If set, indicates the RMPP version used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_user_mad_reg_req {
    pub id: __u32,
    pub method_mask: [packed_ulong; IB_USER_MAD_LONGS_PER_METHOD_MASK],
    pub qpn: __u8,
    pub mgmt_class: __u8,
    pub mgmt_class_version: __u8,
    pub oui: [__u8; 3],
    pub rmpp_version: __u8,
}

//
// struct ib_user_mad_reg_req2 - MAD registration request
//
// @id                 - Set by the _kernel_; used by userspace to identify the
// registered agent in future requests.
// @qpn                - Queue pair number; must be 0 or 1.
// @mgmt_class         - Indicates which management class of MADs should be
// receive by the caller.  This field is only required if
// the user wishes to receive unsolicited MADs, otherwise
// it should be 0.
// @mgmt_class_version - Indicates which version of MADs for the given
// management class to receive.
// @res                - Ignored.
// @flags              - additional registration flags; Must be in the set of
// flags defined in IB_USER_MAD_REG_FLAGS_CAP
// @method_mask        - The caller wishes to receive unsolicited MADs for the
// methods whose bit(s) is(are) set.
// @oui                - Indicates IEEE OUI to use when mgmt_class is a vendor
// class in the range from 0x30 to 0x4f. Otherwise not
// used.
// @rmpp_version       - If set, indicates the RMPP version to use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_user_mad_reg_req2 {
    pub id: __u32,
    pub qpn: __u32,
    pub mgmt_class: __u8,
    pub mgmt_class_version: __u8,
    pub res: __u16,
    pub flags: __u32,
    pub method_mask: [__aligned_u64; 2],
    pub oui: __u32,
    pub rmpp_version: __u8,
    pub reserved: [__u8; 3],
}

