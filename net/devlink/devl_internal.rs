//! Automatically rewritten from C Header to Rust Module
//! Source: net/devlink/devl_internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2016 Mellanox Technologies. All rights reserved.
// Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dev_stats {
    pub reload_stats: [u32; DEVLINK_RELOAD_STATS_ARRAY_SIZE],
    pub remote_reload_stats: [u32; DEVLINK_RELOAD_STATS_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink {
    pub index: u32,
    pub ports: xarray,
    pub rate_list: list_head,
    pub sb_list: list_head,
    pub dpipe_table_list: list_head,
    pub resource_list: list_head,
    pub params: xarray,
    pub region_list: list_head,
    pub reporter_list: list_head,
    pub dpipe_headers: *mut devlink_dpipe_headers,
    pub trap_list: list_head,
    pub trap_group_list: list_head,
    pub trap_policer_list: list_head,
    pub linecard_list: list_head,
    pub ops: *const devlink_ops,
    pub snapshot_ids: xarray,
    pub stats: devlink_dev_stats,
    pub dev: *mut device,
    pub dev_name_index: *const c_char,
    pub dev_driver: *const device_driver,
    pub _net: possible_net_t,
// Serializes access to devlink instance specific objects such as
// port, sb, dpipe, resource, params, region, traps and more.
//
    pub lock: mutex,
    pub lock_key: lock_class_key,
    pub reload_failed:1: u8,
    pub refcount: refcount_t,
    pub rwork: rcu_work,
    pub rel: *mut devlink_rel,
    pub nested_rels: xarray,
    pub __aligned(NETDEV_ALIGN): char priv[],
}

// devlink instances are open to the access from the user space after
// devlink_register() call. Such logical barrier allows us to have certain
// expectations related to locking.
//
// Before *_register() - we are in initialization stage and no parallel
// access possible to the devlink instance. All drivers perform that phase
// by implicitly holding device_lock.
//
// After *_register() - users and driver can access devlink instance at
// the same time.
//

// Iterate over devlink pointers which were possible to get reference to.
// devlink_put() needs to be called for each iterated devlink pointer
// in loop body in order to release the reference.
//

extern "C" {
    pub fn xa_get_mark(_arg: &devlinks, _arg: devlink->index, _arg: DEVLINK_REGISTERED) -> return;
}
extern "C" {
    pub fn __devl_is_registered(_arg: devlink) -> return;
}
extern "C" {
    pub fn devlink_rel_notify_cb_t(devlink: *mut devlink, obj_index: u32) -> typedef void;
}
// Returns the locked+referenced nested-in instance or NULL.
extern "C" {
    pub fn devlink_rel_nested_in_clear(rel_index: u32);
}
extern "C" {
    pub fn devlink_rel_nested_in_notify(devlink: *mut devlink);
}
// Netlink
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_nl_ctx {
    pub devlink: *mut devlink,
    pub devlink_port: *mut devlink_port,
    pub parent_devlink: *mut devlink,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devlink_multicast_groups {
    DEVLINK_MCGRP_CONFIG,
}

// state held across netlink dumps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_nl_dump_state {
    pub instance: c_ulong,
    pub idx: c_int,
// DEVLINK_CMD_REGION_READ
    pub start_offset: u64,
}

// DEVLINK_CMD_HEALTH_REPORTER_DUMP_GET
// DEVLINK_CMD_RESOURCE_DUMP
extern "C" {
    pub fn nla_put_u64_64bit(_arg: msg, _arg: attrtype, _arg: val, _arg: DEVLINK_ATTR_PAD) -> return;
}
extern "C" {
    pub fn devlink_nl_msg_reply_and_new(msg: *mut sk_buff, info: *mut genl_info) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_obj_desc {
    pub rcu: rcu_head,
    pub bus_name: *const c_char,
    pub dev_name: *const c_char,
    pub port_index: c_uint,
    pub port_index_valid: bool,
    pub devlink_index: c_uint,
    pub devlink_index_valid: bool,
    pub data: [c_long; ],
}

extern "C" {
    pub fn devlink_nl_notify_filter(dsk: *mut sock, skb: *mut sk_buff, data: *mut c_void) -> c_int;
}
// Notify
extern "C" {
    pub fn devlink_notify_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_notify_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_ports_notify_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_ports_notify_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_params_notify_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_params_notify_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_regions_notify_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_regions_notify_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_trap_policers_notify_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_trap_policers_notify_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_trap_groups_notify_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_trap_groups_notify_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_traps_notify_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_traps_notify_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_rates_notify_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_rates_notify_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_linecards_notify_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_linecards_notify_unregister(devlink: *mut devlink);
}
// Ports

// Reload
extern "C" {
    pub fn devlink_reload_actions_valid(ops: *const devlink_ops) -> bool;
}
// Params
extern "C" {
    pub fn devlink_params_driverinit_load_new(devlink: *mut devlink);
}
// Resources
// Rates
extern "C" {
    pub fn devlink_rate_is_node(devlink_rate: *const devlink_rate) -> bool;
}
// Linecards
extern "C" {
    pub fn devlink_linecard_index(linecard: *mut devlink_linecard) -> c_uint;
}
