//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/radio_2055.h
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

pub const B2055_GEN_SPARE: c_uint = 0x00 /* GEN spare */;
pub const B2055_SP_PINPD: c_uint = 0x02 /* SP PIN PD */;
pub const B2055_C1_SP_RSSI: c_uint = 0x03 /* SP RSSI Core 1 */;
pub const B2055_C1_SP_PDMISC: c_uint = 0x04 /* SP PD MISC Core 1 */;
pub const B2055_C2_SP_RSSI: c_uint = 0x05 /* SP RSSI Core 2 */;
pub const B2055_C2_SP_PDMISC: c_uint = 0x06 /* SP PD MISC Core 2 */;
pub const B2055_C1_SP_RXGC1: c_uint = 0x07 /* SP RX GC1 Core 1 */;
pub const B2055_C1_SP_RXGC2: c_uint = 0x08 /* SP RX GC2 Core 1 */;
pub const B2055_C2_SP_RXGC1: c_uint = 0x09 /* SP RX GC1 Core 2 */;
pub const B2055_C2_SP_RXGC2: c_uint = 0x0A /* SP RX GC2 Core 2 */;
pub const B2055_C1_SP_LPFBWSEL: c_uint = 0x0B /* SP LPF BW select Core 1 */;
pub const B2055_C2_SP_LPFBWSEL: c_uint = 0x0C /* SP LPF BW select Core 2 */;
pub const B2055_C1_SP_TXGC1: c_uint = 0x0D /* SP TX GC1 Core 1 */;
pub const B2055_C1_SP_TXGC2: c_uint = 0x0E /* SP TX GC2 Core 1 */;
pub const B2055_C2_SP_TXGC1: c_uint = 0x0F /* SP TX GC1 Core 2 */;
pub const B2055_C2_SP_TXGC2: c_uint = 0x10 /* SP TX GC2 Core 2 */;
pub const B2055_MASTER1: c_uint = 0x11 /* Master control 1 */;
pub const B2055_MASTER2: c_uint = 0x12 /* Master control 2 */;
pub const B2055_PD_LGEN: c_uint = 0x13 /* PD LGEN */;
pub const B2055_PD_PLLTS: c_uint = 0x14 /* PD PLL TS */;
pub const B2055_C1_PD_LGBUF: c_uint = 0x15 /* PD Core 1 LGBUF */;
pub const B2055_C1_PD_TX: c_uint = 0x16 /* PD Core 1 TX */;
pub const B2055_C1_PD_RXTX: c_uint = 0x17 /* PD Core 1 RXTX */;
pub const B2055_C1_PD_RSSIMISC: c_uint = 0x18 /* PD Core 1 RSSI MISC */;
pub const B2055_C2_PD_LGBUF: c_uint = 0x19 /* PD Core 2 LGBUF */;
pub const B2055_C2_PD_TX: c_uint = 0x1A /* PD Core 2 TX */;
pub const B2055_C2_PD_RXTX: c_uint = 0x1B /* PD Core 2 RXTX */;
pub const B2055_C2_PD_RSSIMISC: c_uint = 0x1C /* PD Core 2 RSSI MISC */;
pub const B2055_PWRDET_LGEN: c_uint = 0x1D /* PWRDET LGEN */;
pub const B2055_C1_PWRDET_LGBUF: c_uint = 0x1E /* PWRDET LGBUF Core 1 */;
pub const B2055_C1_PWRDET_RXTX: c_uint = 0x1F /* PWRDET RXTX Core 1 */;
pub const B2055_C2_PWRDET_LGBUF: c_uint = 0x20 /* PWRDET LGBUF Core 2 */;
pub const B2055_C2_PWRDET_RXTX: c_uint = 0x21 /* PWRDET RXTX Core 2 */;
pub const B2055_RRCCAL_CS: c_uint = 0x22 /* RRCCAL Control spare */;
pub const B2055_RRCCAL_NOPTSEL: c_uint = 0x23 /* RRCCAL N OPT SEL */;
pub const B2055_CAL_MISC: c_uint = 0x24 /* CAL MISC */;
pub const B2055_CAL_COUT: c_uint = 0x25 /* CAL Counter out */;
pub const B2055_CAL_COUT2: c_uint = 0x26 /* CAL Counter out 2 */;
pub const B2055_CAL_CVARCTL: c_uint = 0x27 /* CAL CVAR Control */;
pub const B2055_CAL_RVARCTL: c_uint = 0x28 /* CAL RVAR Control */;
pub const B2055_CAL_LPOCTL: c_uint = 0x29 /* CAL LPO Control */;
pub const B2055_CAL_TS: c_uint = 0x2A /* CAL TS */;
pub const B2055_CAL_RCCALRTS: c_uint = 0x2B /* CAL RCCAL READ TS */;
pub const B2055_CAL_RCALRTS: c_uint = 0x2C /* CAL RCAL READ TS */;
pub const B2055_PADDRV: c_uint = 0x2D /* PAD driver */;
pub const B2055_XOCTL1: c_uint = 0x2E /* XO Control 1 */;
pub const B2055_XOCTL2: c_uint = 0x2F /* XO Control 2 */;
pub const B2055_XOREGUL: c_uint = 0x30 /* XO Regulator */;
pub const B2055_XOMISC: c_uint = 0x31 /* XO misc */;
pub const B2055_PLL_LFC1: c_uint = 0x32 /* PLL LF C1 */;
pub const B2055_PLL_CALVTH: c_uint = 0x33 /* PLL CAL VTH */;
pub const B2055_PLL_LFC2: c_uint = 0x34 /* PLL LF C2 */;
pub const B2055_PLL_REF: c_uint = 0x35 /* PLL reference */;
pub const B2055_PLL_LFR1: c_uint = 0x36 /* PLL LF R1 */;
pub const B2055_PLL_PFDCP: c_uint = 0x37 /* PLL PFD CP */;
pub const B2055_PLL_IDAC_CPOPAMP: c_uint = 0x38 /* PLL IDAC CPOPAMP */;
pub const B2055_PLL_CPREG: c_uint = 0x39 /* PLL CP Regulator */;
pub const B2055_PLL_RCAL: c_uint = 0x3A /* PLL RCAL */;
pub const B2055_RF_PLLMOD0: c_uint = 0x3B /* RF PLL MOD0 */;
pub const B2055_RF_PLLMOD1: c_uint = 0x3C /* RF PLL MOD1 */;
pub const B2055_RF_MMDIDAC1: c_uint = 0x3D /* RF MMD IDAC 1 */;
pub const B2055_RF_MMDIDAC0: c_uint = 0x3E /* RF MMD IDAC 0 */;
pub const B2055_RF_MMDSP: c_uint = 0x3F /* RF MMD spare */;
pub const B2055_VCO_CAL1: c_uint = 0x40 /* VCO cal 1 */;
pub const B2055_VCO_CAL2: c_uint = 0x41 /* VCO cal 2 */;
pub const B2055_VCO_CAL3: c_uint = 0x42 /* VCO cal 3 */;
pub const B2055_VCO_CAL4: c_uint = 0x43 /* VCO cal 4 */;
pub const B2055_VCO_CAL5: c_uint = 0x44 /* VCO cal 5 */;
pub const B2055_VCO_CAL6: c_uint = 0x45 /* VCO cal 6 */;
pub const B2055_VCO_CAL7: c_uint = 0x46 /* VCO cal 7 */;
pub const B2055_VCO_CAL8: c_uint = 0x47 /* VCO cal 8 */;
pub const B2055_VCO_CAL9: c_uint = 0x48 /* VCO cal 9 */;
pub const B2055_VCO_CAL10: c_uint = 0x49 /* VCO cal 10 */;
pub const B2055_VCO_CAL11: c_uint = 0x4A /* VCO cal 11 */;
pub const B2055_VCO_CAL12: c_uint = 0x4B /* VCO cal 12 */;
pub const B2055_VCO_CAL13: c_uint = 0x4C /* VCO cal 13 */;
pub const B2055_VCO_CAL14: c_uint = 0x4D /* VCO cal 14 */;
pub const B2055_VCO_CAL15: c_uint = 0x4E /* VCO cal 15 */;
pub const B2055_VCO_CAL16: c_uint = 0x4F /* VCO cal 16 */;
pub const B2055_VCO_KVCO: c_uint = 0x50 /* VCO KVCO */;
pub const B2055_VCO_CAPTAIL: c_uint = 0x51 /* VCO CAP TAIL */;
pub const B2055_VCO_IDACVCO: c_uint = 0x52 /* VCO IDAC VCO */;
pub const B2055_VCO_REG: c_uint = 0x53 /* VCO Regulator */;
pub const B2055_PLL_RFVTH: c_uint = 0x54 /* PLL RF VTH */;
pub const B2055_LGBUF_CENBUF: c_uint = 0x55 /* LGBUF CEN BUF */;
pub const B2055_LGEN_TUNE1: c_uint = 0x56 /* LGEN tune 1 */;
pub const B2055_LGEN_TUNE2: c_uint = 0x57 /* LGEN tune 2 */;
pub const B2055_LGEN_IDAC1: c_uint = 0x58 /* LGEN IDAC 1 */;
pub const B2055_LGEN_IDAC2: c_uint = 0x59 /* LGEN IDAC 2 */;
pub const B2055_LGEN_BIASC: c_uint = 0x5A /* LGEN BIAS counter */;
pub const B2055_LGEN_BIASIDAC: c_uint = 0x5B /* LGEN BIAS IDAC */;
pub const B2055_LGEN_RCAL: c_uint = 0x5C /* LGEN RCAL */;
pub const B2055_LGEN_DIV: c_uint = 0x5D /* LGEN div */;
pub const B2055_LGEN_SPARE2: c_uint = 0x5E /* LGEN spare 2 */;
pub const B2055_C1_LGBUF_ATUNE: c_uint = 0x5F /* Core 1 LGBUF A tune */;
pub const B2055_C1_LGBUF_GTUNE: c_uint = 0x60 /* Core 1 LGBUF G tune */;
pub const B2055_C1_LGBUF_DIV: c_uint = 0x61 /* Core 1 LGBUF div */;
pub const B2055_C1_LGBUF_AIDAC: c_uint = 0x62 /* Core 1 LGBUF A IDAC */;
pub const B2055_C1_LGBUF_GIDAC: c_uint = 0x63 /* Core 1 LGBUF G IDAC */;
pub const B2055_C1_LGBUF_IDACFO: c_uint = 0x64 /* Core 1 LGBUF IDAC filter override */;
pub const B2055_C1_LGBUF_SPARE: c_uint = 0x65 /* Core 1 LGBUF spare */;
pub const B2055_C1_RX_RFSPC1: c_uint = 0x66 /* Core 1 RX RF SPC1 */;
pub const B2055_C1_RX_RFR1: c_uint = 0x67 /* Core 1 RX RF reg 1 */;
pub const B2055_C1_RX_RFR2: c_uint = 0x68 /* Core 1 RX RF reg 2 */;
pub const B2055_C1_RX_RFRCAL: c_uint = 0x69 /* Core 1 RX RF RCAL */;
pub const B2055_C1_RX_BB_BLCMP: c_uint = 0x6A /* Core 1 RX Baseband BUFI LPF CMP */;
pub const B2055_C1_RX_BB_LPF: c_uint = 0x6B /* Core 1 RX Baseband LPF */;
pub const B2055_C1_RX_BB_MIDACHP: c_uint = 0x6C /* Core 1 RX Baseband MIDAC High-pass */;
pub const B2055_C1_RX_BB_VGA1IDAC: c_uint = 0x6D /* Core 1 RX Baseband VGA1 IDAC */;
pub const B2055_C1_RX_BB_VGA2IDAC: c_uint = 0x6E /* Core 1 RX Baseband VGA2 IDAC */;
pub const B2055_C1_RX_BB_VGA3IDAC: c_uint = 0x6F /* Core 1 RX Baseband VGA3 IDAC */;
pub const B2055_C1_RX_BB_BUFOCTL: c_uint = 0x70 /* Core 1 RX Baseband BUFO Control */;
pub const B2055_C1_RX_BB_RCCALCTL: c_uint = 0x71 /* Core 1 RX Baseband RCCAL Control */;
pub const B2055_C1_RX_BB_RSSICTL1: c_uint = 0x72 /* Core 1 RX Baseband RSSI Control 1 */;
pub const B2055_C1_RX_BB_RSSICTL2: c_uint = 0x73 /* Core 1 RX Baseband RSSI Control 2 */;
pub const B2055_C1_RX_BB_RSSICTL3: c_uint = 0x74 /* Core 1 RX Baseband RSSI Control 3 */;
pub const B2055_C1_RX_BB_RSSICTL4: c_uint = 0x75 /* Core 1 RX Baseband RSSI Control 4 */;
pub const B2055_C1_RX_BB_RSSICTL5: c_uint = 0x76 /* Core 1 RX Baseband RSSI Control 5 */;
pub const B2055_C1_RX_BB_REG: c_uint = 0x77 /* Core 1 RX Baseband Regulator */;
pub const B2055_C1_RX_BB_SPARE1: c_uint = 0x78 /* Core 1 RX Baseband spare 1 */;
pub const B2055_C1_RX_TXBBRCAL: c_uint = 0x79 /* Core 1 RX TX BB RCAL */;
pub const B2055_C1_TX_RF_SPGA: c_uint = 0x7A /* Core 1 TX RF SGM PGA */;
pub const B2055_C1_TX_RF_SPAD: c_uint = 0x7B /* Core 1 TX RF SGM PAD */;
pub const B2055_C1_TX_RF_CNTPGA1: c_uint = 0x7C /* Core 1 TX RF counter PGA 1 */;
pub const B2055_C1_TX_RF_CNTPAD1: c_uint = 0x7D /* Core 1 TX RF counter PAD 1 */;
pub const B2055_C1_TX_RF_PGAIDAC: c_uint = 0x7E /* Core 1 TX RF PGA IDAC */;
pub const B2055_C1_TX_PGAPADTN: c_uint = 0x7F /* Core 1 TX PGA PAD TN */;
pub const B2055_C1_TX_PADIDAC1: c_uint = 0x80 /* Core 1 TX PAD IDAC 1 */;
pub const B2055_C1_TX_PADIDAC2: c_uint = 0x81 /* Core 1 TX PAD IDAC 2 */;
pub const B2055_C1_TX_MXBGTRIM: c_uint = 0x82 /* Core 1 TX MX B/G TRIM */;
pub const B2055_C1_TX_RF_RCAL: c_uint = 0x83 /* Core 1 TX RF RCAL */;
pub const B2055_C1_TX_RF_PADTSSI1: c_uint = 0x84 /* Core 1 TX RF PAD TSSI1 */;
pub const B2055_C1_TX_RF_PADTSSI2: c_uint = 0x85 /* Core 1 TX RF PAD TSSI2 */;
pub const B2055_C1_TX_RF_SPARE: c_uint = 0x86 /* Core 1 TX RF spare */;
pub const B2055_C1_TX_RF_IQCAL1: c_uint = 0x87 /* Core 1 TX RF I/Q CAL 1 */;
pub const B2055_C1_TX_RF_IQCAL2: c_uint = 0x88 /* Core 1 TX RF I/Q CAL 2 */;
pub const B2055_C1_TXBB_RCCAL: c_uint = 0x89 /* Core 1 TXBB RC CAL Control */;
pub const B2055_C1_TXBB_LPF1: c_uint = 0x8A /* Core 1 TXBB LPF 1 */;
pub const B2055_C1_TX_VOSCNCL: c_uint = 0x8B /* Core 1 TX VOS CNCL */;
pub const B2055_C1_TX_LPF_MXGMIDAC: c_uint = 0x8C /* Core 1 TX LPF MXGM IDAC */;
pub const B2055_C1_TX_BB_MXGM: c_uint = 0x8D /* Core 1 TX BB MXGM */;
pub const B2055_C2_LGBUF_ATUNE: c_uint = 0x8E /* Core 2 LGBUF A tune */;
pub const B2055_C2_LGBUF_GTUNE: c_uint = 0x8F /* Core 2 LGBUF G tune */;
pub const B2055_C2_LGBUF_DIV: c_uint = 0x90 /* Core 2 LGBUF div */;
pub const B2055_C2_LGBUF_AIDAC: c_uint = 0x91 /* Core 2 LGBUF A IDAC */;
pub const B2055_C2_LGBUF_GIDAC: c_uint = 0x92 /* Core 2 LGBUF G IDAC */;
pub const B2055_C2_LGBUF_IDACFO: c_uint = 0x93 /* Core 2 LGBUF IDAC filter override */;
pub const B2055_C2_LGBUF_SPARE: c_uint = 0x94 /* Core 2 LGBUF spare */;
pub const B2055_C2_RX_RFSPC1: c_uint = 0x95 /* Core 2 RX RF SPC1 */;
pub const B2055_C2_RX_RFR1: c_uint = 0x96 /* Core 2 RX RF reg 1 */;
pub const B2055_C2_RX_RFR2: c_uint = 0x97 /* Core 2 RX RF reg 2 */;
pub const B2055_C2_RX_RFRCAL: c_uint = 0x98 /* Core 2 RX RF RCAL */;
pub const B2055_C2_RX_BB_BLCMP: c_uint = 0x99 /* Core 2 RX Baseband BUFI LPF CMP */;
pub const B2055_C2_RX_BB_LPF: c_uint = 0x9A /* Core 2 RX Baseband LPF */;
pub const B2055_C2_RX_BB_MIDACHP: c_uint = 0x9B /* Core 2 RX Baseband MIDAC High-pass */;
pub const B2055_C2_RX_BB_VGA1IDAC: c_uint = 0x9C /* Core 2 RX Baseband VGA1 IDAC */;
pub const B2055_C2_RX_BB_VGA2IDAC: c_uint = 0x9D /* Core 2 RX Baseband VGA2 IDAC */;
pub const B2055_C2_RX_BB_VGA3IDAC: c_uint = 0x9E /* Core 2 RX Baseband VGA3 IDAC */;
pub const B2055_C2_RX_BB_BUFOCTL: c_uint = 0x9F /* Core 2 RX Baseband BUFO Control */;
pub const B2055_C2_RX_BB_RCCALCTL: c_uint = 0xA0 /* Core 2 RX Baseband RCCAL Control */;
pub const B2055_C2_RX_BB_RSSICTL1: c_uint = 0xA1 /* Core 2 RX Baseband RSSI Control 1 */;
pub const B2055_C2_RX_BB_RSSICTL2: c_uint = 0xA2 /* Core 2 RX Baseband RSSI Control 2 */;
pub const B2055_C2_RX_BB_RSSICTL3: c_uint = 0xA3 /* Core 2 RX Baseband RSSI Control 3 */;
pub const B2055_C2_RX_BB_RSSICTL4: c_uint = 0xA4 /* Core 2 RX Baseband RSSI Control 4 */;
pub const B2055_C2_RX_BB_RSSICTL5: c_uint = 0xA5 /* Core 2 RX Baseband RSSI Control 5 */;
pub const B2055_C2_RX_BB_REG: c_uint = 0xA6 /* Core 2 RX Baseband Regulator */;
pub const B2055_C2_RX_BB_SPARE1: c_uint = 0xA7 /* Core 2 RX Baseband spare 1 */;
pub const B2055_C2_RX_TXBBRCAL: c_uint = 0xA8 /* Core 2 RX TX BB RCAL */;
pub const B2055_C2_TX_RF_SPGA: c_uint = 0xA9 /* Core 2 TX RF SGM PGA */;
pub const B2055_C2_TX_RF_SPAD: c_uint = 0xAA /* Core 2 TX RF SGM PAD */;
pub const B2055_C2_TX_RF_CNTPGA1: c_uint = 0xAB /* Core 2 TX RF counter PGA 1 */;
pub const B2055_C2_TX_RF_CNTPAD1: c_uint = 0xAC /* Core 2 TX RF counter PAD 1 */;
pub const B2055_C2_TX_RF_PGAIDAC: c_uint = 0xAD /* Core 2 TX RF PGA IDAC */;
pub const B2055_C2_TX_PGAPADTN: c_uint = 0xAE /* Core 2 TX PGA PAD TN */;
pub const B2055_C2_TX_PADIDAC1: c_uint = 0xAF /* Core 2 TX PAD IDAC 1 */;
pub const B2055_C2_TX_PADIDAC2: c_uint = 0xB0 /* Core 2 TX PAD IDAC 2 */;
pub const B2055_C2_TX_MXBGTRIM: c_uint = 0xB1 /* Core 2 TX MX B/G TRIM */;
pub const B2055_C2_TX_RF_RCAL: c_uint = 0xB2 /* Core 2 TX RF RCAL */;
pub const B2055_C2_TX_RF_PADTSSI1: c_uint = 0xB3 /* Core 2 TX RF PAD TSSI1 */;
pub const B2055_C2_TX_RF_PADTSSI2: c_uint = 0xB4 /* Core 2 TX RF PAD TSSI2 */;
pub const B2055_C2_TX_RF_SPARE: c_uint = 0xB5 /* Core 2 TX RF spare */;
pub const B2055_C2_TX_RF_IQCAL1: c_uint = 0xB6 /* Core 2 TX RF I/Q CAL 1 */;
pub const B2055_C2_TX_RF_IQCAL2: c_uint = 0xB7 /* Core 2 TX RF I/Q CAL 2 */;
pub const B2055_C2_TXBB_RCCAL: c_uint = 0xB8 /* Core 2 TXBB RC CAL Control */;
pub const B2055_C2_TXBB_LPF1: c_uint = 0xB9 /* Core 2 TXBB LPF 1 */;
pub const B2055_C2_TX_VOSCNCL: c_uint = 0xBA /* Core 2 TX VOS CNCL */;
pub const B2055_C2_TX_LPF_MXGMIDAC: c_uint = 0xBB /* Core 2 TX LPF MXGM IDAC */;
pub const B2055_C2_TX_BB_MXGM: c_uint = 0xBC /* Core 2 TX BB MXGM */;
pub const B2055_PRG_GCHP21: c_uint = 0xBD /* PRG GC HPVGA23 21 */;
pub const B2055_PRG_GCHP22: c_uint = 0xBE /* PRG GC HPVGA23 22 */;
pub const B2055_PRG_GCHP23: c_uint = 0xBF /* PRG GC HPVGA23 23 */;
pub const B2055_PRG_GCHP24: c_uint = 0xC0 /* PRG GC HPVGA23 24 */;
pub const B2055_PRG_GCHP25: c_uint = 0xC1 /* PRG GC HPVGA23 25 */;
pub const B2055_PRG_GCHP26: c_uint = 0xC2 /* PRG GC HPVGA23 26 */;
pub const B2055_PRG_GCHP27: c_uint = 0xC3 /* PRG GC HPVGA23 27 */;
pub const B2055_PRG_GCHP28: c_uint = 0xC4 /* PRG GC HPVGA23 28 */;
pub const B2055_PRG_GCHP29: c_uint = 0xC5 /* PRG GC HPVGA23 29 */;
pub const B2055_PRG_GCHP30: c_uint = 0xC6 /* PRG GC HPVGA23 30 */;
pub const B2055_C1_LNA_GAINBST: c_uint = 0xCD /* Core 1 LNA GAINBST */;
pub const B2055_C1_B0NB_RSSIVCM: c_uint = 0xD2 /* Core 1 B0 narrow-band RSSI VCM */;
pub const B2055_C1_GENSPARE2: c_uint = 0xD6 /* Core 1 GEN spare 2 */;
pub const B2055_C2_LNA_GAINBST: c_uint = 0xD9 /* Core 2 LNA GAINBST */;
pub const B2055_C2_B0NB_RSSIVCM: c_uint = 0xDE /* Core 2 B0 narrow-band RSSI VCM */;
pub const B2055_C2_GENSPARE2: c_uint = 0xE2 /* Core 2 GEN spare 2 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_nphy_channeltab_entry_rev2 {
// The channel number
    pub channel: u8,
// The channel frequency in MHz
    pub freq: u16,
// An unknown value
    pub unk2: u16,
// Radio register values on channelswitch
    pub radio_pll_ref: u8,
    pub radio_rf_pllmod0: u8,
    pub radio_rf_pllmod1: u8,
    pub radio_vco_captail: u8,
    pub radio_vco_cal1: u8,
    pub radio_vco_cal2: u8,
    pub radio_pll_lfc1: u8,
    pub radio_pll_lfr1: u8,
    pub radio_pll_lfc2: u8,
    pub radio_lgbuf_cenbuf: u8,
    pub radio_lgen_tune1: u8,
    pub radio_lgen_tune2: u8,
    pub radio_c1_lgbuf_atune: u8,
    pub radio_c1_lgbuf_gtune: u8,
    pub radio_c1_rx_rfr1: u8,
    pub radio_c1_tx_pgapadtn: u8,
    pub radio_c1_tx_mxbgtrim: u8,
    pub radio_c2_lgbuf_atune: u8,
    pub radio_c2_lgbuf_gtune: u8,
    pub radio_c2_rx_rfr1: u8,
    pub radio_c2_tx_pgapadtn: u8,
    pub radio_c2_tx_mxbgtrim: u8,
// PHY register values on channelswitch
    pub phy_regs: b43_phy_n_sfo_cfg,
}

// Upload the default register value table.
// If "ghz5" is true, we upload the 5Ghz table. Otherwise the 2.4Ghz
// table is uploaded. If "ignore_uploadflag" is true, we upload any value
// and ignore the "UPLOAD" flag.
// Get the NPHY Channel Switch Table entry for a channel.
// Returns NULL on failure to find an entry.
