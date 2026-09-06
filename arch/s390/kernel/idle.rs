//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/idle.c
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
// Idle functions for s390.
//
// Copyright IBM Corp. 2014
//
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//

    DEFINE_PER_CPU(struct s390_idle_data, s390_idle);
#[no_mangle]
unsafe extern "C" fn __account_idle_time_irq() -> __always_inline void {
    static __always_inline void __account_idle_time_irq(void)
    {
    struct s390_idle_data *idle = this_cpu_ptr(&s390_idle);
    unsigned long idle_time;
    idle_time = idle.clock_idle_exit.tod - idle.clock_idle_enter.tod;
    account_idle_time(cputime_to_nsecs(idle_time));
    }
#[no_mangle]
unsafe extern "C" fn __account_idle_time_setup() -> __always_inline void {
    static __always_inline void __account_idle_time_setup(void)
    {
    struct s390_idle_data *idle = this_cpu_ptr(&s390_idle);
    store_tod_clock_ext(&idle.clock_idle_enter);
    idle.timer_idle_enter = get_cpu_timer();
    idle.clock_idle_exit = idle.clock_idle_enter;
    }

#[no_mangle]
unsafe extern "C" fn arch_cpu_in_idle_time(cpu: c_int) -> u64 {
    static u64 arch_cpu_in_idle_time(int cpu)
    {
    struct s390_idle_data *idle = &per_cpu(s390_idle, cpu);
    union tod_clock now;
    u64 idle_time;
    if (!idle.in_idle)
    return 0;
    store_tod_clock_ext(&now);
    if (tod_after(idle.clock_idle_exit.tod, idle.clock_idle_enter.tod))
    idle_time = idle.clock_idle_exit.tod - idle.clock_idle_enter.tod;
    else
    idle_time = now.tod - idle.clock_idle_enter.tod;
    return cputime_to_nsecs(idle_time);
    }
#[no_mangle]
unsafe extern "C" fn arch_cpu_idle_time(cpu: c_int, idx: enum cpu_usage_stat, compute_delta: bool) -> u64 {
    static u64 arch_cpu_idle_time(int cpu, enum cpu_usage_stat idx, bool compute_delta)
    {
    struct kernel_cpustat *kc = &kcpustat_cpu(cpu);
    u64 *cpustat = kc.cpustat;
    unsigned int seq;
    u64 idle_time;
//
// The open coded seqcount writer in entry.S relies on the
// raw counting mechanism without any writer protection.
//
    typecheck(typeof(kc.idle_sleeptime_seq), seqcount_t);
    do {
    seq = read_seqcount_begin(&kc.idle_sleeptime_seq);
    idle_time = cpustat[idx];
    if (compute_delta)
    idle_time += arch_cpu_in_idle_time(cpu);
    } while (read_seqcount_retry(&kc.idle_sleeptime_seq, seq));
    return idle_time;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_kcpustat_field_idle(cpu: c_int) -> u64 {
    u64 arch_kcpustat_field_idle(int cpu)
    {
    return arch_cpu_idle_time(cpu, CPUTIME_IDLE, !nr_iowait_cpu(cpu));
    }
    EXPORT_SYMBOL_GPL(arch_kcpustat_field_idle);
#[no_mangle]
pub unsafe extern "C" fn arch_kcpustat_field_iowait(cpu: c_int) -> u64 {
    u64 arch_kcpustat_field_iowait(int cpu)
    {
    return arch_cpu_idle_time(cpu, CPUTIME_IOWAIT, nr_iowait_cpu(cpu));
    }
    EXPORT_SYMBOL_GPL(arch_kcpustat_field_iowait);
#[no_mangle]
pub unsafe extern "C" fn account_idle_time_irq() {
    void account_idle_time_irq(void)
    {
    struct s390_idle_data *idle = this_cpu_ptr(&s390_idle);
    struct kernel_cpustat *kc = kcpustat_this_cpu;
    write_seqcount_begin(&kc.idle_sleeptime_seq);
    idle.in_idle = false;
    __account_idle_time_irq();
    write_seqcount_end(&kc.idle_sleeptime_seq);
    }
#[no_mangle]
unsafe extern "C" fn account_idle_time_setup() -> __always_inline void {
    static __always_inline void account_idle_time_setup(void)
    {
    struct s390_idle_data *idle = this_cpu_ptr(&s390_idle);
    struct kernel_cpustat *kc = kcpustat_this_cpu;
    raw_write_seqcount_begin(&kc.idle_sleeptime_seq);
    idle.in_idle = true;
    __account_idle_time_setup();
    raw_write_seqcount_end(&kc.idle_sleeptime_seq);
    }

#[no_mangle]
pub unsafe extern "C" fn account_idle_time_irq() {
    void account_idle_time_irq(void)
    {
    __account_idle_time_irq();
    }
#[no_mangle]
unsafe extern "C" fn account_idle_time_setup() -> __always_inline void {
    static __always_inline void account_idle_time_setup(void)
    {
    __account_idle_time_setup();
    }

#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle() -> void noinstr {
    void noinstr arch_cpu_idle(void)
    {
    struct s390_idle_data *idle = this_cpu_ptr(&s390_idle);
    unsigned long psw_mask;
// Wait for external, I/O or machine check interrupt.
    psw_mask = PSW_KERNEL_BITS | PSW_MASK_WAIT |
    PSW_MASK_IO | PSW_MASK_EXT | PSW_MASK_MCHECK;
    set_cpu_flag(CIF_ENABLED_WAIT);
    if (smp_cpu_mtid)
    stcctm(MT_DIAG, smp_cpu_mtid, (u64 *)&idle.mt_cycles_enter);
    account_idle_time_setup();
    bpon();
    __load_psw_mask(psw_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle_enter() {
    void arch_cpu_idle_enter(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle_exit() {
    void arch_cpu_idle_exit(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle_dead() -> void __noreturn {
    void __noreturn arch_cpu_idle_dead(void)
    {
    cpu_die();
    }
