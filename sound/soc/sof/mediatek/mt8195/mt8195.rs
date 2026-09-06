//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/mediatek/mt8195/mt8195.h
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
// Copyright (c) 2021 MediaTek Corporation. All rights reserved.
//
// Header file for the mt8195 DSP register definition
//
pub const DSP_REG_BASE: c_uint = 0x10803000;
pub const SCP_CFGREG_BASE: c_uint = 0x10724000;
pub const DSP_SYSAO_BASE: c_uint = 0x1080C000;
//
// R E G I S T E R       TABLE
//
pub const DSP_JTAGMUX: c_uint = 0x0000;
pub const DSP_ALTRESETVEC: c_uint = 0x0004;
pub const DSP_PDEBUGDATA: c_uint = 0x0008;
pub const DSP_PDEBUGBUS0: c_uint = 0x000c;

pub const DSP_PDEBUGBUS1: c_uint = 0x0010;
pub const DSP_PDEBUGINST: c_uint = 0x0014;
pub const DSP_PDEBUGLS0STAT: c_uint = 0x0018;
pub const DSP_PDEBUGLS1STAT: c_uint = 0x001c;
pub const DSP_PDEBUGPC: c_uint = 0x0020;
pub const DSP_RESET_SW: c_uint = 0x0024 /*reset sw*/;

pub const DSP_PFAULTBUS: c_uint = 0x0028;
pub const DSP_PFAULTINFO: c_uint = 0x002c;
pub const DSP_GPR00: c_uint = 0x0030;
pub const DSP_GPR01: c_uint = 0x0034;
pub const DSP_GPR02: c_uint = 0x0038;
pub const DSP_GPR03: c_uint = 0x003c;
pub const DSP_GPR04: c_uint = 0x0040;
pub const DSP_GPR05: c_uint = 0x0044;
pub const DSP_GPR06: c_uint = 0x0048;
pub const DSP_GPR07: c_uint = 0x004c;
pub const DSP_GPR08: c_uint = 0x0050;
pub const DSP_GPR09: c_uint = 0x0054;
pub const DSP_GPR0A: c_uint = 0x0058;
pub const DSP_GPR0B: c_uint = 0x005c;
pub const DSP_GPR0C: c_uint = 0x0060;
pub const DSP_GPR0D: c_uint = 0x0064;
pub const DSP_GPR0E: c_uint = 0x0068;
pub const DSP_GPR0F: c_uint = 0x006c;
pub const DSP_GPR10: c_uint = 0x0070;
pub const DSP_GPR11: c_uint = 0x0074;
pub const DSP_GPR12: c_uint = 0x0078;
pub const DSP_GPR13: c_uint = 0x007c;
pub const DSP_GPR14: c_uint = 0x0080;
pub const DSP_GPR15: c_uint = 0x0084;
pub const DSP_GPR16: c_uint = 0x0088;
pub const DSP_GPR17: c_uint = 0x008c;
pub const DSP_GPR18: c_uint = 0x0090;
pub const DSP_GPR19: c_uint = 0x0094;
pub const DSP_GPR1A: c_uint = 0x0098;
pub const DSP_GPR1B: c_uint = 0x009c;
pub const DSP_GPR1C: c_uint = 0x00a0;
pub const DSP_GPR1D: c_uint = 0x00a4;
pub const DSP_GPR1E: c_uint = 0x00a8;
pub const DSP_GPR1F: c_uint = 0x00ac;
pub const DSP_TCM_OFFSET: c_uint = 0x00b0    /* not used */;
pub const DSP_DDR_OFFSET: c_uint = 0x00b4    /* not used */;
pub const DSP_INTFDSP: c_uint = 0x00d0;
pub const DSP_INTFDSP_CLR: c_uint = 0x00d4;
pub const DSP_SRAM_PD_SW1: c_uint = 0x00d8;
pub const DSP_SRAM_PD_SW2: c_uint = 0x00dc;
pub const DSP_OCD: c_uint = 0x00e0;
pub const DSP_RG_DSP_IRQ_POL: c_uint = 0x00f0    /* not used */;
pub const DSP_DSP_IRQ_EN: c_uint = 0x00f4    /* not used */;
pub const DSP_DSP_IRQ_LEVEL: c_uint = 0x00f8    /* not used */;
pub const DSP_DSP_IRQ_STATUS: c_uint = 0x00fc    /* not used */;
pub const DSP_RG_INT2CIRQ: c_uint = 0x0114;
pub const DSP_RG_INT_POL_CTL0: c_uint = 0x0120;
pub const DSP_RG_INT_EN_CTL0: c_uint = 0x0130;
pub const DSP_RG_INT_LV_CTL0: c_uint = 0x0140;
pub const DSP_RG_INT_STATUS0: c_uint = 0x0150;
pub const DSP_PDEBUGSTATUS0: c_uint = 0x0200;
pub const DSP_PDEBUGSTATUS1: c_uint = 0x0204;
pub const DSP_PDEBUGSTATUS2: c_uint = 0x0208;
pub const DSP_PDEBUGSTATUS3: c_uint = 0x020c;
pub const DSP_PDEBUGSTATUS4: c_uint = 0x0210;
pub const DSP_PDEBUGSTATUS5: c_uint = 0x0214;
pub const DSP_PDEBUGSTATUS6: c_uint = 0x0218;
pub const DSP_PDEBUGSTATUS7: c_uint = 0x021c;
pub const DSP_DSP2PSRAM_PRIORITY: c_uint = 0x0220  /* not used */;
pub const DSP_AUDIO_DSP2SPM_INT: c_uint = 0x0224;
pub const DSP_AUDIO_DSP2SPM_INT_ACK: c_uint = 0x0228;
pub const DSP_AUDIO_DSP_DEBUG_SEL: c_uint = 0x022C;
pub const DSP_AUDIO_DSP_EMI_BASE_ADDR: c_uint = 0x02E0  /* not used */;
pub const DSP_AUDIO_DSP_SHARED_IRAM: c_uint = 0x02E4;
pub const DSP_AUDIO_DSP_CKCTRL_P2P_CK_CON: c_uint = 0x02F0;
pub const DSP_RG_SEMAPHORE00: c_uint = 0x0300;
pub const DSP_RG_SEMAPHORE01: c_uint = 0x0304;
pub const DSP_RG_SEMAPHORE02: c_uint = 0x0308;
pub const DSP_RG_SEMAPHORE03: c_uint = 0x030C;
pub const DSP_RG_SEMAPHORE04: c_uint = 0x0310;
pub const DSP_RG_SEMAPHORE05: c_uint = 0x0314;
pub const DSP_RG_SEMAPHORE06: c_uint = 0x0318;
pub const DSP_RG_SEMAPHORE07: c_uint = 0x031C;
pub const DSP_RESERVED_0: c_uint = 0x03F0;
pub const DSP_RESERVED_1: c_uint = 0x03F4;
// dsp wdt
pub const DSP_WDT_MODE: c_uint = 0x0400;
// dsp mbox
pub const DSP_MBOX_IN_CMD: c_uint = 0x00;
pub const DSP_MBOX_IN_CMD_CLR: c_uint = 0x04;
pub const DSP_MBOX_OUT_CMD: c_uint = 0x1c;
pub const DSP_MBOX_OUT_CMD_CLR: c_uint = 0x20;
pub const DSP_MBOX_IN_MSG0: c_uint = 0x08;
pub const DSP_MBOX_IN_MSG1: c_uint = 0x0C;
pub const DSP_MBOX_OUT_MSG0: c_uint = 0x24;
pub const DSP_MBOX_OUT_MSG1: c_uint = 0x28;
// dsp sys ao

pub const DSP_SRAM_POOL_PD_MASK: c_uint = 0xf;

// DSP memories
pub const MBOX_OFFSET: c_uint = 0x800000 /* DRAM */;
pub const MBOX_SIZE: c_uint = 0x1000 /* consistent with which in memory.h of sof fw */;
pub const DSP_DRAM_SIZE: c_uint = 0x1000000 /* 16M */;
pub const DSP_REG_BAR: c_int = 4;
pub const DSP_MBOX0_BAR: c_int = 5;
pub const DSP_MBOX1_BAR: c_int = 6;
pub const DSP_MBOX2_BAR: c_int = 7;
pub const SIZE_SHARED_DRAM_DL: c_uint = 0x40000 /*Shared buffer for Downlink*/;
pub const SIZE_SHARED_DRAM_UL: c_uint = 0x40000 /*Shared buffer for Uplink*/;

pub const SRAM_PHYS_BASE_FROM_DSP_VIEW: c_uint = 0x40000000 /* MT8195 DSP view */;
pub const DRAM_PHYS_BASE_FROM_DSP_VIEW: c_uint = 0x60000000 /* MT8195 DSP view */;
// remap dram between AP and DSP view, 4KB aligned
pub const DRAM_REMAP_SHIFT: c_int = 12;

// suspend dsp idle check interval and timeout

extern "C" {
    pub fn sof_hifixdsp_boot_sequence(sdev: *mut snd_sof_dev, boot_addr: u32);
}
extern "C" {
    pub fn sof_hifixdsp_shutdown(sdev: *mut snd_sof_dev);
}
