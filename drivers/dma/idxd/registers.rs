//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/idxd/registers.h
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
// Copyright(c) 2019 Intel Corporation. All rights rsvd.

// PCI Config
pub const PCI_DEVICE_ID_INTEL_IAA_PTL: c_uint = 0xb02d;
pub const PCI_DEVICE_ID_INTEL_IAA_WCL: c_uint = 0xfd2d;
pub const DEVICE_VERSION_1: c_uint = 0x100;
pub const DEVICE_VERSION_2: c_uint = 0x200;
pub const DEVICE_VERSION_3: c_uint = 0x300;
pub const IDXD_MMIO_BAR: c_int = 0;
pub const IDXD_WQ_BAR: c_int = 2;

// MMIO Device BAR0 Registers
pub const IDXD_VER_OFFSET: c_uint = 0x00;
pub const IDXD_VER_MAJOR_MASK: c_uint = 0xf0;
pub const IDXD_VER_MINOR_MASK: c_uint = 0x0f;

#[repr(C)]
#[derive(Copy, Clone)]
pub union gen_cap_reg {
    pub block_on_fault:1: u64,
    pub overlap_copy:1: u64,
    pub cache_control_mem:1: u64,
    pub cache_control_cache:1: u64,
    pub cmd_cap:1: u64,
    pub rsvd:3: u64,
    pub dest_readback:1: u64,
    pub drain_readback:1: u64,
    pub rsvd2:3: u64,
    pub evl_support:2: u64,
    pub batch_continuation:1: u64,
    pub max_xfer_shift:5: u64,
    pub max_batch_shift:4: u64,
    pub max_ims_mult:6: u64,
    pub config_en:1: u64,
    pub rsvd3:32: u64,
}

pub const IDXD_GENCAP_OFFSET: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub union wq_cap_reg {
    pub total_wq_size:16: u64,
    pub num_wqs:8: u64,
    pub wqcfg_size:4: u64,
    pub rsvd:20: u64,
    pub shared_mode:1: u64,
    pub dedicated_mode:1: u64,
    pub wq_ats_support:1: u64,
    pub priority:1: u64,
    pub occupancy:1: u64,
    pub occupancy_int:1: u64,
    pub op_config:1: u64,
    pub wq_prs_support:1: u64,
    pub rsvd4:8: u64,
}

pub const IDXD_WQCAP_OFFSET: c_uint = 0x20;
pub const IDXD_WQCFG_MIN: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub union group_cap_reg {
    pub num_groups:8: u64,
    pub /: *mut *mut u64 total_rdbufs:8; / formerly total_tokens,
    pub /: *mut *mut u64 rdbuf_ctrl:1; / formerly token_en,
    pub /: *mut *mut u64 rdbuf_limit:1; / formerly token_limit,
    pub /: *mut *mut u64 progress_limit:1; / descriptor and batch descriptor,
    pub rsvd:45: u64,
}

pub const IDXD_GRPCAP_OFFSET: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub union engine_cap_reg {
    pub num_engines:8: u64,
    pub rsvd:56: u64,
}

pub const IDXD_ENGCAP_OFFSET: c_uint = 0x38;
pub const IDXD_OPCAP_NOOP: c_uint = 0x0001;
pub const IDXD_OPCAP_BATCH: c_uint = 0x0002;
pub const IDXD_OPCAP_MEMMOVE: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opcap {
    pub bits: [u64; 4],
}

pub const IDXD_OPCAP_OFFSET: c_uint = 0x40;
pub const IDXD_TABLE_OFFSET: c_uint = 0x60;
#[repr(C)]
#[derive(Copy, Clone)]
pub union offsets_reg {
    pub grpcfg:16: u64,
    pub wqcfg:16: u64,
    pub msix_perm:16: u64,
    pub ims:16: u64,
    pub perfmon:16: u64,
    pub rsvd:48: u64,
}

pub const IDXD_TABLE_MULT: c_uint = 0x100;
pub const IDXD_GENCFG_OFFSET: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub union gencfg_reg {
    pub rdbuf_limit:8: u32,
    pub rsvd:4: u32,
    pub user_int_en:1: u32,
    pub evl_en:1: u32,
    pub rsvd2:18: u32,
}

pub const IDXD_GENCTRL_OFFSET: c_uint = 0x88;
#[repr(C)]
#[derive(Copy, Clone)]
pub union genctrl_reg {
    pub softerr_int_en:1: u32,
    pub halt_int_en:1: u32,
    pub evl_int_en:1: u32,
    pub rsvd:29: u32,
}

pub const IDXD_GENSTATS_OFFSET: c_uint = 0x90;
#[repr(C)]
#[derive(Copy, Clone)]
pub union gensts_reg {
    pub state:2: u32,
    pub reset_type:2: u32,
    pub rsvd:28: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_device_status_state {
    IDXD_DEVICE_STATE_DISABLED = 0,
    IDXD_DEVICE_STATE_ENABLED,
    IDXD_DEVICE_STATE_DRAIN,
    IDXD_DEVICE_STATE_HALT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_device_reset_type {
    IDXD_DEVICE_RESET_SOFTWARE = 0,
    IDXD_DEVICE_RESET_FLR,
    IDXD_DEVICE_RESET_WARM,
    IDXD_DEVICE_RESET_COLD,
}

pub const IDXD_INTCAUSE_OFFSET: c_uint = 0x98;
pub const IDXD_INTC_ERR: c_uint = 0x01;
pub const IDXD_INTC_CMD: c_uint = 0x02;
pub const IDXD_INTC_OCCUPY: c_uint = 0x04;
pub const IDXD_INTC_PERFMON_OVFL: c_uint = 0x08;
pub const IDXD_INTC_HALT_STATE: c_uint = 0x10;
pub const IDXD_INTC_EVL: c_uint = 0x20;
pub const IDXD_INTC_INT_HANDLE_REVOKED: c_uint = 0x80000000;
pub const IDXD_CMD_OFFSET: c_uint = 0xa0;
#[repr(C)]
#[derive(Copy, Clone)]
pub union idxd_command_reg {
    pub operand:20: u32,
    pub cmd:5: u32,
    pub rsvd:6: u32,
    pub int_req:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_cmd {
    IDXD_CMD_ENABLE_DEVICE = 1,
    IDXD_CMD_DISABLE_DEVICE,
    IDXD_CMD_DRAIN_ALL,
    IDXD_CMD_ABORT_ALL,
    IDXD_CMD_RESET_DEVICE,
    IDXD_CMD_ENABLE_WQ,
    IDXD_CMD_DISABLE_WQ,
    IDXD_CMD_DRAIN_WQ,
    IDXD_CMD_ABORT_WQ,
    IDXD_CMD_RESET_WQ,
    IDXD_CMD_DRAIN_PASID,
    IDXD_CMD_ABORT_PASID,
    IDXD_CMD_REQUEST_INT_HANDLE,
    IDXD_CMD_RELEASE_INT_HANDLE,
}

pub const CMD_INT_HANDLE_IMS: c_uint = 0x10000;
pub const IDXD_CMDSTS_OFFSET: c_uint = 0xa8;
#[repr(C)]
#[derive(Copy, Clone)]
pub union cmdsts_reg {
    pub err: u8,
    pub result: u16,
    pub rsvd:7: u8,
    pub active:1: u8,
}

pub const IDXD_CMDSTS_ACTIVE: c_uint = 0x80000000;
pub const IDXD_CMDSTS_ERR_MASK: c_uint = 0xff;
pub const IDXD_CMDSTS_RES_SHIFT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_cmdsts_err {
    IDXD_CMDSTS_SUCCESS = 0,
    IDXD_CMDSTS_INVAL_CMD,
    IDXD_CMDSTS_INVAL_WQIDX,
    IDXD_CMDSTS_HW_ERR,
// enable device errors
    IDXD_CMDSTS_ERR_DEV_ENABLED = 0x10,
    IDXD_CMDSTS_ERR_CONFIG,
    IDXD_CMDSTS_ERR_BUSMASTER_EN,
    IDXD_CMDSTS_ERR_PASID_INVAL,
    IDXD_CMDSTS_ERR_WQ_SIZE_ERANGE,
    IDXD_CMDSTS_ERR_GRP_CONFIG,
    IDXD_CMDSTS_ERR_GRP_CONFIG2,
    IDXD_CMDSTS_ERR_GRP_CONFIG3,
    IDXD_CMDSTS_ERR_GRP_CONFIG4,
// enable wq errors
    IDXD_CMDSTS_ERR_DEV_NOTEN = 0x20,
    IDXD_CMDSTS_ERR_WQ_ENABLED,
    IDXD_CMDSTS_ERR_WQ_SIZE,
    IDXD_CMDSTS_ERR_WQ_PRIOR,
    IDXD_CMDSTS_ERR_WQ_MODE,
    IDXD_CMDSTS_ERR_BOF_EN,
    IDXD_CMDSTS_ERR_PASID_EN,
    IDXD_CMDSTS_ERR_MAX_BATCH_SIZE,
    IDXD_CMDSTS_ERR_MAX_XFER_SIZE,
// disable device errors
    IDXD_CMDSTS_ERR_DIS_DEV_EN = 0x31,
// disable WQ, drain WQ, abort WQ, reset WQ
    IDXD_CMDSTS_ERR_DEV_NOT_EN,
// request interrupt handle
    IDXD_CMDSTS_ERR_INVAL_INT_IDX = 0x41,
    IDXD_CMDSTS_ERR_NO_HANDLE,
}

pub const IDXD_CMDCAP_OFFSET: c_uint = 0xb0;
pub const IDXD_SWERR_OFFSET: c_uint = 0xc0;
pub const IDXD_SWERR_VALID: c_uint = 0x00000001;
pub const IDXD_SWERR_OVERFLOW: c_uint = 0x00000002;

#[repr(C)]
#[derive(Copy, Clone)]
pub union sw_err_reg {
    pub valid:1: u64,
    pub overflow:1: u64,
    pub desc_valid:1: u64,
    pub wq_idx_valid:1: u64,
    pub batch:1: u64,
    pub fault_rw:1: u64,
    pub priv:1: u64,
    pub rsvd:1: u64,
    pub error:8: u64,
    pub wq_idx:8: u64,
    pub rsvd2:8: u64,
    pub operation:8: u64,
    pub pasid:20: u64,
    pub rsvd3:4: u64,
    pub batch_idx:16: u64,
    pub rsvd4:16: u64,
    pub invalid_flags:32: u64,
    pub fault_addr: u64,
    pub rsvd5: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union iaa_cap_reg {
    pub dec_aecs_format_ver:1: u64,
    pub drop_init_bits:1: u64,
    pub chaining:1: u64,
    pub force_array_output_mod:1: u64,
    pub load_part_aecs:1: u64,
    pub comp_early_abort:1: u64,
    pub nested_comp:1: u64,
    pub diction_comp:1: u64,
    pub header_gen:1: u64,
    pub crypto_gcm:1: u64,
    pub crypto_cfb:1: u64,
    pub crypto_xts:1: u64,
    pub rsvd:52: u64,
}

pub const IDXD_IAACAP_OFFSET: c_uint = 0x180;
pub const IDXD_EVLCFG_OFFSET: c_uint = 0xe0;
#[repr(C)]
#[derive(Copy, Clone)]
pub union evlcfg_reg {
    pub pasid_en:1: u64,
    pub priv:1: u64,
    pub rsvd:10: u64,
    pub base_addr:52: u64,
    pub size:16: u64,
    pub pasid:20: u64,
    pub rsvd2:28: u64,
}

pub const IDXD_EVL_SIZE_MIN: c_uint = 0x0040;
pub const IDXD_EVL_SIZE_MAX: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub union msix_perm {
    pub rsvd:2: u32,
    pub ignore:1: u32,
    pub pasid_en:1: u32,
    pub rsvd2:8: u32,
    pub pasid:20: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union group_flags {
    pub tc_a:3: u64,
    pub tc_b:3: u64,
    pub rsvd:1: u64,
    pub use_rdbuf_limit:1: u64,
    pub rdbufs_reserved:8: u64,
    pub rsvd2:4: u64,
    pub rdbufs_allowed:8: u64,
    pub rsvd3:4: u64,
    pub desc_progress_limit:2: u64,
    pub rsvd4:2: u64,
    pub batch_progress_limit:2: u64,
    pub rsvd5:26: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grpcfg {
    pub wqs: [u64; 4],
    pub engines: u64,
    pub flags: group_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union wqcfg {
// bytes 0-3
    pub wq_size: u16,
    pub rsvd: u16,
// bytes 4-7
    pub wq_thresh: u16,
    pub rsvd1: u16,
// bytes 8-11
    pub /: *mut *mut u32 mode:1; / shared or dedicated,
    pub /: *mut *mut u32 bof:1; / block on fault,
    pub wq_ats_disable:1: u32,
    pub wq_prs_disable:1: u32,
    pub priority:4: u32,
    pub pasid:20: u32,
    pub pasid_en:1: u32,
    pub priv:1: u32,
    pub rsvd3:2: u32,
// bytes 12-15
    pub max_xfer_shift:5: u32,
    pub max_batch_shift:4: u32,
    pub max_sgl_shift:4: u32,
    pub rsvd4:19: u32,
// bytes 16-19
    pub occupancy_inth: u16,
    pub occupancy_table_sel:1: u16,
    pub rsvd5:15: u16,
// bytes 20-23
    pub occupancy_limit: u16,
    pub occupancy_int_en:1: u16,
    pub rsvd6:15: u16,
// bytes 24-27
    pub occupancy: u16,
    pub occupancy_int:1: u16,
    pub rsvd7:12: u16,
    pub mode_support:1: u16,
    pub wq_state:2: u16,
// bytes 28-31
    pub rsvd8: u32,
// bytes 32-63
    pub op_config: [u64; 4],
}

pub const WQCFG_PASID_IDX: c_int = 2;
pub const WQCFG_PRIVL_IDX: c_int = 2;
pub const WQCFG_OCCUP_IDX: c_int = 6;
pub const WQCFG_OCCUP_MASK: c_uint = 0xffff;
//
// This macro calculates the offset into the WQCFG register
// idxd - struct idxd
// n - wq id
// ofs - the index of the 32b dword for the config register
//
// The WQCFG register block is divided into groups per each wq. The n index
// allows us to move to the register group that's for that particular wq.
// Each register is 32bits. The ofs gives us the number of register to access.
//

pub const GRPCFG_SIZE: c_int = 64;
pub const GRPWQCFG_STRIDES: c_int = 4;
//
// This macro calculates the offset into the GRPCFG register
// idxd - struct idxd
// n - group id
// ofs - the index of the 64b qword for the config register
//
// The GRPCFG register block is divided into three sub-registers, which
// are GRPWQCFG, GRPENGCFG and GRPFLGCFG. The n index allows us to move
// to the register block that contains the three sub-registers.
// Each register block is 64bits. And the ofs gives us the offset
// within the GRPWQCFG register to access.
//

// Following is performance monitor registers
pub const IDXD_PERFCAP_OFFSET: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub union idxd_perfcap {
    pub num_perf_counter:6: u64,
    pub rsvd1:2: u64,
    pub counter_width:8: u64,
    pub num_event_category:4: u64,
    pub global_event_category:16: u64,
    pub filter:8: u64,
    pub rsvd2:8: u64,
    pub cap_per_counter:1: u64,
    pub writeable_counter:1: u64,
    pub counter_freeze:1: u64,
    pub overflow_interrupt:1: u64,
    pub rsvd3:8: u64,
}

pub const IDXD_EVNTCAP_OFFSET: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub union idxd_evntcap {
    pub events:28: u64,
    pub rsvd:36: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_event {
    pub event_category:4: u32,
    pub events:28: u32,
}

pub const IDXD_CNTRCAP_OFFSET: c_uint = 0x800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_cntrcap {
    pub counter_width:8: u32,
    pub rsvd:20: u32,
    pub num_events:4: u32,
}

pub const IDXD_PERFRST_OFFSET: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub union idxd_perfrst {
    pub perfrst_config:1: u32,
    pub perfrst_counter:1: u32,
    pub rsvd:30: u32,
}

pub const IDXD_OVFSTATUS_OFFSET: c_uint = 0x30;
pub const IDXD_PERFFRZ_OFFSET: c_uint = 0x20;
pub const IDXD_CNTRCFG_OFFSET: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub union idxd_cntrcfg {
    pub enable:1: u64,
    pub interrupt_ovf:1: u64,
    pub global_freeze_ovf:1: u64,
    pub rsvd1:5: u64,
    pub event_category:4: u64,
    pub rsvd2:20: u64,
    pub events:28: u64,
    pub rsvd3:4: u64,
}

pub const IDXD_FLTCFG_OFFSET: c_uint = 0x300;
pub const IDXD_CNTRDATA_OFFSET: c_uint = 0x200;
#[repr(C)]
#[derive(Copy, Clone)]
pub union idxd_cntrdata {
    pub event_count_value: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union event_cfg {
    pub event_cat:4: u64,
    pub event_enc:28: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union filter_cfg {
    pub wq:32: u64,
    pub tc:8: u64,
    pub pg_sz:4: u64,
    pub xfer_sz:8: u64,
    pub eng:8: u64,
}

pub const IDXD_EVLSTATUS_OFFSET: c_uint = 0xf0;
#[repr(C)]
#[derive(Copy, Clone)]
pub union evl_status_reg {
    pub head:16: u32,
    pub rsvd:16: u32,
    pub tail:16: u32,
    pub rsvd2:14: u32,
    pub int_pending:1: u32,
    pub rsvd3:1: u32,
}

pub const IDXD_DSACAP0_OFFSET: c_uint = 0x180;
#[repr(C)]
#[derive(Copy, Clone)]
pub union dsacap0_reg {
    pub bits: u64,
    pub max_sgl_shift:4: u64,
    pub max_gr_block_shift:4: u64,
    pub ops_inter_domain:7: u64,
    pub rsvd1:17: u64,
    pub sgl_formats:16: u64,
    pub max_sg_process:8: u64,
    pub rsvd2:8: u64,
}

pub const IDXD_DSACAP1_OFFSET: c_uint = 0x188;
#[repr(C)]
#[derive(Copy, Clone)]
pub union dsacap1_reg {
    pub bits: u64,
}

pub const IDXD_DSACAP2_OFFSET: c_uint = 0x190;
#[repr(C)]
#[derive(Copy, Clone)]
pub union dsacap2_reg {
    pub bits: u64,
}

pub const IDXD_MAX_BATCH_IDENT: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __evl_entry {
    pub rsvd:2: u64,
    pub desc_valid:1: u64,
    pub wq_idx_valid:1: u64,
    pub batch:1: u64,
    pub fault_rw:1: u64,
    pub priv:1: u64,
    pub err_info_valid:1: u64,
    pub error:8: u64,
    pub wq_idx:8: u64,
    pub batch_id:8: u64,
    pub operation:8: u64,
    pub pasid:20: u64,
    pub rsvd2:4: u64,
    pub batch_idx: u16,
    pub rsvd3: u16,
// Invalid Flags 0x11
    pub invalid_flags: u32,
// Invalid Int Handle 0x19
// Page fault 0x1a
// Page fault 0x06, 0x1f, only operand_id
// Page fault before drain or in batch, 0x26, 0x27
    pub int_handle: u16,
    pub rci:1: u16,
    pub ims:1: u16,
    pub rcr:1: u16,
    pub first_err_in_batch:1: u16,
    pub rsvd4_2:9: u16,
    pub operand_id:3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_evl_entry {
    pub e: __evl_entry,
    pub cr: dsa_completion_record,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iax_evl_entry {
    pub e: __evl_entry,
    pub rsvd: [u64; 4],
    pub cr: iax_completion_record,
}
