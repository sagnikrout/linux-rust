//! Automatically rewritten from C Header to Rust Module
//! Source: net/ipv4/tcp_vegas.h
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


// SPDX-License-Identifier: GPL-2.0
//
// TCP Vegas congestion control interface
//
pub const __TCP_VEGAS_H: c_int = 1;
// Vegas variables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vegas {
    pub /: *mut *mut u32 beg_snd_nxt; / right edge during last RTT,
    pub /: *mut *mut u32 beg_snd_una; / left edge during last RTT,
    pub /: *mut *mut u32 beg_snd_cwnd; / saves the size of the cwnd,
    pub /: *mut *mut u8 doing_vegas_now;/ if true, do vegas for this RTT,
    pub /: *mut *mut u16 cntRTT; / # of RTTs measured within last RTT,
    pub /: *mut *mut u32 minRTT; / min of RTTs measured within last RTT (in usec),
    pub /: *mut *mut u32 baseRTT; / the min of all Vegas RTT measurements seen (in usec),
}

extern "C" {
    pub fn tcp_vegas_init(sk: *mut sock);
}
extern "C" {
    pub fn tcp_vegas_state(sk: *mut sock, ca_state: u8);
}
extern "C" {
    pub fn tcp_vegas_pkts_acked(sk: *mut sock, sample: *const ack_sample);
}
extern "C" {
    pub fn tcp_vegas_cwnd_event(sk: *mut sock, event: tcp_ca_event);
}
extern "C" {
    pub fn tcp_vegas_cwnd_event_tx_start(sk: *mut sock);
}
