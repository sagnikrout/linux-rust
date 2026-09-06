//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irqchip.c
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
// Copyright (C) 2012 Thomas Petazzoni
//
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

//
// This special of_device_id is the sentinel at the end of the
// of_device_id[] array of all irqchips. It is automatically placed at
// the end of the array by the linker, thanks to being part of a
// special section.
//
    static const struct of_device_id
    irqchip_of_match_end __used __section("__irqchip_of_table_end");
    extern struct of_device_id __irqchip_of_table[];
#[no_mangle]
pub unsafe extern "C" fn irqchip_init() -> void __init {
    void __init irqchip_init(void)
    {
    of_irq_init(__irqchip_of_table);
    acpi_probe_device_table(irqchip);
    }
#[no_mangle]
pub unsafe extern "C" fn platform_irqchip_probe(pdev: *mut platform_device) -> c_int {
    int platform_irqchip_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device_node *par_np __free(device_node) = of_irq_find_parent(np);
    let mut irq_probe: platform_irq_probe_t = of_device_get_match_data(&pdev.dev);
    if (!irq_probe)
    return -EINVAL;
    if (par_np == np)
    par_np = core::ptr::null_mut();
//
// If there's a parent interrupt controller and  none of the parent irq
// domains have been registered, that means the parent interrupt
// controller has not been initialized yet.  it's not time for this
// interrupt controller to initialize. So, defer probe of this
// interrupt controller. The actual initialization callback of this
// interrupt controller can check for specific domains as necessary.
//
    if (par_np && !irq_find_matching_host(par_np, DOMAIN_BUS_ANY))
    return -EPROBE_DEFER;
    return irq_probe(pdev, par_np);
    }
    EXPORT_SYMBOL_GPL(platform_irqchip_probe);
