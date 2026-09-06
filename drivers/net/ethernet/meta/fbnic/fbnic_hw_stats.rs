//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_hw_stats.h
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_stat_counter {
    pub value: u64,
    pub old_reg_value_32: u32,
    pub old_reg_value_64: u64,
    pub u: },
    pub reported: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_hw_stat {
    pub frames: fbnic_stat_counter,
    pub bytes: fbnic_stat_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_fec_stats {
    pub uncorrectable_blocks: fbnic_stat_counter corrected_blocks,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_pcs_stats {
    pub lanes: [fbnic_stat_counter; FBNIC_PCS_MAX_LANES],
    pub SymbolErrorDuringCarrier: },
}

// Note: not updated by fbnic_get_hw_stats()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_eth_ctrl_stats {
    pub MACControlFramesTransmitted: fbnic_stat_counter,
    pub MACControlFramesReceived: fbnic_stat_counter,
}

// Note: not updated by fbnic_get_hw_stats()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_rmon_stats {
    pub undersize_pkts: fbnic_stat_counter,
    pub oversize_pkts: fbnic_stat_counter,
    pub fragments: fbnic_stat_counter,
    pub jabbers: fbnic_stat_counter,
    pub hist: [fbnic_stat_counter; ETHTOOL_RMON_HIST_MAX],
    pub hist_tx: [fbnic_stat_counter; ETHTOOL_RMON_HIST_MAX],
}

// Note: not updated by fbnic_get_hw_stats()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_pause_stats {
    pub tx_pause_frames: fbnic_stat_counter,
    pub rx_pause_frames: fbnic_stat_counter,
    pub tx_pause_storm_events: fbnic_stat_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_eth_mac_stats {
    pub FramesTransmittedOK: fbnic_stat_counter,
    pub FramesReceivedOK: fbnic_stat_counter,
    pub FrameCheckSequenceErrors: fbnic_stat_counter,
    pub AlignmentErrors: fbnic_stat_counter,
    pub OctetsTransmittedOK: fbnic_stat_counter,
    pub FramesLostDueToIntMACXmitError: fbnic_stat_counter,
    pub OctetsReceivedOK: fbnic_stat_counter,
    pub FramesLostDueToIntMACRcvError: fbnic_stat_counter,
    pub MulticastFramesXmittedOK: fbnic_stat_counter,
    pub BroadcastFramesXmittedOK: fbnic_stat_counter,
    pub MulticastFramesReceivedOK: fbnic_stat_counter,
    pub BroadcastFramesReceivedOK: fbnic_stat_counter,
    pub FrameTooLongErrors: fbnic_stat_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_phy_stats {
    pub fec: fbnic_fec_stats,
    pub pcs: fbnic_pcs_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_mac_stats {
    pub eth_mac: fbnic_eth_mac_stats,
    pub pause: fbnic_pause_stats,
    pub eth_ctrl: fbnic_eth_ctrl_stats,
    pub rmon: fbnic_rmon_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_tmi_stats {
    pub drop: fbnic_hw_stat,
    pub ptp_bad_ts: fbnic_stat_counter ptp_illegal_req, ptp_good_ts,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_tti_stats {
    pub tbi_drop: fbnic_hw_stat cm_drop, frame_drop,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_rpc_stats {
    pub unkn_ext_hdr: fbnic_stat_counter unkn_etype,,
    pub ipv6_esp: fbnic_stat_counter ipv4_frag, ipv6_frag, ipv4_esp,,
    pub ovr_size_err: fbnic_stat_counter tcp_opt_err, out_of_hdr_err,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_rxb_enqueue_stats {
    pub drbo: fbnic_hw_stat,
    pub mac_err: fbnic_stat_counter integrity_err,,
    pub frm_err: fbnic_stat_counter parser_err,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_rxb_fifo_stats {
    pub trunc: fbnic_hw_stat drop,,
    pub trans_ecn: fbnic_stat_counter trans_drop,,
    pub level: fbnic_stat_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_rxb_dequeue_stats {
    pub pbuf: fbnic_hw_stat intf,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_rxb_stats {
    pub enq: [fbnic_rxb_enqueue_stats; FBNIC_RXB_ENQUEUE_INDICES],
    pub fifo: [fbnic_rxb_fifo_stats; FBNIC_RXB_FIFO_INDICES],
    pub deq: [fbnic_rxb_dequeue_stats; FBNIC_RXB_DEQUEUE_INDICES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_hw_q_stats {
    pub rde_pkt_err: fbnic_stat_counter,
    pub rde_pkt_cq_drop: fbnic_stat_counter,
    pub rde_pkt_bdq_drop: fbnic_stat_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_pcie_stats {
    pub ob_rd_dword: fbnic_stat_counter ob_rd_tlp,,
    pub ob_wr_dword: fbnic_stat_counter ob_wr_tlp,,
    pub ob_cpl_dword: fbnic_stat_counter ob_cpl_tlp,,
    pub ob_rd_no_tag: fbnic_stat_counter,
    pub ob_rd_no_cpl_cred: fbnic_stat_counter,
    pub ob_rd_no_np_cred: fbnic_stat_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbnic_hw_stats {
    pub phy: fbnic_phy_stats,
    pub mac: fbnic_mac_stats,
    pub tmi: fbnic_tmi_stats,
    pub tti: fbnic_tti_stats,
    pub rpc: fbnic_rpc_stats,
    pub rxb: fbnic_rxb_stats,
    pub hw_q: [fbnic_hw_q_stats; FBNIC_MAX_QUEUES],
    pub pcie: fbnic_pcie_stats,
// Lock protecting the access to hw stats
    pub lock: spinlock_t,
}

extern "C" {
    pub fn fbnic_stat_rd64(fbd: *mut fbnic_dev, reg: u32, offset: u32) -> u64;
}
extern "C" {
    pub fn fbnic_reset_hw_stats(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_init_hw_stats(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_get_hw_stats32(fbd: *mut fbnic_dev);
}
extern "C" {
    pub fn fbnic_get_hw_stats(fbd: *mut fbnic_dev);
}
