//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/iscsi_target.h
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

extern "C" {
    pub fn iscsit_put_tiqn_for_login(: *mut iscsi_tiqn);
}
extern "C" {
    pub fn iscsit_del_tiqn(: *mut iscsi_tiqn);
}
extern "C" {
    pub fn iscsit_access_np(: *mut iscsi_np, : *mut iscsi_portal_group) -> c_int;
}
extern "C" {
    pub fn iscsit_login_kref_put(: *mut kref);
}
extern "C" {
    pub fn iscsit_del_np(: *mut iscsi_np) -> c_int;
}
extern "C" {
    pub fn iscsit_reject_cmd(cmd: *mut iscsit_cmd, _arg: u8, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn iscsit_set_unsolicited_dataout(: *mut iscsit_cmd);
}
extern "C" {
    pub fn iscsit_logout_closesession(: *mut iscsit_cmd, : *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn iscsit_logout_closeconnection(: *mut iscsit_cmd, : *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn iscsit_logout_removeconnforrecovery(: *mut iscsit_cmd, : *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn iscsit_build_r2ts_for_cmd(: *mut iscsit_conn, : *mut iscsit_cmd, recovery: bool) -> c_int;
}
extern "C" {
    pub fn iscsit_thread_get_cpumask(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsi_target_tx_thread(: *mut c_void) -> c_int;
}
extern "C" {
    pub fn iscsi_target_rx_thread(: *mut c_void) -> c_int;
}
extern "C" {
    pub fn iscsit_close_connection(: *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn iscsit_close_session(: *mut iscsit_session, can_sleep: bool) -> c_int;
}
extern "C" {
    pub fn iscsit_stop_session(: *mut iscsit_session, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn iscsit_release_sessions_for_tpg(: *mut iscsi_portal_group, _arg: c_int) -> c_int;
}
