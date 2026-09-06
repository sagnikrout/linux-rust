//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nfnetlink_cttimeout.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctnl_timeout_msg_types {
    IPCTNL_MSG_TIMEOUT_NEW,
    IPCTNL_MSG_TIMEOUT_GET,
    IPCTNL_MSG_TIMEOUT_DELETE,
    IPCTNL_MSG_TIMEOUT_DEFAULT_SET,
    IPCTNL_MSG_TIMEOUT_DEFAULT_GET,

    IPCTNL_MSG_TIMEOUT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout {
    CTA_TIMEOUT_UNSPEC,
    CTA_TIMEOUT_NAME,
    CTA_TIMEOUT_L3PROTO,
    CTA_TIMEOUT_L4PROTO,
    CTA_TIMEOUT_DATA,
    CTA_TIMEOUT_USE,
    __CTA_TIMEOUT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_generic {
    CTA_TIMEOUT_GENERIC_UNSPEC,
    CTA_TIMEOUT_GENERIC_TIMEOUT,
    __CTA_TIMEOUT_GENERIC_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_tcp {
    CTA_TIMEOUT_TCP_UNSPEC,
    CTA_TIMEOUT_TCP_SYN_SENT,
    CTA_TIMEOUT_TCP_SYN_RECV,
    CTA_TIMEOUT_TCP_ESTABLISHED,
    CTA_TIMEOUT_TCP_FIN_WAIT,
    CTA_TIMEOUT_TCP_CLOSE_WAIT,
    CTA_TIMEOUT_TCP_LAST_ACK,
    CTA_TIMEOUT_TCP_TIME_WAIT,
    CTA_TIMEOUT_TCP_CLOSE,
    CTA_TIMEOUT_TCP_SYN_SENT2,
    CTA_TIMEOUT_TCP_RETRANS,
    CTA_TIMEOUT_TCP_UNACK,
    __CTA_TIMEOUT_TCP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_udp {
    CTA_TIMEOUT_UDP_UNSPEC,
    CTA_TIMEOUT_UDP_UNREPLIED,
    CTA_TIMEOUT_UDP_REPLIED,
    __CTA_TIMEOUT_UDP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_udplite {
    CTA_TIMEOUT_UDPLITE_UNSPEC,
    CTA_TIMEOUT_UDPLITE_UNREPLIED,
    CTA_TIMEOUT_UDPLITE_REPLIED,
    __CTA_TIMEOUT_UDPLITE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_icmp {
    CTA_TIMEOUT_ICMP_UNSPEC,
    CTA_TIMEOUT_ICMP_TIMEOUT,
    __CTA_TIMEOUT_ICMP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_dccp {
    CTA_TIMEOUT_DCCP_UNSPEC,
    CTA_TIMEOUT_DCCP_REQUEST,
    CTA_TIMEOUT_DCCP_RESPOND,
    CTA_TIMEOUT_DCCP_PARTOPEN,
    CTA_TIMEOUT_DCCP_OPEN,
    CTA_TIMEOUT_DCCP_CLOSEREQ,
    CTA_TIMEOUT_DCCP_CLOSING,
    CTA_TIMEOUT_DCCP_TIMEWAIT,
    __CTA_TIMEOUT_DCCP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_sctp {
    CTA_TIMEOUT_SCTP_UNSPEC,
    CTA_TIMEOUT_SCTP_CLOSED,
    CTA_TIMEOUT_SCTP_COOKIE_WAIT,
    CTA_TIMEOUT_SCTP_COOKIE_ECHOED,
    CTA_TIMEOUT_SCTP_ESTABLISHED,
    CTA_TIMEOUT_SCTP_SHUTDOWN_SENT,
    CTA_TIMEOUT_SCTP_SHUTDOWN_RECD,
    CTA_TIMEOUT_SCTP_SHUTDOWN_ACK_SENT,
    CTA_TIMEOUT_SCTP_HEARTBEAT_SENT,
    CTA_TIMEOUT_SCTP_HEARTBEAT_ACKED, /* no longer used */
    __CTA_TIMEOUT_SCTP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_icmpv6 {
    CTA_TIMEOUT_ICMPV6_UNSPEC,
    CTA_TIMEOUT_ICMPV6_TIMEOUT,
    __CTA_TIMEOUT_ICMPV6_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctattr_timeout_gre {
    CTA_TIMEOUT_GRE_UNSPEC,
    CTA_TIMEOUT_GRE_UNREPLIED,
    CTA_TIMEOUT_GRE_REPLIED,
    __CTA_TIMEOUT_GRE_MAX
}

pub const CTNL_TIMEOUT_NAME_MAX: c_int = 32;
