//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/vmxnet3/upt1_defs.h
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
// Linux driver for VMware's vmxnet3 ethernet NIC.
//
// Copyright (C) 2008-2022, VMware, Inc. All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; version 2 of the License and no later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
//
// The full GNU General Public License is included in this distribution in
// the file called "COPYING".
//
// Maintained by: pv-drivers@vmware.com
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UPT1_TxStats {
    pub /: *mut *mut u64 TSOPktsTxOK; / TSO pkts post-segmentation,
    pub TSOBytesTxOK: u64,
    pub ucastPktsTxOK: u64,
    pub ucastBytesTxOK: u64,
    pub mcastPktsTxOK: u64,
    pub mcastBytesTxOK: u64,
    pub bcastPktsTxOK: u64,
    pub bcastBytesTxOK: u64,
    pub pktsTxError: u64,
    pub pktsTxDiscard: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UPT1_RxStats {
    pub /: *mut *mut u64 LROPktsRxOK; / LRO pkts,
    pub /: *mut *mut u64 LROBytesRxOK; / bytes from LRO pkts,
// the following counters are for pkts from the wire, i.e., pre-LRO
    pub ucastPktsRxOK: u64,
    pub ucastBytesRxOK: u64,
    pub mcastPktsRxOK: u64,
    pub mcastBytesRxOK: u64,
    pub bcastPktsRxOK: u64,
    pub bcastBytesRxOK: u64,
    pub pktsRxOutOfBuf: u64,
    pub pktsRxError: u64,
}

// interrupt moderation level
// values for UPT1_RSSConf.hashFunc
pub const UPT1_RSS_MAX_KEY_SIZE: c_int = 40;
pub const UPT1_RSS_MAX_IND_TABLE_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UPT1_RSSConf {
    pub hashType: u16,
    pub hashFunc: u16,
    pub hashKeySize: u16,
    pub indTableSize: u16,
    pub hashKey: [u8; UPT1_RSS_MAX_KEY_SIZE],
    pub indTable: [u8; UPT1_RSS_MAX_IND_TABLE_SIZE],
}

// features
// offloading
//
