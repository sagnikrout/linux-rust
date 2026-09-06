//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/idxd.h
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


// SPDX-License-Identifier: LGPL-2.1 WITH Linux-syscall-note
// Copyright(c) 2019 Intel Corporation. All rights rsvd.

// Driver command error status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_scmd_stat {
    IDXD_SCMD_DEV_ENABLED = 0x80000010,
    IDXD_SCMD_DEV_NOT_ENABLED = 0x80000020,
    IDXD_SCMD_WQ_ENABLED = 0x80000021,
    IDXD_SCMD_DEV_DMA_ERR = 0x80020000,
    IDXD_SCMD_WQ_NO_GRP = 0x80030000,
    IDXD_SCMD_WQ_NO_NAME = 0x80040000,
    IDXD_SCMD_WQ_NO_SVM = 0x80050000,
    IDXD_SCMD_WQ_NO_THRESH = 0x80060000,
    IDXD_SCMD_WQ_PORTAL_ERR = 0x80070000,
    IDXD_SCMD_WQ_RES_ALLOC_ERR = 0x80080000,
    IDXD_SCMD_PERCPU_ERR = 0x80090000,
    IDXD_SCMD_DMA_CHAN_ERR = 0x800a0000,
    IDXD_SCMD_CDEV_ERR = 0x800b0000,
    IDXD_SCMD_WQ_NO_SWQ_SUPPORT = 0x800c0000,
    IDXD_SCMD_WQ_NONE_CONFIGURED = 0x800d0000,
    IDXD_SCMD_WQ_NO_SIZE = 0x800e0000,
    IDXD_SCMD_WQ_NO_PRIV = 0x800f0000,
    IDXD_SCMD_WQ_IRQ_ERR = 0x80100000,
    IDXD_SCMD_WQ_USER_NO_IOMMU = 0x80110000,
    IDXD_SCMD_DEV_EVL_ERR = 0x80120000,
    IDXD_SCMD_WQ_NO_DRV_NAME = 0x80200000,
}

pub const IDXD_SCMD_SOFTERR_MASK: c_uint = 0x80000000;
pub const IDXD_SCMD_SOFTERR_SHIFT: c_int = 16;
// Descriptor flags
pub const IDXD_OP_FLAG_FENCE: c_uint = 0x0001;
pub const IDXD_OP_FLAG_BOF: c_uint = 0x0002;
pub const IDXD_OP_FLAG_CRAV: c_uint = 0x0004;
pub const IDXD_OP_FLAG_RCR: c_uint = 0x0008;
pub const IDXD_OP_FLAG_RCI: c_uint = 0x0010;
pub const IDXD_OP_FLAG_CRSTS: c_uint = 0x0020;
pub const IDXD_OP_FLAG_CR: c_uint = 0x0080;
pub const IDXD_OP_FLAG_CC: c_uint = 0x0100;
pub const IDXD_OP_FLAG_ADDR1_TCS: c_uint = 0x0200;
pub const IDXD_OP_FLAG_ADDR2_TCS: c_uint = 0x0400;
pub const IDXD_OP_FLAG_ADDR3_TCS: c_uint = 0x0800;
pub const IDXD_OP_FLAG_CR_TCS: c_uint = 0x1000;
pub const IDXD_OP_FLAG_STORD: c_uint = 0x2000;
pub const IDXD_OP_FLAG_DRDBK: c_uint = 0x4000;
pub const IDXD_OP_FLAG_DSTS: c_uint = 0x8000;
// IAX
pub const IDXD_OP_FLAG_RD_SRC2_AECS: c_uint = 0x010000;
pub const IDXD_OP_FLAG_RD_SRC2_2ND: c_uint = 0x020000;
pub const IDXD_OP_FLAG_WR_SRC2_AECS_COMP: c_uint = 0x040000;
pub const IDXD_OP_FLAG_WR_SRC2_AECS_OVFL: c_uint = 0x080000;
pub const IDXD_OP_FLAG_SRC2_STS: c_uint = 0x100000;
pub const IDXD_OP_FLAG_CRC_RFC3720: c_uint = 0x200000;
// Opcode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsa_opcode {
    DSA_OPCODE_NOOP = 0,
    DSA_OPCODE_BATCH,
    DSA_OPCODE_DRAIN,
    DSA_OPCODE_MEMMOVE,
    DSA_OPCODE_MEMFILL,
    DSA_OPCODE_COMPARE,
    DSA_OPCODE_COMPVAL,
    DSA_OPCODE_CR_DELTA,
    DSA_OPCODE_AP_DELTA,
    DSA_OPCODE_DUALCAST,
    DSA_OPCODE_TRANSL_FETCH,
    DSA_OPCODE_CRCGEN = 0x10,
    DSA_OPCODE_COPY_CRC,
    DSA_OPCODE_DIF_CHECK,
    DSA_OPCODE_DIF_INS,
    DSA_OPCODE_DIF_STRP,
    DSA_OPCODE_DIF_UPDT,
    DSA_OPCODE_DIX_GEN = 0x17,
    DSA_OPCODE_CFLUSH = 0x20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iax_opcode {
    IAX_OPCODE_NOOP = 0,
    IAX_OPCODE_DRAIN = 2,
    IAX_OPCODE_MEMMOVE,
    IAX_OPCODE_DECOMPRESS = 0x42,
    IAX_OPCODE_COMPRESS,
    IAX_OPCODE_CRC64,
    IAX_OPCODE_ZERO_DECOMP_32 = 0x48,
    IAX_OPCODE_ZERO_DECOMP_16,
    IAX_OPCODE_ZERO_COMP_32 = 0x4c,
    IAX_OPCODE_ZERO_COMP_16,
    IAX_OPCODE_SCAN = 0x50,
    IAX_OPCODE_SET_MEMBER,
    IAX_OPCODE_EXTRACT,
    IAX_OPCODE_SELECT,
    IAX_OPCODE_RLE_BURST,
    IAX_OPCODE_FIND_UNIQUE,
    IAX_OPCODE_EXPAND,
}

// Completion record status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsa_completion_status {
    DSA_COMP_NONE = 0,
    DSA_COMP_SUCCESS,
    DSA_COMP_SUCCESS_PRED,
    DSA_COMP_PAGE_FAULT_NOBOF,
    DSA_COMP_PAGE_FAULT_IR,
    DSA_COMP_BATCH_FAIL,
    DSA_COMP_BATCH_PAGE_FAULT,
    DSA_COMP_DR_OFFSET_NOINC,
    DSA_COMP_DR_OFFSET_ERANGE,
    DSA_COMP_DIF_ERR,
    DSA_COMP_BAD_OPCODE = 0x10,
    DSA_COMP_INVALID_FLAGS,
    DSA_COMP_NOZERO_RESERVE,
    DSA_COMP_XFER_ERANGE,
    DSA_COMP_DESC_CNT_ERANGE,
    DSA_COMP_DR_ERANGE,
    DSA_COMP_OVERLAP_BUFFERS,
    DSA_COMP_DCAST_ERR,
    DSA_COMP_DESCLIST_ALIGN,
    DSA_COMP_INT_HANDLE_INVAL,
    DSA_COMP_CRA_XLAT,
    DSA_COMP_CRA_ALIGN,
    DSA_COMP_ADDR_ALIGN,
    DSA_COMP_PRIV_BAD,
    DSA_COMP_TRAFFIC_CLASS_CONF,
    DSA_COMP_PFAULT_RDBA,
    DSA_COMP_HW_ERR1,
    DSA_COMP_HW_ERR_DRB,
    DSA_COMP_TRANSLATION_FAIL,
    DSA_COMP_DRAIN_EVL = 0x26,
    DSA_COMP_BATCH_EVL_ERR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iax_completion_status {
    IAX_COMP_NONE = 0,
    IAX_COMP_SUCCESS,
    IAX_COMP_PAGE_FAULT_IR = 0x04,
    IAX_COMP_ANALYTICS_ERROR = 0x0a,
    IAX_COMP_OUTBUF_OVERFLOW,
    IAX_COMP_BAD_OPCODE = 0x10,
    IAX_COMP_INVALID_FLAGS,
    IAX_COMP_NOZERO_RESERVE,
    IAX_COMP_INVALID_SIZE,
    IAX_COMP_OVERLAP_BUFFERS = 0x16,
    IAX_COMP_INT_HANDLE_INVAL = 0x19,
    IAX_COMP_CRA_XLAT,
    IAX_COMP_CRA_ALIGN,
    IAX_COMP_ADDR_ALIGN,
    IAX_COMP_PRIV_BAD,
    IAX_COMP_TRAFFIC_CLASS_CONF,
    IAX_COMP_PFAULT_RDBA,
    IAX_COMP_HW_ERR1,
    IAX_COMP_HW_ERR_DRB,
    IAX_COMP_TRANSLATION_FAIL,
    IAX_COMP_PRS_TIMEOUT,
    IAX_COMP_WATCHDOG,
    IAX_COMP_INVALID_COMP_FLAG = 0x30,
    IAX_COMP_INVALID_FILTER_FLAG,
    IAX_COMP_INVALID_INPUT_SIZE,
    IAX_COMP_INVALID_NUM_ELEMS,
    IAX_COMP_INVALID_SRC1_WIDTH,
    IAX_COMP_INVALID_INVERT_OUT,
}

pub const DSA_COMP_STATUS_MASK: c_uint = 0x7f;
pub const DSA_COMP_STATUS_WRITE: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_hw_desc {
    pub pasid:20: __u32,
    pub rsvd:11: __u32,
    pub priv:1: __u32,
    pub flags:24: __u32,
    pub opcode:8: __u32,
    pub completion_addr: __u64,
    pub src_addr: __u64,
    pub rdback_addr: __u64,
    pub pattern: __u64,
    pub desc_list_addr: __u64,
    pub pattern_lower: __u64,
    pub transl_fetch_addr: __u64,
}

// create delta record
// CRC
// DIF check or strip
// DIF insert
// DIF update
// Fill
// Translation fetch
// DIX generate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iax_hw_desc {
    pub pasid:20: __u32,
    pub rsvd:11: __u32,
    pub priv:1: __u32,
    pub flags:24: __u32,
    pub opcode:8: __u32,
    pub completion_addr: __u64,
    pub src1_addr: __u64,
    pub dst_addr: __u64,
    pub src1_size: __u32,
    pub int_handle: __u16,
    pub compr_flags: __u16,
    pub decompr_flags: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_raw_desc {
    pub field: [__u64; 8],
    pub __attribute__((packed)): },
//
// The status field will be modified by hardware, therefore it should be
// volatile and prevent the compiler from optimize the read.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_completion_record {
    pub status: volatile __u8,
    pub result: __u8,
    pub dif_status: __u8,
}

// common record
// DIF check & strip
// DIF insert
// DIF update
// DIX generate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_raw_completion_record {
    pub field: [__u64; 4],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iax_completion_record {
    pub status: volatile __u8,
    pub error_code: __u8,
    pub fault_info: __u8,
    pub rsvd: __u8,
    pub bytes_completed: __u32,
    pub fault_addr: __u64,
    pub invalid_flags: __u32,
    pub rsvd2: __u32,
    pub output_size: __u32,
    pub output_bits: __u8,
    pub rsvd3: __u8,
    pub xor_csum: __u16,
    pub crc: __u32,
    pub min: __u32,
    pub max: __u32,
    pub sum: __u32,
    pub rsvd4: [__u64; 2],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iax_raw_completion_record {
    pub field: [__u64; 8],
    pub __attribute__((packed)): },
