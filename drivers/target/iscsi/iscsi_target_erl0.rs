//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/iscsi_target_erl0.h
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
    pub fn iscsit_set_dataout_sequence_values(: *mut iscsit_cmd);
}
extern "C" {
    pub fn iscsit_check_pre_dataout(: *mut iscsit_cmd, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn iscsit_check_post_dataout(: *mut iscsit_cmd, : *mut c_uchar, _arg: u8) -> c_int;
}
extern "C" {
    pub fn iscsit_start_time2retain_handler(: *mut iscsit_session);
}
extern "C" {
    pub fn iscsit_handle_time2retain_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn iscsit_stop_time2retain_timer(: *mut iscsit_session) -> c_int;
}
extern "C" {
    pub fn iscsit_connection_reinstatement_rcfr(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_cause_connection_reinstatement(: *mut iscsit_conn, _arg: c_int);
}
extern "C" {
    pub fn iscsit_fall_back_to_erl0(: *mut iscsit_session);
}
extern "C" {
    pub fn iscsit_take_action_for_connection_exit(: *mut iscsit_conn, : *mut bool);
}
