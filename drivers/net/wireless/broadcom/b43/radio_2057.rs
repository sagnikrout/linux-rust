//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/radio_2057.h
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

pub const R2057_DACBUF_VINCM_CORE0: c_uint = 0x000;
pub const R2057_IDCODE: c_uint = 0x001;
pub const R2057_RCCAL_MASTER: c_uint = 0x002;
pub const R2057_RCCAL_CAP_SIZE: c_uint = 0x003;
pub const R2057_RCAL_CONFIG: c_uint = 0x004;
pub const R2057_GPAIO_CONFIG: c_uint = 0x005;
pub const R2057_GPAIO_SEL1: c_uint = 0x006;
pub const R2057_GPAIO_SEL0: c_uint = 0x007;
pub const R2057_CLPO_CONFIG: c_uint = 0x008;
pub const R2057_BANDGAP_CONFIG: c_uint = 0x009;
pub const R2057_BANDGAP_RCAL_TRIM: c_uint = 0x00a;
pub const R2057_AFEREG_CONFIG: c_uint = 0x00b;
pub const R2057_TEMPSENSE_CONFIG: c_uint = 0x00c;
pub const R2057_XTAL_CONFIG1: c_uint = 0x00d;
pub const R2057_XTAL_ICORE_SIZE: c_uint = 0x00e;
pub const R2057_XTAL_BUF_SIZE: c_uint = 0x00f;
pub const R2057_XTAL_PULLCAP_SIZE: c_uint = 0x010;
pub const R2057_RFPLL_MASTER: c_uint = 0x011;
pub const R2057_VCOMONITOR_VTH_L: c_uint = 0x012;
pub const R2057_VCOMONITOR_VTH_H: c_uint = 0x013;
pub const R2057_VCOCAL_BIASRESET_RFPLLREG_VOUT: c_uint = 0x014;
pub const R2057_VCO_VARCSIZE_IDAC: c_uint = 0x015;
pub const R2057_VCOCAL_COUNTVAL0: c_uint = 0x016;
pub const R2057_VCOCAL_COUNTVAL1: c_uint = 0x017;
pub const R2057_VCOCAL_INTCLK_COUNT: c_uint = 0x018;
pub const R2057_VCOCAL_MASTER: c_uint = 0x019;
pub const R2057_VCOCAL_NUMCAPCHANGE: c_uint = 0x01a;
pub const R2057_VCOCAL_WINSIZE: c_uint = 0x01b;
pub const R2057_VCOCAL_DELAY_AFTER_REFRESH: c_uint = 0x01c;
pub const R2057_VCOCAL_DELAY_AFTER_CLOSELOOP: c_uint = 0x01d;
pub const R2057_VCOCAL_DELAY_AFTER_OPENLOOP: c_uint = 0x01e;
pub const R2057_VCOCAL_DELAY_BEFORE_OPENLOOP: c_uint = 0x01f;
pub const R2057_VCO_FORCECAPEN_FORCECAP1: c_uint = 0x020;
pub const R2057_VCO_FORCECAP0: c_uint = 0x021;
pub const R2057_RFPLL_REFMASTER_SPAREXTALSIZE: c_uint = 0x022;
pub const R2057_RFPLL_PFD_RESET_PW: c_uint = 0x023;
pub const R2057_RFPLL_LOOPFILTER_R2: c_uint = 0x024;
pub const R2057_RFPLL_LOOPFILTER_R1: c_uint = 0x025;
pub const R2057_RFPLL_LOOPFILTER_C3: c_uint = 0x026;
pub const R2057_RFPLL_LOOPFILTER_C2: c_uint = 0x027;
pub const R2057_RFPLL_LOOPFILTER_C1: c_uint = 0x028;
pub const R2057_CP_KPD_IDAC: c_uint = 0x029;
pub const R2057_RFPLL_IDACS: c_uint = 0x02a;
pub const R2057_RFPLL_MISC_EN: c_uint = 0x02b;
pub const R2057_RFPLL_MMD0: c_uint = 0x02c;
pub const R2057_RFPLL_MMD1: c_uint = 0x02d;
pub const R2057_RFPLL_MISC_CAL_RESETN: c_uint = 0x02e;
pub const R2057_JTAGXTAL_SIZE_CPBIAS_FILTRES: c_uint = 0x02f;
pub const R2057_VCO_ALCREF_BBPLLXTAL_SIZE: c_uint = 0x030;
pub const R2057_VCOCAL_READCAP0: c_uint = 0x031;
pub const R2057_VCOCAL_READCAP1: c_uint = 0x032;
pub const R2057_VCOCAL_STATUS: c_uint = 0x033;
pub const R2057_LOGEN_PUS: c_uint = 0x034;
pub const R2057_LOGEN_PTAT_RESETS: c_uint = 0x035;
pub const R2057_VCOBUF_IDACS: c_uint = 0x036;
pub const R2057_VCOBUF_TUNE: c_uint = 0x037;
pub const R2057_CMOSBUF_TX2GQ_IDACS: c_uint = 0x038;
pub const R2057_CMOSBUF_TX2GI_IDACS: c_uint = 0x039;
pub const R2057_CMOSBUF_TX5GQ_IDACS: c_uint = 0x03a;
pub const R2057_CMOSBUF_TX5GI_IDACS: c_uint = 0x03b;
pub const R2057_CMOSBUF_RX2GQ_IDACS: c_uint = 0x03c;
pub const R2057_CMOSBUF_RX2GI_IDACS: c_uint = 0x03d;
pub const R2057_CMOSBUF_RX5GQ_IDACS: c_uint = 0x03e;
pub const R2057_CMOSBUF_RX5GI_IDACS: c_uint = 0x03f;
pub const R2057_LOGEN_MX2G_IDACS: c_uint = 0x040;
pub const R2057_LOGEN_MX2G_TUNE: c_uint = 0x041;
pub const R2057_LOGEN_MX5G_IDACS: c_uint = 0x042;
pub const R2057_LOGEN_MX5G_TUNE: c_uint = 0x043;
pub const R2057_LOGEN_MX5G_RCCR: c_uint = 0x044;
pub const R2057_LOGEN_INDBUF2G_IDAC: c_uint = 0x045;
pub const R2057_LOGEN_INDBUF2G_IBOOST: c_uint = 0x046;
pub const R2057_LOGEN_INDBUF2G_TUNE: c_uint = 0x047;
pub const R2057_LOGEN_INDBUF5G_IDAC: c_uint = 0x048;
pub const R2057_LOGEN_INDBUF5G_IBOOST: c_uint = 0x049;
pub const R2057_LOGEN_INDBUF5G_TUNE: c_uint = 0x04a;
pub const R2057_CMOSBUF_TX_RCCR: c_uint = 0x04b;
pub const R2057_CMOSBUF_RX_RCCR: c_uint = 0x04c;
pub const R2057_LOGEN_SEL_PKDET: c_uint = 0x04d;
pub const R2057_CMOSBUF_SHAREIQ_PTAT: c_uint = 0x04e;
// MISC core 0
pub const R2057_RXTXBIAS_CONFIG_CORE0: c_uint = 0x04f;
pub const R2057_TXGM_TXRF_PUS_CORE0: c_uint = 0x050;
pub const R2057_TXGM_IDAC_BLEED_CORE0: c_uint = 0x051;
pub const R2057_TXGM_GAIN_CORE0: c_uint = 0x056;
pub const R2057_TXGM2G_PKDET_PUS_CORE0: c_uint = 0x057;
pub const R2057_PAD2G_PTATS_CORE0: c_uint = 0x058;
pub const R2057_PAD2G_IDACS_CORE0: c_uint = 0x059;
pub const R2057_PAD2G_BOOST_PU_CORE0: c_uint = 0x05a;
pub const R2057_PAD2G_CASCV_GAIN_CORE0: c_uint = 0x05b;
pub const R2057_TXMIX2G_TUNE_BOOST_PU_CORE0: c_uint = 0x05c;
pub const R2057_TXMIX2G_LODC_CORE0: c_uint = 0x05d;
pub const R2057_PAD2G_TUNE_PUS_CORE0: c_uint = 0x05e;
pub const R2057_IPA2G_GAIN_CORE0: c_uint = 0x05f;
pub const R2057_TSSI2G_SPARE1_CORE0: c_uint = 0x060;
pub const R2057_TSSI2G_SPARE2_CORE0: c_uint = 0x061;
pub const R2057_IPA2G_TUNEV_CASCV_PTAT_CORE0: c_uint = 0x062;
pub const R2057_IPA2G_IMAIN_CORE0: c_uint = 0x063;
pub const R2057_IPA2G_CASCONV_CORE0: c_uint = 0x064;
pub const R2057_IPA2G_CASCOFFV_CORE0: c_uint = 0x065;
pub const R2057_IPA2G_BIAS_FILTER_CORE0: c_uint = 0x066;
pub const R2057_TX5G_PKDET_CORE0: c_uint = 0x069;
pub const R2057_PGA_PTAT_TXGM5G_PU_CORE0: c_uint = 0x06a;
pub const R2057_PAD5G_PTATS1_CORE0: c_uint = 0x06b;
pub const R2057_PAD5G_CLASS_PTATS2_CORE0: c_uint = 0x06c;
pub const R2057_PGA_BOOSTPTAT_IMAIN_CORE0: c_uint = 0x06d;
pub const R2057_PAD5G_CASCV_IMAIN_CORE0: c_uint = 0x06e;
pub const R2057_TXMIX5G_IBOOST_PAD_IAUX_CORE0: c_uint = 0x06f;
pub const R2057_PGA_BOOST_TUNE_CORE0: c_uint = 0x070;
pub const R2057_PGA_GAIN_CORE0: c_uint = 0x071;
pub const R2057_PAD5G_CASCOFFV_GAIN_PUS_CORE0: c_uint = 0x072;
pub const R2057_TXMIX5G_BOOST_TUNE_CORE0: c_uint = 0x073;
pub const R2057_PAD5G_TUNE_MISC_PUS_CORE0: c_uint = 0x074;
pub const R2057_IPA5G_IAUX_CORE0: c_uint = 0x075;
pub const R2057_IPA5G_GAIN_CORE0: c_uint = 0x076;
pub const R2057_TSSI5G_SPARE1_CORE0: c_uint = 0x077;
pub const R2057_TSSI5G_SPARE2_CORE0: c_uint = 0x078;
pub const R2057_IPA5G_CASCOFFV_PU_CORE0: c_uint = 0x079;
pub const R2057_IPA5G_PTAT_CORE0: c_uint = 0x07a;
pub const R2057_IPA5G_IMAIN_CORE0: c_uint = 0x07b;
pub const R2057_IPA5G_CASCONV_CORE0: c_uint = 0x07c;
pub const R2057_IPA5G_BIAS_FILTER_CORE0: c_uint = 0x07d;
pub const R2057_PAD_BIAS_FILTER_BWS_CORE0: c_uint = 0x080;
pub const R2057_TR2G_CONFIG1_CORE0_NU: c_uint = 0x081;
pub const R2057_TR2G_CONFIG2_CORE0_NU: c_uint = 0x082;
pub const R2057_LNA5G_RFEN_CORE0: c_uint = 0x083;
pub const R2057_TR5G_CONFIG2_CORE0_NU: c_uint = 0x084;
pub const R2057_RXRFBIAS_IBOOST_PU_CORE0: c_uint = 0x085;
pub const R2057_RXRF_IABAND_RXGM_IMAIN_PTAT_CORE0: c_uint = 0x086;
pub const R2057_RXGM_CMFBITAIL_AUXPTAT_CORE0: c_uint = 0x087;
pub const R2057_RXMIX_ICORE_RXGM_IAUX_CORE0: c_uint = 0x088;
pub const R2057_RXMIX_CMFBITAIL_PU_CORE0: c_uint = 0x089;
pub const R2057_LNA2_IMAIN_PTAT_PU_CORE0: c_uint = 0x08a;
pub const R2057_LNA2_IAUX_PTAT_CORE0: c_uint = 0x08b;
pub const R2057_LNA1_IMAIN_PTAT_PU_CORE0: c_uint = 0x08c;
pub const R2057_LNA15G_INPUT_MATCH_TUNE_CORE0: c_uint = 0x08d;
pub const R2057_RXRFBIAS_BANDSEL_CORE0: c_uint = 0x08e;
pub const R2057_TIA_CONFIG_CORE0: c_uint = 0x08f;
pub const R2057_TIA_IQGAIN_CORE0: c_uint = 0x090;
pub const R2057_TIA_IBIAS2_CORE0: c_uint = 0x091;
pub const R2057_TIA_IBIAS1_CORE0: c_uint = 0x092;
pub const R2057_TIA_SPARE_Q_CORE0: c_uint = 0x093;
pub const R2057_TIA_SPARE_I_CORE0: c_uint = 0x094;
pub const R2057_RXMIX2G_PUS_CORE0: c_uint = 0x095;
pub const R2057_RXMIX2G_VCMREFS_CORE0: c_uint = 0x096;
pub const R2057_RXMIX2G_LODC_QI_CORE0: c_uint = 0x097;
pub const R2057_W12G_BW_LNA2G_PUS_CORE0: c_uint = 0x098;
pub const R2057_LNA2G_GAIN_CORE0: c_uint = 0x099;
pub const R2057_LNA2G_TUNE_CORE0: c_uint = 0x09a;
pub const R2057_RXMIX5G_PUS_CORE0: c_uint = 0x09b;
pub const R2057_RXMIX5G_VCMREFS_CORE0: c_uint = 0x09c;
pub const R2057_RXMIX5G_LODC_QI_CORE0: c_uint = 0x09d;
pub const R2057_W15G_BW_LNA5G_PUS_CORE0: c_uint = 0x09e;
pub const R2057_LNA5G_GAIN_CORE0: c_uint = 0x09f;
pub const R2057_LNA5G_TUNE_CORE0: c_uint = 0x0a0;
pub const R2057_LPFSEL_TXRX_RXBB_PUS_CORE0: c_uint = 0x0a1;
pub const R2057_RXBB_BIAS_MASTER_CORE0: c_uint = 0x0a2;
pub const R2057_RXBB_VGABUF_IDACS_CORE0: c_uint = 0x0a3;
pub const R2057_LPF_VCMREF_TXBUF_VCMREF_CORE0: c_uint = 0x0a4;
pub const R2057_TXBUF_VINCM_CORE0: c_uint = 0x0a5;
pub const R2057_TXBUF_IDACS_CORE0: c_uint = 0x0a6;
pub const R2057_LPF_RESP_RXBUF_BW_CORE0: c_uint = 0x0a7;
pub const R2057_RXBB_CC_CORE0: c_uint = 0x0a8;
pub const R2057_RXBB_SPARE3_CORE0: c_uint = 0x0a9;
pub const R2057_RXBB_RCCAL_HPC_CORE0: c_uint = 0x0aa;
pub const R2057_LPF_IDACS_CORE0: c_uint = 0x0ab;
pub const R2057_LPFBYP_DCLOOP_BYP_IDAC_CORE0: c_uint = 0x0ac;
pub const R2057_TXBUF_GAIN_CORE0: c_uint = 0x0ad;
pub const R2057_AFELOOPBACK_AACI_RESP_CORE0: c_uint = 0x0ae;
pub const R2057_RXBUF_DEGEN_CORE0: c_uint = 0x0af;
pub const R2057_RXBB_SPARE2_CORE0: c_uint = 0x0b0;
pub const R2057_RXBB_SPARE1_CORE0: c_uint = 0x0b1;
pub const R2057_RSSI_MASTER_CORE0: c_uint = 0x0b2;
pub const R2057_W2_MASTER_CORE0: c_uint = 0x0b3;
pub const R2057_NB_MASTER_CORE0: c_uint = 0x0b4;
pub const R2057_W2_IDACS0_Q_CORE0: c_uint = 0x0b5;
pub const R2057_W2_IDACS1_Q_CORE0: c_uint = 0x0b6;
pub const R2057_W2_IDACS0_I_CORE0: c_uint = 0x0b7;
pub const R2057_W2_IDACS1_I_CORE0: c_uint = 0x0b8;
pub const R2057_RSSI_GPAIOSEL_W1_IDACS_CORE0: c_uint = 0x0b9;
pub const R2057_NB_IDACS_Q_CORE0: c_uint = 0x0ba;
pub const R2057_NB_IDACS_I_CORE0: c_uint = 0x0bb;
pub const R2057_BACKUP4_CORE0: c_uint = 0x0c1;
pub const R2057_BACKUP3_CORE0: c_uint = 0x0c2;
pub const R2057_BACKUP2_CORE0: c_uint = 0x0c3;
pub const R2057_BACKUP1_CORE0: c_uint = 0x0c4;
pub const R2057_SPARE16_CORE0: c_uint = 0x0c5;
pub const R2057_SPARE15_CORE0: c_uint = 0x0c6;
pub const R2057_SPARE14_CORE0: c_uint = 0x0c7;
pub const R2057_SPARE13_CORE0: c_uint = 0x0c8;
pub const R2057_SPARE12_CORE0: c_uint = 0x0c9;
pub const R2057_SPARE11_CORE0: c_uint = 0x0ca;
pub const R2057_TX2G_BIAS_RESETS_CORE0: c_uint = 0x0cb;
pub const R2057_TX5G_BIAS_RESETS_CORE0: c_uint = 0x0cc;
pub const R2057_IQTEST_SEL_PU: c_uint = 0x0cd;
pub const R2057_XTAL_CONFIG2: c_uint = 0x0ce;
pub const R2057_BUFS_MISC_LPFBW_CORE0: c_uint = 0x0cf;
pub const R2057_TXLPF_RCCAL_CORE0: c_uint = 0x0d0;
pub const R2057_RXBB_GPAIOSEL_RXLPF_RCCAL_CORE0: c_uint = 0x0d1;
pub const R2057_LPF_GAIN_CORE0: c_uint = 0x0d2;
pub const R2057_DACBUF_IDACS_BW_CORE0: c_uint = 0x0d3;
// MISC core 1
pub const R2057_RXTXBIAS_CONFIG_CORE1: c_uint = 0x0d4;
pub const R2057_TXGM_TXRF_PUS_CORE1: c_uint = 0x0d5;
pub const R2057_TXGM_IDAC_BLEED_CORE1: c_uint = 0x0d6;
pub const R2057_TXGM_GAIN_CORE1: c_uint = 0x0db;
pub const R2057_TXGM2G_PKDET_PUS_CORE1: c_uint = 0x0dc;
pub const R2057_PAD2G_PTATS_CORE1: c_uint = 0x0dd;
pub const R2057_PAD2G_IDACS_CORE1: c_uint = 0x0de;
pub const R2057_PAD2G_BOOST_PU_CORE1: c_uint = 0x0df;
pub const R2057_PAD2G_CASCV_GAIN_CORE1: c_uint = 0x0e0;
pub const R2057_TXMIX2G_TUNE_BOOST_PU_CORE1: c_uint = 0x0e1;
pub const R2057_TXMIX2G_LODC_CORE1: c_uint = 0x0e2;
pub const R2057_PAD2G_TUNE_PUS_CORE1: c_uint = 0x0e3;
pub const R2057_IPA2G_GAIN_CORE1: c_uint = 0x0e4;
pub const R2057_TSSI2G_SPARE1_CORE1: c_uint = 0x0e5;
pub const R2057_TSSI2G_SPARE2_CORE1: c_uint = 0x0e6;
pub const R2057_IPA2G_TUNEV_CASCV_PTAT_CORE1: c_uint = 0x0e7;
pub const R2057_IPA2G_IMAIN_CORE1: c_uint = 0x0e8;
pub const R2057_IPA2G_CASCONV_CORE1: c_uint = 0x0e9;
pub const R2057_IPA2G_CASCOFFV_CORE1: c_uint = 0x0ea;
pub const R2057_IPA2G_BIAS_FILTER_CORE1: c_uint = 0x0eb;
pub const R2057_TX5G_PKDET_CORE1: c_uint = 0x0ee;
pub const R2057_PGA_PTAT_TXGM5G_PU_CORE1: c_uint = 0x0ef;
pub const R2057_PAD5G_PTATS1_CORE1: c_uint = 0x0f0;
pub const R2057_PAD5G_CLASS_PTATS2_CORE1: c_uint = 0x0f1;
pub const R2057_PGA_BOOSTPTAT_IMAIN_CORE1: c_uint = 0x0f2;
pub const R2057_PAD5G_CASCV_IMAIN_CORE1: c_uint = 0x0f3;
pub const R2057_TXMIX5G_IBOOST_PAD_IAUX_CORE1: c_uint = 0x0f4;
pub const R2057_PGA_BOOST_TUNE_CORE1: c_uint = 0x0f5;
pub const R2057_PGA_GAIN_CORE1: c_uint = 0x0f6;
pub const R2057_PAD5G_CASCOFFV_GAIN_PUS_CORE1: c_uint = 0x0f7;
pub const R2057_TXMIX5G_BOOST_TUNE_CORE1: c_uint = 0x0f8;
pub const R2057_PAD5G_TUNE_MISC_PUS_CORE1: c_uint = 0x0f9;
pub const R2057_IPA5G_IAUX_CORE1: c_uint = 0x0fa;
pub const R2057_IPA5G_GAIN_CORE1: c_uint = 0x0fb;
pub const R2057_TSSI5G_SPARE1_CORE1: c_uint = 0x0fc;
pub const R2057_TSSI5G_SPARE2_CORE1: c_uint = 0x0fd;
pub const R2057_IPA5G_CASCOFFV_PU_CORE1: c_uint = 0x0fe;
pub const R2057_IPA5G_PTAT_CORE1: c_uint = 0x0ff;
pub const R2057_IPA5G_IMAIN_CORE1: c_uint = 0x100;
pub const R2057_IPA5G_CASCONV_CORE1: c_uint = 0x101;
pub const R2057_IPA5G_BIAS_FILTER_CORE1: c_uint = 0x102;
pub const R2057_PAD_BIAS_FILTER_BWS_CORE1: c_uint = 0x105;
pub const R2057_TR2G_CONFIG1_CORE1_NU: c_uint = 0x106;
pub const R2057_TR2G_CONFIG2_CORE1_NU: c_uint = 0x107;
pub const R2057_LNA5G_RFEN_CORE1: c_uint = 0x108;
pub const R2057_TR5G_CONFIG2_CORE1_NU: c_uint = 0x109;
pub const R2057_RXRFBIAS_IBOOST_PU_CORE1: c_uint = 0x10a;
pub const R2057_RXRF_IABAND_RXGM_IMAIN_PTAT_CORE1: c_uint = 0x10b;
pub const R2057_RXGM_CMFBITAIL_AUXPTAT_CORE1: c_uint = 0x10c;
pub const R2057_RXMIX_ICORE_RXGM_IAUX_CORE1: c_uint = 0x10d;
pub const R2057_RXMIX_CMFBITAIL_PU_CORE1: c_uint = 0x10e;
pub const R2057_LNA2_IMAIN_PTAT_PU_CORE1: c_uint = 0x10f;
pub const R2057_LNA2_IAUX_PTAT_CORE1: c_uint = 0x110;
pub const R2057_LNA1_IMAIN_PTAT_PU_CORE1: c_uint = 0x111;
pub const R2057_LNA15G_INPUT_MATCH_TUNE_CORE1: c_uint = 0x112;
pub const R2057_RXRFBIAS_BANDSEL_CORE1: c_uint = 0x113;
pub const R2057_TIA_CONFIG_CORE1: c_uint = 0x114;
pub const R2057_TIA_IQGAIN_CORE1: c_uint = 0x115;
pub const R2057_TIA_IBIAS2_CORE1: c_uint = 0x116;
pub const R2057_TIA_IBIAS1_CORE1: c_uint = 0x117;
pub const R2057_TIA_SPARE_Q_CORE1: c_uint = 0x118;
pub const R2057_TIA_SPARE_I_CORE1: c_uint = 0x119;
pub const R2057_RXMIX2G_PUS_CORE1: c_uint = 0x11a;
pub const R2057_RXMIX2G_VCMREFS_CORE1: c_uint = 0x11b;
pub const R2057_RXMIX2G_LODC_QI_CORE1: c_uint = 0x11c;
pub const R2057_W12G_BW_LNA2G_PUS_CORE1: c_uint = 0x11d;
pub const R2057_LNA2G_GAIN_CORE1: c_uint = 0x11e;
pub const R2057_LNA2G_TUNE_CORE1: c_uint = 0x11f;
pub const R2057_RXMIX5G_PUS_CORE1: c_uint = 0x120;
pub const R2057_RXMIX5G_VCMREFS_CORE1: c_uint = 0x121;
pub const R2057_RXMIX5G_LODC_QI_CORE1: c_uint = 0x122;
pub const R2057_W15G_BW_LNA5G_PUS_CORE1: c_uint = 0x123;
pub const R2057_LNA5G_GAIN_CORE1: c_uint = 0x124;
pub const R2057_LNA5G_TUNE_CORE1: c_uint = 0x125;
pub const R2057_LPFSEL_TXRX_RXBB_PUS_CORE1: c_uint = 0x126;
pub const R2057_RXBB_BIAS_MASTER_CORE1: c_uint = 0x127;
pub const R2057_RXBB_VGABUF_IDACS_CORE1: c_uint = 0x128;
pub const R2057_LPF_VCMREF_TXBUF_VCMREF_CORE1: c_uint = 0x129;
pub const R2057_TXBUF_VINCM_CORE1: c_uint = 0x12a;
pub const R2057_TXBUF_IDACS_CORE1: c_uint = 0x12b;
pub const R2057_LPF_RESP_RXBUF_BW_CORE1: c_uint = 0x12c;
pub const R2057_RXBB_CC_CORE1: c_uint = 0x12d;
pub const R2057_RXBB_SPARE3_CORE1: c_uint = 0x12e;
pub const R2057_RXBB_RCCAL_HPC_CORE1: c_uint = 0x12f;
pub const R2057_LPF_IDACS_CORE1: c_uint = 0x130;
pub const R2057_LPFBYP_DCLOOP_BYP_IDAC_CORE1: c_uint = 0x131;
pub const R2057_TXBUF_GAIN_CORE1: c_uint = 0x132;
pub const R2057_AFELOOPBACK_AACI_RESP_CORE1: c_uint = 0x133;
pub const R2057_RXBUF_DEGEN_CORE1: c_uint = 0x134;
pub const R2057_RXBB_SPARE2_CORE1: c_uint = 0x135;
pub const R2057_RXBB_SPARE1_CORE1: c_uint = 0x136;
pub const R2057_RSSI_MASTER_CORE1: c_uint = 0x137;
pub const R2057_W2_MASTER_CORE1: c_uint = 0x138;
pub const R2057_NB_MASTER_CORE1: c_uint = 0x139;
pub const R2057_W2_IDACS0_Q_CORE1: c_uint = 0x13a;
pub const R2057_W2_IDACS1_Q_CORE1: c_uint = 0x13b;
pub const R2057_W2_IDACS0_I_CORE1: c_uint = 0x13c;
pub const R2057_W2_IDACS1_I_CORE1: c_uint = 0x13d;
pub const R2057_RSSI_GPAIOSEL_W1_IDACS_CORE1: c_uint = 0x13e;
pub const R2057_NB_IDACS_Q_CORE1: c_uint = 0x13f;
pub const R2057_NB_IDACS_I_CORE1: c_uint = 0x140;
pub const R2057_BACKUP4_CORE1: c_uint = 0x146;
pub const R2057_BACKUP3_CORE1: c_uint = 0x147;
pub const R2057_BACKUP2_CORE1: c_uint = 0x148;
pub const R2057_BACKUP1_CORE1: c_uint = 0x149;
pub const R2057_SPARE16_CORE1: c_uint = 0x14a;
pub const R2057_SPARE15_CORE1: c_uint = 0x14b;
pub const R2057_SPARE14_CORE1: c_uint = 0x14c;
pub const R2057_SPARE13_CORE1: c_uint = 0x14d;
pub const R2057_SPARE12_CORE1: c_uint = 0x14e;
pub const R2057_SPARE11_CORE1: c_uint = 0x14f;
pub const R2057_TX2G_BIAS_RESETS_CORE1: c_uint = 0x150;
pub const R2057_TX5G_BIAS_RESETS_CORE1: c_uint = 0x151;
pub const R2057_SPARE8_CORE1: c_uint = 0x152;
pub const R2057_SPARE7_CORE1: c_uint = 0x153;
pub const R2057_BUFS_MISC_LPFBW_CORE1: c_uint = 0x154;
pub const R2057_TXLPF_RCCAL_CORE1: c_uint = 0x155;
pub const R2057_RXBB_GPAIOSEL_RXLPF_RCCAL_CORE1: c_uint = 0x156;
pub const R2057_LPF_GAIN_CORE1: c_uint = 0x157;
pub const R2057_DACBUF_IDACS_BW_CORE1: c_uint = 0x158;
pub const R2057_DACBUF_VINCM_CORE1: c_uint = 0x159;
pub const R2057_RCCAL_START_R1_Q1_P1: c_uint = 0x15a;
pub const R2057_RCCAL_X1: c_uint = 0x15b;
pub const R2057_RCCAL_TRC0: c_uint = 0x15c;
pub const R2057_RCCAL_TRC1: c_uint = 0x15d;
pub const R2057_RCCAL_DONE_OSCCAP: c_uint = 0x15e;
pub const R2057_RCCAL_N0_0: c_uint = 0x15f;
pub const R2057_RCCAL_N0_1: c_uint = 0x160;
pub const R2057_RCCAL_N1_0: c_uint = 0x161;
pub const R2057_RCCAL_N1_1: c_uint = 0x162;
pub const R2057_RCAL_STATUS: c_uint = 0x163;
pub const R2057_XTALPUOVR_PINCTRL: c_uint = 0x164;
pub const R2057_OVR_REG0: c_uint = 0x165;
pub const R2057_OVR_REG1: c_uint = 0x166;
pub const R2057_OVR_REG2: c_uint = 0x167;
pub const R2057_OVR_REG3: c_uint = 0x168;
pub const R2057_OVR_REG4: c_uint = 0x169;
pub const R2057_RCCAL_SCAP_VAL: c_uint = 0x16a;
pub const R2057_RCCAL_BCAP_VAL: c_uint = 0x16b;
pub const R2057_RCCAL_HPC_VAL: c_uint = 0x16c;
pub const R2057_RCCAL_OVERRIDES: c_uint = 0x16d;
// TX core 0
pub const R2057_TX0_IQCAL_GAIN_BW: c_uint = 0x170;
pub const R2057_TX0_LOFT_FINE_I: c_uint = 0x171;
pub const R2057_TX0_LOFT_FINE_Q: c_uint = 0x172;
pub const R2057_TX0_LOFT_COARSE_I: c_uint = 0x173;
pub const R2057_TX0_LOFT_COARSE_Q: c_uint = 0x174;
pub const R2057_TX0_TX_SSI_MASTER: c_uint = 0x175;
pub const R2057_TX0_IQCAL_VCM_HG: c_uint = 0x176;
pub const R2057_TX0_IQCAL_IDAC: c_uint = 0x177;
pub const R2057_TX0_TSSI_VCM: c_uint = 0x178;
pub const R2057_TX0_TX_SSI_MUX: c_uint = 0x179;
pub const R2057_TX0_TSSIA: c_uint = 0x17a;
pub const R2057_TX0_TSSIG: c_uint = 0x17b;
pub const R2057_TX0_TSSI_MISC1: c_uint = 0x17c;
pub const R2057_TX0_TXRXCOUPLE_2G_ATTEN: c_uint = 0x17d;
pub const R2057_TX0_TXRXCOUPLE_2G_PWRUP: c_uint = 0x17e;
pub const R2057_TX0_TXRXCOUPLE_5G_ATTEN: c_uint = 0x17f;
pub const R2057_TX0_TXRXCOUPLE_5G_PWRUP: c_uint = 0x180;
// TX core 1
pub const R2057_TX1_IQCAL_GAIN_BW: c_uint = 0x190;
pub const R2057_TX1_LOFT_FINE_I: c_uint = 0x191;
pub const R2057_TX1_LOFT_FINE_Q: c_uint = 0x192;
pub const R2057_TX1_LOFT_COARSE_I: c_uint = 0x193;
pub const R2057_TX1_LOFT_COARSE_Q: c_uint = 0x194;
pub const R2057_TX1_TX_SSI_MASTER: c_uint = 0x195;
pub const R2057_TX1_IQCAL_VCM_HG: c_uint = 0x196;
pub const R2057_TX1_IQCAL_IDAC: c_uint = 0x197;
pub const R2057_TX1_TSSI_VCM: c_uint = 0x198;
pub const R2057_TX1_TX_SSI_MUX: c_uint = 0x199;
pub const R2057_TX1_TSSIA: c_uint = 0x19a;
pub const R2057_TX1_TSSIG: c_uint = 0x19b;
pub const R2057_TX1_TSSI_MISC1: c_uint = 0x19c;
pub const R2057_TX1_TXRXCOUPLE_2G_ATTEN: c_uint = 0x19d;
pub const R2057_TX1_TXRXCOUPLE_2G_PWRUP: c_uint = 0x19e;
pub const R2057_TX1_TXRXCOUPLE_5G_ATTEN: c_uint = 0x19f;
pub const R2057_TX1_TXRXCOUPLE_5G_PWRUP: c_uint = 0x1a0;
pub const R2057_AFE_VCM_CAL_MASTER_CORE0: c_uint = 0x1a1;
pub const R2057_AFE_SET_VCM_I_CORE0: c_uint = 0x1a2;
pub const R2057_AFE_SET_VCM_Q_CORE0: c_uint = 0x1a3;
pub const R2057_AFE_STATUS_VCM_IQADC_CORE0: c_uint = 0x1a4;
pub const R2057_AFE_STATUS_VCM_I_CORE0: c_uint = 0x1a5;
pub const R2057_AFE_STATUS_VCM_Q_CORE0: c_uint = 0x1a6;
pub const R2057_AFE_VCM_CAL_MASTER_CORE1: c_uint = 0x1a7;
pub const R2057_AFE_SET_VCM_I_CORE1: c_uint = 0x1a8;
pub const R2057_AFE_SET_VCM_Q_CORE1: c_uint = 0x1a9;
pub const R2057_AFE_STATUS_VCM_IQADC_CORE1: c_uint = 0x1aa;
pub const R2057_AFE_STATUS_VCM_I_CORE1: c_uint = 0x1ab;
pub const R2057_AFE_STATUS_VCM_Q_CORE1: c_uint = 0x1ac;
pub const R2057v7_DACBUF_VINCM_CORE0: c_uint = 0x1ad;
pub const R2057v7_RCCAL_MASTER: c_uint = 0x1ae;
pub const R2057v7_TR2G_CONFIG3_CORE0_NU: c_uint = 0x1af;
pub const R2057v7_TR2G_CONFIG3_CORE1_NU: c_uint = 0x1b0;
pub const R2057v7_LOGEN_PUS1: c_uint = 0x1b1;
pub const R2057v7_OVR_REG5: c_uint = 0x1b2;
pub const R2057v7_OVR_REG6: c_uint = 0x1b3;
pub const R2057v7_OVR_REG7: c_uint = 0x1b4;
pub const R2057v7_OVR_REG8: c_uint = 0x1b5;
pub const R2057v7_OVR_REG9: c_uint = 0x1b6;
pub const R2057v7_OVR_REG10: c_uint = 0x1b7;
pub const R2057v7_OVR_REG11: c_uint = 0x1b8;
pub const R2057v7_OVR_REG12: c_uint = 0x1b9;
pub const R2057v7_OVR_REG13: c_uint = 0x1ba;
pub const R2057v7_OVR_REG14: c_uint = 0x1bb;
pub const R2057v7_OVR_REG15: c_uint = 0x1bc;
pub const R2057v7_OVR_REG16: c_uint = 0x1bd;
pub const R2057v7_OVR_REG1: c_uint = 0x1be;
pub const R2057v7_OVR_REG18: c_uint = 0x1bf;
pub const R2057v7_OVR_REG19: c_uint = 0x1c0;
pub const R2057v7_OVR_REG20: c_uint = 0x1c1;
pub const R2057v7_OVR_REG21: c_uint = 0x1c2;
pub const R2057v7_OVR_REG2: c_uint = 0x1c3;
pub const R2057v7_OVR_REG23: c_uint = 0x1c4;
pub const R2057v7_OVR_REG24: c_uint = 0x1c5;
pub const R2057v7_OVR_REG25: c_uint = 0x1c6;
pub const R2057v7_OVR_REG26: c_uint = 0x1c7;
pub const R2057v7_OVR_REG27: c_uint = 0x1c8;
pub const R2057v7_OVR_REG28: c_uint = 0x1c9;
pub const R2057v7_IQTEST_SEL_PU2: c_uint = 0x1ca;
pub const R2057_VCM_MASK: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_nphy_chantabent_rev7 {
// The channel frequency in MHz
    pub freq: u16,
// Radio regs values on channelswitch
    pub radio_vcocal_countval0: u8,
    pub radio_vcocal_countval1: u8,
    pub radio_rfpll_refmaster_sparextalsize: u8,
    pub radio_rfpll_loopfilter_r1: u8,
    pub radio_rfpll_loopfilter_c2: u8,
    pub radio_rfpll_loopfilter_c1: u8,
    pub radio_cp_kpd_idac: u8,
    pub radio_rfpll_mmd0: u8,
    pub radio_rfpll_mmd1: u8,
    pub radio_vcobuf_tune: u8,
    pub radio_logen_mx2g_tune: u8,
    pub radio_logen_mx5g_tune: u8,
    pub radio_logen_indbuf2g_tune: u8,
    pub radio_logen_indbuf5g_tune: u8,
    pub radio_txmix2g_tune_boost_pu_core0: u8,
    pub radio_pad2g_tune_pus_core0: u8,
    pub radio_pga_boost_tune_core0: u8,
    pub radio_txmix5g_boost_tune_core0: u8,
    pub radio_pad5g_tune_misc_pus_core0: u8,
    pub radio_lna2g_tune_core0: u8,
    pub radio_lna5g_tune_core0: u8,
    pub radio_txmix2g_tune_boost_pu_core1: u8,
    pub radio_pad2g_tune_pus_core1: u8,
    pub radio_pga_boost_tune_core1: u8,
    pub radio_txmix5g_boost_tune_core1: u8,
    pub radio_pad5g_tune_misc_pus_core1: u8,
    pub radio_lna2g_tune_core1: u8,
    pub radio_lna5g_tune_core1: u8,
// PHY res values on channelswitch
    pub phy_regs: b43_phy_n_sfo_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_nphy_chantabent_rev7_2g {
// The channel frequency in MHz
    pub freq: u16,
// Radio regs values on channelswitch
    pub radio_vcocal_countval0: u8,
    pub radio_vcocal_countval1: u8,
    pub radio_rfpll_refmaster_sparextalsize: u8,
    pub radio_rfpll_loopfilter_r1: u8,
    pub radio_rfpll_loopfilter_c2: u8,
    pub radio_rfpll_loopfilter_c1: u8,
    pub radio_cp_kpd_idac: u8,
    pub radio_rfpll_mmd0: u8,
    pub radio_rfpll_mmd1: u8,
    pub radio_vcobuf_tune: u8,
    pub radio_logen_mx2g_tune: u8,
    pub radio_logen_indbuf2g_tune: u8,
    pub radio_txmix2g_tune_boost_pu_core0: u8,
    pub radio_pad2g_tune_pus_core0: u8,
    pub radio_lna2g_tune_core0: u8,
    pub radio_txmix2g_tune_boost_pu_core1: u8,
    pub radio_pad2g_tune_pus_core1: u8,
    pub radio_lna2g_tune_core1: u8,
// PHY regs values on channelswitch
    pub phy_regs: b43_phy_n_sfo_cfg,
}

extern "C" {
    pub fn r2057_upload_inittabs(dev: *mut b43_wldev);
}
