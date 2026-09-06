//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/cnic.h
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


// cnic.h: QLogic CNIC core network driver.
//
// Copyright (c) 2006-2014 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
pub const HC_INDEX_ISCSI_EQ_CONS: c_int = 6;
pub const HC_INDEX_FCOE_EQ_CONS: c_int = 3;
pub const HC_SP_INDEX_ETH_ISCSI_CQ_CONS: c_int = 5;
pub const HC_SP_INDEX_ETH_ISCSI_RX_CQ_CONS: c_int = 1;
pub const KWQ_PAGE_CNT: c_int = 4;
pub const KCQ_PAGE_CNT: c_int = 16;
pub const KWQ_CID: c_int = 24;
pub const KCQ_CID: c_int = 25;
//
// krnlq_context definition
//
pub const L5_KRNLQ_FLAGS: c_uint = 0x00000000;
pub const L5_KRNLQ_SIZE: c_uint = 0x00000000;
pub const L5_KRNLQ_TYPE: c_uint = 0x00000000;

pub const L5_KRNLQ_HOST_QIDX: c_uint = 0x00000004;
pub const L5_KRNLQ_HOST_FW_QIDX: c_uint = 0x00000008;
pub const L5_KRNLQ_NX_QE_SELF_SEQ: c_uint = 0x0000000c;
pub const L5_KRNLQ_QE_SELF_SEQ_MAX: c_uint = 0x0000000c;
pub const L5_KRNLQ_NX_QE_HADDR_HI: c_uint = 0x00000010;
pub const L5_KRNLQ_NX_QE_HADDR_LO: c_uint = 0x00000014;
pub const L5_KRNLQ_PGTBL_PGIDX: c_uint = 0x00000018;
pub const L5_KRNLQ_NX_PG_QIDX: c_uint = 0x00000018;
pub const L5_KRNLQ_PGTBL_NPAGES: c_uint = 0x0000001c;
pub const L5_KRNLQ_QIDX_INCR: c_uint = 0x0000001c;
pub const L5_KRNLQ_PGTBL_HADDR_HI: c_uint = 0x00000020;
pub const L5_KRNLQ_PGTBL_HADDR_LO: c_uint = 0x00000024;
pub const BNX2_PG_CTX_MAP: c_uint = 0x1a0034;
pub const BNX2_ISCSI_CTX_MAP: c_uint = 0x1a0074;
pub const MAX_COMPLETED_KCQE: c_int = 64;
pub const MAX_CNIC_L5_CONTEXT: c_int = 256;

pub const MAX_ISCSI_TBL_SZ: c_int = 256;
pub const CNIC_LOCAL_PORT_MIN: c_int = 60000;
pub const CNIC_LOCAL_PORT_MAX: c_int = 61024;

pub const DEF_IPID_START: c_uint = 0x8000;
pub const DEF_KA_TIMEOUT: c_int = 10000;
pub const DEF_KA_INTERVAL: c_int = 300000;
pub const DEF_KA_MAX_PROBE_COUNT: c_int = 3;
pub const DEF_TOS: c_int = 0;
pub const DEF_TTL: c_uint = 0xfe;
pub const DEF_SND_SEQ_SCALE: c_int = 0;
pub const DEF_RCV_BUF: c_uint = 0xffff;
pub const DEF_SND_BUF: c_uint = 0xffff;
pub const DEF_SEED: c_int = 0;
pub const DEF_MAX_RT_TIME: c_int = 500;
pub const DEF_MAX_DA_COUNT: c_int = 2;
pub const DEF_SWS_TIMER: c_int = 1000;
pub const DEF_MAX_CWND: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_ctx {
    pub cid: u32,
    pub ctx: *mut c_void,
    pub mapping: dma_addr_t,
}

pub const BNX2_MAX_CID: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_dma {
    pub num_pages: c_int,
    pub pg_arr: *mut c_void,
    pub pg_map_arr: *mut dma_addr_t,
    pub pgtbl_size: c_int,
    pub pgtbl: *mut u32,
    pub pgtbl_map: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_id_tbl {
    pub lock: spinlock_t,
    pub start: u32,
    pub max: u32,
    pub next: u32,
    pub table: *mut c_ulong,
}

pub const CNIC_KWQ16_DATA_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kwqe_16_data {
    pub data: [u8; CNIC_KWQ16_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_iscsi {
    pub task_array_info: cnic_dma,
    pub r2tq_info: cnic_dma,
    pub hq_info: cnic_dma,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_context {
    pub cid: u32,
    pub kwqe_data: *mut kwqe_16_data,
    pub kwqe_data_mapping: dma_addr_t,
    pub waitq: wait_queue_head_t,
    pub wait_cond: c_int,
    pub timestamp: c_ulong,
    pub ctx_flags: c_ulong,
pub const CTX_FL_OFFLD_START: c_int = 0;
pub const CTX_FL_DELETE_WAIT: c_int = 1;
pub const CTX_FL_CID_ERROR: c_int = 2;
    pub ulp_proto_id: u8,
    pub iscsi: *mut cnic_iscsi,
    pub proto: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcq_info {
    pub dma: cnic_dma,
    pub kcq: *mut kcqe,
    pub hw_prod_idx_ptr: *mut u16,
    pub sw_prod_idx: u16,
    pub status_idx_ptr: *mut u16,
    pub io_addr: u32,
    pub (*next_idx)(u16): *mut u16,
    pub (*hw_idx)(u16): *mut u16,
}

pub const UIO_USE_TX_DOORBELL: c_uint = 0x017855DB;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_uio_dev {
    pub cnic_uinfo: uio_info,
    pub uio_dev: u32,
    pub l2_ring_size: c_int,
    pub l2_ring: *mut c_void,
    pub l2_ring_map: dma_addr_t,
    pub l2_buf_size: c_int,
    pub l2_buf: *mut c_void,
    pub l2_buf_map: dma_addr_t,
    pub dev: *mut cnic_dev,
    pub pdev: *mut pci_dev,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_local {
    pub cnic_ulp_lock: spinlock_t,
    pub ulp_handle: [*mut c_void; MAX_CNIC_ULP_TYPE],
    pub ulp_flags: [c_ulong; MAX_CNIC_ULP_TYPE],
pub const ULP_F_INIT: c_int = 0;
pub const ULP_F_START: c_int = 1;
pub const ULP_F_CALL_PENDING: c_int = 2;
    pub ulp_ops: [*mut cnic_ulp_ops __rcu; MAX_CNIC_ULP_TYPE],
    pub cnic_local_flags: c_ulong,
pub const CNIC_LCL_FL_KWQ_INIT: c_uint = 0x0;
pub const CNIC_LCL_FL_L2_WAIT: c_uint = 0x1;
pub const CNIC_LCL_FL_RINGS_INITED: c_uint = 0x2;
pub const CNIC_LCL_FL_STOP_ISCSI: c_uint = 0x4;
    pub dev: *mut cnic_dev,
    pub ethdev: *mut cnic_eth_dev,
    pub udev: *mut cnic_uio_dev,
    pub l2_rx_ring_size: c_int,
    pub l2_single_buf_size: c_int,
    pub rx_cons_ptr: *mut u16,
    pub tx_cons_ptr: *mut u16,
    pub rx_cons: u16,
    pub tx_cons: u16,
    pub kwq_info: cnic_dma,
    pub kwq: *mut kwqe,
    pub kwq_16_data_info: cnic_dma,
    pub max_kwq_idx: u16,
    pub kwq_prod_idx: u16,
    pub kwq_io_addr: u32,
    pub kwq_con_idx_ptr: *mut u16,
    pub kwq_con_idx: u16,
    pub kcq1: kcq_info,
    pub kcq2: kcq_info,
    pub gen: *mut c_void,
    pub bnx2: *mut status_block_msix,
    pub bnx2x_e1x: *mut host_hc_status_block_e1x,
// index values - which counter to update
pub const SM_RX_ID: c_int = 0;
pub const SM_TX_ID: c_int = 1;
    pub status_blk: },
    pub status_blk_map: dma_addr_t,
    pub bnx2x_def_status_blk: *mut host_sp_status_block,
    pub status_blk_num: u32,
    pub bnx2x_igu_sb_id: u32,
    pub int_num: u32,
    pub last_status_idx: u32,
    pub cnic_irq_bh_work: work_struct,
    pub completed_kcq: [*mut kcqe; MAX_COMPLETED_KCQE],
    pub csk_tbl: *mut cnic_sock,
    pub csk_port_tbl: cnic_id_tbl,
    pub gbl_buf_info: cnic_dma,
    pub iscsi_tbl: *mut cnic_iscsi,
    pub ctx_tbl: *mut cnic_context,
    pub cid_tbl: cnic_id_tbl,
    pub iscsi_conn: core::sync::atomic::AtomicI32,
    pub iscsi_start_cid: u32,
    pub fcoe_init_cid: u32,
    pub fcoe_start_cid: u32,
    pub fcoe_cid_tbl: cnic_id_tbl,
    pub max_cid_space: u32,
// per connection parameters
    pub num_iscsi_tasks: c_int,
    pub num_ccells: c_int,
    pub task_array_size: c_int,
    pub r2tq_size: c_int,
    pub hq_size: c_int,
    pub num_cqs: c_int,
    pub delete_task: delayed_work,
    pub ctx_arr: *mut cnic_ctx,
    pub ctx_blks: c_int,
    pub ctx_blk_size: c_int,
    pub ctx_align: c_ulong,
    pub cids_per_blk: c_int,
    pub chip_id: u32,
    pub func: c_int,
    pub shmem_base: u32,
    pub cnic_ops: *mut cnic_ops,
    pub ): *mut *mut int (start_hw)(struct cnic_dev,
    pub ): *mut *mut void (stop_hw)(struct cnic_dev,
    pub ): *mut cnic_dma,
    pub ): *mut *mut int (alloc_resc)(struct cnic_dev,
    pub ): *mut *mut void (free_resc)(struct cnic_dev,
    pub ): *mut *mut int (start_cm)(struct cnic_dev,
    pub ): *mut *mut void (stop_cm)(struct cnic_dev,
    pub ): *mut *mut void (enable_int)(struct cnic_dev,
    pub ): *mut *mut void (disable_int_sync)(struct cnic_dev,
    pub ): *mut *mut void (ack_int)(struct cnic_dev,
    pub index): *mut *mut *mut void (arm_int)(struct cnic_dev , u32,
    pub opcode): *mut *mut *mut void (close_conn)(struct cnic_sock , u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2x_bd_chain_next {
    pub addr_lo: u32,
    pub addr_hi: u32,
    pub reserved: [u8; 8],
}

pub const CDU_REGION_NUMBER_XCM_AG: c_int = 2;
pub const CDU_REGION_NUMBER_UCM_AG: c_int = 4;

pub const BNX2X_CONTEXT_MEM_SIZE: c_int = 1024;
pub const BNX2X_FCOE_CID: c_int = 16;
pub const BNX2X_ISCSI_START_CID: c_int = 18;
pub const BNX2X_ISCSI_NUM_CONNECTIONS: c_int = 128;
pub const BNX2X_ISCSI_TASK_CONTEXT_SIZE: c_int = 128;
pub const BNX2X_ISCSI_MAX_PENDING_R2TS: c_int = 4;
pub const BNX2X_ISCSI_R2TQE_SIZE: c_int = 8;
pub const BNX2X_ISCSI_HQ_BD_SIZE: c_int = 64;
pub const BNX2X_ISCSI_GLB_BUF_SIZE: c_int = 64;
pub const BNX2X_ISCSI_PBL_NOT_CACHED: c_uint = 0xff;
pub const BNX2X_ISCSI_PDU_HEADER_NOT_CACHED: c_uint = 0xff;
pub const BNX2X_FCOE_NUM_CONNECTIONS: c_int = 1024;

pub const BNX2X_SHMEM_MF_BLK_OFFSET: c_uint = 0x7e4;

