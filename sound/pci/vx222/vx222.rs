//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/vx222/vx222.h
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
// Driver for Digigram VX222 PCI soundcards
//
// Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_vx222 {
    pub core: vx_core,
// h/w config; for PLX and for DSP
    pub pci: *mut pci_dev,
    pub port: [c_ulong; 2],
    pub /: *mut *mut unsigned int regCDSP; / current CDSP register,
    pub /: *mut *mut unsigned int regCFG; / current CFG register,
    pub /: *mut *mut unsigned int regSELMIC; / current SELMIC reg. (for VX222 Mic),
    pub /: *mut *mut int input_level[2]; / input level for vx222 mic,
    pub /: *mut *mut int mic_level; / mic level for vx222 mic,
}

// we use a lookup table with 148 values, see vx_mixer.c
pub const VX2_AKM_LEVEL_MAX: c_uint = 0x93;
// Offset of registers with base equal to portDSP.
pub const VX_RESET_DMA_REGISTER_OFFSET: c_uint = 0x00000008;
// Constants used to access the INTCSR register.
pub const VX_INTCSR_VALUE: c_uint = 0x00000001;
pub const VX_PCI_INTERRUPT_MASK: c_uint = 0x00000040;
// Constants used to access the CDSP register (0x20).
pub const VX_CDSP_TEST1_MASK: c_uint = 0x00000080;
pub const VX_CDSP_TOR1_MASK: c_uint = 0x00000040;
pub const VX_CDSP_TOR2_MASK: c_uint = 0x00000020;
pub const VX_CDSP_RESERVED0_0_MASK: c_uint = 0x00000010;
pub const VX_CDSP_CODEC_RESET_MASK: c_uint = 0x00000008;
pub const VX_CDSP_VALID_IRQ_MASK: c_uint = 0x00000004;
pub const VX_CDSP_TEST0_MASK: c_uint = 0x00000002;
pub const VX_CDSP_DSP_RESET_MASK: c_uint = 0x00000001;
pub const VX_CDSP_GPIO_OUT_MASK: c_uint = 0x00000060;

// Constants used to access the CFG register (0x24).
pub const VX_CFG_SYNCDSP_MASK: c_uint = 0x00000080;
pub const VX_CFG_RESERVED0_0_MASK: c_uint = 0x00000040;
pub const VX_CFG_RESERVED1_0_MASK: c_uint = 0x00000020;
pub const VX_CFG_RESERVED2_0_MASK: c_uint = 0x00000010;
pub const VX_CFG_DATAIN_SEL_MASK: c_uint = 0x00000008     // 0 (ana), 1 (UER);
pub const VX_CFG_RESERVED3_0_MASK: c_uint = 0x00000004;
pub const VX_CFG_RESERVED4_0_MASK: c_uint = 0x00000002;
pub const VX_CFG_CLOCKIN_SEL_MASK: c_uint = 0x00000001     // 0 (internal), 1 (AES/EBU);
// Constants used to access the STATUS register (0x30).
pub const VX_STATUS_DATA_XICOR_MASK: c_uint = 0x00000080;
pub const VX_STATUS_VAL_TEST1_MASK: c_uint = 0x00000040;
pub const VX_STATUS_VAL_TEST0_MASK: c_uint = 0x00000020;
pub const VX_STATUS_RESERVED0_MASK: c_uint = 0x00000010;
pub const VX_STATUS_VAL_TOR1_MASK: c_uint = 0x00000008;
pub const VX_STATUS_VAL_TOR0_MASK: c_uint = 0x00000004;
pub const VX_STATUS_LEVEL_IN_MASK: c_uint = 0x00000002    // 6 dBu (0), 22 dBu (1);
pub const VX_STATUS_MEMIRQ_MASK: c_uint = 0x00000001;
pub const VX_STATUS_GPIO_IN_MASK: c_uint = 0x0000000C;

// Constants used to access the MICRO INPUT SELECT register (0x40).
pub const MICRO_SELECT_INPUT_NORM: c_uint = 0x00;
pub const MICRO_SELECT_INPUT_MUTE: c_uint = 0x01;
pub const MICRO_SELECT_INPUT_LIMIT: c_uint = 0x02;
pub const MICRO_SELECT_INPUT_MASK: c_uint = 0x03;
pub const MICRO_SELECT_PREAMPLI_G_0: c_uint = 0x00;
pub const MICRO_SELECT_PREAMPLI_G_1: c_uint = 0x04;
pub const MICRO_SELECT_PREAMPLI_G_2: c_uint = 0x08;
pub const MICRO_SELECT_PREAMPLI_G_3: c_uint = 0x0C;
pub const MICRO_SELECT_PREAMPLI_MASK: c_uint = 0x0C;
pub const MICRO_SELECT_PREAMPLI_OFFSET: c_int = 2;
pub const MICRO_SELECT_RAISE_COMPR: c_uint = 0x10;
pub const MICRO_SELECT_NOISE_T_52DB: c_uint = 0x00;
pub const MICRO_SELECT_NOISE_T_42DB: c_uint = 0x20;
pub const MICRO_SELECT_NOISE_T_32DB: c_uint = 0x40;
pub const MICRO_SELECT_NOISE_T_MASK: c_uint = 0x60;
pub const MICRO_SELECT_PHANTOM_ALIM: c_uint = 0x80;
