//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_esai.h
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
// fsl_esai.h - ALSA ESAI interface for the Freescale i.MX SoC
//
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//
// Author: Nicolin Chen <Guangyu.Chen@freescale.com>
//
// ESAI Register Map
pub const REG_ESAI_ETDR: c_uint = 0x00;
pub const REG_ESAI_ERDR: c_uint = 0x04;
pub const REG_ESAI_ECR: c_uint = 0x08;
pub const REG_ESAI_ESR: c_uint = 0x0C;
pub const REG_ESAI_TFCR: c_uint = 0x10;
pub const REG_ESAI_TFSR: c_uint = 0x14;
pub const REG_ESAI_RFCR: c_uint = 0x18;
pub const REG_ESAI_RFSR: c_uint = 0x1C;

pub const REG_ESAI_TX0: c_uint = 0x80;
pub const REG_ESAI_TX1: c_uint = 0x84;
pub const REG_ESAI_TX2: c_uint = 0x88;
pub const REG_ESAI_TX3: c_uint = 0x8C;
pub const REG_ESAI_TX4: c_uint = 0x90;
pub const REG_ESAI_TX5: c_uint = 0x94;
pub const REG_ESAI_TSR: c_uint = 0x98;
pub const REG_ESAI_RX0: c_uint = 0xA0;
pub const REG_ESAI_RX1: c_uint = 0xA4;
pub const REG_ESAI_RX2: c_uint = 0xA8;
pub const REG_ESAI_RX3: c_uint = 0xAC;
pub const REG_ESAI_SAISR: c_uint = 0xCC;
pub const REG_ESAI_SAICR: c_uint = 0xD0;
pub const REG_ESAI_TCR: c_uint = 0xD4;
pub const REG_ESAI_TCCR: c_uint = 0xD8;
pub const REG_ESAI_RCR: c_uint = 0xDC;
pub const REG_ESAI_RCCR: c_uint = 0xE0;

pub const REG_ESAI_TSMA: c_uint = 0xE4;
pub const REG_ESAI_TSMB: c_uint = 0xE8;
pub const REG_ESAI_RSMA: c_uint = 0xEC;
pub const REG_ESAI_RSMB: c_uint = 0xF0;

pub const REG_ESAI_PRRC: c_uint = 0xF8;
pub const REG_ESAI_PCRC: c_uint = 0xFC;
// ESAI Control Register -- REG_ESAI_ECR 0x8
pub const ESAI_ECR_ETI_SHIFT: c_int = 19;

pub const ESAI_ECR_ETO_SHIFT: c_int = 18;

pub const ESAI_ECR_ERI_SHIFT: c_int = 17;

pub const ESAI_ECR_ERO_SHIFT: c_int = 16;

pub const ESAI_ECR_ERST_SHIFT: c_int = 1;

pub const ESAI_ECR_ESAIEN_SHIFT: c_int = 0;

// ESAI Status Register -- REG_ESAI_ESR 0xC
pub const ESAI_ESR_TINIT_SHIFT: c_int = 10;

pub const ESAI_ESR_RFF_SHIFT: c_int = 9;

pub const ESAI_ESR_TFE_SHIFT: c_int = 8;

pub const ESAI_ESR_TLS_SHIFT: c_int = 7;

pub const ESAI_ESR_TDE_SHIFT: c_int = 6;

pub const ESAI_ESR_TED_SHIFT: c_int = 5;

pub const ESAI_ESR_TD_SHIFT: c_int = 4;

pub const ESAI_ESR_RLS_SHIFT: c_int = 3;

pub const ESAI_ESR_RDE_SHIFT: c_int = 2;

pub const ESAI_ESR_RED_SHIFT: c_int = 1;

pub const ESAI_ESR_RD_SHIFT: c_int = 0;

//
// Transmit FIFO Configuration Register -- REG_ESAI_TFCR 0x10
// Receive FIFO Configuration Register -- REG_ESAI_RFCR 0x18
//
pub const ESAI_xFCR_TIEN_SHIFT: c_int = 19;

pub const ESAI_xFCR_REXT_SHIFT: c_int = 19;

pub const ESAI_xFCR_xWA_SHIFT: c_int = 16;
pub const ESAI_xFCR_xWA_WIDTH: c_int = 3;

pub const ESAI_xFCR_xFWM_SHIFT: c_int = 8;
pub const ESAI_xFCR_xFWM_WIDTH: c_int = 8;

pub const ESAI_xFCR_xE_SHIFT: c_int = 2;
pub const ESAI_xFCR_TE_WIDTH: c_int = 6;
pub const ESAI_xFCR_RE_WIDTH: c_int = 4;

pub const ESAI_xFCR_xFR_SHIFT: c_int = 1;

pub const ESAI_xFCR_xFEN_SHIFT: c_int = 0;

//
// Transmit FIFO Status Register -- REG_ESAI_TFSR 0x14
// Receive FIFO Status Register --REG_ESAI_RFSR 0x1C
//
pub const ESAI_xFSR_NTFO_SHIFT: c_int = 12;
pub const ESAI_xFSR_NRFI_SHIFT: c_int = 12;
pub const ESAI_xFSR_NTFI_SHIFT: c_int = 8;
pub const ESAI_xFSR_NRFO_SHIFT: c_int = 8;
pub const ESAI_xFSR_NTFx_WIDTH: c_int = 3;
pub const ESAI_xFSR_NRFx_WIDTH: c_int = 2;

pub const ESAI_xFSR_xFCNT_SHIFT: c_int = 0;
pub const ESAI_xFSR_xFCNT_WIDTH: c_int = 8;

// ESAI Transmit Slot Register -- REG_ESAI_TSR 0x98
pub const ESAI_TSR_SHIFT: c_int = 0;
pub const ESAI_TSR_WIDTH: c_int = 24;

// Serial Audio Interface Status Register -- REG_ESAI_SAISR 0xCC
pub const ESAI_SAISR_TODFE_SHIFT: c_int = 17;

pub const ESAI_SAISR_TEDE_SHIFT: c_int = 16;

pub const ESAI_SAISR_TDE_SHIFT: c_int = 15;

pub const ESAI_SAISR_TUE_SHIFT: c_int = 14;

pub const ESAI_SAISR_TFS_SHIFT: c_int = 13;

pub const ESAI_SAISR_RODF_SHIFT: c_int = 10;

pub const ESAI_SAISR_REDF_SHIFT: c_int = 9;

pub const ESAI_SAISR_RDF_SHIFT: c_int = 8;

pub const ESAI_SAISR_ROE_SHIFT: c_int = 7;

pub const ESAI_SAISR_RFS_SHIFT: c_int = 6;

pub const ESAI_SAISR_IF2_SHIFT: c_int = 2;

pub const ESAI_SAISR_IF1_SHIFT: c_int = 1;

pub const ESAI_SAISR_IF0_SHIFT: c_int = 0;

// Serial Audio Interface Control Register -- REG_ESAI_SAICR 0xD0
pub const ESAI_SAICR_ALC_SHIFT: c_int = 8;

pub const ESAI_SAICR_TEBE_SHIFT: c_int = 7;

pub const ESAI_SAICR_SYNC_SHIFT: c_int = 6;

pub const ESAI_SAICR_OF2_SHIFT: c_int = 2;

pub const ESAI_SAICR_OF1_SHIFT: c_int = 1;

pub const ESAI_SAICR_OF0_SHIFT: c_int = 0;

//
// Transmit Control Register -- REG_ESAI_TCR 0xD4
// Receive Control Register -- REG_ESAI_RCR 0xDC
//
pub const ESAI_xCR_xLIE_SHIFT: c_int = 23;

pub const ESAI_xCR_xIE_SHIFT: c_int = 22;

pub const ESAI_xCR_xEDIE_SHIFT: c_int = 21;

pub const ESAI_xCR_xEIE_SHIFT: c_int = 20;

pub const ESAI_xCR_xPR_SHIFT: c_int = 19;

pub const ESAI_xCR_PADC_SHIFT: c_int = 17;

pub const ESAI_xCR_xFSR_SHIFT: c_int = 16;

pub const ESAI_xCR_xFSL_SHIFT: c_int = 15;

pub const ESAI_xCR_xSWS_SHIFT: c_int = 10;
pub const ESAI_xCR_xSWS_WIDTH: c_int = 5;

pub const ESAI_xCR_xMOD_SHIFT: c_int = 8;
pub const ESAI_xCR_xMOD_WIDTH: c_int = 2;

pub const ESAI_xCR_xWA_SHIFT: c_int = 7;

pub const ESAI_xCR_xSHFD_SHIFT: c_int = 6;

pub const ESAI_xCR_xE_SHIFT: c_int = 0;
pub const ESAI_xCR_TE_WIDTH: c_int = 6;
pub const ESAI_xCR_RE_WIDTH: c_int = 4;

//
// Transmit Clock Control Register -- REG_ESAI_TCCR 0xD8
// Receive Clock Control Register -- REG_ESAI_RCCR 0xE0
//
pub const ESAI_xCCR_xHCKD_SHIFT: c_int = 23;

pub const ESAI_xCCR_xFSD_SHIFT: c_int = 22;

pub const ESAI_xCCR_xCKD_SHIFT: c_int = 21;

pub const ESAI_xCCR_xHCKP_SHIFT: c_int = 20;

pub const ESAI_xCCR_xFSP_SHIFT: c_int = 19;

pub const ESAI_xCCR_xCKP_SHIFT: c_int = 18;

pub const ESAI_xCCR_xFP_SHIFT: c_int = 14;
pub const ESAI_xCCR_xFP_WIDTH: c_int = 4;

pub const ESAI_xCCR_xDC_SHIFT: c_int = 9;
pub const ESAI_xCCR_xDC_WIDTH: c_int = 5;

pub const ESAI_xCCR_xPSR_SHIFT: c_int = 8;

pub const ESAI_xCCR_xPM_SHIFT: c_int = 0;
pub const ESAI_xCCR_xPM_WIDTH: c_int = 8;

// Transmit Slot Mask Register A/B -- REG_ESAI_TSMA/B 0xE4 ~ 0xF0
pub const ESAI_xSMA_xS_SHIFT: c_int = 0;
pub const ESAI_xSMA_xS_WIDTH: c_int = 16;

pub const ESAI_xSMB_xS_SHIFT: c_int = 0;
pub const ESAI_xSMB_xS_WIDTH: c_int = 16;

// Port C Direction Register -- REG_ESAI_PRRC 0xF8
pub const ESAI_PRRC_PDC_SHIFT: c_int = 0;
pub const ESAI_PRRC_PDC_WIDTH: c_int = 12;

// Port C Control Register -- REG_ESAI_PCRC 0xFC
pub const ESAI_PCRC_PC_SHIFT: c_int = 0;
pub const ESAI_PCRC_PC_WIDTH: c_int = 12;

pub const ESAI_GPIO: c_uint = 0xfff;
// ESAI clock source
pub const ESAI_HCKT_FSYS: c_int = 0;
pub const ESAI_HCKT_EXTAL: c_int = 1;
pub const ESAI_HCKR_FSYS: c_int = 2;
pub const ESAI_HCKR_EXTAL: c_int = 3;
// ESAI clock divider
pub const ESAI_TX_DIV_PSR: c_int = 0;
pub const ESAI_TX_DIV_PM: c_int = 1;
pub const ESAI_TX_DIV_FP: c_int = 2;
pub const ESAI_RX_DIV_PSR: c_int = 3;
pub const ESAI_RX_DIV_PM: c_int = 4;
pub const ESAI_RX_DIV_FP: c_int = 5;
