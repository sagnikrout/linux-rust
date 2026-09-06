//! Automatically rewritten from C Header to Rust Module
//! Source: net/batman-adv/main.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) B.A.T.M.A.N. contributors:
//
// Marek Lindner, Simon Wunderlich
//

// B.A.T.M.A.N. parameters
pub const BATADV_TQ_MAX_VALUE: c_int = 255;
pub const BATADV_THROUGHPUT_MAX_VALUE: c_uint = 0xFFFFFFFF;
pub const BATADV_JITTER: c_int = 20;

// Time To Live of broadcast messages
pub const BATADV_TTL: c_int = 50;
// maximum sequence number age of broadcast messages
pub const BATADV_BCAST_MAX_AGE: c_int = 64;
// purge originators after time in seconds if no valid packet comes in
// -> TODO: check influence on BATADV_TQ_LOCAL_WINDOW_SIZE
//

// sliding packet range of received originator messages in sequence numbers
// (should be a multiple of our word size)
//
pub const BATADV_TQ_LOCAL_WINDOW_SIZE: c_int = 64;
// milliseconds we have to keep pending tt_req
pub const BATADV_TT_REQUEST_TIMEOUT: c_int = 3000;
pub const BATADV_TQ_GLOBAL_WINDOW_SIZE: c_int = 5;
pub const BATADV_TQ_LOCAL_BIDRECT_SEND_MINIMUM: c_int = 1;
pub const BATADV_TQ_LOCAL_BIDRECT_RECV_MINIMUM: c_int = 1;
pub const BATADV_TQ_TOTAL_BIDRECT_LIMIT: c_int = 1;
// B.A.T.M.A.N. V

pub const BATADV_ELP_PROBES_PER_NODE: c_int = 2;

pub const BATADV_ELP_MAX_AGE: c_int = 64;
pub const BATADV_OGM_MAX_ORIGDIFF: c_int = 5;
pub const BATADV_OGM_MAX_AGE: c_int = 64;
// number of OGMs sent with the last tt diff
pub const BATADV_TT_OGM_APPEND_MAX: c_int = 3;
// Time in which a client can roam at most ROAMING_MAX_COUNT times in
// milliseconds
//
pub const BATADV_ROAMING_MAX_TIME: c_int = 20000;
pub const BATADV_ROAMING_MAX_COUNT: c_int = 5;
pub const BATADV_NO_FLAGS: c_int = 0;

pub const BATADV_NO_MARK: c_int = 0;
// default interface for multi interface operation. The default interface is
// used for communication which originated locally (i.e. is not forwarded)
// or where special forwarding is not desired/necessary.
//

// number of packets to send for broadcasts on different interface types
pub const BATADV_NUM_BCASTS_DEFAULT: c_int = 1;
pub const BATADV_NUM_BCASTS_WIRELESS: c_int = 3;
// length of the single packet used by the TP meter

// msecs after which an ARP_REQUEST is sent in broadcast as fallback
pub const ARP_REQ_DELAY: c_int = 250;
// numbers of originator to contact for any PUT/GET DHT operation
pub const BATADV_DAT_CANDIDATES_NUM: c_int = 3;
// BATADV_TQ_SIMILARITY_THRESHOLD - TQ points that a secondary metric can differ
// at most from the primary one in order to be still considered acceptable
//
pub const BATADV_TQ_SIMILARITY_THRESHOLD: c_int = 50;
pub const BATADV_MAX_AGGREGATION_PACKETS: c_int = 32;
pub const BATADV_MAX_AGGREGATION_BYTES: c_int = 512;
pub const BATADV_MAX_AGGREGATION_MS: c_int = 100;

pub const BATADV_BLA_WAIT_PERIODS: c_int = 3;
pub const BATADV_BLA_LOOPDETECT_PERIODS: c_int = 6;

pub const BATADV_DUPLIST_SIZE: c_int = 16;

// don't reset again within 30 seconds
pub const BATADV_RESET_PROTECTION_MS: c_int = 30000;
pub const BATADV_EXPECTED_SEQNO_RANGE: c_int = 65536;
//
// BATADV_TP_MAX_NUM - maximum number of simultaneously active tp sessions
//
pub const BATADV_TP_MAX_NUM: c_int = 5;
//
// enum batadv_mesh_state - State of a mesh interface
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum batadv_mesh_state {
// @BATADV_MESH_INACTIVE: mesh interface is not yet running
    BATADV_MESH_INACTIVE,

// @BATADV_MESH_ACTIVE: interface is up and running
    BATADV_MESH_ACTIVE,

// @BATADV_MESH_DEACTIVATING: interface is getting shut down
    BATADV_MESH_DEACTIVATING,
}

pub const BATADV_BCAST_QUEUE_LEN: c_int = 256;
pub const BATADV_BATMAN_QUEUE_LEN: c_int = 256;
//
// enum batadv_uev_action - action type of uevent
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum batadv_uev_action {
// @BATADV_UEV_ADD: gateway was selected (after none was selected)
    BATADV_UEV_ADD = 0,

//
// @BATADV_UEV_DEL: selected gateway was removed and none is selected
// anymore
//
    BATADV_UEV_DEL,

//
// @BATADV_UEV_CHANGE: a different gateway was selected as based gateway
//
    BATADV_UEV_CHANGE,

//
// @BATADV_UEV_LOOPDETECT: loop was detected which cannot be handled by
// bridge loop avoidance
//
    BATADV_UEV_LOOPDETECT,
}

//
// enum batadv_uev_type - Type of uevent
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum batadv_uev_type {
// @BATADV_UEV_GW: selected gateway was modified
    BATADV_UEV_GW = 0,

// @BATADV_UEV_BLA: bridge loop avoidance event
    BATADV_UEV_BLA,
}

pub const BATADV_GW_THRESHOLD: c_int = 50;
// Number of fragment chains for each orig_node
pub const BATADV_FRAG_BUFFER_COUNT: c_int = 8;
// Maximum number of fragments for one packet
pub const BATADV_FRAG_MAX_FRAGMENTS: c_int = 16;
// Maxumim size of each fragment
pub const BATADV_FRAG_MAX_FRAG_SIZE: c_int = 1280;
// Time to keep fragments while waiting for rest of the fragments
pub const BATADV_FRAG_TIMEOUT: c_int = 10000;
pub const BATADV_DAT_CANDIDATE_NOT_FOUND: c_int = 0;
pub const BATADV_DAT_CANDIDATE_ORIG: c_int = 1;
// Debug Messages

// Append 'batman-adv: ' before kernel messages

// Kernel headers

//
// batadv_print_vid() - return printable version of vid information
// @vid: the VLAN identifier
//
// Return: -1 when no VLAN is used, VLAN id otherwise
//
extern "C" {
    pub fn batadv_mesh_init(mesh_iface: *mut net_device) -> c_int;
}
extern "C" {
    pub fn batadv_mesh_free(mesh_iface: *mut net_device);
}
extern "C" {
    pub fn batadv_is_my_mac(bat_priv: *mut batadv_priv, addr: *const u8) -> bool;
}
extern "C" {
    pub fn batadv_max_header_len() -> c_int;
}
extern "C" {
    pub fn batadv_skb_set_priority(skb: *mut sk_buff, offset: c_int);
}
extern "C" {
    pub fn batadv_recv_handler_unregister(packet_type: u8);
}
//
// batadv_compare_eth() - Compare two not u16 aligned Ethernet addresses
// @data1: Pointer to a six-byte array containing the Ethernet address
// @data2: Pointer other six-byte array containing the Ethernet address
//
// note: can't use ether_addr_equal() as it requires aligned memory
//
// Return: true if they are the same ethernet addr
//
extern "C" {
    pub fn ether_addr_equal_unaligned(_arg: data1, _arg: data2) -> return;
}
//
// batadv_has_timed_out() - compares current time (jiffies) and timestamp +
// timeout
// @timestamp:		base value to compare with (in jiffies)
// @timeout:		added to base value before comparing (in milliseconds)
//
// Return: true if current time is after timestamp + timeout
//
extern "C" {
    pub fn time_is_before_jiffies(msecs_to_jiffies(timeout): timestamp +) -> return;
}
//
// batadv_atomic_dec_not_zero() - Decrease unless the number is 0
// @v: pointer of type atomic_t
//
// Return: non-zero if v was not 0, and zero otherwise.
//

//
// batadv_smallest_signed_int() - Returns the smallest signed integer in two's
// complement with the sizeof x
// @x: type of integer
//
// Return: smallest signed integer of type
//

//
// batadv_seq_before() - Checks if a sequence number x is a predecessor of y
// @x: potential predecessor of @y
// @y: value to compare @x against
//
// It handles overflows/underflows and can correctly check for a predecessor
// unless the variable sequence number has grown by more than
// 2**(bitwidth(x)-1)-1.
//
// This means that for a u8 with the maximum value 255, it would think:
//
// * when adding nothing - it is neither a predecessor nor a successor
// * before adding more than 127 to the starting value - it is a predecessor,
// * when adding 128 - it is neither a predecessor nor a successor,
// * after adding more than 127 to the starting value - it is a successor
//
// Return: true when x is a predecessor of y, false otherwise
//

//
// batadv_seq_after() - Checks if a sequence number x is a successor of y
// @x: potential successor of @y
// @y: value to compare @x against
//
// It handles overflows/underflows and can correctly check for a successor
// unless the variable sequence number has grown by more than
// 2**(bitwidth(x)-1)-1.
//
// This means that for a u8 with the maximum value 255, it would think:
//
// * when adding nothing - it is neither a predecessor nor a successor
// * before adding more than 127 to the starting value - it is a predecessor,
// * when adding 128 - it is neither a predecessor nor a successor,
// * after adding more than 127 to the starting value - it is a successor
//
// Return: true when x is a successor of y, false otherwise
//

//
// batadv_add_counter() - Add to per cpu statistics counter of mesh interface
// @bat_priv: the bat priv with all the mesh interface information
// @idx: counter index which should be modified
// @count: value to increase counter by
//
// Stop preemption on local cpu while incrementing the counter
//
// batadv_inc_counter() - Increase per cpu statistics counter of mesh interface
// @b: the bat priv with all the mesh interface information
// @i: counter index which should be modified
//

//
// BATADV_SKB_CB() - Get batadv_skb_cb from skb control buffer
// @__skb: skb holding the control buffer
//
// The members of the control buffer are defined in struct batadv_skb_cb in
// types.h. The macro is inspired by the similar macro TCP_SKB_CB() in tcp.h.
//
// Return: pointer to the batadv_skb_cb of the skb
//

extern "C" {
    pub fn batadv_get_vid(skb: *mut sk_buff, header_len: usize) -> c_ushort;
}
extern "C" {
    pub fn batadv_vlan_ap_isola_get(bat_priv: *mut batadv_priv, vid: c_ushort) -> bool;
}
