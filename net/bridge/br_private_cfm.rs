//! Automatically rewritten from C Header to Rust Module
//! Source: net/bridge/br_private_cfm.h
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
pub struct br_cfm_mep_create {
    pub /: *mut *mut br_cfm_domain domain; / Domain for this MEP,
    pub /: *mut *mut br_cfm_mep_direction direction; / Up or Down MEP direction,
    pub /: *mut *mut u32 ifindex; / Residence port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_mep_config {
    pub mdlevel: u32,
    pub /: *mut *mut u32 mepid; / MEPID for this MEP,
    pub /: *mut *mut mac_addr unicast_mac; / The MEP unicast MAC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_maid {
    pub data: [u8; CFM_MAID_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_cc_config {
// Expected received CCM PDU MAID.
    pub exp_maid: br_cfm_maid,
// Expected received CCM PDU interval.
// Transmitting CCM PDU interval when CCM tx is enabled.
    pub exp_interval: br_cfm_ccm_interval,
    pub /: *mut *mut bool enable; / Enable/disable CCM PDU handling,
}

// Transmitted CCM Remote Defect Indication status set.
// This RDI is inserted in transmitted CCM PDUs if CCM transmission is enabled.
// See br_cfm_cc_ccm_tx() with interval != BR_CFM_CCM_INTERVAL_NONE
//
// OAM PDU Tx information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_cc_ccm_tx_info {
    pub dmac: mac_addr,
// The CCM will be transmitted for this period in seconds.
// Call br_cfm_cc_ccm_tx before timeout to keep transmission alive.
// When period is zero any ongoing transmission will be stopped.
//
    pub period: u32,
    pub /: *mut *mut bool seq_no_update; / Update Tx CCM sequence number,
    pub /: *mut *mut bool if_tlv; / Insert Interface Status TLV,
    pub /: *mut *mut u8 if_tlv_value; / Interface Status TLV value,
    pub /: *mut *mut bool port_tlv; / Insert Port Status TLV,
    pub /: *mut *mut u8 port_tlv_value; / Port Status TLV value,
// Sender ID TLV ??
// Organization-Specific TLV ??
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_mep_status {
// Indications that an OAM PDU has been seen.
    pub /: *mut *mut bool opcode_unexp_seen; / RX of OAM PDU with unexpected opcode,
    pub /: *mut *mut bool version_unexp_seen; / RX of OAM PDU with unexpected version,
    pub /: *mut *mut bool rx_level_low_seen; / Rx of OAM PDU with level low,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_cc_peer_status {
// This CCM related status is based on the latest received CCM PDU.
    pub /: *mut *mut u8 port_tlv_value; / Port Status TLV value,
    pub /: *mut *mut u8 if_tlv_value; / Interface Status TLV value,
// CCM has not been received for 3.25 intervals
    pub ccm_defect:1: u8,
// (RDI == 1) for last received CCM PDU
    pub rdi:1: u8,
// Indications that a CCM PDU has been seen.
    pub /: *mut *mut u8 seen:1; / CCM PDU received,
    pub /: *mut *mut u8 tlv_seen:1; / CCM PDU with TLV received,
// CCM PDU with unexpected sequence number received
    pub seq_unexp_seen:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_mep {
// list header of MEP instances
    pub head: hlist_node,
    pub instance: u32,
    pub create: br_cfm_mep_create,
    pub config: br_cfm_mep_config,
    pub cc_config: br_cfm_cc_config,
    pub cc_ccm_tx_info: br_cfm_cc_ccm_tx_info,
// List of multiple peer MEPs
    pub peer_mep_list: hlist_head,
    pub b_port: *mut net_bridge_port __rcu,
    pub ccm_tx_end: c_ulong,
    pub ccm_tx_dwork: delayed_work,
    pub ccm_tx_snumber: u32,
    pub ccm_rx_snumber: u32,
    pub status: br_cfm_mep_status,
    pub rdi: bool,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_peer_mep {
    pub head: hlist_node,
    pub mep: *mut br_cfm_mep,
    pub ccm_rx_dwork: delayed_work,
    pub mepid: u32,
    pub cc_status: br_cfm_cc_peer_status,
    pub ccm_rx_count_miss: u32,
    pub rcu: rcu_head,
}
