//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/adc/xilinx-xadc.h
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
// Xilinx XADC driver
//
// Copyright 2013 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

extern "C" {
    pub fn xadc_handle_events(indio_dev: *mut iio_dev, events: c_ulong);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xadc_external_mux_mode {
    XADC_EXTERNAL_MUX_NONE,
    XADC_EXTERNAL_MUX_SINGLE,
    XADC_EXTERNAL_MUX_DUAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xadc {
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub ops: *const xadc_ops,
    pub threshold: [u16; 16],
    pub temp_hysteresis: u16,
    pub alarm_mask: c_uint,
    pub data: *mut u16,
    pub trigger: *mut iio_trigger,
    pub convst_trigger: *mut iio_trigger,
    pub samplerate_trigger: *mut iio_trigger,
    pub external_mux_mode: xadc_external_mux_mode,
    pub zynq_masked_alarm: c_uint,
    pub zynq_intmask: c_uint,
    pub zynq_unmask_work: delayed_work,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub completion: completion,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xadc_type {
    XADC_TYPE_S7, /* Series 7 */
    XADC_TYPE_US, /* UltraScale and UltraScale+ */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xadc_ops {
    pub val): *mut *mut *mut int (read)(struct xadc xadc, unsigned int reg, uint16_t,
    pub val): *mut *mut *mut int (write)(struct xadc xadc, unsigned int reg, uint16_t,
    pub irq): c_int,
    pub alarm): *mut *mut *mut void (update_alarm)(struct xadc xadc, unsigned int,
    pub xadc): *mut *mut unsigned long (get_dclk_rate)(struct xadc,
    pub devid): *mut *mut irqreturn_t (interrupt_handler)(int irq, void,
    pub flags: c_uint,
    pub type: xadc_type,
    pub temp_scale: c_int,
    pub temp_offset: c_int,
}

// XADC hardmacro register definitions
pub const XADC_REG_TEMP: c_uint = 0x00;
pub const XADC_REG_VCCINT: c_uint = 0x01;
pub const XADC_REG_VCCAUX: c_uint = 0x02;
pub const XADC_REG_VPVN: c_uint = 0x03;
pub const XADC_REG_VREFP: c_uint = 0x04;
pub const XADC_REG_VREFN: c_uint = 0x05;
pub const XADC_REG_VCCBRAM: c_uint = 0x06;
pub const XADC_REG_VCCPINT: c_uint = 0x0d;
pub const XADC_REG_VCCPAUX: c_uint = 0x0e;
pub const XADC_REG_VCCO_DDR: c_uint = 0x0f;

pub const XADC_REG_MAX_TEMP: c_uint = 0x20;
pub const XADC_REG_MAX_VCCINT: c_uint = 0x21;
pub const XADC_REG_MAX_VCCAUX: c_uint = 0x22;
pub const XADC_REG_MAX_VCCBRAM: c_uint = 0x23;
pub const XADC_REG_MIN_TEMP: c_uint = 0x24;
pub const XADC_REG_MIN_VCCINT: c_uint = 0x25;
pub const XADC_REG_MIN_VCCAUX: c_uint = 0x26;
pub const XADC_REG_MIN_VCCBRAM: c_uint = 0x27;
pub const XADC_REG_MAX_VCCPINT: c_uint = 0x28;
pub const XADC_REG_MAX_VCCPAUX: c_uint = 0x29;
pub const XADC_REG_MAX_VCCO_DDR: c_uint = 0x2a;
pub const XADC_REG_MIN_VCCPINT: c_uint = 0x2c;
pub const XADC_REG_MIN_VCCPAUX: c_uint = 0x2d;
pub const XADC_REG_MIN_VCCO_DDR: c_uint = 0x2e;
pub const XADC_REG_CONF0: c_uint = 0x40;
pub const XADC_REG_CONF1: c_uint = 0x41;
pub const XADC_REG_CONF2: c_uint = 0x42;

pub const XADC_REG_FLAG: c_uint = 0x3f;

pub const XADC_CONF1_ALARM_MASK: c_uint = 0x0f0f;
pub const XADC_CONF2_DIV_MASK: c_uint = 0xff00;
pub const XADC_CONF2_DIV_OFFSET: c_int = 8;

pub const XADC_THRESHOLD_TEMP_MAX: c_uint = 0x0;
pub const XADC_THRESHOLD_VCCINT_MAX: c_uint = 0x1;
pub const XADC_THRESHOLD_VCCAUX_MAX: c_uint = 0x2;
pub const XADC_THRESHOLD_OT_MAX: c_uint = 0x3;
pub const XADC_THRESHOLD_TEMP_MIN: c_uint = 0x4;
pub const XADC_THRESHOLD_VCCINT_MIN: c_uint = 0x5;
pub const XADC_THRESHOLD_VCCAUX_MIN: c_uint = 0x6;
pub const XADC_THRESHOLD_OT_MIN: c_uint = 0x7;
pub const XADC_THRESHOLD_VCCBRAM_MAX: c_uint = 0x8;
pub const XADC_THRESHOLD_VCCPINT_MAX: c_uint = 0x9;
pub const XADC_THRESHOLD_VCCPAUX_MAX: c_uint = 0xa;
pub const XADC_THRESHOLD_VCCODDR_MAX: c_uint = 0xb;
pub const XADC_THRESHOLD_VCCBRAM_MIN: c_uint = 0xc;
pub const XADC_THRESHOLD_VCCPINT_MIN: c_uint = 0xd;
pub const XADC_THRESHOLD_VCCPAUX_MIN: c_uint = 0xe;
pub const XADC_THRESHOLD_VCCODDR_MIN: c_uint = 0xf;
