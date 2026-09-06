//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/hisi_sas/hisi_sas.h
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
// Copyright (c) 2015 Linaro Ltd.
// Copyright (c) 2015 Hisilicon Limited.
//

pub const HISI_SAS_MAX_PHYS: c_int = 9;
pub const HISI_SAS_MAX_QUEUES: c_int = 32;
pub const HISI_SAS_QUEUE_SLOTS: c_int = 4096;
pub const HISI_SAS_MAX_ITCT_ENTRIES: c_int = 1024;

pub const HISI_SAS_RESETTING_BIT: c_int = 0;
pub const HISI_SAS_REJECT_CMD_BIT: c_int = 1;
pub const HISI_SAS_PM_BIT: c_int = 2;
pub const HISI_SAS_HW_FAULT_BIT: c_int = 3;

pub const HISI_SAS_RESERVED_IPTT: c_int = 96;

pub const HISI_SAS_IOST_ITCT_CACHE_NUM: c_int = 64;
pub const HISI_SAS_IOST_ITCT_CACHE_DW_SZ: c_int = 10;
pub const HISI_SAS_FIFO_DATA_DW_SIZE: c_int = 32;
pub const HISI_SAS_REG_MEM_SIZE: c_int = 4;
pub const HISI_SAS_MAX_CDB_LEN: c_int = 16;
pub const HISI_SAS_BLK_QUEUE_DEPTH: c_int = 64;
pub const BYTE_TO_DW: c_int = 4;
pub const BYTE_TO_DDW: c_int = 8;

pub const HISI_SAS_MAX_SMP_RESP_SZ: c_int = 1028;
pub const HISI_SAS_MAX_STP_RESP_SZ: c_int = 28;
pub const HISI_SAS_SATA_PROTOCOL_NONDATA: c_uint = 0x1;
pub const HISI_SAS_SATA_PROTOCOL_PIO: c_uint = 0x2;
pub const HISI_SAS_SATA_PROTOCOL_DMA: c_uint = 0x4;
pub const HISI_SAS_SATA_PROTOCOL_FPDMA: c_uint = 0x8;
pub const HISI_SAS_SATA_PROTOCOL_ATAPI: c_uint = 0x10;

pub const HISI_SAS_DELAY_FOR_PHY_DISABLE: c_int = 100;
pub const NAME_BUF_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_status {
    HISI_SAS_DEV_INIT,
    HISI_SAS_DEV_NORMAL,
    HISI_SAS_DEV_NCQ_ERR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hisi_sas_dev_type {
    HISI_SAS_DEV_TYPE_STP = 0,
    HISI_SAS_DEV_TYPE_SSP,
    HISI_SAS_DEV_TYPE_SATA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_hw_error {
    pub irq_msk: u32,
    pub msk: u32,
    pub shift: c_int,
    pub msg: *const c_char,
    pub reg: c_int,
    pub sub: *const hisi_sas_hw_error,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_rst {
    pub hisi_hba: *mut hisi_hba,
    pub completion: *mut completion,
    pub work: work_struct,
    pub done: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hisi_sas_bit_err_type {
    HISI_SAS_ERR_SINGLE_BIT_ECC = 0x0,
    HISI_SAS_ERR_MULTI_BIT_ECC = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hisi_sas_phy_event {
    HISI_PHYE_PHY_UP   = 0U,
    HISI_PHYE_LINK_RESET,
    HISI_PHYE_PHY_UP_PM,
    HISI_PHYES_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_debugfs_fifo {
    pub signal_sel: u32,
    pub dump_msk: u32,
    pub dump_mode: u32,
    pub trigger: u32,
    pub trigger_msk: u32,
    pub trigger_mode: u32,
    pub rd_data: [u32; HISI_SAS_FIFO_DATA_DW_SIZE],
}

pub const FRAME_RCVD_BUF: c_int = 32;
pub const SAS_PHY_RESV_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_phy {
    pub works: [work_struct; HISI_PHYES_NUM],
    pub hisi_hba: *mut hisi_hba,
    pub port: *mut hisi_sas_port,
    pub sas_phy: asd_sas_phy,
    pub identify: sas_identify,
    pub reset_completion: *mut completion,
    pub timer: timer_list,
    pub lock: spinlock_t,
    pub /: *mut *mut u64 port_id; / from hw,
    pub frame_rcvd_size: u64,
    pub frame_rcvd: [u8; FRAME_RCVD_BUF],
    pub phy_attached: u8,
    pub in_reset: u8,
    pub reserved: [u8; SAS_PHY_RESV_SIZE],
    pub phy_type: u32,
    pub code_violation_err_count: u32,
    pub minimum_linkrate: sas_linkrate,
    pub maximum_linkrate: sas_linkrate,
    pub enable: c_int,
    pub wait_phyup_cnt: c_int,
    pub down_cnt: core::sync::atomic::AtomicI32,
// Trace FIFO
    pub fifo: hisi_sas_debugfs_fifo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_port {
    pub sas_port: asd_sas_port,
    pub port_attached: u8,
    pub /: *mut *mut u8 id; / from hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_cq {
    pub hisi_hba: *mut hisi_hba,
    pub irq_mask: *const cpumask,
    pub rd_point: c_int,
    pub id: c_int,
    pub irq_no: c_int,
    pub poll_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_dq {
    pub hisi_hba: *mut hisi_hba,
    pub list: list_head,
    pub lock: spinlock_t,
    pub wr_point: c_int,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_device {
    pub hisi_hba: *mut hisi_hba,
    pub sas_device: *mut domain_device,
    pub completion: *mut completion,
    pub dq: *mut hisi_sas_dq,
    pub list: list_head,
    pub dev_type: sas_device_type,
    pub dev_status: dev_status,
    pub device_id: c_int,
    pub sata_idx: c_int,
    pub /: *mut *mut spinlock_t lock; / For protecting slots,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_slot {
    pub entry: list_head,
    pub delivery: list_head,
    pub task: *mut sas_task,
    pub port: *mut hisi_sas_port,
    pub n_elem: u64,
    pub n_elem_dif: u64,
    pub dlvry_queue: c_int,
    pub dlvry_queue_slot: c_int,
    pub cmplt_queue: c_int,
    pub cmplt_queue_slot: c_int,
    pub abort: c_int,
    pub ready: c_int,
    pub device_id: c_int,
    pub cmd_hdr: *mut c_void,
    pub cmd_hdr_dma: dma_addr_t,
    pub internal_abort_timer: timer_list,
    pub is_internal: bool,
    pub tmf: *mut sas_tmf_task,
// Do not reorder/change members after here
    pub buf: *mut c_void,
    pub buf_dma: dma_addr_t,
    pub idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_iost_itct_cache {
    pub data: [u32; HISI_SAS_IOST_ITCT_CACHE_DW_SZ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hisi_sas_debugfs_reg_array_member {
    DEBUGFS_GLOBAL = 0,
    DEBUGFS_AXI,
    DEBUGFS_RAS,
    DEBUGFS_REGS_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hisi_sas_debugfs_cache_type {
    HISI_SAS_ITCT_CACHE,
    HISI_SAS_IOST_CACHE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hisi_sas_debugfs_bist_ffe_cfg {
    FFE_SAS_1_5_GBPS,
    FFE_SAS_3_0_GBPS,
    FFE_SAS_6_0_GBPS,
    FFE_SAS_12_0_GBPS,
    FFE_RESV,
    FFE_SATA_1_5_GBPS,
    FFE_SATA_3_0_GBPS,
    FFE_SATA_6_0_GBPS,
    FFE_CFG_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hisi_sas_debugfs_bist_fixed_code {
    FIXED_CODE,
    FIXED_CODE_1,
    FIXED_CODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_hw {
    pub hisi_hba): *mut *mut int (hw_init)(struct hisi_hba,
    pub hisi_hba): *mut *mut int (fw_info_check)(struct hisi_hba,
    pub hisi_hba): *mut *mut int (interrupt_preinit)(struct hisi_hba,
    pub device): *mut hisi_sas_device,
    pub device): *mut domain_device,
    pub device): *mut *mut *mut hisi_sas_device (alloc_dev)(domain_device,
    pub phy_no): *mut *mut *mut void (sl_notify_ssp)(struct hisi_hba hisi_hba, int,
    pub dq): *mut *mut void (start_delivery)(struct hisi_sas_dq,
    pub slot): *mut hisi_sas_slot,
    pub slot): *mut hisi_sas_slot,
    pub slot): *mut hisi_sas_slot,
    pub slot): *mut hisi_sas_slot,
    pub hisi_hba): *mut *mut void (phys_init)(struct hisi_hba,
    pub phy_no): *mut *mut *mut void (phy_start)(struct hisi_hba hisi_hba, int,
    pub phy_no): *mut *mut *mut void (phy_disable)(struct hisi_hba hisi_hba, int,
    pub phy_no): *mut *mut *mut void (phy_hard_reset)(struct hisi_hba hisi_hba, int,
    pub phy_no): *mut *mut *mut void (get_events)(struct hisi_hba hisi_hba, int,
    pub linkrates): *mut sas_phy_linkrates,
    pub (*phy_get_max_linkrate)(void): *mut sas_linkrate,
    pub dev): *mut hisi_sas_device,
    pub sas_dev): *mut *mut void (free_device)(struct hisi_sas_device,
    pub port_id): *mut *mut *mut int (get_wideport_bitmap)(struct hisi_hba hisi_hba, int,
    pub device): *mut domain_device,
    pub hisi_hba): *mut *mut int (soft_reset)(struct hisi_hba,
    pub hisi_hba): *mut *mut u32 (get_phys_state)(struct hisi_hba,
    pub write_data): *mut u8 reg_index, u8 reg_count, u8,
    pub timeout_ms): int delay_ms, int,
    pub hisi_hba): *mut *mut int (debugfs_snapshot_regs)(struct hisi_hba,
    pub complete_hdr_size: c_int,
    pub sht: *const scsi_host_template,
}

pub const HISI_SAS_MAX_DEBUGFS_DUMP: c_int = 50;
pub const HISI_SAS_DEFAULT_DEBUGFS_DUMP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_debugfs_cq {
    pub cq: *mut hisi_sas_cq,
    pub complete_hdr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_debugfs_dq {
    pub dq: *mut hisi_sas_dq,
    pub hdr: *mut hisi_sas_cmd_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_debugfs_regs {
    pub hisi_hba: *mut hisi_hba,
    pub data: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_debugfs_port {
    pub phy: *mut hisi_sas_phy,
    pub data: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_debugfs_iost {
    pub iost: *mut hisi_sas_iost,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_debugfs_itct {
    pub itct: *mut hisi_sas_itct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_debugfs_iost_cache {
    pub cache: *mut hisi_sas_iost_itct_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_debugfs_itct_cache {
    pub cache: *mut hisi_sas_iost_itct_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_hba {
// This must be the first element, used by SHOST_TO_SAS_HA
    pub p: *mut sas_ha_struct,
    pub platform_dev: *mut platform_device,
    pub pci_dev: *mut pci_dev,
    pub dev: *mut device,
    pub prot_mask: c_int,
    pub regs: *mut void __iomem,
    pub sgpio_regs: *mut void __iomem,
    pub ctrl: *mut regmap,
    pub ctrl_reset_reg: u32,
    pub ctrl_reset_sts_reg: u32,
    pub ctrl_clock_ena_reg: u32,
    pub refclk_frequency_mhz: u32,
    pub sas_addr: [u8; SAS_ADDR_SIZE],
    pub /: *mut *mut *mut int irq_map; / v2 hw,
    pub n_phy: c_int,
    pub lock: spinlock_t,
    pub sem: semaphore,
    pub timer: timer_list,
    pub wq: *mut workqueue_struct,
    pub slot_index_count: c_int,
    pub last_slot_index: c_int,
    pub last_dev_id: c_int,
    pub slot_index_tags: *mut c_ulong,
    pub reject_stp_links_msk: c_ulong,
// SCSI/SAS glue
    pub sha: sas_ha_struct,
    pub shost: *mut Scsi_Host,
    pub cq: [hisi_sas_cq; HISI_SAS_MAX_QUEUES],
    pub dq: [hisi_sas_dq; HISI_SAS_MAX_QUEUES],
    pub phy: [hisi_sas_phy; HISI_SAS_MAX_PHYS],
    pub port: [hisi_sas_port; HISI_SAS_MAX_PHYS],
    pub queue_count: c_int,
    pub devices: [hisi_sas_device; HISI_SAS_MAX_DEVICES],
    pub cmd_hdr: [*mut hisi_sas_cmd_hdr; HISI_SAS_MAX_QUEUES],
    pub cmd_hdr_dma: [dma_addr_t; HISI_SAS_MAX_QUEUES],
    pub complete_hdr: [*mut c_void; HISI_SAS_MAX_QUEUES],
    pub complete_hdr_dma: [dma_addr_t; HISI_SAS_MAX_QUEUES],
    pub initial_fis: *mut hisi_sas_initial_fis,
    pub initial_fis_dma: dma_addr_t,
    pub itct: *mut hisi_sas_itct,
    pub itct_dma: dma_addr_t,
    pub iost: *mut hisi_sas_iost,
    pub iost_dma: dma_addr_t,
    pub breakpoint: *mut hisi_sas_breakpoint,
    pub breakpoint_dma: dma_addr_t,
    pub sata_breakpoint: *mut hisi_sas_breakpoint,
    pub sata_breakpoint_dma: dma_addr_t,
    pub slot_info: *mut hisi_sas_slot,
    pub flags: c_ulong,
    pub /: *const *const *const hisi_sas_hw hw; / Low level hw interface,
    pub sata_dev_bitmap: [c_ulong; BITS_TO_LONGS(HISI_SAS_MAX_DEVICES)],
    pub rst_work: work_struct,
    pub phy_state: u32,
    pub /: *mut *mut u32 intr_coal_ticks; / Time of interrupt coalesce in us,
    pub /: *mut *mut u32 intr_coal_count; / Interrupt count to coalesce,
    pub cq_nvecs: c_int,
// bist
    pub debugfs_bist_linkrate: sas_linkrate,
    pub debugfs_bist_code_mode: c_int,
    pub debugfs_bist_phy_no: c_int,
    pub debugfs_bist_mode: c_int,
    pub debugfs_bist_cnt: u32,
    pub debugfs_bist_enable: c_int,
    pub debugfs_bist_ffe: [u32; HISI_SAS_MAX_PHYS][FFE_CFG_MAX],
    pub debugfs_bist_fixed_code: [u32; FIXED_CODE_MAX],
// debugfs memories
// Put Global AXI and RAS Register into register array
    pub debugfs_regs: [hisi_sas_debugfs_regs; HISI_SAS_MAX_DEBUGFS_DUMP][DEBUGFS_REGS_NUM],
    pub debugfs_port_reg: [hisi_sas_debugfs_port; HISI_SAS_MAX_DEBUGFS_DUMP][HISI_SAS_MAX_PHYS],
    pub debugfs_cq: [hisi_sas_debugfs_cq; HISI_SAS_MAX_DEBUGFS_DUMP][HISI_SAS_MAX_QUEUES],
    pub debugfs_dq: [hisi_sas_debugfs_dq; HISI_SAS_MAX_DEBUGFS_DUMP][HISI_SAS_MAX_QUEUES],
    pub debugfs_iost: [hisi_sas_debugfs_iost; HISI_SAS_MAX_DEBUGFS_DUMP],
    pub debugfs_itct: [hisi_sas_debugfs_itct; HISI_SAS_MAX_DEBUGFS_DUMP],
    pub debugfs_iost_cache: [hisi_sas_debugfs_iost_cache; HISI_SAS_MAX_DEBUGFS_DUMP],
    pub debugfs_itct_cache: [hisi_sas_debugfs_itct_cache; HISI_SAS_MAX_DEBUGFS_DUMP],
    pub debugfs_timestamp: [u64; HISI_SAS_MAX_DEBUGFS_DUMP],
    pub debugfs_dump_index: c_int,
    pub debugfs_dir: *mut dentry,
    pub debugfs_dump_dentry: *mut dentry,
    pub debugfs_bist_dentry: *mut dentry,
    pub debugfs_fifo_dentry: *mut dentry,
    pub iopoll_q_cnt: c_int,
}

// Generic HW DMA host memory structures
// Delivery queue header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_cmd_hdr {
// dw0
    pub dw0: __le32,
// dw1
    pub dw1: __le32,
// dw2
    pub dw2: __le32,
// dw3
    pub transfer_tags: __le32,
// dw4
    pub data_transfer_len: __le32,
// dw5
    pub first_burst_num: __le32,
// dw6
    pub sg_len: __le32,
// dw7
    pub dw7: __le32,
// dw8-9
    pub cmd_table_addr: __le64,
// dw10-11
    pub sts_buffer_addr: __le64,
// dw12-13
    pub prd_table_addr: __le64,
// dw14-15
    pub dif_prd_table_addr: __le64,
}

pub const ITCT_RESV_DDW: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_itct {
    pub qw0: __le64,
    pub sas_addr: __le64,
    pub qw2: __le64,
    pub qw3: __le64,
    pub qw4_15: [__le64; ITCT_RESV_DDW],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_iost {
    pub qw0: __le64,
    pub qw1: __le64,
    pub qw2: __le64,
    pub qw3: __le64,
}

pub const ERROR_RECORD_BUF_DW: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_err_record {
    pub data: [u32; ERROR_RECORD_BUF_DW],
}

pub const FIS_RESV_DW: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_initial_fis {
    pub err_record: hisi_sas_err_record,
    pub fis: dev_to_host_fis,
    pub rsvd: [u32; FIS_RESV_DW],
}

pub const BREAKPOINT_DATA_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_breakpoint {
    pub data: [u8; BREAKPOINT_DATA_SIZE],
}

pub const BREAKPOINT_TAG_NUM: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_sata_breakpoint {
    pub tag: [hisi_sas_breakpoint; BREAKPOINT_TAG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_sge {
    pub addr: __le64,
    pub page_ctrl_0: __le32,
    pub page_ctrl_1: __le32,
    pub data_len: __le32,
    pub data_off: __le32,
}

pub const SMP_CMD_TABLE_SIZE: c_int = 44;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_command_table_smp {
    pub bytes: [u8; SMP_CMD_TABLE_SIZE],
}

pub const DUMMY_BUF_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_command_table_stp {
    pub command_fis: host_to_dev_fis,
    pub dummy: [u8; DUMMY_BUF_SIZE],
    pub atapi_cdb: [u8; ATAPI_CDB_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_sge_page {
    pub sge: [hisi_sas_sge; HISI_SAS_SGE_PAGE_CNT],
    pub __aligned(16): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_sge_dif_page {
    pub sge: [hisi_sas_sge; HISI_SAS_SGE_DIF_PAGE_CNT],
    pub __aligned(16): },
pub const PROT_BUF_SIZE: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_command_table_ssp {
    pub hdr: ssp_frame_hdr,
    pub task: ssp_command_iu,
    pub prot: [u32; PROT_BUF_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hisi_sas_command_table {
    pub ssp: hisi_sas_command_table_ssp,
    pub smp: hisi_sas_command_table_smp,
    pub stp: hisi_sas_command_table_stp,
    pub __aligned(16): },
pub const IU_BUF_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_status_buffer {
    pub err: hisi_sas_err_record,
    pub iu: [u8; IU_BUF_SIZE],
    pub __aligned(16): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_slot_buf_table {
    pub status_buffer: hisi_sas_status_buffer,
    pub command_header: hisi_sas_command_table,
    pub sge_page: hisi_sas_sge_page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sas_slot_dif_buf_table {
    pub slot_buf: hisi_sas_slot_buf_table,
    pub sge_dif_page: hisi_sas_sge_dif_page,
}

extern "C" {
    pub fn hisi_sas_stop_phys(hisi_hba: *mut hisi_hba);
}
extern "C" {
    pub fn hisi_sas_alloc(hisi_hba: *mut hisi_hba) -> c_int;
}
extern "C" {
    pub fn hisi_sas_free(hisi_hba: *mut hisi_hba);
}
extern "C" {
    pub fn hisi_sas_get_ata_protocol(task: *mut sas_task) -> u8;
}
extern "C" {
    pub fn hisi_sas_get_fw_info(hisi_hba: *mut hisi_hba) -> c_int;
}
extern "C" {
    pub fn hisi_sas_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn hisi_sas_sdev_configure(sdev: *mut scsi_device, lim: *mut queue_limits) -> c_int;
}
extern "C" {
    pub fn hisi_sas_sdev_init(sdev: *mut scsi_device) -> c_int;
}
extern "C" {
    pub fn hisi_sas_scan_finished(shost: *mut Scsi_Host, time: c_ulong) -> c_int;
}
extern "C" {
    pub fn hisi_sas_scan_start(shost: *mut Scsi_Host);
}
extern "C" {
    pub fn hisi_sas_host_reset(shost: *mut Scsi_Host, reset_type: c_int) -> c_int;
}
extern "C" {
    pub fn hisi_sas_phy_bcast(phy: *mut hisi_sas_phy);
}
extern "C" {
    pub fn hisi_sas_init_mem(hisi_hba: *mut hisi_hba);
}
extern "C" {
    pub fn hisi_sas_rst_work_handler(work: *mut work_struct);
}
extern "C" {
    pub fn hisi_sas_sync_rst_work_handler(work: *mut work_struct);
}
extern "C" {
    pub fn hisi_sas_phy_oob_ready(hisi_hba: *mut hisi_hba, phy_no: c_int);
}
extern "C" {
    pub fn hisi_sas_release_tasks(hisi_hba: *mut hisi_hba);
}
extern "C" {
    pub fn hisi_sas_get_prog_phy_linkrate_mask(max: sas_linkrate) -> u8;
}
extern "C" {
    pub fn hisi_sas_sync_cqs(hisi_hba: *mut hisi_hba);
}
extern "C" {
    pub fn hisi_sas_sync_poll_cqs(hisi_hba: *mut hisi_hba);
}
extern "C" {
    pub fn hisi_sas_controller_reset_prepare(hisi_hba: *mut hisi_hba);
}
extern "C" {
    pub fn hisi_sas_controller_reset_done(hisi_hba: *mut hisi_hba);
}
