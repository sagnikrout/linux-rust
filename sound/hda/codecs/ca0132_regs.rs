//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/codecs/ca0132_regs.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// HD audio codec driver for Creative CA0132 chip.
// CA0132 registers defines.
//
// Copyright (c) 2011, Creative Technology Ltd.
//
pub const DSP_CHIP_OFFSET: c_uint = 0x100000;
pub const DSP_DBGCNTL_MODULE_OFFSET: c_uint = 0xE30;

pub const DSP_DBGCNTL_EXEC_LOBIT: c_uint = 0x0;
pub const DSP_DBGCNTL_EXEC_HIBIT: c_uint = 0x3;
pub const DSP_DBGCNTL_EXEC_MASK: c_uint = 0xF;
pub const DSP_DBGCNTL_SS_LOBIT: c_uint = 0x4;
pub const DSP_DBGCNTL_SS_HIBIT: c_uint = 0x7;
pub const DSP_DBGCNTL_SS_MASK: c_uint = 0xF0;
pub const DSP_DBGCNTL_STATE_LOBIT: c_uint = 0xA;
pub const DSP_DBGCNTL_STATE_HIBIT: c_uint = 0xD;
pub const DSP_DBGCNTL_STATE_MASK: c_uint = 0x3C00;
pub const XRAM_CHIP_OFFSET: c_uint = 0x0;
pub const XRAM_XRAM_CHANNEL_COUNT: c_uint = 0xE000;
pub const XRAM_XRAM_MODULE_OFFSET: c_uint = 0x0;
pub const XRAM_XRAM_CHAN_INCR: c_int = 4;

pub const YRAM_CHIP_OFFSET: c_uint = 0x40000;
pub const YRAM_YRAM_CHANNEL_COUNT: c_uint = 0x8000;
pub const YRAM_YRAM_MODULE_OFFSET: c_uint = 0x0;
pub const YRAM_YRAM_CHAN_INCR: c_int = 4;

pub const UC_CHIP_OFFSET: c_uint = 0x80000;
pub const UC_UC_CHANNEL_COUNT: c_uint = 0x10000;
pub const UC_UC_MODULE_OFFSET: c_uint = 0x0;
pub const UC_UC_CHAN_INCR: c_int = 4;

pub const AXRAM_CHIP_OFFSET: c_uint = 0x3C000;
pub const AXRAM_AXRAM_CHANNEL_COUNT: c_uint = 0x1000;
pub const AXRAM_AXRAM_MODULE_OFFSET: c_uint = 0x0;
pub const AXRAM_AXRAM_CHAN_INCR: c_int = 4;

pub const AYRAM_CHIP_OFFSET: c_uint = 0x78000;
pub const AYRAM_AYRAM_CHANNEL_COUNT: c_uint = 0x1000;
pub const AYRAM_AYRAM_MODULE_OFFSET: c_uint = 0x0;
pub const AYRAM_AYRAM_CHAN_INCR: c_int = 4;

pub const DSPDMAC_CHIP_OFFSET: c_uint = 0x110000;
pub const DSPDMAC_DMA_CFG_CHANNEL_COUNT: c_int = 12;
pub const DSPDMAC_DMACFG_MODULE_OFFSET: c_uint = 0xF00;
pub const DSPDMAC_DMACFG_CHAN_INCR: c_uint = 0x10;

pub const DSPDMAC_DMACFG_DBADR_LOBIT: c_uint = 0x0;
pub const DSPDMAC_DMACFG_DBADR_HIBIT: c_uint = 0x10;
pub const DSPDMAC_DMACFG_DBADR_MASK: c_uint = 0x1FFFF;
pub const DSPDMAC_DMACFG_LP_LOBIT: c_uint = 0x11;
pub const DSPDMAC_DMACFG_LP_HIBIT: c_uint = 0x11;
pub const DSPDMAC_DMACFG_LP_MASK: c_uint = 0x20000;
pub const DSPDMAC_DMACFG_AINCR_LOBIT: c_uint = 0x12;
pub const DSPDMAC_DMACFG_AINCR_HIBIT: c_uint = 0x12;
pub const DSPDMAC_DMACFG_AINCR_MASK: c_uint = 0x40000;
pub const DSPDMAC_DMACFG_DWR_LOBIT: c_uint = 0x13;
pub const DSPDMAC_DMACFG_DWR_HIBIT: c_uint = 0x13;
pub const DSPDMAC_DMACFG_DWR_MASK: c_uint = 0x80000;
pub const DSPDMAC_DMACFG_AJUMP_LOBIT: c_uint = 0x14;
pub const DSPDMAC_DMACFG_AJUMP_HIBIT: c_uint = 0x17;
pub const DSPDMAC_DMACFG_AJUMP_MASK: c_uint = 0xF00000;
pub const DSPDMAC_DMACFG_AMODE_LOBIT: c_uint = 0x18;
pub const DSPDMAC_DMACFG_AMODE_HIBIT: c_uint = 0x19;
pub const DSPDMAC_DMACFG_AMODE_MASK: c_uint = 0x3000000;
pub const DSPDMAC_DMACFG_LK_LOBIT: c_uint = 0x1A;
pub const DSPDMAC_DMACFG_LK_HIBIT: c_uint = 0x1A;
pub const DSPDMAC_DMACFG_LK_MASK: c_uint = 0x4000000;
pub const DSPDMAC_DMACFG_AICS_LOBIT: c_uint = 0x1B;
pub const DSPDMAC_DMACFG_AICS_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_DMACFG_AICS_MASK: c_uint = 0xF8000000;
pub const DSPDMAC_DMACFG_LP_SINGLE: c_int = 0;
pub const DSPDMAC_DMACFG_LP_LOOPING: c_int = 1;
pub const DSPDMAC_DMACFG_AINCR_XANDY: c_int = 0;
pub const DSPDMAC_DMACFG_AINCR_XORY: c_int = 1;
pub const DSPDMAC_DMACFG_DWR_DMA_RD: c_int = 0;
pub const DSPDMAC_DMACFG_DWR_DMA_WR: c_int = 1;
pub const DSPDMAC_DMACFG_AMODE_LINEAR: c_int = 0;
pub const DSPDMAC_DMACFG_AMODE_RSV1: c_int = 1;
pub const DSPDMAC_DMACFG_AMODE_WINTLV: c_int = 2;
pub const DSPDMAC_DMACFG_AMODE_GINTLV: c_int = 3;
pub const DSPDMAC_DSP_ADR_OFS_CHANNEL_COUNT: c_int = 12;
pub const DSPDMAC_DSPADROFS_MODULE_OFFSET: c_uint = 0xF04;
pub const DSPDMAC_DSPADROFS_CHAN_INCR: c_uint = 0x10;

pub const DSPDMAC_DSPADROFS_COFS_LOBIT: c_uint = 0x0;
pub const DSPDMAC_DSPADROFS_COFS_HIBIT: c_uint = 0xF;
pub const DSPDMAC_DSPADROFS_COFS_MASK: c_uint = 0xFFFF;
pub const DSPDMAC_DSPADROFS_BOFS_LOBIT: c_uint = 0x10;
pub const DSPDMAC_DSPADROFS_BOFS_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_DSPADROFS_BOFS_MASK: c_uint = 0xFFFF0000;
pub const DSPDMAC_DSP_ADR_WOFS_CHANNEL_COUNT: c_int = 12;
pub const DSPDMAC_DSPADRWOFS_MODULE_OFFSET: c_uint = 0xF04;
pub const DSPDMAC_DSPADRWOFS_CHAN_INCR: c_uint = 0x10;

pub const DSPDMAC_DSPADRWOFS_WCOFS_LOBIT: c_uint = 0x0;
pub const DSPDMAC_DSPADRWOFS_WCOFS_HIBIT: c_uint = 0xA;
pub const DSPDMAC_DSPADRWOFS_WCOFS_MASK: c_uint = 0x7FF;
pub const DSPDMAC_DSPADRWOFS_WCBFR_LOBIT: c_uint = 0xB;
pub const DSPDMAC_DSPADRWOFS_WCBFR_HIBIT: c_uint = 0xF;
pub const DSPDMAC_DSPADRWOFS_WCBFR_MASK: c_uint = 0xF800;
pub const DSPDMAC_DSPADRWOFS_WBOFS_LOBIT: c_uint = 0x10;
pub const DSPDMAC_DSPADRWOFS_WBOFS_HIBIT: c_uint = 0x1A;
pub const DSPDMAC_DSPADRWOFS_WBOFS_MASK: c_uint = 0x7FF0000;
pub const DSPDMAC_DSPADRWOFS_WBBFR_LOBIT: c_uint = 0x1B;
pub const DSPDMAC_DSPADRWOFS_WBBFR_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_DSPADRWOFS_WBBFR_MASK: c_uint = 0xF8000000;
pub const DSPDMAC_DSP_ADR_GOFS_CHANNEL_COUNT: c_int = 12;
pub const DSPDMAC_DSPADRGOFS_MODULE_OFFSET: c_uint = 0xF04;
pub const DSPDMAC_DSPADRGOFS_CHAN_INCR: c_uint = 0x10;

pub const DSPDMAC_DSPADRGOFS_GCOFS_LOBIT: c_uint = 0x0;
pub const DSPDMAC_DSPADRGOFS_GCOFS_HIBIT: c_uint = 0x9;
pub const DSPDMAC_DSPADRGOFS_GCOFS_MASK: c_uint = 0x3FF;
pub const DSPDMAC_DSPADRGOFS_GCS_LOBIT: c_uint = 0xA;
pub const DSPDMAC_DSPADRGOFS_GCS_HIBIT: c_uint = 0xC;
pub const DSPDMAC_DSPADRGOFS_GCS_MASK: c_uint = 0x1C00;
pub const DSPDMAC_DSPADRGOFS_GCBFR_LOBIT: c_uint = 0xD;
pub const DSPDMAC_DSPADRGOFS_GCBFR_HIBIT: c_uint = 0xF;
pub const DSPDMAC_DSPADRGOFS_GCBFR_MASK: c_uint = 0xE000;
pub const DSPDMAC_DSPADRGOFS_GBOFS_LOBIT: c_uint = 0x10;
pub const DSPDMAC_DSPADRGOFS_GBOFS_HIBIT: c_uint = 0x19;
pub const DSPDMAC_DSPADRGOFS_GBOFS_MASK: c_uint = 0x3FF0000;
pub const DSPDMAC_DSPADRGOFS_GBS_LOBIT: c_uint = 0x1A;
pub const DSPDMAC_DSPADRGOFS_GBS_HIBIT: c_uint = 0x1C;
pub const DSPDMAC_DSPADRGOFS_GBS_MASK: c_uint = 0x1C000000;
pub const DSPDMAC_DSPADRGOFS_GBBFR_LOBIT: c_uint = 0x1D;
pub const DSPDMAC_DSPADRGOFS_GBBFR_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_DSPADRGOFS_GBBFR_MASK: c_uint = 0xE0000000;
pub const DSPDMAC_XFR_CNT_CHANNEL_COUNT: c_int = 12;
pub const DSPDMAC_XFRCNT_MODULE_OFFSET: c_uint = 0xF08;
pub const DSPDMAC_XFRCNT_CHAN_INCR: c_uint = 0x10;

pub const DSPDMAC_XFRCNT_CCNT_LOBIT: c_uint = 0x0;
pub const DSPDMAC_XFRCNT_CCNT_HIBIT: c_uint = 0xF;
pub const DSPDMAC_XFRCNT_CCNT_MASK: c_uint = 0xFFFF;
pub const DSPDMAC_XFRCNT_BCNT_LOBIT: c_uint = 0x10;
pub const DSPDMAC_XFRCNT_BCNT_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_XFRCNT_BCNT_MASK: c_uint = 0xFFFF0000;
pub const DSPDMAC_IRQ_CNT_CHANNEL_COUNT: c_int = 12;
pub const DSPDMAC_IRQCNT_MODULE_OFFSET: c_uint = 0xF0C;
pub const DSPDMAC_IRQCNT_CHAN_INCR: c_uint = 0x10;

pub const DSPDMAC_IRQCNT_CICNT_LOBIT: c_uint = 0x0;
pub const DSPDMAC_IRQCNT_CICNT_HIBIT: c_uint = 0xF;
pub const DSPDMAC_IRQCNT_CICNT_MASK: c_uint = 0xFFFF;
pub const DSPDMAC_IRQCNT_BICNT_LOBIT: c_uint = 0x10;
pub const DSPDMAC_IRQCNT_BICNT_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_IRQCNT_BICNT_MASK: c_uint = 0xFFFF0000;
pub const DSPDMAC_AUD_CHSEL_CHANNEL_COUNT: c_int = 12;
pub const DSPDMAC_AUDCHSEL_MODULE_OFFSET: c_uint = 0xFC0;
pub const DSPDMAC_AUDCHSEL_CHAN_INCR: c_uint = 0x4;

pub const DSPDMAC_AUDCHSEL_ACS_LOBIT: c_uint = 0x0;
pub const DSPDMAC_AUDCHSEL_ACS_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_AUDCHSEL_ACS_MASK: c_uint = 0xFFFFFFFF;
pub const DSPDMAC_CHNLSTART_MODULE_OFFSET: c_uint = 0xFF0;

pub const DSPDMAC_CHNLSTART_EN_LOBIT: c_uint = 0x0;
pub const DSPDMAC_CHNLSTART_EN_HIBIT: c_uint = 0xB;
pub const DSPDMAC_CHNLSTART_EN_MASK: c_uint = 0xFFF;
pub const DSPDMAC_CHNLSTART_VAI1_LOBIT: c_uint = 0xC;
pub const DSPDMAC_CHNLSTART_VAI1_HIBIT: c_uint = 0xF;
pub const DSPDMAC_CHNLSTART_VAI1_MASK: c_uint = 0xF000;
pub const DSPDMAC_CHNLSTART_DIS_LOBIT: c_uint = 0x10;
pub const DSPDMAC_CHNLSTART_DIS_HIBIT: c_uint = 0x1B;
pub const DSPDMAC_CHNLSTART_DIS_MASK: c_uint = 0xFFF0000;
pub const DSPDMAC_CHNLSTART_VAI2_LOBIT: c_uint = 0x1C;
pub const DSPDMAC_CHNLSTART_VAI2_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_CHNLSTART_VAI2_MASK: c_uint = 0xF0000000;
pub const DSPDMAC_CHNLSTATUS_MODULE_OFFSET: c_uint = 0xFF4;

pub const DSPDMAC_CHNLSTATUS_ISC_LOBIT: c_uint = 0x0;
pub const DSPDMAC_CHNLSTATUS_ISC_HIBIT: c_uint = 0xB;
pub const DSPDMAC_CHNLSTATUS_ISC_MASK: c_uint = 0xFFF;
pub const DSPDMAC_CHNLSTATUS_AOO_LOBIT: c_uint = 0xC;
pub const DSPDMAC_CHNLSTATUS_AOO_HIBIT: c_uint = 0xC;
pub const DSPDMAC_CHNLSTATUS_AOO_MASK: c_uint = 0x1000;
pub const DSPDMAC_CHNLSTATUS_AOU_LOBIT: c_uint = 0xD;
pub const DSPDMAC_CHNLSTATUS_AOU_HIBIT: c_uint = 0xD;
pub const DSPDMAC_CHNLSTATUS_AOU_MASK: c_uint = 0x2000;
pub const DSPDMAC_CHNLSTATUS_AIO_LOBIT: c_uint = 0xE;
pub const DSPDMAC_CHNLSTATUS_AIO_HIBIT: c_uint = 0xE;
pub const DSPDMAC_CHNLSTATUS_AIO_MASK: c_uint = 0x4000;
pub const DSPDMAC_CHNLSTATUS_AIU_LOBIT: c_uint = 0xF;
pub const DSPDMAC_CHNLSTATUS_AIU_HIBIT: c_uint = 0xF;
pub const DSPDMAC_CHNLSTATUS_AIU_MASK: c_uint = 0x8000;
pub const DSPDMAC_CHNLSTATUS_IEN_LOBIT: c_uint = 0x10;
pub const DSPDMAC_CHNLSTATUS_IEN_HIBIT: c_uint = 0x1B;
pub const DSPDMAC_CHNLSTATUS_IEN_MASK: c_uint = 0xFFF0000;
pub const DSPDMAC_CHNLSTATUS_VAI0_LOBIT: c_uint = 0x1C;
pub const DSPDMAC_CHNLSTATUS_VAI0_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_CHNLSTATUS_VAI0_MASK: c_uint = 0xF0000000;
pub const DSPDMAC_CHNLPROP_MODULE_OFFSET: c_uint = 0xFF8;

pub const DSPDMAC_CHNLPROP_DCON_LOBIT: c_uint = 0x0;
pub const DSPDMAC_CHNLPROP_DCON_HIBIT: c_uint = 0xB;
pub const DSPDMAC_CHNLPROP_DCON_MASK: c_uint = 0xFFF;
pub const DSPDMAC_CHNLPROP_FFS_LOBIT: c_uint = 0xC;
pub const DSPDMAC_CHNLPROP_FFS_HIBIT: c_uint = 0xC;
pub const DSPDMAC_CHNLPROP_FFS_MASK: c_uint = 0x1000;
pub const DSPDMAC_CHNLPROP_NAJ_LOBIT: c_uint = 0xD;
pub const DSPDMAC_CHNLPROP_NAJ_HIBIT: c_uint = 0xD;
pub const DSPDMAC_CHNLPROP_NAJ_MASK: c_uint = 0x2000;
pub const DSPDMAC_CHNLPROP_ENH_LOBIT: c_uint = 0xE;
pub const DSPDMAC_CHNLPROP_ENH_HIBIT: c_uint = 0xE;
pub const DSPDMAC_CHNLPROP_ENH_MASK: c_uint = 0x4000;
pub const DSPDMAC_CHNLPROP_MSPCE_LOBIT: c_uint = 0x10;
pub const DSPDMAC_CHNLPROP_MSPCE_HIBIT: c_uint = 0x1B;
pub const DSPDMAC_CHNLPROP_MSPCE_MASK: c_uint = 0xFFF0000;
pub const DSPDMAC_CHNLPROP_AC_LOBIT: c_uint = 0x1C;
pub const DSPDMAC_CHNLPROP_AC_HIBIT: c_uint = 0x1F;
pub const DSPDMAC_CHNLPROP_AC_MASK: c_uint = 0xF0000000;
pub const DSPDMAC_ACTIVE_MODULE_OFFSET: c_uint = 0xFFC;

pub const DSPDMAC_ACTIVE_AAR_LOBIT: c_uint = 0x0;
pub const DSPDMAC_ACTIVE_AAR_HIBIT: c_uint = 0xB;
pub const DSPDMAC_ACTIVE_AAR_MASK: c_uint = 0xFFF;
pub const DSPDMAC_ACTIVE_WFR_LOBIT: c_uint = 0xC;
pub const DSPDMAC_ACTIVE_WFR_HIBIT: c_uint = 0x17;
pub const DSPDMAC_ACTIVE_WFR_MASK: c_uint = 0xFFF000;
pub const DSP_AUX_MEM_BASE: c_uint = 0xE000;

pub const U64K: c_uint = 0x10000UL;

