//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/atmbr2684.h
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
// Type of media we're bridging (ethernet, token ring, etc)  Currently only
// ethernet is supported
//

// used only at device creation:

//
// Is there FCS inbound on this VC?  This currently isn't supported.
//

//
// Is there FCS outbound on this VC?  This currently isn't supported.
//

//
// Does this VC include LLC encapsulation?
//

//
// Is this VC bridged or routed?
//

//
// This is for the ATM_NEWBACKENDIF call - these are like socket families:
// the first element of the structure is the backend number and the rest
// is per-backend specific
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_newif_br2684 {
    pub /: *mut *mut atm_backend_t backend_num; / ATM_BACKEND_BR2684,
    pub /: *mut *mut *mut int media; / BR2684_MEDIA_, flags in upper bits,
    pub ifname: [c_char; IFNAMSIZ],
    pub mtu: c_int,
}

//
// This structure is used to specify a br2684 interface - either by a
// positive integer (returned by ATM_NEWBACKENDIF) or the interfaces name
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br2684_if_spec {
    pub /: *mut *mut *mut int method; / BR2684_FIND_,
    pub ifname: [c_char; IFNAMSIZ],
    pub devnum: c_int,
    pub spec: },
}

//
// This is for the ATM_SETBACKEND call - these are like socket families:
// the first element of the structure is the backend number and the rest
// is per-backend specific
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atm_backend_br2684 {
    pub /: *mut *mut atm_backend_t backend_num; / ATM_BACKEND_BR2684,
    pub ifspec: br2684_if_spec,
    pub /: *mut *mut *mut int fcs_in; / BR2684_FCSIN_,
    pub /: *mut *mut *mut int fcs_out; / BR2684_FCSOUT_,
    pub /: *mut *mut int fcs_auto; / 1: fcs_{in,out} disabled if no FCS rx'ed,
    pub /: *mut *mut *mut int encaps; / BR2684_ENCAPS_,
    pub /: *mut *mut int has_vpiid; / 1: use vpn_id - Unsupported,
    pub vpn_id: [__u8; 7],
    pub /: *mut *mut int send_padding; / unsupported,
    pub /: *mut *mut int min_size; / we will pad smaller packets than this,
}

//
// The BR2684_SETFILT ioctl is an experimental mechanism for folks
// terminating a large number of IP-only vcc's.  When netfilter allows
// efficient per-if in/out filters, this support will be removed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br2684_filter {
    pub /: *mut *mut __be32 prefix; / network byte order,
    pub /: *mut *mut __be32 netmask; / 0 = disable filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br2684_filter_set {
    pub ifspec: br2684_if_spec,
    pub filter: br2684_filter,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br2684_payload {
    p_routed = BR2684_PAYLOAD_ROUTED,
    p_bridged = BR2684_PAYLOAD_BRIDGED,
}

