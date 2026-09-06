//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl_ifc.h
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
// Freescale Integrated Flash Controller
//
// Copyright 2011 Freescale Semiconductor, Inc
//
// Author: Dipen Dudhat <dipen.dudhat@freescale.com>
//

//
// The actual number of banks implemented depends on the IFC version
// - IFC version 1.0 implements 4 banks.
// - IFC version 1.1 onward implements 8 banks.
//
pub const FSL_IFC_BANK_COUNT: c_int = 8;
pub const FSL_IFC_VERSION_MASK: c_uint = 0x0F0F0000;
pub const FSL_IFC_VERSION_1_0_0: c_uint = 0x01000000;
pub const FSL_IFC_VERSION_1_1_0: c_uint = 0x01010000;
pub const FSL_IFC_VERSION_2_0_0: c_uint = 0x02000000;

//
// CSPR - Chip Select Property Register
//
pub const CSPR_BA: c_uint = 0xFFFF0000;
pub const CSPR_BA_SHIFT: c_int = 16;
pub const CSPR_PORT_SIZE: c_uint = 0x00000180;
pub const CSPR_PORT_SIZE_SHIFT: c_int = 7;
// Port Size 8 bit
pub const CSPR_PORT_SIZE_8: c_uint = 0x00000080;
// Port Size 16 bit
pub const CSPR_PORT_SIZE_16: c_uint = 0x00000100;
// Port Size 32 bit
pub const CSPR_PORT_SIZE_32: c_uint = 0x00000180;
// Write Protect
pub const CSPR_WP: c_uint = 0x00000040;
pub const CSPR_WP_SHIFT: c_int = 6;
// Machine Select
pub const CSPR_MSEL: c_uint = 0x00000006;
pub const CSPR_MSEL_SHIFT: c_int = 1;
// NOR
pub const CSPR_MSEL_NOR: c_uint = 0x00000000;
// NAND
pub const CSPR_MSEL_NAND: c_uint = 0x00000002;
// GPCM
pub const CSPR_MSEL_GPCM: c_uint = 0x00000004;
// Bank Valid
pub const CSPR_V: c_uint = 0x00000001;
pub const CSPR_V_SHIFT: c_int = 0;
//
// Address Mask Register
//
pub const IFC_AMASK_MASK: c_uint = 0xFFFF0000;
pub const IFC_AMASK_SHIFT: c_int = 16;

//
// Chip Select Option Register IFC_NAND Machine
//
// Enable ECC Encoder
pub const CSOR_NAND_ECC_ENC_EN: c_uint = 0x80000000;
pub const CSOR_NAND_ECC_MODE_MASK: c_uint = 0x30000000;
// 4 bit correction per 520 Byte sector
pub const CSOR_NAND_ECC_MODE_4: c_uint = 0x00000000;
// 8 bit correction per 528 Byte sector
pub const CSOR_NAND_ECC_MODE_8: c_uint = 0x10000000;
// Enable ECC Decoder
pub const CSOR_NAND_ECC_DEC_EN: c_uint = 0x04000000;
// Row Address Length
pub const CSOR_NAND_RAL_MASK: c_uint = 0x01800000;
pub const CSOR_NAND_RAL_SHIFT: c_int = 20;
pub const CSOR_NAND_RAL_1: c_uint = 0x00000000;
pub const CSOR_NAND_RAL_2: c_uint = 0x00800000;
pub const CSOR_NAND_RAL_3: c_uint = 0x01000000;
pub const CSOR_NAND_RAL_4: c_uint = 0x01800000;
// Page Size 512b, 2k, 4k
pub const CSOR_NAND_PGS_MASK: c_uint = 0x00180000;
pub const CSOR_NAND_PGS_SHIFT: c_int = 16;
pub const CSOR_NAND_PGS_512: c_uint = 0x00000000;
pub const CSOR_NAND_PGS_2K: c_uint = 0x00080000;
pub const CSOR_NAND_PGS_4K: c_uint = 0x00100000;
pub const CSOR_NAND_PGS_8K: c_uint = 0x00180000;
// Spare region Size
pub const CSOR_NAND_SPRZ_MASK: c_uint = 0x0000E000;
pub const CSOR_NAND_SPRZ_SHIFT: c_int = 13;
pub const CSOR_NAND_SPRZ_16: c_uint = 0x00000000;
pub const CSOR_NAND_SPRZ_64: c_uint = 0x00002000;
pub const CSOR_NAND_SPRZ_128: c_uint = 0x00004000;
pub const CSOR_NAND_SPRZ_210: c_uint = 0x00006000;
pub const CSOR_NAND_SPRZ_218: c_uint = 0x00008000;
pub const CSOR_NAND_SPRZ_224: c_uint = 0x0000A000;
pub const CSOR_NAND_SPRZ_CSOR_EXT: c_uint = 0x0000C000;
// Pages Per Block
pub const CSOR_NAND_PB_MASK: c_uint = 0x00000700;
pub const CSOR_NAND_PB_SHIFT: c_int = 8;

// Time for Read Enable High to Output High Impedance
pub const CSOR_NAND_TRHZ_MASK: c_uint = 0x0000001C;
pub const CSOR_NAND_TRHZ_SHIFT: c_int = 2;
pub const CSOR_NAND_TRHZ_20: c_uint = 0x00000000;
pub const CSOR_NAND_TRHZ_40: c_uint = 0x00000004;
pub const CSOR_NAND_TRHZ_60: c_uint = 0x00000008;
pub const CSOR_NAND_TRHZ_80: c_uint = 0x0000000C;
pub const CSOR_NAND_TRHZ_100: c_uint = 0x00000010;
// Buffer control disable
pub const CSOR_NAND_BCTLD: c_uint = 0x00000001;
//
// Chip Select Option Register - NOR Flash Mode
//
// Enable Address shift Mode
pub const CSOR_NOR_ADM_SHFT_MODE_EN: c_uint = 0x80000000;
// Page Read Enable from NOR device
pub const CSOR_NOR_PGRD_EN: c_uint = 0x10000000;
// AVD Toggle Enable during Burst Program
pub const CSOR_NOR_AVD_TGL_PGM_EN: c_uint = 0x01000000;
// Address Data Multiplexing Shift
pub const CSOR_NOR_ADM_MASK: c_uint = 0x0003E000;
pub const CSOR_NOR_ADM_SHIFT_SHIFT: c_int = 13;

// Type of the NOR device hooked
pub const CSOR_NOR_NOR_MODE_AYSNC_NOR: c_uint = 0x00000000;
pub const CSOR_NOR_NOR_MODE_AVD_NOR: c_uint = 0x00000020;
// Time for Read Enable High to Output High Impedance
pub const CSOR_NOR_TRHZ_MASK: c_uint = 0x0000001C;
pub const CSOR_NOR_TRHZ_SHIFT: c_int = 2;
pub const CSOR_NOR_TRHZ_20: c_uint = 0x00000000;
pub const CSOR_NOR_TRHZ_40: c_uint = 0x00000004;
pub const CSOR_NOR_TRHZ_60: c_uint = 0x00000008;
pub const CSOR_NOR_TRHZ_80: c_uint = 0x0000000C;
pub const CSOR_NOR_TRHZ_100: c_uint = 0x00000010;
// Buffer control disable
pub const CSOR_NOR_BCTLD: c_uint = 0x00000001;
//
// Chip Select Option Register - GPCM Mode
//
// GPCM Mode - Normal
pub const CSOR_GPCM_GPMODE_NORMAL: c_uint = 0x00000000;
// GPCM Mode - GenericASIC
pub const CSOR_GPCM_GPMODE_ASIC: c_uint = 0x80000000;
// Parity Mode odd/even
pub const CSOR_GPCM_PARITY_EVEN: c_uint = 0x40000000;
// Parity Checking enable/disable
pub const CSOR_GPCM_PAR_EN: c_uint = 0x20000000;
// GPCM Timeout Count
pub const CSOR_GPCM_GPTO_MASK: c_uint = 0x0F000000;
pub const CSOR_GPCM_GPTO_SHIFT: c_int = 24;

// GPCM External Access Termination mode for read access
pub const CSOR_GPCM_RGETA_EXT: c_uint = 0x00080000;
// GPCM External Access Termination mode for write access
pub const CSOR_GPCM_WGETA_EXT: c_uint = 0x00040000;
// Address Data Multiplexing Shift
pub const CSOR_GPCM_ADM_MASK: c_uint = 0x0003E000;
pub const CSOR_GPCM_ADM_SHIFT_SHIFT: c_int = 13;

// Generic ASIC Parity error indication delay
pub const CSOR_GPCM_GAPERRD_MASK: c_uint = 0x00000180;
pub const CSOR_GPCM_GAPERRD_SHIFT: c_int = 7;

// Time for Read Enable High to Output High Impedance
pub const CSOR_GPCM_TRHZ_MASK: c_uint = 0x0000001C;
pub const CSOR_GPCM_TRHZ_20: c_uint = 0x00000000;
pub const CSOR_GPCM_TRHZ_40: c_uint = 0x00000004;
pub const CSOR_GPCM_TRHZ_60: c_uint = 0x00000008;
pub const CSOR_GPCM_TRHZ_80: c_uint = 0x0000000C;
pub const CSOR_GPCM_TRHZ_100: c_uint = 0x00000010;
// Buffer control disable
pub const CSOR_GPCM_BCTLD: c_uint = 0x00000001;
//
// Ready Busy Status Register (RB_STAT)
//
// CSn is READY
pub const IFC_RB_STAT_READY_CS0: c_uint = 0x80000000;
pub const IFC_RB_STAT_READY_CS1: c_uint = 0x40000000;
pub const IFC_RB_STAT_READY_CS2: c_uint = 0x20000000;
pub const IFC_RB_STAT_READY_CS3: c_uint = 0x10000000;
//
// General Control Register (GCR)
//
pub const IFC_GCR_MASK: c_uint = 0x8000F800;
// reset all IFC hardware
pub const IFC_GCR_SOFT_RST_ALL: c_uint = 0x80000000;
// Turnaroud Time of external buffer
pub const IFC_GCR_TBCTL_TRN_TIME: c_uint = 0x0000F800;
pub const IFC_GCR_TBCTL_TRN_TIME_SHIFT: c_int = 11;
//
// Common Event and Error Status Register (CM_EVTER_STAT)
//
// Chip select error
pub const IFC_CM_EVTER_STAT_CSER: c_uint = 0x80000000;
//
// Common Event and Error Enable Register (CM_EVTER_EN)
//
// Chip select error checking enable
pub const IFC_CM_EVTER_EN_CSEREN: c_uint = 0x80000000;
//
// Common Event and Error Interrupt Enable Register (CM_EVTER_INTR_EN)
//
// Chip select error interrupt enable
pub const IFC_CM_EVTER_INTR_EN_CSERIREN: c_uint = 0x80000000;
//
// Common Transfer Error Attribute Register-0 (CM_ERATTR0)
//
// transaction type of error Read/Write
pub const IFC_CM_ERATTR0_ERTYP_READ: c_uint = 0x80000000;
pub const IFC_CM_ERATTR0_ERAID: c_uint = 0x0FF00000;
pub const IFC_CM_ERATTR0_ERAID_SHIFT: c_int = 20;
pub const IFC_CM_ERATTR0_ESRCID: c_uint = 0x0000FF00;
pub const IFC_CM_ERATTR0_ESRCID_SHIFT: c_int = 8;
//
// Clock Control Register (CCR)
//
pub const IFC_CCR_MASK: c_uint = 0x0F0F8800;
// Clock division ratio
pub const IFC_CCR_CLK_DIV_MASK: c_uint = 0x0F000000;
pub const IFC_CCR_CLK_DIV_SHIFT: c_int = 24;

// IFC Clock Delay
pub const IFC_CCR_CLK_DLY_MASK: c_uint = 0x000F0000;
pub const IFC_CCR_CLK_DLY_SHIFT: c_int = 16;

// Invert IFC clock before sending out
pub const IFC_CCR_INV_CLK_EN: c_uint = 0x00008000;
// Fedback IFC Clock
pub const IFC_CCR_FB_IFC_CLK_SEL: c_uint = 0x00000800;
//
// Clock Status Register (CSR)
//
// Clk is stable
pub const IFC_CSR_CLK_STAT_STABLE: c_uint = 0x80000000;
//
// IFC_NAND Machine Specific Registers
//
// NAND Configuration Register (NCFGR)
//
// Auto Boot Mode
pub const IFC_NAND_NCFGR_BOOT: c_uint = 0x80000000;
// SRAM Initialization
pub const IFC_NAND_NCFGR_SRAM_INIT_EN: c_uint = 0x20000000;
// Addressing Mode-ROW0+n/COL0
pub const IFC_NAND_NCFGR_ADDR_MODE_RC0: c_uint = 0x00000000;
// Addressing Mode-ROW0+n/COL0+n
pub const IFC_NAND_NCFGR_ADDR_MODE_RC1: c_uint = 0x00400000;
// Number of loop iterations of FIR sequences for multi page operations
pub const IFC_NAND_NCFGR_NUM_LOOP_MASK: c_uint = 0x0000F000;
pub const IFC_NAND_NCFGR_NUM_LOOP_SHIFT: c_int = 12;

// Number of wait cycles
pub const IFC_NAND_NCFGR_NUM_WAIT_MASK: c_uint = 0x000000FF;
pub const IFC_NAND_NCFGR_NUM_WAIT_SHIFT: c_int = 0;
//
// NAND Flash Command Registers (NAND_FCR0/NAND_FCR1)
//
// General purpose FCM flash command bytes CMD0-CMD7
pub const IFC_NAND_FCR0_CMD0: c_uint = 0xFF000000;
pub const IFC_NAND_FCR0_CMD0_SHIFT: c_int = 24;
pub const IFC_NAND_FCR0_CMD1: c_uint = 0x00FF0000;
pub const IFC_NAND_FCR0_CMD1_SHIFT: c_int = 16;
pub const IFC_NAND_FCR0_CMD2: c_uint = 0x0000FF00;
pub const IFC_NAND_FCR0_CMD2_SHIFT: c_int = 8;
pub const IFC_NAND_FCR0_CMD3: c_uint = 0x000000FF;
pub const IFC_NAND_FCR0_CMD3_SHIFT: c_int = 0;
pub const IFC_NAND_FCR1_CMD4: c_uint = 0xFF000000;
pub const IFC_NAND_FCR1_CMD4_SHIFT: c_int = 24;
pub const IFC_NAND_FCR1_CMD5: c_uint = 0x00FF0000;
pub const IFC_NAND_FCR1_CMD5_SHIFT: c_int = 16;
pub const IFC_NAND_FCR1_CMD6: c_uint = 0x0000FF00;
pub const IFC_NAND_FCR1_CMD6_SHIFT: c_int = 8;
pub const IFC_NAND_FCR1_CMD7: c_uint = 0x000000FF;
pub const IFC_NAND_FCR1_CMD7_SHIFT: c_int = 0;
//
// Flash ROW and COL Address Register (ROWn, COLn)
//
// Main/spare region locator
pub const IFC_NAND_COL_MS: c_uint = 0x80000000;
// Column Address
pub const IFC_NAND_COL_CA_MASK: c_uint = 0x00000FFF;
//
// NAND Flash Byte Count Register (NAND_BC)
//
// Byte Count for read/Write
pub const IFC_NAND_BC: c_uint = 0x000001FF;
//
// NAND Flash Instruction Registers (NAND_FIR0/NAND_FIR1/NAND_FIR2)
//
// NAND Machine specific opcodes OP0-OP14
pub const IFC_NAND_FIR0_OP0: c_uint = 0xFC000000;
pub const IFC_NAND_FIR0_OP0_SHIFT: c_int = 26;
pub const IFC_NAND_FIR0_OP1: c_uint = 0x03F00000;
pub const IFC_NAND_FIR0_OP1_SHIFT: c_int = 20;
pub const IFC_NAND_FIR0_OP2: c_uint = 0x000FC000;
pub const IFC_NAND_FIR0_OP2_SHIFT: c_int = 14;
pub const IFC_NAND_FIR0_OP3: c_uint = 0x00003F00;
pub const IFC_NAND_FIR0_OP3_SHIFT: c_int = 8;
pub const IFC_NAND_FIR0_OP4: c_uint = 0x000000FC;
pub const IFC_NAND_FIR0_OP4_SHIFT: c_int = 2;
pub const IFC_NAND_FIR1_OP5: c_uint = 0xFC000000;
pub const IFC_NAND_FIR1_OP5_SHIFT: c_int = 26;
pub const IFC_NAND_FIR1_OP6: c_uint = 0x03F00000;
pub const IFC_NAND_FIR1_OP6_SHIFT: c_int = 20;
pub const IFC_NAND_FIR1_OP7: c_uint = 0x000FC000;
pub const IFC_NAND_FIR1_OP7_SHIFT: c_int = 14;
pub const IFC_NAND_FIR1_OP8: c_uint = 0x00003F00;
pub const IFC_NAND_FIR1_OP8_SHIFT: c_int = 8;
pub const IFC_NAND_FIR1_OP9: c_uint = 0x000000FC;
pub const IFC_NAND_FIR1_OP9_SHIFT: c_int = 2;
pub const IFC_NAND_FIR2_OP10: c_uint = 0xFC000000;
pub const IFC_NAND_FIR2_OP10_SHIFT: c_int = 26;
pub const IFC_NAND_FIR2_OP11: c_uint = 0x03F00000;
pub const IFC_NAND_FIR2_OP11_SHIFT: c_int = 20;
pub const IFC_NAND_FIR2_OP12: c_uint = 0x000FC000;
pub const IFC_NAND_FIR2_OP12_SHIFT: c_int = 14;
pub const IFC_NAND_FIR2_OP13: c_uint = 0x00003F00;
pub const IFC_NAND_FIR2_OP13_SHIFT: c_int = 8;
pub const IFC_NAND_FIR2_OP14: c_uint = 0x000000FC;
pub const IFC_NAND_FIR2_OP14_SHIFT: c_int = 2;
//
// Instruction opcodes to be programmed
// in FIR registers- 6bits
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ifc_nand_fir_opcodes {
    IFC_FIR_OP_NOP,
    IFC_FIR_OP_CA0,
    IFC_FIR_OP_CA1,
    IFC_FIR_OP_CA2,
    IFC_FIR_OP_CA3,
    IFC_FIR_OP_RA0,
    IFC_FIR_OP_RA1,
    IFC_FIR_OP_RA2,
    IFC_FIR_OP_RA3,
    IFC_FIR_OP_CMD0,
    IFC_FIR_OP_CMD1,
    IFC_FIR_OP_CMD2,
    IFC_FIR_OP_CMD3,
    IFC_FIR_OP_CMD4,
    IFC_FIR_OP_CMD5,
    IFC_FIR_OP_CMD6,
    IFC_FIR_OP_CMD7,
    IFC_FIR_OP_CW0,
    IFC_FIR_OP_CW1,
    IFC_FIR_OP_CW2,
    IFC_FIR_OP_CW3,
    IFC_FIR_OP_CW4,
    IFC_FIR_OP_CW5,
    IFC_FIR_OP_CW6,
    IFC_FIR_OP_CW7,
    IFC_FIR_OP_WBCD,
    IFC_FIR_OP_RBCD,
    IFC_FIR_OP_BTRD,
    IFC_FIR_OP_RDSTAT,
    IFC_FIR_OP_NWAIT,
    IFC_FIR_OP_WFR,
    IFC_FIR_OP_SBRD,
    IFC_FIR_OP_UA,
    IFC_FIR_OP_RB,
}

//
// NAND Chip Select Register (NAND_CSEL)
//
pub const IFC_NAND_CSEL: c_uint = 0x0C000000;
pub const IFC_NAND_CSEL_SHIFT: c_int = 26;
pub const IFC_NAND_CSEL_CS0: c_uint = 0x00000000;
pub const IFC_NAND_CSEL_CS1: c_uint = 0x04000000;
pub const IFC_NAND_CSEL_CS2: c_uint = 0x08000000;
pub const IFC_NAND_CSEL_CS3: c_uint = 0x0C000000;
//
// NAND Operation Sequence Start (NANDSEQ_STRT)
//
// NAND Flash Operation Start
pub const IFC_NAND_SEQ_STRT_FIR_STRT: c_uint = 0x80000000;
// Automatic Erase
pub const IFC_NAND_SEQ_STRT_AUTO_ERS: c_uint = 0x00800000;
// Automatic Program
pub const IFC_NAND_SEQ_STRT_AUTO_PGM: c_uint = 0x00100000;
// Automatic Copyback
pub const IFC_NAND_SEQ_STRT_AUTO_CPB: c_uint = 0x00020000;
// Automatic Read Operation
pub const IFC_NAND_SEQ_STRT_AUTO_RD: c_uint = 0x00004000;
// Automatic Status Read
pub const IFC_NAND_SEQ_STRT_AUTO_STAT_RD: c_uint = 0x00000800;
//
// NAND Event and Error Status Register (NAND_EVTER_STAT)
//
// Operation Complete
pub const IFC_NAND_EVTER_STAT_OPC: c_uint = 0x80000000;
// Flash Timeout Error
pub const IFC_NAND_EVTER_STAT_FTOER: c_uint = 0x08000000;
// Write Protect Error
pub const IFC_NAND_EVTER_STAT_WPER: c_uint = 0x04000000;
// ECC Error
pub const IFC_NAND_EVTER_STAT_ECCER: c_uint = 0x02000000;
// RCW Load Done
pub const IFC_NAND_EVTER_STAT_RCW_DN: c_uint = 0x00008000;
// Boot Loadr Done
pub const IFC_NAND_EVTER_STAT_BOOT_DN: c_uint = 0x00004000;
// Bad Block Indicator search select
pub const IFC_NAND_EVTER_STAT_BBI_SRCH_SE: c_uint = 0x00000800;
//
// NAND Flash Page Read Completion Event Status Register
// (PGRDCMPL_EVT_STAT)
//
pub const PGRDCMPL_EVT_STAT_MASK: c_uint = 0xFFFF0000;
// Small Page 0-15 Done

// Large Page(2K) 0-3 Done

// Large Page(4K) 0-1 Done

//
// NAND Event and Error Enable Register (NAND_EVTER_EN)
//
// Operation complete event enable
pub const IFC_NAND_EVTER_EN_OPC_EN: c_uint = 0x80000000;
// Page read complete event enable
pub const IFC_NAND_EVTER_EN_PGRDCMPL_EN: c_uint = 0x20000000;
// Flash Timeout error enable
pub const IFC_NAND_EVTER_EN_FTOER_EN: c_uint = 0x08000000;
// Write Protect error enable
pub const IFC_NAND_EVTER_EN_WPER_EN: c_uint = 0x04000000;
// ECC error logging enable
pub const IFC_NAND_EVTER_EN_ECCER_EN: c_uint = 0x02000000;
//
// NAND Event and Error Interrupt Enable Register (NAND_EVTER_INTR_EN)
//
// Enable interrupt for operation complete
pub const IFC_NAND_EVTER_INTR_OPCIR_EN: c_uint = 0x80000000;
// Enable interrupt for Page read complete
pub const IFC_NAND_EVTER_INTR_PGRDCMPLIR_EN: c_uint = 0x20000000;
// Enable interrupt for Flash timeout error
pub const IFC_NAND_EVTER_INTR_FTOERIR_EN: c_uint = 0x08000000;
// Enable interrupt for Write protect error
pub const IFC_NAND_EVTER_INTR_WPERIR_EN: c_uint = 0x04000000;
// Enable interrupt for ECC error
pub const IFC_NAND_EVTER_INTR_ECCERIR_EN: c_uint = 0x02000000;
//
// NAND Transfer Error Attribute Register-0 (NAND_ERATTR0)
//
pub const IFC_NAND_ERATTR0_MASK: c_uint = 0x0C080000;
// Error on CS0-3 for NAND
pub const IFC_NAND_ERATTR0_ERCS_CS0: c_uint = 0x00000000;
pub const IFC_NAND_ERATTR0_ERCS_CS1: c_uint = 0x04000000;
pub const IFC_NAND_ERATTR0_ERCS_CS2: c_uint = 0x08000000;
pub const IFC_NAND_ERATTR0_ERCS_CS3: c_uint = 0x0C000000;
// Transaction type of error Read/Write
pub const IFC_NAND_ERATTR0_ERTTYPE_READ: c_uint = 0x00080000;
//
// NAND Flash Status Register (NAND_FSR)
//
// First byte of data read from read status op
pub const IFC_NAND_NFSR_RS0: c_uint = 0xFF000000;
// Second byte of data read from read status op
pub const IFC_NAND_NFSR_RS1: c_uint = 0x00FF0000;
//
// ECC Error Status Registers (ECCSTAT0-ECCSTAT3)
//
// Number of ECC errors on sector n (n = 0-15)
pub const IFC_NAND_ECCSTAT0_ERRCNT_SECTOR0_MASK: c_uint = 0x0F000000;
pub const IFC_NAND_ECCSTAT0_ERRCNT_SECTOR0_SHIFT: c_int = 24;
pub const IFC_NAND_ECCSTAT0_ERRCNT_SECTOR1_MASK: c_uint = 0x000F0000;
pub const IFC_NAND_ECCSTAT0_ERRCNT_SECTOR1_SHIFT: c_int = 16;
pub const IFC_NAND_ECCSTAT0_ERRCNT_SECTOR2_MASK: c_uint = 0x00000F00;
pub const IFC_NAND_ECCSTAT0_ERRCNT_SECTOR2_SHIFT: c_int = 8;
pub const IFC_NAND_ECCSTAT0_ERRCNT_SECTOR3_MASK: c_uint = 0x0000000F;
pub const IFC_NAND_ECCSTAT0_ERRCNT_SECTOR3_SHIFT: c_int = 0;
pub const IFC_NAND_ECCSTAT1_ERRCNT_SECTOR4_MASK: c_uint = 0x0F000000;
pub const IFC_NAND_ECCSTAT1_ERRCNT_SECTOR4_SHIFT: c_int = 24;
pub const IFC_NAND_ECCSTAT1_ERRCNT_SECTOR5_MASK: c_uint = 0x000F0000;
pub const IFC_NAND_ECCSTAT1_ERRCNT_SECTOR5_SHIFT: c_int = 16;
pub const IFC_NAND_ECCSTAT1_ERRCNT_SECTOR6_MASK: c_uint = 0x00000F00;
pub const IFC_NAND_ECCSTAT1_ERRCNT_SECTOR6_SHIFT: c_int = 8;
pub const IFC_NAND_ECCSTAT1_ERRCNT_SECTOR7_MASK: c_uint = 0x0000000F;
pub const IFC_NAND_ECCSTAT1_ERRCNT_SECTOR7_SHIFT: c_int = 0;
pub const IFC_NAND_ECCSTAT2_ERRCNT_SECTOR8_MASK: c_uint = 0x0F000000;
pub const IFC_NAND_ECCSTAT2_ERRCNT_SECTOR8_SHIFT: c_int = 24;
pub const IFC_NAND_ECCSTAT2_ERRCNT_SECTOR9_MASK: c_uint = 0x000F0000;
pub const IFC_NAND_ECCSTAT2_ERRCNT_SECTOR9_SHIFT: c_int = 16;
pub const IFC_NAND_ECCSTAT2_ERRCNT_SECTOR10_MASK: c_uint = 0x00000F00;
pub const IFC_NAND_ECCSTAT2_ERRCNT_SECTOR10_SHIFT: c_int = 8;
pub const IFC_NAND_ECCSTAT2_ERRCNT_SECTOR11_MASK: c_uint = 0x0000000F;
pub const IFC_NAND_ECCSTAT2_ERRCNT_SECTOR11_SHIFT: c_int = 0;
pub const IFC_NAND_ECCSTAT3_ERRCNT_SECTOR12_MASK: c_uint = 0x0F000000;
pub const IFC_NAND_ECCSTAT3_ERRCNT_SECTOR12_SHIFT: c_int = 24;
pub const IFC_NAND_ECCSTAT3_ERRCNT_SECTOR13_MASK: c_uint = 0x000F0000;
pub const IFC_NAND_ECCSTAT3_ERRCNT_SECTOR13_SHIFT: c_int = 16;
pub const IFC_NAND_ECCSTAT3_ERRCNT_SECTOR14_MASK: c_uint = 0x00000F00;
pub const IFC_NAND_ECCSTAT3_ERRCNT_SECTOR14_SHIFT: c_int = 8;
pub const IFC_NAND_ECCSTAT3_ERRCNT_SECTOR15_MASK: c_uint = 0x0000000F;
pub const IFC_NAND_ECCSTAT3_ERRCNT_SECTOR15_SHIFT: c_int = 0;
//
// NAND Control Register (NANDCR)
//
pub const IFC_NAND_NCR_FTOCNT_MASK: c_uint = 0x1E000000;
pub const IFC_NAND_NCR_FTOCNT_SHIFT: c_int = 25;

//
// NAND_AUTOBOOT_TRGR
//
// Trigger RCW load
pub const IFC_NAND_AUTOBOOT_TRGR_RCW_LD: c_uint = 0x80000000;
// Trigget Auto Boot
pub const IFC_NAND_AUTOBOOT_TRGR_BOOT_LD: c_uint = 0x20000000;
//
// NAND_MDR
//
// 1st read data byte when opcode SBRD
pub const IFC_NAND_MDR_RDATA0: c_uint = 0xFF000000;
// 2nd read data byte when opcode SBRD
pub const IFC_NAND_MDR_RDATA1: c_uint = 0x00FF0000;
//
// NOR Machine Specific Registers
//
// NOR Event and Error Status Register (NOR_EVTER_STAT)
//
// NOR Command Sequence Operation Complete
pub const IFC_NOR_EVTER_STAT_OPC_NOR: c_uint = 0x80000000;
// Write Protect Error
pub const IFC_NOR_EVTER_STAT_WPER: c_uint = 0x04000000;
// Command Sequence Timeout Error
pub const IFC_NOR_EVTER_STAT_STOER: c_uint = 0x01000000;
//
// NOR Event and Error Enable Register (NOR_EVTER_EN)
//
// NOR Command Seq complete event enable
pub const IFC_NOR_EVTER_EN_OPCEN_NOR: c_uint = 0x80000000;
// Write Protect Error Checking Enable
pub const IFC_NOR_EVTER_EN_WPEREN: c_uint = 0x04000000;
// Timeout Error Enable
pub const IFC_NOR_EVTER_EN_STOEREN: c_uint = 0x01000000;
//
// NOR Event and Error Interrupt Enable Register (NOR_EVTER_INTR_EN)
//
// Enable interrupt for OPC complete
pub const IFC_NOR_EVTER_INTR_OPCEN_NOR: c_uint = 0x80000000;
// Enable interrupt for write protect error
pub const IFC_NOR_EVTER_INTR_WPEREN: c_uint = 0x04000000;
// Enable interrupt for timeout error
pub const IFC_NOR_EVTER_INTR_STOEREN: c_uint = 0x01000000;
//
// NOR Transfer Error Attribute Register-0 (NOR_ERATTR0)
//
// Source ID for error transaction
pub const IFC_NOR_ERATTR0_ERSRCID: c_uint = 0xFF000000;
// AXI ID for error transation
pub const IFC_NOR_ERATTR0_ERAID: c_uint = 0x000FF000;
// Chip select corresponds to NOR error
pub const IFC_NOR_ERATTR0_ERCS_CS0: c_uint = 0x00000000;
pub const IFC_NOR_ERATTR0_ERCS_CS1: c_uint = 0x00000010;
pub const IFC_NOR_ERATTR0_ERCS_CS2: c_uint = 0x00000020;
pub const IFC_NOR_ERATTR0_ERCS_CS3: c_uint = 0x00000030;
// Type of transaction read/write
pub const IFC_NOR_ERATTR0_ERTYPE_READ: c_uint = 0x00000001;
//
// NOR Transfer Error Attribute Register-2 (NOR_ERATTR2)
//
pub const IFC_NOR_ERATTR2_ER_NUM_PHASE_EXP: c_uint = 0x000F0000;
pub const IFC_NOR_ERATTR2_ER_NUM_PHASE_PER: c_uint = 0x00000F00;
//
// NOR Control Register (NORCR)
//
pub const IFC_NORCR_MASK: c_uint = 0x0F0F0000;
// No. of Address/Data Phase
pub const IFC_NORCR_NUM_PHASE_MASK: c_uint = 0x0F000000;
pub const IFC_NORCR_NUM_PHASE_SHIFT: c_int = 24;

// Sequence Timeout Count
pub const IFC_NORCR_STOCNT_MASK: c_uint = 0x000F0000;
pub const IFC_NORCR_STOCNT_SHIFT: c_int = 16;

//
// GPCM Machine specific registers
//
// GPCM Event and Error Status Register (GPCM_EVTER_STAT)
//
// Timeout error
pub const IFC_GPCM_EVTER_STAT_TOER: c_uint = 0x04000000;
// Parity error
pub const IFC_GPCM_EVTER_STAT_PER: c_uint = 0x01000000;
//
// GPCM Event and Error Enable Register (GPCM_EVTER_EN)
//
// Timeout error enable
pub const IFC_GPCM_EVTER_EN_TOER_EN: c_uint = 0x04000000;
// Parity error enable
pub const IFC_GPCM_EVTER_EN_PER_EN: c_uint = 0x01000000;
//
// GPCM Event and Error Interrupt Enable Register (GPCM_EVTER_INTR_EN)
//
// Enable Interrupt for timeout error
pub const IFC_GPCM_EEIER_TOERIR_EN: c_uint = 0x04000000;
// Enable Interrupt for Parity error
pub const IFC_GPCM_EEIER_PERIR_EN: c_uint = 0x01000000;
//
// GPCM Transfer Error Attribute Register-0 (GPCM_ERATTR0)
//
// Source ID for error transaction
pub const IFC_GPCM_ERATTR0_ERSRCID: c_uint = 0xFF000000;
// AXI ID for error transaction
pub const IFC_GPCM_ERATTR0_ERAID: c_uint = 0x000FF000;
// Chip select corresponds to GPCM error
pub const IFC_GPCM_ERATTR0_ERCS_CS0: c_uint = 0x00000000;
pub const IFC_GPCM_ERATTR0_ERCS_CS1: c_uint = 0x00000040;
pub const IFC_GPCM_ERATTR0_ERCS_CS2: c_uint = 0x00000080;
pub const IFC_GPCM_ERATTR0_ERCS_CS3: c_uint = 0x000000C0;
// Type of transaction read/Write
pub const IFC_GPCM_ERATTR0_ERTYPE_READ: c_uint = 0x00000001;
//
// GPCM Transfer Error Attribute Register-2 (GPCM_ERATTR2)
//
// On which beat of address/data parity error is observed
pub const IFC_GPCM_ERATTR2_PERR_BEAT: c_uint = 0x00000C00;
// Parity Error on byte
pub const IFC_GPCM_ERATTR2_PERR_BYTE: c_uint = 0x000000F0;
// Parity Error reported in addr or data phase
pub const IFC_GPCM_ERATTR2_PERR_DATA_PHASE: c_uint = 0x00000001;
//
// GPCM Status Register (GPCM_STAT)
//
pub const IFC_GPCM_STAT_BSY: c_uint = 0x80000000  /* GPCM is busy */;
//
// IFC Controller NAND Machine registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ifc_nand {
    pub ncfgr: __be32,
    pub res1: [u32; 0x4],
    pub nand_fcr0: __be32,
    pub nand_fcr1: __be32,
    pub res2: [u32; 0x8],
    pub row0: __be32,
    pub res3: u32,
    pub col0: __be32,
    pub res4: u32,
    pub row1: __be32,
    pub res5: u32,
    pub col1: __be32,
    pub res6: u32,
    pub row2: __be32,
    pub res7: u32,
    pub col2: __be32,
    pub res8: u32,
    pub row3: __be32,
    pub res9: u32,
    pub col3: __be32,
    pub res10: [u32; 0x24],
    pub nand_fbcr: __be32,
    pub res11: u32,
    pub nand_fir0: __be32,
    pub nand_fir1: __be32,
    pub nand_fir2: __be32,
    pub res12: [u32; 0x10],
    pub nand_csel: __be32,
    pub res13: u32,
    pub nandseq_strt: __be32,
    pub res14: u32,
    pub nand_evter_stat: __be32,
    pub res15: u32,
    pub pgrdcmpl_evt_stat: __be32,
    pub res16: [u32; 0x2],
    pub nand_evter_en: __be32,
    pub res17: [u32; 0x2],
    pub nand_evter_intr_en: __be32,
    pub nand_vol_addr_stat: __be32,
    pub res18: u32,
    pub nand_erattr0: __be32,
    pub nand_erattr1: __be32,
    pub res19: [u32; 0x10],
    pub nand_fsr: __be32,
    pub res20: u32,
    pub nand_eccstat: [__be32; 8],
    pub res21: [u32; 0x1c],
    pub nanndcr: __be32,
    pub res22: [u32; 0x2],
    pub nand_autoboot_trgr: __be32,
    pub res23: u32,
    pub nand_mdr: __be32,
    pub res24: [u32; 0x1C],
    pub nand_dll_lowcfg0: __be32,
    pub nand_dll_lowcfg1: __be32,
    pub res25: u32,
    pub nand_dll_lowstat: __be32,
    pub res26: [u32; 0x3c],
}

//
// IFC controller NOR Machine registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ifc_nor {
    pub nor_evter_stat: __be32,
    pub res1: [u32; 0x2],
    pub nor_evter_en: __be32,
    pub res2: [u32; 0x2],
    pub nor_evter_intr_en: __be32,
    pub res3: [u32; 0x2],
    pub nor_erattr0: __be32,
    pub nor_erattr1: __be32,
    pub nor_erattr2: __be32,
    pub res4: [u32; 0x4],
    pub norcr: __be32,
    pub res5: [u32; 0xEF],
}

//
// IFC controller GPCM Machine registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ifc_gpcm {
    pub gpcm_evter_stat: __be32,
    pub res1: [u32; 0x2],
    pub gpcm_evter_en: __be32,
    pub res2: [u32; 0x2],
    pub gpcm_evter_intr_en: __be32,
    pub res3: [u32; 0x2],
    pub gpcm_erattr0: __be32,
    pub gpcm_erattr1: __be32,
    pub gpcm_erattr2: __be32,
    pub gpcm_stat: __be32,
}

//
// IFC Controller Registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ifc_global {
    pub ifc_rev: __be32,
    pub res1: [u32; 0x2],
    pub cspr_ext: __be32,
    pub cspr: __be32,
    pub res2: u32,
    pub cspr_cs: [}; FSL_IFC_BANK_COUNT],
    pub res3: [u32; 0xd],
    pub amask: __be32,
    pub res4: [u32; 0x2],
    pub amask_cs: [}; FSL_IFC_BANK_COUNT],
    pub res5: [u32; 0xc],
    pub csor: __be32,
    pub csor_ext: __be32,
    pub res6: u32,
    pub csor_cs: [}; FSL_IFC_BANK_COUNT],
    pub res7: [u32; 0xc],
    pub ftim: [__be32; 4],
    pub res8: [u32; 0x8],
    pub ftim_cs: [}; FSL_IFC_BANK_COUNT],
    pub res9: [u32; 0x30],
    pub rb_stat: __be32,
    pub rb_map: __be32,
    pub wb_map: __be32,
    pub ifc_gcr: __be32,
    pub res10: [u32; 0x2],
    pub cm_evter_stat: __be32,
    pub res11: [u32; 0x2],
    pub cm_evter_en: __be32,
    pub res12: [u32; 0x2],
    pub cm_evter_intr_en: __be32,
    pub res13: [u32; 0x2],
    pub cm_erattr0: __be32,
    pub cm_erattr1: __be32,
    pub res14: [u32; 0x2],
    pub ifc_ccr: __be32,
    pub ifc_csr: __be32,
    pub ddr_ccr_low: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ifc_runtime {
    pub ifc_nand: fsl_ifc_nand,
    pub ifc_nor: fsl_ifc_nor,
    pub ifc_gpcm: fsl_ifc_gpcm,
}

extern "C" {
    pub fn convert_ifc_address(addr_base: phys_addr_t) -> c_uint;
}
extern "C" {
    pub fn fsl_ifc_find(addr_base: phys_addr_t) -> c_int;
}
// overview of the fsl ifc controller
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ifc_ctrl {
// device info
    pub dev: *mut device,
    pub gregs: *mut fsl_ifc_global __iomem,
    pub rregs: *mut fsl_ifc_runtime __iomem,
    pub irq: c_int,
    pub nand_irq: c_int,
    pub lock: spinlock_t,
    pub nand: *mut c_void,
    pub version: c_int,
    pub banks: c_int,
    pub nand_stat: u32,
    pub nand_wait: wait_queue_head_t,
    pub little_endian: bool,
}

extern "C" {
    pub fn ioread8(_arg: addr) -> return;
}
