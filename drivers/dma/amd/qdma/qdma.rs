//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/amd/qdma/qdma.h
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
// DMA header for AMD Queue-based DMA Subsystem
//
// Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
//

pub const DISABLE: c_int = 0;
pub const ENABLE: c_int = 1;
pub const QDMA_MIN_IRQ: c_int = 3;
pub const QDMA_INTR_NAME_MAX_LEN: c_int = 30;

pub const QDMA_IDENTIFIER: c_uint = 0x1FD3;

pub const QDMA_DEFAULT_RING_ID: c_int = 0;

pub const QDMA_DMAP_REG_STRIDE: c_int = 16;

pub const QDMA_MM_DESC_LEN_BITS: c_int = 28;

pub const QDMA_MIN_DMA_ALLOC_SIZE: c_int = 4096;

pub const QDMA_IDENTIFIER_REGOFF: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdma_reg_fields {
    QDMA_REGF_IRQ_ENABLE,
    QDMA_REGF_WBK_ENABLE,
    QDMA_REGF_WBI_CHECK,
    QDMA_REGF_IRQ_ARM,
    QDMA_REGF_IRQ_VEC,
    QDMA_REGF_IRQ_AGG,
    QDMA_REGF_WBI_INTVL_ENABLE,
    QDMA_REGF_MRKR_DISABLE,
    QDMA_REGF_QUEUE_ENABLE,
    QDMA_REGF_QUEUE_MODE,
    QDMA_REGF_DESC_BASE,
    QDMA_REGF_DESC_SIZE,
    QDMA_REGF_RING_ID,
    QDMA_REGF_CMD_INDX,
    QDMA_REGF_CMD_CMD,
    QDMA_REGF_CMD_TYPE,
    QDMA_REGF_CMD_BUSY,
    QDMA_REGF_QUEUE_COUNT,
    QDMA_REGF_QUEUE_MAX,
    QDMA_REGF_QUEUE_BASE,
    QDMA_REGF_FUNCTION_ID,
    QDMA_REGF_INTR_AGG_BASE,
    QDMA_REGF_INTR_VECTOR,
    QDMA_REGF_INTR_SIZE,
    QDMA_REGF_INTR_VALID,
    QDMA_REGF_INTR_COLOR,
    QDMA_REGF_INTR_FUNCTION_ID,
    QDMA_REGF_ERR_INT_FUNC,
    QDMA_REGF_ERR_INT_VEC,
    QDMA_REGF_ERR_INT_ARM,
    QDMA_REGF_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdma_regs {
    QDMA_REGO_CTXT_DATA,
    QDMA_REGO_CTXT_CMD,
    QDMA_REGO_CTXT_MASK,
    QDMA_REGO_MM_H2C_CTRL,
    QDMA_REGO_MM_C2H_CTRL,
    QDMA_REGO_QUEUE_COUNT,
    QDMA_REGO_RING_SIZE,
    QDMA_REGO_H2C_PIDX,
    QDMA_REGO_C2H_PIDX,
    QDMA_REGO_INTR_CIDX,
    QDMA_REGO_FUNC_ID,
    QDMA_REGO_ERR_INT,
    QDMA_REGO_ERR_STAT,
    QDMA_REGO_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_reg_field {
    pub /: *mut *mut u16 lsb; / Least significant bit of field,
    pub /: *mut *mut u16 msb; / Most significant bit of field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_reg {
    pub off: u32,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdma_desc_size {
    QDMA_DESC_SIZE_8B,
    QDMA_DESC_SIZE_16B,
    QDMA_DESC_SIZE_32B,
    QDMA_DESC_SIZE_64B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdma_queue_op_mode {
    QDMA_QUEUE_OP_STREAM,
    QDMA_QUEUE_OP_MM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdma_ctxt_type {
    QDMA_CTXT_DESC_SW_C2H,
    QDMA_CTXT_DESC_SW_H2C,
    QDMA_CTXT_DESC_HW_C2H,
    QDMA_CTXT_DESC_HW_H2C,
    QDMA_CTXT_DESC_CR_C2H,
    QDMA_CTXT_DESC_CR_H2C,
    QDMA_CTXT_WRB,
    QDMA_CTXT_PFTCH,
    QDMA_CTXT_INTR_COAL,
    QDMA_CTXT_RSVD,
    QDMA_CTXT_HOST_PROFILE,
    QDMA_CTXT_TIMER,
    QDMA_CTXT_FMAP,
    QDMA_CTXT_FNC_STS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qdma_ctxt_cmd {
    QDMA_CTXT_CLEAR,
    QDMA_CTXT_WRITE,
    QDMA_CTXT_READ,
    QDMA_CTXT_INVALIDATE,
    QDMA_CTXT_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_ctxt_sw_desc {
    pub desc_base: u64,
    pub vec: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_ctxt_intr {
    pub agg_base: u64,
    pub vec: u16,
    pub size: u32,
    pub valid: bool,
    pub color: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_ctxt_fmap {
    pub qbase: u16,
    pub qmax: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_mm_desc {
    pub src_addr: __le64,
    pub len: __le32,
    pub reserved1: __le32,
    pub dst_addr: __le64,
    pub reserved2: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_mm_vdesc {
    pub vdesc: virt_dma_desc,
    pub queue: *mut qdma_queue,
    pub sgl: *mut scatterlist,
    pub sg_off: u64,
    pub sg_len: u32,
    pub dev_addr: u64,
    pub pidx: u32,
    pub pending_descs: u32,
    pub cfg: dma_slave_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_queue {
    pub qdev: *mut qdma_device,
    pub vchan: virt_dma_chan,
    pub dir: dma_transfer_direction,
    pub cfg: dma_slave_config,
    pub desc_base: *mut qdma_mm_desc,
    pub submitted_vdesc: *mut qdma_mm_vdesc,
    pub issued_vdesc: *mut qdma_mm_vdesc,
    pub dma_desc_base: dma_addr_t,
    pub pidx_reg: u32,
    pub cidx_reg: u32,
    pub ring_size: u32,
    pub idx_mask: u32,
    pub qid: u16,
    pub pidx: u32,
    pub cidx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_intr_ring {
    pub qdev: *mut qdma_device,
    pub base: *mut __le64,
    pub dev_base: dma_addr_t,
    pub msix_name: [c_char; QDMA_INTR_NAME_MAX_LEN],
    pub msix_vector: u32,
    pub msix_id: u16,
    pub ring_size: u32,
    pub ridx: u16,
    pub cidx: u16,
    pub color: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qdma_device {
    pub pdev: *mut platform_device,
    pub dma_dev: dma_device,
    pub regmap: *mut regmap,
    pub /: *mut *mut mutex ctxt_lock; / protect ctxt registers,
    pub rfields: *const qdma_reg_field,
    pub roffs: *const qdma_reg,
    pub h2c_queues: *mut qdma_queue,
    pub c2h_queues: *mut qdma_queue,
    pub qintr_rings: *mut qdma_intr_ring,
    pub qintr_ring_num: u32,
    pub qintr_ring_idx: u32,
    pub chan_num: u32,
    pub queue_irq_start: u32,
    pub queue_irq_num: u32,
    pub err_irq_idx: u32,
    pub fid: u32,
}
