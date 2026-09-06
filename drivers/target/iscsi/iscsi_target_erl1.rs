//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/iscsi/iscsi_target_erl1.h
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
    pub fn iscsit_dump_data_payload(: *mut iscsit_conn, _arg: u32, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn iscsit_handle_data_ack(: *mut iscsit_conn, _arg: u32, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_dataout_datapduinorder_no_fbit(: *mut iscsit_cmd, : *mut iscsi_pdu) -> c_int;
}
extern "C" {
    pub fn iscsit_recover_dataout_sequence(: *mut iscsit_cmd, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_clear_ooo_cmdsns_for_conn(: *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_free_all_ooo_cmdsns(: *mut iscsit_session);
}
extern "C" {
    pub fn iscsit_execute_ooo_cmdsns(: *mut iscsit_session) -> c_int;
}
extern "C" {
    pub fn iscsit_execute_cmd(: *mut iscsit_cmd, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn iscsit_handle_ooo_cmdsn(: *mut iscsit_session, : *mut iscsit_cmd, _arg: u32) -> c_int;
}
extern "C" {
    pub fn iscsit_remove_ooo_cmdsn(: *mut iscsit_session, : *mut iscsi_ooo_cmdsn);
}
extern "C" {
    pub fn iscsit_handle_dataout_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn iscsit_mod_dataout_timer(: *mut iscsit_cmd);
}
extern "C" {
    pub fn iscsit_start_dataout_timer(: *mut iscsit_cmd, : *mut iscsit_conn);
}
extern "C" {
    pub fn iscsit_stop_dataout_timer(: *mut iscsit_cmd);
}
