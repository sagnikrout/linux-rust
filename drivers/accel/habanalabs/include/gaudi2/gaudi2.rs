//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/gaudi2.h
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
//
// Copyright 2020-2022 HabanaLabs, Ltd.
// All Rights Reserved.
//
pub const SRAM_CFG_BAR_ID: c_int = 0;
pub const MSIX_BAR_ID: c_int = 2;
pub const DRAM_BAR_ID: c_int = 4;
// Refers to CFG_REGION_SIZE, BAR0_RSRVD_SIZE and SRAM_SIZE
pub const CFG_BAR_SIZE: c_uint = 0x10000000ull		/* 256MB */;
pub const MSIX_BAR_SIZE: c_uint = 0x4000ull		/* 16KB */;
pub const CFG_BASE: c_uint = 0x1000007FF8000000ull;
pub const CFG_SIZE: c_uint = 0x8000000ull		/* 96MB CFG + 32MB DBG*/;
pub const CFG_REGION_SIZE: c_uint = 0xC000000ull		/* 192MB */;
pub const STM_FLASH_BASE_ADDR: c_uint = 0x1000007FF4000000ull	/* Not 256MB aligned */;
pub const STM_FLASH_ALIGNED_OFF: c_uint = 0x4000000ull		/* 256 MB alignment */;
pub const STM_FLASH_SIZE: c_uint = 0x2000000ull		/* 32MB */;
pub const SPI_FLASH_BASE_ADDR: c_uint = 0x1000007FF6000000ull;
pub const SPI_FLASH_SIZE: c_uint = 0x1000000ull		/* 16MB */;
pub const SCRATCHPAD_SRAM_ADDR: c_uint = 0x1000007FF7FE0000ull;
pub const SCRATCHPAD_SRAM_SIZE: c_uint = 0x10000ull		/* 64KB */;
pub const PCIE_FW_SRAM_ADDR: c_uint = 0x1000007FF7FF0000ull;
pub const PCIE_FW_SRAM_SIZE: c_uint = 0x8000			/* 32KB */;
pub const BAR0_RSRVD_BASE_ADDR: c_uint = 0x1000FFFFFC000000ull;
pub const BAR0_RSRVD_SIZE: c_uint = 0x1000000ull		/* 16MB */;
pub const SRAM_BASE_ADDR: c_uint = 0x1000FFFFFD000000ull;
pub const SRAM_SIZE: c_uint = 0x3000000ull		/* 48MB */;
pub const DRAM_PHYS_BASE: c_uint = 0x1001000000000000ull;
// every hint address is masked accordingly
pub const DRAM_VA_HINT_MASK: c_uint = 0xFFFFFFFFFFFFull	/* 48bit mask */;
pub const HOST_PHYS_BASE_0: c_uint = 0x0000000000000000ull;
pub const HOST_PHYS_SIZE_0: c_uint = 0x0100000000000000ull	/* 64PB (56 bits) */;
pub const HOST_PHYS_BASE_1: c_uint = 0xFF00000000000000ull;
pub const HOST_PHYS_SIZE_1: c_uint = 0x0100000000000000ull	/* 64PB (56 bits) */;
pub const RESERVED_VA_RANGE_FOR_ARC_ON_HBM_START: c_uint = 0x1001500000000000ull;
pub const RESERVED_VA_RANGE_FOR_ARC_ON_HBM_END: c_uint = 0x10016FFFFFFFFFFFull;
pub const RESERVED_VA_FOR_VIRTUAL_MSIX_DOORBELL_START: c_uint = 0xFFF077FFFFFF0000ull;
pub const RESERVED_VA_FOR_VIRTUAL_MSIX_DOORBELL_END: c_uint = 0xFFF077FFFFFFFFFFull;
pub const RESERVED_VA_RANGE_FOR_ARC_ON_HOST_START: c_uint = 0xFFF0780000000000ull;
pub const RESERVED_VA_RANGE_FOR_ARC_ON_HOST_END: c_uint = 0xFFF07FFFFFFFFFFFull;
pub const RESERVED_VA_RANGE_FOR_ARC_ON_HOST_HPAGE_START: c_uint = 0xFFF0F80000000000ull;
pub const RESERVED_VA_RANGE_FOR_ARC_ON_HOST_HPAGE_END: c_uint = 0xFFF0FFFFFFFFFFFFull;
pub const RESERVED_MSIX_UNEXPECTED_USER_ERROR_INTERRUPT: c_int = 127;
pub const GAUDI2_MSIX_ENTRIES: c_int = 128;

pub const MAX_ASID: c_int = 2;
pub const NUM_ARC_CPUS: c_int = 69;
// Every ARC cpu in the system contains a single DCCM block
// except MME and Scheduler ARCs which contain 2 DCCM blocks
//
pub const ARC_DCCM_BLOCK_SIZE: c_uint = 0x8000;
pub const NUM_OF_DCORES: c_int = 4;
pub const NUM_OF_SFT: c_int = 4;
pub const NUM_OF_PSOC_ARC: c_int = 2;
pub const NUM_OF_SCHEDULER_ARC: c_int = 6;
pub const NUM_OF_PQ_PER_QMAN: c_int = 4;
pub const NUM_OF_CQ_PER_QMAN: c_int = 5;
pub const NUM_OF_CP_PER_QMAN: c_int = 5;
pub const NUM_OF_EDMA_PER_DCORE: c_int = 2;
pub const NUM_OF_HIF_PER_DCORE: c_int = 4;
pub const NUM_OF_PDMA: c_int = 2;
pub const NUM_OF_TPC_PER_DCORE: c_int = 6;
pub const NUM_DCORE0_TPC: c_int = 7;

pub const NUM_OF_DEC_PER_DCORE: c_int = 2;
pub const NUM_OF_ROT: c_int = 2;
pub const NUM_OF_HMMU_PER_DCORE: c_int = 4;
pub const NUM_OF_MME_PER_DCORE: c_int = 1;
pub const NUM_OF_MME_SBTE_PER_DCORE: c_int = 5;
pub const NUM_OF_MME_WB_PER_DCORE: c_int = 2;
pub const NUM_OF_RTR_PER_DCORE: c_int = 8;
pub const NUM_OF_VDEC_PER_DCORE: c_int = 2;
pub const NUM_OF_IF_RTR_PER_SFT: c_int = 3;
pub const NUM_OF_PCIE_VDEC: c_int = 2;
pub const NUM_OF_ARC_FARMS_ARC: c_int = 4;
pub const NUM_OF_XBAR: c_int = 4;
pub const TPC_NUM_OF_KERNEL_TENSORS: c_int = 16;
pub const TPC_NUM_OF_QM_TENSORS: c_int = 16;
pub const MME_NUM_OF_LFSR_SEEDS: c_int = 256;
pub const NIC_NUMBER_OF_MACROS: c_int = 12;
pub const NIC_NUMBER_OF_QM_PER_MACRO: c_int = 2;

pub const DEVICE_CACHE_LINE_SIZE: c_int = 128;
