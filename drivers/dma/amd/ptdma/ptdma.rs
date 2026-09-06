//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/amd/ptdma/ptdma.h
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
// AMD Passthru DMA device driver
// -- Based on the CCP driver
//
// Copyright (C) 2016,2021 Advanced Micro Devices, Inc.
//
// Author: Sanjay R Mehta <sanju.mehta@amd.com>
// Author: Tom Lendacky <thomas.lendacky@amd.com>
// Author: Gary R Hook <gary.hook@amd.com>
//

pub const MAX_PT_NAME_LEN: c_int = 16;
pub const MAX_DMAPOOL_NAME_LEN: c_int = 32;
pub const MAX_HW_QUEUES: c_int = 1;
pub const MAX_CMD_QLEN: c_int = 100;
pub const PT_ENGINE_PASSTHRU: c_int = 5;
// Register Mappings
pub const IRQ_MASK_REG: c_uint = 0x040;
pub const IRQ_STATUS_REG: c_uint = 0x200;

pub const CMD_QUEUE_PRIO_OFFSET: c_uint = 0x00;
pub const CMD_REQID_CONFIG_OFFSET: c_uint = 0x04;
pub const CMD_TIMEOUT_OFFSET: c_uint = 0x08;
pub const CMD_PT_VERSION: c_uint = 0x10;
pub const CMD_Q_CONTROL_BASE: c_uint = 0x0000;
pub const CMD_Q_TAIL_LO_BASE: c_uint = 0x0004;
pub const CMD_Q_HEAD_LO_BASE: c_uint = 0x0008;
pub const CMD_Q_INT_ENABLE_BASE: c_uint = 0x000C;
pub const CMD_Q_INTERRUPT_STATUS_BASE: c_uint = 0x0010;
pub const CMD_Q_STATUS_BASE: c_uint = 0x0100;
pub const CMD_Q_INT_STATUS_BASE: c_uint = 0x0104;
pub const CMD_Q_DMA_STATUS_BASE: c_uint = 0x0108;
pub const CMD_Q_DMA_READ_STATUS_BASE: c_uint = 0x010C;
pub const CMD_Q_DMA_WRITE_STATUS_BASE: c_uint = 0x0110;
pub const CMD_Q_ABORT_BASE: c_uint = 0x0114;
pub const CMD_Q_AX_CACHE_BASE: c_uint = 0x0118;
pub const CMD_CONFIG_OFFSET: c_uint = 0x1120;
pub const CMD_CLK_GATE_CTL_OFFSET: c_uint = 0x6004;
pub const CMD_DESC_DW0_VAL: c_uint = 0x500012;
// Address offset for virtual queue registers
pub const CMD_Q_STATUS_INCR: c_uint = 0x1000;
// Bit masks
pub const CMD_CONFIG_REQID: c_int = 0;
pub const CMD_TIMEOUT_DISABLE: c_int = 0;
pub const CMD_CLK_DYN_GATING_DIS: c_int = 0;
pub const CMD_CLK_SW_GATE_MODE: c_int = 0;
pub const CMD_CLK_GATE_CTL: c_int = 0;

pub const CMD_Q_LEN: c_int = 32;

// Local Storage Block
pub const LSB_START: c_int = 0;
pub const LSB_END: c_int = 127;

pub const PT_DMAPOOL_MAX_SIZE: c_int = 64;

pub const PT_PASSTHRU_BLOCKSIZE: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_tasklet_data {
    pub completion: completion,
    pub cmd: *mut pt_cmd,
}

//
// struct pt_passthru_engine - pass-through operation
// without performing DMA mapping
// @mask: mask to be applied to data
// @mask_len: length in bytes of mask
// @src_dma: data to be used for this operation
// @dst_dma: data produced by this operation
// @src_len: length in bytes of data used for this operation
//
// Variables required to be set when calling pt_enqueue_cmd():
// - bit_mod, byte_swap, src, dst, src_len
// - mask, mask_len if bit_mod is not PT_PASSTHRU_BITWISE_NOOP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_passthru_engine {
    pub mask: dma_addr_t,
    pub /: *mut *mut u32 mask_len; / In bytes,
    pub dst_dma: dma_addr_t src_dma,,
    pub /: *mut *mut u64 src_len; / In bytes,
}

//
// struct pt_cmd - PTDMA operation request
// @entry: list element
// @work: work element used for callbacks
// @pt: PT device to be run on
// @ret: operation return code
// @flags: cmd processing flags
// @engine: PTDMA operation to perform (passthru)
// @engine_error: PT engine return code
// @passthru: engine specific structures, refer to specific engine struct below
// @callback: operation completion callback function
// @data: parameter value to be supplied to the callback function
//
// Variables required to be set when calling pt_enqueue_cmd():
// - engine, callback
// - See the operation structures below for what is required for each
// operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_cmd {
    pub entry: list_head,
    pub work: work_struct,
    pub pt: *mut pt_device,
    pub ret: c_int,
    pub engine: u32,
    pub engine_error: u32,
    pub passthru: pt_passthru_engine,
// Completion callback support
    pub err): *mut *mut *mut void (pt_cmd_callback)(void data, int,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_dma_desc {
    pub vd: virt_dma_desc,
    pub pt: *mut pt_device,
    pub status: dma_status,
    pub len: usize,
    pub issued_to_hw: bool,
    pub pt_cmd: pt_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_dma_chan {
    pub vc: virt_dma_chan,
    pub pt: *mut pt_device,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_cmd_queue {
    pub pt: *mut pt_device,
// Queue dma pool
    pub dma_pool: *mut dma_pool,
// Queue base address (not necessarily aligned)
    pub qbase: *mut ptdma_desc,
// Aligned queue start address (per requirement)
    pub ____cacheline_aligned: spinlock_t q_lock,
    pub qidx: c_uint,
    pub qsize: c_uint,
    pub qbase_dma: dma_addr_t,
    pub qdma_tail: dma_addr_t,
    pub active: c_uint,
    pub suspended: c_uint,
// Interrupt flag
    pub int_en: bool,
// Register addresses for queue
    pub reg_control: *mut void __iomem,
    pub /: *mut *mut u32 qcontrol; / Cached control register,
// Status values from job
    pub int_status: u32,
    pub q_status: u32,
    pub q_int_status: u32,
    pub cmd_error: u32,
// Queue Statistics
    pub total_pt_ops: c_ulong,
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_device {
    pub entry: list_head,
    pub ord: c_uint,
    pub name: [c_char; MAX_PT_NAME_LEN],
    pub dev: *mut device,
// Bus specific device information
    pub pt_msix: *mut pt_msix,
    pub dev_vdata: *mut pt_dev_vdata,
    pub pt_irq: c_uint,
// I/O area used for device communication
    pub io_regs: *mut void __iomem,
    pub ____cacheline_aligned: spinlock_t cmd_lock,
    pub cmd_count: c_uint,
    pub cmd: list_head,
//
// The command queue. This represent the queue available on the
// PTDMA that are available for processing cmds
//
    pub cmd_q: pt_cmd_queue,
// Support for the DMA Engine capabilities
    pub dma_dev: dma_device,
    pub pt_dma_chan: *mut pt_dma_chan,
    pub dma_desc_cache: *mut kmem_cache,
    pub lsb_queue: wait_queue_head_t,
// Device Statistics
    pub total_interrupts: c_ulong,
    pub tdata: pt_tasklet_data,
    pub ver: c_int,
}

//
// descriptor for PTDMA commands
// 8 32-bit words:
// word 0: function; engine; control bits
// word 1: length of source data
// word 2: low 32 bits of source pointer
// word 3: upper 16 bits of source pointer; source memory type
// word 4: low 32 bits of destination pointer
// word 5: upper 16 bits of destination pointer; destination memory type
// word 6: reserved 32 bits
// word 7: reserved 32 bits
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dword3 {
    pub src_hi:16: c_uint,
    pub src_mem:2: c_uint,
    pub lsb_cxt_id:8: c_uint,
    pub rsvd1:5: c_uint,
    pub fixed:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dword5 {
    pub dst_hi:16: c_uint,
    pub dst_mem:2: c_uint,
    pub rsvd1:13: c_uint,
    pub fixed:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptdma_desc {
    pub dw0: u32,
    pub length: u32,
    pub src_lo: u32,
    pub dw3: dword3,
    pub dst_lo: u32,
    pub dw5: dword5,
    pub rsvd1: __le32,
    pub rsvd2: __le32,
}

// Structure to hold PT device data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_dev_vdata {
    pub bar: c_uint,
}

extern "C" {
    pub fn pt_dmaengine_register(pt: *mut pt_device) -> c_int;
}
extern "C" {
    pub fn pt_dmaengine_unregister(pt: *mut pt_device);
}
extern "C" {
    pub fn ptdma_debugfs_setup(pt: *mut pt_device);
}
extern "C" {
    pub fn pt_core_init(pt: *mut pt_device) -> c_int;
}
extern "C" {
    pub fn pt_core_destroy(pt: *mut pt_device);
}
extern "C" {
    pub fn pt_check_status_trans(pt: *mut pt_device, cmd_q: *mut pt_cmd_queue);
}
extern "C" {
    pub fn pt_start_queue(cmd_q: *mut pt_cmd_queue);
}
extern "C" {
    pub fn pt_stop_queue(cmd_q: *mut pt_cmd_queue);
}
