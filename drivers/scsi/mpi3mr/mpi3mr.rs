//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpi3mr/mpi3mr.h
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
//
// Driver for Broadcom MPI3 Storage Controllers
//
// Copyright (C) 2017-2023 Broadcom Inc.
// (mailto: mpi3mr-linuxdrv.pdl@broadcom.com)
//

// Macro flag: #define MPI3MR_H_INCLUDED

// Global list and lock for storing multiple adapters managed by the driver

pub const MPI3MR_NAME_LENGTH: c_int = 64;

// Definitions for internal SGL and Chain SGL buffers
pub const MPI3MR_PAGE_SIZE_4K: c_int = 4096;
pub const MPI3MR_DEFAULT_SGL_ENTRIES: c_int = 256;
pub const MPI3MR_MAX_SGL_ENTRIES: c_int = 2048;
// Definitions for MAX values for shost
pub const MPI3MR_MAX_CMDS_LUN: c_int = 128;
pub const MPI3MR_MAX_CDB_LENGTH: c_int = 32;
// Admin queue management definitions

pub const MPI3MR_ADMIN_REQ_FRAME_SZ: c_int = 128;
pub const MPI3MR_ADMIN_REPLY_FRAME_SZ: c_int = 16;
// Operational queue management definitions
pub const MPI3MR_OP_REQ_Q_QD: c_int = 512;
pub const MPI3MR_OP_REP_Q_QD: c_int = 1024;
pub const MPI3MR_OP_REP_Q_QD2K: c_int = 2048;
pub const MPI3MR_OP_REP_Q_QD4K: c_int = 4096;
pub const MPI3MR_OP_REQ_Q_SEG_SIZE: c_int = 4096;
pub const MPI3MR_OP_REP_Q_SEG_SIZE: c_int = 4096;
pub const MPI3MR_MAX_SEG_LIST_SIZE: c_int = 4096;
// Reserved Host Tag definitions
pub const MPI3MR_HOSTTAG_INVALID: c_uint = 0xFFFF;
pub const MPI3MR_HOSTTAG_INITCMDS: c_int = 1;
pub const MPI3MR_HOSTTAG_BSG_CMDS: c_int = 2;
pub const MPI3MR_HOSTTAG_PEL_ABORT: c_int = 3;
pub const MPI3MR_HOSTTAG_PEL_WAIT: c_int = 4;
pub const MPI3MR_HOSTTAG_BLK_TMS: c_int = 5;
pub const MPI3MR_HOSTTAG_CFG_CMDS: c_int = 6;
pub const MPI3MR_HOSTTAG_TRANSPORT_CMDS: c_int = 7;
pub const MPI3MR_NUM_DEVRMCMD: c_int = 16;

pub const MPI3MR_NUM_EVTACKCMD: c_int = 4;

// Reduced resource count definition for crash kernel
pub const MPI3MR_HOST_IOS_KDUMP: c_int = 128;
// command/controller interaction timeout definitions in seconds
pub const MPI3MR_INTADMCMD_TIMEOUT: c_int = 60;
pub const MPI3MR_PORTENABLE_TIMEOUT: c_int = 300;
pub const MPI3MR_PORTENABLE_POLL_INTERVAL: c_int = 5;
pub const MPI3MR_ABORTTM_TIMEOUT: c_int = 60;
pub const MPI3MR_RESETTM_TIMEOUT: c_int = 60;
pub const MPI3MR_RESET_HOST_IOWAIT_TIMEOUT: c_int = 5;
pub const MPI3MR_TSUPDATE_INTERVAL: c_int = 900;
pub const MPI3MR_DEFAULT_SHUTDOWN_TIME: c_int = 120;
pub const MPI3MR_RAID_ERRREC_RESET_TIMEOUT: c_int = 180;
pub const MPI3MR_PREPARE_FOR_RESET_TIMEOUT: c_int = 180;
pub const MPI3MR_RESET_ACK_TIMEOUT: c_int = 30;
pub const MPI3MR_MUR_TIMEOUT: c_int = 120;
pub const MPI3MR_RESET_TIMEOUT: c_int = 510;

pub const MPI3MR_RESET_TOPOLOGY_SETTLE_TIME: c_int = 10;

// Internal admin command state definitions
pub const MPI3MR_CMD_NOTUSED: c_uint = 0x8000;
pub const MPI3MR_CMD_COMPLETE: c_uint = 0x0001;
pub const MPI3MR_CMD_PENDING: c_uint = 0x0002;
pub const MPI3MR_CMD_REPLY_VALID: c_uint = 0x0004;
pub const MPI3MR_CMD_RESET: c_uint = 0x0008;
// Definitions for Event replies and sense buffer allocated per controller
pub const MPI3MR_NUM_EVT_REPLIES: c_int = 64;
pub const MPI3MR_SENSE_BUF_SZ: c_int = 256;
pub const MPI3MR_SENSEBUF_FACTOR: c_int = 3;
pub const MPI3MR_CHAINBUF_FACTOR: c_int = 3;
pub const MPI3MR_CHAINBUFDIX_FACTOR: c_int = 2;
// Invalid target device handle
pub const MPI3MR_INVALID_DEV_HANDLE: c_uint = 0xFFFF;
// Controller Reset related definitions
pub const MPI3MR_HOSTDIAG_UNLOCK_RETRY_COUNT: c_int = 5;
pub const MPI3MR_MAX_RESET_RETRY_COUNT: c_int = 3;
pub const MPI3MR_MAX_SHUTDOWN_RETRY_COUNT: c_int = 2;
// ResponseCode definitions

// Command retry count definitions
pub const MPI3MR_DEV_RMHS_RETRY_COUNT: c_int = 3;
pub const MPI3MR_PEL_RETRY_COUNT: c_int = 3;
// Default target device queue depth
pub const MPI3MR_DEFAULT_SDEV_QD: c_int = 32;
// Definitions for Threaded IRQ poll
pub const MPI3MR_IRQ_POLL_SLEEP: c_int = 20;
pub const MPI3MR_IRQ_POLL_TRIGGER_IOCOUNT: c_int = 8;
// Definitions for the controller security status
pub const MPI3MR_CTLR_SECURITY_STATUS_MASK: c_uint = 0x0C;
pub const MPI3MR_CTLR_SECURE_DBG_STATUS_MASK: c_uint = 0x02;
pub const MPI3MR_INVALID_DEVICE: c_uint = 0x00;
pub const MPI3MR_CONFIG_SECURE_DEVICE: c_uint = 0x04;
pub const MPI3MR_HARD_SECURE_DEVICE: c_uint = 0x08;
pub const MPI3MR_TAMPERED_DEVICE: c_uint = 0x0C;

pub const MPI3MR_MAX_NUM_HDB: c_int = 2;
pub const MPI3MR_HDB_TRIGGER_TYPE_UNKNOWN: c_int = 0;
pub const MPI3MR_HDB_TRIGGER_TYPE_FAULT: c_int = 1;
pub const MPI3MR_HDB_TRIGGER_TYPE_ELEMENT: c_int = 2;
pub const MPI3MR_HDB_TRIGGER_TYPE_GLOBAL: c_int = 3;
pub const MPI3MR_HDB_TRIGGER_TYPE_SOFT_RESET: c_int = 4;
pub const MPI3MR_HDB_TRIGGER_TYPE_FW_RELEASED: c_int = 5;
pub const MPI3MR_HDB_REFRESH_TYPE_RESERVED: c_int = 0;
pub const MPI3MR_HDB_REFRESH_TYPE_CURRENT: c_int = 1;
pub const MPI3MR_HDB_REFRESH_TYPE_DEFAULT: c_int = 2;
pub const MPI3MR_HDB_HDB_REFRESH_TYPE_PERSISTENT: c_int = 3;

pub const MPI3MR_MAX_NUM_HDB: c_int = 2;
pub const MPI3MR_HDB_QUERY_ELEMENT_TRIGGER_FORMAT_INDEX: c_int = 0;
pub const MPI3MR_HDB_QUERY_ELEMENT_TRIGGER_FORMAT_DATA: c_int = 1;
pub const MPI3MR_THRESHOLD_REPLY_COUNT: c_int = 100;
// SGE Flag definition

// MSI Index from Reply Queue Index

//
// Maximum data transfer size definitions for management
// application commands
//

pub const MPI3MR_MAX_APP_XFER_SEGMENTS: c_int = 512;
//
// 2048 sectors are for data buffers and additional 512 sectors for
// other buffers
//

pub const MPI3MR_WRITE_SAME_MAX_LEN_256_BLKS: c_int = 256;
pub const MPI3MR_WRITE_SAME_MAX_LEN_2048_BLKS: c_int = 2048;

//
// struct mpi3mr_nvme_pt_sge -  Structure to store SGEs for NVMe
// Encapsulated commands.
//
// @base_addr: Physical address
// @length: SGE length
// @rsvd: Reserved
// @rsvd1: Reserved
// @sub_type: sgl sub type
// @type: sgl type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_nvme_pt_sge {
    pub base_addr: __le64,
    pub length: __le32,
    pub rsvd: u16,
    pub rsvd1: u8,
    pub sub_type:4: u8,
    pub type:4: u8,
}

//
// struct mpi3mr_buf_map -  local structure to
// track kernel and user buffers associated with an BSG
// structure.
//
// @bsg_buf: BSG buffer virtual address
// @bsg_buf_len:  BSG buffer length
// @kern_buf: Kernel buffer virtual address
// @kern_buf_len: Kernel buffer length
// @kern_buf_dma: Kernel buffer DMA address
// @data_dir: Data direction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_buf_map {
    pub bsg_buf: *mut c_void,
    pub bsg_buf_len: u32,
    pub kern_buf: *mut c_void,
    pub kern_buf_len: u32,
    pub kern_buf_dma: dma_addr_t,
    pub data_dir: u8,
    pub num_dma_desc: u16,
    pub dma_desc: *mut dma_memory_desc,
}

// IOC State definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpi3mr_iocstate {
    MRIOC_STATE_READY = 1,
    MRIOC_STATE_RESET,
    MRIOC_STATE_FAULT,
    MRIOC_STATE_BECOMING_READY,
    MRIOC_STATE_RESET_REQUESTED,
    MRIOC_STATE_UNRECOVERABLE,
}

// Reset reason code definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpi3mr_reset_reason {
    MPI3MR_RESET_FROM_BRINGUP = 1,
    MPI3MR_RESET_FROM_FAULT_WATCH = 2,
    MPI3MR_RESET_FROM_APP = 3,
    MPI3MR_RESET_FROM_EH_HOS = 4,
    MPI3MR_RESET_FROM_TM_TIMEOUT = 5,
    MPI3MR_RESET_FROM_APP_TIMEOUT = 6,
    MPI3MR_RESET_FROM_MUR_FAILURE = 7,
    MPI3MR_RESET_FROM_CTLR_CLEANUP = 8,
    MPI3MR_RESET_FROM_CIACTIV_FAULT = 9,
    MPI3MR_RESET_FROM_PE_TIMEOUT = 10,
    MPI3MR_RESET_FROM_TSU_TIMEOUT = 11,
    MPI3MR_RESET_FROM_DELREQQ_TIMEOUT = 12,
    MPI3MR_RESET_FROM_DELREPQ_TIMEOUT = 13,
    MPI3MR_RESET_FROM_CREATEREPQ_TIMEOUT = 14,
    MPI3MR_RESET_FROM_CREATEREQQ_TIMEOUT = 15,
    MPI3MR_RESET_FROM_IOCFACTS_TIMEOUT = 16,
    MPI3MR_RESET_FROM_IOCINIT_TIMEOUT = 17,
    MPI3MR_RESET_FROM_EVTNOTIFY_TIMEOUT = 18,
    MPI3MR_RESET_FROM_EVTACK_TIMEOUT = 19,
    MPI3MR_RESET_FROM_CIACTVRST_TIMER = 20,
    MPI3MR_RESET_FROM_GETPKGVER_TIMEOUT = 21,
    MPI3MR_RESET_FROM_PELABORT_TIMEOUT = 22,
    MPI3MR_RESET_FROM_SYSFS = 23,
    MPI3MR_RESET_FROM_SYSFS_TIMEOUT = 24,
    MPI3MR_RESET_FROM_DIAG_BUFFER_POST_TIMEOUT = 25,
    MPI3MR_RESET_FROM_DIAG_BUFFER_RELEASE_TIMEOUT = 26,
    MPI3MR_RESET_FROM_FIRMWARE = 27,
    MPI3MR_RESET_FROM_CFG_REQ_TIMEOUT = 29,
    MPI3MR_RESET_FROM_SAS_TRANSPORT_TIMEOUT = 30,
    MPI3MR_RESET_FROM_TRIGGER = 31,
    MPI3MR_RESET_FROM_INVALID_COMPLETION = 32,
}

pub const MPI3MR_RESET_REASON_OSTYPE_LINUX: c_int = 1;
pub const MPI3MR_RESET_REASON_OSTYPE_SHIFT: c_int = 28;
pub const MPI3MR_RESET_REASON_IOCNUM_SHIFT: c_int = 20;
// Queue type definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum queue_type {
    MPI3MR_DEFAULT_QUEUE = 0,
    MPI3MR_POLL_QUEUE,
}

//
// struct mpi3mr_compimg_ver - replica of component image
// version defined in mpi30_image.h in host endianness
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_compimg_ver {
    pub build_num: u16,
    pub cust_id: u16,
    pub ph_minor: u8,
    pub ph_major: u8,
    pub gen_minor: u8,
    pub gen_major: u8,
}

//
// struct mpi3mr_ioc_facs - replica of component image version
// defined in mpi30_ioc.h in host endianness
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_ioc_facts {
    pub ioc_capabilities: u32,
    pub fw_ver: mpi3mr_compimg_ver,
    pub mpi_version: u32,
    pub diag_trace_sz: u32,
    pub diag_fw_sz: u32,
    pub diag_drvr_sz: u32,
    pub max_reqs: u16,
    pub product_id: u16,
    pub op_req_sz: u16,
    pub reply_sz: u16,
    pub exceptions: u16,
    pub max_perids: u16,
    pub max_pds: u16,
    pub max_sasexpanders: u16,
    pub max_data_length: u32,
    pub max_sasinitiators: u16,
    pub max_enclosures: u16,
    pub max_pcie_switches: u16,
    pub max_nvme: u16,
    pub max_vds: u16,
    pub max_hpds: u16,
    pub max_advhpds: u16,
    pub max_raid_pds: u16,
    pub min_devhandle: u16,
    pub max_devhandle: u16,
    pub max_op_req_q: u16,
    pub max_op_reply_q: u16,
    pub shutdown_timeout: u16,
    pub ioc_num: u8,
    pub who_init: u8,
    pub max_msix_vectors: u16,
    pub personality: u8,
    pub dma_mask: u8,
    pub max_req_limit: bool,
    pub protocol_flags: u8,
    pub sge_mod_mask: u8,
    pub sge_mod_value: u8,
    pub sge_mod_shift: u8,
    pub max_dev_per_tg: u8,
    pub max_io_throttle_group: u16,
    pub io_throttle_data_length: u16,
    pub io_throttle_low: u16,
    pub io_throttle_high: u16,
}

//
// struct segments - memory descriptor structure to store
// virtual and dma addresses for operational queue segments.
//
// @segment: virtual address
// @segment_dma: dma address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct segments {
    pub segment: *mut c_void,
    pub segment_dma: dma_addr_t,
}

//
// struct op_req_qinfo -  Operational Request Queue Information
//
// @ci: consumer index
// @pi: producer index
// @num_request: Maximum number of entries in the queue
// @qid: Queue Id starting from 1
// @reply_qid: Associated reply queue Id
// @num_segments: Number of discontiguous memory segments
// @segment_qd: Depth of each segments
// @q_lock: Concurrent queue access lock
// @q_segments: Segment descriptor pointer
// @q_segment_list: Segment list base virtual address
// @q_segment_list_dma: Segment list base DMA address
// @last_full_host_tag: Hosttag of last IO returned to SML
// due to queue full
// @qfull_io_count: Number of IOs returned back to SML
// due to queue full
// @qfull_instances: Total queue full occurrences.One occurrence
// starts with queue full detection and ends
// with queue full breaks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_req_qinfo {
    pub ci: u16,
    pub pi: u16,
    pub num_requests: u16,
    pub qid: u16,
    pub reply_qid: u16,
    pub num_segments: u16,
    pub segment_qd: u16,
    pub q_lock: spinlock_t,
    pub q_segments: *mut segments,
    pub q_segment_list: *mut c_void,
    pub q_segment_list_dma: dma_addr_t,
    pub last_full_host_tag: u16,
    pub qfull_io_count: u64,
    pub qfull_instances: u32,
}

//
// struct op_reply_qinfo -  Operational Reply Queue Information
//
// @ci: consumer index
// @qid: Queue Id starting from 1
// @num_replies: Maximum number of entries in the queue
// @num_segments: Number of discontiguous memory segments
// @segment_qd: Depth of each segments
// @q_segments: Segment descriptor pointer
// @q_segment_list: Segment list base virtual address
// @q_segment_list_dma: Segment list base DMA address
// @ephase: Expected phased identifier for the reply queue
// @pend_ios: Number of IOs pending in HW for this queue
// @enable_irq_poll: Flag to indicate polling is enabled
// @in_use: Queue is handled by poll/ISR
// @qtype: Type of queue (types defined in enum queue_type)
// @qfull_watermark: Watermark defined in reply queue to avoid
// reply queue full
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_reply_qinfo {
    pub ci: u16,
    pub qid: u16,
    pub num_replies: u16,
    pub num_segments: u16,
    pub segment_qd: u16,
    pub q_segments: *mut segments,
    pub q_segment_list: *mut c_void,
    pub q_segment_list_dma: dma_addr_t,
    pub ephase: u8,
    pub pend_ios: core::sync::atomic::AtomicI32,
    pub enable_irq_poll: bool,
    pub in_use: core::sync::atomic::AtomicI32,
    pub qtype: queue_type,
    pub qfull_watermark: u16,
}

//
// struct mpi3mr_intr_info -  Interrupt cookie information
//
// @mrioc: Adapter instance reference
// @os_irq: irq number
// @msix_index: MSIx index
// @op_reply_q: Associated operational reply queue
// @name: Dev name for the irq claiming device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_intr_info {
    pub mrioc: *mut mpi3mr_ioc,
    pub os_irq: c_int,
    pub msix_index: u16,
    pub op_reply_q: *mut op_reply_qinfo,
    pub name: [c_char; MPI3MR_NAME_LENGTH],
}

//
// struct mpi3mr_throttle_group_info - Throttle group info
//
// @io_divert: Flag indicates io divert is on or off for the TG
// @need_qd_reduction: Flag to indicate QD reduction is needed
// @qd_reduction: Queue Depth reduction in units of 10%
// @fw_qd: QueueDepth value reported by the firmware
// @modified_qd: Modified QueueDepth value due to throttling
// @id: Throttle Group ID.
// @high: High limit to turn on throttling in 512 byte blocks
// @low: Low limit to turn off throttling in 512 byte blocks
// @pend_large_data_sz: Counter to track pending large data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_throttle_group_info {
    pub io_divert: u8,
    pub need_qd_reduction: u8,
    pub qd_reduction: u8,
    pub fw_qd: u16,
    pub modified_qd: u16,
    pub id: u16,
    pub high: u32,
    pub low: u32,
    pub pend_large_data_sz: core::sync::atomic::AtomicI32,
}

// HBA port flags
pub const MPI3MR_HBA_PORT_FLAG_DIRTY: c_uint = 0x01;
pub const MPI3MR_HBA_PORT_FLAG_NEW: c_uint = 0x02;
// IOCTL data transfer sge
pub const MPI3MR_NUM_IOCTL_SGE: c_int = 256;

//
// struct mpi3mr_hba_port - HBA's port information
// @port_id: Port number
// @flags: HBA port flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_hba_port {
    pub list: list_head,
    pub port_id: u8,
    pub flags: u8,
}

//
// struct mpi3mr_sas_port - Internal SAS port information
// @port_list: List of ports belonging to a SAS node
// @num_phys: Number of phys associated with port
// @marked_responding: used while refresing the sas ports
// @lowest_phy: lowest phy ID of current sas port, valid for controller port
// @phy_mask: phy_mask of current sas port, valid for controller port
// @hba_port: HBA port entry
// @remote_identify: Attached device identification
// @rphy: SAS transport layer rphy object
// @port: SAS transport layer port object
// @phy_list: mpi3mr_sas_phy objects belonging to this port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_sas_port {
    pub port_list: list_head,
    pub num_phys: u8,
    pub marked_responding: u8,
    pub lowest_phy: c_int,
    pub phy_mask: u64,
    pub hba_port: *mut mpi3mr_hba_port,
    pub remote_identify: sas_identify,
    pub rphy: *mut sas_rphy,
    pub port: *mut sas_port,
    pub phy_list: list_head,
}

//
// struct mpi3mr_sas_phy - Internal SAS Phy information
// @port_siblings: List of phys belonging to a port
// @identify: Phy identification
// @remote_identify: Attached device identification
// @phy: SAS transport layer Phy object
// @phy_id: Unique phy id within a port
// @handle: Firmware device handle for this phy
// @attached_handle: Firmware device handle for attached device
// @phy_belongs_to_port: Flag to indicate phy belongs to port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_sas_phy {
    pub port_siblings: list_head,
    pub identify: sas_identify,
    pub remote_identify: sas_identify,
    pub phy: *mut sas_phy,
    pub phy_id: u8,
    pub handle: u16,
    pub attached_handle: u16,
    pub phy_belongs_to_port: u8,
    pub hba_port: *mut mpi3mr_hba_port,
}

//
// struct mpi3mr_sas_node - SAS host/expander information
// @list: List of sas nodes in a controller
// @parent_dev: Parent device class
// @num_phys: Number phys belonging to sas_node
// @sas_address: SAS address of sas_node
// @handle: Firmware device handle for this sas_host/expander
// @sas_address_parent: SAS address of parent expander or host
// @enclosure_handle: Firmware handle of enclosure of this node
// @device_info: Capabilities of this sas_host/expander
// @non_responding: used to refresh the expander devices during reset
// @host_node: Flag to indicate this is a host_node
// @hba_port: HBA port entry
// @phy: A list of phys that make up this sas_host/expander
// @sas_port_list: List of internal ports of this node
// @rphy: sas_rphy object of this expander node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_sas_node {
    pub list: list_head,
    pub parent_dev: *mut device,
    pub num_phys: u8,
    pub sas_address: u64,
    pub handle: u16,
    pub sas_address_parent: u64,
    pub enclosure_handle: u16,
    pub enclosure_logical_id: u64,
    pub non_responding: u8,
    pub host_node: u8,
    pub hba_port: *mut mpi3mr_hba_port,
    pub phy: *mut mpi3mr_sas_phy,
    pub sas_port_list: list_head,
    pub rphy: *mut sas_rphy,
}

//
// struct mpi3mr_enclosure_node - enclosure information
// @list: List of enclosures
// @pg0: Enclosure page 0;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_enclosure_node {
    pub list: list_head,
    pub pg0: mpi3_enclosure_page0,
}

//
// struct tgt_dev_sas_sata - SAS/SATA device specific
// information cached from firmware given data
//
// @sas_address: World wide unique SAS address
// @sas_address_parent: Sas address of parent expander or host
// @dev_info: Device information bits
// @phy_id: Phy identifier provided in device page 0
// @attached_phy_id: Attached phy identifier provided in device page 0
// @negotiated_link_rate: Negotiated link rate from device page 0
// @sas_transport_attached: Is this device exposed to transport
// @pend_sas_rphy_add: Flag to check device is in process of add
// @hba_port: HBA port entry
// @rphy: SAS transport layer rphy object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tgt_dev_sas_sata {
    pub sas_address: u64,
    pub sas_address_parent: u64,
    pub dev_info: u16,
    pub phy_id: u8,
    pub attached_phy_id: u8,
    pub negotiated_link_rate: u8,
    pub sas_transport_attached: u8,
    pub pend_sas_rphy_add: u8,
    pub hba_port: *mut mpi3mr_hba_port,
    pub rphy: *mut sas_rphy,
}

//
// struct tgt_dev_pcie - PCIe device specific information cached
// from firmware given data
//
// @mdts: Maximum data transfer size
// @capb: Device capabilities
// @pgsz: Device page size
// @abort_to: Timeout for abort TM
// @reset_to: Timeout for Target/LUN reset TM
// @dev_info: Device information bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tgt_dev_pcie {
    pub mdts: u32,
    pub capb: u16,
    pub pgsz: u8,
    pub abort_to: u8,
    pub reset_to: u8,
    pub dev_info: u16,
}

//
// struct tgt_dev_vd - virtual device specific information
// cached from firmware given data
//
// @state: State of the VD
// @tg_qd_reduction: Queue Depth reduction in units of 10%
// @tg_id: VDs throttle group ID
// @high: High limit to turn on throttling in 512 byte blocks
// @low: Low limit to turn off throttling in 512 byte blocks
// @tg: Pointer to throttle group info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tgt_dev_vd {
    pub state: u8,
    pub tg_qd_reduction: u8,
    pub tg_id: u16,
    pub tg_high: u32,
    pub tg_low: u32,
    pub abort_to: u8,
    pub reset_to: u8,
    pub tg: *mut mpi3mr_throttle_group_info,
}

//
// union _form_spec_inf - union of device specific information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union _form_spec_inf {
    pub sas_sata_inf: tgt_dev_sas_sata,
    pub pcie_inf: tgt_dev_pcie,
    pub vd_inf: tgt_dev_vd,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpi3mr_dev_state {
    MPI3MR_DEV_CREATED = 1,
    MPI3MR_DEV_REMOVE_HS_STARTED = 2,
    MPI3MR_DEV_DELETED = 3,
}

//
// struct mpi3mr_tgt_dev - target device data structure
//
// @list: List pointer
// @starget: Scsi_target pointer
// @dev_handle: FW device handle
// @parent_handle: FW parent device handle
// @slot: Slot number
// @encl_handle: FW enclosure handle
// @perst_id: FW assigned Persistent ID
// @devpg0_flag: Device Page0 flag
// @dev_type: SAS/SATA/PCIE device type
// @is_hidden: Should be exposed to upper layers or not
// @host_exposed: Already exposed to host or not
// @io_unit_port: IO Unit port ID
// @non_stl: Is this device not to be attached with SAS TL
// @io_throttle_enabled: I/O throttling needed or not
// @wslen: Write same max length
// @q_depth: Device specific Queue Depth
// @wwid: World wide ID
// @enclosure_logical_id: Enclosure logical identifier
// @dev_spec: Device type specific information
// @abort_to: Timeout for abort TM
// @reset_to: Timeout for Target/LUN reset TM
// @ref_count: Reference count
// @state: device state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_tgt_dev {
    pub list: list_head,
    pub starget: *mut scsi_target,
    pub dev_handle: u16,
    pub parent_handle: u16,
    pub slot: u16,
    pub encl_handle: u16,
    pub perst_id: u16,
    pub devpg0_flag: u16,
    pub dev_type: u8,
    pub is_hidden: u8,
    pub host_exposed: u8,
    pub io_unit_port: u8,
    pub non_stl: u8,
    pub io_throttle_enabled: u8,
    pub wslen: u16,
    pub q_depth: u16,
    pub wwid: u64,
    pub enclosure_logical_id: u64,
    pub dev_spec: _form_spec_inf,
    pub ref_count: kref,
    pub state: mpi3mr_dev_state,
}

//
// mpi3mr_tgtdev_get - k reference incrementor
// @s: Target device reference
//
// Increment target device reference count.
//
// mpi3mr_free_tgtdev - target device memory dealloctor
// @r: k reference pointer of the target device
//
// Free target device memory when no reference.
//
// mpi3mr_tgtdev_put - k reference decrementor
// @s: Target device reference
//
// Decrement target device reference count.
//
// struct mpi3mr_stgt_priv_data - SCSI target private structure
//
// @starget: Scsi_target pointer
// @dev_handle: FW device handle
// @perst_id: FW assigned Persistent ID
// @num_luns: Number of Logical Units
// @block_io: I/O blocked to the device or not
// @dev_removed: Device removed in the Firmware
// @dev_removedelay: Device is waiting to be removed in FW
// @dev_type: Device type
// @dev_nvme_dif: Device is NVMe DIF enabled
// @wslen: Write same max length
// @io_throttle_enabled: I/O throttling needed or not
// @io_divert: Flag indicates io divert is on or off for the dev
// @throttle_group: Pointer to throttle group info
// @tgt_dev: Internal target device pointer
// @pend_count: Counter to track pending I/Os during error
// handling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_stgt_priv_data {
    pub starget: *mut scsi_target,
    pub dev_handle: u16,
    pub perst_id: u16,
    pub num_luns: u32,
    pub block_io: core::sync::atomic::AtomicI32,
    pub dev_removed: u8,
    pub dev_removedelay: u8,
    pub dev_type: u8,
    pub dev_nvme_dif: u8,
    pub wslen: u16,
    pub io_throttle_enabled: u8,
    pub io_divert: u8,
    pub throttle_group: *mut mpi3mr_throttle_group_info,
    pub tgt_dev: *mut mpi3mr_tgt_dev,
    pub pend_count: u32,
}

//
// struct mpi3mr_stgt_priv_data - SCSI device private structure
//
// @tgt_priv_data: Scsi_target private data pointer
// @lun_id: LUN ID of the device
// @ncq_prio_enable: NCQ priority enable for SATA device
// @pend_count: Counter to track pending I/Os during error
// handling
// @wslen: Write same max length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_sdev_priv_data {
    pub tgt_priv_data: *mut mpi3mr_stgt_priv_data,
    pub lun_id: u32,
    pub ncq_prio_enable: u8,
    pub pend_count: u32,
    pub wslen: u16,
}

//
// struct mpi3mr_drv_cmd - Internal command tracker
//
// @mutex: Command mutex
// @done: Completeor for wakeup
// @reply: Firmware reply for internal commands
// @sensebuf: Sensebuf for SCSI IO commands
// @iou_rc: IO Unit control reason code
// @state: Command State
// @dev_handle: Firmware handle for device specific commands
// @ioc_status: IOC status from the firmware
// @ioc_loginfo:IOC log info from the firmware
// @is_waiting: Is the command issued in block mode
// @is_sense: Is Sense data present
// @retry_count: Retry count for retriable commands
// @host_tag: Host tag used by the command
// @callback: Callback for non blocking commands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_drv_cmd {
    pub mutex: mutex,
    pub done: completion,
    pub reply: *mut c_void,
    pub sensebuf: *mut u8,
    pub iou_rc: u8,
    pub state: u16,
    pub dev_handle: u16,
    pub ioc_status: u16,
    pub ioc_loginfo: u32,
    pub is_waiting: u8,
    pub is_sense: u8,
    pub retry_count: u8,
    pub host_tag: u16,
    pub drv_cmd): *mut mpi3mr_drv_cmd,
}

//
// union mpi3mr_trigger_data - Trigger data information
// @fault: Fault code
// @global: Global trigger data
// @element: element trigger data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3mr_trigger_data {
    pub fault: u16,
    pub global: u64,
    pub element: mpi3_driver2_trigger_element,
}

//
// struct trigger_event_data - store trigger related
// information.
//
// @trace_hdb: Trace diag buffer descriptor reference
// @fw_hdb: FW diag buffer descriptor reference
// @trigger_type: Trigger type
// @trigger_specific_data: Trigger specific data
// @snapdump: Snapdump enable or disable flag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trigger_event_data {
    pub trace_hdb: *mut diag_buffer_desc,
    pub fw_hdb: *mut diag_buffer_desc,
    pub trigger_type: u8,
    pub trigger_specific_data: mpi3mr_trigger_data,
    pub snapdump: bool,
}

//
// struct diag_buffer_desc - memory descriptor structure to
// store virtual, dma addresses, size, buffer status for host
// diagnostic buffers.
//
// @type: Buffer type
// @trigger_data: Trigger data
// @trigger_type: Trigger type
// @status: Buffer status
// @size: Buffer size
// @addr: Virtual address
// @dma_addr: Buffer DMA address
// @is_segmented: The buffer is segmented or not
// @disabled_after_reset: The buffer is disabled after reset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag_buffer_desc {
    pub type: u8,
    pub trigger_data: mpi3mr_trigger_data,
    pub trigger_type: u8,
    pub status: u8,
    pub size: u32,
    pub addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub is_segmented: bool,
    pub disabled_after_reset: bool,
}

//
// struct dma_memory_desc - memory descriptor structure to store
// virtual address, dma address and size for any generic dma
// memory allocations in the driver.
//
// @size: buffer size
// @addr: virtual address
// @dma_addr: dma address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_memory_desc {
    pub size: u32,
    pub addr: *mut c_void,
    pub dma_addr: dma_addr_t,
}

//
// struct chain_element - memory descriptor structure to store
// virtual and dma addresses for chain elements.
//
// @addr: virtual address
// @dma_addr: dma address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chain_element {
    pub addr: *mut c_void,
    pub dma_addr: dma_addr_t,
}

//
// struct scmd_priv - SCSI command private data
//
// @host_tag: Host tag specific to operational queue
// @in_lld_scope: Command in LLD scope or not
// @meta_sg_valid: DIX command with meta data SGL or not
// @scmd: SCSI Command pointer
// @req_q_idx: Operational request queue index
// @chain_idx: Chain frame index
// @meta_chain_idx: Chain frame index of meta data SGL
// @mpi3mr_scsiio_req: MPI SCSI IO request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmd_priv {
    pub host_tag: u16,
    pub in_lld_scope: u8,
    pub meta_sg_valid: u8,
    pub scmd: *mut scsi_cmnd,
    pub req_q_idx: u16,
    pub chain_idx: c_int,
    pub meta_chain_idx: c_int,
    pub mpi3mr_scsiio_req: [u8; MPI3MR_ADMIN_REQ_FRAME_SZ],
}

//
// struct mpi3mr_ioc - Adapter anchor structure stored in shost
// private data
//
// @list: List pointer
// @pdev: PCI device pointer
// @shost: Scsi_Host pointer
// @id: Controller ID
// @cpu_count: Number of online CPUs
// @irqpoll_sleep: usleep unit used in threaded isr irqpoll
// @name: Controller ASCII name
// @driver_name: Driver ASCII name
// @sysif_regs: System interface registers virtual address
// @sysif_regs_phys: System interface registers physical address
// @bars: PCI BARS
// @dma_mask: DMA mask
// @msix_count: Number of MSIX vectors used
// @intr_enabled: Is interrupts enabled
// @num_admin_req: Number of admin requests
// @admin_req_q_sz: Admin request queue size
// @admin_req_pi: Admin request queue producer index
// @admin_req_ci: Admin request queue consumer index
// @admin_req_base: Admin request queue base virtual address
// @admin_req_dma: Admin request queue base dma address
// @admin_req_lock: Admin queue access lock
// @num_admin_replies: Number of admin replies
// @admin_reply_q_sz: Admin reply queue size
// @admin_reply_ci: Admin reply queue consumer index
// @admin_reply_ephase:Admin reply queue expected phase
// @admin_reply_base: Admin reply queue base virtual address
// @admin_reply_dma: Admin reply queue base dma address
// @admin_reply_q_in_use: Queue is handled by poll/ISR
// @admin_pend_isr: Count of unprocessed admin ISR/poll calls
// due to another thread processing replies
// @ready_timeout: Controller ready timeout
// @intr_info: Interrupt cookie pointer
// @intr_info_count: Number of interrupt cookies
// @is_intr_info_set: Flag to indicate intr info is setup
// @num_queues: Number of operational queues
// @num_op_req_q: Number of operational request queues
// @req_qinfo: Operational request queue info pointer
// @num_op_reply_q: Number of operational reply queues
// @op_reply_qinfo: Operational reply queue info pointer
// @init_cmds: Command tracker for initialization commands
// @cfg_cmds: Command tracker for configuration requests
// @facts: Cached IOC facts data
// @op_reply_desc_sz: Operational reply descriptor size
// @num_reply_bufs: Number of reply buffers allocated
// @reply_buf_pool: Reply buffer pool
// @reply_buf: Reply buffer base virtual address
// @reply_buf_dma: Reply buffer DMA address
// @reply_buf_dma_max_address: Reply DMA address max limit
// @reply_free_qsz: Reply free queue size
// @reply_free_q_pool: Reply free queue pool
// @reply_free_q: Reply free queue base virtual address
// @reply_free_q_dma: Reply free queue base DMA address
// @reply_free_queue_lock: Reply free queue lock
// @reply_free_queue_host_index: Reply free queue host index
// @num_sense_bufs: Number of sense buffers
// @sense_buf_pool: Sense buffer pool
// @sense_buf: Sense buffer base virtual address
// @sense_buf_dma: Sense buffer base DMA address
// @sense_buf_q_sz: Sense buffer queue size
// @sense_buf_q_pool: Sense buffer queue pool
// @sense_buf_q: Sense buffer queue virtual address
// @sense_buf_q_dma: Sense buffer queue DMA address
// @sbq_lock: Sense buffer queue lock
// @sbq_host_index: Sense buffer queuehost index
// @event_masks: Event mask bitmap
// @fwevt_worker_thread: Firmware event worker thread
// @fwevt_lock: Firmware event lock
// @fwevt_list: Firmware event list
// @watchdog_work_q: Fault watchdog worker thread
// @watchdog_work: Fault watchdog work
// @watchdog_lock: Fault watchdog lock
// @is_driver_loading: Is driver still loading
// @scan_started: Async scan started
// @scan_failed: Asycn scan failed
// @stop_drv_processing: Stop all command processing
// @device_refresh_on: Don't process the events until devices are refreshed
// @max_host_ios: Maximum host I/O count
// @max_sgl_entries: Max SGL entries per I/O
// @chain_buf_count: Chain buffer count
// @chain_buf_pool: Chain buffer pool
// @chain_sgl_list: Chain SGL list
// @chain_bitmap: Chain buffer allocator bitmap
// @chain_buf_lock: Chain buffer list lock
// @bsg_cmds: Command tracker for BSG command
// @host_tm_cmds: Command tracker for task management commands
// @dev_rmhs_cmds: Command tracker for device removal commands
// @evtack_cmds: Command tracker for event ack commands
// @devrem_bitmap: Device removal bitmap
// @dev_handle_bitmap_bits: Number of bits in device handle bitmap
// @removepend_bitmap: Remove pending bitmap
// @delayed_rmhs_list: Delayed device removal list
// @evtack_cmds_bitmap: Event Ack bitmap
// @delayed_evtack_cmds_list: Delayed event acknowledgment list
// @ts_update_counter: Timestamp update counter
// @ts_update_interval: Timestamp update interval
// @reset_in_progress: Reset in progress flag
// @unrecoverable: Controller unrecoverable flag
// @io_admin_reset_sync: Manage state of I/O ops during an admin reset process
// @prev_reset_result: Result of previous reset
// @reset_mutex: Controller reset mutex
// @reset_waitq: Controller reset  wait queue
// @prepare_for_reset: Prepare for reset event received
// @prepare_for_reset_timeout_counter: Prepare for reset timeout
// @prp_list_virt: NVMe encapsulated PRP list virtual base
// @prp_list_dma: NVMe encapsulated PRP list DMA
// @prp_sz: NVME encapsulated PRP list size
// @diagsave_timeout: Diagnostic information save timeout
// @logging_level: Controller debug logging level
// @flush_io_count: I/O count to flush after reset
// @current_event: Firmware event currently in process
// @driver_info: Driver, Kernel, OS information to firmware
// @change_count: Topology change count
// @pel_enabled: Persistent Event Log(PEL) enabled or not
// @pel_abort_requested: PEL abort is requested or not
// @pel_class: PEL Class identifier
// @pel_locale: PEL Locale identifier
// @pel_cmds: Command tracker for PEL wait command
// @pel_abort_cmd: Command tracker for PEL abort command
// @pel_newest_seqnum: Newest PEL sequenece number
// @pel_seqnum_virt: PEL sequence number virtual address
// @pel_seqnum_dma: PEL sequence number DMA address
// @pel_seqnum_sz: PEL sequenece number size
// @op_reply_q_offset: Operational reply queue offset with MSIx
// @default_qcount: Total Default queues
// @active_poll_qcount: Currently active poll queue count
// @requested_poll_qcount: User requested poll queue count
// @fault_during_init: Indicates a firmware fault occurred during initialization
// @saved_fault_code: Firmware fault code captured at the time of failure
// @saved_fault_info: Additional firmware-provided fault information
// @fwfault_counter: Count of firmware faults detected by the driver
// @bsg_dev: BSG device structure
// @bsg_queue: Request queue for BSG device
// @stop_bsgs: Stop BSG request flag
// @logdata_buf: Circular buffer to store log data entries
// @logdata_buf_idx: Index of entry in buffer to store
// @logdata_entry_sz: log data entry size
// @adm_req_q_bar_writeq_lock: Admin request queue lock
// @adm_reply_q_bar_writeq_lock: Admin reply queue lock
// @pend_large_data_sz: Counter to track pending large data
// @io_throttle_data_length: I/O size to track in 512b blocks
// @io_throttle_high: I/O size to start throttle in 512b blocks
// @io_throttle_low: I/O size to stop throttle in 512b blocks
// @num_io_throttle_group: Maximum number of throttle groups
// @throttle_groups: Pointer to throttle group info structures
// @sas_transport_enabled: SAS transport enabled or not
// @scsi_device_channel: Channel ID for SCSI devices
// @transport_cmds: Command tracker for SAS transport commands
// @sas_hba: SAS node for the controller
// @sas_expander_list: SAS node list of expanders
// @sas_node_lock: Lock to protect SAS node list
// @hba_port_table_list: List of HBA Ports
// @enclosure_list: List of Enclosure objects
// @diag_buffers: Host diagnostic buffers
// @driver_pg2:  Driver page 2 pointer
// @reply_trigger_present: Reply trigger present flag
// @event_trigger_present: Event trigger present flag
// @scsisense_trigger_present: Scsi sense trigger present flag
// @ioctl_dma_pool: DMA pool for IOCTL data buffers
// @ioctl_sge: DMA buffer descriptors for IOCTL data
// @ioctl_chain_sge: DMA buffer descriptor for IOCTL chain
// @ioctl_resp_sge: DMA buffer descriptor for Mgmt cmd response
// @ioctl_sges_allocated: Flag for IOCTL SGEs allocated or not
// @trace_release_trigger_active: Trace trigger active flag
// @fw_release_trigger_active: Fw release trigger active flag
// @snapdump_trigger_active: Snapdump trigger active flag
// @pci_err_recovery: PCI error recovery in progress
// @block_on_pci_err: Block IO during PCI error recovery
// @reply_qfull_count: Occurences of reply queue full avoidance kicking-in
// @prevent_reply_qfull: Enable reply queue prevention
// @seg_tb_support: Segmented trace buffer support
// @num_tb_segs: Number of Segments in Trace buffer
// @trace_buf_pool: DMA pool for Segmented trace buffer segments
// @trace_buf: Trace buffer segments memory descriptor
// @invalid_io_comp: Invalid IO completion
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_ioc {
    pub list: list_head,
    pub pdev: *mut pci_dev,
    pub shost: *mut Scsi_Host,
    pub id: u8,
    pub cpu_count: c_int,
    pub enable_segqueue: bool,
    pub irqpoll_sleep: u32,
    pub name: [c_char; MPI3MR_NAME_LENGTH],
    pub driver_name: [c_char; MPI3MR_NAME_LENGTH],
    pub sysif_regs: *mut mpi3_sysif_registers __iomem,
    pub sysif_regs_phys: resource_size_t,
    pub bars: c_int,
    pub dma_mask: u64,
    pub msix_count: u16,
    pub intr_enabled: u8,
    pub num_admin_req: u16,
    pub admin_req_q_sz: u32,
    pub admin_req_pi: u16,
    pub admin_req_ci: u16,
    pub admin_req_base: *mut c_void,
    pub admin_req_dma: dma_addr_t,
    pub admin_req_lock: spinlock_t,
    pub num_admin_replies: u16,
    pub admin_reply_q_sz: u32,
    pub admin_reply_ci: u16,
    pub admin_reply_ephase: u8,
    pub admin_reply_base: *mut c_void,
    pub admin_reply_dma: dma_addr_t,
    pub admin_reply_q_in_use: core::sync::atomic::AtomicI32,
    pub admin_pend_isr: core::sync::atomic::AtomicI32,
    pub ready_timeout: u32,
    pub intr_info: *mut mpi3mr_intr_info,
    pub intr_info_count: u16,
    pub is_intr_info_set: bool,
    pub num_queues: u16,
    pub num_op_req_q: u16,
    pub req_qinfo: *mut op_req_qinfo,
    pub num_op_reply_q: u16,
    pub op_reply_qinfo: *mut op_reply_qinfo,
    pub init_cmds: mpi3mr_drv_cmd,
    pub cfg_cmds: mpi3mr_drv_cmd,
    pub facts: mpi3mr_ioc_facts,
    pub op_reply_desc_sz: u16,
    pub num_reply_bufs: u32,
    pub reply_buf_pool: *mut dma_pool,
    pub reply_buf: *mut u8,
    pub reply_buf_dma: dma_addr_t,
    pub reply_buf_dma_max_address: dma_addr_t,
    pub reply_free_qsz: u16,
    pub reply_sz: u16,
    pub reply_free_q_pool: *mut dma_pool,
    pub reply_free_q: *mut __le64,
    pub reply_free_q_dma: dma_addr_t,
    pub reply_free_queue_lock: spinlock_t,
    pub reply_free_queue_host_index: u32,
    pub num_sense_bufs: u32,
    pub sense_buf_pool: *mut dma_pool,
    pub sense_buf: *mut u8,
    pub sense_buf_dma: dma_addr_t,
    pub sense_buf_q_sz: u16,
    pub sense_buf_q_pool: *mut dma_pool,
    pub sense_buf_q: *mut __le64,
    pub sense_buf_q_dma: dma_addr_t,
    pub sbq_lock: spinlock_t,
    pub sbq_host_index: u32,
    pub event_masks: [u32; MPI3_EVENT_NOTIFY_EVENTMASK_WORDS],
    pub fwevt_worker_thread: *mut workqueue_struct,
    pub fwevt_lock: spinlock_t,
    pub fwevt_list: list_head,
    pub watchdog_work_q: *mut workqueue_struct,
    pub watchdog_work: delayed_work,
    pub watchdog_lock: spinlock_t,
    pub is_driver_loading: u8,
    pub scan_started: u8,
    pub scan_failed: u16,
    pub stop_drv_processing: u8,
    pub device_refresh_on: u8,
    pub max_host_ios: u16,
    pub tgtdev_lock: spinlock_t,
    pub tgtdev_list: list_head,
    pub max_sgl_entries: u16,
    pub chain_buf_count: u32,
    pub chain_buf_pool: *mut dma_pool,
    pub chain_sgl_list: *mut chain_element,
    pub chain_bitmap: *mut c_ulong,
    pub chain_buf_lock: spinlock_t,
    pub bsg_cmds: mpi3mr_drv_cmd,
    pub host_tm_cmds: mpi3mr_drv_cmd,
    pub dev_rmhs_cmds: [mpi3mr_drv_cmd; MPI3MR_NUM_DEVRMCMD],
    pub evtack_cmds: [mpi3mr_drv_cmd; MPI3MR_NUM_EVTACKCMD],
    pub devrem_bitmap: *mut c_ulong,
    pub dev_handle_bitmap_bits: u16,
    pub removepend_bitmap: *mut c_ulong,
    pub delayed_rmhs_list: list_head,
    pub evtack_cmds_bitmap: *mut c_ulong,
    pub delayed_evtack_cmds_list: list_head,
    pub ts_update_counter: u16,
    pub ts_update_interval: u16,
    pub reset_in_progress: u8,
    pub unrecoverable: u8,
    pub io_admin_reset_sync: u8,
    pub prev_reset_result: c_int,
    pub reset_mutex: mutex,
    pub reset_waitq: wait_queue_head_t,
    pub prepare_for_reset: u8,
    pub prepare_for_reset_timeout_counter: u16,
    pub prp_list_virt: *mut c_void,
    pub prp_list_dma: dma_addr_t,
    pub prp_sz: u32,
    pub diagsave_timeout: u16,
    pub logging_level: c_int,
    pub flush_io_count: u16,
    pub current_event: *mut mpi3mr_fwevt,
    pub driver_info: mpi3_driver_info_layout,
    pub change_count: u16,
    pub pel_enabled: u8,
    pub pel_abort_requested: u8,
    pub pel_class: u8,
    pub pel_locale: u16,
    pub pel_cmds: mpi3mr_drv_cmd,
    pub pel_abort_cmd: mpi3mr_drv_cmd,
    pub pel_newest_seqnum: u32,
    pub pel_seqnum_virt: *mut c_void,
    pub pel_seqnum_dma: dma_addr_t,
    pub pel_seqnum_sz: u32,
    pub op_reply_q_offset: u16,
    pub default_qcount: u16,
    pub active_poll_qcount: u16,
    pub requested_poll_qcount: u16,
    pub fault_during_init: u8,
    pub saved_fault_code: u32,
    pub saved_fault_info: [u32; 3],
    pub fwfault_counter: u64,
    pub bsg_dev: device,
    pub bsg_queue: *mut request_queue,
    pub stop_bsgs: u8,
    pub logdata_buf: *mut u8,
    pub logdata_buf_idx: u16,
    pub logdata_entry_sz: u16,
    pub adm_req_q_bar_writeq_lock: spinlock_t,
    pub adm_reply_q_bar_writeq_lock: spinlock_t,
    pub pend_large_data_sz: core::sync::atomic::AtomicI32,
    pub io_throttle_data_length: u32,
    pub io_throttle_high: u32,
    pub io_throttle_low: u32,
    pub num_io_throttle_group: u16,
    pub throttle_groups: *mut mpi3mr_throttle_group_info,
    pub sas_transport_enabled: u8,
    pub scsi_device_channel: u8,
    pub transport_cmds: mpi3mr_drv_cmd,
    pub sas_hba: mpi3mr_sas_node,
    pub sas_expander_list: list_head,
    pub sas_node_lock: spinlock_t,
    pub hba_port_table_list: list_head,
    pub enclosure_list: list_head,
    pub ioctl_dma_pool: *mut dma_pool,
    pub ioctl_sge: [dma_memory_desc; MPI3MR_NUM_IOCTL_SGE],
    pub ioctl_chain_sge: dma_memory_desc,
    pub ioctl_resp_sge: dma_memory_desc,
    pub ioctl_sges_allocated: bool,
    pub reply_trigger_present: bool,
    pub event_trigger_present: bool,
    pub scsisense_trigger_present: bool,
    pub diag_buffers: [diag_buffer_desc; MPI3MR_MAX_NUM_HDB],
    pub driver_pg2: *mut mpi3_driver_page2,
    pub trigger_lock: spinlock_t,
    pub snapdump_trigger_active: bool,
    pub trace_release_trigger_active: bool,
    pub fw_release_trigger_active: bool,
    pub pci_err_recovery: bool,
    pub block_on_pci_err: bool,
    pub reply_qfull_count: core::sync::atomic::AtomicI32,
    pub prevent_reply_qfull: bool,
    pub seg_tb_support: bool,
    pub num_tb_segs: u32,
    pub trace_buf_pool: *mut dma_pool,
    pub trace_buf: *mut segments,
    pub invalid_io_comp: u8,
}

//
// struct mpi3mr_fwevt - Firmware event structure.
//
// @list: list head
// @work: Work structure
// @mrioc: Adapter instance reference
// @event_id: MPI3 firmware event ID
// @send_ack: Event acknowledgment required or not
// @process_evt: Bottomhalf processing required or not
// @evt_ctx: Event context to send in Ack
// @event_data_size: size of the event data in bytes
// @pending_at_sml: waiting for device add/remove API to complete
// @discard: discard this event
// @ref_count: kref count
// @event_data: Actual MPI3 event data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3mr_fwevt {
    pub list: list_head,
    pub work: work_struct,
    pub mrioc: *mut mpi3mr_ioc,
    pub event_id: u16,
    pub send_ack: bool,
    pub process_evt: bool,
    pub evt_ctx: u32,
    pub event_data_size: u16,
    pub pending_at_sml: bool,
    pub discard: bool,
    pub ref_count: kref,
    pub __aligned(4): char event_data[],
}

//
// struct delayed_dev_rmhs_node - Delayed device removal node
//
// @list: list head
// @handle: Device handle
// @iou_rc: IO Unit Control Reason Code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delayed_dev_rmhs_node {
    pub list: list_head,
    pub handle: u16,
    pub iou_rc: u8,
}

//
// struct delayed_evt_ack_node - Delayed event ack node
// @list: list head
// @event: MPI3 event ID
// @event_ctx: event context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct delayed_evt_ack_node {
    pub list: list_head,
    pub event: u8,
    pub event_ctx: u32,
}

extern "C" {
    pub fn mpi3mr_setup_resources(mrioc: *mut mpi3mr_ioc) -> c_int;
}
extern "C" {
    pub fn mpi3mr_cleanup_resources(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_init_ioc(mrioc: *mut mpi3mr_ioc) -> c_int;
}
extern "C" {
    pub fn mpi3mr_reinit_ioc(mrioc: *mut mpi3mr_ioc, is_resume: u8) -> c_int;
}
extern "C" {
    pub fn mpi3mr_cleanup_ioc(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_issue_port_enable(mrioc: *mut mpi3mr_ioc, async: u8) -> c_int;
}
extern "C" {
    pub fn mpi3mr_build_zero_len_sge(paddr: *mut c_void);
}
extern "C" {
    pub fn mpi3mr_memset_buffers(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_free_mem(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_start_watchdog(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_stop_watchdog(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_ioc_disable_intr(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_ioc_enable_intr(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_get_iocstate(mrioc: *mut mpi3mr_ioc) -> mpi3mr_iocstate;
}
extern "C" {
    pub fn mpi3mr_wait_for_host_io(mrioc: *mut mpi3mr_ioc, timeout: u32);
}
extern "C" {
    pub fn mpi3mr_cleanup_fwevt_list(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_flush_host_io(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_invalidate_devhandles(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_flush_delayed_cmd_lists(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_check_rh_fault_ioc(mrioc: *mut mpi3mr_ioc, reason_code: u32);
}
extern "C" {
    pub fn mpi3mr_print_fault_info(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_check_rh_fault_ioc(mrioc: *mut mpi3mr_ioc, reason_code: u32);
}
extern "C" {
    pub fn mpi3mr_blk_mq_poll(shost: *mut Scsi_Host, queue_num: c_uint) -> c_int;
}
extern "C" {
    pub fn mpi3mr_bsg_init(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_bsg_exit(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_is_expander_device(device_info: u16) -> u8;
}
extern "C" {
    pub fn mpi3mr_expander_add(mrioc: *mut mpi3mr_ioc, handle: u16) -> c_int;
}
// mrioc, u16 handle);
extern "C" {
    pub fn mpi3mr_sas_host_refresh(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_sas_host_add(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_refresh_sas_ports(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_refresh_expanders(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_add_event_wait_for_device_refresh(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_flush_drv_cmds(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_flush_cmds_for_unrecovered_controller(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_free_enclosure_list(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_process_admin_reply_q(mrioc: *mut mpi3mr_ioc) -> c_int;
}
extern "C" {
    pub fn mpi3mr_alloc_diag_bufs(mrioc: *mut mpi3mr_ioc);
}
extern "C" {
    pub fn mpi3mr_post_diag_bufs(mrioc: *mut mpi3mr_ioc) -> c_int;
}
extern "C" {
    pub fn mpi3mr_release_diag_bufs(mrioc: *mut mpi3mr_ioc, skip_rel_action: u8);
}
extern "C" {
    pub fn mpi3mr_refresh_trigger(mrioc: *mut mpi3mr_ioc, page_type: u8) -> c_int;
}
extern "C" {
    pub fn mpi3mr_event_trigger(mrioc: *mut mpi3mr_ioc, event: u8);
}
extern "C" {
    pub fn mpi3mr_global_trigger(mrioc: *mut mpi3mr_ioc, trigger_data: u64);
}
