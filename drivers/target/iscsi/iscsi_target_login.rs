//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/iscsi_target_login.h
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
    pub fn iscsi_check_for_session_reinstatement(: *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn iscsi_login_post_auth_non_zero_tsih(: *mut iscsit_conn, _arg: u16, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_accept_np(: *mut iscsi_np, : *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn iscsit_get_login_rx(: *mut iscsit_conn, : *mut iscsi_login) -> c_int;
}
extern "C" {
    pub fn iscsit_put_login_tx(: *mut iscsit_conn, : *mut iscsi_login, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_free_conn(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_start_kthreads(: *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn iscsi_post_login_handler(: *mut iscsi_np, : *mut iscsit_conn, _arg: u8);
}
extern "C" {
    pub fn iscsi_target_login_sess_out(: *mut iscsit_conn, _arg: bool, _arg: bool);
}
extern "C" {
    pub fn iscsi_target_login_thread(: *mut c_void) -> c_int;
}
