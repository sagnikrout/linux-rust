//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ax25.h
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
// These are the public elements of the Linux kernel AX.25 code. A similar
// file netrom.h exists for the NET/ROM protocol.
//

pub const AX25_MTU: c_int = 256;
pub const AX25_MAX_DIGIS: c_int = 8;
pub const AX25_WINDOW: c_int = 1;
pub const AX25_T1: c_int = 2;
pub const AX25_N2: c_int = 3;
pub const AX25_T3: c_int = 4;
pub const AX25_T2: c_int = 5;
pub const AX25_BACKOFF: c_int = 6;
pub const AX25_EXTSEQ: c_int = 7;
pub const AX25_PIDINCL: c_int = 8;
pub const AX25_IDLE: c_int = 9;
pub const AX25_PACLEN: c_int = 10;
pub const AX25_IAMDIGI: c_int = 12;
pub const AX25_KILL: c_int = 99;

pub const AX25_SET_RT_IPMODE: c_int = 2;
pub const AX25_NOUID_DEFAULT: c_int = 0;
pub const AX25_NOUID_BLOCK: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_ax25 {
    pub sax25_family: __kernel_sa_family_t,
    pub sax25_call: ax25_address,
    pub sax25_ndigis: c_int,
// Digipeater ax25_address sets follow
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct full_sockaddr_ax25 {
    pub fsa_ax25: sockaddr_ax25,
    pub fsa_digipeater: [ax25_address; AX25_MAX_DIGIS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax25_routes_struct {
    pub port_addr: ax25_address,
    pub dest_addr: ax25_address,
    pub digi_count: c_uchar,
    pub digi_addr: [ax25_address; AX25_MAX_DIGIS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax25_route_opt_struct {
    pub port_addr: ax25_address,
    pub dest_addr: ax25_address,
    pub cmd: c_int,
    pub arg: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax25_ctl_struct {
    pub port_addr: ax25_address,
    pub source_addr: ax25_address,
    pub dest_addr: ax25_address,
    pub cmd: c_uint,
    pub arg: c_ulong,
    pub digi_count: c_uchar,
    pub digi_addr: [ax25_address; AX25_MAX_DIGIS],
}

// this will go away. Please do not export to user land
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax25_info_struct_deprecated {
    pub n2count: unsigned int n2,,
    pub t1timer: unsigned int t1,,
    pub t2timer: unsigned int t2,,
    pub t3timer: unsigned int t3,,
    pub idletimer: unsigned int idle,,
    pub state: c_uint,
    pub snd_q: unsigned int rcv_q,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax25_info_struct {
    pub n2count: unsigned int n2,,
    pub t1timer: unsigned int t1,,
    pub t2timer: unsigned int t2,,
    pub t3timer: unsigned int t3,,
    pub idletimer: unsigned int idle,,
    pub state: c_uint,
    pub snd_q: unsigned int rcv_q,,
    pub vs_max: unsigned int vs, vr, va,,
    pub paclen: c_uint,
    pub window: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax25_fwd_struct {
    pub port_from: ax25_address,
    pub port_to: ax25_address,
}
