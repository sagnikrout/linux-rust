//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clk/at91_pmc.h
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
// include/linux/clk/at91_pmc.h
//
// Copyright (C) 2005 Ivan Kokshaysky
// Copyright (C) SAN People
//
// Power Management Controller (PMC) - System peripherals registers.
// Based on AT91RM9200 datasheet revision E.
//

pub const AT91_PMC_SCER: c_uint = 0x00			/* System Clock Enable Register */;
pub const AT91_PMC_SCDR: c_uint = 0x04			/* System Clock Disable Register */;
pub const AT91_PMC_SCSR: c_uint = 0x08			/* System Clock Status Register */;

pub const AT91_PMC_PLL_CTRL0: c_uint = 0x0C		/* PLL Control Register 0 [for SAM9X60] */;

pub const AT91_PMC_PLL_CTRL1: c_uint = 0x10		/* PLL Control Register 1 [for SAM9X60] */;
pub const AT91_PMC_PCER: c_uint = 0x10			/* Peripheral Clock Enable Register */;
pub const AT91_PMC_PCDR: c_uint = 0x14			/* Peripheral Clock Disable Register */;
pub const AT91_PMC_PCSR: c_uint = 0x18			/* Peripheral Clock Status Register */;
pub const AT91_PMC_PLL_ACR: c_uint = 0x18			/* PLL Analog Control Register [for SAM9X60] */;

pub const AT91_CKGR_UCKR: c_uint = 0x1C			/* UTMI Clock Register [some SAM9] */;

pub const AT91_PMC_PLL_UPDT: c_uint = 0x1C		/* PMC PLL update register [for SAM9X60] */;

pub const AT91_CKGR_MOR: c_uint = 0x20			/* Main Oscillator Register [not on SAM9RL] */;

pub const AT91_CKGR_MCFR: c_uint = 0x24			/* Main Clock Frequency Register */;

pub const AT91_CKGR_PLLAR: c_uint = 0x28			/* PLL A Register */;
pub const AT91_PMC_RATIO: c_uint = 0x2c			/* Processor clock ratio register [SAMA7G5 only] */;

pub const AT91_CKGR_PLLBR: c_uint = 0x2c			/* PLL B Register */;

pub const AT91_PMC_CPU_CKR: c_uint = 0x28			/* CPU Clock Register */;
pub const AT91_PMC_MCKR: c_uint = 0x30			/* Master Clock Register */;

pub const PMC_PRES_OFFSET: c_int = 2;

pub const PMC_ALT_PRES_OFFSET: c_int = 4;

pub const AT91_PMC_MCR_V2: c_uint = 0x30				/* Master Clock Register [SAMA7G5 only] */;

pub const AT91_PMC_XTALF: c_uint = 0x34			/* Main XTAL Frequency Register [SAMA7G5 only] */;
pub const AT91_PMC_USB: c_uint = 0x38			/* USB Clock Register [some SAM9 only] */;

pub const AT91_PMC_SMD: c_uint = 0x3c			/* Soft Modem Clock Register [some SAM9 only] */;

pub const AT91_PMC_IER: c_uint = 0x60			/* Interrupt Enable Register */;
pub const AT91_PMC_IDR: c_uint = 0x64			/* Interrupt Disable Register */;
pub const AT91_PMC_SR: c_uint = 0x68			/* Status Register */;

pub const AT91_PMC_IMR: c_uint = 0x6c			/* Interrupt Mask Register */;
pub const AT91_PMC_FSMR: c_uint = 0x70		/* Fast Startup Mode Register */;

pub const AT91_PMC_FSPR: c_uint = 0x74		/* Fast Startup Polarity Reg */;
pub const AT91_PMC_FS_INPUT_MASK: c_uint = 0x7ff;
pub const AT91_PMC_PLLICPR: c_uint = 0x80			/* PLL Charge Pump Current Register */;
pub const AT91_PMC_PROT: c_uint = 0xe4			/* Write Protect Mode Register [some SAM9] */;

pub const AT91_PMC_WPSR: c_uint = 0xe8			/* Write Protect Status Register [some SAM9] */;

pub const AT91_PMC_PLL_ISR0: c_uint = 0xEC			/* PLL Interrupt Status Register 0 [SAM9X60 only] */;
pub const AT91_PMC_PCER1: c_uint = 0x100			/* Peripheral Clock Enable Register 1 [SAMA5 only]*/;
pub const AT91_PMC_PCDR1: c_uint = 0x104			/* Peripheral Clock Enable Register 1 */;
pub const AT91_PMC_PCSR1: c_uint = 0x108			/* Peripheral Clock Enable Register 1 */;
pub const AT91_PMC_PCR: c_uint = 0x10c			/* Peripheral Control Register [some SAM9 and SAMA5] */;
pub const AT91_PMC_PCR_PID_MASK: c_uint = 0x3f;

pub const AT91_PMC_AUDIO_PLL0: c_uint = 0x14c;

pub const AT91_PMC_AUDIO_PLL_ND_OFFSET: c_int = 8;

pub const AT91_PMC_AUDIO_PLL_QDPMC_OFFSET: c_int = 16;

pub const AT91_PMC_AUDIO_PLL1: c_uint = 0x150;
pub const AT91_PMC_AUDIO_PLL_FRACR_MASK: c_uint = 0x3fffff;
pub const AT91_PMC_AUDIO_PLL_QDPAD_OFFSET: c_int = 24;

pub const AT91_PMC_AUDIO_PLL_QDPAD_EXTDIV_OFFSET: c_int = 26;
pub const AT91_PMC_AUDIO_PLL_QDPAD_EXTDIV_MAX: c_uint = 0x1f;

