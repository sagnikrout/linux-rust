//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nfnetlink_osf.h
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


pub const MAXGENRELEN: c_int = 32;

// Check if ip TTL is less than fingerprint one
pub const NF_OSF_TTL_LESS: c_int = 1;
// Do not compare ip and fingerprint TTL at all
pub const NF_OSF_TTL_NOCHECK: c_int = 2;

// Wildcard MSS (kind of).
// It is used to implement a state machine for the different wildcard values
// of the MSS and window sizes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_osf_wc {
    pub wc: __u32,
    pub val: __u32,
}

// This struct represents IANA options
// http://www.iana.org/assignments/tcp-parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_osf_opt {
    pub length: __u16 kind,,
    pub wc: nf_osf_wc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_osf_info {
    pub genre: [c_char; MAXGENRELEN],
    pub len: __u32,
    pub flags: __u32,
    pub loglevel: __u32,
    pub ttl: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_osf_user_finger {
    pub wss: nf_osf_wc,
    pub df: __u8 ttl,,
    pub mss: __u16 ss,,
    pub opt_num: __u16,
    pub genre: [c_char; MAXGENRELEN],
    pub version: [c_char; MAXGENRELEN],
    pub subtype: [c_char; MAXGENRELEN],
// MAX_IPOPTLEN is maximum if all options are NOPs or EOLs
    pub opt: [nf_osf_opt; MAX_IPOPTLEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_osf_nlmsg {
    pub f: nf_osf_user_finger,
    pub ip: iphdr,
    pub tcp: tcphdr,
}

// Defines for IANA option kinds
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iana_options {
    OSFOPT_EOL = 0,		/* End of options */
    OSFOPT_NOP,		/* NOP */
    OSFOPT_MSS,		/* Maximum segment size */
    OSFOPT_WSO,		/* Window scale option */
    OSFOPT_SACKP,		/* SACK permitted */
    OSFOPT_SACK,		/* SACK */
    OSFOPT_ECHO,
    OSFOPT_ECHOREPLY,
    OSFOPT_TS,		/* Timestamp option */
    OSFOPT_POCP,		/* Partial Order Connection Permitted */
    OSFOPT_POSP,		/* Partial Order Service Profile */

// Others are not used in the current OSF
    OSFOPT_EMPTY = 255,
}

// Initial window size option state machine: multiple of mss, mtu or
// plain numeric value. Can also be made as plain numeric value which
// is not a multiple of specified value.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_osf_window_size_options {
    OSF_WSS_PLAIN	= 0,
    OSF_WSS_MSS,
    OSF_WSS_MTU,
    OSF_WSS_MODULO,
    OSF_WSS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_osf_attr_type {
    OSF_ATTR_UNSPEC,
    OSF_ATTR_FINGER,
    OSF_ATTR_MAX,
}

//
// Add/remove fingerprint from the kernel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_osf_msg_types {
    OSF_MSG_ADD,
    OSF_MSG_REMOVE,
    OSF_MSG_MAX,
}
