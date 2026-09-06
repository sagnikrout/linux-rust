//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/slhc_vj.h
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
// Definitions for tcp compression routines.
//
// $Header: slcompress.h,v 1.10 89/12/31 08:53:02 van Exp $
//
// Copyright (c) 1989 Regents of the University of California.
// All rights reserved.
//
// Redistribution and use in source and binary forms are permitted
// provided that the above copyright notice and this paragraph are
// duplicated in all such forms and that any documentation,
// advertising materials, and other materials related to such
// distribution and use acknowledge that the software was developed
// by the University of California, Berkeley.  The name of the
// University may not be used to endorse or promote products derived
// from this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE.
//
// Van Jacobson (van@helios.ee.lbl.gov), Dec 31, 1989:
// - Initial distribution.
//
// modified for KA9Q Internet Software Package by
// Katie Stevens (dkstevens@ucdavis.edu)
// University of California, Davis
// Computing Services
// - 01-31-90	initial adaptation
//
// - Feb 1991	Bill_Simpson@um.cc.umich.edu
// variable number of conversation slots
// allow zero or one slots
// separate routines
// status display
//
// Compressed packet format:
//
// The first octet contains the packet type (top 3 bits), TCP
// 'push' bit, and flags that indicate which of the 4 TCP sequence
// numbers have changed (bottom 5 bits).  The next octet is a
// conversation number that associates a saved IP/TCP header with
// the compressed packet.  The next two octets are the TCP checksum
// from the original datagram.  The next 0 to 15 octets are
// sequence number changes, one change per bit set in the header
// (there may be no changes and there are two special cases where
// the receiver implicitly knows what changed -- see below).
//
// There are 5 numbers which can change (they are always inserted
// in the following order): TCP urgent pointer, window,
// acknowledgment, sequence number and IP ID.  (The urgent pointer
// is different from the others in that its value is sent, not the
// change in value.)  Since typical use of SLIP links is biased
// toward small packets (see comments on MTU/MSS below), changes
// use a variable length coding with one octet for numbers in the
// range 1 - 255 and 3 octets (0, MSB, LSB) for numbers in the
// range 256 - 65535 or 0.  (If the change in sequence number or
// ack is more than 65535, an uncompressed packet is sent.)
//
// Packet types (must not conflict with IP protocol version)
//
// The top nibble of the first octet is the packet type.  There are
// three possible types: IP (not proto TCP or tcp with one of the
// control flags set); uncompressed TCP (a normal IP/TCP packet but
// with the 8-bit protocol field replaced by an 8-bit connection id --
// this type of packet syncs the sender & receiver); and compressed
// TCP (described above).
//
// LSB of 4-bit field is TCP "PUSH" bit (a worthless anachronism) and
// is logically part of the 4-bit "changes" field that follows.  Top
// three bits are actual packet type.  For backward compatibility
// and in the interest of conserving bits, numbers are chosen so the
// IP protocol version number (4) which normally appears in this nibble
// means "IP packet".
//

// SLIP compression masks for len/vers byte
pub const SL_TYPE_IP: c_uint = 0x40;
pub const SL_TYPE_UNCOMPRESSED_TCP: c_uint = 0x70;
pub const SL_TYPE_COMPRESSED_TCP: c_uint = 0x80;
pub const SL_TYPE_ERROR: c_uint = 0x00;
// Bits in first octet of compressed packet
pub const NEW_C: c_uint = 0x40	/* flag bits for what changed in a packet */;
pub const NEW_I: c_uint = 0x20;
pub const NEW_S: c_uint = 0x08;
pub const NEW_A: c_uint = 0x04;
pub const NEW_W: c_uint = 0x02;
pub const NEW_U: c_uint = 0x01;
// reserved, special-case values of above

pub const TCP_PUSH_BIT: c_uint = 0x10;
//
// data type and sizes conversion assumptions:
//
// VJ code		KA9Q style	generic
// u_char		byte_t		unsigned char	 8 bits
// u_short		int16		unsigned short	16 bits
// u_int		int16		unsigned short	16 bits
// u_long		unsigned long	unsigned long	32 bits
// int		int32		long		32 bits
//
pub type byte_t = __u8;
pub type int32 = __u32;
//
// "state" data for each active tcp conversation on the wire.  This is
// basically a copy of the entire IP/TCP header from the last packet
// we saw from the conversation together with a small identifier
// the transmit & receive ends of the line use to locate saved header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstate {
    pub /: *mut *mut byte_t cs_this; / connection id number (xmit),
    pub /: *mut *mut bool initialized; / true if initialized,
    pub /: *mut *mut *mut cstate next; / next in ring (xmit),
    pub /: *mut *mut iphdr cs_ip; / ip/tcp hdr from most recent packet,
    pub cs_tcp: tcphdr,
    pub cs_ipopt: [c_uchar; 64],
    pub cs_tcpopt: [c_uchar; 64],
    pub cs_hsize: c_int,
}

//
// all the state data for one serial line (we need one of these per line).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slcompress {
    pub (array)*/: *mut *mut *mut cstate tstate; / transmit connection states,
    pub (array)*/: *mut *mut *mut cstate rstate; / receive connection states,
    pub (0-l)*/: *mut *mut byte_t tslot_limit; / highest transmit slot id,
    pub (0-l)*/: *mut *mut byte_t rslot_limit; / highest receive slot id,
    pub /: *mut *mut byte_t xmit_oldest; / oldest xmit in ring,
    pub /: *mut *mut byte_t xmit_current; / most recent xmit id,
    pub /: *mut *mut byte_t recv_current; / most recent rcvd id,
    pub flags: byte_t,
pub const SLF_TOSS: c_uint = 0x01	/* tossing rcvd frames until id received */;
    pub /: *mut *mut int32 sls_o_nontcp; / outbound non-TCP packets,
    pub /: *mut *mut int32 sls_o_tcp; / outbound TCP packets,
    pub /: *mut *mut int32 sls_o_uncompressed; / outbound uncompressed packets,
    pub /: *mut *mut int32 sls_o_compressed; / outbound compressed packets,
    pub /: *mut *mut int32 sls_o_searches; / searches for connection state,
    pub /: *mut *mut int32 sls_o_misses; / times couldn't find conn. state,
    pub /: *mut *mut int32 sls_i_uncompressed; / inbound uncompressed packets,
    pub /: *mut *mut int32 sls_i_compressed; / inbound compressed packets,
    pub /: *mut *mut int32 sls_i_error; / inbound error packets,
    pub /: *mut *mut int32 sls_i_tossed; / inbound packets tossed because of error,
    pub sls_i_runt: int32,
    pub sls_i_badcheck: int32,
}

// In slhc.c:
extern "C" {
    pub fn slhc_free(comp: *mut slcompress);
}
extern "C" {
    pub fn slhc_uncompress(comp: *mut slcompress, icp: *mut c_uchar, isize: c_int) -> c_int;
}
extern "C" {
    pub fn slhc_remember(comp: *mut slcompress, icp: *mut c_uchar, isize: c_int) -> c_int;
}
extern "C" {
    pub fn slhc_toss(comp: *mut slcompress) -> c_int;
}
