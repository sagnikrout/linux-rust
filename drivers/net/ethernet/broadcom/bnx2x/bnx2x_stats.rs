//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_stats.h
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


// bnx2x_stats.h: QLogic Everest network driver.
//
// Copyright (c) 2007-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Maintained by: Ariel Elior <ariel.elior@qlogic.com>
// Written by: Eliezer Tamir
// Based on code from Michael Chan's bnx2 driver
// UDP CSUM errata workaround by Arik Gendelman
// Slowpath and fastpath rework by Vladislav Zolotarov
// Statistics and Link management by Yitchak Gertner
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nig_stats {
    pub brb_discard: u32,
    pub brb_packet: u32,
    pub brb_truncate: u32,
    pub flow_ctrl_discard: u32,
    pub flow_ctrl_octets: u32,
    pub flow_ctrl_packet: u32,
    pub mng_discard: u32,
    pub mng_octet_inp: u32,
    pub mng_octet_out: u32,
    pub mng_packet_inp: u32,
    pub mng_packet_out: u32,
    pub pbf_octets: u32,
    pub pbf_packet: u32,
    pub safc_inp: u32,
    pub egress_mac_pkt0_lo: u32,
    pub egress_mac_pkt0_hi: u32,
    pub egress_mac_pkt1_lo: u32,
    pub egress_mac_pkt1_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_stats_event {
    STATS_EVENT_PMF = 0,
    STATS_EVENT_LINK_UP,
    STATS_EVENT_UPDATE,
    STATS_EVENT_STOP,
    STATS_EVENT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bnx2x_stats_state {
    STATS_STATE_DISABLED = 0,
    STATS_STATE_ENABLED,
    STATS_STATE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_eth_stats {
    pub total_bytes_received_hi: u32,
    pub total_bytes_received_lo: u32,
    pub total_bytes_transmitted_hi: u32,
    pub total_bytes_transmitted_lo: u32,
    pub total_unicast_packets_received_hi: u32,
    pub total_unicast_packets_received_lo: u32,
    pub total_multicast_packets_received_hi: u32,
    pub total_multicast_packets_received_lo: u32,
    pub total_broadcast_packets_received_hi: u32,
    pub total_broadcast_packets_received_lo: u32,
    pub total_unicast_packets_transmitted_hi: u32,
    pub total_unicast_packets_transmitted_lo: u32,
    pub total_multicast_packets_transmitted_hi: u32,
    pub total_multicast_packets_transmitted_lo: u32,
    pub total_broadcast_packets_transmitted_hi: u32,
    pub total_broadcast_packets_transmitted_lo: u32,
    pub valid_bytes_received_hi: u32,
    pub valid_bytes_received_lo: u32,
    pub error_bytes_received_hi: u32,
    pub error_bytes_received_lo: u32,
    pub etherstatsoverrsizepkts_hi: u32,
    pub etherstatsoverrsizepkts_lo: u32,
    pub no_buff_discard_hi: u32,
    pub no_buff_discard_lo: u32,
    pub rx_stat_ifhcinbadoctets_hi: u32,
    pub rx_stat_ifhcinbadoctets_lo: u32,
    pub tx_stat_ifhcoutbadoctets_hi: u32,
    pub tx_stat_ifhcoutbadoctets_lo: u32,
    pub rx_stat_dot3statsfcserrors_hi: u32,
    pub rx_stat_dot3statsfcserrors_lo: u32,
    pub rx_stat_dot3statsalignmenterrors_hi: u32,
    pub rx_stat_dot3statsalignmenterrors_lo: u32,
    pub rx_stat_dot3statscarriersenseerrors_hi: u32,
    pub rx_stat_dot3statscarriersenseerrors_lo: u32,
    pub rx_stat_falsecarriererrors_hi: u32,
    pub rx_stat_falsecarriererrors_lo: u32,
    pub rx_stat_etherstatsundersizepkts_hi: u32,
    pub rx_stat_etherstatsundersizepkts_lo: u32,
    pub rx_stat_dot3statsframestoolong_hi: u32,
    pub rx_stat_dot3statsframestoolong_lo: u32,
    pub rx_stat_etherstatsfragments_hi: u32,
    pub rx_stat_etherstatsfragments_lo: u32,
    pub rx_stat_etherstatsjabbers_hi: u32,
    pub rx_stat_etherstatsjabbers_lo: u32,
    pub rx_stat_maccontrolframesreceived_hi: u32,
    pub rx_stat_maccontrolframesreceived_lo: u32,
    pub rx_stat_bmac_xpf_hi: u32,
    pub rx_stat_bmac_xpf_lo: u32,
    pub rx_stat_bmac_xcf_hi: u32,
    pub rx_stat_bmac_xcf_lo: u32,
    pub rx_stat_xoffstateentered_hi: u32,
    pub rx_stat_xoffstateentered_lo: u32,
    pub rx_stat_xonpauseframesreceived_hi: u32,
    pub rx_stat_xonpauseframesreceived_lo: u32,
    pub rx_stat_xoffpauseframesreceived_hi: u32,
    pub rx_stat_xoffpauseframesreceived_lo: u32,
    pub tx_stat_outxonsent_hi: u32,
    pub tx_stat_outxonsent_lo: u32,
    pub tx_stat_outxoffsent_hi: u32,
    pub tx_stat_outxoffsent_lo: u32,
    pub tx_stat_flowcontroldone_hi: u32,
    pub tx_stat_flowcontroldone_lo: u32,
    pub tx_stat_etherstatscollisions_hi: u32,
    pub tx_stat_etherstatscollisions_lo: u32,
    pub tx_stat_dot3statssinglecollisionframes_hi: u32,
    pub tx_stat_dot3statssinglecollisionframes_lo: u32,
    pub tx_stat_dot3statsmultiplecollisionframes_hi: u32,
    pub tx_stat_dot3statsmultiplecollisionframes_lo: u32,
    pub tx_stat_dot3statsdeferredtransmissions_hi: u32,
    pub tx_stat_dot3statsdeferredtransmissions_lo: u32,
    pub tx_stat_dot3statsexcessivecollisions_hi: u32,
    pub tx_stat_dot3statsexcessivecollisions_lo: u32,
    pub tx_stat_dot3statslatecollisions_hi: u32,
    pub tx_stat_dot3statslatecollisions_lo: u32,
    pub tx_stat_etherstatspkts64octets_hi: u32,
    pub tx_stat_etherstatspkts64octets_lo: u32,
    pub tx_stat_etherstatspkts65octetsto127octets_hi: u32,
    pub tx_stat_etherstatspkts65octetsto127octets_lo: u32,
    pub tx_stat_etherstatspkts128octetsto255octets_hi: u32,
    pub tx_stat_etherstatspkts128octetsto255octets_lo: u32,
    pub tx_stat_etherstatspkts256octetsto511octets_hi: u32,
    pub tx_stat_etherstatspkts256octetsto511octets_lo: u32,
    pub tx_stat_etherstatspkts512octetsto1023octets_hi: u32,
    pub tx_stat_etherstatspkts512octetsto1023octets_lo: u32,
    pub tx_stat_etherstatspkts1024octetsto1522octets_hi: u32,
    pub tx_stat_etherstatspkts1024octetsto1522octets_lo: u32,
    pub tx_stat_etherstatspktsover1522octets_hi: u32,
    pub tx_stat_etherstatspktsover1522octets_lo: u32,
    pub tx_stat_bmac_2047_hi: u32,
    pub tx_stat_bmac_2047_lo: u32,
    pub tx_stat_bmac_4095_hi: u32,
    pub tx_stat_bmac_4095_lo: u32,
    pub tx_stat_bmac_9216_hi: u32,
    pub tx_stat_bmac_9216_lo: u32,
    pub tx_stat_bmac_16383_hi: u32,
    pub tx_stat_bmac_16383_lo: u32,
    pub tx_stat_dot3statsinternalmactransmiterrors_hi: u32,
    pub tx_stat_dot3statsinternalmactransmiterrors_lo: u32,
    pub tx_stat_bmac_ufl_hi: u32,
    pub tx_stat_bmac_ufl_lo: u32,
    pub pause_frames_received_hi: u32,
    pub pause_frames_received_lo: u32,
    pub pause_frames_sent_hi: u32,
    pub pause_frames_sent_lo: u32,
    pub etherstatspkts1024octetsto1522octets_hi: u32,
    pub etherstatspkts1024octetsto1522octets_lo: u32,
    pub etherstatspktsover1522octets_hi: u32,
    pub etherstatspktsover1522octets_lo: u32,
    pub brb_drop_hi: u32,
    pub brb_drop_lo: u32,
    pub brb_truncate_hi: u32,
    pub brb_truncate_lo: u32,
    pub mac_filter_discard: u32,
    pub mf_tag_discard: u32,
    pub brb_truncate_discard: u32,
    pub mac_discard: u32,
    pub driver_xoff: u32,
    pub rx_err_discard_pkt: u32,
    pub rx_skb_alloc_failed: u32,
    pub hw_csum_err: u32,
    pub nig_timer_max: u32,
// TPA
    pub total_tpa_aggregations_hi: u32,
    pub total_tpa_aggregations_lo: u32,
    pub total_tpa_aggregated_frames_hi: u32,
    pub total_tpa_aggregated_frames_lo: u32,
    pub total_tpa_bytes_hi: u32,
    pub total_tpa_bytes_lo: u32,
// PFC
    pub pfc_frames_received_hi: u32,
    pub pfc_frames_received_lo: u32,
    pub pfc_frames_sent_hi: u32,
    pub pfc_frames_sent_lo: u32,
// Recovery
    pub recoverable_error: u32,
    pub unrecoverable_error: u32,
    pub driver_filtered_tx_pkt: u32,
// src: Clear-on-Read register; Will not survive PMF Migration
    pub eee_tx_lpi: u32,
// PTP
    pub ptp_skip_tx_ts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_eth_q_stats {
    pub total_unicast_bytes_received_hi: u32,
    pub total_unicast_bytes_received_lo: u32,
    pub total_broadcast_bytes_received_hi: u32,
    pub total_broadcast_bytes_received_lo: u32,
    pub total_multicast_bytes_received_hi: u32,
    pub total_multicast_bytes_received_lo: u32,
    pub total_bytes_received_hi: u32,
    pub total_bytes_received_lo: u32,
    pub total_unicast_bytes_transmitted_hi: u32,
    pub total_unicast_bytes_transmitted_lo: u32,
    pub total_broadcast_bytes_transmitted_hi: u32,
    pub total_broadcast_bytes_transmitted_lo: u32,
    pub total_multicast_bytes_transmitted_hi: u32,
    pub total_multicast_bytes_transmitted_lo: u32,
    pub total_bytes_transmitted_hi: u32,
    pub total_bytes_transmitted_lo: u32,
    pub total_unicast_packets_received_hi: u32,
    pub total_unicast_packets_received_lo: u32,
    pub total_multicast_packets_received_hi: u32,
    pub total_multicast_packets_received_lo: u32,
    pub total_broadcast_packets_received_hi: u32,
    pub total_broadcast_packets_received_lo: u32,
    pub total_unicast_packets_transmitted_hi: u32,
    pub total_unicast_packets_transmitted_lo: u32,
    pub total_multicast_packets_transmitted_hi: u32,
    pub total_multicast_packets_transmitted_lo: u32,
    pub total_broadcast_packets_transmitted_hi: u32,
    pub total_broadcast_packets_transmitted_lo: u32,
    pub valid_bytes_received_hi: u32,
    pub valid_bytes_received_lo: u32,
    pub etherstatsoverrsizepkts_hi: u32,
    pub etherstatsoverrsizepkts_lo: u32,
    pub no_buff_discard_hi: u32,
    pub no_buff_discard_lo: u32,
    pub driver_xoff: u32,
    pub rx_err_discard_pkt: u32,
    pub rx_skb_alloc_failed: u32,
    pub hw_csum_err: u32,
    pub total_packets_received_checksum_discarded_hi: u32,
    pub total_packets_received_checksum_discarded_lo: u32,
    pub total_packets_received_ttl0_discarded_hi: u32,
    pub total_packets_received_ttl0_discarded_lo: u32,
    pub total_transmitted_dropped_packets_error_hi: u32,
    pub total_transmitted_dropped_packets_error_lo: u32,
// TPA
    pub total_tpa_aggregations_hi: u32,
    pub total_tpa_aggregations_lo: u32,
    pub total_tpa_aggregated_frames_hi: u32,
    pub total_tpa_aggregated_frames_lo: u32,
    pub total_tpa_bytes_hi: u32,
    pub total_tpa_bytes_lo: u32,
    pub driver_filtered_tx_pkt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_eth_stats_old {
    pub rx_stat_dot3statsframestoolong_hi: u32,
    pub rx_stat_dot3statsframestoolong_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_eth_q_stats_old {
// Fields to perserve over fw reset
    pub total_unicast_bytes_received_hi: u32,
    pub total_unicast_bytes_received_lo: u32,
    pub total_broadcast_bytes_received_hi: u32,
    pub total_broadcast_bytes_received_lo: u32,
    pub total_multicast_bytes_received_hi: u32,
    pub total_multicast_bytes_received_lo: u32,
    pub total_unicast_bytes_transmitted_hi: u32,
    pub total_unicast_bytes_transmitted_lo: u32,
    pub total_broadcast_bytes_transmitted_hi: u32,
    pub total_broadcast_bytes_transmitted_lo: u32,
    pub total_multicast_bytes_transmitted_hi: u32,
    pub total_multicast_bytes_transmitted_lo: u32,
    pub total_tpa_bytes_hi: u32,
    pub total_tpa_bytes_lo: u32,
// Fields to perserve last of
    pub total_bytes_received_hi: u32,
    pub total_bytes_received_lo: u32,
    pub total_bytes_transmitted_hi: u32,
    pub total_bytes_transmitted_lo: u32,
    pub total_unicast_packets_received_hi: u32,
    pub total_unicast_packets_received_lo: u32,
    pub total_multicast_packets_received_hi: u32,
    pub total_multicast_packets_received_lo: u32,
    pub total_broadcast_packets_received_hi: u32,
    pub total_broadcast_packets_received_lo: u32,
    pub total_unicast_packets_transmitted_hi: u32,
    pub total_unicast_packets_transmitted_lo: u32,
    pub total_multicast_packets_transmitted_hi: u32,
    pub total_multicast_packets_transmitted_lo: u32,
    pub total_broadcast_packets_transmitted_hi: u32,
    pub total_broadcast_packets_transmitted_lo: u32,
    pub valid_bytes_received_hi: u32,
    pub valid_bytes_received_lo: u32,
    pub total_tpa_bytes_hi_old: u32,
    pub total_tpa_bytes_lo_old: u32,
    pub driver_xoff_old: u32,
    pub rx_err_discard_pkt_old: u32,
    pub rx_skb_alloc_failed_old: u32,
    pub hw_csum_err_old: u32,
    pub driver_filtered_tx_pkt_old: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_net_stats_old {
    pub rx_dropped: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_fw_port_stats_old {
    pub mac_filter_discard: u32,
    pub mf_tag_discard: u32,
    pub brb_truncate_discard: u32,
    pub mac_discard: u32,
}

//
// Macros
//
// sum[hi:lo] += add[hi:lo]

// The _force is for cases where high value is 0

// difference = minuend - subtrahend

// underflow */ \
// we can 'loan' 1 */ \
// m_hi <= s_hi */ \
// m_lo >= s_lo */ \
// m_hi >= s_hi */ \

// sum[hi:lo] += add

// minuend -= subtrahend

// minuend[hi:lo] -= subtrahend

// forward
extern "C" {
    pub fn bnx2x_memset_stats(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_stats_init(bp: *mut bnx2x);
}
extern "C" {
    pub fn bnx2x_stats_handle(bp: *mut bnx2x, event: bnx2x_stats_event);
}
//
// bnx2x_save_statistics - save statistics when unloading.
//
// @bp:		driver handle
//
extern "C" {
    pub fn bnx2x_save_statistics(bp: *mut bnx2x);
}
