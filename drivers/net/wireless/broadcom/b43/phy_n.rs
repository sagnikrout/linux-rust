//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/phy_n.h
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

// N-PHY registers.

pub const B43_NPHY_BBCFG_RSTCCA: c_uint = 0x4000 /* Reset CCA */;
pub const B43_NPHY_BBCFG_RSTRX: c_uint = 0x8000 /* Reset RX */;

pub const B43_NPHY_BANDCTL_5GHZ: c_uint = 0x0001 /* Use the 5GHz band */;

pub const B43_NPHY_C1_CGAINI_GAINBKOFF: c_uint = 0x001F /* Gain backoff */;
pub const B43_NPHY_C1_CGAINI_GAINBKOFF_SHIFT: c_int = 0;
pub const B43_NPHY_C1_CGAINI_CLIPGBKOFF: c_uint = 0x03E0 /* Clip gain backoff */;
pub const B43_NPHY_C1_CGAINI_CLIPGBKOFF_SHIFT: c_int = 5;
pub const B43_NPHY_C1_CGAINI_GAINSTEP: c_uint = 0x1C00 /* Gain step */;
pub const B43_NPHY_C1_CGAINI_GAINSTEP_SHIFT: c_int = 10;
pub const B43_NPHY_C1_CGAINI_CL2DETECT: c_uint = 0x2000 /* Clip 2 detect mask */;

pub const B43_NPHY_C1_CCK_CGAINI_GAINBKOFF: c_uint = 0x001F /* Gain backoff */;
pub const B43_NPHY_C1_CCK_CGAINI_CLIPGBKOFF: c_uint = 0x01E0 /* CCK barely clip gain backoff */;

pub const B43_NPHY_C1_MINGAIN: c_uint = 0x00FF /* Minimum gain */;
pub const B43_NPHY_C1_MINGAIN_SHIFT: c_int = 0;
pub const B43_NPHY_C1_MAXGAIN: c_uint = 0xFF00 /* Maximum gain */;
pub const B43_NPHY_C1_MAXGAIN_SHIFT: c_int = 8;

pub const B43_NPHY_C1_CCK_MINGAIN: c_uint = 0x00FF /* Minimum gain */;
pub const B43_NPHY_C1_CCK_MINGAIN_SHIFT: c_int = 0;
pub const B43_NPHY_C1_CCK_MAXGAIN: c_uint = 0xFF00 /* Maximum gain */;
pub const B43_NPHY_C1_CCK_MAXGAIN_SHIFT: c_int = 8;

pub const B43_NPHY_C1_INITGAIN_EXTLNA: c_uint = 0x0001 /* External LNA index */;
pub const B43_NPHY_C1_INITGAIN_LNA: c_uint = 0x0006 /* LNA index */;
pub const B43_NPHY_C1_INITGAIN_LNAIDX_SHIFT: c_int = 1;
pub const B43_NPHY_C1_INITGAIN_HPVGA1: c_uint = 0x0078 /* HPVGA1 index */;
pub const B43_NPHY_C1_INITGAIN_HPVGA1_SHIFT: c_int = 3;
pub const B43_NPHY_C1_INITGAIN_HPVGA2: c_uint = 0x0F80 /* HPVGA2 index */;
pub const B43_NPHY_C1_INITGAIN_HPVGA2_SHIFT: c_int = 7;
pub const B43_NPHY_C1_INITGAIN_TRRX: c_uint = 0x1000 /* TR RX index */;
pub const B43_NPHY_C1_INITGAIN_TRTX: c_uint = 0x2000 /* TR TX index */;

pub const B43_NPHY_C1_CLIPWBTHRES_CLIP2: c_uint = 0x003F /* Clip 2 */;
pub const B43_NPHY_C1_CLIPWBTHRES_CLIP2_SHIFT: c_int = 0;
pub const B43_NPHY_C1_CLIPWBTHRES_CLIP1: c_uint = 0x0FC0 /* Clip 1 */;
pub const B43_NPHY_C1_CLIPWBTHRES_CLIP1_SHIFT: c_int = 6;

pub const B43_NPHY_C2_CGAINI_GAINBKOFF: c_uint = 0x001F /* Gain backoff */;
pub const B43_NPHY_C2_CGAINI_GAINBKOFF_SHIFT: c_int = 0;
pub const B43_NPHY_C2_CGAINI_CLIPGBKOFF: c_uint = 0x03E0 /* Clip gain backoff */;
pub const B43_NPHY_C2_CGAINI_CLIPGBKOFF_SHIFT: c_int = 5;
pub const B43_NPHY_C2_CGAINI_GAINSTEP: c_uint = 0x1C00 /* Gain step */;
pub const B43_NPHY_C2_CGAINI_GAINSTEP_SHIFT: c_int = 10;
pub const B43_NPHY_C2_CGAINI_CL2DETECT: c_uint = 0x2000 /* Clip 2 detect mask */;

pub const B43_NPHY_C2_CCK_CGAINI_GAINBKOFF: c_uint = 0x001F /* Gain backoff */;
pub const B43_NPHY_C2_CCK_CGAINI_CLIPGBKOFF: c_uint = 0x01E0 /* CCK barely clip gain backoff */;

pub const B43_NPHY_C2_MINGAIN: c_uint = 0x00FF /* Minimum gain */;
pub const B43_NPHY_C2_MINGAIN_SHIFT: c_int = 0;
pub const B43_NPHY_C2_MAXGAIN: c_uint = 0xFF00 /* Maximum gain */;
pub const B43_NPHY_C2_MAXGAIN_SHIFT: c_int = 8;

pub const B43_NPHY_C2_CCK_MINGAIN: c_uint = 0x00FF /* Minimum gain */;
pub const B43_NPHY_C2_CCK_MINGAIN_SHIFT: c_int = 0;
pub const B43_NPHY_C2_CCK_MAXGAIN: c_uint = 0xFF00 /* Maximum gain */;
pub const B43_NPHY_C2_CCK_MAXGAIN_SHIFT: c_int = 8;

pub const B43_NPHY_C2_INITGAIN_EXTLNA: c_uint = 0x0001 /* External LNA index */;
pub const B43_NPHY_C2_INITGAIN_LNA: c_uint = 0x0006 /* LNA index */;
pub const B43_NPHY_C2_INITGAIN_LNAIDX_SHIFT: c_int = 1;
pub const B43_NPHY_C2_INITGAIN_HPVGA1: c_uint = 0x0078 /* HPVGA1 index */;
pub const B43_NPHY_C2_INITGAIN_HPVGA1_SHIFT: c_int = 3;
pub const B43_NPHY_C2_INITGAIN_HPVGA2: c_uint = 0x0F80 /* HPVGA2 index */;
pub const B43_NPHY_C2_INITGAIN_HPVGA2_SHIFT: c_int = 7;
pub const B43_NPHY_C2_INITGAIN_TRRX: c_uint = 0x1000 /* TR RX index */;
pub const B43_NPHY_C2_INITGAIN_TRTX: c_uint = 0x2000 /* TR TX index */;

pub const B43_NPHY_C2_CLIPWBTHRES_CLIP2: c_uint = 0x003F /* Clip 2 */;
pub const B43_NPHY_C2_CLIPWBTHRES_CLIP2_SHIFT: c_int = 0;
pub const B43_NPHY_C2_CLIPWBTHRES_CLIP1: c_uint = 0x0FC0 /* Clip 1 */;
pub const B43_NPHY_C2_CLIPWBTHRES_CLIP1_SHIFT: c_int = 6;

pub const B43_NPHY_RFCTL_CMD_START: c_uint = 0x0001 /* Start sequence */;
pub const B43_NPHY_RFCTL_CMD_RXTX: c_uint = 0x0002 /* RX/TX */;
pub const B43_NPHY_RFCTL_CMD_CORESEL: c_uint = 0x0038 /* Core select */;
pub const B43_NPHY_RFCTL_CMD_CORESEL_SHIFT: c_int = 3;
pub const B43_NPHY_RFCTL_CMD_PORFORCE: c_uint = 0x0040 /* POR force */;
pub const B43_NPHY_RFCTL_CMD_OEPORFORCE: c_uint = 0x0080 /* OE POR force */;
pub const B43_NPHY_RFCTL_CMD_RXEN: c_uint = 0x0100 /* RX enable */;
pub const B43_NPHY_RFCTL_CMD_TXEN: c_uint = 0x0200 /* TX enable */;
pub const B43_NPHY_RFCTL_CMD_CHIP0PU: c_uint = 0x0400 /* Chip0 PU */;
pub const B43_NPHY_RFCTL_CMD_EN: c_uint = 0x0800 /* Radio enabled */;
pub const B43_NPHY_RFCTL_CMD_SEQENCORE: c_uint = 0xF000 /* Seq en core */;
pub const B43_NPHY_RFCTL_CMD_SEQENCORE_SHIFT: c_int = 12;

pub const B43_NPHY_RFCTL_RSSIO1_RXPD: c_uint = 0x0001 /* RX PD */;
pub const B43_NPHY_RFCTL_RSSIO1_TXPD: c_uint = 0x0002 /* TX PD */;
pub const B43_NPHY_RFCTL_RSSIO1_PAPD: c_uint = 0x0004 /* PA PD */;
pub const B43_NPHY_RFCTL_RSSIO1_RSSICTL: c_uint = 0x0030 /* RSSI control */;
pub const B43_NPHY_RFCTL_RSSIO1_LPFBW: c_uint = 0x00C0 /* LPF bandwidth */;
pub const B43_NPHY_RFCTL_RSSIO1_HPFBWHI: c_uint = 0x0100 /* HPF bandwidth high */;
pub const B43_NPHY_RFCTL_RSSIO1_HIQDISCO: c_uint = 0x0200 /* HIQ dis core */;

pub const B43_NPHY_RFCTL_RSSIO2_RXPD: c_uint = 0x0001 /* RX PD */;
pub const B43_NPHY_RFCTL_RSSIO2_TXPD: c_uint = 0x0002 /* TX PD */;
pub const B43_NPHY_RFCTL_RSSIO2_PAPD: c_uint = 0x0004 /* PA PD */;
pub const B43_NPHY_RFCTL_RSSIO2_RSSICTL: c_uint = 0x0030 /* RSSI control */;
pub const B43_NPHY_RFCTL_RSSIO2_LPFBW: c_uint = 0x00C0 /* LPF bandwidth */;
pub const B43_NPHY_RFCTL_RSSIO2_HPFBWHI: c_uint = 0x0100 /* HPF bandwidth high */;
pub const B43_NPHY_RFCTL_RSSIO2_HIQDISCO: c_uint = 0x0200 /* HIQ dis core */;

pub const B43_NPHY_RFCTL_RSSIO3_RXPD: c_uint = 0x0001 /* RX PD */;
pub const B43_NPHY_RFCTL_RSSIO3_TXPD: c_uint = 0x0002 /* TX PD */;
pub const B43_NPHY_RFCTL_RSSIO3_PAPD: c_uint = 0x0004 /* PA PD */;
pub const B43_NPHY_RFCTL_RSSIO3_RSSICTL: c_uint = 0x0030 /* RSSI control */;
pub const B43_NPHY_RFCTL_RSSIO3_LPFBW: c_uint = 0x00C0 /* LPF bandwidth */;
pub const B43_NPHY_RFCTL_RSSIO3_HPFBWHI: c_uint = 0x0100 /* HPF bandwidth high */;
pub const B43_NPHY_RFCTL_RSSIO3_HIQDISCO: c_uint = 0x0200 /* HIQ dis core */;

pub const B43_NPHY_RFCTL_RSSIO4_RXPD: c_uint = 0x0001 /* RX PD */;
pub const B43_NPHY_RFCTL_RSSIO4_TXPD: c_uint = 0x0002 /* TX PD */;
pub const B43_NPHY_RFCTL_RSSIO4_PAPD: c_uint = 0x0004 /* PA PD */;
pub const B43_NPHY_RFCTL_RSSIO4_RSSICTL: c_uint = 0x0030 /* RSSI control */;
pub const B43_NPHY_RFCTL_RSSIO4_LPFBW: c_uint = 0x00C0 /* LPF bandwidth */;
pub const B43_NPHY_RFCTL_RSSIO4_HPFBWHI: c_uint = 0x0100 /* HPF bandwidth high */;
pub const B43_NPHY_RFCTL_RSSIO4_HIQDISCO: c_uint = 0x0200 /* HIQ dis core */;

pub const B43_NPHY_SCRAM_SIGCTL_INITST: c_uint = 0x007F /* Initial state value */;
pub const B43_NPHY_SCRAM_SIGCTL_INITST_SHIFT: c_int = 0;
pub const B43_NPHY_SCRAM_SIGCTL_SCM: c_uint = 0x0080 /* Scram control mode */;
pub const B43_NPHY_SCRAM_SIGCTL_SICE: c_uint = 0x0100 /* Scram index control enable */;
pub const B43_NPHY_SCRAM_SIGCTL_START: c_uint = 0xFE00 /* Scram start bit */;
pub const B43_NPHY_SCRAM_SIGCTL_START_SHIFT: c_int = 9;

pub const B43_NPHY_RXCTL_BSELU20: c_uint = 0x0010 /* Band select upper 20 */;
pub const B43_NPHY_RXCTL_RIFSEN: c_uint = 0x0080 /* RIFS enable */;

pub const B43_NPHY_RFSEQMODE_CAOVER: c_uint = 0x0001 /* Core active override */;
pub const B43_NPHY_RFSEQMODE_TROVER: c_uint = 0x0002 /* Trigger override */;

pub const B43_NPHY_RFSEQCA_TXEN: c_uint = 0x000F /* TX enable */;
pub const B43_NPHY_RFSEQCA_TXEN_SHIFT: c_int = 0;
pub const B43_NPHY_RFSEQCA_RXEN: c_uint = 0x00F0 /* RX enable */;
pub const B43_NPHY_RFSEQCA_RXEN_SHIFT: c_int = 4;
pub const B43_NPHY_RFSEQCA_TXDIS: c_uint = 0x0F00 /* TX disable */;
pub const B43_NPHY_RFSEQCA_TXDIS_SHIFT: c_int = 8;
pub const B43_NPHY_RFSEQCA_RXDIS: c_uint = 0xF000 /* RX disable */;
pub const B43_NPHY_RFSEQCA_RXDIS_SHIFT: c_int = 12;

pub const B43_NPHY_RFSEQTR_RX2TX: c_uint = 0x0001 /* RX2TX */;
pub const B43_NPHY_RFSEQTR_TX2RX: c_uint = 0x0002 /* TX2RX */;
pub const B43_NPHY_RFSEQTR_UPGH: c_uint = 0x0004 /* Update gain H */;
pub const B43_NPHY_RFSEQTR_UPGL: c_uint = 0x0008 /* Update gain L */;
pub const B43_NPHY_RFSEQTR_UPGU: c_uint = 0x0010 /* Update gain U */;
pub const B43_NPHY_RFSEQTR_RST2RX: c_uint = 0x0020 /* Reset to RX */;

pub const B43_NPHY_CLASSCTL_CCKEN: c_uint = 0x0001 /* CCK enable */;
pub const B43_NPHY_CLASSCTL_OFDMEN: c_uint = 0x0002 /* OFDM enable */;
pub const B43_NPHY_CLASSCTL_WAITEDEN: c_uint = 0x0004 /* Waited enable */;

pub const B43_NPHY_IQFLIP_ADC1: c_uint = 0x0001 /* ADC1 */;
pub const B43_NPHY_IQFLIP_ADC2: c_uint = 0x0010 /* ADC2 */;

pub const B43_NPHY_BPHY_CTL2_LUT: c_uint = 0x001F /* LUT index */;
pub const B43_NPHY_BPHY_CTL2_LUT_SHIFT: c_int = 0;
pub const B43_NPHY_BPHY_CTL2_MACDEL: c_uint = 0x7FE0 /* MAC delay */;
pub const B43_NPHY_BPHY_CTL2_MACDEL_SHIFT: c_int = 5;

pub const B43_NPHY_IQLOCAL_CMD_EN: c_uint = 0x8000;

pub const B43_NPHY_SAMP_CMD_STOP: c_uint = 0x0002 /* Stop */;

pub const B43_NPHY_BPHY_CTL3_SCALE: c_uint = 0x00FF /* Scale */;
pub const B43_NPHY_BPHY_CTL3_SCALE_SHIFT: c_int = 0;
pub const B43_NPHY_BPHY_CTL3_FSC: c_uint = 0xFF00 /* Frame start count value */;
pub const B43_NPHY_BPHY_CTL3_FSC_SHIFT: c_int = 8;

pub const B43_NPHY_MIMOCFG_GFMIX: c_uint = 0x0004 /* Greenfield or mixed mode */;
pub const B43_NPHY_MIMOCFG_AUTO: c_uint = 0x0100 /* Greenfield/mixed mode auto */;

pub const B43_NPHY_TSSIBIAS_BIAS: c_uint = 0x00FF /* Bias */;
pub const B43_NPHY_TSSIBIAS_BIAS_SHIFT: c_int = 0;
pub const B43_NPHY_TSSIBIAS_VAL: c_uint = 0xFF00 /* Value */;
pub const B43_NPHY_TSSIBIAS_VAL_SHIFT: c_int = 8;

pub const B43_NPHY_ESTPWR_PWR: c_uint = 0x00FF /* Estimated power */;
pub const B43_NPHY_ESTPWR_PWR_SHIFT: c_int = 0;
pub const B43_NPHY_ESTPWR_VALID: c_uint = 0x0100 /* Estimated power valid */;

pub const B43_NPHY_TSSI_MAXTXFDT_VAL: c_uint = 0x00FF /* max TX frame delay time */;
pub const B43_NPHY_TSSI_MAXTXFDT_VAL_SHIFT: c_int = 0;

pub const B43_NPHY_TSSI_MAXTDT_VAL: c_uint = 0x00FF /* max TSSI delay time */;
pub const B43_NPHY_TSSI_MAXTDT_VAL_SHIFT: c_int = 0;

pub const B43_NPHY_ITSSI_VAL: c_uint = 0x00FF /* Idle TSSI */;
pub const B43_NPHY_ITSSI_VAL_SHIFT: c_int = 0;

pub const B43_NPHY_TSSIMODE_EN: c_uint = 0x0001 /* TSSI enable */;
pub const B43_NPHY_TSSIMODE_PDEN: c_uint = 0x0002 /* Power det enable */;

pub const B43_NPHY_IQEST_CMD_START: c_uint = 0x0001 /* Start */;
pub const B43_NPHY_IQEST_CMD_MODE: c_uint = 0x0002 /* Mode */;

pub const B43_NPHY_IQEST_WT_VAL: c_uint = 0x00FF /* Wait time */;
pub const B43_NPHY_IQEST_WT_VAL_SHIFT: c_int = 0;

pub const B43_NPHY_PIL_DW_BPSK: c_uint = 0x000F /* BPSK */;
pub const B43_NPHY_PIL_DW_BPSK_SHIFT: c_int = 0;
pub const B43_NPHY_PIL_DW_QPSK: c_uint = 0x00F0 /* QPSK */;
pub const B43_NPHY_PIL_DW_QPSK_SHIFT: c_int = 4;
pub const B43_NPHY_PIL_DW_16QAM: c_uint = 0x0F00 /* 16-QAM */;
pub const B43_NPHY_PIL_DW_16QAM_SHIFT: c_int = 8;
pub const B43_NPHY_PIL_DW_64QAM: c_uint = 0xF000 /* 64-QAM */;
pub const B43_NPHY_PIL_DW_64QAM_SHIFT: c_int = 12;

pub const B43_NPHY_OVER_DGAIN_FDGV: c_uint = 0x0007 /* Force digital gain value */;
pub const B43_NPHY_OVER_DGAIN_FDGV_SHIFT: c_int = 0;
pub const B43_NPHY_OVER_DGAIN_FDGEN: c_uint = 0x0008 /* Force digital gain enable */;
pub const B43_NPHY_OVER_DGAIN_CCKDGECV: c_uint = 0xFF00 /* CCK digital gain enable count value */;
pub const B43_NPHY_OVER_DGAIN_CCKDGECV_SHIFT: c_int = 8;

pub const B43_NPHY_TXPCTL_CMD_INIT: c_uint = 0x007F /* Init */;
pub const B43_NPHY_TXPCTL_CMD_INIT_SHIFT: c_int = 0;
pub const B43_NPHY_TXPCTL_CMD_COEFF: c_uint = 0x2000 /* Power control coefficients */;
pub const B43_NPHY_TXPCTL_CMD_HWPCTLEN: c_uint = 0x4000 /* Hardware TX power control enable */;
pub const B43_NPHY_TXPCTL_CMD_PCTLEN: c_uint = 0x8000 /* TX power control enable */;

pub const B43_NPHY_TXPCTL_N_TSSID: c_uint = 0x00FF /* N TSSI delay */;
pub const B43_NPHY_TXPCTL_N_TSSID_SHIFT: c_int = 0;
pub const B43_NPHY_TXPCTL_N_NPTIL2: c_uint = 0x0700 /* N PT integer log2 */;
pub const B43_NPHY_TXPCTL_N_NPTIL2_SHIFT: c_int = 8;

pub const B43_NPHY_TXPCTL_ITSSI_0: c_uint = 0x003F /* Idle TSSI 0 */;
pub const B43_NPHY_TXPCTL_ITSSI_0_SHIFT: c_int = 0;
pub const B43_NPHY_TXPCTL_ITSSI_1: c_uint = 0x3F00 /* Idle TSSI 1 */;
pub const B43_NPHY_TXPCTL_ITSSI_1_SHIFT: c_int = 8;
pub const B43_NPHY_TXPCTL_ITSSI_BINF: c_uint = 0x8000 /* Raw TSSI offset bin format */;

pub const B43_NPHY_TXPCTL_TPWR_0: c_uint = 0x00FF /* Power 0 */;
pub const B43_NPHY_TXPCTL_TPWR_0_SHIFT: c_int = 0;
pub const B43_NPHY_TXPCTL_TPWR_1: c_uint = 0xFF00 /* Power 1 */;
pub const B43_NPHY_TXPCTL_TPWR_1_SHIFT: c_int = 8;

pub const B43_NPHY_TXPCTL_BIDX_0: c_uint = 0x007F /* uC base index 0 */;
pub const B43_NPHY_TXPCTL_BIDX_0_SHIFT: c_int = 0;
pub const B43_NPHY_TXPCTL_BIDX_1: c_uint = 0x7F00 /* uC base index 1 */;
pub const B43_NPHY_TXPCTL_BIDX_1_SHIFT: c_int = 8;
pub const B43_NPHY_TXPCTL_BIDX_LOAD: c_uint = 0x8000 /* Load base index */;

pub const B43_NPHY_TXPCTL_PIDX_0: c_uint = 0x007F /* uC power index 0 */;
pub const B43_NPHY_TXPCTL_PIDX_0_SHIFT: c_int = 0;
pub const B43_NPHY_TXPCTL_PIDX_1: c_uint = 0x7F00 /* uC power index 1 */;
pub const B43_NPHY_TXPCTL_PIDX_1_SHIFT: c_int = 8;

pub const B43_NPHY_TXPCTL_STAT_EST: c_uint = 0x00FF /* Estimated power */;
pub const B43_NPHY_TXPCTL_STAT_EST_SHIFT: c_int = 0;
pub const B43_NPHY_TXPCTL_STAT_BIDX: c_uint = 0x7F00 /* Base index */;
pub const B43_NPHY_TXPCTL_STAT_BIDX_SHIFT: c_int = 8;
pub const B43_NPHY_TXPCTL_STAT_ESTVALID: c_uint = 0x8000 /* Estimated power valid */;

pub const B43_NPHY_FINERX2_CGC_DECGC: c_uint = 0x0008 /* Decode gated clocks */;

pub const B43_NPHY_TXPCTL_INIT_PIDXI1: c_uint = 0x00FF /* Power index init 1 */;
pub const B43_NPHY_TXPCTL_INIT_PIDXI1_SHIFT: c_int = 0;

// REV3+

pub const B43_PHY_B_BBCFG_RSTCCA: c_uint = 0x4000 /* Reset CCA */;
pub const B43_PHY_B_BBCFG_RSTRX: c_uint = 0x8000 /* Reset RX */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_nphy_spur_avoid {
    B43_SPUR_AVOID_DISABLE,
    B43_SPUR_AVOID_AUTO,
    B43_SPUR_AVOID_FORCE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_chanspec {
    pub center_freq: u16,
    pub channel_type: nl80211_channel_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_n_iq_comp {
    pub a0: i16,
    pub b0: i16,
    pub a1: i16,
    pub b1: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_n_rssical_cache {
    pub rssical_radio_regs_2G: [u16; 2],
    pub rssical_phy_regs_2G: [u16; 12],
    pub rssical_radio_regs_5G: [u16; 2],
    pub rssical_phy_regs_5G: [u16; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_n_cal_cache {
    pub txcal_radio_regs_2G: [u16; 8],
    pub txcal_coeffs_2G: [u16; 8],
    pub rxcal_coeffs_2G: b43_phy_n_iq_comp,
    pub txcal_radio_regs_5G: [u16; 8],
    pub txcal_coeffs_5G: [u16; 8],
    pub rxcal_coeffs_5G: b43_phy_n_iq_comp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_n_txpwrindex {
    pub index: i8,
    pub index_internal: i8,
    pub index_internal_save: i8,
    pub AfectrlOverride: u16,
    pub AfeCtrlDacGain: u16,
    pub rad_gain: u16,
    pub bbmult: u8,
    pub iqcomp_a: u16,
    pub iqcomp_b: u16,
    pub locomp: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_n_pwr_ctl_info {
    pub idle_tssi_2g: u8,
    pub idle_tssi_5g: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_n {
    pub antsel_type: u8,
    pub cal_orig_pwr_idx: [u8; 2],
    pub measure_hold: u8,
    pub phyrxchain: u8,
    pub hw_phyrxchain: u8,
    pub hw_phytxchain: u8,
    pub perical: u8,
    pub deaf_count: u32,
    pub rxcalparams: u32,
    pub hang_avoid: bool,
    pub mute: bool,
    pub papd_epsilon_offset: [u16; 2],
    pub preamble_override: i32,
    pub bb_mult_save: u32,
    pub gain_boost: bool,
    pub elna_gain_config: bool,
    pub band5g_pwrgain: bool,
    pub use_int_tx_iq_lo_cal: bool,
    pub lpf_bw_overrode_for_sample_play: bool,
    pub mphase_cal_phase_id: u8,
    pub mphase_txcal_cmdidx: u16,
    pub mphase_txcal_numcmds: u16,
    pub mphase_txcal_bestcoeffs: [u16; 11],
    pub txpwrctrl: bool,
    pub pwg_gain_5ghz: bool,
    pub tx_pwr_idx: [u8; 2],
    pub tx_power_offset: [i8; 101],
    pub adj_pwr_tbl: [u16; 84],
    pub txcal_bbmult: u16,
    pub txiqlocal_bestc: [u16; 11],
    pub txiqlocal_coeffsvalid: bool,
    pub txpwrindex: [b43_phy_n_txpwrindex; 2],
    pub pwr_ctl_info: [b43_phy_n_pwr_ctl_info; 2],
    pub txiqlocal_chanspec: b43_chanspec,
    pub tx_pwr_max_ppr: b43_ppr,
    pub tx_pwr_last_recalc_freq: u16,
    pub tx_pwr_last_recalc_limit: c_int,
    pub txrx_chain: u8,
    pub tx_rx_cal_phy_saveregs: [u16; 11],
    pub tx_rx_cal_radio_saveregs: [u16; 22],
    pub rfctrl_intc1_save: u16,
    pub rfctrl_intc2_save: u16,
    pub classifier_state: u16,
    pub clip_state: [u16; 2],
    pub spur_avoid: b43_nphy_spur_avoid,
    pub aband_spurwar_en: bool,
    pub gband_spurwar_en: bool,
    pub ipa2g_on: bool,
    pub iqcal_chanspec_2G: b43_chanspec,
    pub rssical_chanspec_2G: b43_chanspec,
    pub ipa5g_on: bool,
    pub iqcal_chanspec_5G: b43_chanspec,
    pub rssical_chanspec_5G: b43_chanspec,
    pub rssical_cache: b43_phy_n_rssical_cache,
    pub cal_cache: b43_phy_n_cal_cache,
    pub crsminpwr_adjusted: bool,
    pub noisevars_adjusted: bool,
}
