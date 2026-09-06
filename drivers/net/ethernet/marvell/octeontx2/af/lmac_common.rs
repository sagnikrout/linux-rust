//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/lmac_common.h
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
// Marvell CN10K RPM driver
//
// Copyright (C) 2020 Marvell.
//

//
// struct lmac - per lmac locks and properties
// @wq_cmd_cmplt:	waitq to keep the process blocked until cmd completion
// @cmd_lock:		Lock to serialize the command interface
// @resp:		command response
// @link_info:		link related information
// @mac_to_index_bmap:	Mac address to CGX table index mapping
// @rx_fc_pfvf_bmap:    Receive flow control enabled netdev mapping
// @tx_fc_pfvf_bmap:    Transmit flow control enabled netdev mapping
// @event_cb:		callback for linkchange events
// @event_cb_lock:	lock for serializing callback with unregister
// @cgx:		parent cgx port
// @mcast_filters_count:  Number of multicast filters installed
// @lmac_id:		lmac port id
// @lmac_type:	        lmac type like SGMII/XAUI
// @cmd_pend:		flag set before new command is started
// flag cleared after command response is received
// @name:		lmac port name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lmac {
    pub wq_cmd_cmplt: wait_queue_head_t,
// Lock to serialize the command interface
    pub cmd_lock: mutex,
    pub resp: u64,
    pub link_info: cgx_link_user_info,
    pub mac_to_index_bmap: rsrc_bmap,
    pub rx_fc_pfvf_bmap: rsrc_bmap,
    pub tx_fc_pfvf_bmap: rsrc_bmap,
    pub event_cb: cgx_event_cb,
// lock for serializing callback with unregister
    pub event_cb_lock: spinlock_t,
    pub cgx: *mut cgx,
    pub mcast_filters_count: u8,
    pub lmac_id: u8,
    pub lmac_type: u8,
    pub cmd_pend: bool,
    pub name: *mut c_char,
}

// CGX & RPM has different feature set
// update the structure fields with different one
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_ops {
    pub name: *mut c_char,
// Features like RXSTAT, TXSTAT, DMAC FILTER csrs differs by fixed
// bar offset for example
// CGX DMAC_CTL0  0x1f8
// RPM DMAC_CTL0  0x4ff8
//
    pub csr_offset: u64,
// For ATF to send events to kernel, there is no dedicated interrupt
// defined hence CGX uses OVERFLOW bit in CMR_INT. RPM block supports
// SW_INT so that ATF triggers this interrupt after processing of
// requested command
//
    pub int_register: u64,
    pub int_set_reg: u64,
// lmac offset is different is RPM
    pub lmac_offset: u8,
    pub irq_offset: u8,
    pub int_ena_bit: u8,
    pub lmac_fwi: u8,
    pub non_contiguous_serdes_lane: bool,
// RPM & CGX differs in number of Receive/transmit stats
    pub rx_stats_cnt: u8,
    pub tx_stats_cnt: u8,
// Unlike CN10K which shares same CSR offset with CGX
// CNF10KB has different csr offset
//
    pub rxid_map_offset: u64,
    pub dmac_filter_count: u8,
// Incase of RPM get number of lmacs from RPMX_CMR_RX_LMACS[LMAC_EXIST]
// number of setbits in lmac_exist tells number of lmacs
//
    pub cgx): *mut *mut int (get_nr_lmacs)(void,
    pub lmac_id): *mut *mut *mut u8 (get_lmac_type)(void cgx, int,
    pub lmac_id): *mut *mut *mut u32 (lmac_fifo_len)(void cgx, int,
    pub enable): bool,
// Register Stats related functions
    pub rx_stat): *mut int idx, u64,
    pub tx_stat): *mut int idx, u64,
// Enable LMAC Pause Frame Configuration
    pub enable): bool,
    pub rx_pause): *mut u8,
    pub rx_pause): u8,
    pub enable): bool,
// Enable/Disable Inbound PTP
    pub enable): bool,
    pub enable): *mut *mut *mut int (mac_rx_tx_enable)(void cgxd, int lmac_id, bool,
    pub enable): *mut *mut *mut int (mac_tx_enable)(void cgxd, int lmac_id, bool,
    pub pfc_en): u8 tx_pause, u8 rx_pause, u16,
    pub rx_pause): *mut *mut u8 tx_pause, u8,
    pub pf_req_flr): *mut *mut *mut int (mac_reset)(void cgxd, int lmac_id, u8,
// FEC stats
    pub rsp): *mut cgx_fec_stats_rsp,
    pub lmac_id): *mut *mut *mut int (mac_stats_reset)(void cgxd, int,
    pub enable): *mut *mut *mut void (mac_x2p_reset)(void cgxd, bool,
    pub enable): *mut *mut *mut int (mac_enadis_rx)(void cgxd, int lmac_id, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgx {
    pub reg_base: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub cgx_id: u8,
    pub lmac_count: u8,
// number of LMACs per MAC could be 4 or 8
    pub max_lmac_per_mac: u8,
// length of fifo varies depending on the number
// of LMACS
//
    pub fifo_len: u32,
pub const MAX_LMAC_COUNT: c_int = 8;
    pub lmac_idmap: [*mut lmac; MAX_LMAC_COUNT],
    pub cgx_cmd_work: work_struct,
    pub cgx_cmd_workq: *mut workqueue_struct,
    pub cgx_list: list_head,
    pub hw_features: u64,
    pub mac_ops: *mut mac_ops,
    pub /: *mut *mut unsigned long lmac_bmap; / bitmap of enabled lmacs,
// Lock to serialize read/write of global csrs like
// RPMX_MTI_STAT_DATA_HI_CDC etc
//
    pub lock: mutex,
}

pub type rpm_t = cgx;
// Function Declarations
extern "C" {
    pub fn cgx_write(cgx: *mut cgx, lmac: u64, offset: u64, val: u64);
}
extern "C" {
    pub fn cgx_read(cgx: *mut cgx, lmac: u64, offset: u64) -> u64;
}
extern "C" {
    pub fn cgx_fwi_cmd_send(req: u64, resp: *mut u64, lmac: *mut lmac) -> c_int;
}
extern "C" {
    pub fn cgx_fwi_cmd_generic(req: u64, resp: *mut u64, cgx: *mut cgx, lmac_id: c_int) -> c_int;
}
extern "C" {
    pub fn is_lmac_valid(cgx: *mut cgx, lmac_id: c_int) -> bool;
}
