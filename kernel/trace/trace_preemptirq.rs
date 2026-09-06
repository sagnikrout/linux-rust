//! Automatically rewritten from C to Rust
//! Source: kernel/trace/trace_preemptirq.c
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
// preemptoff and irqoff tracepoints
//
// Copyright (C) Joel Fernandes (Google) <joel@joelfernandes.org>
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// Use regular trace points on architectures that implement noinstr
// tooling: these calls will only happen with RCU enabled, which can
// use a regular tracepoint.
//
// On older architectures, RCU may not be watching in idle. In that
// case, wake up RCU to watch while calling the tracepoint. These
// aren't NMI-safe - so exclude NMI contexts:
//

    do {							\
    if (__trace_##point##_enabled()) {		\
    bool exit_rcu = false;			\
    if (in_nmi())				\
    break;				\
    if (!IS_ENABLED(CONFIG_TINY_RCU) &&	\
    is_idle_task(current)) {		\
    ct_irq_enter();			\
    exit_rcu = true;		\
    }					\
    trace_##point(args);			\
    if (exit_rcu)				\
    ct_irq_exit();			\
    }						\
    } while (0)

// Per-cpu variable to prevent redundant calls when IRQs already off
    static DEFINE_PER_CPU(int, tracing_irq_cpu);
//
// Like trace_hardirqs_on() but without the lockdep invocation. This is
// used in the low level entry code where the ordering vs. RCU is important
// and lockdep uses a staged approach which splits the lockdep hardirq
// tracking into a RCU on and a RCU off section.
//
#[no_mangle]
pub unsafe extern "C" fn trace_hardirqs_on_prepare() {
    void trace_hardirqs_on_prepare(void)
    {
    if (this_cpu_read(tracing_irq_cpu)) {
    trace(irq_enable, TP_ARGS(CALLER_ADDR0, CALLER_ADDR1));
    tracer_hardirqs_on(CALLER_ADDR0, CALLER_ADDR1);
    this_cpu_write(tracing_irq_cpu, 0);
    }
    }
    EXPORT_SYMBOL(trace_hardirqs_on_prepare);
    NOKPROBE_SYMBOL(trace_hardirqs_on_prepare);
#[no_mangle]
pub unsafe extern "C" fn trace_hardirqs_on() {
    void trace_hardirqs_on(void)
    {
    if (this_cpu_read(tracing_irq_cpu)) {
    trace(irq_enable, TP_ARGS(CALLER_ADDR0, CALLER_ADDR1));
    tracer_hardirqs_on(CALLER_ADDR0, CALLER_ADDR1);
    this_cpu_write(tracing_irq_cpu, 0);
    }
    lockdep_hardirqs_on_prepare();
    lockdep_hardirqs_on(CALLER_ADDR0);
    }
    EXPORT_SYMBOL(trace_hardirqs_on);
    NOKPROBE_SYMBOL(trace_hardirqs_on);
//
// Like trace_hardirqs_off() but without the lockdep invocation. This is
// used in the low level entry code where the ordering vs. RCU is important
// and lockdep uses a staged approach which splits the lockdep hardirq
// tracking into a RCU on and a RCU off section.
//
#[no_mangle]
pub unsafe extern "C" fn trace_hardirqs_off_finish() {
    void trace_hardirqs_off_finish(void)
    {
    if (!this_cpu_read(tracing_irq_cpu)) {
    this_cpu_write(tracing_irq_cpu, 1);
    tracer_hardirqs_off(CALLER_ADDR0, CALLER_ADDR1);
    trace(irq_disable, TP_ARGS(CALLER_ADDR0, CALLER_ADDR1));
    }
    }
    EXPORT_SYMBOL(trace_hardirqs_off_finish);
    NOKPROBE_SYMBOL(trace_hardirqs_off_finish);
#[no_mangle]
pub unsafe extern "C" fn trace_hardirqs_off() {
    void trace_hardirqs_off(void)
    {
    lockdep_hardirqs_off(CALLER_ADDR0);
    if (!this_cpu_read(tracing_irq_cpu)) {
    this_cpu_write(tracing_irq_cpu, 1);
    tracer_hardirqs_off(CALLER_ADDR0, CALLER_ADDR1);
    trace(irq_disable, TP_ARGS(CALLER_ADDR0, CALLER_ADDR1));
    }
    }
    EXPORT_SYMBOL(trace_hardirqs_off);
    NOKPROBE_SYMBOL(trace_hardirqs_off);

#[no_mangle]
pub unsafe extern "C" fn trace_preempt_on(a0: c_ulong, a1: c_ulong) {
    void trace_preempt_on(unsigned long a0, unsigned long a1)
    {
    trace(preempt_enable, TP_ARGS(a0, a1));
    tracer_preempt_on(a0, a1);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_preempt_off(a0: c_ulong, a1: c_ulong) {
    void trace_preempt_off(unsigned long a0, unsigned long a1)
    {
    trace(preempt_disable, TP_ARGS(a0, a1));
    tracer_preempt_off(a0, a1);
    }
