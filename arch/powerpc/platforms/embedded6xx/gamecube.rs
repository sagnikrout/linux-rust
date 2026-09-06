//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/embedded6xx/gamecube.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// arch/powerpc/platforms/embedded6xx/gamecube.c
//
// Nintendo GameCube board-specific support
// Copyright (C) 2004-2009 The GameCube Linux Team
// Copyright (C) 2007,2008,2009 Albert Herranz
//

#[no_mangle]
unsafe extern "C" fn gamecube_spin() -> void __noreturn {
    static void __noreturn gamecube_spin(void)
    {
// spin until power button pressed
    for (;;)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn gamecube_restart(cmd: *mut c_char) -> void __noreturn {
    static void __noreturn gamecube_restart(char *cmd)
    {
    local_irq_disable();
    flipper_platform_reset();
    gamecube_spin();
    }
#[no_mangle]
unsafe extern "C" fn gamecube_power_off() {
    static void gamecube_power_off(void)
    {
    local_irq_disable();
    gamecube_spin();
    }
#[no_mangle]
unsafe extern "C" fn gamecube_halt() -> void __noreturn {
    static void __noreturn gamecube_halt(void)
    {
    gamecube_restart(core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn gamecube_probe() -> int __init {
    static int __init gamecube_probe(void)
    {
    pm_power_off = gamecube_power_off;
    ug_udbg_init();
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn gamecube_shutdown() {
    static void gamecube_shutdown(void)
    {
    flipper_quiesce();
    }
    define_machine(gamecube) {
    .name			= "gamecube",
    .compatible		= "nintendo,gamecube",
    .probe			= gamecube_probe,
    .restart		= gamecube_restart,
    .halt			= gamecube_halt,
    .init_IRQ		= flipper_pic_probe,
    .get_irq		= flipper_pic_get_irq,
    .progress		= udbg_progress,
    .machine_shutdown	= gamecube_shutdown,
    };
    static const struct of_device_id gamecube_of_bus[] = {
    { .compatible = "nintendo,flipper", },
    { },
    };
#[no_mangle]
unsafe extern "C" fn gamecube_device_probe() -> int __init {
    static int __init gamecube_device_probe(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), gamecube_of_bus, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(gamecube, gamecube_device_probe);
