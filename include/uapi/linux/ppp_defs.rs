//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ppp_defs.h
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
// ppp_defs.h - PPP definitions.
//
// Copyright 1994-2000 Paul Mackerras.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// version 2 as published by the Free Software Foundation.
//

//
// The basic PPP frame.
//

//
// Significant octet values.
//
pub const PPP_ALLSTATIONS: c_uint = 0xff	/* All-Stations broadcast address */;
pub const PPP_UI: c_uint = 0x03	/* Unnumbered Information */;
pub const PPP_FLAG: c_uint = 0x7e	/* Flag Sequence */;
pub const PPP_ESCAPE: c_uint = 0x7d	/* Asynchronous Control Escape */;
pub const PPP_TRANS: c_uint = 0x20	/* Asynchronous transparency modifier */;
//
// Protocol field values.
//
pub const PPP_IP: c_uint = 0x21	/* Internet Protocol */;
pub const PPP_AT: c_uint = 0x29	/* AppleTalk Protocol */;
pub const PPP_IPX: c_uint = 0x2b	/* IPX protocol */;
pub const PPP_VJC_COMP: c_uint = 0x2d	/* VJ compressed TCP */;
pub const PPP_VJC_UNCOMP: c_uint = 0x2f	/* VJ uncompressed TCP */;
pub const PPP_MP: c_uint = 0x3d	/* Multilink protocol */;
pub const PPP_IPV6: c_uint = 0x57	/* Internet Protocol Version 6 */;
pub const PPP_COMPFRAG: c_uint = 0xfb	/* fragment compressed below bundle */;
pub const PPP_COMP: c_uint = 0xfd	/* compressed packet */;
pub const PPP_MPLS_UC: c_uint = 0x0281	/* Multi Protocol Label Switching - Unicast */;
pub const PPP_MPLS_MC: c_uint = 0x0283	/* Multi Protocol Label Switching - Multicast */;
pub const PPP_IPCP: c_uint = 0x8021	/* IP Control Protocol */;
pub const PPP_ATCP: c_uint = 0x8029	/* AppleTalk Control Protocol */;
pub const PPP_IPXCP: c_uint = 0x802b	/* IPX Control Protocol */;
pub const PPP_IPV6CP: c_uint = 0x8057	/* IPv6 Control Protocol */;
pub const PPP_CCPFRAG: c_uint = 0x80fb	/* CCP at link level (below MP bundle) */;
pub const PPP_CCP: c_uint = 0x80fd	/* Compression Control Protocol */;
pub const PPP_MPLSCP: c_uint = 0x80fd	/* MPLS Control Protocol */;
pub const PPP_LCP: c_uint = 0xc021	/* Link Control Protocol */;
pub const PPP_PAP: c_uint = 0xc023	/* Password Authentication Protocol */;
pub const PPP_LQR: c_uint = 0xc025	/* Link Quality Report protocol */;
pub const PPP_CHAP: c_uint = 0xc223	/* Cryptographic Handshake Auth. Protocol */;
pub const PPP_CBCP: c_uint = 0xc029	/* Callback Control Protocol */;
//
// Values for FCS calculations.
//
pub const PPP_INITFCS: c_uint = 0xffff	/* Initial FCS value */;
pub const PPP_GOODFCS: c_uint = 0xf0b8	/* Good final FCS value */;
//
// Extended asyncmap - allows any character to be escaped.
//
// What to do with network protocol (NP) packets.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NPmode {
    NPMODE_PASS,		/* pass the packet through */
    NPMODE_DROP,		/* silently drop the packet */
    NPMODE_ERROR,		/* return an error */
    NPMODE_QUEUE		/* save it up for later. */
}

//
// Statistics for LQRP and pppstats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppstat {
    pub /: *mut *mut __u32 ppp_discards; / # frames discarded,
    pub /: *mut *mut __u32 ppp_ibytes; / bytes received,
    pub /: *mut *mut __u32 ppp_ioctects; / bytes received not in error,
    pub /: *mut *mut __u32 ppp_ipackets; / packets received,
    pub /: *mut *mut __u32 ppp_ierrors; / receive errors,
    pub /: *mut *mut __u32 ppp_ilqrs; / # LQR frames received,
    pub /: *mut *mut __u32 ppp_obytes; / raw bytes sent,
    pub /: *mut *mut __u32 ppp_ooctects; / frame bytes sent,
    pub /: *mut *mut __u32 ppp_opackets; / packets sent,
    pub /: *mut *mut __u32 ppp_oerrors; / transmit errors,
    pub /: *mut *mut __u32 ppp_olqrs; / # LQR frames sent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vjstat {
    pub /: *mut *mut __u32 vjs_packets; / outbound packets,
    pub /: *mut *mut __u32 vjs_compressed; / outbound compressed packets,
    pub /: *mut *mut __u32 vjs_searches; / searches for connection state,
    pub /: *mut *mut __u32 vjs_misses; / times couldn't find conn. state,
    pub /: *mut *mut __u32 vjs_uncompressedin; / inbound uncompressed packets,
    pub /: *mut *mut __u32 vjs_compressedin; / inbound compressed packets,
    pub /: *mut *mut __u32 vjs_errorin; / inbound unknown type packets,
    pub /: *mut *mut __u32 vjs_tossed; / inbound packets tossed because of error,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compstat {
    pub /: *mut *mut __u32 unc_bytes; / total uncompressed bytes,
    pub /: *mut *mut __u32 unc_packets; / total uncompressed packets,
    pub /: *mut *mut __u32 comp_bytes; / compressed bytes,
    pub /: *mut *mut __u32 comp_packets; / compressed packets,
    pub /: *mut *mut __u32 inc_bytes; / incompressible bytes,
    pub /: *mut *mut __u32 inc_packets; / incompressible packets,
// the compression ratio is defined as in_count / bytes_out
    pub /: *mut *mut __u32 in_count; / Bytes received,
    pub /: *mut *mut __u32 bytes_out; / Bytes transmitted,
    pub /: *mut *mut double ratio; / not computed in kernel.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppp_stats {
    pub /: *mut *mut pppstat p; / basic PPP statistics,
    pub /: *mut *mut vjstat vj; / VJ header compression statistics,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppp_comp_stats {
    pub /: *mut *mut compstat c; / packet compression statistics,
    pub /: *mut *mut compstat d; / packet decompression statistics,
}

//
// The following structure records the time in seconds since
// the last NP packet was sent or received.
//
// Linux implements both 32-bit and 64-bit time_t versions
// for compatibility with user space that defines ppp_idle
// based on the libc time_t.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppp_idle {
    pub /: *mut *mut __kernel_old_time_t xmit_idle; / time since last NP packet sent,
    pub /: *mut *mut __kernel_old_time_t recv_idle; / time since last NP packet received,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppp_idle32 {
    pub /: *mut *mut __s32 xmit_idle; / time since last NP packet sent,
    pub /: *mut *mut __s32 recv_idle; / time since last NP packet received,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppp_idle64 {
    pub /: *mut *mut __s64 xmit_idle; / time since last NP packet sent,
    pub /: *mut *mut __s64 recv_idle; / time since last NP packet received,
}
