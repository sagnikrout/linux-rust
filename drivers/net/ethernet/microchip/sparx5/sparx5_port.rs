//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_port.h
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver
//
// Copyright (c) 2021 Microchip Technology Inc. and its subsidiaries.
//

// Port PCP rewrite mode
pub const SPARX5_PORT_REW_TAG_CTRL_CLASSIFIED: c_int = 0;
pub const SPARX5_PORT_REW_TAG_CTRL_DEFAULT: c_int = 1;
pub const SPARX5_PORT_REW_TAG_CTRL_MAPPED: c_int = 2;
// Port DSCP rewrite mode
pub const SPARX5_PORT_REW_DSCP_NONE: c_int = 0;
pub const SPARX5_PORT_REW_DSCP_IF_ZERO: c_int = 1;
pub const SPARX5_PORT_REW_DSCP_SELECTED: c_int = 2;
pub const SPARX5_PORT_REW_DSCP_ALL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_status {
    pub link: bool,
    pub link_down: bool,
    pub speed: c_int,
    pub an_complete: bool,
    pub duplex: c_int,
    pub pause: c_int,
}

extern "C" {
    pub fn sparx5_port_enable(port: *mut sparx5_port, enable: bool);
}
extern "C" {
    pub fn sparx5_port_fwd_urg(sparx5: *mut sparx5, speed: u32) -> c_int;
}
pub const SPARX5_PORT_QOS_PCP_COUNT: c_int = 8;
pub const SPARX5_PORT_QOS_DEI_COUNT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_qos_pcp_map {
    pub map: [u8; SPARX5_PORT_QOS_PCP_DEI_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_qos_pcp_rewr_map {
    pub map: [u16; SPX5_PRIOS],
}

pub const SPARX5_PORT_QOS_DP_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_qos_dscp_rewr_map {
    pub SPARX5_PORT_QOS_DP_NUM]: *mut *mut u16 map[SPX5_PRIOS,
}

pub const SPARX5_PORT_QOS_DSCP_COUNT: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_qos_dscp_map {
    pub map: [u8; SPARX5_PORT_QOS_DSCP_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_qos_pcp {
    pub map: sparx5_port_qos_pcp_map,
    pub qos_enable: bool,
    pub dp_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_qos_pcp_rewr {
    pub map: sparx5_port_qos_pcp_rewr_map,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_qos_dscp {
    pub map: sparx5_port_qos_dscp_map,
    pub qos_enable: bool,
    pub dp_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_qos_dscp_rewr {
    pub map: sparx5_port_qos_dscp_rewr_map,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_qos {
    pub pcp: sparx5_port_qos_pcp,
    pub pcp_rewr: sparx5_port_qos_pcp_rewr,
    pub dscp: sparx5_port_qos_dscp,
    pub dscp_rewr: sparx5_port_qos_dscp_rewr,
    pub default_prio: u8,
}

extern "C" {
    pub fn sparx5_port_qos_set(port: *mut sparx5_port, qos: *mut sparx5_port_qos) -> c_int;
}
