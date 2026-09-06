//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inet_timewait_sock.h
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for a generic INET TIMEWAIT sock
//
// From code originally in net/tcp.h
//

//
// This is a TIME_WAIT sock. It works around the memory consumption
// problems of sockets in such a state on heavily loaded servers, but
// without violating the protocol specification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_timewait_sock {
//
// Now struct sock also uses sock_common, so please just
// don't add nothing before this first member (__tw_common) --acme
//
    pub __tw_common: sock_common,

    pub tw_mark: __u32,
    pub tw_substate: c_uchar,
    pub tw_rcv_wscale: c_uchar,
// Socket demultiplex comparisons on incoming packets.
// these three are in inet_sock
    pub tw_sport: __be16,
// And these are ours.
    pub 8: tw_tos :,
    pub tw_txhash: u32,
    pub tw_priority: u32,
//
// @tw_reuse_stamp: Time of entry into %TCP_TIME_WAIT state in msec.
//
    pub tw_entry_stamp: u32,
    pub tw_timer: timer_list,
    pub tw_tb: *mut inet_bind_bucket,
    pub tw_tb2: *mut inet_bind2_bucket,

    pub psp_assoc: *mut psp_assoc __rcu,

    pub skb): *mut sk_buff,

}

extern "C" {
    pub fn inet_twsk_free(tw: *mut inet_timewait_sock);
}
extern "C" {
    pub fn inet_twsk_put(tw: *mut inet_timewait_sock);
}
extern "C" {
    pub fn inet_twsk_deschedule_put(tw: *mut inet_timewait_sock);
}
extern "C" {
    pub fn inet_twsk_purge(hashinfo: *mut inet_hashinfo);
}
extern "C" {
    pub fn read_pnet(_arg: &twsk->tw_net) -> return;
}
