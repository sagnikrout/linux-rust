//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/nf_conntrack_sip.h
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

pub const SIP_PORT: c_int = 5060;
pub const SIP_TIMEOUT: c_int = 3600;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_sip_master {
    pub register_cseq: c_uint,
    pub invite_cseq: c_uint,
    pub forced_dport: __be16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sip_expectation_classes {
    SIP_EXPECT_SIGNALLING,
    SIP_EXPECT_AUDIO,
    SIP_EXPECT_VIDEO,
    SIP_EXPECT_IMAGE,
    __SIP_EXPECT_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdp_media_type {
    pub name: *const c_char,
    pub len: c_uint,
    pub class: sip_expectation_classes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sip_handler {
    pub method: *const c_char,
    pub len: c_uint,
    pub cseq): c_uint,
    pub code): unsigned int cseq, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sip_header {
    pub name: *const c_char,
    pub cname: *const c_char,
    pub search: *const c_char,
    pub len: c_uint,
    pub clen: c_uint,
    pub slen: c_uint,
    pub shift): *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sip_header_types {
    SIP_HDR_CSEQ,
    SIP_HDR_FROM,
    SIP_HDR_TO,
    SIP_HDR_CONTACT,
    SIP_HDR_VIA_UDP,
    SIP_HDR_VIA_TCP,
    SIP_HDR_EXPIRES,
    SIP_HDR_CONTENT_LENGTH,
    SIP_HDR_CALL_ID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdp_header_types {
    SDP_HDR_UNSPEC,
    SDP_HDR_VERSION,
    SDP_HDR_OWNER,
    SDP_HDR_CONNECTION,
    SDP_HDR_MEDIA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_sip_hooks {
    pub datalen): *mut c_uint,
    pub off): unsigned int protoff, s32,
    pub matchlen): c_uint,
    pub addr): *const nf_inet_addr,
    pub port): u_int16_t,
    pub addr): *const nf_inet_addr,
    pub rtp_addr): *mut nf_inet_addr,
}
