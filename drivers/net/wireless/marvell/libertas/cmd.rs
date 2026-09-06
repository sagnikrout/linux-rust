//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas/cmd.h
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
// Copyright (C) 2007, Red Hat, Inc.

// Command & response transfer between host and card
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ctrl_node {
    pub list: list_head,
    pub result: c_int,
// command response
    pub ): *mut cmd_header,
    pub callback_arg: c_ulong,
// command data
    pub cmdbuf: *mut cmd_header,
// wait queue
    pub cmdwaitqwoken: u16,
    pub cmdwait_q: wait_queue_head_t,
}

// lbs_cmd() infers the size of the buffer to copy data back into, from

extern "C" {
    pub fn lbs_allocate_cmd_buffer(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_free_cmd_buffer(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_execute_next_command(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_process_command_response(priv: *mut lbs_private, data: *mut u8, len: u32) -> c_int;
}
// From cmdresp.c
// Events
extern "C" {
    pub fn lbs_process_event(priv: *mut lbs_private, event: u32);
}
// Actual commands
extern "C" {
    pub fn lbs_update_hw_spec(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_set_channel(priv: *mut lbs_private, channel: u8) -> c_int;
}
extern "C" {
    pub fn lbs_update_channel(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_ps_confirm_sleep(priv: *mut lbs_private);
}
extern "C" {
    pub fn lbs_set_radio(priv: *mut lbs_private, preamble: u8, radio_on: u8) -> c_int;
}
extern "C" {
    pub fn lbs_set_mac_control(priv: *mut lbs_private);
}
extern "C" {
    pub fn lbs_set_mac_control_sync(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_set_snmp_mib(priv: *mut lbs_private, oid: u32, val: u16) -> c_int;
}
// Commands only used in wext.c, assoc. and scan.c
extern "C" {
    pub fn lbs_set_deep_sleep(priv: *mut lbs_private, deep_sleep: c_int) -> c_int;
}
extern "C" {
    pub fn lbs_set_host_sleep(priv: *mut lbs_private, host_sleep: c_int) -> c_int;
}
extern "C" {
    pub fn lbs_set_monitor_mode(priv: *mut lbs_private, enable: c_int) -> c_int;
}
extern "C" {
    pub fn lbs_get_rssi(priv: *mut lbs_private, snr: *mut i8, nf: *mut i8) -> c_int;
}
extern "C" {
    pub fn lbs_set_11d_domain_info(priv: *mut lbs_private) -> c_int;
}
extern "C" {
    pub fn lbs_get_reg(priv: *mut lbs_private, reg: u16, offset: u16, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn lbs_set_reg(priv: *mut lbs_private, reg: u16, offset: u16, value: u32) -> c_int;
}
extern "C" {
    pub fn lbs_set_ps_mode(priv: *mut lbs_private, cmd_action: u16, block: bool) -> c_int;
}
