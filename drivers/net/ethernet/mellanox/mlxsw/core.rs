//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/core.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2015-2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_core_resource_id {
    MLXSW_CORE_RESOURCE_PORTS = 1,
    MLXSW_CORE_RESOURCE_MAX,
}

extern "C" {
    pub fn mlxsw_core_max_ports(mlxsw_core: *const mlxsw_core) -> c_uint;
}
extern "C" {
    pub fn mlxsw_core_max_lag(mlxsw_core: *mut mlxsw_core, p_max_lag: *mut u16) -> c_int;
}
extern "C" {
    pub fn mlxsw_core_driver_register(mlxsw_driver: *mut mlxsw_driver) -> c_int;
}
extern "C" {
    pub fn mlxsw_core_driver_unregister(mlxsw_driver: *mut mlxsw_driver);
}
extern "C" {
    pub fn mlxsw_core_bus_device_unregister(mlxsw_core: *mut mlxsw_core, reload: bool);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_tx_info {
    pub local_port: u16,
    pub is_emad: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_txhdr_info {
    pub tx_info: mlxsw_tx_info,
    pub data: bool,
    pub /: *mut *mut u16 max_fid; / Used for PTP packets which are sent as data.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_rx_md_info {
    pub napi: *mut napi_struct,
    pub cookie_index: u32,
    pub latency: u32,
    pub tx_congestion: u32,
// Valid when 'tx_port_valid' is set.
    pub tx_sys_port: u16,
    pub tx_lag_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_rx_listener {
    pub priv): *mut *mut *mut void (func)(struct sk_buff skb, u16 local_port, void,
    pub local_port: u16,
    pub mirror_reason: u8,
    pub trap_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_event_listener {
    pub priv): *mut *mut char payload, void,
    pub trap_id: mlxsw_event_trap_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_listener {
    pub trap_id: u16,
    pub rx_listener: mlxsw_rx_listener,
    pub event_listener: mlxsw_event_listener,
}

// is registered.
//

extern "C" {
    pub fn mlxsw_reg_trans_bulk_wait(bulk_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn mlxsw_irq_event_cb_t(mlxsw_core: *mut mlxsw_core) -> typedef void;
}
extern "C" {
    pub fn mlxsw_core_irq_event_handlers_call(mlxsw_core: *mut mlxsw_core);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_rx_info {
    pub is_lag: bool,
    pub sys_port: u16,
    pub lag_id: u16,
    pub u: },
    pub lag_port_index: u16,
    pub mirror_reason: u8,
    pub trap_id: c_int,
}

extern "C" {
    pub fn mlxsw_core_port_fini(mlxsw_core: *mut mlxsw_core, local_port: u16);
}
extern "C" {
    pub fn mlxsw_core_cpu_port_fini(mlxsw_core: *mut mlxsw_core);
}
extern "C" {
    pub fn mlxsw_core_schedule_dw(dwork: *mut delayed_work, delay: c_ulong) -> c_int;
}
extern "C" {
    pub fn mlxsw_core_schedule_work(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn mlxsw_core_flush_owq();
}
pub const MLXSW_CONFIG_PROFILE_SWID_COUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_swid_config {
    pub type: u8,
    pub properties: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_config_profile {
    pub max_vepa_channels: u8,
    pub max_lag: u16,
    pub max_mid: u16,
    pub max_pgt: u16,
    pub max_system_port: u16,
    pub max_vlan_groups: u16,
    pub max_regions: u16,
    pub max_flood_tables: u8,
    pub max_vid_flood_tables: u8,
// Flood mode to use if used_flood_mode. If flood_mode_prefer_cff,
// the backup flood mode (if any) when CFF unsupported.
//
    pub flood_mode: u8,
    pub max_fid_offset_flood_tables: u8,
    pub fid_offset_flood_table_size: u16,
    pub max_fid_flood_tables: u8,
    pub fid_flood_table_size: u16,
    pub max_ib_mc: u16,
    pub max_pkey: u16,
    pub ar_sec: u8,
    pub adaptive_routing_group_cap: u16,
    pub arn: u8,
    pub ubridge: u8,
    pub kvd_linear_size: u32,
    pub kvd_hash_single_parts: u8,
    pub kvd_hash_double_parts: u8,
    pub cqe_time_stamp_type: u8,
    pub lag_mode_prefer_sw: bool,
    pub flood_mode_prefer_cff: bool,
    pub swid_config: [mlxsw_swid_config; MLXSW_CONFIG_PROFILE_SWID_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_driver {
    pub list: list_head,
    pub kind: *const c_char,
    pub priv_size: usize,
    pub fw_req_rev: *const mlxsw_fw_rev,
    pub fw_filename: *const c_char,
    pub extack): *mut netlink_ext_ack,
    pub mlxsw_core): *mut *mut void (fini)(struct mlxsw_core,
    pub extack): *mut unsigned int count, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub priv): *mut c_void,
    pub pool_info): *mut devlink_sb_pool_info,
    pub extack): *mut netlink_ext_ack,
    pub p_threshold): *mut u32,
    pub extack): *mut u32 threshold, struct netlink_ext_ack,
    pub p_threshold): *mut *mut u16 p_pool_index, u32,
    pub extack): *mut netlink_ext_ack,
    pub sb_index): c_uint,
    pub sb_index): c_uint,
    pub p_max): *mut *mut u32 p_cur, u32,
    pub p_max): *mut *mut u32 p_cur, u32,
    pub trap_ctx): *const *const devlink_trap trap, void,
    pub trap_ctx): *const *const devlink_trap trap, void,
    pub extack): *mut netlink_ext_ack,
    pub group): *const devlink_trap_group,
    pub extack): *mut netlink_ext_ack,
    pub policer): *const devlink_trap_policer,
    pub policer): *const devlink_trap_policer,
    pub extack): *mut netlink_ext_ack,
    pub p_drops): *mut u64,
    pub mlxsw_core): *mut *mut int (resources_register)(struct mlxsw_core,
    pub p_linear_size): *mut u64,
// Notify a driver that a timestamped packet was transmitted. Driver
// is responsible for freeing the passed-in SKB.
//
    pub local_port): *mut *mut sk_buff skb, u16,
    pub profile: *const mlxsw_config_profile,
    pub sdq_supports_cqe_v2: bool,
}

extern "C" {
    pub fn mlxsw_core_read_frc_h(mlxsw_core: *mut mlxsw_core) -> u32;
}
extern "C" {
    pub fn mlxsw_core_read_frc_l(mlxsw_core: *mut mlxsw_core) -> u32;
}
extern "C" {
    pub fn mlxsw_core_read_utc_sec(mlxsw_core: *mut mlxsw_core) -> u32;
}
extern "C" {
    pub fn mlxsw_core_read_utc_nsec(mlxsw_core: *mut mlxsw_core) -> u32;
}
extern "C" {
    pub fn mlxsw_core_sdq_supports_cqe_v2(mlxsw_core: *mut mlxsw_core) -> bool;
}

extern "C" {
    pub fn devlink_net(_arg: priv_to_devlink(mlxsw_core)) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_bus {
    pub kind: *const c_char,
    pub res): *mut mlxsw_res,
    pub bus_priv): *mut *mut void (fini)(void,
    pub tx_info): *const mlxsw_tx_info,
    pub txhdr_info): *const mlxsw_txhdr_info,
    pub p_status): *mut u8,
    pub bus_priv): *mut *mut u32 (read_frc_h)(void,
    pub bus_priv): *mut *mut u32 (read_frc_l)(void,
    pub bus_priv): *mut *mut u32 (read_utc_sec)(void,
    pub bus_priv): *mut *mut u32 (read_utc_nsec)(void,
    pub bus_priv): *mut *mut mlxsw_cmd_mbox_config_profile_lag_mode (lag_mode)(void,
    pub priv): *mut *mut mlxsw_cmd_mbox_config_profile_flood_mode (flood_mode)(void,
    pub features: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_fw_rev {
    pub major: u16,
    pub minor: u16,
    pub subminor: u16,
    pub can_reset_minor: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_bus_info {
    pub device_kind: *const c_char,
    pub device_name: *const c_char,
    pub dev: *mut device,
    pub fw_rev: mlxsw_fw_rev,
    pub vsd: [u8; MLXSW_CMD_BOARDINFO_VSD_LEN],
    pub psid: [u8; MLXSW_CMD_BOARDINFO_PSID_LEN],
}

extern "C" {
    pub fn mlxsw_hwmon_fini(mlxsw_hwmon: *mut mlxsw_hwmon);
}

extern "C" {
    pub fn mlxsw_thermal_fini(thermal: *mut mlxsw_thermal);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_devlink_param_id {
    MLXSW_DEVLINK_PARAM_ID_BASE = DEVLINK_PARAM_GENERIC_ID_MAX,
    MLXSW_DEVLINK_PARAM_ID_ACL_REGION_REHASH_INTERVAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_cqe_ts {
    pub sec: u8,
    pub nsec: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_skb_cb {
    pub tx_info: mlxsw_tx_info,
    pub rx_md_info: mlxsw_rx_md_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_linecard_status_event_type {
    MLXSW_LINECARD_STATUS_EVENT_TYPE_PROVISION,
    MLXSW_LINECARD_STATUS_EVENT_TYPE_UNPROVISION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_linecard_device_info {
    pub fw_major: u16,
    pub fw_minor: u16,
    pub fw_sub_minor: u16,
    pub psid: [c_char; MLXSW_REG_MGIR_FW_INFO_PSID_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_linecard {
    pub slot_index: u8,
    pub linecards: *mut mlxsw_linecards,
    pub devlink_linecard: *mut devlink_linecard,
    pub /: *mut *mut mutex lock; / Locks accesses to the linecard structure,
    pub name: [c_char; MLXSW_REG_MDDQ_SLOT_ASCII_NAME_LEN],
    pub /: *mut *mut char mbct_pl[MLXSW_REG_MBCT_LEN]; / Too big for stack,
    pub status_event_type_to: mlxsw_linecard_status_event_type,
    pub status_event_to_dw: delayed_work,
    pub hw_revision: u16,
    pub ini_version: u16,
    pub bdev: *mut mlxsw_linecard_bdev,
    pub info: mlxsw_linecard_device_info,
    pub index: u8,
    pub device: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_linecards {
    pub mlxsw_core: *mut mlxsw_core,
    pub bus_info: *const mlxsw_bus_info,
    pub count: u8,
    pub types_info: *mut mlxsw_linecard_types_info,
    pub event_ops_list: list_head,
    pub /: *mut *mut mutex event_ops_list_lock; / Locks accesses to event ops list,
    pub __counted_by(count): mlxsw_linecard linecards[],
}

extern "C" {
    pub fn mlxsw_linecards_fini(mlxsw_core: *mut mlxsw_core);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_linecards_event_ops {
    pub got_active: *mut mlxsw_linecards_event_op_t,
    pub got_inactive: *mut mlxsw_linecards_event_op_t,
}

extern "C" {
    pub fn mlxsw_linecard_bdev_add(linecard: *mut mlxsw_linecard) -> c_int;
}
extern "C" {
    pub fn mlxsw_linecard_bdev_del(linecard: *mut mlxsw_linecard);
}
extern "C" {
    pub fn mlxsw_linecard_driver_register() -> c_int;
}
extern "C" {
    pub fn mlxsw_linecard_driver_unregister();
}
