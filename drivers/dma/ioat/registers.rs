//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ioat/registers.h
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
// Copyright(c) 2004 - 2009 Intel Corporation. All rights reserved.
//
pub const IOAT_PCI_DMACTRL_OFFSET: c_uint = 0x48;
pub const IOAT_PCI_DMACTRL_DMA_EN: c_uint = 0x00000001;
pub const IOAT_PCI_DMACTRL_MSI_EN: c_uint = 0x00000002;
pub const IOAT_PCI_DEVICE_ID_OFFSET: c_uint = 0x02;
pub const IOAT_PCI_DMAUNCERRSTS_OFFSET: c_uint = 0x148;
pub const IOAT_PCI_CHANERR_INT_OFFSET: c_uint = 0x180;
pub const IOAT_PCI_CHANERRMASK_INT_OFFSET: c_uint = 0x184;
// MMIO Device Registers
pub const IOAT_CHANCNT_OFFSET: c_uint = 0x00	/*  8-bit */;
pub const IOAT_XFERCAP_OFFSET: c_uint = 0x01	/*  8-bit */;
pub const IOAT_XFERCAP_4KB: c_int = 12;
pub const IOAT_XFERCAP_8KB: c_int = 13;
pub const IOAT_XFERCAP_16KB: c_int = 14;
pub const IOAT_XFERCAP_32KB: c_int = 15;
pub const IOAT_XFERCAP_32GB: c_int = 0;
pub const IOAT_GENCTRL_OFFSET: c_uint = 0x02	/*  8-bit */;
pub const IOAT_GENCTRL_DEBUG_EN: c_uint = 0x01;
pub const IOAT_INTRCTRL_OFFSET: c_uint = 0x03	/*  8-bit */;
pub const IOAT_INTRCTRL_MASTER_INT_EN: c_uint = 0x01	/* Master Interrupt Enable */;
pub const IOAT_INTRCTRL_INT_STATUS: c_uint = 0x02	/* ATTNSTATUS -or- Channel Int */;
pub const IOAT_INTRCTRL_INT: c_uint = 0x04	/* INT_STATUS -and- MASTER_INT_EN */;
pub const IOAT_INTRCTRL_MSIX_VECTOR_CONTROL: c_uint = 0x08	/* Enable all MSI-X vectors */;
pub const IOAT_ATTNSTATUS_OFFSET: c_uint = 0x04	/* Each bit is a channel */;
pub const IOAT_VER_OFFSET: c_uint = 0x08	/*  8-bit */;
pub const IOAT_VER_MAJOR_MASK: c_uint = 0xF0;
pub const IOAT_VER_MINOR_MASK: c_uint = 0x0F;

pub const IOAT_PERPORTOFFSET_OFFSET: c_uint = 0x0A	/* 16-bit */;
pub const IOAT_INTRDELAY_OFFSET: c_uint = 0x0C	/* 16-bit */;
pub const IOAT_INTRDELAY_MASK: c_uint = 0x3FFF	/* Interrupt Delay Time */;
pub const IOAT_INTRDELAY_COALESE_SUPPORT: c_uint = 0x8000	/* Interrupt Coalescing Supported */;
pub const IOAT_DEVICE_STATUS_OFFSET: c_uint = 0x0E	/* 16-bit */;
pub const IOAT_DEVICE_STATUS_DEGRADED_MODE: c_uint = 0x0001;
pub const IOAT_DEVICE_MMIO_RESTRICTED: c_uint = 0x0002;
pub const IOAT_DEVICE_MEMORY_BYPASS: c_uint = 0x0004;
pub const IOAT_DEVICE_ADDRESS_REMAPPING: c_uint = 0x0008;
pub const IOAT_DMA_CAP_OFFSET: c_uint = 0x10	/* 32-bit */;
pub const IOAT_CAP_PAGE_BREAK: c_uint = 0x00000001;
pub const IOAT_CAP_CRC: c_uint = 0x00000002;
pub const IOAT_CAP_SKIP_MARKER: c_uint = 0x00000004;
pub const IOAT_CAP_DCA: c_uint = 0x00000010;
pub const IOAT_CAP_CRC_MOVE: c_uint = 0x00000020;
pub const IOAT_CAP_FILL_BLOCK: c_uint = 0x00000040;
pub const IOAT_CAP_APIC: c_uint = 0x00000080;
pub const IOAT_CAP_XOR: c_uint = 0x00000100;
pub const IOAT_CAP_PQ: c_uint = 0x00000200;
pub const IOAT_CAP_DWBES: c_uint = 0x00002000;
pub const IOAT_CAP_RAID16SS: c_uint = 0x00020000;
pub const IOAT_CAP_DPS: c_uint = 0x00800000;
pub const IOAT_PREFETCH_LIMIT_OFFSET: c_uint = 0x4C	/* CHWPREFLMT */;
pub const IOAT_CHANNEL_MMIO_SIZE: c_uint = 0x80	/* Each Channel MMIO space is this size */;
// DMA Channel Registers
pub const IOAT_CHANCTRL_OFFSET: c_uint = 0x00	/* 16-bit Channel Control Register */;
pub const IOAT_CHANCTRL_CHANNEL_PRIORITY_MASK: c_uint = 0xF000;
pub const IOAT3_CHANCTRL_COMPL_DCA_EN: c_uint = 0x0200;
pub const IOAT_CHANCTRL_CHANNEL_IN_USE: c_uint = 0x0100;
pub const IOAT_CHANCTRL_DESCRIPTOR_ADDR_SNOOP_CONTROL: c_uint = 0x0020;
pub const IOAT_CHANCTRL_ERR_INT_EN: c_uint = 0x0010;
pub const IOAT_CHANCTRL_ANY_ERR_ABORT_EN: c_uint = 0x0008;
pub const IOAT_CHANCTRL_ERR_COMPLETION_EN: c_uint = 0x0004;
pub const IOAT_CHANCTRL_INT_REARM: c_uint = 0x0001;

pub const IOAT_DMA_COMP_OFFSET: c_uint = 0x02	/* 16-bit DMA channel compatibility */;
pub const IOAT_DMA_COMP_V1: c_uint = 0x0001	/* Compatibility with DMA version 1 */;
pub const IOAT_DMA_COMP_V2: c_uint = 0x0002	/* Compatibility with DMA version 2 */;
pub const IOAT_CHANSTS_OFFSET: c_uint = 0x08	/* 64-bit Channel Status Register */;

pub const IOAT_CHANSTS_SOFT_ERR: c_uint = 0x10ULL;
pub const IOAT_CHANSTS_UNAFFILIATED_ERR: c_uint = 0x8ULL;
pub const IOAT_CHANSTS_STATUS: c_uint = 0x7ULL;
pub const IOAT_CHANSTS_ACTIVE: c_uint = 0x0;
pub const IOAT_CHANSTS_DONE: c_uint = 0x1;
pub const IOAT_CHANSTS_SUSPENDED: c_uint = 0x2;
pub const IOAT_CHANSTS_HALTED: c_uint = 0x3;
pub const IOAT_CHAN_DMACOUNT_OFFSET: c_uint = 0x06    /* 16-bit DMA Count register */;
pub const IOAT_DCACTRL_OFFSET: c_uint = 0x30   /* 32 bit Direct Cache Access Control Register */;
pub const IOAT_DCACTRL_CMPL_WRITE_ENABLE: c_uint = 0x10000;
pub const IOAT_DCACTRL_TARGET_CPU_MASK: c_uint = 0xFFFF /* APIC ID */;
// CB DCA Memory Space Registers
pub const IOAT_DCAOFFSET_OFFSET: c_uint = 0x14;
// CB_BAR + IOAT_DCAOFFSET value
pub const IOAT_DCA_VER_OFFSET: c_uint = 0x00;
pub const IOAT_DCA_VER_MAJOR_MASK: c_uint = 0xF0;
pub const IOAT_DCA_VER_MINOR_MASK: c_uint = 0x0F;
pub const IOAT_DCA_COMP_OFFSET: c_uint = 0x02;
pub const IOAT_DCA_COMP_V1: c_uint = 0x1;
pub const IOAT_FSB_CAPABILITY_OFFSET: c_uint = 0x04;
pub const IOAT_FSB_CAPABILITY_PREFETCH: c_uint = 0x1;
pub const IOAT_PCI_CAPABILITY_OFFSET: c_uint = 0x06;
pub const IOAT_PCI_CAPABILITY_MEMWR: c_uint = 0x1;
pub const IOAT_FSB_CAP_ENABLE_OFFSET: c_uint = 0x08;
pub const IOAT_FSB_CAP_ENABLE_PREFETCH: c_uint = 0x1;
pub const IOAT_PCI_CAP_ENABLE_OFFSET: c_uint = 0x0A;
pub const IOAT_PCI_CAP_ENABLE_MEMWR: c_uint = 0x1;
pub const IOAT_APICID_TAG_MAP_OFFSET: c_uint = 0x0C;
pub const IOAT_APICID_TAG_MAP_TAG0: c_uint = 0x0000000F;
pub const IOAT_APICID_TAG_MAP_TAG0_SHIFT: c_int = 0;
pub const IOAT_APICID_TAG_MAP_TAG1: c_uint = 0x000000F0;
pub const IOAT_APICID_TAG_MAP_TAG1_SHIFT: c_int = 4;
pub const IOAT_APICID_TAG_MAP_TAG2: c_uint = 0x00000F00;
pub const IOAT_APICID_TAG_MAP_TAG2_SHIFT: c_int = 8;
pub const IOAT_APICID_TAG_MAP_TAG3: c_uint = 0x0000F000;
pub const IOAT_APICID_TAG_MAP_TAG3_SHIFT: c_int = 12;
pub const IOAT_APICID_TAG_MAP_TAG4: c_uint = 0x000F0000;
pub const IOAT_APICID_TAG_MAP_TAG4_SHIFT: c_int = 16;
pub const IOAT_APICID_TAG_CB2_VALID: c_uint = 0x8080808080;
pub const IOAT_DCA_GREQID_OFFSET: c_uint = 0x10;
pub const IOAT_DCA_GREQID_SIZE: c_uint = 0x04;
pub const IOAT_DCA_GREQID_MASK: c_uint = 0xFFFF;
pub const IOAT_DCA_GREQID_IGNOREFUN: c_uint = 0x10000000;
pub const IOAT_DCA_GREQID_VALID: c_uint = 0x20000000;
pub const IOAT_DCA_GREQID_LASTID: c_uint = 0x80000000;
pub const IOAT3_CSI_CAPABILITY_OFFSET: c_uint = 0x08;
pub const IOAT3_CSI_CAPABILITY_PREFETCH: c_uint = 0x1;
pub const IOAT3_PCI_CAPABILITY_OFFSET: c_uint = 0x0A;
pub const IOAT3_PCI_CAPABILITY_MEMWR: c_uint = 0x1;
pub const IOAT3_CSI_CONTROL_OFFSET: c_uint = 0x0C;
pub const IOAT3_CSI_CONTROL_PREFETCH: c_uint = 0x1;
pub const IOAT3_PCI_CONTROL_OFFSET: c_uint = 0x0E;
pub const IOAT3_PCI_CONTROL_MEMWR: c_uint = 0x1;
pub const IOAT3_APICID_TAG_MAP_OFFSET: c_uint = 0x10;
pub const IOAT3_APICID_TAG_MAP_OFFSET_LOW: c_uint = 0x10;
pub const IOAT3_APICID_TAG_MAP_OFFSET_HIGH: c_uint = 0x14;
pub const IOAT3_DCA_GREQID_OFFSET: c_uint = 0x02;
pub const IOAT1_CHAINADDR_OFFSET: c_uint = 0x0C	/* 64-bit Descriptor Chain Address Register */;
pub const IOAT2_CHAINADDR_OFFSET: c_uint = 0x10	/* 64-bit Descriptor Chain Address Register */;

pub const IOAT1_CHAINADDR_OFFSET_LOW: c_uint = 0x0C;
pub const IOAT2_CHAINADDR_OFFSET_LOW: c_uint = 0x10;

pub const IOAT1_CHAINADDR_OFFSET_HIGH: c_uint = 0x10;
pub const IOAT2_CHAINADDR_OFFSET_HIGH: c_uint = 0x14;

pub const IOAT1_CHANCMD_OFFSET: c_uint = 0x14	/*  8-bit DMA Channel Command Register */;
pub const IOAT2_CHANCMD_OFFSET: c_uint = 0x04	/*  8-bit DMA Channel Command Register */;

pub const IOAT_CHANCMD_RESET: c_uint = 0x20;
pub const IOAT_CHANCMD_RESUME: c_uint = 0x10;
pub const IOAT_CHANCMD_ABORT: c_uint = 0x08;
pub const IOAT_CHANCMD_SUSPEND: c_uint = 0x04;
pub const IOAT_CHANCMD_APPEND: c_uint = 0x02;
pub const IOAT_CHANCMD_START: c_uint = 0x01;
pub const IOAT_CHANCMP_OFFSET: c_uint = 0x18	/* 64-bit Channel Completion Address Register */;
pub const IOAT_CHANCMP_OFFSET_LOW: c_uint = 0x18;
pub const IOAT_CHANCMP_OFFSET_HIGH: c_uint = 0x1C;
pub const IOAT_CDAR_OFFSET: c_uint = 0x20	/* 64-bit Current Descriptor Address Register */;
pub const IOAT_CDAR_OFFSET_LOW: c_uint = 0x20;
pub const IOAT_CDAR_OFFSET_HIGH: c_uint = 0x24;
pub const IOAT_CHANERR_OFFSET: c_uint = 0x28	/* 32-bit Channel Error Register */;
pub const IOAT_CHANERR_SRC_ADDR_ERR: c_uint = 0x0001;
pub const IOAT_CHANERR_DEST_ADDR_ERR: c_uint = 0x0002;
pub const IOAT_CHANERR_NEXT_ADDR_ERR: c_uint = 0x0004;
pub const IOAT_CHANERR_NEXT_DESC_ALIGN_ERR: c_uint = 0x0008;
pub const IOAT_CHANERR_CHAIN_ADDR_VALUE_ERR: c_uint = 0x0010;
pub const IOAT_CHANERR_CHANCMD_ERR: c_uint = 0x0020;
pub const IOAT_CHANERR_CHIPSET_UNCORRECTABLE_DATA_INTEGRITY_ERR: c_uint = 0x0040;
pub const IOAT_CHANERR_DMA_UNCORRECTABLE_DATA_INTEGRITY_ERR: c_uint = 0x0080;
pub const IOAT_CHANERR_READ_DATA_ERR: c_uint = 0x0100;
pub const IOAT_CHANERR_WRITE_DATA_ERR: c_uint = 0x0200;
pub const IOAT_CHANERR_CONTROL_ERR: c_uint = 0x0400;
pub const IOAT_CHANERR_LENGTH_ERR: c_uint = 0x0800;
pub const IOAT_CHANERR_COMPLETION_ADDR_ERR: c_uint = 0x1000;
pub const IOAT_CHANERR_INT_CONFIGURATION_ERR: c_uint = 0x2000;
pub const IOAT_CHANERR_SOFT_ERR: c_uint = 0x4000;
pub const IOAT_CHANERR_UNAFFILIATED_ERR: c_uint = 0x8000;
pub const IOAT_CHANERR_XOR_P_OR_CRC_ERR: c_uint = 0x10000;
pub const IOAT_CHANERR_XOR_Q_ERR: c_uint = 0x20000;
pub const IOAT_CHANERR_DESCRIPTOR_COUNT_ERR: c_uint = 0x40000;

pub const IOAT_CHANERR_MASK_OFFSET: c_uint = 0x2C	/* 32-bit Channel Error Register */;
pub const IOAT_CHAN_DRSCTL_OFFSET: c_uint = 0xB6;
pub const IOAT_CHAN_DRSZ_4KB: c_uint = 0x0000;
pub const IOAT_CHAN_DRSZ_8KB: c_uint = 0x0001;
pub const IOAT_CHAN_DRSZ_2MB: c_uint = 0x0009;
pub const IOAT_CHAN_DRS_EN: c_uint = 0x0100;
pub const IOAT_CHAN_DRS_AUTOWRAP: c_uint = 0x0200;
pub const IOAT_CHAN_LTR_SWSEL_OFFSET: c_uint = 0xBC;
pub const IOAT_CHAN_LTR_SWSEL_ACTIVE: c_uint = 0x0;
pub const IOAT_CHAN_LTR_SWSEL_IDLE: c_uint = 0x1;
pub const IOAT_CHAN_LTR_ACTIVE_OFFSET: c_uint = 0xC0;
pub const IOAT_CHAN_LTR_ACTIVE_SNVAL: c_uint = 0x0000	/* 0 us */;
pub const IOAT_CHAN_LTR_ACTIVE_SNLATSCALE: c_uint = 0x0800	/* 1us scale */;
pub const IOAT_CHAN_LTR_ACTIVE_SNREQMNT: c_uint = 0x8000	/* snoop req enable */;
pub const IOAT_CHAN_LTR_IDLE_OFFSET: c_uint = 0xC4;
pub const IOAT_CHAN_LTR_IDLE_SNVAL: c_uint = 0x0258	/* 600 us */;
pub const IOAT_CHAN_LTR_IDLE_SNLATSCALE: c_uint = 0x0800	/* 1us scale */;
pub const IOAT_CHAN_LTR_IDLE_SNREQMNT: c_uint = 0x8000	/* snoop req enable */;
