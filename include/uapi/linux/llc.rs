//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/llc.h
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


// SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note
//
// IEEE 802.2 User Interface SAPs for Linux, data structures and indicators.
//
// Copyright (c) 2001 by Jay Schulist <jschlst@samba.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_llc {
    pub /: *mut *mut __kernel_sa_family_t sllc_family; / AF_LLC,
    pub /: *mut *mut __kernel_sa_family_t sllc_arphrd; / ARPHRD_ETHER,
    pub sllc_test: c_uchar,
    pub sllc_xid: c_uchar,
    pub /: *mut *mut unsigned char sllc_ua; / UA data, only for SOCK_STREAM.,
    pub sllc_sap: c_uchar,
    pub sllc_mac: [c_uchar; IFHWADDRLEN],
    pub IFHWADDRLEN]: *mut *mut sizeof(unsigned char)  4 -,
}

// sockopt definitions.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum llc_sockopts {
    LLC_OPT_UNKNOWN = 0,
    LLC_OPT_RETRY,		/* max retrans attempts. */
    LLC_OPT_SIZE,		/* max PDU size (octets). */
    LLC_OPT_ACK_TMR_EXP,	/* ack expire time (secs). */
    LLC_OPT_P_TMR_EXP,	/* pf cycle expire time (secs). */
    LLC_OPT_REJ_TMR_EXP,	/* rej sent expire time (secs). */
    LLC_OPT_BUSY_TMR_EXP,	/* busy state expire time (secs). */
    LLC_OPT_TX_WIN,		/* tx window size. */
    LLC_OPT_RX_WIN,		/* rx window size. */
    LLC_OPT_PKTINFO,	/* ancillary packet information. */
    LLC_OPT_MAX
}

pub const LLC_OPT_MAX_RETRY: c_int = 100;
pub const LLC_OPT_MAX_SIZE: c_int = 4196;
pub const LLC_OPT_MAX_WIN: c_int = 127;
pub const LLC_OPT_MAX_ACK_TMR_EXP: c_int = 60;
pub const LLC_OPT_MAX_P_TMR_EXP: c_int = 60;
pub const LLC_OPT_MAX_REJ_TMR_EXP: c_int = 60;
pub const LLC_OPT_MAX_BUSY_TMR_EXP: c_int = 60;
// LLC SAP types.
pub const LLC_SAP_NULL: c_uint = 0x00		/* NULL SAP. 			*/;
pub const LLC_SAP_LLC: c_uint = 0x02		/* LLC Sublayer Management. 	*/;
pub const LLC_SAP_SNA: c_uint = 0x04		/* SNA Path Control. 		*/;
pub const LLC_SAP_PNM: c_uint = 0x0E		/* Proway Network Management.	*/;
pub const LLC_SAP_IP: c_uint = 0x06		/* TCP/IP. 			*/;
pub const LLC_SAP_BSPAN: c_uint = 0x42		/* Bridge Spanning Tree Proto	*/;
pub const LLC_SAP_MMS: c_uint = 0x4E		/* Manufacturing Message Srv.	*/;
pub const LLC_SAP_8208: c_uint = 0x7E		/* ISO 8208			*/;
pub const LLC_SAP_3COM: c_uint = 0x80		/* 3COM. 			*/;
pub const LLC_SAP_PRO: c_uint = 0x8E		/* Proway Active Station List	*/;
pub const LLC_SAP_SNAP: c_uint = 0xAA		/* SNAP. 			*/;
pub const LLC_SAP_BANYAN: c_uint = 0xBC		/* Banyan. 			*/;
pub const LLC_SAP_IPX: c_uint = 0xE0		/* IPX/SPX. 			*/;
pub const LLC_SAP_NETBEUI: c_uint = 0xF0		/* NetBEUI. 			*/;
pub const LLC_SAP_LANMGR: c_uint = 0xF4		/* LanManager. 			*/;
pub const LLC_SAP_IMPL: c_uint = 0xF8		/* IMPL				*/;
pub const LLC_SAP_DISC: c_uint = 0xFC		/* Discovery			*/;
pub const LLC_SAP_OSI: c_uint = 0xFE		/* OSI Network Layers. 		*/;
pub const LLC_SAP_LAR: c_uint = 0xDC		/* LAN Address Resolution 	*/;
pub const LLC_SAP_RM: c_uint = 0xD4		/* Resource Management 		*/;
pub const LLC_SAP_GLOBAL: c_uint = 0xFF		/* Global SAP. 			*/;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_pktinfo {
    pub lpi_ifindex: c_int,
    pub lpi_sap: c_uchar,
    pub lpi_mac: [c_uchar; IFHWADDRLEN],
}
