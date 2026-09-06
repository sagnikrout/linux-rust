//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/smc_diag.h
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

// Request structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_diag_req {
    pub diag_family: __u8,
    pub pad: [__u8; 2],
    pub /: *mut *mut __u8 diag_ext; / Query extended information,
    pub id: inet_diag_sockid,
}

// Base info structure. It contains socket identity (addrs/ports/cookie) based
// on the internal clcsock, and more SMC-related socket data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_diag_msg {
    pub diag_family: __u8,
    pub diag_state: __u8,
    pub diag_mode: __u8,
    pub /: *mut *mut __u8 diag_fallback; / the old name of the field,
}

// Mode of a connection
// Extensions

// SMC_DIAG_CONNINFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_diag_cursor {
    pub reserved: __u16,
    pub wrap: __u16,
    pub count: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_diag_conninfo {
    pub /: *mut *mut __u32 token; / unique connection id,
    pub /: *mut *mut __u32 sndbuf_size; / size of send buffer,
    pub /: *mut *mut __u32 rmbe_size; / size of RMB element,
    pub /: *mut *mut __u32 peer_rmbe_size; / size of peer RMB element,
// local RMB element cursors
    pub /: *mut *mut smc_diag_cursor rx_prod; / received producer cursor,
    pub /: *mut *mut smc_diag_cursor rx_cons; / received consumer cursor,
// peer RMB element cursors
    pub /: *mut *mut smc_diag_cursor tx_prod; / sent producer cursor,
    pub /: *mut *mut smc_diag_cursor tx_cons; / sent consumer cursor,
    pub /: *mut *mut __u8 rx_prod_flags; / received producer flags,
    pub flags*/: *mut *mut __u8 rx_conn_state_flags; / recvd connection,
    pub /: *mut *mut __u8 tx_prod_flags; / sent producer flags,
    pub flags*/: *mut *mut __u8 tx_conn_state_flags; / sent connection,
// send buffer cursors
    pub /: *mut *mut smc_diag_cursor tx_prep; / prepared to be sent cursor,
    pub /: *mut *mut smc_diag_cursor tx_sent; / sent cursor,
    pub /: *mut *mut smc_diag_cursor tx_fin; / confirmed sent cursor,
}

// SMC_DIAG_LINKINFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_diag_linkinfo {
    pub /: *mut *mut __u8 link_id; / link identifier,
    pub /: *mut *mut __u8 ibname[IB_DEVICE_NAME_MAX]; / name of the RDMA device,
    pub /: *mut *mut __u8 ibport; / RDMA device port number,
    pub /: *mut *mut __u8 gid[40]; / local GID,
    pub /: *mut *mut __u8 peer_gid[40]; / peer GID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_diag_lgrinfo {
    pub lnk: [smc_diag_linkinfo; 1],
    pub role: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_diag_fallback {
    pub reason: __u32,
    pub peer_diagnosis: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smcd_diag_dmbinfo {
    pub /: *mut *mut __u32 linkid; / Link identifier,
    pub /: *mut *mut __aligned_u64 peer_gid; / Peer GID,
    pub /: *mut *mut __aligned_u64 my_gid; / My GID,
    pub /: *mut *mut __aligned_u64 token; / Token of DMB,
    pub /: *mut *mut __aligned_u64 peer_token; / Token of remote DMBE,
    pub /: *mut *mut __aligned_u64 peer_gid_ext; / Peer GID (extended part),
    pub /: *mut *mut __aligned_u64 my_gid_ext; / My GID (extended part),
}
