//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/qe/qe_io.c
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
// arch/powerpc/sysdev/qe_lib/qe_io.c
//
// QE Parallel I/O ports configuration routines
//
// Copyright 2006 Freescale Semiconductor, Inc. All rights reserved.
//
// Author: Li Yang <LeoLi@freescale.com>
// Based on code from Shlomi Gridish <gridish@freescale.com>
//

    static struct qe_pio_regs __iomem *par_io;
    let mut num_par_io_ports: static int = 0;
#[no_mangle]
pub unsafe extern "C" fn par_io_init(np: *mut device_node) -> c_int {
    int par_io_init(struct device_node *np)
    {
    struct resource res;
    int ret;
    u32 num_ports;
// Map Parallel I/O ports registers
    ret = of_address_to_resource(np, 0, &res);
    if (ret)
    return ret;
    par_io = ioremap(res.start, resource_size(&res));
    if (!par_io)
    return -ENOMEM;
    if (!of_property_read_u32(np, "num-ports", &num_ports))
    num_par_io_ports = num_ports;
    return 0;
    }
    void __par_io_config_pin(struct qe_pio_regs __iomem *par_io, u8 pin, int dir,
    int open_drain, int assignment, int has_irq)
    {
    u32 pin_mask1bit;
    u32 pin_mask2bits;
    u32 new_mask2bits;
    u32 tmp_val;
// calculate pin location for single and 2 bits information
    pin_mask1bit = (u32) (1 << (QE_PIO_PINS - (pin + 1)));
// Set open drain, if required
    tmp_val = ioread32be(&par_io.cpodr);
    if (open_drain)
    iowrite32be(pin_mask1bit | tmp_val, &par_io.cpodr);
    else
    iowrite32be(~pin_mask1bit & tmp_val, &par_io.cpodr);
// define direction
    tmp_val = (pin > (QE_PIO_PINS / 2) - 1) ?
    ioread32be(&par_io.cpdir2) :
    ioread32be(&par_io.cpdir1);
// get all bits mask for 2 bit per port
    pin_mask2bits = (u32) (0x3 << (QE_PIO_PINS -
    (pin % (QE_PIO_PINS / 2) + 1) * 2));
// Get the final mask we need for the right definition
    new_mask2bits = (u32) (dir << (QE_PIO_PINS -
    (pin % (QE_PIO_PINS / 2) + 1) * 2));
// clear and set 2 bits mask
    if (pin > (QE_PIO_PINS / 2) - 1) {
    iowrite32be(~pin_mask2bits & tmp_val, &par_io.cpdir2);
    tmp_val &= ~pin_mask2bits;
    iowrite32be(new_mask2bits | tmp_val, &par_io.cpdir2);
    } else {
    iowrite32be(~pin_mask2bits & tmp_val, &par_io.cpdir1);
    tmp_val &= ~pin_mask2bits;
    iowrite32be(new_mask2bits | tmp_val, &par_io.cpdir1);
    }
// define pin assignment
    tmp_val = (pin > (QE_PIO_PINS / 2) - 1) ?
    ioread32be(&par_io.cppar2) :
    ioread32be(&par_io.cppar1);
    new_mask2bits = (u32) (assignment << (QE_PIO_PINS -
    (pin % (QE_PIO_PINS / 2) + 1) * 2));
// clear and set 2 bits mask
    if (pin > (QE_PIO_PINS / 2) - 1) {
    iowrite32be(~pin_mask2bits & tmp_val, &par_io.cppar2);
    tmp_val &= ~pin_mask2bits;
    iowrite32be(new_mask2bits | tmp_val, &par_io.cppar2);
    } else {
    iowrite32be(~pin_mask2bits & tmp_val, &par_io.cppar1);
    tmp_val &= ~pin_mask2bits;
    iowrite32be(new_mask2bits | tmp_val, &par_io.cppar1);
    }
    }
    EXPORT_SYMBOL(__par_io_config_pin);
    int par_io_config_pin(u8 port, u8 pin, int dir, int open_drain,
    int assignment, int has_irq)
    {
    if (!par_io || port >= num_par_io_ports)
    return -EINVAL;
    __par_io_config_pin(&par_io[port], pin, dir, open_drain, assignment,
    has_irq);
    return 0;
    }
    EXPORT_SYMBOL(par_io_config_pin);
#[no_mangle]
pub unsafe extern "C" fn par_io_data_set(port: u8, pin: u8, val: u8) -> c_int {
    int par_io_data_set(u8 port, u8 pin, u8 val)
    {
    u32 pin_mask, tmp_val;
    if (port >= num_par_io_ports)
    return -EINVAL;
    if (pin >= QE_PIO_PINS)
    return -EINVAL;
// calculate pin location
    pin_mask = (u32) (1 << (QE_PIO_PINS - 1 - pin));
    tmp_val = ioread32be(&par_io[port].cpdata);
    if (val == 0)		/* clear */
    iowrite32be(~pin_mask & tmp_val, &par_io[port].cpdata);
    else			/* set */
    iowrite32be(pin_mask | tmp_val, &par_io[port].cpdata);
    return 0;
    }
    EXPORT_SYMBOL(par_io_data_set);
#[no_mangle]
pub unsafe extern "C" fn par_io_of_config(np: *mut device_node) -> c_int {
    int par_io_of_config(struct device_node *np)
    {
    struct device_node *pio;
    int pio_map_len;
    const __be32 *pio_map;
    if (par_io == core::ptr::null_mut()) {
    printk(KERN_ERR "par_io not initialized\n");
    return -1;
    }
    pio = of_parse_phandle(np, "pio-handle", 0);
    if (pio == core::ptr::null_mut()) {
    printk(KERN_ERR "pio-handle not available\n");
    return -1;
    }
    pio_map = of_get_property(pio, "pio-map", &pio_map_len);
    if (pio_map == core::ptr::null_mut()) {
    printk(KERN_ERR "pio-map is not set!\n");
    return -1;
    }
    pio_map_len /= sizeof(unsigned int);
    if ((pio_map_len % 6) != 0) {
    printk(KERN_ERR "pio-map format wrong!\n");
    return -1;
    }
    while (pio_map_len > 0) {
    let mut port: u8 = be32_to_cpu(pio_map[0]);
    let mut pin: u8 = be32_to_cpu(pio_map[1]);
    let mut dir: c_int = be32_to_cpu(pio_map[2]);
    let mut open_drain: c_int = be32_to_cpu(pio_map[3]);
    let mut assignment: c_int = be32_to_cpu(pio_map[4]);
    let mut has_irq: c_int = be32_to_cpu(pio_map[5]);
    par_io_config_pin(port, pin, dir, open_drain,
    assignment, has_irq);
    pio_map += 6;
    pio_map_len -= 6;
    }
    of_node_put(pio);
    return 0;
    }
    EXPORT_SYMBOL(par_io_of_config);
#[no_mangle]
unsafe extern "C" fn par_io_populate() -> int __init {
    static int __init par_io_populate(void)
    {
    struct device_node *np = of_find_node_by_type(core::ptr::null_mut(), "par_io");
    if (!np)
    return 0;
    of_platform_default_populate(np, core::ptr::null_mut(), core::ptr::null_mut());
    of_node_put(np);
    return 0;
    }
    arch_initcall(par_io_populate);
