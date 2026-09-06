//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ethosu/ethosu_device.h
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


// SPDX-License-Identifier: GPL-2.0-only or MIT
// Copyright 2025 Arm, Ltd.

pub const NPU_REG_ID: c_uint = 0x0000;
pub const NPU_REG_STATUS: c_uint = 0x0004;
pub const NPU_REG_CMD: c_uint = 0x0008;
pub const NPU_REG_RESET: c_uint = 0x000c;
pub const NPU_REG_QBASE: c_uint = 0x0010;
pub const NPU_REG_QBASE_HI: c_uint = 0x0014;
pub const NPU_REG_QREAD: c_uint = 0x0018;
pub const NPU_REG_QCONFIG: c_uint = 0x001c;
pub const NPU_REG_QSIZE: c_uint = 0x0020;
pub const NPU_REG_PROT: c_uint = 0x0024;
pub const NPU_REG_CONFIG: c_uint = 0x0028;
pub const NPU_REG_REGIONCFG: c_uint = 0x003c;
pub const NPU_REG_AXILIMIT0: c_uint = 0x0040		// U65;
pub const NPU_REG_AXILIMIT1: c_uint = 0x0044		// U65;
pub const NPU_REG_AXILIMIT2: c_uint = 0x0048		// U65;
pub const NPU_REG_AXILIMIT3: c_uint = 0x004c		// U65;
pub const NPU_REG_MEM_ATTR0: c_uint = 0x0040		// U85;
pub const NPU_REG_MEM_ATTR1: c_uint = 0x0044		// U85;
pub const NPU_REG_MEM_ATTR2: c_uint = 0x0048		// U85;
pub const NPU_REG_MEM_ATTR3: c_uint = 0x004c		// U85;
pub const NPU_REG_AXI_SRAM: c_uint = 0x0050		// U85;
pub const NPU_REG_AXI_EXT: c_uint = 0x0054		// U85;

pub const NPU_BASEP_REGION_MAX: c_int = 8;
pub const NPU_REG_PMCR: c_uint = 0x0180;
pub const NPU_REG_PMCNTENSET: c_uint = 0x0184;
pub const NPU_REG_PMCNTENCLR: c_uint = 0x0188;
pub const NPU_REG_PMCCNTR_LO: c_uint = 0x01A0;
pub const NPU_REG_PMCCNTR_HI: c_uint = 0x01A4;
pub const NPU_REG_PMCCNTR_CFG: c_uint = 0x01A8;

pub const PMU_EV_TYPE_NONE: c_int = 0;
pub const PMU_EV_TYPE_CYCLES: c_uint = 0x11;
pub const PMU_EV_TYPE_IDLE: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethosu_cmds {
    NPU_OP_CONV = 0x2,
    NPU_OP_DEPTHWISE = 0x3,
    NPU_OP_POOL = 0x5,
    NPU_OP_ELEMENTWISE = 0x6,
    NPU_OP_RESIZE = 0x7,	// U85 only
    NPU_OP_DMA_START = 0x10,
    NPU_SET_IFM_PAD_TOP = 0x100,
    NPU_SET_IFM_PAD_LEFT = 0x101,
    NPU_SET_IFM_PAD_RIGHT = 0x102,
    NPU_SET_IFM_PAD_BOTTOM = 0x103,
    NPU_SET_IFM_DEPTH_M1 = 0x104,
    NPU_SET_IFM_PRECISION = 0x105,
    NPU_SET_IFM_BROADCAST = 0x108,
    NPU_SET_IFM_WIDTH0_M1 = 0x10a,
    NPU_SET_IFM_HEIGHT0_M1 = 0x10b,
    NPU_SET_IFM_HEIGHT1_M1 = 0x10c,
    NPU_SET_IFM_REGION = 0x10f,
    NPU_SET_OFM_WIDTH_M1 = 0x111,
    NPU_SET_OFM_HEIGHT_M1 = 0x112,
    NPU_SET_OFM_DEPTH_M1 = 0x113,
    NPU_SET_OFM_PRECISION = 0x114,
    NPU_SET_OFM_WIDTH0_M1 = 0x11a,
    NPU_SET_OFM_HEIGHT0_M1 = 0x11b,
    NPU_SET_OFM_HEIGHT1_M1 = 0x11c,
    NPU_SET_OFM_REGION = 0x11f,
    NPU_SET_KERNEL_WIDTH_M1 = 0x120,
    NPU_SET_KERNEL_HEIGHT_M1 = 0x121,
    NPU_SET_KERNEL_STRIDE = 0x122,
    NPU_SET_WEIGHT_REGION = 0x128,
    NPU_SET_SCALE_REGION = 0x129,
    NPU_SET_DMA0_SRC_REGION = 0x130,
    NPU_SET_DMA0_DST_REGION = 0x131,
    NPU_SET_DMA0_SIZE0 = 0x132,
    NPU_SET_DMA0_SIZE1 = 0x133,
    NPU_SET_IFM2_BROADCAST = 0x180,
    NPU_SET_IFM2_PRECISION = 0x185,
    NPU_SET_IFM2_WIDTH0_M1 = 0x18a,
    NPU_SET_IFM2_HEIGHT0_M1 = 0x18b,
    NPU_SET_IFM2_HEIGHT1_M1 = 0x18c,
    NPU_SET_IFM2_REGION = 0x18f,
    NPU_SET_IFM_BASE0 = 0x4000,
    NPU_SET_IFM_BASE1 = 0x4001,
    NPU_SET_IFM_BASE2 = 0x4002,
    NPU_SET_IFM_BASE3 = 0x4003,
    NPU_SET_IFM_STRIDE_X = 0x4004,
    NPU_SET_IFM_STRIDE_Y = 0x4005,
    NPU_SET_IFM_STRIDE_C = 0x4006,
    NPU_SET_OFM_BASE0 = 0x4010,
    NPU_SET_OFM_BASE1 = 0x4011,
    NPU_SET_OFM_BASE2 = 0x4012,
    NPU_SET_OFM_BASE3 = 0x4013,
    NPU_SET_OFM_STRIDE_X = 0x4014,
    NPU_SET_OFM_STRIDE_Y = 0x4015,
    NPU_SET_OFM_STRIDE_C = 0x4016,
    NPU_SET_WEIGHT_BASE = 0x4020,
    NPU_SET_WEIGHT_LENGTH = 0x4021,
    NPU_SET_SCALE_BASE = 0x4022,
    NPU_SET_SCALE_LENGTH = 0x4023,
    NPU_SET_DMA0_SRC = 0x4030,
    NPU_SET_DMA0_DST = 0x4031,
    NPU_SET_DMA0_LEN = 0x4032,
    NPU_SET_DMA0_SRC_STRIDE0 = 0x4033,
    NPU_SET_DMA0_SRC_STRIDE1 = 0x4034,
    NPU_SET_DMA0_DST_STRIDE0 = 0x4035,
    NPU_SET_DMA0_DST_STRIDE1 = 0x4036,
    NPU_SET_IFM2_BASE0 = 0x4080,
    NPU_SET_IFM2_BASE1 = 0x4081,
    NPU_SET_IFM2_BASE2 = 0x4082,
    NPU_SET_IFM2_BASE3 = 0x4083,
    NPU_SET_IFM2_STRIDE_X = 0x4084,
    NPU_SET_IFM2_STRIDE_Y = 0x4085,
    NPU_SET_IFM2_STRIDE_C = 0x4086,
    NPU_SET_WEIGHT1_BASE = 0x4090,
    NPU_SET_WEIGHT1_LENGTH = 0x4091,
    NPU_SET_SCALE1_BASE = 0x4092,
    NPU_SET_WEIGHT2_BASE = 0x4092,
    NPU_SET_SCALE1_LENGTH = 0x4093,
    NPU_SET_WEIGHT2_LENGTH = 0x4093,
    NPU_SET_WEIGHT3_BASE = 0x4094,
    NPU_SET_WEIGHT3_LENGTH = 0x4095,
}

//
// struct ethosu_device - Ethosu device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethosu_device {
// @base: Base drm_device.
    pub base: drm_device,
// @iomem: CPU mapping of the registers.
    pub regs: *mut void __iomem,
    pub pmu_regs: *mut void __iomem,
    pub sram: *mut void __iomem,
    pub srampool: *mut gen_pool,
    pub sramphys: dma_addr_t,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub irq: c_int,
    pub npu_info: drm_ethosu_npu_info,
    pub in_flight_job: *mut ethosu_job,
// For dma_fence
    pub fence_lock: spinlock_t,
    pub sched: drm_gpu_scheduler,
// For ethosu_job_do_push()
    pub sched_lock: mutex,
    pub fence_context: u64,
    pub emit_seqno: u64,
// Tracks the performance monitor state.
// Protects @active.
    pub lock: mutex,
// Perfmon currently programmed in HW (or NULL if none).
    pub active: *mut ethosu_perfmon,
    pub perfmon_state: },
    pub global_perfmon: *mut ethosu_perfmon,
}

