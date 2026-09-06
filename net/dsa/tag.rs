//! Automatically rewritten from C Header to Rust Module
//! Source: net/dsa/tag.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_tag_driver {
    pub ops: *const dsa_device_ops,
    pub list: list_head,
    pub owner: *mut module,
}

extern "C" {
    pub fn dsa_tag_driver_put(ops: *const dsa_device_ops);
}
//
// dsa_software_untag_vlan_aware_bridge: Software untagging for VLAN-aware bridge
// @skb: Pointer to received socket buffer (packet)
// @br: Pointer to bridge upper interface of ingress port
// @vid: Parsed VID from packet
//
// The bridge can process tagged packets. Software like STP/PTP may not. The
// bridge can also process untagged packets, to the same effect as if they were
// tagged with the PVID of the ingress port. So packets tagged with the PVID of
// the bridge port must be software-untagged, to support both use cases.
//
// dsa_software_untag_vlan_unaware_bridge: Software untagging for VLAN-unaware bridge
// @skb: Pointer to received socket buffer (packet)
// @br: Pointer to bridge upper interface of ingress port
// @vid: Parsed VID from packet
//
// The bridge ignores all VLAN tags. Software like STP/PTP may not (it may run
// on the plain port, or on a VLAN upper interface). Maybe packets are coming
// to software as tagged with a driver-defined VID which is NOT equal to the
// PVID of the bridge port (since the bridge is VLAN-unaware, its configuration
// should NOT be committed to hardware). DSA needs a method for this private
// VID to be communicated by software to it, and if packets are tagged with it,
// software-untag them. Note: the private VID may be different per bridge, to
// support the FDB isolation use case.
//
// FIXME: this is currently implemented based on the broken assumption that
// the "private VID" used by the driver in VLAN-unaware mode is equal to the
// bridge PVID. It should not be, except for a coincidence; the bridge PVID is
// irrelevant to the data path in the VLAN-unaware mode. Thus, the VID that
// this function removes is wrong.
//
// All users of ds->untag_bridge_pvid should fix their drivers, if necessary,
// to make the two independent. Only then, if there still remains a need to
// strip the private VID from packets, then a new ds->ops->get_private_vid()
// API shall be introduced to communicate to DSA what this VID is, which needs
// to be stripped here.
//
// The sad part about attempting to untag from DSA is that we
// don't know, unless we check, if the skb will end up in
// the bridge's data path - br_allowed_ingress() - or not.
// For example, there might be an 8021q upper for the
// default_pvid of the bridge, which will steal VLAN-tagged traffic
// from the bridge's data path. This is a configuration that DSA
// supports because vlan_filtering is 0. In that case, we should
// definitely keep the tag, to make sure it keeps working.
//
// dsa_software_vlan_untag: Software VLAN untagging in DSA receive path
// @skb: Pointer to socket buffer (packet)
//
// Receive path method for switches which send some packets as VLAN-tagged
// towards the CPU port (generally from VLAN-aware bridge ports) even when the
// packet was not tagged on the wire. Called when ds->untag_bridge_pvid
// (legacy) or ds->untag_vlan_aware_bridge_pvid is set to true.
//
// As a side effect of this method, any VLAN tag from the skb head is moved
// to hwaccel.
//
// software untagging for standalone ports not yet necessary
// Move VLAN tag from data to hwaccel
// For switches without hardware support for DSA tagging to be able
// to support termination through the bridge.
//
// Since the bridge might learn this packet, keep the CPU port
// affinity with the port that will be used for the reply on
// xmit.
//
// If the ingress port offloads the bridge, we mark the frame as autonomously
// forwarded by hardware, so the software bridge doesn't forward in twice, back
// to us, because we already did. However, if we're in fallback mode and we do
// software bridging, we are not offloading it, therefore the dp->bridge
// pointer is not populated, and flooding needs to be done by software (we are
// effectively operating in standalone ports mode).
//
// Helper for removing DSA header tags from packets in the RX path.
// Must not be called before skb_pull(len).
// skb->data
// |
// v
// |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
// +-----------------------+-----------------------+---------------+-------+
// |    Destination MAC    |      Source MAC       |  DSA header   | EType |
// +-----------------------+-----------------------+---------------+-------+
// |               |
// <----- len ----->                               <----- len ----->
// |
// >>>>>>>   v
// >>>>>>>   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
// >>>>>>>   +-----------------------+-----------------------+-------+
// >>>>>>>   |    Destination MAC    |      Source MAC       | EType |
// +-----------------------+-----------------------+-------+
// ^
// |
// skb->data
//
// Helper for creating space for DSA header tags in TX path packets.
// Must not be called before skb_push(len).
//
// Before:
//
// <<<<<<<   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
// ^     <<<<<<<   +-----------------------+-----------------------+-------+
// |     <<<<<<<   |    Destination MAC    |      Source MAC       | EType |
// |               +-----------------------+-----------------------+-------+
// <----- len ----->
// |
// skb->data
//
// After:
//
// |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
// +-----------------------+-----------------------+---------------+-------+
// |    Destination MAC    |      Source MAC       |  DSA header   | EType |
// +-----------------------+-----------------------+---------------+-------+
// ^                                               |               |
// |                                               <----- len ----->
// skb->data
//
// On RX, eth_type_trans() on the DSA conduit pulls ETH_HLEN bytes starting from
// skb_mac_header(skb), which leaves skb->data pointing at the first byte after
// what the DSA conduit perceives as the EtherType (the beginning of the L3
// protocol). Since DSA EtherType header taggers treat the EtherType as part of
// the DSA tag itself, and the EtherType is 2 bytes in length, the DSA header
// is located 2 bytes behind skb->data. Note that EtherType in this context
// means the first 2 bytes of the DSA header, not the encapsulated EtherType
// that will become visible after the DSA header is stripped.
//
// On TX, skb->data points to the MAC header, which means that EtherType
// header taggers start exactly where the EtherType is (the EtherType is
// treated as part of the DSA header).
//
// Create 2 modaliases per tagging protocol, one to auto-load the module
// given the ID reported by get_tag_protocol(), and the other by name.
//

//
// module_dsa_tag_drivers() - Helper macro for registering DSA tag
// drivers
// @__ops_array: Array of tag driver structures
//
// Helper macro for DSA tag drivers which do not do anything special
// in module init/exit. Each module may only use this macro once, and
// calling it replaces module_init() and module_exit().
//

// Create a static structure we can build a linked list of dsa_tag
// drivers
//

//
// module_dsa_tag_driver() - Helper macro for registering a single DSA tag
// driver
// @__ops: Single tag driver structures
//
// Helper macro for DSA tag drivers which do not do anything special
// in module init/exit. Each module may only use this macro once, and
// calling it replaces module_init() and module_exit().
//

