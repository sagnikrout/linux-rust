//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mptcp.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_info {
    pub mptcpi_subflows: __u8,

    pub mptcpi_add_addr_signal: __u8,
    pub mptcpi_add_addr_accepted: __u8,
    pub mptcpi_subflows_max: __u8,

    pub mptcpi_add_addr_signal_max: __u8,

    pub mptcpi_add_addr_accepted_max: __u8,

// 16-bit hole that can no longer be filled
    pub mptcpi_flags: __u32,
    pub mptcpi_token: __u32,
    pub mptcpi_write_seq: __u64,
    pub mptcpi_snd_una: __u64,
    pub mptcpi_rcv_nxt: __u64,
    pub mptcpi_local_addr_used: __u8,
    pub mptcpi_local_addr_max: __u8,

    pub mptcpi_csum_enabled: __u8,
// 8-bit hole that can no longer be filled
    pub mptcpi_retransmits: __u32,
    pub mptcpi_bytes_retrans: __u64,
    pub mptcpi_bytes_sent: __u64,
    pub mptcpi_bytes_received: __u64,
    pub mptcpi_bytes_acked: __u64,
    pub mptcpi_subflows_total: __u8,
    pub mptcpi_endp_laminar_max: __u8,
    pub mptcpi_endp_fullmesh_max: __u8,
    pub reserved: __u8,
    pub mptcpi_last_data_sent: __u32,
    pub mptcpi_last_data_recv: __u32,
    pub mptcpi_last_ack_recv: __u32,
}

// MPTCP Reset reason codes, rfc8684
pub const MPTCP_RST_EUNSPEC: c_int = 0;
pub const MPTCP_RST_EMPTCP: c_int = 1;
pub const MPTCP_RST_ERESOURCE: c_int = 2;
pub const MPTCP_RST_EPROHIBIT: c_int = 3;
pub const MPTCP_RST_EWQ2BIG: c_int = 4;
pub const MPTCP_RST_EBADPERF: c_int = 5;
pub const MPTCP_RST_EMIDDLEBOX: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_subflow_data {
    pub /: *mut *mut __u32 size_subflow_data; / size of this structure in userspace,
    pub /: *mut *mut __u32 num_subflows; / must be 0, set by kernel,
    pub /: *mut *mut __u32 size_kernel; / must be 0, set by kernel,
    pub /: *mut *mut __u32 size_user; / size of one element in data[],
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_subflow_addrs {
    pub sa_family: __kernel_sa_family_t,
    pub sa_local: sockaddr,
    pub sin_local: sockaddr_in,
    pub sin6_local: sockaddr_in6,
    pub ss_local: __kernel_sockaddr_storage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_subflow_info {
    pub id: __u32,
    pub addrs: mptcp_subflow_addrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_full_info {
    pub /: *mut *mut __u32 size_tcpinfo_kernel; / must be 0, set by kernel,
    pub size_tcpinfo_user: __u32,
    pub /: *mut *mut __u32 size_sfinfo_kernel; / must be 0, set by kernel,
    pub size_sfinfo_user: __u32,
    pub /: *mut *mut __u32 num_subflows; / must be 0, set by kernel (real subflow count),
    pub in: *mut *mut __u32 size_arrays_user; / max subflows that userspace is interested,
// the buffers at subflow_info/tcp_info
// are respectively at least:
// size_arrays * size_sfinfo_user
// size_arrays * size_tcpinfo_user
// bytes wide
//
    pub subflow_info: __aligned_u64,
    pub tcp_info: __aligned_u64,
    pub mptcp_info: mptcp_info,
}

// MPTCP socket options
pub const MPTCP_INFO: c_int = 1;
pub const MPTCP_TCPINFO: c_int = 2;
pub const MPTCP_SUBFLOW_ADDRS: c_int = 3;
pub const MPTCP_FULL_INFO: c_int = 4;
