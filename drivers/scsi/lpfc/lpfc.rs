//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc.h
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
// Copyright (C) 2004-2016 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
// Portions Copyright (C) 2004-2005 Christoph Hellwig
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

pub const ELX_MODEL_NAME_SIZE: c_int = 80;
pub const ELX_FW_NAME_SIZE: c_int = 84;
pub const LPFC_PCI_DEV_LP: c_uint = 0x1;
pub const LPFC_PCI_DEV_OC: c_uint = 0x2;
pub const LPFC_SLI_REV2: c_int = 2;
pub const LPFC_SLI_REV3: c_int = 3;
pub const LPFC_SLI_REV4: c_int = 4;

pub const LPFC_DEFAULT_XPSGL_SIZE: c_int = 256;
pub const LPFC_MAX_SG_TABLESIZE: c_uint = 0xffff;
pub const LPFC_MIN_SG_SLI4_BUF_SZ: c_uint = 0x800	/* based on LPFC_DEFAULT_SG_SEG_CNT */;

pub const LPFC_MAX_SGE_SIZE: c_uint = 0x80000000 /* Maximum data allowed in a SGE */;

pub const LPFC_MIN_TGT_QDEPTH: c_int = 10;
pub const LPFC_MAX_TGT_QDEPTH: c_uint = 0xFFFF;
//
// Following time intervals are used of adjusting SCSI device
// queue depths when there are driver resource error or Firmware
// resource error.
//

// Number of exchanges reserved for discovery to complete
pub const LPFC_DISC_IOCB_BUFF_COUNT: c_int = 20;

// Error Attention event polling interval

// Define macros for 64 bit support

// Provide maximum configuration definitions.

pub const FC_MAX_ADPTMSG: c_int = 64;
pub const MAX_HBAEVT: c_int = 32;
pub const MAX_HBAS_NO_RESET: c_int = 16;
// Number of MSI-X vectors the driver uses
pub const LPFC_MSIX_VECTORS: c_int = 2;
// lpfc wait event data ready flag

// queue dump line buffer size
pub const LPFC_LBUF_SZ: c_int = 128;
// mailbox system shutdown options
pub const LPFC_MBX_NO_WAIT: c_int = 0;
pub const LPFC_MBX_WAIT: c_int = 1;
pub const LPFC_CFG_PARAM_MAGIC_NUM: c_uint = 0xFEAA0005;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_polling_flags {
    ENABLE_FCP_RING_POLLING = 0x1,
    DISABLE_FCP_RING_INT    = 0x2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_prof {
    pub cmd_cpu: [u16; 40],
    pub rsp_cpu: [u16; 40],
    pub qh_cpu: [u16; 40],
    pub wqidx: [u16; 40],
}

//
// Provide for FC4 TYPE x28 - NVME.  The
// bit mask for FCP and NVME is 0x8 identically
// because they are 32 bit positions distance.
//
pub const LPFC_FC4_TYPE_BITMASK: c_uint = 0x00000100;
// Provide DMA memory definitions the driver uses per port instance.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_dmabuf {
    pub list: list_head,
    pub /: *mut *mut *mut void virt; / virtual address ptr,
    pub /: *mut *mut dma_addr_t phys; / mapped address,
    pub /: *mut *mut uint32_t buffer_tag; / used for tagged queue ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_nvmet_ctxbuf {
    pub list: list_head,
    pub context: *mut lpfc_async_xchg_ctx,
    pub iocbq: *mut lpfc_iocbq,
    pub sglq: *mut lpfc_sglq,
    pub defer_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_dma_pool {
    pub elements: *mut lpfc_dmabuf,
    pub max_count: u32,
    pub current_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbq_dmabuf {
    pub hbuf: lpfc_dmabuf,
    pub dbuf: lpfc_dmabuf,
    pub total_size: u16,
    pub bytes_recv: u16,
    pub tag: u32,
    pub cq_event: lpfc_cq_event,
    pub time_stamp: c_ulong,
    pub context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rqb_dmabuf {
    pub hbuf: lpfc_dmabuf,
    pub dbuf: lpfc_dmabuf,
    pub total_size: u16,
    pub bytes_recv: u16,
    pub idx: u16,
    pub /: *mut *mut *mut lpfc_queue hrq; / ptr to associated Header RQ,
    pub /: *mut *mut *mut lpfc_queue drq; / ptr to associated Data RQ,
}

// Priority bit.  Set value to exceed low water mark in lpfc_mem.
pub const MEM_PRI: c_uint = 0x100;
//
// Device VPD save area
//

//
// lpfc stat counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_stats {
// Statistics for ELS commands
    pub elsLogiCol: u32,
    pub elsRetryExceeded: u32,
    pub elsXmitRetry: u32,
    pub elsDelayRetry: u32,
    pub elsRcvDrop: u32,
    pub elsRcvFrame: u32,
    pub elsRcvRSCN: u32,
    pub elsRcvRNID: u32,
    pub elsRcvFARP: u32,
    pub elsRcvFARPR: u32,
    pub elsRcvFLOGI: u32,
    pub elsRcvPLOGI: u32,
    pub elsRcvADISC: u32,
    pub elsRcvPDISC: u32,
    pub elsRcvFAN: u32,
    pub elsRcvLOGO: u32,
    pub elsRcvPRLO: u32,
    pub elsRcvPRLI: u32,
    pub elsRcvLIRR: u32,
    pub elsRcvRLS: u32,
    pub elsRcvRPL: u32,
    pub elsRcvRRQ: u32,
    pub elsRcvRTV: u32,
    pub elsRcvECHO: u32,
    pub elsRcvLCB: u32,
    pub elsRcvRDP: u32,
    pub elsRcvRDF: u32,
    pub elsXmitFLOGI: u32,
    pub elsXmitFDISC: u32,
    pub elsXmitPLOGI: u32,
    pub elsXmitPRLI: u32,
    pub elsXmitADISC: u32,
    pub elsXmitLOGO: u32,
    pub elsXmitSCR: u32,
    pub elsXmitRSCN: u32,
    pub elsXmitRNID: u32,
    pub elsXmitFARP: u32,
    pub elsXmitFARPR: u32,
    pub elsXmitACC: u32,
    pub elsXmitLSRJT: u32,
    pub frameRcvBcast: u32,
    pub frameRcvMulti: u32,
    pub strayXmitCmpl: u32,
    pub frameXmitDelay: u32,
    pub xriCmdCmpl: u32,
    pub xriStatErr: u32,
    pub LinkUp: u32,
    pub LinkDown: u32,
    pub LinkMultiEvent: u32,
    pub NoRcvBuf: u32,
    pub fcpCmd: u32,
    pub fcpCmpl: u32,
    pub fcpRspErr: u32,
    pub fcpRemoteStop: u32,
    pub fcpPortRjt: u32,
    pub fcpPortBusy: u32,
    pub fcpError: u32,
    pub fcpLocalErr: u32,
}

// Data structure to keep withheld FLOGI_ACC information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_defer_flogi_acc {
    pub flag: bool,
    pub rx_id: u16,
    pub ox_id: u16,
    pub ndlp: *mut lpfc_nodelist,
}

pub const LPFC_MAX_VMID_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub union lpfc_vmid_io_tag {
    pub /: *mut *mut u32 app_id; / App Id vmid,
    pub /: *mut *mut u8 cs_ctl_vmid; / Priority tag vmid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vmid {
    pub flag: u8,
pub const LPFC_VMID_SLOT_FREE: c_uint = 0x0;
pub const LPFC_VMID_SLOT_USED: c_uint = 0x1;
pub const LPFC_VMID_REQ_REGISTER: c_uint = 0x2;
pub const LPFC_VMID_REGISTERED: c_uint = 0x4;
pub const LPFC_VMID_DE_REGISTER: c_uint = 0x8;
    pub host_vmid: [c_char; LPFC_MAX_VMID_SIZE],
    pub un: lpfc_vmid_io_tag,
    pub hnode: hlist_node,
    pub io_rd_cnt: u64,
    pub io_wr_cnt: u64,
    pub vmid_len: u8,
    pub /: *mut *mut u8 delete_inactive; / Delete if inactive flag 0 = no, 1 = yes,
    pub hash_index: u32,
    pub last_io_time: *mut u64 __percpu,
}

// Macro flag: #define lpfc_vmid_is_type_priority_tag(vport)\
pub const LPFC_VMID_HASH_SIZE: c_int = 256;
pub const LPFC_VMID_HASH_MASK: c_int = 255;
pub const LPFC_VMID_HASH_SHIFT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vmid_context {
    pub vmp: *mut lpfc_vmid,
    pub nlp: *mut lpfc_nodelist,
    pub instantiated: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vmid_priority_range {
    pub low: u8,
    pub high: u8,
    pub qos: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vmid_priority_info {
    pub num_descriptors: u32,
    pub vmid_range: *mut lpfc_vmid_priority_range,
}

pub const QFPA_EVEN_ONLY: c_uint = 0x01;
pub const QFPA_ODD_ONLY: c_uint = 0x02;
pub const QFPA_EVEN_ODD: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum discovery_state {
    LPFC_VPORT_UNKNOWN     =  0,    /* vport state is unknown */
    LPFC_VPORT_FAILED      =  1,    /* vport has failed */
    LPFC_LOCAL_CFG_LINK    =  6,    /* local NPORT Id configured */
    LPFC_FLOGI             =  7,    /* FLOGI sent to Fabric */
    LPFC_FDISC             =  8,    /* FDISC sent for vport */
    LPFC_FABRIC_CFG_LINK   =  9,    /* Fabric assigned NPORT Id
// configured
    LPFC_NS_REG            =  10,   /* Register with NameServer */
    LPFC_NS_QRY            =  11,   /* Query NameServer for NPort ID list */
    LPFC_BUILD_DISC_LIST   =  12,   /* Build ADISC and PLOGI lists for
// device authentication / discovery
    LPFC_DISC_AUTH         =  13,   /* Processing ADISC list */
    LPFC_VPORT_READY       =  32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hba_state {
    LPFC_LINK_UNKNOWN    =   0,   /* HBA state is unknown */
    LPFC_WARM_START      =   1,   /* HBA state after selective reset */
    LPFC_INIT_START      =   2,   /* Initial state after board reset */
    LPFC_INIT_MBX_CMDS   =   3,   /* Initialize HBA with mbox commands */
    LPFC_LINK_DOWN       =   4,   /* HBA initialized, link is down */
    LPFC_LINK_UP         =   5,   /* Link is up  - issue READ_LA */
    LPFC_CLEAR_LA        =   6,   /* authentication cmplt - issue
// CLEAR_LA
    LPFC_HBA_READY       =  32,
    LPFC_HBA_ERROR       =  -1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_hba_flag {
    HBA_ERATT_HANDLED	= 0, /* This flag is set when eratt handled */
    DEFER_ERATT		= 1, /* Deferred error attn in progress */
    HBA_FCOE_MODE		= 2, /* HBA function in FCoE Mode */
    HBA_SP_QUEUE_EVT	= 3, /* Slow-path qevt posted to worker thread*/
    HBA_POST_RECEIVE_BUFFER = 4, /* Rcv buffers need to be posted */
    HBA_PERSISTENT_TOPO	= 5, /* Persistent topology support in hba */
    ELS_XRI_ABORT_EVENT	= 6, /* ELS_XRI abort event was queued */
    ASYNC_EVENT		= 7,
    LINK_DISABLED		= 8, /* Link disabled by user */
    FCF_TS_INPROG           = 9, /* FCF table scan in progress */
    FCF_RR_INPROG           = 10, /* FCF roundrobin flogi in progress */
    HBA_FIP_SUPPORT		= 11, /* FIP support in HBA */
    HBA_DEVLOSS_TMO         = 13, /* HBA in devloss timeout */
    HBA_RRQ_ACTIVE		= 14, /* process the rrq active list */
    HBA_IOQ_FLUSH		= 15, /* I/O queues being flushed */
    HBA_RECOVERABLE_UE	= 17, /* FW supports recoverable UE */
    HBA_FORCED_LINK_SPEED	= 18, /*
// Firmware supports Forced Link
// Speed capability
//
    HBA_FLOGI_ISSUED	= 20, /* FLOGI was issued */
    HBA_DEFER_FLOGI		= 23, /* Defer FLOGI till read_sparm cmpl */
    HBA_SETUP		= 24, /* HBA setup completed */
    HBA_NEEDS_CFG_PORT	= 25, /* SLI3: CONFIG_PORT mbox needed */
    HBA_HBEAT_INP		= 26, /* mbox HBEAT is in progress */
    HBA_HBEAT_TMO		= 27, /* HBEAT initiated after timeout */
    HBA_FLOGI_OUTSTANDING	= 28, /* FLOGI is outstanding */
    HBA_RHBA_CMPL		= 29, /* RHBA FDMI cmd is successful */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_trunk_link_state {
    pub state: hba_state,
    pub fault: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_trunk_link {
    pub phy_lnk_speed: u32,
}

// Format of congestion module parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_cgn_param {
    pub cgn_param_magic: u32,
    pub /: *mut *mut uint8_t cgn_param_version; / version 1,
    pub /: *mut *mut uint8_t cgn_param_mode; / 0=off 1=managed 2=monitor only,
pub const LPFC_CFG_OFF: c_int = 0;
pub const LPFC_CFG_MANAGED: c_int = 1;
pub const LPFC_CFG_MONITOR: c_int = 2;
    pub cgn_rsvd1: u8,
    pub cgn_rsvd2: u8,
    pub cgn_param_level0: u8,
    pub cgn_param_level1: u8,
    pub cgn_param_level2: u8,
    pub byte11: u8,
    pub byte12: u8,
    pub byte13: u8,
    pub byte14: u8,
    pub byte15: u8,
}

// Max number of days of congestion data
pub const LPFC_MAX_CGN_DAYS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_cgn_ts {
    pub month: u8,
    pub day: u8,
    pub year: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

// Format of congestion buffer info
// This structure defines memory thats allocated and registered with
// the HBA firmware. When adding or removing fields from this structure
// the alignment must match the HBA firmware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_cgn_info {
// Header
    pub /: *mut *mut __le16 cgn_info_size; / is sizeof(struct lpfc_cgn_info),
    pub /: *mut *mut uint8_t cgn_info_version; / represents format of structure,
pub const LPFC_CGN_INFO_V1: c_int = 1;
pub const LPFC_CGN_INFO_V2: c_int = 2;
pub const LPFC_CGN_INFO_V3: c_int = 3;
pub const LPFC_CGN_INFO_V4: c_int = 4;
    pub /: *mut *mut uint8_t cgn_info_mode; / 0=off 1=managed 2=monitor only,
    pub cgn_info_detect: u8,
    pub cgn_info_action: u8,
    pub cgn_info_level0: u8,
    pub cgn_info_level1: u8,
    pub cgn_info_level2: u8,
// Start Time
    pub base_time: lpfc_cgn_ts,
// minute / hours / daily indices
    pub cgn_index_minute: u8,
    pub cgn_index_hour: u8,
    pub cgn_index_day: u8,
    pub cgn_warn_freq: __le16,
    pub cgn_alarm_freq: __le16,
    pub cgn_lunq: __le16,
    pub cgn_pad1: [u8; 8],
// Driver Information
    pub cgn_drvr_min: [__le16; 60],
    pub cgn_drvr_hr: [__le32; 24],
    pub cgn_drvr_day: [__le32; LPFC_MAX_CGN_DAYS],
// Congestion Warnings
    pub cgn_warn_min: [__le16; 60],
    pub cgn_warn_hr: [__le32; 24],
    pub cgn_warn_day: [__le32; LPFC_MAX_CGN_DAYS],
// Latency Information
    pub cgn_latency_min: [__le32; 60],
    pub cgn_latency_hr: [__le32; 24],
    pub cgn_latency_day: [__le32; LPFC_MAX_CGN_DAYS],
// Bandwidth Information
    pub cgn_bw_min: [__le16; 60],
    pub cgn_bw_hr: [__le16; 24],
    pub cgn_bw_day: [__le16; LPFC_MAX_CGN_DAYS],
// Congestion Alarms
    pub cgn_alarm_min: [__le16; 60],
    pub cgn_alarm_hr: [__le32; 24],
    pub cgn_alarm_day: [__le32; LPFC_MAX_CGN_DAYS],
    pub /: *mut *mut uint8_t cgn_stat_npm; / Notifications per minute,
// Start Time
    pub /: *mut *mut lpfc_cgn_ts stat_start; / Base time,
    pub cgn_pad2: u8,
    pub cgn_notification: __le32,
    pub cgn_peer_notification: __le32,
    pub link_integ_notification: __le32,
    pub delivery_notification: __le32,
    pub /: *mut *mut lpfc_cgn_ts stat_fpin; / Last congestion notification FPIN,
    pub /: *mut *mut lpfc_cgn_ts stat_peer; / Last peer congestion FPIN,
    pub /: *mut *mut lpfc_cgn_ts stat_lnk; / Last link integrity FPIN,
    pub /: *mut *mut lpfc_cgn_ts stat_delivery; / Last delivery notification FPIN,
    pub cgn_info_crc: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_cgn_stat {
    pub total_bytes: core::sync::atomic::AtomicI64,
    pub rcv_bytes: core::sync::atomic::AtomicI64,
    pub rx_latency: core::sync::atomic::AtomicI64,
pub const LPFC_CGN_NOT_SENT: c_uint = 0xFFFFFFFFFFFFFFFFLL;
    pub rx_io_cnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_cgn_acqe_stat {
    pub alarm: core::sync::atomic::AtomicI64,
    pub warn: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_fc_flag {
// Several of these flags are HBA centric and should be moved to
// phba->link_flag (e.g. FC_PTP, FC_PUBLIC_LOOP)
//
    FC_PT2PT,			/* pt2pt with no fabric */
    FC_PT2PT_PLOGI,			/* pt2pt initiate PLOGI */
    FC_DISC_TMO,			/* Discovery timer running */
    FC_PUBLIC_LOOP,			/* Public loop */
    FC_LBIT,			/* LOGIN bit in loopinit set */
    FC_RSCN_MODE,			/* RSCN cmd rcv'ed */
    FC_NLP_MORE,			/* More node to process in node tbl */
    FC_OFFLINE_MODE,		/* Interface is offline for diag */
    FC_FABRIC,			/* We are fabric attached */
    FC_VPORT_LOGO_RCVD,		/* LOGO received on vport */
    FC_RSCN_DISCOVERY,		/* Auth all devices after RSCN */
    FC_LOGO_RCVD_DID_CHNG,		/* FDISC on phys port detect DID chng */
    FC_PT2PT_NO_NVME,		/* Don't send NVME PRLI */
    FC_SCSI_SCAN_TMO,		/* scsi scan timer running */
    FC_ABORT_DISCOVERY,		/* we want to abort discovery */
    FC_NDISC_ACTIVE,		/* NPort discovery active */
    FC_BYPASSED_MODE,		/* NPort is in bypassed mode */
    FC_VPORT_NEEDS_REG_VPI,		/* Needs to have its vpi registered */
    FC_RSCN_DEFERRED,		/* A deferred RSCN being processed */
    FC_VPORT_NEEDS_INIT_VPI,	/* Need to INIT_VPI before FDISC */
    FC_VPORT_CVL_RCVD,		/* VLink failed due to CVL */
    FC_VFI_REGISTERED,		/* VFI is registered */
    FC_FDISC_COMPLETED,		/* FDISC completed */
    FC_DISC_DELAYED,		/* Delay NPort discovery */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_load_flag {
    FC_LOADING,			/* HBA in process of loading drvr */
    FC_UNLOADING,			/* HBA in process of unloading drvr */
    FC_ALLOW_FDMI,			/* port is ready for FDMI requests */
    FC_ALLOW_VMID,			/* Allow VMID I/Os */
    FC_DEREGISTER_ALL_APP_ID	/* Deregister all VMIDs */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_vport {
    pub phba: *mut lpfc_hba,
    pub listentry: list_head,
    pub port_type: u8,
pub const LPFC_PHYSICAL_PORT: c_int = 1;
pub const LPFC_NPIV_PORT: c_int = 2;
pub const LPFC_FABRIC_PORT: c_int = 3;
    pub port_state: discovery_state,
    pub vpi: u16,
    pub vfi: u16,
    pub vpi_state: u8,
pub const LPFC_VPI_REGISTERED: c_uint = 0x1;
    pub /: *mut *mut unsigned long fc_flag; / FC flags,
    pub ct_flags: u32,
pub const FC_CT_RFF_ID: c_uint = 0x1	 /* RFF_ID accepted by switch */;
pub const FC_CT_RNN_ID: c_uint = 0x2	 /* RNN_ID accepted by switch */;
pub const FC_CT_RSNN_NN: c_uint = 0x4	 /* RSNN_NN accepted by switch */;
pub const FC_CT_RSPN_ID: c_uint = 0x8	 /* RSPN_ID accepted by switch */;
pub const FC_CT_RFT_ID: c_uint = 0x10	 /* RFT_ID accepted by switch */;
pub const FC_CT_RPRT_DEFER: c_uint = 0x20	 /* Defer issuing FDMI RPRT */;
pub const FC_CT_RSPNI_PNI: c_uint = 0x40	 /* RSPNI_PNI accepted by switch */;
    pub fc_nodes: list_head,
    pub /: *mut *mut spinlock_t fc_nodes_list_lock; / spinlock for fc_nodes list,
// Keep counters for the number of entries in each list.
    pub fc_plogi_cnt: core::sync::atomic::AtomicI32,
    pub fc_adisc_cnt: core::sync::atomic::AtomicI32,
    pub fc_reglogin_cnt: core::sync::atomic::AtomicI32,
    pub fc_prli_cnt: core::sync::atomic::AtomicI32,
    pub fc_unmap_cnt: core::sync::atomic::AtomicI32,
    pub fc_map_cnt: core::sync::atomic::AtomicI32,
    pub fc_npr_cnt: core::sync::atomic::AtomicI32,
    pub fc_unused_cnt: core::sync::atomic::AtomicI32,
    pub /: *mut *mut serv_parm fc_sparam; / buffer for our service parameters,
    pub /: *mut *mut uint32_t fc_myDID; / fibre channel S_ID,
    pub /: *mut *mut uint32_t fc_prevDID; / previous fibre channel S_ID,
    pub fabric_portname: lpfc_name,
    pub fabric_nodename: lpfc_name,
    pub /: *mut *mut int32_t stopped; / HBA has not been restarted since last ERATT,
    pub /: *mut *mut uint8_t fc_linkspeed; / Link speed after last READ_LA,
    pub /: *mut *mut uint32_t num_disc_nodes; / in addition to hba_state,
    pub /: *mut *mut uint32_t gidft_inp; / cnt of outstanding GID_FTs,
    pub /: *mut *mut uint32_t fc_rscn_id_cnt; / count of RSCNs payloads in list,
    pub /: *mut *mut uint32_t fc_rscn_flush; / flag use of fc_rscn_id_list,
    pub fc_rscn_id_list: [*mut lpfc_dmabuf; FC_MAX_HOLD_RSCN],
    pub /: *mut *mut lpfc_name fc_nodename; / fc nodename,
    pub /: *mut *mut lpfc_name fc_portname; / fc portname,
    pub /: *mut *mut timer_list fc_disctmo; / Discovery rescue timer,
    pub /: *mut *mut uint8_t fc_ns_retry; / retries for fabric nameserver,
    pub /: *mut *mut uint32_t fc_prli_sent; / cntr for outstanding PRLIs,
    pub work_port_lock: spinlock_t,
    pub /: *mut *mut uint32_t work_port_events; / Timeout to be handled,
pub const WORKER_DISC_TMO: c_uint = 0x1	/* vport: Discovery timeout */;
pub const WORKER_ELS_TMO: c_uint = 0x2	/* vport: ELS timeout */;
pub const WORKER_DELAYED_DISC_TMO: c_uint = 0x8	/* vport: delayed discovery */;
pub const WORKER_MBOX_TMO: c_uint = 0x100	/* hba: MBOX timeout */;
pub const WORKER_HB_TMO: c_uint = 0x200	/* hba: Heart beat timeout */;
pub const WORKER_FABRIC_BLOCK_TMO: c_uint = 0x400	/* hba: fabric block timeout */;
pub const WORKER_RAMP_DOWN_QUEUE: c_uint = 0x800	/* hba: Decrease Q depth */;
pub const WORKER_RAMP_UP_QUEUE: c_uint = 0x1000	/* hba: Increase Q depth */;
pub const WORKER_SERVICE_TXQ: c_uint = 0x2000	/* hba: IOCBs on the txq */;
pub const WORKER_CHECK_INACTIVE_VMID: c_uint = 0x4000	/* hba: check inactive vmids */;
pub const WORKER_CHECK_VMID_ISSUE_QFPA: c_uint = 0x8000	/* vport: Check if qfpa needs;
// to be issued
    pub els_tmofunc: timer_list,
    pub delayed_disc_tmo: timer_list,
    pub load_flag: c_ulong,
// Vport Config Parameters
    pub cfg_scan_down: u32,
    pub cfg_lun_queue_depth: u32,
    pub cfg_nodev_tmo: u32,
    pub cfg_devloss_tmo: u32,
    pub cfg_restrict_login: u32,
    pub cfg_peer_port_login: u32,
    pub cfg_fcp_class: u32,
    pub cfg_use_adisc: u32,
    pub cfg_discovery_threads: u32,
    pub cfg_log_verbose: u32,
    pub cfg_enable_fc4_type: u32,
    pub cfg_max_luns: u32,
    pub cfg_enable_da_id: u32,
    pub cfg_max_scsicmpl_time: u32,
    pub cfg_tgt_queue_depth: u32,
    pub cfg_first_burst_size: u32,
    pub dev_loss_tmo_changed: u32,
// VMID parameters
    pub lpfc_vmid_host_uuid: [u8; 16],
    pub /: *mut *mut u32 max_vmid; / maximum VMIDs allowed per port,
    pub /: *mut *mut u32 cur_vmid_cnt; / Current VMID count,
pub const LPFC_MIN_VMID: c_int = 4;
pub const LPFC_MAX_VMID: c_int = 255;
    pub /: *mut *mut u32 vmid_inactivity_timeout; / Time after which the VMID,
// deregisters from switch
    pub vmid_priority_tagging: u32,

    pub vmid_priority_range: *mut c_ulong,
pub const LPFC_VMID_MAX_PRIORITY_RANGE: c_int = 256;
pub const LPFC_VMID_PRIORITY_BITMAP_SIZE: c_int = 32;
    pub vmid_flag: u8,
pub const LPFC_VMID_IN_USE: c_uint = 0x1;
pub const LPFC_VMID_ISSUE_QFPA: c_uint = 0x2;
pub const LPFC_VMID_QFPA_CMPL: c_uint = 0x4;
pub const LPFC_VMID_QOS_ENABLED: c_uint = 0x8;
pub const LPFC_VMID_TIMER_ENBLD: c_uint = 0x10;
pub const LPFC_VMID_TYPE_PRIO: c_uint = 0x20;
    pub qfpa_res: *mut fc_qfpa_res,
    pub fc_vport: *mut fc_vport,
    pub vmid: *mut lpfc_vmid,
    pub 8): DECLARE_HASHTABLE(hash_table,,
    pub vmid_lock: rwlock_t,
    pub vmid_priority: lpfc_vmid_priority_info,

    pub vport_debugfs_root: *mut dentry,
    pub disc_trc: *mut lpfc_debugfs_trc,
    pub disc_trc_cnt: core::sync::atomic::AtomicI32,

    pub rcv_buffer_list: list_head,
    pub rcv_buffer_time_stamp: c_ulong,
    pub vport_flag: u32,
pub const STATIC_VPORT: c_uint = 0x1;
pub const FAWWPN_PARAM_CHG: c_uint = 0x2;
    pub fdmi_num_disc: u16,
    pub fdmi_hba_mask: u32,
    pub fdmi_port_mask: u32,
// There is a single nvme instance per vport.
    pub localport: *mut nvme_fc_local_port,
    pub /: *mut *mut uint8_t nvmei_support; / driver supports NVME Initiator,
    pub /: *mut *mut uint32_t rcv_flogi_cnt; / How many unsol FLOGIs ACK'd.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbq_s {
    pub /: *mut *mut uint16_t entry_count; / Current number of HBQ slots,
    pub /: *mut *mut uint16_t buffer_count; / Current number of buffers posted,
    pub /: *mut *mut uint32_t next_hbqPutIdx; / Index to next HBQ slot to use,
    pub /: *mut *mut uint32_t hbqPutIdx; / HBQ slot to use,
    pub /: *mut *mut uint32_t local_hbqGetIdx; / Local copy of Get index from Port,
    pub /: *mut *mut *mut void hbq_virt; / Virtual ptr to this hbq,
    pub /: *mut *mut list_head hbq_buffer_list; / buffers assigned to this HBQ,
// Callback for HBQ buffer allocation
    pub ): *mut *mut *mut hbq_dmabuf (hbq_alloc_buffer) (lpfc_hba,
// Callback for HBQ buffer free
    pub ): *mut hbq_dmabuf,
}

// this matches the position in the lpfc_hbq_defs array
pub const LPFC_ELS_HBQ: c_int = 0;
pub const LPFC_MAX_HBQS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hba_temp_state {
    HBA_NORMAL_TEMP,
    HBA_OVER_TEMP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intr_type_t {
    NONE = 0,
    INTx,
    MSI,
    MSIX,
}

pub const LPFC_CT_CTX_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unsol_rcv_ct_ctx {
    pub ctxt_id: u32,
    pub SID: u32,
    pub valid: u32,
pub const UNSOL_INVALID: c_int = 0;
pub const UNSOL_VALID: c_int = 1;
    pub oxid: u16,
    pub rxid: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nemb_type {
    nemb_mse = 1,
    nemb_hbd
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox_type {
    mbox_rd = 1,
    mbox_wr
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_type {
    dma_mbox = 1,
    dma_ebuf
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sta_type {
    sta_pre_addr = 1,
    sta_pos_addr
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_mbox_ext_buf_ctx {
    pub state: u32,
pub const LPFC_BSG_MBOX_IDLE: c_int = 0;
pub const LPFC_BSG_MBOX_HOST: c_int = 1;
pub const LPFC_BSG_MBOX_PORT: c_int = 2;
pub const LPFC_BSG_MBOX_DONE: c_int = 3;
pub const LPFC_BSG_MBOX_ABTS: c_int = 4;
    pub nembType: nemb_type,
    pub mboxType: mbox_type,
    pub numBuf: u32,
    pub mbxTag: u32,
    pub seqNum: u32,
    pub mbx_dmabuf: *mut lpfc_dmabuf,
    pub ext_dmabuf_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_epd_pool {
// Expedite pool
    pub list: list_head,
    pub count: u32,
    pub /: *mut *mut spinlock_t lock; / lock for expedite pool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_state {
    INACTIVE,
    REG_INPROGRESS,
    ACTIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_ras_fwlog {
    pub fwlog_buff: *mut u8,
    pub /: *mut *mut uint32_t fw_buffcount; / Buffer size posted to FW,

    pub /: *mut *mut uint32_t fw_loglevel; / Log level set,
    pub lwpd: lpfc_dmabuf,
    pub fwlog_buff_list: list_head,
// RAS support status on adapter
    pub /: *mut *mut bool ras_hwsupport; / RAS Support available on HW or not,
    pub /: *mut *mut bool ras_enabled; / Ras Enabled for the function,
pub const LPFC_RAS_DISABLE_LOGGING: c_uint = 0x00;
pub const LPFC_RAS_ENABLE_LOGGING: c_uint = 0x01;
    pub /: *mut *mut ras_state state; / RAS logging running state,
}

pub const DBG_LOG_STR_SZ: c_int = 256;
pub const DBG_LOG_SZ: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbg_log_ent {
    pub log: [c_char; DBG_LOG_STR_SZ],
    pub t_ns: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_irq_chann_mode {
// Assign IRQs to all possible cpus that have hardware queues
    NORMAL_MODE,

// Assign IRQs only to cpus on the same numa node as HBA
    NUMA_MODE,

// Assign IRQs only on non-hyperthreaded CPUs. This is the
// same as normal_mode, but assign IRQS only on physical CPUs.
//
    NHT_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpfc_hba_bit_flags {
    FABRIC_COMANDS_BLOCKED,
    HBA_PCI_ERR,
    MBX_TMO_ERR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_hba {
// SCSI interface function jump table entries
    pub cmnd): *mut scsi_cmnd,
    pub ): *mut *mut (struct lpfc_hba , struct lpfc_io_buf,
    pub ): *mut *mut (struct lpfc_hba , struct lpfc_io_buf,
    pub ): *mut *mut (struct lpfc_hba , struct lpfc_io_buf,
    pub ): *mut (struct lpfc_hba,
    pub ): *mut lpfc_nodelist,
    pub tmo): u8,
    pub task_mgmt_cmd): u64 lun, u8,
// IOCB interface function jump table entries
    pub uint32_t): *mut *mut lpfc_iocbq ,,
    pub flag): *mut *mut lpfc_iocbq piocb, uint32_t,
    pub ): *mut lpfc_iocbq,
    pub phba): *mut *mut int (lpfc_hba_down_post)(struct lpfc_hba,
// MBOX interface function jump table entries
    pub uint32_t): *mut *mut *mut (struct lpfc_hba , LPFC_MBOXQ_t ,,
// Slow-path IOCB process function jump table entries
    pub mask): u32,
// INIT device interface function jump table entries
    pub ): *mut *mut (struct lpfc_hba , uint32_t, struct hbq_dmabuf,
    pub ): *mut (struct lpfc_hba,
    pub uint32_t): *mut *mut (struct lpfc_hba ,,
    pub ): *mut (struct lpfc_hba,
    pub ): *mut (struct lpfc_hba,
    pub uint32_t): *mut *mut (struct lpfc_hba ,,
    pub uint32_t): *mut *mut (struct lpfc_hba ,,
    pub ): *mut (struct lpfc_hba,
    pub ): *mut *mut (struct lpfc_hba , struct lpfc_io_buf,
// Prep SLI WQE/IOCB jump table entries
    pub expect_rsp): u8 tmo, u8,
    pub tmo): u32 num_entry, u8,
    pub cr_cx_cmd): u8 last_seq, u8,
    pub wqec): bool,
// expedite pool
    pub epd_pool: lpfc_epd_pool,
// SLI4 specific HBA data structure
    pub sli4_hba: lpfc_sli4_hba,
    pub wq: *mut workqueue_struct,
    pub eq_delay_work: delayed_work,
pub const LPFC_IDLE_STAT_DELAY: c_int = 1000;
    pub idle_stat_delay_work: delayed_work,
    pub sli: lpfc_sli,
    pub /: *mut *mut uint8_t pci_dev_grp; / lpfc PCI dev group: 0x0, 0x1, 0x2,...,
    pub /: *mut *mut uint32_t sli_rev; / SLI2, SLI3, or SLI4,
    pub /: *mut *mut uint32_t sli3_options; / Mask of enabled SLI3 options,
pub const LPFC_SLI3_HBQ_ENABLED: c_uint = 0x01;
pub const LPFC_SLI3_NPIV_ENABLED: c_uint = 0x02;
pub const LPFC_SLI3_VPORT_TEARDOWN: c_uint = 0x04;
pub const LPFC_SLI3_CRP_ENABLED: c_uint = 0x08;
pub const LPFC_SLI3_BG_ENABLED: c_uint = 0x20;
pub const LPFC_SLI3_DSS_ENABLED: c_uint = 0x40;
pub const LPFC_SLI4_PHWQ_ENABLED: c_uint = 0x100;
    pub iocb_cmd_size: u32,
    pub iocb_rsp_size: u32,
    pub trunk_link: lpfc_trunk_link,
    pub link_state: hba_state,
    pub /: *mut *mut uint32_t link_flag; / link state flags,
pub const LS_LOOPBACK_MODE: c_uint = 0x1	/* NPort is in Loopback mode */;
// This flag is set while issuing
// INIT_LINK mailbox command
pub const LS_NPIV_FAB_SUPPORTED: c_uint = 0x2	/* Fabric supports NPIV */;
pub const LS_IGNORE_ERATT: c_uint = 0x4	/* intr handler should ignore ERATT */;
pub const LS_MDS_LINK_DOWN: c_uint = 0x8	/* MDS Diagnostics Link Down */;
pub const LS_MDS_LOOPBACK: c_uint = 0x10	/* MDS Diagnostics Link Up (Loopback) */;
pub const LS_CT_VEN_RPA: c_uint = 0x20	/* Vendor RPA sent to switch */;
pub const LS_EXTERNAL_LOOPBACK: c_uint = 0x40	/* External loopback plug inserted */;
    pub /: *mut *mut unsigned long hba_flag; / hba generic flags,
    pub /: *mut *mut *mut completion fw_dump_cmpl; / cmpl event tracker for fw_dump,
    pub active*/: *mut *mut uint32_t fcp_ring_in_use; / When polling test if intr-hndlr,
    pub slim2p: lpfc_dmabuf,
    pub mbox: *mut MAILBOX_t,
    pub mbox_ext: *mut u32,
    pub mbox_ext_buf_ctx: lpfc_mbox_ext_buf_ctx,
    pub ha_copy: u32,
    pub pcb: *mut _PCB,
    pub IOCBs: *mut _IOCB,
    pub hbqslimp: lpfc_dmabuf,
    pub /: *mut *mut uint8_t fc_linkspeed; / Link speed after last READ_LA,
    pub /: *mut *mut uint32_t fc_eventTag; / event tag for link attention,
    pub link_events: u32,
// These fields used to be binfo
    pub /: *mut *mut uint32_t fc_pref_DID; / preferred D_ID,
    pub /: *mut *mut uint8_t fc_pref_ALPA; / preferred AL_PA,
    pub /: *mut *mut uint32_t fc_edtovResol; / E_D_TOV timer resolution,
    pub /: *mut *mut uint32_t fc_edtov; / E_D_TOV timer value,
    pub /: *mut *mut uint32_t fc_arbtov; / ARB_TOV timer value,
    pub /: *mut *mut uint32_t fc_ratov; / R_A_TOV timer value,
    pub /: *mut *mut uint32_t fc_rttov; / R_T_TOV timer value,
    pub /: *mut *mut uint32_t fc_altov; / AL_TOV timer value,
    pub /: *mut *mut uint32_t fc_crtov; / C_R_TOV timer value,
    pub /: *mut *mut serv_parm fc_fabparam; / fabric service parameters buffer,
    pub /: *mut *mut uint8_t alpa_map[128]; / AL_PA map from READ_LA,
    pub lmt: u32,
    pub /: *mut *mut uint32_t fc_topology; / link topology, from LINK INIT,
    pub /: *mut *mut uint32_t fc_topology_changed; / link topology, from LINK INIT,
    pub fc_stat: lpfc_stats,
    pub /: *mut *mut uint32_t nport_event_cnt; / timestamp for nlplist entry,
    pub /: *mut *mut unsigned long pni; / 64-bit Platform Name Identifier,
    pub wwnn: [u8; 8],
    pub wwpn: [u8; 8],
    pub RandomData: [u32; 7],
    pub fcp_embed_io: u8,
    pub /: *mut *mut uint8_t nvmet_support; / driver supports NVMET,
pub const LPFC_NVMET_MAX_PORTS: c_int = 32;
    pub mds_diags_support: u8,
    pub bbcredit_support: u8,
    pub enab_exp_wqcq_pages: u8,
    pub /: *mut *mut u8 nsler; / Firmware supports FC-NVMe-2 SLER,
// HBA Config Parameters
    pub cfg_ack0: u32,
    pub cfg_xri_rebalancing: u32,
    pub cfg_xpsgl: u32,
    pub cfg_enable_npiv: u32,
    pub cfg_enable_rrq: u32,
    pub cfg_topology: u32,
    pub cfg_link_speed: u32,

    pub cfg_fcf_failover_policy: u32,
    pub cfg_fcp_io_sched: u32,
    pub cfg_ns_query: u32,
    pub cfg_fcp2_no_tgt_reset: u32,
    pub cfg_cr_delay: u32,
    pub cfg_cr_count: u32,
    pub cfg_multi_ring_support: u32,
    pub cfg_multi_ring_rctl: u32,
    pub cfg_multi_ring_type: u32,
    pub cfg_poll: u32,
    pub cfg_poll_tmo: u32,
    pub cfg_task_mgmt_tmo: u32,
    pub cfg_use_msi: u32,
    pub cfg_auto_imax: u32,
    pub cfg_fcp_imax: u32,
    pub cfg_force_rscn: u32,
    pub cfg_cq_poll_threshold: u32,
    pub cfg_cq_max_proc_limit: u32,
    pub cfg_fcp_cpu_map: u32,
    pub cfg_fcp_mq_threshold: u32,
    pub cfg_hdw_queue: u32,
    pub cfg_irq_chann: u32,
    pub cfg_suppress_rsp: u32,
    pub cfg_nvme_oas: u32,
    pub cfg_nvme_embed_cmd: u32,
    pub cfg_nvmet_mrq_post: u32,
    pub cfg_nvmet_mrq: u32,
    pub cfg_enable_nvmet: u32,
    pub cfg_nvme_enable_fb: u32,
    pub cfg_nvmet_fb_size: u32,
    pub cfg_total_seg_cnt: u32,
    pub cfg_sg_seg_cnt: u32,
    pub cfg_nvme_seg_cnt: u32,
    pub cfg_scsi_seg_cnt: u32,
    pub cfg_sg_dma_buf_size: u32,
    pub cfg_hba_queue_depth: u32,
    pub cfg_enable_hba_reset: u32,
    pub cfg_enable_hba_heartbeat: u32,
    pub cfg_fof: u32,
    pub cfg_EnableXLane: u32,
    pub cfg_oas_tgt_wwpn: [u8; 8],
    pub cfg_oas_vpt_wwpn: [u8; 8],
    pub cfg_oas_lun_state: u32,
pub const OAS_LUN_ENABLE: c_int = 1;
pub const OAS_LUN_DISABLE: c_int = 0;
    pub cfg_oas_lun_status: u32,
pub const OAS_LUN_STATUS_EXISTS: c_uint = 0x01;
    pub cfg_oas_flags: u32,
pub const OAS_FIND_ANY_VPORT: c_uint = 0x01;
pub const OAS_FIND_ANY_TARGET: c_uint = 0x02;
pub const OAS_LUN_VALID: c_uint = 0x04;
    pub cfg_oas_priority: u32,
    pub cfg_XLanePriority: u32,
    pub cfg_enable_bg: u32,
    pub cfg_prot_mask: u32,
    pub cfg_prot_guard: u32,
    pub cfg_hostmem_hgp: u32,
    pub cfg_log_verbose: u32,
    pub cfg_enable_fc4_type: u32,
pub const LPFC_ENABLE_FCP: c_int = 1;
pub const LPFC_ENABLE_NVME: c_int = 2;
pub const LPFC_ENABLE_BOTH: c_int = 3;

    pub cfg_sriov_nr_virtfn: u32,
    pub cfg_request_firmware_upgrade: u32,
    pub cfg_suppress_link_up: u32,
    pub cfg_rrq_xri_bitmap_sz: u32,
    pub cfg_fcp_wait_abts_rsp: u32,
    pub cfg_delay_discovery: u32,
    pub cfg_sli_mode: u32,

    pub cfg_fdmi_on: u32,

    pub cfg_enable_SmartSAN: u32,
    pub cfg_enable_mds_diags: u32,
    pub cfg_ras_fwlog_level: u32,
    pub cfg_ras_fwlog_buffsize: u32,
    pub cfg_ras_fwlog_func: u32,
    pub /: *mut *mut uint32_t cfg_enable_bbcr; / Enable BB Credit Recovery,
    pub /: *mut *mut uint32_t cfg_enable_dpp; / Enable Direct Packet Push,
    pub cfg_enable_mi: u32,
    pub targetport: *mut nvmet_fc_target_port,
    pub /: *mut *mut lpfc_vpd_t vpd; / vital product data,
    pub /: *mut *mut u32 cfg_max_vmid; / maximum VMIDs allowed per port,
    pub cfg_vmid_app_header: u32,
pub const LPFC_VMID_APP_HEADER_DISABLE: c_int = 0;
pub const LPFC_VMID_APP_HEADER_ENABLE: c_int = 1;
    pub cfg_vmid_priority_tagging: u32,
    pub /: *mut *mut u32 cfg_vmid_inactivity_timeout; / Time after which the VMID,
// deregisters from switch
    pub pcidev: *mut pci_dev,
    pub work_list: list_head,
    pub /: *mut *mut uint32_t work_ha; / Host Attention Bits for WT,
    pub /: *mut *mut uint32_t work_ha_mask; / HA Bits owned by WT,
    pub /: *mut *mut uint32_t work_hs; / HS stored in case of ERRAT,
    pub /: *mut *mut uint32_t work_status[2]; / Extra status from SLIM,
    pub work_waitq: wait_queue_head_t,
    pub worker_thread: *mut task_struct,
    pub data_flags: c_ulong,
    pub border_sge_num: u32,
    pub /: *mut *mut uint32_t hbq_in_use; / HBQs in use flag,
    pub /: *mut *mut uint32_t hbq_count; / Count of configured HBQs,
    pub /: *mut *mut hbq_s hbqs[LPFC_MAX_HBQS]; / local copy of hbq indicies,
    pub /: *mut *mut phys_addr_t pci_bar0_map; / Physical address for PCI BAR0,
    pub /: *mut *mut phys_addr_t pci_bar1_map; / Physical address for PCI BAR1,
    pub /: *mut *mut phys_addr_t pci_bar2_map; / Physical address for PCI BAR2,
    pub for: *mut *mut *mut void __iomem slim_memmap_p; / Kernel memory mapped address,
    pub for: *mut *mut *mut void __iomem ctrl_regs_memmap_p;/ Kernel memory mapped address,
    pub for: *mut *mut *mut void __iomem pci_bar0_memmap_p; / Kernel memory mapped address,
    pub for: *mut *mut *mut void __iomem pci_bar2_memmap_p; / Kernel memory mapped address,
    pub for: *mut *mut *mut void __iomem pci_bar4_memmap_p; / Kernel memory mapped address,
pub const PCI_64BIT_BAR0: c_int = 0;
pub const PCI_64BIT_BAR2: c_int = 2;
pub const PCI_64BIT_BAR4: c_int = 4;
    pub /: *mut *mut *mut void __iomem MBslimaddr; / virtual address for mbox cmds,
    pub /: *mut *mut *mut void __iomem HAregaddr; / virtual address for host attn reg,
    pub /: *mut *mut *mut void __iomem CAregaddr; / virtual address for chip attn reg,
    pub status: *mut *mut *mut void __iomem HSregaddr; / virtual address for host,
    pub /: *mut *mut *mut void __iomem HCregaddr; / virtual address for host ctl reg,
    pub /: *mut *mut *mut lpfc_hgp __iomem host_gp; / Host side get/put pointers,
    pub port_gp: *mut lpfc_pgp,
    pub /: *mut *mut *mut uint32_t __iomem hbq_put; / Address in SLIM to HBQ put ptrs,
    pub /: *mut *mut *mut uint32_t hbq_get; / Host mem address of HBQ get ptrs,
    pub /: *mut *mut int brd_no; / FC board number,
    pub /: *mut *mut char SerialNumber[32]; / adapter Serial Number,
    pub /: *mut *mut char OptionROMVersion[32]; / adapter BIOS / Fcode version,
    pub /: *mut *mut char BIOSVersion[16]; / Boot BIOS version,
    pub /: *mut *mut char ModelDesc[256]; / Model Description,
    pub /: *mut *mut char ModelName[80]; / Model Name,
    pub /: *mut *mut char ProgramType[256]; / Program Type,
    pub /: *mut *mut char Port[20]; / Port No,
    pub /: *mut *mut uint8_t vpd_flag; / VPD data flag,
pub const VPD_MODEL_DESC: c_uint = 0x1         /* valid vpd model description */;
pub const VPD_MODEL_NAME: c_uint = 0x2         /* valid vpd model name */;
pub const VPD_PROGRAM_TYPE: c_uint = 0x4         /* valid vpd program type */;
pub const VPD_PORT: c_uint = 0x8         /* valid vpd port data */;
pub const VPD_MASK: c_uint = 0xf         /* mask for any vpd data */;
    pub fcp_poll_timer: timer_list,
    pub eratt_poll: timer_list,
    pub eratt_poll_interval: u32,
    pub bg_guard_err_cnt: u64,
    pub bg_apptag_err_cnt: u64,
    pub bg_reftag_err_cnt: u64,
// fastpath list.
    pub /: *mut *mut spinlock_t scsi_buf_list_get_lock; / SCSI buf alloc list lock,
    pub /: *mut *mut spinlock_t scsi_buf_list_put_lock; / SCSI buf free list lock,
    pub lpfc_scsi_buf_list_get: list_head,
    pub lpfc_scsi_buf_list_put: list_head,
    pub total_scsi_bufs: u32,
    pub lpfc_iocb_list: list_head,
    pub total_iocbq_bufs: u32,
    pub /: *mut *mut spinlock_t rrq_list_lock; / lock for active_rrq_list,
    pub active_rrq_list: list_head,
    pub hbalock: spinlock_t,
    pub /: *mut *mut work_unblock_request_work; / SCSI layer unblock IOs,
// dma_mem_pools
    pub lpfc_sg_dma_buf_pool: *mut dma_pool,
    pub lpfc_mbuf_pool: *mut dma_pool,
    pub /: *mut *mut *mut dma_pool lpfc_hrb_pool; / header receive buffer pool,
    pub /: *mut *mut *mut dma_pool lpfc_drb_pool; / data receive buffer pool,
    pub /: *mut *mut *mut dma_pool lpfc_nvmet_drb_pool; / data receive buffer pool,
    pub /: *mut *mut *mut dma_pool lpfc_hbq_pool; / SLI3 hbq buffer pool,
    pub lpfc_cmd_rsp_buf_pool: *mut dma_pool,
    pub lpfc_mbuf_safety_pool: lpfc_dma_pool,
    pub mbox_mem_pool: *mut mempool_t,
    pub nlp_mem_pool: *mut mempool_t,
    pub rrq_pool: *mut mempool_t,
    pub active_rrq_pool: *mut mempool_t,
    pub link_stats: fc_host_statistics,
    pub irq_chann_mode: lpfc_irq_chann_mode,
    pub intr_type: intr_type_t,
    pub intr_mode: u32,
pub const LPFC_INTR_ERROR: c_uint = 0xFFFFFFFF;
    pub port_list: list_head,
    pub /: *mut *mut spinlock_t port_list_lock; / lock for port_list mutations,
    pub /: *mut *mut *mut lpfc_vport pport; / physical lpfc_vport pointer,
    pub /: *mut *mut uint16_t max_vpi; / Maximum virtual nports,
pub const LPFC_MAX_VPI: c_uint = 0xFF		/* Max number VPI supported 0 - 0xff */;
pub const LPFC_MAX_VPORTS: c_uint = 0x100		/* Max vports per port, with pport */;
    pub /*: *mut uint16_t max_vports;,
// For IOV HBAs max_vpi can change
// after a reset. max_vports is max
// number of vports present. This can
// be greater than max_vpi.
//
    pub vpi_base: u16,
    pub vfi_base: u16,
    pub /: *mut *mut *mut unsigned long vpi_bmask; / vpi allocation table,
    pub vpi_ids: *mut u16,
    pub vpi_count: u16,
    pub lpfc_vpi_blk_list: list_head,
// Data structure used by fabric iocb scheduler
    pub fabric_iocb_list: list_head,
    pub fabric_iocb_count: core::sync::atomic::AtomicI32,
    pub fabric_block_timer: timer_list,
    pub bit_flags: c_ulong,
    pub num_rsrc_err: core::sync::atomic::AtomicI32,
    pub last_rsrc_error_time: c_ulong,
    pub last_ramp_down_time: c_ulong,

    pub hba_debugfs_root: *mut dentry,
    pub debugfs_vport_count: c_uint,
    pub nvmeio_trc: *mut lpfc_debugfs_nvmeio_trc,
    pub nvmeio_trc_cnt: core::sync::atomic::AtomicI32,
    pub nvmeio_trc_size: u32,
    pub nvmeio_trc_output_idx: u32,
// T10 DIF error injection
    pub lpfc_injerr_wgrd_cnt: u32,
    pub lpfc_injerr_wapp_cnt: u32,
    pub lpfc_injerr_wref_cnt: u32,
    pub lpfc_injerr_rgrd_cnt: u32,
    pub lpfc_injerr_rapp_cnt: u32,
    pub lpfc_injerr_rref_cnt: u32,
    pub lpfc_injerr_nportid: u32,
    pub lpfc_injerr_wwpn: lpfc_name,
    pub lpfc_injerr_lba: sector_t,

    pub slow_ring_trc: *mut lpfc_debugfs_trc,
    pub slow_ring_trc_cnt: core::sync::atomic::AtomicI32,
// iDiag debugfs sub-directory
    pub idiag_root: *mut dentry,
    pub lpfc_idiag_last_eq: u8,

    pub nvmeio_trc_on: u16,
// Used for deferred freeing of ELS data buffers
    pub elsbuf: list_head,
    pub elsbuf_cnt: c_int,
    pub elsbuf_prev_cnt: c_int,
    pub temp_sensor_support: u8,
// Fields used for heart beat.
    pub last_completion_time: c_ulong,
    pub skipped_hb: c_ulong,
    pub hb_tmofunc: timer_list,
    pub rrq_tmr: timer_list,
    pub over_temp_state: hba_temp_state,
//
// Following bit will be set for all buffer tags which are not
// associated with any HBQ.
//

    pub buffer_tag_count: u32,
// Maximum number of events that can be outstanding at any time
pub const LPFC_MAX_EVT_COUNT: c_int = 512;
    pub fast_event_count: core::sync::atomic::AtomicI32,
    pub fcoe_eventtag: u32,
    pub fcoe_eventtag_at_fcf_scan: u32,
    pub fcoe_cvl_eventtag: u32,
    pub fcoe_cvl_eventtag_attn: u32,
    pub fcf: lpfc_fcf,
    pub fc_map: [u8; 3],
    pub valid_vlan: u8,
    pub vlan_id: u16,
    pub fcf_conn_rec_list: list_head,
    pub defer_flogi_acc: lpfc_defer_flogi_acc,
    pub /: *mut *mut spinlock_t ct_ev_lock; / synchronize access to ct_ev_waiters,
    pub ct_ev_waiters: list_head,
    pub ct_ctx: [unsol_rcv_ct_ctx; LPFC_CT_CTX_MAX],
    pub ctx_idx: u32,
    pub inactive_vmid_poll: timer_list,
// RAS Support
    pub /: *mut *mut spinlock_t ras_fwlog_lock; / do not take while holding another lock,
    pub ras_fwlog: lpfc_ras_fwlog,
    pub iocb_cnt: u32,
    pub iocb_max: u32,
    pub sdev_cnt: core::sync::atomic::AtomicI32,
    pub /: *mut *mut spinlock_t devicelock; / lock for luns list,
    pub device_data_mem_pool: *mut mempool_t,
    pub luns: list_head,
pub const LPFC_TRANSGRESSION_HIGH_TEMPERATURE: c_uint = 0x0080;
pub const LPFC_TRANSGRESSION_LOW_TEMPERATURE: c_uint = 0x0040;
pub const LPFC_TRANSGRESSION_HIGH_VOLTAGE: c_uint = 0x0020;
pub const LPFC_TRANSGRESSION_LOW_VOLTAGE: c_uint = 0x0010;
pub const LPFC_TRANSGRESSION_HIGH_TXBIAS: c_uint = 0x0008;
pub const LPFC_TRANSGRESSION_LOW_TXBIAS: c_uint = 0x0004;
pub const LPFC_TRANSGRESSION_HIGH_TXPOWER: c_uint = 0x0002;
pub const LPFC_TRANSGRESSION_LOW_TXPOWER: c_uint = 0x0001;
pub const LPFC_TRANSGRESSION_HIGH_RXPOWER: c_uint = 0x8000;
pub const LPFC_TRANSGRESSION_LOW_RXPOWER: c_uint = 0x4000;
    pub sfp_alarm: u16,
    pub sfp_warning: u16,

    pub hdwqstat_on: u16,
pub const LPFC_CHECK_OFF: c_int = 0;
pub const LPFC_CHECK_NVME_IO: c_int = 1;
pub const LPFC_CHECK_NVMET_IO: c_int = 2;
pub const LPFC_CHECK_SCSI_IO: c_int = 4;
    pub ktime_on: u16,
    pub ktime_data_samples: u64,
    pub ktime_status_samples: u64,
    pub ktime_last_cmd: u64,
    pub ktime_seg1_total: u64,
    pub ktime_seg1_min: u64,
    pub ktime_seg1_max: u64,
    pub ktime_seg2_total: u64,
    pub ktime_seg2_min: u64,
    pub ktime_seg2_max: u64,
    pub ktime_seg3_total: u64,
    pub ktime_seg3_min: u64,
    pub ktime_seg3_max: u64,
    pub ktime_seg4_total: u64,
    pub ktime_seg4_min: u64,
    pub ktime_seg4_max: u64,
    pub ktime_seg5_total: u64,
    pub ktime_seg5_min: u64,
    pub ktime_seg5_max: u64,
    pub ktime_seg6_total: u64,
    pub ktime_seg6_min: u64,
    pub ktime_seg6_max: u64,
    pub ktime_seg7_total: u64,
    pub ktime_seg7_min: u64,
    pub ktime_seg7_max: u64,
    pub ktime_seg8_total: u64,
    pub ktime_seg8_min: u64,
    pub ktime_seg8_max: u64,
    pub ktime_seg9_total: u64,
    pub ktime_seg9_min: u64,
    pub ktime_seg9_max: u64,
    pub ktime_seg10_total: u64,
    pub ktime_seg10_min: u64,
    pub ktime_seg10_max: u64,

// CMF objects
    pub cmf_stat: *mut lpfc_cgn_stat __percpu,
    pub /: *mut *mut uint32_t cmf_interval_rate; / timer interval limit in ms,
    pub cmf_timer_cnt: u32,
pub const LPFC_CMF_INTERVAL: c_int = 90;
    pub cmf_link_byte_count: u64,
    pub cmf_max_line_rate: u64,
    pub cmf_max_bytes_per_interval: u64,
    pub cmf_last_sync_bw: u64,
pub const LPFC_CMF_BLK_SIZE: c_int = 512;
    pub cmf_timer: hrtimer,
    pub /: *mut *mut hrtimer cmf_stats_timer; / 1 minute stats timer,
    pub cmf_bw_wait: core::sync::atomic::AtomicI32,
    pub cmf_busy: core::sync::atomic::AtomicI32,
    pub /: *mut *mut atomic_t cmf_stop_io; / To block request and stop IO's,
    pub cmf_active_mode: u32,
    pub cmf_info_per_interval: u32,
pub const LPFC_MAX_CMF_INFO: c_int = 32;
    pub /: *mut *mut timespec64 cmf_latency; / Interval congestion timestamp,
    pub /: *mut *mut uint32_t cmf_last_ts; / Interval congestion time (ms),
    pub cmf_active_info: u32,
// Signal / FPIN handling for Congestion Mgmt
    pub /: *mut *mut u8 cgn_reg_fpin; / Negotiated value from RDF,
    pub /: *mut *mut u8 cgn_init_reg_fpin; / Initial value from READ_CONFIG,
pub const LPFC_CGN_FPIN_NONE: c_uint = 0x0;
pub const LPFC_CGN_FPIN_WARN: c_uint = 0x1;
pub const LPFC_CGN_FPIN_ALARM: c_uint = 0x2;

    pub /: *mut *mut u8 cgn_reg_signal; / Negotiated value from EDC,
    pub /: *mut *mut u8 cgn_init_reg_signal; / Initial value from READ_CONFIG,
// cgn_reg_signal and cgn_init_reg_signal use
// enum fc_edc_cg_signal_cap_types
//
    pub /: *mut *mut u16 cgn_fpin_frequency; / In units of msecs,
pub const LPFC_FPIN_INIT_FREQ: c_uint = 0xffff;
    pub cgn_sig_freq: u32,
    pub cgn_acqe_cnt: u32,
// RX monitor handling for CMF
    pub rx_monitor: *mut lpfc_rx_info_monitor,
    pub /: *mut *mut atomic_t rx_max_read_cnt; / Maximum read bytes,
    pub rx_block_cnt: u64,
// Congestion parameters from flash
    pub cgn_p: lpfc_cgn_param,
// Statistics counter for ACQE cgn alarms and warnings
    pub cgn_acqe_stat: lpfc_cgn_acqe_stat,
// Congestion buffer information
    pub /: *mut *mut *mut lpfc_dmabuf cgn_i; / Congestion Info buffer,
    pub /: *mut *mut atomic_t cgn_fabric_warn_cnt; / Total warning cgn events for info,
    pub /: *mut *mut atomic_t cgn_fabric_alarm_cnt; / Total alarm cgn events for info,
    pub /: *mut *mut atomic_t cgn_sync_warn_cnt; / Total warning events for SYNC wqe,
    pub /: *mut *mut atomic_t cgn_sync_alarm_cnt; / Total alarm events for SYNC wqe,
    pub /: *mut *mut atomic_t cgn_driver_evt_cnt; / Total driver cgn events for fmw,
    pub cgn_latency_evt_cnt: core::sync::atomic::AtomicI32,
    pub /: *mut *mut atomic64_t cgn_latency_evt; / Avg latency per minute,
    pub cgn_evt_timestamp: c_ulong,

    pub cgn_evt_minute: u32,

pub const LPFC_MIN_HOUR: c_int = 60;
pub const LPFC_HOUR_DAY: c_int = 24;

    pub /: *mut *mut hlist_node cpuhp; / used for cpuhp per hba callback,
    pub cpuhp_poll_timer: timer_list,
    pub /: *mut *mut list_head poll_list; / slowpath eq polling list,
    pub os_host_name: [c_char; MAXHOSTNAMELEN],
// LD Signaling
    pub degrade_activate_threshold: u32,
    pub degrade_deactivate_threshold: u32,
    pub fec_degrade_interval: u32,
    pub dbg_log_idx: core::sync::atomic::AtomicI32,
    pub dbg_log_cnt: core::sync::atomic::AtomicI32,
    pub dbg_log_dmping: core::sync::atomic::AtomicI32,
    pub dbg_log: [dbg_log_ent; DBG_LOG_SZ],
}

pub const LPFC_MAX_RXMONITOR_ENTRY: c_int = 800;
pub const LPFC_MAX_RXMONITOR_DUMP: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_info_entry {
    pub /: *mut *mut uint64_t cmf_bytes; / Total no of read bytes for CMF_SYNC_WQE,
    pub /: *mut *mut uint64_t total_bytes; / Total no of read bytes requested,
    pub /: *mut *mut uint64_t rcv_bytes; / Total no of read bytes completed,
    pub avg_io_size: u64,
    pub /: *mut *mut uint64_t avg_io_latency;/ Average io latency in microseconds,
    pub /: *mut *mut uint64_t max_read_cnt; / Maximum read bytes,
    pub max_bytes_per_interval: u64,
    pub cmf_busy: u32,
    pub /: *mut *mut uint32_t cmf_info; / CMF_SYNC_WQE info,
    pub io_cnt: u32,
    pub timer_utilization: u32,
    pub timer_interval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rx_info_monitor {
    pub /: *mut *mut *mut rx_info_entry ring; / info organized in a circular buffer,
    pub /: *mut *mut u32 head_idx, tail_idx; / index to head/tail of ring,
    pub /: *mut *mut spinlock_t lock; / spinlock for ring,
    pub /: *mut *mut u32 entries; / storing number entries/size of ring,
}

extern "C" {
    pub fn container_of(vport: *mut *mut (void ), Scsi_Host: struct, _arg: hostdata[0]) -> return;
}
// Set the lpfc data pending flag
// Wake up worker thread
// data = temp;
//
// There was a link/board error. Read the status register to retrieve
// the error event and process it.
//
// Save status info and check for unplug error
// Clear chip Host Attention error bit
// Return NULL if sli_rev has become invalid due to bad fw
//
// lpfc_next_online_cpu - Finds next online CPU on cpumask
// @mask: Pointer to phba's cpumask member.
// @start: starting cpu index
//
// Returns: next online CPU in @mask on success
//
// Note: If no valid cpu found, then nr_cpu_ids is returned.
//
extern "C" {
    pub fn cpumask_next_and_wrap(_arg: start, _arg: mask, _arg: cpu_online_mask) -> return;
}
//
// lpfc_next_present_cpu - Finds next present CPU after n
// @n: the cpu prior to search
//
// Returns: next present CPU after CPU @n
//
// Note: If no next present cpu, then fallback to first present cpu.
//
extern "C" {
    pub fn cpumask_next_wrap(_arg: n, _arg: cpu_present_mask) -> return;
}
//
// lpfc_sli4_mod_hba_eq_delay - update EQ delay
// @phba: Pointer to HBA context object.
// @eq: The Event Queue to update.
// @delay: The delay value (in us) to be written.
//
// Macro that declares tables and a routine to perform enum type to
// ascii string lookup.
//
// Defines a <key,value> table for an enum. Uses xxx_INIT defines for
// the enum to populate the table.  Macro defines a routine (named
// by caller) that will search all elements of the table for the key
// and return the name string if found or "Unrecognized" if not found.
//

//
// lpfc_is_vmid_enabled - returns if VMID is enabled for either switch types
// @phba: Pointer to HBA context object.
//
// Relationship between the enable, target support and if vmid tag is required
// for the particular combination
// ---------------------------------------------------
// Switch    Enable Flag  Target Support  VMID Needed
// ---------------------------------------------------
// App Id     0              NA              N
// App Id     1               0              N
// App Id     1               1              Y
// Pr Tag     0              NA              N
// Pr Tag     1               0              N
// Pr Tag     1               1              Y
// Pr Tag     2               *              Y
// ---------------------------------------------------
//
// Returns: whether VMID is enabled
//
extern "C" {
    pub fn bf_get(_arg: lpfc_wcqe_c_status, _arg: &iocbq->wcqe_cmpl) -> return;
}
extern "C" {
    pub fn bf_get(_arg: wqe_cmnd, _arg: &iocbq->wqe.generic.wqe_com) -> return;
}
extern "C" {
    pub fn bf_get(_arg: wqe_ctxt_tag, _arg: &iocbq->wqe.generic.wqe_com) -> return;
}
extern "C" {
    pub fn bf_get(_arg: wqe_rcvoxid, _arg: &iocbq->wqe.generic.wqe_com) -> return;
}
extern "C" {
    pub fn bf_get(_arg: wqe_els_did, _arg: &iocbq->wqe.els_req.wqe_dest) -> return;
}
