//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/stm/stm32_sai.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// STM32 ALSA SoC Digital Audio Interface (SAI) driver.
//
// Copyright (C) 2016, STMicroelectronics - All Rights Reserved
// Author(s): Olivier Moysan <olivier.moysan@st.com> for STMicroelectronics.
//

// SAI Register Map
// Global configuration register
pub const STM_SAI_GCR: c_uint = 0x00;
// Sub-block A&B registers offsets, relative to A&B sub-block addresses
pub const STM_SAI_CR1_REGX: c_uint = 0x00	/* A offset: 0x04. B offset: 0x24 */;
pub const STM_SAI_CR2_REGX: c_uint = 0x04;
pub const STM_SAI_FRCR_REGX: c_uint = 0x08;
pub const STM_SAI_SLOTR_REGX: c_uint = 0x0C;
pub const STM_SAI_IMR_REGX: c_uint = 0x10;
pub const STM_SAI_SR_REGX: c_uint = 0x14;
pub const STM_SAI_CLRFR_REGX: c_uint = 0x18;
pub const STM_SAI_DR_REGX: c_uint = 0x1C;
// Sub-block A registers, relative to sub-block A address
pub const STM_SAI_PDMCR_REGX: c_uint = 0x40;
pub const STM_SAI_PDMLY_REGX: c_uint = 0x44;
// Hardware configuration registers
pub const STM_SAI_HWCFGR: c_uint = 0x3F0;
pub const STM_SAI_VERR: c_uint = 0x3F4;
pub const STM_SAI_IDR: c_uint = 0x3F8;
pub const STM_SAI_SIDR: c_uint = 0x3FC;
// Bit definition for SAI_GCR register
pub const SAI_GCR_SYNCIN_SHIFT: c_int = 0;
pub const SAI_GCR_SYNCIN_WDTH: c_int = 2;

pub const SAI_GCR_SYNCOUT_SHIFT: c_int = 4;

// Bit definition for SAI_XCR1 register
pub const SAI_XCR1_RX_TX_SHIFT: c_int = 0;

pub const SAI_XCR1_SLAVE_SHIFT: c_int = 1;

pub const SAI_XCR1_PRTCFG_SHIFT: c_int = 2;

pub const SAI_XCR1_DS_SHIFT: c_int = 5;

pub const SAI_XCR1_LSBFIRST_SHIFT: c_int = 8;

pub const SAI_XCR1_CKSTR_SHIFT: c_int = 9;

pub const SAI_XCR1_SYNCEN_SHIFT: c_int = 10;

pub const SAI_XCR1_MONO_SHIFT: c_int = 12;

pub const SAI_XCR1_OUTDRIV_SHIFT: c_int = 13;

pub const SAI_XCR1_SAIEN_SHIFT: c_int = 16;

pub const SAI_XCR1_DMAEN_SHIFT: c_int = 17;

pub const SAI_XCR1_NODIV_SHIFT: c_int = 19;

pub const SAI_XCR1_MCKDIV_SHIFT: c_int = 20;

pub const SAI_XCR1_OSR_SHIFT: c_int = 26;

pub const SAI_XCR1_MCKEN_SHIFT: c_int = 27;

// Bit definition for SAI_XCR2 register
pub const SAI_XCR2_FTH_SHIFT: c_int = 0;

pub const SAI_XCR2_FFLUSH_SHIFT: c_int = 3;

pub const SAI_XCR2_TRIS_SHIFT: c_int = 4;

pub const SAI_XCR2_MUTE_SHIFT: c_int = 5;

pub const SAI_XCR2_MUTEVAL_SHIFT: c_int = 6;

pub const SAI_XCR2_MUTECNT_SHIFT: c_int = 7;

pub const SAI_XCR2_CPL_SHIFT: c_int = 13;

pub const SAI_XCR2_COMP_SHIFT: c_int = 14;

// Bit definition for SAI_XFRCR register
pub const SAI_XFRCR_FRL_SHIFT: c_int = 0;

pub const SAI_XFRCR_FSALL_SHIFT: c_int = 8;

pub const SAI_XFRCR_FSDEF_SHIFT: c_int = 16;

pub const SAI_XFRCR_FSPOL_SHIFT: c_int = 17;

pub const SAI_XFRCR_FSOFF_SHIFT: c_int = 18;

// Bit definition for SAI_XSLOTR register
pub const SAI_XSLOTR_FBOFF_SHIFT: c_int = 0;

pub const SAI_XSLOTR_SLOTSZ_SHIFT: c_int = 6;

pub const SAI_XSLOTR_NBSLOT_SHIFT: c_int = 8;

pub const SAI_XSLOTR_SLOTEN_SHIFT: c_int = 16;
pub const SAI_XSLOTR_SLOTEN_WIDTH: c_int = 16;

// Bit definition for SAI_XIMR register

pub const SAI_XIMR_SHIFT: c_int = 0;

// Bit definition for SAI_XSR register

pub const SAI_XSR_SHIFT: c_int = 0;

// Bit definition for SAI_XCLRFR register

pub const SAI_XCLRFR_SHIFT: c_int = 0;

// Bit definition for SAI_PDMCR register

pub const SAI_PDMCR_MICNBR_SHIFT: c_int = 4;

// Bit definition for (SAI_PDMDLY register
pub const SAI_PDMDLY_1L_SHIFT: c_int = 0;

pub const SAI_PDMDLY_1L_WIDTH: c_int = 3;
pub const SAI_PDMDLY_1R_SHIFT: c_int = 4;

pub const SAI_PDMDLY_1R_WIDTH: c_int = 3;
pub const SAI_PDMDLY_2L_SHIFT: c_int = 8;

pub const SAI_PDMDLY_2L_WIDTH: c_int = 3;
pub const SAI_PDMDLY_2R_SHIFT: c_int = 12;

pub const SAI_PDMDLY_2R_WIDTH: c_int = 3;
pub const SAI_PDMDLY_3L_SHIFT: c_int = 16;

pub const SAI_PDMDLY_3L_WIDTH: c_int = 3;
pub const SAI_PDMDLY_3R_SHIFT: c_int = 20;

pub const SAI_PDMDLY_3R_WIDTH: c_int = 3;
pub const SAI_PDMDLY_4L_SHIFT: c_int = 24;

pub const SAI_PDMDLY_4L_WIDTH: c_int = 3;
pub const SAI_PDMDLY_4R_SHIFT: c_int = 28;

pub const SAI_PDMDLY_4R_WIDTH: c_int = 3;
// Registers below apply to SAI version 2.1 and more
// Bit definition for SAI_HWCFGR register

// Bit definition for SAI_VERR register

// Bit definition for SAI_IDR register

// Bit definition for SAI_SIDR register

pub const SAI_IPIDR_NUMBER: c_uint = 0x00130031;
// SAI version numbers are 1.x for F4. Major version number set to 1 for F4

// Dummy version number for H7 socs and next
pub const STM_SAI_STM32H7: c_uint = 0x0;

// Macro flag: #define STM_SAI_HAS_SPDIF_PDM(ip)\
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stm32_sai_syncout {
    STM_SAI_SYNC_OUT_NONE,
    STM_SAI_SYNC_OUT_A,
    STM_SAI_SYNC_OUT_B,
}

//
// struct stm32_sai_conf - SAI configuration
// @get_sai_ck_parent: get parent clock of SAI kernel clock
// @version: SAI version
// @fifo_size: SAI fifo size as words number
// @has_spdif_pdm: SAI S/PDIF and PDM features support flag
// @no_dma_burst: Support only DMA single transfers if set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_sai_conf {
    pub sai): *mut *mut int (get_sai_ck_parent)(struct stm32_sai_data,
    pub version: u32,
    pub fifo_size: u32,
    pub has_spdif_pdm: bool,
    pub no_dma_burst: bool,
}

//
// struct stm32_sai_data - private data of SAI instance driver
// @pdev: device data pointer
// @base: common register bank virtual base address
// @pclk: SAI bus clock
// @clk_x8k: SAI parent clock for sampling frequencies multiple of 8kHz
// @clk_x11k: SAI parent clock for sampling frequencies multiple of 11kHz
// @conf: SAI hardware capabitilites
// @irq: SAI interrupt line
// @set_sync: pointer to synchro mode configuration callback
// @gcr: SAI Global Configuration Register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_sai_data {
    pub pdev: *mut platform_device,
    pub base: *mut void __iomem,
    pub pclk: *mut clk,
    pub clk_x8k: *mut clk,
    pub clk_x11k: *mut clk,
    pub conf: stm32_sai_conf,
    pub irq: c_int,
    pub synci): *mut *mut device_node np_provider, int synco, int,
    pub gcr: u32,
}
