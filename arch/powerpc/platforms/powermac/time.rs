//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/powermac/time.c
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
// Support for periodic interrupts (100 per second) and for getting
// the current time from the RTC on Power Macintoshes.
//
// We use the decrementer register for our periodic interrupts.
//
// Paul Mackerras	August 1996.
// Copyright (C) 1996 Paul Mackerras.
// Copyright (C) 2003-2005 Benjamin Herrenschmidt.
//

// Macro flag: #define DBG(x...)

//
// Calibrate the decrementer frequency with the VIA timer 1.
//

// VIA registers
pub const RS: c_uint = 0x200		/* skip between registers */;

// Bits in ACR
pub const T1MODE: c_uint = 0xc0		/* Timer 1 mode */;
pub const T1MODE_CONT: c_uint = 0x40		/*  continuous interrupts */;
// Bits in IFR and IER
pub const T1_INT: c_uint = 0x40		/* Timer 1 interrupt */;
#[no_mangle]
pub unsafe extern "C" fn pmac_time_init() -> long __init {
    long __init pmac_time_init(void)
    {
    let mut delta: i32 = 0;

    int dst;
    delta = ((s32)pmac_xpram_read(PMAC_XPRAM_MACHINE_LOC + 0x9)) << 16;
    delta |= ((s32)pmac_xpram_read(PMAC_XPRAM_MACHINE_LOC + 0xa)) << 8;
    delta |= pmac_xpram_read(PMAC_XPRAM_MACHINE_LOC + 0xb);
    if (delta & 0x00800000UL)
    delta |= 0xFF000000UL;
    dst = ((pmac_xpram_read(PMAC_XPRAM_MACHINE_LOC + 0x8) & 0x80) != 0);
    printk("GMT Delta read from XPRAM: %d minutes, DST: %s\n", delta/60,
    str_on_off(dst));

    return delta;
    }

#[no_mangle]
unsafe extern "C" fn smu_get_time() -> time64_t {
    static time64_t smu_get_time(void)
    {
    struct rtc_time tm;
    if (smu_get_rtc_time(&tm, 1))
    return 0;
    return rtc_tm_to_time64(&tm);
    }

// Can't be __init, it's called when suspending and resuming
#[no_mangle]
pub unsafe extern "C" fn pmac_get_boot_time() -> time64_t {
    time64_t pmac_get_boot_time(void)
    {
// Get the time from the RTC, used only at boot time
    switch (sys_ctrler) {

    case SYS_CTRLER_CUDA:
    return cuda_get_time();

    case SYS_CTRLER_PMU:
    return pmu_get_time();

    case SYS_CTRLER_SMU:
    return smu_get_time();

    default:
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_get_rtc_time(tm: *mut rtc_time) {
    void pmac_get_rtc_time(struct rtc_time *tm)
    {
// Get the time from the RTC, used only at boot time
    switch (sys_ctrler) {

    case SYS_CTRLER_CUDA:
    rtc_time64_to_tm(cuda_get_time(), tm);
    break;

    case SYS_CTRLER_PMU:
    rtc_time64_to_tm(pmu_get_time(), tm);
    break;

    case SYS_CTRLER_SMU:
    smu_get_rtc_time(tm, 1);
    break;

    default:
    ;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pmac_set_rtc_time(tm: *mut rtc_time) -> c_int {
    int pmac_set_rtc_time(struct rtc_time *tm)
    {
    switch (sys_ctrler) {

    case SYS_CTRLER_CUDA:
    return cuda_set_rtc_time(tm);

    case SYS_CTRLER_PMU:
    return pmu_set_rtc_time(tm);

    case SYS_CTRLER_SMU:
    return smu_set_rtc_time(tm, 1);

    default:
    return -ENODEV;
    }
    }

//
// Calibrate the decrementer register using VIA timer 1.
// This is used both on powermacs and CHRP machines.
//
#[no_mangle]
unsafe extern "C" fn via_calibrate_decr() -> int __init {
    static int __init via_calibrate_decr(void)
    {
    struct device_node *vias;
    volatile unsigned char __iomem *via;
    let mut count: c_int = VIA_TIMER_FREQ_6 / 100;
    unsigned int dstart, dend;
    struct resource rsrc;
    vias = of_find_node_by_name(core::ptr::null_mut(), "via-cuda");
    if (vias == core::ptr::null_mut())
    vias = of_find_node_by_name(core::ptr::null_mut(), "via-pmu");
    if (vias == core::ptr::null_mut())
    vias = of_find_node_by_name(core::ptr::null_mut(), "via");
    if (vias == core::ptr::null_mut() || of_address_to_resource(vias, 0, &rsrc)) {
    of_node_put(vias);
    return 0;
    }
    of_node_put(vias);
    via = early_ioremap(rsrc.start, resource_size(&rsrc));
    if (via == core::ptr::null_mut()) {
    printk(KERN_ERR "Failed to map VIA for timer calibration !\n");
    return 0;
    }
// set timer 1 for continuous interrupts
    out_8(&via[ACR], (via[ACR] & ~T1MODE) | T1MODE_CONT);
// set the counter to a small value
    out_8(&via[T1CH], 2);
// set the latch to `count'
    out_8(&via[T1LL], count);
    out_8(&via[T1LH], count >> 8);
// wait until it hits 0
    while ((in_8(&via[IFR]) & T1_INT) == 0)
    ;
    dstart = get_dec();
// clear the interrupt & wait until it hits 0 again
    in_8(&via[T1CL]);
    while ((in_8(&via[IFR]) & T1_INT) == 0)
    ;
    dend = get_dec();
    ppc_tb_freq = (dstart - dend) * 100 / 6;
    early_iounmap((void *)via, resource_size(&rsrc));
    return 1;
    }

//
// Query the OF and get the decr frequency.
//
#[no_mangle]
pub unsafe extern "C" fn pmac_calibrate_decr() -> void __init {
    void __init pmac_calibrate_decr(void)
    {
    generic_calibrate_decr();

// We assume MacRISC2 machines have correct device-tree
// calibration. That's better since the VIA itself seems
// to be slightly off. --BenH
//
    if (!of_machine_is_compatible("MacRISC2") &&
    !of_machine_is_compatible("MacRISC3") &&
    !of_machine_is_compatible("MacRISC4"))
    if (via_calibrate_decr())
    return;
// Special case: QuickSilver G4s seem to have a badly calibrated
// timebase-frequency in OF, VIA is much better on these. We should
// probably implement calibration based on the KL timer on these
// machines anyway... -BenH
//
    if (of_machine_is_compatible("PowerMac3,5"))
    if (via_calibrate_decr())
    return;

    }
