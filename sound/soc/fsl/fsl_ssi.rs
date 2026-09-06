//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_ssi.h
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
// fsl_ssi.h - ALSA SSI interface for the Freescale MPC8610 and i.MX SoC
//
// Author: Timur Tabi <timur@freescale.com>
//
// Copyright 2007-2008 Freescale Semiconductor, Inc.
//
// -- SSI Register Map --
// SSI Transmit Data Register 0
pub const REG_SSI_STX0: c_uint = 0x00;
// SSI Transmit Data Register 1
pub const REG_SSI_STX1: c_uint = 0x04;
// SSI Receive Data Register 0
pub const REG_SSI_SRX0: c_uint = 0x08;
// SSI Receive Data Register 1
pub const REG_SSI_SRX1: c_uint = 0x0c;
// SSI Control Register
pub const REG_SSI_SCR: c_uint = 0x10;
// SSI Interrupt Status Register
pub const REG_SSI_SISR: c_uint = 0x14;
// SSI Interrupt Enable Register
pub const REG_SSI_SIER: c_uint = 0x18;
// SSI Transmit Configuration Register
pub const REG_SSI_STCR: c_uint = 0x1c;
// SSI Receive Configuration Register
pub const REG_SSI_SRCR: c_uint = 0x20;

// SSI Transmit Clock Control Register
pub const REG_SSI_STCCR: c_uint = 0x24;
// SSI Receive Clock Control Register
pub const REG_SSI_SRCCR: c_uint = 0x28;

// SSI FIFO Control/Status Register
pub const REG_SSI_SFCSR: c_uint = 0x2c;
//
// SSI Test Register (Intended for debugging purposes only)
//
// Note: STR is not documented in recent IMX datasheet, but
// is described in IMX51 reference manual at section 56.3.3.14
//
pub const REG_SSI_STR: c_uint = 0x30;
//
// SSI Option Register (Intended for internal use only)
//
// Note: SOR is not documented in recent IMX datasheet, but
// is described in IMX51 reference manual at section 56.3.3.15
//
pub const REG_SSI_SOR: c_uint = 0x34;
// SSI AC97 Control Register
pub const REG_SSI_SACNT: c_uint = 0x38;
// SSI AC97 Command Address Register
pub const REG_SSI_SACADD: c_uint = 0x3c;
// SSI AC97 Command Data Register
pub const REG_SSI_SACDAT: c_uint = 0x40;
// SSI AC97 Tag Register
pub const REG_SSI_SATAG: c_uint = 0x44;
// SSI Transmit Time Slot Mask Register
pub const REG_SSI_STMSK: c_uint = 0x48;
// SSI  Receive Time Slot Mask Register
pub const REG_SSI_SRMSK: c_uint = 0x4c;

//
// SSI AC97 Channel Status Register
//
// The status could be changed by:
// 1) Writing a '1' bit at some position in SACCEN sets relevant bit in SACCST
// 2) Writing a '1' bit at some position in SACCDIS unsets the relevant bit
// 3) Receivng a '1' in SLOTREQ bit from external CODEC via AC Link
//
pub const REG_SSI_SACCST: c_uint = 0x50;
// SSI AC97 Channel Enable Register -- Set bits in SACCST
pub const REG_SSI_SACCEN: c_uint = 0x54;
// SSI AC97 Channel Disable Register -- Clear bits in SACCST
pub const REG_SSI_SACCDIS: c_uint = 0x58;
// -- SSI Register Field Maps --
// SSI Control Register -- REG_SSI_SCR 0x10
pub const SSI_SCR_SYNC_TX_FS: c_uint = 0x00001000;
pub const SSI_SCR_RFR_CLK_DIS: c_uint = 0x00000800;
pub const SSI_SCR_TFR_CLK_DIS: c_uint = 0x00000400;
pub const SSI_SCR_TCH_EN: c_uint = 0x00000100;
pub const SSI_SCR_SYS_CLK_EN: c_uint = 0x00000080;
pub const SSI_SCR_I2S_MODE_MASK: c_uint = 0x00000060;
pub const SSI_SCR_I2S_MODE_NORMAL: c_uint = 0x00000000;
pub const SSI_SCR_I2S_MODE_MASTER: c_uint = 0x00000020;
pub const SSI_SCR_I2S_MODE_SLAVE: c_uint = 0x00000040;
pub const SSI_SCR_SYN: c_uint = 0x00000010;
pub const SSI_SCR_NET: c_uint = 0x00000008;

pub const SSI_SCR_RE: c_uint = 0x00000004;
pub const SSI_SCR_TE: c_uint = 0x00000002;
pub const SSI_SCR_SSIEN: c_uint = 0x00000001;
// SSI Interrupt Status Register -- REG_SSI_SISR 0x14
pub const SSI_SISR_RFRC: c_uint = 0x01000000;
pub const SSI_SISR_TFRC: c_uint = 0x00800000;
pub const SSI_SISR_CMDAU: c_uint = 0x00040000;
pub const SSI_SISR_CMDDU: c_uint = 0x00020000;
pub const SSI_SISR_RXT: c_uint = 0x00010000;
pub const SSI_SISR_RDR1: c_uint = 0x00008000;
pub const SSI_SISR_RDR0: c_uint = 0x00004000;
pub const SSI_SISR_TDE1: c_uint = 0x00002000;
pub const SSI_SISR_TDE0: c_uint = 0x00001000;
pub const SSI_SISR_ROE1: c_uint = 0x00000800;
pub const SSI_SISR_ROE0: c_uint = 0x00000400;
pub const SSI_SISR_TUE1: c_uint = 0x00000200;
pub const SSI_SISR_TUE0: c_uint = 0x00000100;
pub const SSI_SISR_TFS: c_uint = 0x00000080;
pub const SSI_SISR_RFS: c_uint = 0x00000040;
pub const SSI_SISR_TLS: c_uint = 0x00000020;
pub const SSI_SISR_RLS: c_uint = 0x00000010;
pub const SSI_SISR_RFF1: c_uint = 0x00000008;
pub const SSI_SISR_RFF0: c_uint = 0x00000004;
pub const SSI_SISR_TFE1: c_uint = 0x00000002;
pub const SSI_SISR_TFE0: c_uint = 0x00000001;
// SSI Interrupt Enable Register -- REG_SSI_SIER 0x18
pub const SSI_SIER_RFRC_EN: c_uint = 0x01000000;
pub const SSI_SIER_TFRC_EN: c_uint = 0x00800000;
pub const SSI_SIER_RDMAE: c_uint = 0x00400000;
pub const SSI_SIER_RIE: c_uint = 0x00200000;
pub const SSI_SIER_TDMAE: c_uint = 0x00100000;
pub const SSI_SIER_TIE: c_uint = 0x00080000;
pub const SSI_SIER_CMDAU_EN: c_uint = 0x00040000;
pub const SSI_SIER_CMDDU_EN: c_uint = 0x00020000;
pub const SSI_SIER_RXT_EN: c_uint = 0x00010000;
pub const SSI_SIER_RDR1_EN: c_uint = 0x00008000;
pub const SSI_SIER_RDR0_EN: c_uint = 0x00004000;
pub const SSI_SIER_TDE1_EN: c_uint = 0x00002000;
pub const SSI_SIER_TDE0_EN: c_uint = 0x00001000;
pub const SSI_SIER_ROE1_EN: c_uint = 0x00000800;
pub const SSI_SIER_ROE0_EN: c_uint = 0x00000400;
pub const SSI_SIER_TUE1_EN: c_uint = 0x00000200;
pub const SSI_SIER_TUE0_EN: c_uint = 0x00000100;
pub const SSI_SIER_TFS_EN: c_uint = 0x00000080;
pub const SSI_SIER_RFS_EN: c_uint = 0x00000040;
pub const SSI_SIER_TLS_EN: c_uint = 0x00000020;
pub const SSI_SIER_RLS_EN: c_uint = 0x00000010;
pub const SSI_SIER_RFF1_EN: c_uint = 0x00000008;
pub const SSI_SIER_RFF0_EN: c_uint = 0x00000004;
pub const SSI_SIER_TFE1_EN: c_uint = 0x00000002;
pub const SSI_SIER_TFE0_EN: c_uint = 0x00000001;
// SSI Transmit Configuration Register -- REG_SSI_STCR 0x1C
pub const SSI_STCR_TXBIT0: c_uint = 0x00000200;
pub const SSI_STCR_TFEN1: c_uint = 0x00000100;
pub const SSI_STCR_TFEN0: c_uint = 0x00000080;
pub const SSI_STCR_TFDIR: c_uint = 0x00000040;
pub const SSI_STCR_TXDIR: c_uint = 0x00000020;
pub const SSI_STCR_TSHFD: c_uint = 0x00000010;
pub const SSI_STCR_TSCKP: c_uint = 0x00000008;
pub const SSI_STCR_TFSI: c_uint = 0x00000004;
pub const SSI_STCR_TFSL: c_uint = 0x00000002;
pub const SSI_STCR_TEFS: c_uint = 0x00000001;
// SSI Receive Configuration Register -- REG_SSI_SRCR 0x20
pub const SSI_SRCR_RXEXT: c_uint = 0x00000400;
pub const SSI_SRCR_RXBIT0: c_uint = 0x00000200;
pub const SSI_SRCR_RFEN1: c_uint = 0x00000100;
pub const SSI_SRCR_RFEN0: c_uint = 0x00000080;
pub const SSI_SRCR_RFDIR: c_uint = 0x00000040;
pub const SSI_SRCR_RXDIR: c_uint = 0x00000020;
pub const SSI_SRCR_RSHFD: c_uint = 0x00000010;
pub const SSI_SRCR_RSCKP: c_uint = 0x00000008;
pub const SSI_SRCR_RFSI: c_uint = 0x00000004;
pub const SSI_SRCR_RFSL: c_uint = 0x00000002;
pub const SSI_SRCR_REFS: c_uint = 0x00000001;
//
// SSI Transmit Clock Control Register -- REG_SSI_STCCR 0x24
// SSI Receive Clock Control Register -- REG_SSI_SRCCR 0x28
//
pub const SSI_SxCCR_DIV2_SHIFT: c_int = 18;
pub const SSI_SxCCR_DIV2: c_uint = 0x00040000;
pub const SSI_SxCCR_PSR_SHIFT: c_int = 17;
pub const SSI_SxCCR_PSR: c_uint = 0x00020000;
pub const SSI_SxCCR_WL_SHIFT: c_int = 13;
pub const SSI_SxCCR_WL_MASK: c_uint = 0x0001E000;

pub const SSI_SxCCR_DC_SHIFT: c_int = 8;
pub const SSI_SxCCR_DC_MASK: c_uint = 0x00001F00;

pub const SSI_SxCCR_PM_SHIFT: c_int = 0;
pub const SSI_SxCCR_PM_MASK: c_uint = 0x000000FF;

//
// SSI FIFO Control/Status Register -- REG_SSI_SFCSR 0x2c
//
// Tx or Rx FIFO Counter -- SSI_SFCSR_xFCNTy Read-Only
// Tx or Rx FIFO Watermarks -- SSI_SFCSR_xFWMy Read/Write
//
pub const SSI_SFCSR_RFCNT1_SHIFT: c_int = 28;
pub const SSI_SFCSR_RFCNT1_MASK: c_uint = 0xF0000000;

pub const SSI_SFCSR_TFCNT1_SHIFT: c_int = 24;
pub const SSI_SFCSR_TFCNT1_MASK: c_uint = 0x0F000000;

pub const SSI_SFCSR_RFWM1_SHIFT: c_int = 20;
pub const SSI_SFCSR_RFWM1_MASK: c_uint = 0x00F00000;

pub const SSI_SFCSR_TFWM1_SHIFT: c_int = 16;
pub const SSI_SFCSR_TFWM1_MASK: c_uint = 0x000F0000;

pub const SSI_SFCSR_RFCNT0_SHIFT: c_int = 12;
pub const SSI_SFCSR_RFCNT0_MASK: c_uint = 0x0000F000;

pub const SSI_SFCSR_TFCNT0_SHIFT: c_int = 8;
pub const SSI_SFCSR_TFCNT0_MASK: c_uint = 0x00000F00;

pub const SSI_SFCSR_RFWM0_SHIFT: c_int = 4;
pub const SSI_SFCSR_RFWM0_MASK: c_uint = 0x000000F0;

pub const SSI_SFCSR_TFWM0_SHIFT: c_int = 0;
pub const SSI_SFCSR_TFWM0_MASK: c_uint = 0x0000000F;

// SSI Test Register -- REG_SSI_STR 0x30
pub const SSI_STR_TEST: c_uint = 0x00008000;
pub const SSI_STR_RCK2TCK: c_uint = 0x00004000;
pub const SSI_STR_RFS2TFS: c_uint = 0x00002000;

pub const SSI_STR_TXD2RXD: c_uint = 0x00000080;
pub const SSI_STR_TCK2RCK: c_uint = 0x00000040;
pub const SSI_STR_TFS2RFS: c_uint = 0x00000020;

// SSI Option Register -- REG_SSI_SOR 0x34
pub const SSI_SOR_CLKOFF: c_uint = 0x00000040;
pub const SSI_SOR_RX_CLR: c_uint = 0x00000020;
pub const SSI_SOR_TX_CLR: c_uint = 0x00000010;

pub const SSI_SOR_INIT: c_uint = 0x00000008;
pub const SSI_SOR_WAIT_SHIFT: c_int = 1;
pub const SSI_SOR_WAIT_MASK: c_uint = 0x00000006;

pub const SSI_SOR_SYNRST: c_uint = 0x00000001;
// SSI AC97 Control Register -- REG_SSI_SACNT 0x38

pub const SSI_SACNT_WR: c_uint = 0x00000010;
pub const SSI_SACNT_RD: c_uint = 0x00000008;
pub const SSI_SACNT_RDWR_MASK: c_uint = 0x00000018;
pub const SSI_SACNT_TIF: c_uint = 0x00000004;
pub const SSI_SACNT_FV: c_uint = 0x00000002;
pub const SSI_SACNT_AC97EN: c_uint = 0x00000001;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ssi_dbg {
    pub dbg_dir: *mut dentry,
    pub rfrc: c_uint,
    pub tfrc: c_uint,
    pub cmdau: c_uint,
    pub cmddu: c_uint,
    pub rxt: c_uint,
    pub rdr1: c_uint,
    pub rdr0: c_uint,
    pub tde1: c_uint,
    pub tde0: c_uint,
    pub roe1: c_uint,
    pub roe0: c_uint,
    pub tue1: c_uint,
    pub tue0: c_uint,
    pub tfs: c_uint,
    pub rfs: c_uint,
    pub tls: c_uint,
    pub rls: c_uint,
    pub rff1: c_uint,
    pub rff0: c_uint,
    pub tfe1: c_uint,
    pub tfe0: c_uint,
    pub stats: },
}

extern "C" {
    pub fn fsl_ssi_dbg_isr(ssi_dbg: *mut fsl_ssi_dbg, sisr: u32);
}
extern "C" {
    pub fn fsl_ssi_debugfs_create(ssi_dbg: *mut fsl_ssi_dbg, dev: *mut device);
}
extern "C" {
    pub fn fsl_ssi_debugfs_remove(ssi_dbg: *mut fsl_ssi_dbg);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ssi_dbg {
}

