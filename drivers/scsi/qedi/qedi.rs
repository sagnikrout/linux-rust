//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedi/qedi.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// QLogic iSCSI Offload Driver
// Copyright (c) 2016 Cavium Inc.
//

//
// PCI function probe defines
//
pub const QEDI_MODE_NORMAL: c_int = 0;
pub const QEDI_MODE_RECOVERY: c_int = 1;
pub const QEDI_MODE_SHUTDOWN: c_int = 2;
pub const ISCSI_WQE_SET_PTU_INVALIDATE: c_int = 1;
pub const QEDI_MAX_ISCSI_TASK: c_int = 4096;
pub const QEDI_MAX_TASK_NUM: c_uint = 0x0FFF;
pub const QEDI_MAX_ISCSI_CONNS_PER_HBA: c_int = 1024;

pub const MAX_OUTSTANDING_TASKS_PER_CON: c_int = 1024;
pub const QEDI_MAX_BD_LEN: c_uint = 0xffff;
pub const QEDI_BD_SPLIT_SZ: c_uint = 0x1000;
pub const QEDI_PAGE_SIZE: c_int = 4096;
pub const QEDI_FAST_SGE_COUNT: c_int = 4;
// MAX Length for cached SGL

pub const QEDI_LOCAL_PORT_MIN: c_int = 60000;
pub const QEDI_LOCAL_PORT_MAX: c_int = 61024;

pub const QEDI_LOCAL_PORT_INVALID: c_uint = 0xffff;
pub const TX_RX_RING: c_int = 16;

pub const QEDI_HW_DMA_BOUNDARY: c_uint = 0xfff;
pub const QEDI_PATH_HANDLE: c_uint = 0xFE0000000UL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qedi_nvm_tgts {
    QEDI_NVM_TGT_PRI,
    QEDI_NVM_TGT_SEC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_nvm_iscsi_image {
    pub iscsi_cfg: nvm_iscsi_cfg,
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_uio_ctrl {
// meta data
    pub uio_hsi_version: u32,
// user writes
    pub host_tx_prod: u32,
    pub host_rx_cons: u32,
    pub host_rx_bd_cons: u32,
    pub host_tx_pkt_len: u32,
    pub host_rx_cons_cnt: u32,
// driver writes
    pub hw_tx_cons: u32,
    pub hw_rx_prod: u32,
    pub hw_rx_bd_prod: u32,
    pub hw_rx_prod_cnt: u32,
// other
    pub mac_addr: [u8; 6],
    pub reserve: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_rx_bd {
    pub rx_pkt_index: u32,
    pub rx_pkt_len: u32,
    pub vlan_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_uio_dev {
    pub qedi_uinfo: uio_info,
    pub uio_dev: u32,
    pub list: list_head,
    pub ll2_ring_size: u32,
    pub ll2_ring: *mut c_void,
    pub ll2_buf_size: u32,
    pub ll2_buf: *mut c_void,
    pub rx_pkt: *mut c_void,
    pub tx_pkt: *mut c_void,
    pub qedi: *mut qedi_ctx,
    pub pdev: *mut pci_dev,
    pub uctrl: *mut c_void,
}

// List to maintain the skb pointers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_work_list {
    pub list: list_head,
    pub skb: *mut sk_buff,
    pub vlan_id: u16,
}

// Queue sizes in number of elements

pub const QEDI_CQ_SIZE: c_int = 2048;

pub const QEDI_PROTO_CQ_PROD_IDX: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_glbl_q_params {
    pub /: *mut *mut u64 hw_p_cq; / Completion queue PBL,
    pub /: *mut *mut u64 hw_p_rq; / Request queue PBL,
    pub /: *mut *mut u64 hw_p_cmdq; / Command queue PBL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct global_queue {
    pub cq: *mut iscsi_cqe,
    pub cq_dma: dma_addr_t,
    pub cq_mem_size: u32,
    pub /: *mut *mut u32 cq_cons_idx; / Completion queue consumer index,
    pub cq_pbl: *mut c_void,
    pub cq_pbl_dma: dma_addr_t,
    pub cq_pbl_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_fastpath {
    pub sb_info: *mut qed_sb_info,
    pub sb_id: u16,
pub const QEDI_NAME_SIZE: c_int = 16;
    pub name: [c_char; QEDI_NAME_SIZE],
    pub qedi: *mut qedi_ctx,
}

// Used to pass fastpath information needed to process CQEs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_io_work {
    pub list: list_head,
    pub cqe: iscsi_cqe_solicited,
    pub que_idx: u16,
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
    pub conn_cid_tbl: *mut qedi_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_portid_tbl {
    pub /: *mut *mut spinlock_t lock; / Port id lock,
    pub start: u16,
    pub max: u16,
    pub next: u16,
    pub table: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_itt_map {
    pub itt: __le32,
    pub p_cmd: *mut qedi_cmd,
}

// I/O tracing entry
pub const QEDI_IO_TRACE_SIZE: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_io_log {
pub const QEDI_IO_TRACE_REQ: c_int = 0;
pub const QEDI_IO_TRACE_RSP: c_int = 1;
    pub direction: u8,
    pub task_id: u16,
    pub cid: u32,
    pub /: *mut *mut u32 port_id; / Remote port fabric ID,
    pub lun: c_int,
    pub /: *mut *mut u8 op; / SCSI CDB,
    pub lba: [u8; 4],
    pub /: *mut *mut unsigned int bufflen; / SCSI buffer length,
    pub /: *mut *mut unsigned int sg_count; / Number of SG elements,
    pub /: *mut *mut u8 fast_sgs; / number of fast sgls,
    pub /: *mut *mut u8 slow_sgs; / number of slow sgls,
    pub /: *mut *mut u8 cached_sgs; / number of cached sgls,
    pub /: *mut *mut int result; / Result passed back to mid-layer,
    pub /: *mut *mut unsigned long jiffies; / Time stamp when I/O logged,
    pub /: *mut *mut int refcount; / Reference count for task id,
    pub by: *mut *mut unsigned int blk_req_cpu; / CPU that the task is queued on,
// blk layer
//
    pub /: *mut *mut unsigned int req_cpu; / CPU that the task is queued on,
    pub /: *mut *mut unsigned int intr_cpu; / Interrupt CPU that the task is received on,
    pub and: *mut *mut unsigned int blk_rsp_cpu;/ CPU that task is actually processed,
// returned to blk layer
//
    pub cached_sge: bool,
    pub slow_sge: bool,
    pub fast_sge: bool,
}

// Number of entries in BDQ
pub const QEDI_BDQ_NUM: c_int = 256;
pub const QEDI_BDQ_BUF_SIZE: c_int = 256;
// DMA coherent buffers for BDQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_bdq_buf {
    pub buf_addr: *mut c_void,
    pub buf_dma: dma_addr_t,
}

// Main port level struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_ctx {
    pub dbg_ctx: qedi_dbg_ctx,
    pub shost: *mut Scsi_Host,
    pub pdev: *mut pci_dev,
    pub cdev: *mut qed_dev,
    pub dev_info: qed_dev_iscsi_info,
    pub int_info: qed_int_info,
    pub p_cpuq: *mut qedi_glbl_q_params,
    pub global_queues: *mut global_queue,
// uio declaration
    pub udev: *mut qedi_uio_dev,
    pub ll2_skb_list: list_head,
    pub /: *mut *mut spinlock_t ll2_lock; / Light L2 lock,
    pub /: *mut *mut spinlock_t hba_lock; / per port lock,
    pub ll2_recv_thread: *mut task_struct,
    pub qedi_err_flags: c_ulong,
pub const QEDI_ERR_ATTN_CLR_EN: c_int = 0;
pub const QEDI_ERR_IS_RECOVERABLE: c_int = 2;
pub const QEDI_ERR_OVERRIDE_EN: c_int = 31;
    pub flags: c_ulong,
pub const UIO_DEV_OPENED: c_int = 1;
pub const QEDI_IOTHREAD_WAKE: c_int = 2;
pub const QEDI_IN_RECOVERY: c_int = 5;
pub const QEDI_IN_OFFLINE: c_int = 6;
pub const QEDI_IN_SHUTDOWN: c_int = 7;
pub const QEDI_BLOCK_IO: c_int = 8;
    pub mac: [u8; ETH_ALEN],
    pub src_ip: [u32; 4],
    pub ip_type: u8,
// Physical address of above array
    pub hw_p_cpuq: dma_addr_t,
    pub bdq: [qedi_bdq_buf; QEDI_BDQ_NUM],
    pub bdq_pbl: *mut c_void,
    pub bdq_pbl_dma: dma_addr_t,
    pub bdq_pbl_mem_size: usize,
    pub bdq_pbl_list: *mut c_void,
    pub bdq_pbl_list_dma: dma_addr_t,
    pub bdq_pbl_list_num_entries: u8,
    pub iscsi_image: *mut qedi_nvm_iscsi_image,
    pub nvm_buf_dma: dma_addr_t,
    pub bdq_primary_prod: *mut void __iomem,
    pub bdq_secondary_prod: *mut void __iomem,
    pub bdq_prod_idx: u16,
    pub rq_num_entries: u16,
    pub max_sqes: u32,
    pub num_queues: u8,
    pub max_active_conns: u32,
    pub msix_count: i32,
    pub cid_que: iscsi_cid_queue,
    pub ep_tbl: *mut qedi_endpoint,
    pub lcl_port_tbl: qedi_portid_tbl,
// Rx fast path intr context
    pub sb_array: *mut qed_sb_info,
    pub fp_array: *mut qedi_fastpath,
    pub tasks: qed_iscsi_tid,
pub const QEDI_LINK_DOWN: c_int = 0;
pub const QEDI_LINK_UP: c_int = 1;
    pub link_state: core::sync::atomic::AtomicI32,
pub const QEDI_RESERVE_TASK_ID: c_int = 0;
pub const MAX_ISCSI_TASK_ENTRIES: c_int = 4096;

    pub BITS_PER_LONG]: unsigned long task_idx_map[MAX_ISCSI_TASK_ENTRIES /,
    pub itt_map: *mut qedi_itt_map,
    pub tid_reuse_count: [u16; QEDI_MAX_ISCSI_TASK],
    pub pf_params: qed_pf_params,
    pub tmf_thread: *mut workqueue_struct,
    pub offload_thread: *mut workqueue_struct,
    pub ll2_mtu: u16,
    pub dpc_wq: *mut workqueue_struct,
    pub recovery_work: delayed_work,
    pub board_disable_work: delayed_work,
    pub /: *mut *mut spinlock_t task_idx_lock; / To protect gbl context,
    pub last_tidx_alloc: i32,
    pub last_tidx_clear: i32,
    pub io_trace_buf: [qedi_io_log; QEDI_IO_TRACE_SIZE],
    pub /: *mut *mut spinlock_t io_trace_lock; / prtect trace Log buf,
    pub io_trace_idx: u16,
    pub intr_cpu: c_uint,
    pub cached_sgls: u32,
    pub use_cached_sge: bool,
    pub slow_sgls: u32,
    pub use_slow_sge: bool,
    pub fast_sgls: u32,
    pub use_fast_sge: bool,
    pub num_offloads: core::sync::atomic::AtomicI32,
pub const SYSFS_FLAG_FW_SEL_BOOT: c_int = 2;
pub const IPV6_LEN: c_int = 41;
pub const IPV4_LEN: c_int = 17;
    pub boot_kset: *mut iscsi_boot_kset,
// Used for iscsi statistics
    pub stats_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_work {
    pub list: list_head,
    pub qedi: *mut qedi_ctx,
    pub cqe: iscsi_cqe,
    pub que_idx: u16,
    pub is_solicited: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedi_percpu_s {
    pub iothread: *mut task_struct,
    pub work_list: list_head,
    pub /: *mut *mut spinlock_t p_work_lock; / Per cpu worker lock,
}

