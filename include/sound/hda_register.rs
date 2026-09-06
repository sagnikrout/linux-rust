//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hda_register.h
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
// HD-audio controller (Azalia) registers and helpers
//
// For traditional reasons, we still use azx_ prefix here
//

pub const AZX_REG_GCAP: c_uint = 0x00;

pub const AZX_REG_VMIN: c_uint = 0x02;
pub const AZX_REG_VMAJ: c_uint = 0x03;
pub const AZX_REG_OUTPAY: c_uint = 0x04;
pub const AZX_REG_INPAY: c_uint = 0x06;
pub const AZX_REG_GCTL: c_uint = 0x08;

pub const AZX_REG_WAKEEN: c_uint = 0x0c;
pub const AZX_REG_STATESTS: c_uint = 0x0e;
pub const AZX_REG_GSTS: c_uint = 0x10;

pub const AZX_REG_GCAP2: c_uint = 0x12;
pub const AZX_REG_LLCH: c_uint = 0x14;
pub const AZX_REG_OUTSTRMPAY: c_uint = 0x18;
pub const AZX_REG_INSTRMPAY: c_uint = 0x1A;
pub const AZX_REG_INTCTL: c_uint = 0x20;
pub const AZX_REG_INTSTS: c_uint = 0x24;
pub const AZX_REG_WALLCLK: c_uint = 0x30	/* 24Mhz source */;
pub const AZX_REG_OLD_SSYNC: c_uint = 0x34	/* SSYNC for old ICH */;
pub const AZX_REG_SSYNC: c_uint = 0x38;
pub const AZX_REG_CORBLBASE: c_uint = 0x40;
pub const AZX_REG_CORBUBASE: c_uint = 0x44;
pub const AZX_REG_CORBWP: c_uint = 0x48;
pub const AZX_REG_CORBRP: c_uint = 0x4a;

pub const AZX_REG_CORBCTL: c_uint = 0x4c;

pub const AZX_REG_CORBSTS: c_uint = 0x4d;

pub const AZX_REG_CORBSIZE: c_uint = 0x4e;
pub const AZX_REG_RIRBLBASE: c_uint = 0x50;
pub const AZX_REG_RIRBUBASE: c_uint = 0x54;
pub const AZX_REG_RIRBWP: c_uint = 0x58;

pub const AZX_REG_RINTCNT: c_uint = 0x5a;
pub const AZX_REG_RIRBCTL: c_uint = 0x5c;

pub const AZX_REG_RIRBSTS: c_uint = 0x5d;

pub const AZX_REG_RIRBSIZE: c_uint = 0x5e;
pub const AZX_REG_IC: c_uint = 0x60;
pub const AZX_REG_IR: c_uint = 0x64;
pub const AZX_REG_IRS: c_uint = 0x68;

pub const AZX_REG_DPLBASE: c_uint = 0x70;
pub const AZX_REG_DPUBASE: c_uint = 0x74;
pub const AZX_DPLBASE_ENABLE: c_uint = 0x1	/* Enable position buffer */;
// SD offset: SDI0=0x80, SDI1=0xa0, ... SDO3=0x160
// stream register offsets from stream base
pub const AZX_REG_SD_CTL: c_uint = 0x00;
pub const AZX_REG_SD_CTL_3B: c_uint = 0x02 /* 3rd byte of SD_CTL register */;
pub const AZX_REG_SD_STS: c_uint = 0x03;
pub const AZX_REG_SD_LPIB: c_uint = 0x04;
pub const AZX_REG_SD_CBL: c_uint = 0x08;
pub const AZX_REG_SD_LVI: c_uint = 0x0c;
pub const AZX_REG_SD_FIFOW: c_uint = 0x0e;
pub const AZX_REG_SD_FIFOSIZE: c_uint = 0x10;
pub const AZX_REG_SD_FORMAT: c_uint = 0x12;
pub const AZX_REG_SD_FIFOL: c_uint = 0x14;
pub const AZX_REG_SD_BDLPL: c_uint = 0x18;
pub const AZX_REG_SD_BDLPU: c_uint = 0x1c;

// GTS registers
pub const AZX_REG_LLCH: c_uint = 0x14;
pub const AZX_REG_GTS_BASE: c_uint = 0x520;

// Haswell/Broadwell display HD-A controller Extended Mode registers
pub const AZX_REG_HSW_EM4: c_uint = 0x100c;
pub const AZX_REG_HSW_EM5: c_uint = 0x1010;
// Skylake/Broxton vendor-specific registers
pub const AZX_REG_VS_EM1: c_uint = 0x1000;
pub const AZX_REG_VS_INRC: c_uint = 0x1004;
pub const AZX_REG_VS_OUTRC: c_uint = 0x1008;
pub const AZX_REG_VS_FIFOTRK: c_uint = 0x100C;
pub const AZX_REG_VS_FIFOTRK2: c_uint = 0x1010;
pub const AZX_REG_VS_EM2: c_uint = 0x1030;
pub const AZX_REG_VS_EM3L: c_uint = 0x1038;
pub const AZX_REG_VS_EM3U: c_uint = 0x103C;
pub const AZX_REG_VS_EM4L: c_uint = 0x1040;
pub const AZX_REG_VS_EM4U: c_uint = 0x1044;
pub const AZX_REG_VS_LTRP: c_uint = 0x1048;
pub const AZX_REG_VS_D0I3C: c_uint = 0x104A;
pub const AZX_REG_VS_PCE: c_uint = 0x104B;
pub const AZX_REG_VS_L2MAGC: c_uint = 0x1050;
pub const AZX_REG_VS_L2LAHPT: c_uint = 0x1054;
pub const AZX_REG_VS_SDXDPIB_XBASE: c_uint = 0x1084;
pub const AZX_REG_VS_SDXDPIB_XINTERVAL: c_uint = 0x20;
pub const AZX_REG_VS_SDXEFIFOS_XBASE: c_uint = 0x1094;
pub const AZX_REG_VS_SDXEFIFOS_XINTERVAL: c_uint = 0x20;

// PCI space
pub const AZX_PCIREG_TCSEL: c_uint = 0x44;
//
// other constants
//
// max number of fragments - we may use more if allocating more pages for BDL
pub const BDL_SIZE: c_int = 4096;

pub const AZX_MAX_FRAG: c_int = 32;
//
// max buffer size - artificial 4MB limit per stream to avoid big allocations
// In theory it can be really big, but as it is per stream on systems with many streams memory could
// be quickly saturated if userspace requests maximum buffer size for each of them.
//

// RIRB int mask: overrun[2], response[0]
pub const RIRB_INT_RESPONSE: c_uint = 0x01;
pub const RIRB_INT_OVERRUN: c_uint = 0x04;
pub const RIRB_INT_MASK: c_uint = 0x05;
// STATESTS int mask: S3,SD2,SD1,SD0

// SD_CTL bits
pub const SD_CTL_STREAM_RESET: c_uint = 0x01	/* stream reset bit */;
pub const SD_CTL_DMA_START: c_uint = 0x02	/* stream DMA start bit */;

pub const SD_CTL_STREAM_TAG_SHIFT: c_int = 20;
// SD_CTL and SD_STS
pub const SD_INT_DESC_ERR: c_uint = 0x10	/* descriptor error interrupt */;
pub const SD_INT_FIFO_ERR: c_uint = 0x08	/* FIFO error interrupt */;
pub const SD_INT_COMPLETE: c_uint = 0x04	/* completion interrupt */;

pub const SD_CTL_STRIPE_MASK: c_uint = 0x3	/* stripe control mask */;
// SD_STS
pub const SD_STS_FIFO_READY: c_uint = 0x20	/* FIFO ready */;
// INTCTL and INTSTS
pub const AZX_INT_ALL_STREAM: c_uint = 0x3fffffff	   /* all stream interrupts */;
pub const AZX_INT_CTRL_EN: c_uint = 0x40000000 /* controller interrupt enable bit */;
pub const AZX_INT_GLOBAL_EN: c_uint = 0x80000000 /* global interrupt enable bit */;
// below are so far hardcoded - should read registers in future
pub const AZX_MAX_CORB_ENTRIES: c_int = 256;
pub const AZX_MAX_RIRB_ENTRIES: c_int = 256;
// Capability header  Structure
pub const AZX_REG_CAP_HDR: c_uint = 0x0;
pub const AZX_CAP_HDR_VER_OFF: c_int = 28;

pub const AZX_CAP_HDR_ID_OFF: c_int = 16;

pub const AZX_CAP_HDR_NXT_PTR_MASK: c_uint = 0xFFFF;
// registers of Software Position Based FIFO Capability Structure
pub const AZX_SPB_CAP_ID: c_uint = 0x4;
pub const AZX_REG_SPB_BASE_ADDR: c_uint = 0x700;
pub const AZX_REG_SPB_SPBFCH: c_uint = 0x00;
pub const AZX_REG_SPB_SPBFCCTL: c_uint = 0x04;
// Base used to calculate the iterating register offset
pub const AZX_SPB_BASE: c_uint = 0x08;
// Interval used to calculate the iterating register offset
pub const AZX_SPB_INTERVAL: c_uint = 0x08;
// SPIB base
pub const AZX_SPB_SPIB: c_uint = 0x00;
// SPIB MAXFIFO base
pub const AZX_SPB_MAXFIFO: c_uint = 0x04;
// registers of Global Time Synchronization Capability Structure
pub const AZX_GTS_CAP_ID: c_uint = 0x1;
pub const AZX_REG_GTS_GTSCH: c_uint = 0x00;
pub const AZX_REG_GTS_GTSCD: c_uint = 0x04;
pub const AZX_REG_GTS_GTSCTLAC: c_uint = 0x0C;
pub const AZX_GTS_BASE: c_uint = 0x20;
pub const AZX_GTS_INTERVAL: c_uint = 0x20;
// registers for Processing Pipe Capability Structure
pub const AZX_PP_CAP_ID: c_uint = 0x3;
pub const AZX_REG_PP_PPCH: c_uint = 0x10;
pub const AZX_REG_PP_PPCTL: c_uint = 0x04;

// _X_ = dma engine # and cannot * exceed 29 (per spec max 30 dma engines)

pub const AZX_REG_PP_PPSTS: c_uint = 0x08;
pub const AZX_PPHC_BASE: c_uint = 0x10;
pub const AZX_PPHC_INTERVAL: c_uint = 0x10;
pub const AZX_REG_PPHCLLPL: c_uint = 0x0;
pub const AZX_REG_PPHCLLPU: c_uint = 0x4;
pub const AZX_REG_PPHCLDPL: c_uint = 0x8;
pub const AZX_REG_PPHCLDPU: c_uint = 0xC;
pub const AZX_PPLC_BASE: c_uint = 0x10;
pub const AZX_PPLC_MULTI: c_uint = 0x10;
pub const AZX_PPLC_INTERVAL: c_uint = 0x10;
pub const AZX_REG_PPLCCTL: c_uint = 0x0;
pub const AZX_PPLCCTL_STRM_BITS: c_int = 4;
pub const AZX_PPLCCTL_STRM_SHIFT: c_int = 20;

pub const AZX_REG_PPLCFMT: c_uint = 0x4;
pub const AZX_REG_PPLCLLPL: c_uint = 0x8;
pub const AZX_REG_PPLCLLPU: c_uint = 0xC;
// registers for Multiple Links Capability Structure
pub const AZX_ML_CAP_ID: c_uint = 0x2;
pub const AZX_REG_ML_MLCH: c_uint = 0x00;
pub const AZX_REG_ML_MLCD: c_uint = 0x04;
pub const AZX_ML_BASE: c_uint = 0x40;
pub const AZX_ML_INTERVAL: c_uint = 0x40;
// HDaudio registers valid for HDaudio and HDaudio extended links
pub const AZX_REG_ML_LCAP: c_uint = 0x00;

pub const AZX_ML_HDA_LCAP_ALT_HDA: c_uint = 0x0;
pub const AZX_ML_HDA_LCAP_ALT_HDA_EXT: c_uint = 0x1;

pub const AZX_REG_ML_LCTL: c_uint = 0x04;

pub const AZX_ML_LCTL_CPA_SHIFT: c_int = 23;

pub const AZX_ML_LCTL_SPA_SHIFT: c_int = 16;

pub const AZX_REG_ML_LOSIDV: c_uint = 0x08;
// bit0 is reserved, with BIT(1) mapping to stream1
pub const AZX_ML_LOSIDV_STREAM_MASK: c_uint = 0xFFFE;
pub const AZX_REG_ML_LSDIID: c_uint = 0x0C;

// HDaudio registers only valid if LCAP.ALT == 0
pub const AZX_REG_ML_LPSOO: c_uint = 0x10;
pub const AZX_REG_ML_LPSIO: c_uint = 0x12;
pub const AZX_REG_ML_LWALFC: c_uint = 0x18;
pub const AZX_REG_ML_LOUTPAY: c_uint = 0x20;
pub const AZX_REG_ML_LINPAY: c_uint = 0x30;
// HDaudio Extended link registers only valid if LCAP.ALT == 1
pub const AZX_REG_ML_LSYNC: c_uint = 0x1C;

pub const AZX_REG_ML_LSYNC_CMDSYNC_SHIFT: c_int = 24;

pub const AZX_REG_ML_LEPTR: c_uint = 0x20;

pub const AZX_REG_ML_LEPTR_ID_SHIFT: c_int = 24;
pub const AZX_REG_ML_LEPTR_ID_SDW: c_uint = 0x00;
pub const AZX_REG_ML_LEPTR_ID_INTEL_SSP: c_uint = 0xC0;
pub const AZX_REG_ML_LEPTR_ID_INTEL_DMIC: c_uint = 0xC1;
pub const AZX_REG_ML_LEPTR_ID_INTEL_UAOL: c_uint = 0xC2;

// registers for DMA Resume Capability Structure
pub const AZX_DRSM_CAP_ID: c_uint = 0x5;
pub const AZX_REG_DRSM_CTL: c_uint = 0x4;
// Base used to calculate the iterating register offset
pub const AZX_DRSM_BASE: c_uint = 0x08;
// Interval used to calculate the iterating register offset
pub const AZX_DRSM_INTERVAL: c_uint = 0x08;
// Global time synchronization registers
pub const GTSCC_TSCCD_MASK: c_uint = 0x80000000;

pub const GTSCC_TSCCI_MASK: c_uint = 0x20;
pub const GTSCC_CDMAS_DMA_DIR_SHIFT: c_int = 4;
pub const WALFCC_CIF_MASK: c_uint = 0x1FF;
pub const WALFCC_FN_SHIFT: c_int = 9;
pub const HDA_CLK_CYCLES_PER_FRAME: c_int = 512;
//
// An error occurs near frame "rollover". The clocks in frame value indicates
// whether this error may have occurred. Here we use the value of 10. Please
// see the errata for the right number [<10]
//
pub const HDA_MAX_CYCLE_VALUE: c_int = 499;
pub const HDA_MAX_CYCLE_OFFSET: c_int = 10;
pub const HDA_MAX_CYCLE_READ_RETRY: c_int = 10;
pub const TSCCU_CCU_SHIFT: c_int = 32;
pub const LLPC_CCU_SHIFT: c_int = 32;
//
// helpers to read the stream position
//
extern "C" {
    pub fn snd_hdac_stream_readl(_arg: stream, _arg: SD_LPIB) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: *mut stream->posbuf) -> return;
}
