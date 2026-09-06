//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/inet.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Swansea University Computer Society NET3
//
// This work is derived from NET2Debugged, which is in turn derived
// from NET2D which was written by:
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// This work was derived from Ross Biro's inspirational work
// for the LINUX operating system.  His version numbers were:
//
// $Id: Space.c,v     0.8.4.5  1992/12/12 19:25:04 bir7 Exp $
// $Id: arp.c,v       0.8.4.6  1993/01/28 22:30:00 bir7 Exp $
// $Id: arp.h,v       0.8.4.6  1993/01/28 22:30:00 bir7 Exp $
// $Id: dev.c,v       0.8.4.13 1993/01/23 18:00:11 bir7 Exp $
// $Id: dev.h,v       0.8.4.7  1993/01/23 18:00:11 bir7 Exp $
// $Id: eth.c,v       0.8.4.4  1993/01/22 23:21:38 bir7 Exp $
// $Id: eth.h,v       0.8.4.1  1992/11/10 00:17:18 bir7 Exp $
// $Id: icmp.c,v      0.8.4.9  1993/01/23 18:00:11 bir7 Exp $
// $Id: icmp.h,v      0.8.4.2  1992/11/15 14:55:30 bir7 Exp $
// $Id: ip.c,v        0.8.4.8  1992/12/12 19:25:04 bir7 Exp $
// $Id: ip.h,v        0.8.4.2  1993/01/23 18:00:11 bir7 Exp $
// $Id: loopback.c,v  0.8.4.8  1993/01/23 18:00:11 bir7 Exp $
// $Id: packet.c,v    0.8.4.7  1993/01/26 22:04:00 bir7 Exp $
// $Id: protocols.c,v 0.8.4.3  1992/11/15 14:55:30 bir7 Exp $
// $Id: raw.c,v       0.8.4.12 1993/01/26 22:04:00 bir7 Exp $
// $Id: sock.c,v      0.8.4.6  1993/01/28 22:30:00 bir7 Exp $
// $Id: sock.h,v      0.8.4.7  1993/01/26 22:04:00 bir7 Exp $
// $Id: tcp.c,v       0.8.4.16 1993/01/26 22:04:00 bir7 Exp $
// $Id: tcp.h,v       0.8.4.7  1993/01/22 22:58:08 bir7 Exp $
// $Id: timer.c,v     0.8.4.8  1993/01/23 18:00:11 bir7 Exp $
// $Id: timer.h,v     0.8.4.2  1993/01/23 18:00:11 bir7 Exp $
// $Id: udp.c,v       0.8.4.12 1993/01/26 22:04:00 bir7 Exp $
// $Id: udp.h,v       0.8.4.1  1992/11/10 00:17:18 bir7 Exp $
// $Id: we.c,v        0.8.4.10 1993/01/23 18:00:11 bir7 Exp $
// $Id: wereg.h,v     0.8.4.1  1992/11/10 00:17:18 bir7 Exp $
//

//
// These mimic similar macros defined in user-space for inet_ntop(3).
// See /usr/include/netinet/in.h .
//

extern "C" {
    pub fn in_aton(str: *const c_char) -> __be32;
}
extern "C" {
    pub fn in4_pton(src: *const c_char, srclen: c_int, dst: *mut u8, delim: c_int, end: *const c_char) -> c_int;
}
extern "C" {
    pub fn in6_pton(src: *const c_char, srclen: c_int, dst: *mut u8, delim: c_int, end: *const c_char) -> c_int;
}
extern "C" {
    pub fn inet_addr_is_any(addr: *mut sockaddr_storage) -> bool;
}
