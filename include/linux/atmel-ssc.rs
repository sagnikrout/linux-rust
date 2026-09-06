//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/atmel-ssc.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ssc_platform_data {
    pub use_dma: c_int,
    pub has_fslen_ext: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssc_device {
    pub list: list_head,
    pub phybase: dma_addr_t,
    pub regs: *mut void __iomem,
    pub pdev: *mut platform_device,
    pub pdata: *mut atmel_ssc_platform_data,
    pub clk: *mut clk,
    pub user: c_int,
    pub irq: c_int,
    pub clk_from_rk_pin: bool,
    pub sound_dai: bool,
}

extern "C" {
    pub fn ssc_request(ssc_num: c_uint) -> *mut ssc_device  __must_check;
}
extern "C" {
    pub fn ssc_free(ssc: *mut ssc_device);
}
// SSC register offsets
// SSC Control Register
pub const SSC_CR: c_uint = 0x00000000;
pub const SSC_CR_RXDIS_SIZE: c_int = 1;
pub const SSC_CR_RXDIS_OFFSET: c_int = 1;
pub const SSC_CR_RXEN_SIZE: c_int = 1;
pub const SSC_CR_RXEN_OFFSET: c_int = 0;
pub const SSC_CR_SWRST_SIZE: c_int = 1;
pub const SSC_CR_SWRST_OFFSET: c_int = 15;
pub const SSC_CR_TXDIS_SIZE: c_int = 1;
pub const SSC_CR_TXDIS_OFFSET: c_int = 9;
pub const SSC_CR_TXEN_SIZE: c_int = 1;
pub const SSC_CR_TXEN_OFFSET: c_int = 8;
// SSC Clock Mode Register
pub const SSC_CMR: c_uint = 0x00000004;
pub const SSC_CMR_DIV_SIZE: c_int = 12;
pub const SSC_CMR_DIV_OFFSET: c_int = 0;
// SSC Receive Clock Mode Register
pub const SSC_RCMR: c_uint = 0x00000010;
pub const SSC_RCMR_CKG_SIZE: c_int = 2;
pub const SSC_RCMR_CKG_OFFSET: c_int = 6;
pub const SSC_RCMR_CKI_SIZE: c_int = 1;
pub const SSC_RCMR_CKI_OFFSET: c_int = 5;
pub const SSC_RCMR_CKO_SIZE: c_int = 3;
pub const SSC_RCMR_CKO_OFFSET: c_int = 2;
pub const SSC_RCMR_CKS_SIZE: c_int = 2;
pub const SSC_RCMR_CKS_OFFSET: c_int = 0;
pub const SSC_RCMR_PERIOD_SIZE: c_int = 8;
pub const SSC_RCMR_PERIOD_OFFSET: c_int = 24;
pub const SSC_RCMR_START_SIZE: c_int = 4;
pub const SSC_RCMR_START_OFFSET: c_int = 8;
pub const SSC_RCMR_STOP_SIZE: c_int = 1;
pub const SSC_RCMR_STOP_OFFSET: c_int = 12;
pub const SSC_RCMR_STTDLY_SIZE: c_int = 8;
pub const SSC_RCMR_STTDLY_OFFSET: c_int = 16;
// SSC Receive Frame Mode Register
pub const SSC_RFMR: c_uint = 0x00000014;
pub const SSC_RFMR_DATLEN_SIZE: c_int = 5;
pub const SSC_RFMR_DATLEN_OFFSET: c_int = 0;
pub const SSC_RFMR_DATNB_SIZE: c_int = 4;
pub const SSC_RFMR_DATNB_OFFSET: c_int = 8;
pub const SSC_RFMR_FSEDGE_SIZE: c_int = 1;
pub const SSC_RFMR_FSEDGE_OFFSET: c_int = 24;
//
// The FSLEN_EXT exist on at91sam9rl, at91sam9g10,
// at91sam9g20, and at91sam9g45 and newer SoCs
//
pub const SSC_RFMR_FSLEN_EXT_SIZE: c_int = 4;
pub const SSC_RFMR_FSLEN_EXT_OFFSET: c_int = 28;
pub const SSC_RFMR_FSLEN_SIZE: c_int = 4;
pub const SSC_RFMR_FSLEN_OFFSET: c_int = 16;
pub const SSC_RFMR_FSOS_SIZE: c_int = 4;
pub const SSC_RFMR_FSOS_OFFSET: c_int = 20;
pub const SSC_RFMR_LOOP_SIZE: c_int = 1;
pub const SSC_RFMR_LOOP_OFFSET: c_int = 5;
pub const SSC_RFMR_MSBF_SIZE: c_int = 1;
pub const SSC_RFMR_MSBF_OFFSET: c_int = 7;
// SSC Transmit Clock Mode Register
pub const SSC_TCMR: c_uint = 0x00000018;
pub const SSC_TCMR_CKG_SIZE: c_int = 2;
pub const SSC_TCMR_CKG_OFFSET: c_int = 6;
pub const SSC_TCMR_CKI_SIZE: c_int = 1;
pub const SSC_TCMR_CKI_OFFSET: c_int = 5;
pub const SSC_TCMR_CKO_SIZE: c_int = 3;
pub const SSC_TCMR_CKO_OFFSET: c_int = 2;
pub const SSC_TCMR_CKS_SIZE: c_int = 2;
pub const SSC_TCMR_CKS_OFFSET: c_int = 0;
pub const SSC_TCMR_PERIOD_SIZE: c_int = 8;
pub const SSC_TCMR_PERIOD_OFFSET: c_int = 24;
pub const SSC_TCMR_START_SIZE: c_int = 4;
pub const SSC_TCMR_START_OFFSET: c_int = 8;
pub const SSC_TCMR_STTDLY_SIZE: c_int = 8;
pub const SSC_TCMR_STTDLY_OFFSET: c_int = 16;
// SSC Transmit Frame Mode Register
pub const SSC_TFMR: c_uint = 0x0000001c;
pub const SSC_TFMR_DATDEF_SIZE: c_int = 1;
pub const SSC_TFMR_DATDEF_OFFSET: c_int = 5;
pub const SSC_TFMR_DATLEN_SIZE: c_int = 5;
pub const SSC_TFMR_DATLEN_OFFSET: c_int = 0;
pub const SSC_TFMR_DATNB_SIZE: c_int = 4;
pub const SSC_TFMR_DATNB_OFFSET: c_int = 8;
pub const SSC_TFMR_FSDEN_SIZE: c_int = 1;
pub const SSC_TFMR_FSDEN_OFFSET: c_int = 23;
pub const SSC_TFMR_FSEDGE_SIZE: c_int = 1;
pub const SSC_TFMR_FSEDGE_OFFSET: c_int = 24;
//
// The FSLEN_EXT exist on at91sam9rl, at91sam9g10,
// at91sam9g20, and at91sam9g45 and newer SoCs
//
pub const SSC_TFMR_FSLEN_EXT_SIZE: c_int = 4;
pub const SSC_TFMR_FSLEN_EXT_OFFSET: c_int = 28;
pub const SSC_TFMR_FSLEN_SIZE: c_int = 4;
pub const SSC_TFMR_FSLEN_OFFSET: c_int = 16;
pub const SSC_TFMR_FSOS_SIZE: c_int = 3;
pub const SSC_TFMR_FSOS_OFFSET: c_int = 20;
pub const SSC_TFMR_MSBF_SIZE: c_int = 1;
pub const SSC_TFMR_MSBF_OFFSET: c_int = 7;
// SSC Receive Hold Register
pub const SSC_RHR: c_uint = 0x00000020;
pub const SSC_RHR_RDAT_SIZE: c_int = 32;
pub const SSC_RHR_RDAT_OFFSET: c_int = 0;
// SSC Transmit Hold Register
pub const SSC_THR: c_uint = 0x00000024;
pub const SSC_THR_TDAT_SIZE: c_int = 32;
pub const SSC_THR_TDAT_OFFSET: c_int = 0;
// SSC Receive Sync. Holding Register
pub const SSC_RSHR: c_uint = 0x00000030;
pub const SSC_RSHR_RSDAT_SIZE: c_int = 16;
pub const SSC_RSHR_RSDAT_OFFSET: c_int = 0;
// SSC Transmit Sync. Holding Register
pub const SSC_TSHR: c_uint = 0x00000034;
pub const SSC_TSHR_TSDAT_SIZE: c_int = 16;
pub const SSC_TSHR_RSDAT_OFFSET: c_int = 0;
// SSC Receive Compare 0 Register
pub const SSC_RC0R: c_uint = 0x00000038;
pub const SSC_RC0R_CP0_SIZE: c_int = 16;
pub const SSC_RC0R_CP0_OFFSET: c_int = 0;
// SSC Receive Compare 1 Register
pub const SSC_RC1R: c_uint = 0x0000003c;
pub const SSC_RC1R_CP1_SIZE: c_int = 16;
pub const SSC_RC1R_CP1_OFFSET: c_int = 0;
// SSC Status Register
pub const SSC_SR: c_uint = 0x00000040;
pub const SSC_SR_CP0_SIZE: c_int = 1;
pub const SSC_SR_CP0_OFFSET: c_int = 8;
pub const SSC_SR_CP1_SIZE: c_int = 1;
pub const SSC_SR_CP1_OFFSET: c_int = 9;
pub const SSC_SR_ENDRX_SIZE: c_int = 1;
pub const SSC_SR_ENDRX_OFFSET: c_int = 6;
pub const SSC_SR_ENDTX_SIZE: c_int = 1;
pub const SSC_SR_ENDTX_OFFSET: c_int = 2;
pub const SSC_SR_OVRUN_SIZE: c_int = 1;
pub const SSC_SR_OVRUN_OFFSET: c_int = 5;
pub const SSC_SR_RXBUFF_SIZE: c_int = 1;
pub const SSC_SR_RXBUFF_OFFSET: c_int = 7;
pub const SSC_SR_RXEN_SIZE: c_int = 1;
pub const SSC_SR_RXEN_OFFSET: c_int = 17;
pub const SSC_SR_RXRDY_SIZE: c_int = 1;
pub const SSC_SR_RXRDY_OFFSET: c_int = 4;
pub const SSC_SR_RXSYN_SIZE: c_int = 1;
pub const SSC_SR_RXSYN_OFFSET: c_int = 11;
pub const SSC_SR_TXBUFE_SIZE: c_int = 1;
pub const SSC_SR_TXBUFE_OFFSET: c_int = 3;
pub const SSC_SR_TXEMPTY_SIZE: c_int = 1;
pub const SSC_SR_TXEMPTY_OFFSET: c_int = 1;
pub const SSC_SR_TXEN_SIZE: c_int = 1;
pub const SSC_SR_TXEN_OFFSET: c_int = 16;
pub const SSC_SR_TXRDY_SIZE: c_int = 1;
pub const SSC_SR_TXRDY_OFFSET: c_int = 0;
pub const SSC_SR_TXSYN_SIZE: c_int = 1;
pub const SSC_SR_TXSYN_OFFSET: c_int = 10;
// SSC Interrupt Enable Register
pub const SSC_IER: c_uint = 0x00000044;
pub const SSC_IER_CP0_SIZE: c_int = 1;
pub const SSC_IER_CP0_OFFSET: c_int = 8;
pub const SSC_IER_CP1_SIZE: c_int = 1;
pub const SSC_IER_CP1_OFFSET: c_int = 9;
pub const SSC_IER_ENDRX_SIZE: c_int = 1;
pub const SSC_IER_ENDRX_OFFSET: c_int = 6;
pub const SSC_IER_ENDTX_SIZE: c_int = 1;
pub const SSC_IER_ENDTX_OFFSET: c_int = 2;
pub const SSC_IER_OVRUN_SIZE: c_int = 1;
pub const SSC_IER_OVRUN_OFFSET: c_int = 5;
pub const SSC_IER_RXBUFF_SIZE: c_int = 1;
pub const SSC_IER_RXBUFF_OFFSET: c_int = 7;
pub const SSC_IER_RXRDY_SIZE: c_int = 1;
pub const SSC_IER_RXRDY_OFFSET: c_int = 4;
pub const SSC_IER_RXSYN_SIZE: c_int = 1;
pub const SSC_IER_RXSYN_OFFSET: c_int = 11;
pub const SSC_IER_TXBUFE_SIZE: c_int = 1;
pub const SSC_IER_TXBUFE_OFFSET: c_int = 3;
pub const SSC_IER_TXEMPTY_SIZE: c_int = 1;
pub const SSC_IER_TXEMPTY_OFFSET: c_int = 1;
pub const SSC_IER_TXRDY_SIZE: c_int = 1;
pub const SSC_IER_TXRDY_OFFSET: c_int = 0;
pub const SSC_IER_TXSYN_SIZE: c_int = 1;
pub const SSC_IER_TXSYN_OFFSET: c_int = 10;
// SSC Interrupt Disable Register
pub const SSC_IDR: c_uint = 0x00000048;
pub const SSC_IDR_CP0_SIZE: c_int = 1;
pub const SSC_IDR_CP0_OFFSET: c_int = 8;
pub const SSC_IDR_CP1_SIZE: c_int = 1;
pub const SSC_IDR_CP1_OFFSET: c_int = 9;
pub const SSC_IDR_ENDRX_SIZE: c_int = 1;
pub const SSC_IDR_ENDRX_OFFSET: c_int = 6;
pub const SSC_IDR_ENDTX_SIZE: c_int = 1;
pub const SSC_IDR_ENDTX_OFFSET: c_int = 2;
pub const SSC_IDR_OVRUN_SIZE: c_int = 1;
pub const SSC_IDR_OVRUN_OFFSET: c_int = 5;
pub const SSC_IDR_RXBUFF_SIZE: c_int = 1;
pub const SSC_IDR_RXBUFF_OFFSET: c_int = 7;
pub const SSC_IDR_RXRDY_SIZE: c_int = 1;
pub const SSC_IDR_RXRDY_OFFSET: c_int = 4;
pub const SSC_IDR_RXSYN_SIZE: c_int = 1;
pub const SSC_IDR_RXSYN_OFFSET: c_int = 11;
pub const SSC_IDR_TXBUFE_SIZE: c_int = 1;
pub const SSC_IDR_TXBUFE_OFFSET: c_int = 3;
pub const SSC_IDR_TXEMPTY_SIZE: c_int = 1;
pub const SSC_IDR_TXEMPTY_OFFSET: c_int = 1;
pub const SSC_IDR_TXRDY_SIZE: c_int = 1;
pub const SSC_IDR_TXRDY_OFFSET: c_int = 0;
pub const SSC_IDR_TXSYN_SIZE: c_int = 1;
pub const SSC_IDR_TXSYN_OFFSET: c_int = 10;
// SSC Interrupt Mask Register
pub const SSC_IMR: c_uint = 0x0000004c;
pub const SSC_IMR_CP0_SIZE: c_int = 1;
pub const SSC_IMR_CP0_OFFSET: c_int = 8;
pub const SSC_IMR_CP1_SIZE: c_int = 1;
pub const SSC_IMR_CP1_OFFSET: c_int = 9;
pub const SSC_IMR_ENDRX_SIZE: c_int = 1;
pub const SSC_IMR_ENDRX_OFFSET: c_int = 6;
pub const SSC_IMR_ENDTX_SIZE: c_int = 1;
pub const SSC_IMR_ENDTX_OFFSET: c_int = 2;
pub const SSC_IMR_OVRUN_SIZE: c_int = 1;
pub const SSC_IMR_OVRUN_OFFSET: c_int = 5;
pub const SSC_IMR_RXBUFF_SIZE: c_int = 1;
pub const SSC_IMR_RXBUFF_OFFSET: c_int = 7;
pub const SSC_IMR_RXRDY_SIZE: c_int = 1;
pub const SSC_IMR_RXRDY_OFFSET: c_int = 4;
pub const SSC_IMR_RXSYN_SIZE: c_int = 1;
pub const SSC_IMR_RXSYN_OFFSET: c_int = 11;
pub const SSC_IMR_TXBUFE_SIZE: c_int = 1;
pub const SSC_IMR_TXBUFE_OFFSET: c_int = 3;
pub const SSC_IMR_TXEMPTY_SIZE: c_int = 1;
pub const SSC_IMR_TXEMPTY_OFFSET: c_int = 1;
pub const SSC_IMR_TXRDY_SIZE: c_int = 1;
pub const SSC_IMR_TXRDY_OFFSET: c_int = 0;
pub const SSC_IMR_TXSYN_SIZE: c_int = 1;
pub const SSC_IMR_TXSYN_OFFSET: c_int = 10;
// SSC PDC Receive Pointer Register
pub const SSC_PDC_RPR: c_uint = 0x00000100;
// SSC PDC Receive Counter Register
pub const SSC_PDC_RCR: c_uint = 0x00000104;
// SSC PDC Transmit Pointer Register
pub const SSC_PDC_TPR: c_uint = 0x00000108;
// SSC PDC Receive Next Pointer Register
pub const SSC_PDC_RNPR: c_uint = 0x00000110;
// SSC PDC Receive Next Counter Register
pub const SSC_PDC_RNCR: c_uint = 0x00000114;
// SSC PDC Transmit Counter Register
pub const SSC_PDC_TCR: c_uint = 0x0000010c;
// SSC PDC Transmit Next Pointer Register
pub const SSC_PDC_TNPR: c_uint = 0x00000118;
// SSC PDC Transmit Next Counter Register
pub const SSC_PDC_TNCR: c_uint = 0x0000011c;
// SSC PDC Transfer Control Register
pub const SSC_PDC_PTCR: c_uint = 0x00000120;
pub const SSC_PDC_PTCR_RXTDIS_SIZE: c_int = 1;
pub const SSC_PDC_PTCR_RXTDIS_OFFSET: c_int = 1;
pub const SSC_PDC_PTCR_RXTEN_SIZE: c_int = 1;
pub const SSC_PDC_PTCR_RXTEN_OFFSET: c_int = 0;
pub const SSC_PDC_PTCR_TXTDIS_SIZE: c_int = 1;
pub const SSC_PDC_PTCR_TXTDIS_OFFSET: c_int = 9;
pub const SSC_PDC_PTCR_TXTEN_SIZE: c_int = 1;
pub const SSC_PDC_PTCR_TXTEN_OFFSET: c_int = 8;
// SSC PDC Transfer Status Register
pub const SSC_PDC_PTSR: c_uint = 0x00000124;
pub const SSC_PDC_PTSR_RXTEN_SIZE: c_int = 1;
pub const SSC_PDC_PTSR_RXTEN_OFFSET: c_int = 0;
pub const SSC_PDC_PTSR_TXTEN_SIZE: c_int = 1;
pub const SSC_PDC_PTSR_TXTEN_OFFSET: c_int = 8;
// Bit manipulation macros

// Register access macros

