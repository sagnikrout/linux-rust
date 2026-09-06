//! Automatically rewritten from C to Rust
//! Source: init/calibrate.c
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
// calibrate.c: default delay calibration
//
// Excised from init/main.c
// Copyright (C) 1991, 1992  Linus Torvalds
//

    unsigned long lpj_fine;
    unsigned long preset_lpj;
#[no_mangle]
unsafe extern "C" fn lpj_setup(str: *mut c_char) -> int __init {
    static int __init lpj_setup(char *str)
    {
    return kstrtoul(str, 0, &preset_lpj) == 0;
    }
    __setup("lpj=", lpj_setup);

// This routine uses the delay_read_timer() routine and gets the
// loops per jiffy directly, instead of guessing it using delay().
// Also, this code tries to handle non-maskable asynchronous events
// (like SMIs)
//

pub const MAX_DIRECT_CALIBRATION_RETRIES: c_int = 5;
#[no_mangle]
unsafe extern "C" fn calibrate_delay_direct() -> c_ulong {
    static unsigned long calibrate_delay_direct(void)
    {
    unsigned long pre_start, start, post_start;
    unsigned long pre_end, end, post_end;
    unsigned long start_jiffies;
    unsigned long timer_rate_min, timer_rate_max;
    let mut good_timer_sum: c_ulong = 0;
    let mut good_timer_count: c_ulong = 0;
    unsigned long measured_times[MAX_DIRECT_CALIBRATION_RETRIES];
    int max = -1; /* index of measured_times with max/min values or not set */
    let mut min: c_int = -1;
    int i;
    if (!delay_read_timer(&pre_start))
    return 0;
//
// A simple loop like
// while ( jiffies < start_jiffies+1)
// start = delay_read_timer();
// will not do. As we don't really know whether jiffy switch
// happened first or timer_value was read first. And some asynchronous
// event can happen between these two events introducing errors in lpj.
//
// So, we do
// 1. pre_start <- When we are sure that jiffy switch hasn't happened
// 2. check jiffy switch
// 3. start <- timer value before or after jiffy switch
// 4. post_start <- When we are sure that jiffy switch has happened
//
// Note, we don't know anything about order of 2 and 3.
// Now, by looking at post_start and pre_start difference, we can
// check whether any asynchronous event happened or not
//
    for (i = 0; i < MAX_DIRECT_CALIBRATION_RETRIES; i++) {
    pre_start = 0;
    delay_read_timer(&start);
    start_jiffies = jiffies;
    while (time_before_eq(jiffies, start_jiffies + 1)) {
    pre_start = start;
    delay_read_timer(&start);
    }
    delay_read_timer(&post_start);
    pre_end = 0;
    end = post_start;
    while (time_before_eq(jiffies, start_jiffies + 1 +
    DELAY_CALIBRATION_TICKS)) {
    pre_end = end;
    delay_read_timer(&end);
    }
    delay_read_timer(&post_end);
    timer_rate_max = (post_end - pre_start) /
    DELAY_CALIBRATION_TICKS;
    timer_rate_min = (pre_end - post_start) /
    DELAY_CALIBRATION_TICKS;
//
// If the upper limit and lower limit of the timer_rate is
// >= 12.5% apart, redo calibration.
//
    if (start >= post_end)
    printk(KERN_NOTICE "calibrate_delay_direct() ignoring "
    "timer_rate as we had a TSC wrap around"
    " start=%lu >=post_end=%lu\n",
    start, post_end);
    if (start < post_end && pre_start != 0 && pre_end != 0 &&
    (timer_rate_max - timer_rate_min) < (timer_rate_max >> 3)) {
    good_timer_count++;
    good_timer_sum += timer_rate_max;
    measured_times[i] = timer_rate_max;
    if (max < 0 || timer_rate_max > measured_times[max])
    max = i;
    if (min < 0 || timer_rate_max < measured_times[min])
    min = i;
    } else
    measured_times[i] = 0;
    }
//
// Find the maximum & minimum - if they differ too much throw out the
// one with the largest difference from the mean and try again...
//
    while (good_timer_count > 1) {
    unsigned long estimate;
    unsigned long maxdiff;
// compute the estimate
    estimate = (good_timer_sum/good_timer_count);
    maxdiff = estimate >> 3;
// if range is within 12% let's take it
    if ((measured_times[max] - measured_times[min]) < maxdiff)
    return estimate;
// ok - drop the worse value and try again...
    good_timer_sum = 0;
    good_timer_count = 0;
    if ((measured_times[max] - estimate) <
    (estimate - measured_times[min])) {
    printk(KERN_NOTICE "calibrate_delay_direct() dropping "
    "min bogoMips estimate %d = %lu\n",
    min, measured_times[min]);
    measured_times[min] = 0;
    min = max;
    } else {
    printk(KERN_NOTICE "calibrate_delay_direct() dropping "
    "max bogoMips estimate %d = %lu\n",
    max, measured_times[max]);
    measured_times[max] = 0;
    max = min;
    }
    for (i = 0; i < MAX_DIRECT_CALIBRATION_RETRIES; i++) {
    if (measured_times[i] == 0)
    continue;
    good_timer_count++;
    good_timer_sum += measured_times[i];
    if (measured_times[i] < measured_times[min])
    min = i;
    if (measured_times[i] > measured_times[max])
    max = i;
    }
    }
    printk(KERN_NOTICE "calibrate_delay_direct() failed to get a good "
    "estimate for loops_per_jiffy.\nProbably due to long platform "
    "interrupts. Consider using \"lpj=\" boot option.\n");
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn calibrate_delay_direct() -> c_ulong {
    static unsigned long calibrate_delay_direct(void)
    {
    return 0;
    }

//
// This is the number of bits of precision for the loops_per_jiffy.  Each
// time we refine our estimate after the first takes 1.5/HZ seconds, so try
// to start with a good estimate.
// For the boot cpu we can skip the delay calibration and assign it a value
// calculated based on the timer frequency.
// For the rest of the CPUs we cannot assume that the timer frequency is same as
// the cpu frequency, hence do the calibration for those.
//
pub const LPS_PREC: c_int = 8;
#[no_mangle]
unsafe extern "C" fn calibrate_delay_converge() -> c_ulong {
    static unsigned long calibrate_delay_converge(void)
    {
// First stage - slowly accelerate to find initial bounds
    unsigned long lpj, lpj_base, ticks, loopadd, loopadd_base, chop_limit;
    let mut trials: c_int = 0, band = 0, trial_in_band = 0;
    lpj = (1<<12);
// wait for "start of" clock tick
    ticks = jiffies;
    while (ticks == jiffies)
    ; /* nothing */
// Go ..
    ticks = jiffies;
    do {
    if (++trial_in_band == (1<<band)) {
    ++band;
    trial_in_band = 0;
    }
    __delay(lpj * band);
    trials += band;
    } while (ticks == jiffies);
//
// We overshot, so retreat to a clear underestimate. Then estimate
// the largest likely undershoot. This defines our chop bounds.
//
    trials -= band;
    loopadd_base = lpj * band;
    lpj_base = lpj * trials;
    recalibrate:
    lpj = lpj_base;
    loopadd = loopadd_base;
//
// Do a binary approximation to get lpj set to
// equal one clock (up to LPS_PREC bits)
//
    chop_limit = lpj >> LPS_PREC;
    while (loopadd > chop_limit) {
    lpj += loopadd;
    ticks = jiffies;
    while (ticks == jiffies)
    ; /* nothing */
    ticks = jiffies;
    __delay(lpj);
    if (jiffies != ticks)	/* longer than 1 tick */
    lpj -= loopadd;
    loopadd >>= 1;
    }
//
// If we incremented every single time possible, presume we've
// massively underestimated initially, and retry with a higher
// start, and larger range. (Only seen on x86_64, due to SMIs)
//
    if (lpj + loopadd * 2 == lpj_base + loopadd_base * 2) {
    lpj_base = lpj;
    loopadd_base <<= 2;
    goto recalibrate;
    }
    return lpj;
    }
    static DEFINE_PER_CPU(unsigned long, cpu_loops_per_jiffy) = { 0 };
//
// Check if cpu calibration delay is already known. For example,
// some processors with multi-core sockets may have all cores
// with the same calibration delay.
//
// Architectures should override this function if a faster calibration
// method is available.
//
#[no_mangle]
pub unsafe extern "C" fn __attribute__(calibrate_delay_is_known(void: (weak))) -> c_ulong {
    unsigned long __attribute__((weak)) calibrate_delay_is_known(void)
    {
    return 0;
    }
//
// Indicate the cpu delay calibration is done. This can be used by
// architectures to stop accepting delay timer registrations after this point.
//
#[no_mangle]
pub unsafe extern "C" fn __attribute__(calibration_delay_done(void: (weak))) {
    void __attribute__((weak)) calibration_delay_done(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn calibrate_delay() {
    void calibrate_delay(void)
    {
    unsigned long lpj;
    static bool printed;
    let mut this_cpu: c_int = smp_processor_id();
    if (per_cpu(cpu_loops_per_jiffy, this_cpu)) {
    lpj = per_cpu(cpu_loops_per_jiffy, this_cpu);
    if (!printed)
    pr_info("Calibrating delay loop (skipped) "
    "already calibrated this CPU");
    } else if (preset_lpj) {
    lpj = preset_lpj;
    if (!printed)
    pr_info("Calibrating delay loop (skipped) "
    "preset value.. ");
    } else if ((!printed) && lpj_fine) {
    lpj = lpj_fine;
    pr_info("Calibrating delay loop (skipped), "
    "value calculated using timer frequency.. ");
    } else if ((lpj = calibrate_delay_is_known())) {
    ;
    } else if ((lpj = calibrate_delay_direct()) != 0) {
    if (!printed)
    pr_info("Calibrating delay using timer "
    "specific routine.. ");
    } else {
    if (!printed)
    pr_info("Calibrating delay loop... ");
    lpj = calibrate_delay_converge();
    }
    per_cpu(cpu_loops_per_jiffy, this_cpu) = lpj;
    if (!printed)
    pr_cont("%lu.%02lu BogoMIPS (lpj=%lu)\n",
    lpj/(500000/HZ),
    (lpj/(5000/HZ)) % 100, lpj);
    loops_per_jiffy = lpj;
    printed = true;
    calibration_delay_done();
    }
