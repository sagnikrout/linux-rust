//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/mt6359.h
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
// Copyright (C) 2020 MediaTek Inc.
// Author: Argus Lin <argus.lin@mediatek.com>
//
// Register Bit Define
pub const MT6359_TOP0_ID: c_uint = 0x0;
pub const MT6359_SMT_CON1: c_uint = 0x32;
pub const MT6359_DRV_CON2: c_uint = 0x3c;
pub const MT6359_DRV_CON3: c_uint = 0x3e;
pub const MT6359_DRV_CON4: c_uint = 0x40;
pub const MT6359_TOP_CKPDN_CON0: c_uint = 0x10c;
pub const MT6359_TOP_CKPDN_CON0_SET: c_uint = 0x10e;
pub const MT6359_TOP_CKPDN_CON0_CLR: c_uint = 0x110;
pub const MT6359_AUXADC_RQST0: c_uint = 0x1108;
pub const MT6359_AUXADC_CON10: c_uint = 0x11a0;
pub const MT6359_AUXADC_ACCDET: c_uint = 0x11ba;
pub const MT6359_LDO_VUSB_OP_EN: c_uint = 0x1d0c;
pub const MT6359_LDO_VUSB_OP_EN_SET: c_uint = 0x1d0e;
pub const MT6359_LDO_VUSB_OP_EN_CLR: c_uint = 0x1d10;
pub const MT6359_AUD_TOP_CKPDN_CON0: c_uint = 0x230c;
pub const MT6359_AUD_TOP_CKPDN_CON0_SET: c_uint = 0x230e;
pub const MT6359_AUD_TOP_CKPDN_CON0_CLR: c_uint = 0x2310;
pub const MT6359_AUD_TOP_RST_CON0: c_uint = 0x2320;
pub const MT6359_AUD_TOP_RST_CON0_SET: c_uint = 0x2322;
pub const MT6359_AUD_TOP_RST_CON0_CLR: c_uint = 0x2324;
pub const MT6359_AUD_TOP_INT_CON0: c_uint = 0x2328;
pub const MT6359_AUD_TOP_INT_CON0_SET: c_uint = 0x232a;
pub const MT6359_AUD_TOP_INT_CON0_CLR: c_uint = 0x232c;
pub const MT6359_AUD_TOP_INT_MASK_CON0: c_uint = 0x232e;
pub const MT6359_AUD_TOP_INT_MASK_CON0_SET: c_uint = 0x2330;
pub const MT6359_AUD_TOP_INT_MASK_CON0_CLR: c_uint = 0x2332;
pub const MT6359_AUD_TOP_INT_STATUS0: c_uint = 0x2334;
pub const MT6359_AFE_NCP_CFG2: c_uint = 0x24e2;
pub const MT6359_AUDENC_DSN_ID: c_uint = 0x2500;
pub const MT6359_AUDENC_DSN_REV0: c_uint = 0x2502;
pub const MT6359_AUDENC_DSN_DBI: c_uint = 0x2504;
pub const MT6359_AUDENC_DSN_FPI: c_uint = 0x2506;
pub const MT6359_AUDENC_ANA_CON0: c_uint = 0x2508;
pub const MT6359_AUDENC_ANA_CON1: c_uint = 0x250a;
pub const MT6359_AUDENC_ANA_CON2: c_uint = 0x250c;
pub const MT6359_AUDENC_ANA_CON3: c_uint = 0x250e;
pub const MT6359_AUDENC_ANA_CON4: c_uint = 0x2510;
pub const MT6359_AUDENC_ANA_CON5: c_uint = 0x2512;
pub const MT6359_AUDENC_ANA_CON6: c_uint = 0x2514;
pub const MT6359_AUDENC_ANA_CON7: c_uint = 0x2516;
pub const MT6359_AUDENC_ANA_CON8: c_uint = 0x2518;
pub const MT6359_AUDENC_ANA_CON9: c_uint = 0x251a;
pub const MT6359_AUDENC_ANA_CON10: c_uint = 0x251c;
pub const MT6359_AUDENC_ANA_CON11: c_uint = 0x251e;
pub const MT6359_AUDENC_ANA_CON12: c_uint = 0x2520;
pub const MT6359_AUDENC_ANA_CON13: c_uint = 0x2522;
pub const MT6359_AUDENC_ANA_CON14: c_uint = 0x2524;
pub const MT6359_AUDENC_ANA_CON15: c_uint = 0x2526;
pub const MT6359_AUDENC_ANA_CON16: c_uint = 0x2528;
pub const MT6359_AUDENC_ANA_CON17: c_uint = 0x252a;
pub const MT6359_AUDENC_ANA_CON18: c_uint = 0x252c;
pub const MT6359_AUDENC_ANA_CON19: c_uint = 0x252e;
pub const MT6359_AUDENC_ANA_CON20: c_uint = 0x2530;
pub const MT6359_AUDENC_ANA_CON21: c_uint = 0x2532;
pub const MT6359_AUDENC_ANA_CON22: c_uint = 0x2534;
pub const MT6359_AUDENC_ANA_CON23: c_uint = 0x2536;
pub const MT6359_AUDDEC_DSN_ID: c_uint = 0x2580;
pub const MT6359_AUDDEC_DSN_REV0: c_uint = 0x2582;
pub const MT6359_AUDDEC_DSN_DBI: c_uint = 0x2584;
pub const MT6359_AUDDEC_DSN_FPI: c_uint = 0x2586;
pub const MT6359_AUDDEC_ANA_CON0: c_uint = 0x2588;
pub const MT6359_AUDDEC_ANA_CON1: c_uint = 0x258a;
pub const MT6359_AUDDEC_ANA_CON2: c_uint = 0x258c;
pub const MT6359_AUDDEC_ANA_CON3: c_uint = 0x258e;
pub const MT6359_AUDDEC_ANA_CON4: c_uint = 0x2590;
pub const MT6359_AUDDEC_ANA_CON5: c_uint = 0x2592;
pub const MT6359_AUDDEC_ANA_CON6: c_uint = 0x2594;
pub const MT6359_AUDDEC_ANA_CON7: c_uint = 0x2596;
pub const MT6359_AUDDEC_ANA_CON8: c_uint = 0x2598;
pub const MT6359_AUDDEC_ANA_CON9: c_uint = 0x259a;
pub const MT6359_AUDDEC_ANA_CON10: c_uint = 0x259c;
pub const MT6359_AUDDEC_ANA_CON11: c_uint = 0x259e;
pub const MT6359_AUDDEC_ANA_CON12: c_uint = 0x25a0;
pub const MT6359_AUDDEC_ANA_CON13: c_uint = 0x25a2;
pub const MT6359_AUDDEC_ANA_CON14: c_uint = 0x25a4;
pub const MT6359_ACCDET_DSN_DIG_ID: c_uint = 0x2680;
pub const MT6359_ACCDET_DSN_DIG_REV0: c_uint = 0x2682;
pub const MT6359_ACCDET_DSN_DBI: c_uint = 0x2684;
pub const MT6359_ACCDET_DSN_FPI: c_uint = 0x2686;
pub const MT6359_ACCDET_CON0: c_uint = 0x2688;
pub const MT6359_ACCDET_CON1: c_uint = 0x268a;
pub const MT6359_ACCDET_CON2: c_uint = 0x268c;
pub const MT6359_ACCDET_CON3: c_uint = 0x268e;
pub const MT6359_ACCDET_CON4: c_uint = 0x2690;
pub const MT6359_ACCDET_CON5: c_uint = 0x2692;
pub const MT6359_ACCDET_CON6: c_uint = 0x2694;
pub const MT6359_ACCDET_CON7: c_uint = 0x2696;
pub const MT6359_ACCDET_CON8: c_uint = 0x2698;
pub const MT6359_ACCDET_CON9: c_uint = 0x269a;
pub const MT6359_ACCDET_CON10: c_uint = 0x269c;
pub const MT6359_ACCDET_CON11: c_uint = 0x269e;
pub const MT6359_ACCDET_CON12: c_uint = 0x26a0;
pub const MT6359_ACCDET_CON13: c_uint = 0x26a2;
pub const MT6359_ACCDET_CON14: c_uint = 0x26a4;
pub const MT6359_ACCDET_CON15: c_uint = 0x26a6;
pub const MT6359_ACCDET_CON16: c_uint = 0x26a8;
pub const MT6359_ACCDET_CON17: c_uint = 0x26aa;
pub const MT6359_ACCDET_CON18: c_uint = 0x26ac;
pub const MT6359_ACCDET_CON19: c_uint = 0x26ae;
pub const MT6359_ACCDET_CON20: c_uint = 0x26b0;
pub const MT6359_ACCDET_CON21: c_uint = 0x26b2;
pub const MT6359_ACCDET_CON22: c_uint = 0x26b4;
pub const MT6359_ACCDET_CON23: c_uint = 0x26b6;
pub const MT6359_ACCDET_CON24: c_uint = 0x26b8;
pub const MT6359_ACCDET_CON25: c_uint = 0x26ba;
pub const MT6359_ACCDET_CON26: c_uint = 0x26bc;
pub const MT6359_ACCDET_CON27: c_uint = 0x26be;
pub const MT6359_ACCDET_CON28: c_uint = 0x26c0;
pub const MT6359_ACCDET_CON29: c_uint = 0x26c2;
pub const MT6359_ACCDET_CON30: c_uint = 0x26c4;
pub const MT6359_ACCDET_CON31: c_uint = 0x26c6;
pub const MT6359_ACCDET_CON32: c_uint = 0x26c8;
pub const MT6359_ACCDET_CON33: c_uint = 0x26ca;
pub const MT6359_ACCDET_CON34: c_uint = 0x26cc;
pub const MT6359_ACCDET_CON35: c_uint = 0x26ce;
pub const MT6359_ACCDET_CON36: c_uint = 0x26d0;
pub const MT6359_ACCDET_CON37: c_uint = 0x26d2;
pub const MT6359_ACCDET_CON38: c_uint = 0x26d4;
pub const MT6359_ACCDET_CON39: c_uint = 0x26d6;
pub const MT6359_ACCDET_CON40: c_uint = 0x26d8;

pub const TOP0_ANA_ID_SFT: c_int = 0;
pub const TOP0_ANA_ID_MASK: c_uint = 0xFF;

pub const AUXADC_RQST_CH0_SFT: c_int = 0;
pub const AUXADC_RQST_CH0_MASK: c_uint = 0x1;

pub const AUXADC_ACCDET_ANASWCTRL_EN_SFT: c_int = 6;
pub const AUXADC_ACCDET_ANASWCTRL_EN_MASK: c_uint = 0x1;

pub const AUXADC_ACCDET_AUTO_SPL_SFT: c_int = 0;
pub const AUXADC_ACCDET_AUTO_SPL_MASK: c_uint = 0x1;

pub const AUXADC_ACCDET_AUTO_RQST_CLR_SFT: c_int = 1;
pub const AUXADC_ACCDET_AUTO_RQST_CLR_MASK: c_uint = 0x1;

pub const AUXADC_ACCDET_DIG1_RSV0_SFT: c_int = 2;
pub const AUXADC_ACCDET_DIG1_RSV0_MASK: c_uint = 0x3F;

pub const AUXADC_ACCDET_DIG0_RSV0_SFT: c_int = 8;
pub const AUXADC_ACCDET_DIG0_RSV0_MASK: c_uint = 0xFF;

pub const RG_ACCDET_CK_PDN_SFT: c_int = 0;
pub const RG_ACCDET_CK_PDN_MASK: c_uint = 0x1;

pub const RG_ACCDET_RST_SFT: c_int = 1;
pub const RG_ACCDET_RST_MASK: c_uint = 0x1;

pub const BANK_ACCDET_SWRST_SFT: c_int = 0;
pub const BANK_ACCDET_SWRST_MASK: c_uint = 0x1;

pub const RG_INT_EN_ACCDET_SFT: c_int = 5;
pub const RG_INT_EN_ACCDET_MASK: c_uint = 0x1;

pub const RG_INT_EN_ACCDET_EINT0_SFT: c_int = 6;
pub const RG_INT_EN_ACCDET_EINT0_MASK: c_uint = 0x1;

pub const RG_INT_EN_ACCDET_EINT1_SFT: c_int = 7;
pub const RG_INT_EN_ACCDET_EINT1_MASK: c_uint = 0x1;

pub const RG_INT_MASK_ACCDET_SFT: c_int = 5;
pub const RG_INT_MASK_ACCDET_MASK: c_uint = 0x1;

pub const RG_INT_MASK_ACCDET_EINT0_SFT: c_int = 6;
pub const RG_INT_MASK_ACCDET_EINT0_MASK: c_uint = 0x1;

pub const RG_INT_MASK_ACCDET_EINT1_SFT: c_int = 7;
pub const RG_INT_MASK_ACCDET_EINT1_MASK: c_uint = 0x1;

pub const RG_INT_STATUS_ACCDET_SFT: c_int = 5;
pub const RG_INT_STATUS_ACCDET_MASK: c_uint = 0x1;

pub const RG_INT_STATUS_ACCDET_EINT0_SFT: c_int = 6;
pub const RG_INT_STATUS_ACCDET_EINT0_MASK: c_uint = 0x1;

pub const RG_INT_STATUS_ACCDET_EINT1_SFT: c_int = 7;
pub const RG_INT_STATUS_ACCDET_EINT1_MASK: c_uint = 0x1;

pub const RG_INT_RAW_STATUS_ACCDET_SFT: c_int = 5;
pub const RG_INT_RAW_STATUS_ACCDET_MASK: c_uint = 0x1;

pub const RG_INT_RAW_STATUS_ACCDET_EINT0_SFT: c_int = 6;
pub const RG_INT_RAW_STATUS_ACCDET_EINT0_MASK: c_uint = 0x1;

pub const RG_INT_RAW_STATUS_ACCDET_EINT1_SFT: c_int = 7;
pub const RG_INT_RAW_STATUS_ACCDET_EINT1_MASK: c_uint = 0x1;

pub const RG_AUDACCDETMICBIAS0PULLLOW_SFT: c_int = 0;
pub const RG_AUDACCDETMICBIAS0PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETMICBIAS1PULLLOW_SFT: c_int = 1;
pub const RG_AUDACCDETMICBIAS1PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETMICBIAS2PULLLOW_SFT: c_int = 2;
pub const RG_AUDACCDETMICBIAS2PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETVIN1PULLLOW_SFT: c_int = 3;
pub const RG_AUDACCDETVIN1PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETVTHACAL_SFT: c_int = 4;
pub const RG_AUDACCDETVTHACAL_MASK: c_uint = 0x1;

pub const RG_AUDACCDETVTHBCAL_SFT: c_int = 5;
pub const RG_AUDACCDETVTHBCAL_MASK: c_uint = 0x1;

pub const RG_AUDACCDETTVDET_SFT: c_int = 6;
pub const RG_AUDACCDETTVDET_MASK: c_uint = 0x1;

pub const RG_ACCDETSEL_SFT: c_int = 7;
pub const RG_ACCDETSEL_MASK: c_uint = 0x1;

pub const RG_AUDPWDBMICBIAS1_SFT: c_int = 0;
pub const RG_AUDPWDBMICBIAS1_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1BYPASSEN_SFT: c_int = 1;
pub const RG_AUDMICBIAS1BYPASSEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1LOWPEN_SFT: c_int = 2;
pub const RG_AUDMICBIAS1LOWPEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1VREF_SFT: c_int = 4;
pub const RG_AUDMICBIAS1VREF_MASK: c_uint = 0x7;

pub const RG_AUDMICBIAS1DCSW1PEN_SFT: c_int = 8;
pub const RG_AUDMICBIAS1DCSW1PEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1DCSW1NEN_SFT: c_int = 9;
pub const RG_AUDMICBIAS1DCSW1NEN_MASK: c_uint = 0x1;

pub const RG_BANDGAPGEN_SFT: c_int = 10;
pub const RG_BANDGAPGEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1HVEN_SFT: c_int = 12;
pub const RG_AUDMICBIAS1HVEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1HVVREF_SFT: c_int = 13;
pub const RG_AUDMICBIAS1HVVREF_MASK: c_uint = 0x1;

pub const RG_EINT0NOHYS_SFT: c_int = 10;
pub const RG_EINT0NOHYS_MASK: c_uint = 0x1;

pub const RG_EINT0CONFIGACCDET_SFT: c_int = 11;
pub const RG_EINT0CONFIGACCDET_MASK: c_uint = 0x1;

pub const RG_EINT0HIRENB_SFT: c_int = 12;
pub const RG_EINT0HIRENB_MASK: c_uint = 0x1;

pub const RG_ACCDET2AUXRESBYPASS_SFT: c_int = 13;
pub const RG_ACCDET2AUXRESBYPASS_MASK: c_uint = 0x1;

pub const RG_ACCDET2AUXSWEN_SFT: c_int = 14;
pub const RG_ACCDET2AUXSWEN_MASK: c_uint = 0x1;

pub const RG_AUDACCDETMICBIAS3PULLLOW_SFT: c_int = 15;
pub const RG_AUDACCDETMICBIAS3PULLLOW_MASK: c_uint = 0x1;

pub const RG_EINT1CONFIGACCDET_SFT: c_int = 0;
pub const RG_EINT1CONFIGACCDET_MASK: c_uint = 0x1;

pub const RG_EINT1HIRENB_SFT: c_int = 1;
pub const RG_EINT1HIRENB_MASK: c_uint = 0x1;

pub const RG_EINT1NOHYS_SFT: c_int = 2;
pub const RG_EINT1NOHYS_MASK: c_uint = 0x1;

pub const RG_MTEST_EN_SFT: c_int = 8;
pub const RG_MTEST_EN_MASK: c_uint = 0x1;

pub const RG_MTEST_SEL_SFT: c_int = 9;
pub const RG_MTEST_SEL_MASK: c_uint = 0x1;

pub const RG_MTEST_CURRENT_SFT: c_int = 10;
pub const RG_MTEST_CURRENT_MASK: c_uint = 0x1;

pub const RG_ANALOGFDEN_SFT: c_int = 12;
pub const RG_ANALOGFDEN_MASK: c_uint = 0x1;

pub const RG_FDVIN1PPULLLOW_SFT: c_int = 13;
pub const RG_FDVIN1PPULLLOW_MASK: c_uint = 0x1;

pub const RG_FDEINT0TYPE_SFT: c_int = 14;
pub const RG_FDEINT0TYPE_MASK: c_uint = 0x1;

pub const RG_FDEINT1TYPE_SFT: c_int = 15;
pub const RG_FDEINT1TYPE_MASK: c_uint = 0x1;

pub const RG_EINT0CMPEN_SFT: c_int = 0;
pub const RG_EINT0CMPEN_MASK: c_uint = 0x1;

pub const RG_EINT0CMPMEN_SFT: c_int = 1;
pub const RG_EINT0CMPMEN_MASK: c_uint = 0x1;

pub const RG_EINT0EN_SFT: c_int = 2;
pub const RG_EINT0EN_MASK: c_uint = 0x1;

pub const RG_EINT0CEN_SFT: c_int = 3;
pub const RG_EINT0CEN_MASK: c_uint = 0x1;

pub const RG_EINT0INVEN_SFT: c_int = 4;
pub const RG_EINT0INVEN_MASK: c_uint = 0x1;

pub const RG_EINT0CTURBO_SFT: c_int = 5;
pub const RG_EINT0CTURBO_MASK: c_uint = 0x7;

pub const RG_EINT1CMPEN_SFT: c_int = 8;
pub const RG_EINT1CMPEN_MASK: c_uint = 0x1;

pub const RG_EINT1CMPMEN_SFT: c_int = 9;
pub const RG_EINT1CMPMEN_MASK: c_uint = 0x1;

pub const RG_EINT1EN_SFT: c_int = 10;
pub const RG_EINT1EN_MASK: c_uint = 0x1;

pub const RG_EINT1CEN_SFT: c_int = 11;
pub const RG_EINT1CEN_MASK: c_uint = 0x1;

pub const RG_EINT1INVEN_SFT: c_int = 12;
pub const RG_EINT1INVEN_MASK: c_uint = 0x1;

pub const RG_EINT1CTURBO_SFT: c_int = 13;
pub const RG_EINT1CTURBO_MASK: c_uint = 0x7;

pub const ACCDET_ANA_ID_SFT: c_int = 0;
pub const ACCDET_ANA_ID_MASK: c_uint = 0xFF;

pub const ACCDET_DIG_ID_SFT: c_int = 8;
pub const ACCDET_DIG_ID_MASK: c_uint = 0xFF;

pub const ACCDET_ANA_MINOR_REV_SFT: c_int = 0;
pub const ACCDET_ANA_MINOR_REV_MASK: c_uint = 0xF;

pub const ACCDET_ANA_MAJOR_REV_SFT: c_int = 4;
pub const ACCDET_ANA_MAJOR_REV_MASK: c_uint = 0xF;

pub const ACCDET_DIG_MINOR_REV_SFT: c_int = 8;
pub const ACCDET_DIG_MINOR_REV_MASK: c_uint = 0xF;

pub const ACCDET_DIG_MAJOR_REV_SFT: c_int = 12;
pub const ACCDET_DIG_MAJOR_REV_MASK: c_uint = 0xF;

pub const ACCDET_DSN_CBS_SFT: c_int = 0;
pub const ACCDET_DSN_CBS_MASK: c_uint = 0x3;

pub const ACCDET_DSN_BIX_SFT: c_int = 2;
pub const ACCDET_DSN_BIX_MASK: c_uint = 0x3;

pub const ACCDET_ESP_SFT: c_int = 8;
pub const ACCDET_ESP_MASK: c_uint = 0xFF;

pub const ACCDET_DSN_FPI_SFT: c_int = 0;
pub const ACCDET_DSN_FPI_MASK: c_uint = 0xFF;

pub const ACCDET_AUXADC_SEL_SFT: c_int = 0;
pub const ACCDET_AUXADC_SEL_MASK: c_uint = 0x1;

pub const ACCDET_AUXADC_SW_SFT: c_int = 1;
pub const ACCDET_AUXADC_SW_MASK: c_uint = 0x1;

pub const ACCDET_TEST_AUXADC_SFT: c_int = 2;
pub const ACCDET_TEST_AUXADC_MASK: c_uint = 0x1;

pub const ACCDET_AUXADC_ANASWCTRL_SEL_SFT: c_int = 8;
pub const ACCDET_AUXADC_ANASWCTRL_SEL_MASK: c_uint = 0x1;

pub const AUDACCDETAUXADCSWCTRL_SEL_SFT: c_int = 9;
pub const AUDACCDETAUXADCSWCTRL_SEL_MASK: c_uint = 0x1;

pub const AUDACCDETAUXADCSWCTRL_SW_SFT: c_int = 10;
pub const AUDACCDETAUXADCSWCTRL_SW_MASK: c_uint = 0x1;

pub const ACCDET_TEST_ANA_SFT: c_int = 11;
pub const ACCDET_TEST_ANA_MASK: c_uint = 0x1;

pub const RG_AUDACCDETRSV_SFT: c_int = 13;
pub const RG_AUDACCDETRSV_MASK: c_uint = 0x3;

pub const ACCDET_SW_EN_SFT: c_int = 0;
pub const ACCDET_SW_EN_MASK: c_uint = 0x1;

pub const ACCDET_SEQ_INIT_SFT: c_int = 1;
pub const ACCDET_SEQ_INIT_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_SW_EN_SFT: c_int = 2;
pub const ACCDET_EINT0_SW_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_SEQ_INIT_SFT: c_int = 3;
pub const ACCDET_EINT0_SEQ_INIT_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_SW_EN_SFT: c_int = 4;
pub const ACCDET_EINT1_SW_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_SEQ_INIT_SFT: c_int = 5;
pub const ACCDET_EINT1_SEQ_INIT_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_INVERTER_SW_EN_SFT: c_int = 6;
pub const ACCDET_EINT0_INVERTER_SW_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_INVERTER_SEQ_INIT_SFT: c_int = 7;
pub const ACCDET_EINT0_INVERTER_SEQ_INIT_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_INVERTER_SW_EN_SFT: c_int = 8;
pub const ACCDET_EINT1_INVERTER_SW_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_INVERTER_SEQ_INIT_SFT: c_int = 9;
pub const ACCDET_EINT1_INVERTER_SEQ_INIT_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_M_SW_EN_SFT: c_int = 10;
pub const ACCDET_EINT0_M_SW_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_M_SW_EN_SFT: c_int = 11;
pub const ACCDET_EINT1_M_SW_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_M_DETECT_EN_SFT: c_int = 12;
pub const ACCDET_EINT_M_DETECT_EN_MASK: c_uint = 0x1;

pub const ACCDET_CMP_PWM_EN_SFT: c_int = 0;
pub const ACCDET_CMP_PWM_EN_MASK: c_uint = 0x1;

pub const ACCDET_VTH_PWM_EN_SFT: c_int = 1;
pub const ACCDET_VTH_PWM_EN_MASK: c_uint = 0x1;

pub const ACCDET_MBIAS_PWM_EN_SFT: c_int = 2;
pub const ACCDET_MBIAS_PWM_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_EN_PWM_EN_SFT: c_int = 3;
pub const ACCDET_EINT_EN_PWM_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_CMPEN_PWM_EN_SFT: c_int = 4;
pub const ACCDET_EINT_CMPEN_PWM_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_CMPMEN_PWM_EN_SFT: c_int = 5;
pub const ACCDET_EINT_CMPMEN_PWM_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_CTURBO_PWM_EN_SFT: c_int = 6;
pub const ACCDET_EINT_CTURBO_PWM_EN_MASK: c_uint = 0x1;

pub const ACCDET_CMP_PWM_IDLE_SFT: c_int = 8;
pub const ACCDET_CMP_PWM_IDLE_MASK: c_uint = 0x1;

pub const ACCDET_VTH_PWM_IDLE_SFT: c_int = 9;
pub const ACCDET_VTH_PWM_IDLE_MASK: c_uint = 0x1;

pub const ACCDET_MBIAS_PWM_IDLE_SFT: c_int = 10;
pub const ACCDET_MBIAS_PWM_IDLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CMPEN_PWM_IDLE_SFT: c_int = 11;
pub const ACCDET_EINT0_CMPEN_PWM_IDLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CMPEN_PWM_IDLE_SFT: c_int = 12;
pub const ACCDET_EINT1_CMPEN_PWM_IDLE_MASK: c_uint = 0x1;

pub const ACCDET_PWM_EN_SW_SFT: c_int = 13;
pub const ACCDET_PWM_EN_SW_MASK: c_uint = 0x1;

pub const ACCDET_PWM_EN_SEL_SFT: c_int = 14;
pub const ACCDET_PWM_EN_SEL_MASK: c_uint = 0x3;

pub const ACCDET_PWM_WIDTH_SFT: c_int = 0;
pub const ACCDET_PWM_WIDTH_MASK: c_uint = 0xFFFF;

pub const ACCDET_PWM_THRESH_SFT: c_int = 0;
pub const ACCDET_PWM_THRESH_MASK: c_uint = 0xFFFF;

pub const ACCDET_RISE_DELAY_SFT: c_int = 0;
pub const ACCDET_RISE_DELAY_MASK: c_uint = 0x7FFF;

pub const ACCDET_FALL_DELAY_SFT: c_int = 15;
pub const ACCDET_FALL_DELAY_MASK: c_uint = 0x1;

pub const ACCDET_EINT_CMPMEN_PWM_THRESH_SFT: c_int = 0;
pub const ACCDET_EINT_CMPMEN_PWM_THRESH_MASK: c_uint = 0x7;

pub const ACCDET_EINT_CMPMEN_PWM_WIDTH_SFT: c_int = 4;
pub const ACCDET_EINT_CMPMEN_PWM_WIDTH_MASK: c_uint = 0x7;

pub const ACCDET_EINT_EN_PWM_THRESH_SFT: c_int = 0;
pub const ACCDET_EINT_EN_PWM_THRESH_MASK: c_uint = 0x7;

pub const ACCDET_EINT_EN_PWM_WIDTH_SFT: c_int = 4;
pub const ACCDET_EINT_EN_PWM_WIDTH_MASK: c_uint = 0x3;

pub const ACCDET_EINT_CMPEN_PWM_THRESH_SFT: c_int = 8;
pub const ACCDET_EINT_CMPEN_PWM_THRESH_MASK: c_uint = 0x7;

pub const ACCDET_EINT_CMPEN_PWM_WIDTH_SFT: c_int = 12;
pub const ACCDET_EINT_CMPEN_PWM_WIDTH_MASK: c_uint = 0x3;

pub const ACCDET_DEBOUNCE0_SFT: c_int = 0;
pub const ACCDET_DEBOUNCE0_MASK: c_uint = 0xFFFF;

pub const ACCDET_DEBOUNCE1_SFT: c_int = 0;
pub const ACCDET_DEBOUNCE1_MASK: c_uint = 0xFFFF;

pub const ACCDET_DEBOUNCE2_SFT: c_int = 0;
pub const ACCDET_DEBOUNCE2_MASK: c_uint = 0xFFFF;

pub const ACCDET_DEBOUNCE3_SFT: c_int = 0;
pub const ACCDET_DEBOUNCE3_MASK: c_uint = 0xFFFF;

pub const ACCDET_CONNECT_AUXADC_TIME_DIG_SFT: c_int = 0;
pub const ACCDET_CONNECT_AUXADC_TIME_DIG_MASK: c_uint = 0xFFFF;

pub const ACCDET_CONNECT_AUXADC_TIME_ANA_SFT: c_int = 0;
pub const ACCDET_CONNECT_AUXADC_TIME_ANA_MASK: c_uint = 0xFFFF;

pub const ACCDET_EINT_DEBOUNCE0_SFT: c_int = 0;
pub const ACCDET_EINT_DEBOUNCE0_MASK: c_uint = 0xF;

pub const ACCDET_EINT_DEBOUNCE1_SFT: c_int = 4;
pub const ACCDET_EINT_DEBOUNCE1_MASK: c_uint = 0xF;

pub const ACCDET_EINT_DEBOUNCE2_SFT: c_int = 8;
pub const ACCDET_EINT_DEBOUNCE2_MASK: c_uint = 0xF;

pub const ACCDET_EINT_DEBOUNCE3_SFT: c_int = 12;
pub const ACCDET_EINT_DEBOUNCE3_MASK: c_uint = 0xF;

pub const ACCDET_EINT_INVERTER_DEBOUNCE_SFT: c_int = 0;
pub const ACCDET_EINT_INVERTER_DEBOUNCE_MASK: c_uint = 0xF;

pub const ACCDET_IVAL_CUR_IN_SFT: c_int = 0;
pub const ACCDET_IVAL_CUR_IN_MASK: c_uint = 0x3;

pub const ACCDET_IVAL_SAM_IN_SFT: c_int = 2;
pub const ACCDET_IVAL_SAM_IN_MASK: c_uint = 0x3;

pub const ACCDET_IVAL_MEM_IN_SFT: c_int = 4;
pub const ACCDET_IVAL_MEM_IN_MASK: c_uint = 0x3;

pub const ACCDET_EINT_IVAL_CUR_IN_SFT: c_int = 6;
pub const ACCDET_EINT_IVAL_CUR_IN_MASK: c_uint = 0x3;

pub const ACCDET_EINT_IVAL_SAM_IN_SFT: c_int = 8;
pub const ACCDET_EINT_IVAL_SAM_IN_MASK: c_uint = 0x3;

pub const ACCDET_EINT_IVAL_MEM_IN_SFT: c_int = 10;
pub const ACCDET_EINT_IVAL_MEM_IN_MASK: c_uint = 0x3;

pub const ACCDET_IVAL_SEL_SFT: c_int = 12;
pub const ACCDET_IVAL_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_IVAL_SEL_SFT: c_int = 13;
pub const ACCDET_EINT_IVAL_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_INVERTER_IVAL_CUR_IN_SFT: c_int = 0;
pub const ACCDET_EINT_INVERTER_IVAL_CUR_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_INVERTER_IVAL_SAM_IN_SFT: c_int = 1;
pub const ACCDET_EINT_INVERTER_IVAL_SAM_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_INVERTER_IVAL_MEM_IN_SFT: c_int = 2;
pub const ACCDET_EINT_INVERTER_IVAL_MEM_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_INVERTER_IVAL_SEL_SFT: c_int = 3;
pub const ACCDET_EINT_INVERTER_IVAL_SEL_MASK: c_uint = 0x1;

pub const ACCDET_IRQ_SFT: c_int = 0;
pub const ACCDET_IRQ_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_IRQ_SFT: c_int = 2;
pub const ACCDET_EINT0_IRQ_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_IRQ_SFT: c_int = 3;
pub const ACCDET_EINT1_IRQ_MASK: c_uint = 0x1;

pub const ACCDET_EINT_IN_INVERSE_SFT: c_int = 4;
pub const ACCDET_EINT_IN_INVERSE_MASK: c_uint = 0x1;

pub const ACCDET_IRQ_CLR_SFT: c_int = 8;
pub const ACCDET_IRQ_CLR_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_IRQ_CLR_SFT: c_int = 10;
pub const ACCDET_EINT0_IRQ_CLR_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_IRQ_CLR_SFT: c_int = 11;
pub const ACCDET_EINT1_IRQ_CLR_MASK: c_uint = 0x1;

pub const ACCDET_EINT_M_PLUG_IN_NUM_SFT: c_int = 12;
pub const ACCDET_EINT_M_PLUG_IN_NUM_MASK: c_uint = 0x7;

pub const ACCDET_DA_STABLE_SFT: c_int = 0;
pub const ACCDET_DA_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_EN_STABLE_SFT: c_int = 1;
pub const ACCDET_EINT0_EN_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CMPEN_STABLE_SFT: c_int = 2;
pub const ACCDET_EINT0_CMPEN_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CMPMEN_STABLE_SFT: c_int = 3;
pub const ACCDET_EINT0_CMPMEN_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CTURBO_STABLE_SFT: c_int = 4;
pub const ACCDET_EINT0_CTURBO_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CEN_STABLE_SFT: c_int = 5;
pub const ACCDET_EINT0_CEN_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_EN_STABLE_SFT: c_int = 6;
pub const ACCDET_EINT1_EN_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CMPEN_STABLE_SFT: c_int = 7;
pub const ACCDET_EINT1_CMPEN_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CMPMEN_STABLE_SFT: c_int = 8;
pub const ACCDET_EINT1_CMPMEN_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CTURBO_STABLE_SFT: c_int = 9;
pub const ACCDET_EINT1_CTURBO_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CEN_STABLE_SFT: c_int = 10;
pub const ACCDET_EINT1_CEN_STABLE_MASK: c_uint = 0x1;

pub const ACCDET_HWMODE_EN_SFT: c_int = 0;
pub const ACCDET_HWMODE_EN_MASK: c_uint = 0x1;

pub const ACCDET_HWMODE_SEL_SFT: c_int = 1;
pub const ACCDET_HWMODE_SEL_MASK: c_uint = 0x3;

pub const ACCDET_PLUG_OUT_DETECT_SFT: c_int = 3;
pub const ACCDET_PLUG_OUT_DETECT_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_REVERSE_SFT: c_int = 4;
pub const ACCDET_EINT0_REVERSE_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_REVERSE_SFT: c_int = 5;
pub const ACCDET_EINT1_REVERSE_MASK: c_uint = 0x1;

pub const ACCDET_EINT_HWMODE_EN_SFT: c_int = 8;
pub const ACCDET_EINT_HWMODE_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_PLUG_OUT_BYPASS_DEB_SFT: c_int = 9;
pub const ACCDET_EINT_PLUG_OUT_BYPASS_DEB_MASK: c_uint = 0x1;

pub const ACCDET_EINT_M_PLUG_IN_EN_SFT: c_int = 10;
pub const ACCDET_EINT_M_PLUG_IN_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_M_HWMODE_EN_SFT: c_int = 11;
pub const ACCDET_EINT_M_HWMODE_EN_MASK: c_uint = 0x1;

pub const ACCDET_TEST_CMPEN_SFT: c_int = 0;
pub const ACCDET_TEST_CMPEN_MASK: c_uint = 0x1;

pub const ACCDET_TEST_VTHEN_SFT: c_int = 1;
pub const ACCDET_TEST_VTHEN_MASK: c_uint = 0x1;

pub const ACCDET_TEST_MBIASEN_SFT: c_int = 2;
pub const ACCDET_TEST_MBIASEN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_TEST_EN_SFT: c_int = 3;
pub const ACCDET_EINT_TEST_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_TEST_INVEN_SFT: c_int = 4;
pub const ACCDET_EINT_TEST_INVEN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_TEST_CMPEN_SFT: c_int = 5;
pub const ACCDET_EINT_TEST_CMPEN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_TEST_CMPMEN_SFT: c_int = 6;
pub const ACCDET_EINT_TEST_CMPMEN_MASK: c_uint = 0x1;

pub const ACCDET_EINT_TEST_CTURBO_SFT: c_int = 7;
pub const ACCDET_EINT_TEST_CTURBO_MASK: c_uint = 0x1;

pub const ACCDET_EINT_TEST_CEN_SFT: c_int = 8;
pub const ACCDET_EINT_TEST_CEN_MASK: c_uint = 0x1;

pub const ACCDET_TEST_B_SFT: c_int = 9;
pub const ACCDET_TEST_B_MASK: c_uint = 0x1;

pub const ACCDET_TEST_A_SFT: c_int = 10;
pub const ACCDET_TEST_A_MASK: c_uint = 0x1;

pub const ACCDET_EINT_TEST_CMPOUT_SFT: c_int = 11;
pub const ACCDET_EINT_TEST_CMPOUT_MASK: c_uint = 0x1;

pub const ACCDET_EINT_TEST_CMPMOUT_SFT: c_int = 12;
pub const ACCDET_EINT_TEST_CMPMOUT_MASK: c_uint = 0x1;

pub const ACCDET_EINT_TEST_INVOUT_SFT: c_int = 13;
pub const ACCDET_EINT_TEST_INVOUT_MASK: c_uint = 0x1;

pub const ACCDET_CMPEN_SEL_SFT: c_int = 0;
pub const ACCDET_CMPEN_SEL_MASK: c_uint = 0x1;

pub const ACCDET_VTHEN_SEL_SFT: c_int = 1;
pub const ACCDET_VTHEN_SEL_MASK: c_uint = 0x1;

pub const ACCDET_MBIASEN_SEL_SFT: c_int = 2;
pub const ACCDET_MBIASEN_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_EN_SEL_SFT: c_int = 3;
pub const ACCDET_EINT_EN_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_INVEN_SEL_SFT: c_int = 4;
pub const ACCDET_EINT_INVEN_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_CMPEN_SEL_SFT: c_int = 5;
pub const ACCDET_EINT_CMPEN_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_CMPMEN_SEL_SFT: c_int = 6;
pub const ACCDET_EINT_CMPMEN_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_CTURBO_SEL_SFT: c_int = 7;
pub const ACCDET_EINT_CTURBO_SEL_MASK: c_uint = 0x1;

pub const ACCDET_B_SEL_SFT: c_int = 9;
pub const ACCDET_B_SEL_MASK: c_uint = 0x1;

pub const ACCDET_A_SEL_SFT: c_int = 10;
pub const ACCDET_A_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_CMPOUT_SEL_SFT: c_int = 11;
pub const ACCDET_EINT_CMPOUT_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_CMPMOUT_SEL_SFT: c_int = 12;
pub const ACCDET_EINT_CMPMOUT_SEL_MASK: c_uint = 0x1;

pub const ACCDET_EINT_INVOUT_SEL_SFT: c_int = 13;
pub const ACCDET_EINT_INVOUT_SEL_MASK: c_uint = 0x1;

pub const ACCDET_CMPEN_SW_SFT: c_int = 0;
pub const ACCDET_CMPEN_SW_MASK: c_uint = 0x1;

pub const ACCDET_VTHEN_SW_SFT: c_int = 1;
pub const ACCDET_VTHEN_SW_MASK: c_uint = 0x1;

pub const ACCDET_MBIASEN_SW_SFT: c_int = 2;
pub const ACCDET_MBIASEN_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_EN_SW_SFT: c_int = 3;
pub const ACCDET_EINT0_EN_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_INVEN_SW_SFT: c_int = 4;
pub const ACCDET_EINT0_INVEN_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CMPEN_SW_SFT: c_int = 5;
pub const ACCDET_EINT0_CMPEN_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CMPMEN_SW_SFT: c_int = 6;
pub const ACCDET_EINT0_CMPMEN_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CTURBO_SW_SFT: c_int = 7;
pub const ACCDET_EINT0_CTURBO_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_EN_SW_SFT: c_int = 8;
pub const ACCDET_EINT1_EN_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_INVEN_SW_SFT: c_int = 9;
pub const ACCDET_EINT1_INVEN_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CMPEN_SW_SFT: c_int = 10;
pub const ACCDET_EINT1_CMPEN_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CMPMEN_SW_SFT: c_int = 11;
pub const ACCDET_EINT1_CMPMEN_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CTURBO_SW_SFT: c_int = 12;
pub const ACCDET_EINT1_CTURBO_SW_MASK: c_uint = 0x1;

pub const ACCDET_B_SW_SFT: c_int = 0;
pub const ACCDET_B_SW_MASK: c_uint = 0x1;

pub const ACCDET_A_SW_SFT: c_int = 1;
pub const ACCDET_A_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CMPOUT_SW_SFT: c_int = 2;
pub const ACCDET_EINT0_CMPOUT_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CMPMOUT_SW_SFT: c_int = 3;
pub const ACCDET_EINT0_CMPMOUT_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_INVOUT_SW_SFT: c_int = 4;
pub const ACCDET_EINT0_INVOUT_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CMPOUT_SW_SFT: c_int = 5;
pub const ACCDET_EINT1_CMPOUT_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CMPMOUT_SW_SFT: c_int = 6;
pub const ACCDET_EINT1_CMPMOUT_SW_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_INVOUT_SW_SFT: c_int = 7;
pub const ACCDET_EINT1_INVOUT_SW_MASK: c_uint = 0x1;

pub const AD_AUDACCDETCMPOB_SFT: c_int = 0;
pub const AD_AUDACCDETCMPOB_MASK: c_uint = 0x1;

pub const AD_AUDACCDETCMPOA_SFT: c_int = 1;
pub const AD_AUDACCDETCMPOA_MASK: c_uint = 0x1;

pub const ACCDET_CUR_IN_SFT: c_int = 2;
pub const ACCDET_CUR_IN_MASK: c_uint = 0x3;

pub const ACCDET_SAM_IN_SFT: c_int = 4;
pub const ACCDET_SAM_IN_MASK: c_uint = 0x3;

pub const ACCDET_MEM_IN_SFT: c_int = 6;
pub const ACCDET_MEM_IN_MASK: c_uint = 0x3;

pub const ACCDET_STATE_SFT: c_int = 8;
pub const ACCDET_STATE_MASK: c_uint = 0x7;

pub const DA_AUDACCDETMBIASCLK_SFT: c_int = 12;
pub const DA_AUDACCDETMBIASCLK_MASK: c_uint = 0x1;

pub const DA_AUDACCDETVTHCLK_SFT: c_int = 13;
pub const DA_AUDACCDETVTHCLK_MASK: c_uint = 0x1;

pub const DA_AUDACCDETCMPCLK_SFT: c_int = 14;
pub const DA_AUDACCDETCMPCLK_MASK: c_uint = 0x1;

pub const DA_AUDACCDETAUXADCSWCTRL_SFT: c_int = 15;
pub const DA_AUDACCDETAUXADCSWCTRL_MASK: c_uint = 0x1;

pub const AD_EINT0CMPMOUT_SFT: c_int = 0;
pub const AD_EINT0CMPMOUT_MASK: c_uint = 0x1;

pub const AD_EINT0CMPOUT_SFT: c_int = 1;
pub const AD_EINT0CMPOUT_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_CUR_IN_SFT: c_int = 2;
pub const ACCDET_EINT0_CUR_IN_MASK: c_uint = 0x3;

pub const ACCDET_EINT0_SAM_IN_SFT: c_int = 4;
pub const ACCDET_EINT0_SAM_IN_MASK: c_uint = 0x3;

pub const ACCDET_EINT0_MEM_IN_SFT: c_int = 6;
pub const ACCDET_EINT0_MEM_IN_MASK: c_uint = 0x3;

pub const ACCDET_EINT0_STATE_SFT: c_int = 8;
pub const ACCDET_EINT0_STATE_MASK: c_uint = 0x7;

pub const DA_EINT0CMPEN_SFT: c_int = 13;
pub const DA_EINT0CMPEN_MASK: c_uint = 0x1;

pub const DA_EINT0CMPMEN_SFT: c_int = 14;
pub const DA_EINT0CMPMEN_MASK: c_uint = 0x1;

pub const DA_EINT0CTURBO_SFT: c_int = 15;
pub const DA_EINT0CTURBO_MASK: c_uint = 0x1;

pub const AD_EINT1CMPMOUT_SFT: c_int = 0;
pub const AD_EINT1CMPMOUT_MASK: c_uint = 0x1;

pub const AD_EINT1CMPOUT_SFT: c_int = 1;
pub const AD_EINT1CMPOUT_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_CUR_IN_SFT: c_int = 2;
pub const ACCDET_EINT1_CUR_IN_MASK: c_uint = 0x3;

pub const ACCDET_EINT1_SAM_IN_SFT: c_int = 4;
pub const ACCDET_EINT1_SAM_IN_MASK: c_uint = 0x3;

pub const ACCDET_EINT1_MEM_IN_SFT: c_int = 6;
pub const ACCDET_EINT1_MEM_IN_MASK: c_uint = 0x3;

pub const ACCDET_EINT1_STATE_SFT: c_int = 8;
pub const ACCDET_EINT1_STATE_MASK: c_uint = 0x7;

pub const DA_EINT1CMPEN_SFT: c_int = 13;
pub const DA_EINT1CMPEN_MASK: c_uint = 0x1;

pub const DA_EINT1CMPMEN_SFT: c_int = 14;
pub const DA_EINT1CMPMEN_MASK: c_uint = 0x1;

pub const DA_EINT1CTURBO_SFT: c_int = 15;
pub const DA_EINT1CTURBO_MASK: c_uint = 0x1;

pub const AD_EINT0INVOUT_SFT: c_int = 0;
pub const AD_EINT0INVOUT_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_INVERTER_CUR_IN_SFT: c_int = 1;
pub const ACCDET_EINT0_INVERTER_CUR_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_INVERTER_SAM_IN_SFT: c_int = 2;
pub const ACCDET_EINT0_INVERTER_SAM_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_INVERTER_MEM_IN_SFT: c_int = 3;
pub const ACCDET_EINT0_INVERTER_MEM_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_INVERTER_STATE_SFT: c_int = 8;
pub const ACCDET_EINT0_INVERTER_STATE_MASK: c_uint = 0x7;

pub const DA_EINT0EN_SFT: c_int = 12;
pub const DA_EINT0EN_MASK: c_uint = 0x1;

pub const DA_EINT0INVEN_SFT: c_int = 13;
pub const DA_EINT0INVEN_MASK: c_uint = 0x1;

pub const DA_EINT0CEN_SFT: c_int = 14;
pub const DA_EINT0CEN_MASK: c_uint = 0x1;

pub const AD_EINT1INVOUT_SFT: c_int = 0;
pub const AD_EINT1INVOUT_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_INVERTER_CUR_IN_SFT: c_int = 1;
pub const ACCDET_EINT1_INVERTER_CUR_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_INVERTER_SAM_IN_SFT: c_int = 2;
pub const ACCDET_EINT1_INVERTER_SAM_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_INVERTER_MEM_IN_SFT: c_int = 3;
pub const ACCDET_EINT1_INVERTER_MEM_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_INVERTER_STATE_SFT: c_int = 8;
pub const ACCDET_EINT1_INVERTER_STATE_MASK: c_uint = 0x7;

pub const DA_EINT1EN_SFT: c_int = 12;
pub const DA_EINT1EN_MASK: c_uint = 0x1;

pub const DA_EINT1INVEN_SFT: c_int = 13;
pub const DA_EINT1INVEN_MASK: c_uint = 0x1;

pub const DA_EINT1CEN_SFT: c_int = 14;
pub const DA_EINT1CEN_MASK: c_uint = 0x1;

pub const ACCDET_EN_SFT: c_int = 0;
pub const ACCDET_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_EN_SFT: c_int = 1;
pub const ACCDET_EINT0_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_EN_SFT: c_int = 2;
pub const ACCDET_EINT1_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_M_EN_SFT: c_int = 3;
pub const ACCDET_EINT0_M_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_DETECT_MOISTURE_SFT: c_int = 4;
pub const ACCDET_EINT0_DETECT_MOISTURE_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_PLUG_IN_SFT: c_int = 5;
pub const ACCDET_EINT0_PLUG_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_M_PLUG_IN_SFT: c_int = 6;
pub const ACCDET_EINT0_M_PLUG_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_M_EN_SFT: c_int = 7;
pub const ACCDET_EINT1_M_EN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_DETECT_MOISTURE_SFT: c_int = 8;
pub const ACCDET_EINT1_DETECT_MOISTURE_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_PLUG_IN_SFT: c_int = 9;
pub const ACCDET_EINT1_PLUG_IN_MASK: c_uint = 0x1;

pub const ACCDET_EINT1_M_PLUG_IN_SFT: c_int = 10;
pub const ACCDET_EINT1_M_PLUG_IN_MASK: c_uint = 0x1;

pub const ACCDET_CUR_DEB_SFT: c_int = 0;
pub const ACCDET_CUR_DEB_MASK: c_uint = 0xFFFF;

pub const ACCDET_EINT0_CUR_DEB_SFT: c_int = 0;
pub const ACCDET_EINT0_CUR_DEB_MASK: c_uint = 0x7FFF;

pub const ACCDET_EINT1_CUR_DEB_SFT: c_int = 0;
pub const ACCDET_EINT1_CUR_DEB_MASK: c_uint = 0x7FFF;

pub const ACCDET_EINT0_INVERTER_CUR_DEB_SFT: c_int = 0;
pub const ACCDET_EINT0_INVERTER_CUR_DEB_MASK: c_uint = 0x7FFF;

pub const ACCDET_EINT1_INVERTER_CUR_DEB_SFT: c_int = 0;
pub const ACCDET_EINT1_INVERTER_CUR_DEB_MASK: c_uint = 0x7FFF;

pub const AD_AUDACCDETCMPOB_MON_SFT: c_int = 0;
pub const AD_AUDACCDETCMPOB_MON_MASK: c_uint = 0x1;

pub const AD_AUDACCDETCMPOA_MON_SFT: c_int = 1;
pub const AD_AUDACCDETCMPOA_MON_MASK: c_uint = 0x1;

pub const AD_EINT0CMPMOUT_MON_SFT: c_int = 2;
pub const AD_EINT0CMPMOUT_MON_MASK: c_uint = 0x1;

pub const AD_EINT0CMPOUT_MON_SFT: c_int = 3;
pub const AD_EINT0CMPOUT_MON_MASK: c_uint = 0x1;

pub const AD_EINT0INVOUT_MON_SFT: c_int = 4;
pub const AD_EINT0INVOUT_MON_MASK: c_uint = 0x1;

pub const AD_EINT1CMPMOUT_MON_SFT: c_int = 5;
pub const AD_EINT1CMPMOUT_MON_MASK: c_uint = 0x1;

pub const AD_EINT1CMPOUT_MON_SFT: c_int = 6;
pub const AD_EINT1CMPOUT_MON_MASK: c_uint = 0x1;

pub const AD_EINT1INVOUT_MON_SFT: c_int = 7;
pub const AD_EINT1INVOUT_MON_MASK: c_uint = 0x1;

pub const DA_AUDACCDETCMPCLK_MON_SFT: c_int = 0;
pub const DA_AUDACCDETCMPCLK_MON_MASK: c_uint = 0x1;

pub const DA_AUDACCDETVTHCLK_MON_SFT: c_int = 1;
pub const DA_AUDACCDETVTHCLK_MON_MASK: c_uint = 0x1;

pub const DA_AUDACCDETMBIASCLK_MON_SFT: c_int = 2;
pub const DA_AUDACCDETMBIASCLK_MON_MASK: c_uint = 0x1;

pub const DA_AUDACCDETAUXADCSWCTRL_MON_SFT: c_int = 3;
pub const DA_AUDACCDETAUXADCSWCTRL_MON_MASK: c_uint = 0x1;

pub const DA_EINT0CTURBO_MON_SFT: c_int = 0;
pub const DA_EINT0CTURBO_MON_MASK: c_uint = 0x1;

pub const DA_EINT0CMPMEN_MON_SFT: c_int = 1;
pub const DA_EINT0CMPMEN_MON_MASK: c_uint = 0x1;

pub const DA_EINT0CMPEN_MON_SFT: c_int = 2;
pub const DA_EINT0CMPEN_MON_MASK: c_uint = 0x1;

pub const DA_EINT0INVEN_MON_SFT: c_int = 3;
pub const DA_EINT0INVEN_MON_MASK: c_uint = 0x1;

pub const DA_EINT0CEN_MON_SFT: c_int = 4;
pub const DA_EINT0CEN_MON_MASK: c_uint = 0x1;

pub const DA_EINT0EN_MON_SFT: c_int = 5;
pub const DA_EINT0EN_MON_MASK: c_uint = 0x1;

pub const DA_EINT1CTURBO_MON_SFT: c_int = 8;
pub const DA_EINT1CTURBO_MON_MASK: c_uint = 0x1;

pub const DA_EINT1CMPMEN_MON_SFT: c_int = 9;
pub const DA_EINT1CMPMEN_MON_MASK: c_uint = 0x1;

pub const DA_EINT1CMPEN_MON_SFT: c_int = 10;
pub const DA_EINT1CMPEN_MON_MASK: c_uint = 0x1;

pub const DA_EINT1INVEN_MON_SFT: c_int = 11;
pub const DA_EINT1INVEN_MON_MASK: c_uint = 0x1;

pub const DA_EINT1CEN_MON_SFT: c_int = 12;
pub const DA_EINT1CEN_MON_MASK: c_uint = 0x1;

pub const DA_EINT1EN_MON_SFT: c_int = 13;
pub const DA_EINT1EN_MON_MASK: c_uint = 0x1;

pub const ACCDET_EINT0_M_PLUG_IN_COUNT_SFT: c_int = 0;
pub const ACCDET_EINT0_M_PLUG_IN_COUNT_MASK: c_uint = 0x7;

pub const ACCDET_EINT1_M_PLUG_IN_COUNT_SFT: c_int = 4;
pub const ACCDET_EINT1_M_PLUG_IN_COUNT_MASK: c_uint = 0x7;

pub const ACCDET_MON_FLAG_EN_SFT: c_int = 0;
pub const ACCDET_MON_FLAG_EN_MASK: c_uint = 0x1;

pub const ACCDET_MON_FLAG_SEL_SFT: c_int = 4;
pub const ACCDET_MON_FLAG_SEL_MASK: c_uint = 0xF;

pub const RG_AUDPWDBMICBIAS0_SFT: c_int = 0;
pub const RG_AUDPWDBMICBIAS0_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLON_SFT: c_int = 0;
pub const RG_AUDPREAMPLON_MASK: c_uint = 0x1;

pub const RG_CLKSQ_EN_SFT: c_int = 0;
pub const RG_CLKSQ_EN_MASK: c_uint = 0x1;

pub const RG_RTC32K_CK_PDN_SFT: c_int = 15;
pub const RG_RTC32K_CK_PDN_MASK: c_uint = 0x1;

pub const RG_HPLOUTPUTSTBENH_VAUDP32_SFT: c_int = 0;
pub const RG_HPLOUTPUTSTBENH_VAUDP32_MASK: c_uint = 0x7;

pub const AUXADC_RQST_CH5_SFT: c_int = 5;
pub const AUXADC_RQST_CH5_MASK: c_uint = 0x1;

pub const RG_LDO_VUSB_HW0_OP_EN_SFT: c_int = 0;
pub const RG_LDO_VUSB_HW0_OP_EN_MASK: c_uint = 0x1;

pub const RG_HPROUTPUTSTBENH_VAUDP32_SFT: c_int = 4;
pub const RG_HPROUTPUTSTBENH_VAUDP32_MASK: c_uint = 0x7;

pub const RG_NCP_PDDIS_EN_SFT: c_int = 0;
pub const RG_NCP_PDDIS_EN_MASK: c_uint = 0x1;

pub const RG_SCK32K_CK_PDN_SFT: c_int = 0;
pub const RG_SCK32K_CK_PDN_MASK: c_uint = 0x1;

// AUDENC_ANA_CON18:

// AUXADC_ADC5:  Auxadc CH5 read data

// AUXADC_RQST0_SET:  Auxadc CH5 request, relevant 0x07EC

// AUXADC_RQST0_CLR:  Auxadc CH5 request, relevant 0x07EC

// ACCDET_CON25: RO, accdet FSM state,etc.

// ACCDET_CON19

// The following are used for mt6359.c
// MT6359_DCXO_CW12
pub const RG_XO_AUDIO_EN_M_SFT: c_int = 13;
// AUD_TOP_CKPDN_CON0
pub const RG_VOW13M_CK_PDN_SFT: c_int = 13;
pub const RG_VOW13M_CK_PDN_MASK: c_uint = 0x1;

pub const RG_VOW32K_CK_PDN_SFT: c_int = 12;
pub const RG_VOW32K_CK_PDN_MASK: c_uint = 0x1;

pub const RG_AUD_INTRP_CK_PDN_SFT: c_int = 8;
pub const RG_AUD_INTRP_CK_PDN_MASK: c_uint = 0x1;

pub const RG_PAD_AUD_CLK_MISO_CK_PDN_SFT: c_int = 7;
pub const RG_PAD_AUD_CLK_MISO_CK_PDN_MASK: c_uint = 0x1;

pub const RG_AUDNCP_CK_PDN_SFT: c_int = 6;
pub const RG_AUDNCP_CK_PDN_MASK: c_uint = 0x1;

pub const RG_ZCD13M_CK_PDN_SFT: c_int = 5;
pub const RG_ZCD13M_CK_PDN_MASK: c_uint = 0x1;

pub const RG_AUDIF_CK_PDN_SFT: c_int = 2;
pub const RG_AUDIF_CK_PDN_MASK: c_uint = 0x1;

pub const RG_AUD_CK_PDN_SFT: c_int = 1;
pub const RG_AUD_CK_PDN_MASK: c_uint = 0x1;

pub const RG_ACCDET_CK_PDN_SFT: c_int = 0;
pub const RG_ACCDET_CK_PDN_MASK: c_uint = 0x1;

// AUD_TOP_CKPDN_CON0_SET
pub const RG_AUD_TOP_CKPDN_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_TOP_CKPDN_CON0_SET_MASK: c_uint = 0x3fff;

// AUD_TOP_CKPDN_CON0_CLR
pub const RG_AUD_TOP_CKPDN_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_TOP_CKPDN_CON0_CLR_MASK: c_uint = 0x3fff;

// AUD_TOP_CKSEL_CON0
pub const RG_AUDIF_CK_CKSEL_SFT: c_int = 3;
pub const RG_AUDIF_CK_CKSEL_MASK: c_uint = 0x1;

pub const RG_AUD_CK_CKSEL_SFT: c_int = 2;
pub const RG_AUD_CK_CKSEL_MASK: c_uint = 0x1;

// AUD_TOP_CKSEL_CON0_SET
pub const RG_AUD_TOP_CKSEL_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_TOP_CKSEL_CON0_SET_MASK: c_uint = 0xf;

// AUD_TOP_CKSEL_CON0_CLR
pub const RG_AUD_TOP_CKSEL_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_TOP_CKSEL_CON0_CLR_MASK: c_uint = 0xf;

// AUD_TOP_CKTST_CON0
pub const RG_VOW13M_CK_TSTSEL_SFT: c_int = 9;
pub const RG_VOW13M_CK_TSTSEL_MASK: c_uint = 0x1;

pub const RG_VOW13M_CK_TST_DIS_SFT: c_int = 8;
pub const RG_VOW13M_CK_TST_DIS_MASK: c_uint = 0x1;

pub const RG_AUD26M_CK_TSTSEL_SFT: c_int = 4;
pub const RG_AUD26M_CK_TSTSEL_MASK: c_uint = 0x1;

pub const RG_AUDIF_CK_TSTSEL_SFT: c_int = 3;
pub const RG_AUDIF_CK_TSTSEL_MASK: c_uint = 0x1;

pub const RG_AUD_CK_TSTSEL_SFT: c_int = 2;
pub const RG_AUD_CK_TSTSEL_MASK: c_uint = 0x1;

pub const RG_AUD26M_CK_TST_DIS_SFT: c_int = 0;
pub const RG_AUD26M_CK_TST_DIS_MASK: c_uint = 0x1;

// AUD_TOP_CLK_HWEN_CON0
pub const RG_AUD_INTRP_CK_PDN_HWEN_SFT: c_int = 0;
pub const RG_AUD_INTRP_CK_PDN_HWEN_MASK: c_uint = 0x1;

// AUD_TOP_CLK_HWEN_CON0_SET
pub const RG_AUD_INTRP_CK_PND_HWEN_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_INTRP_CK_PND_HWEN_CON0_SET_MASK: c_uint = 0xffff;

// AUD_TOP_CLK_HWEN_CON0_CLR
pub const RG_AUD_INTRP_CLK_PDN_HWEN_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_INTRP_CLK_PDN_HWEN_CON0_CLR_MASK: c_uint = 0xffff;

// AUD_TOP_RST_CON0
pub const RG_AUDNCP_RST_SFT: c_int = 3;
pub const RG_AUDNCP_RST_MASK: c_uint = 0x1;

pub const RG_ZCD_RST_SFT: c_int = 2;
pub const RG_ZCD_RST_MASK: c_uint = 0x1;

pub const RG_ACCDET_RST_SFT: c_int = 1;
pub const RG_ACCDET_RST_MASK: c_uint = 0x1;

pub const RG_AUDIO_RST_SFT: c_int = 0;
pub const RG_AUDIO_RST_MASK: c_uint = 0x1;

// AUD_TOP_RST_CON0_SET
pub const RG_AUD_TOP_RST_CON0_SET_SFT: c_int = 0;
pub const RG_AUD_TOP_RST_CON0_SET_MASK: c_uint = 0xf;

// AUD_TOP_RST_CON0_CLR
pub const RG_AUD_TOP_RST_CON0_CLR_SFT: c_int = 0;
pub const RG_AUD_TOP_RST_CON0_CLR_MASK: c_uint = 0xf;

// AUD_TOP_RST_BANK_CON0
pub const BANK_AUDZCD_SWRST_SFT: c_int = 2;
pub const BANK_AUDZCD_SWRST_MASK: c_uint = 0x1;

pub const BANK_AUDIO_SWRST_SFT: c_int = 1;
pub const BANK_AUDIO_SWRST_MASK: c_uint = 0x1;

pub const BANK_ACCDET_SWRST_SFT: c_int = 0;
pub const BANK_ACCDET_SWRST_MASK: c_uint = 0x1;

// AFE_UL_DL_CON0
pub const AFE_UL_LR_SWAP_SFT: c_int = 15;
pub const AFE_UL_LR_SWAP_MASK: c_uint = 0x1;

pub const AFE_DL_LR_SWAP_SFT: c_int = 14;
pub const AFE_DL_LR_SWAP_MASK: c_uint = 0x1;

pub const AFE_ON_SFT: c_int = 0;
pub const AFE_ON_MASK: c_uint = 0x1;

// AFE_DL_SRC2_CON0_L
pub const DL_2_SRC_ON_TMP_CTL_PRE_SFT: c_int = 0;
pub const DL_2_SRC_ON_TMP_CTL_PRE_MASK: c_uint = 0x1;

// AFE_UL_SRC_CON0_H
pub const C_DIGMIC_PHASE_SEL_CH1_CTL_SFT: c_int = 11;
pub const C_DIGMIC_PHASE_SEL_CH1_CTL_MASK: c_uint = 0x7;

pub const C_DIGMIC_PHASE_SEL_CH2_CTL_SFT: c_int = 8;
pub const C_DIGMIC_PHASE_SEL_CH2_CTL_MASK: c_uint = 0x7;

pub const C_TWO_DIGITAL_MIC_CTL_SFT: c_int = 7;
pub const C_TWO_DIGITAL_MIC_CTL_MASK: c_uint = 0x1;

// AFE_UL_SRC_CON0_L
pub const DMIC_LOW_POWER_MODE_CTL_SFT: c_int = 14;
pub const DMIC_LOW_POWER_MODE_CTL_MASK: c_uint = 0x3;

pub const DIGMIC_4P33M_SEL_CTL_SFT: c_int = 6;
pub const DIGMIC_4P33M_SEL_CTL_MASK: c_uint = 0x1;

pub const DIGMIC_3P25M_1P625M_SEL_CTL_SFT: c_int = 5;
pub const DIGMIC_3P25M_1P625M_SEL_CTL_MASK: c_uint = 0x1;

pub const UL_LOOP_BACK_MODE_CTL_SFT: c_int = 2;
pub const UL_LOOP_BACK_MODE_CTL_MASK: c_uint = 0x1;

pub const UL_SDM_3_LEVEL_CTL_SFT: c_int = 1;
pub const UL_SDM_3_LEVEL_CTL_MASK: c_uint = 0x1;

pub const UL_SRC_ON_TMP_CTL_SFT: c_int = 0;
pub const UL_SRC_ON_TMP_CTL_MASK: c_uint = 0x1;

// AFE_ADDA6_L_SRC_CON0_H
pub const ADDA6_C_DIGMIC_PHASE_SEL_CH1_CTL_SFT: c_int = 11;
pub const ADDA6_C_DIGMIC_PHASE_SEL_CH1_CTL_MASK: c_uint = 0x7;

pub const ADDA6_C_DIGMIC_PHASE_SEL_CH2_CTL_SFT: c_int = 8;
pub const ADDA6_C_DIGMIC_PHASE_SEL_CH2_CTL_MASK: c_uint = 0x7;

pub const ADDA6_C_TWO_DIGITAL_MIC_CTL_SFT: c_int = 7;
pub const ADDA6_C_TWO_DIGITAL_MIC_CTL_MASK: c_uint = 0x1;

// AFE_ADDA6_UL_SRC_CON0_L
pub const ADDA6_DMIC_LOW_POWER_MODE_CTL_SFT: c_int = 14;
pub const ADDA6_DMIC_LOW_POWER_MODE_CTL_MASK: c_uint = 0x3;

pub const ADDA6_DIGMIC_4P33M_SEL_CTL_SFT: c_int = 6;
pub const ADDA6_DIGMIC_4P33M_SEL_CTL_MASK: c_uint = 0x1;

pub const ADDA6_DIGMIC_3P25M_1P625M_SEL_CTL_SFT: c_int = 5;
pub const ADDA6_DIGMIC_3P25M_1P625M_SEL_CTL_MASK: c_uint = 0x1;

pub const ADDA6_UL_LOOP_BACK_MODE_CTL_SFT: c_int = 2;
pub const ADDA6_UL_LOOP_BACK_MODE_CTL_MASK: c_uint = 0x1;

pub const ADDA6_UL_SDM_3_LEVEL_CTL_SFT: c_int = 1;
pub const ADDA6_UL_SDM_3_LEVEL_CTL_MASK: c_uint = 0x1;

pub const ADDA6_UL_SRC_ON_TMP_CTL_SFT: c_int = 0;
pub const ADDA6_UL_SRC_ON_TMP_CTL_MASK: c_uint = 0x1;

// AFE_TOP_CON0
pub const ADDA6_MTKAIF_SINE_ON_SFT: c_int = 4;
pub const ADDA6_MTKAIF_SINE_ON_MASK: c_uint = 0x1;

pub const ADDA6_UL_SINE_ON_SFT: c_int = 3;
pub const ADDA6_UL_SINE_ON_MASK: c_uint = 0x1;

pub const MTKAIF_SINE_ON_SFT: c_int = 2;
pub const MTKAIF_SINE_ON_MASK: c_uint = 0x1;

pub const UL_SINE_ON_SFT: c_int = 1;
pub const UL_SINE_ON_MASK: c_uint = 0x1;

pub const DL_SINE_ON_SFT: c_int = 0;
pub const DL_SINE_ON_MASK: c_uint = 0x1;

// AUDIO_TOP_CON0
pub const PDN_AFE_CTL_SFT: c_int = 7;
pub const PDN_AFE_CTL_MASK: c_uint = 0x1;

pub const PDN_DAC_CTL_SFT: c_int = 6;
pub const PDN_DAC_CTL_MASK: c_uint = 0x1;

pub const PDN_ADC_CTL_SFT: c_int = 5;
pub const PDN_ADC_CTL_MASK: c_uint = 0x1;

pub const PDN_ADDA6_ADC_CTL_SFT: c_int = 4;
pub const PDN_ADDA6_ADC_CTL_MASK: c_uint = 0x1;

pub const PDN_I2S_DL_CTL_SFT: c_int = 3;
pub const PDN_I2S_DL_CTL_MASK: c_uint = 0x1;

pub const PWR_CLK_DIS_CTL_SFT: c_int = 2;
pub const PWR_CLK_DIS_CTL_MASK: c_uint = 0x1;

pub const PDN_AFE_TESTMODEL_CTL_SFT: c_int = 1;
pub const PDN_AFE_TESTMODEL_CTL_MASK: c_uint = 0x1;

pub const PDN_RESERVED_SFT: c_int = 0;
pub const PDN_RESERVED_MASK: c_uint = 0x1;

// AFE_MON_DEBUG0
pub const AUDIO_SYS_TOP_MON_SWAP_SFT: c_int = 14;
pub const AUDIO_SYS_TOP_MON_SWAP_MASK: c_uint = 0x3;

pub const AUDIO_SYS_TOP_MON_SEL_SFT: c_int = 8;
pub const AUDIO_SYS_TOP_MON_SEL_MASK: c_uint = 0x1f;

pub const AFE_MON_SEL_SFT: c_int = 0;
pub const AFE_MON_SEL_MASK: c_uint = 0xff;

// AFUNC_AUD_CON0
pub const CCI_AUD_ANACK_SEL_SFT: c_int = 15;
pub const CCI_AUD_ANACK_SEL_MASK: c_uint = 0x1;

pub const CCI_AUDIO_FIFO_WPTR_SFT: c_int = 12;
pub const CCI_AUDIO_FIFO_WPTR_MASK: c_uint = 0x7;

pub const CCI_SCRAMBLER_CG_EN_SFT: c_int = 11;
pub const CCI_SCRAMBLER_CG_EN_MASK: c_uint = 0x1;

pub const CCI_LCH_INV_SFT: c_int = 10;
pub const CCI_LCH_INV_MASK: c_uint = 0x1;

pub const CCI_RAND_EN_SFT: c_int = 9;
pub const CCI_RAND_EN_MASK: c_uint = 0x1;

pub const CCI_SPLT_SCRMB_CLK_ON_SFT: c_int = 8;
pub const CCI_SPLT_SCRMB_CLK_ON_MASK: c_uint = 0x1;

pub const CCI_SPLT_SCRMB_ON_SFT: c_int = 7;
pub const CCI_SPLT_SCRMB_ON_MASK: c_uint = 0x1;

pub const CCI_AUD_IDAC_TEST_EN_SFT: c_int = 6;
pub const CCI_AUD_IDAC_TEST_EN_MASK: c_uint = 0x1;

pub const CCI_ZERO_PAD_DISABLE_SFT: c_int = 5;
pub const CCI_ZERO_PAD_DISABLE_MASK: c_uint = 0x1;

pub const CCI_AUD_SPLIT_TEST_EN_SFT: c_int = 4;
pub const CCI_AUD_SPLIT_TEST_EN_MASK: c_uint = 0x1;

pub const CCI_AUD_SDM_MUTEL_SFT: c_int = 3;
pub const CCI_AUD_SDM_MUTEL_MASK: c_uint = 0x1;

pub const CCI_AUD_SDM_MUTER_SFT: c_int = 2;
pub const CCI_AUD_SDM_MUTER_MASK: c_uint = 0x1;

pub const CCI_AUD_SDM_7BIT_SEL_SFT: c_int = 1;
pub const CCI_AUD_SDM_7BIT_SEL_MASK: c_uint = 0x1;

pub const CCI_SCRAMBLER_EN_SFT: c_int = 0;
pub const CCI_SCRAMBLER_EN_MASK: c_uint = 0x1;

// AFUNC_AUD_CON1
pub const AUD_SDM_TEST_L_SFT: c_int = 8;
pub const AUD_SDM_TEST_L_MASK: c_uint = 0xff;

pub const AUD_SDM_TEST_R_SFT: c_int = 0;
pub const AUD_SDM_TEST_R_MASK: c_uint = 0xff;

// AFUNC_AUD_CON2
pub const CCI_AUD_DAC_ANA_MUTE_SFT: c_int = 7;
pub const CCI_AUD_DAC_ANA_MUTE_MASK: c_uint = 0x1;

pub const CCI_AUD_DAC_ANA_RSTB_SEL_SFT: c_int = 6;
pub const CCI_AUD_DAC_ANA_RSTB_SEL_MASK: c_uint = 0x1;

pub const CCI_AUDIO_FIFO_CLKIN_INV_SFT: c_int = 4;
pub const CCI_AUDIO_FIFO_CLKIN_INV_MASK: c_uint = 0x1;

pub const CCI_AUDIO_FIFO_ENABLE_SFT: c_int = 3;
pub const CCI_AUDIO_FIFO_ENABLE_MASK: c_uint = 0x1;

pub const CCI_ACD_MODE_SFT: c_int = 2;
pub const CCI_ACD_MODE_MASK: c_uint = 0x1;

pub const CCI_AFIFO_CLK_PWDB_SFT: c_int = 1;
pub const CCI_AFIFO_CLK_PWDB_MASK: c_uint = 0x1;

pub const CCI_ACD_FUNC_RSTB_SFT: c_int = 0;
pub const CCI_ACD_FUNC_RSTB_MASK: c_uint = 0x1;

// AFUNC_AUD_CON3
pub const SDM_ANA13M_TESTCK_SEL_SFT: c_int = 15;
pub const SDM_ANA13M_TESTCK_SEL_MASK: c_uint = 0x1;

pub const SDM_ANA13M_TESTCK_SRC_SEL_SFT: c_int = 12;
pub const SDM_ANA13M_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

pub const SDM_TESTCK_SRC_SEL_SFT: c_int = 8;
pub const SDM_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

pub const DIGMIC_TESTCK_SRC_SEL_SFT: c_int = 4;
pub const DIGMIC_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

pub const DIGMIC_TESTCK_SEL_SFT: c_int = 0;
pub const DIGMIC_TESTCK_SEL_MASK: c_uint = 0x1;

// AFUNC_AUD_CON4
pub const UL_FIFO_WCLK_INV_SFT: c_int = 8;
pub const UL_FIFO_WCLK_INV_MASK: c_uint = 0x1;

pub const UL_FIFO_DIGMIC_WDATA_TESTSRC_SEL_SFT: c_int = 6;
pub const UL_FIFO_DIGMIC_WDATA_TESTSRC_SEL_MASK: c_uint = 0x1;

pub const UL_FIFO_WDATA_TESTEN_SFT: c_int = 5;
pub const UL_FIFO_WDATA_TESTEN_MASK: c_uint = 0x1;

pub const UL_FIFO_WDATA_TESTSRC_SEL_SFT: c_int = 4;
pub const UL_FIFO_WDATA_TESTSRC_SEL_MASK: c_uint = 0x1;

pub const UL_FIFO_WCLK_6P5M_TESTCK_SEL_SFT: c_int = 3;
pub const UL_FIFO_WCLK_6P5M_TESTCK_SEL_MASK: c_uint = 0x1;

pub const UL_FIFO_WCLK_6P5M_TESTCK_SRC_SEL_SFT: c_int = 0;
pub const UL_FIFO_WCLK_6P5M_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

// AFUNC_AUD_CON5
pub const R_AUD_DAC_POS_LARGE_MONO_SFT: c_int = 8;
pub const R_AUD_DAC_POS_LARGE_MONO_MASK: c_uint = 0xff;

pub const R_AUD_DAC_NEG_LARGE_MONO_SFT: c_int = 0;
pub const R_AUD_DAC_NEG_LARGE_MONO_MASK: c_uint = 0xff;

// AFUNC_AUD_CON6
pub const R_AUD_DAC_POS_SMALL_MONO_SFT: c_int = 12;
pub const R_AUD_DAC_POS_SMALL_MONO_MASK: c_uint = 0xf;

pub const R_AUD_DAC_NEG_SMALL_MONO_SFT: c_int = 8;
pub const R_AUD_DAC_NEG_SMALL_MONO_MASK: c_uint = 0xf;

pub const R_AUD_DAC_POS_TINY_MONO_SFT: c_int = 6;
pub const R_AUD_DAC_POS_TINY_MONO_MASK: c_uint = 0x3;

pub const R_AUD_DAC_NEG_TINY_MONO_SFT: c_int = 4;
pub const R_AUD_DAC_NEG_TINY_MONO_MASK: c_uint = 0x3;

pub const R_AUD_DAC_MONO_SEL_SFT: c_int = 3;
pub const R_AUD_DAC_MONO_SEL_MASK: c_uint = 0x1;

pub const R_AUD_DAC_3TH_SEL_SFT: c_int = 1;
pub const R_AUD_DAC_3TH_SEL_MASK: c_uint = 0x1;

pub const R_AUD_DAC_SW_RSTB_SFT: c_int = 0;
pub const R_AUD_DAC_SW_RSTB_MASK: c_uint = 0x1;

// AFUNC_AUD_CON7
pub const UL2_DIGMIC_TESTCK_SRC_SEL_SFT: c_int = 10;
pub const UL2_DIGMIC_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

pub const UL2_DIGMIC_TESTCK_SEL_SFT: c_int = 9;
pub const UL2_DIGMIC_TESTCK_SEL_MASK: c_uint = 0x1;

pub const UL2_FIFO_WCLK_INV_SFT: c_int = 8;
pub const UL2_FIFO_WCLK_INV_MASK: c_uint = 0x1;

pub const UL2_FIFO_DIGMIC_WDATA_TESTSRC_SEL_SFT: c_int = 6;
pub const UL2_FIFO_DIGMIC_WDATA_TESTSRC_SEL_MASK: c_uint = 0x1;

pub const UL2_FIFO_WDATA_TESTEN_SFT: c_int = 5;
pub const UL2_FIFO_WDATA_TESTEN_MASK: c_uint = 0x1;

pub const UL2_FIFO_WDATA_TESTSRC_SEL_SFT: c_int = 4;
pub const UL2_FIFO_WDATA_TESTSRC_SEL_MASK: c_uint = 0x1;

pub const UL2_FIFO_WCLK_6P5M_TESTCK_SEL_SFT: c_int = 3;
pub const UL2_FIFO_WCLK_6P5M_TESTCK_SEL_MASK: c_uint = 0x1;

pub const UL2_FIFO_WCLK_6P5M_TESTCK_SRC_SEL_SFT: c_int = 0;
pub const UL2_FIFO_WCLK_6P5M_TESTCK_SRC_SEL_MASK: c_uint = 0x7;

// AFUNC_AUD_CON8
pub const SPLITTER2_DITHER_EN_SFT: c_int = 9;
pub const SPLITTER2_DITHER_EN_MASK: c_uint = 0x1;

pub const SPLITTER1_DITHER_EN_SFT: c_int = 8;
pub const SPLITTER1_DITHER_EN_MASK: c_uint = 0x1;

pub const SPLITTER2_DITHER_GAIN_SFT: c_int = 4;
pub const SPLITTER2_DITHER_GAIN_MASK: c_uint = 0xf;

pub const SPLITTER1_DITHER_GAIN_SFT: c_int = 0;
pub const SPLITTER1_DITHER_GAIN_MASK: c_uint = 0xf;

// AFUNC_AUD_CON9
pub const CCI_AUD_ANACK_SEL_2ND_SFT: c_int = 15;
pub const CCI_AUD_ANACK_SEL_2ND_MASK: c_uint = 0x1;

pub const CCI_AUDIO_FIFO_WPTR_2ND_SFT: c_int = 12;
pub const CCI_AUDIO_FIFO_WPTR_2ND_MASK: c_uint = 0x7;

pub const CCI_SCRAMBLER_CG_EN_2ND_SFT: c_int = 11;
pub const CCI_SCRAMBLER_CG_EN_2ND_MASK: c_uint = 0x1;

pub const CCI_LCH_INV_2ND_SFT: c_int = 10;
pub const CCI_LCH_INV_2ND_MASK: c_uint = 0x1;

pub const CCI_RAND_EN_2ND_SFT: c_int = 9;
pub const CCI_RAND_EN_2ND_MASK: c_uint = 0x1;

pub const CCI_SPLT_SCRMB_CLK_ON_2ND_SFT: c_int = 8;
pub const CCI_SPLT_SCRMB_CLK_ON_2ND_MASK: c_uint = 0x1;

pub const CCI_SPLT_SCRMB_ON_2ND_SFT: c_int = 7;
pub const CCI_SPLT_SCRMB_ON_2ND_MASK: c_uint = 0x1;

pub const CCI_AUD_IDAC_TEST_EN_2ND_SFT: c_int = 6;
pub const CCI_AUD_IDAC_TEST_EN_2ND_MASK: c_uint = 0x1;

pub const CCI_ZERO_PAD_DISABLE_2ND_SFT: c_int = 5;
pub const CCI_ZERO_PAD_DISABLE_2ND_MASK: c_uint = 0x1;

pub const CCI_AUD_SPLIT_TEST_EN_2ND_SFT: c_int = 4;
pub const CCI_AUD_SPLIT_TEST_EN_2ND_MASK: c_uint = 0x1;

pub const CCI_AUD_SDM_MUTEL_2ND_SFT: c_int = 3;
pub const CCI_AUD_SDM_MUTEL_2ND_MASK: c_uint = 0x1;

pub const CCI_AUD_SDM_MUTER_2ND_SFT: c_int = 2;
pub const CCI_AUD_SDM_MUTER_2ND_MASK: c_uint = 0x1;

pub const CCI_AUD_SDM_7BIT_SEL_2ND_SFT: c_int = 1;
pub const CCI_AUD_SDM_7BIT_SEL_2ND_MASK: c_uint = 0x1;

pub const CCI_SCRAMBLER_EN_2ND_SFT: c_int = 0;
pub const CCI_SCRAMBLER_EN_2ND_MASK: c_uint = 0x1;

// AFUNC_AUD_CON10
pub const AUD_SDM_TEST_L_2ND_SFT: c_int = 8;
pub const AUD_SDM_TEST_L_2ND_MASK: c_uint = 0xff;

pub const AUD_SDM_TEST_R_2ND_SFT: c_int = 0;
pub const AUD_SDM_TEST_R_2ND_MASK: c_uint = 0xff;

// AFUNC_AUD_CON11
pub const CCI_AUD_DAC_ANA_MUTE_2ND_SFT: c_int = 7;
pub const CCI_AUD_DAC_ANA_MUTE_2ND_MASK: c_uint = 0x1;

pub const CCI_AUD_DAC_ANA_RSTB_SEL_2ND_SFT: c_int = 6;
pub const CCI_AUD_DAC_ANA_RSTB_SEL_2ND_MASK: c_uint = 0x1;

pub const CCI_AUDIO_FIFO_CLKIN_INV_2ND_SFT: c_int = 4;
pub const CCI_AUDIO_FIFO_CLKIN_INV_2ND_MASK: c_uint = 0x1;

pub const CCI_AUDIO_FIFO_ENABLE_2ND_SFT: c_int = 3;
pub const CCI_AUDIO_FIFO_ENABLE_2ND_MASK: c_uint = 0x1;

pub const CCI_ACD_MODE_2ND_SFT: c_int = 2;
pub const CCI_ACD_MODE_2ND_MASK: c_uint = 0x1;

pub const CCI_AFIFO_CLK_PWDB_2ND_SFT: c_int = 1;
pub const CCI_AFIFO_CLK_PWDB_2ND_MASK: c_uint = 0x1;

pub const CCI_ACD_FUNC_RSTB_2ND_SFT: c_int = 0;
pub const CCI_ACD_FUNC_RSTB_2ND_MASK: c_uint = 0x1;

// AFUNC_AUD_CON12
pub const SPLITTER2_DITHER_EN_2ND_SFT: c_int = 9;
pub const SPLITTER2_DITHER_EN_2ND_MASK: c_uint = 0x1;

pub const SPLITTER1_DITHER_EN_2ND_SFT: c_int = 8;
pub const SPLITTER1_DITHER_EN_2ND_MASK: c_uint = 0x1;

pub const SPLITTER2_DITHER_GAIN_2ND_SFT: c_int = 4;
pub const SPLITTER2_DITHER_GAIN_2ND_MASK: c_uint = 0xf;

pub const SPLITTER1_DITHER_GAIN_2ND_SFT: c_int = 0;
pub const SPLITTER1_DITHER_GAIN_2ND_MASK: c_uint = 0xf;

// AFUNC_AUD_MON0
pub const AUD_SCR_OUT_L_SFT: c_int = 8;
pub const AUD_SCR_OUT_L_MASK: c_uint = 0xff;

pub const AUD_SCR_OUT_R_SFT: c_int = 0;
pub const AUD_SCR_OUT_R_MASK: c_uint = 0xff;

// AFUNC_AUD_MON1
pub const AUD_SCR_OUT_L_2ND_SFT: c_int = 8;
pub const AUD_SCR_OUT_L_2ND_MASK: c_uint = 0xff;

pub const AUD_SCR_OUT_R_2ND_SFT: c_int = 0;
pub const AUD_SCR_OUT_R_2ND_MASK: c_uint = 0xff;

// AUDRC_TUNE_MON0
pub const ASYNC_TEST_OUT_BCK_SFT: c_int = 15;
pub const ASYNC_TEST_OUT_BCK_MASK: c_uint = 0x1;

pub const RGS_AUDRCTUNE1READ_SFT: c_int = 8;
pub const RGS_AUDRCTUNE1READ_MASK: c_uint = 0x1f;

pub const RGS_AUDRCTUNE0READ_SFT: c_int = 0;
pub const RGS_AUDRCTUNE0READ_MASK: c_uint = 0x1f;

// AFE_ADDA_MTKAIF_FIFO_CFG0
pub const AFE_RESERVED_SFT: c_int = 1;
pub const AFE_RESERVED_MASK: c_uint = 0x7fff;

pub const RG_MTKAIF_RXIF_FIFO_INTEN_SFT: c_int = 0;
pub const RG_MTKAIF_RXIF_FIFO_INTEN_MASK: c_uint = 0x1;

// AFE_ADDA_MTKAIF_FIFO_LOG_MON1
pub const MTKAIF_RXIF_WR_FULL_STATUS_SFT: c_int = 1;
pub const MTKAIF_RXIF_WR_FULL_STATUS_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_RD_EMPTY_STATUS_SFT: c_int = 0;
pub const MTKAIF_RXIF_RD_EMPTY_STATUS_MASK: c_uint = 0x1;

// AFE_ADDA_MTKAIF_MON0
pub const MTKAIFTX_V3_SYNC_OUT_SFT: c_int = 15;
pub const MTKAIFTX_V3_SYNC_OUT_MASK: c_uint = 0x1;

pub const MTKAIFTX_V3_SDATA_OUT3_SFT: c_int = 14;
pub const MTKAIFTX_V3_SDATA_OUT3_MASK: c_uint = 0x1;

pub const MTKAIFTX_V3_SDATA_OUT2_SFT: c_int = 13;
pub const MTKAIFTX_V3_SDATA_OUT2_MASK: c_uint = 0x1;

pub const MTKAIFTX_V3_SDATA_OUT1_SFT: c_int = 12;
pub const MTKAIFTX_V3_SDATA_OUT1_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_FIFO_STATUS_SFT: c_int = 0;
pub const MTKAIF_RXIF_FIFO_STATUS_MASK: c_uint = 0xfff;

// AFE_ADDA_MTKAIF_MON1
pub const MTKAIFRX_V3_SYNC_IN_SFT: c_int = 15;
pub const MTKAIFRX_V3_SYNC_IN_MASK: c_uint = 0x1;

pub const MTKAIFRX_V3_SDATA_IN3_SFT: c_int = 14;
pub const MTKAIFRX_V3_SDATA_IN3_MASK: c_uint = 0x1;

pub const MTKAIFRX_V3_SDATA_IN2_SFT: c_int = 13;
pub const MTKAIFRX_V3_SDATA_IN2_MASK: c_uint = 0x1;

pub const MTKAIFRX_V3_SDATA_IN1_SFT: c_int = 12;
pub const MTKAIFRX_V3_SDATA_IN1_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_SEARCH_FAIL_FLAG_SFT: c_int = 11;
pub const MTKAIF_RXIF_SEARCH_FAIL_FLAG_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_INVALID_FLAG_SFT: c_int = 8;
pub const MTKAIF_RXIF_INVALID_FLAG_MASK: c_uint = 0x1;

pub const MTKAIF_RXIF_INVALID_CYCLE_SFT: c_int = 0;
pub const MTKAIF_RXIF_INVALID_CYCLE_MASK: c_uint = 0xff;

// AFE_ADDA_MTKAIF_MON2
pub const MTKAIF_TXIF_IN_CH2_SFT: c_int = 8;
pub const MTKAIF_TXIF_IN_CH2_MASK: c_uint = 0xff;

pub const MTKAIF_TXIF_IN_CH1_SFT: c_int = 0;
pub const MTKAIF_TXIF_IN_CH1_MASK: c_uint = 0xff;

// AFE_ADDA6_MTKAIF_MON3
pub const ADDA6_MTKAIF_TXIF_IN_CH2_SFT: c_int = 8;
pub const ADDA6_MTKAIF_TXIF_IN_CH2_MASK: c_uint = 0xff;

pub const ADDA6_MTKAIF_TXIF_IN_CH1_SFT: c_int = 0;
pub const ADDA6_MTKAIF_TXIF_IN_CH1_MASK: c_uint = 0xff;

// AFE_ADDA_MTKAIF_MON4
pub const MTKAIF_RXIF_OUT_CH2_SFT: c_int = 8;
pub const MTKAIF_RXIF_OUT_CH2_MASK: c_uint = 0xff;

pub const MTKAIF_RXIF_OUT_CH1_SFT: c_int = 0;
pub const MTKAIF_RXIF_OUT_CH1_MASK: c_uint = 0xff;

// AFE_ADDA_MTKAIF_MON5
pub const MTKAIF_RXIF_OUT_CH3_SFT: c_int = 0;
pub const MTKAIF_RXIF_OUT_CH3_MASK: c_uint = 0xff;

// AFE_ADDA_MTKAIF_CFG0
pub const RG_MTKAIF_RXIF_CLKINV_SFT: c_int = 15;
pub const RG_MTKAIF_RXIF_CLKINV_MASK: c_uint = 0x1;

pub const RG_ADDA6_MTKAIF_TXIF_PROTOCOL2_SFT: c_int = 9;
pub const RG_ADDA6_MTKAIF_TXIF_PROTOCOL2_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_PROTOCOL2_SFT: c_int = 8;
pub const RG_MTKAIF_RXIF_PROTOCOL2_MASK: c_uint = 0x1;

pub const RG_MTKAIF_BYPASS_SRC_MODE_SFT: c_int = 6;
pub const RG_MTKAIF_BYPASS_SRC_MODE_MASK: c_uint = 0x3;

pub const RG_MTKAIF_BYPASS_SRC_TEST_SFT: c_int = 5;
pub const RG_MTKAIF_BYPASS_SRC_TEST_MASK: c_uint = 0x1;

pub const RG_MTKAIF_TXIF_PROTOCOL2_SFT: c_int = 4;
pub const RG_MTKAIF_TXIF_PROTOCOL2_MASK: c_uint = 0x1;

pub const RG_ADDA6_MTKAIF_PMIC_TXIF_8TO5_SFT: c_int = 3;
pub const RG_ADDA6_MTKAIF_PMIC_TXIF_8TO5_MASK: c_uint = 0x1;

pub const RG_MTKAIF_PMIC_TXIF_8TO5_SFT: c_int = 2;
pub const RG_MTKAIF_PMIC_TXIF_8TO5_MASK: c_uint = 0x1;

pub const RG_MTKAIF_LOOPBACK_TEST2_SFT: c_int = 1;
pub const RG_MTKAIF_LOOPBACK_TEST2_MASK: c_uint = 0x1;

pub const RG_MTKAIF_LOOPBACK_TEST1_SFT: c_int = 0;
pub const RG_MTKAIF_LOOPBACK_TEST1_MASK: c_uint = 0x1;

// AFE_ADDA_MTKAIF_RX_CFG0
pub const RG_MTKAIF_RXIF_VOICE_MODE_SFT: c_int = 12;
pub const RG_MTKAIF_RXIF_VOICE_MODE_MASK: c_uint = 0xf;

pub const RG_MTKAIF_RXIF_DATA_BIT_SFT: c_int = 8;
pub const RG_MTKAIF_RXIF_DATA_BIT_MASK: c_uint = 0x7;

pub const RG_MTKAIF_RXIF_FIFO_RSP_SFT: c_int = 4;
pub const RG_MTKAIF_RXIF_FIFO_RSP_MASK: c_uint = 0x7;

pub const RG_MTKAIF_RXIF_DETECT_ON_SFT: c_int = 3;
pub const RG_MTKAIF_RXIF_DETECT_ON_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_DATA_MODE_SFT: c_int = 0;
pub const RG_MTKAIF_RXIF_DATA_MODE_MASK: c_uint = 0x1;

// AFE_ADDA_MTKAIF_RX_CFG1
pub const RG_MTKAIF_RXIF_SYNC_SEARCH_TABLE_SFT: c_int = 12;
pub const RG_MTKAIF_RXIF_SYNC_SEARCH_TABLE_MASK: c_uint = 0xf;

pub const RG_MTKAIF_RXIF_INVALID_SYNC_CHECK_ROUND_SFT: c_int = 8;
pub const RG_MTKAIF_RXIF_INVALID_SYNC_CHECK_ROUND_MASK: c_uint = 0xf;

pub const RG_MTKAIF_RXIF_SYNC_CHECK_ROUND_SFT: c_int = 4;
pub const RG_MTKAIF_RXIF_SYNC_CHECK_ROUND_MASK: c_uint = 0xf;

pub const RG_MTKAIF_RXIF_VOICE_MODE_PROTOCOL2_SFT: c_int = 0;
pub const RG_MTKAIF_RXIF_VOICE_MODE_PROTOCOL2_MASK: c_uint = 0xf;

// AFE_ADDA_MTKAIF_RX_CFG2
pub const RG_MTKAIF_RXIF_P2_INPUT_SEL_SFT: c_int = 15;
pub const RG_MTKAIF_RXIF_P2_INPUT_SEL_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_SYNC_WORD2_DISABLE_SFT: c_int = 14;
pub const RG_MTKAIF_RXIF_SYNC_WORD2_DISABLE_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_SYNC_WORD1_DISABLE_SFT: c_int = 13;
pub const RG_MTKAIF_RXIF_SYNC_WORD1_DISABLE_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_CLEAR_SYNC_FAIL_SFT: c_int = 12;
pub const RG_MTKAIF_RXIF_CLEAR_SYNC_FAIL_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_SYNC_CNT_TABLE_SFT: c_int = 0;
pub const RG_MTKAIF_RXIF_SYNC_CNT_TABLE_MASK: c_uint = 0xfff;

// AFE_ADDA_MTKAIF_RX_CFG3
pub const RG_MTKAIF_RXIF_LOOPBACK_USE_NLE_SFT: c_int = 7;
pub const RG_MTKAIF_RXIF_LOOPBACK_USE_NLE_MASK: c_uint = 0x1;

pub const RG_MTKAIF_RXIF_FIFO_RSP_PROTOCOL2_SFT: c_int = 4;
pub const RG_MTKAIF_RXIF_FIFO_RSP_PROTOCOL2_MASK: c_uint = 0x7;

pub const RG_MTKAIF_RXIF_DETECT_ON_PROTOCOL2_SFT: c_int = 3;
pub const RG_MTKAIF_RXIF_DETECT_ON_PROTOCOL2_MASK: c_uint = 0x1;

// AFE_ADDA_MTKAIF_SYNCWORD_CFG0
pub const RG_MTKAIF_RX_SYNC_WORD2_SFT: c_int = 4;
pub const RG_MTKAIF_RX_SYNC_WORD2_MASK: c_uint = 0x7;

pub const RG_MTKAIF_RX_SYNC_WORD1_SFT: c_int = 0;
pub const RG_MTKAIF_RX_SYNC_WORD1_MASK: c_uint = 0x7;

// AFE_ADDA_MTKAIF_SYNCWORD_CFG1
pub const RG_ADDA6_MTKAIF_TX_SYNC_WORD2_SFT: c_int = 12;
pub const RG_ADDA6_MTKAIF_TX_SYNC_WORD2_MASK: c_uint = 0x7;

pub const RG_ADDA6_MTKAIF_TX_SYNC_WORD1_SFT: c_int = 8;
pub const RG_ADDA6_MTKAIF_TX_SYNC_WORD1_MASK: c_uint = 0x7;

pub const RG_ADDA_MTKAIF_TX_SYNC_WORD2_SFT: c_int = 4;
pub const RG_ADDA_MTKAIF_TX_SYNC_WORD2_MASK: c_uint = 0x7;

pub const RG_ADDA_MTKAIF_TX_SYNC_WORD1_SFT: c_int = 0;
pub const RG_ADDA_MTKAIF_TX_SYNC_WORD1_MASK: c_uint = 0x7;

// AFE_SGEN_CFG0
pub const SGEN_AMP_DIV_CH1_CTL_SFT: c_int = 12;
pub const SGEN_AMP_DIV_CH1_CTL_MASK: c_uint = 0xf;

pub const SGEN_DAC_EN_CTL_SFT: c_int = 7;
pub const SGEN_DAC_EN_CTL_MASK: c_uint = 0x1;

pub const SGEN_MUTE_SW_CTL_SFT: c_int = 6;
pub const SGEN_MUTE_SW_CTL_MASK: c_uint = 0x1;

pub const R_AUD_SDM_MUTE_L_SFT: c_int = 5;
pub const R_AUD_SDM_MUTE_L_MASK: c_uint = 0x1;

pub const R_AUD_SDM_MUTE_R_SFT: c_int = 4;
pub const R_AUD_SDM_MUTE_R_MASK: c_uint = 0x1;

pub const R_AUD_SDM_MUTE_L_2ND_SFT: c_int = 3;
pub const R_AUD_SDM_MUTE_L_2ND_MASK: c_uint = 0x1;

pub const R_AUD_SDM_MUTE_R_2ND_SFT: c_int = 2;
pub const R_AUD_SDM_MUTE_R_2ND_MASK: c_uint = 0x1;

// AFE_SGEN_CFG1
pub const C_SGEN_RCH_INV_5BIT_SFT: c_int = 15;
pub const C_SGEN_RCH_INV_5BIT_MASK: c_uint = 0x1;

pub const C_SGEN_RCH_INV_8BIT_SFT: c_int = 14;
pub const C_SGEN_RCH_INV_8BIT_MASK: c_uint = 0x1;

pub const SGEN_FREQ_DIV_CH1_CTL_SFT: c_int = 0;
pub const SGEN_FREQ_DIV_CH1_CTL_MASK: c_uint = 0x1f;

// AFE_ADC_ASYNC_FIFO_CFG
pub const RG_UL_ASYNC_FIFO_SOFT_RST_EN_SFT: c_int = 5;
pub const RG_UL_ASYNC_FIFO_SOFT_RST_EN_MASK: c_uint = 0x1;

pub const RG_UL_ASYNC_FIFO_SOFT_RST_SFT: c_int = 4;
pub const RG_UL_ASYNC_FIFO_SOFT_RST_MASK: c_uint = 0x1;

pub const RG_AMIC_UL_ADC_CLK_SEL_SFT: c_int = 1;
pub const RG_AMIC_UL_ADC_CLK_SEL_MASK: c_uint = 0x1;

// AFE_ADC_ASYNC_FIFO_CFG1
pub const RG_UL2_ASYNC_FIFO_SOFT_RST_EN_SFT: c_int = 5;
pub const RG_UL2_ASYNC_FIFO_SOFT_RST_EN_MASK: c_uint = 0x1;

pub const RG_UL2_ASYNC_FIFO_SOFT_RST_SFT: c_int = 4;
pub const RG_UL2_ASYNC_FIFO_SOFT_RST_MASK: c_uint = 0x1;

// AFE_DCCLK_CFG0
pub const DCCLK_DIV_SFT: c_int = 5;
pub const DCCLK_DIV_MASK: c_uint = 0x7ff;

pub const DCCLK_INV_SFT: c_int = 4;
pub const DCCLK_INV_MASK: c_uint = 0x1;

pub const DCCLK_REF_CK_SEL_SFT: c_int = 2;
pub const DCCLK_REF_CK_SEL_MASK: c_uint = 0x3;

pub const DCCLK_PDN_SFT: c_int = 1;
pub const DCCLK_PDN_MASK: c_uint = 0x1;

pub const DCCLK_GEN_ON_SFT: c_int = 0;
pub const DCCLK_GEN_ON_MASK: c_uint = 0x1;

// AFE_DCCLK_CFG1
pub const RESYNC_SRC_SEL_SFT: c_int = 10;
pub const RESYNC_SRC_SEL_MASK: c_uint = 0x3;

pub const RESYNC_SRC_CK_INV_SFT: c_int = 9;
pub const RESYNC_SRC_CK_INV_MASK: c_uint = 0x1;

pub const DCCLK_RESYNC_BYPASS_SFT: c_int = 8;
pub const DCCLK_RESYNC_BYPASS_MASK: c_uint = 0x1;

pub const DCCLK_PHASE_SEL_SFT: c_int = 4;
pub const DCCLK_PHASE_SEL_MASK: c_uint = 0xf;

// AUDIO_DIG_CFG
pub const RG_AUD_PAD_TOP_DAT_MISO2_LOOPBACK_SFT: c_int = 15;
pub const RG_AUD_PAD_TOP_DAT_MISO2_LOOPBACK_MASK: c_uint = 0x1;

pub const RG_AUD_PAD_TOP_PHASE_MODE2_SFT: c_int = 8;
pub const RG_AUD_PAD_TOP_PHASE_MODE2_MASK: c_uint = 0x7f;

pub const RG_AUD_PAD_TOP_DAT_MISO_LOOPBACK_SFT: c_int = 7;
pub const RG_AUD_PAD_TOP_DAT_MISO_LOOPBACK_MASK: c_uint = 0x1;

pub const RG_AUD_PAD_TOP_PHASE_MODE_SFT: c_int = 0;
pub const RG_AUD_PAD_TOP_PHASE_MODE_MASK: c_uint = 0x7f;

// AUDIO_DIG_CFG1
pub const RG_AUD_PAD_TOP_DAT_MISO3_LOOPBACK_SFT: c_int = 7;
pub const RG_AUD_PAD_TOP_DAT_MISO3_LOOPBACK_MASK: c_uint = 0x1;

pub const RG_AUD_PAD_TOP_PHASE_MODE3_SFT: c_int = 0;
pub const RG_AUD_PAD_TOP_PHASE_MODE3_MASK: c_uint = 0x7f;

// AFE_AUD_PAD_TOP
pub const RG_AUD_PAD_TOP_TX_FIFO_RSP_SFT: c_int = 12;
pub const RG_AUD_PAD_TOP_TX_FIFO_RSP_MASK: c_uint = 0x7;

pub const RG_AUD_PAD_TOP_MTKAIF_CLK_PROTOCOL2_SFT: c_int = 11;
pub const RG_AUD_PAD_TOP_MTKAIF_CLK_PROTOCOL2_MASK: c_uint = 0x1;

pub const RG_AUD_PAD_TOP_TX_FIFO_ON_SFT: c_int = 8;
pub const RG_AUD_PAD_TOP_TX_FIFO_ON_MASK: c_uint = 0x1;

// AFE_AUD_PAD_TOP_MON
pub const ADDA_AUD_PAD_TOP_MON_SFT: c_int = 0;
pub const ADDA_AUD_PAD_TOP_MON_MASK: c_uint = 0xffff;

// AFE_AUD_PAD_TOP_MON1
pub const ADDA_AUD_PAD_TOP_MON1_SFT: c_int = 0;
pub const ADDA_AUD_PAD_TOP_MON1_MASK: c_uint = 0xffff;

// AFE_AUD_PAD_TOP_MON2
pub const ADDA_AUD_PAD_TOP_MON2_SFT: c_int = 0;
pub const ADDA_AUD_PAD_TOP_MON2_MASK: c_uint = 0xffff;

// AFE_DL_NLE_CFG
pub const NLE_RCH_HPGAIN_SEL_SFT: c_int = 10;
pub const NLE_RCH_HPGAIN_SEL_MASK: c_uint = 0x1;

pub const NLE_RCH_CH_SEL_SFT: c_int = 9;
pub const NLE_RCH_CH_SEL_MASK: c_uint = 0x1;

pub const NLE_RCH_ON_SFT: c_int = 8;
pub const NLE_RCH_ON_MASK: c_uint = 0x1;

pub const NLE_LCH_HPGAIN_SEL_SFT: c_int = 2;
pub const NLE_LCH_HPGAIN_SEL_MASK: c_uint = 0x1;

pub const NLE_LCH_CH_SEL_SFT: c_int = 1;
pub const NLE_LCH_CH_SEL_MASK: c_uint = 0x1;

pub const NLE_LCH_ON_SFT: c_int = 0;
pub const NLE_LCH_ON_MASK: c_uint = 0x1;

// AFE_DL_NLE_MON
pub const NLE_MONITOR_SFT: c_int = 0;
pub const NLE_MONITOR_MASK: c_uint = 0x3fff;

// AFE_CG_EN_MON
pub const CK_CG_EN_MON_SFT: c_int = 0;
pub const CK_CG_EN_MON_MASK: c_uint = 0x3f;

// AFE_MIC_ARRAY_CFG
pub const RG_AMIC_ADC1_SOURCE_SEL_SFT: c_int = 10;
pub const RG_AMIC_ADC1_SOURCE_SEL_MASK: c_uint = 0x3;

pub const RG_AMIC_ADC2_SOURCE_SEL_SFT: c_int = 8;
pub const RG_AMIC_ADC2_SOURCE_SEL_MASK: c_uint = 0x3;

pub const RG_AMIC_ADC3_SOURCE_SEL_SFT: c_int = 6;
pub const RG_AMIC_ADC3_SOURCE_SEL_MASK: c_uint = 0x3;

pub const RG_DMIC_ADC1_SOURCE_SEL_SFT: c_int = 4;
pub const RG_DMIC_ADC1_SOURCE_SEL_MASK: c_uint = 0x3;

pub const RG_DMIC_ADC2_SOURCE_SEL_SFT: c_int = 2;
pub const RG_DMIC_ADC2_SOURCE_SEL_MASK: c_uint = 0x3;

pub const RG_DMIC_ADC3_SOURCE_SEL_SFT: c_int = 0;
pub const RG_DMIC_ADC3_SOURCE_SEL_MASK: c_uint = 0x3;

// AFE_CHOP_CFG0
pub const RG_CHOP_DIV_SEL_SFT: c_int = 4;
pub const RG_CHOP_DIV_SEL_MASK: c_uint = 0x1f;

pub const RG_CHOP_DIV_EN_SFT: c_int = 0;
pub const RG_CHOP_DIV_EN_MASK: c_uint = 0x1;

// AFE_MTKAIF_MUX_CFG
pub const RG_ADDA6_EN_SEL_SFT: c_int = 12;
pub const RG_ADDA6_EN_SEL_MASK: c_uint = 0x1;

pub const RG_ADDA6_CH2_SEL_SFT: c_int = 10;
pub const RG_ADDA6_CH2_SEL_MASK: c_uint = 0x3;

pub const RG_ADDA6_CH1_SEL_SFT: c_int = 8;
pub const RG_ADDA6_CH1_SEL_MASK: c_uint = 0x3;

pub const RG_ADDA_EN_SEL_SFT: c_int = 4;
pub const RG_ADDA_EN_SEL_MASK: c_uint = 0x1;

pub const RG_ADDA_CH2_SEL_SFT: c_int = 2;
pub const RG_ADDA_CH2_SEL_MASK: c_uint = 0x3;

pub const RG_ADDA_CH1_SEL_SFT: c_int = 0;
pub const RG_ADDA_CH1_SEL_MASK: c_uint = 0x3;

// AFE_PMIC_NEWIF_CFG3
pub const RG_UP8X_SYNC_WORD_SFT: c_int = 0;
pub const RG_UP8X_SYNC_WORD_MASK: c_uint = 0xffff;

// AFE_NCP_CFG0
pub const RG_NCP_CK1_VALID_CNT_SFT: c_int = 9;
pub const RG_NCP_CK1_VALID_CNT_MASK: c_uint = 0x7f;

pub const RG_NCP_ADITH_SFT: c_int = 8;
pub const RG_NCP_ADITH_MASK: c_uint = 0x1;

pub const RG_NCP_DITHER_EN_SFT: c_int = 7;
pub const RG_NCP_DITHER_EN_MASK: c_uint = 0x1;

pub const RG_NCP_DITHER_FIXED_CK0_ACK1_2P_SFT: c_int = 4;
pub const RG_NCP_DITHER_FIXED_CK0_ACK1_2P_MASK: c_uint = 0x7;

pub const RG_NCP_DITHER_FIXED_CK0_ACK2_2P_SFT: c_int = 1;
pub const RG_NCP_DITHER_FIXED_CK0_ACK2_2P_MASK: c_uint = 0x7;

pub const RG_NCP_ON_SFT: c_int = 0;
pub const RG_NCP_ON_MASK: c_uint = 0x1;

// AFE_NCP_CFG1
pub const RG_XY_VAL_CFG_EN_SFT: c_int = 15;
pub const RG_XY_VAL_CFG_EN_MASK: c_uint = 0x1;

pub const RG_X_VAL_CFG_SFT: c_int = 8;
pub const RG_X_VAL_CFG_MASK: c_uint = 0x7f;

pub const RG_Y_VAL_CFG_SFT: c_int = 0;
pub const RG_Y_VAL_CFG_MASK: c_uint = 0x7f;

// AFE_NCP_CFG2
pub const RG_NCP_NONCLK_SET_SFT: c_int = 1;
pub const RG_NCP_NONCLK_SET_MASK: c_uint = 0x1;

pub const RG_NCP_PDDIS_EN_SFT: c_int = 0;
pub const RG_NCP_PDDIS_EN_MASK: c_uint = 0x1;

// AUDENC_ANA_CON0
pub const RG_AUDPREAMPLON_SFT: c_int = 0;
pub const RG_AUDPREAMPLON_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLDCCEN_SFT: c_int = 1;
pub const RG_AUDPREAMPLDCCEN_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLDCPRECHARGE_SFT: c_int = 2;
pub const RG_AUDPREAMPLDCPRECHARGE_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLPGATEST_SFT: c_int = 3;
pub const RG_AUDPREAMPLPGATEST_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLVSCALE_SFT: c_int = 4;
pub const RG_AUDPREAMPLVSCALE_MASK: c_uint = 0x3;

pub const RG_AUDPREAMPLINPUTSEL_SFT: c_int = 6;
pub const RG_AUDPREAMPLINPUTSEL_MASK: c_uint = 0x3;

pub const RG_AUDPREAMPLGAIN_SFT: c_int = 8;
pub const RG_AUDPREAMPLGAIN_MASK: c_uint = 0x7;

pub const RG_BULKL_VCM_EN_SFT: c_int = 11;
pub const RG_BULKL_VCM_EN_MASK: c_uint = 0x1;

pub const RG_AUDADCLPWRUP_SFT: c_int = 12;
pub const RG_AUDADCLPWRUP_MASK: c_uint = 0x1;

pub const RG_AUDADCLINPUTSEL_SFT: c_int = 13;
pub const RG_AUDADCLINPUTSEL_MASK: c_uint = 0x3;

// AUDENC_ANA_CON1
pub const RG_AUDPREAMPRON_SFT: c_int = 0;
pub const RG_AUDPREAMPRON_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPRDCCEN_SFT: c_int = 1;
pub const RG_AUDPREAMPRDCCEN_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPRDCPRECHARGE_SFT: c_int = 2;
pub const RG_AUDPREAMPRDCPRECHARGE_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPRPGATEST_SFT: c_int = 3;
pub const RG_AUDPREAMPRPGATEST_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPRVSCALE_SFT: c_int = 4;
pub const RG_AUDPREAMPRVSCALE_MASK: c_uint = 0x3;

pub const RG_AUDPREAMPRINPUTSEL_SFT: c_int = 6;
pub const RG_AUDPREAMPRINPUTSEL_MASK: c_uint = 0x3;

pub const RG_AUDPREAMPRGAIN_SFT: c_int = 8;
pub const RG_AUDPREAMPRGAIN_MASK: c_uint = 0x7;

pub const RG_BULKR_VCM_EN_SFT: c_int = 11;
pub const RG_BULKR_VCM_EN_MASK: c_uint = 0x1;

pub const RG_AUDADCRPWRUP_SFT: c_int = 12;
pub const RG_AUDADCRPWRUP_MASK: c_uint = 0x1;

pub const RG_AUDADCRINPUTSEL_SFT: c_int = 13;
pub const RG_AUDADCRINPUTSEL_MASK: c_uint = 0x3;

// AUDENC_ANA_CON2
pub const RG_AUDPREAMP3ON_SFT: c_int = 0;
pub const RG_AUDPREAMP3ON_MASK: c_uint = 0x1;

pub const RG_AUDPREAMP3DCCEN_SFT: c_int = 1;
pub const RG_AUDPREAMP3DCCEN_MASK: c_uint = 0x1;

pub const RG_AUDPREAMP3DCPRECHARGE_SFT: c_int = 2;
pub const RG_AUDPREAMP3DCPRECHARGE_MASK: c_uint = 0x1;

pub const RG_AUDPREAMP3PGATEST_SFT: c_int = 3;
pub const RG_AUDPREAMP3PGATEST_MASK: c_uint = 0x1;

pub const RG_AUDPREAMP3VSCALE_SFT: c_int = 4;
pub const RG_AUDPREAMP3VSCALE_MASK: c_uint = 0x3;

pub const RG_AUDPREAMP3INPUTSEL_SFT: c_int = 6;
pub const RG_AUDPREAMP3INPUTSEL_MASK: c_uint = 0x3;

pub const RG_AUDPREAMP3GAIN_SFT: c_int = 8;
pub const RG_AUDPREAMP3GAIN_MASK: c_uint = 0x7;

pub const RG_BULK3_VCM_EN_SFT: c_int = 11;
pub const RG_BULK3_VCM_EN_MASK: c_uint = 0x1;

pub const RG_AUDADC3PWRUP_SFT: c_int = 12;
pub const RG_AUDADC3PWRUP_MASK: c_uint = 0x1;

pub const RG_AUDADC3INPUTSEL_SFT: c_int = 13;
pub const RG_AUDADC3INPUTSEL_MASK: c_uint = 0x3;

// AUDENC_ANA_CON3
pub const RG_AUDULHALFBIAS_SFT: c_int = 0;
pub const RG_AUDULHALFBIAS_MASK: c_uint = 0x1;

pub const RG_AUDGLBVOWLPWEN_SFT: c_int = 1;
pub const RG_AUDGLBVOWLPWEN_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPLPEN_SFT: c_int = 2;
pub const RG_AUDPREAMPLPEN_MASK: c_uint = 0x1;

pub const RG_AUDADC1STSTAGELPEN_SFT: c_int = 3;
pub const RG_AUDADC1STSTAGELPEN_MASK: c_uint = 0x1;

pub const RG_AUDADC2NDSTAGELPEN_SFT: c_int = 4;
pub const RG_AUDADC2NDSTAGELPEN_MASK: c_uint = 0x1;

pub const RG_AUDADCFLASHLPEN_SFT: c_int = 5;
pub const RG_AUDADCFLASHLPEN_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPIDDTEST_SFT: c_int = 6;
pub const RG_AUDPREAMPIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADC1STSTAGEIDDTEST_SFT: c_int = 8;
pub const RG_AUDADC1STSTAGEIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADC2NDSTAGEIDDTEST_SFT: c_int = 10;
pub const RG_AUDADC2NDSTAGEIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADCREFBUFIDDTEST_SFT: c_int = 12;
pub const RG_AUDADCREFBUFIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADCFLASHIDDTEST_SFT: c_int = 14;
pub const RG_AUDADCFLASHIDDTEST_MASK: c_uint = 0x3;

// AUDENC_ANA_CON4
pub const RG_AUDRULHALFBIAS_SFT: c_int = 0;
pub const RG_AUDRULHALFBIAS_MASK: c_uint = 0x1;

pub const RG_AUDGLBRVOWLPWEN_SFT: c_int = 1;
pub const RG_AUDGLBRVOWLPWEN_MASK: c_uint = 0x1;

pub const RG_AUDRPREAMPLPEN_SFT: c_int = 2;
pub const RG_AUDRPREAMPLPEN_MASK: c_uint = 0x1;

pub const RG_AUDRADC1STSTAGELPEN_SFT: c_int = 3;
pub const RG_AUDRADC1STSTAGELPEN_MASK: c_uint = 0x1;

pub const RG_AUDRADC2NDSTAGELPEN_SFT: c_int = 4;
pub const RG_AUDRADC2NDSTAGELPEN_MASK: c_uint = 0x1;

pub const RG_AUDRADCFLASHLPEN_SFT: c_int = 5;
pub const RG_AUDRADCFLASHLPEN_MASK: c_uint = 0x1;

pub const RG_AUDRPREAMPIDDTEST_SFT: c_int = 6;
pub const RG_AUDRPREAMPIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDRADC1STSTAGEIDDTEST_SFT: c_int = 8;
pub const RG_AUDRADC1STSTAGEIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDRADC2NDSTAGEIDDTEST_SFT: c_int = 10;
pub const RG_AUDRADC2NDSTAGEIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDRADCREFBUFIDDTEST_SFT: c_int = 12;
pub const RG_AUDRADCREFBUFIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDRADCFLASHIDDTEST_SFT: c_int = 14;
pub const RG_AUDRADCFLASHIDDTEST_MASK: c_uint = 0x3;

// AUDENC_ANA_CON5
pub const RG_AUDADCCLKRSTB_SFT: c_int = 0;
pub const RG_AUDADCCLKRSTB_MASK: c_uint = 0x1;

pub const RG_AUDADCCLKSEL_SFT: c_int = 1;
pub const RG_AUDADCCLKSEL_MASK: c_uint = 0x3;

pub const RG_AUDADCCLKSOURCE_SFT: c_int = 3;
pub const RG_AUDADCCLKSOURCE_MASK: c_uint = 0x3;

pub const RG_AUDADCCLKGENMODE_SFT: c_int = 5;
pub const RG_AUDADCCLKGENMODE_MASK: c_uint = 0x3;

pub const RG_AUDPREAMP_ACCFS_SFT: c_int = 7;
pub const RG_AUDPREAMP_ACCFS_MASK: c_uint = 0x1;

pub const RG_AUDPREAMPAAFEN_SFT: c_int = 8;
pub const RG_AUDPREAMPAAFEN_MASK: c_uint = 0x1;

pub const RG_DCCVCMBUFLPMODSEL_SFT: c_int = 9;
pub const RG_DCCVCMBUFLPMODSEL_MASK: c_uint = 0x1;

pub const RG_DCCVCMBUFLPSWEN_SFT: c_int = 10;
pub const RG_DCCVCMBUFLPSWEN_MASK: c_uint = 0x1;

pub const RG_AUDSPAREPGA_SFT: c_int = 11;
pub const RG_AUDSPAREPGA_MASK: c_uint = 0x1f;

// AUDENC_ANA_CON6
pub const RG_AUDADC1STSTAGESDENB_SFT: c_int = 0;
pub const RG_AUDADC1STSTAGESDENB_MASK: c_uint = 0x1;

pub const RG_AUDADC2NDSTAGERESET_SFT: c_int = 1;
pub const RG_AUDADC2NDSTAGERESET_MASK: c_uint = 0x1;

pub const RG_AUDADC3RDSTAGERESET_SFT: c_int = 2;
pub const RG_AUDADC3RDSTAGERESET_MASK: c_uint = 0x1;

pub const RG_AUDADCFSRESET_SFT: c_int = 3;
pub const RG_AUDADCFSRESET_MASK: c_uint = 0x1;

pub const RG_AUDADCWIDECM_SFT: c_int = 4;
pub const RG_AUDADCWIDECM_MASK: c_uint = 0x1;

pub const RG_AUDADCNOPATEST_SFT: c_int = 5;
pub const RG_AUDADCNOPATEST_MASK: c_uint = 0x1;

pub const RG_AUDADCBYPASS_SFT: c_int = 6;
pub const RG_AUDADCBYPASS_MASK: c_uint = 0x1;

pub const RG_AUDADCFFBYPASS_SFT: c_int = 7;
pub const RG_AUDADCFFBYPASS_MASK: c_uint = 0x1;

pub const RG_AUDADCDACFBCURRENT_SFT: c_int = 8;
pub const RG_AUDADCDACFBCURRENT_MASK: c_uint = 0x1;

pub const RG_AUDADCDACIDDTEST_SFT: c_int = 9;
pub const RG_AUDADCDACIDDTEST_MASK: c_uint = 0x3;

pub const RG_AUDADCDACNRZ_SFT: c_int = 11;
pub const RG_AUDADCDACNRZ_MASK: c_uint = 0x1;

pub const RG_AUDADCNODEM_SFT: c_int = 12;
pub const RG_AUDADCNODEM_MASK: c_uint = 0x1;

pub const RG_AUDADCDACTEST_SFT: c_int = 13;
pub const RG_AUDADCDACTEST_MASK: c_uint = 0x1;

pub const RG_AUDADCDAC0P25FS_SFT: c_int = 14;
pub const RG_AUDADCDAC0P25FS_MASK: c_uint = 0x1;

pub const RG_AUDADCRDAC0P25FS_SFT: c_int = 15;
pub const RG_AUDADCRDAC0P25FS_MASK: c_uint = 0x1;

// AUDENC_ANA_CON7
pub const RG_AUDADCTESTDATA_SFT: c_int = 0;
pub const RG_AUDADCTESTDATA_MASK: c_uint = 0xffff;

// AUDENC_ANA_CON8
pub const RG_AUDRCTUNEL_SFT: c_int = 0;
pub const RG_AUDRCTUNEL_MASK: c_uint = 0x1f;

pub const RG_AUDRCTUNELSEL_SFT: c_int = 5;
pub const RG_AUDRCTUNELSEL_MASK: c_uint = 0x1;

pub const RG_AUDRCTUNER_SFT: c_int = 8;
pub const RG_AUDRCTUNER_MASK: c_uint = 0x1f;

pub const RG_AUDRCTUNERSEL_SFT: c_int = 13;
pub const RG_AUDRCTUNERSEL_MASK: c_uint = 0x1;

// AUDENC_ANA_CON9
pub const RG_AUD3CTUNEL_SFT: c_int = 0;
pub const RG_AUD3CTUNEL_MASK: c_uint = 0x1f;

pub const RG_AUD3CTUNELSEL_SFT: c_int = 5;
pub const RG_AUD3CTUNELSEL_MASK: c_uint = 0x1;

pub const RGS_AUDRCTUNE3READ_SFT: c_int = 6;
pub const RGS_AUDRCTUNE3READ_MASK: c_uint = 0x1f;

pub const RG_AUD3SPARE_SFT: c_int = 11;
pub const RG_AUD3SPARE_MASK: c_uint = 0x1f;

// AUDENC_ANA_CON10
pub const RGS_AUDRCTUNELREAD_SFT: c_int = 0;
pub const RGS_AUDRCTUNELREAD_MASK: c_uint = 0x1f;

pub const RGS_AUDRCTUNERREAD_SFT: c_int = 8;
pub const RGS_AUDRCTUNERREAD_MASK: c_uint = 0x1f;

// AUDENC_ANA_CON11
pub const RG_AUDSPAREVA30_SFT: c_int = 0;
pub const RG_AUDSPAREVA30_MASK: c_uint = 0xff;

pub const RG_AUDSPAREVA18_SFT: c_int = 8;
pub const RG_AUDSPAREVA18_MASK: c_uint = 0xff;

// AUDENC_ANA_CON12
pub const RG_AUDPGA_DECAP_SFT: c_int = 0;
pub const RG_AUDPGA_DECAP_MASK: c_uint = 0x1;

pub const RG_AUDPGA_CAPRA_SFT: c_int = 1;
pub const RG_AUDPGA_CAPRA_MASK: c_uint = 0x1;

pub const RG_AUDPGA_ACCCMP_SFT: c_int = 2;
pub const RG_AUDPGA_ACCCMP_MASK: c_uint = 0x1;

pub const RG_AUDENC_SPARE2_SFT: c_int = 3;
pub const RG_AUDENC_SPARE2_MASK: c_uint = 0x1fff;

// AUDENC_ANA_CON13
pub const RG_AUDDIGMICEN_SFT: c_int = 0;
pub const RG_AUDDIGMICEN_MASK: c_uint = 0x1;

pub const RG_AUDDIGMICBIAS_SFT: c_int = 1;
pub const RG_AUDDIGMICBIAS_MASK: c_uint = 0x3;

pub const RG_DMICHPCLKEN_SFT: c_int = 3;
pub const RG_DMICHPCLKEN_MASK: c_uint = 0x1;

pub const RG_AUDDIGMICPDUTY_SFT: c_int = 4;
pub const RG_AUDDIGMICPDUTY_MASK: c_uint = 0x3;

pub const RG_AUDDIGMICNDUTY_SFT: c_int = 6;
pub const RG_AUDDIGMICNDUTY_MASK: c_uint = 0x3;

pub const RG_DMICMONEN_SFT: c_int = 8;
pub const RG_DMICMONEN_MASK: c_uint = 0x1;

pub const RG_DMICMONSEL_SFT: c_int = 9;
pub const RG_DMICMONSEL_MASK: c_uint = 0x7;

// AUDENC_ANA_CON14
pub const RG_AUDDIGMIC1EN_SFT: c_int = 0;
pub const RG_AUDDIGMIC1EN_MASK: c_uint = 0x1;

pub const RG_AUDDIGMICBIAS1_SFT: c_int = 1;
pub const RG_AUDDIGMICBIAS1_MASK: c_uint = 0x3;

pub const RG_DMIC1HPCLKEN_SFT: c_int = 3;
pub const RG_DMIC1HPCLKEN_MASK: c_uint = 0x1;

pub const RG_AUDDIGMIC1PDUTY_SFT: c_int = 4;
pub const RG_AUDDIGMIC1PDUTY_MASK: c_uint = 0x3;

pub const RG_AUDDIGMIC1NDUTY_SFT: c_int = 6;
pub const RG_AUDDIGMIC1NDUTY_MASK: c_uint = 0x3;

pub const RG_DMIC1MONEN_SFT: c_int = 8;
pub const RG_DMIC1MONEN_MASK: c_uint = 0x1;

pub const RG_DMIC1MONSEL_SFT: c_int = 9;
pub const RG_DMIC1MONSEL_MASK: c_uint = 0x7;

pub const RG_AUDSPAREVMIC_SFT: c_int = 12;
pub const RG_AUDSPAREVMIC_MASK: c_uint = 0xf;

// AUDENC_ANA_CON15
pub const RG_AUDPWDBMICBIAS0_SFT: c_int = 0;
pub const RG_AUDPWDBMICBIAS0_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0BYPASSEN_SFT: c_int = 1;
pub const RG_AUDMICBIAS0BYPASSEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0LOWPEN_SFT: c_int = 2;
pub const RG_AUDMICBIAS0LOWPEN_MASK: c_uint = 0x1;

pub const RG_AUDPWDBMICBIAS3_SFT: c_int = 3;
pub const RG_AUDPWDBMICBIAS3_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0VREF_SFT: c_int = 4;
pub const RG_AUDMICBIAS0VREF_MASK: c_uint = 0x7;

pub const RG_AUDMICBIAS0DCSW0P1EN_SFT: c_int = 8;
pub const RG_AUDMICBIAS0DCSW0P1EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW0P2EN_SFT: c_int = 9;
pub const RG_AUDMICBIAS0DCSW0P2EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW0NEN_SFT: c_int = 10;
pub const RG_AUDMICBIAS0DCSW0NEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW2P1EN_SFT: c_int = 12;
pub const RG_AUDMICBIAS0DCSW2P1EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW2P2EN_SFT: c_int = 13;
pub const RG_AUDMICBIAS0DCSW2P2EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS0DCSW2NEN_SFT: c_int = 14;
pub const RG_AUDMICBIAS0DCSW2NEN_MASK: c_uint = 0x1;

// AUDENC_ANA_CON16
pub const RG_AUDPWDBMICBIAS1_SFT: c_int = 0;
pub const RG_AUDPWDBMICBIAS1_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1BYPASSEN_SFT: c_int = 1;
pub const RG_AUDMICBIAS1BYPASSEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1LOWPEN_SFT: c_int = 2;
pub const RG_AUDMICBIAS1LOWPEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1VREF_SFT: c_int = 4;
pub const RG_AUDMICBIAS1VREF_MASK: c_uint = 0x7;

pub const RG_AUDMICBIAS1DCSW1PEN_SFT: c_int = 8;
pub const RG_AUDMICBIAS1DCSW1PEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1DCSW1NEN_SFT: c_int = 9;
pub const RG_AUDMICBIAS1DCSW1NEN_MASK: c_uint = 0x1;

pub const RG_BANDGAPGEN_SFT: c_int = 10;
pub const RG_BANDGAPGEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1HVEN_SFT: c_int = 12;
pub const RG_AUDMICBIAS1HVEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS1HVVREF_SFT: c_int = 13;
pub const RG_AUDMICBIAS1HVVREF_MASK: c_uint = 0x1;

// AUDENC_ANA_CON17
pub const RG_AUDPWDBMICBIAS2_SFT: c_int = 0;
pub const RG_AUDPWDBMICBIAS2_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS2BYPASSEN_SFT: c_int = 1;
pub const RG_AUDMICBIAS2BYPASSEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS2LOWPEN_SFT: c_int = 2;
pub const RG_AUDMICBIAS2LOWPEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS2VREF_SFT: c_int = 4;
pub const RG_AUDMICBIAS2VREF_MASK: c_uint = 0x7;

pub const RG_AUDMICBIAS2DCSW3P1EN_SFT: c_int = 8;
pub const RG_AUDMICBIAS2DCSW3P1EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS2DCSW3P2EN_SFT: c_int = 9;
pub const RG_AUDMICBIAS2DCSW3P2EN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIAS2DCSW3NEN_SFT: c_int = 10;
pub const RG_AUDMICBIAS2DCSW3NEN_MASK: c_uint = 0x1;

pub const RG_AUDMICBIASSPARE_SFT: c_int = 12;
pub const RG_AUDMICBIASSPARE_MASK: c_uint = 0xf;

// AUDENC_ANA_CON18
pub const RG_AUDACCDETMICBIAS0PULLLOW_SFT: c_int = 0;
pub const RG_AUDACCDETMICBIAS0PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETMICBIAS1PULLLOW_SFT: c_int = 1;
pub const RG_AUDACCDETMICBIAS1PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETMICBIAS2PULLLOW_SFT: c_int = 2;
pub const RG_AUDACCDETMICBIAS2PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETVIN1PULLLOW_SFT: c_int = 3;
pub const RG_AUDACCDETVIN1PULLLOW_MASK: c_uint = 0x1;

pub const RG_AUDACCDETVTHACAL_SFT: c_int = 4;
pub const RG_AUDACCDETVTHACAL_MASK: c_uint = 0x1;

pub const RG_AUDACCDETVTHBCAL_SFT: c_int = 5;
pub const RG_AUDACCDETVTHBCAL_MASK: c_uint = 0x1;

pub const RG_AUDACCDETTVDET_SFT: c_int = 6;
pub const RG_AUDACCDETTVDET_MASK: c_uint = 0x1;

pub const RG_ACCDETSEL_SFT: c_int = 7;
pub const RG_ACCDETSEL_MASK: c_uint = 0x1;

pub const RG_SWBUFMODSEL_SFT: c_int = 8;
pub const RG_SWBUFMODSEL_MASK: c_uint = 0x1;

pub const RG_SWBUFSWEN_SFT: c_int = 9;
pub const RG_SWBUFSWEN_MASK: c_uint = 0x1;

pub const RG_EINT0NOHYS_SFT: c_int = 10;
pub const RG_EINT0NOHYS_MASK: c_uint = 0x1;

pub const RG_EINT0CONFIGACCDET_SFT: c_int = 11;
pub const RG_EINT0CONFIGACCDET_MASK: c_uint = 0x1;

pub const RG_EINT0HIRENB_SFT: c_int = 12;
pub const RG_EINT0HIRENB_MASK: c_uint = 0x1;

pub const RG_ACCDET2AUXRESBYPASS_SFT: c_int = 13;
pub const RG_ACCDET2AUXRESBYPASS_MASK: c_uint = 0x1;

pub const RG_ACCDET2AUXSWEN_SFT: c_int = 14;
pub const RG_ACCDET2AUXSWEN_MASK: c_uint = 0x1;

pub const RG_AUDACCDETMICBIAS3PULLLOW_SFT: c_int = 15;
pub const RG_AUDACCDETMICBIAS3PULLLOW_MASK: c_uint = 0x1;

// AUDENC_ANA_CON19
pub const RG_EINT1CONFIGACCDET_SFT: c_int = 0;
pub const RG_EINT1CONFIGACCDET_MASK: c_uint = 0x1;

pub const RG_EINT1HIRENB_SFT: c_int = 1;
pub const RG_EINT1HIRENB_MASK: c_uint = 0x1;

pub const RG_EINT1NOHYS_SFT: c_int = 2;
pub const RG_EINT1NOHYS_MASK: c_uint = 0x1;

pub const RG_EINTCOMPVTH_SFT: c_int = 4;
pub const RG_EINTCOMPVTH_MASK: c_uint = 0xf;

pub const RG_MTEST_EN_SFT: c_int = 8;
pub const RG_MTEST_EN_MASK: c_uint = 0x1;

pub const RG_MTEST_SEL_SFT: c_int = 9;
pub const RG_MTEST_SEL_MASK: c_uint = 0x1;

pub const RG_MTEST_CURRENT_SFT: c_int = 10;
pub const RG_MTEST_CURRENT_MASK: c_uint = 0x1;

pub const RG_ANALOGFDEN_SFT: c_int = 12;
pub const RG_ANALOGFDEN_MASK: c_uint = 0x1;

pub const RG_FDVIN1PPULLLOW_SFT: c_int = 13;
pub const RG_FDVIN1PPULLLOW_MASK: c_uint = 0x1;

pub const RG_FDEINT0TYPE_SFT: c_int = 14;
pub const RG_FDEINT0TYPE_MASK: c_uint = 0x1;

pub const RG_FDEINT1TYPE_SFT: c_int = 15;
pub const RG_FDEINT1TYPE_MASK: c_uint = 0x1;

// AUDENC_ANA_CON20
pub const RG_EINT0CMPEN_SFT: c_int = 0;
pub const RG_EINT0CMPEN_MASK: c_uint = 0x1;

pub const RG_EINT0CMPMEN_SFT: c_int = 1;
pub const RG_EINT0CMPMEN_MASK: c_uint = 0x1;

pub const RG_EINT0EN_SFT: c_int = 2;
pub const RG_EINT0EN_MASK: c_uint = 0x1;

pub const RG_EINT0CEN_SFT: c_int = 3;
pub const RG_EINT0CEN_MASK: c_uint = 0x1;

pub const RG_EINT0INVEN_SFT: c_int = 4;
pub const RG_EINT0INVEN_MASK: c_uint = 0x1;

pub const RG_EINT0CTURBO_SFT: c_int = 5;
pub const RG_EINT0CTURBO_MASK: c_uint = 0x7;

pub const RG_EINT1CMPEN_SFT: c_int = 8;
pub const RG_EINT1CMPEN_MASK: c_uint = 0x1;

pub const RG_EINT1CMPMEN_SFT: c_int = 9;
pub const RG_EINT1CMPMEN_MASK: c_uint = 0x1;

pub const RG_EINT1EN_SFT: c_int = 10;
pub const RG_EINT1EN_MASK: c_uint = 0x1;

pub const RG_EINT1CEN_SFT: c_int = 11;
pub const RG_EINT1CEN_MASK: c_uint = 0x1;

pub const RG_EINT1INVEN_SFT: c_int = 12;
pub const RG_EINT1INVEN_MASK: c_uint = 0x1;

pub const RG_EINT1CTURBO_SFT: c_int = 13;
pub const RG_EINT1CTURBO_MASK: c_uint = 0x7;

// AUDENC_ANA_CON21
pub const RG_ACCDETSPARE_SFT: c_int = 0;
pub const RG_ACCDETSPARE_MASK: c_uint = 0xffff;

// AUDENC_ANA_CON22
pub const RG_AUDENCSPAREVA30_SFT: c_int = 0;
pub const RG_AUDENCSPAREVA30_MASK: c_uint = 0xff;

pub const RG_AUDENCSPAREVA18_SFT: c_int = 8;
pub const RG_AUDENCSPAREVA18_MASK: c_uint = 0xff;

// AUDENC_ANA_CON23
pub const RG_CLKSQ_EN_SFT: c_int = 0;
pub const RG_CLKSQ_EN_MASK: c_uint = 0x1;

pub const RG_CLKSQ_IN_SEL_TEST_SFT: c_int = 1;
pub const RG_CLKSQ_IN_SEL_TEST_MASK: c_uint = 0x1;

pub const RG_CM_REFGENSEL_SFT: c_int = 2;
pub const RG_CM_REFGENSEL_MASK: c_uint = 0x1;

pub const RG_AUDIO_VOW_EN_SFT: c_int = 3;
pub const RG_AUDIO_VOW_EN_MASK: c_uint = 0x1;

pub const RG_CLKSQ_EN_VOW_SFT: c_int = 4;
pub const RG_CLKSQ_EN_VOW_MASK: c_uint = 0x1;

pub const RG_CLKAND_EN_VOW_SFT: c_int = 5;
pub const RG_CLKAND_EN_VOW_MASK: c_uint = 0x1;

pub const RG_VOWCLK_SEL_EN_VOW_SFT: c_int = 6;
pub const RG_VOWCLK_SEL_EN_VOW_MASK: c_uint = 0x1;

pub const RG_SPARE_VOW_SFT: c_int = 7;
pub const RG_SPARE_VOW_MASK: c_uint = 0x7;

// AUDDEC_ANA_CON0
pub const RG_AUDDACLPWRUP_VAUDP32_SFT: c_int = 0;
pub const RG_AUDDACLPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDDACRPWRUP_VAUDP32_SFT: c_int = 1;
pub const RG_AUDDACRPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUD_DAC_PWR_UP_VA32_SFT: c_int = 2;
pub const RG_AUD_DAC_PWR_UP_VA32_MASK: c_uint = 0x1;

pub const RG_AUD_DAC_PWL_UP_VA32_SFT: c_int = 3;
pub const RG_AUD_DAC_PWL_UP_VA32_MASK: c_uint = 0x1;

pub const RG_AUDHPLPWRUP_VAUDP32_SFT: c_int = 4;
pub const RG_AUDHPLPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPRPWRUP_VAUDP32_SFT: c_int = 5;
pub const RG_AUDHPRPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPLPWRUP_IBIAS_VAUDP32_SFT: c_int = 6;
pub const RG_AUDHPLPWRUP_IBIAS_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPRPWRUP_IBIAS_VAUDP32_SFT: c_int = 7;
pub const RG_AUDHPRPWRUP_IBIAS_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPLMUXINPUTSEL_VAUDP32_SFT: c_int = 8;
pub const RG_AUDHPLMUXINPUTSEL_VAUDP32_MASK: c_uint = 0x3;

pub const RG_AUDHPRMUXINPUTSEL_VAUDP32_SFT: c_int = 10;
pub const RG_AUDHPRMUXINPUTSEL_VAUDP32_MASK: c_uint = 0x3;

pub const RG_AUDHPLSCDISABLE_VAUDP32_SFT: c_int = 12;
pub const RG_AUDHPLSCDISABLE_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPRSCDISABLE_VAUDP32_SFT: c_int = 13;
pub const RG_AUDHPRSCDISABLE_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPLBSCCURRENT_VAUDP32_SFT: c_int = 14;
pub const RG_AUDHPLBSCCURRENT_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPRBSCCURRENT_VAUDP32_SFT: c_int = 15;
pub const RG_AUDHPRBSCCURRENT_VAUDP32_MASK: c_uint = 0x1;

// AUDDEC_ANA_CON1
pub const RG_AUDHPLOUTPWRUP_VAUDP32_SFT: c_int = 0;
pub const RG_AUDHPLOUTPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPROUTPWRUP_VAUDP32_SFT: c_int = 1;
pub const RG_AUDHPROUTPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPLOUTAUXPWRUP_VAUDP32_SFT: c_int = 2;
pub const RG_AUDHPLOUTAUXPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPROUTAUXPWRUP_VAUDP32_SFT: c_int = 3;
pub const RG_AUDHPROUTAUXPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HPLAUXFBRSW_EN_VAUDP32_SFT: c_int = 4;
pub const RG_HPLAUXFBRSW_EN_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HPRAUXFBRSW_EN_VAUDP32_SFT: c_int = 5;
pub const RG_HPRAUXFBRSW_EN_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HPLSHORT2HPLAUX_EN_VAUDP32_SFT: c_int = 6;
pub const RG_HPLSHORT2HPLAUX_EN_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HPRSHORT2HPRAUX_EN_VAUDP32_SFT: c_int = 7;
pub const RG_HPRSHORT2HPRAUX_EN_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HPLOUTSTGCTRL_VAUDP32_SFT: c_int = 8;
pub const RG_HPLOUTSTGCTRL_VAUDP32_MASK: c_uint = 0x7;

pub const RG_HPROUTSTGCTRL_VAUDP32_SFT: c_int = 12;
pub const RG_HPROUTSTGCTRL_VAUDP32_MASK: c_uint = 0x7;

// AUDDEC_ANA_CON2
pub const RG_HPLOUTPUTSTBENH_VAUDP32_SFT: c_int = 0;
pub const RG_HPLOUTPUTSTBENH_VAUDP32_MASK: c_uint = 0x7;

pub const RG_HPROUTPUTSTBENH_VAUDP32_SFT: c_int = 4;
pub const RG_HPROUTPUTSTBENH_VAUDP32_MASK: c_uint = 0x7;

pub const RG_AUDHPSTARTUP_VAUDP32_SFT: c_int = 7;
pub const RG_AUDHPSTARTUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDREFN_DERES_EN_VAUDP32_SFT: c_int = 8;
pub const RG_AUDREFN_DERES_EN_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HPINPUTSTBENH_VAUDP32_SFT: c_int = 9;
pub const RG_HPINPUTSTBENH_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HPINPUTRESET0_VAUDP32_SFT: c_int = 10;
pub const RG_HPINPUTRESET0_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HPOUTPUTRESET0_VAUDP32_SFT: c_int = 11;
pub const RG_HPOUTPUTRESET0_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HPPSHORT2VCM_VAUDP32_SFT: c_int = 12;
pub const RG_HPPSHORT2VCM_VAUDP32_MASK: c_uint = 0x7;

pub const RG_AUDHPTRIM_EN_VAUDP32_SFT: c_int = 15;
pub const RG_AUDHPTRIM_EN_VAUDP32_MASK: c_uint = 0x1;

// AUDDEC_ANA_CON3
pub const RG_AUDHPLTRIM_VAUDP32_SFT: c_int = 0;
pub const RG_AUDHPLTRIM_VAUDP32_MASK: c_uint = 0x1f;

pub const RG_AUDHPLFINETRIM_VAUDP32_SFT: c_int = 5;
pub const RG_AUDHPLFINETRIM_VAUDP32_MASK: c_uint = 0x7;

pub const RG_AUDHPRTRIM_VAUDP32_SFT: c_int = 8;
pub const RG_AUDHPRTRIM_VAUDP32_MASK: c_uint = 0x1f;

pub const RG_AUDHPRFINETRIM_VAUDP32_SFT: c_int = 13;
pub const RG_AUDHPRFINETRIM_VAUDP32_MASK: c_uint = 0x7;

// AUDDEC_ANA_CON4
pub const RG_AUDHPDIFFINPBIASADJ_VAUDP32_SFT: c_int = 0;
pub const RG_AUDHPDIFFINPBIASADJ_VAUDP32_MASK: c_uint = 0x7;

pub const RG_AUDHPLFCOMPRESSEL_VAUDP32_SFT: c_int = 4;
pub const RG_AUDHPLFCOMPRESSEL_VAUDP32_MASK: c_uint = 0x7;

pub const RG_AUDHPHFCOMPRESSEL_VAUDP32_SFT: c_int = 8;
pub const RG_AUDHPHFCOMPRESSEL_VAUDP32_MASK: c_uint = 0x7;

pub const RG_AUDHPHFCOMPBUFGAINSEL_VAUDP32_SFT: c_int = 12;
pub const RG_AUDHPHFCOMPBUFGAINSEL_VAUDP32_MASK: c_uint = 0x3;

pub const RG_AUDHPCOMP_EN_VAUDP32_SFT: c_int = 15;
pub const RG_AUDHPCOMP_EN_VAUDP32_MASK: c_uint = 0x1;

// AUDDEC_ANA_CON5
pub const RG_AUDHPDECMGAINADJ_VAUDP32_SFT: c_int = 0;
pub const RG_AUDHPDECMGAINADJ_VAUDP32_MASK: c_uint = 0x7;

pub const RG_AUDHPDEDMGAINADJ_VAUDP32_SFT: c_int = 4;
pub const RG_AUDHPDEDMGAINADJ_VAUDP32_MASK: c_uint = 0x7;

// AUDDEC_ANA_CON6
pub const RG_AUDHSPWRUP_VAUDP32_SFT: c_int = 0;
pub const RG_AUDHSPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHSPWRUP_IBIAS_VAUDP32_SFT: c_int = 1;
pub const RG_AUDHSPWRUP_IBIAS_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHSMUXINPUTSEL_VAUDP32_SFT: c_int = 2;
pub const RG_AUDHSMUXINPUTSEL_VAUDP32_MASK: c_uint = 0x3;

pub const RG_AUDHSSCDISABLE_VAUDP32_SFT: c_int = 4;
pub const RG_AUDHSSCDISABLE_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHSBSCCURRENT_VAUDP32_SFT: c_int = 5;
pub const RG_AUDHSBSCCURRENT_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHSSTARTUP_VAUDP32_SFT: c_int = 6;
pub const RG_AUDHSSTARTUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HSOUTPUTSTBENH_VAUDP32_SFT: c_int = 7;
pub const RG_HSOUTPUTSTBENH_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HSINPUTSTBENH_VAUDP32_SFT: c_int = 8;
pub const RG_HSINPUTSTBENH_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HSINPUTRESET0_VAUDP32_SFT: c_int = 9;
pub const RG_HSINPUTRESET0_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HSOUTPUTRESET0_VAUDP32_SFT: c_int = 10;
pub const RG_HSOUTPUTRESET0_VAUDP32_MASK: c_uint = 0x1;

pub const RG_HSOUT_SHORTVCM_VAUDP32_SFT: c_int = 11;
pub const RG_HSOUT_SHORTVCM_VAUDP32_MASK: c_uint = 0x1;

// AUDDEC_ANA_CON7
pub const RG_AUDLOLPWRUP_VAUDP32_SFT: c_int = 0;
pub const RG_AUDLOLPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDLOLPWRUP_IBIAS_VAUDP32_SFT: c_int = 1;
pub const RG_AUDLOLPWRUP_IBIAS_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDLOLMUXINPUTSEL_VAUDP32_SFT: c_int = 2;
pub const RG_AUDLOLMUXINPUTSEL_VAUDP32_MASK: c_uint = 0x3;

pub const RG_AUDLOLSCDISABLE_VAUDP32_SFT: c_int = 4;
pub const RG_AUDLOLSCDISABLE_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDLOLBSCCURRENT_VAUDP32_SFT: c_int = 5;
pub const RG_AUDLOLBSCCURRENT_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDLOSTARTUP_VAUDP32_SFT: c_int = 6;
pub const RG_AUDLOSTARTUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_LOINPUTSTBENH_VAUDP32_SFT: c_int = 7;
pub const RG_LOINPUTSTBENH_VAUDP32_MASK: c_uint = 0x1;

pub const RG_LOOUTPUTSTBENH_VAUDP32_SFT: c_int = 8;
pub const RG_LOOUTPUTSTBENH_VAUDP32_MASK: c_uint = 0x1;

pub const RG_LOINPUTRESET0_VAUDP32_SFT: c_int = 9;
pub const RG_LOINPUTRESET0_VAUDP32_MASK: c_uint = 0x1;

pub const RG_LOOUTPUTRESET0_VAUDP32_SFT: c_int = 10;
pub const RG_LOOUTPUTRESET0_VAUDP32_MASK: c_uint = 0x1;

pub const RG_LOOUT_SHORTVCM_VAUDP32_SFT: c_int = 11;
pub const RG_LOOUT_SHORTVCM_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDDACTPWRUP_VAUDP32_SFT: c_int = 12;
pub const RG_AUDDACTPWRUP_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUD_DAC_PWT_UP_VA32_SFT: c_int = 13;
pub const RG_AUD_DAC_PWT_UP_VA32_MASK: c_uint = 0x1;

// AUDDEC_ANA_CON8
pub const RG_AUDTRIMBUF_INPUTMUXSEL_VAUDP32_SFT: c_int = 0;
pub const RG_AUDTRIMBUF_INPUTMUXSEL_VAUDP32_MASK: c_uint = 0xf;

pub const RG_AUDTRIMBUF_GAINSEL_VAUDP32_SFT: c_int = 4;
pub const RG_AUDTRIMBUF_GAINSEL_VAUDP32_MASK: c_uint = 0x3;

pub const RG_AUDTRIMBUF_EN_VAUDP32_SFT: c_int = 6;
pub const RG_AUDTRIMBUF_EN_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDHPSPKDET_INPUTMUXSEL_VAUDP32_SFT: c_int = 8;
pub const RG_AUDHPSPKDET_INPUTMUXSEL_VAUDP32_MASK: c_uint = 0x3;

pub const RG_AUDHPSPKDET_OUTPUTMUXSEL_VAUDP32_SFT: c_int = 10;
pub const RG_AUDHPSPKDET_OUTPUTMUXSEL_VAUDP32_MASK: c_uint = 0x3;

pub const RG_AUDHPSPKDET_EN_VAUDP32_SFT: c_int = 12;
pub const RG_AUDHPSPKDET_EN_VAUDP32_MASK: c_uint = 0x1;

// AUDDEC_ANA_CON9
pub const RG_ABIDEC_RSVD0_VA32_SFT: c_int = 0;
pub const RG_ABIDEC_RSVD0_VA32_MASK: c_uint = 0xff;

pub const RG_ABIDEC_RSVD0_VAUDP32_SFT: c_int = 8;
pub const RG_ABIDEC_RSVD0_VAUDP32_MASK: c_uint = 0xff;

// AUDDEC_ANA_CON10
pub const RG_ABIDEC_RSVD1_VAUDP32_SFT: c_int = 0;
pub const RG_ABIDEC_RSVD1_VAUDP32_MASK: c_uint = 0xff;

pub const RG_ABIDEC_RSVD2_VAUDP32_SFT: c_int = 8;
pub const RG_ABIDEC_RSVD2_VAUDP32_MASK: c_uint = 0xff;

// AUDDEC_ANA_CON11
pub const RG_AUDZCDMUXSEL_VAUDP32_SFT: c_int = 0;
pub const RG_AUDZCDMUXSEL_VAUDP32_MASK: c_uint = 0x7;

pub const RG_AUDZCDCLKSEL_VAUDP32_SFT: c_int = 3;
pub const RG_AUDZCDCLKSEL_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDBIASADJ_0_VAUDP32_SFT: c_int = 7;
pub const RG_AUDBIASADJ_0_VAUDP32_MASK: c_uint = 0x1ff;

// AUDDEC_ANA_CON12
pub const RG_AUDBIASADJ_1_VAUDP32_SFT: c_int = 0;
pub const RG_AUDBIASADJ_1_VAUDP32_MASK: c_uint = 0xff;

pub const RG_AUDIBIASPWRDN_VAUDP32_SFT: c_int = 8;
pub const RG_AUDIBIASPWRDN_VAUDP32_MASK: c_uint = 0x1;

// AUDDEC_ANA_CON13
pub const RG_RSTB_DECODER_VA32_SFT: c_int = 0;
pub const RG_RSTB_DECODER_VA32_MASK: c_uint = 0x1;

pub const RG_SEL_DECODER_96K_VA32_SFT: c_int = 1;
pub const RG_SEL_DECODER_96K_VA32_MASK: c_uint = 0x1;

pub const RG_SEL_DELAY_VCORE_SFT: c_int = 2;
pub const RG_SEL_DELAY_VCORE_MASK: c_uint = 0x1;

pub const RG_AUDGLB_PWRDN_VA32_SFT: c_int = 4;
pub const RG_AUDGLB_PWRDN_VA32_MASK: c_uint = 0x1;

pub const RG_AUDGLB_LP_VOW_EN_VA32_SFT: c_int = 5;
pub const RG_AUDGLB_LP_VOW_EN_VA32_MASK: c_uint = 0x1;

pub const RG_AUDGLB_LP2_VOW_EN_VA32_SFT: c_int = 6;
pub const RG_AUDGLB_LP2_VOW_EN_VA32_MASK: c_uint = 0x1;

// AUDDEC_ANA_CON14
pub const RG_LCLDO_DEC_EN_VA32_SFT: c_int = 0;
pub const RG_LCLDO_DEC_EN_VA32_MASK: c_uint = 0x1;

pub const RG_LCLDO_DEC_PDDIS_EN_VA18_SFT: c_int = 1;
pub const RG_LCLDO_DEC_PDDIS_EN_VA18_MASK: c_uint = 0x1;

pub const RG_LCLDO_DEC_REMOTE_SENSE_VA18_SFT: c_int = 2;
pub const RG_LCLDO_DEC_REMOTE_SENSE_VA18_MASK: c_uint = 0x1;

pub const RG_NVREG_EN_VAUDP32_SFT: c_int = 4;
pub const RG_NVREG_EN_VAUDP32_MASK: c_uint = 0x1;

pub const RG_NVREG_PULL0V_VAUDP32_SFT: c_int = 5;
pub const RG_NVREG_PULL0V_VAUDP32_MASK: c_uint = 0x1;

pub const RG_AUDPMU_RSVD_VA18_SFT: c_int = 8;
pub const RG_AUDPMU_RSVD_VA18_MASK: c_uint = 0xff;

// MT6359_ZCD_CON0
pub const RG_AUDZCDENABLE_SFT: c_int = 0;
pub const RG_AUDZCDENABLE_MASK: c_uint = 0x1;

pub const RG_AUDZCDGAINSTEPTIME_SFT: c_int = 1;
pub const RG_AUDZCDGAINSTEPTIME_MASK: c_uint = 0x7;

pub const RG_AUDZCDGAINSTEPSIZE_SFT: c_int = 4;
pub const RG_AUDZCDGAINSTEPSIZE_MASK: c_uint = 0x3;

pub const RG_AUDZCDTIMEOUTMODESEL_SFT: c_int = 6;
pub const RG_AUDZCDTIMEOUTMODESEL_MASK: c_uint = 0x1;

// MT6359_ZCD_CON1
pub const RG_AUDLOLGAIN_SFT: c_int = 0;
pub const RG_AUDLOLGAIN_MASK: c_uint = 0x1f;

pub const RG_AUDLORGAIN_SFT: c_int = 7;
pub const RG_AUDLORGAIN_MASK: c_uint = 0x1f;

// MT6359_ZCD_CON2
pub const RG_AUDHPLGAIN_SFT: c_int = 0;
pub const RG_AUDHPLGAIN_MASK: c_uint = 0x1f;

pub const RG_AUDHPRGAIN_SFT: c_int = 7;
pub const RG_AUDHPRGAIN_MASK: c_uint = 0x1f;

// MT6359_ZCD_CON3
pub const RG_AUDHSGAIN_SFT: c_int = 0;
pub const RG_AUDHSGAIN_MASK: c_uint = 0x1f;

// MT6359_ZCD_CON4
pub const RG_AUDIVLGAIN_SFT: c_int = 0;
pub const RG_AUDIVLGAIN_MASK: c_uint = 0x7;

pub const RG_AUDIVRGAIN_SFT: c_int = 8;
pub const RG_AUDIVRGAIN_MASK: c_uint = 0x7;

// MT6359_ZCD_CON5
pub const RG_AUDINTGAIN1_SFT: c_int = 0;
pub const RG_AUDINTGAIN1_MASK: c_uint = 0x3f;

pub const RG_AUDINTGAIN2_SFT: c_int = 8;
pub const RG_AUDINTGAIN2_MASK: c_uint = 0x3f;

// audio register
pub const MT6359_GPIO_DIR0: c_uint = 0x88;
pub const MT6359_GPIO_DIR0_SET: c_uint = 0x8a;
pub const MT6359_GPIO_DIR0_CLR: c_uint = 0x8c;
pub const MT6359_GPIO_DIR1: c_uint = 0x8e;
pub const MT6359_GPIO_DIR1_SET: c_uint = 0x90;
pub const MT6359_GPIO_DIR1_CLR: c_uint = 0x92;
pub const MT6359_DCXO_CW11: c_uint = 0x7a6;
pub const MT6359_DCXO_CW12: c_uint = 0x7a8;
pub const MT6359_GPIO_MODE0: c_uint = 0xcc;
pub const MT6359_GPIO_MODE0_SET: c_uint = 0xce;
pub const MT6359_GPIO_MODE0_CLR: c_uint = 0xd0;
pub const MT6359_GPIO_MODE1: c_uint = 0xd2;
pub const MT6359_GPIO_MODE1_SET: c_uint = 0xd4;
pub const MT6359_GPIO_MODE1_CLR: c_uint = 0xd6;
pub const MT6359_GPIO_MODE2: c_uint = 0xd8;
pub const MT6359_GPIO_MODE2_SET: c_uint = 0xda;
pub const MT6359_GPIO_MODE2_CLR: c_uint = 0xdc;
pub const MT6359_GPIO_MODE3: c_uint = 0xde;
pub const MT6359_GPIO_MODE3_SET: c_uint = 0xe0;
pub const MT6359_GPIO_MODE3_CLR: c_uint = 0xe2;
pub const MT6359_GPIO_MODE4: c_uint = 0xe4;
pub const MT6359_GPIO_MODE4_SET: c_uint = 0xe6;
pub const MT6359_GPIO_MODE4_CLR: c_uint = 0xe8;
pub const MT6359_AUD_TOP_ID: c_uint = 0x2300;
pub const MT6359_AUD_TOP_REV0: c_uint = 0x2302;
pub const MT6359_AUD_TOP_DBI: c_uint = 0x2304;
pub const MT6359_AUD_TOP_DXI: c_uint = 0x2306;
pub const MT6359_AUD_TOP_CKPDN_TPM0: c_uint = 0x2308;
pub const MT6359_AUD_TOP_CKPDN_TPM1: c_uint = 0x230a;
pub const MT6359_AUD_TOP_CKPDN_CON0: c_uint = 0x230c;
pub const MT6359_AUD_TOP_CKPDN_CON0_SET: c_uint = 0x230e;
pub const MT6359_AUD_TOP_CKPDN_CON0_CLR: c_uint = 0x2310;
pub const MT6359_AUD_TOP_CKSEL_CON0: c_uint = 0x2312;
pub const MT6359_AUD_TOP_CKSEL_CON0_SET: c_uint = 0x2314;
pub const MT6359_AUD_TOP_CKSEL_CON0_CLR: c_uint = 0x2316;
pub const MT6359_AUD_TOP_CKTST_CON0: c_uint = 0x2318;
pub const MT6359_AUD_TOP_CLK_HWEN_CON0: c_uint = 0x231a;
pub const MT6359_AUD_TOP_CLK_HWEN_CON0_SET: c_uint = 0x231c;
pub const MT6359_AUD_TOP_CLK_HWEN_CON0_CLR: c_uint = 0x231e;
pub const MT6359_AUD_TOP_RST_CON0: c_uint = 0x2320;
pub const MT6359_AUD_TOP_RST_CON0_SET: c_uint = 0x2322;
pub const MT6359_AUD_TOP_RST_CON0_CLR: c_uint = 0x2324;
pub const MT6359_AUD_TOP_RST_BANK_CON0: c_uint = 0x2326;
pub const MT6359_AUD_TOP_INT_CON0: c_uint = 0x2328;
pub const MT6359_AUD_TOP_INT_CON0_SET: c_uint = 0x232a;
pub const MT6359_AUD_TOP_INT_CON0_CLR: c_uint = 0x232c;
pub const MT6359_AUD_TOP_INT_MASK_CON0: c_uint = 0x232e;
pub const MT6359_AUD_TOP_INT_MASK_CON0_SET: c_uint = 0x2330;
pub const MT6359_AUD_TOP_INT_MASK_CON0_CLR: c_uint = 0x2332;
pub const MT6359_AUD_TOP_INT_STATUS0: c_uint = 0x2334;
pub const MT6359_AUD_TOP_INT_RAW_STATUS0: c_uint = 0x2336;
pub const MT6359_AUD_TOP_INT_MISC_CON0: c_uint = 0x2338;
pub const MT6359_AUD_TOP_MON_CON0: c_uint = 0x233a;
pub const MT6359_AUDIO_DIG_DSN_ID: c_uint = 0x2380;
pub const MT6359_AUDIO_DIG_DSN_REV0: c_uint = 0x2382;
pub const MT6359_AUDIO_DIG_DSN_DBI: c_uint = 0x2384;
pub const MT6359_AUDIO_DIG_DSN_DXI: c_uint = 0x2386;
pub const MT6359_AFE_UL_DL_CON0: c_uint = 0x2388;
pub const MT6359_AFE_DL_SRC2_CON0_L: c_uint = 0x238a;
pub const MT6359_AFE_UL_SRC_CON0_H: c_uint = 0x238c;
pub const MT6359_AFE_UL_SRC_CON0_L: c_uint = 0x238e;
pub const MT6359_AFE_ADDA6_L_SRC_CON0_H: c_uint = 0x2390;
pub const MT6359_AFE_ADDA6_UL_SRC_CON0_L: c_uint = 0x2392;
pub const MT6359_AFE_TOP_CON0: c_uint = 0x2394;
pub const MT6359_AUDIO_TOP_CON0: c_uint = 0x2396;
pub const MT6359_AFE_MON_DEBUG0: c_uint = 0x2398;
pub const MT6359_AFUNC_AUD_CON0: c_uint = 0x239a;
pub const MT6359_AFUNC_AUD_CON1: c_uint = 0x239c;
pub const MT6359_AFUNC_AUD_CON2: c_uint = 0x239e;
pub const MT6359_AFUNC_AUD_CON3: c_uint = 0x23a0;
pub const MT6359_AFUNC_AUD_CON4: c_uint = 0x23a2;
pub const MT6359_AFUNC_AUD_CON5: c_uint = 0x23a4;
pub const MT6359_AFUNC_AUD_CON6: c_uint = 0x23a6;
pub const MT6359_AFUNC_AUD_CON7: c_uint = 0x23a8;
pub const MT6359_AFUNC_AUD_CON8: c_uint = 0x23aa;
pub const MT6359_AFUNC_AUD_CON9: c_uint = 0x23ac;
pub const MT6359_AFUNC_AUD_CON10: c_uint = 0x23ae;
pub const MT6359_AFUNC_AUD_CON11: c_uint = 0x23b0;
pub const MT6359_AFUNC_AUD_CON12: c_uint = 0x23b2;
pub const MT6359_AFUNC_AUD_MON0: c_uint = 0x23b4;
pub const MT6359_AFUNC_AUD_MON1: c_uint = 0x23b6;
pub const MT6359_AUDRC_TUNE_MON0: c_uint = 0x23b8;
pub const MT6359_AFE_ADDA_MTKAIF_FIFO_CFG0: c_uint = 0x23ba;
pub const MT6359_AFE_ADDA_MTKAIF_FIFO_LOG_MON1: c_uint = 0x23bc;
pub const MT6359_AFE_ADDA_MTKAIF_MON0: c_uint = 0x23be;
pub const MT6359_AFE_ADDA_MTKAIF_MON1: c_uint = 0x23c0;
pub const MT6359_AFE_ADDA_MTKAIF_MON2: c_uint = 0x23c2;
pub const MT6359_AFE_ADDA6_MTKAIF_MON3: c_uint = 0x23c4;
pub const MT6359_AFE_ADDA_MTKAIF_MON4: c_uint = 0x23c6;
pub const MT6359_AFE_ADDA_MTKAIF_MON5: c_uint = 0x23c8;
pub const MT6359_AFE_ADDA_MTKAIF_CFG0: c_uint = 0x23ca;
pub const MT6359_AFE_ADDA_MTKAIF_RX_CFG0: c_uint = 0x23cc;
pub const MT6359_AFE_ADDA_MTKAIF_RX_CFG1: c_uint = 0x23ce;
pub const MT6359_AFE_ADDA_MTKAIF_RX_CFG2: c_uint = 0x23d0;
pub const MT6359_AFE_ADDA_MTKAIF_RX_CFG3: c_uint = 0x23d2;
pub const MT6359_AFE_ADDA_MTKAIF_SYNCWORD_CFG0: c_uint = 0x23d4;
pub const MT6359_AFE_ADDA_MTKAIF_SYNCWORD_CFG1: c_uint = 0x23d6;
pub const MT6359_AFE_SGEN_CFG0: c_uint = 0x23d8;
pub const MT6359_AFE_SGEN_CFG1: c_uint = 0x23da;
pub const MT6359_AFE_ADC_ASYNC_FIFO_CFG: c_uint = 0x23dc;
pub const MT6359_AFE_ADC_ASYNC_FIFO_CFG1: c_uint = 0x23de;
pub const MT6359_AFE_DCCLK_CFG0: c_uint = 0x23e0;
pub const MT6359_AFE_DCCLK_CFG1: c_uint = 0x23e2;
pub const MT6359_AUDIO_DIG_CFG: c_uint = 0x23e4;
pub const MT6359_AUDIO_DIG_CFG1: c_uint = 0x23e6;
pub const MT6359_AFE_AUD_PAD_TOP: c_uint = 0x23e8;
pub const MT6359_AFE_AUD_PAD_TOP_MON: c_uint = 0x23ea;
pub const MT6359_AFE_AUD_PAD_TOP_MON1: c_uint = 0x23ec;
pub const MT6359_AFE_AUD_PAD_TOP_MON2: c_uint = 0x23ee;
pub const MT6359_AFE_DL_NLE_CFG: c_uint = 0x23f0;
pub const MT6359_AFE_DL_NLE_MON: c_uint = 0x23f2;
pub const MT6359_AFE_CG_EN_MON: c_uint = 0x23f4;
pub const MT6359_AFE_MIC_ARRAY_CFG: c_uint = 0x23f6;
pub const MT6359_AFE_CHOP_CFG0: c_uint = 0x23f8;
pub const MT6359_AFE_MTKAIF_MUX_CFG: c_uint = 0x23fa;
pub const MT6359_AUDIO_DIG_2ND_DSN_ID: c_uint = 0x2400;
pub const MT6359_AUDIO_DIG_2ND_DSN_REV0: c_uint = 0x2402;
pub const MT6359_AUDIO_DIG_2ND_DSN_DBI: c_uint = 0x2404;
pub const MT6359_AUDIO_DIG_2ND_DSN_DXI: c_uint = 0x2406;
pub const MT6359_AFE_PMIC_NEWIF_CFG3: c_uint = 0x2408;
pub const MT6359_AUDIO_DIG_3RD_DSN_ID: c_uint = 0x2480;
pub const MT6359_AUDIO_DIG_3RD_DSN_REV0: c_uint = 0x2482;
pub const MT6359_AUDIO_DIG_3RD_DSN_DBI: c_uint = 0x2484;
pub const MT6359_AUDIO_DIG_3RD_DSN_DXI: c_uint = 0x2486;
pub const MT6359_AFE_NCP_CFG0: c_uint = 0x24de;
pub const MT6359_AFE_NCP_CFG1: c_uint = 0x24e0;
pub const MT6359_AFE_NCP_CFG2: c_uint = 0x24e2;
pub const MT6359_AUDENC_DSN_ID: c_uint = 0x2500;
pub const MT6359_AUDENC_DSN_REV0: c_uint = 0x2502;
pub const MT6359_AUDENC_DSN_DBI: c_uint = 0x2504;
pub const MT6359_AUDENC_DSN_FPI: c_uint = 0x2506;
pub const MT6359_AUDENC_ANA_CON0: c_uint = 0x2508;
pub const MT6359_AUDENC_ANA_CON1: c_uint = 0x250a;
pub const MT6359_AUDENC_ANA_CON2: c_uint = 0x250c;
pub const MT6359_AUDENC_ANA_CON3: c_uint = 0x250e;
pub const MT6359_AUDENC_ANA_CON4: c_uint = 0x2510;
pub const MT6359_AUDENC_ANA_CON5: c_uint = 0x2512;
pub const MT6359_AUDENC_ANA_CON6: c_uint = 0x2514;
pub const MT6359_AUDENC_ANA_CON7: c_uint = 0x2516;
pub const MT6359_AUDENC_ANA_CON8: c_uint = 0x2518;
pub const MT6359_AUDENC_ANA_CON9: c_uint = 0x251a;
pub const MT6359_AUDENC_ANA_CON10: c_uint = 0x251c;
pub const MT6359_AUDENC_ANA_CON11: c_uint = 0x251e;
pub const MT6359_AUDENC_ANA_CON12: c_uint = 0x2520;
pub const MT6359_AUDENC_ANA_CON13: c_uint = 0x2522;
pub const MT6359_AUDENC_ANA_CON14: c_uint = 0x2524;
pub const MT6359_AUDENC_ANA_CON15: c_uint = 0x2526;
pub const MT6359_AUDENC_ANA_CON16: c_uint = 0x2528;
pub const MT6359_AUDENC_ANA_CON17: c_uint = 0x252a;
pub const MT6359_AUDENC_ANA_CON18: c_uint = 0x252c;
pub const MT6359_AUDENC_ANA_CON19: c_uint = 0x252e;
pub const MT6359_AUDENC_ANA_CON20: c_uint = 0x2530;
pub const MT6359_AUDENC_ANA_CON21: c_uint = 0x2532;
pub const MT6359_AUDENC_ANA_CON22: c_uint = 0x2534;
pub const MT6359_AUDENC_ANA_CON23: c_uint = 0x2536;
pub const MT6359_AUDDEC_DSN_ID: c_uint = 0x2580;
pub const MT6359_AUDDEC_DSN_REV0: c_uint = 0x2582;
pub const MT6359_AUDDEC_DSN_DBI: c_uint = 0x2584;
pub const MT6359_AUDDEC_DSN_FPI: c_uint = 0x2586;
pub const MT6359_AUDDEC_ANA_CON0: c_uint = 0x2588;
pub const MT6359_AUDDEC_ANA_CON1: c_uint = 0x258a;
pub const MT6359_AUDDEC_ANA_CON2: c_uint = 0x258c;
pub const MT6359_AUDDEC_ANA_CON3: c_uint = 0x258e;
pub const MT6359_AUDDEC_ANA_CON4: c_uint = 0x2590;
pub const MT6359_AUDDEC_ANA_CON5: c_uint = 0x2592;
pub const MT6359_AUDDEC_ANA_CON6: c_uint = 0x2594;
pub const MT6359_AUDDEC_ANA_CON7: c_uint = 0x2596;
pub const MT6359_AUDDEC_ANA_CON8: c_uint = 0x2598;
pub const MT6359_AUDDEC_ANA_CON9: c_uint = 0x259a;
pub const MT6359_AUDDEC_ANA_CON10: c_uint = 0x259c;
pub const MT6359_AUDDEC_ANA_CON11: c_uint = 0x259e;
pub const MT6359_AUDDEC_ANA_CON12: c_uint = 0x25a0;
pub const MT6359_AUDDEC_ANA_CON13: c_uint = 0x25a2;
pub const MT6359_AUDDEC_ANA_CON14: c_uint = 0x25a4;
pub const MT6359_AUDZCD_DSN_ID: c_uint = 0x2600;
pub const MT6359_AUDZCD_DSN_REV0: c_uint = 0x2602;
pub const MT6359_AUDZCD_DSN_DBI: c_uint = 0x2604;
pub const MT6359_AUDZCD_DSN_FPI: c_uint = 0x2606;
pub const MT6359_ZCD_CON0: c_uint = 0x2608;
pub const MT6359_ZCD_CON1: c_uint = 0x260a;
pub const MT6359_ZCD_CON2: c_uint = 0x260c;
pub const MT6359_ZCD_CON3: c_uint = 0x260e;
pub const MT6359_ZCD_CON4: c_uint = 0x2610;
pub const MT6359_ZCD_CON5: c_uint = 0x2612;
pub const MT6359_ACCDET_DSN_DIG_ID: c_uint = 0x2680;
pub const MT6359_ACCDET_DSN_DIG_REV0: c_uint = 0x2682;
pub const MT6359_ACCDET_DSN_DBI: c_uint = 0x2684;
pub const MT6359_ACCDET_DSN_FPI: c_uint = 0x2686;
pub const MT6359_ACCDET_CON0: c_uint = 0x2688;
pub const MT6359_ACCDET_CON1: c_uint = 0x268a;
pub const MT6359_ACCDET_CON2: c_uint = 0x268c;
pub const MT6359_ACCDET_CON3: c_uint = 0x268e;
pub const MT6359_ACCDET_CON4: c_uint = 0x2690;
pub const MT6359_ACCDET_CON5: c_uint = 0x2692;
pub const MT6359_ACCDET_CON6: c_uint = 0x2694;
pub const MT6359_ACCDET_CON7: c_uint = 0x2696;
pub const MT6359_ACCDET_CON8: c_uint = 0x2698;
pub const MT6359_ACCDET_CON9: c_uint = 0x269a;
pub const MT6359_ACCDET_CON10: c_uint = 0x269c;
pub const MT6359_ACCDET_CON11: c_uint = 0x269e;
pub const MT6359_ACCDET_CON12: c_uint = 0x26a0;
pub const MT6359_ACCDET_CON13: c_uint = 0x26a2;
pub const MT6359_ACCDET_CON14: c_uint = 0x26a4;
pub const MT6359_ACCDET_CON15: c_uint = 0x26a6;
pub const MT6359_ACCDET_CON16: c_uint = 0x26a8;
pub const MT6359_ACCDET_CON17: c_uint = 0x26aa;
pub const MT6359_ACCDET_CON18: c_uint = 0x26ac;
pub const MT6359_ACCDET_CON19: c_uint = 0x26ae;
pub const MT6359_ACCDET_CON20: c_uint = 0x26b0;
pub const MT6359_ACCDET_CON21: c_uint = 0x26b2;
pub const MT6359_ACCDET_CON22: c_uint = 0x26b4;
pub const MT6359_ACCDET_CON23: c_uint = 0x26b6;
pub const MT6359_ACCDET_CON24: c_uint = 0x26b8;
pub const MT6359_ACCDET_CON25: c_uint = 0x26ba;
pub const MT6359_ACCDET_CON26: c_uint = 0x26bc;
pub const MT6359_ACCDET_CON27: c_uint = 0x26be;
pub const MT6359_ACCDET_CON28: c_uint = 0x26c0;
pub const MT6359_ACCDET_CON29: c_uint = 0x26c2;
pub const MT6359_ACCDET_CON30: c_uint = 0x26c4;
pub const MT6359_ACCDET_CON31: c_uint = 0x26c6;
pub const MT6359_ACCDET_CON32: c_uint = 0x26c8;
pub const MT6359_ACCDET_CON33: c_uint = 0x26ca;
pub const MT6359_ACCDET_CON34: c_uint = 0x26cc;
pub const MT6359_ACCDET_CON35: c_uint = 0x26ce;
pub const MT6359_ACCDET_CON36: c_uint = 0x26d0;
pub const MT6359_ACCDET_CON37: c_uint = 0x26d2;
pub const MT6359_ACCDET_CON38: c_uint = 0x26d4;
pub const MT6359_ACCDET_CON39: c_uint = 0x26d6;
pub const MT6359_ACCDET_CON40: c_uint = 0x26d8;

// dl bias
pub const DRBIAS_MASK: c_uint = 0x7;

pub const IBIAS_MASK: c_uint = 0x3;

// dl gain

pub const DL_GAIN_REG_MASK: c_uint = 0x0f9f;
// mic type mux

// Supply widget subseq
// common
// playback
// capture
// dl pga gain
// Mic Type MUX
// UL SRC MUX
// MISO MUX
// DMIC MUX
// ADC L MUX
// PGA L MUX
// PGA R MUX
// PGA 3 MUX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6359_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub dl_rate: [c_uint; MT6359_AIF_NUM],
    pub ul_rate: [c_uint; MT6359_AIF_NUM],
    pub ana_gain: [c_int; AUDIO_ANALOG_VOLUME_TYPE_MAX],
    pub mux_select: [c_uint; MUX_NUM],
    pub dmic_one_wire_mode: c_uint,
    pub dev_counter: [c_int; DEVICE_NUM],
    pub hp_gain_ctl: c_int,
    pub hp_hifi_mode: c_int,
    pub mtkaif_protocol: c_int,
}

extern "C" {
    pub fn mt6359_mtkaif_calibration_enable(cmpnt: *mut snd_soc_component);
}
extern "C" {
    pub fn mt6359_mtkaif_calibration_disable(cmpnt: *mut snd_soc_component);
}
