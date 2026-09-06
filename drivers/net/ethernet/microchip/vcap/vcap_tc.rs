//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/vcap/vcap_tc.h
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


// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2023 Microchip Technology Inc. and its subsidiaries.
// Microchip VCAP TC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_tc_flower_parse_usage {
    pub fco: *mut flow_cls_offload,
    pub frule: *mut flow_rule,
    pub vrule: *mut vcap_rule,
    pub admin: *mut vcap_admin,
    pub l3_proto: u16,
    pub l4_proto: u8,
    pub tpid: u16,
    pub used_keys: c_ulonglong,
}

extern "C" {
    pub fn vcap_tc_flower_handler_ethaddr_usage(st: *mut vcap_tc_flower_parse_usage) -> c_int;
}
extern "C" {
    pub fn vcap_tc_flower_handler_ipv4_usage(st: *mut vcap_tc_flower_parse_usage) -> c_int;
}
extern "C" {
    pub fn vcap_tc_flower_handler_ipv6_usage(st: *mut vcap_tc_flower_parse_usage) -> c_int;
}
extern "C" {
    pub fn vcap_tc_flower_handler_portnum_usage(st: *mut vcap_tc_flower_parse_usage) -> c_int;
}
extern "C" {
    pub fn vcap_tc_flower_handler_cvlan_usage(st: *mut vcap_tc_flower_parse_usage) -> c_int;
}
extern "C" {
    pub fn vcap_tc_flower_handler_tcp_usage(st: *mut vcap_tc_flower_parse_usage) -> c_int;
}
extern "C" {
    pub fn vcap_tc_flower_handler_arp_usage(st: *mut vcap_tc_flower_parse_usage) -> c_int;
}
extern "C" {
    pub fn vcap_tc_flower_handler_ip_usage(st: *mut vcap_tc_flower_parse_usage) -> c_int;
}
