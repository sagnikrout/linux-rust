//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_hippi.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Global definitions for the HIPPI interface.
//
// Version:	@(#)if_hippi.h	1.0.0	05/26/97
//
// Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Donald Becker, <becker@super.org>
// Alan Cox, <alan@lxorguk.ukuu.org.uk>
// Steve Whitehouse, <gw7rrm@eeshack3.swan.ac.uk>
// Jes Sorensen, <Jes.Sorensen@cern.ch>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

//
// HIPPI magic constants.
//

// Max. bytes in frame without FCS
//
// Define LLC and SNAP constants.
//
pub const HIPPI_EXTENDED_SAP: c_uint = 0xAA;
pub const HIPPI_UI_CMD: c_uint = 0x03;
//
// Do we need to list some sort of ID's here?
//
// HIPPI statistics collection data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipnet_statistics {
    pub /: *mut *mut int rx_packets; / total packets received,
    pub /: *mut *mut int tx_packets; / total packets transmitted,
    pub /: *mut *mut int rx_errors; / bad packets received,
    pub /: *mut *mut int tx_errors; / packet transmit problems,
    pub /: *mut *mut int rx_dropped; / no space in linux buffers,
    pub /: *mut *mut int tx_dropped; / no space available in linux,
// detailed rx_errors:
    pub rx_length_errors: c_int,
    pub /: *mut *mut int rx_over_errors; / receiver ring buff overflow,
    pub /: *mut *mut int rx_crc_errors; / recved pkt with crc error,
    pub /: *mut *mut int rx_frame_errors; / recv'd frame alignment error,
    pub /: *mut *mut int rx_fifo_errors; / recv'r fifo overrun,
    pub /: *mut *mut int rx_missed_errors; / receiver missed packet,
// detailed tx_errors
    pub tx_aborted_errors: c_int,
    pub tx_carrier_errors: c_int,
    pub tx_fifo_errors: c_int,
    pub tx_heartbeat_errors: c_int,
    pub tx_window_errors: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hippi_fp_hdr {

    pub /: *mut *mut __u8 ulp; / must contain 4,

    pub /: *mut *mut __u8 d1_data_present:1; / must be 1,
    pub /: *mut *mut __u8 start_d2_burst_boundary:1; / must be zero,
    pub /: *mut *mut __u8 reserved:6; / must be zero,

    pub reserved1:5: __u16,
    pub /: *mut *mut __u16 d1_area_size:8; / must be 3,
    pub /: *mut *mut __u16 d2_offset:3; / must be zero,

    pub /: *mut *mut __u8 reserved:6; / must be zero,
    pub /: *mut *mut __u8 start_d2_burst_boundary:1; / must be zero,
    pub /: *mut *mut __u8 d1_data_present:1; / must be 1,

    pub /: *mut *mut __u16 d2_offset:3; / must be zero,
    pub /: *mut *mut __u16 d1_area_size:8; / must be 3,
    pub /: *mut *mut __u16 reserved1:5; / must be zero,

    pub fixed: __be32,

    pub d2_size: __be32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hippi_le_hdr {

    pub fc:3: __u8,
    pub double_wide:1: __u8,
    pub message_type:4: __u8,

    pub message_type:4: __u8,
    pub double_wide:1: __u8,
    pub fc:3: __u8,
    pub dest_switch_addr: [__u8; 3],    pub src_switch_addr: [__u8; 3],
    pub reserved: __u16,
    pub daddr: [__u8; HIPPI_ALEN],
    pub locally_administered: __u16,
    pub saddr: [__u8; HIPPI_ALEN],
    pub __attribute__((packed)): },
pub const HIPPI_OUI_LEN: c_int = 3;
//
// Looks like the dsap and ssap fields have been swapped by mistake in
// RFC 2067 "IP over HIPPI".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hippi_snap_hdr {
    pub /: *mut *mut __u8 dsap; / always 0xAA,
    pub /: *mut *mut __u8 ssap; / always 0xAA,
    pub /: *mut *mut __u8 ctrl; / always 0x03,
    pub (zero)*/: *mut *mut __u8 oui[HIPPI_OUI_LEN]; / organizational universal id,
    pub /: *mut *mut __be16 ethertype; / packet type ID field,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hippi_hdr {
    pub fp: hippi_fp_hdr,
    pub le: hippi_le_hdr,
    pub snap: hippi_snap_hdr,
    pub __attribute__((packed)): },
