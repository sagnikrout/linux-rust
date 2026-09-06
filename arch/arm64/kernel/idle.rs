//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/idle.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level idle sequences
//

    enum {
    ARM64_IDLE_WFI,
    ARM64_IDLE_YIELD,
    ARM64_IDLE_NOP,
    } idle = ARM64_IDLE_WFI;
#[no_mangle]
unsafe extern "C" fn setup_idle(arg: *mut c_char) -> int __init {
    static int __init setup_idle(char *arg)
    {
    if (!arg)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, _arg: "wfi")) -> else {
    else if (!strcmp(arg, "wfi"))
    idle = ARM64_IDLE_WFI;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, _arg: "yield")) -> else {
    else if (!strcmp(arg, "yield"))
    idle = ARM64_IDLE_YIELD;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, _arg: "nop")) -> else {
    else if (!strcmp(arg, "nop"))
    idle = ARM64_IDLE_NOP;
    else
    return -1;
    return 0;
    }
    early_param("idle", setup_idle);
//
// cpu_do_idle()
//
// Idle the processor (wait for interrupt).
//
// If the CPU supports priority masking we must do additional work to
// ensure that interrupts are not masked at the PMR (because the core will
// not wake up if we block the wake up signal in the interrupt controller).
//
#[no_mangle]
pub unsafe extern "C" fn cpu_do_idle() -> void __cpuidle {
    void __cpuidle cpu_do_idle(void)
    {
    struct arm_cpuidle_irq_context context;
    arm_cpuidle_save_irq_context(&context);
    if (likely(idle == ARM64_IDLE_WFI)) {
    dsb(sy);
    wfi();
    } else if (idle == ARM64_IDLE_YIELD) {
    dsb(sy);
    asm volatile("yield" ::: "memory");
    }
    arm_cpuidle_restore_irq_context(&context);
    }
//
// This is our default idle handler.
//
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle() -> void __cpuidle {
    void __cpuidle arch_cpu_idle(void)
    {
//
// This should do all the clock switching and wait for interrupt
// tricks
//
    cpu_do_idle();
    }
