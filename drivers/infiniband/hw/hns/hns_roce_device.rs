//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hns/hns_roce_device.h
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
// Copyright (c) 2016 Hisilicon Limited.
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

pub const PCI_REVISION_ID_HIP08: c_uint = 0x21;
pub const PCI_REVISION_ID_HIP09: c_uint = 0x30;
pub const HNS_ROCE_MAX_MSG_LEN: c_uint = 0x80000000;
pub const HNS_ROCE_IB_MIN_SQ_STRIDE: c_int = 6;
pub const BA_BYTE_LEN: c_int = 8;
pub const HNS_ROCE_MIN_CQE_NUM: c_uint = 0x40;
pub const HNS_ROCE_MIN_SRQ_WQE_NUM: c_int = 1;
pub const HNS_ROCE_MAX_IRQ_NUM: c_int = 128;
pub const HNS_ROCE_SGE_IN_WQE: c_int = 2;
pub const HNS_ROCE_SGE_SHIFT: c_int = 4;
pub const EQ_ENABLE: c_int = 1;
pub const EQ_DISABLE: c_int = 0;
pub const HNS_ROCE_CEQ: c_int = 0;
pub const HNS_ROCE_AEQ: c_int = 1;
pub const HNS_ROCE_CEQE_SIZE: c_uint = 0x4;
pub const HNS_ROCE_AEQE_SIZE: c_uint = 0x10;
pub const HNS_ROCE_V3_EQE_SIZE: c_uint = 0x40;
pub const HNS_ROCE_V2_CQE_SIZE: c_int = 32;
pub const HNS_ROCE_V3_CQE_SIZE: c_int = 64;
pub const HNS_ROCE_V2_QPC_SZ: c_int = 256;
pub const HNS_ROCE_V3_QPC_SZ: c_int = 512;
pub const HNS_ROCE_MAX_PORTS: c_int = 6;
pub const HNS_ROCE_GID_SIZE: c_int = 16;
pub const HNS_ROCE_SGE_SIZE: c_int = 16;
pub const HNS_ROCE_DWQE_SIZE: c_int = 65536;
pub const HNS_ROCE_HOP_NUM_0: c_uint = 0xff;
pub const MR_TYPE_MR: c_uint = 0x00;
pub const MR_TYPE_FRMR: c_uint = 0x01;
pub const MR_TYPE_DMA: c_uint = 0x03;
pub const HNS_ROCE_FRMR_MAX_PA: c_int = 512;
pub const HNS_ROCE_FRMR_ALIGN_SIZE: c_int = 128;
pub const PKEY_ID: c_uint = 0xffff;
pub const NODE_DESC_SIZE: c_int = 64;
pub const DB_REG_OFFSET: c_uint = 0x1000;
// Configure to HW for PAGE_SIZE larger than 4KB

pub const ATOMIC_WR_LEN: c_int = 8;
pub const HNS_ROCE_IDX_QUE_ENTRY_SZ: c_int = 4;
pub const SRQ_DB_REG: c_uint = 0x230;
pub const HNS_ROCE_QP_BANK_NUM: c_int = 8;
pub const HNS_ROCE_CQ_BANK_NUM: c_int = 4;
pub const CQ_BANKID_SHIFT: c_int = 2;

pub const VALID_CQ_BANK_MASK_DEFAULT: c_uint = 0xF;
pub const VALID_CQ_BANK_MASK_LIMIT: c_uint = 0x9;
pub const VALID_EXT_SGE_QP_BANK_MASK_LIMIT: c_uint = 0x42;
pub const HNS_ROCE_MAX_CQ_COUNT: c_uint = 0xFFFF;
pub const HNS_ROCE_MAX_CQ_PERIOD: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_event {
    HNS_ROCE_EVENT_TYPE_PATH_MIG                  = 0x01,
    HNS_ROCE_EVENT_TYPE_PATH_MIG_FAILED           = 0x02,
    HNS_ROCE_EVENT_TYPE_COMM_EST                  = 0x03,
    HNS_ROCE_EVENT_TYPE_SQ_DRAINED                = 0x04,
    HNS_ROCE_EVENT_TYPE_WQ_CATAS_ERROR            = 0x05,
    HNS_ROCE_EVENT_TYPE_INV_REQ_LOCAL_WQ_ERROR    = 0x06,
    HNS_ROCE_EVENT_TYPE_LOCAL_WQ_ACCESS_ERROR     = 0x07,
    HNS_ROCE_EVENT_TYPE_SRQ_LIMIT_REACH           = 0x08,
    HNS_ROCE_EVENT_TYPE_SRQ_LAST_WQE_REACH        = 0x09,
    HNS_ROCE_EVENT_TYPE_SRQ_CATAS_ERROR           = 0x0a,
    HNS_ROCE_EVENT_TYPE_CQ_ACCESS_ERROR           = 0x0b,
    HNS_ROCE_EVENT_TYPE_CQ_OVERFLOW               = 0x0c,
    HNS_ROCE_EVENT_TYPE_CQ_ID_INVALID             = 0x0d,
    HNS_ROCE_EVENT_TYPE_PORT_CHANGE               = 0x0f,
// 0x10 and 0x11 is unused in currently application case
    HNS_ROCE_EVENT_TYPE_DB_OVERFLOW               = 0x12,
    HNS_ROCE_EVENT_TYPE_MB                        = 0x13,
    HNS_ROCE_EVENT_TYPE_FLR			      = 0x15,
    HNS_ROCE_EVENT_TYPE_XRCD_VIOLATION	      = 0x16,
    HNS_ROCE_EVENT_TYPE_INVALID_XRCETH	      = 0x17,
}

pub const HNS_ROCE_DB_TYPE_COUNT: c_int = 2;
pub const HNS_ROCE_DB_UNIT_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_reset_stage {
    HNS_ROCE_STATE_NON_RST,
    HNS_ROCE_STATE_RST_BEF_DOWN,
    HNS_ROCE_STATE_RST_DOWN,
    HNS_ROCE_STATE_RST_UNINIT,
    HNS_ROCE_STATE_RST_INIT,
    HNS_ROCE_STATE_RST_INITED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_instance_state {
    HNS_ROCE_STATE_NON_INIT,
    HNS_ROCE_STATE_INIT,
    HNS_ROCE_STATE_INITED,
    HNS_ROCE_STATE_UNINIT,
    HNS_ROCE_STATE_BOND_UNINIT,
}

pub const HNS_ROCE_CMD_SUCCESS: c_int = 1;
pub const HNS_ROCE_MAX_HOP_NUM: c_int = 3;
// The minimum page size is 4K for hardware
pub const HNS_HW_PAGE_SHIFT: c_int = 12;

pub const HNS_HW_MAX_PAGE_SHIFT: c_int = 27;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_uar {
    pub pfn: u64,
    pub index: c_ulong,
    pub logic_idx: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_mmap_type {
    HNS_ROCE_MMAP_TYPE_DB = 1,
    HNS_ROCE_MMAP_TYPE_DWQE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_user_mmap_entry {
    pub rdma_entry: rdma_user_mmap_entry,
    pub mmap_type: hns_roce_mmap_type,
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ucontext {
    pub ibucontext: ib_ucontext,
    pub uar: hns_roce_uar,
    pub page_list: list_head,
    pub page_mutex: mutex,
    pub db_mmap_entry: *mut hns_user_mmap_entry,
    pub config: u32,
    pub cq_bank_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_pd {
    pub ibpd: ib_pd,
    pub pdn: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_xrcd {
    pub ibxrcd: ib_xrcd,
    pub xrcdn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_bitmap {
// Bitmap Traversal last a bit which is 1
    pub last: c_ulong,
    pub top: c_ulong,
    pub max: c_ulong,
    pub reserved_top: c_ulong,
    pub mask: c_ulong,
    pub lock: spinlock_t,
    pub table: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ida {
    pub ida: ida,
    pub /: *mut *mut u32 min; / Lowest ID to allocate.,
    pub /: *mut *mut u32 max; / Highest ID to allocate.,
}

// For Hardware Entry Memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_hem_table {
// HEM type: 0 = qpc, 1 = mtt, 2 = cqc, 3 = srq, 4 = other
    pub type: u32,
// HEM array elment num
    pub num_hem: c_ulong,
// Single obj size
    pub obj_size: c_ulong,
    pub table_chunk_size: c_ulong,
    pub mutex: mutex,
    pub hem: *mut hns_roce_hem,
    pub bt_l1: *mut u64,
    pub bt_l1_dma_addr: *mut dma_addr_t,
    pub bt_l0: *mut u64,
    pub bt_l0_dma_addr: *mut dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_buf_region {
    pub /: *mut *mut u32 offset; / page offset,
    pub /: *mut *mut u32 count; / page count,
    pub /: *mut *mut int hopnum; / addressing hop num,
}

pub const HNS_ROCE_MAX_BT_REGION: c_int = 3;
pub const HNS_ROCE_MAX_BT_LEVEL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_hem_list {
    pub root_bt: list_head,
// link all bt dma mem by hop config
    pub mid_bt: [list_head; HNS_ROCE_MAX_BT_REGION][HNS_ROCE_MAX_BT_LEVEL],
    pub /: *mut *mut list_head btm_bt; / link all bottom bt in @mid_bt,
    pub /: *mut *mut dma_addr_t root_ba; / pointer to the root ba table,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtr_type {
    MTR_DEFAULT = 0,
    MTR_PBL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_buf_attr {
    pub /: *mut *mut size_t size; / region size,
    pub /: *mut *mut int hopnum; / multi-hop addressing hop num,
    pub region: [}; HNS_ROCE_MAX_BT_REGION],
    pub /: *mut *mut unsigned int region_count; / valid region count,
    pub /: *mut *mut unsigned int page_shift; / buffer page shift,
    pub /: *mut *mut unsigned int user_access; / umem access flag,
    pub iova: u64,
    pub type: mtr_type,
    pub /: *mut *mut bool mtt_only; / only alloc buffer-required MTT memory,
    pub /: *mut *mut bool adaptive; / adaptive for page_shift and hopnum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_hem_cfg {
    pub /: *mut *mut dma_addr_t root_ba; / root BA table's address,
    pub /: *mut *mut bool is_direct; / addressing without BA table,
    pub /: *mut *mut unsigned int ba_pg_shift; / BA table page shift,
    pub /: *mut *mut unsigned int buf_pg_shift; / buffer page shift,
    pub /: *mut *mut unsigned int buf_pg_count; / buffer page count,
    pub region: [hns_roce_buf_region; HNS_ROCE_MAX_BT_REGION],
    pub region_count: c_uint,
}

// memory translate region
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_mtr {
    pub /: *mut *mut hns_roce_hem_list hem_list; / multi-hop addressing resource,
    pub /: *mut *mut *mut ib_umem umem; / user space buffer,
    pub /: *mut *mut *mut hns_roce_buf kmem; / kernel space buffer,
    pub /: *mut *mut hns_roce_hem_cfg hem_cfg; / config for hardware addressing,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_mr {
    pub ibmr: ib_mr,
    pub /: *mut *mut u64 iova; / MR's virtual original addr,
    pub /: *mut *mut u64 size; / Address range of MR,
    pub /: *mut *mut u32 key; / Key of MR,
    pub /: *mut *mut u32 pd; / PD num of MR,
    pub /: *mut *mut u32 access; / Access permission of MR,
    pub /: *mut *mut int enabled; / MR's active status,
    pub /: *mut *mut int type; / MR's register type,
    pub /: *mut *mut u32 pbl_hop_num; / multi-hop number,
    pub pbl_mtr: hns_roce_mtr,
    pub npages: u32,
    pub page_list: *mut dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_mr_table {
    pub mtpt_ida: hns_roce_ida,
    pub mtpt_table: hns_roce_hem_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_wq {
    pub /: *mut *mut *mut u64 wrid; / Work request ID,
    pub lock: spinlock_t,
    pub /: *mut *mut u32 wqe_cnt; / WQE num,
    pub max_gs: u32,
    pub rsv_sge: u32,
    pub offset: u32,
    pub /: *mut *mut u32 wqe_shift; / WQE size,
    pub head: u32,
    pub tail: u32,
    pub db_reg: *mut void __iomem,
    pub ext_sge_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_sge {
    pub /: *mut *mut unsigned int sge_cnt; / SGE num,
    pub offset: u32,
    pub /: *mut *mut u32 sge_shift; / SGE size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_buf_list {
    pub buf: *mut c_void,
    pub map: dma_addr_t,
}

//
// %HNS_ROCE_BUF_DIRECT indicates that the all memory must be in a continuous
// dma address range.
//
// %HNS_ROCE_BUF_NOSLEEP indicates that the caller cannot sleep.
//
// %HNS_ROCE_BUF_NOFAIL allocation only failed when allocated size is zero, even
// the allocated size is smaller than the required size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_buf {
    pub trunk_list: *mut hns_roce_buf_list,
    pub ntrunks: u32,
    pub npages: u32,
    pub trunk_shift: c_uint,
    pub page_shift: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_db_pgdir {
    pub list: list_head,
    pub HNS_ROCE_DB_PER_PAGE): DECLARE_BITMAP(order0,,
    pub HNS_ROCE_DB_TYPE_COUNT): DECLARE_BITMAP(order1, HNS_ROCE_DB_PER_PAGE /,
    pub bits: [*mut c_ulong; HNS_ROCE_DB_TYPE_COUNT],
    pub page: *mut u32,
    pub db_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_user_db_page {
    pub list: list_head,
    pub umem: *mut ib_umem,
    pub user_virt: c_ulong,
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_db {
    pub db_record: *mut u32,
    pub pgdir: *mut hns_roce_db_pgdir,
    pub user_page: *mut hns_roce_user_db_page,
    pub u: },
    pub dma: dma_addr_t,
    pub virt_addr: *mut c_void,
    pub index: c_ulong,
    pub order: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cq {
    pub ib_cq: ib_cq,
    pub mtr: hns_roce_mtr,
    pub db: hns_roce_db,
    pub flags: u32,
    pub lock: spinlock_t,
    pub cq_depth: u32,
    pub cons_index: u32,
    pub set_ci_db: *mut u32,
    pub db_reg: *mut void __iomem,
    pub arm_sn: c_int,
    pub cqe_size: c_int,
    pub cqn: c_ulong,
    pub vector: u32,
    pub refcount: refcount_t,
    pub free: completion,
    pub /: *mut *mut list_head sq_list; / all qps on this send cq,
    pub /: *mut *mut list_head rq_list; / all qps on this recv cq,
    pub /: *mut *mut int is_armed; / cq is armed,
    pub /: *mut *mut list_head node; / all armed cqs are on a list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_idx_que {
    pub mtr: hns_roce_mtr,
    pub entry_shift: u32,
    pub bitmap: *mut c_ulong,
    pub head: u32,
    pub tail: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_srq {
    pub ibsrq: ib_srq,
    pub srqn: c_ulong,
    pub wqe_cnt: u32,
    pub max_gs: c_int,
    pub rsv_sge: u32,
    pub wqe_shift: u32,
    pub cqn: u32,
    pub xrcdn: u32,
    pub db_reg: *mut void __iomem,
    pub refcount: refcount_t,
    pub free: completion,
    pub buf_mtr: hns_roce_mtr,
    pub wrid: *mut u64,
    pub idx_que: hns_roce_idx_que,
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub event): *mut *mut *mut void (event)(struct hns_roce_srq srq, enum hns_roce_event,
    pub rdb: hns_roce_db,
    pub cap_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_uar_table {
    pub bitmap: hns_roce_bitmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_bank {
    pub ida: ida,
    pub /: *mut *mut u32 inuse; / Number of IDs allocated,
    pub /: *mut *mut u32 min; / Lowest ID to allocate.,
    pub /: *mut *mut u32 max; / Highest ID to allocate.,
    pub /: *mut *mut u32 next; / Next ID to allocate.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_qp_table {
    pub qp_table: hns_roce_hem_table,
    pub irrl_table: hns_roce_hem_table,
    pub trrl_table: hns_roce_hem_table,
    pub sccc_table: hns_roce_hem_table,
    pub scc_mutex: mutex,
    pub bank: [hns_roce_bank; HNS_ROCE_QP_BANK_NUM],
    pub bank_mutex: mutex,
    pub dip_xa: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cq_table {
    pub array: xarray,
    pub table: hns_roce_hem_table,
    pub bank: [hns_roce_bank; HNS_ROCE_CQ_BANK_NUM],
    pub bank_mutex: mutex,
    pub ctx_num: [u32; HNS_ROCE_CQ_BANK_NUM],
    pub valid_cq_bank_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_srq_table {
    pub srq_ida: hns_roce_ida,
    pub xa: xarray,
    pub table: hns_roce_hem_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_av {
    pub port: u8,
    pub gid_index: u8,
    pub stat_rate: u8,
    pub hop_limit: u8,
    pub flowlabel: u32,
    pub udp_sport: u16,
    pub sl: u8,
    pub tclass: u8,
    pub dgid: [u8; HNS_ROCE_GID_SIZE],
    pub mac: [u8; ETH_ALEN],
    pub vlan_id: u16,
    pub vlan_en: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ah {
    pub ibah: ib_ah,
    pub av: hns_roce_av,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cmd_context {
    pub done: completion,
    pub result: c_int,
    pub next: c_int,
    pub out_param: u64,
    pub token: u16,
    pub busy: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_cmdq_state {
    HNS_ROCE_CMDQ_STATE_NORMAL,
    HNS_ROCE_CMDQ_STATE_FATAL_ERR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cmdq {
    pub pool: *mut dma_pool,
    pub poll_sem: semaphore,
//
// Event mode: cmd register mutex protection,
// ensure to not exceed max_cmds and user use limit region
//
    pub event_sem: semaphore,
    pub max_cmds: c_int,
    pub context_lock: spinlock_t,
    pub free_head: c_int,
    pub context: *mut hns_roce_cmd_context,
//
// Process whether use event mode, init default non-zero
// After the event queue of cmd event ready,
// can switch into event mode
// close device, switch into poll mode(non event mode)
//
    pub use_events: u8,
    pub state: hns_roce_cmdq_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cmd_mailbox {
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_mbox_msg {
    pub in_param: u64,
    pub out_param: u64,
    pub cmd: u8,
    pub tag: u32,
    pub token: u16,
    pub event_en: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_work {
    pub hr_dev: *mut hns_roce_dev,
    pub work: work_struct,
    pub event_type: c_int,
    pub sub_type: c_int,
    pub queue_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_cong_type {
    CONG_TYPE_DCQCN,
    CONG_TYPE_LDCP,
    CONG_TYPE_HC3,
    CONG_TYPE_DIP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_qp {
    pub ibqp: ib_qp,
    pub rq: hns_roce_wq,
    pub rdb: hns_roce_db,
    pub sdb: hns_roce_db,
    pub en_flags: c_ulong,
    pub sq_signal_bits: ib_sig_type,
    pub sq: hns_roce_wq,
    pub mtr: hns_roce_mtr,
    pub buff_size: u32,
    pub mutex: mutex,
    pub port: u8,
    pub phy_port: u8,
    pub sl: u8,
    pub resp_depth: u8,
    pub state: u8,
    pub atomic_rd_en: u32,
    pub qkey: u32,
    pub event_type): hns_roce_event,
    pub qpn: c_ulong,
    pub xrcdn: u32,
    pub refcount: refcount_t,
    pub free: completion,
    pub sge: hns_roce_sge,
    pub next_sge: u32,
    pub path_mtu: ib_mtu,
    pub max_inline_data: u32,
    pub free_mr_en: u8,
// 0: flush needed, 1: unneeded
    pub flush_flag: c_ulong,
    pub flush_work: hns_roce_work,
    pub /: *mut *mut list_head node; / all qps are on a list,
    pub /: *mut *mut list_head rq_node; / all recv qps are on a list,
    pub /: *mut *mut list_head sq_node; / all send qps are on a list,
    pub dwqe_mmap_entry: *mut hns_user_mmap_entry,
    pub config: u32,
    pub cong_type: hns_roce_cong_type,
    pub tc_mode: u8,
    pub priority: u8,
    pub flush_lock: spinlock_t,
    pub dip: *mut hns_roce_dip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ib_iboe {
    pub lock: spinlock_t,
    pub netdevs: [*mut net_device; HNS_ROCE_MAX_PORTS],
    pub nb: notifier_block,
    pub phy_port: [u8; HNS_ROCE_MAX_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_ceqe {
    pub comp: __le32,
    pub rsv: [__le32; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_aeqe {
    pub asyn: __le32,
    pub num: __le32,
    pub rsv0: u32,
    pub rsv1: u32,
    pub queue_event: },
    pub out_param: __le64,
    pub token: __le16,
    pub status: u8,
    pub rsv0: u8,
    pub cmd: } __packed,
    pub event: },
    pub rsv: [__le32; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_eq {
    pub hr_dev: *mut hns_roce_dev,
    pub db_reg: *mut void __iomem,
    pub /: *mut *mut int type_flag; / Aeq:1 ceq:0,
    pub eqn: c_int,
    pub entries: u32,
    pub eqe_size: c_int,
    pub irq: c_int,
    pub cons_index: u32,
    pub over_ignore: c_int,
    pub coalesce: c_int,
    pub arm_st: c_int,
    pub hop_num: c_int,
    pub mtr: hns_roce_mtr,
    pub eq_max_cnt: u16,
    pub eq_period: u32,
    pub shift: c_int,
    pub event_type: c_int,
    pub sub_type: c_int,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_eq_table {
    pub eq: *mut hns_roce_eq,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_scc_algo {
    HNS_ROCE_SCC_ALGO_DCQCN = 0,
    HNS_ROCE_SCC_ALGO_LDCP,
    HNS_ROCE_SCC_ALGO_HC3,
    HNS_ROCE_SCC_ALGO_DIP,
    HNS_ROCE_SCC_ALGO_TOTAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_caps {
    pub fw_ver: u64,
    pub num_ports: u8,
    pub gid_table_len: [c_int; HNS_ROCE_MAX_PORTS],
    pub pkey_table_len: [c_int; HNS_ROCE_MAX_PORTS],
    pub local_ca_ack_delay: c_int,
    pub num_uars: c_int,
    pub phy_num_uars: u32,
    pub max_sq_sg: u32,
    pub max_sq_inline: u32,
    pub max_rq_sg: u32,
    pub rsv0: u32,
    pub num_qps: u32,
    pub reserved_qps: u32,
    pub num_srqs: u32,
    pub max_wqes: u32,
    pub max_srq_wrs: u32,
    pub max_srq_sges: u32,
    pub max_sq_desc_sz: u32,
    pub max_rq_desc_sz: u32,
    pub rsv2: u32,
    pub max_qp_init_rdma: c_int,
    pub max_qp_dest_rdma: c_int,
    pub num_cqs: u32,
    pub max_cqes: u32,
    pub min_cqes: u32,
    pub min_wqes: u32,
    pub reserved_cqs: u32,
    pub reserved_srqs: u32,
    pub num_aeq_vectors: c_int,
    pub num_comp_vectors: c_int,
    pub num_other_vectors: c_int,
    pub num_mtpts: u32,
    pub rsv1: u32,
    pub num_srqwqe_segs: u32,
    pub num_idx_segs: u32,
    pub reserved_mrws: c_int,
    pub reserved_uars: c_int,
    pub num_pds: c_int,
    pub reserved_pds: c_int,
    pub num_xrcds: u32,
    pub reserved_xrcds: u32,
    pub mtt_entry_sz: u32,
    pub cqe_sz: u32,
    pub page_size_cap: u32,
    pub reserved_lkey: u32,
    pub mtpt_entry_sz: c_int,
    pub qpc_sz: c_int,
    pub irrl_entry_sz: c_int,
    pub trrl_entry_sz: c_int,
    pub cqc_entry_sz: c_int,
    pub sccc_sz: c_int,
    pub qpc_timer_entry_sz: c_int,
    pub cqc_timer_entry_sz: c_int,
    pub srqc_entry_sz: c_int,
    pub idx_entry_sz: c_int,
    pub pbl_ba_pg_sz: u32,
    pub pbl_buf_pg_sz: u32,
    pub pbl_hop_num: u32,
    pub aeqe_depth: c_int,
    pub ceqe_depth: c_int,
    pub aeqe_size: u32,
    pub ceqe_size: u32,
    pub max_mtu: ib_mtu,
    pub qpc_bt_num: u32,
    pub qpc_timer_bt_num: u32,
    pub srqc_bt_num: u32,
    pub cqc_bt_num: u32,
    pub cqc_timer_bt_num: u32,
    pub mpt_bt_num: u32,
    pub eqc_bt_num: u32,
    pub smac_bt_num: u32,
    pub sgid_bt_num: u32,
    pub sccc_bt_num: u32,
    pub gmv_bt_num: u32,
    pub qpc_ba_pg_sz: u32,
    pub qpc_buf_pg_sz: u32,
    pub qpc_hop_num: u32,
    pub srqc_ba_pg_sz: u32,
    pub srqc_buf_pg_sz: u32,
    pub srqc_hop_num: u32,
    pub cqc_ba_pg_sz: u32,
    pub cqc_buf_pg_sz: u32,
    pub cqc_hop_num: u32,
    pub mpt_ba_pg_sz: u32,
    pub mpt_buf_pg_sz: u32,
    pub mpt_hop_num: u32,
    pub mtt_ba_pg_sz: u32,
    pub mtt_buf_pg_sz: u32,
    pub mtt_hop_num: u32,
    pub wqe_sq_hop_num: u32,
    pub wqe_sge_hop_num: u32,
    pub wqe_rq_hop_num: u32,
    pub sccc_ba_pg_sz: u32,
    pub sccc_buf_pg_sz: u32,
    pub sccc_hop_num: u32,
    pub qpc_timer_ba_pg_sz: u32,
    pub qpc_timer_buf_pg_sz: u32,
    pub qpc_timer_hop_num: u32,
    pub cqc_timer_ba_pg_sz: u32,
    pub cqc_timer_buf_pg_sz: u32,
    pub cqc_timer_hop_num: u32,
    pub /: *mut *mut *mut u32 cqe_ba_pg_sz; / page_size = 4K(2^cqe_ba_pg_sz),
    pub cqe_buf_pg_sz: u32,
    pub cqe_hop_num: u32,
    pub srqwqe_ba_pg_sz: u32,
    pub srqwqe_buf_pg_sz: u32,
    pub srqwqe_hop_num: u32,
    pub idx_ba_pg_sz: u32,
    pub idx_buf_pg_sz: u32,
    pub idx_hop_num: u32,
    pub eqe_ba_pg_sz: u32,
    pub eqe_buf_pg_sz: u32,
    pub eqe_hop_num: u32,
    pub gmv_entry_num: u32,
    pub gmv_entry_sz: u32,
    pub gmv_ba_pg_sz: u32,
    pub gmv_buf_pg_sz: u32,
    pub gmv_hop_num: u32,
    pub sl_num: u32,
    pub llm_buf_pg_sz: u32,
    pub /: *mut *mut u32 chunk_sz; / chunk size in non multihop mode,
    pub flags: u64,
    pub default_ceq_max_cnt: u16,
    pub default_ceq_period: u16,
    pub default_aeq_max_cnt: u16,
    pub default_aeq_period: u16,
    pub default_aeq_arm_st: u16,
    pub default_ceq_arm_st: u16,
    pub cong_cap: u8,
    pub default_cong_type: hns_roce_cong_type,
    pub max_ack_req_msg_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_device_state {
    HNS_ROCE_DEVICE_STATE_INITED,
    HNS_ROCE_DEVICE_STATE_RST_DOWN,
    HNS_ROCE_DEVICE_STATE_UNINIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_hw_pkt_stat_index {
    HNS_ROCE_HW_RX_RC_PKT_CNT,
    HNS_ROCE_HW_RX_UC_PKT_CNT,
    HNS_ROCE_HW_RX_UD_PKT_CNT,
    HNS_ROCE_HW_RX_XRC_PKT_CNT,
    HNS_ROCE_HW_RX_PKT_CNT,
    HNS_ROCE_HW_RX_ERR_PKT_CNT,
    HNS_ROCE_HW_RX_CNP_PKT_CNT,
    HNS_ROCE_HW_TX_RC_PKT_CNT,
    HNS_ROCE_HW_TX_UC_PKT_CNT,
    HNS_ROCE_HW_TX_UD_PKT_CNT,
    HNS_ROCE_HW_TX_XRC_PKT_CNT,
    HNS_ROCE_HW_TX_PKT_CNT,
    HNS_ROCE_HW_TX_ERR_PKT_CNT,
    HNS_ROCE_HW_TX_CNP_PKT_CNT,
    HNS_ROCE_HW_TRP_GET_MPT_ERR_PKT_CNT,
    HNS_ROCE_HW_TRP_GET_IRRL_ERR_PKT_CNT,
    HNS_ROCE_HW_ECN_DB_CNT,
    HNS_ROCE_HW_RX_BUF_CNT,
    HNS_ROCE_HW_TRP_RX_SOF_CNT,
    HNS_ROCE_HW_CQ_CQE_CNT,
    HNS_ROCE_HW_CQ_POE_CNT,
    HNS_ROCE_HW_CQ_NOTIFY_CNT,
    HNS_ROCE_HW_CNT_TOTAL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_sw_dfx_stat_index {
    HNS_ROCE_DFX_AEQE_CNT,
    HNS_ROCE_DFX_CEQE_CNT,
    HNS_ROCE_DFX_CMDS_CNT,
    HNS_ROCE_DFX_CMDS_ERR_CNT,
    HNS_ROCE_DFX_MBX_POSTED_CNT,
    HNS_ROCE_DFX_MBX_POLLED_CNT,
    HNS_ROCE_DFX_MBX_EVENT_CNT,
    HNS_ROCE_DFX_QP_CREATE_ERR_CNT,
    HNS_ROCE_DFX_QP_MODIFY_ERR_CNT,
    HNS_ROCE_DFX_CQ_CREATE_ERR_CNT,
    HNS_ROCE_DFX_CQ_MODIFY_ERR_CNT,
    HNS_ROCE_DFX_SRQ_CREATE_ERR_CNT,
    HNS_ROCE_DFX_SRQ_MODIFY_ERR_CNT,
    HNS_ROCE_DFX_XRCD_ALLOC_ERR_CNT,
    HNS_ROCE_DFX_MR_REG_ERR_CNT,
    HNS_ROCE_DFX_MR_REREG_ERR_CNT,
    HNS_ROCE_DFX_AH_CREATE_ERR_CNT,
    HNS_ROCE_DFX_MMAP_ERR_CNT,
    HNS_ROCE_DFX_UCTX_ALLOC_ERR_CNT,
    HNS_ROCE_DFX_CNT_TOTAL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_hw {
    pub hr_dev): *mut *mut int (cmq_init)(struct hns_roce_dev,
    pub hr_dev): *mut *mut void (cmq_exit)(struct hns_roce_dev,
    pub hr_dev): *mut *mut int (hw_profile)(struct hns_roce_dev,
    pub hr_dev): *mut *mut int (hw_init)(struct hns_roce_dev,
    pub hr_dev): *mut *mut void (hw_exit)(struct hns_roce_dev,
    pub mbox_msg): *mut hns_roce_mbox_msg,
    pub hr_dev): *mut *mut int (poll_mbox_done)(struct hns_roce_dev,
    pub is_busy): *mut *mut *mut bool (chk_mbox_avail)(struct hns_roce_dev hr_dev, bool,
    pub attr): *const *const ib_gid gid, struct ib_gid_attr,
    pub addr): *const u8,
    pub mr): *mut hns_roce_mr,
    pub mb_buf): *mut c_void,
    pub mr): *mut *mut *mut int (frmr_write_mtpt)(void mb_buf, struct hns_roce_mr,
    pub dma_handle): dma_addr_t,
    pub step_idx): *mut *mut hns_roce_hem_table table, int obj, u32,
    pub step_idx): u32,
    pub udata): *mut ib_qp_state new_state, struct ib_udata,
    pub hr_qp): *mut hns_roce_qp,
    pub hr_dev): *mut *mut void (dereg_mr)(struct hns_roce_dev,
    pub hr_dev): *mut *mut int (init_eq)(struct hns_roce_dev,
    pub hr_dev): *mut *mut void (cleanup_eq)(struct hns_roce_dev,
    pub mb_buf): *mut *mut *mut int (write_srqc)(struct hns_roce_srq srq, void,
    pub buffer): *mut *mut *mut int (query_cqc)(struct hns_roce_dev hr_dev, u32 cqn, void,
    pub buffer): *mut *mut *mut int (query_qpc)(struct hns_roce_dev hr_dev, u32 qpn, void,
    pub buffer): *mut *mut *mut int (query_mpt)(struct hns_roce_dev hr_dev, u32 key, void,
    pub buffer): *mut *mut *mut int (query_srqc)(struct hns_roce_dev hr_dev, u32 srqn, void,
    pub buffer): *mut *mut *mut int (query_sccc)(struct hns_roce_dev hr_dev, u32 qpn, void,
    pub hw_counters): *mut *mut u64 stats, u32 port, int,
    pub priority): *mut *mut u8 tc_mode, u8,
    pub hns_roce_dev_ops: *const ib_device_ops,
    pub hns_roce_dev_srq_ops: *const ib_device_ops,
    pub algo): hns_roce_scc_algo,
    pub alog): hns_roce_scc_algo,
}

pub const HNS_ROCE_SCC_PARAM_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_scc_param {
    pub param: [__le32; HNS_ROCE_SCC_PARAM_SIZE],
    pub algo_type: hns_roce_scc_algo,
    pub hr_dev: *mut hns_roce_dev,
    pub /: *mut *mut mutex scc_mutex; / protect @param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_dev {
    pub ib_dev: ib_device,
    pub pci_dev: *mut pci_dev,
    pub dev: *mut device,
    pub priv_uar: hns_roce_uar,
    pub irq_names: [*const c_char; HNS_ROCE_MAX_IRQ_NUM],
    pub sm_lock: spinlock_t,
    pub active: bool,
    pub is_reset: bool,
    pub dis_db: bool,
    pub reset_cnt: c_ulong,
    pub iboe: hns_roce_ib_iboe,
    pub state: hns_roce_device_state,
    pub /: *mut *mut list_head qp_list; / list of all qps on this dev,
    pub /: *mut *mut spinlock_t qp_list_lock; / protect qp_list,
    pub pgdir_list: list_head,
    pub pgdir_mutex: mutex,
    pub irq: [c_int; HNS_ROCE_MAX_IRQ_NUM],
    pub reg_base: *mut u8 __iomem,
    pub mem_base: *mut void __iomem,
    pub caps: hns_roce_caps,
    pub qp_table_xa: xarray,
    pub dev_addr: [c_uchar; HNS_ROCE_MAX_PORTS][ETH_ALEN],
    pub sys_image_guid: u64,
    pub vendor_id: u32,
    pub vendor_part_id: u32,
    pub hw_rev: u32,
    pub priv_addr: *mut void __iomem,
    pub cmd: hns_roce_cmdq,
    pub pd_ida: hns_roce_ida,
    pub xrcd_ida: hns_roce_ida,
    pub uar_ida: hns_roce_ida,
    pub mr_table: hns_roce_mr_table,
    pub cq_table: hns_roce_cq_table,
    pub srq_table: hns_roce_srq_table,
    pub qp_table: hns_roce_qp_table,
    pub eq_table: hns_roce_eq_table,
    pub qpc_timer_table: hns_roce_hem_table,
    pub cqc_timer_table: hns_roce_hem_table,
// GMV is the memory area that the driver allocates for the hardware
// to store SGID, SMAC and VLAN information.
//
    pub gmv_table: hns_roce_hem_table,
    pub cmd_mod: c_int,
    pub loop_idc: c_int,
    pub sdb_offset: u32,
    pub odb_offset: u32,
    pub hw: *const hns_roce_hw,
    pub priv: *mut c_void,
    pub irq_workq: *mut workqueue_struct,
    pub ecc_work: work_struct,
    pub func_num: u32,
    pub is_vf: u32,
    pub cong_algo_tmpl_id: u32,
    pub dwqe_page: u64,
    pub dbgfs: hns_roce_dev_debugfs,
    pub dfx_cnt: *mut core::sync::atomic::AtomicI64,
    pub scc_param: *mut hns_roce_scc_param,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hns_roce_trace_type {
    TRACE_SQ,
    TRACE_RQ,
    TRACE_SRQ,
}

extern "C" {
    pub fn container_of(_arg: ib_dev, hns_roce_dev: struct, _arg: ib_dev) -> return;
}
// to_hr_ucontext(struct ib_ucontext *ibucontext)
extern "C" {
    pub fn container_of(_arg: ibucontext, hns_roce_ucontext: struct, _arg: ibucontext) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, hns_roce_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibxrcd, hns_roce_xrcd: struct, _arg: ibxrcd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, hns_roce_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibmr, hns_roce_mr: struct, _arg: ibmr) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibqp, hns_roce_qp: struct, _arg: ibqp) -> return;
}
extern "C" {
    pub fn container_of(_arg: ib_cq, hns_roce_cq: struct, _arg: ib_cq) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibsrq, hns_roce_srq: struct, _arg: ibsrq) -> return;
}
extern "C" {
    pub fn container_of(_arg: rdma_entry, hns_user_mmap_entry: struct, _arg: rdma_entry) -> return;
}
// __hns_roce_qp_lookup(struct hns_roce_dev *hr_dev, u32 qpn)
extern "C" {
    pub fn xa_load(_arg: &hr_dev->qp_table_xa, _arg: qpn) -> return;
}
extern "C" {
    pub fn hns_roce_buf_dma_addr(_arg: buf, buf->page_shift: idx <<) -> return;
}

extern "C" {
    pub fn hr_hw_page_align(buf_shift: count <<) -> return;
}
extern "C" {
    pub fn ilog2(_arg: to_hr_hem_entries_count(count, _arg: buf_shift)) -> return;
}
pub const DSCP_SHIFT: c_int = 2;
extern "C" {
    pub fn hns_roce_init_uar_table(dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_uar_alloc(dev: *mut hns_roce_dev, uar: *mut hns_roce_uar) -> c_int;
}
extern "C" {
    pub fn hns_roce_cmd_init(hr_dev: *mut hns_roce_dev) -> c_int;
}
extern "C" {
    pub fn hns_roce_cmd_cleanup(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_cmd_use_events(hr_dev: *mut hns_roce_dev) -> c_int;
}
extern "C" {
    pub fn hns_roce_cmd_use_polling(hr_dev: *mut hns_roce_dev);
}
// hns roce hw need current block and next block addr from mtt
pub const MTT_MIN_COUNT: c_int = 2;
extern "C" {
    pub fn hns_roce_init_pd_table(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_init_mr_table(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_init_cq_table(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_init_qp_table(hr_dev: *mut hns_roce_dev) -> c_int;
}
extern "C" {
    pub fn hns_roce_init_srq_table(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_init_xrcd_table(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_cleanup_cq_table(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_cleanup_qp_table(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_cleanup_bitmap(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_query_ah(ibah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int;
}
extern "C" {
    pub fn hns_roce_alloc_pd(pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn hns_roce_dealloc_pd(pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn hns_roce_dereg_mr(ibmr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn key_to_hw_index(key: u32) -> c_ulong;
}
extern "C" {
    pub fn hns_roce_buf_free(hr_dev: *mut hns_roce_dev, buf: *mut hns_roce_buf);
}
extern "C" {
    pub fn hns_roce_destroy_srq(ibsrq: *mut ib_srq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn hns_roce_alloc_xrcd(ib_xrcd: *mut ib_xrcd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn hns_roce_dealloc_xrcd(ib_xrcd: *mut ib_xrcd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn init_flush_work(hr_dev: *mut hns_roce_dev, hr_qp: *mut hns_roce_qp);
}
extern "C" {
    pub fn hns_roce_qp_remove(hr_dev: *mut hns_roce_dev, hr_qp: *mut hns_roce_qp);
}
extern "C" {
    pub fn send_ieth(wr: *const ib_send_wr) -> __be32;
}
extern "C" {
    pub fn to_hr_qp_type(qp_type: c_int) -> c_int;
}
extern "C" {
    pub fn hns_roce_destroy_cq(ib_cq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn hns_roce_free_db(hr_dev: *mut hns_roce_dev, db: *mut hns_roce_db);
}
extern "C" {
    pub fn hns_roce_cq_completion(hr_dev: *mut hns_roce_dev, cqn: u32);
}
extern "C" {
    pub fn hns_roce_cq_event(hr_dev: *mut hns_roce_dev, cqn: u32, event_type: c_int);
}
extern "C" {
    pub fn flush_cqe(dev: *mut hns_roce_dev, qp: *mut hns_roce_qp);
}
extern "C" {
    pub fn hns_roce_qp_event(hr_dev: *mut hns_roce_dev, qpn: u32, event_type: c_int);
}
extern "C" {
    pub fn hns_roce_flush_cqe(hr_dev: *mut hns_roce_dev, qpn: u32);
}
extern "C" {
    pub fn hns_roce_srq_event(hr_dev: *mut hns_roce_dev, srqn: u32, event_type: c_int);
}
extern "C" {
    pub fn hns_roce_handle_device_err(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_init(hr_dev: *mut hns_roce_dev) -> c_int;
}
extern "C" {
    pub fn hns_roce_exit(hr_dev: *mut hns_roce_dev, bond_cleanup: bool);
}
extern "C" {
    pub fn hns_roce_fill_res_cq_entry(msg: *mut sk_buff, ib_cq: *mut ib_cq) -> c_int;
}
extern "C" {
    pub fn hns_roce_fill_res_cq_entry_raw(msg: *mut sk_buff, ib_cq: *mut ib_cq) -> c_int;
}
extern "C" {
    pub fn hns_roce_fill_res_qp_entry(msg: *mut sk_buff, ib_qp: *mut ib_qp) -> c_int;
}
extern "C" {
    pub fn hns_roce_fill_res_qp_entry_raw(msg: *mut sk_buff, ib_qp: *mut ib_qp) -> c_int;
}
extern "C" {
    pub fn hns_roce_fill_res_mr_entry(msg: *mut sk_buff, ib_mr: *mut ib_mr) -> c_int;
}
extern "C" {
    pub fn hns_roce_fill_res_mr_entry_raw(msg: *mut sk_buff, ib_mr: *mut ib_mr) -> c_int;
}
extern "C" {
    pub fn hns_roce_fill_res_srq_entry(msg: *mut sk_buff, ib_srq: *mut ib_srq) -> c_int;
}
extern "C" {
    pub fn hns_roce_fill_res_srq_entry_raw(msg: *mut sk_buff, ib_srq: *mut ib_srq) -> c_int;
}
extern "C" {
    pub fn check_sl_valid(hr_dev: *mut hns_roce_dev, sl: u8) -> bool;
}
extern "C" {
    pub fn hns_roce_put_cq_bankid_for_uctx(uctx: *mut hns_roce_ucontext);
}
extern "C" {
    pub fn hns_roce_get_cq_bankid_for_uctx(uctx: *mut hns_roce_ucontext);
}
