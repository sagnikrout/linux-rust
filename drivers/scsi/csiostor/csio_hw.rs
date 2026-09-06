//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/csiostor/csio_hw.h
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
// This file is part of the Chelsio FCoE driver for Linux.
//
// Copyright (c) 2008-2012 Chelsio Communications, Inc. All rights reserved.
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

//
// An error value used by host. Should not clash with FW defined return values.
//
pub const FW_HOSTERROR: c_int = 255;

pub const CSIO_MAX_PFN: c_int = 8;
pub const CSIO_MAX_PPORTS: c_int = 4;
pub const CSIO_MAX_LUN: c_uint = 0xFFFF;
pub const CSIO_MAX_QUEUE: c_int = 2048;
pub const CSIO_MAX_CMD_PER_LUN: c_int = 32;

pub const CSIO_MAX_SECTOR_SIZE: c_int = 128;
pub const CSIO_MIN_T6_FW: c_uint = 0x01102D00  /* FW 1.16.45.0 */;
// Interrupts

// (Forward intr iq + fw iq)

pub const CSIO_MAX_SCSI_CPU: c_int = 128;

// Queues
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_msix_entries {
    pub msix*/: *mut *mut *mut void dev_id; / Priv object associated w/ this,
    pub /: *mut *mut char desc[24]; / Description of this vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_scsi_qset {
    pub /: *mut *mut int iq_idx; / Ingress index,
    pub /: *mut *mut int eq_idx; / Egress index,
    pub /: *mut *mut uint32_t intr_idx; / MSIX Vector index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_scsi_cpu_info {
    pub max_cpus: i16,
}

pub const CSIO_VENDOR_ID: c_uint = 0x1425;
pub const CSIO_ASIC_DEVID_PROTO_MASK: c_uint = 0xFF00;
pub const CSIO_ASIC_DEVID_TYPE_MASK: c_uint = 0x00FF;

//
// Hard parameters used to initialize the card in the absence of a
// configuration file.
//
// General
// Slowpath events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_evt {
    CSIO_EVT_FW  = 0,	/* FW event */
    CSIO_EVT_MBX,		/* MBX event */
    CSIO_EVT_SCN,		/* State change notification */
    CSIO_EVT_DEV_LOSS,	/* Device loss event */
    CSIO_EVT_MAX,		/* Max supported event */
}

pub const CSIO_EVT_MSG_SIZE: c_int = 512;
pub const CSIO_EVTQ_SIZE: c_int = 512;
// Event msg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_evt_msg {
    pub queue*/: *mut *mut list_head list; / evt,
    pub type: csio_evt,
    pub data: [u8; CSIO_EVT_MSG_SIZE],
}

// serial flash and firmware constants
// flash command opcodes
// Management module

// mgmt module stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_mgmtm_stats {
    pub /: *mut *mut uint32_t n_abort_req; / Total abort request,
    pub /: *mut *mut uint32_t n_abort_rsp; / Total abort response,
    pub /: *mut *mut uint32_t n_close_req; / Total close request,
    pub /: *mut *mut uint32_t n_close_rsp; / Total close response,
    pub /: *mut *mut uint32_t n_err; / Total Errors,
    pub /: *mut *mut uint32_t n_drop; / Total request dropped,
    pub /: *mut *mut uint32_t n_active; / Count of active_q,
    pub /: *mut *mut uint32_t n_cbfn; / Count of cbfn_q,
}

// MGMT module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_mgmtm {
    pub /: *mut *mut *mut csio_hw hw; / Pointer to HW moduel,
    pub /: *mut *mut int eq_idx; / Egress queue index,
    pub /: *mut *mut int iq_idx; / Ingress queue index,
    pub /: *mut *mut int msi_vec; / MSI vector,
    pub /: *mut *mut list_head active_q; / Outstanding ELS/CT,
    pub /: *mut *mut list_head abort_q; / Outstanding abort req,
    pub /: *mut *mut list_head cbfn_q; / Completion queue,
    pub /: *mut *mut list_head mgmt_req_freelist; / Free poll of reqs,
// ELSCT request freelist
    pub /: *mut *mut timer_list mgmt_timer; / MGMT timer,
    pub /: *mut *mut csio_mgmtm_stats stats; / ELS/CT stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_adap_desc {
    pub model_no: [c_char; 16],
    pub description: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_params {
    pub vendor_id: u16,
    pub device_id: u16,
    pub vpd_cap_addr: c_int,
    pub speed: u16,
    pub width: u8,
}

// User configurable hw parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_hw_params {
    pub flash: *mut *mut uint32_t sf_size; / serial,
// size in bytes
//
    pub /: *mut *mut uint32_t sf_nsec; / # of flash sectors,
    pub pci: pci_params,
    pub for: *mut *mut uint32_t log_level; / Module-level,
// debug log.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_vpd {
    pub cclk: u32,
    pub 1]: uint8_t ec[EC_LEN +,
    pub 1]: uint8_t sn[SERNUM_LEN +,
    pub 1]: uint8_t id[ID_LEN +,
}

// Firmware Port Capabilities types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_caps {
    FW_CAPS_UNKNOWN = 0,    /* 0'ed out initial state */
    FW_CAPS16       = 1,    /* old Firmware: 16-bit Port Capabilities */
    FW_CAPS32       = 2,    /* new Firmware: 32-bit Port Capabilities */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_pause {
    PAUSE_RX      = 1 << 0,
    PAUSE_TX      = 1 << 1,
    PAUSE_AUTONEG = 1 << 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_fec {
    FEC_AUTO	= 1 << 0,  /* IEEE 802.3 "automatic" */
    FEC_RS		= 1 << 1,  /* Reed-Solomon */
    FEC_BASER_RS	= 1 << 2   /* BaseR/Reed-Solomon */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_config {
    pub /: *mut *mut fw_port_cap32_t pcaps; / link capabilities,
    pub /: *mut *mut fw_port_cap32_t def_acaps; / default advertised capabilities,
    pub /: *mut *mut fw_port_cap32_t acaps; / advertised capabilities,
    pub /: *mut *mut fw_port_cap32_t lpacaps; / peer advertised capabilities,
    pub /: *mut *mut fw_port_cap32_t speed_caps; / speed(s) user has requested,
    pub /: *mut *mut unsigned int speed; / actual link speed (Mb/s),
    pub /: *mut *mut cc_pause requested_fc; / flow control user has requested,
    pub /: *mut *mut cc_pause fc; / actual link flow control,
    pub /: *mut *mut cc_fec requested_fec; / Forward Error Correction:,
    pub /: *mut *mut cc_fec fec; / requested and actual in use,
    pub /: *mut *mut unsigned char autoneg; / autonegotiating?,
    pub /: *mut *mut unsigned char link_ok; / link up?,
    pub /: *mut *mut unsigned char link_down_rc; / link down reason,
}

// Enable or disable autonegotiation.
pub const AUTONEG_DISABLE: c_uint = 0x00;
pub const AUTONEG_ENABLE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_pport {
    pub pcap: u16,
    pub acap: u16,
    pub portid: u8,
    pub link_status: u8,
    pub link_speed: u16,
    pub mac: [u8; 6],
    pub mod_type: u8,
    pub rsvd1: u8,
    pub rsvd2: u8,
    pub rsvd3: u8,
    pub link_cfg: link_config,
}

// fcoe resource information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_fcoe_res_info {
    pub e_d_tov: u16,
    pub r_a_tov_seq: u16,
    pub r_a_tov_els: u16,
    pub r_r_tov: u16,
    pub max_xchgs: u32,
    pub max_ssns: u32,
    pub used_xchgs: u32,
    pub used_ssns: u32,
    pub max_fcfs: u32,
    pub max_vnps: u32,
    pub used_fcfs: u32,
    pub used_vnps: u32,
}

// HW State machine Events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_hw_ev {
    CSIO_HWE_CFG = (uint32_t)1, /* Starts off the State machine */
    CSIO_HWE_INIT,	         /* Config done, start Init      */
    CSIO_HWE_INIT_DONE,      /* Init Mailboxes sent, HW ready */
    CSIO_HWE_FATAL,		 /* Fatal error during initialization */
    CSIO_HWE_PCIERR_DETECTED,/* PCI error recovery detetced */
    CSIO_HWE_PCIERR_SLOT_RESET, /* Slot reset after PCI recoviery */
    CSIO_HWE_PCIERR_RESUME,  /* Resume after PCI error recovery */
    CSIO_HWE_QUIESCED,	 /* HBA quiesced */
    CSIO_HWE_HBA_RESET,      /* HBA reset requested */
    CSIO_HWE_HBA_RESET_DONE, /* HBA reset completed */
    CSIO_HWE_FW_DLOAD,       /* FW download requested */
    CSIO_HWE_PCI_REMOVE,     /* PCI de-instantiation */
    CSIO_HWE_SUSPEND,        /* HW suspend for Online(hot) replacement */
    CSIO_HWE_RESUME,         /* HW resume for Online(hot) replacement */
    CSIO_HWE_MAX,		 /* Max HW event */
}

// hw stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_hw_stats {
    pub /: *mut *mut uint32_t n_evt_activeq; / Number of event in active Q,
    pub /: *mut *mut uint32_t n_evt_freeq; / Number of event in free Q,
    pub /: *mut *mut uint32_t n_evt_drop; / Number of event droped,
    pub /: *mut *mut uint32_t n_evt_unexp; / Number of unexpected events,
    pub /: *mut *mut uint32_t n_pcich_offline;/ Number of pci channel offline,
    pub /: *mut *mut uint32_t n_lnlkup_miss; / Number of lnode lookup miss,
    pub message*/: *mut *mut uint32_t n_cpl_fw6_msg; / Number of cpl fw6,
    pub payload*/: *mut *mut uint32_t n_cpl_fw6_pld; / Number of cpl fw6,
    pub /: *mut *mut uint32_t n_cpl_unexp; / Number of unexpected cpl,
    pub /: *mut *mut uint32_t n_mbint_unexp; / Number of unexpected mbox,
// interrupt
    pub /: *mut *mut uint32_t n_plint_unexp; / Number of unexpected PL,
// interrupt
    pub /: *mut *mut uint32_t n_plint_cnt; / Number of PL interrupt,
    pub /: *mut *mut uint32_t n_int_stray; / Number of stray interrupt,
    pub /: *mut *mut uint32_t n_err; / Number of hw errors,
    pub /: *mut *mut uint32_t n_err_fatal; / Number of fatal errors,
    pub /: *mut *mut uint32_t n_err_nomem; / Number of memory alloc failure,
    pub /: *mut *mut uint32_t n_err_io; / Number of IO failure,
    pub /: *mut *mut csio_hw_ev n_evt_sm[CSIO_HWE_MAX]; / Number of sm events,
    pub /: *mut *mut uint64_t n_reset_start; / Start time after the reset,
    pub rsvd1: u32,
}

// Defines for hw->flags
pub const CSIO_HWF_MASTER: c_uint = 0x00000001	/* This is the Master;
// function for the
// card.
//
pub const CSIO_HWF_HW_INTR_ENABLED: c_uint = 0x00000002	/* Are HW Interrupt;
// enable bit set?
//
pub const CSIO_HWF_FWEVT_PENDING: c_uint = 0x00000004	/* FW events pending */;
pub const CSIO_HWF_Q_MEM_ALLOCED: c_uint = 0x00000008	/* Queues have been;
// allocated memory.
//
pub const CSIO_HWF_Q_FW_ALLOCED: c_uint = 0x00000010	/* Queues have been;
// allocated in FW.
//
pub const CSIO_HWF_VPD_VALID: c_uint = 0x00000020	/* Valid VPD copied */;

// id cached
pub const CSIO_HWF_FWEVT_STOP: c_uint = 0x00000080	/* Stop processing;
// FW events
//
pub const CSIO_HWF_USING_SOFT_PARAMS: c_uint = 0x00000100      /* Using FW config;
// params
//
pub const CSIO_HWF_HOST_INTR_ENABLED: c_uint = 0x00000200	/* Are host interrupts;
// enabled?
//
pub const CSIO_HWF_ROOT_NO_RELAXED_ORDERING: c_uint = 0x00000400	/* Is PCIe relaxed;
// ordering enabled
//

// Defines for intr_mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csio_intr_mode {
    CSIO_IM_NONE = 0,
    CSIO_IM_INTX = 1,
    CSIO_IM_MSI  = 2,
    CSIO_IM_MSIX = 3,
}

// Master HW structure: One per function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csio_hw {
    pub should: *mut *mut csio_sm sm; / State machine:,
// be the 1st member.
//
    pub /: *mut *mut spinlock_t lock; / Lock for hw,
    pub module*/: *mut *mut csio_scsim scsim; / SCSI,
    pub module*/: *mut *mut csio_wrm wrm; / Work request,
    pub /: *mut *mut *mut pci_dev pdev; / PCI device,
    pub of: *mut *mut *mut void __iomem regstart; / Virtual address,
// register map
//
// SCSI queue sets
    pub SCSI: *mut *mut uint32_t num_sqsets; / Number of,
// queue sets
    pub that: *mut *mut uint32_t num_scsi_msix_cpus; / Number of CPUs,
// will be used
// for ingress
// processing.
//
    pub sqset: [csio_scsi_qset; CSIO_MAX_PPORTS][CSIO_MAX_SCSI_CPU],
    pub scsi_cpu_info: [csio_scsi_cpu_info; CSIO_MAX_PPORTS],
    pub /: *mut *mut uint32_t evtflag; / Event flag,
    pub /: *mut *mut uint32_t flags; / HW flags,
    pub /: *mut *mut csio_mgmtm mgmtm; / management module,
    pub /: *mut *mut csio_mbm mbm; / Mailbox module,
// Lnodes
    pub /: *mut *mut uint32_t num_lns; / Number of lnodes,
    pub /: *mut *mut *mut csio_lnode rln; / Root lnode,
    pub list: *mut *mut list_head sln_head; / Sibling node,
// list
//
    pub interrupt: *mut *mut int intr_iq_idx; / Forward,
// queue.
//
    pub /: *mut *mut int fwevt_iq_idx; / FW evt queue,
    pub for: *mut *mut work_evtq_work; / Worker thread,
// HW events.
//
    pub evt: *mut *mut list_head evt_free_q; / freelist of,
// elements
//
    pub queue*/: *mut *mut list_head evt_active_q; / active evt,
// board related info
    pub name: [c_char; 32],
    pub hw_ver: [c_char; 16],
    pub model_desc: [c_char; 32],
    pub drv_version: [c_char; 32],
    pub fwrev_str: [c_char; 32],
    pub optrom_ver: u32,
    pub fwrev: u32,
    pub tp_vers: u32,
    pub chip_ver: c_char,
    pub /: *mut *mut uint16_t chip_id; / Tells T4/T5 chip,
    pub fw_state: csio_dev_state,
    pub vpd: csio_vpd,
    pub Function: *mut *mut uint8_t pfn; / Physical,
// number
//
    pub /: *mut *mut uint32_t port_vec; / Port vector,
    pub physical: *mut *mut uint8_t num_pports; / Number of,
// ports.
//
    pub /: *mut *mut uint8_t rst_retries; / Reset retries,
    pub /: *mut *mut uint8_t cur_evt; / current s/m evt,
    pub /: *mut *mut uint8_t prev_evt; / Previous s/m evt,
    pub /: *mut *mut uint32_t dev_num; / device number,
    pub /: *mut *mut csio_pport pport[CSIO_MAX_PPORTS]; / Ports (XGMACs),
    pub /: *mut *mut csio_hw_params params; / Hw parameters,
    pub /: *mut *mut *mut dma_pool scsi_dma_pool; / DMA pool for SCSI,
    pub pool*/: *mut *mut *mut mempool_t mb_mempool; / Mailbox memory,
    pub /: *mut *mut *mut mempool_t rnode_mempool; / rnode memory pool,
// Interrupt
    pub /: *mut *mut csio_intr_mode intr_mode; / INTx, MSI, MSIX,
    pub MSIX/interrupt: *mut *mut uint32_t fwevt_intr_idx; / FW evt,
// index
//
    pub MSIX/intr: *mut *mut uint32_t nondata_intr_idx; / nondata,
// idx
//
    pub of: *mut *mut uint8_t cfg_neq; / FW configured no,
// egress queues
//
    pub of: *mut *mut uint8_t cfg_niq; / FW configured no,
// iq queues.
//
    pub /: *mut *mut csio_fcoe_res_info fres_info; / Fcoe resource info,
    pub specific: *mut *mut *mut csio_hw_chip_ops chip_ops; / T4/T5 Chip,
// Operations
//
// MSIX vectors
    pub msix_entries: [csio_msix_entries; CSIO_MAX_MSIX_VECS],
    pub /: *mut *mut *mut dentry debugfs_root; / Debug FS,
    pub /: *mut *mut csio_hw_stats stats; / Hw statistics,
}

// Register access macros

extern "C" {
    pub fn csio_set_reg_field(: *mut csio_hw, _arg: u32, _arg: u32, _arg: u32);
}
// Core clocks <==> uSecs
// add Core Clock / 2 to round ticks to nearest uS
// Easy access macros

// Printing/logging

extern "C" {
    pub fn csio_mgmt_req_lookup(: *mut csio_mgmtm, : *mut csio_ioreq) -> c_int;
}
extern "C" {
    pub fn csio_hw_intr_disable(: *mut csio_hw);
}
extern "C" {
    pub fn csio_hw_slow_intr_handler(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn fwcap_to_fwspeed(acaps: fw_port_cap32_t) -> fw_port_cap32_t;
}
extern "C" {
    pub fn fwcaps16_to_caps32(caps16: fw_port_cap16_t) -> fw_port_cap32_t;
}
extern "C" {
    pub fn fwcaps32_to_caps16(caps32: fw_port_cap32_t) -> fw_port_cap16_t;
}
extern "C" {
    pub fn lstatus_to_fwcap(lstatus: u32) -> fw_port_cap32_t;
}
extern "C" {
    pub fn csio_hw_start(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_hw_stop(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_hw_reset(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_is_hw_ready(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_is_hw_removing(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_fwevtq_handler(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_evtq_worker(: *mut work_struct);
}
extern "C" {
    pub fn csio_enqueue_evt(: *mut csio_hw, csio_evt: enum, : *mut c_void, _arg: u16) -> c_int;
}
extern "C" {
    pub fn csio_evtq_flush(hw: *mut csio_hw);
}
extern "C" {
    pub fn csio_request_irqs(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_intr_enable(: *mut csio_hw);
}
extern "C" {
    pub fn csio_intr_disable(: *mut csio_hw, _arg: bool);
}
extern "C" {
    pub fn csio_hw_fatal_err(: *mut csio_hw);
}
extern "C" {
    pub fn csio_config_queues(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_hw_init(: *mut csio_hw) -> c_int;
}
extern "C" {
    pub fn csio_hw_exit(: *mut csio_hw);
}
