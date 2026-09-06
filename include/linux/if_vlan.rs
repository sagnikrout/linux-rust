//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_vlan.h
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
// VLAN		An implementation of 802.1Q VLAN tagging.
//
// Authors:	Ben Greear <greearb@candelatech.com>
//

// (in addition to the Ethernet header)
//

//
// According to 802.3ac, the packet can be 4 bytes longer. --Klika Jan
//

//
// struct vlan_hdr - vlan header
// @h_vlan_TCI: priority and VLAN ID
// @h_vlan_encapsulated_proto: packet type ID or len
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_hdr {
    pub h_vlan_TCI: __be16,
    pub h_vlan_encapsulated_proto: __be16,
}

//
// struct vlan_ethhdr - vlan ethernet header (ethhdr + vlan_hdr)
// @h_dest: destination ethernet address
// @h_source: source ethernet address
// @h_vlan_proto: ethernet protocol
// @h_vlan_TCI: priority and VLAN ID
// @h_vlan_encapsulated_proto: packet type ID or len
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_ethhdr {
    pub h_dest: [c_uchar; ETH_ALEN],
    pub h_source: [c_uchar; ETH_ALEN],
    pub h_vlan_proto: __be16,
    pub h_vlan_TCI: __be16,
    pub h_vlan_encapsulated_proto: __be16,
}

// Prefer this version in TX path, instead of
// skb_reset_mac_header() + vlan_eth_hdr()
//
pub const VLAN_PRIO_MASK: c_uint = 0xe000 /* Priority Code Point */;
pub const VLAN_PRIO_SHIFT: c_int = 13;
pub const VLAN_CFI_MASK: c_uint = 0x1000 /* Canonical Format Indicator / Drop Eligible Indicator */;
pub const VLAN_VID_MASK: c_uint = 0x0fff /* VLAN Identifier */;
pub const VLAN_N_VID: c_int = 4096;
// found in socket.c
extern "C" {
    pub fn vlan_ioctl_set(: *mut *mut int (hook)(struct net, ): *mut void __user);
}

extern "C" {
    pub fn notifier_to_errno(_arg: call_netdevice_notifiers(NETDEV_CVLAN_FILTER_PUSH_INFO, _arg: dev)) -> return;
}
extern "C" {
    pub fn notifier_to_errno(_arg: call_netdevice_notifiers(NETDEV_SVLAN_FILTER_PUSH_INFO, _arg: dev)) -> return;
}
//
// struct vlan_pcpu_stats - VLAN percpu rx/tx stats
// @rx_packets: number of received packets
// @rx_bytes: number of received bytes
// @rx_multicast: number of received multicast packets
// @tx_packets: number of transmitted packets
// @tx_bytes: number of transmitted bytes
// @syncp: synchronization point for 64bit counters
// @rx_errors: number of rx errors
// @tx_dropped: number of tx drops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_pcpu_stats {
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub rx_multicast: u64_stats_t,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub syncp: u64_stats_sync,
    pub rx_errors: u32,
    pub tx_dropped: u32,
}

extern "C" {
    pub fn vlan_dev_vlan_id(dev: *const net_device) -> u16;
}
extern "C" {
    pub fn vlan_dev_vlan_proto(dev: *const net_device) -> __be16;
}
//
// struct vlan_priority_tci_mapping - vlan egress priority mappings
// @priority: skb priority
// @vlan_qos: vlan priority: (skb->priority << 13) & 0xE000
// @next: pointer to next struct
// @rcu: used for deferred freeing of mapping nodes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_priority_tci_mapping {
    pub priority: u32,
    pub vlan_qos: u16,
    pub next: *mut vlan_priority_tci_mapping __rcu,
    pub rcu: rcu_head,
}

//
// struct vlan_dev_priv - VLAN private device data
// @nr_ingress_mappings: number of ingress priority mappings
// @ingress_priority_map: ingress priority mappings
// @nr_egress_mappings: number of egress priority mappings
// @egress_priority_map: hash of egress priority mappings
// @vlan_proto: VLAN encapsulation protocol
// @vlan_id: VLAN identifier
// @flags: device flags
// @real_dev: underlying netdevice
// @dev_tracker: refcount tracker for @real_dev reference
// @real_dev_addr: address of underlying netdevice
// @dent: proc dir entry
// @vlan_pcpu_stats: ptr to percpu rx stats
// @netpoll: netpoll instance "propagated" down to @real_dev
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_dev_priv {
    pub nr_ingress_mappings: c_uint,
    pub ingress_priority_map: [u32; 8],
    pub nr_egress_mappings: c_uint,
    pub egress_priority_map: [*mut vlan_priority_tci_mapping __rcu; 16],
    pub vlan_proto: __be16,
    pub vlan_id: u16,
    pub flags: u16,
    pub real_dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub real_dev_addr: [c_uchar; ETH_ALEN],
    pub dent: *mut proc_dir_entry,
    pub vlan_pcpu_stats: *mut vlan_pcpu_stats __percpu,

    pub netpoll: *mut netpoll,

}

extern "C" {
    pub fn netdev_priv(_arg: dev) -> return;
}
// This should already be shifted to mask correctly with
// the VLAN's TCI.
//
extern "C" {
    pub fn vlan_do_receive(skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn vlan_vid_add(dev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn vlan_vid_del(dev: *mut net_device, proto: __be16, vid: u16);
}
extern "C" {
    pub fn vlan_uses_dev(dev: *const net_device) -> bool;
}

//
// eth_type_vlan - check for valid vlan ether type.
// @ethertype: ether type to check
//
// Returns: true if the ether type is a vlan ether type.
//
// __vlan_insert_inner_tag - inner VLAN tag inserting
// @skb: skbuff to tag
// @vlan_proto: VLAN encapsulation protocol
// @vlan_tci: VLAN TCI to insert
// @mac_len: MAC header length including outer vlan headers
//
// Inserts the VLAN tag into @skb as part of the payload at offset mac_len
// Does not change skb->protocol so this function can be used during receive.
//
// Returns: error if skb_cow_head fails.
//
// Move the mac header sans proto to the beginning of the new header.
// first, the ethernet type
// h_vlan_encapsulated_proto should already be populated, and
// skb->data has space for h_vlan_proto
//
// h_vlan_encapsulated_proto should not be populated, and
// skb->data has no space for h_vlan_proto
//
// now, the TCI
//
// __vlan_insert_tag - regular VLAN tag inserting
// @skb: skbuff to tag
// @vlan_proto: VLAN encapsulation protocol
// @vlan_tci: VLAN TCI to insert
//
// Inserts the VLAN tag into @skb as part of the payload
// Does not change skb->protocol so this function can be used during receive.
//
// Returns: error if skb_cow_head fails.
//
extern "C" {
    pub fn __vlan_insert_inner_tag(_arg: skb, _arg: vlan_proto, _arg: vlan_tci, _arg: ETH_HLEN) -> return;
}
//
// vlan_insert_inner_tag - inner VLAN tag inserting
// @skb: skbuff to tag
// @vlan_proto: VLAN encapsulation protocol
// @vlan_tci: VLAN TCI to insert
// @mac_len: MAC header length including outer vlan headers
//
// Inserts the VLAN tag into @skb as part of the payload at offset mac_len
// Returns a VLAN tagged skb. This might change skb->head.
//
// Following the skb_unshare() example, in case of error, the calling function
// doesn't have to worry about freeing the original skb.
//
// Does not change skb->protocol so this function can be used during receive.
//
// Return: modified @skb on success, NULL on error (@skb is freed).
//
// vlan_insert_tag - regular VLAN tag inserting
// @skb: skbuff to tag
// @vlan_proto: VLAN encapsulation protocol
// @vlan_tci: VLAN TCI to insert
//
// Inserts the VLAN tag into @skb as part of the payload
// Returns a VLAN tagged skb. This might change skb->head.
//
// Following the skb_unshare() example, in case of error, the calling function
// doesn't have to worry about freeing the original skb.
//
// Does not change skb->protocol so this function can be used during receive.
//
// Return: modified @skb on success, NULL on error (@skb is freed).
//
extern "C" {
    pub fn vlan_insert_inner_tag(_arg: skb, _arg: vlan_proto, _arg: vlan_tci, _arg: ETH_HLEN) -> return;
}
//
// vlan_insert_tag_set_proto - regular VLAN tag inserting
// @skb: skbuff to tag
// @vlan_proto: VLAN encapsulation protocol
// @vlan_tci: VLAN TCI to insert
//
// Inserts the VLAN tag into @skb as part of the payload
// Returns a VLAN tagged skb. This might change skb->head.
//
// Following the skb_unshare() example, in case of error, the calling function
// doesn't have to worry about freeing the original skb.
//
// Return: modified @skb on success, NULL on error (@skb is freed).
//
// __vlan_hwaccel_clear_tag - clear hardware accelerated VLAN info
// @skb: skbuff to clear
//
// Clears the VLAN information from @skb
//
// __vlan_hwaccel_copy_tag - copy hardware accelerated VLAN info from another skb
// @dst: skbuff to copy to
// @src: skbuff to copy from
//
// Copies VLAN information from @src to @dst (for branchless code)
//
// __vlan_hwaccel_push_inside - pushes vlan tag to the payload
// @skb: skbuff to tag
//
// Pushes the VLAN tag from @skb->vlan_tci inside to the payload.
//
// Following the skb_unshare() example, in case of error, the calling function
// doesn't have to worry about freeing the original skb.
//
// __vlan_hwaccel_put_tag - hardware accelerated VLAN inserting
// @skb: skbuff to tag
// @vlan_proto: VLAN encapsulation protocol
// @vlan_tci: VLAN TCI to insert
//
// Puts the VLAN TCI in @skb->vlan_tci and lets the device do the rest
//
// __vlan_get_tag - get the VLAN ID that is part of the payload
// @skb: skbuff to query
// @vlan_tci: buffer to store value
//
// Returns: error if the skb is not of VLAN type
//
// vlan_tci = ntohs(veth->h_vlan_TCI);
//
// __vlan_hwaccel_get_tag - get the VLAN ID that is in @skb->cb[]
// @skb: skbuff to query
// @vlan_tci: buffer to store value
//
// Returns: error if @skb->vlan_tci is not set correctly
//
// vlan_tci = skb_vlan_tag_get(skb);
// vlan_tci = 0;
//
// vlan_get_tag - get the VLAN ID from the skb
// @skb: skbuff to query
// @vlan_tci: buffer to store value
//
// Returns: error if the skb is not VLAN tagged
//
extern "C" {
    pub fn __vlan_hwaccel_get_tag(_arg: skb, _arg: vlan_tci) -> return;
}
extern "C" {
    pub fn __vlan_get_tag(_arg: skb, _arg: vlan_tci) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_type_depth {
    pub type: __be16,
    pub depth: u16,
}

//
// vlan_get_protocol_offset_inline() - get protocol EtherType.
// @skb: skbuff to query
// @type: first vlan protocol
// @mac_offset: MAC offset
// @depth: buffer to store length of eth and vlan tags in bytes
//
// Returns: the EtherType of the packet, regardless of whether it is
// vlan encapsulated (normal or hardware accelerated) or not.
//
// depth = res.depth;
// depth = skb->mac_len;
extern "C" {
    pub fn vlan_get_protocol_offset_inline(_arg: skb, _arg: type, _arg: 0, _arg: depth) -> return;
}
//
// vlan_get_protocol - get protocol EtherType.
// @skb: skbuff to query
//
// Returns: the EtherType of the packet, regardless of whether it is
// vlan encapsulated (normal or hardware accelerated) or not.
//
extern "C" {
    pub fn __vlan_get_protocol(_arg: skb, _arg: skb->protocol, _arg: NULL) -> return;
}
// This version of __vlan_get_protocol() also pulls mac header in skb->head
// depth = maclen;
// A getter for the SKB protocol field which will handle VLAN tags consistently
// whether VLAN acceleration is enabled or not.
//
// VLAN acceleration strips the VLAN header from the skb and
// moves it to skb->vlan_proto
//
extern "C" {
    pub fn vlan_get_protocol(_arg: skb) -> return;
}
//
// Was a VLAN packet, grab the encapsulated protocol, which the layer
// three protocols care about.
//
// This is a magic hack to spot IPX packets. Older Novell
// breaks the protocol design and runs IPX over 802.3 without
// an 802.2 LLC layer. We look for FFFF which isn't a used
// 802.2 SSAP/DSAP. This won't work for fault tolerant netware
// but does for the rest.
//
// Real 802.2 LLC
//
// vlan_remove_tag - remove outer VLAN tag from payload
// @skb: skbuff to remove tag from
// @vlan_tci: buffer to store value
//
// Expects the skb to contain a VLAN tag in the payload, and to have skb->data
// pointing at the MAC header.
//
// vlan_tci = ntohs(vhdr->h_vlan_TCI);
//
// skb_vlan_tagged - check if skb is vlan tagged.
// @skb: skbuff to query
//
// Returns: true if the skb is tagged, regardless of whether it is hardware
// accelerated or not.
//
// skb_vlan_tagged_multi - check if skb is vlan tagged with multiple headers.
// @skb: skbuff to query
//
// Returns: true if the skb is tagged with multiple vlan headers, regardless
// of whether it is hardware accelerated or not.
//
// vlan_features_check - drop unsafe features for skb with multiple tags.
// @skb: skbuff to query
// @features: features to be checked
//
// Returns: features without unsafe ones if the skb has multiple tags.
//
// In the case of multi-tagged packets, use a direct mask
// instead of using netdev_interesect_features(), to make
// sure that only devices supporting NETIF_F_HW_CSUM will
// have checksum offloading support.
//
// compare_vlan_header - Compare two vlan headers
// @h1: Pointer to vlan header
// @h2: Pointer to vlan header
//
// Compare two vlan headers.
//
// Please note that alignment of h1 & h2 are only guaranteed to be 16 bits.
//
// Return: 0 if equal, arbitrary non-zero value if not equal.
//

