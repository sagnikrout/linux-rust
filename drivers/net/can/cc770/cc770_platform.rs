//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/cc770/cc770_platform.c
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
// Driver for CC770 and AN82527 CAN controllers on the platform bus
//
// Copyright (C) 2009, 2011 Wolfgang Grandegger <wg@grandegger.com>
//
// If platform data are used you should have similar definitions
// in your board-specific code:
//
// static struct cc770_platform_data myboard_cc770_pdata = {
// .osc_freq = 16000000,
// .cir = 0x41,
// .cor = 0x20,
// .bcr = 0x40,
// };
//
// Please see include/linux/can/platform/cc770.h for description of
// above fields.
//
// If the device tree is used, you need a CAN node definition in your
// DTS file similar to:
//
// can@3,100 {
// compatible = "bosch,cc770";
// reg = <3 0x100 0x80>;
// interrupts = <2 0>;
// interrupt-parent = <&mpic>;
// bosch,external-clock-frequency = <16000000>;
// };
//
// See "Documentation/devicetree/bindings/net/can/cc770.txt" for further
// information.
//

    MODULE_AUTHOR("Wolfgang Grandegger <wg@grandegger.com>");
    MODULE_DESCRIPTION("Socket-CAN driver for CC770 on the platform bus");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRV_NAME);
pub const CC770_PLATFORM_CAN_CLOCK: c_int = 16000000;
#[no_mangle]
unsafe extern "C" fn cc770_platform_read_reg(priv: *const cc770_priv, reg: c_int) -> u8 {
    static u8 cc770_platform_read_reg(const struct cc770_priv *priv, int reg)
    {
    return ioread8(priv.reg_base + reg);
    }
    static void cc770_platform_write_reg(const struct cc770_priv *priv, int reg,
    u8 val)
    {
    iowrite8(val, priv.reg_base + reg);
    }
    static int cc770_get_of_node_data(struct platform_device *pdev,
    struct cc770_priv *priv)
    {
    let mut clkext: u32 = CC770_PLATFORM_CAN_CLOCK, clkout = 0;
    struct device_node *np = pdev.dev.of_node;
    of_property_read_u32(np, "bosch,external-clock-frequency", &clkext);
    priv.can.clock.freq = clkext;
// The system clock may not exceed 10 MHz
    if (priv.can.clock.freq > 10000000) {
    priv.cpu_interface |= CPUIF_DSC;
    priv.can.clock.freq /= 2;
    }
// The memory clock may not exceed 8 MHz
    if (priv.can.clock.freq > 8000000)
    priv.cpu_interface |= CPUIF_DMC;
    if (of_property_read_bool(np, "bosch,divide-memory-clock"))
    priv.cpu_interface |= CPUIF_DMC;
    if (of_property_read_bool(np, "bosch,iso-low-speed-mux"))
    priv.cpu_interface |= CPUIF_MUX;
    if (!of_property_read_bool(np, "bosch,no-comperator-bypass"))
    priv.bus_config |= BUSCFG_CBY;
    if (of_property_read_bool(np, "bosch,disconnect-rx0-input"))
    priv.bus_config |= BUSCFG_DR0;
    if (of_property_read_bool(np, "bosch,disconnect-rx1-input"))
    priv.bus_config |= BUSCFG_DR1;
    if (of_property_read_bool(np, "bosch,disconnect-tx1-output"))
    priv.bus_config |= BUSCFG_DT1;
    if (of_property_read_bool(np, "bosch,polarity-dominant"))
    priv.bus_config |= BUSCFG_POL;
    of_property_read_u32(np, "bosch,clock-out-frequency", &clkout);
    if (clkout > 0) {
    let mut cdv: u32 = clkext / clkout;
    if (cdv > 0 && cdv < 16) {
    u32 slew;
    priv.cpu_interface |= CPUIF_CEN;
    priv.clkout |= (cdv - 1) & CLKOUT_CD_MASK;
    if (of_property_read_u32(np, "bosch,slew-rate", &slew)) {
// Determine default slew rate
    slew = (CLKOUT_SL_MASK >>
    CLKOUT_SL_SHIFT) -
    ((cdv * clkext - 1) / 8000000);
    if (slew > (CLKOUT_SL_MASK >> CLKOUT_SL_SHIFT))
    slew = 0;
    }
    priv.clkout |= (slew << CLKOUT_SL_SHIFT) &
    CLKOUT_SL_MASK;
    } else {
    dev_dbg(&pdev.dev, "invalid clock-out-frequency\n");
    }
    }
    return 0;
    }
    static int cc770_get_platform_data(struct platform_device *pdev,
    struct cc770_priv *priv)
    {
    struct cc770_platform_data *pdata = dev_get_platdata(&pdev.dev);
    priv.can.clock.freq = pdata.osc_freq;
    if (priv.cpu_interface & CPUIF_DSC)
    priv.can.clock.freq /= 2;
    priv.clkout = pdata.cor;
    priv.bus_config = pdata.bcr;
    priv.cpu_interface = pdata.cir;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cc770_platform_probe(pdev: *mut platform_device) -> c_int {
    static int cc770_platform_probe(struct platform_device *pdev)
    {
    struct net_device *dev;
    struct cc770_priv *priv;
    struct resource *mem;
    resource_size_t mem_size;
    void __iomem *base;
    int err, irq;
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    irq = platform_get_irq(pdev, 0);
    if (!mem || irq <= 0)
    return -ENODEV;
    mem_size = resource_size(mem);
    if (!request_mem_region(mem.start, mem_size, pdev.name))
    return -EBUSY;
    base = ioremap(mem.start, mem_size);
    if (!base) {
    err = -ENOMEM;
    goto exit_release_mem;
    }
    dev = alloc_cc770dev(0);
    if (!dev) {
    err = -ENOMEM;
    goto exit_unmap_mem;
    }
    dev.irq = irq;
    priv = netdev_priv(dev);
    priv.read_reg = cc770_platform_read_reg;
    priv.write_reg = cc770_platform_write_reg;
    priv.irq_flags = IRQF_SHARED;
    priv.reg_base = base;
    if (pdev.dev.of_node)
    err = cc770_get_of_node_data(pdev, priv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: dev_get_platdata(&pdev->dev)) -> else {
    else if (dev_get_platdata(&pdev.dev))
    err = cc770_get_platform_data(pdev, priv);
    else
    err = -ENODEV;
    if (err)
    goto exit_free_cc770;
    dev_dbg(&pdev.dev,
    "reg_base=0x%p irq=%d clock=%d cpu_interface=0x%02x "
    "bus_config=0x%02x clkout=0x%02x\n",
    priv.reg_base, dev.irq, priv.can.clock.freq,
    priv.cpu_interface, priv.bus_config, priv.clkout);
    platform_set_drvdata(pdev, dev);
    SET_NETDEV_DEV(dev, &pdev.dev);
    err = register_cc770dev(dev);
    if (err) {
    dev_err(&pdev.dev,
    "couldn't register CC700 device (err=%d)\n", err);
    goto exit_free_cc770;
    }
    return 0;
    exit_free_cc770:
    free_cc770dev(dev);
    exit_unmap_mem:
    iounmap(base);
    exit_release_mem:
    release_mem_region(mem.start, mem_size);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cc770_platform_remove(pdev: *mut platform_device) {
    static void cc770_platform_remove(struct platform_device *pdev)
    {
    struct net_device *dev = platform_get_drvdata(pdev);
    struct cc770_priv *priv = netdev_priv(dev);
    struct resource *mem;
    unregister_cc770dev(dev);
    iounmap(priv.reg_base);
    free_cc770dev(dev);
    mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    release_mem_region(mem.start, resource_size(mem));
    }
    static const struct of_device_id cc770_platform_table[] = {
    {.compatible = "bosch,cc770"}, /* CC770 from Bosch */
    {.compatible = "intc,82527"},  /* AN82527 from Intel CP */
    {},
    };
    MODULE_DEVICE_TABLE(of, cc770_platform_table);
    static struct platform_driver cc770_platform_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = cc770_platform_table,
    },
    .probe = cc770_platform_probe,
    .remove = cc770_platform_remove,
    };
    module_platform_driver(cc770_platform_driver);
