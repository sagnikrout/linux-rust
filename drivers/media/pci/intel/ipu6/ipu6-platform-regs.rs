//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-platform-regs.h
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
// Copyright (C) 2018 - 2024 Intel Corporation

//
// IPU6 uses uniform address within IPU6, therefore all subsystem registers
// locates in one single space starts from 0 but in different sctions with
// different addresses, the subsystem offsets are defined to 0 as the
// register definition will have the address offset to 0.
//
pub const IPU6_UNIFIED_OFFSET: c_int = 0;
pub const IPU6_ISYS_IOMMU0_OFFSET: c_uint = 0x2e0000;
pub const IPU6_ISYS_IOMMU1_OFFSET: c_uint = 0x2e0500;
pub const IPU6_ISYS_IOMMUI_OFFSET: c_uint = 0x2e0a00;
pub const IPU6_PSYS_IOMMU0_OFFSET: c_uint = 0x1b0000;
pub const IPU6_PSYS_IOMMU1_OFFSET: c_uint = 0x1b0700;
pub const IPU6_PSYS_IOMMU1R_OFFSET: c_uint = 0x1b0e00;
pub const IPU6_PSYS_IOMMUI_OFFSET: c_uint = 0x1b1500;
// the offset from IOMMU base register
pub const IPU6_MMU_L1_STREAM_ID_REG_OFFSET: c_uint = 0x0c;
pub const IPU6_MMU_L2_STREAM_ID_REG_OFFSET: c_uint = 0x4c;
pub const IPU6_PSYS_MMU1W_L2_STREAM_ID_REG_OFFSET: c_uint = 0x8c;
pub const IPU6_MMU_INFO_OFFSET: c_uint = 0x8;
pub const IPU6_ISYS_SPC_OFFSET: c_uint = 0x210000;
pub const IPU6SE_PSYS_SPC_OFFSET: c_uint = 0x110000;
pub const IPU6_PSYS_SPC_OFFSET: c_uint = 0x118000;
pub const IPU6_ISYS_DMEM_OFFSET: c_uint = 0x200000;
pub const IPU6_PSYS_DMEM_OFFSET: c_uint = 0x100000;
pub const IPU6_REG_ISYS_UNISPART_IRQ_EDGE: c_uint = 0x27c000;
pub const IPU6_REG_ISYS_UNISPART_IRQ_MASK: c_uint = 0x27c004;
pub const IPU6_REG_ISYS_UNISPART_IRQ_STATUS: c_uint = 0x27c008;
pub const IPU6_REG_ISYS_UNISPART_IRQ_CLEAR: c_uint = 0x27c00c;
pub const IPU6_REG_ISYS_UNISPART_IRQ_ENABLE: c_uint = 0x27c010;
pub const IPU6_REG_ISYS_UNISPART_IRQ_LEVEL_NOT_PULSE: c_uint = 0x27c014;
pub const IPU6_REG_ISYS_UNISPART_SW_IRQ_REG: c_uint = 0x27c414;
pub const IPU6_REG_ISYS_UNISPART_SW_IRQ_MUX_REG: c_uint = 0x27c418;

pub const IPU6_REG_ISYS_ISL_TOP_IRQ_EDGE: c_uint = 0x2b0200;
pub const IPU6_REG_ISYS_ISL_TOP_IRQ_MASK: c_uint = 0x2b0204;
pub const IPU6_REG_ISYS_ISL_TOP_IRQ_STATUS: c_uint = 0x2b0208;
pub const IPU6_REG_ISYS_ISL_TOP_IRQ_CLEAR: c_uint = 0x2b020c;
pub const IPU6_REG_ISYS_ISL_TOP_IRQ_ENABLE: c_uint = 0x2b0210;
pub const IPU6_REG_ISYS_ISL_TOP_IRQ_LEVEL_NOT_PULSE: c_uint = 0x2b0214;
pub const IPU6_REG_ISYS_CMPR_TOP_IRQ_EDGE: c_uint = 0x2d2100;
pub const IPU6_REG_ISYS_CMPR_TOP_IRQ_MASK: c_uint = 0x2d2104;
pub const IPU6_REG_ISYS_CMPR_TOP_IRQ_STATUS: c_uint = 0x2d2108;
pub const IPU6_REG_ISYS_CMPR_TOP_IRQ_CLEAR: c_uint = 0x2d210c;
pub const IPU6_REG_ISYS_CMPR_TOP_IRQ_ENABLE: c_uint = 0x2d2110;
pub const IPU6_REG_ISYS_CMPR_TOP_IRQ_LEVEL_NOT_PULSE: c_uint = 0x2d2114;
// CDC Burst collector thresholds for isys - 3 FIFOs i = 0..2

pub const IPU6_CSI_IRQ_NUM_PER_PIPE: c_int = 4;
pub const IPU6SE_ISYS_CSI_PORT_NUM: c_int = 4;
pub const IPU6_ISYS_CSI_PORT_NUM: c_int = 8;

// PKG DIR OFFSET in IMR in secure mode
pub const IPU6_PKG_DIR_IMR_OFFSET: c_uint = 0x40;
pub const IPU6_ISYS_REG_SPC_STATUS_CTRL: c_uint = 0x0;

pub const IPU6_PSYS_REG_SPC_STATUS_CTRL: c_uint = 0x0;
pub const IPU6_PSYS_REG_SPC_START_PC: c_uint = 0x4;
pub const IPU6_PSYS_REG_SPC_ICACHE_BASE: c_uint = 0x10;
pub const IPU6_REG_PSYS_INFO_SEG_0_CONFIG_ICACHE_MASTER: c_uint = 0x14;

pub const IPU6_PSYS_REG_SPP0_STATUS_CTRL: c_uint = 0x20000;

//
// s2m_pixel_soc_pixel_remapping is dedicated for the enabling of the
// pixel s2m remp ability.Remap here  means that s2m rearange the order
// of the pixels in each 4 pixels group.
// For examle, mirroring remping means that if input's 4 first pixels
// are 1 2 3 4 then in output we should see 4 3 2 1 in this 4 first pixels.
// 0xE4 is from s2m MAS document. It means no remapping.
//
pub const S2M_PIXEL_SOC_PIXEL_REMAPPING_FLAG_NO_REMAPPING: c_uint = 0xe4;
//
// csi_be_soc_pixel_remapping is for the enabling of the pixel remapping.
// This remapping is exactly like the stream2mmio remapping.
//
pub const CSI_BE_SOC_PIXEL_REMAPPING_FLAG_NO_REMAPPING: c_uint = 0xe4;
pub const IPU6_REG_DMA_TOP_AB_GROUP1_BASE_ADDR: c_uint = 0x1ae000;
pub const IPU6_REG_DMA_TOP_AB_GROUP2_BASE_ADDR: c_uint = 0x1af000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipu6_device_ab_group1_target_id {
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R0_SPC_DMEM,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R1_SPC_DMEM,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R2_SPC_DMEM,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R3_SPC_STATUS_REG,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R4_SPC_MASTER_BASE_ADDR,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R5_SPC_PC_STALL,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R6_SPC_EQ,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R7_SPC_RESERVED,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R8_SPC_RESERVED,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R9_SPP0,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R10_SPP1,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R11_CENTRAL_R1,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R12_IRQ,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R13_CENTRAL_R2,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R14_DMA,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R15_DMA,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R16_GP,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R17_ZLW_INSERTER,
    IPU6_DEVICE_AB_GROUP1_TARGET_ID_R18_AB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nci_ab_access_mode {
    NCI_AB_ACCESS_MODE_RW,	/* read & write */
    NCI_AB_ACCESS_MODE_RO,	/* read only */
    NCI_AB_ACCESS_MODE_WO,	/* write only */
    NCI_AB_ACCESS_MODE_NA,	/* No access at all */
}

// IRQ-related registers in PSYS
pub const IPU6_REG_PSYS_GPDEV_IRQ_EDGE: c_uint = 0x1aa200;
pub const IPU6_REG_PSYS_GPDEV_IRQ_MASK: c_uint = 0x1aa204;
pub const IPU6_REG_PSYS_GPDEV_IRQ_STATUS: c_uint = 0x1aa208;
pub const IPU6_REG_PSYS_GPDEV_IRQ_CLEAR: c_uint = 0x1aa20c;
pub const IPU6_REG_PSYS_GPDEV_IRQ_ENABLE: c_uint = 0x1aa210;
pub const IPU6_REG_PSYS_GPDEV_IRQ_LEVEL_NOT_PULSE: c_uint = 0x1aa214;
// There are 8 FW interrupts, n = 0..7
pub const IPU6_PSYS_GPDEV_FWIRQ0: c_int = 5;
pub const IPU6_PSYS_GPDEV_FWIRQ1: c_int = 6;
pub const IPU6_PSYS_GPDEV_FWIRQ2: c_int = 7;
pub const IPU6_PSYS_GPDEV_FWIRQ3: c_int = 8;
pub const IPU6_PSYS_GPDEV_FWIRQ4: c_int = 9;
pub const IPU6_PSYS_GPDEV_FWIRQ5: c_int = 10;
pub const IPU6_PSYS_GPDEV_FWIRQ6: c_int = 11;
pub const IPU6_PSYS_GPDEV_FWIRQ7: c_int = 12;

