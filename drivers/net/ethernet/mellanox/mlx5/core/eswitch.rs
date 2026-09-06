//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/eswitch.h
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


//
// Copyright (c) 2015, Mellanox Technologies, Ltd.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_mapped_obj_type {
    MLX5_MAPPED_OBJ_CHAIN,
    MLX5_MAPPED_OBJ_SAMPLE,
    MLX5_MAPPED_OBJ_INT_PORT_METADATA,
    MLX5_MAPPED_OBJ_ACT_MISS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_mapped_obj {
    pub type: mlx5_mapped_obj_type,
    pub chain: u32,
    pub act_miss_cookie: u64,
    pub group_id: u32,
    pub rate: u32,
    pub trunc_size: u32,
    pub tunnel_id: u32,
    pub sample: },
    pub int_port_metadata: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_pf_info {
    pub pf_not_exist: bool,
    pub pf_disabled: bool,
    pub num_of_vfs: u16,
    pub total_vfs: u16,
    pub host_number: u16,
    pub pf_num: u16,
}

pub const ESW_OFFLOADS_DEFAULT_NUM_GROUPS: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_ingress {
    pub acl: *mut mlx5_flow_table,
    pub allow_rule: *mut mlx5_flow_handle,
    pub allow_spoofchk_only_grp: *mut mlx5_flow_group,
    pub allow_untagged_spoofchk_grp: *mut mlx5_flow_group,
    pub allow_untagged_only_grp: *mut mlx5_flow_group,
    pub drop_grp: *mut mlx5_flow_group,
    pub drop_rule: *mut mlx5_flow_handle,
    pub drop_counter: *mut mlx5_fc,
    pub legacy: },
// Optional group to add an FTE to do internal priority
// tagging on ingress packets.
//
    pub metadata_prio_tag_grp: *mut mlx5_flow_group,
// Group to add default match-all FTE entry to tag ingress
// packet with metadata.
//
    pub metadata_allmatch_grp: *mut mlx5_flow_group,
// Optional group to add a drop all rule
    pub drop_grp: *mut mlx5_flow_group,
    pub modify_metadata: *mut mlx5_modify_hdr,
    pub modify_metadata_rule: *mut mlx5_flow_handle,
    pub drop_rule: *mut mlx5_flow_handle,
    pub offloads: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vport_egress_acl_type {
    VPORT_EGRESS_ACL_TYPE_DEFAULT,
    VPORT_EGRESS_ACL_TYPE_SHARED_FDB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vport_egress {
    pub acl: *mut mlx5_flow_table,
    pub type: vport_egress_acl_type,
    pub allowed_vlan: *mut mlx5_flow_handle,
    pub vlan_grp: *mut mlx5_flow_group,
    pub drop_grp: *mut mlx5_flow_group,
    pub drop_rule: *mut mlx5_flow_handle,
    pub drop_counter: *mut mlx5_fc,
    pub legacy: },
    pub fwd_grp: *mut mlx5_flow_group,
    pub fwd_rule: *mut mlx5_flow_handle,
    pub bounce_rules: xarray,
    pub bounce_grp: *mut mlx5_flow_group,
    pub offloads: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vport_drop_stats {
    pub rx_dropped: u64,
    pub tx_dropped: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vport_info {
    pub mac: [u8; ETH_ALEN],
    pub vlan: u16,
    pub node_guid: u64,
    pub link_state: c_int,
    pub qos: u8,
    pub 1: u8 spoofchk:,
    pub 1: u8 trusted:,
    pub 1: u8 roce_enabled:,
    pub 1: u8 mig_enabled:,
    pub 1: u8 ipsec_crypto_enabled:,
    pub 1: u8 ipsec_packet_enabled:,
}

// Vport context events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_eswitch_vport_event {
    MLX5_VPORT_UC_ADDR_CHANGE = BIT(0),
    MLX5_VPORT_MC_ADDR_CHANGE = BIT(1),
    MLX5_VPORT_PROMISC_CHANGE = BIT(3),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_devlink_port {
    pub dl_port: devlink_port,
    pub vport: *mut mlx5_vport,
}

extern "C" {
    pub fn container_of(_arg: dl_port, mlx5_devlink_port: struct, _arg: dl_port) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vport {
    pub dev: *mut mlx5_core_dev,
    pub uc_list: [hlist_head; MLX5_L2_ADDR_HASH_SIZE],
    pub mc_list: [hlist_head; MLX5_L2_ADDR_HASH_SIZE],
    pub promisc_rule: *mut mlx5_flow_handle,
    pub allmulti_rule: *mut mlx5_flow_handle,
    pub vport_change_handler: work_struct,
    pub ingress: vport_ingress,
    pub egress: vport_egress,
    pub default_metadata: u32,
    pub metadata: u32,
    pub vhca_id: c_int,
    pub /: *mut *mut bool adjacent; / delegated vhca from adjacent function,
    pub /: *mut *mut u16 parent_pci_devfn; / Adjacent parent PCI device function,
    pub /: *mut *mut u16 function_id; / Function ID of the delegated VPort,
    pub adj_info: },
    pub info: mlx5_vport_info,
// Protected by either the shared devlink (dev->shd) lock or by
// esw->state_lock. See esw_assert_qos_lock_held() for more details.
// The Vport QoS can either be disabled (sched_node is NULL) or in one
// of three states:
// 1. Regular QoS (sched_node is a vport node).
// 2. TC QoS enabled on the vport (sched_node is a TC arbiter).
// 3. TC QoS enabled on the vport's parent node
// (sched_node is a rate limit node).
// When TC is enabled in either mode, the vport owns vport TC scheduling
// nodes.
//
// Vport scheduling node.
    pub sched_node: *mut mlx5_esw_sched_node,
// Array of vport traffic class scheduling nodes.
    pub sched_nodes: *mut mlx5_esw_sched_node,
    pub qos: },
    pub vport: u16,
    pub enabled: bool,
    pub max_eqs_set: bool,
    pub pf_activated: bool,
    pub enabled_events: mlx5_eswitch_vport_event,
    pub index: c_int,
    pub dl_port: *mut mlx5_devlink_port,
    pub agg_max_tx_speed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eswitch_fdb {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct legacy_fdb {
    pub fdb: *mut mlx5_flow_table,
    pub addr_grp: *mut mlx5_flow_group,
    pub allmulti_grp: *mut mlx5_flow_group,
    pub promisc_grp: *mut mlx5_flow_group,
    pub vepa_fdb: *mut mlx5_flow_table,
    pub vepa_uplink_rule: *mut mlx5_flow_handle,
    pub vepa_star_rule: *mut mlx5_flow_handle,
    pub legacy: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct offloads_fdb {
    pub ns: *mut mlx5_flow_namespace,
    pub drop_root: *mut mlx5_flow_table,
    pub drop_root_rule: *mut mlx5_flow_handle,
    pub drop_root_fc: *mut mlx5_fc,
    pub tc_miss_table: *mut mlx5_flow_table,
    pub slow_fdb: *mut mlx5_flow_table,
    pub send_to_vport_grp: *mut mlx5_flow_group,
    pub send_to_vport_meta_grp: *mut mlx5_flow_group,
    pub peer_miss_grp: *mut mlx5_flow_group,
    pub peer_miss_rules: xarray,
    pub miss_grp: *mut mlx5_flow_group,
    pub send_to_vport_meta_rules: *mut mlx5_flow_handle,
    pub miss_rule_uni: *mut mlx5_flow_handle,
    pub miss_rule_multi: *mut mlx5_flow_handle,
    pub esw_chains_priv: *mut mlx5_fs_chains,
    pub 8): DECLARE_HASHTABLE(table,,
// Protects vports.table
    pub lock: mutex,
    pub vports: },
    pub indir: *mut mlx5_esw_indir_table,
    pub offloads: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_offload {
    pub ft_offloads_restore: *mut mlx5_flow_table,
    pub restore_group: *mut mlx5_flow_group,
    pub restore_copy_hdr_id: *mut mlx5_modify_hdr,
    pub reg_c0_obj_pool: *mut mapping_ctx,
    pub ft_offloads: *mut mlx5_flow_table,
    pub vport_rx_group: *mut mlx5_flow_group,
    pub vport_rx_drop_group: *mut mlx5_flow_group,
    pub vport_rx_drop_rule: *mut mlx5_flow_handle,
    pub ft_ipsec_tx_pol: *mut mlx5_flow_table,
    pub vport_reps: xarray,
    pub peer_flows: [list_head; MLX5_MAX_PORTS],
    pub peer_mutex: mutex,
    pub /: *mut *mut mutex encap_tbl_lock; / protects encap_tbl,
    pub 8): DECLARE_HASHTABLE(encap_tbl,,
    pub /: *mut *mut mutex decap_tbl_lock; / protects decap_tbl,
    pub 8): DECLARE_HASHTABLE(decap_tbl,,
    pub mod_hdr: mod_hdr_tbl,
    pub 8): DECLARE_HASHTABLE(termtbl_tbl,,
    pub /: *mut *mut mutex termtbl_mutex; / protects termtbl hash,
    pub vhca_map: xarray,
    pub /: *mut *mut mutex reps_lock; / protects representor load/unload/register,
    pub rep_ops: [*const mlx5_eswitch_rep_ops; NUM_REP_TYPES],
    pub inline_mode: u8,
    pub num_flows: core::sync::atomic::AtomicI64,
    pub num_block_encap: u64,
    pub num_block_mode: u64,
    pub encap: devlink_eswitch_encap_mode,
    pub vport_metadata_ida: ida,
}

// E-Switch MC FDB table hash node
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esw_mc_addr {
    pub node: l2addr_node,
    pub /: *mut *mut *mut mlx5_flow_handle uplink_rule; / Forward to uplink rule,
    pub refcnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_host_work {
    pub work: work_struct,
    pub esw: *mut mlx5_eswitch,
    pub work_gen: c_int,
    pub func: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_spf {
    pub vport_num: u16,
    pub vhca_id: u16,
    pub host_number: u16,
    pub pf_num: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_functions {
    pub nb: mlx5_nb,
    pub host_funcs_disabled: bool,
    pub num_vfs: u16,
    pub num_ec_vfs: u16,
    pub hpf_host_number: u16,
    pub hpf_pf_num: u16,
    pub has_spf_sfs: bool,
    pub spfs: *mut mlx5_esw_spf,
    pub num_spfs: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eswitch {
    pub dev: *mut mlx5_core_dev,
    pub nb: mlx5_nb,
    pub fdb_table: mlx5_eswitch_fdb,
// legacy data structures
    pub mc_table: [hlist_head; MLX5_L2_ADDR_HASH_SIZE],
    pub mc_promisc: esw_mc_addr,
// end of legacy
    pub debugfs_root: *mut dentry,
    pub work_queue: *mut workqueue_struct,
    pub vports: xarray,
    pub vhca_type_map: xarray,
    pub flags: u32,
    pub total_vports: c_int,
    pub enabled_vports: c_int,
// Synchronize between vport change events
// and async SRIOV admin state changes
//
    pub state_lock: mutex,
// Protects eswitch mode change that occurs via one or more
// user commands, i.e. sriov state change, devlink commands.
//
    pub mode_lock: rw_semaphore,
    pub user_count: core::sync::atomic::AtomicI64,
    pub work_queue_wait: wait_queue_head_t,
// QoS changes are serialized by either the shared devlink (dev->shd)
// lock or by esw->state_lock. See esw_assert_qos_lock_held() for more
// details.
//
// Initially 0, meaning no QoS users and QoS is disabled.
    pub refcnt: refcount_t,
// The root node of the hierarchy.
    pub root: *mut mlx5_esw_sched_node,
    pub qos: },
    pub br_offloads: *mut mlx5_esw_bridge_offloads,
    pub offloads: mlx5_esw_offload,
    pub last_vport_idx: u32,
    pub mode: c_int,
    pub offloads_inactive: bool,
    pub manager_vport: u16,
    pub first_host_vport: u16,
    pub num_peers: u8,
    pub esw_funcs: mlx5_esw_functions,
    pub large_group_num: u32,
    pub params: },
    pub paired: xarray,
    pub devcom: *mut mlx5_devcom_comp_dev,
    pub enabled_ipsec_vf_count: u16,
    pub eswitch_operation_in_progress: bool,
    pub generation: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn esw_offloads_disable(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn esw_offloads_enable(esw: *mut mlx5_eswitch) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_offloads_init_deferred_metadata(esw: *mut mlx5_eswitch) -> c_int;
}
extern "C" {
    pub fn esw_offloads_cleanup(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn esw_offloads_init(esw: *mut mlx5_eswitch) -> c_int;
}
extern "C" {
    pub fn mlx5_eswitch_del_send_to_vport_meta_rule(rule: *mut mlx5_flow_handle);
}
extern "C" {
    pub fn mlx5_esw_vport_match_metadata_supported(esw: *const mlx5_eswitch) -> bool;
}
extern "C" {
    pub fn mlx5_esw_match_metadata_alloc(esw: *mut mlx5_eswitch) -> u32;
}
extern "C" {
    pub fn mlx5_esw_match_metadata_free(esw: *mut mlx5_eswitch, metadata: u32);
}
extern "C" {
    pub fn mlx5_esw_qos_modify_vport_rate(esw: *mut mlx5_eswitch, vport_num: u16, rate_mbps: u32) -> c_int;
}
// E-Switch API
extern "C" {
    pub fn mlx5_eswitch_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_eswitch_cleanup(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_esw_vport_alloc(esw: *mut mlx5_eswitch, index: c_int, vport_num: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_vport_free(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport);
}

extern "C" {
    pub fn mlx5_eswitch_enable_locked(esw: *mut mlx5_eswitch, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_eswitch_enable(esw: *mut mlx5_eswitch, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_eswitch_disable_sriov(esw: *mut mlx5_eswitch, clear_vf: bool);
}
extern "C" {
    pub fn mlx5_eswitch_disable_locked(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_eswitch_disable(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_esw_offloads_devcom_cleanup(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_esw_offloads_devcom_is_ready(esw: *mut mlx5_eswitch) -> bool;
}
extern "C" {
    pub fn mlx5_eswitch_set_vepa(esw: *mut mlx5_eswitch, setting: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_eswitch_get_vepa(esw: *mut mlx5_eswitch, setting: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5_eswitch_del_send_to_vport_rule(rule: *mut mlx5_flow_handle);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_flow_match_level {
    MLX5_MATCH_NONE	= MLX5_INLINE_MODE_NONE,
    MLX5_MATCH_L2	= MLX5_INLINE_MODE_L2,
    MLX5_MATCH_L3	= MLX5_INLINE_MODE_IP,
    MLX5_MATCH_L4	= MLX5_INLINE_MODE_TCP_UDP,
}

// current maximum for flow based vport multicasting
pub const MLX5_MAX_FLOW_FWD_VPORTS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_flow_attr {
    pub in_rep: *mut mlx5_eswitch_rep,
    pub in_mdev: *mut mlx5_core_dev,
    pub counter_dev: *mut mlx5_core_dev,
    pub dest_int_port: *mut mlx5e_tc_int_port,
    pub int_port: *mut mlx5e_tc_int_port,
    pub split_count: c_int,
    pub out_count: c_int,
    pub vlan_proto: [__be16; MLX5_FS_VLAN_DEPTH],
    pub vlan_vid: [u16; MLX5_FS_VLAN_DEPTH],
    pub vlan_prio: [u8; MLX5_FS_VLAN_DEPTH],
    pub total_vlan: u8,
    pub flags: u32,
    pub vport_valid: bool,
    pub vport: u16,
    pub pkt_reformat: *mut mlx5_pkt_reformat,
    pub mdev: *mut mlx5_core_dev,
    pub termtbl: *mut mlx5_termtbl_handle,
    pub src_port_rewrite_act_id: c_int,
    pub dests: [}; MLX5_MAX_FLOW_FWD_VPORTS],
    pub rx_tun_attr: *mut mlx5_rx_tun_attr,
    pub eth: ethhdr,
    pub decap_pkt_reformat: *mut mlx5_pkt_reformat,
}

extern "C" {
    pub fn mlx5_devlink_eswitch_mode_get(devlink: *mut devlink, mode: *mut u16) -> c_int;
}
extern "C" {
    pub fn mlx5_devlink_eswitch_inline_mode_get(devlink: *mut devlink, mode: *mut u8) -> c_int;
}

extern "C" {
    pub fn mlx5_esw_pf_enable_hca(dev: *mut mlx5_core_dev, vport_num: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_pf_disable_hca(dev: *mut mlx5_core_dev, vport_num: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_host_pf_enable_hca(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_host_pf_disable_hca(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_adjacent_vhcas_setup(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_esw_adjacent_vhcas_cleanup(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_esw_destroy_esw_vport(dev: *mut mlx5_core_dev, vport: u16);
}

extern "C" {
    pub fn mlx5_core_is_ecpf_esw_manager(_arg: dev) -> return;
}
// TODO: This mlx5e_tc function shouldn't be called by eswitch
extern "C" {
    pub fn mlx5e_tc_clean_fdb_peer_flows(esw: *mut mlx5_eswitch);
}
// Each mark identifies eswitch vport type.
// MLX5_ESW_VPT_HOST_FN is used to identify both PF and VF ports using
// a single mark.
// MLX5_ESW_VPT_VF identifies a SRIOV VF vport.
// MLX5_ESW_VPT_SF identifies SF vport.
//

// The vport iterator is valid only after vport are initialized in mlx5_eswitch_init.
// Borrowed the idea from xa_for_each_marked() but with support for desired last element.
//

// This macro should only be used if EC SRIOV is enabled.
//
// Because there were no more marks available on the xarray this uses a
// for_each_range approach. The range is only valid when EC SRIOV is enabled
//

// SPF vport numbers are not contiguous, iterate via the spfs array
// and look up each vport in the xarray.
//

extern "C" {
    pub fn mlx5_eswitch_is_vf_vport(esw: *mut mlx5_eswitch, vport_num: u16) -> bool;
}
extern "C" {
    pub fn mlx5_esw_spf_vport_to_idx(esw: *mut mlx5_eswitch, vport_num: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_is_spf_vport(esw: *mut mlx5_eswitch, vport_num: u16) -> bool;
}
extern "C" {
    pub fn mlx5_eswitch_is_pf_vf_vport(esw: *mut mlx5_eswitch, vport_num: u16) -> bool;
}
extern "C" {
    pub fn mlx5_esw_is_sf_vport(esw: *mut mlx5_eswitch, vport_num: u16) -> bool;
}
extern "C" {
    pub fn mlx5_esw_funcs_changed_handler(nb: *mut notifier_block, type: c_ulong, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mlx5_eswitch_disable_pf_vf_vports(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_esw_vport_disable(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esw_vport_tbl_namespace {
    pub max_fte: c_int,
    pub max_num_groups: c_int,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vport_tbl_attr {
    pub chain: u32,
    pub prio: u16,
    pub vport: u16,
    pub vport_ns: *mut esw_vport_tbl_namespace,
}

extern "C" {
    pub fn mlx5_esw_offloads_init_pf_vf_rep(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_offloads_cleanup_pf_vf_rep(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport);
}
extern "C" {
    pub fn mlx5_esw_offloads_cleanup_sf_rep(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport);
}
extern "C" {
    pub fn mlx5_esw_offloads_load_rep(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_offloads_unload_rep(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport);
}
extern "C" {
    pub fn mlx5_eswitch_unload_sf_vport(esw: *mut mlx5_eswitch, vport_num: u16);
}
extern "C" {
    pub fn mlx5_eswitch_unload_vf_vports(esw: *mut mlx5_eswitch, num_vfs: u16);
}
extern "C" {
    pub fn mlx5_esw_offloads_sf_devlink_port_cleanup(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport);
}
extern "C" {
    pub fn mlx5_esw_offloads_devlink_port_register(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_offloads_devlink_port_unregister(vport: *mut mlx5_vport);
}
extern "C" {
    pub fn mlx5_esw_sf_max_hpf_functions(dev: *mut mlx5_core_dev, max_sfs: *mut u16, sf_base_id: *mut u16) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_get_num_spfs(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_get_hpf_host_number(dev: *mut mlx5_core_dev) -> u16;
}
extern "C" {
    pub fn mlx5_esw_get_hpf_pf_num(dev: *mut mlx5_core_dev) -> u16;
}
extern "C" {
    pub fn mlx5_esw_sf_controller_to_pfnum(dev: *mut mlx5_core_dev, controller: u32) -> u16;
}
extern "C" {
    pub fn mlx5_esw_has_spf_sfs(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_eswitch_vhca_id_to_vport(esw: *mut mlx5_eswitch, vhca_id: u16, vport_num: *mut u16) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_vport_vhca_id(esw: *mut mlx5_eswitch, vportn: u16, vhca_id: *mut u16) -> bool;
}
extern "C" {
    pub fn mlx5_esw_vhca_id_to_func_type(dev: *mut mlx5_core_dev, vhca_id: u16) -> u16;
}
//
// struct mlx5_esw_event_info - Indicates eswitch mode changed/changing.
//
// @new_mode: New mode of eswitch.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_esw_event_info {
    pub new_mode: u16,
}

extern "C" {
    pub fn mlx5_esw_hold(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_esw_release(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_esw_get(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_esw_put(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_esw_try_lock(esw: *mut mlx5_eswitch) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_lock(esw: *mut mlx5_eswitch) -> c_int;
}
extern "C" {
    pub fn mlx5_esw_unlock(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn esw_vport_change_handle_locked(vport: *mut mlx5_vport);
}
extern "C" {
    pub fn mlx5_esw_offloads_controller_valid(esw: *const mlx5_eswitch, controller: u32) -> bool;
}
extern "C" {
    pub fn mlx5_eswitch_reload_ib_reps(esw: *mut mlx5_eswitch) -> c_int;
}
extern "C" {
    pub fn mlx5_eswitch_unload_reps(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_eswitch_block_encap(dev: *mut mlx5_core_dev, from_fdb: bool) -> bool;
}
extern "C" {
    pub fn mlx5_eswitch_unblock_encap(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_eswitch_block_mode(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_eswitch_unblock_mode(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_eswitch_block_ipsec(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_eswitch_unblock_ipsec(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_esw_ipsec_vf_offload_supported(dev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_esw_host_functions_enabled(dev: *const mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5_eswitch_safe_aux_devs_remove(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_esw_reps_block(esw: *mut mlx5_eswitch);
}
extern "C" {
    pub fn mlx5_esw_reps_unblock(esw: *mut mlx5_eswitch);
}

// eswitch API stubs
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

