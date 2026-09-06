//! Automatically rewritten from C Header to Rust Module
//! Source: net/bridge/br_private.h
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
//
// Linux ethernet bridge
//
// Authors:
// Lennert Buytenhek		<buytenh@gnu.org>
//

pub const BR_HASH_BITS: c_int = 8;

pub const BR_PORT_BITS: c_int = 10;

pub const BR_MULTICAST_DEFAULT_HASH_MAX: c_int = 4096;

// Control of forwarding link local multicast
pub const BR_GROUPFWD_DEFAULT: c_int = 0;
// Don't allow forwarding of control protocols like STP, MAC PAUSE and LACP

// The Nearest Customer Bridge Group Address, 01-80-C2-00-00-[00,0B,0C,0D,0F]
pub const BR_GROUPFWD_8021AD: c_uint = 0xB801u;
// Path to usermode spanning tree program

pub type bridge_id = bridge_id;
pub type mac_addr = mac_addr;
pub type port_id = __u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_id {
    pub prio: [c_uchar; 2],
    pub addr: [c_uchar; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_addr {
    pub addr: [c_uchar; ETH_ALEN],
}

// our own querier
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_mcast_own_query {
    pub timer: timer_list,
    pub startup_sent: u32,
}

// other querier
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_mcast_other_query {
    pub timer: timer_list,
    pub delay_timer: timer_list,
}

// selected querier
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_mcast_querier {
    pub addr: br_ip,
    pub port_ifidx: c_int,
    pub seq: seqcount_spinlock_t,
}

// IGMP/MLD statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bridge_mcast_stats {
    pub mstats: br_mcast_stats,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mdb_src_entry {
    pub addr: br_ip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_mdb_config {
    pub br: *mut net_bridge,
    pub p: *mut net_bridge_port,
    pub entry: *mut br_mdb_entry,
    pub group: br_ip,
    pub src_entry: bool,
    pub filter_mode: u8,
    pub nlflags: u16,
    pub src_entries: *mut br_mdb_src_entry,
    pub num_src_entries: c_int,
    pub rt_protocol: u8,
}

// net_bridge_mcast_port must be always defined due to forwarding stubs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_mcast_port {

    pub port: *mut net_bridge_port,
    pub vlan: *mut net_bridge_vlan,
    pub ip4_own_query: bridge_mcast_own_query,
    pub ip4_mc_router_timer: timer_list,
    pub ip4_rlist: hlist_node,

    pub ip6_own_query: bridge_mcast_own_query,
    pub ip6_mc_router_timer: timer_list,
    pub ip6_rlist: hlist_node,

    pub multicast_router: c_uchar,
    pub mdb_n_entries: u32,
    pub mdb_max_entries: u32,
    pub query_queue: sk_buff_head,
    pub query_queue_work: work_struct,

}

// net_bridge_mcast must be always defined due to forwarding stubs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_mcast {

    pub br: *mut net_bridge,
    pub vlan: *mut net_bridge_vlan,
    pub multicast_last_member_count: u32,
    pub multicast_startup_query_count: u32,
    pub multicast_querier: u8,
    pub multicast_igmp_version: u8,
    pub multicast_router: u8,

    pub multicast_mld_version: u8,

    pub multicast_last_member_interval: c_ulong,
    pub multicast_membership_interval: c_ulong,
    pub multicast_querier_interval: c_ulong,
    pub multicast_query_interval: c_ulong,
    pub multicast_query_response_interval: c_ulong,
    pub multicast_startup_query_interval: c_ulong,
    pub ip4_mc_router_list: hlist_head,
    pub ip4_mc_router_timer: timer_list,
    pub ip4_other_query: bridge_mcast_other_query,
    pub ip4_own_query: bridge_mcast_own_query,
    pub ip4_querier: bridge_mcast_querier,

    pub ip6_mc_router_list: hlist_head,
    pub ip6_mc_router_timer: timer_list,
    pub ip6_other_query: bridge_mcast_other_query,
    pub ip6_own_query: bridge_mcast_own_query,
    pub ip6_querier: bridge_mcast_querier,

    pub query_queue: sk_buff_head,
    pub query_queue_work: work_struct,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_tunnel_info {
    pub tunnel_id: __be64,
    pub tunnel_dst: *mut metadata_dst __rcu,
}

// private vlan flags
//
// struct net_bridge_vlan - per-vlan entry
//
// @vnode: rhashtable member
// @tnode: rhashtable member
// @vid: VLAN id
// @flags: bridge vlan flags
// @priv_flags: private (in-kernel) bridge vlan flags
// @state: STP state (e.g. blocking, learning, forwarding)
// @stats: per-cpu VLAN statistics
// @br: if MASTER flag set, this points to a bridge struct
// @port: if MASTER flag unset, this points to a port struct
// @refcnt: if MASTER flag set, this is bumped for each port referencing it
// @brvlan: if MASTER flag unset, this points to the global per-VLAN context
// for this VLAN entry
// @tinfo: bridge tunnel info
// @br_mcast_ctx: if MASTER flag set, this is the global vlan multicast context
// @port_mcast_ctx: if MASTER flag unset, this is the per-port/vlan multicast
// context
// @msti: if MASTER flag set, this holds the VLANs MST instance
// @vlist: sorted list of VLAN entries
// @rcu: used for entry destruction
//
// This structure is shared between the global per-VLAN entries contained in
// the bridge rhashtable and the local per-port per-VLAN entries contained in
// the port's rhashtable. The union entries should be interpreted depending on
// the entry flags that are set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_vlan {
    pub vnode: rhash_head,
    pub tnode: rhash_head,
    pub vid: u16,
    pub flags: u16,
    pub priv_flags: u16,
    pub state: u8,
    pub stats: *mut pcpu_sw_netstats __percpu,
    pub br: *mut net_bridge,
    pub port: *mut net_bridge_port,
}

//
// struct net_bridge_vlan_group
//
// @vlan_hash: VLAN entry rhashtable
// @tunnel_hash: Hash table to map from tunnel key ID (e.g. VXLAN VNI) to VLAN
// @vlan_list: sorted VLAN entry list
// @num_vlans: number of total VLAN entries
// @pvid: PVID VLAN id
// @pvid_state: PVID's STP state (e.g. forwarding, learning, blocking)
//
// IMPORTANT: Be careful when checking if there're VLAN entries using list
// primitives because the bridge can have entries in its list which
// are just for global context but not for filtering, i.e. they have
// the master flag set but not the brentry flag. If you have to check
// if there're "real" entries in the bridge please test @num_vlans
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_vlan_group {
    pub vlan_hash: rhashtable,
    pub tunnel_hash: rhashtable,
    pub vlan_list: list_head,
    pub num_vlans: u16,
    pub pvid: u16,
    pub pvid_state: u8,
}

// bridge fdb flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_fdb_key {
    pub addr: mac_addr,
    pub vlan_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_fdb_entry {
    pub rhnode: rhash_head,
    pub dst: *mut net_bridge_port,
    pub key: net_bridge_fdb_key,
    pub fdb_node: hlist_node,
    pub flags: c_ulong,
// write-heavy members should not affect lookups
    pub ____cacheline_aligned_in_smp: unsigned long updated,
    pub used: c_ulong,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_fdb_flush_desc {
    pub flags: c_ulong,
    pub flags_mask: c_ulong,
    pub port_ifindex: c_int,
    pub vlan_id: u16,
}

pub const PG_SRC_ENT_LIMIT: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_mcast_gc {
    pub gc_node: hlist_node,
    pub gc): *mut *mut void (destroy)(struct net_bridge_mcast_gc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_group_src {
    pub node: hlist_node,
    pub addr: br_ip,
    pub pg: *mut net_bridge_port_group,
    pub flags: u8,
    pub src_query_rexmit_cnt: u8,
    pub timer: timer_list,
    pub br: *mut net_bridge,
    pub mcast_gc: net_bridge_mcast_gc,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_port_group_sg_key {
    pub port: *mut net_bridge_port,
    pub addr: br_ip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_port_group {
    pub next: *mut net_bridge_port_group __rcu,
    pub key: net_bridge_port_group_sg_key,
    pub __aligned(2): unsigned char eth_addr[ETH_ALEN],
    pub flags: c_uchar,
    pub filter_mode: c_uchar,
    pub grp_query_rexmit_cnt: c_uchar,
    pub rt_protocol: c_uchar,
    pub src_list: hlist_head,
    pub src_ents: c_uint,
    pub timer: timer_list,
    pub rexmit_timer: timer_list,
    pub mglist: hlist_node,
    pub eht_set_tree: rb_root,
    pub eht_host_tree: rb_root,
    pub rhnode: rhash_head,
    pub mcast_gc: net_bridge_mcast_gc,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_mdb_entry {
    pub rhnode: rhash_head,
    pub br: *mut net_bridge,
    pub ports: *mut net_bridge_port_group __rcu,
    pub addr: br_ip,
    pub host_joined: bool,
    pub timer: timer_list,
    pub mdb_node: hlist_node,
    pub mcast_gc: net_bridge_mcast_gc,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge_port {
    pub br: *mut net_bridge,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub list: list_head,
    pub flags: c_ulong,

    pub vlgrp: *mut net_bridge_vlan_group __rcu,

    pub backup_port: *mut net_bridge_port __rcu,
    pub backup_nhid: u32,
// STP
    pub priority: u8,
    pub state: u8,
    pub port_no: u16,
    pub topology_change_ack: c_uchar,
    pub config_pending: c_uchar,
    pub port_id: port_id,
    pub designated_port: port_id,
    pub designated_root: bridge_id,
    pub designated_bridge: bridge_id,
    pub path_cost: u32,
    pub designated_cost: u32,
    pub designated_age: c_ulong,
    pub forward_delay_timer: timer_list,
    pub hold_timer: timer_list,
    pub message_age_timer: timer_list,
    pub kobj: kobject,
    pub rcu: rcu_head,
    pub multicast_ctx: net_bridge_mcast_port,

    pub mcast_stats: *mut bridge_mcast_stats __percpu,
    pub multicast_eht_hosts_limit: u32,
    pub multicast_eht_hosts_cnt: u32,
    pub mglist: hlist_head,
    pub sysfs_name: [c_char; IFNAMSIZ],
    pub np: *mut netpoll,

// Identifier used to group ports that share the same switchdev
// hardware domain.
//
    pub hwdom: c_int,
    pub offload_count: c_int,
    pub ppid: netdev_phys_item_id,

    pub group_fwd_mask: u16,
    pub backup_redirected_cnt: u16,
    pub stp_xstats: bridge_stp_xstats,
}

extern "C" {
    pub fn rcu_dereference(_arg: dev->rx_handler_data) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_bridge_opts {
    BROPT_VLAN_ENABLED,
    BROPT_VLAN_STATS_ENABLED,
    BROPT_NF_CALL_IPTABLES,
    BROPT_NF_CALL_IP6TABLES,
    BROPT_NF_CALL_ARPTABLES,
    BROPT_GROUP_ADDR_SET,
    BROPT_MULTICAST_ENABLED,
    BROPT_MULTICAST_QUERY_USE_IFADDR,
    BROPT_MULTICAST_STATS_ENABLED,
    BROPT_HAS_IPV6_ADDR,
    BROPT_NEIGH_SUPPRESS_ENABLED,
    BROPT_MTU_SET_BY_USER,
    BROPT_VLAN_STATS_PER_PORT,
    BROPT_NO_LL_LEARN,
    BROPT_VLAN_BRIDGE_BINDING,
    BROPT_MCAST_VLAN_SNOOPING_ENABLED,
    BROPT_MST_ENABLED,
    BROPT_MDB_OFFLOAD_FAIL_NOTIFICATION,
    BROPT_FDB_LOCAL_VLAN_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_bridge {
    pub lock: spinlock_t,
    pub hash_lock: spinlock_t,
    pub frame_type_list: hlist_head,
    pub dev: *mut net_device,
    pub options: c_ulong,
// These fields are accessed on each packet

    pub vlan_proto: __be16,
    pub default_pvid: u16,
    pub vlgrp: *mut net_bridge_vlan_group __rcu,

    pub fdb_hash_tbl: rhashtable,
    pub port_list: list_head,

    pub fake_rtable: rtable,
    pub fake_rt6_info: rt6_info,
}

// STP

// Counter used to make sure that hardware domains get unique
// identifiers in case a bridge spans multiple switchdev instances.
//
// Bit mask of hardware domain numbers in use

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_input_skb_cb {
    pub brdev: *mut net_device,
    pub frag_max_size: u16,

    pub igmp: u8,
    pub mrouters_only:1: u8,

    pub proxyarp_replied:1: u8,
    pub src_port_isolated:1: u8,
    pub promisc:1: u8,
    pub grat_arp:1: u8,

    pub vlan_filtered:1: u8,

    pub br_netfilter_broute:1: u8,

// Set if TX data plane offloading is used towards at least one
// hardware domain.
//
    pub tx_fwd_offload:1: u8,
// The switchdev hardware domain from which this packet was received.
// If skb->offload_fwd_mark was set, then this packet was already
// forwarded by hardware to the other ports in the source hardware
// domain, otherwise it wasn't.
//
    pub src_hwdom: c_int,
// Bit mask of hardware domains towards this packet has already been
// transmitted using the TX data plane offload.
//
    pub fwd_hwdoms: c_ulong,

    pub backup_nhid: u32,
}

// called under bridge lock
// check if a VLAN entry is global
// check if a VLAN entry is used by the bridge
// check if we should use the vlan entry, returns false if it's only context
// pvid flag is not allowed in ranges
// when cur is the range end, check if:
// - it has range start flag
// - range ids are invalid (end is equal to or before start)
//
// check for required range flags

extern "C" {
    pub fn test_bit(_arg: opt, _arg: &br->options) -> return;
}
extern "C" {
    pub fn br_boolopt_get(br: *const net_bridge, opt: br_boolopt_id) -> c_int;
}
extern "C" {
    pub fn br_opt_toggle(br: *mut net_bridge, opt: net_bridge_opts, on: bool);
}

// br_device.c
extern "C" {
    pub fn br_dev_setup(dev: *mut net_device);
}
extern "C" {
    pub fn br_dev_delete(dev: *mut net_device, list: *mut list_head);
}
extern "C" {
    pub fn br_dev_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}

extern "C" {
    pub fn br_netpoll_enable(p: *mut net_bridge_port) -> c_int;
}
extern "C" {
    pub fn br_netpoll_disable(p: *mut net_bridge_port);
}

// br_fdb.c

extern "C" {
    pub fn br_fdb_init() -> c_int;
}
extern "C" {
    pub fn br_fdb_fini();
}
extern "C" {
    pub fn br_fdb_hash_init(br: *mut net_bridge) -> c_int;
}
extern "C" {
    pub fn br_fdb_hash_fini(br: *mut net_bridge);
}
extern "C" {
    pub fn br_fdb_changeaddr(p: *mut net_bridge_port, newaddr: *const c_uchar);
}
extern "C" {
    pub fn br_fdb_change_mac_address(br: *mut net_bridge, newaddr: *const u8);
}
extern "C" {
    pub fn br_fdb_cleanup(work: *mut work_struct);
}
extern "C" {
    pub fn br_fdb_sync_static(br: *mut net_bridge, p: *mut net_bridge_port) -> c_int;
}
extern "C" {
    pub fn br_fdb_unsync_static(br: *mut net_bridge, p: *mut net_bridge_port);
}
// br_forward.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_pkt_type {
    BR_PKT_UNICAST,
    BR_PKT_MULTICAST,
    BR_PKT_BROADCAST
}

extern "C" {
    pub fn br_dev_queue_push_xmit(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn br_forward_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
// return true if both source port and dest port are isolated
// br_if.c
extern "C" {
    pub fn br_port_carrier_check(p: *mut net_bridge_port, notified: *mut bool);
}
extern "C" {
    pub fn br_add_bridge(net: *mut net, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn br_del_bridge(net: *mut net, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn br_del_if(br: *mut net_bridge, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn br_mtu_auto_adjust(br: *mut net_bridge);
}
extern "C" {
    pub fn br_port_flags_change(port: *mut net_bridge_port, mask: c_ulong);
}
extern "C" {
    pub fn br_manage_promisc(br: *mut net_bridge);
}
extern "C" {
    pub fn nbp_backup_change(p: *mut net_bridge_port, backup_dev: *mut net_device) -> c_int;
}
// br_input.c
extern "C" {
    pub fn br_handle_frame_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_frame_type {
    pub type: __be16,
    pub skb): *mut sk_buff,
    pub list: hlist_node,
}

extern "C" {
    pub fn br_add_frame(br: *mut net_bridge, ft: *mut br_frame_type);
}
extern "C" {
    pub fn br_del_frame(br: *mut net_bridge, ft: *mut br_frame_type);
}
extern "C" {
    pub fn rcu_dereference(br_get_rx_handler(dev: dev->rx_handler) ==) -> return;
}
extern "C" {
    pub fn rcu_dereference_rtnl(br_get_rx_handler(dev: dev->rx_handler) ==) -> return;
}
// br_ioctl.c
extern "C" {
    pub fn br_ioctl_stub(net: *mut net, cmd: c_uint, uarg: *mut void __user) -> c_int;
}
// br_multicast.c

extern "C" {
    pub fn br_multicast_add_port(port: *mut net_bridge_port) -> c_int;
}
extern "C" {
    pub fn br_multicast_del_port(port: *mut net_bridge_port);
}
extern "C" {
    pub fn br_multicast_enable_port(port: *mut net_bridge_port);
}
extern "C" {
    pub fn br_multicast_disable_port(port: *mut net_bridge_port);
}
extern "C" {
    pub fn br_multicast_init(br: *mut net_bridge);
}
extern "C" {
    pub fn br_multicast_join_snoopers(br: *mut net_bridge);
}
extern "C" {
    pub fn br_multicast_leave_snoopers(br: *mut net_bridge);
}
extern "C" {
    pub fn br_multicast_open(br: *mut net_bridge);
}
extern "C" {
    pub fn br_multicast_stop(br: *mut net_bridge);
}
extern "C" {
    pub fn br_multicast_dev_del(br: *mut net_bridge);
}
extern "C" {
    pub fn br_multicast_set_router(brmctx: *mut net_bridge_mcast, val: c_ulong) -> c_int;
}
extern "C" {
    pub fn br_multicast_set_vlan_router(v: *mut net_bridge_vlan, mcast_router: u8) -> c_int;
}
extern "C" {
    pub fn br_multicast_set_querier(brmctx: *mut net_bridge_mcast, val: c_ulong) -> c_int;
}

extern "C" {
    pub fn br_multicast_del_port_group(p: *mut net_bridge_port_group);
}
extern "C" {
    pub fn br_mdb_hash_init(br: *mut net_bridge) -> c_int;
}
extern "C" {
    pub fn br_mdb_hash_fini(br: *mut net_bridge);
}
extern "C" {
    pub fn br_multicast_init_stats(br: *mut net_bridge) -> c_int;
}
extern "C" {
    pub fn br_multicast_uninit_stats(br: *mut net_bridge);
}
extern "C" {
    pub fn br_multicast_ngroups_get(pmctx: *const net_bridge_mcast_port) -> u32;
}
extern "C" {
    pub fn br_multicast_ngroups_set_max(pmctx: *mut net_bridge_mcast_port, max: u32);
}
extern "C" {
    pub fn br_multicast_ngroups_get_max(pmctx: *const net_bridge_mcast_port) -> u32;
}
extern "C" {
    pub fn br_multicast_host_leave(mp: *mut net_bridge_mdb_entry, notify: bool);
}
extern "C" {
    pub fn __br_multicast_del_group_src(src: *mut net_bridge_group_src);
}
extern "C" {
    pub fn br_multicast_ctx_deinit(brmctx: *mut net_bridge_mcast);
}
extern "C" {
    pub fn br_multicast_port_ctx_deinit(pmctx: *mut net_bridge_mcast_port);
}
extern "C" {
    pub fn br_multicast_update_vlan_mcast_ctx(v: *mut net_bridge_vlan, state: u8);
}
extern "C" {
    pub fn br_multicast_toggle_one_vlan(vlan: *mut net_bridge_vlan, on: bool);
}
extern "C" {
    pub fn br_multicast_toggle_global_vlan(vlan: *mut net_bridge_vlan, on: bool) -> bool;
}
extern "C" {
    pub fn br_multicast_querier_state_size() -> usize;
}
extern "C" {
    pub fn br_rports_size(brmctx: *const net_bridge_mcast) -> usize;
}

extern "C" {
    pub fn rcu_dereference(_arg: hlist_first_rcu(&brmctx->ip6_mc_router_list)) -> return;
}

extern "C" {
    pub fn rcu_dereference(_arg: hlist_first_rcu(&brmctx->ip4_mc_router_list)) -> return;
}

extern "C" {
    pub fn timer_pending(_arg: &brmctx->ip4_mc_router_timer) -> return;
}

extern "C" {
    pub fn timer_pending(_arg: &brmctx->ip6_mc_router_timer) -> return;
}

extern "C" {
    pub fn br_ip4_multicast_is_router(_arg: brmctx) -> return;
}
extern "C" {
    pub fn br_ip6_multicast_is_router(_arg: brmctx) -> return;
}

extern "C" {
    pub fn ipv4_is_zeronet(_arg: ip->src.ip4) -> return;
}

extern "C" {
    pub fn ipv6_addr_any(_arg: &ip->src.ip6) -> return;
}

// br_vlan.c

extern "C" {
    pub fn br_should_learn(p: *mut net_bridge_port, skb: *mut sk_buff, vid: *mut u16) -> bool;
}
extern "C" {
    pub fn br_vlan_delete(br: *mut net_bridge, vid: u16) -> c_int;
}
extern "C" {
    pub fn br_vlan_flush(br: *mut net_bridge);
}
extern "C" {
    pub fn br_recalculate_fwd_mask(br: *mut net_bridge);
}
extern "C" {
    pub fn br_vlan_set_stats(br: *mut net_bridge, val: c_ulong) -> c_int;
}
extern "C" {
    pub fn br_vlan_set_stats_per_port(br: *mut net_bridge, val: c_ulong) -> c_int;
}
extern "C" {
    pub fn br_vlan_init(br: *mut net_bridge) -> c_int;
}
extern "C" {
    pub fn nbp_vlan_delete(port: *mut net_bridge_port, vid: u16) -> c_int;
}
extern "C" {
    pub fn nbp_vlan_flush(port: *mut net_bridge_port);
}
extern "C" {
    pub fn nbp_vlan_init(port: *mut net_bridge_port, extack: *mut netlink_ext_ack) -> c_int;
}
extern "C" {
    pub fn nbp_get_num_vlan_infos(p: *mut net_bridge_port, filter_mask: u32) -> c_int;
}
extern "C" {
    pub fn br_vlan_port_event(p: *mut net_bridge_port, event: c_ulong);
}
extern "C" {
    pub fn br_vlan_rtnl_init() -> c_int;
}
extern "C" {
    pub fn br_vlan_rtnl_uninit();
}
extern "C" {
    pub fn rtnl_dereference(_arg: br->vlgrp) -> return;
}
extern "C" {
    pub fn rtnl_dereference(_arg: p->vlgrp) -> return;
}
extern "C" {
    pub fn rcu_dereference(_arg: br->vlgrp) -> return;
}
extern "C" {
    pub fn rcu_dereference(_arg: p->vlgrp) -> return;
}
// Since bridge now depends on 8021Q module, but the time bridge sees the
// skb, the vlan tag will always be present if the frame was tagged.
//
// vid = skb_vlan_tag_get_id(skb);
// vid = 0;

// vlan = NULL;
// changed = false;

// br_vlan_options.c

extern "C" {
    pub fn br_vlan_opts_nl_size() -> usize;
}
// vlan state manipulation helpers using *_ONCE to annotate lock-free access,
// while br_vlan_set_state() may access data protected by multicast_lock.
//
extern "C" {
    pub fn READ_ONCE(_arg: v->state) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: vg->pvid_state) -> return;
}
// learn_allow is true at ingress and false at egress

// br_mst.c

// check the port's vlan group to avoid racing with port deletion
extern "C" {
    pub fn br_mst_vlan_set_msti(v: *mut net_bridge_vlan, msti: u16) -> c_int;
}
extern "C" {
    pub fn br_mst_vlan_init_state(v: *mut net_bridge_vlan);
}
extern "C" {
    pub fn br_mst_info_size(vg: *const net_bridge_vlan_group) -> usize;
}
extern "C" {
    pub fn br_mst_uninit(br: *mut net_bridge);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_br_ops {
    pub skb): *mut *mut int (br_dev_xmit_hook)(struct sk_buff,
}

// br_netfilter.c

extern "C" {
    pub fn br_nf_core_init() -> c_int;
}
extern "C" {
    pub fn br_nf_core_fini();
}
extern "C" {
    pub fn br_netfilter_rtable_init(: *mut net_bridge);
}

// Macro flag: #define br_netfilter_rtable_init(x)

// br_stp.c
extern "C" {
    pub fn br_set_state(p: *mut net_bridge_port, state: c_uint);
}
extern "C" {
    pub fn br_init_port(p: *mut net_bridge_port);
}
extern "C" {
    pub fn br_become_designated_port(p: *mut net_bridge_port);
}
extern "C" {
    pub fn __br_set_forward_delay(br: *mut net_bridge, t: c_ulong);
}
extern "C" {
    pub fn br_set_forward_delay(br: *mut net_bridge, x: c_ulong) -> c_int;
}
extern "C" {
    pub fn br_set_hello_time(br: *mut net_bridge, x: c_ulong) -> c_int;
}
extern "C" {
    pub fn br_set_max_age(br: *mut net_bridge, x: c_ulong) -> c_int;
}
extern "C" {
    pub fn __set_ageing_time(dev: *mut net_device, t: c_ulong) -> c_int;
}
extern "C" {
    pub fn br_set_ageing_time(br: *mut net_bridge, ageing_time: clock_t) -> c_int;
}
// br_stp_if.c
extern "C" {
    pub fn br_stp_enable_bridge(br: *mut net_bridge);
}
extern "C" {
    pub fn br_stp_disable_bridge(br: *mut net_bridge);
}
extern "C" {
    pub fn br_stp_enable_port(p: *mut net_bridge_port);
}
extern "C" {
    pub fn br_stp_disable_port(p: *mut net_bridge_port);
}
extern "C" {
    pub fn br_stp_recalculate_bridge_id(br: *mut net_bridge) -> bool;
}
extern "C" {
    pub fn br_stp_change_bridge_id(br: *mut net_bridge, a: *const c_uchar);
}
extern "C" {
    pub fn br_stp_set_bridge_priority(br: *mut net_bridge, newprio: u16);
}
extern "C" {
    pub fn br_stp_set_port_priority(p: *mut net_bridge_port, newprio: c_ulong) -> c_int;
}
extern "C" {
    pub fn br_stp_set_path_cost(p: *mut net_bridge_port, path_cost: c_ulong) -> c_int;
}
extern "C" {
    pub fn br_show_bridge_id(buf: *mut c_char, id: *const bridge_id) -> isize;
}
// br_stp_bpdu.c
// br_stp_timer.c
extern "C" {
    pub fn br_stp_timer_init(br: *mut net_bridge);
}
extern "C" {
    pub fn br_stp_port_timer_init(p: *mut net_bridge_port);
}
extern "C" {
    pub fn br_timer_value(timer: *const timer_list) -> c_ulong;
}
// br.c
// br_mrp.c

extern "C" {
    pub fn br_mrp_enabled(br: *mut net_bridge) -> bool;
}
extern "C" {
    pub fn br_mrp_port_del(br: *mut net_bridge, p: *mut net_bridge_port);
}
extern "C" {
    pub fn br_mrp_fill_info(skb: *mut sk_buff, br: *mut net_bridge) -> c_int;
}

// br_cfm.c

extern "C" {
    pub fn br_cfm_created(br: *mut net_bridge) -> bool;
}
extern "C" {
    pub fn br_cfm_port_del(br: *mut net_bridge, p: *mut net_bridge_port);
}
extern "C" {
    pub fn br_cfm_config_fill_info(skb: *mut sk_buff, br: *mut net_bridge) -> c_int;
}
extern "C" {
    pub fn br_cfm_mep_count(br: *mut net_bridge, count: *mut u32) -> c_int;
}
extern "C" {
    pub fn br_cfm_peer_mep_count(br: *mut net_bridge, count: *mut u32) -> c_int;
}

// count = 0;

// br_netlink.c
extern "C" {
    pub fn br_netlink_init() -> c_int;
}
extern "C" {
    pub fn br_netlink_fini();
}
extern "C" {
    pub fn br_dellink(dev: *mut net_device, nlmsg: *mut nlmsghdr, flags: u16) -> c_int;
}

// br_sysfs_if.c
extern "C" {
    pub fn br_sysfs_addif(p: *mut net_bridge_port) -> c_int;
}
extern "C" {
    pub fn br_sysfs_renameif(p: *mut net_bridge_port) -> c_int;
}
// br_sysfs_br.c
extern "C" {
    pub fn br_sysfs_addbr(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn br_sysfs_delbr(dev: *mut net_device);
}

// br_switchdev.c

extern "C" {
    pub fn br_switchdev_frame_uses_tx_fwd_offload(skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn br_switchdev_frame_set_offload_fwd_mark(skb: *mut sk_buff);
}
extern "C" {
    pub fn br_switchdev_port_vlan_del(dev: *mut net_device, vid: u16) -> c_int;
}
extern "C" {
    pub fn br_switchdev_init(br: *mut net_bridge);
}

// br_arp_nd_proxy.c
extern "C" {
    pub fn br_recalculate_neigh_suppress_enabled(br: *mut net_bridge);
}
extern "C" {
    pub fn br_is_neigh_suppress_enabled(p: *const net_bridge_port, vid: u16) -> bool;
}
extern "C" {
    pub fn br_is_neigh_forward_grat_enabled(p: *const net_bridge_port, vid: u16) -> bool;
}
