//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ppp-ioctl.h
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
// ppp-ioctl.h - PPP ioctl definitions.
//
// Copyright 1999-2002 Paul Mackerras.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// version 2 as published by the Free Software Foundation.
//

//
// Bit definitions for flags argument to PPPIOCGFLAGS/PPPIOCSFLAGS.
//
pub const SC_COMP_PROT: c_uint = 0x00000001	/* protocol compression (output) */;
pub const SC_COMP_AC: c_uint = 0x00000002	/* header compression (output) */;
pub const SC_COMP_TCP: c_uint = 0x00000004	/* TCP (VJ) compression (output) */;
pub const SC_NO_TCP_CCID: c_uint = 0x00000008	/* disable VJ connection-id comp. */;
pub const SC_REJ_COMP_AC: c_uint = 0x00000010	/* reject adrs/ctrl comp. on input */;
pub const SC_REJ_COMP_TCP: c_uint = 0x00000020	/* reject TCP (VJ) comp. on input */;
pub const SC_CCP_OPEN: c_uint = 0x00000040	/* Look at CCP packets */;
pub const SC_CCP_UP: c_uint = 0x00000080	/* May send/recv compressed packets */;
pub const SC_ENABLE_IP: c_uint = 0x00000100	/* IP packets may be exchanged */;
pub const SC_LOOP_TRAFFIC: c_uint = 0x00000200	/* send traffic to pppd */;
pub const SC_MULTILINK: c_uint = 0x00000400	/* do multilink encapsulation */;
pub const SC_MP_SHORTSEQ: c_uint = 0x00000800	/* use short MP sequence numbers */;
pub const SC_COMP_RUN: c_uint = 0x00001000	/* compressor has been inited */;
pub const SC_DECOMP_RUN: c_uint = 0x00002000	/* decompressor has been inited */;
pub const SC_MP_XSHORTSEQ: c_uint = 0x00004000	/* transmit short MP seq numbers */;
pub const SC_DEBUG: c_uint = 0x00010000	/* enable debug messages */;
pub const SC_LOG_INPKT: c_uint = 0x00020000	/* log contents of good pkts recvd */;
pub const SC_LOG_OUTPKT: c_uint = 0x00040000	/* log contents of pkts sent */;
pub const SC_LOG_RAWIN: c_uint = 0x00080000	/* log all chars received */;
pub const SC_LOG_FLUSH: c_uint = 0x00100000	/* log all chars flushed */;
pub const SC_SYNC: c_uint = 0x00200000	/* synchronous serial mode */;
pub const SC_MUST_COMP: c_uint = 0x00400000	/* no uncompressed packets may be sent or received */;
pub const SC_MASK: c_uint = 0x0f600fff	/* bits that user can change */;
// state bits
pub const SC_XMIT_BUSY: c_uint = 0x10000000	/* (used by isdn_ppp?) */;
pub const SC_RCV_ODDP: c_uint = 0x08000000	/* have rcvd char with odd parity */;
pub const SC_RCV_EVNP: c_uint = 0x04000000	/* have rcvd char with even parity */;
pub const SC_RCV_B7_1: c_uint = 0x02000000	/* have rcvd char with bit 7 = 1 */;
pub const SC_RCV_B7_0: c_uint = 0x01000000	/* have rcvd char with bit 7 = 0 */;
pub const SC_DC_FERROR: c_uint = 0x00800000	/* fatal decomp error detected */;
pub const SC_DC_ERROR: c_uint = 0x00400000	/* non-fatal decomp error detected */;
// Used with PPPIOCGNPMODE/PPPIOCSNPMODE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npioctl {
    pub /: *mut *mut int protocol; / PPP protocol, e.g. PPP_IP,
    pub mode: NPmode,
}

// Structure describing a CCP configuration option, for PPPIOCSCOMPRESS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppp_option_data {
    pub ptr: *mut __u8 __user,
    pub length: __u32,
    pub transmit: c_int,
}

// For PPPIOCGL2TPSTATS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppol2tp_ioc_stats {
    pub /: *mut *mut __u16 tunnel_id; / redundant,
    pub /: *mut *mut __u16 session_id; / if zero, get tunnel stats,
    pub using_ipsec:1: __u32,
    pub tx_packets: __aligned_u64,
    pub tx_bytes: __aligned_u64,
    pub tx_errors: __aligned_u64,
    pub rx_packets: __aligned_u64,
    pub rx_bytes: __aligned_u64,
    pub rx_seq_discards: __aligned_u64,
    pub rx_oos_packets: __aligned_u64,
    pub rx_errors: __aligned_u64,
}

//
// Ioctl definitions.
//

