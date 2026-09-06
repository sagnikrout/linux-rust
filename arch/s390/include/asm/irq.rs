//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/irq.h
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
pub const EXT_INTERRUPT: c_int = 0;
pub const IO_INTERRUPT: c_int = 1;
pub const THIN_INTERRUPT: c_int = 2;
pub const NR_IRQS_BASE: c_int = 3;

// External interruption codes
pub const EXT_IRQ_INTERRUPT_KEY: c_uint = 0x0040;
pub const EXT_IRQ_CLK_COMP: c_uint = 0x1004;
pub const EXT_IRQ_CPU_TIMER: c_uint = 0x1005;
pub const EXT_IRQ_WARNING_TRACK: c_uint = 0x1007;
pub const EXT_IRQ_MALFUNC_ALERT: c_uint = 0x1200;
pub const EXT_IRQ_EMERGENCY_SIG: c_uint = 0x1201;
pub const EXT_IRQ_EXTERNAL_CALL: c_uint = 0x1202;
pub const EXT_IRQ_TIMING_ALERT: c_uint = 0x1406;
pub const EXT_IRQ_MEASURE_ALERT: c_uint = 0x1407;
pub const EXT_IRQ_SERVICE_SIG: c_uint = 0x2401;
pub const EXT_IRQ_CP_SERVICE: c_uint = 0x2603;
pub const EXT_IRQ_IUCV: c_uint = 0x4000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum interruption_class {
    IRQEXT_CLK,
    IRQEXT_EXC,
    IRQEXT_EMS,
    IRQEXT_TMR,
    IRQEXT_TLA,
    IRQEXT_PFL,
    IRQEXT_DSD,
    IRQEXT_VRT,
    IRQEXT_SCP,
    IRQEXT_IUC,
    IRQEXT_CMS,
    IRQEXT_CMC,
    IRQEXT_FTP,
    IRQEXT_WTI,
    IRQIO_CIO,
    IRQIO_DAS,
    IRQIO_C15,
    IRQIO_C70,
    IRQIO_TAP,
    IRQIO_VMR,
    IRQIO_CTC,
    IRQIO_ADM,
    IRQIO_CSC,
    IRQIO_VIR,
    IRQIO_QAI,
    IRQIO_APB,
    IRQIO_PCF,
    IRQIO_PCD,
    IRQIO_MSI,
    IRQIO_VAI,
    IRQIO_GAL,
    NMI_NMI,
    CPU_RST,
    NR_ARCH_IRQS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_stat {
    pub irqs: [c_uint; NR_ARCH_IRQS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_code {
    pub subcode: c_ushort,
    pub code: c_ushort,
}

extern "C" {
    pub fn void(ext_code: *mut *mut ext_int_handler_t)(struct, int: unsigned, long: unsigned) -> typedef;
}
extern "C" {
    pub fn register_external_irq(code: u16, handler: ext_int_handler_t) -> c_int;
}
extern "C" {
    pub fn unregister_external_irq(code: u16, handler: ext_int_handler_t) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irq_subclass {
    IRQ_SUBCLASS_MEASUREMENT_ALERT = 5,
    IRQ_SUBCLASS_SERVICE_SIGNAL = 9,
    IRQ_SUBCLASS_WARNING_TRACK = 33,
}

extern "C" {
    pub fn irq_subclass_register(subclass: irq_subclass);
}
extern "C" {
    pub fn irq_subclass_unregister(subclass: irq_subclass);
}

