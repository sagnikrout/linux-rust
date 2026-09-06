//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_arp.h
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
// Global definitions for the ARP (RFC 826) protocol.
//
// Version:	@(#)if_arp.h	1.0.1	04/16/93
//
// Authors:	Original taken from Berkeley UNIX 4.3, (c) UCB 1986-1988
// Portions taken from the KA9Q/NOS (v2.00m PA0GRI) source.
// Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
// Florian La Roche,
// Jonathan Layes <layes@loran.com>
// Arnaldo Carvalho de Melo <acme@conectiva.com.br> ARPHRD_HWX25
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// ARP protocol HARDWARE identifiers.

// Dummy types for non ARP hardware
pub const ARPHRD_SLIP: c_int = 256;
pub const ARPHRD_CSLIP: c_int = 257;
pub const ARPHRD_SLIP6: c_int = 258;
pub const ARPHRD_CSLIP6: c_int = 259;

pub const ARPHRD_ADAPT: c_int = 264;
pub const ARPHRD_ROSE: c_int = 270;

pub const ARPHRD_MCTP: c_int = 290;
pub const ARPHRD_PPP: c_int = 512;

// ARP works differently on different FC media .. so

// 787->799 reserved for fibrechannel media types

pub const ARPHRD_IEEE802154: c_int = 804;

pub const ARPHRD_VOID: c_uint = 0xFFFF	/* Void type, nothing is known */;
pub const ARPHRD_NONE: c_uint = 0xFFFE	/* zero header length */;
// ARP protocol opcodes.

// ARP ioctl request.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpreq {
    pub /: *mut *mut sockaddr arp_pa; / protocol address,
    pub /: *mut *mut sockaddr arp_ha; / hardware address,
    pub /: *mut *mut int arp_flags; / flags,
    pub /: *mut *mut sockaddr arp_netmask; / netmask (only for proxy arps),
    pub arp_dev: [c_char; IFNAMSIZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpreq_old {
    pub /: *mut *mut sockaddr arp_pa; / protocol address,
    pub /: *mut *mut sockaddr arp_ha; / hardware address,
    pub /: *mut *mut int arp_flags; / flags,
    pub /: *mut *mut sockaddr arp_netmask; / netmask (only for proxy arps),
}

// ARP Flag values.
pub const ATF_COM: c_uint = 0x02		/* completed entry (ha valid)	*/;
pub const ATF_PERM: c_uint = 0x04		/* permanent entry		*/;
pub const ATF_PUBL: c_uint = 0x08		/* publish entry		*/;
pub const ATF_USETRAILERS: c_uint = 0x10		/* has requested trailers	*/;
pub const ATF_NETMASK: c_uint = 0x20            /* want to use a netmask (only;
pub const ATF_DONTPUB: c_uint = 0x40		/* don't answer this addresses	*/;
//
// This structure defines an ethernet arp header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arphdr {
    pub /: *mut *mut __be16 ar_hrd; / format of hardware address,
    pub /: *mut *mut __be16 ar_pro; / format of protocol address,
    pub /: *mut *mut unsigned char ar_hln; / length of hardware address,
    pub /: *mut *mut unsigned char ar_pln; / length of protocol address,
    pub /: *mut *mut __be16 ar_op; / ARP opcode (command),

//
// Ethernet looks like this : This bit is variable sized however...
//
    pub /: *mut *mut unsigned char ar_sha[ETH_ALEN]; / sender hardware address,
    pub /: *mut *mut unsigned char ar_sip[4]; / sender IP address,
    pub /: *mut *mut unsigned char ar_tha[ETH_ALEN]; / target hardware address,
    pub /: *mut *mut unsigned char ar_tip[4]; / target IP address,

}
