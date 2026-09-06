//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_gen6_ras.h
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
// Copyright(c) 2025 Intel Corporation

// Error source registers
pub const ADF_GEN6_ERRSOU0: c_uint = 0x41A200;
pub const ADF_GEN6_ERRSOU1: c_uint = 0x41A204;
pub const ADF_GEN6_ERRSOU2: c_uint = 0x41A208;
pub const ADF_GEN6_ERRSOU3: c_uint = 0x41A20C;
// Error source mask registers
pub const ADF_GEN6_ERRMSK0: c_uint = 0x41A210;
pub const ADF_GEN6_ERRMSK1: c_uint = 0x41A214;
pub const ADF_GEN6_ERRMSK2: c_uint = 0x41A218;
pub const ADF_GEN6_ERRMSK3: c_uint = 0x41A21C;
// ERRSOU0 Correctable error mask

// HI AE Uncorrectable error log
pub const ADF_GEN6_HIAEUNCERRLOG_CPP0: c_uint = 0x41A300;
// HI AE Uncorrectable error log enable
pub const ADF_GEN6_HIAEUNCERRLOGENABLE_CPP0: c_uint = 0x41A320;
// HI AE Correctable error log
pub const ADF_GEN6_HIAECORERRLOG_CPP0: c_uint = 0x41A308;
// HI AE Correctable error log enable
pub const ADF_GEN6_HIAECORERRLOGENABLE_CPP0: c_uint = 0x41A318;
// HI CPP Agent Command parity error log
pub const ADF_GEN6_HICPPAGENTCMDPARERRLOG: c_uint = 0x41A310;
// HI CPP Agent command parity error logging enable
pub const ADF_GEN6_HICPPAGENTCMDPARERRLOGENABLE: c_uint = 0x41A314;
pub const ADF_6XXX_HICPPAGENTCMDPARERRLOG_MASK: c_uint = 0x1B;
// RI Memory parity error status register
pub const ADF_GEN6_RIMEM_PARERR_STS: c_uint = 0x41B128;
// RI Memory parity error reporting enable
pub const ADF_GEN6_RI_MEM_PAR_ERR_EN0: c_uint = 0x41B12C;
//
// RI Memory parity error mask
// BIT(4) - ri_tlq_phdr parity error
// BIT(5) - ri_tlq_pdata parity error
// BIT(6) - ri_tlq_nphdr parity error
// BIT(7) - ri_tlq_npdata parity error
// BIT(8) - ri_tlq_cplhdr parity error
// BIT(10) - BIT(13) - ri_tlq_cpldata[0:3] parity error
// BIT(19) - ri_cds_cmd_fifo parity error
// BIT(20) - ri_obc_ricpl_fifo parity error
// BIT(21) - ri_obc_tiricpl_fifo parity error
// BIT(22) - ri_obc_cppcpl_fifo parity error
// BIT(23) - ri_obc_pendcpl_fifo parity error
// BIT(24) - ri_cpp_cmd_fifo parity error
// BIT(25) - ri_cds_ticmd_fifo parity error
// BIT(26) - riti_cmd_fifo parity error
// BIT(27) - ri_int_msixtbl parity error
// BIT(28) - ri_int_imstbl parity error
// BIT(30) - ri_kpt_fuses parity error
//

// TI CI parity status
pub const ADF_GEN6_TI_CI_PAR_STS: c_uint = 0x50060C;
// TI CI parity reporting mask
pub const ADF_GEN6_TI_CI_PAR_ERR_MASK: c_uint = 0x500608;
//
// TI CI parity status mask
// BIT(0) - CdCmdQ_sts patiry error status
// BIT(1) - CdDataQ_sts parity error status
// BIT(3) - CPP_SkidQ_sts parity error status
//

// TI PULLFUB parity status
pub const ADF_GEN6_TI_PULL0FUB_PAR_STS: c_uint = 0x500618;
// TI PULLFUB parity error reporting mask
pub const ADF_GEN6_TI_PULL0FUB_PAR_ERR_MASK: c_uint = 0x500614;
//
// TI PULLFUB parity status mask
// BIT(0) - TrnPullReqQ_sts parity status
// BIT(1) - TrnSharedDataQ_sts parity status
// BIT(2) - TrnPullReqDataQ_sts parity status
// BIT(4) - CPP_CiPullReqQ_sts parity status
// BIT(5) - CPP_TrnPullReqQ_sts parity status
// BIT(6) - CPP_PullidQ_sts parity status
// BIT(7) - CPP_WaitDataQ_sts parity status
// BIT(8) - CPP_CdDataQ_sts parity status
// BIT(9) - CPP_TrnDataQP0_sts parity status
// BIT(10) - BIT(11) - CPP_TrnDataQRF[00:01]_sts parity status
// BIT(12) - CPP_TrnDataQP1_sts parity status
// BIT(13) - BIT(14) - CPP_TrnDataQRF[10:11]_sts parity status
//

// TI PUSHUB parity status
pub const ADF_GEN6_TI_PUSHFUB_PAR_STS: c_uint = 0x500630;
// TI PUSHFUB parity error reporting mask
pub const ADF_GEN6_TI_PUSHFUB_PAR_ERR_MASK: c_uint = 0x50062C;
//
// TI PUSHUB parity status mask
// BIT(0) - SbPushReqQ_sts parity status
// BIT(1) - BIT(2) - SbPushDataQ[0:1]_sts parity status
// BIT(4) - CPP_CdPushReqQ_sts parity status
// BIT(5) - BIT(6) - CPP_CdPushDataQ[0:1]_sts parity status
// BIT(7) - CPP_SbPushReqQ_sts parity status
// BIT(8) - CPP_SbPushDataQP_sts parity status
// BIT(9) - BIT(10) - CPP_SbPushDataQRF[0:1]_sts parity status
//

// TI CD parity status
pub const ADF_GEN6_TI_CD_PAR_STS: c_uint = 0x50063C;
// TI CD parity error mask
pub const ADF_GEN6_TI_CD_PAR_ERR_MASK: c_uint = 0x500638;
//
// TI CD parity status mask
// BIT(0) - BIT(15) - CtxMdRam[0:15]_sts parity status
// BIT(16) - Leaf2ClusterRam_sts parity status
// BIT(17) - BIT(18) - Ring2LeafRam[0:1]_sts parity status
// BIT(19) - VirtualQ_sts parity status
// BIT(20) - DtRdQ_sts parity status
// BIT(21) - DtWrQ_sts parity status
// BIT(22) - RiCmdQ_sts parity status
// BIT(23) - BypassQ_sts parity status
// BIT(24) - DtRdQ_sc_sts parity status
// BIT(25) - DtWrQ_sc_sts parity status
//

// TI TRNSB parity status
pub const ADF_GEN6_TI_TRNSB_PAR_STS: c_uint = 0x500648;
// TI TRNSB parity error reporting mask
pub const ADF_GEN6_TI_TRNSB_PAR_ERR_MASK: c_uint = 0x500644;
//
// TI TRNSB parity status mask
// BIT(0) - TrnPHdrQP_sts parity status
// BIT(1) - TrnPHdrQRF_sts parity status
// BIT(2) - TrnPDataQP_sts parity status
// BIT(3) - BIT(6) - TrnPDataQRF[0:3]_sts parity status
// BIT(7) - TrnNpHdrQP_sts parity status
// BIT(8) - BIT(9) - TrnNpHdrQRF[0:1]_sts parity status
// BIT(10) - TrnCplHdrQ_sts parity status
// BIT(11) - TrnPutObsReqQ_sts parity status
// BIT(12) - TrnPushReqQ_sts parity status
// BIT(13) - SbSplitIdRam_sts parity status
// BIT(14) - SbReqCountQ_sts parity status
// BIT(15) - SbCplTrkRam_sts parity status
// BIT(16) - SbGetObsReqQ_sts parity status
// BIT(17) - SbEpochIdQ_sts parity status
// BIT(18) - SbAtCplHdrQ_sts parity status
// BIT(19) - SbAtCplDataQ_sts parity status
// BIT(20) - SbReqCountRam_sts parity status
// BIT(21) - SbAtCplHdrQ_sc_sts parity status
//

// Status register to log misc error on RI
pub const ADF_GEN6_RIMISCSTS: c_uint = 0x41B1B8;
// Status control register to log misc RI error
pub const ADF_GEN6_RIMISCCTL: c_uint = 0x41B1BC;
//
// ERRSOU2 bit mask
// BIT(0) - SSM Interrupt Mask
// BIT(1) - CFC on CPP. ORed of CFC Push error and Pull error
// BIT(2) - BIT(4) - CPP attention interrupts
// BIT(18) - PM interrupt
//

pub const ADF_GEN6_IAINTSTATSSM: c_uint = 0x28;
// IAINTSTATSSM error bit mask definitions

pub const ADF_GEN6_UERRSSMSH: c_uint = 0x18;
//
// UERRSSMSH error bit mask definitions
//
// BIT(0) - Indicates one uncorrectable error
// BIT(15) - Indicates multiple uncorrectable errors
// in device shared memory
//

//
// CERRSSMSH error bit
// BIT(0) - Indicates one correctable error
//

pub const ADF_GEN6_CERRSSMSH: c_uint = 0x10;
pub const ADF_GEN6_INTMASKSSM: c_uint = 0x0;
//
// Error reporting mask in INTMASKSSM
// BIT(0) - Shared memory uncorrectable interrupt mask
// BIT(2) - PPERR interrupt mask
// BIT(4) - SCM parity error interrupt mask
// BIT(5) - CPP parity error interrupt mask
// BIT(6) - SHRAM RF parity error interrupt mask
// BIT(7) - AXI unexpected completion error mask
//

// CPP push or pull error
pub const ADF_GEN6_PPERR: c_uint = 0x8;

//
// SSM_FERR_STATUS error bit mask definitions
//

pub const ADF_GEN6_SSM_FERR_STATUS: c_uint = 0x9C;
pub const ADF_GEN6_CPP_CFC_ERR_STATUS: c_uint = 0x640C04;
//
// BIT(0) - Indicates one or more CPP CFC errors
// BIT(1) - Indicates multiple CPP CFC errors
// BIT(7) - Indicates CPP CFC command parity error type
// BIT(8) - Indicates CPP CFC data parity error type
//

//
// BIT(0) - Enables CFC to detect and log a push/pull data error
// BIT(1) - Enables CFC to generate interrupt to PCIEP for a CPP error
// BIT(4) - When 1 parity detection is disabled
// BIT(5) - When 1 parity detection is disabled on CPP command bus
// BIT(6) - When 1 parity detection is disabled on CPP push/pull bus
// BIT(9) - When 1 RF parity error detection is disabled
//

pub const ADF_GEN6_CPP_CFC_ERR_CTRL: c_uint = 0x640C00;
//
// BIT(0) - Clears bit(0) of ADF_GEN6_CPP_CFC_ERR_STATUS
// when an error is reported on CPP
// BIT(1) - Clears bit(1) of ADF_GEN6_CPP_CFC_ERR_STATUS
// when multiple errors are reported on CPP
// BIT(2) - Clears bit(2) of ADF_GEN6_CPP_CFC_ERR_STATUS
// when attention interrupt is reported
//

pub const ADF_GEN6_CPP_CFC_ERR_STATUS_CLR: c_uint = 0x640C08;
//
// ERRSOU3 bit masks
// BIT(0) - indicates error response order overflow and/or BME error
// BIT(1) - indicates RI push/pull error
// BIT(2) - indicates TI push/pull error
// BIT(5) - indicates TI pull parity error
// BIT(6) - indicates RI push parity error
// BIT(7) - indicates VFLR interrupt
// BIT(8) - indicates ring pair interrupts for ATU detected fault
// BIT(9) - indicates rate limiting error
//

// Rate limiting error log register
pub const ADF_GEN6_RLT_ERRLOG: c_uint = 0x508814;

// TI misc status register
pub const ADF_GEN6_TIMISCSTS: c_uint = 0x50054C;
// TI misc error reporting mask
pub const ADF_GEN6_TIMISCCTL: c_uint = 0x500548;
//
// TI Misc error reporting control mask
// BIT(0) - Enables error detection and logging in TIMISCSTS register
// BIT(1) - It has effect only when SRIOV enabled, this bit is 0 by default
// BIT(2) - Enables the D-F-x counter within the dispatch arbiter
// to start based on the command triggered from
// BIT(30) - Disables VFLR functionality
// bits 1, 2 and 30 value should be preserved and not meant to be changed
// within RAS.
//

// RI CPP interface status register
pub const ADF_GEN6_RICPPINTSTS: c_uint = 0x41A330;
//
// Uncorrectable error mask in RICPPINTSTS register
// BIT(0) - RI asserted the CPP error signal during a push
// BIT(1) - RI detected the CPP error signal asserted during a pull
// BIT(2) - RI detected a push data parity error
// BIT(3) - RI detected a push valid parity error
//

// RI CPP interface register control
pub const ADF_GEN6_RICPPINTCTL: c_uint = 0x41A32C;
//
// Control bit mask for RICPPINTCTL register
// BIT(0) - value of 1 enables error detection and reporting
// on the RI CPP Push interface
// BIT(1) - value of 1 enables error detection and reporting
// on the RI CPP Pull interface
// BIT(2) - value of 1 enables error detection and reporting
// on the RI Parity
// BIT(3) - value of 1 enable checking parity on CPP
//

// TI CPP interface status register
pub const ADF_GEN6_TICPPINTSTS: c_uint = 0x50053C;
//
// Uncorrectable error mask in TICPPINTSTS register
// BIT(0) - value of 1 indicates that the TI asserted
// the CPP error signal during a push
// BIT(1) - value of 1 indicates that the TI detected
// the CPP error signal asserted during a pull
// BIT(2) - value of 1 indicates that the TI detected
// a pull data parity error
//

// TI CPP interface status register control
pub const ADF_GEN6_TICPPINTCTL: c_uint = 0x500538;
//
// Control bit mask for TICPPINTCTL register
// BIT(0) - value of 1 enables error detection and reporting on
// the TI CPP Push interface
// BIT(1) - value of 1 enables error detection and reporting on
// the TI CPP Push interface
// BIT(2) - value of 1 enables parity error detection and logging on
// the TI CPP Pull interface
// BIT(3) - value of 1 enables CPP CMD and Pull Data parity checking
//

// ATU fault status register

// Command parity error detected on IOSFP command to QAT

pub const ADF_GEN6_GENSTS: c_uint = 0x41A220;

pub const ADF_GEN6_GENSTS_PFLR: c_uint = 0x1;
pub const ADF_GEN6_GENSTS_COLD_RESET: c_uint = 0x3;
pub const ADF_GEN6_GENSTS_DEVHALT: c_uint = 0x1;
extern "C" {
    pub fn adf_gen6_init_ras_ops(ras_ops: *mut adf_ras_ops);
}
