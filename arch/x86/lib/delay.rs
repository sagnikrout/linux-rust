//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/delay.c
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
// Precise Delay Loops for i386
//
// Copyright (C) 1993 Linus Torvalds
// Copyright (C) 1997 Martin Mares <mj@atrey.karlin.mff.cuni.cz>
// Copyright (C) 2008 Jiri Hladky <hladky _dot_ jiri _at_ gmail _dot_ com>
//
// The __delay function must _NOT_ be inlined as its execution time
// depends wildly on alignment on many x86 processors. The additional
// jump magic is needed to get the timing stable on all the CPU's
// we have to worry about.
//

    static void delay_loop(u64 __loops);
//
// Calibration and selection of the delay mechanism happens only once
// during boot.
//
    static void (*delay_fn)(u64) __ro_after_init = delay_loop;
    static void (*delay_halt_fn)(u64 start, u64 cycles) __ro_after_init;
// simple loop based delay:
#[no_mangle]
unsafe extern "C" fn delay_loop(__loops: u64) {
    static void delay_loop(u64 __loops)
    {
    let mut loops: c_ulong = (unsigned long)__loops;
    asm volatile(
    "	test %0,%0	\n"
    "	jz 3f		\n"
    "	jmp 1f		\n"
    ".align 16		\n"
    "1:	jmp 2f		\n"
    ".align 16		\n"
    "2:	dec %0		\n"
    "	jnz 2b		\n"
    "3:	dec %0		\n"
    : "+a" (loops)
    :
    );
    }
// TSC based delay:
#[no_mangle]
unsafe extern "C" fn delay_tsc(cycles: u64) {
    static void delay_tsc(u64 cycles)
    {
    u64 bclock, now;
    int cpu;
    preempt_disable();
    cpu = smp_processor_id();
    bclock = rdtsc_ordered();
    for (;;) {
    now = rdtsc_ordered();
    if ((now - bclock) >= cycles)
    break;
// Allow RT tasks to run
    preempt_enable();
    native_pause();
    preempt_disable();
//
// It is possible that we moved to another CPU, and
// since TSC's are per-cpu we need to calculate
// that. The delay must guarantee that we wait "at
// least" the amount of time. Being moved to another
// CPU could make the wait longer but we just need to
// make sure we waited long enough. Rebalance the
// counter for this CPU.
//
    if (unlikely(cpu != smp_processor_id())) {
    cycles -= (now - bclock);
    cpu = smp_processor_id();
    bclock = rdtsc_ordered();
    }
    }
    preempt_enable();
    }
//
// On Intel the TPAUSE instruction waits until any of:
// 1) the TSC counter exceeds the value provided in EDX:EAX
// 2) global timeout in IA32_UMWAIT_CONTROL is exceeded
// 3) an external interrupt occurs
//
#[no_mangle]
unsafe extern "C" fn delay_halt_tpause(start: u64, cycles: u64) {
    static void delay_halt_tpause(u64 start, u64 cycles)
    {
    let mut until: u64 = start + cycles;
    u32 eax, edx;
    eax = lower_32_bits(until);
    edx = upper_32_bits(until);
//
// Hard code the deeper (C0.2) sleep state because exit latency is
// small compared to the "microseconds" that usleep() will delay.
//
    __tpause(TPAUSE_C02_STATE, edx, eax);
    }
//
// On some AMD platforms, MWAITX has a configurable 32-bit timer, that
// counts with TSC frequency. The input value is the number of TSC cycles
// to wait. MWAITX will also exit when the timer expires.
//
#[no_mangle]
unsafe extern "C" fn delay_halt_mwaitx(unused: u64, cycles: u64) {
    static void delay_halt_mwaitx(u64 unused, u64 cycles)
    {
    u64 delay;
    delay = min_t(u64, MWAITX_MAX_WAIT_CYCLES, cycles);
//
// Use cpu_tss_rw as a cacheline-aligned, seldom accessed per-cpu
// variable as the monitor target.
//
    __monitorx(raw_cpu_ptr(&cpu_tss_rw), 0, 0);
//
// AMD, like Intel, supports the EAX hint and EAX=0xf means, do not
// enter any deep C-state and we use it here in delay() to minimize
// wakeup latency.
//
    __mwaitx(MWAITX_DISABLE_CSTATES, delay, MWAITX_ECX_TIMER_ENABLE);
    }
//
// Call a vendor specific function to delay for a given amount of time. Because
// these functions may return earlier than requested, check for actual elapsed
// time and call again until done.
//
#[no_mangle]
unsafe extern "C" fn delay_halt(__cycles: u64) {
    static void delay_halt(u64 __cycles)
    {
    u64 start, end, cycles = __cycles;
//
// Timer value of 0 causes MWAITX to wait indefinitely, unless there
// is a store on the memory monitored by MONITORX.
//
    if (!cycles)
    return;
    start = rdtsc_ordered();
    for (;;) {
    delay_halt_fn(start, cycles);
    end = rdtsc_ordered();
    if (cycles <= end - start)
    break;
    cycles -= end - start;
    start = end;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn use_tsc_delay() -> void __init {
    void __init use_tsc_delay(void)
    {
    if (delay_fn == delay_loop)
    delay_fn = delay_tsc;
    }
#[no_mangle]
pub unsafe extern "C" fn use_tpause_delay() -> void __init {
    void __init use_tpause_delay(void)
    {
    delay_halt_fn = delay_halt_tpause;
    delay_fn = delay_halt;
    }
#[no_mangle]
pub unsafe extern "C" fn use_mwaitx_delay() {
    void use_mwaitx_delay(void)
    {
    delay_halt_fn = delay_halt_mwaitx;
    delay_fn = delay_halt;
    }
#[no_mangle]
pub unsafe extern "C" fn delay_read_timer(timer_val: *mut c_ulong) -> bool {
    bool delay_read_timer(unsigned long *timer_val)
    {
    if (delay_fn == delay_tsc) {
// timer_val = rdtsc();
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn __delay(loops: c_ulong) {
    void __delay(unsigned long loops)
    {
    delay_fn(loops);
    }
    EXPORT_SYMBOL(__delay);
#[no_mangle]
pub unsafe extern "C" fn __const_udelay(xloops: c_ulong) -> noinline void {
    noinline void __const_udelay(unsigned long xloops)
    {
    let mut lpj: c_ulong = this_cpu_read(cpu_info.loops_per_jiffy) ? : loops_per_jiffy;
    int d0;
    xloops *= 4;
    asm("mull %%edx"
    :"=d" (xloops), "=&a" (d0)
    :"1" (xloops), "0" (lpj * (HZ / 4)));
    __delay(++xloops);
    }
    EXPORT_SYMBOL(__const_udelay);
#[no_mangle]
pub unsafe extern "C" fn __udelay(usecs: c_ulong) {
    void __udelay(unsigned long usecs)
    {
    __const_udelay(usecs * 0x000010c7); /* 2**32 / 1000000 (rounded up) */
    }
    EXPORT_SYMBOL(__udelay);
#[no_mangle]
pub unsafe extern "C" fn __ndelay(nsecs: c_ulong) {
    void __ndelay(unsigned long nsecs)
    {
    __const_udelay(nsecs * 0x00005); /* 2**32 / 1000000000 (rounded up) */
    }
    EXPORT_SYMBOL(__ndelay);
