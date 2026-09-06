//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/i8253.c
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
// 8253/PIT functions
//

//
// HPET replaces the PIT, when enabled. So we need to know, which of
// the two timers is used
//
    struct clock_event_device *global_clock_event;
//
// Modern chipsets can disable the PIT clock which makes it unusable. It
// would be possible to enable the clock but the registers are chipset
// specific and not discoverable. Avoid the whack a mole game.
//
// These platforms have discoverable TSC/CPU frequencies but this also
// requires to know the local APIC timer frequency as it normally is
// calibrated against the PIT interrupt.
//
#[no_mangle]
unsafe extern "C" fn use_pit() -> bool __init {
    static bool __init use_pit(void)
    {
    if (!boot_cpu_has(X86_FEATURE_TSC))
    return true;
// This also returns true when APIC is disabled
    return apic_needs_pit();
    }
#[no_mangle]
pub unsafe extern "C" fn pit_timer_init() -> bool __init {
    bool __init pit_timer_init(void)
    {
    if (!use_pit()) {
//
// Don't just ignore the PIT. Ensure it's stopped, because
// VMMs otherwise steal CPU time just to pointlessly waggle
// the (masked) IRQ.
//
    scoped_guard(irq)
    clockevent_i8253_disable();
    return false;
    }
    clockevent_i8253_init(true);
    global_clock_event = &i8253_clockevent;
    return true;
    }

#[no_mangle]
unsafe extern "C" fn init_pit_clocksource() -> int __init {
    static int __init init_pit_clocksource(void)
    {
//
// Several reasons not to register PIT as a clocksource:
//
// - On SMP PIT does not scale due to i8253_lock
// - when HPET is enabled
// - when local APIC timer is active (PIT is switched off)
//
    if (num_possible_cpus() > 1 || is_hpet_enabled() ||
    !clockevent_state_periodic(&i8253_clockevent))
    return 0;
    return clocksource_i8253_init();
    }
    arch_initcall(init_pit_clocksource);
