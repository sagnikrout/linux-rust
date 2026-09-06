//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/ti/davinci-mcasp.h
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
// ALSA SoC McASP Audio Layer for TI DAVINCI processor
//
// MCASP related definitions
//
// Author: Nirmal Pandey <n-pandey@ti.com>,
// Suresh Rajashekara <suresh.r@ti.com>
// Steve Chen <schen@.mvista.com>
//
// Copyright:   (C) 2009 MontaVista Software, Inc., <source@mvista.com>
// Copyright:   (C) 2009  Texas Instruments, India
//
// McASP register definitions
//
pub const DAVINCI_MCASP_PID_REG: c_uint = 0x00;
pub const DAVINCI_MCASP_PWREMUMGT_REG: c_uint = 0x04;
pub const DAVINCI_MCASP_PFUNC_REG: c_uint = 0x10;
pub const DAVINCI_MCASP_PDIR_REG: c_uint = 0x14;
pub const DAVINCI_MCASP_PDOUT_REG: c_uint = 0x18;
pub const DAVINCI_MCASP_PDSET_REG: c_uint = 0x1c;
pub const DAVINCI_MCASP_PDCLR_REG: c_uint = 0x20;
pub const DAVINCI_MCASP_TLGC_REG: c_uint = 0x30;
pub const DAVINCI_MCASP_TLMR_REG: c_uint = 0x34;
pub const DAVINCI_MCASP_GBLCTL_REG: c_uint = 0x44;
pub const DAVINCI_MCASP_AMUTE_REG: c_uint = 0x48;
pub const DAVINCI_MCASP_LBCTL_REG: c_uint = 0x4c;
pub const DAVINCI_MCASP_TXDITCTL_REG: c_uint = 0x50;
pub const DAVINCI_MCASP_GBLCTLR_REG: c_uint = 0x60;
pub const DAVINCI_MCASP_RXMASK_REG: c_uint = 0x64;
pub const DAVINCI_MCASP_RXFMT_REG: c_uint = 0x68;
pub const DAVINCI_MCASP_RXFMCTL_REG: c_uint = 0x6c;
pub const DAVINCI_MCASP_ACLKRCTL_REG: c_uint = 0x70;
pub const DAVINCI_MCASP_AHCLKRCTL_REG: c_uint = 0x74;
pub const DAVINCI_MCASP_RXTDM_REG: c_uint = 0x78;
pub const DAVINCI_MCASP_EVTCTLR_REG: c_uint = 0x7c;
pub const DAVINCI_MCASP_RXSTAT_REG: c_uint = 0x80;
pub const DAVINCI_MCASP_RXTDMSLOT_REG: c_uint = 0x84;
pub const DAVINCI_MCASP_RXCLKCHK_REG: c_uint = 0x88;
pub const DAVINCI_MCASP_REVTCTL_REG: c_uint = 0x8c;
pub const DAVINCI_MCASP_GBLCTLX_REG: c_uint = 0xa0;
pub const DAVINCI_MCASP_TXMASK_REG: c_uint = 0xa4;
pub const DAVINCI_MCASP_TXFMT_REG: c_uint = 0xa8;
pub const DAVINCI_MCASP_TXFMCTL_REG: c_uint = 0xac;
pub const DAVINCI_MCASP_ACLKXCTL_REG: c_uint = 0xb0;
pub const DAVINCI_MCASP_AHCLKXCTL_REG: c_uint = 0xb4;
pub const DAVINCI_MCASP_TXTDM_REG: c_uint = 0xb8;
pub const DAVINCI_MCASP_EVTCTLX_REG: c_uint = 0xbc;
pub const DAVINCI_MCASP_TXSTAT_REG: c_uint = 0xc0;
pub const DAVINCI_MCASP_TXTDMSLOT_REG: c_uint = 0xc4;
pub const DAVINCI_MCASP_TXCLKCHK_REG: c_uint = 0xc8;
pub const DAVINCI_MCASP_XEVTCTL_REG: c_uint = 0xcc;
// Left(even TDM Slot) Channel Status Register File
pub const DAVINCI_MCASP_DITCSRA_REG: c_uint = 0x100;
// Right(odd TDM slot) Channel Status Register File
pub const DAVINCI_MCASP_DITCSRB_REG: c_uint = 0x118;
// Left(even TDM slot) User Data Register File
pub const DAVINCI_MCASP_DITUDRA_REG: c_uint = 0x130;
// Right(odd TDM Slot) User Data Register File
pub const DAVINCI_MCASP_DITUDRB_REG: c_uint = 0x148;
// Serializer n Control Register
pub const DAVINCI_MCASP_XRSRCTL_BASE_REG: c_uint = 0x180;

// Transmit Buffer for Serializer n

// Receive Buffer for Serializer n

// McASP FIFO Registers

// FIFO register offsets from AFIFO base

//
// DAVINCI_MCASP_PWREMUMGT_REG - Power Down and Emulation Management
// Register Bits
//

//
// DAVINCI_MCASP_PFUNC_REG - Pin Function / GPIO Enable Register Bits
// DAVINCI_MCASP_PDIR_REG - Pin Direction Register Bits
// DAVINCI_MCASP_PDOUT_REG - Pin output in GPIO mode
// DAVINCI_MCASP_PDSET_REG - Pin input in GPIO mode
//

pub const PIN_BIT_AMUTE: c_int = 25;
pub const PIN_BIT_ACLKX: c_int = 26;
pub const PIN_BIT_AHCLKX: c_int = 27;
pub const PIN_BIT_AFSX: c_int = 28;
pub const PIN_BIT_ACLKR: c_int = 29;
pub const PIN_BIT_AHCLKR: c_int = 30;
pub const PIN_BIT_AFSR: c_int = 31;
//
// DAVINCI_MCASP_TXDITCTL_REG - Transmit DIT Control Register Bits
//

//
// DAVINCI_MCASP_TXFMT_REG - Transmit Bitstream Format Register Bits
//

//
// DAVINCI_MCASP_RXFMT_REG - Receive Bitstream Format Register Bits
//

//
// DAVINCI_MCASP_TXFMCTL_REG -  Transmit Frame Control Register Bits
//

//
// DAVINCI_MCASP_RXFMCTL_REG - Receive Frame Control Register Bits
//

//
// DAVINCI_MCASP_ACLKXCTL_REG - Transmit Clock Control Register Bits
//

pub const ACLKXDIV_MASK: c_uint = 0x1f;
//
// DAVINCI_MCASP_ACLKRCTL_REG Receive Clock Control Register Bits
//

pub const ACLKRDIV_MASK: c_uint = 0x1f;
//
// DAVINCI_MCASP_AHCLKXCTL_REG - High Frequency Transmit Clock Control
// Register Bits
//

pub const AHCLKXDIV_MASK: c_uint = 0xfff;
//
// DAVINCI_MCASP_AHCLKRCTL_REG - High Frequency Receive Clock Control
// Register Bits
//

pub const AHCLKRDIV_MASK: c_uint = 0xfff;
//
// DAVINCI_MCASP_XRSRCTL_BASE_REG -  Serializer Control Register Bits
//

pub const SRMOD_MASK: c_int = 3;
pub const SRMOD_INACTIVE: c_int = 0;
//
// DAVINCI_MCASP_LBCTL_REG - Loop Back Control Register Bits
//

//
// DAVINCI_MCASP_TXTDMSLOT_REG - Transmit TDM Slot Register configuration
//

//
// DAVINCI_MCASP_RXTDMSLOT_REG - Receive TDM Slot Register configuration
//

//
// DAVINCI_MCASP_GBLCTL_REG -  Global Control Register Bits
//

//
// DAVINCI_MCASP_TXSTAT_REG - Transmitter Status Register Bits
// DAVINCI_MCASP_RXSTAT_REG - Receiver Status Register Bits
//

//
// DAVINCI_MCASP_AMUTE_REG -  Mute Control Register Bits
//

//
// DAVINCI_MCASP_REVTCTL_REG - Receiver DMA Event Control Register bits
//

//
// DAVINCI_MCASP_XEVTCTL_REG - Transmitter DMA Event Control Register bits
//

//
// DAVINCI_MCASP_EVTCTLR_REG - Receiver Interrupt Control Register Bits
//

//
// DAVINCI_MCASP_EVTCTLX_REG - Transmitter Interrupt Control Register Bits
//

//
// DAVINCI_MCASP_W[R]FIFOCTL - Write/Read FIFO Control Register bits
//

// Source of High-frequency transmit/receive clock

// clock divider IDs

