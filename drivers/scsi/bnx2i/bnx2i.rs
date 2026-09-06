//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bnx2i/bnx2i.h
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


// bnx2i.h: QLogic NetXtreme II iSCSI driver.
//
// Copyright (c) 2006 - 2013 Broadcom Corporation
// Copyright (c) 2007, 2008 Red Hat, Inc.  All rights reserved.
// Copyright (c) 2007, 2008 Mike Christie
// Copyright (c) 2014, QLogic Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Written by: Anil Veerabhadrappa (anilgv@broadcom.com)
// Previously Maintained by: Eddie Wai (eddie.wai@broadcom.com)
// Maintained by: QLogic-Storage-Upstream@qlogic.com
//

pub const BNX2I_MAX_ADAPTERS: c_int = 8;
pub const ISCSI_MAX_CONNS_PER_HBA: c_int = 128;

pub const ISCSI_MAX_CMDS_PER_SESS: c_int = 128;
// Total active commands across all connections supported by devices

pub const ISCSI_MAX_BDS_PER_CMD: c_int = 32;
pub const MAX_PAGES_PER_CTRL_STRUCT_POOL: c_int = 8;
pub const BNX2I_RESERVED_SLOW_PATH_CMD_SLOTS: c_int = 4;
pub const BNX2X_DB_SHIFT: c_int = 3;
// 5706/08 hardware has limit on maximum buffer size per BD it can handle
pub const MAX_BD_LENGTH: c_int = 65535;
pub const BD_SPLIT_SIZE: c_int = 32768;
// min, max & default values for SQ/RQ/CQ size, configurable via' modparam
pub const BNX2I_SQ_WQES_MIN: c_int = 16;
pub const BNX2I_570X_SQ_WQES_MAX: c_int = 128;
pub const BNX2I_5770X_SQ_WQES_MAX: c_int = 512;
pub const BNX2I_570X_SQ_WQES_DEFAULT: c_int = 128;
pub const BNX2I_5770X_SQ_WQES_DEFAULT: c_int = 128;
pub const BNX2I_570X_CQ_WQES_MAX: c_int = 128;
pub const BNX2I_5770X_CQ_WQES_MAX: c_int = 512;
pub const BNX2I_RQ_WQES_MIN: c_int = 16;
pub const BNX2I_RQ_WQES_MAX: c_int = 32;
pub const BNX2I_RQ_WQES_DEFAULT: c_int = 16;
// CCELLs per conn
pub const BNX2I_CCELLS_MIN: c_int = 16;
pub const BNX2I_CCELLS_MAX: c_int = 96;
pub const BNX2I_CCELLS_DEFAULT: c_int = 64;
pub const ITT_INVALID_SIGNATURE: c_uint = 0xFFFF;
pub const ISCSI_CMD_CLEANUP_TIMEOUT: c_int = 100;
pub const BNX2I_CONN_CTX_BUF_SIZE: c_int = 16384;
pub const BNX2I_SQ_WQE_SIZE: c_int = 64;
pub const BNX2I_RQ_WQE_SIZE: c_int = 256;
pub const BNX2I_CQE_SIZE: c_int = 64;
pub const MB_KERNEL_CTX_SHIFT: c_int = 8;

pub const CTX_SHIFT: c_int = 7;

pub const CTX_OFFSET: c_uint = 0x10000;
pub const MAX_CID_CNT: c_uint = 0x4000;
pub const BNX2I_570X_PAGE_SIZE_DEFAULT: c_int = 4096;
// 5709 context registers
pub const BNX2_MQ_CONFIG2: c_uint = 0x00003d00;

// 57710's BAR2 is mapped to doorbell registers
pub const BNX2X_DOORBELL_PCI_BAR: c_int = 2;
pub const BNX2X_MAX_CQS: c_int = 8;
pub const CNIC_ARM_CQE: c_int = 1;
pub const CNIC_ARM_CQE_FP: c_int = 2;
pub const CNIC_DISARM_CQE: c_int = 0;

// out = cpu_to_le64(val);			\

//
// struct generic_pdu_resc - login pdu resource structure
//
// @req_buf:            driver buffer used to stage payload associated with
// the login request
// @req_dma_addr:       dma address for iscsi login request payload buffer
// @req_buf_size:       actual login request payload length
// @req_wr_ptr:         pointer into login request buffer when next data is
// to be written
// @resp_hdr:           iscsi header where iscsi login response header is to
// be recreated
// @resp_buf:           buffer to stage login response payload
// @resp_dma_addr:      login response payload buffer dma address
// @resp_buf_size:      login response paylod length
// @resp_wr_ptr:        pointer into login response buffer when next data is
// to be written
// @req_bd_tbl:         iscsi login request payload BD table
// @req_bd_dma:         login request BD table dma address
// @resp_bd_tbl:        iscsi login response payload BD table
// @resp_bd_dma:        login request BD table dma address
//
// following structure defines buffer info for generic pdus such as iSCSI Login,
// Logout and NOP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_pdu_resc {
    pub req_buf: *mut c_char,
    pub req_dma_addr: dma_addr_t,
    pub req_buf_size: u32,
    pub req_wr_ptr: *mut c_char,
    pub resp_hdr: iscsi_hdr,
    pub resp_buf: *mut c_char,
    pub resp_dma_addr: dma_addr_t,
    pub resp_buf_size: u32,
    pub resp_wr_ptr: *mut c_char,
    pub req_bd_tbl: *mut c_char,
    pub req_bd_dma: dma_addr_t,
    pub resp_bd_tbl: *mut c_char,
    pub resp_bd_dma: dma_addr_t,
}

//
// struct bd_resc_page - tracks DMA'able memory allocated for BD tables
//
// @link:               list head to link elements
// @max_ptrs:           maximun pointers that can be stored in this page
// @num_valid:          number of pointer valid in this page
// @page:               base addess for page pointer array
//
// structure to track DMA'able memory allocated for command BD tables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bd_resc_page {
    pub link: list_head,
    pub max_ptrs: u32,
    pub num_valid: u32,
    pub page: [*mut c_void; 1],
}

//
// struct io_bdt - I/O buffer destricptor table
//
// @bd_tbl:             BD table's virtual address
// @bd_tbl_dma:         BD table's dma address
// @bd_valid:           num valid BD entries
//
// IO BD table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_bdt {
    pub bd_tbl: *mut iscsi_bd,
    pub bd_tbl_dma: dma_addr_t,
    pub bd_valid: u16,
}

//
// bnx2i_cmd - iscsi command structure
//
// @hdr:                iSCSI header
// @conn:               iscsi_conn pointer
// @scsi_cmd:           SCSI-ML task pointer corresponding to this iscsi cmd
// @sg:                 SG list
// @io_tbl:             buffer descriptor (BD) table
// @bd_tbl_dma:         buffer descriptor (BD) table's dma address
// @req:                bnx2i specific command request struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_cmd {
    pub hdr: iscsi_hdr,
    pub conn: *mut bnx2i_conn,
    pub scsi_cmd: *mut scsi_cmnd,
    pub sg: *mut scatterlist,
    pub io_tbl: io_bdt,
    pub bd_tbl_dma: dma_addr_t,
    pub req: bnx2i_cmd_request,
}

//
// struct bnx2i_conn - iscsi connection structure
//
// @cls_conn:              pointer to iscsi cls conn
// @hba:                   adapter structure pointer
// @iscsi_conn_cid:        iscsi conn id
// @fw_cid:                firmware iscsi context id
// @ep:                    endpoint structure pointer
// @gen_pdu:               login/nopout/logout pdu resources
// @violation_notified:    bit mask used to track iscsi error/warning messages
// already printed out
// @work_cnt:              keeps track of the number of outstanding work
//
// iSCSI connection structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_conn {
    pub cls_conn: *mut iscsi_cls_conn,
    pub hba: *mut bnx2i_hba,
    pub cmd_cleanup_cmpl: completion,
    pub iscsi_conn_cid: u32,
pub const BNX2I_CID_RESERVED: c_uint = 0x5AFF;
    pub fw_cid: u32,
    pub poll_timer: timer_list,
//
// Queue Pair (QP) related structure elements.
//
    pub ep: *mut bnx2i_endpoint,
//
// Buffer for login negotiation process
//
    pub gen_pdu: generic_pdu_resc,
    pub violation_notified: u64,
    pub work_cnt: core::sync::atomic::AtomicI32,
}

//
// struct iscsi_cid_queue - Per adapter iscsi cid queue
//
// @cid_que_base:           queue base memory
// @cid_que:                queue memory pointer
// @cid_q_prod_idx:         produce index
// @cid_q_cons_idx:         consumer index
// @cid_q_max_idx:          max index. used to detect wrap around condition
// @cid_free_cnt:           queue size
// @conn_cid_tbl:           iscsi cid to conn structure mapping table
//
// Per adapter iSCSI CID Queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_cid_queue {
    pub cid_que_base: *mut c_void,
    pub cid_que: *mut u32,
    pub cid_q_prod_idx: u32,
    pub cid_q_cons_idx: u32,
    pub cid_q_max_idx: u32,
    pub cid_free_cnt: u32,
    pub conn_cid_tbl: *mut bnx2i_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_stats_info {
    pub rx_pdus: u64,
    pub rx_bytes: u64,
    pub tx_pdus: u64,
    pub tx_bytes: u64,
}

//
// struct bnx2i_hba - bnx2i adapter structure
//
// @link:                  list head to link elements
// @cnic:                  pointer to cnic device
// @pcidev:                pointer to pci dev
// @netdev:                pointer to netdev structure
// @regview:               mapped PCI register space
// @age:                   age, incremented by every recovery
// @cnic_dev_type:         cnic device type, 5706/5708/5709/57710
// @mail_queue_access:     mailbox queue access mode, applicable to 5709 only
// @reg_with_cnic:         indicates whether the device is register with CNIC
// @adapter_state:         adapter state, UP, GOING_DOWN, LINK_DOWN
// @mtu_supported:         Ethernet MTU supported
// @shost:                 scsi host pointer
// @max_sqes:              SQ size
// @max_rqes:              RQ size
// @max_cqes:              CQ size
// @num_ccell:             number of command cells per connection
// @ofld_conns_active:     active connection list
// @eh_wait:               wait queue for the endpoint to shutdown
// @max_active_conns:      max offload connections supported by this device
// @cid_que:               iscsi cid queue
// @ep_rdwr_lock:          read / write lock to synchronize various ep lists
// @ep_ofld_list:          connection list for pending offload completion
// @ep_active_list:        connection list for active offload endpoints
// @ep_destroy_list:       connection list for pending offload completion
// @mp_bd_tbl:             BD table to be used with middle path requests
// @mp_bd_dma:             DMA address of 'mp_bd_tbl' memory buffer
// @dummy_buffer:          Dummy buffer to be used with zero length scsicmd reqs
// @dummy_buf_dma:         DMA address of 'dummy_buffer' memory buffer
// @lock:              	   lock to synchonize access to hba structure
// @hba_shutdown_tmo:      Timeout value to shutdown each connection
// @conn_teardown_tmo:     Timeout value to tear down each connection
// @conn_ctx_destroy_tmo:  Timeout value to destroy context of each connection
// @pci_did:               PCI device ID
// @pci_vid:               PCI vendor ID
// @pci_sdid:              PCI subsystem device ID
// @pci_svid:              PCI subsystem vendor ID
// @pci_func:              PCI function number in system pci tree
// @pci_devno:             PCI device number in system pci tree
// @num_wqe_sent:          statistic counter, total wqe's sent
// @num_cqe_rcvd:          statistic counter, total cqe's received
// @num_intr_claimed:      statistic counter, total interrupts claimed
// @link_changed_count:    statistic counter, num of link change notifications
// received
// @ipaddr_changed_count:  statistic counter, num times IP address changed while
// at least one connection is offloaded
// @num_sess_opened:       statistic counter, total num sessions opened
// @num_conn_opened:       statistic counter, total num conns opened on this hba
// @ctx_ccell_tasks:       captures number of ccells and tasks supported by
// currently offloaded connection, used to decode
// context memory
// @stat_lock:		   spin lock used by the statistic collector (32 bit)
// @stats:		   local iSCSI statistic collection place holder
//
// Adapter Data Structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_hba {
    pub link: list_head,
    pub cnic: *mut cnic_dev,
    pub pcidev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub regview: *mut void __iomem,
    pub reg_base: resource_size_t,
    pub age: u32,
    pub cnic_dev_type: c_ulong,
pub const BNX2I_NX2_DEV_5706: c_uint = 0x0;
pub const BNX2I_NX2_DEV_5708: c_uint = 0x1;
pub const BNX2I_NX2_DEV_5709: c_uint = 0x2;
pub const BNX2I_NX2_DEV_57710: c_uint = 0x3;
    pub mail_queue_access: u32,
pub const BNX2I_MQ_KERNEL_MODE: c_uint = 0x0;
pub const BNX2I_MQ_KERNEL_BYPASS_MODE: c_uint = 0x1;
pub const BNX2I_MQ_BIN_MODE: c_uint = 0x2;
    pub reg_with_cnic: c_ulong,
pub const BNX2I_CNIC_REGISTERED: c_int = 1;
    pub adapter_state: c_ulong,
pub const ADAPTER_STATE_UP: c_int = 0;
pub const ADAPTER_STATE_GOING_DOWN: c_int = 1;
pub const ADAPTER_STATE_LINK_DOWN: c_int = 2;
pub const ADAPTER_STATE_INIT_FAILED: c_int = 31;
    pub mtu_supported: c_uint,
pub const BNX2I_MAX_MTU_SUPPORTED: c_int = 9000;
    pub shost: *mut Scsi_Host,
    pub max_sqes: u32,
    pub max_rqes: u32,
    pub max_cqes: u32,
    pub num_ccell: u32,
    pub ofld_conns_active: c_int,
    pub eh_wait: wait_queue_head_t,
    pub max_active_conns: c_int,
    pub cid_que: iscsi_cid_queue,
    pub ep_rdwr_lock: rwlock_t,
    pub ep_ofld_list: list_head,
    pub ep_active_list: list_head,
    pub ep_destroy_list: list_head,
//
// BD table to be used with MP (Middle Path requests.
//
    pub mp_bd_tbl: *mut c_char,
    pub mp_bd_dma: dma_addr_t,
    pub dummy_buffer: *mut c_char,
    pub dummy_buf_dma: dma_addr_t,
    pub /: *mut *mut spinlock_t lock; / protects hba structure access,
    pub /: *mut *mut mutex net_dev_lock;/ sync net device access,
    pub hba_shutdown_tmo: c_int,
    pub conn_teardown_tmo: c_int,
    pub conn_ctx_destroy_tmo: c_int,
//
// PCI related info.
//
    pub pci_did: u16,
    pub pci_vid: u16,
    pub pci_sdid: u16,
    pub pci_svid: u16,
    pub pci_func: u16,
    pub pci_devno: u16,
//
// Following are a bunch of statistics useful during development
// and later stage for score boarding.
//
    pub num_wqe_sent: u32,
    pub num_cqe_rcvd: u32,
    pub num_intr_claimed: u32,
    pub link_changed_count: u32,
    pub ipaddr_changed_count: u32,
    pub num_sess_opened: u32,
    pub num_conn_opened: u32,
    pub ctx_ccell_tasks: c_uint,

    pub stat_lock: spinlock_t,

    pub bnx2i_stats: bnx2i_stats_info,
    pub stats: iscsi_stats_info,
}

//
// QP [ SQ / RQ / CQ ] info.
//
// SQ/RQ/CQ generic structure definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sqe {
    pub sqe_byte: [u8; BNX2I_SQ_WQE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rqe {
    pub rqe_byte: [u8; BNX2I_RQ_WQE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqe {
    pub cqe_byte: [u8; BNX2I_CQE_SIZE],
}

//
// CQ DB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_iscsi_cq_pend_cmpl {
// CQ producer, updated by Ustorm
    pub ustrom_prod: u16,
// CQ pending completion counter
    pub pend_cntr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_5771x_cq_db {
    pub qp_pend_cmpl: [bnx2x_iscsi_cq_pend_cmpl; BNX2X_MAX_CQS],
// CQ pending completion ITT array
    pub itt: [u16; BNX2X_MAX_CQS],
// Cstorm CQ sequence to notify array, updated by driver */;
    pub sqn: [u16; BNX2X_MAX_CQS],
    pub /: *mut *mut u32 reserved[4] / 16 byte allignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_5771x_sq_rq_db {
    pub prod_idx: u16,
    pub /: *mut *mut u8 reserved0[62]; / Pad structure size to 64 bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_5771x_dbell_hdr {
    pub header: u8,
// 1 for rx doorbell, 0 for tx doorbell

pub const B577XX_DOORBELL_HDR_RX_SHIFT: c_int = 0;
// 0 for normal doorbell, 1 for advertise wnd doorbell

pub const B577XX_DOORBELL_HDR_DB_TYPE_SHIFT: c_int = 1;
// rdma tx only: DPM transaction size specifier (64/128/256/512B)

pub const B577XX_DOORBELL_HDR_DPM_SIZE_SHIFT: c_int = 2;
// connection type

pub const B577XX_DOORBELL_HDR_CONN_TYPE_SHIFT: c_int = 4;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_5771x_dbell {
    pub dbell: bnx2i_5771x_dbell_hdr,
    pub pad: [u8; 3],
}

//
// struct qp_info - QP (share queue region) atrributes structure
//
// @ctx_base:           ioremapped pci register base to access doorbell register
// pertaining to this offloaded connection
// @sq_virt:            virtual address of send queue (SQ) region
// @sq_phys:            DMA address of SQ memory region
// @sq_mem_size:        SQ size
// @sq_prod_qe:         SQ producer entry pointer
// @sq_cons_qe:         SQ consumer entry pointer
// @sq_first_qe:        virtual address of first entry in SQ
// @sq_last_qe:         virtual address of last entry in SQ
// @sq_prod_idx:        SQ producer index
// @sq_cons_idx:        SQ consumer index
// @sqe_left:           number sq entry left
// @sq_pgtbl_virt:      page table describing buffer consituting SQ region
// @sq_pgtbl_phys:      dma address of 'sq_pgtbl_virt'
// @sq_pgtbl_size:      SQ page table size
// @cq_virt:            virtual address of completion queue (CQ) region
// @cq_phys:            DMA address of RQ memory region
// @cq_mem_size:        CQ size
// @cq_prod_qe:         CQ producer entry pointer
// @cq_cons_qe:         CQ consumer entry pointer
// @cq_first_qe:        virtual address of first entry in CQ
// @cq_last_qe:         virtual address of last entry in CQ
// @cq_prod_idx:        CQ producer index
// @cq_cons_idx:        CQ consumer index
// @cqe_left:           number cq entry left
// @cqe_size:           size of each CQ entry
// @cqe_exp_seq_sn:     next expected CQE sequence number
// @cq_pgtbl_virt:      page table describing buffer consituting CQ region
// @cq_pgtbl_phys:      dma address of 'cq_pgtbl_virt'
// @cq_pgtbl_size:    	CQ page table size
// @rq_virt:            virtual address of receive queue (RQ) region
// @rq_phys:            DMA address of RQ memory region
// @rq_mem_size:        RQ size
// @rq_prod_qe:         RQ producer entry pointer
// @rq_cons_qe:         RQ consumer entry pointer
// @rq_first_qe:        virtual address of first entry in RQ
// @rq_last_qe:         virtual address of last entry in RQ
// @rq_prod_idx:        RQ producer index
// @rq_cons_idx:        RQ consumer index
// @rqe_left:           number rq entry left
// @rq_pgtbl_virt:      page table describing buffer consituting RQ region
// @rq_pgtbl_phys:      dma address of 'rq_pgtbl_virt'
// @rq_pgtbl_size:      RQ page table size
//
// queue pair (QP) is a per connection shared data structure which is used
// to send work requests (SQ), receive completion notifications (CQ)
// and receive asynchoronous / scsi sense info (RQ). 'qp_info' structure
// below holds queue memory, consumer/producer indexes and page table
// information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qp_info {
    pub ctx_base: *mut void __iomem,
pub const DPM_TRIGER_TYPE: c_uint = 0x40;
pub const BNX2I_570x_QUE_DB_SIZE: c_int = 0;
pub const BNX2I_5771x_QUE_DB_SIZE: c_int = 16;
    pub sq_virt: *mut sqe,
    pub sq_phys: dma_addr_t,
    pub sq_mem_size: u32,
    pub sq_prod_qe: *mut sqe,
    pub sq_cons_qe: *mut sqe,
    pub sq_first_qe: *mut sqe,
    pub sq_last_qe: *mut sqe,
    pub sq_prod_idx: u16,
    pub sq_cons_idx: u16,
    pub sqe_left: u32,
    pub sq_pgtbl_virt: *mut c_void,
    pub sq_pgtbl_phys: dma_addr_t,
    pub /: *mut *mut u32 sq_pgtbl_size; / set to PAGE_SIZE for 5708 & 5709,
    pub cq_virt: *mut cqe,
    pub cq_phys: dma_addr_t,
    pub cq_mem_size: u32,
    pub cq_prod_qe: *mut cqe,
    pub cq_cons_qe: *mut cqe,
    pub cq_first_qe: *mut cqe,
    pub cq_last_qe: *mut cqe,
    pub cq_prod_idx: u16,
    pub cq_cons_idx: u16,
    pub cqe_left: u32,
    pub cqe_size: u32,
    pub cqe_exp_seq_sn: u32,
    pub cq_pgtbl_virt: *mut c_void,
    pub cq_pgtbl_phys: dma_addr_t,
    pub /: *mut *mut u32 cq_pgtbl_size; / set to PAGE_SIZE for 5708 & 5709,
    pub rq_virt: *mut rqe,
    pub rq_phys: dma_addr_t,
    pub rq_mem_size: u32,
    pub rq_prod_qe: *mut rqe,
    pub rq_cons_qe: *mut rqe,
    pub rq_first_qe: *mut rqe,
    pub rq_last_qe: *mut rqe,
    pub rq_prod_idx: u16,
    pub rq_cons_idx: u16,
    pub rqe_left: u32,
    pub rq_pgtbl_virt: *mut c_void,
    pub rq_pgtbl_phys: dma_addr_t,
    pub /: *mut *mut u32 rq_pgtbl_size; / set to PAGE_SIZE for 5708 & 5709,
}

//
// CID handles
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep_handles {
    pub fw_cid: u32,
    pub drv_iscsi_cid: u32,
    pub pg_cid: u16,
    pub rsvd: u16,
}

//
// struct bnx2i_endpoint - representation of tcp connection in NX2 world
//
// @link:               list head to link elements
// @hba:                adapter to which this connection belongs
// @conn:               iscsi connection this EP is linked to
// @cls_ep:             associated iSCSI endpoint pointer
// @cm_sk:              cnic sock struct
// @hba_age:            age to detect if 'iscsid' issues ep_disconnect()
// after HBA reset is completed by bnx2i/cnic/bnx2
// modules
// @state:              tracks offload connection state machine
// @timestamp:          tracks the start time when the ep begins to connect
// @num_active_cmds:    tracks the number of outstanding commands for this ep
// @ec_shift:           the amount of shift as part of the event coal calc
// @qp:                 QP information
// @ids:                contains chip allocated *context id* & driver assigned
// *iscsi cid
// @ofld_timer:         offload timer to detect timeout
// @ofld_wait:          wait queue
//
// Endpoint Structure - equivalent of tcp socket structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_endpoint {
    pub link: list_head,
    pub hba: *mut bnx2i_hba,
    pub conn: *mut bnx2i_conn,
    pub cls_ep: *mut iscsi_endpoint,
    pub cm_sk: *mut cnic_sock,
    pub hba_age: u32,
    pub state: u32,
    pub timestamp: c_ulong,
    pub num_active_cmds: core::sync::atomic::AtomicI32,
    pub ec_shift: u32,
    pub qp: qp_info,
    pub ids: ep_handles,

    pub ofld_timer: timer_list,
    pub ofld_wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_work {
    pub list: list_head,
    pub session: *mut iscsi_session,
    pub bnx2i_conn: *mut bnx2i_conn,
    pub cqe: cqe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2i_percpu_s {
    pub iothread: *mut task_struct,
    pub work_list: list_head,
    pub p_work_lock: spinlock_t,
}

// Global variables
//
// Function Prototypes
//
extern "C" {
    pub fn bnx2i_identify_device(hba: *mut bnx2i_hba, dev: *mut cnic_dev);
}
extern "C" {
    pub fn bnx2i_ulp_init(dev: *mut cnic_dev);
}
extern "C" {
    pub fn bnx2i_ulp_exit(dev: *mut cnic_dev);
}
extern "C" {
    pub fn bnx2i_start(handle: *mut c_void);
}
extern "C" {
    pub fn bnx2i_stop(handle: *mut c_void);
}
extern "C" {
    pub fn bnx2i_get_stats(handle: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bnx2i_free_hba(hba: *mut bnx2i_hba);
}
extern "C" {
    pub fn bnx2i_get_rq_buf(conn: *mut bnx2i_conn, ptr: *mut c_char, len: c_int);
}
extern "C" {
    pub fn bnx2i_put_rq_buf(conn: *mut bnx2i_conn, count: c_int);
}
extern "C" {
    pub fn bnx2i_iscsi_unmap_sg_list(cmd: *mut bnx2i_cmd);
}
extern "C" {
    pub fn bnx2i_drop_session(session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn bnx2i_send_fw_iscsi_init_msg(hba: *mut bnx2i_hba) -> c_int;
}
extern "C" {
    pub fn bnx2i_update_iscsi_conn(conn: *mut iscsi_conn);
}
extern "C" {
    pub fn bnx2i_ep_ofld_timer(t: *mut timer_list);
}
extern "C" {
    pub fn bnx2i_map_ep_dbell_regs(ep: *mut bnx2i_endpoint) -> c_int;
}
extern "C" {
    pub fn bnx2i_arm_cq_event_coalescing(ep: *mut bnx2i_endpoint, action: u8) -> c_int;
}
extern "C" {
    pub fn bnx2i_hw_ep_disconnect(bnx2i_ep: *mut bnx2i_endpoint) -> c_int;
}
extern "C" {
    pub fn bnx2i_percpu_io_thread(arg: *mut c_void) -> c_int;
}
