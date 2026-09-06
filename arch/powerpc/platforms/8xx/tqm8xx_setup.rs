//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/8xx/tqm8xx_setup.c
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
// Platform setup for the MPC8xx based boards from TQM.
//
// Heiko Schocher <hs@denx.de>
// Copyright 2010 DENX Software Engineering GmbH
//
// based on:
// Vitaly Bordug <vbordug@ru.mvista.com>
//
// Copyright 2005 MontaVista Software Inc.
//
// Heavily modified by Scott Wood <scottwood@freescale.com>
// Copyright 2007 Freescale Semiconductor, Inc.
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpm_pin {
    pub flags: int port, pin,,
}

    static struct cpm_pin tqm8xx_pins[] __initdata = {
// SMC1
    {CPM_PORTB, 24, CPM_PIN_INPUT}, /* RX */
    {CPM_PORTB, 25, CPM_PIN_INPUT | CPM_PIN_SECONDARY}, /* TX */
// SCC1
    {CPM_PORTA, 5, CPM_PIN_INPUT}, /* CLK1 */
    {CPM_PORTA, 7, CPM_PIN_INPUT}, /* CLK2 */
    {CPM_PORTA, 14, CPM_PIN_INPUT}, /* TX */
    {CPM_PORTA, 15, CPM_PIN_INPUT}, /* RX */
    {CPM_PORTC, 15, CPM_PIN_INPUT | CPM_PIN_SECONDARY}, /* TENA */
    {CPM_PORTC, 10, CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO},
    {CPM_PORTC, 11, CPM_PIN_INPUT | CPM_PIN_SECONDARY | CPM_PIN_GPIO},
    };
    static struct cpm_pin tqm8xx_fec_pins[] __initdata = {
// MII
    {CPM_PORTD, 3, CPM_PIN_OUTPUT},
    {CPM_PORTD, 4, CPM_PIN_OUTPUT},
    {CPM_PORTD, 5, CPM_PIN_OUTPUT},
    {CPM_PORTD, 6, CPM_PIN_OUTPUT},
    {CPM_PORTD, 7, CPM_PIN_OUTPUT},
    {CPM_PORTD, 8, CPM_PIN_OUTPUT},
    {CPM_PORTD, 9, CPM_PIN_OUTPUT},
    {CPM_PORTD, 10, CPM_PIN_OUTPUT},
    {CPM_PORTD, 11, CPM_PIN_OUTPUT},
    {CPM_PORTD, 12, CPM_PIN_OUTPUT},
    {CPM_PORTD, 13, CPM_PIN_OUTPUT},
    {CPM_PORTD, 14, CPM_PIN_OUTPUT},
    {CPM_PORTD, 15, CPM_PIN_OUTPUT},
    };
#[no_mangle]
unsafe extern "C" fn init_pins(n: c_int, pin: *mut cpm_pin) -> void __init {
    static void __init init_pins(int n, struct cpm_pin *pin)
    {
    int i;
    for (i = 0; i < n; i++) {
    cpm1_set_pin(pin.port, pin.pin, pin.flags);
    pin++;
    }
    }
#[no_mangle]
unsafe extern "C" fn init_ioports() -> void __init {
    static void __init init_ioports(void)
    {
    struct device_node *dnode;
    struct property *prop;
    int	len;
    init_pins(ARRAY_SIZE(tqm8xx_pins), &tqm8xx_pins[0]);
    cpm1_clk_setup(CPM_CLK_SMC1, CPM_BRG1, CPM_CLK_RTX);
    dnode = of_find_node_by_name(core::ptr::null_mut(), "aliases");
    if (dnode == core::ptr::null_mut())
    return;
    prop = of_find_property(dnode, "ethernet1", &len);
    of_node_put(dnode);
    if (prop == core::ptr::null_mut())
    return;
// init FEC pins
    init_pins(ARRAY_SIZE(tqm8xx_fec_pins), &tqm8xx_fec_pins[0]);
    }
#[no_mangle]
unsafe extern "C" fn tqm8xx_setup_arch() -> void __init {
    static void __init tqm8xx_setup_arch(void)
    {
    cpm_reset();
    init_ioports();
    }
    static const struct of_device_id of_bus_ids[] __initconst = {
    { .name = "soc", },
    { .name = "cpm", },
    { .name = "localbus", },
    { .compatible = "simple-bus" },
    {},
    };
#[no_mangle]
unsafe extern "C" fn declare_of_platform_devices() -> int __init {
    static int __init declare_of_platform_devices(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), of_bus_ids, core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(tqm8xx, declare_of_platform_devices);
    define_machine(tqm8xx) {
    .name			= "TQM8xx",
    .compatible		= "tqc,tqm8xx",
    .setup_arch		= tqm8xx_setup_arch,
    .init_IRQ		= mpc8xx_pic_init,
    .get_irq		= mpc8xx_get_irq,
    .restart		= mpc8xx_restart,
    .calibrate_decr		= mpc8xx_calibrate_decr,
    .set_rtc_time		= mpc8xx_set_rtc_time,
    .get_rtc_time		= mpc8xx_get_rtc_time,
    .progress		= udbg_progress,
    };
