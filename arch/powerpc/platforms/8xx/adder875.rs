//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/8xx/adder875.c
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
// Analogue & Micro Adder MPC875 board support
//
// Author: Scott Wood <scottwood@freescale.com>
//
// Copyright (c) 2007 Freescale Semiconductor, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_pin {
    pub flags: int port, pin,,
}

    static __initdata struct cpm_pin adder875_pins[] = {
// SMC1
    {CPM_PORTB, 24, CPM_PIN_INPUT}, /* RX */
    {CPM_PORTB, 25, CPM_PIN_INPUT | CPM_PIN_SECONDARY}, /* TX */
// MII1
    {CPM_PORTA, 0, CPM_PIN_INPUT},
    {CPM_PORTA, 1, CPM_PIN_INPUT},
    {CPM_PORTA, 2, CPM_PIN_INPUT},
    {CPM_PORTA, 3, CPM_PIN_INPUT},
    {CPM_PORTA, 4, CPM_PIN_OUTPUT},
    {CPM_PORTA, 10, CPM_PIN_OUTPUT},
    {CPM_PORTA, 11, CPM_PIN_OUTPUT},
    {CPM_PORTB, 19, CPM_PIN_INPUT},
    {CPM_PORTB, 31, CPM_PIN_INPUT},
    {CPM_PORTC, 12, CPM_PIN_INPUT},
    {CPM_PORTC, 13, CPM_PIN_INPUT},
    {CPM_PORTE, 30, CPM_PIN_OUTPUT},
    {CPM_PORTE, 31, CPM_PIN_OUTPUT},
// MII2
    {CPM_PORTE, 14, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 15, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 16, CPM_PIN_OUTPUT},
    {CPM_PORTE, 17, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 18, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 19, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 20, CPM_PIN_OUTPUT | CPM_PIN_SECONDARY},
    {CPM_PORTE, 21, CPM_PIN_OUTPUT},
    {CPM_PORTE, 22, CPM_PIN_OUTPUT},
    {CPM_PORTE, 23, CPM_PIN_OUTPUT},
    {CPM_PORTE, 24, CPM_PIN_OUTPUT},
    {CPM_PORTE, 25, CPM_PIN_OUTPUT},
    {CPM_PORTE, 26, CPM_PIN_OUTPUT},
    {CPM_PORTE, 27, CPM_PIN_OUTPUT},
    {CPM_PORTE, 28, CPM_PIN_OUTPUT},
    {CPM_PORTE, 29, CPM_PIN_OUTPUT},
    };
#[no_mangle]
unsafe extern "C" fn init_ioports() -> void __init {
    static void __init init_ioports(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(adder875_pins); i++) {
    const struct cpm_pin *pin = &adder875_pins[i];
    cpm1_set_pin(pin.port, pin.pin, pin.flags);
    }
    cpm1_clk_setup(CPM_CLK_SMC1, CPM_BRG1, CPM_CLK_RTX);
// Set FEC1 and FEC2 to MII mode
    clrbits32(&mpc8xx_immr.im_cpm.cp_cptr, 0x00000180);
    }
#[no_mangle]
unsafe extern "C" fn adder875_setup() -> void __init {
    static void __init adder875_setup(void)
    {
    cpm_reset();
    init_ioports();
    }
    static const struct of_device_id of_bus_ids[] __initconst = {
    { .compatible = "simple-bus", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn declare_of_platform_devices() -> int __init {
    static int __init declare_of_platform_devices(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), of_bus_ids, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(adder875, declare_of_platform_devices);
    define_machine(adder875) {
    .name = "Adder MPC875",
    .compatible = "analogue-and-micro,adder875",
    .setup_arch = adder875_setup,
    .init_IRQ = mpc8xx_pic_init,
    .get_irq = mpc8xx_get_irq,
    .restart = mpc8xx_restart,
    .progress = udbg_progress,
    };
