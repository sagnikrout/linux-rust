//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bond_alb.h
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
// Copyright(c) 1999 - 2004 Intel Corporation. All rights reserved.
//

// Used for division - never set
// to zero !!!
//
pub const BOND_ALB_DEFAULT_LP_INTERVAL: c_int = 1;

// learning packets to the switch
//

// ALB_TIMER_TICKS_PER_SEC)

// ALB_TIMER_TICKS_PER_SEC)

// Note that this value MUST NOT be smaller
// because the key hash table is BYTE wide !
//
pub const TLB_NULL_INDEX: c_uint = 0xffffffff;
// rlb defs
pub const RLB_HASH_TABLE_SIZE: c_int = 256;
pub const RLB_NULL_INDEX: c_uint = 0xffffffff;

pub const RLB_ARP_BURST_SIZE: c_int = 2;

// rebalance interval (5 min).
//
// RLB_PROMISC_TIMEOUT = 10 sec equals the time that the current slave is
// promiscuous after failover
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlb_client_info {
    pub transmitting: *mut *mut *mut slave tx_slave; / A pointer to slave used for,
// packets to a Client that the Hash function
// gave this entry index.
//
    pub that: *mut *mut u32 tx_bytes; / Each Client accumulates the BytesTx,
// were transmitted to it, and after each
// CallBack the LoadHistory is divided
// by the balance interval
//
    pub Bytes: *mut *mut u32 load_history; / This field contains the amount of,
// that were transmitted to this client by
// the server on the previous balance
// interval in Bps.
//
    pub assigned: *mut *mut u32 next; / The next Hash table entry index,,
// to use the same adapter for transmit.
//
    pub index,: *mut *mut u32 prev; / The previous Hash table entry,
// assigned to use the same
//
}

// -------------------------------------------------------------------------
// struct rlb_client_info contains all info related to a specific rx client
// connection. This is the Clients Hash Table entry struct.
// Note that this is not a proper hash table; if a new client's IP address
// hash collides with an existing client entry, the old entry is replaced.
//
// There is a linked list (linked by the used_next and used_prev members)
// linking all the used entries of the hash table. This allows updating
// all the clients without walking over all the unused elements of the table.
//
// There are also linked lists of entries with identical hash(ip_src). These
// allow cleaning up the table from ip_src<->mac_src associations that have
// become outdated and would cause sending out invalid ARP updates to the
// network. These are linked by the (src_next and src_prev members).
// -------------------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rlb_client_info {
    pub /: *mut *mut __be32 ip_src; / the server IP address,
    pub /: *mut *mut __be32 ip_dst; / the client IP address,
    pub /: *mut *mut u8 mac_src[ETH_ALEN]; / the server MAC address,
    pub /: *mut *mut u8 mac_dst[ETH_ALEN]; / the client MAC address,
// list of used hash table entries, starting at rx_hashtbl_used_head
    pub used_next: u32,
    pub used_prev: u32,
// ip_src based hashing
    pub /: *mut *mut u32 src_next; / next entry with same hash(ip_src),
    pub /: *mut *mut u32 src_prev; / prev entry with same hash(ip_src),
    pub /: *mut *mut u32 src_first; / first entry with hash(ip_src) == this entry's index,
    pub /: *mut *mut u8 assigned; / checking whether this entry is assigned,
    pub /: *mut *mut u8 ntt; / flag - need to transmit client info,
    pub /: *mut *mut *mut slave slave; / the slave assigned to this client,
    pub /: *mut *mut unsigned short vlan_id; / VLAN tag associated with IP address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlb_slave_info {
    pub clients: *mut *mut u32 head; / Index to the head of the bi-directional,
// hash table entries list. The entries in the list
// are the entries that were assigned to use this
// slave for transmit.
//
    pub clients: *mut *mut u32 load; / Each slave sums the loadHistory of all,
// assigned to it
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alb_bond_info {
    pub /: *mut *mut *mut tlb_client_info tx_hashtbl; / Dynamically allocated,
    pub unbalanced_load: u32,
    pub tx_rebalance_counter: core::sync::atomic::AtomicI32,
    pub lp_counter: c_int,
// -------- rlb parameters --------
    pub rlb_enabled: c_int,
    pub /: *mut *mut *mut rlb_client_info rx_hashtbl; / Receive hash table,
    pub rx_hashtbl_used_head: u32,
    pub transmit: *mut *mut u8 rx_ntt; / flag - need to,
// to all rx clients
//
    pub /: *mut *mut *mut slave rx_slave;/ last slave to xmit from,
    pub /: *mut *mut u8 primary_is_promisc; / boolean,
    pub primary: *mut *mut u32 rlb_promisc_timeout_counter;/ counts,
// promiscuity time
//
    pub rlb_update_delay_counter: u32,
    pub retries: *mut *mut u32 rlb_update_retry_counter;/ counter of,
// of client update
//
    pub the: *mut *mut u8 rlb_rebalance; / flag - indicates that,
// rx traffic should be
// rebalanced
//
}

extern "C" {
    pub fn bond_alb_initialize(bond: *mut bonding, rlb_enabled: c_int) -> c_int;
}
extern "C" {
    pub fn bond_alb_deinitialize(bond: *mut bonding);
}
extern "C" {
    pub fn bond_alb_init_slave(bond: *mut bonding, slave: *mut slave) -> c_int;
}
extern "C" {
    pub fn bond_alb_deinit_slave(bond: *mut bonding, slave: *mut slave);
}
extern "C" {
    pub fn bond_alb_handle_link_change(bond: *mut bonding, slave: *mut slave, link: c_char);
}
extern "C" {
    pub fn bond_alb_handle_active_change(bond: *mut bonding, new_slave: *mut slave);
}
extern "C" {
    pub fn bond_alb_xmit(skb: *mut sk_buff, bond_dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn bond_tlb_xmit(skb: *mut sk_buff, bond_dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn bond_alb_monitor(: *mut work_struct);
}
extern "C" {
    pub fn bond_alb_set_mac_address(bond_dev: *mut net_device, addr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bond_alb_clear_vlan(bond: *mut bonding, vlan_id: c_ushort);
}
