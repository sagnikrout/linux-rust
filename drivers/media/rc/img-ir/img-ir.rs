//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/rc/img-ir/img-ir.h
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
// ImgTec IR Decoder found in PowerDown Controller.
//
// Copyright 2010-2014 Imagination Technologies Ltd.
//

// registers
// relative to the start of the IR block of registers
pub const IMG_IR_CONTROL: c_uint = 0x00;
pub const IMG_IR_STATUS: c_uint = 0x04;
pub const IMG_IR_DATA_LW: c_uint = 0x08;
pub const IMG_IR_DATA_UP: c_uint = 0x0c;
pub const IMG_IR_LEAD_SYMB_TIMING: c_uint = 0x10;
pub const IMG_IR_S00_SYMB_TIMING: c_uint = 0x14;
pub const IMG_IR_S01_SYMB_TIMING: c_uint = 0x18;
pub const IMG_IR_S10_SYMB_TIMING: c_uint = 0x1c;
pub const IMG_IR_S11_SYMB_TIMING: c_uint = 0x20;
pub const IMG_IR_FREE_SYMB_TIMING: c_uint = 0x24;
pub const IMG_IR_POW_MOD_PARAMS: c_uint = 0x28;
pub const IMG_IR_POW_MOD_ENABLE: c_uint = 0x2c;
pub const IMG_IR_IRQ_MSG_DATA_LW: c_uint = 0x30;
pub const IMG_IR_IRQ_MSG_DATA_UP: c_uint = 0x34;
pub const IMG_IR_IRQ_MSG_MASK_LW: c_uint = 0x38;
pub const IMG_IR_IRQ_MSG_MASK_UP: c_uint = 0x3c;
pub const IMG_IR_IRQ_ENABLE: c_uint = 0x40;
pub const IMG_IR_IRQ_STATUS: c_uint = 0x44;
pub const IMG_IR_IRQ_CLEAR: c_uint = 0x48;
pub const IMG_IR_IRCORE_ID: c_uint = 0xf0;
pub const IMG_IR_CORE_REV: c_uint = 0xf4;
pub const IMG_IR_CORE_DES1: c_uint = 0xf8;
pub const IMG_IR_CORE_DES2: c_uint = 0xfc;
// field masks
// IMG_IR_CONTROL
pub const IMG_IR_DECODEN: c_uint = 0x40000000;
pub const IMG_IR_CODETYPE: c_uint = 0x30000000;
pub const IMG_IR_CODETYPE_SHIFT: c_int = 28;
pub const IMG_IR_HDRTOG: c_uint = 0x08000000;
pub const IMG_IR_LDRDEC: c_uint = 0x04000000;
pub const IMG_IR_DECODINPOL: c_uint = 0x02000000	/* active high */;
pub const IMG_IR_BITORIEN: c_uint = 0x01000000	/* MSB first */;
pub const IMG_IR_D1VALIDSEL: c_uint = 0x00008000;
pub const IMG_IR_BITINV: c_uint = 0x00000040	/* don't invert */;
pub const IMG_IR_DECODEND2: c_uint = 0x00000010;
pub const IMG_IR_BITORIEND2: c_uint = 0x00000002	/* MSB first */;
pub const IMG_IR_BITINVD2: c_uint = 0x00000001	/* don't invert */;
// IMG_IR_STATUS
pub const IMG_IR_RXDVALD2: c_uint = 0x00001000;
pub const IMG_IR_IRRXD: c_uint = 0x00000400;
pub const IMG_IR_TOGSTATE: c_uint = 0x00000200;
pub const IMG_IR_RXDVAL: c_uint = 0x00000040;
pub const IMG_IR_RXDLEN: c_uint = 0x0000003f;
pub const IMG_IR_RXDLEN_SHIFT: c_int = 0;
// IMG_IR_LEAD_SYMB_TIMING, IMG_IR_Sxx_SYMB_TIMING
pub const IMG_IR_PD_MAX: c_uint = 0xff000000;
pub const IMG_IR_PD_MAX_SHIFT: c_int = 24;
pub const IMG_IR_PD_MIN: c_uint = 0x00ff0000;
pub const IMG_IR_PD_MIN_SHIFT: c_int = 16;
pub const IMG_IR_W_MAX: c_uint = 0x0000ff00;
pub const IMG_IR_W_MAX_SHIFT: c_int = 8;
pub const IMG_IR_W_MIN: c_uint = 0x000000ff;
pub const IMG_IR_W_MIN_SHIFT: c_int = 0;
// IMG_IR_FREE_SYMB_TIMING
pub const IMG_IR_MAXLEN: c_uint = 0x0007e000;
pub const IMG_IR_MAXLEN_SHIFT: c_int = 13;
pub const IMG_IR_MINLEN: c_uint = 0x00001f00;
pub const IMG_IR_MINLEN_SHIFT: c_int = 8;
pub const IMG_IR_FT_MIN: c_uint = 0x000000ff;
pub const IMG_IR_FT_MIN_SHIFT: c_int = 0;
// IMG_IR_POW_MOD_PARAMS
pub const IMG_IR_PERIOD_LEN: c_uint = 0x3f000000;
pub const IMG_IR_PERIOD_LEN_SHIFT: c_int = 24;
pub const IMG_IR_PERIOD_DUTY: c_uint = 0x003f0000;
pub const IMG_IR_PERIOD_DUTY_SHIFT: c_int = 16;
pub const IMG_IR_STABLE_STOP: c_uint = 0x00003f00;
pub const IMG_IR_STABLE_STOP_SHIFT: c_int = 8;
pub const IMG_IR_STABLE_START: c_uint = 0x0000003f;
pub const IMG_IR_STABLE_START_SHIFT: c_int = 0;
// IMG_IR_POW_MOD_ENABLE
pub const IMG_IR_POWER_OUT_EN: c_uint = 0x00000002;
pub const IMG_IR_POWER_MOD_EN: c_uint = 0x00000001;
// IMG_IR_IRQ_ENABLE, IMG_IR_IRQ_STATUS, IMG_IR_IRQ_CLEAR
pub const IMG_IR_IRQ_DEC2_ERR: c_uint = 0x00000080;
pub const IMG_IR_IRQ_DEC_ERR: c_uint = 0x00000040;
pub const IMG_IR_IRQ_ACT_LEVEL: c_uint = 0x00000020;
pub const IMG_IR_IRQ_FALL_EDGE: c_uint = 0x00000010;
pub const IMG_IR_IRQ_RISE_EDGE: c_uint = 0x00000008;
pub const IMG_IR_IRQ_DATA_MATCH: c_uint = 0x00000004;
pub const IMG_IR_IRQ_DATA2_VALID: c_uint = 0x00000002;
pub const IMG_IR_IRQ_DATA_VALID: c_uint = 0x00000001;
pub const IMG_IR_IRQ_ALL: c_uint = 0x000000ff;

// IMG_IR_CORE_ID
pub const IMG_IR_CORE_ID: c_uint = 0x00ff0000;
pub const IMG_IR_CORE_ID_SHIFT: c_int = 16;
pub const IMG_IR_CORE_CONFIG: c_uint = 0x0000ffff;
pub const IMG_IR_CORE_CONFIG_SHIFT: c_int = 0;
// IMG_IR_CORE_REV
pub const IMG_IR_DESIGNER: c_uint = 0xff000000;
pub const IMG_IR_DESIGNER_SHIFT: c_int = 24;
pub const IMG_IR_MAJOR_REV: c_uint = 0x00ff0000;
pub const IMG_IR_MAJOR_REV_SHIFT: c_int = 16;
pub const IMG_IR_MINOR_REV: c_uint = 0x0000ff00;
pub const IMG_IR_MINOR_REV_SHIFT: c_int = 8;
pub const IMG_IR_MAINT_REV: c_uint = 0x000000ff;
pub const IMG_IR_MAINT_REV_SHIFT: c_int = 0;
//
// struct img_ir_priv - Private driver data.
// @dev:		Platform device.
// @irq:		IRQ number.
// @clk:		Input clock.
// @sys_clk:		System clock.
// @reg_base:		Iomem base address of IR register block.
// @lock:		Protects IR registers and variables in this struct.
// @raw:		Driver data for raw decoder.
// @hw:			Driver data for hardware decoder.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_priv {
    pub dev: *mut device,
    pub irq: c_int,
    pub clk: *mut clk,
    pub sys_clk: *mut clk,
    pub reg_base: *mut void __iomem,
    pub lock: spinlock_t,
    pub raw: img_ir_priv_raw,
    pub hw: img_ir_priv_hw,
}

// Hardware access
extern "C" {
    pub fn ioread32(reg_offs: priv->reg_base +) -> return;
}
