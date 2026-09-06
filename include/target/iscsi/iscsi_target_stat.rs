//! Automatically rewritten from C Header to Rust Module
//! Source: include/target/iscsi/iscsi_target_stat.h
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
// For struct iscsi_tiqn->tiqn_wwn default groups
//
// For struct iscsi_session->se_sess default groups
//
// iSCSI session error types
pub const ISCSI_SESS_ERR_UNKNOWN: c_int = 0;
pub const ISCSI_SESS_ERR_DIGEST: c_int = 1;
pub const ISCSI_SESS_ERR_CXN_TIMEOUT: c_int = 2;
pub const ISCSI_SESS_ERR_PDU_FORMAT: c_int = 3;
// iSCSI session error stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_sess_err_stats {
    pub lock: spinlock_t,
    pub digest_errors: u32,
    pub cxn_timeout_errors: u32,
    pub pdu_format_errors: u32,
    pub last_sess_failure_type: u32,
    pub last_sess_fail_rem_name: [c_char; ISCSI_IQN_LEN],
    pub ____cacheline_aligned: },
// iSCSI login failure types (sub oids)
pub const ISCSI_LOGIN_FAIL_OTHER: c_int = 2;
pub const ISCSI_LOGIN_FAIL_REDIRECT: c_int = 3;
pub const ISCSI_LOGIN_FAIL_AUTHORIZE: c_int = 4;
pub const ISCSI_LOGIN_FAIL_AUTHENTICATE: c_int = 5;
pub const ISCSI_LOGIN_FAIL_NEGOTIATE: c_int = 6;
// iSCSI login stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_login_stats {
    pub lock: spinlock_t,
    pub accepts: u32,
    pub other_fails: u32,
    pub redirects: u32,
    pub authorize_fails: u32,
    pub authenticate_fails: u32,
    pub /: *mut *mut u32 negotiate_fails; / used for notifications,
    pub /: *mut *mut u64 last_fail_time; / time stamp (jiffies),
    pub last_fail_type: u32,
    pub last_intr_fail_ip_family: c_int,
    pub last_intr_fail_sockaddr: sockaddr_storage,
    pub last_intr_fail_name: [c_char; ISCSI_IQN_LEN],
    pub ____cacheline_aligned: },
// iSCSI logout stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_logout_stats {
    pub lock: spinlock_t,
    pub normal_logouts: u32,
    pub abnormal_logouts: u32,
    pub ____cacheline_aligned: },
