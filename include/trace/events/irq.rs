//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/irq.h
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
// irq_handler_entry - called immediately before the irq action handler
// @irq: irq number
// @action: pointer to struct irqaction
//
// The struct irqaction pointed to by @action contains various
// information about the handler, including the device name,
// @action->name, and the device id, @action->dev_id. When used in
// conjunction with the irq_handler_exit tracepoint, we can figure
// out irq handler latencies.
//
// irq_handler_exit - called immediately after the irq action handler returns
// @irq: irq number
// @action: pointer to struct irqaction
// @ret: return value
//
// If the @ret value is set to IRQ_HANDLED, then we know that the corresponding
// @action->handler successfully handled this irq. Otherwise, the irq might be
// a shared irq line, or the irq was not handled successfully. Can be used in
// conjunction with the irq_handler_entry to understand irq handler latencies.
//
// softirq_entry - called immediately before the softirq handler
// @vec_nr:  softirq vector number
//
// When used in combination with the softirq_exit tracepoint
// we can determine the softirq handler routine.
//
// softirq_exit - called immediately after the softirq handler returns
// @vec_nr:  softirq vector number
//
// When used in combination with the softirq_entry tracepoint
// we can determine the softirq handler routine.
//
// softirq_raise - called immediately when a softirq is raised
// @vec_nr:  softirq vector number
//
// When used in combination with the softirq_entry tracepoint
// we can determine the softirq raise to run latency.
//
// tasklet_entry - called immediately before the tasklet is run
// @t: tasklet pointer
// @func: tasklet callback or function being run
//
// Used to find individual tasklet execution time
//
// tasklet_exit - called immediately after the tasklet is run
// @t: tasklet pointer
// @func: tasklet callback or function being run
//
// Used to find individual tasklet execution time
//

// This part must be outside protection
