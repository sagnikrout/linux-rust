//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/lima/lima_regs.h
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
// Copyright 2010-2017 ARM Limited. All rights reserved.
// Copyright 2017-2019 Qiang Yu <yuq825@gmail.com>
//
// This file's register definition is collected from the
// official ARM Mali Utgard GPU kernel driver source code
//
// PMU regs
pub const LIMA_PMU_POWER_UP: c_uint = 0x00;
pub const LIMA_PMU_POWER_DOWN: c_uint = 0x04;

//
// On Mali450 each block automatically starts up its corresponding L2
// and the PPs are not fully independent controllable.
// Instead PP0, PP1-3 and PP4-7 can be turned on or off.
//

pub const LIMA_PMU_STATUS: c_uint = 0x08;
pub const LIMA_PMU_INT_MASK: c_uint = 0x0C;
pub const LIMA_PMU_INT_RAWSTAT: c_uint = 0x10;
pub const LIMA_PMU_INT_CLEAR: c_uint = 0x18;

pub const LIMA_PMU_SW_DELAY: c_uint = 0x1C;
// L2 cache regs
pub const LIMA_L2_CACHE_SIZE: c_uint = 0x0004;
pub const LIMA_L2_CACHE_STATUS: c_uint = 0x0008;

pub const LIMA_L2_CACHE_COMMAND: c_uint = 0x0010;

pub const LIMA_L2_CACHE_CLEAR_PAGE: c_uint = 0x0014;
pub const LIMA_L2_CACHE_MAX_READS: c_uint = 0x0018;
pub const LIMA_L2_CACHE_ENABLE: c_uint = 0x001C;

pub const LIMA_L2_CACHE_PERFCNT_SRC0: c_uint = 0x0020;
pub const LIMA_L2_CACHE_PERFCNT_VAL0: c_uint = 0x0024;
pub const LIMA_L2_CACHE_PERFCNT_SRC1: c_uint = 0x0028;
pub const LIMA_L2_CACHE_ERFCNT_VAL1: c_uint = 0x002C;
// GP regs
pub const LIMA_GP_VSCL_START_ADDR: c_uint = 0x00;
pub const LIMA_GP_VSCL_END_ADDR: c_uint = 0x04;
pub const LIMA_GP_PLBUCL_START_ADDR: c_uint = 0x08;
pub const LIMA_GP_PLBUCL_END_ADDR: c_uint = 0x0c;
pub const LIMA_GP_PLBU_ALLOC_START_ADDR: c_uint = 0x10;
pub const LIMA_GP_PLBU_ALLOC_END_ADDR: c_uint = 0x14;
pub const LIMA_GP_CMD: c_uint = 0x20;

pub const LIMA_GP_INT_RAWSTAT: c_uint = 0x24;
pub const LIMA_GP_INT_CLEAR: c_uint = 0x28;
pub const LIMA_GP_INT_MASK: c_uint = 0x2C;
pub const LIMA_GP_INT_STAT: c_uint = 0x30;

pub const LIMA_GP_WRITE_BOUND_LOW: c_uint = 0x34;
pub const LIMA_GP_PERF_CNT_0_ENABLE: c_uint = 0x3C;
pub const LIMA_GP_PERF_CNT_1_ENABLE: c_uint = 0x40;
pub const LIMA_GP_PERF_CNT_0_SRC: c_uint = 0x44;
pub const LIMA_GP_PERF_CNT_1_SRC: c_uint = 0x48;
pub const LIMA_GP_PERF_CNT_0_VALUE: c_uint = 0x4C;
pub const LIMA_GP_PERF_CNT_1_VALUE: c_uint = 0x50;
pub const LIMA_GP_PERF_CNT_0_LIMIT: c_uint = 0x54;
pub const LIMA_GP_STATUS: c_uint = 0x68;

pub const LIMA_GP_VERSION: c_uint = 0x6C;
pub const LIMA_GP_VSCL_START_ADDR_READ: c_uint = 0x80;
pub const LIMA_GP_PLBCL_START_ADDR_READ: c_uint = 0x84;
pub const LIMA_GP_CONTR_AXI_BUS_ERROR_STAT: c_uint = 0x94;

// PP regs
pub const LIMA_PP_FRAME: c_uint = 0x0000;
pub const LIMA_PP_RSW: c_uint = 0x0004;
pub const LIMA_PP_STACK: c_uint = 0x0030;
pub const LIMA_PP_STACK_SIZE: c_uint = 0x0034;
pub const LIMA_PP_ORIGIN_OFFSET_X: c_uint = 0x0040;

pub const LIMA_PP_WB_SOURCE_SELECT: c_uint = 0x0000;
pub const LIMA_PP_WB_SOURCE_ADDR: c_uint = 0x0004;
pub const LIMA_PP_VERSION: c_uint = 0x1000;
pub const LIMA_PP_CURRENT_REND_LIST_ADDR: c_uint = 0x1004;
pub const LIMA_PP_STATUS: c_uint = 0x1008;

pub const LIMA_PP_CTRL: c_uint = 0x100c;

pub const LIMA_PP_INT_RAWSTAT: c_uint = 0x1020;
pub const LIMA_PP_INT_CLEAR: c_uint = 0x1024;
pub const LIMA_PP_INT_MASK: c_uint = 0x1028;
pub const LIMA_PP_INT_STATUS: c_uint = 0x102c;

pub const LIMA_PP_WRITE_BOUNDARY_LOW: c_uint = 0x1044;
pub const LIMA_PP_BUS_ERROR_STATUS: c_uint = 0x1050;
pub const LIMA_PP_PERF_CNT_0_ENABLE: c_uint = 0x1080;
pub const LIMA_PP_PERF_CNT_0_SRC: c_uint = 0x1084;
pub const LIMA_PP_PERF_CNT_0_LIMIT: c_uint = 0x1088;
pub const LIMA_PP_PERF_CNT_0_VALUE: c_uint = 0x108c;
pub const LIMA_PP_PERF_CNT_1_ENABLE: c_uint = 0x10a0;
pub const LIMA_PP_PERF_CNT_1_SRC: c_uint = 0x10a4;
pub const LIMA_PP_PERF_CNT_1_LIMIT: c_uint = 0x10a8;
pub const LIMA_PP_PERF_CNT_1_VALUE: c_uint = 0x10ac;
pub const LIMA_PP_PERFMON_CONTR: c_uint = 0x10b0;
pub const LIMA_PP_PERFMON_BASE: c_uint = 0x10b4;

// MMU regs
pub const LIMA_MMU_DTE_ADDR: c_uint = 0x0000;
pub const LIMA_MMU_STATUS: c_uint = 0x0004;

pub const LIMA_MMU_COMMAND: c_uint = 0x0008;
pub const LIMA_MMU_COMMAND_ENABLE_PAGING: c_uint = 0x00;
pub const LIMA_MMU_COMMAND_DISABLE_PAGING: c_uint = 0x01;
pub const LIMA_MMU_COMMAND_ENABLE_STALL: c_uint = 0x02;
pub const LIMA_MMU_COMMAND_DISABLE_STALL: c_uint = 0x03;
pub const LIMA_MMU_COMMAND_ZAP_CACHE: c_uint = 0x04;
pub const LIMA_MMU_COMMAND_PAGE_FAULT_DONE: c_uint = 0x05;
pub const LIMA_MMU_COMMAND_HARD_RESET: c_uint = 0x06;
pub const LIMA_MMU_PAGE_FAULT_ADDR: c_uint = 0x000C;
pub const LIMA_MMU_ZAP_ONE_LINE: c_uint = 0x0010;
pub const LIMA_MMU_INT_RAWSTAT: c_uint = 0x0014;
pub const LIMA_MMU_INT_CLEAR: c_uint = 0x0018;
pub const LIMA_MMU_INT_MASK: c_uint = 0x001C;

pub const LIMA_MMU_INT_STATUS: c_uint = 0x0020;

pub const LIMA_VM_FLAG_MASK: c_uint = 0x1FF;

// DLBU regs
pub const LIMA_DLBU_MASTER_TLLIST_PHYS_ADDR: c_uint = 0x0000;
pub const LIMA_DLBU_MASTER_TLLIST_VADDR: c_uint = 0x0004;
pub const LIMA_DLBU_TLLIST_VBASEADDR: c_uint = 0x0008;
pub const LIMA_DLBU_FB_DIM: c_uint = 0x000C;
pub const LIMA_DLBU_TLLIST_CONF: c_uint = 0x0010;
pub const LIMA_DLBU_START_TILE_POS: c_uint = 0x0014;
pub const LIMA_DLBU_PP_ENABLE_MASK: c_uint = 0x0018;
// BCAST regs
pub const LIMA_BCAST_BROADCAST_MASK: c_uint = 0x0;
pub const LIMA_BCAST_INTERRUPT_MASK: c_uint = 0x4;
