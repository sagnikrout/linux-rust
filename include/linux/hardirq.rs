//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hardirq.h
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

extern "C" {
    pub fn synchronize_irq(irq: c_uint);
}
extern "C" {
    pub fn synchronize_hardirq(irq: c_uint) -> bool;
}

extern "C" {
    pub fn __rcu_irq_enter_check_tick();
}

//
// It is safe to do non-atomic ops on ->hardirq_context,
// because NMI handlers may not preempt and the ops are
// always balanced, so the interrupted value of ->hardirq_context
// will always be restored.
//

//
// Like __irq_enter() without time accounting for fast
// interrupts, e.g. reschedule IPI where time accounting
// is more expensive than the actual interrupt.
//

//
// Enter irq context (on NO_HZ, update jiffies):
//
extern "C" {
    pub fn irq_enter();
}
//
// Like irq_enter(), but RCU is already watching.
//
extern "C" {
    pub fn irq_enter_rcu();
}
//
// Exit irq context without processing softirqs:
//

//
// Like __irq_exit() without time accounting
//

//
// Exit irq context and process softirqs if needed:
//
extern "C" {
    pub fn irq_exit();
}
//
// Like irq_exit(), but return with RCU watching.
//
extern "C" {
    pub fn irq_exit_rcu();
}

// Maximum NMI nesting is 15. */		\

//
// NMI vs Tracing
// --------------
//
// We must not land in a tracer until (or after) we've changed preempt_count
// such that in_nmi() becomes true. To that effect all NMI C entry points must
// be marked 'notrace' and call nmi_enter() as soon as possible.
//
// nmi_enter() can nest - nesting is tracked in a per-CPU counter.
//

