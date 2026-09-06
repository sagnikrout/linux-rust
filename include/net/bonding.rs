//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bonding.h
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


// SPDX-License-Identifier: GPL-1.0+
//
// Bond several ethernet interfaces into a Cisco, running 'Etherchannel'.
//
// Portions are (c) Copyright 1995 Simon "Guru Aleph-Null" Janes
// NCM: Network and Communications Management, Inc.
//
// BUT, I'm the one who modified it for ethernet, so:
// (c) Copyright 1999, Thomas Davis, tadavis@lbl.gov
//

pub const BOND_MAX_ARP_TARGETS: c_int = 16;

pub const BOND_DEFAULT_MIIMON: c_int = 100;

// slave list primitives

// IMPORTANT: bond_first/last_slave can return NULL in case of an empty list

// Caller must have rcu_read_lock

//
// bond_for_each_slave - iterate over all slaves
// @bond:	the bond holding this list
// @pos:	current slave
// @iter:	list_head * iterator
//
// Caller must hold RTNL
//

// Caller must have rcu_read_lock

extern "C" {
    pub fn netpoll_tx_running(_arg: dev) -> return;
}

// Macro flag: #define block_netpoll_tx()
// Macro flag: #define unblock_netpoll_tx()

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bond_params {
    pub mode: c_int,
    pub xmit_policy: c_int,
    pub miimon: c_int,
    pub num_peer_notif: u8,
    pub missed_max: u8,
    pub arp_interval: c_int,
    pub arp_validate: c_int,
    pub arp_all_targets: c_int,
    pub fail_over_mac: c_int,
    pub updelay: c_int,
    pub downdelay: c_int,
    pub peer_notif_delay: c_int,
    pub lacp_active: c_int,
    pub lacp_fast: c_int,
    pub lacp_strict: c_int,
    pub min_links: c_uint,
    pub ad_select: c_int,
    pub primary: [c_char; IFNAMSIZ],
    pub primary_reselect: c_int,
    pub arp_targets: [__be32; BOND_MAX_ARP_TARGETS],
    pub tx_queues: c_int,
    pub all_slaves_active: c_int,
    pub resend_igmp: c_int,
    pub lp_interval: c_int,
    pub packets_per_slave: c_int,
    pub tlb_dynamic_lb: c_int,
    pub reciprocal_packets_per_slave: reciprocal_value,
    pub ad_actor_sys_prio: u16,
    pub ad_user_port_key: u16,
    pub ns_targets: [in6_addr; BOND_MAX_NS_TARGETS],
    pub coupled_control: c_int,
    pub broadcast_neighbor: c_int,
// 2 bytes of padding : see ether_addr_equal_64bits()
    pub 2]: u8 ad_actor_system[ETH_ALEN +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slave {
    pub /: *mut *mut *mut net_device dev; / first - useful for panic debug,
    pub /: *mut *mut *mut bonding bond; / our master,
    pub delay: c_int,
// all 4 in jiffies
    pub last_link_up: c_ulong,
    pub last_tx: c_ulong,
    pub last_rx: c_ulong,
    pub target_last_arp_rx: [c_ulong; BOND_MAX_ARP_TARGETS],
    pub /: *mut *mut s8 link; / one of BOND_LINK_XXXX,
    pub /: *mut *mut s8 link_new_state; / one of BOND_LINK_XXXX,
    pub /: *mut *mut should_notify_link:1; / indicates whether the link changed,
    pub duplex: u8,
    pub original_mtu: u32,
    pub link_failure_count: u32,
    pub speed: u32,
    pub queue_id: u16,
    pub perm_hwaddr: [u8; MAX_ADDR_LEN],
    pub prio: c_int,
    pub ad_info: *mut ad_slave_info,
    pub tlb_info: tlb_slave_info,

    pub np: *mut netpoll,

    pub notify_work: delayed_work,
    pub kobj: kobject,
    pub slave_stats: rtnl_link_stats64,
}

extern "C" {
    pub fn container_of(_arg: kobj, slave: struct, _arg: kobj) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bond_up_slave {
    pub count: c_uint,
    pub rcu: rcu_head,
    pub arr: [*mut slave; ],
}

//
// Link pseudo-state only used internally by monitors
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bond_ipsec {
    pub list: list_head,
    pub xs: *mut xfrm_state,
}

//
// Here are the locking policies for the two bonding locks:
// Get rcu_read_lock when reading or RTNL when writing slave list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bonding {
    pub /: *mut *mut *mut net_device dev; / first - useful for panic debug,
    pub curr_active_slave: *mut slave __rcu,
    pub current_arp_slave: *mut slave __rcu,
    pub primary_slave: *mut slave __rcu,
    pub usable_slaves: *mut bond_up_slave __rcu,
    pub all_slaves: *mut bond_up_slave __rcu,
    pub force_primary: bool,
    pub notifier_ctx: bool,
    pub /: *mut *mut s32 slave_cnt; / never change this value outside the attach/detach wrappers,
    pub ): *mut slave,
// mode_lock is used for mode-specific locking needs, currently used by:
// 3ad mode (4) - protect against running bond_3ad_unbind_slave() and
// bond_3ad_state_machine_handler() concurrently and also
// the access to the state machine shared variables.
// TLB mode (5) - to sync the use and modifications of its hash table
// ALB mode (6) - to sync the use and modifications of its hash table
//
    pub mode_lock: spinlock_t,
    pub stats_lock: spinlock_t,
    pub send_peer_notif: u32,
    pub igmp_retrans: u8,

    pub proc_entry: *mut proc_dir_entry,
    pub proc_file_name: [c_char; IFNAMSIZ],
    pub bond_list: list_head,
    pub rr_tx_counter: *mut u32 __percpu,
    pub ad_info: ad_bond_info,
    pub alb_info: alb_bond_info,
    pub params: bond_params,
    pub wq: *mut workqueue_struct,
    pub mii_work: delayed_work,
    pub arp_work: delayed_work,
    pub alb_work: delayed_work,
    pub ad_work: delayed_work,
    pub mcast_work: delayed_work,
    pub slave_arr_work: delayed_work,
    pub peer_notify_work: delayed_work,

// debugging support via debugfs
    pub debug_dir: *mut dentry,

    pub bond_stats: rtnl_link_stats64,

    pub ipsec_list: list_head,
// protecting ipsec_list
    pub ipsec_lock: mutex,

    pub xdp_prog: *mut bpf_prog,
}

extern "C" {
    pub fn bond_queue_slave_event(slave: *mut slave);
}
extern "C" {
    pub fn bond_lower_state_changed(slave: *mut slave);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bond_vlan_tag {
    pub vlan_proto: __be16,
    pub vlan_id: c_ushort,
}

//
// Returns NULL if the net_device does not belong to any of the bond's slaves
//
// Caller must hold bond lock for read
//
extern "C" {
    pub fn netdev_lower_dev_get_private(_arg: bond->dev, _arg: slave_dev) -> return;
}
extern "C" {
    pub fn BOND_MODE(bond_is_lb(bond: bond) == BOND_MODE_8023AD ||) -> return;
}
extern "C" {
    pub fn bond_mode_uses_primary(_arg: BOND_MODE(bond)) -> return;
}
extern "C" {
    pub fn netif_running(netif_carrier_ok(slave->dev: slave->dev) &&) -> return;
}
pub const BOND_PRI_RESELECT_ALWAYS: c_int = 0;
pub const BOND_PRI_RESELECT_BETTER: c_int = 1;
pub const BOND_PRI_RESELECT_FAILURE: c_int = 2;
pub const BOND_FOM_NONE: c_int = 0;
pub const BOND_FOM_ACTIVE: c_int = 1;
pub const BOND_FOM_FOLLOW: c_int = 2;
pub const BOND_ARP_TARGETS_ANY: c_int = 0;
pub const BOND_ARP_TARGETS_ALL: c_int = 1;
pub const BOND_ARP_VALIDATE_NONE: c_int = 0;

// Get the oldest arp which we've received on this slave for bond's
// arp_targets.
//
extern "C" {
    pub fn slave_oldest_target_arp_rx(_arg: bond, _arg: slave) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: slave->last_rx) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: slave->last_tx) -> return;
}

extern "C" {
    pub fn netpoll_send_skb(_arg: slave->np, _arg: skb) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bond_net {
    pub /: *mut *mut *mut net net; / Associated network namespace,
    pub dev_list: list_head,

    pub proc_dir: *mut proc_dir_entry,

    pub class_attr_bonding_masters: class_attribute,
}

extern "C" {
    pub fn bond_rcv_validate(skb: *const sk_buff, bond: *mut bonding, slave: *mut slave) -> c_int;
}
extern "C" {
    pub fn bond_dev_queue_xmit(bond: *mut bonding, skb: *mut sk_buff, slave_dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn bond_create(net: *mut net, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn bond_create_sysfs(net: *mut bond_net) -> c_int;
}
extern "C" {
    pub fn bond_destroy_sysfs(net: *mut bond_net);
}
extern "C" {
    pub fn bond_prepare_sysfs_group(bond: *mut bonding);
}
extern "C" {
    pub fn bond_sysfs_slave_add(slave: *mut slave) -> c_int;
}
extern "C" {
    pub fn bond_sysfs_slave_del(slave: *mut slave);
}
extern "C" {
    pub fn bond_xdp_set_features(bond_dev: *mut net_device);
}
extern "C" {
    pub fn bond_release(bond_dev: *mut net_device, slave_dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn bond_xmit_hash(bond: *mut bonding, skb: *mut sk_buff) -> u32;
}
extern "C" {
    pub fn bond_set_carrier(bond: *mut bonding) -> c_int;
}
extern "C" {
    pub fn bond_select_active_slave(bond: *mut bonding);
}
extern "C" {
    pub fn bond_change_active_slave(bond: *mut bonding, new_active: *mut slave);
}
extern "C" {
    pub fn bond_create_debugfs();
}
extern "C" {
    pub fn bond_destroy_debugfs();
}
extern "C" {
    pub fn bond_debug_register(bond: *mut bonding);
}
extern "C" {
    pub fn bond_debug_unregister(bond: *mut bonding);
}
extern "C" {
    pub fn bond_debug_reregister(bond: *mut bonding);
}
extern "C" {
    pub fn __bond_xdp_check(mode: c_int, xmit_policy: c_int) -> bool;
}
extern "C" {
    pub fn bond_xdp_check(bond: *mut bonding, mode: c_int) -> bool;
}
extern "C" {
    pub fn bond_setup(bond_dev: *mut net_device);
}
extern "C" {
    pub fn bond_get_num_tx_queues() -> c_uint;
}
extern "C" {
    pub fn bond_netlink_init() -> c_int;
}
extern "C" {
    pub fn bond_netlink_fini();
}
extern "C" {
    pub fn bond_update_slave_arr(bond: *mut bonding, skipslave: *mut slave) -> c_int;
}
extern "C" {
    pub fn bond_slave_arr_work_rearm(bond: *mut bonding, delay: c_ulong);
}
extern "C" {
    pub fn bond_peer_notify_work_rearm(bond: *mut bonding, delay: c_ulong);
}
extern "C" {
    pub fn bond_work_init_all(bond: *mut bonding);
}
extern "C" {
    pub fn bond_work_cancel_all(bond: *mut bonding);
}

extern "C" {
    pub fn bond_create_proc_entry(bond: *mut bonding);
}
extern "C" {
    pub fn bond_remove_proc_entry(bond: *mut bonding);
}
extern "C" {
    pub fn bond_create_proc_dir(bn: *mut bond_net);
}
extern "C" {
    pub fn bond_destroy_proc_dir(bn: *mut bond_net);
}

// Caller must hold rcu_read_lock() for read
// Check if the ip is present in arp ip list, or first free slot if ip == 0
// Returns -1 if not found, index if found
//

// exported from bond_main.c
// exported from bond_netlink.c
// exported from bond_sysfs_slave.c
// exported from bond_3ad.c
