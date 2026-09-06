//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/6lowpan.h
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
// Copyright 2011, Siemens AG
// written by Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
//
// Based on patches from Jon Smirl <jonsmirl@gmail.com>
// Copyright (c) 2011 Jon Smirl <jonsmirl@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
//
// Jon's code is based on 6lowpan implementation for Contiki which is:
// Copyright (c) 2008, Swedish Institute of Computer Science.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of the Institute nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE INSTITUTE AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL THE INSTITUTE OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

// special link-layer handling

pub const EUI64_ADDR_LEN: c_int = 8;
pub const LOWPAN_NHC_MAX_ID_LEN: c_int = 1;
// Maximum next header compression length which we currently support inclusive
// possible inline data.
//

// Max IPHC Header len without IPv6 hdr specific inline data.
// Useful for getting the "extra" bytes we need at worst case compression.
//
// LOWPAN_IPHC + CID + LOWPAN_NHC_MAX_ID_LEN
//

// Maximum worst case IPHC header buffer size

// SCI/DCI is 4 bit width, so we have maximum 16 entries

pub const LOWPAN_DISPATCH_IPV6: c_uint = 0x41 /* 01000001 = 65 */;
pub const LOWPAN_DISPATCH_IPHC: c_uint = 0x60 /* 011xxxxx = ... */;
pub const LOWPAN_DISPATCH_IPHC_MASK: c_uint = 0xe0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lowpan_lltypes {
    LOWPAN_LLTYPE_BTLE,
    LOWPAN_LLTYPE_IEEE802154,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lowpan_iphc_ctx_flags {
    LOWPAN_IPHC_CTX_FLAG_ACTIVE,
    LOWPAN_IPHC_CTX_FLAG_COMPRESSION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lowpan_iphc_ctx {
    pub id: u8,
    pub pfx: in6_addr,
    pub plen: u8,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lowpan_iphc_ctx_table {
    pub lock: spinlock_t,
    pub ops: *const lowpan_iphc_ctx_ops,
    pub table: [lowpan_iphc_ctx; LOWPAN_IPHC_CTX_TABLE_SIZE],
}

extern "C" {
    pub fn test_bit(_arg: LOWPAN_IPHC_CTX_FLAG_ACTIVE, _arg: &ctx->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: LOWPAN_IPHC_CTX_FLAG_COMPRESSION, _arg: &ctx->flags) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lowpan_dev {
    pub lltype: lowpan_lltypes,
    pub iface_debugfs: *mut dentry,
    pub ctx: lowpan_iphc_ctx_table,
// must be last
    pub )): *mut u8 priv[] __aligned(sizeof(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lowpan_802154_neigh {
    pub short_addr: __le16,
}

extern "C" {
    pub fn netdev_priv(_arg: dev) -> return;
}
// private device info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lowpan_802154_dev {
    pub /: *mut *mut *mut net_device wdev; / wpan device ptr,
    pub fragment_tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lowpan_802154_cb {
    pub d_tag: u16,
    pub d_size: c_uint,
    pub d_offset: u8,
}

// fe:80::XXXX:XXXX:XXXX:XXXX
// \_________________
// hwaddr
//
// second bit-flip (Universe/Local)
// is done according RFC2464
//
// fe:80::XXXX:XXff:feXX:XXXX
// \_________________
// hwaddr
//

// print data in line
// print data in a table format:
//
// addr: xx xx xx xx xx xx
// ...
//

//
// lowpan_fetch_skb - getting inline data from 6LoWPAN header
//
// This function will pull data from sk buffer and put it into data to
// remove the 6LoWPAN inline data. This function returns true if the
// sk buffer is too small to pull the amount of data which is specified
// by len.
//
// @skb: the buffer where the inline data should be pulled from.
// @data: destination buffer for the inline data.
// @len: amount of data which should be pulled in bytes.
//
// First bit of addr is multicast, reserved or 802.15.4 specific
// hc_ptr += len;
extern "C" {
    pub fn lowpan_unregister_netdevice(dev: *mut net_device);
}
extern "C" {
    pub fn lowpan_unregister_netdev(dev: *mut net_device);
}
//
// lowpan_header_decompress - replace 6LoWPAN header with IPv6 header
//
// This function replaces the IPHC 6LoWPAN header which should be pointed at
// skb->data and skb_network_header, with the IPv6 header.
// It would be nice that the caller have the necessary headroom of IPv6 header
// and greatest Transport layer header, this would reduce the overhead for
// reallocate headroom.
//
// @skb: the buffer which should be manipulate.
// @dev: the lowpan net device pointer.
// @daddr: destination lladdr of mac header which is used for compression
// methods.
// @saddr: source lladdr of mac header which is used for compression
// methods.
//
// lowpan_header_compress - replace IPv6 header with 6LoWPAN header
//
// This function replaces the IPv6 header which should be pointed at
// skb->data and skb_network_header, with the IPHC 6LoWPAN header.
// The caller need to be sure that the sk buffer is not shared and at have
// at least a headroom which is smaller or equal LOWPAN_IPHC_MAX_HEADER_LEN,
// which is the IPHC "more bytes than IPv6 header" at worst case.
//
// @skb: the buffer which should be manipulate.
// @dev: the lowpan net device pointer.
// @daddr: destination lladdr of mac header which is used for compression
// methods.
// @saddr: source lladdr of mac header which is used for compression
// methods.
//
