//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/heathrow.h
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
// heathrow.h: definitions for using the "Heathrow" I/O controller chip.
//
// Grabbed from Open Firmware definitions on a PowerBook G3 Series
//
// Copyright (C) 1997 Paul Mackerras.
//
// Front light color on Yikes/B&W G3. 32 bits
pub const HEATHROW_FRONT_LIGHT: c_uint = 0x32 /* (set to 0 or 0xffffffff) */;
// Brightness/contrast (gossamer iMac ?). 8 bits
pub const HEATHROW_BRIGHTNESS_CNTL: c_uint = 0x32;
pub const HEATHROW_CONTRAST_CNTL: c_uint = 0x33;
// offset from ohare base for feature control register
pub const HEATHROW_MBCR: c_uint = 0x34	/* Media bay control */;
pub const HEATHROW_FCR: c_uint = 0x38	/* Feature control */;
pub const HEATHROW_AUX_CNTL_REG: c_uint = 0x3c	/* Aux control */;
//
// Bits in feature control register.
// Bits postfixed with a _N are in inverse logic
//
pub const HRW_SCC_TRANS_EN_N: c_uint = 0x00000001	/* Also controls modem power */;
pub const HRW_BAY_POWER_N: c_uint = 0x00000002;
pub const HRW_BAY_PCI_ENABLE: c_uint = 0x00000004;
pub const HRW_BAY_IDE_ENABLE: c_uint = 0x00000008;
pub const HRW_BAY_FLOPPY_ENABLE: c_uint = 0x00000010;
pub const HRW_IDE0_ENABLE: c_uint = 0x00000020;
pub const HRW_IDE0_RESET_N: c_uint = 0x00000040;
pub const HRW_BAY_DEV_MASK: c_uint = 0x0000001c;
pub const HRW_BAY_RESET_N: c_uint = 0x00000080;
pub const HRW_IOBUS_ENABLE: c_uint = 0x00000100	/* Internal IDE ? */;
pub const HRW_SCC_ENABLE: c_uint = 0x00000200;
pub const HRW_MESH_ENABLE: c_uint = 0x00000400;
pub const HRW_SWIM_ENABLE: c_uint = 0x00000800;
pub const HRW_SOUND_POWER_N: c_uint = 0x00001000;
pub const HRW_SOUND_CLK_ENABLE: c_uint = 0x00002000;
pub const HRW_SCCA_IO: c_uint = 0x00004000;
pub const HRW_SCCB_IO: c_uint = 0x00008000;
pub const HRW_PORT_OR_DESK_VIA_N: c_uint = 0x00010000	/* This one is 0 on PowerBook */;
pub const HRW_PWM_MON_ID_N: c_uint = 0x00020000	/* ??? (0) */;
pub const HRW_HOOK_MB_CNT_N: c_uint = 0x00040000	/* ??? (0) */;
pub const HRW_SWIM_CLONE_FLOPPY: c_uint = 0x00080000	/* ??? (0) */;
pub const HRW_AUD_RUN22: c_uint = 0x00100000	/* ??? (1) */;
pub const HRW_SCSI_LINK_MODE: c_uint = 0x00200000	/* Read ??? (1) */;
pub const HRW_ARB_BYPASS: c_uint = 0x00400000	/* Disable internal PCI arbitrer */;
pub const HRW_IDE1_RESET_N: c_uint = 0x00800000	/* Media bay */;
pub const HRW_SLOW_SCC_PCLK: c_uint = 0x01000000	/* ??? (0) */;
pub const HRW_RESET_SCC: c_uint = 0x02000000;
pub const HRW_MFDC_CELL_ENABLE: c_uint = 0x04000000	/* ??? (0) */;
pub const HRW_USE_MFDC: c_uint = 0x08000000	/* ??? (0) */;
pub const HRW_BMAC_IO_ENABLE: c_uint = 0x60000000	/* two bits, not documented in OF */;
pub const HRW_BMAC_RESET: c_uint = 0x80000000	/* not documented in OF */;
// We OR those features at boot on desktop G3s

// Looks like Heathrow has some sort of GPIOs as well...
pub const HRW_GPIO_MODEM_RESET: c_uint = 0x6d;

