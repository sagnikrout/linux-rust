//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/microwatt/setup.c
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


//
// Microwatt FPGA-based SoC platform setup code.
//
// Copyright 2020 Paul Mackerras (paulus@ozlabs.org), IBM Corp.
//

#[no_mangle]
unsafe extern "C" fn microwatt_init_IRQ() -> void __init {
    static void __init microwatt_init_IRQ(void)
    {
    xics_init();
    }
#[no_mangle]
unsafe extern "C" fn microwatt_populate() -> int __init {
    static int __init microwatt_populate(void)
    {
    return of_platform_default_populate(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    }
    machine_arch_initcall(microwatt, microwatt_populate);
#[no_mangle]
unsafe extern "C" fn microwatt_probe() -> int __init {
    static int __init microwatt_probe(void)
    {
// Main reason for having this is to start the other CPU(s)
    if (IS_ENABLED(CONFIG_SMP))
    microwatt_init_smp();
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn microwatt_setup_arch() -> void __init {
    static void __init microwatt_setup_arch(void)
    {
    microwatt_rng_init();
    }
#[no_mangle]
unsafe extern "C" fn microwatt_idle() {
    static void microwatt_idle(void)
    {
    if (!prep_irq_for_idle_irqsoff())
    return;
    __asm__ __volatile__ ("wait");
    }
    define_machine(microwatt) {
    .name			= "microwatt",
    .compatible		= "microwatt-soc",
    .probe			= microwatt_probe,
    .init_IRQ		= microwatt_init_IRQ,
    .setup_arch		= microwatt_setup_arch,
    .progress		= udbg_progress,
    .power_save		= microwatt_idle,
    };
