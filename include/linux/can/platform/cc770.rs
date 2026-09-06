//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/can/platform/cc770.h
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
// CPU Interface Register (0x02)
pub const CPUIF_CEN: c_uint = 0x01	/* Clock Out Enable */;
pub const CPUIF_MUX: c_uint = 0x04	/* Multiplex */;
pub const CPUIF_SLP: c_uint = 0x08	/* Sleep */;
pub const CPUIF_PWD: c_uint = 0x10	/* Power Down Mode */;
pub const CPUIF_DMC: c_uint = 0x20	/* Divide Memory Clock */;
pub const CPUIF_DSC: c_uint = 0x40	/* Divide System Clock */;
pub const CPUIF_RST: c_uint = 0x80	/* Hardware Reset Status */;
// Clock Out Register (0x1f)
pub const CLKOUT_CD_MASK: c_uint = 0x0f	/* Clock Divider mask */;
pub const CLKOUT_SL_MASK: c_uint = 0x30	/* Slew Rate mask */;
pub const CLKOUT_SL_SHIFT: c_int = 4;
// Bus Configuration Register (0x2f)
pub const BUSCFG_DR0: c_uint = 0x01	/* Disconnect RX0 Input / Select RX input */;
pub const BUSCFG_DR1: c_uint = 0x02	/* Disconnect RX1 Input / Silent mode */;
pub const BUSCFG_DT1: c_uint = 0x08	/* Disconnect TX1 Output */;
pub const BUSCFG_POL: c_uint = 0x20	/* Polarity dominant or recessive */;
pub const BUSCFG_CBY: c_uint = 0x40	/* Input Comparator Bypass */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc770_platform_data {
    pub /: *mut *mut u32 osc_freq; / CAN bus oscillator frequency in Hz,
    pub /: *mut *mut u8 cir; / CPU Interface Register,
    pub /: *mut *mut u8 cor; / Clock Out Register,
    pub /: *mut *mut u8 bcr; / Bus Configuration Register,
}
