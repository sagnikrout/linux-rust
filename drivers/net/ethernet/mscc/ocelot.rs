//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mscc/ocelot.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Microsemi Ocelot Switch driver
//
// Copyright (c) 2017 Microsemi Corporation
//

pub const OCELOT_STANDALONE_PVID: c_int = 0;
pub const OCELOT_BUFFER_CELL_SZ: c_int = 60;

pub const OCELOT_PTP_QUEUE_SZ: c_int = 128;
pub const OCELOT_JUMBO_MTU: c_int = 9000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_port_tc {
    pub block_shared: bool,
    pub offload_cnt: c_ulong,
    pub ingress_mirred_id: c_ulong,
    pub egress_mirred_id: c_ulong,
    pub police_id: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_port_private {
    pub port: ocelot_port,
    pub dev: *mut net_device,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub tc: ocelot_port_tc,
}

// A (PGID) port mask structure, encoding the 2^ocelot->num_phys_ports
// possibilities of egress port masks for L2 multicast traffic.
// For a switch with 9 user ports, there are 512 possible port masks, but the
// hardware only has 46 individual PGIDs that it can forward multicast traffic
// to. So we need a structure that maps the limited PGID indices to the port
// destinations requested by the user for L2 multicast.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_pgid {
    pub ports: c_ulong,
    pub index: c_int,
    pub refcount: refcount_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_multicast {
    pub list: list_head,
    pub entry_type: macaccess_entry_type,
    pub addr: [c_uchar; ETH_ALEN],
    pub vid: u16,
    pub ports: u16,
    pub pgid: *mut ocelot_pgid,
}

// target = reg >> TARGET_OFFSET;
// addr = ocelot->map[*target][reg & REG_MASK];
extern "C" {
    pub fn ocelot_netdev_to_port(ocelot: *mut ocelot, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ocelot_release_port(ocelot_port: *mut ocelot_port);
}
extern "C" {
    pub fn ocelot_port_devlink_teardown(ocelot: *mut ocelot, port: c_int);
}
extern "C" {
    pub fn ocelot_trap_del(ocelot: *mut ocelot, port: c_int, cookie: c_ulong) -> c_int;
}
extern "C" {
    pub fn ocelot_mirror_put(ocelot: *mut ocelot);
}
extern "C" {
    pub fn ocelot_stats_init(ocelot: *mut ocelot) -> c_int;
}
extern "C" {
    pub fn ocelot_stats_deinit(ocelot: *mut ocelot);
}
extern "C" {
    pub fn ocelot_mm_init(ocelot: *mut ocelot) -> c_int;
}
extern "C" {
    pub fn ocelot_port_update_active_preemptible_tcs(ocelot: *mut ocelot, port: c_int);
}
