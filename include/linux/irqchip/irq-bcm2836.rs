//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/irq-bcm2836.h
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
// Root interrupt controller for the BCM2836 (Raspberry Pi 2).
//
// Copyright 2015 Broadcom
//
pub const LOCAL_CONTROL: c_uint = 0x000;
pub const LOCAL_PRESCALER: c_uint = 0x008;
//
// The low 2 bits identify the CPU that the GPU IRQ goes to, and the
// next 2 bits identify the CPU that the GPU FIQ goes to.
//
pub const LOCAL_GPU_ROUTING: c_uint = 0x00c;
// When setting bits 0-3, enables PMU interrupts on that CPU.
pub const LOCAL_PM_ROUTING_SET: c_uint = 0x010;
// When setting bits 0-3, disables PMU interrupts on that CPU.
pub const LOCAL_PM_ROUTING_CLR: c_uint = 0x014;
//
// The low 4 bits of this are the CPU's timer IRQ enables, and the
// next 4 bits are the CPU's timer FIQ enables (which override the IRQ
// bits).
//
pub const LOCAL_TIMER_INT_CONTROL0: c_uint = 0x040;
//
// The low 4 bits of this are the CPU's per-mailbox IRQ enables, and
// the next 4 bits are the CPU's per-mailbox FIQ enables (which
// override the IRQ bits).
//
pub const LOCAL_MAILBOX_INT_CONTROL0: c_uint = 0x050;
//
// The CPU's interrupt status register.  Bits are defined by the
// LOCAL_IRQ_* bits below.
//
pub const LOCAL_IRQ_PENDING0: c_uint = 0x060;
// Same status bits as above, but for FIQ.
pub const LOCAL_FIQ_PENDING0: c_uint = 0x070;
//
// Mailbox write-to-set bits.  There are 16 mailboxes, 4 per CPU, and
// these bits are organized by mailbox number and then CPU number.  We
// use mailbox 0 for IPIs.  The mailbox's interrupt is raised while
// any bit is set.
//
pub const LOCAL_MAILBOX0_SET0: c_uint = 0x080;
pub const LOCAL_MAILBOX3_SET0: c_uint = 0x08c;
// Mailbox write-to-clear bits.
pub const LOCAL_MAILBOX0_CLR0: c_uint = 0x0c0;
pub const LOCAL_MAILBOX3_CLR0: c_uint = 0x0cc;
pub const LOCAL_IRQ_CNTPSIRQ: c_int = 0;
pub const LOCAL_IRQ_CNTPNSIRQ: c_int = 1;
pub const LOCAL_IRQ_CNTHPIRQ: c_int = 2;
pub const LOCAL_IRQ_CNTVIRQ: c_int = 3;
pub const LOCAL_IRQ_MAILBOX0: c_int = 4;
pub const LOCAL_IRQ_MAILBOX1: c_int = 5;
pub const LOCAL_IRQ_MAILBOX2: c_int = 6;
pub const LOCAL_IRQ_MAILBOX3: c_int = 7;
pub const LOCAL_IRQ_GPU_FAST: c_int = 8;
pub const LOCAL_IRQ_PMU_FAST: c_int = 9;
