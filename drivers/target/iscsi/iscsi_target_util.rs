//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/iscsi_target_util.h
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

pub const MARKER_SIZE: c_int = 8;
extern "C" {
    pub fn iscsit_add_r2t_to_list(: *mut iscsit_cmd, _arg: u32, _arg: u32, _arg: c_int, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_free_r2t(: *mut iscsi_r2t, : *mut iscsit_cmd);
}
extern "C" {
    pub fn iscsit_free_r2ts_from_list(: *mut iscsit_cmd);
}
extern "C" {
    pub fn iscsit_add_cmd_to_immediate_queue(: *mut iscsit_cmd, : *mut iscsit_conn, _arg: u8);
}
extern "C" {
    pub fn iscsit_add_cmd_to_response_queue(: *mut iscsit_cmd, : *mut iscsit_conn, _arg: u8) -> c_int;
}
extern "C" {
    pub fn iscsit_conn_all_queues_empty(: *mut iscsit_conn) -> bool;
}
extern "C" {
    pub fn iscsit_free_queue_reqs_for_conn(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_release_cmd(: *mut iscsit_cmd);
}
extern "C" {
    pub fn __iscsit_free_cmd(: *mut iscsit_cmd, _arg: bool);
}
extern "C" {
    pub fn iscsit_free_cmd(: *mut iscsit_cmd, _arg: bool);
}
extern "C" {
    pub fn iscsit_check_session_usage_count(sess: *mut iscsit_session, can_sleep: bool) -> bool;
}
extern "C" {
    pub fn iscsit_dec_session_usage_count(: *mut iscsit_session);
}
extern "C" {
    pub fn iscsit_inc_session_usage_count(: *mut iscsit_session);
}
extern "C" {
    pub fn iscsit_check_conn_usage_count(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_dec_conn_usage_count(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_inc_conn_usage_count(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_handle_nopin_response_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn iscsit_mod_nopin_response_timer(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_start_nopin_response_timer(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_stop_nopin_response_timer(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_handle_nopin_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn __iscsit_start_nopin_timer(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_start_nopin_timer(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_stop_nopin_timer(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_login_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn iscsit_start_login_timer(: *mut iscsit_conn, kthr: *mut task_struct);
}
extern "C" {
    pub fn iscsit_stop_login_timer(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_set_login_timer_kworker(: *mut iscsit_conn, kthr: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn iscsit_send_tx_data(: *mut iscsit_cmd, : *mut iscsit_conn, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn iscsit_fe_sendpage_sg(: *mut iscsit_cmd, : *mut iscsit_conn) -> c_int;
}
extern "C" {
    pub fn iscsit_tx_login_rsp(: *mut iscsit_conn, _arg: u8, _arg: u8) -> c_int;
}
extern "C" {
    pub fn rx_data(: *mut iscsit_conn, : *mut kvec, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tx_data(: *mut iscsit_conn, : *mut kvec, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn iscsit_collect_login_stats(: *mut iscsit_conn, _arg: u8, _arg: u8);
}
extern "C" {
    pub fn iscsit_fill_cxn_timeout_err_stats(: *mut iscsit_session);
}
