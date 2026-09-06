//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4vf/t4vf_common.h
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
// This file is part of the Chelsio T4 PCI-E SR-IOV Virtual Function Ethernet
// driver for Linux.
//
// Copyright (c) 2009-2010 Chelsio Communications, Inc. All rights reserved.
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

// All T4 and later chips have their PCI-E Device IDs encoded as 0xVFPP where:
//
// V  = "4" for T4; "5" for T5, etc. or
// = "a" for T4 FPGA; "b" for T4 FPGA, etc.
// F  = "0" for PF 0..3; "4".."7" for PF4..7; and "8" for VFs
// PP = adapter product designation
//
pub const CHELSIO_T4: c_uint = 0x4;
pub const CHELSIO_T5: c_uint = 0x5;
pub const CHELSIO_T6: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chip_type {
    T4_A1 = CHELSIO_CHIP_CODE(CHELSIO_T4, 1),
    T4_A2 = CHELSIO_CHIP_CODE(CHELSIO_T4, 2),
    T4_FIRST_REV	= T4_A1,
    T4_LAST_REV	= T4_A2,

    T5_A0 = CHELSIO_CHIP_CODE(CHELSIO_T5, 0),
    T5_A1 = CHELSIO_CHIP_CODE(CHELSIO_T5, 1),
    T5_FIRST_REV	= T5_A0,
    T5_LAST_REV	= T5_A1,
}

//
// The "len16" field of a Firmware Command Structure ...
//

//
// Per-VF statistics.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t4vf_port_stats {
//
// TX statistics.
//
    pub /: *mut *mut u64 tx_bcast_bytes; / broadcast,
    pub tx_bcast_frames: u64,
    pub /: *mut *mut u64 tx_mcast_bytes; / multicast,
    pub tx_mcast_frames: u64,
    pub /: *mut *mut u64 tx_ucast_bytes; / unicast,
    pub tx_ucast_frames: u64,
    pub /: *mut *mut u64 tx_drop_frames; / TX dropped frames,
    pub /: *mut *mut u64 tx_offload_bytes; / offload,
    pub tx_offload_frames: u64,
//
// RX statistics.
//
    pub /: *mut *mut u64 rx_bcast_bytes; / broadcast,
    pub rx_bcast_frames: u64,
    pub /: *mut *mut u64 rx_mcast_bytes; / multicast,
    pub rx_mcast_frames: u64,
    pub rx_ucast_bytes: u64,
    pub /: *mut *mut u64 rx_ucast_frames; / unicast,
    pub /: *mut *mut u64 rx_err_frames; / RX error frames,
}

//
// Per-"port" (Virtual Interface) link configuration ...
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps {
    FW_CAPS_UNKNOWN	= 0,	/* 0'ed out initial state */
    FW_CAPS16	= 1,	/* old Firmware: 16-bit Port Capabilities */
    FW_CAPS32	= 2,	/* new Firmware: 32-bit Port Capabilities */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_pause {
    PAUSE_RX	= 1 << 0,
    PAUSE_TX	= 1 << 1,
    PAUSE_AUTONEG	= 1 << 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_fec {
    FEC_AUTO	= 1 << 0,	/* IEEE 802.3 "automatic" */
    FEC_RS		= 1 << 1,	/* Reed-Solomon */
    FEC_BASER_RS	= 1 << 2,	/* BaseR/Reed-Solomon */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_config {
    pub /: *mut *mut fw_port_cap32_t pcaps; / link capabilities,
    pub /: *mut *mut fw_port_cap32_t acaps; / advertised capabilities,
    pub /: *mut *mut fw_port_cap32_t lpacaps; / peer advertised capabilities,
    pub /: *mut *mut fw_port_cap32_t speed_caps; / speed(s) user has requested,
    pub /: *mut *mut u32 speed; / actual link speed,
    pub /: *mut *mut cc_pause requested_fc; / flow control user has requested,
    pub /: *mut *mut cc_pause fc; / actual link flow control,
    pub /: *mut *mut cc_pause advertised_fc; / actual advertised flow control,
    pub /: *mut *mut cc_fec auto_fec; / Forward Error Correction:,
    pub /: *mut *mut cc_fec requested_fec; / "automatic" (IEEE 802.3),,
    pub /: *mut *mut cc_fec fec; / requested, and actual in use,
    pub /: *mut *mut unsigned char autoneg; / autonegotiating?,
    pub /: *mut *mut unsigned char link_ok; / link up?,
    pub /: *mut *mut unsigned char link_down_rc; / link down reason,
}

// Return true if the Link Configuration supports "High Speeds" (those greater
// than 1Gb/s).
//
// General device parameters ...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_params {
    pub /: *mut *mut u32 fwrev; / firmware version,
    pub /: *mut *mut u32 tprev; / TP Microcode Version,
}

//
// Scatter Gather Engine parameters.  These are almost all determined by the
// Physical Function Driver.  We just need to grab them to see within which
// environment we're playing ...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sge_params {
    pub /: *mut *mut u32 sge_control; / padding, boundaries, lengths, etc.,
    pub /: *mut *mut u32 sge_control2; / T5: more of the same,
    pub /: *mut *mut u32 sge_host_page_size; / PF0-7 page sizes,
    pub /: *mut *mut u32 sge_egress_queues_per_page; / PF0-7 egress queues/page,
    pub /: *mut *mut u32 sge_ingress_queues_per_page;/ PF0-7 ingress queues/page,
    pub /: *mut *mut u32 sge_vf_hps; / host page size for our vf,
    pub /: *mut *mut u32 sge_vf_eq_qpp; / egress queues/page for our VF,
    pub /: *mut *mut u32 sge_vf_iq_qpp; / ingress queues/page for our VF,
    pub /: *mut *mut u32 sge_fl_buffer_size[16]; / free list buffer sizes,
    pub /: *mut *mut u32 sge_ingress_rx_threshold; / RX counter interrupt threshold[4],
    pub /: *mut *mut u32 sge_congestion_control; / congestion thresholds, etc.,
    pub /: *mut *mut u32 sge_timer_value_0_and_1; / interrupt coalescing timer values,
    pub sge_timer_value_2_and_3: u32,
    pub sge_timer_value_4_and_5: u32,
}

//
// Vital Product Data parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpd_params {
    pub /: *mut *mut u32 cclk; / Core Clock (KHz),
}

// Stores chip specific parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_specific_params {
    pub sge_fl_db: u32,
    pub mps_tcam_size: u16,
}

//
// Global Receive Side Scaling (RSS) parameters in host-native format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rss_params {
    pub /: *mut *mut unsigned int mode; / RSS mode,
    pub /: *mut *mut unsigned int synmapen:1; / SYN Map Enable,
    pub /: *mut *mut unsigned int syn4tupenipv6:1; / enable hashing 4-tuple IPv6 SYNs,
    pub /: *mut *mut unsigned int syn2tupenipv6:1; / enable hashing 2-tuple IPv6 SYNs,
    pub /: *mut *mut unsigned int syn4tupenipv4:1; / enable hashing 4-tuple IPv4 SYNs,
    pub /: *mut *mut unsigned int syn2tupenipv4:1; / enable hashing 2-tuple IPv4 SYNs,
    pub /: *mut *mut unsigned int ofdmapen:1; / Offload Map Enable,
    pub /: *mut *mut unsigned int tnlmapen:1; / Tunnel Map Enable,
    pub /: *mut *mut unsigned int tnlalllookup:1; / Tunnel All Lookup,
    pub /: *mut *mut unsigned int hashtoeplitz:1; / use Toeplitz hash,
    pub basicvirtual: },
    pub u: },
}

//
// Virtual Interface RSS Configuration in host-native format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union rss_vi_config {
    pub /: *mut *mut u16 defaultq; / Ingress Queue ID for !tnlalllookup,
    pub /: *mut *mut unsigned int ip6fourtupen:1; / hash 4-tuple IPv6 ingress packets,
    pub /: *mut *mut unsigned int ip6twotupen:1; / hash 2-tuple IPv6 ingress packets,
    pub /: *mut *mut unsigned int ip4fourtupen:1; / hash 4-tuple IPv4 ingress packets,
    pub /: *mut *mut unsigned int ip4twotupen:1; / hash 2-tuple IPv4 ingress packets,
    pub /: *mut *mut int udpen; / hash 4-tuple UDP ingress packets,
    pub basicvirtual: },
}

//
// Maximum resources provisioned for a PCI VF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_resources {
    pub /: *mut *mut unsigned int nvi; / N virtual interfaces,
    pub /: *mut *mut unsigned int neq; / N egress Qs,
    pub /: *mut *mut unsigned int nethctrl; / N egress ETH or CTRL Qs,
    pub /: *mut *mut unsigned int niqflint; / N ingress Qs/w free list(s) & intr,
    pub /: *mut *mut unsigned int niq; / N ingress Qs,
    pub /: *mut *mut unsigned int tc; / PCI-E traffic class,
    pub /: *mut *mut unsigned int pmask; / port access rights mask,
    pub /: *mut *mut unsigned int nexactf; / N exact MPS filters,
    pub /: *mut *mut unsigned int r_caps; / read capabilities,
    pub /: *mut *mut unsigned int wx_caps; / write/execute capabilities,
}

//
// Per-"adapter" (Virtual Function) parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adapter_params {
    pub /: *mut *mut dev_params dev; / general device parameters,
    pub /: *mut *mut sge_params sge; / Scatter Gather Engine,
    pub /: *mut *mut vpd_params vpd; / Vital Product Data,
    pub /: *mut *mut rss_params rss; / Receive Side Scaling,
    pub /: *mut *mut vf_resources vfres; / Virtual Function Resource limits,
    pub /: *mut *mut arch_specific_params arch; / chip specific params,
    pub /: *mut *mut chip_type chip; / chip code,
    pub /: *mut *mut u8 nports; / # of Ethernet "ports",
    pub /: *mut *mut u8 fw_caps_support; / 32-bit Port Capabilities,
}

// Firmware Mailbox Command/Reply log.  All values are in Host-Endian format.
// The access and execute times are signed in order to accommodate negative
// error returns.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_cmd {
    pub /: *mut *mut u64 cmd[MBOX_LEN / 8]; / a Firmware Mailbox Command/Reply,
    pub /: *mut *mut u64 timestamp; / OS-dependent timestamp,
    pub /: *mut *mut u32 seqno; / sequence number,
    pub /: *mut *mut s16 access; / time (ms) to access mailbox,
    pub /: *mut *mut s16 execute; / time (ms) to execute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_cmd_log {
    pub /: *mut *mut unsigned int size; / number of entries in the log,
    pub /: *mut *mut unsigned int cursor; / next position in the log to write,
    pub /: *mut *mut u32 seqno; / next sequence number,
// variable length mailbox command log starts here
}

// Given a pointer to a Firmware Mailbox Command Log and a log entry index,
// return a pointer to the specified entry.
//

extern "C" {
    pub fn t4vf_wr_mbox_core(: *mut adapter, : *const c_void, _arg: c_int, : *mut c_void, _arg: bool) -> c_int;
}
extern "C" {
    pub fn t4vf_wr_mbox_core(_arg: adapter, _arg: cmd, _arg: size, _arg: rpl, _arg: true) -> return;
}
extern "C" {
    pub fn t4vf_wr_mbox_core(_arg: adapter, _arg: cmd, _arg: size, _arg: rpl, _arg: false) -> return;
}

//
// hash_mac_addr - return the hash value of a MAC address
// @addr: the 48-bit Ethernet MAC address
//
// Hashes a MAC address according to the hash function used by hardware
// inexact (hash) address matching.
//
extern "C" {
    pub fn t4vf_wait_dev_ready(: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4vf_port_init(: *mut adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn t4vf_fw_reset(: *mut adapter) -> c_int;
}
extern "C" {
    pub fn t4vf_set_params(: *mut adapter, int: unsigned, : *const u32, : *const u32) -> c_int;
}
extern "C" {
    pub fn t4vf_fl_pkt_align(adapter: *mut adapter) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t4_bar2_qtype {
    int t4vf_bar2_sge_qregs(struct adapter *adapter,
    unsigned int qid,
    enum t4_bar2_qtype qtype,
    u64 *pbar2_qoffset,
    unsigned int *pbar2_qid);

    unsigned int t4vf_get_pf_from_vf(struct adapter *);
    int t4vf_get_sge_params(struct adapter *);
    int t4vf_get_vpd_params(struct adapter *);
    int t4vf_get_dev_params(struct adapter *);
    int t4vf_get_rss_glb_config(struct adapter *);
    int t4vf_get_vfres(struct adapter *);

    int t4vf_read_rss_vi_config(struct adapter *, unsigned int,
    union rss_vi_config *);
    int t4vf_write_rss_vi_config(struct adapter *, unsigned int,
    union rss_vi_config *);
    int t4vf_config_rss_range(struct adapter *, unsigned int, int, int,
    const u16 *, int);

    int t4vf_alloc_vi(struct adapter *, int);
    int t4vf_free_vi(struct adapter *, int);
    int t4vf_enable_vi(struct adapter *adapter, unsigned int viid, bool rx_en,
    bool tx_en);
    int t4vf_enable_pi(struct adapter *adapter, struct port_info *pi, bool rx_en,
    bool tx_en);
    int t4vf_identify_port(struct adapter *, unsigned int, unsigned int);

    int t4vf_set_rxmode(struct adapter *, unsigned int, int, int, int, int, int,
    bool);
    int t4vf_alloc_mac_filt(struct adapter *, unsigned int, bool, unsigned int,
    const u8 **, u16 *, u64 *, bool);
    int t4vf_free_mac_filt(struct adapter *, unsigned int, unsigned int naddr,
    const u8 **, bool);
    int t4vf_change_mac(struct adapter *, unsigned int, int, const u8 *, bool);
    int t4vf_set_addr_hash(struct adapter *, unsigned int, bool, u64, bool);
    int t4vf_get_port_stats(struct adapter *, int, struct t4vf_port_stats *);

    int t4vf_iq_free(struct adapter *, unsigned int, unsigned int, unsigned int,
    unsigned int);
    int t4vf_eth_eq_free(struct adapter *, unsigned int);

    int t4vf_update_port_info(struct port_info *pi);
    int t4vf_handle_fw_rpl(struct adapter *, const __be64 *);
    int t4vf_prep_adapter(struct adapter *);
    int t4vf_get_vf_mac_acl(struct adapter *adapter, unsigned int port,
    unsigned int *naddr, u8 *addr);
    int t4vf_get_vf_vlan_acl(struct adapter *adapter);
