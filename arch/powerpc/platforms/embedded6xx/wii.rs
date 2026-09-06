//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/embedded6xx/wii.c
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
// arch/powerpc/platforms/embedded6xx/wii.c
//
// Nintendo Wii board-specific support
// Copyright (C) 2008-2009 The GameCube Linux Team
// Copyright (C) 2008,2009 Albert Herranz
//

// control block

pub const HW_CTRL_RESETS: c_uint = 0x94;

// gpio

    static void __iomem *hw_ctrl;
    static void __iomem *hw_gpio;
#[no_mangle]
unsafe extern "C" fn wii_spin() -> void __noreturn {
    static void __noreturn wii_spin(void)
    {
    local_irq_disable();
    for (;;)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn wii_ioremap_hw_regs(name: *mut c_char, compatible: *mut c_char) -> *mut void __iomem __init {
    static void __iomem *__init wii_ioremap_hw_regs(char *name, char *compatible)
    {
    void __iomem *hw_regs = core::ptr::null_mut();
    struct device_node *np;
    struct resource res;
    let mut error: c_int = -ENODEV;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), compatible);
    if (!np) {
    pr_err("no compatible node found for %s\n", compatible);
    goto out;
    }
    error = of_address_to_resource(np, 0, &res);
    if (error) {
    pr_err("no valid reg found for %pOFn\n", np);
    goto out_put;
    }
    hw_regs = ioremap(res.start, resource_size(&res));
    if (hw_regs) {
    pr_info("%s at 0x%pa mapped to 0x%p\n", name,
    &res.start, hw_regs);
    }
    out_put:
    of_node_put(np);
    out:
    return hw_regs;
    }
#[no_mangle]
unsafe extern "C" fn wii_setup_arch() -> void __init {
    static void __init wii_setup_arch(void)
    {
    hw_ctrl = wii_ioremap_hw_regs("hw_ctrl", HW_CTRL_COMPATIBLE);
    hw_gpio = wii_ioremap_hw_regs("hw_gpio", HW_GPIO_COMPATIBLE);
    if (hw_gpio) {
// turn off the front blue led and IR light
    clrbits32(hw_gpio + HW_GPIO_OUT(0),
    HW_GPIO_SLOT_LED | HW_GPIO_SENSOR_BAR);
    }
    }
#[no_mangle]
unsafe extern "C" fn wii_restart(cmd: *mut c_char) -> void __noreturn {
    static void __noreturn wii_restart(char *cmd)
    {
    local_irq_disable();
    if (hw_ctrl) {
// clear the system reset pin to cause a reset
    clrbits32(hw_ctrl + HW_CTRL_RESETS, HW_CTRL_RESETS_SYS);
    }
    wii_spin();
    }
#[no_mangle]
unsafe extern "C" fn wii_power_off() {
    static void wii_power_off(void)
    {
    local_irq_disable();
    if (hw_gpio) {
//
// set the owner of the shutdown pin to ARM, because it is
// accessed through the registers for the ARM, below
//
    clrbits32(hw_gpio + HW_GPIO_OWNER, HW_GPIO_SHUTDOWN);
// make sure that the poweroff GPIO is configured as output
    setbits32(hw_gpio + HW_GPIO_DIR(1), HW_GPIO_SHUTDOWN);
// drive the poweroff GPIO high
    setbits32(hw_gpio + HW_GPIO_OUT(1), HW_GPIO_SHUTDOWN);
    }
    wii_spin();
    }
#[no_mangle]
unsafe extern "C" fn wii_halt() -> void __noreturn {
    static void __noreturn wii_halt(void)
    {
    if (ppc_md.restart)
    ppc_md.restart(core::ptr::null_mut());
    wii_spin();
    }
#[no_mangle]
unsafe extern "C" fn wii_pic_probe() -> void __init {
    static void __init wii_pic_probe(void)
    {
    flipper_pic_probe();
    hlwd_pic_probe();
    }
#[no_mangle]
unsafe extern "C" fn wii_probe() -> int __init {
    static int __init wii_probe(void)
    {
    pm_power_off = wii_power_off;
    ug_udbg_init();
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn wii_shutdown() {
    static void wii_shutdown(void)
    {
    hlwd_quiesce();
    flipper_quiesce();
    }
    static const struct of_device_id wii_of_bus[] = {
    { .compatible = "nintendo,hollywood", },
    { },
    };
#[no_mangle]
unsafe extern "C" fn wii_device_probe() -> int __init {
    static int __init wii_device_probe(void)
    {
    of_platform_populate(core::ptr::null_mut(), wii_of_bus, core::ptr::null_mut(), core::ptr::null_mut());
    return 0;
    }
    machine_device_initcall(wii, wii_device_probe);
    define_machine(wii) {
    .name			= "wii",
    .compatible		= "nintendo,wii",
    .probe			= wii_probe,
    .setup_arch		= wii_setup_arch,
    .restart		= wii_restart,
    .halt			= wii_halt,
    .init_IRQ		= wii_pic_probe,
    .get_irq		= flipper_pic_get_irq,
    .progress		= udbg_progress,
    .machine_shutdown	= wii_shutdown,
    };
