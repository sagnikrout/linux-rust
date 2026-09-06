//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum.h
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

pub const MLXSW_SP_FID_8021D_MAX: c_int = 1024;
pub const MLXSW_SP_MID_MAX: c_int = 7000;

pub const MLXSW_SP_KVD_GRANULARITY: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_resource_id {
    MLXSW_SP_RESOURCE_KVD = MLXSW_CORE_RESOURCE_MAX,
    MLXSW_SP_RESOURCE_KVD_LINEAR,
    MLXSW_SP_RESOURCE_KVD_HASH_SINGLE,
    MLXSW_SP_RESOURCE_KVD_HASH_DOUBLE,
    MLXSW_SP_RESOURCE_KVD_LINEAR_SINGLE,
    MLXSW_SP_RESOURCE_KVD_LINEAR_CHUNKS,
    MLXSW_SP_RESOURCE_KVD_LINEAR_LARGE_CHUNKS,
    MLXSW_SP_RESOURCE_SPAN,
    MLXSW_SP_RESOURCE_COUNTERS,
    MLXSW_SP_RESOURCE_COUNTERS_FLOW,
    MLXSW_SP_RESOURCE_COUNTERS_RIF,
    MLXSW_SP_RESOURCE_GLOBAL_POLICERS,
    MLXSW_SP_RESOURCE_SINGLE_RATE_POLICERS,
    MLXSW_SP_RESOURCE_RIF_MAC_PROFILES,
    MLXSW_SP_RESOURCE_RIFS,
    MLXSW_SP_RESOURCE_PORT_RANGE_REGISTERS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_rif_type {
    MLXSW_SP_RIF_TYPE_SUBPORT,
    MLXSW_SP_RIF_TYPE_VLAN,
    MLXSW_SP_RIF_TYPE_FID,
    MLXSW_SP_RIF_TYPE_IPIP_LB, /* IP-in-IP loopback. */
    MLXSW_SP_RIF_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_fid_type {
    MLXSW_SP_FID_TYPE_8021Q,
    MLXSW_SP_FID_TYPE_8021D,
    MLXSW_SP_FID_TYPE_RFID,
    MLXSW_SP_FID_TYPE_DUMMY,
    MLXSW_SP_FID_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_nve_type {
    MLXSW_SP_NVE_TYPE_VXLAN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_port_mapping {
    pub module: u8,
    pub slot_index: u8,
    pub /: *mut *mut u8 width; / Number of lanes used by the port,
    pub /: *mut *mut u8 module_width; / Number of lanes in the module (static),
    pub lane: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_port_mapping_events {
    pub queue: list_head,
    pub /: *mut *mut spinlock_t queue_lock; / protects queue,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_parsing {
    pub parsing_depth_ref: refcount_t,
    pub parsing_depth: u16,
    pub vxlan_udp_dport: u16,
    pub /: *mut *mut mutex lock; / Protects parsing configuration,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp {
    pub ports: *mut mlxsw_sp_port,
    pub core: *mut mlxsw_core,
    pub bus_info: *const mlxsw_bus_info,
    pub base_mac: [c_uchar; ETH_ALEN],
    pub mac_mask: *const c_uchar,
    pub lags: *mut mlxsw_sp_lag,
    pub max_lag: u16,
    pub port_mapping: *mut mlxsw_sp_port_mapping,
    pub port_mapping_events: mlxsw_sp_port_mapping_events,
    pub sample_trigger_ht: rhashtable,
    pub sb: *mut mlxsw_sp_sb,
    pub bridge: *mut mlxsw_sp_bridge,
    pub router: *mut mlxsw_sp_router,
    pub mr: *mut mlxsw_sp_mr,
    pub afa: *mut mlxsw_afa,
    pub acl: *mut mlxsw_sp_acl,
    pub fid_core: *mut mlxsw_sp_fid_core,
    pub policer_core: *mut mlxsw_sp_policer_core,
    pub pr_core: *mut mlxsw_sp_port_range_core,
    pub kvdl: *mut mlxsw_sp_kvdl,
    pub nve: *mut mlxsw_sp_nve,
    pub netdevice_nb: notifier_block,
    pub clock: *mut mlxsw_sp_ptp_clock,
    pub ptp_state: *mut mlxsw_sp_ptp_state,
    pub counter_pool: *mut mlxsw_sp_counter_pool,
    pub span: *mut mlxsw_sp_span,
    pub trap: *mut mlxsw_sp_trap,
    pub parsing: mlxsw_sp_parsing,
    pub switchdev_ops: *const mlxsw_sp_switchdev_ops,
    pub kvdl_ops: *const mlxsw_sp_kvdl_ops,
    pub afa_ops: *const mlxsw_afa_ops,
    pub afk_ops: *const mlxsw_afk_ops,
    pub mr_tcam_ops: *const mlxsw_sp_mr_tcam_ops,
    pub acl_rulei_ops: *const mlxsw_sp_acl_rulei_ops,
    pub acl_tcam_ops: *const mlxsw_sp_acl_tcam_ops,
    pub acl_bf_ops: *const mlxsw_sp_acl_bf_ops,
    pub nve_ops_arr: *const mlxsw_sp_nve_ops,
    pub sb_vals: *const mlxsw_sp_sb_vals,
    pub sb_ops: *const mlxsw_sp_sb_ops,
    pub port_type_speed_ops: *const mlxsw_sp_port_type_speed_ops,
    pub ptp_ops: *const mlxsw_sp_ptp_ops,
    pub span_ops: *const mlxsw_sp_span_ops,
    pub policer_core_ops: *const mlxsw_sp_policer_core_ops,
    pub trap_ops: *const mlxsw_sp_trap_ops,
    pub mall_ops: *const mlxsw_sp_mall_ops,
    pub router_ops: *const mlxsw_sp_router_ops,
    pub listeners: *const mlxsw_listener,
    pub fid_core_ops: *const mlxsw_sp_fid_core_ops,
    pub listeners_count: usize,
    pub lowest_shaper_bs: u32,
    pub ipv6_addr_ht: rhashtable,
    pub /: *mut *mut mutex ipv6_addr_ht_lock; / Protects ipv6_addr_ht,
    pub pgt: *mut mlxsw_sp_pgt,
    pub pgt_smpe_index_valid: bool,
    pub lag_pgt_base: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_ptp_ops {
    pub dev): *mut *mut *mut (clock_init)(struct mlxsw_sp mlxsw_sp, struct device,
    pub clock): *mut *mut void (clock_fini)(struct mlxsw_sp_ptp_clock,
    pub mlxsw_sp): *mut *mut *mut mlxsw_sp_ptp_state (init)(mlxsw_sp,
    pub ptp_state): *mut *mut void (fini)(struct mlxsw_sp_ptp_state,
// Notify a driver that a packet that might be PTP was received. Driver
// is responsible for freeing the passed-in SKB.
//
    pub local_port): u16,
// Notify a driver that a timestamped packet was transmitted. Driver
// is responsible for freeing the passed-in SKB.
//
    pub local_port): u16,
    pub config): *mut kernel_hwtstamp_config,
    pub extack): *mut netlink_ext_ack,
    pub work): *mut *mut void (shaper_work)(struct work_struct,
    pub info): *mut kernel_ethtool_ts_info,
    pub (*get_stats_count)(void): *mut c_int,
    pub p): *mut *mut void (get_stats_strings)(u8,
    pub data_index): *mut *mut u64 data, int,
    pub tx_as_data: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_fid_core_ops {
    pub mlxsw_sp): *mut *mut int (init)(struct mlxsw_sp,
    pub mlxsw_sp): *mut *mut void (fini)(struct mlxsw_sp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_port_pcpu_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub syncp: u64_stats_sync,
    pub tx_dropped: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_sample_trigger_type {
    MLXSW_SP_SAMPLE_TRIGGER_TYPE_INGRESS,
    MLXSW_SP_SAMPLE_TRIGGER_TYPE_EGRESS,
    MLXSW_SP_SAMPLE_TRIGGER_TYPE_POLICY_ENGINE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_sample_trigger {
    pub type: mlxsw_sp_sample_trigger_type,
    pub /: *mut *mut u16 local_port; / Reserved when trigger type is not ingress / egress.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_sample_params {
    pub psample_group: *mut psample_group,
    pub trunc_size: u32,
    pub rate: u32,
    pub truncate: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_port_vlan {
    pub list: list_head,
    pub mlxsw_sp_port: *mut mlxsw_sp_port,
    pub fid: *mut mlxsw_sp_fid,
    pub vid: u16,
    pub bridge_port: *mut mlxsw_sp_bridge_port,
    pub bridge_vlan_node: list_head,
}

// No need an internal lock; At worse - miss a single periodic iteration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_port_xstats {
    pub ecn: u64,
    pub tc_ecn: [u64; TC_MAX_QUEUE],
    pub wred_drop: [u64; TC_MAX_QUEUE],
    pub tail_drop: [u64; TC_MAX_QUEUE],
    pub backlog: [u64; TC_MAX_QUEUE],
    pub tx_bytes: [u64; IEEE_8021QAZ_MAX_TCS],
    pub tx_packets: [u64; IEEE_8021QAZ_MAX_TCS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_ptp_port_dir_stats {
    pub packets: u64,
    pub timestamps: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_ptp_port_stats {
    pub rx_gcd: mlxsw_sp_ptp_port_dir_stats,
    pub tx_gcd: mlxsw_sp_ptp_port_dir_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_port {
    pub dev: *mut net_device,
    pub pcpu_stats: *mut mlxsw_sp_port_pcpu_stats __percpu,
    pub mlxsw_sp: *mut mlxsw_sp,
    pub local_port: u16,
    pub pvid: u16,
    pub lag_id: u16,
    pub link: },
    pub ets: *mut ieee_ets,
    pub maxrate: *mut ieee_maxrate,
    pub pfc: *mut ieee_pfc,
    pub trust_state: mlxsw_reg_qpts_trust_state,
    pub dcb: },
    pub the: *mut *mut mlxsw_sp_port_mapping mapping; / mapping is constant during,
// mlxsw_sp_port lifetime, however
// the same localport can have
// different mapping.
//

    pub stats: rtnl_link_stats64,
    pub xstats: mlxsw_sp_port_xstats,
    pub update_dw: delayed_work,
    pub periodic_hw_stats: },
    pub vlans_list: list_head,
    pub default_vlan: *mut mlxsw_sp_port_vlan,
    pub qdisc: *mut mlxsw_sp_qdisc_state,
    pub acl_rule_count: unsigned,
    pub ing_flow_block: *mut mlxsw_sp_flow_block,
    pub eg_flow_block: *mut mlxsw_sp_flow_block,
    pub shaper_dw: delayed_work,
    pub hwtstamp_config: kernel_hwtstamp_config,
    pub ing_types: u16,
    pub egr_types: u16,
    pub stats: mlxsw_sp_ptp_port_stats,
    pub ptp: },
    pub max_speed: u32,
    pub hdroom: *mut mlxsw_sp_hdroom,
    pub module_overheat_initial_val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_port_type_speed_ops {
    pub cmd): *mut ethtool_link_ksettings,
    pub mode): *mut c_ulong,
    pub ptys_eth_proto): *mut *mut *mut u32 (from_ptys_speed)(struct mlxsw_sp mlxsw_sp, u32,
    pub cmd): *mut ethtool_link_ksettings,
    pub p_max_speed): *mut *mut *mut int (ptys_max_speed)(struct mlxsw_sp_port mlxsw_sp_port, u32,
    pub cmd): *const ethtool_link_ksettings,
    pub cmd): *const ethtool_link_ksettings,
    pub autoneg): u16 local_port, u32 proto_admin, bool,
    pub p_eth_proto_oper): *mut u32,
    pub eth_proto_cap): *mut *mut u32 (ptys_proto_cap_masked_get)(u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_ports_bitmap {
    pub bitmap: *mut c_ulong,
    pub nbits: c_uint,
}

// trap_en = !!__INET_ECN_decapsulate(outer_ecn, inner_ecn, &set_ce);
// p_vid = vid;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_flood_type {
    MLXSW_SP_FLOOD_TYPE_UC,
    MLXSW_SP_FLOOD_TYPE_BC,
    MLXSW_SP_FLOOD_TYPE_MC,
// For RSP FIDs in CFF mode.
    MLXSW_SP_FLOOD_TYPE_NOT_UC,
// For NVE traffic.
    MLXSW_SP_FLOOD_TYPE_ANY,
}

// spectrum_buffers.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_hdroom_prio {
// Number of port buffer associated with this priority. This is the
// actually configured value.
//
    pub buf_idx: u8,
// Value of buf_idx deduced from the DCB ETS configuration.
    pub ets_buf_idx: u8,
// Value of buf_idx taken from the dcbnl_setbuffer configuration.
    pub set_buf_idx: u8,
    pub lossy: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_hdroom_buf {
    pub thres_cells: u32,
    pub size_cells: u32,
// Size requirement form dcbnl_setbuffer.
    pub set_size_cells: u32,
    pub lossy: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_hdroom_mode {
    MLXSW_SP_HDROOM_MODE_DCB,
    MLXSW_SP_HDROOM_MODE_TC,
}

pub const MLXSW_SP_PB_COUNT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_hdroom {
    pub mode: mlxsw_sp_hdroom_mode,
    pub prio: [mlxsw_sp_hdroom_prio; IEEE_8021Q_MAX_PRIORITIES],
    pub prios: },
    pub buf: [mlxsw_sp_hdroom_buf; MLXSW_SP_PB_COUNT],
    pub bufs: },
// Size actually configured for the internal buffer. Equal to
// reserve when internal buffer is enabled.
//
    pub size_cells: u32,
// Space reserved in the headroom for the internal buffer. Port
// buffers are not allowed to grow into this space.
//
    pub reserve_cells: u32,
    pub enable: bool,
    pub int_buf: },
    pub delay_bytes: c_int,
    pub mtu: c_int,
}

extern "C" {
    pub fn mlxsw_sp_buffers_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_buffers_fini(mlxsw_sp: *mut mlxsw_sp);
}
extern "C" {
    pub fn mlxsw_sp_port_buffers_init(mlxsw_sp_port: *mut mlxsw_sp_port) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_port_buffers_fini(mlxsw_sp_port: *mut mlxsw_sp_port);
}
extern "C" {
    pub fn mlxsw_sp_cells_bytes(mlxsw_sp: *const mlxsw_sp, cells: u32) -> u32;
}
extern "C" {
    pub fn mlxsw_sp_bytes_cells(mlxsw_sp: *const mlxsw_sp, bytes: u32) -> u32;
}
extern "C" {
    pub fn mlxsw_sp_hdroom_prios_reset_buf_idx(hdroom: *mut mlxsw_sp_hdroom);
}
extern "C" {
    pub fn mlxsw_sp_hdroom_bufs_reset_lossiness(hdroom: *mut mlxsw_sp_hdroom);
}
// spectrum_switchdev.c
extern "C" {
    pub fn mlxsw_sp_switchdev_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_switchdev_fini(mlxsw_sp: *mut mlxsw_sp);
}
// spectrum.c
extern "C" {
    pub fn mlxsw_sp_port_speed_get(mlxsw_sp_port: *mut mlxsw_sp_port, speed: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_stp_spms_state(stp_state: u8) -> mlxsw_reg_spms_state;
}
extern "C" {
    pub fn mlxsw_sp_port_vp_mode_set(mlxsw_sp_port: *mut mlxsw_sp_port, enable: bool) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_ethtype_to_sver_type(ethtype: u16, p_sver_type: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_port_vlan_destroy(mlxsw_sp_port_vlan: *mut mlxsw_sp_port_vlan);
}
extern "C" {
    pub fn mlxsw_sp_port_dev_check(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn mlxsw_sp_parsing_depth_inc(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_parsing_depth_dec(mlxsw_sp: *mut mlxsw_sp);
}
// spectrum_dcb.c

extern "C" {
    pub fn mlxsw_sp_port_dcb_init(mlxsw_sp_port: *mut mlxsw_sp_port) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_port_dcb_fini(mlxsw_sp_port: *mut mlxsw_sp_port);
}

// spectrum_router.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_l3proto {
    MLXSW_SP_L3_PROTO_IPV4,
    MLXSW_SP_L3_PROTO_IPV6,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlxsw_sp_l3addr {
    pub addr4: __be32,
    pub addr6: in6_addr,
}

extern "C" {
    pub fn mlxsw_sp_rif_index(rif: *const mlxsw_sp_rif) -> u16;
}
extern "C" {
    pub fn mlxsw_sp_router_fini(mlxsw_sp: *mut mlxsw_sp);
}
extern "C" {
    pub fn mlxsw_sp_router_port(mlxsw_sp: *const mlxsw_sp) -> u16;
}
extern "C" {
    pub fn mlxsw_sp_router_ul_rif_put(mlxsw_sp: *mut mlxsw_sp, ul_rif_index: u16);
}
// spectrum_kvdl.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_kvdl_entry_type {
    MLXSW_SP_KVDL_ENTRY_TYPE_ADJ,
    MLXSW_SP_KVDL_ENTRY_TYPE_ACTSET,
    MLXSW_SP_KVDL_ENTRY_TYPE_PBS,
    MLXSW_SP_KVDL_ENTRY_TYPE_MCRIGR,
    MLXSW_SP_KVDL_ENTRY_TYPE_IPV6_ADDRESS,
    MLXSW_SP_KVDL_ENTRY_TYPE_TNUMT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_kvdl_ops {
    pub priv_size: usize,
    pub priv): *mut *mut *mut int (init)(struct mlxsw_sp mlxsw_sp, void,
    pub priv): *mut *mut *mut void (fini)(struct mlxsw_sp mlxsw_sp, void,
    pub p_entry_index): *mut unsigned int entry_count, u32,
    pub entry_index): unsigned int entry_count, int,
    pub p_alloc_count): *mut c_uint,
    pub priv): *mut *mut *mut int (resources_register)(struct mlxsw_sp mlxsw_sp, void,
}

extern "C" {
    pub fn mlxsw_sp_kvdl_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_kvdl_fini(mlxsw_sp: *mut mlxsw_sp);
}
// spectrum1_kvdl.c
extern "C" {
    pub fn mlxsw_sp1_kvdl_resources_register(mlxsw_core: *mut mlxsw_core) -> c_int;
}
// spectrum2_kvdl.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_acl_mangle_field {
    MLXSW_SP_ACL_MANGLE_FIELD_IP_DSFIELD,
    MLXSW_SP_ACL_MANGLE_FIELD_IP_DSCP,
    MLXSW_SP_ACL_MANGLE_FIELD_IP_ECN,
    MLXSW_SP_ACL_MANGLE_FIELD_IP_SPORT,
    MLXSW_SP_ACL_MANGLE_FIELD_IP_DPORT,
    MLXSW_SP_ACL_MANGLE_FIELD_IP4_SIP,
    MLXSW_SP_ACL_MANGLE_FIELD_IP4_DIP,
    MLXSW_SP_ACL_MANGLE_FIELD_IP6_SIP_1,
    MLXSW_SP_ACL_MANGLE_FIELD_IP6_SIP_2,
    MLXSW_SP_ACL_MANGLE_FIELD_IP6_SIP_3,
    MLXSW_SP_ACL_MANGLE_FIELD_IP6_SIP_4,
    MLXSW_SP_ACL_MANGLE_FIELD_IP6_DIP_1,
    MLXSW_SP_ACL_MANGLE_FIELD_IP6_DIP_2,
    MLXSW_SP_ACL_MANGLE_FIELD_IP6_DIP_3,
    MLXSW_SP_ACL_MANGLE_FIELD_IP6_DIP_4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_rule_info {
    pub priority: c_uint,
    pub values: mlxsw_afk_element_values,
    pub act_block: *mut mlxsw_afa_block,
    pub counter_index: c_uint,
    pub policer_index: u16,
    pub src_port_range_reg_index: u8,
    pub dst_port_range_reg_index: u8,
    pub prev_val: u32,
    pub prev_field: mlxsw_sp_acl_mangle_field,
    pub ipv6: },
}

// spectrum_flow.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_flow_block {
    pub binding_list: list_head,
    pub list: list_head,
    pub min_prio: c_uint,
    pub max_prio: c_uint,
    pub mall: },
    pub ruleset_zero: *mut mlxsw_sp_acl_ruleset,
    pub mlxsw_sp: *mut mlxsw_sp,
    pub rule_count: c_uint,
    pub disable_count: c_uint,
    pub ingress_blocker_rule_count: c_uint,
    pub egress_blocker_rule_count: c_uint,
    pub ingress_binding_count: c_uint,
    pub egress_binding_count: c_uint,
    pub net: *mut net,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_flow_block_binding {
    pub list: list_head,
    pub mlxsw_sp_port: *mut mlxsw_sp_port,
    pub ingress: bool,
}

extern "C" {
    pub fn mlxsw_sp_flow_block_destroy(block: *mut mlxsw_sp_flow_block);
}
// spectrum_acl.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_acl_profile {
    MLXSW_SP_ACL_PROFILE_FLOWER,
    MLXSW_SP_ACL_PROFILE_MR,
}

extern "C" {
    pub fn mlxsw_sp_acl_ruleset_group_id(ruleset: *mut mlxsw_sp_acl_ruleset) -> u16;
}
extern "C" {
    pub fn mlxsw_sp_acl_rulei_commit(rulei: *mut mlxsw_sp_acl_rule_info) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_acl_rulei_act_continue(rulei: *mut mlxsw_sp_acl_rule_info) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_acl_rulei_act_terminate(rulei: *mut mlxsw_sp_acl_rule_info) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_acl_rulei_act_trap(rulei: *mut mlxsw_sp_acl_rule_info) -> c_int;
}
extern "C" {
    pub fn mlxsw_afa_cookie_lookup(_arg: mlxsw_sp->afa, _arg: cookie_index) -> return;
}
extern "C" {
    pub fn mlxsw_sp_acl_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_acl_fini(mlxsw_sp: *mut mlxsw_sp);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_rulei_ops {
    pub extack): *mut netlink_ext_ack,
}

// spectrum_acl_tcam.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_tcam_ops {
    pub key_type: mlxsw_reg_ptar_key_type,
    pub priv_size: usize,
    pub tcam): *mut mlxsw_sp_acl_tcam,
    pub priv): *mut *mut *mut void (fini)(struct mlxsw_sp mlxsw_sp, void,
    pub region_priv_size: usize,
    pub hints_priv): *mut c_void,
    pub region_priv): *mut *mut *mut void (region_fini)(struct mlxsw_sp mlxsw_sp, void,
    pub region): *mut mlxsw_sp_acl_tcam_region,
    pub region_priv): *mut *mut *mut void  (region_rehash_hints_get)(void,
    pub hints_priv): *mut *mut void (region_rehash_hints_put)(void,
    pub chunk_priv_size: usize,
    pub priority): c_uint,
    pub chunk_priv): *mut *mut void (chunk_fini)(void,
    pub entry_priv_size: usize,
    pub rulei): *mut mlxsw_sp_acl_rule_info,
    pub entry_priv): *mut c_void,
    pub rulei): *mut mlxsw_sp_acl_rule_info,
    pub activity): *mut bool,
}

// spectrum1_acl_tcam.c
// spectrum2_acl_tcam.c
// spectrum_acl_flex_actions.c
// spectrum_acl_flex_keys.c
// spectrum_acl_bloom_filter.c
// spectrum_matchall.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mall_ops {
    pub extack): *mut netlink_ext_ack,
    pub mall_entry): *mut mlxsw_sp_mall_entry,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_mall_action_type {
    MLXSW_SP_MALL_ACTION_TYPE_MIRROR,
    MLXSW_SP_MALL_ACTION_TYPE_SAMPLE,
    MLXSW_SP_MALL_ACTION_TYPE_TRAP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mall_mirror_entry {
    pub to_dev: *const net_device,
    pub span_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mall_trap_entry {
    pub span_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mall_sample_entry {
    pub params: mlxsw_sp_sample_params,
    pub /: *mut *mut int span_id; / Relevant for Spectrum-2 onwards.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mall_entry {
    pub list: list_head,
    pub cookie: c_ulong,
    pub priority: c_uint,
    pub type: mlxsw_sp_mall_action_type,
    pub ingress: bool,
    pub mirror: mlxsw_sp_mall_mirror_entry,
    pub trap: mlxsw_sp_mall_trap_entry,
    pub sample: mlxsw_sp_mall_sample_entry,
}

// spectrum_flower.c
// spectrum_qdisc.c
extern "C" {
    pub fn mlxsw_sp_tc_qdisc_init(mlxsw_sp_port: *mut mlxsw_sp_port) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_tc_qdisc_fini(mlxsw_sp_port: *mut mlxsw_sp_port);
}
// spectrum_fid.c
extern "C" {
    pub fn mlxsw_sp_fid_nve_ifindex(fid: *const mlxsw_sp_fid, nve_ifindex: *mut c_int) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_fid_vni(fid: *const mlxsw_sp_fid, vni: *mut __be32) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_fid_nve_flood_index_clear(fid: *mut mlxsw_sp_fid);
}
extern "C" {
    pub fn mlxsw_sp_fid_nve_flood_index_is_set(fid: *const mlxsw_sp_fid) -> bool;
}
extern "C" {
    pub fn mlxsw_sp_fid_vni_clear(fid: *mut mlxsw_sp_fid);
}
extern "C" {
    pub fn mlxsw_sp_fid_vni_is_set(fid: *const mlxsw_sp_fid) -> bool;
}
extern "C" {
    pub fn mlxsw_sp_fid_index(fid: *const mlxsw_sp_fid) -> u16;
}
extern "C" {
    pub fn mlxsw_sp_fid_type(fid: *const mlxsw_sp_fid) -> mlxsw_sp_fid_type;
}
extern "C" {
    pub fn mlxsw_sp_fid_rif_set(fid: *mut mlxsw_sp_fid, rif: *mut mlxsw_sp_rif) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_fid_rif_unset(fid: *mut mlxsw_sp_fid);
}
extern "C" {
    pub fn mlxsw_sp_fid_8021q_vid(fid: *const mlxsw_sp_fid) -> u16;
}
extern "C" {
    pub fn mlxsw_sp_fid_put(fid: *mut mlxsw_sp_fid);
}
extern "C" {
    pub fn mlxsw_sp_port_fids_init(mlxsw_sp_port: *mut mlxsw_sp_port) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_port_fids_fini(mlxsw_sp_port: *mut mlxsw_sp_port);
}
extern "C" {
    pub fn mlxsw_sp_fid_port_join_lag(mlxsw_sp_port: *const mlxsw_sp_port) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_fid_port_leave_lag(mlxsw_sp_port: *const mlxsw_sp_port);
}
// spectrum_mr.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_mr_route_prio {
    MLXSW_SP_MR_ROUTE_PRIO_SG,
    MLXSW_SP_MR_ROUTE_PRIO_STARG,
    MLXSW_SP_MR_ROUTE_PRIO_CATCHALL,
    __MLXSW_SP_MR_ROUTE_PRIO_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_mr_tcam_ops {
    pub priv_size: usize,
    pub priv): *mut *mut *mut int (init)(struct mlxsw_sp mlxsw_sp, void,
    pub priv): *mut *mut void (fini)(void,
    pub route_priv_size: usize,
    pub prio): mlxsw_sp_mr_route_prio,
    pub key): *mut mlxsw_sp_mr_route_key,
    pub afa_block): *mut mlxsw_afa_block,
}

// spectrum1_mr_tcam.c
// spectrum2_mr_tcam.c
// spectrum_nve.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_nve_params {
    pub type: mlxsw_sp_nve_type,
    pub vni: __be32,
    pub dev: *const net_device,
    pub ethertype: u16,
}

extern "C" {
    pub fn mlxsw_sp_port_nve_init(mlxsw_sp_port: *mut mlxsw_sp_port) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_port_nve_fini(mlxsw_sp_port: *mut mlxsw_sp_port);
}
extern "C" {
    pub fn mlxsw_sp_nve_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_nve_fini(mlxsw_sp: *mut mlxsw_sp);
}
// spectrum_trap.c
extern "C" {
    pub fn mlxsw_sp_devlink_traps_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_devlink_traps_fini(mlxsw_sp: *mut mlxsw_sp);
}
extern "C" {
    pub fn mlxsw_core_net(_arg: mlxsw_sp->core) -> return;
}
// spectrum_ethtool.c
// spectrum_policer.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_policer_type {
    MLXSW_SP_POLICER_TYPE_SINGLE_RATE,

    __MLXSW_SP_POLICER_TYPE_MAX,
    MLXSW_SP_POLICER_TYPE_MAX = __MLXSW_SP_POLICER_TYPE_MAX - 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_policer_params {
    pub rate: u64,
    pub burst: u64,
    pub bytes: bool,
}

extern "C" {
    pub fn mlxsw_sp_policers_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_policers_fini(mlxsw_sp: *mut mlxsw_sp);
}
extern "C" {
    pub fn mlxsw_sp_policer_resources_register(mlxsw_core: *mut mlxsw_core) -> c_int;
}
// spectrum_pgt.c
extern "C" {
    pub fn mlxsw_sp_pgt_mid_alloc(mlxsw_sp: *mut mlxsw_sp, p_mid: *mut u16) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_pgt_mid_free(mlxsw_sp: *mut mlxsw_sp, mid_base: u16);
}
extern "C" {
    pub fn mlxsw_sp_pgt_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_pgt_fini(mlxsw_sp: *mut mlxsw_sp);
}
// spectrum_port_range.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_port_range {
    pub min: u16,
    pub max: u16,
    pub /: *mut *mut u8 source:1; / Source or destination,
}

extern "C" {
    pub fn mlxsw_sp_port_range_reg_put(mlxsw_sp: *mut mlxsw_sp, prr_index: u8);
}
extern "C" {
    pub fn mlxsw_sp_port_range_init(mlxsw_sp: *mut mlxsw_sp) -> c_int;
}
extern "C" {
    pub fn mlxsw_sp_port_range_fini(mlxsw_sp: *mut mlxsw_sp);
}
