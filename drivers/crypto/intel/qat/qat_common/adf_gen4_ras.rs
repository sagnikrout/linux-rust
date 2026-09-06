//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_gen4_ras.h
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
// Copyright(c) 2023 Intel Corporation

// ERRSOU0 Correctable error mask

// HI AE Correctable error log
pub const ADF_GEN4_HIAECORERRLOG_CPP0: c_uint = 0x41A308;
// HI AE Correctable error log enable
pub const ADF_GEN4_HIAECORERRLOGENABLE_CPP0: c_uint = 0x41A318;

// HI AE Uncorrectable error log
pub const ADF_GEN4_HIAEUNCERRLOG_CPP0: c_uint = 0x41A300;
// HI AE Uncorrectable error log enable
pub const ADF_GEN4_HIAEUNCERRLOGENABLE_CPP0: c_uint = 0x41A320;
// HI CPP Agent Command parity error log
pub const ADF_GEN4_HICPPAGENTCMDPARERRLOG: c_uint = 0x41A310;
// HI CPP Agent Command parity error logging enable
pub const ADF_GEN4_HICPPAGENTCMDPARERRLOGENABLE: c_uint = 0x41A314;
// RI Memory parity error status register
pub const ADF_GEN4_RIMEM_PARERR_STS: c_uint = 0x41B128;
// RI Memory parity error reporting enable
pub const ADF_GEN4_RI_MEM_PAR_ERR_EN0: c_uint = 0x41B12C;
//
// RI Memory parity error mask
// BIT(0) - BIT(3) - ri_iosf_pdata_rxq[0:3] parity error
// BIT(4) - ri_tlq_phdr parity error
// BIT(5) - ri_tlq_pdata parity error
// BIT(6) - ri_tlq_nphdr parity error
// BIT(7) - ri_tlq_npdata parity error
// BIT(8) - BIT(9) - ri_tlq_cplhdr[0:1] parity error
// BIT(10) - BIT(17) - ri_tlq_cpldata[0:7] parity error
// BIT(18) - set this bit to 1 to enable logging status to ri_mem_par_err_sts0
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
pub const ADF_GEN4_TI_CI_PAR_STS: c_uint = 0x50060C;
// TI CI parity reporting mask
pub const ADF_GEN4_TI_CI_PAR_ERR_MASK: c_uint = 0x500608;
//
// TI CI parity status mask
// BIT(0) - CdCmdQ_sts patiry error status
// BIT(1) - CdDataQ_sts parity error status
// BIT(3) - CPP_SkidQ_sts parity error status
// BIT(7) - CPP_SkidQ_sc_sts parity error status
//

// TI PULLFUB parity status
pub const ADF_GEN4_TI_PULL0FUB_PAR_STS: c_uint = 0x500618;
// TI PULLFUB parity error reporting mask
pub const ADF_GEN4_TI_PULL0FUB_PAR_ERR_MASK: c_uint = 0x500614;
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
pub const ADF_GEN4_TI_PUSHFUB_PAR_STS: c_uint = 0x500630;
// TI PUSHFUB parity error reporting mask
pub const ADF_GEN4_TI_PUSHFUB_PAR_ERR_MASK: c_uint = 0x50062C;
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
pub const ADF_GEN4_TI_CD_PAR_STS: c_uint = 0x50063C;
// TI CD parity error mask
pub const ADF_GEN4_TI_CD_PAR_ERR_MASK: c_uint = 0x500638;
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
pub const ADF_GEN4_TI_TRNSB_PAR_STS: c_uint = 0x500648;
// TI TRNSB Parity error reporting mask
pub const ADF_GEN4_TI_TRNSB_PAR_ERR_MASK: c_uint = 0x500644;
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
pub const ADF_GEN4_RIMISCSTS: c_uint = 0x41B1B8;
// Status control register to log misc RI error
pub const ADF_GEN4_RIMISCCTL: c_uint = 0x41B1BC;
//
// ERRSOU2 bit mask
// BIT(0) - SSM Interrupt Mask
// BIT(1) - CFC on CPP. ORed of CFC Push error and Pull error
// BIT(2) - BIT(4) - CPP attention interrupts, deprecated on gen4 devices
// BIT(18) - PM interrupt
//

pub const ADF_GEN4_IAINTSTATSSM: c_uint = 0x28;
// IAINTSTATSSM error bit mask definitions

pub const ADF_GEN4_UERRSSMSH: c_uint = 0x18;
//
// UERRSSMSH error bit masks definitions
//
// BIT(0) - Indicates one uncorrectable error
// BIT(15) - Indicates multiple uncorrectable errors
// in device shared memory
//

pub const ADF_GEN4_UERRSSMSHAD: c_uint = 0x1C;
pub const ADF_GEN4_CERRSSMSH: c_uint = 0x10;
//
// CERRSSMSH error bit
// BIT(0) - Indicates one correctable error
//

pub const ADF_GEN4_CERRSSMSHAD: c_uint = 0x14;
// SSM error handling features enable register
pub const ADF_GEN4_SSMFEATREN: c_uint = 0x198;
//
// Disable SSM error detection and reporting features
// enabled by device driver on RAS initialization
//
// following bits should be cleared :
// BIT(4)  - Disable parity for CPP parity
// BIT(12) - Disable logging push/pull data error in pperr register.
// BIT(16) - BIT(23) - Disable parity for SPPs
// BIT(24) - BIT(27) - Disable parity for SPPs, if it's supported on the device.
//

pub const ADF_GEN4_INTMASKSSM: c_uint = 0x0;
//
// Error reporting mask in INTMASKSSM
// BIT(0) - Shared memory uncorrectable interrupt mask
// BIT(1) - Shared memory correctable interrupt mask
// BIT(2) - PPERR interrupt mask
// BIT(3) - CPP parity error Interrupt mask
// BIT(4) - SSM interrupt generated by SER correctable error mask
// BIT(5) - SSM interrupt generated by SER uncorrectable error
// - not stop and scream - mask
//

// CPP push or pull error
pub const ADF_GEN4_PPERR: c_uint = 0x8;

pub const ADF_GEN4_PPERRID: c_uint = 0xC;
// Slice hang handling related registers
pub const ADF_GEN4_SLICEHANGSTATUS_ATH_CPH: c_uint = 0x84;
pub const ADF_GEN4_SLICEHANGSTATUS_CPR_XLT: c_uint = 0x88;
pub const ADF_GEN4_SLICEHANGSTATUS_DCPR_UCS: c_uint = 0x90;
pub const ADF_GEN4_SLICEHANGSTATUS_WAT_WCP: c_uint = 0x8C;
pub const ADF_GEN4_SLICEHANGSTATUS_PKE: c_uint = 0x94;
pub const ADF_GEN4_SHINTMASKSSM_ATH_CPH: c_uint = 0xF0;
pub const ADF_GEN4_SHINTMASKSSM_CPR_XLT: c_uint = 0xF4;
pub const ADF_GEN4_SHINTMASKSSM_DCPR_UCS: c_uint = 0xFC;
pub const ADF_GEN4_SHINTMASKSSM_WAT_WCP: c_uint = 0xF8;
pub const ADF_GEN4_SHINTMASKSSM_PKE: c_uint = 0x100;
// SPP pull cmd parity err_*slice* CSR
pub const ADF_GEN4_SPPPULLCMDPARERR_ATH_CPH: c_uint = 0x1A4;
pub const ADF_GEN4_SPPPULLCMDPARERR_CPR_XLT: c_uint = 0x1A8;
pub const ADF_GEN4_SPPPULLCMDPARERR_DCPR_UCS: c_uint = 0x1B0;
pub const ADF_GEN4_SPPPULLCMDPARERR_PKE: c_uint = 0x1B4;
pub const ADF_GEN4_SPPPULLCMDPARERR_WAT_WCP: c_uint = 0x1AC;
// SPP pull data parity err_*slice* CSR
pub const ADF_GEN4_SPPPULLDATAPARERR_ATH_CPH: c_uint = 0x1BC;
pub const ADF_GEN4_SPPPULLDATAPARERR_CPR_XLT: c_uint = 0x1C0;
pub const ADF_GEN4_SPPPULLDATAPARERR_DCPR_UCS: c_uint = 0x1C8;
pub const ADF_GEN4_SPPPULLDATAPARERR_PKE: c_uint = 0x1CC;
pub const ADF_GEN4_SPPPULLDATAPARERR_WAT_WCP: c_uint = 0x1C4;
// SPP push cmd parity err_*slice* CSR
pub const ADF_GEN4_SPPPUSHCMDPARERR_ATH_CPH: c_uint = 0x1D4;
pub const ADF_GEN4_SPPPUSHCMDPARERR_CPR_XLT: c_uint = 0x1D8;
pub const ADF_GEN4_SPPPUSHCMDPARERR_DCPR_UCS: c_uint = 0x1E0;
pub const ADF_GEN4_SPPPUSHCMDPARERR_PKE: c_uint = 0x1E4;
pub const ADF_GEN4_SPPPUSHCMDPARERR_WAT_WCP: c_uint = 0x1DC;
// SPP push data parity err_*slice* CSR
pub const ADF_GEN4_SPPPUSHDATAPARERR_ATH_CPH: c_uint = 0x1EC;
pub const ADF_GEN4_SPPPUSHDATAPARERR_CPR_XLT: c_uint = 0x1F0;
pub const ADF_GEN4_SPPPUSHDATAPARERR_DCPR_UCS: c_uint = 0x1F8;
pub const ADF_GEN4_SPPPUSHDATAPARERR_PKE: c_uint = 0x1FC;
pub const ADF_GEN4_SPPPUSHDATAPARERR_WAT_WCP: c_uint = 0x1F4;
// Accelerator SPP parity error mask registers
pub const ADF_GEN4_SPPPARERRMSK_ATH_CPH: c_uint = 0x204;
pub const ADF_GEN4_SPPPARERRMSK_CPR_XLT: c_uint = 0x208;
pub const ADF_GEN4_SPPPARERRMSK_DCPR_UCS: c_uint = 0x210;
pub const ADF_GEN4_SPPPARERRMSK_PKE: c_uint = 0x214;
pub const ADF_GEN4_SPPPARERRMSK_WAT_WCP: c_uint = 0x20C;
pub const ADF_GEN4_SSMCPPERR: c_uint = 0x224;
//
// Uncorrectable error mask in SSMCPPERR
// BIT(0) - indicates CPP command parity error
// BIT(1) - indicates CPP Main Push PPID parity error
// BIT(2) - indicates CPP Main ePPID parity error
// BIT(3) - indicates CPP Main push data parity error
// BIT(4) - indicates CPP Main Pull PPID parity error
// BIT(5) - indicates CPP target pull data parity error
//

pub const ADF_GEN4_SSMSOFTERRORPARITY_SRC: c_uint = 0x9C;
pub const ADF_GEN4_SSMSOFTERRORPARITYMASK_SRC: c_uint = 0xB8;
pub const ADF_GEN4_SSMSOFTERRORPARITY_ATH_CPH: c_uint = 0xA0;
pub const ADF_GEN4_SSMSOFTERRORPARITYMASK_ATH_CPH: c_uint = 0xBC;
pub const ADF_GEN4_SSMSOFTERRORPARITY_CPR_XLT: c_uint = 0xA4;
pub const ADF_GEN4_SSMSOFTERRORPARITYMASK_CPR_XLT: c_uint = 0xC0;
pub const ADF_GEN4_SSMSOFTERRORPARITY_DCPR_UCS: c_uint = 0xAC;
pub const ADF_GEN4_SSMSOFTERRORPARITYMASK_DCPR_UCS: c_uint = 0xC8;
pub const ADF_GEN4_SSMSOFTERRORPARITY_PKE: c_uint = 0xB0;
pub const ADF_GEN4_SSMSOFTERRORPARITYMASK_PKE: c_uint = 0xCC;
pub const ADF_GEN4_SSMSOFTERRORPARITY_WAT_WCP: c_uint = 0xA8;
pub const ADF_GEN4_SSMSOFTERRORPARITYMASK_WAT_WCP: c_uint = 0xC4;
// RF parity error detected in SharedRAM

pub const ADF_GEN4_SER_ERR_SSMSH: c_uint = 0x44C;
//
// Fatal error mask in SER_ERR_SSMSH
// BIT(0) - Indicates an uncorrectable error has occurred in the
// accelerator controller command RFs
// BIT(2) - Parity error occurred in the bank SPP fifos
// BIT(3) - Indicates Parity error occurred in following fifos in
// the design
// BIT(4) - Parity error occurred in flops in the design
// BIT(5) - Uncorrectable error has occurred in the
// target push and pull data register flop
// BIT(7) - Indicates Parity error occurred in the Resource Manager
// pending lock request fifos
// BIT(8) - Indicates Parity error occurred in the Resource Manager
// MECTX command queues logic
// BIT(9) - Indicates Parity error occurred in the Resource Manager
// MECTX sigdone fifo flops
// BIT(10) - Indicates an uncorrectable error has occurred in the
// Resource Manager MECTX command RFs
// BIT(14) - Parity error occurred in Buffer Manager sigdone FIFO
//

//
// Uncorrectable error mask in SER_ERR_SSMSH
// BIT(12) Parity error occurred in Buffer Manager pool 0
// BIT(13) Parity error occurred in Buffer Manager pool 1
//

//
// Correctable error mask in SER_ERR_SSMSH
// BIT(1) - Indicates a correctable Error has occurred
// in the slice controller command RFs
// BIT(6) - Indicates a correctable Error has occurred in
// the target push and pull data RFs
// BIT(11) - Indicates an correctable Error has occurred in
// the Resource Manager MECTX command RFs
//

// SSM shared memory SER error reporting mask
pub const ADF_GEN4_SER_EN_SSMSH: c_uint = 0x450;
//
// SSM SER error reporting mask in SER_en_err_ssmsh
// BIT(0) - Enables uncorrectable Error detection in :
// 1) slice controller command RFs.
// 2) target push/pull data registers
// BIT(1) - Enables correctable Error detection in :
// 1) slice controller command RFs
// 2) target push/pull data registers
// BIT(2) - Enables Parity error detection in
// 1) bank SPP fifos
// 2) gen4_pull_id_queue
// 3) gen4_push_id_queue
// 4) AE_pull_sigdn_fifo
// 5) DT_push_sigdn_fifo
// 6) slx_push_sigdn_fifo
// 7) secure_push_cmd_fifo
// 8) secure_pull_cmd_fifo
// 9) Head register in FIFO wrapper
// 10) current_cmd in individual push queue
// 11) current_cmd in individual pull queue
// 12) push_command_rxp arbitrated in ssm_push_cmd_queues
// 13) pull_command_rxp arbitrated in ssm_pull_cmd_queues
// BIT(3) - Enables uncorrectable Error detection in
// the resource manager mectx cmd RFs.
// BIT(4) - Enables correctable error detection in the Resource Manager
// mectx command RFs
// BIT(5) - Enables Parity error detection in
// 1) resource manager lock request fifo
// 2) mectx cmdqueues logic
// 3) mectx sigdone fifo
// BIT(6) - Enables Parity error detection in Buffer Manager pools
// and sigdone fifo
//

pub const ADF_GEN4_CPP_CFC_ERR_STATUS: c_uint = 0x640C04;
//
// BIT(1) - Indicates multiple CPP CFC errors
// BIT(7) - Indicates CPP CFC command parity error type
// BIT(8) - Indicated CPP CFC data parity error type
//

//
// BIT(0) - Enables CFC to detect and log push/pull data error
// BIT(1) - Enables CFC to generate interrupt to PCIEP for CPP error
// BIT(4) - When 1 Parity detection is disabled
// BIT(5) - When 1 Parity detection is disabled on CPP command bus
// BIT(6) - When 1 Parity detection is disabled on CPP push/pull bus
// BIT(9) - When 1 RF parity error detection is disabled
//

pub const ADF_GEN4_CPP_CFC_ERR_CTRL: c_uint = 0x640C00;
//
// BIT(0) - Clears bit(0) of ADF_GEN4_CPP_CFC_ERR_STATUS
// when an error is reported on CPP
// BIT(1) - Clears bit(1) of ADF_GEN4_CPP_CFC_ERR_STATUS
// when multiple errors are reported on CPP
// BIT(2) - Clears bit(2) of ADF_GEN4_CPP_CFC_ERR_STATUS
// when attention interrupt is reported
//

pub const ADF_GEN4_CPP_CFC_ERR_STATUS_CLR: c_uint = 0x640C08;
pub const ADF_GEN4_CPP_CFC_ERR_PPID_LO: c_uint = 0x640C0C;
pub const ADF_GEN4_CPP_CFC_ERR_PPID_HI: c_uint = 0x640C10;
// Exception reporting in QAT SSM CMP
pub const ADF_GEN4_EXPRPSSMCPR: c_uint = 0x2000;
//
// Uncorrectable error mask in EXPRPSSMCPR
// BIT(2) - Hard fatal error
// BIT(16) - Parity error detected in CPR Push FIFO
// BIT(17) - Parity error detected in CPR Pull FIFO
// BIT(18) - Parity error detected in CPR Hash Table
// BIT(19) - Parity error detected in CPR History Buffer Copy 0
// BIT(20) - Parity error detected in CPR History Buffer Copy 1
// BIT(21) - Parity error detected in CPR History Buffer Copy 2
// BIT(22) - Parity error detected in CPR History Buffer Copy 3
// BIT(23) - Parity error detected in CPR History Buffer Copy 4
// BIT(24) - Parity error detected in CPR History Buffer Copy 5
// BIT(25) - Parity error detected in CPR History Buffer Copy 6
// BIT(26) - Parity error detected in CPR History Buffer Copy 7
//

// Exception reporting in QAT SSM XLT
pub const ADF_GEN4_EXPRPSSMXLT: c_uint = 0xA000;
//
// Uncorrectable error mask in EXPRPSSMXLT
// BIT(2) - If set, an Uncorrectable Error event occurred
// BIT(16) - Parity error detected in XLT Push FIFO
// BIT(17) - Parity error detected in XLT Pull FIFO
// BIT(18) - Parity error detected in XLT HCTB0
// BIT(19) - Parity error detected in XLT HCTB1
// BIT(20) - Parity error detected in XLT HCTB2
// BIT(21) - Parity error detected in XLT HCTB3
// BIT(22) - Parity error detected in XLT CBCL
// BIT(23) - Parity error detected in XLT LITPTR
//

//
// Correctable error mask in EXPRPSSMXLT
// BIT(3) - Correctable error event occurred.
//

// Exception reporting in QAT SSM DCMP

//
// Uncorrectable error mask in EXPRPSSMDCPR
// BIT(2) - Even hard fatal error
// BIT(4) - Odd hard fatal error
// BIT(6) - decode soft error
// BIT(16) - Parity error detected in CPR Push FIFO
// BIT(17) - Parity error detected in CPR Pull FIFO
// BIT(18) - Parity error detected in the Input Buffer
// BIT(19) - symbuf0parerr
// Parity error detected in CPR Push FIFO
// BIT(20) - symbuf1parerr
// Parity error detected in CPR Push FIFO
//

//
// Correctable error mask in EXPRPSSMDCPR
// BIT(3) - Even ecc correctable error
// BIT(5) - Odd ecc correctable error
//

pub const ADF_GEN4_DCPR_SLICES_NUM: c_int = 3;
//
// ERRSOU3 bit masks
// BIT(0) - indicates error Response Order Overflow and/or BME error
// BIT(1) - indicates RI push/pull error
// BIT(2) - indicates TI push/pull error
// BIT(3) - indicates ARAM correctable error
// BIT(4) - indicates ARAM uncorrectable error
// BIT(5) - indicates TI pull parity error
// BIT(6) - indicates RI push parity error
// BIT(7) - indicates VFLR interrupt
// BIT(8) - indicates ring pair interrupts for ATU detected fault
// BIT(9) - indicates error when accessing RLT block
//

// TI Misc status register
pub const ADF_GEN4_TIMISCSTS: c_uint = 0x50054C;
// TI Misc error reporting mask
pub const ADF_GEN4_TIMISCCTL: c_uint = 0x500548;
//
// TI Misc error reporting control mask
// BIT(0) - Enables error detection and logging in TIMISCSTS register
// BIT(1) - It has effect only when SRIOV enabled, this bit is 0 by default
// BIT(2) - Enables the D-F-x counter within the dispatch arbiter
// to start based on the command triggered from
// BIT(30) - Disables VFLR functionality
// By setting this bit will revert to CPM1.x functionality
// bits 1, 2 and 30 value should be preserved and not meant to be changed
// within RAS.
//

// RI CPP interface status register
pub const ADF_GEN4_RICPPINTSTS: c_uint = 0x41A330;
//
// Uncorrectable error mask in RICPPINTSTS register
// BIT(0) - RI asserted the CPP error signal during a push
// BIT(1) - RI detected the CPP error signal asserted during a pull
// BIT(2) - RI detected a push data parity error
// BIT(3) - RI detected a push valid parity error
//

// RI CPP interface status register control
pub const ADF_GEN4_RICPPINTCTL: c_uint = 0x41A32C;
//
// Control bit mask for RICPPINTCTL register
// BIT(0) - value of 1 enables error detection and reporting
// on the RI CPP Push interface
// BIT(1) - value of 1 enables error detection and reporting
// on the RI CPP Pull interface
// BIT(2) - value of 1 enables error detection and reporting
// on the RI Parity
// BIT(3) - value of 1 enable checking parity on CPP
// BIT(4) - value of 1 enables the stop feature of the stop and stream
// for all RI CPP Command RFs
//

// Push ID of the command which triggered the transaction error on RI
pub const ADF_GEN4_RIERRPUSHID: c_uint = 0x41A334;
// Pull ID of the command which triggered the transaction error on RI
pub const ADF_GEN4_RIERRPULLID: c_uint = 0x41A338;
// TI CPP interface status register
pub const ADF_GEN4_TICPPINTSTS: c_uint = 0x50053C;
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
pub const ADF_GEN4_TICPPINTCTL: c_uint = 0x500538;
//
// Control bit mask for TICPPINTCTL register
// BIT(0) - value of 1 enables error detection and reporting on
// the TI CPP Push interface
// BIT(1) - value of 1 enables error detection and reporting on
// the TI CPP Push interface
// BIT(2) - value of 1 enables parity error detection and logging on
// the TI CPP Pull interface
// BIT(3) - value of 1 enables CPP CMD and Pull Data parity checking
// BIT(4) - value of 1 enables TI stop part of stop and scream mode on
// CPP/RF Parity error
//

// Push ID of the command which triggered the transaction error on TI
pub const ADF_GEN4_TIERRPUSHID: c_uint = 0x500540;
// Pull ID of the command which triggered the transaction error on TI
pub const ADF_GEN4_TIERRPULLID: c_uint = 0x500544;
// Correctable error in ARAM agent register
pub const ADF_GEN4_REG_ARAMCERR: c_uint = 0x1700;

//
// Correctable error enablement in ARAM bit mask
// BIT(3) - enable ARAM RAM to fix and log correctable error
// BIT(26) - enables ARAM agent to generate interrupt for correctable error
//

// Correctable error address in ARAM agent register
pub const ADF_GEN4_REG_ARAMCERRAD: c_uint = 0x1708;
// Uncorrectable error in ARAM agent register
pub const ADF_GEN4_REG_ARAMUERR: c_uint = 0x1704;
//
// ARAM error bit mask
// BIT(0) - indicates error logged in ARAMCERR or ARAMUCERR
// BIT(18) - indicates uncorrectable multiple errors in ARAM agent
//

//
// Uncorrectable error enablement in ARAM bit mask
// BIT(3) - enable ARAM RAM to fix and log uncorrectable error
// BIT(19) - enables ARAM agent to generate interrupt for uncorrectable error
//

// Unorrectable error address in ARAM agent register
pub const ADF_GEN4_REG_ARAMUERRAD: c_uint = 0x170C;
// Uncorrectable error transaction push/pull ID registers
pub const ADF_GEN4_REG_ERRPPID_LO: c_uint = 0x1714;
pub const ADF_GEN4_REG_ERRPPID_HI: c_uint = 0x1718;
// ARAM ECC block error enablement
pub const ADF_GEN4_REG_ARAMCERRUERR_EN: c_uint = 0x1808;
//
// ARAM ECC block error control bit masks
// BIT(0) - enable ARAM CD ECC block error detecting
// BIT(1) - enable ARAM pull request ECC error detecting
// BIT(2) - enable ARAM command dispatch ECC error detecting
// BIT(3) - enable ARAM read datapath push ECC error detecting
// BIT(4) - enable ARAM read datapath pull ECC error detecting
// BIT(5) - enable ARAM RMW ECC error detecting
// BIT(6) - enable ARAM write datapath RMW ECC error detecting
// BIT(7) - enable ARAM write datapath ECC error detecting
//

// ARAM misc memory target error registers
pub const ADF_GEN4_REG_CPPMEMTGTERR: c_uint = 0x1710;
//
// ARAM misc memory target error bit masks
// BIT(0) - indicates an error in ARAM target memory
// BIT(1) - indicates multiple errors in ARAM target memory
// BIT(4) - indicates pull error in ARAM target memory
// BIT(5) - indicates parity pull error in ARAM target memory
// BIT(6) - indicates push error in ARAM target memory
//

//
// ARAM misc memory target error enablement mask
// BIT(2) - enables CPP memory to detect and log push/pull data error
// BIT(7) - enables push/pull error to generate interrupts to RI
// BIT(8) - enables ARAM to check parity on pull data and CPP command buses
// BIT(9) - enables ARAM to autopush to AE when push/parity error is detected
// on lookaside DT
//

// ATU fault status register

// Command Parity error detected on IOSFP Command to QAT

extern "C" {
    pub fn adf_gen4_init_ras_ops(ras_ops: *mut adf_ras_ops);
}
