//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/flower/main.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017-2018 Netronome Systems, Inc.
pub const __NFP_FLOWER_H__: c_int = 1;

pub const NFP_FLOWER_MASK_ENTRY_RS: c_int = 256;
pub const NFP_FLOWER_MASK_ELEMENT_RS: c_int = 1;
pub const NFP_FLOWER_MASK_HASH_BITS: c_int = 10;
pub const NFP_FLOWER_KEY_MAX_LW: c_int = 32;

pub const NFP_FL_MASK_REUSE_TIME_NS: c_int = 40000;
pub const NFP_FL_MASK_ID_LOCATION: c_int = 1;
// Extra features bitmap.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_mask_id {
    pub mask_id_free_list: circ_buf,
    pub last_used: *mut ktime_t,
    pub init_unallocated: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_stats_id {
    pub free_list: circ_buf,
    pub init_unalloc: u32,
    pub repeated_em_count: u8,
}

//
// struct nfp_fl_tunnel_offloads - priv data for tunnel offloads
// @offloaded_macs:	Hashtable of the offloaded MAC addresses
// @ipv4_off_list:	List of IPv4 addresses to offload
// @ipv6_off_list:	List of IPv6 addresses to offload
// @ipv4_off_lock:	Lock for the IPv4 address list
// @ipv6_off_lock:	Lock for the IPv6 address list
// @mac_off_ids:	IDA to manage id assignment for offloaded MACs
// @neigh_nb:		Notifier to monitor neighbour state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_tunnel_offloads {
    pub offloaded_macs: rhashtable,
    pub ipv4_off_list: list_head,
    pub ipv6_off_list: list_head,
    pub ipv4_off_lock: mutex,
    pub ipv6_off_lock: mutex,
    pub mac_off_ids: ida,
    pub neigh_nb: notifier_block,
}

//
// struct nfp_tun_neigh_lag - lag info
// @lag_version:	lag version
// @lag_instance:	lag instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_tun_neigh_lag {
    pub lag_version: [u8; 3],
    pub lag_instance: u8,
}

//
// struct nfp_tun_neigh - basic neighbour data
// @dst_addr:	Destination MAC address
// @src_addr:	Source MAC address
// @port_id:	NFP port to output packet on - associated with source IPv4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_tun_neigh {
    pub dst_addr: [u8; ETH_ALEN],
    pub src_addr: [u8; ETH_ALEN],
    pub port_id: __be32,
}

//
// struct nfp_tun_neigh_ext - extended neighbour data
// @vlan_tpid:	VLAN_TPID match field
// @vlan_tci:	VLAN_TCI match field
// @host_ctx:	Host context ID to be saved here
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_tun_neigh_ext {
    pub vlan_tpid: __be16,
    pub vlan_tci: __be16,
    pub host_ctx: __be32,
}

//
// struct nfp_tun_neigh_v4 - neighbour/route entry on the NFP for IPv4
// @dst_ipv4:	Destination IPv4 address
// @src_ipv4:	Source IPv4 address
// @common:	Neighbour/route common info
// @ext:	Neighbour/route extended info
// @lag:	lag port info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_tun_neigh_v4 {
    pub dst_ipv4: __be32,
    pub src_ipv4: __be32,
    pub common: nfp_tun_neigh,
    pub ext: nfp_tun_neigh_ext,
    pub lag: nfp_tun_neigh_lag,
}

//
// struct nfp_tun_neigh_v6 - neighbour/route entry on the NFP for IPv6
// @dst_ipv6:	Destination IPv6 address
// @src_ipv6:	Source IPv6 address
// @common:	Neighbour/route common info
// @ext:	Neighbour/route extended info
// @lag:	lag port info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_tun_neigh_v6 {
    pub dst_ipv6: in6_addr,
    pub src_ipv6: in6_addr,
    pub common: nfp_tun_neigh,
    pub ext: nfp_tun_neigh_ext,
    pub lag: nfp_tun_neigh_lag,
}

//
// struct nfp_neigh_entry
// @neigh_cookie:	Cookie for hashtable lookup
// @ht_node:		rhash_head entry for hashtable
// @list_head:		Needed as member of linked_nn_entries list
// @payload:		The neighbour info payload
// @flow:		Linked flow rule
// @is_ipv6:		Flag to indicate if payload is ipv6 or ipv4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_neigh_entry {
    pub neigh_cookie: c_ulong,
    pub ht_node: rhash_head,
    pub list_head: list_head,
    pub payload: *mut c_char,
    pub flow: *mut nfp_predt_entry,
    pub is_ipv6: bool,
}

//
// struct nfp_predt_entry
// @list_head:		List head to attach to predt_list
// @flow_pay:		Direct link to flow_payload
// @nn_list:		List of linked nfp_neigh_entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_predt_entry {
    pub list_head: list_head,
    pub flow_pay: *mut nfp_fl_payload,
    pub nn_list: list_head,
}

//
// struct nfp_mtu_conf - manage MTU setting
// @portnum:		NFP port number of repr with requested MTU change
// @requested_val:	MTU value requested for repr
// @ack:		Received ack that MTU has been correctly set
// @wait_q:		Wait queue for MTU acknowledgements
// @lock:		Lock for setting/reading MTU variables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_mtu_conf {
    pub portnum: u32,
    pub requested_val: c_uint,
    pub ack: bool,
    pub wait_q: wait_queue_head_t,
    pub lock: spinlock_t,
}

//
// struct nfp_fl_lag - Flower APP priv data for link aggregation
// @work:		Work queue for writing configs to the HW
// @lock:		Lock to protect lag_group_list
// @group_list:		List of all master/slave groups offloaded
// @ida_handle:		IDA to handle group ids
// @pkt_num:		Incremented for each config packet sent
// @batch_ver:		Incremented for each batch of config packets
// @global_inst:	Instance allocator for groups
// @rst_cfg:		Marker to reset HW LAG config
// @retrans_skbs:	Cmsgs that could not be processed by HW and require
// retransmission
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_lag {
    pub work: delayed_work,
    pub lock: mutex,
    pub group_list: list_head,
    pub ida_handle: ida,
    pub pkt_num: c_uint,
    pub batch_ver: c_uint,
    pub global_inst: u8,
    pub rst_cfg: bool,
    pub retrans_skbs: sk_buff_head,
}

//
// struct nfp_fl_internal_ports - Flower APP priv data for additional ports
// @port_ids:	Assignment of ids to any additional ports
// @lock:	Lock for extra ports list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_internal_ports {
    pub port_ids: idr,
    pub lock: spinlock_t,
}

//
// struct nfp_flower_priv - Flower APP per-vNIC priv data
// @app:		Back pointer to app
// @nn:			Pointer to vNIC
// @mask_id_seed:	Seed used for mask hash table
// @flower_version:	HW version of flower
// @flower_ext_feats:	Bitmap of extra features the HW supports
// @flower_en_feats:	Bitmap of features enabled by HW
// @stats_ids:		List of free stats ids
// @mask_ids:		List of free mask ids
// @mask_table:		Hash table used to store masks
// @stats_ring_size:	Maximum number of allowed stats ids
// @flow_table:		Hash table used to store flower rules
// @stats:		Stored stats updates for flower rules
// @stats_lock:		Lock for flower rule stats updates
// @stats_ctx_table:	Hash table to map stats contexts to its flow rule
// @cmsg_work:		Workqueue for control messages processing
// @cmsg_skbs_high:	List of higher priority skbs for control message
// processing
// @cmsg_skbs_low:	List of lower priority skbs for control message
// processing
// @tun:		Tunnel offload data
// @reify_replies:	atomically stores the number of replies received
// from firmware for repr reify
// @reify_wait_queue:	wait queue for repr reify response counting
// @mtu_conf:		Configuration of repr MTU value
// @nfp_lag:		Link aggregation data block
// @indr_block_cb_priv:	List of priv data passed to indirect block cbs
// @non_repr_priv:	List of offloaded non-repr ports and their priv data
// @active_mem_unit:	Current active memory unit for flower rules
// @total_mem_units:	Total number of available memory units for flower rules
// @internal_ports:	Internal port ids used in offloaded rules
// @qos_stats_work:	Workqueue for qos stats processing
// @qos_rate_limiters:	Current active qos rate limiters
// @qos_stats_lock:	Lock on qos stats updates
// @meter_stats_lock:   Lock on meter stats updates
// @meter_table:	Hash table used to store the meter table
// @pre_tun_rule_cnt:	Number of pre-tunnel rules offloaded
// @merge_table:	Hash table to store merged flows
// @ct_zone_table:	Hash table used to store the different zones
// @ct_zone_wc:		Special zone entry for wildcarded zone matches
// @ct_map_table:	Hash table used to referennce ct flows
// @predt_list:		List to keep track of decap pretun flows
// @neigh_table:	Table to keep track of neighbor entries
// @predt_lock:		Lock to serialise predt/neigh table updates
// @nfp_fl_lock:	Lock to protect the flow offload operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_priv {
    pub app: *mut nfp_app,
    pub nn: *mut nfp_net,
    pub mask_id_seed: u32,
    pub flower_version: u64,
    pub flower_ext_feats: u64,
    pub flower_en_feats: u8,
    pub stats_ids: nfp_fl_stats_id,
    pub mask_ids: nfp_fl_mask_id,
    pub NFP_FLOWER_MASK_HASH_BITS): DECLARE_HASHTABLE(mask_table,,
    pub stats_ring_size: u32,
    pub flow_table: rhashtable,
    pub stats: *mut nfp_fl_stats,
    pub /: *mut *mut spinlock_t stats_lock; / lock stats,
    pub stats_ctx_table: rhashtable,
    pub cmsg_work: work_struct,
    pub cmsg_skbs_high: sk_buff_head,
    pub cmsg_skbs_low: sk_buff_head,
    pub tun: nfp_fl_tunnel_offloads,
    pub reify_replies: core::sync::atomic::AtomicI32,
    pub reify_wait_queue: wait_queue_head_t,
    pub mtu_conf: nfp_mtu_conf,
    pub nfp_lag: nfp_fl_lag,
    pub indr_block_cb_priv: list_head,
    pub non_repr_priv: list_head,
    pub active_mem_unit: c_uint,
    pub total_mem_units: c_uint,
    pub internal_ports: nfp_fl_internal_ports,
    pub qos_stats_work: delayed_work,
    pub qos_rate_limiters: c_uint,
    pub /: *mut *mut spinlock_t qos_stats_lock; / Protect the qos stats,
    pub /: *mut *mut mutex meter_stats_lock; / Protect the meter stats,
    pub meter_table: rhashtable,
    pub pre_tun_rule_cnt: c_int,
    pub merge_table: rhashtable,
    pub ct_zone_table: rhashtable,
    pub ct_zone_wc: *mut nfp_fl_ct_zone_entry,
    pub ct_map_table: rhashtable,
    pub predt_list: list_head,
    pub neigh_table: rhashtable,
    pub /: *mut *mut spinlock_t predt_lock; / Lock to serialise predt/neigh table updates,
    pub /: *mut *mut mutex nfp_fl_lock; / Protect the flow operation,
}

//
// struct nfp_fl_qos - Flower APP priv data for quality of service
// @netdev_port_id:	NFP port number of repr with qos info
// @curr_stats:		Currently stored stats updates for qos info
// @prev_stats:		Previously stored updates for qos info
// @last_update:	Stored time when last stats were updated
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_qos {
    pub netdev_port_id: u32,
    pub curr_stats: nfp_stat_pair,
    pub prev_stats: nfp_stat_pair,
    pub last_update: u64,
}

//
// struct nfp_flower_repr_priv - Flower APP per-repr priv data
// @nfp_repr:		Back pointer to nfp_repr
// @lag_port_flags:	Extended port flags to record lag state of repr
// @mac_offloaded:	Flag indicating a MAC address is offloaded for repr
// @offloaded_mac_addr:	MAC address that has been offloaded for repr
// @block_shared:	Flag indicating if offload applies to shared blocks
// @mac_list:		List entry of reprs that share the same offloaded MAC
// @qos_table:		Stored info on filters implementing qos
// @on_bridge:		Indicates if the repr is attached to a bridge
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_repr_priv {
    pub nfp_repr: *mut nfp_repr,
    pub lag_port_flags: c_ulong,
    pub mac_offloaded: bool,
    pub offloaded_mac_addr: [u8; ETH_ALEN],
    pub block_shared: bool,
    pub mac_list: list_head,
    pub qos_table: nfp_fl_qos,
    pub on_bridge: bool,
}

//
// struct nfp_flower_non_repr_priv - Priv data for non-repr offloaded ports
// @list:		List entry of offloaded reprs
// @netdev:		Pointer to non-repr net_device
// @ref_count:		Number of references held for this priv data
// @mac_offloaded:	Flag indicating a MAC address is offloaded for device
// @offloaded_mac_addr:	MAC address that has been offloaded for dev
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_flower_non_repr_priv {
    pub list: list_head,
    pub netdev: *mut net_device,
    pub ref_count: c_int,
    pub mac_offloaded: bool,
    pub offloaded_mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_key_ls {
    pub key_layer_two: u32,
    pub key_layer: u8,
    pub key_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_rule_metadata {
    pub key_len: u8,
    pub mask_len: u8,
    pub act_len: u8,
    pub flags: u8,
    pub host_ctx_id: __be32,
    pub __packed: __be64 host_cookie,
    pub __packed: __be64 flow_version,
    pub shortcut: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_stats {
    pub pkts: u64,
    pub bytes: u64,
    pub used: u64,
}

//
// struct nfp_ipv6_addr_entry - cached IPv6 addresses
// @ipv6_addr:	IP address
// @ref_count:	number of rules currently using this IP
// @list:	list pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_ipv6_addr_entry {
    pub ipv6_addr: in6_addr,
    pub ref_count: c_int,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_payload {
    pub meta: nfp_fl_rule_metadata,
    pub tc_flower_cookie: c_ulong,
    pub fl_node: rhash_head,
    pub rcu: rcu_head,
    pub nfp_tun_ipv4_addr: __be32,
    pub nfp_tun_ipv6: *mut nfp_ipv6_addr_entry,
    pub ingress_dev: *mut net_device,
    pub unmasked_data: *mut c_char,
    pub mask_data: *mut c_char,
    pub action_data: *mut c_char,
    pub linked_flows: list_head,
    pub in_hw: bool,
    pub predt: *mut nfp_predt_entry,
    pub dev: *mut net_device,
    pub vlan_tpid: __be16,
    pub vlan_tci: __be16,
    pub port_idx: __be16,
    pub loc_mac: [u8; ETH_ALEN],
    pub rem_mac: [u8; ETH_ALEN],
    pub is_ipv6: bool,
    pub pre_tun_rule: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_payload_link {
// A link contains a pointer to a merge flow and an associated sub_flow.
// Each merge flow will feature in 2 links to its underlying sub_flows.
// A sub_flow will have at least 1 link to a merge flow or more if it
// has been used to create multiple merge flows.
//
// For a merge flow, 'linked_flows' in its nfp_fl_payload struct lists
// all links to sub_flows (sub_flow.flow) via merge.list.
// For a sub_flow, 'linked_flows' gives all links to merge flows it has
// formed (merge_flow.flow) via sub_flow.list.
//
    pub list: list_head,
    pub flow: *mut nfp_fl_payload,
    pub sub_flow: } merge_flow,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_merge_info {
    pub parent_ctx: u64,
    pub ht_node: rhash_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_stats_frame {
    pub stats_con_id: __be32,
    pub pkt_count: __be32,
    pub byte_count: __be64,
    pub stats_cookie: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_meter_stats_entry {
    pub pkts: u64,
    pub bytes: u64,
    pub drops: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_meter_entry {
    pub ht_node: rhash_head,
    pub meter_id: u32,
    pub bps: bool,
    pub rate: u32,
    pub burst: u32,
    pub used: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_meter_stats {
    pub update: u64,
    pub curr: nfp_meter_stats_entry,
    pub prev: nfp_meter_stats_entry,
    pub stats: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_meter_op {
    NFP_METER_ADD,
    NFP_METER_DEL,
}

// The address of the merged flow acts as its cookie.
// Cookies supplied to us by TC flower are also addresses to allocated
// memory and thus this scheme should not generate any collisions.
//
extern "C" {
    pub fn netif_is_ovs_master(_arg: netdev) -> return;
}
extern "C" {
    pub fn nfp_flower_metadata_cleanup(app: *mut nfp_app);
}
extern "C" {
    pub fn nfp_flower_rx_flow_stats(app: *mut nfp_app, skb: *mut sk_buff);
}
extern "C" {
    pub fn nfp_tunnel_config_start(app: *mut nfp_app) -> c_int;
}
extern "C" {
    pub fn nfp_tunnel_config_stop(app: *mut nfp_app);
}
extern "C" {
    pub fn nfp_tunnel_del_ipv4_off(app: *mut nfp_app, ipv4: __be32);
}
extern "C" {
    pub fn nfp_tunnel_add_ipv4_off(app: *mut nfp_app, ipv4: __be32);
}
extern "C" {
    pub fn nfp_tunnel_request_route_v4(app: *mut nfp_app, skb: *mut sk_buff);
}
extern "C" {
    pub fn nfp_tunnel_request_route_v6(app: *mut nfp_app, skb: *mut sk_buff);
}
extern "C" {
    pub fn nfp_tunnel_keep_alive(app: *mut nfp_app, skb: *mut sk_buff);
}
extern "C" {
    pub fn nfp_tunnel_keep_alive_v6(app: *mut nfp_app, skb: *mut sk_buff);
}
extern "C" {
    pub fn nfp_flower_lag_init(lag: *mut nfp_fl_lag);
}
extern "C" {
    pub fn nfp_flower_lag_cleanup(lag: *mut nfp_fl_lag);
}
extern "C" {
    pub fn nfp_flower_lag_reset(lag: *mut nfp_fl_lag) -> c_int;
}
extern "C" {
    pub fn nfp_flower_lag_unprocessed_msg(app: *mut nfp_app, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn nfp_flower_qos_init(app: *mut nfp_app);
}
extern "C" {
    pub fn nfp_flower_qos_cleanup(app: *mut nfp_app);
}
extern "C" {
    pub fn nfp_flower_stats_rlim_reply(app: *mut nfp_app, skb: *mut sk_buff);
}
extern "C" {
    pub fn nfp_flower_setup_indr_tc_release(cb_priv: *mut c_void);
}
extern "C" {
    pub fn nfp_init_meter_table(app: *mut nfp_app) -> c_int;
}
extern "C" {
    pub fn nfp_flower_stats_meter_request_all(fl_priv: *mut nfp_flower_priv);
}
extern "C" {
    pub fn nfp_act_stats_reply(app: *mut nfp_app, pmsg: *mut c_void);
}
