//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amd/xgbe/xgbe-common.h
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-3-Clause)
//
// Copyright (c) 2014-2025, Advanced Micro Devices, Inc.
// Copyright (c) 2014, Synopsys, Inc.
// All rights reserved
//
// DMA register offsets
pub const DMA_MR: c_uint = 0x3000;
pub const DMA_SBMR: c_uint = 0x3004;
pub const DMA_ISR: c_uint = 0x3008;
pub const DMA_AXIARCR: c_uint = 0x3010;
pub const DMA_AXIAWCR: c_uint = 0x3018;
pub const DMA_AXIAWARCR: c_uint = 0x301c;
pub const DMA_DSR0: c_uint = 0x3020;
pub const DMA_DSR1: c_uint = 0x3024;
pub const DMA_TXEDMACR: c_uint = 0x3040;
pub const DMA_RXEDMACR: c_uint = 0x3044;
// DMA register entry bit positions and sizes
pub const DMA_ISR_MACIS_INDEX: c_int = 17;
pub const DMA_ISR_MACIS_WIDTH: c_int = 1;
pub const DMA_ISR_MTLIS_INDEX: c_int = 16;
pub const DMA_ISR_MTLIS_WIDTH: c_int = 1;
pub const DMA_MR_INTM_INDEX: c_int = 12;
pub const DMA_MR_INTM_WIDTH: c_int = 2;
pub const DMA_MR_SWR_INDEX: c_int = 0;
pub const DMA_MR_SWR_WIDTH: c_int = 1;
pub const DMA_RXEDMACR_RDPS_INDEX: c_int = 0;
pub const DMA_RXEDMACR_RDPS_WIDTH: c_int = 3;
pub const DMA_SBMR_AAL_INDEX: c_int = 12;
pub const DMA_SBMR_AAL_WIDTH: c_int = 1;
pub const DMA_SBMR_EAME_INDEX: c_int = 11;
pub const DMA_SBMR_EAME_WIDTH: c_int = 1;
pub const DMA_SBMR_BLEN_INDEX: c_int = 1;
pub const DMA_SBMR_BLEN_WIDTH: c_int = 7;
pub const DMA_SBMR_RD_OSR_LMT_INDEX: c_int = 16;
pub const DMA_SBMR_RD_OSR_LMT_WIDTH: c_int = 6;
pub const DMA_SBMR_UNDEF_INDEX: c_int = 0;
pub const DMA_SBMR_UNDEF_WIDTH: c_int = 1;
pub const DMA_SBMR_WR_OSR_LMT_INDEX: c_int = 24;
pub const DMA_SBMR_WR_OSR_LMT_WIDTH: c_int = 6;
pub const DMA_TXEDMACR_TDPS_INDEX: c_int = 0;
pub const DMA_TXEDMACR_TDPS_WIDTH: c_int = 3;
// DMA register values
pub const DMA_SBMR_BLEN_256: c_int = 256;
pub const DMA_SBMR_BLEN_128: c_int = 128;
pub const DMA_SBMR_BLEN_64: c_int = 64;
pub const DMA_SBMR_BLEN_32: c_int = 32;
pub const DMA_SBMR_BLEN_16: c_int = 16;
pub const DMA_SBMR_BLEN_8: c_int = 8;
pub const DMA_SBMR_BLEN_4: c_int = 4;
pub const DMA_DSR_RPS_WIDTH: c_int = 4;
pub const DMA_DSR_TPS_WIDTH: c_int = 4;

pub const DMA_DSR0_RPS_START: c_int = 8;
pub const DMA_DSR0_TPS_START: c_int = 12;
pub const DMA_DSRX_FIRST_QUEUE: c_int = 3;
pub const DMA_DSRX_INC: c_int = 4;
pub const DMA_DSRX_QPR: c_int = 4;
pub const DMA_DSRX_RPS_START: c_int = 0;
pub const DMA_DSRX_TPS_START: c_int = 4;
pub const DMA_TPS_STOPPED: c_uint = 0x00;
pub const DMA_TPS_SUSPENDED: c_uint = 0x06;
// DMA channel register offsets
// Multiple channels can be active.  The first channel has registers
// that begin at 0x3100.  Each subsequent channel has registers that
// are accessed using an offset of 0x80 from the previous channel.
//
pub const DMA_CH_BASE: c_uint = 0x3100;
pub const DMA_CH_INC: c_uint = 0x80;
pub const DMA_CH_CR: c_uint = 0x00;
pub const DMA_CH_TCR: c_uint = 0x04;
pub const DMA_CH_RCR: c_uint = 0x08;
pub const DMA_CH_TDLR_HI: c_uint = 0x10;
pub const DMA_CH_TDLR_LO: c_uint = 0x14;
pub const DMA_CH_RDLR_HI: c_uint = 0x18;
pub const DMA_CH_RDLR_LO: c_uint = 0x1c;
pub const DMA_CH_TDTR_LO: c_uint = 0x24;
pub const DMA_CH_RDTR_LO: c_uint = 0x2c;
pub const DMA_CH_TDRLR: c_uint = 0x30;
pub const DMA_CH_RDRLR: c_uint = 0x34;
pub const DMA_CH_IER: c_uint = 0x38;
pub const DMA_CH_RIWT: c_uint = 0x3c;
pub const DMA_CH_CATDR_LO: c_uint = 0x44;
pub const DMA_CH_CARDR_LO: c_uint = 0x4c;
pub const DMA_CH_CATBR_HI: c_uint = 0x50;
pub const DMA_CH_CATBR_LO: c_uint = 0x54;
pub const DMA_CH_CARBR_HI: c_uint = 0x58;
pub const DMA_CH_CARBR_LO: c_uint = 0x5c;
pub const DMA_CH_SR: c_uint = 0x60;
// DMA channel register entry bit positions and sizes
pub const DMA_CH_CR_PBLX8_INDEX: c_int = 16;
pub const DMA_CH_CR_PBLX8_WIDTH: c_int = 1;
pub const DMA_CH_CR_SPH_INDEX: c_int = 24;
pub const DMA_CH_CR_SPH_WIDTH: c_int = 1;
pub const DMA_CH_IER_AIE20_INDEX: c_int = 15;
pub const DMA_CH_IER_AIE20_WIDTH: c_int = 1;
pub const DMA_CH_IER_AIE_INDEX: c_int = 14;
pub const DMA_CH_IER_AIE_WIDTH: c_int = 1;
pub const DMA_CH_IER_FBEE_INDEX: c_int = 12;
pub const DMA_CH_IER_FBEE_WIDTH: c_int = 1;
pub const DMA_CH_IER_NIE20_INDEX: c_int = 16;
pub const DMA_CH_IER_NIE20_WIDTH: c_int = 1;
pub const DMA_CH_IER_NIE_INDEX: c_int = 15;
pub const DMA_CH_IER_NIE_WIDTH: c_int = 1;
pub const DMA_CH_IER_RBUE_INDEX: c_int = 7;
pub const DMA_CH_IER_RBUE_WIDTH: c_int = 1;
pub const DMA_CH_IER_RIE_INDEX: c_int = 6;
pub const DMA_CH_IER_RIE_WIDTH: c_int = 1;
pub const DMA_CH_IER_RSE_INDEX: c_int = 8;
pub const DMA_CH_IER_RSE_WIDTH: c_int = 1;
pub const DMA_CH_IER_TBUE_INDEX: c_int = 2;
pub const DMA_CH_IER_TBUE_WIDTH: c_int = 1;
pub const DMA_CH_IER_TIE_INDEX: c_int = 0;
pub const DMA_CH_IER_TIE_WIDTH: c_int = 1;
pub const DMA_CH_IER_TXSE_INDEX: c_int = 1;
pub const DMA_CH_IER_TXSE_WIDTH: c_int = 1;
pub const DMA_CH_RCR_PBL_INDEX: c_int = 16;
pub const DMA_CH_RCR_PBL_WIDTH: c_int = 6;
pub const DMA_CH_RCR_RBSZ_INDEX: c_int = 1;
pub const DMA_CH_RCR_RBSZ_WIDTH: c_int = 14;
pub const DMA_CH_RCR_SR_INDEX: c_int = 0;
pub const DMA_CH_RCR_SR_WIDTH: c_int = 1;
pub const DMA_CH_RIWT_RWT_INDEX: c_int = 0;
pub const DMA_CH_RIWT_RWT_WIDTH: c_int = 8;
pub const DMA_CH_SR_FBE_INDEX: c_int = 12;
pub const DMA_CH_SR_FBE_WIDTH: c_int = 1;
pub const DMA_CH_SR_RBU_INDEX: c_int = 7;
pub const DMA_CH_SR_RBU_WIDTH: c_int = 1;
pub const DMA_CH_SR_RI_INDEX: c_int = 6;
pub const DMA_CH_SR_RI_WIDTH: c_int = 1;
pub const DMA_CH_SR_RPS_INDEX: c_int = 8;
pub const DMA_CH_SR_RPS_WIDTH: c_int = 1;
pub const DMA_CH_SR_TBU_INDEX: c_int = 2;
pub const DMA_CH_SR_TBU_WIDTH: c_int = 1;
pub const DMA_CH_SR_TI_INDEX: c_int = 0;
pub const DMA_CH_SR_TI_WIDTH: c_int = 1;
pub const DMA_CH_SR_TPS_INDEX: c_int = 1;
pub const DMA_CH_SR_TPS_WIDTH: c_int = 1;
pub const DMA_CH_TCR_OSP_INDEX: c_int = 4;
pub const DMA_CH_TCR_OSP_WIDTH: c_int = 1;
pub const DMA_CH_TCR_PBL_INDEX: c_int = 16;
pub const DMA_CH_TCR_PBL_WIDTH: c_int = 6;
pub const DMA_CH_TCR_ST_INDEX: c_int = 0;
pub const DMA_CH_TCR_ST_WIDTH: c_int = 1;
pub const DMA_CH_TCR_TSE_INDEX: c_int = 12;
pub const DMA_CH_TCR_TSE_WIDTH: c_int = 1;
// DMA channel register values
pub const DMA_OSP_DISABLE: c_uint = 0x00;
pub const DMA_OSP_ENABLE: c_uint = 0x01;
pub const DMA_PBL_1: c_int = 1;
pub const DMA_PBL_2: c_int = 2;
pub const DMA_PBL_4: c_int = 4;
pub const DMA_PBL_8: c_int = 8;
pub const DMA_PBL_16: c_int = 16;
pub const DMA_PBL_32: c_int = 32;

pub const DMA_PBL_X8_DISABLE: c_uint = 0x00;
pub const DMA_PBL_X8_ENABLE: c_uint = 0x01;
// MAC register offsets
pub const MAC_TCR: c_uint = 0x0000;
pub const MAC_RCR: c_uint = 0x0004;
pub const MAC_PFR: c_uint = 0x0008;
pub const MAC_WTR: c_uint = 0x000c;
pub const MAC_HTR0: c_uint = 0x0010;
pub const MAC_VLANTR: c_uint = 0x0050;
pub const MAC_VLANHTR: c_uint = 0x0058;
pub const MAC_VLANIR: c_uint = 0x0060;
pub const MAC_IVLANIR: c_uint = 0x0064;
pub const MAC_RETMR: c_uint = 0x006c;
pub const MAC_Q0TFCR: c_uint = 0x0070;
pub const MAC_RFCR: c_uint = 0x0090;
pub const MAC_RQC0R: c_uint = 0x00a0;
pub const MAC_RQC1R: c_uint = 0x00a4;
pub const MAC_RQC2R: c_uint = 0x00a8;
pub const MAC_RQC3R: c_uint = 0x00ac;
pub const MAC_ISR: c_uint = 0x00b0;
pub const MAC_IER: c_uint = 0x00b4;
pub const MAC_RTSR: c_uint = 0x00b8;
pub const MAC_PMTCSR: c_uint = 0x00c0;
pub const MAC_RWKPFR: c_uint = 0x00c4;
pub const MAC_LPICSR: c_uint = 0x00d0;
pub const MAC_LPITCR: c_uint = 0x00d4;
pub const MAC_TIR: c_uint = 0x00e0;
pub const MAC_VR: c_uint = 0x0110;
pub const MAC_DR: c_uint = 0x0114;
pub const MAC_HWF0R: c_uint = 0x011c;
pub const MAC_HWF1R: c_uint = 0x0120;
pub const MAC_HWF2R: c_uint = 0x0124;
pub const MAC_MDIOSCAR: c_uint = 0x0200;
pub const MAC_MDIOSCCDR: c_uint = 0x0204;
pub const MAC_MDIOISR: c_uint = 0x0214;
pub const MAC_MDIOIER: c_uint = 0x0218;
pub const MAC_MDIOCL22R: c_uint = 0x0220;
pub const MAC_GPIOCR: c_uint = 0x0278;
pub const MAC_GPIOSR: c_uint = 0x027c;
pub const MAC_MACA0HR: c_uint = 0x0300;
pub const MAC_MACA0LR: c_uint = 0x0304;
pub const MAC_MACA1HR: c_uint = 0x0308;
pub const MAC_MACA1LR: c_uint = 0x030c;
pub const MAC_RSSCR: c_uint = 0x0c80;
pub const MAC_RSSAR: c_uint = 0x0c88;
pub const MAC_RSSDR: c_uint = 0x0c8c;
pub const MAC_TSCR: c_uint = 0x0d00;
pub const MAC_SSIR: c_uint = 0x0d04;
pub const MAC_STSR: c_uint = 0x0d08;
pub const MAC_STNR: c_uint = 0x0d0c;
pub const MAC_STSUR: c_uint = 0x0d10;
pub const MAC_STNUR: c_uint = 0x0d14;
pub const MAC_TSAR: c_uint = 0x0d18;
pub const MAC_TSSR: c_uint = 0x0d20;
pub const MAC_TXSNR: c_uint = 0x0d30;
pub const MAC_TXSSR: c_uint = 0x0d34;
pub const MAC_TICNR: c_uint = 0x0d58;
pub const MAC_TICSNR: c_uint = 0x0d5C;
pub const MAC_TECNR: c_uint = 0x0d60;
pub const MAC_TECSNR: c_uint = 0x0d64;
pub const MAC_PPSCR: c_uint = 0x0d70;
pub const MAC_PPS0_TTSR: c_uint = 0x0d80;
pub const MAC_PPS0_TTNSR: c_uint = 0x0d84;
pub const MAC_PPS0_INTERVAL: c_uint = 0x0d88;
pub const MAC_PPS0_WIDTH: c_uint = 0x0d8C;
pub const MAC_QTFCR_INC: c_int = 4;
pub const MAC_MACA_INC: c_int = 4;
pub const MAC_HTR_INC: c_int = 4;
pub const MAC_RQC2_INC: c_int = 4;
pub const MAC_RQC2_Q_PER_REG: c_int = 4;
// PPS helpers

pub const XGBE_PPSCMD_STOP: c_uint = 0x5;
pub const XGBE_PPSCMD_START: c_uint = 0x2;
pub const XGBE_PPSTARGET_PULSE: c_uint = 0x2;
// MAC register entry bit positions and sizes
pub const MAC_HWF0R_ADDMACADRSEL_INDEX: c_int = 18;
pub const MAC_HWF0R_ADDMACADRSEL_WIDTH: c_int = 5;
pub const MAC_HWF0R_ARPOFFSEL_INDEX: c_int = 9;
pub const MAC_HWF0R_ARPOFFSEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_EEESEL_INDEX: c_int = 13;
pub const MAC_HWF0R_EEESEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_GMIISEL_INDEX: c_int = 1;
pub const MAC_HWF0R_GMIISEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_MGKSEL_INDEX: c_int = 7;
pub const MAC_HWF0R_MGKSEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_MMCSEL_INDEX: c_int = 8;
pub const MAC_HWF0R_MMCSEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_RWKSEL_INDEX: c_int = 6;
pub const MAC_HWF0R_RWKSEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_RXCOESEL_INDEX: c_int = 16;
pub const MAC_HWF0R_RXCOESEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_SAVLANINS_INDEX: c_int = 27;
pub const MAC_HWF0R_SAVLANINS_WIDTH: c_int = 1;
pub const MAC_HWF0R_SMASEL_INDEX: c_int = 5;
pub const MAC_HWF0R_SMASEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_TSSEL_INDEX: c_int = 12;
pub const MAC_HWF0R_TSSEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_TSSTSSEL_INDEX: c_int = 25;
pub const MAC_HWF0R_TSSTSSEL_WIDTH: c_int = 2;
pub const MAC_HWF0R_TXCOESEL_INDEX: c_int = 14;
pub const MAC_HWF0R_TXCOESEL_WIDTH: c_int = 1;
pub const MAC_HWF0R_VLHASH_INDEX: c_int = 4;
pub const MAC_HWF0R_VLHASH_WIDTH: c_int = 1;
pub const MAC_HWF0R_VXN_INDEX: c_int = 29;
pub const MAC_HWF0R_VXN_WIDTH: c_int = 1;
pub const MAC_HWF1R_ADDR64_INDEX: c_int = 14;
pub const MAC_HWF1R_ADDR64_WIDTH: c_int = 2;
pub const MAC_HWF1R_ADVTHWORD_INDEX: c_int = 13;
pub const MAC_HWF1R_ADVTHWORD_WIDTH: c_int = 1;
pub const MAC_HWF1R_DBGMEMA_INDEX: c_int = 19;
pub const MAC_HWF1R_DBGMEMA_WIDTH: c_int = 1;
pub const MAC_HWF1R_DCBEN_INDEX: c_int = 16;
pub const MAC_HWF1R_DCBEN_WIDTH: c_int = 1;
pub const MAC_HWF1R_HASHTBLSZ_INDEX: c_int = 24;
pub const MAC_HWF1R_HASHTBLSZ_WIDTH: c_int = 3;
pub const MAC_HWF1R_L3L4FNUM_INDEX: c_int = 27;
pub const MAC_HWF1R_L3L4FNUM_WIDTH: c_int = 4;
pub const MAC_HWF1R_NUMTC_INDEX: c_int = 21;
pub const MAC_HWF1R_NUMTC_WIDTH: c_int = 3;
pub const MAC_HWF1R_RSSEN_INDEX: c_int = 20;
pub const MAC_HWF1R_RSSEN_WIDTH: c_int = 1;
pub const MAC_HWF1R_RXFIFOSIZE_INDEX: c_int = 0;
pub const MAC_HWF1R_RXFIFOSIZE_WIDTH: c_int = 5;
pub const MAC_HWF1R_SPHEN_INDEX: c_int = 17;
pub const MAC_HWF1R_SPHEN_WIDTH: c_int = 1;
pub const MAC_HWF1R_TSOEN_INDEX: c_int = 18;
pub const MAC_HWF1R_TSOEN_WIDTH: c_int = 1;
pub const MAC_HWF1R_TXFIFOSIZE_INDEX: c_int = 6;
pub const MAC_HWF1R_TXFIFOSIZE_WIDTH: c_int = 5;
pub const MAC_HWF2R_AUXSNAPNUM_INDEX: c_int = 28;
pub const MAC_HWF2R_AUXSNAPNUM_WIDTH: c_int = 3;
pub const MAC_HWF2R_PPSOUTNUM_INDEX: c_int = 24;
pub const MAC_HWF2R_PPSOUTNUM_WIDTH: c_int = 3;
pub const MAC_HWF2R_RXCHCNT_INDEX: c_int = 12;
pub const MAC_HWF2R_RXCHCNT_WIDTH: c_int = 4;
pub const MAC_HWF2R_RXQCNT_INDEX: c_int = 0;
pub const MAC_HWF2R_RXQCNT_WIDTH: c_int = 4;
pub const MAC_HWF2R_TXCHCNT_INDEX: c_int = 18;
pub const MAC_HWF2R_TXCHCNT_WIDTH: c_int = 4;
pub const MAC_HWF2R_TXQCNT_INDEX: c_int = 6;
pub const MAC_HWF2R_TXQCNT_WIDTH: c_int = 4;
pub const MAC_IER_TSIE_INDEX: c_int = 12;
pub const MAC_IER_TSIE_WIDTH: c_int = 1;
pub const MAC_ISR_MMCRXIS_INDEX: c_int = 9;
pub const MAC_ISR_MMCRXIS_WIDTH: c_int = 1;
pub const MAC_ISR_MMCTXIS_INDEX: c_int = 10;
pub const MAC_ISR_MMCTXIS_WIDTH: c_int = 1;
pub const MAC_ISR_PMTIS_INDEX: c_int = 4;
pub const MAC_ISR_PMTIS_WIDTH: c_int = 1;
pub const MAC_ISR_SMI_INDEX: c_int = 1;
pub const MAC_ISR_SMI_WIDTH: c_int = 1;
pub const MAC_ISR_TSIS_INDEX: c_int = 12;
pub const MAC_ISR_TSIS_WIDTH: c_int = 1;
pub const MAC_ISR_LS_INDEX: c_int = 24;
pub const MAC_ISR_LS_WIDTH: c_int = 2;
pub const MAC_ISR_LSI_INDEX: c_int = 0;
pub const MAC_ISR_LSI_WIDTH: c_int = 1;
pub const MAC_MACA1HR_AE_INDEX: c_int = 31;
pub const MAC_MACA1HR_AE_WIDTH: c_int = 1;
pub const MAC_MDIOIER_SNGLCOMPIE_INDEX: c_int = 12;
pub const MAC_MDIOIER_SNGLCOMPIE_WIDTH: c_int = 1;
pub const MAC_MDIOISR_SNGLCOMPINT_INDEX: c_int = 12;
pub const MAC_MDIOISR_SNGLCOMPINT_WIDTH: c_int = 1;
pub const MAC_MDIOSCAR_DA_INDEX: c_int = 21;
pub const MAC_MDIOSCAR_DA_WIDTH: c_int = 5;
pub const MAC_MDIOSCAR_PA_INDEX: c_int = 16;
pub const MAC_MDIOSCAR_PA_WIDTH: c_int = 5;
pub const MAC_MDIOSCAR_RA_INDEX: c_int = 0;
pub const MAC_MDIOSCAR_RA_WIDTH: c_int = 16;
pub const MAC_MDIOSCCDR_BUSY_INDEX: c_int = 22;
pub const MAC_MDIOSCCDR_BUSY_WIDTH: c_int = 1;
pub const MAC_MDIOSCCDR_CMD_INDEX: c_int = 16;
pub const MAC_MDIOSCCDR_CMD_WIDTH: c_int = 2;
pub const MAC_MDIOSCCDR_CR_INDEX: c_int = 19;
pub const MAC_MDIOSCCDR_CR_WIDTH: c_int = 3;
pub const MAC_MDIOSCCDR_DATA_INDEX: c_int = 0;
pub const MAC_MDIOSCCDR_DATA_WIDTH: c_int = 16;
pub const MAC_MDIOSCCDR_SADDR_INDEX: c_int = 18;
pub const MAC_MDIOSCCDR_SADDR_WIDTH: c_int = 1;
pub const MAC_PFR_HMC_INDEX: c_int = 2;
pub const MAC_PFR_HMC_WIDTH: c_int = 1;
pub const MAC_PFR_HPF_INDEX: c_int = 10;
pub const MAC_PFR_HPF_WIDTH: c_int = 1;
pub const MAC_PFR_HUC_INDEX: c_int = 1;
pub const MAC_PFR_HUC_WIDTH: c_int = 1;
pub const MAC_PFR_PM_INDEX: c_int = 4;
pub const MAC_PFR_PM_WIDTH: c_int = 1;
pub const MAC_PFR_PR_INDEX: c_int = 0;
pub const MAC_PFR_PR_WIDTH: c_int = 1;
pub const MAC_PFR_VTFE_INDEX: c_int = 16;
pub const MAC_PFR_VTFE_WIDTH: c_int = 1;
pub const MAC_PFR_VUCC_INDEX: c_int = 22;
pub const MAC_PFR_VUCC_WIDTH: c_int = 1;
pub const MAC_PMTCSR_MGKPKTEN_INDEX: c_int = 1;
pub const MAC_PMTCSR_MGKPKTEN_WIDTH: c_int = 1;
pub const MAC_PMTCSR_PWRDWN_INDEX: c_int = 0;
pub const MAC_PMTCSR_PWRDWN_WIDTH: c_int = 1;
pub const MAC_PMTCSR_RWKFILTRST_INDEX: c_int = 31;
pub const MAC_PMTCSR_RWKFILTRST_WIDTH: c_int = 1;
pub const MAC_PMTCSR_RWKPKTEN_INDEX: c_int = 2;
pub const MAC_PMTCSR_RWKPKTEN_WIDTH: c_int = 1;
pub const MAC_Q0TFCR_PT_INDEX: c_int = 16;
pub const MAC_Q0TFCR_PT_WIDTH: c_int = 16;
pub const MAC_Q0TFCR_TFE_INDEX: c_int = 1;
pub const MAC_Q0TFCR_TFE_WIDTH: c_int = 1;
pub const MAC_RCR_ACS_INDEX: c_int = 1;
pub const MAC_RCR_ACS_WIDTH: c_int = 1;
pub const MAC_RCR_CST_INDEX: c_int = 2;
pub const MAC_RCR_CST_WIDTH: c_int = 1;
pub const MAC_RCR_DCRCC_INDEX: c_int = 3;
pub const MAC_RCR_DCRCC_WIDTH: c_int = 1;
pub const MAC_RCR_GPSLCE_INDEX: c_int = 6;
pub const MAC_RCR_GPSLCE_WIDTH: c_int = 1;
pub const MAC_RCR_WD_INDEX: c_int = 7;
pub const MAC_RCR_WD_WIDTH: c_int = 1;
pub const MAC_RCR_HDSMS_INDEX: c_int = 12;
pub const MAC_RCR_HDSMS_WIDTH: c_int = 3;
pub const MAC_RCR_IPC_INDEX: c_int = 9;
pub const MAC_RCR_IPC_WIDTH: c_int = 1;
pub const MAC_RCR_JE_INDEX: c_int = 8;
pub const MAC_RCR_JE_WIDTH: c_int = 1;
pub const MAC_RCR_LM_INDEX: c_int = 10;
pub const MAC_RCR_LM_WIDTH: c_int = 1;
pub const MAC_RCR_RE_INDEX: c_int = 0;
pub const MAC_RCR_RE_WIDTH: c_int = 1;
pub const MAC_RCR_GPSL_INDEX: c_int = 16;
pub const MAC_RCR_GPSL_WIDTH: c_int = 14;
pub const MAC_RFCR_PFCE_INDEX: c_int = 8;
pub const MAC_RFCR_PFCE_WIDTH: c_int = 1;
pub const MAC_RFCR_RFE_INDEX: c_int = 0;
pub const MAC_RFCR_RFE_WIDTH: c_int = 1;
pub const MAC_RFCR_UP_INDEX: c_int = 1;
pub const MAC_RFCR_UP_WIDTH: c_int = 1;
pub const MAC_RQC0R_RXQ0EN_INDEX: c_int = 0;
pub const MAC_RQC0R_RXQ0EN_WIDTH: c_int = 2;
pub const MAC_RSSAR_ADDRT_INDEX: c_int = 2;
pub const MAC_RSSAR_ADDRT_WIDTH: c_int = 1;
pub const MAC_RSSAR_CT_INDEX: c_int = 1;
pub const MAC_RSSAR_CT_WIDTH: c_int = 1;
pub const MAC_RSSAR_OB_INDEX: c_int = 0;
pub const MAC_RSSAR_OB_WIDTH: c_int = 1;
pub const MAC_RSSAR_RSSIA_INDEX: c_int = 8;
pub const MAC_RSSAR_RSSIA_WIDTH: c_int = 8;
pub const MAC_RSSCR_IP2TE_INDEX: c_int = 1;
pub const MAC_RSSCR_IP2TE_WIDTH: c_int = 1;
pub const MAC_RSSCR_RSSE_INDEX: c_int = 0;
pub const MAC_RSSCR_RSSE_WIDTH: c_int = 1;
pub const MAC_RSSCR_TCP4TE_INDEX: c_int = 2;
pub const MAC_RSSCR_TCP4TE_WIDTH: c_int = 1;
pub const MAC_RSSCR_UDP4TE_INDEX: c_int = 3;
pub const MAC_RSSCR_UDP4TE_WIDTH: c_int = 1;
pub const MAC_RSSDR_DMCH_INDEX: c_int = 0;
pub const MAC_RSSDR_DMCH_WIDTH: c_int = 4;
pub const MAC_SSIR_SNSINC_INDEX: c_int = 8;
pub const MAC_SSIR_SNSINC_WIDTH: c_int = 8;
pub const MAC_SSIR_SSINC_INDEX: c_int = 16;
pub const MAC_SSIR_SSINC_WIDTH: c_int = 8;
pub const MAC_TCR_SS_INDEX: c_int = 29;
pub const MAC_TCR_SS_WIDTH: c_int = 3;
pub const MAC_TCR_TE_INDEX: c_int = 0;
pub const MAC_TCR_TE_WIDTH: c_int = 1;
pub const MAC_TCR_VNE_INDEX: c_int = 24;
pub const MAC_TCR_VNE_WIDTH: c_int = 1;
pub const MAC_TCR_VNM_INDEX: c_int = 25;
pub const MAC_TCR_VNM_WIDTH: c_int = 1;
pub const MAC_TCR_JD_INDEX: c_int = 16;
pub const MAC_TCR_JD_WIDTH: c_int = 1;
pub const MAC_TIR_TNID_INDEX: c_int = 0;
pub const MAC_TIR_TNID_WIDTH: c_int = 16;
pub const MAC_TSCR_AV8021ASMEN_INDEX: c_int = 28;
pub const MAC_TSCR_AV8021ASMEN_WIDTH: c_int = 1;
pub const MAC_TSCR_SNAPTYPSEL_INDEX: c_int = 16;
pub const MAC_TSCR_SNAPTYPSEL_WIDTH: c_int = 2;
pub const MAC_TSCR_TSADDREG_INDEX: c_int = 5;
pub const MAC_TSCR_TSADDREG_WIDTH: c_int = 1;
pub const MAC_TSCR_TSUPDT_INDEX: c_int = 3;
pub const MAC_TSCR_TSUPDT_WIDTH: c_int = 1;
pub const MAC_TSCR_TSCFUPDT_INDEX: c_int = 1;
pub const MAC_TSCR_TSCFUPDT_WIDTH: c_int = 1;
pub const MAC_TSCR_TSCTRLSSR_INDEX: c_int = 9;
pub const MAC_TSCR_TSCTRLSSR_WIDTH: c_int = 1;
pub const MAC_TSCR_TSENA_INDEX: c_int = 0;
pub const MAC_TSCR_TSENA_WIDTH: c_int = 1;
pub const MAC_TSCR_TSENALL_INDEX: c_int = 8;
pub const MAC_TSCR_TSENALL_WIDTH: c_int = 1;
pub const MAC_TSCR_TSEVNTENA_INDEX: c_int = 14;
pub const MAC_TSCR_TSEVNTENA_WIDTH: c_int = 1;
pub const MAC_TSCR_TSINIT_INDEX: c_int = 2;
pub const MAC_TSCR_TSINIT_WIDTH: c_int = 1;
pub const MAC_TSCR_TSIPENA_INDEX: c_int = 11;
pub const MAC_TSCR_TSIPENA_WIDTH: c_int = 1;
pub const MAC_TSCR_TSIPV4ENA_INDEX: c_int = 13;
pub const MAC_TSCR_TSIPV4ENA_WIDTH: c_int = 1;
pub const MAC_TSCR_TSIPV6ENA_INDEX: c_int = 12;
pub const MAC_TSCR_TSIPV6ENA_WIDTH: c_int = 1;
pub const MAC_TSCR_TSMSTRENA_INDEX: c_int = 15;
pub const MAC_TSCR_TSMSTRENA_WIDTH: c_int = 1;
pub const MAC_TSCR_TSVER2ENA_INDEX: c_int = 10;
pub const MAC_TSCR_TSVER2ENA_WIDTH: c_int = 1;
pub const MAC_TSCR_TXTSSTSM_INDEX: c_int = 24;
pub const MAC_TSCR_TXTSSTSM_WIDTH: c_int = 1;
pub const MAC_TSSR_TXTSC_INDEX: c_int = 15;
pub const MAC_TSSR_TXTSC_WIDTH: c_int = 1;
pub const MAC_TXSNR_TXTSSTSMIS_INDEX: c_int = 31;
pub const MAC_TXSNR_TXTSSTSMIS_WIDTH: c_int = 1;
pub const MAC_TICSNR_TSICSNS_INDEX: c_int = 8;
pub const MAC_TICSNR_TSICSNS_WIDTH: c_int = 8;
pub const MAC_TECSNR_TSECSNS_INDEX: c_int = 8;
pub const MAC_TECSNR_TSECSNS_WIDTH: c_int = 8;
pub const MAC_VLANHTR_VLHT_INDEX: c_int = 0;
pub const MAC_VLANHTR_VLHT_WIDTH: c_int = 16;
pub const MAC_VLANIR_VLTI_INDEX: c_int = 20;
pub const MAC_VLANIR_VLTI_WIDTH: c_int = 1;
pub const MAC_VLANIR_CSVL_INDEX: c_int = 19;
pub const MAC_VLANIR_CSVL_WIDTH: c_int = 1;
pub const MAC_VLANTR_DOVLTC_INDEX: c_int = 20;
pub const MAC_VLANTR_DOVLTC_WIDTH: c_int = 1;
pub const MAC_VLANTR_ERSVLM_INDEX: c_int = 19;
pub const MAC_VLANTR_ERSVLM_WIDTH: c_int = 1;
pub const MAC_VLANTR_ESVL_INDEX: c_int = 18;
pub const MAC_VLANTR_ESVL_WIDTH: c_int = 1;
pub const MAC_VLANTR_ETV_INDEX: c_int = 16;
pub const MAC_VLANTR_ETV_WIDTH: c_int = 1;
pub const MAC_VLANTR_EVLS_INDEX: c_int = 21;
pub const MAC_VLANTR_EVLS_WIDTH: c_int = 2;
pub const MAC_VLANTR_EVLRXS_INDEX: c_int = 24;
pub const MAC_VLANTR_EVLRXS_WIDTH: c_int = 1;
pub const MAC_VLANTR_VL_INDEX: c_int = 0;
pub const MAC_VLANTR_VL_WIDTH: c_int = 16;
pub const MAC_VLANTR_VTHM_INDEX: c_int = 25;
pub const MAC_VLANTR_VTHM_WIDTH: c_int = 1;
pub const MAC_VLANTR_VTIM_INDEX: c_int = 17;
pub const MAC_VLANTR_VTIM_WIDTH: c_int = 1;
pub const MAC_VR_DEVID_INDEX: c_int = 8;
pub const MAC_VR_DEVID_WIDTH: c_int = 8;
pub const MAC_VR_SNPSVER_INDEX: c_int = 0;
pub const MAC_VR_SNPSVER_WIDTH: c_int = 8;
pub const MAC_VR_USERVER_INDEX: c_int = 16;
pub const MAC_VR_USERVER_WIDTH: c_int = 8;
pub const MAC_PPSx_TTNSR_TRGTBUSY0_INDEX: c_int = 31;
pub const MAC_PPSx_TTNSR_TRGTBUSY0_WIDTH: c_int = 1;
// MMC register offsets
pub const MMC_CR: c_uint = 0x0800;
pub const MMC_RISR: c_uint = 0x0804;
pub const MMC_TISR: c_uint = 0x0808;
pub const MMC_RIER: c_uint = 0x080c;
pub const MMC_TIER: c_uint = 0x0810;
pub const MMC_TXOCTETCOUNT_GB_LO: c_uint = 0x0814;
pub const MMC_TXOCTETCOUNT_GB_HI: c_uint = 0x0818;
pub const MMC_TXFRAMECOUNT_GB_LO: c_uint = 0x081c;
pub const MMC_TXFRAMECOUNT_GB_HI: c_uint = 0x0820;
pub const MMC_TXBROADCASTFRAMES_G_LO: c_uint = 0x0824;
pub const MMC_TXBROADCASTFRAMES_G_HI: c_uint = 0x0828;
pub const MMC_TXMULTICASTFRAMES_G_LO: c_uint = 0x082c;
pub const MMC_TXMULTICASTFRAMES_G_HI: c_uint = 0x0830;
pub const MMC_TX64OCTETS_GB_LO: c_uint = 0x0834;
pub const MMC_TX64OCTETS_GB_HI: c_uint = 0x0838;
pub const MMC_TX65TO127OCTETS_GB_LO: c_uint = 0x083c;
pub const MMC_TX65TO127OCTETS_GB_HI: c_uint = 0x0840;
pub const MMC_TX128TO255OCTETS_GB_LO: c_uint = 0x0844;
pub const MMC_TX128TO255OCTETS_GB_HI: c_uint = 0x0848;
pub const MMC_TX256TO511OCTETS_GB_LO: c_uint = 0x084c;
pub const MMC_TX256TO511OCTETS_GB_HI: c_uint = 0x0850;
pub const MMC_TX512TO1023OCTETS_GB_LO: c_uint = 0x0854;
pub const MMC_TX512TO1023OCTETS_GB_HI: c_uint = 0x0858;
pub const MMC_TX1024TOMAXOCTETS_GB_LO: c_uint = 0x085c;
pub const MMC_TX1024TOMAXOCTETS_GB_HI: c_uint = 0x0860;
pub const MMC_TXUNICASTFRAMES_GB_LO: c_uint = 0x0864;
pub const MMC_TXUNICASTFRAMES_GB_HI: c_uint = 0x0868;
pub const MMC_TXMULTICASTFRAMES_GB_LO: c_uint = 0x086c;
pub const MMC_TXMULTICASTFRAMES_GB_HI: c_uint = 0x0870;
pub const MMC_TXBROADCASTFRAMES_GB_LO: c_uint = 0x0874;
pub const MMC_TXBROADCASTFRAMES_GB_HI: c_uint = 0x0878;
pub const MMC_TXUNDERFLOWERROR_LO: c_uint = 0x087c;
pub const MMC_TXUNDERFLOWERROR_HI: c_uint = 0x0880;
pub const MMC_TXOCTETCOUNT_G_LO: c_uint = 0x0884;
pub const MMC_TXOCTETCOUNT_G_HI: c_uint = 0x0888;
pub const MMC_TXFRAMECOUNT_G_LO: c_uint = 0x088c;
pub const MMC_TXFRAMECOUNT_G_HI: c_uint = 0x0890;
pub const MMC_TXPAUSEFRAMES_LO: c_uint = 0x0894;
pub const MMC_TXPAUSEFRAMES_HI: c_uint = 0x0898;
pub const MMC_TXVLANFRAMES_G_LO: c_uint = 0x089c;
pub const MMC_TXVLANFRAMES_G_HI: c_uint = 0x08a0;
pub const MMC_RXFRAMECOUNT_GB_LO: c_uint = 0x0900;
pub const MMC_RXFRAMECOUNT_GB_HI: c_uint = 0x0904;
pub const MMC_RXOCTETCOUNT_GB_LO: c_uint = 0x0908;
pub const MMC_RXOCTETCOUNT_GB_HI: c_uint = 0x090c;
pub const MMC_RXOCTETCOUNT_G_LO: c_uint = 0x0910;
pub const MMC_RXOCTETCOUNT_G_HI: c_uint = 0x0914;
pub const MMC_RXBROADCASTFRAMES_G_LO: c_uint = 0x0918;
pub const MMC_RXBROADCASTFRAMES_G_HI: c_uint = 0x091c;
pub const MMC_RXMULTICASTFRAMES_G_LO: c_uint = 0x0920;
pub const MMC_RXMULTICASTFRAMES_G_HI: c_uint = 0x0924;
pub const MMC_RXCRCERROR_LO: c_uint = 0x0928;
pub const MMC_RXCRCERROR_HI: c_uint = 0x092c;
pub const MMC_RXRUNTERROR: c_uint = 0x0930;
pub const MMC_RXJABBERERROR: c_uint = 0x0934;
pub const MMC_RXUNDERSIZE_G: c_uint = 0x0938;
pub const MMC_RXOVERSIZE_G: c_uint = 0x093c;
pub const MMC_RX64OCTETS_GB_LO: c_uint = 0x0940;
pub const MMC_RX64OCTETS_GB_HI: c_uint = 0x0944;
pub const MMC_RX65TO127OCTETS_GB_LO: c_uint = 0x0948;
pub const MMC_RX65TO127OCTETS_GB_HI: c_uint = 0x094c;
pub const MMC_RX128TO255OCTETS_GB_LO: c_uint = 0x0950;
pub const MMC_RX128TO255OCTETS_GB_HI: c_uint = 0x0954;
pub const MMC_RX256TO511OCTETS_GB_LO: c_uint = 0x0958;
pub const MMC_RX256TO511OCTETS_GB_HI: c_uint = 0x095c;
pub const MMC_RX512TO1023OCTETS_GB_LO: c_uint = 0x0960;
pub const MMC_RX512TO1023OCTETS_GB_HI: c_uint = 0x0964;
pub const MMC_RX1024TOMAXOCTETS_GB_LO: c_uint = 0x0968;
pub const MMC_RX1024TOMAXOCTETS_GB_HI: c_uint = 0x096c;
pub const MMC_RXUNICASTFRAMES_G_LO: c_uint = 0x0970;
pub const MMC_RXUNICASTFRAMES_G_HI: c_uint = 0x0974;
pub const MMC_RXLENGTHERROR_LO: c_uint = 0x0978;
pub const MMC_RXLENGTHERROR_HI: c_uint = 0x097c;
pub const MMC_RXOUTOFRANGETYPE_LO: c_uint = 0x0980;
pub const MMC_RXOUTOFRANGETYPE_HI: c_uint = 0x0984;
pub const MMC_RXPAUSEFRAMES_LO: c_uint = 0x0988;
pub const MMC_RXPAUSEFRAMES_HI: c_uint = 0x098c;
pub const MMC_RXFIFOOVERFLOW_LO: c_uint = 0x0990;
pub const MMC_RXFIFOOVERFLOW_HI: c_uint = 0x0994;
pub const MMC_RXVLANFRAMES_GB_LO: c_uint = 0x0998;
pub const MMC_RXVLANFRAMES_GB_HI: c_uint = 0x099c;
pub const MMC_RXWATCHDOGERROR: c_uint = 0x09a0;
pub const MMC_RXALIGNMENTERROR: c_uint = 0x09bc;
// MMC register entry bit positions and sizes
pub const MMC_CR_CR_INDEX: c_int = 0;
pub const MMC_CR_CR_WIDTH: c_int = 1;
pub const MMC_CR_CSR_INDEX: c_int = 1;
pub const MMC_CR_CSR_WIDTH: c_int = 1;
pub const MMC_CR_ROR_INDEX: c_int = 2;
pub const MMC_CR_ROR_WIDTH: c_int = 1;
pub const MMC_CR_MCF_INDEX: c_int = 3;
pub const MMC_CR_MCF_WIDTH: c_int = 1;
pub const MMC_CR_MCT_INDEX: c_int = 4;
pub const MMC_CR_MCT_WIDTH: c_int = 2;
pub const MMC_RIER_ALL_INTERRUPTS_INDEX: c_int = 0;
pub const MMC_RIER_ALL_INTERRUPTS_WIDTH: c_int = 23;
pub const MMC_RISR_RXFRAMECOUNT_GB_INDEX: c_int = 0;
pub const MMC_RISR_RXFRAMECOUNT_GB_WIDTH: c_int = 1;
pub const MMC_RISR_RXOCTETCOUNT_GB_INDEX: c_int = 1;
pub const MMC_RISR_RXOCTETCOUNT_GB_WIDTH: c_int = 1;
pub const MMC_RISR_RXOCTETCOUNT_G_INDEX: c_int = 2;
pub const MMC_RISR_RXOCTETCOUNT_G_WIDTH: c_int = 1;
pub const MMC_RISR_RXBROADCASTFRAMES_G_INDEX: c_int = 3;
pub const MMC_RISR_RXBROADCASTFRAMES_G_WIDTH: c_int = 1;
pub const MMC_RISR_RXMULTICASTFRAMES_G_INDEX: c_int = 4;
pub const MMC_RISR_RXMULTICASTFRAMES_G_WIDTH: c_int = 1;
pub const MMC_RISR_RXCRCERROR_INDEX: c_int = 5;
pub const MMC_RISR_RXCRCERROR_WIDTH: c_int = 1;
pub const MMC_RISR_RXRUNTERROR_INDEX: c_int = 6;
pub const MMC_RISR_RXRUNTERROR_WIDTH: c_int = 1;
pub const MMC_RISR_RXJABBERERROR_INDEX: c_int = 7;
pub const MMC_RISR_RXJABBERERROR_WIDTH: c_int = 1;
pub const MMC_RISR_RXUNDERSIZE_G_INDEX: c_int = 8;
pub const MMC_RISR_RXUNDERSIZE_G_WIDTH: c_int = 1;
pub const MMC_RISR_RXOVERSIZE_G_INDEX: c_int = 9;
pub const MMC_RISR_RXOVERSIZE_G_WIDTH: c_int = 1;
pub const MMC_RISR_RX64OCTETS_GB_INDEX: c_int = 10;
pub const MMC_RISR_RX64OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_RISR_RX65TO127OCTETS_GB_INDEX: c_int = 11;
pub const MMC_RISR_RX65TO127OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_RISR_RX128TO255OCTETS_GB_INDEX: c_int = 12;
pub const MMC_RISR_RX128TO255OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_RISR_RX256TO511OCTETS_GB_INDEX: c_int = 13;
pub const MMC_RISR_RX256TO511OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_RISR_RX512TO1023OCTETS_GB_INDEX: c_int = 14;
pub const MMC_RISR_RX512TO1023OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_RISR_RX1024TOMAXOCTETS_GB_INDEX: c_int = 15;
pub const MMC_RISR_RX1024TOMAXOCTETS_GB_WIDTH: c_int = 1;
pub const MMC_RISR_RXUNICASTFRAMES_G_INDEX: c_int = 16;
pub const MMC_RISR_RXUNICASTFRAMES_G_WIDTH: c_int = 1;
pub const MMC_RISR_RXLENGTHERROR_INDEX: c_int = 17;
pub const MMC_RISR_RXLENGTHERROR_WIDTH: c_int = 1;
pub const MMC_RISR_RXOUTOFRANGETYPE_INDEX: c_int = 18;
pub const MMC_RISR_RXOUTOFRANGETYPE_WIDTH: c_int = 1;
pub const MMC_RISR_RXPAUSEFRAMES_INDEX: c_int = 19;
pub const MMC_RISR_RXPAUSEFRAMES_WIDTH: c_int = 1;
pub const MMC_RISR_RXFIFOOVERFLOW_INDEX: c_int = 20;
pub const MMC_RISR_RXFIFOOVERFLOW_WIDTH: c_int = 1;
pub const MMC_RISR_RXVLANFRAMES_GB_INDEX: c_int = 21;
pub const MMC_RISR_RXVLANFRAMES_GB_WIDTH: c_int = 1;
pub const MMC_RISR_RXWATCHDOGERROR_INDEX: c_int = 22;
pub const MMC_RISR_RXWATCHDOGERROR_WIDTH: c_int = 1;
pub const MMC_RISR_RXALIGNMENTERROR_INDEX: c_int = 27;
pub const MMC_RISR_RXALIGNMENTERROR_WIDTH: c_int = 1;
pub const MMC_TIER_ALL_INTERRUPTS_INDEX: c_int = 0;
pub const MMC_TIER_ALL_INTERRUPTS_WIDTH: c_int = 18;
pub const MMC_TISR_TXOCTETCOUNT_GB_INDEX: c_int = 0;
pub const MMC_TISR_TXOCTETCOUNT_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TXFRAMECOUNT_GB_INDEX: c_int = 1;
pub const MMC_TISR_TXFRAMECOUNT_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TXBROADCASTFRAMES_G_INDEX: c_int = 2;
pub const MMC_TISR_TXBROADCASTFRAMES_G_WIDTH: c_int = 1;
pub const MMC_TISR_TXMULTICASTFRAMES_G_INDEX: c_int = 3;
pub const MMC_TISR_TXMULTICASTFRAMES_G_WIDTH: c_int = 1;
pub const MMC_TISR_TX64OCTETS_GB_INDEX: c_int = 4;
pub const MMC_TISR_TX64OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TX65TO127OCTETS_GB_INDEX: c_int = 5;
pub const MMC_TISR_TX65TO127OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TX128TO255OCTETS_GB_INDEX: c_int = 6;
pub const MMC_TISR_TX128TO255OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TX256TO511OCTETS_GB_INDEX: c_int = 7;
pub const MMC_TISR_TX256TO511OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TX512TO1023OCTETS_GB_INDEX: c_int = 8;
pub const MMC_TISR_TX512TO1023OCTETS_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TX1024TOMAXOCTETS_GB_INDEX: c_int = 9;
pub const MMC_TISR_TX1024TOMAXOCTETS_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TXUNICASTFRAMES_GB_INDEX: c_int = 10;
pub const MMC_TISR_TXUNICASTFRAMES_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TXMULTICASTFRAMES_GB_INDEX: c_int = 11;
pub const MMC_TISR_TXMULTICASTFRAMES_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TXBROADCASTFRAMES_GB_INDEX: c_int = 12;
pub const MMC_TISR_TXBROADCASTFRAMES_GB_WIDTH: c_int = 1;
pub const MMC_TISR_TXUNDERFLOWERROR_INDEX: c_int = 13;
pub const MMC_TISR_TXUNDERFLOWERROR_WIDTH: c_int = 1;
pub const MMC_TISR_TXOCTETCOUNT_G_INDEX: c_int = 14;
pub const MMC_TISR_TXOCTETCOUNT_G_WIDTH: c_int = 1;
pub const MMC_TISR_TXFRAMECOUNT_G_INDEX: c_int = 15;
pub const MMC_TISR_TXFRAMECOUNT_G_WIDTH: c_int = 1;
pub const MMC_TISR_TXPAUSEFRAMES_INDEX: c_int = 16;
pub const MMC_TISR_TXPAUSEFRAMES_WIDTH: c_int = 1;
pub const MMC_TISR_TXVLANFRAMES_G_INDEX: c_int = 17;
pub const MMC_TISR_TXVLANFRAMES_G_WIDTH: c_int = 1;
// MTL register offsets
pub const MTL_OMR: c_uint = 0x1000;
pub const MTL_FDCR: c_uint = 0x1008;
pub const MTL_FDSR: c_uint = 0x100c;
pub const MTL_FDDR: c_uint = 0x1010;
pub const MTL_ISR: c_uint = 0x1020;
pub const MTL_RQDCM0R: c_uint = 0x1030;
pub const MTL_TCPM0R: c_uint = 0x1040;
pub const MTL_TCPM1R: c_uint = 0x1044;
pub const MTL_RQDCM_INC: c_int = 4;
pub const MTL_RQDCM_Q_PER_REG: c_int = 4;
pub const MTL_TCPM_INC: c_int = 4;
pub const MTL_TCPM_TC_PER_REG: c_int = 4;
// MTL register entry bit positions and sizes
pub const MTL_OMR_ETSALG_INDEX: c_int = 5;
pub const MTL_OMR_ETSALG_WIDTH: c_int = 2;
pub const MTL_OMR_RAA_INDEX: c_int = 2;
pub const MTL_OMR_RAA_WIDTH: c_int = 1;
// MTL queue register offsets
// Multiple queues can be active.  The first queue has registers
// that begin at 0x1100.  Each subsequent queue has registers that
// are accessed using an offset of 0x80 from the previous queue.
//
pub const MTL_Q_BASE: c_uint = 0x1100;
pub const MTL_Q_INC: c_uint = 0x80;
pub const MTL_Q_TQOMR: c_uint = 0x00;
pub const MTL_Q_TQUR: c_uint = 0x04;
pub const MTL_Q_TQDR: c_uint = 0x08;
pub const MTL_Q_RQOMR: c_uint = 0x40;
pub const MTL_Q_RQMPOCR: c_uint = 0x44;
pub const MTL_Q_RQDR: c_uint = 0x48;
pub const MTL_Q_RQFCR: c_uint = 0x50;
pub const MTL_Q_IER: c_uint = 0x70;
pub const MTL_Q_ISR: c_uint = 0x74;
// MTL queue register entry bit positions and sizes
pub const MTL_Q_RQDR_PRXQ_INDEX: c_int = 16;
pub const MTL_Q_RQDR_PRXQ_WIDTH: c_int = 14;
pub const MTL_Q_RQDR_RXQSTS_INDEX: c_int = 4;
pub const MTL_Q_RQDR_RXQSTS_WIDTH: c_int = 2;
pub const MTL_Q_RQFCR_RFA_INDEX: c_int = 1;
pub const MTL_Q_RQFCR_RFA_WIDTH: c_int = 6;
pub const MTL_Q_RQFCR_RFD_INDEX: c_int = 17;
pub const MTL_Q_RQFCR_RFD_WIDTH: c_int = 6;
pub const MTL_Q_RQOMR_EHFC_INDEX: c_int = 7;
pub const MTL_Q_RQOMR_EHFC_WIDTH: c_int = 1;
pub const MTL_Q_RQOMR_RQS_INDEX: c_int = 16;
pub const MTL_Q_RQOMR_RQS_WIDTH: c_int = 9;
pub const MTL_Q_RQOMR_RSF_INDEX: c_int = 5;
pub const MTL_Q_RQOMR_RSF_WIDTH: c_int = 1;
pub const MTL_Q_RQOMR_RTC_INDEX: c_int = 0;
pub const MTL_Q_RQOMR_RTC_WIDTH: c_int = 2;
pub const MTL_Q_TQDR_TRCSTS_INDEX: c_int = 1;
pub const MTL_Q_TQDR_TRCSTS_WIDTH: c_int = 2;
pub const MTL_Q_TQDR_TXQSTS_INDEX: c_int = 4;
pub const MTL_Q_TQDR_TXQSTS_WIDTH: c_int = 1;
pub const MTL_Q_TQOMR_FTQ_INDEX: c_int = 0;
pub const MTL_Q_TQOMR_FTQ_WIDTH: c_int = 1;
pub const MTL_Q_TQOMR_Q2TCMAP_INDEX: c_int = 8;
pub const MTL_Q_TQOMR_Q2TCMAP_WIDTH: c_int = 3;
pub const MTL_Q_TQOMR_TQS_INDEX: c_int = 16;
pub const MTL_Q_TQOMR_TQS_WIDTH: c_int = 10;
pub const MTL_Q_TQOMR_TSF_INDEX: c_int = 1;
pub const MTL_Q_TQOMR_TSF_WIDTH: c_int = 1;
pub const MTL_Q_TQOMR_TTC_INDEX: c_int = 4;
pub const MTL_Q_TQOMR_TTC_WIDTH: c_int = 3;
pub const MTL_Q_TQOMR_TXQEN_INDEX: c_int = 2;
pub const MTL_Q_TQOMR_TXQEN_WIDTH: c_int = 2;
// MTL queue register value
pub const MTL_RSF_DISABLE: c_uint = 0x00;
pub const MTL_RSF_ENABLE: c_uint = 0x01;
pub const MTL_TSF_DISABLE: c_uint = 0x00;
pub const MTL_TSF_ENABLE: c_uint = 0x01;
pub const MTL_RX_THRESHOLD_64: c_uint = 0x00;
pub const MTL_RX_THRESHOLD_96: c_uint = 0x02;
pub const MTL_RX_THRESHOLD_128: c_uint = 0x03;
pub const MTL_TX_THRESHOLD_32: c_uint = 0x01;
pub const MTL_TX_THRESHOLD_64: c_uint = 0x00;
pub const MTL_TX_THRESHOLD_96: c_uint = 0x02;
pub const MTL_TX_THRESHOLD_128: c_uint = 0x03;
pub const MTL_TX_THRESHOLD_192: c_uint = 0x04;
pub const MTL_TX_THRESHOLD_256: c_uint = 0x05;
pub const MTL_TX_THRESHOLD_384: c_uint = 0x06;
pub const MTL_TX_THRESHOLD_512: c_uint = 0x07;
pub const MTL_ETSALG_WRR: c_uint = 0x00;
pub const MTL_ETSALG_WFQ: c_uint = 0x01;
pub const MTL_ETSALG_DWRR: c_uint = 0x02;
pub const MTL_RAA_SP: c_uint = 0x00;
pub const MTL_RAA_WSP: c_uint = 0x01;
pub const MTL_Q_DISABLED: c_uint = 0x00;
pub const MTL_Q_ENABLED: c_uint = 0x02;
// MTL traffic class register offsets
// Multiple traffic classes can be active.  The first class has registers
// that begin at 0x1100.  Each subsequent queue has registers that
// are accessed using an offset of 0x80 from the previous queue.
//

pub const MTL_TC_ETSCR: c_uint = 0x10;
pub const MTL_TC_ETSSR: c_uint = 0x14;
pub const MTL_TC_QWR: c_uint = 0x18;
// MTL traffic class register entry bit positions and sizes
pub const MTL_TC_ETSCR_TSA_INDEX: c_int = 0;
pub const MTL_TC_ETSCR_TSA_WIDTH: c_int = 2;
pub const MTL_TC_QWR_QW_INDEX: c_int = 0;
pub const MTL_TC_QWR_QW_WIDTH: c_int = 21;
// MTL traffic class register value
pub const MTL_TSA_SP: c_uint = 0x00;
pub const MTL_TSA_ETS: c_uint = 0x02;
// PCS register offsets
pub const PCS_V1_WINDOW_SELECT: c_uint = 0x03fc;
pub const PCS_V2_WINDOW_DEF: c_uint = 0x9060;
pub const PCS_V2_WINDOW_SELECT: c_uint = 0x9064;
pub const PCS_V2_RV_WINDOW_DEF: c_uint = 0x1060;
pub const PCS_V2_RV_WINDOW_SELECT: c_uint = 0x1064;
pub const PCS_V2_YC_WINDOW_DEF: c_uint = 0x18060;
pub const PCS_V2_YC_WINDOW_SELECT: c_uint = 0x18064;
pub const PCS_V3_RN_WINDOW_DEF: c_uint = 0xf8078;
pub const PCS_V3_RN_WINDOW_SELECT: c_uint = 0xf807c;
pub const PCS_P100a_WINDOW_DEF: c_uint = 0x8060;
pub const PCS_P100a_WINDOW_SELECT: c_uint = 0x8080;
pub const PCS_RN_SMN_BASE_ADDR: c_uint = 0x11e00000;
pub const PCS_RN_PORT_ADDR_SIZE: c_uint = 0x100000;
// PCS register entry bit positions and sizes
pub const PCS_V2_WINDOW_DEF_OFFSET_INDEX: c_int = 6;
pub const PCS_V2_WINDOW_DEF_OFFSET_WIDTH: c_int = 14;
pub const PCS_V2_WINDOW_DEF_SIZE_INDEX: c_int = 2;
pub const PCS_V2_WINDOW_DEF_SIZE_WIDTH: c_int = 4;
// SerDes integration register offsets
pub const SIR0_KR_RT_1: c_uint = 0x002c;
pub const SIR0_STATUS: c_uint = 0x0040;
pub const SIR1_SPEED: c_uint = 0x0000;
// SerDes integration register entry bit positions and sizes
pub const SIR0_KR_RT_1_RESET_INDEX: c_int = 11;
pub const SIR0_KR_RT_1_RESET_WIDTH: c_int = 1;
pub const SIR0_STATUS_RX_READY_INDEX: c_int = 0;
pub const SIR0_STATUS_RX_READY_WIDTH: c_int = 1;
pub const SIR0_STATUS_TX_READY_INDEX: c_int = 8;
pub const SIR0_STATUS_TX_READY_WIDTH: c_int = 1;
pub const SIR1_SPEED_CDR_RATE_INDEX: c_int = 12;
pub const SIR1_SPEED_CDR_RATE_WIDTH: c_int = 4;
pub const SIR1_SPEED_DATARATE_INDEX: c_int = 4;
pub const SIR1_SPEED_DATARATE_WIDTH: c_int = 2;
pub const SIR1_SPEED_PLLSEL_INDEX: c_int = 3;
pub const SIR1_SPEED_PLLSEL_WIDTH: c_int = 1;
pub const SIR1_SPEED_RATECHANGE_INDEX: c_int = 6;
pub const SIR1_SPEED_RATECHANGE_WIDTH: c_int = 1;
pub const SIR1_SPEED_TXAMP_INDEX: c_int = 8;
pub const SIR1_SPEED_TXAMP_WIDTH: c_int = 4;
pub const SIR1_SPEED_WORDMODE_INDEX: c_int = 0;
pub const SIR1_SPEED_WORDMODE_WIDTH: c_int = 3;
// SerDes RxTx register offsets
pub const RXTX_REG6: c_uint = 0x0018;
pub const RXTX_REG20: c_uint = 0x0050;
pub const RXTX_REG22: c_uint = 0x0058;
pub const RXTX_REG114: c_uint = 0x01c8;
pub const RXTX_REG129: c_uint = 0x0204;
// SerDes RxTx register entry bit positions and sizes
pub const RXTX_REG6_RESETB_RXD_INDEX: c_int = 8;
pub const RXTX_REG6_RESETB_RXD_WIDTH: c_int = 1;
pub const RXTX_REG20_BLWC_ENA_INDEX: c_int = 2;
pub const RXTX_REG20_BLWC_ENA_WIDTH: c_int = 1;
pub const RXTX_REG114_PQ_REG_INDEX: c_int = 9;
pub const RXTX_REG114_PQ_REG_WIDTH: c_int = 7;
pub const RXTX_REG129_RXDFE_CONFIG_INDEX: c_int = 14;
pub const RXTX_REG129_RXDFE_CONFIG_WIDTH: c_int = 2;
// MAC Control register offsets
pub const XP_PROP_0: c_uint = 0x0000;
pub const XP_PROP_1: c_uint = 0x0004;
pub const XP_PROP_2: c_uint = 0x0008;
pub const XP_PROP_3: c_uint = 0x000c;
pub const XP_PROP_4: c_uint = 0x0010;
pub const XP_PROP_5: c_uint = 0x0014;
pub const XP_MAC_ADDR_LO: c_uint = 0x0020;
pub const XP_MAC_ADDR_HI: c_uint = 0x0024;
pub const XP_ECC_ISR: c_uint = 0x0030;
pub const XP_ECC_IER: c_uint = 0x0034;
pub const XP_ECC_CNT0: c_uint = 0x003c;
pub const XP_ECC_CNT1: c_uint = 0x0040;
pub const XP_DRIVER_INT_REQ: c_uint = 0x0060;
pub const XP_DRIVER_INT_RO: c_uint = 0x0064;
pub const XP_DRIVER_SCRATCH_0: c_uint = 0x0068;
pub const XP_DRIVER_SCRATCH_1: c_uint = 0x006c;
pub const XP_INT_REISSUE_EN: c_uint = 0x0074;
pub const XP_INT_EN: c_uint = 0x0078;
pub const XP_I2C_MUTEX: c_uint = 0x0080;
pub const XP_MDIO_MUTEX: c_uint = 0x0084;
// MAC Control register entry bit positions and sizes
pub const XP_DRIVER_INT_REQ_REQUEST_INDEX: c_int = 0;
pub const XP_DRIVER_INT_REQ_REQUEST_WIDTH: c_int = 1;
pub const XP_DRIVER_INT_RO_STATUS_INDEX: c_int = 0;
pub const XP_DRIVER_INT_RO_STATUS_WIDTH: c_int = 1;
pub const XP_DRIVER_SCRATCH_0_COMMAND_INDEX: c_int = 0;
pub const XP_DRIVER_SCRATCH_0_COMMAND_WIDTH: c_int = 8;
pub const XP_DRIVER_SCRATCH_0_SUB_COMMAND_INDEX: c_int = 8;
pub const XP_DRIVER_SCRATCH_0_SUB_COMMAND_WIDTH: c_int = 8;
pub const XP_ECC_CNT0_RX_DED_INDEX: c_int = 24;
pub const XP_ECC_CNT0_RX_DED_WIDTH: c_int = 8;
pub const XP_ECC_CNT0_RX_SEC_INDEX: c_int = 16;
pub const XP_ECC_CNT0_RX_SEC_WIDTH: c_int = 8;
pub const XP_ECC_CNT0_TX_DED_INDEX: c_int = 8;
pub const XP_ECC_CNT0_TX_DED_WIDTH: c_int = 8;
pub const XP_ECC_CNT0_TX_SEC_INDEX: c_int = 0;
pub const XP_ECC_CNT0_TX_SEC_WIDTH: c_int = 8;
pub const XP_ECC_CNT1_DESC_DED_INDEX: c_int = 8;
pub const XP_ECC_CNT1_DESC_DED_WIDTH: c_int = 8;
pub const XP_ECC_CNT1_DESC_SEC_INDEX: c_int = 0;
pub const XP_ECC_CNT1_DESC_SEC_WIDTH: c_int = 8;
pub const XP_ECC_IER_DESC_DED_INDEX: c_int = 5;
pub const XP_ECC_IER_DESC_DED_WIDTH: c_int = 1;
pub const XP_ECC_IER_DESC_SEC_INDEX: c_int = 4;
pub const XP_ECC_IER_DESC_SEC_WIDTH: c_int = 1;
pub const XP_ECC_IER_RX_DED_INDEX: c_int = 3;
pub const XP_ECC_IER_RX_DED_WIDTH: c_int = 1;
pub const XP_ECC_IER_RX_SEC_INDEX: c_int = 2;
pub const XP_ECC_IER_RX_SEC_WIDTH: c_int = 1;
pub const XP_ECC_IER_TX_DED_INDEX: c_int = 1;
pub const XP_ECC_IER_TX_DED_WIDTH: c_int = 1;
pub const XP_ECC_IER_TX_SEC_INDEX: c_int = 0;
pub const XP_ECC_IER_TX_SEC_WIDTH: c_int = 1;
pub const XP_ECC_ISR_DESC_DED_INDEX: c_int = 5;
pub const XP_ECC_ISR_DESC_DED_WIDTH: c_int = 1;
pub const XP_ECC_ISR_DESC_SEC_INDEX: c_int = 4;
pub const XP_ECC_ISR_DESC_SEC_WIDTH: c_int = 1;
pub const XP_ECC_ISR_RX_DED_INDEX: c_int = 3;
pub const XP_ECC_ISR_RX_DED_WIDTH: c_int = 1;
pub const XP_ECC_ISR_RX_SEC_INDEX: c_int = 2;
pub const XP_ECC_ISR_RX_SEC_WIDTH: c_int = 1;
pub const XP_ECC_ISR_TX_DED_INDEX: c_int = 1;
pub const XP_ECC_ISR_TX_DED_WIDTH: c_int = 1;
pub const XP_ECC_ISR_TX_SEC_INDEX: c_int = 0;
pub const XP_ECC_ISR_TX_SEC_WIDTH: c_int = 1;
pub const XP_I2C_MUTEX_BUSY_INDEX: c_int = 31;
pub const XP_I2C_MUTEX_BUSY_WIDTH: c_int = 1;
pub const XP_I2C_MUTEX_ID_INDEX: c_int = 29;
pub const XP_I2C_MUTEX_ID_WIDTH: c_int = 2;
pub const XP_I2C_MUTEX_ACTIVE_INDEX: c_int = 0;
pub const XP_I2C_MUTEX_ACTIVE_WIDTH: c_int = 1;
pub const XP_MAC_ADDR_HI_VALID_INDEX: c_int = 31;
pub const XP_MAC_ADDR_HI_VALID_WIDTH: c_int = 1;
pub const XP_PROP_0_CONN_TYPE_INDEX: c_int = 28;
pub const XP_PROP_0_CONN_TYPE_WIDTH: c_int = 3;
pub const XP_PROP_0_MDIO_ADDR_INDEX: c_int = 16;
pub const XP_PROP_0_MDIO_ADDR_WIDTH: c_int = 5;
pub const XP_PROP_0_PORT_ID_INDEX: c_int = 0;
pub const XP_PROP_0_PORT_ID_WIDTH: c_int = 8;
pub const XP_PROP_0_PORT_MODE_INDEX: c_int = 8;
pub const XP_PROP_0_PORT_MODE_WIDTH: c_int = 4;
pub const XP_PROP_0_PORT_SPEEDS_INDEX: c_int = 22;
pub const XP_PROP_0_PORT_SPEEDS_WIDTH: c_int = 6;
pub const XP_PROP_1_MAX_RX_DMA_INDEX: c_int = 24;
pub const XP_PROP_1_MAX_RX_DMA_WIDTH: c_int = 5;
pub const XP_PROP_1_MAX_RX_QUEUES_INDEX: c_int = 8;
pub const XP_PROP_1_MAX_RX_QUEUES_WIDTH: c_int = 5;
pub const XP_PROP_1_MAX_TX_DMA_INDEX: c_int = 16;
pub const XP_PROP_1_MAX_TX_DMA_WIDTH: c_int = 5;
pub const XP_PROP_1_MAX_TX_QUEUES_INDEX: c_int = 0;
pub const XP_PROP_1_MAX_TX_QUEUES_WIDTH: c_int = 5;
pub const XP_PROP_2_RX_FIFO_SIZE_INDEX: c_int = 16;
pub const XP_PROP_2_RX_FIFO_SIZE_WIDTH: c_int = 16;
pub const XP_PROP_2_TX_FIFO_SIZE_INDEX: c_int = 0;
pub const XP_PROP_2_TX_FIFO_SIZE_WIDTH: c_int = 16;
pub const XP_PROP_3_GPIO_MASK_INDEX: c_int = 28;
pub const XP_PROP_3_GPIO_MASK_WIDTH: c_int = 4;
pub const XP_PROP_3_GPIO_MOD_ABS_INDEX: c_int = 20;
pub const XP_PROP_3_GPIO_MOD_ABS_WIDTH: c_int = 4;
pub const XP_PROP_3_GPIO_RATE_SELECT_INDEX: c_int = 16;
pub const XP_PROP_3_GPIO_RATE_SELECT_WIDTH: c_int = 4;
pub const XP_PROP_3_GPIO_RX_LOS_INDEX: c_int = 24;
pub const XP_PROP_3_GPIO_RX_LOS_WIDTH: c_int = 4;
pub const XP_PROP_3_GPIO_TX_FAULT_INDEX: c_int = 12;
pub const XP_PROP_3_GPIO_TX_FAULT_WIDTH: c_int = 4;
pub const XP_PROP_3_GPIO_ADDR_INDEX: c_int = 8;
pub const XP_PROP_3_GPIO_ADDR_WIDTH: c_int = 3;
pub const XP_PROP_3_MDIO_RESET_INDEX: c_int = 0;
pub const XP_PROP_3_MDIO_RESET_WIDTH: c_int = 2;
pub const XP_PROP_3_MDIO_RESET_I2C_ADDR_INDEX: c_int = 8;
pub const XP_PROP_3_MDIO_RESET_I2C_ADDR_WIDTH: c_int = 3;
pub const XP_PROP_3_MDIO_RESET_I2C_GPIO_INDEX: c_int = 12;
pub const XP_PROP_3_MDIO_RESET_I2C_GPIO_WIDTH: c_int = 4;
pub const XP_PROP_3_MDIO_RESET_INT_GPIO_INDEX: c_int = 4;
pub const XP_PROP_3_MDIO_RESET_INT_GPIO_WIDTH: c_int = 2;
pub const XP_PROP_4_MUX_ADDR_HI_INDEX: c_int = 8;
pub const XP_PROP_4_MUX_ADDR_HI_WIDTH: c_int = 5;
pub const XP_PROP_4_MUX_ADDR_LO_INDEX: c_int = 0;
pub const XP_PROP_4_MUX_ADDR_LO_WIDTH: c_int = 3;
pub const XP_PROP_4_MUX_CHAN_INDEX: c_int = 4;
pub const XP_PROP_4_MUX_CHAN_WIDTH: c_int = 3;
pub const XP_PROP_4_REDRV_ADDR_INDEX: c_int = 16;
pub const XP_PROP_4_REDRV_ADDR_WIDTH: c_int = 7;
pub const XP_PROP_4_REDRV_IF_INDEX: c_int = 23;
pub const XP_PROP_4_REDRV_IF_WIDTH: c_int = 1;
pub const XP_PROP_4_REDRV_LANE_INDEX: c_int = 24;
pub const XP_PROP_4_REDRV_LANE_WIDTH: c_int = 3;
pub const XP_PROP_4_REDRV_MODEL_INDEX: c_int = 28;
pub const XP_PROP_4_REDRV_MODEL_WIDTH: c_int = 3;
pub const XP_PROP_4_REDRV_PRESENT_INDEX: c_int = 31;
pub const XP_PROP_4_REDRV_PRESENT_WIDTH: c_int = 1;
// I2C Control register offsets
pub const IC_CON: c_uint = 0x0000;
pub const IC_TAR: c_uint = 0x0004;
pub const IC_DATA_CMD: c_uint = 0x0010;
pub const IC_INTR_STAT: c_uint = 0x002c;
pub const IC_INTR_MASK: c_uint = 0x0030;
pub const IC_RAW_INTR_STAT: c_uint = 0x0034;
pub const IC_CLR_INTR: c_uint = 0x0040;
pub const IC_CLR_TX_ABRT: c_uint = 0x0054;
pub const IC_CLR_STOP_DET: c_uint = 0x0060;
pub const IC_ENABLE: c_uint = 0x006c;
pub const IC_TXFLR: c_uint = 0x0074;
pub const IC_RXFLR: c_uint = 0x0078;
pub const IC_TX_ABRT_SOURCE: c_uint = 0x0080;
pub const IC_ENABLE_STATUS: c_uint = 0x009c;
pub const IC_COMP_PARAM_1: c_uint = 0x00f4;
// I2C Control register entry bit positions and sizes
pub const IC_COMP_PARAM_1_MAX_SPEED_MODE_INDEX: c_int = 2;
pub const IC_COMP_PARAM_1_MAX_SPEED_MODE_WIDTH: c_int = 2;
pub const IC_COMP_PARAM_1_RX_BUFFER_DEPTH_INDEX: c_int = 8;
pub const IC_COMP_PARAM_1_RX_BUFFER_DEPTH_WIDTH: c_int = 8;
pub const IC_COMP_PARAM_1_TX_BUFFER_DEPTH_INDEX: c_int = 16;
pub const IC_COMP_PARAM_1_TX_BUFFER_DEPTH_WIDTH: c_int = 8;
pub const IC_CON_MASTER_MODE_INDEX: c_int = 0;
pub const IC_CON_MASTER_MODE_WIDTH: c_int = 1;
pub const IC_CON_RESTART_EN_INDEX: c_int = 5;
pub const IC_CON_RESTART_EN_WIDTH: c_int = 1;
pub const IC_CON_RX_FIFO_FULL_HOLD_INDEX: c_int = 9;
pub const IC_CON_RX_FIFO_FULL_HOLD_WIDTH: c_int = 1;
pub const IC_CON_SLAVE_DISABLE_INDEX: c_int = 6;
pub const IC_CON_SLAVE_DISABLE_WIDTH: c_int = 1;
pub const IC_CON_SPEED_INDEX: c_int = 1;
pub const IC_CON_SPEED_WIDTH: c_int = 2;
pub const IC_DATA_CMD_CMD_INDEX: c_int = 8;
pub const IC_DATA_CMD_CMD_WIDTH: c_int = 1;
pub const IC_DATA_CMD_STOP_INDEX: c_int = 9;
pub const IC_DATA_CMD_STOP_WIDTH: c_int = 1;
pub const IC_ENABLE_ABORT_INDEX: c_int = 1;
pub const IC_ENABLE_ABORT_WIDTH: c_int = 1;
pub const IC_ENABLE_EN_INDEX: c_int = 0;
pub const IC_ENABLE_EN_WIDTH: c_int = 1;
pub const IC_ENABLE_STATUS_EN_INDEX: c_int = 0;
pub const IC_ENABLE_STATUS_EN_WIDTH: c_int = 1;
pub const IC_INTR_MASK_TX_EMPTY_INDEX: c_int = 4;
pub const IC_INTR_MASK_TX_EMPTY_WIDTH: c_int = 1;
pub const IC_RAW_INTR_STAT_RX_FULL_INDEX: c_int = 2;
pub const IC_RAW_INTR_STAT_RX_FULL_WIDTH: c_int = 1;
pub const IC_RAW_INTR_STAT_STOP_DET_INDEX: c_int = 9;
pub const IC_RAW_INTR_STAT_STOP_DET_WIDTH: c_int = 1;
pub const IC_RAW_INTR_STAT_TX_ABRT_INDEX: c_int = 6;
pub const IC_RAW_INTR_STAT_TX_ABRT_WIDTH: c_int = 1;
pub const IC_RAW_INTR_STAT_TX_EMPTY_INDEX: c_int = 4;
pub const IC_RAW_INTR_STAT_TX_EMPTY_WIDTH: c_int = 1;
// I2C Control register value
pub const IC_TX_ABRT_7B_ADDR_NOACK: c_uint = 0x0001;
pub const IC_TX_ABRT_ARB_LOST: c_uint = 0x1000;
// Descriptor/Packet entry bit positions and sizes
pub const RX_PACKET_ERRORS_CRC_INDEX: c_int = 2;
pub const RX_PACKET_ERRORS_CRC_WIDTH: c_int = 1;
pub const RX_PACKET_ERRORS_FRAME_INDEX: c_int = 3;
pub const RX_PACKET_ERRORS_FRAME_WIDTH: c_int = 1;
pub const RX_PACKET_ERRORS_LENGTH_INDEX: c_int = 0;
pub const RX_PACKET_ERRORS_LENGTH_WIDTH: c_int = 1;
pub const RX_PACKET_ERRORS_OVERRUN_INDEX: c_int = 1;
pub const RX_PACKET_ERRORS_OVERRUN_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_CSUM_DONE_INDEX: c_int = 0;
pub const RX_PACKET_ATTRIBUTES_CSUM_DONE_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_VLAN_CTAG_INDEX: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_VLAN_CTAG_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_LAST_INDEX: c_int = 2;
pub const RX_PACKET_ATTRIBUTES_LAST_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_CONTEXT_NEXT_INDEX: c_int = 3;
pub const RX_PACKET_ATTRIBUTES_CONTEXT_NEXT_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_CONTEXT_INDEX: c_int = 4;
pub const RX_PACKET_ATTRIBUTES_CONTEXT_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_RX_TSTAMP_INDEX: c_int = 5;
pub const RX_PACKET_ATTRIBUTES_RX_TSTAMP_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_RSS_HASH_INDEX: c_int = 6;
pub const RX_PACKET_ATTRIBUTES_RSS_HASH_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_FIRST_INDEX: c_int = 7;
pub const RX_PACKET_ATTRIBUTES_FIRST_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_TNP_INDEX: c_int = 8;
pub const RX_PACKET_ATTRIBUTES_TNP_WIDTH: c_int = 1;
pub const RX_PACKET_ATTRIBUTES_TNPCSUM_DONE_INDEX: c_int = 9;
pub const RX_PACKET_ATTRIBUTES_TNPCSUM_DONE_WIDTH: c_int = 1;
pub const RX_NORMAL_DESC0_OVT_INDEX: c_int = 0;
pub const RX_NORMAL_DESC0_OVT_WIDTH: c_int = 16;
pub const RX_NORMAL_DESC2_HL_INDEX: c_int = 0;
pub const RX_NORMAL_DESC2_HL_WIDTH: c_int = 10;
pub const RX_NORMAL_DESC2_TNP_INDEX: c_int = 11;
pub const RX_NORMAL_DESC2_TNP_WIDTH: c_int = 1;
pub const RX_NORMAL_DESC3_CDA_INDEX: c_int = 27;
pub const RX_NORMAL_DESC3_CDA_WIDTH: c_int = 1;
pub const RX_NORMAL_DESC3_CTXT_INDEX: c_int = 30;
pub const RX_NORMAL_DESC3_CTXT_WIDTH: c_int = 1;
pub const RX_NORMAL_DESC3_ES_INDEX: c_int = 15;
pub const RX_NORMAL_DESC3_ES_WIDTH: c_int = 1;
pub const RX_NORMAL_DESC3_ETLT_INDEX: c_int = 16;
pub const RX_NORMAL_DESC3_ETLT_WIDTH: c_int = 4;
pub const RX_NORMAL_DESC3_FD_INDEX: c_int = 29;
pub const RX_NORMAL_DESC3_FD_WIDTH: c_int = 1;
pub const RX_NORMAL_DESC3_INTE_INDEX: c_int = 30;
pub const RX_NORMAL_DESC3_INTE_WIDTH: c_int = 1;
pub const RX_NORMAL_DESC3_L34T_INDEX: c_int = 20;
pub const RX_NORMAL_DESC3_L34T_WIDTH: c_int = 4;
pub const RX_NORMAL_DESC3_LD_INDEX: c_int = 28;
pub const RX_NORMAL_DESC3_LD_WIDTH: c_int = 1;
pub const RX_NORMAL_DESC3_OWN_INDEX: c_int = 31;
pub const RX_NORMAL_DESC3_OWN_WIDTH: c_int = 1;
pub const RX_NORMAL_DESC3_PL_INDEX: c_int = 0;
pub const RX_NORMAL_DESC3_PL_WIDTH: c_int = 14;
pub const RX_NORMAL_DESC3_RSV_INDEX: c_int = 26;
pub const RX_NORMAL_DESC3_RSV_WIDTH: c_int = 1;
pub const RX_DESC3_L34T_IPV4_TCP: c_int = 1;
pub const RX_DESC3_L34T_IPV4_UDP: c_int = 2;
pub const RX_DESC3_L34T_IPV4_ICMP: c_int = 3;
pub const RX_DESC3_L34T_IPV4_UNKNOWN: c_int = 7;
pub const RX_DESC3_L34T_IPV6_TCP: c_int = 9;
pub const RX_DESC3_L34T_IPV6_UDP: c_int = 10;
pub const RX_DESC3_L34T_IPV6_ICMP: c_int = 11;
pub const RX_DESC3_L34T_IPV6_UNKNOWN: c_int = 15;
pub const RX_CONTEXT_DESC3_TSA_INDEX: c_int = 4;
pub const RX_CONTEXT_DESC3_TSA_WIDTH: c_int = 1;
pub const RX_CONTEXT_DESC3_TSD_INDEX: c_int = 6;
pub const RX_CONTEXT_DESC3_TSD_WIDTH: c_int = 1;
pub const TX_PACKET_ATTRIBUTES_CSUM_ENABLE_INDEX: c_int = 0;
pub const TX_PACKET_ATTRIBUTES_CSUM_ENABLE_WIDTH: c_int = 1;
pub const TX_PACKET_ATTRIBUTES_TSO_ENABLE_INDEX: c_int = 1;
pub const TX_PACKET_ATTRIBUTES_TSO_ENABLE_WIDTH: c_int = 1;
pub const TX_PACKET_ATTRIBUTES_VLAN_CTAG_INDEX: c_int = 2;
pub const TX_PACKET_ATTRIBUTES_VLAN_CTAG_WIDTH: c_int = 1;
pub const TX_PACKET_ATTRIBUTES_PTP_INDEX: c_int = 3;
pub const TX_PACKET_ATTRIBUTES_PTP_WIDTH: c_int = 1;
pub const TX_PACKET_ATTRIBUTES_VXLAN_INDEX: c_int = 4;
pub const TX_PACKET_ATTRIBUTES_VXLAN_WIDTH: c_int = 1;
pub const TX_CONTEXT_DESC2_MSS_INDEX: c_int = 0;
pub const TX_CONTEXT_DESC2_MSS_WIDTH: c_int = 15;
pub const TX_CONTEXT_DESC3_CTXT_INDEX: c_int = 30;
pub const TX_CONTEXT_DESC3_CTXT_WIDTH: c_int = 1;
pub const TX_CONTEXT_DESC3_TCMSSV_INDEX: c_int = 26;
pub const TX_CONTEXT_DESC3_TCMSSV_WIDTH: c_int = 1;
pub const TX_CONTEXT_DESC3_VLTV_INDEX: c_int = 16;
pub const TX_CONTEXT_DESC3_VLTV_WIDTH: c_int = 1;
pub const TX_CONTEXT_DESC3_VT_INDEX: c_int = 0;
pub const TX_CONTEXT_DESC3_VT_WIDTH: c_int = 16;
pub const TX_NORMAL_DESC2_HL_B1L_INDEX: c_int = 0;
pub const TX_NORMAL_DESC2_HL_B1L_WIDTH: c_int = 14;
pub const TX_NORMAL_DESC2_IC_INDEX: c_int = 31;
pub const TX_NORMAL_DESC2_IC_WIDTH: c_int = 1;
pub const TX_NORMAL_DESC2_TTSE_INDEX: c_int = 30;
pub const TX_NORMAL_DESC2_TTSE_WIDTH: c_int = 1;
pub const TX_NORMAL_DESC2_VTIR_INDEX: c_int = 14;
pub const TX_NORMAL_DESC2_VTIR_WIDTH: c_int = 2;
pub const TX_NORMAL_DESC3_CIC_INDEX: c_int = 16;
pub const TX_NORMAL_DESC3_CIC_WIDTH: c_int = 2;
pub const TX_NORMAL_DESC3_CPC_INDEX: c_int = 26;
pub const TX_NORMAL_DESC3_CPC_WIDTH: c_int = 2;
pub const TX_NORMAL_DESC3_CTXT_INDEX: c_int = 30;
pub const TX_NORMAL_DESC3_CTXT_WIDTH: c_int = 1;
pub const TX_NORMAL_DESC3_FD_INDEX: c_int = 29;
pub const TX_NORMAL_DESC3_FD_WIDTH: c_int = 1;
pub const TX_NORMAL_DESC3_FL_INDEX: c_int = 0;
pub const TX_NORMAL_DESC3_FL_WIDTH: c_int = 15;
pub const TX_NORMAL_DESC3_LD_INDEX: c_int = 28;
pub const TX_NORMAL_DESC3_LD_WIDTH: c_int = 1;
pub const TX_NORMAL_DESC3_OWN_INDEX: c_int = 31;
pub const TX_NORMAL_DESC3_OWN_WIDTH: c_int = 1;
pub const TX_NORMAL_DESC3_TCPHDRLEN_INDEX: c_int = 19;
pub const TX_NORMAL_DESC3_TCPHDRLEN_WIDTH: c_int = 4;
pub const TX_NORMAL_DESC3_TCPPL_INDEX: c_int = 0;
pub const TX_NORMAL_DESC3_TCPPL_WIDTH: c_int = 18;
pub const TX_NORMAL_DESC3_TSE_INDEX: c_int = 18;
pub const TX_NORMAL_DESC3_TSE_WIDTH: c_int = 1;
pub const TX_NORMAL_DESC3_VNP_INDEX: c_int = 23;
pub const TX_NORMAL_DESC3_VNP_WIDTH: c_int = 3;
pub const TX_NORMAL_DESC2_VLAN_INSERT: c_uint = 0x2;
pub const TX_NORMAL_DESC3_VXLAN_PACKET: c_uint = 0x3;
// MDIO undefined or vendor specific registers

pub const MDIO_PMA_10GBR_PMD_CTRL: c_uint = 0x0096;

pub const MDIO_PMA_10GBR_FECCTRL: c_uint = 0x00ab;

pub const MDIO_PMA_RX_CTRL1: c_uint = 0x8051;

pub const MDIO_PMA_RX_LSTS: c_uint = 0x018020;

pub const MDIO_PMA_RX_EQ_CTRL4: c_uint = 0x0001805C;

pub const MDIO_PMA_MP_MISC_STS: c_uint = 0x0078;

pub const MDIO_PMA_PHY_RX_EQ_CEU: c_uint = 0x1800E;

pub const MDIO_PCS_DIG_CTRL: c_uint = 0x8000;

pub const MDIO_PCS_DIGITAL_STAT: c_uint = 0x8010;

pub const MDIO_AN_XNP: c_uint = 0x0016;

pub const MDIO_AN_LPX: c_uint = 0x0019;

pub const MDIO_AN_COMP_STAT: c_uint = 0x0030;

pub const MDIO_AN_INTMASK: c_uint = 0x8001;

pub const MDIO_AN_INT: c_uint = 0x8002;

pub const MDIO_VEND2_AN_ADVERTISE: c_uint = 0x0004;

pub const MDIO_VEND2_AN_LP_ABILITY: c_uint = 0x0005;

pub const MDIO_VEND2_AN_CTRL: c_uint = 0x8001;

pub const MDIO_VEND2_AN_STAT: c_uint = 0x8002;

pub const MDIO_VEND2_PMA_CDR_CONTROL: c_uint = 0x8056;

pub const MDIO_VEND2_PMA_MISC_CTRL0: c_uint = 0x8090;

// MDIO mask values

pub const XGBE_AN_CL73_INT_MASK: c_uint = 0x07;
pub const XGBE_XNP_MCF_NULL_MESSAGE: c_uint = 0x001;

pub const XGBE_PCS_PSEQ_STATE_MASK: c_uint = 0x1c;
pub const XGBE_PCS_PSEQ_STATE_POWER_GOOD: c_uint = 0x10;

pub const XGBE_AN_CL37_INT_MASK: c_uint = 0x01;
pub const XGBE_AN_CL37_HD_MASK: c_uint = 0x40;
pub const XGBE_AN_CL37_FD_MASK: c_uint = 0x20;
pub const XGBE_AN_CL37_PCS_MODE_MASK: c_uint = 0x06;
pub const XGBE_AN_CL37_PCS_MODE_BASEX: c_uint = 0x00;
pub const XGBE_AN_CL37_PCS_MODE_SGMII: c_uint = 0x04;
pub const XGBE_AN_CL37_TX_CONFIG_MASK: c_uint = 0x08;
pub const XGBE_AN_CL37_MII_CTRL_8BIT: c_uint = 0x0100;
pub const XGBE_PMA_CDR_TRACK_EN_MASK: c_uint = 0x01;
pub const XGBE_PMA_CDR_TRACK_EN_OFF: c_uint = 0x00;
pub const XGBE_PMA_CDR_TRACK_EN_ON: c_uint = 0x01;

pub const XGBE_PMA_RX_RST_0_RESET_ON: c_uint = 0x10;
pub const XGBE_PMA_RX_RST_0_RESET_OFF: c_uint = 0x00;

pub const XGBE_PMA_RX_SIG_DET_0_DISABLE: c_uint = 0x0000;

pub const XGBE_PMA_RX_VALID_0_DISABLE: c_uint = 0x0000;

pub const XGBE_PMA_RX_AD_REQ_DISABLE: c_uint = 0x0000;

pub const XGBE_PMA_PLL_CTRL_DISABLE: c_uint = 0x0000;
// Bit setting and getting macros
// The get macro will extract the current bit field value from within
// the variable
//
// The set macro will clear the current bit field value within the
// variable and then set the bit field of the variable to the
// specified value
//

// Bit setting and getting macros based on register fields
// The get macro uses the bit field definitions formed using the input
// names to extract the current bit field value from within the
// variable
//
// The set macro uses the bit field definitions formed using the input
// names to set the bit field of the variable to the specified value
//

// Macros for reading or writing registers
// The ioread macros will get bit fields or full values using the
// register definitions formed using the input names
//
// The iowrite macros will set bit fields or full values using the
// register definitions formed using the input names
//

// Macros for reading or writing MTL queue or traffic class registers
// Similar to the standard read and write macros except that the
// base register value is calculated by the queue or traffic class number
//

// Macros for reading or writing DMA channel registers
// Similar to the standard read and write macros except that the
// base register value is obtained from the ring
//

// Macros for building, reading or writing register values or bits
// within the register values of XPCS registers.
//

// Macros for building, reading or writing register values or bits
// within the register values of SerDes integration registers.
//

// Macros for building, reading or writing register values or bits
// within the register values of SerDes RxTx registers.
//

// Macros for building, reading or writing register values or bits
// within the register values of MAC Control registers.
//

// Macros for building, reading or writing register values or bits
// within the register values of I2C Control registers.
//

// Macros for building, reading or writing register values or bits
// using MDIO.
//

