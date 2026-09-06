//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_sli4.h
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
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2026 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2009-2016 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//

// Macro flag: #define CONFIG_SCSI_LPFC_DEBUG_FS

pub const LPFC_ACTIVE_MBOX_WAIT_CNT: c_int = 100;
pub const LPFC_XRI_EXCH_BUSY_WAIT_TMO: c_int = 10000;
pub const LPFC_XRI_EXCH_BUSY_WAIT_T1: c_int = 10;
pub const LPFC_XRI_EXCH_BUSY_WAIT_T2: c_int = 30000;
pub const LPFC_RPI_LOW_WATER_MARK: c_int = 10;
pub const LPFC_UNREG_FCF: c_int = 1;
pub const LPFC_SKIP_UNREG_FCF: c_int = 0;
// Amount of time in seconds for waiting FCF rediscovery to complete

// Number of SGL entries can be posted in a 4KB nonembedded mbox command
pub const LPFC_NEMBED_MBOX_SGL_CNT: c_int = 254;
// Multi-queue arrangement for FCP EQ/CQ/WQ tuples
pub const LPFC_HBA_HDWQ_MIN: c_int = 0;
pub const LPFC_HBA_HDWQ_MAX: c_int = 256;

// irq_chann range, values
pub const LPFC_IRQ_CHANN_MIN: c_int = 0;
pub const LPFC_IRQ_CHANN_MAX: c_int = 256;

// FCP MQ queue count limiting
pub const LPFC_FCP_MQ_THRESHOLD_MIN: c_int = 0;
pub const LPFC_FCP_MQ_THRESHOLD_MAX: c_int = 256;
pub const LPFC_FCP_MQ_THRESHOLD_DEF: c_int = 8;
//
// Provide the default FCF Record attributes used by the driver
// when nonFIP mode is configured and there is no other default
// FCF Record attributes.
//
pub const LPFC_FCOE_FCF_DEF_INDEX: c_int = 0;
pub const LPFC_FCOE_FCF_GET_FIRST: c_uint = 0xFFFF;
pub const LPFC_FCOE_FCF_NEXT_NONE: c_uint = 0xFFFF;
pub const LPFC_FCOE_NULL_VID: c_uint = 0xFFF;
pub const LPFC_FCOE_IGNORE_VID: c_uint = 0xFFFF;
// First 3 bytes of default FCF MAC is specified by FC_MAP
pub const LPFC_FCOE_FCF_MAC3: c_uint = 0xFF;
pub const LPFC_FCOE_FCF_MAC4: c_uint = 0xFF;
pub const LPFC_FCOE_FCF_MAC5: c_uint = 0xFE;
pub const LPFC_FCOE_FCF_MAP0: c_uint = 0x0E;
pub const LPFC_FCOE_FCF_MAP1: c_uint = 0xFC;
pub const LPFC_FCOE_FCF_MAP2: c_uint = 0x00;
pub const LPFC_FCOE_MAX_RCV_SIZE: c_uint = 0x800;
pub const LPFC_FCOE_FKA_ADV_PER: c_int = 0;
pub const LPFC_FCOE_FIP_PRIORITY: c_uint = 0x80;

pub const LPFC_FW_RESET_MAXIMUM_WAIT_10MS_CNT: c_int = 12000;
pub const INT_FW_UPGRADE: c_int = 0;
pub const RUN_FW_UPGRADE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_sli4_queue_type {
    LPFC_EQ,
    LPFC_GCQ,
    LPFC_MCQ,
    LPFC_WCQ,
    LPFC_RCQ,
    LPFC_MQ,
    LPFC_WQ,
    LPFC_HRQ,
    LPFC_DRQ
}

// The queue sub-type defines the functional purpose of the queue
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_sli4_queue_subtype {
    LPFC_NONE,
    LPFC_MBOX,
    LPFC_IO,
    LPFC_ELS,
    LPFC_NVMET,
    LPFC_NVME_LS,
    LPFC_USOL
}

// RQ buffer list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rqb {
    pub /: *mut *mut uint16_t entry_count; / Current number of RQ slots,
    pub /: *mut *mut uint16_t buffer_count; / Current number of buffers posted,
    pub /: *mut *mut list_head rqb_buffer_list; / buffers assigned to this HBQ,
// Callback for HBQ buffer allocation
    pub ): *mut *mut *mut rqb_dmabuf (rqb_alloc_buffer)(lpfc_hba,
// Callback for HBQ buffer free
    pub ): *mut rqb_dmabuf,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_poll_mode {
    LPFC_QUEUE_WORK,
    LPFC_THREADED_IRQ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_idle_stat {
    pub prev_idle: u64,
    pub prev_wall: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_queue {
    pub list: list_head,
    pub wq_list: list_head,
//
// If interrupts are in effect on _all_ the eq's the footprint
// of polling code is zero (except mode). This memory is chec-
// ked for every io to see if the io needs to be polled and
// while completion to check if the eq's needs to be rearmed.
// Keep in same cacheline as the queue ptr to avoid cpu fetch
// stalls. Using 1B memory will leave us with 7B hole. Fill
// it with other frequently used members.
//
    pub /: *mut *mut uint16_t last_cpu; / most recent cpu,
    pub hdwq: u16,
    pub qe_valid: u8,
    pub /: *mut *mut uint8_t mode; / interrupt or polling,
pub const LPFC_EQ_INTERRUPT: c_int = 0;
pub const LPFC_EQ_POLL: c_int = 1;
    pub wqfull_list: list_head,
    pub type: lpfc_sli4_queue_type,
    pub subtype: lpfc_sli4_queue_subtype,
    pub phba: *mut lpfc_hba,
    pub child_list: list_head,
    pub page_list: list_head,
    pub sgl_list: list_head,
    pub cpu_list: list_head,
    pub /: *mut *mut uint32_t entry_count; / Number of entries to support on the queue,
    pub /: *mut *mut uint32_t entry_size; / Size of each queue entry.,
    pub entry_cnt_per_pg: u32,
    pub Interval: *mut *mut uint32_t notify_interval; / Queue Notification,
// For chip->host queues (EQ, CQ, RQ):
// specifies the interval (number of
// entries) where the doorbell is rung to
// notify the chip of entry consumption.
// For host->chip queues (WQ):
// specifies the interval (number of
// entries) where consumption CQE is
// requested to indicate WQ entries
// consumed by the chip.
// Not used on an MQ.
//
pub const LPFC_EQ_NOTIFY_INTRVL: c_int = 16;
pub const LPFC_CQ_NOTIFY_INTRVL: c_int = 16;
pub const LPFC_WQ_NOTIFY_INTRVL: c_int = 16;
pub const LPFC_RQ_NOTIFY_INTRVL: c_int = 16;
    pub Limit: *mut *mut uint32_t max_proc_limit; / Queue Processing,
// For chip->host queues (EQ, CQ):
// specifies the maximum number of
// entries to be consumed in one
// processing iteration sequence. Queue
// will be rearmed after each iteration.
// Not used on an MQ, RQ or WQ.
//
pub const LPFC_EQ_MAX_PROC_LIMIT: c_int = 256;
pub const LPFC_CQ_MIN_PROC_LIMIT: c_int = 64;

pub const LPFC_CQ_MIN_THRESHOLD_TO_POLL: c_int = 64;

    pub /: *mut *mut uint32_t queue_claimed; / indicates queue is being processed,
    pub /: *mut *mut uint32_t queue_id; / Queue ID assigned by the hardware,
    pub /: *mut *mut uint32_t assoc_qid; / Queue ID associated with, for CQ/WQ/MQ,
    pub /: *mut *mut uint32_t host_index; / The host's index for putting or getting,
    pub /: *mut *mut uint32_t hba_index; / The last known hba index for get or put,
    pub q_mode: u32,
    pub /: *mut *mut *mut lpfc_sli_ring pring; / ptr to io ring associated with q,
    pub /: *mut *mut *mut lpfc_rqb rqbp; / ptr to RQ buffers,
    pub /: *mut *mut uint16_t page_count; / Number of pages allocated for this queue,
    pub /: *mut *mut uint16_t page_size; / size of page allocated for this queue,
pub const LPFC_EXPANDED_PAGE_SIZE: c_int = 16384;
pub const LPFC_DEFAULT_PAGE_SIZE: c_int = 4096;
    pub /: *mut *mut uint16_t chann; / Hardware Queue association WQ/CQ,
// CPU affinity for EQ
pub const LPFC_FIND_BY_EQ: c_int = 0;
pub const LPFC_FIND_BY_HDWQ: c_int = 1;
    pub db_format: u8,
pub const LPFC_DB_RING_FORMAT: c_uint = 0x01;
pub const LPFC_DB_LIST_FORMAT: c_uint = 0x02;
    pub q_flag: u8,
pub const HBA_NVMET_WQFULL: c_uint = 0x1 /* We hit WQ Full condition for NVMET */;
pub const HBA_NVMET_CQ_NOTIFY: c_uint = 0x1 /* LPFC_NVMET_CQ_NOTIFY CQEs this EQE */;
pub const HBA_EQ_DELAY_CHK: c_uint = 0x2 /* EQ is a candidate for coalescing */;
pub const LPFC_NVMET_CQ_NOTIFY: c_int = 4;
    pub db_regaddr: *mut void __iomem,
    pub dpp_enable: u16,
    pub dpp_id: u16,
    pub dpp_regaddr: *mut void __iomem,
// For q stats
    pub q_cnt_1: u32,
    pub q_cnt_2: u32,
    pub q_cnt_3: u32,
    pub q_cnt_4: u64,
    pub q_cnt_5: u32,
// defines for EQ stats

// defines for CQ stats

// defines for WQ stats

// defines for RQ stats

    pub irqwork: work_struct,
    pub spwork: work_struct,
    pub sched_irqwork: delayed_work,
    pub sched_spwork: delayed_work,
    pub isr_timestamp: u64,
    pub assoc_qp: *mut lpfc_queue,
    pub _poll_list: list_head,
    pub poll_mode: lpfc_poll_mode,
    pub /: *mut *mut *mut void q_pgs[]; / array to index entries per page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli4_link {
    pub speed: u32,
    pub duplex: u8,
    pub status: u8,
    pub type: u8,
    pub number: u8,
    pub fault: u8,
    pub link_status: u8,
    pub topology: u16,
    pub logical_speed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fcf_rec {
    pub fabric_name: [u8; 8],
    pub switch_name: [u8; 8],
    pub mac_addr: [u8; 6],
    pub fcf_indx: u16,
    pub priority: u32,
    pub vlan_id: u16,
    pub addr_mode: u32,
    pub flag: u32,
pub const BOOT_ENABLE: c_uint = 0x01;
pub const RECORD_VALID: c_uint = 0x02;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fcf_pri_rec {
    pub fcf_index: u16,
pub const LPFC_FCF_ON_PRI_LIST: c_uint = 0x0001;
pub const LPFC_FCF_FLOGI_FAILED: c_uint = 0x0002;
    pub flag: u16,
    pub priority: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fcf_pri {
    pub list: list_head,
    pub fcf_rec: lpfc_fcf_pri_rec,
}

//
// Maximum FCF table index, it is for driver internal book keeping, it
// just needs to be no less than the supported HBA's FCF table size.
//
pub const LPFC_SLI4_FCF_TBL_INDX_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fcf {
    pub fcfi: u16,
    pub fcf_flag: u32,
pub const FCF_AVAILABLE: c_uint = 0x01 /* FCF available for discovery */;
pub const FCF_REGISTERED: c_uint = 0x02 /* FCF registered with FW */;
pub const FCF_SCAN_DONE: c_uint = 0x04 /* FCF table scan done */;
pub const FCF_IN_USE: c_uint = 0x08 /* At Least one discovery completed */;
pub const FCF_INIT_DISC: c_uint = 0x10 /* Initial FCF discovery */;
pub const FCF_DEAD_DISC: c_uint = 0x20 /* FCF DEAD fast FCF failover discovery */;
pub const FCF_ACVL_DISC: c_uint = 0x40 /* All CVL fast FCF failover discovery */;

pub const FCF_REDISC_PEND: c_uint = 0x80 /* FCF rediscovery pending */;
pub const FCF_REDISC_EVT: c_uint = 0x100 /* FCF rediscovery event to worker thread */;
pub const FCF_REDISC_FOV: c_uint = 0x200 /* Post FCF rediscovery fast failover */;

    pub fcf_redisc_attempted: u16,
    pub addr_mode: u32,
    pub eligible_fcf_cnt: u32,
    pub current_rec: lpfc_fcf_rec,
    pub failover_rec: lpfc_fcf_rec,
    pub fcf_pri_list: list_head,
    pub fcf_pri: [lpfc_fcf_pri; LPFC_SLI4_FCF_TBL_INDX_MAX],
    pub current_fcf_scan_pri: u32,
    pub redisc_wait: timer_list,
    pub /: *mut *mut *mut unsigned long fcf_rr_bmask; / Eligible FCF indexes for RR failover,
}

pub const LPFC_REGION23_VERSION: c_int = 1;
pub const LPFC_REGION23_LAST_REC: c_uint = 0xff;
pub const DRIVER_SPECIFIC_TYPE: c_uint = 0xA2;
pub const LINUX_DRIVER_ID: c_uint = 0x20;
pub const PORT_STE_TYPE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fip_param_hdr {
    pub type: u8,
pub const FCOE_PARAM_TYPE: c_uint = 0xA0;
    pub length: u8,
pub const FCOE_PARAM_LENGTH: c_int = 2;
    pub parm_version: u8,
pub const FIPP_VERSION: c_uint = 0x01;
    pub parm_flags: u8,
pub const lpfc_fip_param_hdr_fipp_mode_SHIFT: c_int = 6;
pub const lpfc_fip_param_hdr_fipp_mode_MASK: c_uint = 0x3;

pub const FIPP_MODE_ON: c_uint = 0x1;
pub const FIPP_MODE_OFF: c_uint = 0x0;
pub const FIPP_VLAN_VALID: c_uint = 0x1;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fcoe_params {
    pub fc_map: [u8; 3],
    pub reserved1: u8,
    pub vlan_tag: u16,
    pub reserved: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fcf_conn_hdr {
    pub type: u8,
pub const FCOE_CONN_TBL_TYPE: c_uint = 0xA1;
    pub /: *mut *mut uint8_t length; / words,
    pub reserved: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fcf_conn_rec {
    pub flags: u16,
pub const FCFCNCT_VALID: c_uint = 0x0001;
pub const FCFCNCT_BOOT: c_uint = 0x0002;
pub const FCFCNCT_PRIMARY: c_uint = 0x0004   /* if not set, Secondary */;
pub const FCFCNCT_FBNM_VALID: c_uint = 0x0008;
pub const FCFCNCT_SWNM_VALID: c_uint = 0x0010;
pub const FCFCNCT_VLAN_VALID: c_uint = 0x0020;
pub const FCFCNCT_AM_VALID: c_uint = 0x0040;
pub const FCFCNCT_AM_PREFERRED: c_uint = 0x0080   /* if not set, AM Required */;
pub const FCFCNCT_AM_SPMA: c_uint = 0x0100	 /* if not set, FPMA */;
    pub vlan_tag: u16,
    pub fabric_name: [u8; 8],
    pub switch_name: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fcf_conn_entry {
    pub list: list_head,
    pub conn_rec: lpfc_fcf_conn_rec,
}

//
// Define the host's bootstrap mailbox.  This structure contains
// the member attributes needed to create, use, and destroy the
// bootstrap mailbox region.
//
// The macro definitions for the bmbx data structure are defined
// in lpfc_hw4.h with the register definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_bmbx {
    pub dmabuf: *mut lpfc_dmabuf,
    pub dma_address: dma_address,
    pub avirt: *mut c_void,
    pub aphys: dma_addr_t,
    pub bmbx_size: u32,
}

pub const LPFC_EQE_SIZE_4B: c_int = 4;
pub const LPFC_EQE_SIZE_16B: c_int = 16;
pub const LPFC_CQE_SIZE: c_int = 16;
pub const LPFC_WQE_SIZE: c_int = 64;
pub const LPFC_WQE128_SIZE: c_int = 128;
pub const LPFC_MQE_SIZE: c_int = 256;
pub const LPFC_RQE_SIZE: c_int = 8;
pub const LPFC_EQE_DEF_COUNT: c_int = 1024;
pub const LPFC_CQE_DEF_COUNT: c_int = 1024;
pub const LPFC_CQE_EXP_COUNT: c_int = 4096;
pub const LPFC_WQE_DEF_COUNT: c_int = 256;
pub const LPFC_WQE_EXP_COUNT: c_int = 1024;
pub const LPFC_MQE_DEF_COUNT: c_int = 16;
pub const LPFC_RQE_DEF_COUNT: c_int = 512;

//
// SLI4 CT field defines
//
pub const SLI4_CT_RPI: c_int = 0;
pub const SLI4_CT_VPI: c_int = 1;
pub const SLI4_CT_VFI: c_int = 2;
pub const SLI4_CT_FCFI: c_int = 3;
//
// SLI4 specific data structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_max_cfg_param {
    pub max_xri: u16,
    pub xri_base: u16,
    pub xri_used: u16,
    pub max_rpi: u16,
    pub rpi_base: u16,
    pub rpi_used: u16,
    pub max_vpi: u16,
    pub vpi_base: u16,
    pub vpi_used: u16,
    pub max_vfi: u16,
    pub vfi_base: u16,
    pub vfi_used: u16,
    pub max_fcfi: u16,
    pub fcfi_used: u16,
    pub max_eq: u16,
    pub max_rq: u16,
    pub max_cq: u16,
    pub max_wq: u16,
}

// SLI4 HBA multi-fcp queue handler struct
pub const LPFC_SLI4_HANDLER_NAME_SZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_hba_eq_hdl {
    pub idx: u32,
    pub irq: c_int,
    pub handler_name: [c_char; LPFC_SLI4_HANDLER_NAME_SZ],
    pub phba: *mut lpfc_hba,
    pub eq: *mut lpfc_queue,
    pub aff_mask: cpumask,
}

// BB Credit recovery value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_bbscn_params {
    pub word0: u32,
pub const lpfc_bbscn_min_SHIFT: c_int = 0;
pub const lpfc_bbscn_min_MASK: c_uint = 0x0000000F;

pub const lpfc_bbscn_max_SHIFT: c_int = 4;
pub const lpfc_bbscn_max_MASK: c_uint = 0x0000000F;

pub const lpfc_bbscn_def_SHIFT: c_int = 8;
pub const lpfc_bbscn_def_MASK: c_uint = 0x0000000F;

}

// Port Capabilities for SLI4 Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_pc_sli4_params {
    pub supported: u32,
    pub if_type: u32,
    pub sli_rev: u32,
    pub sli_family: u32,
    pub featurelevel_1: u32,
    pub featurelevel_2: u32,
    pub proto_types: u32,
pub const LPFC_SLI4_PROTO_FCOE: c_uint = 0x0000001;
pub const LPFC_SLI4_PROTO_FC: c_uint = 0x0000002;
pub const LPFC_SLI4_PROTO_NIC: c_uint = 0x0000004;
pub const LPFC_SLI4_PROTO_ISCSI: c_uint = 0x0000008;
pub const LPFC_SLI4_PROTO_RDMA: c_uint = 0x0000010;
    pub sge_supp_len: u32,
    pub if_page_sz: u32,
    pub rq_db_window: u32,
    pub loopbk_scope: u32,
    pub oas_supported: u32,
    pub eq_pages_max: u32,
    pub eqe_size: u32,
    pub cq_pages_max: u32,
    pub cqe_size: u32,
    pub mq_pages_max: u32,
    pub mqe_size: u32,
    pub mq_elem_cnt: u32,
    pub wq_pages_max: u32,
    pub wqe_size: u32,
    pub rq_pages_max: u32,
    pub rqe_size: u32,
    pub hdr_pages_max: u32,
    pub hdr_size: u32,
    pub hdr_pp_align: u32,
    pub sgl_pages_max: u32,
    pub sgl_pp_align: u32,
    pub mib_size: u32,
    pub mi_ver: u16,
pub const LPFC_MIB1_SUPPORT: c_int = 1;
pub const LPFC_MIB2_SUPPORT: c_int = 2;
pub const LPFC_MIB3_SUPPORT: c_int = 3;
    pub mi_value: u16,
pub const LPFC_DFLT_MIB_VAL: c_int = 2;
    pub mi_cap: u8,
    pub mib_bde_cnt: u8,
    pub cmf: u8,
    pub cqv: u8,
    pub mqv: u8,
    pub wqv: u8,
    pub rqv: u8,
    pub eqav: u8,
    pub cqav: u8,
    pub wqsize: u8,
    pub bv1s: u8,
    pub pls: u8,
pub const LPFC_WQ_SZ64_SUPPORT: c_int = 1;
pub const LPFC_WQ_SZ128_SUPPORT: c_int = 2;
    pub wqpcnt: u8,
    pub nvme: u8,
}

pub const LPFC_CQ_4K_PAGE_SZ: c_uint = 0x1;
pub const LPFC_CQ_16K_PAGE_SZ: c_uint = 0x4;
pub const LPFC_CQ_32K_PAGE_SZ: c_uint = 0x8;
pub const LPFC_WQ_4K_PAGE_SZ: c_uint = 0x1;
pub const LPFC_WQ_16K_PAGE_SZ: c_uint = 0x4;
pub const LPFC_WQ_32K_PAGE_SZ: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_iov {
    pub pf_number: u32,
    pub vf_number: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli4_lnk_info {
    pub lnk_dv: u8,
pub const LPFC_LNK_DAT_INVAL: c_int = 0;
pub const LPFC_LNK_DAT_VAL: c_int = 1;
    pub lnk_tp: u8,
pub const LPFC_LNK_GE: c_uint = 0x0 /* FCoE */;
pub const LPFC_LNK_FC: c_uint = 0x1 /* FC */;
pub const LPFC_LNK_FC_TRUNKED: c_uint = 0x2 /* FC_Trunked */;
    pub lnk_no: u8,
    pub optic_state: u8,
}

// Used for tracking CPU mapping attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vector_map_info {
    pub phys_id: u16,
    pub core_id: u16,
    pub eq: u16,
    pub hdwq: u16,
    pub flag: u16,
pub const LPFC_CPU_MAP_HYPER: c_uint = 0x1;
pub const LPFC_CPU_MAP_UNASSIGN: c_uint = 0x2;
pub const LPFC_CPU_FIRST_IRQ: c_uint = 0x4;
}

pub const LPFC_VECTOR_MAP_EMPTY: c_uint = 0xffff;
pub const LPFC_IRQ_EMPTY: c_uint = 0xffffffff;
// Multi-XRI pool
pub const XRI_BATCH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_pbl_pool {
    pub list: list_head,
    pub count: u32,
    pub pbl_pool*/: *mut *mut spinlock_t lock; / lock for,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_pvt_pool {
    pub low_watermark: u32,
    pub high_watermark: u32,
    pub list: list_head,
    pub count: u32,
    pub /: *mut *mut spinlock_t lock; / lock for pvt_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_multixri_pool {
    pub xri_limit: u32,
// Starting point when searching a pbl_pool with round-robin method
    pub rrb_next_hwqid: u32,
// Used by lpfc_adjust_pvt_pool_count.
// io_req_count is incremented by 1 during IO submission. The heartbeat
// handler uses these two variables to determine if pvt_pool is idle or
// busy.
//
    pub prev_io_req_count: u32,
    pub io_req_count: u32,
// statistics
    pub pbl_empty_count: u32,

    pub above_limit_count: u32,
    pub below_limit_count: u32,
    pub local_pbl_hit_count: u32,
    pub other_pbl_hit_count: u32,
    pub stat_max_hwm: u32,

    pub stat_pbl_count: u32,
    pub stat_pvt_count: u32,
    pub stat_busy_count: u32,
    pub stat_snapshot_taken: u32,

// TODO: Separate pvt_pool into get and put list
    pub /: *mut *mut lpfc_pbl_pool pbl_pool; / Public free XRI pool,
    pub /: *mut *mut lpfc_pvt_pool pvt_pool; / Private free XRI pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fc4_ctrl_stat {
    pub input_requests: u32,
    pub output_requests: u32,
    pub control_requests: u32,
    pub io_cmpls: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_lock_stat {
    pub alloc_xri_get: u32,
    pub alloc_xri_put: u32,
    pub free_xri: u32,
    pub wq_access: u32,
    pub alloc_pvt_pool: u32,
    pub mv_from_pvt_pool: u32,
    pub mv_to_pub_pool: u32,
    pub mv_to_pvt_pool: u32,
    pub free_pub_pool: u32,
    pub free_pvt_pool: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_eq_intr_info {
    pub list: list_head,
    pub icnt: u32,
}

// SLI4 HBA data structure entries
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli4_hdw_queue {
// Pointers to the constructed SLI4 queues
    pub /: *mut *mut *mut lpfc_queue hba_eq; / Event queues for HBA,
    pub /: *mut *mut *mut lpfc_queue io_cq; / Fast-path FCP & NVME compl queue,
    pub /: *mut *mut *mut lpfc_queue io_wq; / Fast-path FCP & NVME work queue,
    pub io_cq_map: u16,
// Keep track of IO buffers for this hardware queue
    pub /: *mut *mut spinlock_t io_buf_list_get_lock; / Common buf alloc list lock,
    pub lpfc_io_buf_list_get: list_head,
    pub /: *mut *mut spinlock_t io_buf_list_put_lock; / Common buf free list lock,
    pub lpfc_io_buf_list_put: list_head,
    pub /: *mut *mut spinlock_t abts_io_buf_list_lock; / list of aborted IOs,
    pub lpfc_abts_io_buf_list: list_head,
    pub total_io_bufs: u32,
    pub get_io_bufs: u32,
    pub put_io_bufs: u32,
    pub empty_io_bufs: u32,
    pub abts_scsi_io_bufs: u32,
    pub abts_nvme_io_bufs: u32,
// Multi-XRI pool per HWQ
    pub p_multixri_pool: *mut lpfc_multixri_pool,
// FC-4 Stats counters
    pub nvme_cstat: lpfc_fc4_ctrl_stat,
    pub scsi_cstat: lpfc_fc4_ctrl_stat,

    pub lock_conflict: lpfc_lock_stat,

// Per HDWQ pool resources
    pub sgl_list: list_head,
    pub cmd_rsp_buf_list: list_head,
// Lock for syncing Per HDWQ pool resources
    pub hdwq_lock: spinlock_t,
}

// compile time trylock stats

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_hdwq_stat {
    pub hdwq_no: u32,
    pub rcv_io: u32,
    pub xmt_io: u32,
    pub cmpl_io: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sli4_hba {
    pub for: *mut *mut *mut void __iomem conf_regs_memmap_p; / Kernel memory mapped address,
// config space registers
//
    pub for: *mut *mut *mut void __iomem ctrl_regs_memmap_p; / Kernel memory mapped address,
// control registers
//
    pub for: *mut *mut *mut void __iomem drbl_regs_memmap_p; / Kernel memory mapped address,
// doorbell registers
//
    pub for: *mut *mut *mut void __iomem dpp_regs_memmap_p; / Kernel memory mapped address,
// dpp registers
//
    pub for: *mut *mut *mut void __iomem dpp_regs_memmap_wc_p;/ Kernel memory mapped address,
// dpp registers with write combining
//
// IF Type 0, BAR 0 PCI cfg space reg mem map
    pub UERRLOregaddr: *mut void __iomem,
    pub UERRHIregaddr: *mut void __iomem,
    pub UEMASKLOregaddr: *mut void __iomem,
    pub UEMASKHIregaddr: *mut void __iomem,
    pub if_type0: },
// IF Type 2, BAR 0 PCI cfg space reg mem map.
    pub STATUSregaddr: *mut void __iomem,
    pub CTRLregaddr: *mut void __iomem,
    pub ERR1regaddr: *mut void __iomem,
pub const SLIPORT_ERR1_REG_ERR_CODE_1: c_uint = 0x1;
pub const SLIPORT_ERR1_REG_ERR_CODE_2: c_uint = 0x2;
    pub ERR2regaddr: *mut void __iomem,
pub const SLIPORT_ERR2_REG_FW_RESTART: c_uint = 0x0;
pub const SLIPORT_ERR2_REG_FUNC_PROVISON: c_uint = 0x1;
pub const SLIPORT_ERR2_REG_FORCED_DUMP: c_uint = 0x2;
pub const SLIPORT_ERR2_REG_FAILURE_EQ: c_uint = 0x3;
pub const SLIPORT_ERR2_REG_FAILURE_CQ: c_uint = 0x4;
pub const SLIPORT_ERR2_REG_FAILURE_BUS: c_uint = 0x5;
pub const SLIPORT_ERR2_REG_FAILURE_RQ: c_uint = 0x6;
    pub EQDregaddr: *mut void __iomem,
    pub if_type2: },
    pub u: },
// IF type 0, BAR1 and if type 2, Bar 0 CSR register memory map
    pub PSMPHRregaddr: *mut void __iomem,
// Well-known SLI INTF register memory map.
    pub SLIINTFregaddr: *mut void __iomem,
// IF type 0, BAR 1 function CSR register memory map
    pub /: *mut *mut *mut void __iomem ISRregaddr; / HST_ISR register,
    pub /: *mut *mut *mut void __iomem IMRregaddr; / HST_IMR register,
    pub /: *mut *mut *mut void __iomem ISCRregaddr; / HST_ISCR register,
// IF type 0, BAR 0 and if type 2, BAR 0 doorbell register memory map
    pub /: *mut *mut *mut void __iomem RQDBregaddr; / RQ_DOORBELL register,
    pub /: *mut *mut *mut void __iomem WQDBregaddr; / WQ_DOORBELL register,
    pub /: *mut *mut *mut void __iomem CQDBregaddr; / CQ_DOORBELL register,
    pub /: *mut *mut *mut void __iomem EQDBregaddr; / EQ_DOORBELL register,
    pub /: *mut *mut *mut void __iomem MQDBregaddr; / MQ_DOORBELL register,
    pub /: *mut *mut *mut void __iomem BMBXregaddr; / BootStrap MBX register,
    pub ue_mask_lo: u32,
    pub ue_mask_hi: u32,
    pub ue_to_sr: u32,
    pub ue_to_rp: u32,
    pub sli_intf: lpfc_register,
    pub asic_id: lpfc_register,
    pub pc_sli4_params: lpfc_pc_sli4_params,
    pub bbscn_params: lpfc_bbscn_params,
    pub /: *mut *mut *mut lpfc_hba_eq_hdl hba_eq_hdl; / HBA per-WQ handle,
    pub q): *mut *mut void (sli4_eq_clr_intr)(struct lpfc_queue,
    pub arm): uint32_t count, bool,
    pub arm): uint32_t count, bool,
// Pointers to the constructed SLI4 queues
    pub hdwq: *mut lpfc_sli4_hdw_queue,
    pub lpfc_wq_list: list_head,
// Pointers to the constructed SLI4 queues for NVMET
    pub /: *mut *mut *mut *mut lpfc_queue nvmet_cqset; / Fast-path NVMET CQ Set queues,
    pub /: *mut *mut *mut *mut lpfc_queue nvmet_mrq_hdr; / Fast-path NVMET hdr MRQs,
    pub /: *mut *mut *mut *mut lpfc_queue nvmet_mrq_data; / Fast-path NVMET data MRQs,
    pub /: *mut *mut *mut lpfc_queue mbx_cq; / Slow-path mailbox complete queue,
    pub /: *mut *mut *mut lpfc_queue els_cq; / Slow-path ELS response complete queue,
    pub /: *mut *mut *mut lpfc_queue nvmels_cq; / NVME LS complete queue,
    pub /: *mut *mut *mut lpfc_queue mbx_wq; / Slow-path MBOX work queue,
    pub /: *mut *mut *mut lpfc_queue els_wq; / Slow-path ELS work queue,
    pub /: *mut *mut *mut lpfc_queue nvmels_wq; / NVME LS work queue,
    pub /: *mut *mut *mut lpfc_queue hdr_rq; / Slow-path Header Receive queue,
    pub /: *mut *mut *mut lpfc_queue dat_rq; / Slow-path Data Receive queue,
    pub wwnn: lpfc_name,
    pub wwpn: lpfc_name,
    pub /: *mut *mut uint32_t fw_func_mode; / FW function protocol mode,
// Optimized Access Storage specific queues/structures
    pub oas_next_lun: u64,
    pub oas_next_tgt_wwpn: [u8; 8],
    pub oas_next_vpt_wwpn: [u8; 8],
// Setup information for various queue parameters
    pub eq_esize: c_int,
    pub eq_ecount: c_int,
    pub cq_esize: c_int,
    pub cq_ecount: c_int,
    pub wq_esize: c_int,
    pub wq_ecount: c_int,
    pub mq_esize: c_int,
    pub mq_ecount: c_int,
    pub rq_esize: c_int,
    pub rq_ecount: c_int,
pub const LPFC_SP_EQ_MAX_INTR_SEC: c_int = 10000;
pub const LPFC_FP_EQ_MAX_INTR_SEC: c_int = 10000;
    pub intr_enable: u32,
// Indicates whether SLI Port supports FEDIF
    pub encryption_support: bool,
    pub bmbx: lpfc_bmbx,
    pub max_cfg_param: lpfc_max_cfg_param,
    pub /: *mut *mut uint16_t extents_in_use; / must allocate resource extents.,
    pub /: *mut *mut uint16_t rpi_hdrs_in_use; / must post rpi hdrs if set.,
    pub /: *mut *mut uint16_t next_xri; / last_xri - max_cfg_param.xri_base = used,
    pub next_rpi: u16,
    pub io_xri_max: u16,
    pub io_xri_cnt: u16,
    pub io_xri_start: u16,
    pub els_xri_cnt: u16,
    pub nvmet_xri_cnt: u16,
    pub nvmet_io_wait_cnt: u16,
    pub nvmet_io_wait_total: u16,
    pub cq_max: u16,
    pub cq_lookup: *mut lpfc_queue,
    pub lpfc_els_sgl_list: list_head,
    pub lpfc_abts_els_sgl_list: list_head,
    pub /: *mut *mut spinlock_t abts_io_buf_list_lock; / list of aborted SCSI IOs,
    pub lpfc_abts_io_buf_list: list_head,
    pub lpfc_nvmet_sgl_list: list_head,
    pub /: *mut *mut spinlock_t abts_nvmet_buf_list_lock; / list of aborted NVMET IOs,
    pub lpfc_abts_nvmet_ctx_list: list_head,
    pub /: *mut *mut spinlock_t t_active_list_lock; / list of active NVMET IOs,
    pub t_active_ctx_list: list_head,
    pub lpfc_nvmet_io_wait_list: list_head,
    pub nvmet_ctx_info: *mut lpfc_nvmet_ctx_info,
    pub lpfc_sglq_active_list: *mut lpfc_sglq,
    pub lpfc_rpi_hdr_list: list_head,
    pub rpi_bmask: *mut c_ulong,
    pub rpi_ids: *mut u16,
    pub rpi_count: u16,
    pub lpfc_rpi_blk_list: list_head,
    pub xri_bmask: *mut c_ulong,
    pub xri_ids: *mut u16,
    pub lpfc_xri_blk_list: list_head,
    pub vfi_bmask: *mut c_ulong,
    pub vfi_ids: *mut u16,
    pub vfi_count: u16,
    pub lpfc_vfi_blk_list: list_head,
    pub sli4_flags: lpfc_sli4_flags,
    pub sp_queue_event: list_head,
    pub sp_cqe_event_pool: list_head,
    pub sp_asynce_work_queue: list_head,
    pub /: *mut *mut spinlock_t asynce_list_lock; / protect sp_asynce_work_queue list,
    pub sp_els_xri_aborted_work_queue: list_head,
    pub /: *mut *mut spinlock_t els_xri_abrt_list_lock; / protect els_xri_aborted list,
    pub sp_unsol_work_queue: list_head,
    pub link_state: lpfc_sli4_link,
    pub lnk_info: lpfc_sli4_lnk_info,
    pub pport_name_sta: u32,
pub const LPFC_SLI4_PPNAME_NON: c_int = 0;
pub const LPFC_SLI4_PPNAME_GET: c_int = 1;
    pub iov: lpfc_iov,
    pub /: *mut *mut spinlock_t sgl_list_lock; / list of aborted els IOs,
    pub /: *mut *mut spinlock_t nvmet_io_wait_lock; / IOs waiting for ctx resources,
    pub physical_port: u32,
// CPU to vector mapping information
    pub cpu_map: *mut lpfc_vector_map_info,
    pub num_possible_cpu: u16,
    pub num_present_cpu: u16,
    pub irq_aff_mask: cpumask,
    pub curr_disp_cpu: u16,
    pub eq_info: *mut lpfc_eq_intr_info __percpu,

    pub c_stat: *mut lpfc_hdwq_stat __percpu,

    pub idle_stat: *mut lpfc_idle_stat,
    pub conf_trunk: u32,

pub const lpfc_conf_trunk_port0_SHIFT: c_int = 0;
pub const lpfc_conf_trunk_port0_MASK: c_uint = 0x1;

pub const lpfc_conf_trunk_port1_SHIFT: c_int = 1;
pub const lpfc_conf_trunk_port1_MASK: c_uint = 0x1;

pub const lpfc_conf_trunk_port2_SHIFT: c_int = 2;
pub const lpfc_conf_trunk_port2_MASK: c_uint = 0x1;

pub const lpfc_conf_trunk_port3_SHIFT: c_int = 3;
pub const lpfc_conf_trunk_port3_MASK: c_uint = 0x1;

pub const lpfc_conf_trunk_port0_nd_SHIFT: c_int = 4;
pub const lpfc_conf_trunk_port0_nd_MASK: c_uint = 0x1;

pub const lpfc_conf_trunk_port1_nd_SHIFT: c_int = 5;
pub const lpfc_conf_trunk_port1_nd_MASK: c_uint = 0x1;

pub const lpfc_conf_trunk_port2_nd_SHIFT: c_int = 6;
pub const lpfc_conf_trunk_port2_nd_MASK: c_uint = 0x1;

pub const lpfc_conf_trunk_port3_nd_SHIFT: c_int = 7;
pub const lpfc_conf_trunk_port3_nd_MASK: c_uint = 0x1;
    pub flash_id: u8,
    pub asic_rev: u8,
    pub /: *mut *mut uint16_t fawwpn_flag; / FA-WWPN support state,
pub const LPFC_FAWWPN_CONFIG: c_uint = 0x1 /* FA-PWWN is configured */;
pub const LPFC_FAWWPN_FABRIC: c_uint = 0x2 /* FA-PWWN success with Fabric */;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_sge_type {
    GEN_BUFF_TYPE,
    SCSI_BUFF_TYPE,
    NVMET_BUFF_TYPE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_sgl_state {
    SGL_FREED,
    SGL_ALLOCATED,
    SGL_XRI_ABORTED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_sglq {
// lpfc_sglqs are used in double linked lists
    pub list: list_head,
    pub clist: list_head,
    pub /: *mut *mut lpfc_sge_type buff_type; / is this a scsi sgl,
    pub state: lpfc_sgl_state,
    pub /: *mut *mut *mut lpfc_nodelist ndlp; / ndlp associated with IO,
    pub /: *mut *mut uint16_t iotag; / pre-assigned IO tag,
    pub /: *mut *mut uint16_t sli4_lxritag; / logical pre-assigned xri.,
    pub /: *mut *mut uint16_t sli4_xritag; / pre-assigned XRI, (OXID) tag.,
    pub /: *mut *mut *mut sli4_sge sgl; / pre-assigned SGL,
    pub /: *mut *mut *mut void virt; / virtual address.,
    pub /: *mut *mut dma_addr_t phys; / physical address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rpi_hdr {
    pub list: list_head,
    pub len: u32,
    pub dmabuf: *mut lpfc_dmabuf,
    pub page_count: u32,
    pub start_rpi: u32,
    pub next_rpi: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rsrc_blks {
    pub list: list_head,
    pub rsrc_start: u16,
    pub rsrc_size: u16,
    pub rsrc_used: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rdp_context {
    pub ndlp: *mut lpfc_nodelist,
    pub ox_id: u16,
    pub rx_id: u16,
    pub link_stat: READ_LNK_VAR,
    pub page_a0: [u8; DMP_SFF_PAGE_A0_SIZE],
    pub page_a2: [u8; DMP_SFF_PAGE_A2_SIZE],
    pub int): *mut *mut *mut *mut void (cmpl)(struct lpfc_hba , struct lpfc_rdp_context,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_lcb_context {
    pub sub_command: u8,
    pub type: u8,
    pub capability: u8,
    pub frequency: u8,
    pub duration: u16,
    pub ox_id: u16,
    pub rx_id: u16,
    pub ndlp: *mut lpfc_nodelist,
}

//
// SLI4 specific function prototypes
//
extern "C" {
    pub fn lpfc_pci_function_reset(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_pdev_status_reg_wait(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_hba_setup(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_mbox_cmd_free(: *mut lpfc_hba, : *mut lpfcMboxq);
}
extern "C" {
    pub fn lpfc_sli4_mbx_sge_set(: *mut lpfcMboxq, _arg: u32, _arg: dma_addr_t, _arg: u32);
}
extern "C" {
    pub fn lpfc_sli4_hba_reset(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_queue_free(: *mut lpfc_queue);
}
extern "C" {
    pub fn lpfc_eq_create(: *mut lpfc_hba, : *mut lpfc_queue, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_eq_destroy(: *mut lpfc_hba, : *mut lpfc_queue) -> c_int;
}
extern "C" {
    pub fn lpfc_cq_destroy(: *mut lpfc_hba, : *mut lpfc_queue) -> c_int;
}
extern "C" {
    pub fn lpfc_mq_destroy(: *mut lpfc_hba, : *mut lpfc_queue) -> c_int;
}
extern "C" {
    pub fn lpfc_wq_destroy(: *mut lpfc_hba, : *mut lpfc_queue) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_queue_setup(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_queue_unset(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_post_sgl(: *mut lpfc_hba, _arg: dma_addr_t, _arg: dma_addr_t, _arg: u16) -> c_int;
}
extern "C" {
    pub fn lpfc_repost_io_sgl_list(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_next_xritag(: *mut lpfc_hba) -> u16;
}
extern "C" {
    pub fn lpfc_sli4_free_xri(: *mut lpfc_hba, _arg: c_int);
}
extern "C" {
    pub fn lpfc_sli4_post_async_mbox(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn __lpfc_sli4_cq_event_release(: *mut lpfc_hba, : *mut lpfc_cq_event);
}
extern "C" {
    pub fn lpfc_sli4_cq_event_release(: *mut lpfc_hba, : *mut lpfc_cq_event);
}
extern "C" {
    pub fn lpfc_sli4_init_rpi_hdrs(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_post_rpi_hdr(: *mut lpfc_hba, : *mut lpfc_rpi_hdr) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_post_all_rpi_hdrs(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_remove_rpi_hdrs(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_alloc_rpi(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_free_rpi(: *mut lpfc_hba, _arg: c_int);
}
extern "C" {
    pub fn lpfc_sli4_remove_rpis(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_async_event_proc(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_fcf_redisc_event_proc(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_els_xri_abort_event_proc(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_vport_delete_els_xri_aborted(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_sli4_vport_delete_fcp_xri_aborted(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_sli4_brdreset(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_add_fcf_record(: *mut lpfc_hba, : *mut fcf_record) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_remove_dflt_fcf(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_get_els_iocb_cnt(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_get_iocb_cnt(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_init_vpi(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_eq_clr_intr(: *mut lpfc_queue);
}
extern "C" {
    pub fn lpfc_sli4_if6_eq_clr_intr(q: *mut lpfc_queue);
}
extern "C" {
    pub fn lpfc_sli4_fcfi_unreg(: *mut lpfc_hba, _arg: u16);
}
extern "C" {
    pub fn lpfc_sli4_fcf_scan_read_fcf_rec(: *mut lpfc_hba, _arg: u16) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_fcf_rr_read_fcf_rec(: *mut lpfc_hba, _arg: u16) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_read_fcf_rec(: *mut lpfc_hba, _arg: u16) -> c_int;
}
extern "C" {
    pub fn lpfc_mbx_cmpl_fcf_scan_read_fcf_rec(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_fcf_rr_read_fcf_rec(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_read_fcf_rec(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_sli4_unregister_fcf(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_post_status_check(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_config_mbox_subsys_get(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t) -> u8;
}
extern "C" {
    pub fn lpfc_sli_config_mbox_opcode_get(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t) -> u8;
}
extern "C" {
    pub fn lpfc_sli4_ras_dma_free(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_put_sgl_per_hdwq(phba: *mut lpfc_hba, buf: *mut lpfc_io_buf) -> c_int;
}
//
// lpfc_sli4_unrecoverable_port - Check ERR and RN bits in portstat_reg
// @portstat_reg: portstat_reg pointer containing portstat_reg contents
//
// Description:
// Use only for SLI4 interface type-2 or later.  If ERR is set && RN is 0, then
// port is deemed unrecoverable.
//
// Returns:
// true		- ERR && !RN
// false	- otherwise
//
