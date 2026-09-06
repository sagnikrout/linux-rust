//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/inet_diag.h
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

// Just some random number
pub const TCPDIAG_GETSOCK: c_int = 18;
pub const DCCPDIAG_GETSOCK: c_int = 19;
pub const INET_DIAG_GETSOCK_MAX: c_int = 24;
// Socket identity
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_sockid {
    pub idiag_sport: __be16,
    pub idiag_dport: __be16,
    pub idiag_src: [__be32; 4],
    pub idiag_dst: [__be32; 4],
    pub idiag_if: __u32,
    pub idiag_cookie: [__u32; 2],
}

// Request structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_req {
    pub /: *mut *mut __u8 idiag_family; / Family of addresses.,
    pub idiag_src_len: __u8,
    pub idiag_dst_len: __u8,
    pub /: *mut *mut __u8 idiag_ext; / Query extended information,
    pub id: inet_diag_sockid,
    pub /: *mut *mut __u32 idiag_states; / States to dump,
    pub /: *mut *mut __u32 idiag_dbs; / Tables to dump (NI),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_req_v2 {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub idiag_ext: __u8,
    pub pad: __u8,
    pub idiag_states: __u32,
    pub id: inet_diag_sockid,
}

//
// SOCK_RAW sockets require the underlied protocol to be
// additionally specified so we can use @pad member for
// this, but we can't rename it because userspace programs
// still may depend on this name. Instead lets use another
// structure definition as an alias for struct
// @inet_diag_req_v2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_req_raw {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub idiag_ext: __u8,
    pub sdiag_raw_protocol: __u8,
    pub idiag_states: __u32,
    pub id: inet_diag_sockid,
}

// Bytecode is sequence of 4 byte commands followed by variable arguments.
// All the commands identified by "code" are conditional jumps forward:
// to offset cc+"yes" or to offset cc+"no". "yes" is supposed to be
// length of the command and its arguments.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_bc_op {
    pub code: c_uchar,
    pub yes: c_uchar,
    pub no: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_hostcond {
    pub family: __u8,
    pub prefix_len: __u8,
    pub port: c_int,
    pub addr: [__be32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_markcond {
    pub mark: __u32,
    pub mask: __u32,
}

// Base info structure. It contains socket identity (addrs/ports/cookie)
// and, alas, the information shown by netstat.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_msg {
    pub idiag_family: __u8,
    pub idiag_state: __u8,
    pub idiag_timer: __u8,
    pub idiag_retrans: __u8,
    pub id: inet_diag_sockid,
    pub idiag_expires: __u32,
    pub idiag_rqueue: __u32,
    pub idiag_wqueue: __u32,
    pub idiag_uid: __u32,
    pub idiag_inode: __u32,
}

// Extensions
//
// Next extensions cannot be requested in struct inet_diag_req_v2:
// its field idiag_ext has only 8 bits.
//

// INET_DIAG_MEM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_meminfo {
    pub idiag_rmem: __u32,
    pub idiag_wmem: __u32,
    pub idiag_fmem: __u32,
    pub idiag_tmem: __u32,
}

// INET_DIAG_SOCKOPT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_sockopt {
}

// INET_DIAG_VEGASINFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpvegas_info {
    pub tcpv_enabled: __u32,
    pub tcpv_rttcnt: __u32,
    pub tcpv_rtt: __u32,
    pub tcpv_minrtt: __u32,
}

// INET_DIAG_DCTCPINFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_dctcp_info {
    pub dctcp_enabled: __u16,
    pub dctcp_ce_state: __u16,
    pub dctcp_alpha: __u32,
    pub dctcp_ab_ecn: __u32,
    pub dctcp_ab_tot: __u32,
}

// INET_DIAG_BBRINFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_bbr_info {
// u64 bw: max-filtered BW (app throughput) estimate in Byte per sec:
    pub /: *mut *mut __u32 bbr_bw_lo; / lower 32 bits of bw,
    pub /: *mut *mut __u32 bbr_bw_hi; / upper 32 bits of bw,
    pub /: *mut *mut __u32 bbr_min_rtt; / min-filtered RTT in uSec,
    pub /: *mut *mut __u32 bbr_pacing_gain; / pacing gain shifted left 8 bits,
    pub /: *mut *mut __u32 bbr_cwnd_gain; / cwnd gain shifted left 8 bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_cc_info {
    pub vegas: tcpvegas_info,
    pub dctcp: tcp_dctcp_info,
    pub bbr: tcp_bbr_info,
}
