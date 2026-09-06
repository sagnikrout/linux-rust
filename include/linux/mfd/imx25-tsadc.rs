//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/imx25-tsadc.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mx25_tsadc {
    pub regs: *mut regmap,
    pub domain: *mut irq_domain,
    pub clk: *mut clk,
}

pub const MX25_TSC_TGCR: c_uint = 0x00;
pub const MX25_TSC_TGSR: c_uint = 0x04;
pub const MX25_TSC_TICR: c_uint = 0x08;
// The same register layout for TC and GC queue
pub const MX25_ADCQ_FIFO: c_uint = 0x00;
pub const MX25_ADCQ_CR: c_uint = 0x04;
pub const MX25_ADCQ_SR: c_uint = 0x08;
pub const MX25_ADCQ_MR: c_uint = 0x0c;
pub const MX25_ADCQ_ITEM_7_0: c_uint = 0x20;
pub const MX25_ADCQ_ITEM_15_8: c_uint = 0x24;

pub const MX25_ADCQ_MR_MASK: c_uint = 0xffffffff;
// TGCR

// TGSR

// ADCQ_ITEM_*

// ADCQ_FIFO (TCQFIFO and GCQFIFO)

// ADCQ_CR (TCQR and GCQR)

pub const MX25_ADCQ_CR_QSM_PD: c_uint = 0x1;
pub const MX25_ADCQ_CR_QSM_FQS: c_uint = 0x2;
pub const MX25_ADCQ_CR_QSM_FQS_PD: c_uint = 0x3;
// ADCQ_SR (TCQSR and GCQSR)

// ADCQ_MR (TCQMR and GCQMR)

// ADCQ_CFG (TICR, TCC0-7,GCC0-7)

