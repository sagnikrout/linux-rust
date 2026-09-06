//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/riscv/iommu-bits.h
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
// Copyright © 2022-2024 Rivos Inc.
// Copyright © 2023 FORTH-ICS/CARV
// Copyright © 2023 RISC-V IOMMU Task Group
//
// RISC-V IOMMU - Register Layout and Data Structures.
//
// Based on the 'RISC-V IOMMU Architecture Specification', Version 1.0
// Published at  https://github.com/riscv-non-isa/riscv-iommu
//

//
// Chapter 5: Memory Mapped register interface
//
// Common field positions

// 5.3 IOMMU Capabilities (64bits)
pub const RISCV_IOMMU_REG_CAPABILITIES: c_uint = 0x0000;

//
// enum riscv_iommu_igs_settings - Interrupt Generation Support Settings
// @RISCV_IOMMU_CAPABILITIES_IGS_MSI: IOMMU supports only MSI generation
// @RISCV_IOMMU_CAPABILITIES_IGS_WSI: IOMMU supports only Wired-Signaled interrupt
// @RISCV_IOMMU_CAPABILITIES_IGS_BOTH: IOMMU supports both MSI and WSI generation
// @RISCV_IOMMU_CAPABILITIES_IGS_RSRV: Reserved for standard use
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum riscv_iommu_igs_settings {
    RISCV_IOMMU_CAPABILITIES_IGS_MSI = 0,
    RISCV_IOMMU_CAPABILITIES_IGS_WSI = 1,
    RISCV_IOMMU_CAPABILITIES_IGS_BOTH = 2,
    RISCV_IOMMU_CAPABILITIES_IGS_RSRV = 3
}

// 5.4 Features control register (32bits)
pub const RISCV_IOMMU_REG_FCTL: c_uint = 0x0008;

// 5.5 Device-directory-table pointer (64bits)
pub const RISCV_IOMMU_REG_DDTP: c_uint = 0x0010;

//
// enum riscv_iommu_ddtp_modes - IOMMU translation modes
// @RISCV_IOMMU_DDTP_IOMMU_MODE_OFF: No inbound transactions allowed
// @RISCV_IOMMU_DDTP_IOMMU_MODE_BARE: Pass-through mode
// @RISCV_IOMMU_DDTP_IOMMU_MODE_1LVL: One-level DDT
// @RISCV_IOMMU_DDTP_IOMMU_MODE_2LVL: Two-level DDT
// @RISCV_IOMMU_DDTP_IOMMU_MODE_3LVL: Three-level DDT
// @RISCV_IOMMU_DDTP_IOMMU_MODE_MAX: Max value allowed by specification
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum riscv_iommu_ddtp_modes {
    RISCV_IOMMU_DDTP_IOMMU_MODE_OFF = 0,
    RISCV_IOMMU_DDTP_IOMMU_MODE_BARE = 1,
    RISCV_IOMMU_DDTP_IOMMU_MODE_1LVL = 2,
    RISCV_IOMMU_DDTP_IOMMU_MODE_2LVL = 3,
    RISCV_IOMMU_DDTP_IOMMU_MODE_3LVL = 4,
    RISCV_IOMMU_DDTP_IOMMU_MODE_MAX = 4
}

// 5.6 Command Queue Base (64bits)
pub const RISCV_IOMMU_REG_CQB: c_uint = 0x0018;

// 5.7 Command Queue head (32bits)
pub const RISCV_IOMMU_REG_CQH: c_uint = 0x0020;

// 5.8 Command Queue tail (32bits)
pub const RISCV_IOMMU_REG_CQT: c_uint = 0x0024;

// 5.9 Fault Queue Base (64bits)
pub const RISCV_IOMMU_REG_FQB: c_uint = 0x0028;

// 5.10 Fault Queue Head (32bits)
pub const RISCV_IOMMU_REG_FQH: c_uint = 0x0030;

// 5.11 Fault Queue tail (32bits)
pub const RISCV_IOMMU_REG_FQT: c_uint = 0x0034;

// 5.12 Page Request Queue base (64bits)
pub const RISCV_IOMMU_REG_PQB: c_uint = 0x0038;

// 5.13 Page Request Queue head (32bits)
pub const RISCV_IOMMU_REG_PQH: c_uint = 0x0040;

// 5.14 Page Request Queue tail (32bits)
pub const RISCV_IOMMU_REG_PQT: c_uint = 0x0044;

// 5.15 Command Queue CSR (32bits)
pub const RISCV_IOMMU_REG_CQCSR: c_uint = 0x0048;

// 5.16 Fault Queue CSR (32bits)
pub const RISCV_IOMMU_REG_FQCSR: c_uint = 0x004C;

// 5.17 Page Request Queue CSR (32bits)
pub const RISCV_IOMMU_REG_PQCSR: c_uint = 0x0050;

// 5.18 Interrupt Pending Status (32bits)
pub const RISCV_IOMMU_REG_IPSR: c_uint = 0x0054;
pub const RISCV_IOMMU_INTR_CQ: c_int = 0;
pub const RISCV_IOMMU_INTR_FQ: c_int = 1;
pub const RISCV_IOMMU_INTR_PM: c_int = 2;
pub const RISCV_IOMMU_INTR_PQ: c_int = 3;
pub const RISCV_IOMMU_INTR_COUNT: c_int = 4;

// 5.19 Performance monitoring counter overflow status (32bits)
pub const RISCV_IOMMU_REG_IOCOUNTOVF: c_uint = 0x0058;

// 5.20 Performance monitoring counter inhibits (32bits)
pub const RISCV_IOMMU_REG_IOCOUNTINH: c_uint = 0x005C;

// 5.21 Performance monitoring cycles counter (64bits)
pub const RISCV_IOMMU_REG_IOHPMCYCLES: c_uint = 0x0060;

// 5.22 Performance monitoring event counters (31 * 64bits)
pub const RISCV_IOMMU_REG_IOHPMCTR_BASE: c_uint = 0x0068;

// 5.23 Performance monitoring event selectors (31 * 64bits)
pub const RISCV_IOMMU_REG_IOHPMEVT_BASE: c_uint = 0x0160;

// Number of defined performance-monitoring event selectors
pub const RISCV_IOMMU_IOHPMEVT_CNT: c_int = 31;
//
// enum riscv_iommu_hpmevent_id - Performance-monitoring event identifier
//
// @RISCV_IOMMU_HPMEVENT_INVALID: Invalid event, do not count
// @RISCV_IOMMU_HPMEVENT_URQ: Untranslated requests
// @RISCV_IOMMU_HPMEVENT_TRQ: Translated requests
// @RISCV_IOMMU_HPMEVENT_ATS_RQ: ATS translation requests
// @RISCV_IOMMU_HPMEVENT_TLB_MISS: TLB misses
// @RISCV_IOMMU_HPMEVENT_DD_WALK: Device directory walks
// @RISCV_IOMMU_HPMEVENT_PD_WALK: Process directory walks
// @RISCV_IOMMU_HPMEVENT_S_VS_WALKS: First-stage page table walks
// @RISCV_IOMMU_HPMEVENT_G_WALKS: Second-stage page table walks
// @RISCV_IOMMU_HPMEVENT_MAX: Value to denote maximum Event IDs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum riscv_iommu_hpmevent_id {
    RISCV_IOMMU_HPMEVENT_INVALID    = 0,
    RISCV_IOMMU_HPMEVENT_URQ        = 1,
    RISCV_IOMMU_HPMEVENT_TRQ        = 2,
    RISCV_IOMMU_HPMEVENT_ATS_RQ     = 3,
    RISCV_IOMMU_HPMEVENT_TLB_MISS   = 4,
    RISCV_IOMMU_HPMEVENT_DD_WALK    = 5,
    RISCV_IOMMU_HPMEVENT_PD_WALK    = 6,
    RISCV_IOMMU_HPMEVENT_S_VS_WALKS = 7,
    RISCV_IOMMU_HPMEVENT_G_WALKS    = 8,
    RISCV_IOMMU_HPMEVENT_MAX        = 9
}

// 5.24 Translation request IOVA (64bits)
pub const RISCV_IOMMU_REG_TR_REQ_IOVA: c_uint = 0x0258;

// 5.25 Translation request control (64bits)
pub const RISCV_IOMMU_REG_TR_REQ_CTL: c_uint = 0x0260;

// 5.26 Translation request response (64bits)
pub const RISCV_IOMMU_REG_TR_RESPONSE: c_uint = 0x0268;

// 5.27 Interrupt cause to vector (64bits)
pub const RISCV_IOMMU_REG_ICVEC: c_uint = 0x02F8;

// 5.28 MSI Configuration table (32 * 64bits)
pub const RISCV_IOMMU_REG_MSI_CFG_TBL: c_uint = 0x0300;

pub const RISCV_IOMMU_REG_SIZE: c_uint = 0x1000;
//
// Chapter 2: Data structures
//
// Device Directory Table macros for non-leaf nodes
//

//
// struct riscv_iommu_dc - Device Context
// @tc: Translation Control
// @iohgatp: I/O Hypervisor guest address translation and protection
// (Second stage context)
// @ta: Translation Attributes
// @fsc: First stage context
// @msiptp: MSI page table pointer
// @msi_addr_mask: MSI address mask
// @msi_addr_pattern: MSI address pattern
// @_reserved: Reserved for future use, padding
//
// This structure is used for leaf nodes on the Device Directory Table,
// in case RISCV_IOMMU_CAPABILITIES_MSI_FLAT is not set, the bottom 4 fields
// are not present and are skipped with pointer arithmetic to avoid
// casting, check out riscv_iommu_get_dc().
// See section 2.1 for more details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_iommu_dc {
    pub tc: u64,
    pub iohgatp: u64,
    pub ta: u64,
    pub fsc: u64,
    pub msiptp: u64,
    pub msi_addr_mask: u64,
    pub msi_addr_pattern: u64,
    pub _reserved: u64,
}

// Translation control fields

// Second-stage (aka G-stage) context fields

//
// enum riscv_iommu_dc_iohgatp_modes - Guest address translation/protection modes
// @RISCV_IOMMU_DC_IOHGATP_MODE_BARE: No translation/protection
// @RISCV_IOMMU_DC_IOHGATP_MODE_SV32X4: Sv32x4 (2-bit extension of Sv32), when fctl.GXL == 1
// @RISCV_IOMMU_DC_IOHGATP_MODE_SV39X4: Sv39x4 (2-bit extension of Sv39), when fctl.GXL == 0
// @RISCV_IOMMU_DC_IOHGATP_MODE_SV48X4: Sv48x4 (2-bit extension of Sv48), when fctl.GXL == 0
// @RISCV_IOMMU_DC_IOHGATP_MODE_SV57X4: Sv57x4 (2-bit extension of Sv57), when fctl.GXL == 0
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum riscv_iommu_dc_iohgatp_modes {
    RISCV_IOMMU_DC_IOHGATP_MODE_BARE = 0,
    RISCV_IOMMU_DC_IOHGATP_MODE_SV32X4 = 8,
    RISCV_IOMMU_DC_IOHGATP_MODE_SV39X4 = 8,
    RISCV_IOMMU_DC_IOHGATP_MODE_SV48X4 = 9,
    RISCV_IOMMU_DC_IOHGATP_MODE_SV57X4 = 10
}

// Translation attributes fields

// First-stage context fields

//
// enum riscv_iommu_dc_fsc_atp_modes - First stage address translation/protection modes
// @RISCV_IOMMU_DC_FSC_MODE_BARE: No translation/protection
// @RISCV_IOMMU_DC_FSC_IOSATP_MODE_SV32: Sv32, when dc.tc.SXL == 1
// @RISCV_IOMMU_DC_FSC_IOSATP_MODE_SV39: Sv39, when dc.tc.SXL == 0
// @RISCV_IOMMU_DC_FSC_IOSATP_MODE_SV48: Sv48, when dc.tc.SXL == 0
// @RISCV_IOMMU_DC_FSC_IOSATP_MODE_SV57: Sv57, when dc.tc.SXL == 0
// @RISCV_IOMMU_DC_FSC_PDTP_MODE_PD8: 1lvl PDT, 8bit process ids
// @RISCV_IOMMU_DC_FSC_PDTP_MODE_PD17: 2lvl PDT, 17bit process ids
// @RISCV_IOMMU_DC_FSC_PDTP_MODE_PD20: 3lvl PDT, 20bit process ids
//
// FSC holds IOSATP when RISCV_IOMMU_DC_TC_PDTV is 0 and PDTP otherwise.
// IOSATP controls the first stage address translation (same as the satp register on
// the RISC-V MMU), and PDTP holds the process directory table, used to select a
// first stage page table based on a process id (for devices that support multiple
// process ids).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum riscv_iommu_dc_fsc_atp_modes {
    RISCV_IOMMU_DC_FSC_MODE_BARE = 0,
    RISCV_IOMMU_DC_FSC_IOSATP_MODE_SV32 = 8,
    RISCV_IOMMU_DC_FSC_IOSATP_MODE_SV39 = 8,
    RISCV_IOMMU_DC_FSC_IOSATP_MODE_SV48 = 9,
    RISCV_IOMMU_DC_FSC_IOSATP_MODE_SV57 = 10,
    RISCV_IOMMU_DC_FSC_PDTP_MODE_PD8 = 1,
    RISCV_IOMMU_DC_FSC_PDTP_MODE_PD17 = 2,
    RISCV_IOMMU_DC_FSC_PDTP_MODE_PD20 = 3
}

// MSI page table pointer

pub const RISCV_IOMMU_DC_MSIPTP_MODE_OFF: c_int = 0;
pub const RISCV_IOMMU_DC_MSIPTP_MODE_FLAT: c_int = 1;
// MSI address mask

// MSI address pattern

//
// struct riscv_iommu_pc - Process Context
// @ta: Translation Attributes
// @fsc: First stage context
//
// This structure is used for leaf nodes on the Process Directory Table
// See section 2.3 for more details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_iommu_pc {
    pub ta: u64,
    pub fsc: u64,
}

// Translation attributes fields

// First stage context fields

//
// Chapter 3: In-memory queue interface
//
// struct riscv_iommu_command - Generic IOMMU command structure
// @dword0: Includes the opcode and the function identifier
// @dword1: Opcode specific data
//
// The commands are interpreted as two 64bit fields, where the first
// 7bits of the first field are the opcode which also defines the
// command's format, followed by a 3bit field that specifies the
// function invoked by that command, and the rest is opcode-specific.
// This is a generic struct which will be populated differently
// according to each command. For more infos on the commands and
// the command queue check section 3.1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_iommu_command {
    pub dword0: u64,
    pub dword1: u64,
}

// Fields on dword0, common for all commands

// 3.1.1 IOMMU Page-table cache invalidation
// Fields on dword0
pub const RISCV_IOMMU_CMD_IOTINVAL_OPCODE: c_int = 1;
pub const RISCV_IOMMU_CMD_IOTINVAL_FUNC_VMA: c_int = 0;
pub const RISCV_IOMMU_CMD_IOTINVAL_FUNC_GVMA: c_int = 1;

// dword1[61:10] is the 4K-aligned page address

// 3.1.2 IOMMU Command Queue Fences
// Fields on dword0
pub const RISCV_IOMMU_CMD_IOFENCE_OPCODE: c_int = 2;
pub const RISCV_IOMMU_CMD_IOFENCE_FUNC_C: c_int = 0;

// dword1 is the address, word-size aligned and shifted to the right by two bits.
// 3.1.3 IOMMU Directory cache invalidation
// Fields on dword0
pub const RISCV_IOMMU_CMD_IODIR_OPCODE: c_int = 3;
pub const RISCV_IOMMU_CMD_IODIR_FUNC_INVAL_DDT: c_int = 0;
pub const RISCV_IOMMU_CMD_IODIR_FUNC_INVAL_PDT: c_int = 1;

// dword1 is reserved for standard use
// 3.1.4 IOMMU PCIe ATS
// Fields on dword0
pub const RISCV_IOMMU_CMD_ATS_OPCODE: c_int = 4;
pub const RISCV_IOMMU_CMD_ATS_FUNC_INVAL: c_int = 0;
pub const RISCV_IOMMU_CMD_ATS_FUNC_PRGR: c_int = 1;

// dword1 is the ATS payload, two different payload types for INVAL and PRGR
// ATS.INVAL payload

// Bits 1 - 10 are zeroed

// ATS.PRGR payload
// Bits 0 - 31 are zeroed

// Bits 41 - 43 are zeroed

//
// struct riscv_iommu_fq_record - Fault/Event Queue Record
// @hdr: Header, includes fault/event cause, PID/DID, transaction type etc
// @_reserved: Low 32bits for custom use, high 32bits for standard use
// @iotval: Transaction-type/cause specific format
// @iotval2: Cause specific format
//
// The fault/event queue reports events and failures raised when
// processing transactions. Each record is a 32byte structure where
// the first dword has a fixed format for providing generic infos
// regarding the fault/event, and two more dwords are there for
// fault/event-specific information. For more details see section
// 3.2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_iommu_fq_record {
    pub hdr: u64,
    pub _reserved: u64,
    pub iotval: u64,
    pub iotval2: u64,
}

// Fields on header

//
// enum riscv_iommu_fq_causes - Fault/event cause values
// @RISCV_IOMMU_FQ_CAUSE_INST_FAULT: Instruction access fault
// @RISCV_IOMMU_FQ_CAUSE_RD_ADDR_MISALIGNED: Read address misaligned
// @RISCV_IOMMU_FQ_CAUSE_RD_FAULT: Read load fault
// @RISCV_IOMMU_FQ_CAUSE_WR_ADDR_MISALIGNED: Write/AMO address misaligned
// @RISCV_IOMMU_FQ_CAUSE_WR_FAULT: Write/AMO access fault
// @RISCV_IOMMU_FQ_CAUSE_INST_FAULT_S: Instruction page fault
// @RISCV_IOMMU_FQ_CAUSE_RD_FAULT_S: Read page fault
// @RISCV_IOMMU_FQ_CAUSE_WR_FAULT_S: Write/AMO page fault
// @RISCV_IOMMU_FQ_CAUSE_INST_FAULT_VS: Instruction guest page fault
// @RISCV_IOMMU_FQ_CAUSE_RD_FAULT_VS: Read guest page fault
// @RISCV_IOMMU_FQ_CAUSE_WR_FAULT_VS: Write/AMO guest page fault
// @RISCV_IOMMU_FQ_CAUSE_DMA_DISABLED: All inbound transactions disallowed
// @RISCV_IOMMU_FQ_CAUSE_DDT_LOAD_FAULT: DDT entry load access fault
// @RISCV_IOMMU_FQ_CAUSE_DDT_INVALID: DDT entry invalid
// @RISCV_IOMMU_FQ_CAUSE_DDT_MISCONFIGURED: DDT entry misconfigured
// @RISCV_IOMMU_FQ_CAUSE_TTYP_BLOCKED: Transaction type disallowed
// @RISCV_IOMMU_FQ_CAUSE_MSI_LOAD_FAULT: MSI PTE load access fault
// @RISCV_IOMMU_FQ_CAUSE_MSI_INVALID: MSI PTE invalid
// @RISCV_IOMMU_FQ_CAUSE_MSI_MISCONFIGURED: MSI PTE misconfigured
// @RISCV_IOMMU_FQ_CAUSE_MRIF_FAULT: MRIF access fault
// @RISCV_IOMMU_FQ_CAUSE_PDT_LOAD_FAULT: PDT entry load access fault
// @RISCV_IOMMU_FQ_CAUSE_PDT_INVALID: PDT entry invalid
// @RISCV_IOMMU_FQ_CAUSE_PDT_MISCONFIGURED: PDT entry misconfigured
// @RISCV_IOMMU_FQ_CAUSE_DDT_CORRUPTED: DDT data corruption
// @RISCV_IOMMU_FQ_CAUSE_PDT_CORRUPTED: PDT data corruption
// @RISCV_IOMMU_FQ_CAUSE_MSI_PT_CORRUPTED: MSI page table data corruption
// @RISCV_IOMMU_FQ_CAUSE_MRIF_CORRUIPTED: MRIF data corruption
// @RISCV_IOMMU_FQ_CAUSE_INTERNAL_DP_ERROR: Internal data path error
// @RISCV_IOMMU_FQ_CAUSE_MSI_WR_FAULT: IOMMU MSI write access fault
// @RISCV_IOMMU_FQ_CAUSE_PT_CORRUPTED: First/second stage page table data corruption
//
// Values are on table 11 of the spec, encodings 275 - 2047 are reserved for standard
// use, and 2048 - 4095 for custom use.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum riscv_iommu_fq_causes {
    RISCV_IOMMU_FQ_CAUSE_INST_FAULT = 1,
    RISCV_IOMMU_FQ_CAUSE_RD_ADDR_MISALIGNED = 4,
    RISCV_IOMMU_FQ_CAUSE_RD_FAULT = 5,
    RISCV_IOMMU_FQ_CAUSE_WR_ADDR_MISALIGNED = 6,
    RISCV_IOMMU_FQ_CAUSE_WR_FAULT = 7,
    RISCV_IOMMU_FQ_CAUSE_INST_FAULT_S = 12,
    RISCV_IOMMU_FQ_CAUSE_RD_FAULT_S = 13,
    RISCV_IOMMU_FQ_CAUSE_WR_FAULT_S = 15,
    RISCV_IOMMU_FQ_CAUSE_INST_FAULT_VS = 20,
    RISCV_IOMMU_FQ_CAUSE_RD_FAULT_VS = 21,
    RISCV_IOMMU_FQ_CAUSE_WR_FAULT_VS = 23,
    RISCV_IOMMU_FQ_CAUSE_DMA_DISABLED = 256,
    RISCV_IOMMU_FQ_CAUSE_DDT_LOAD_FAULT = 257,
    RISCV_IOMMU_FQ_CAUSE_DDT_INVALID = 258,
    RISCV_IOMMU_FQ_CAUSE_DDT_MISCONFIGURED = 259,
    RISCV_IOMMU_FQ_CAUSE_TTYP_BLOCKED = 260,
    RISCV_IOMMU_FQ_CAUSE_MSI_LOAD_FAULT = 261,
    RISCV_IOMMU_FQ_CAUSE_MSI_INVALID = 262,
    RISCV_IOMMU_FQ_CAUSE_MSI_MISCONFIGURED = 263,
    RISCV_IOMMU_FQ_CAUSE_MRIF_FAULT = 264,
    RISCV_IOMMU_FQ_CAUSE_PDT_LOAD_FAULT = 265,
    RISCV_IOMMU_FQ_CAUSE_PDT_INVALID = 266,
    RISCV_IOMMU_FQ_CAUSE_PDT_MISCONFIGURED = 267,
    RISCV_IOMMU_FQ_CAUSE_DDT_CORRUPTED = 268,
    RISCV_IOMMU_FQ_CAUSE_PDT_CORRUPTED = 269,
    RISCV_IOMMU_FQ_CAUSE_MSI_PT_CORRUPTED = 270,
    RISCV_IOMMU_FQ_CAUSE_MRIF_CORRUIPTED = 271,
    RISCV_IOMMU_FQ_CAUSE_INTERNAL_DP_ERROR = 272,
    RISCV_IOMMU_FQ_CAUSE_MSI_WR_FAULT = 273,
    RISCV_IOMMU_FQ_CAUSE_PT_CORRUPTED = 274
}

//
// enum riscv_iommu_fq_ttypes: Fault/event transaction types
// @RISCV_IOMMU_FQ_TTYP_NONE: None. Fault not caused by an inbound transaction.
// @RISCV_IOMMU_FQ_TTYP_UADDR_INST_FETCH: Instruction fetch from untranslated address
// @RISCV_IOMMU_FQ_TTYP_UADDR_RD: Read from untranslated address
// @RISCV_IOMMU_FQ_TTYP_UADDR_WR: Write/AMO to untranslated address
// @RISCV_IOMMU_FQ_TTYP_TADDR_INST_FETCH: Instruction fetch from translated address
// @RISCV_IOMMU_FQ_TTYP_TADDR_RD: Read from translated address
// @RISCV_IOMMU_FQ_TTYP_TADDR_WR: Write/AMO to translated address
// @RISCV_IOMMU_FQ_TTYP_PCIE_ATS_REQ: PCIe ATS translation request
// @RISCV_IOMMU_FQ_TTYP_PCIE_MSG_REQ: PCIe message request
//
// Values are on table 12 of the spec, type 4 and 10 - 31 are reserved for standard use
// and 31 - 63 for custom use.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum riscv_iommu_fq_ttypes {
    RISCV_IOMMU_FQ_TTYP_NONE = 0,
    RISCV_IOMMU_FQ_TTYP_UADDR_INST_FETCH = 1,
    RISCV_IOMMU_FQ_TTYP_UADDR_RD = 2,
    RISCV_IOMMU_FQ_TTYP_UADDR_WR = 3,
    RISCV_IOMMU_FQ_TTYP_TADDR_INST_FETCH = 5,
    RISCV_IOMMU_FQ_TTYP_TADDR_RD = 6,
    RISCV_IOMMU_FQ_TTYP_TADDR_WR = 7,
    RISCV_IOMMU_FQ_TTYP_PCIE_ATS_REQ = 8,
    RISCV_IOMMU_FQ_TTYP_PCIE_MSG_REQ = 9,
}

//
// struct riscv_iommu_pq_record - PCIe Page Request record
// @hdr: Header, includes PID, DID etc
// @payload: Holds the page address, request group and permission bits
//
// For more infos on the PCIe Page Request queue see chapter 3.3.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_iommu_pq_record {
    pub hdr: u64,
    pub payload: u64,
}

// Header fields

// Payload fields

//
// struct riscv_iommu_msipte - MSI Page Table Entry
// @pte: MSI PTE
// @mrif_info: Memory-resident interrupt file info
//
// The MSI Page Table is used for virtualizing MSIs, so that when
// a device sends an MSI to a guest, the IOMMU can reroute it
// by translating the MSI address, either to a guest interrupt file
// or a memory resident interrupt file (MRIF). Note that this page table
// is an array of MSI PTEs, not a multi-level pt, each entry
// is a leaf entry. For more infos check out the AIA spec, chapter 9.5.
//
// Also in basic mode the mrif_info field is ignored by the IOMMU and can
// be used by software, any other reserved fields on pte must be zeroed-out
// by software.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_iommu_msipte {
    pub pte: u64,
    pub mrif_info: u64,
}

// Fields on pte

// Fields on mrif_info

// Helper functions: command structure builders.
//
// Set NAPOT-encoded address for range invalidation (S=1).
// sz_lg2: log2 of total range in bytes, must be >= 13 (8KiB, 2 pages).
// addr must be naturally aligned to 2^sz_lg2.
//
