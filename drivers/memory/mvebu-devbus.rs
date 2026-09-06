//! Automatically rewritten from C to Rust
//! Source: drivers/memory/mvebu-devbus.c
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
// Marvell EBU SoC Device Bus Controller
// (memory controller for NOR/NAND/SRAM/FPGA devices)
//
// Copyright (C) 2013-2014 Marvell
//

// Register definitions
pub const ARMADA_DEV_WIDTH_SHIFT: c_int = 30;
pub const ARMADA_BADR_SKEW_SHIFT: c_int = 28;
pub const ARMADA_RD_HOLD_SHIFT: c_int = 23;
pub const ARMADA_ACC_NEXT_SHIFT: c_int = 17;
pub const ARMADA_RD_SETUP_SHIFT: c_int = 12;
pub const ARMADA_ACC_FIRST_SHIFT: c_int = 6;
pub const ARMADA_SYNC_ENABLE_SHIFT: c_int = 24;
pub const ARMADA_WR_HIGH_SHIFT: c_int = 16;
pub const ARMADA_WR_LOW_SHIFT: c_int = 8;
pub const ARMADA_READ_PARAM_OFFSET: c_uint = 0x0;
pub const ARMADA_WRITE_PARAM_OFFSET: c_uint = 0x4;

pub const ORION_BADR_SKEW_SHIFT: c_int = 28;

pub const ORION_WR_HIGH_EXT_MASK: c_uint = 0x8;

pub const ORION_WR_LOW_EXT_MASK: c_uint = 0x8;

pub const ORION_ALE_WR_EXT_MASK: c_uint = 0x8;

pub const ORION_ACC_NEXT_EXT_MASK: c_uint = 0x10;

pub const ORION_ACC_FIRST_EXT_MASK: c_uint = 0x10;

pub const ORION_TURN_OFF_EXT_MASK: c_uint = 0x8;
pub const ORION_DEV_WIDTH_SHIFT: c_int = 20;
pub const ORION_WR_HIGH_SHIFT: c_int = 17;
pub const ORION_WR_HIGH_MASK: c_uint = 0x7;
pub const ORION_WR_LOW_SHIFT: c_int = 14;
pub const ORION_WR_LOW_MASK: c_uint = 0x7;
pub const ORION_ALE_WR_SHIFT: c_int = 11;
pub const ORION_ALE_WR_MASK: c_uint = 0x7;
pub const ORION_ACC_NEXT_SHIFT: c_int = 7;
pub const ORION_ACC_NEXT_MASK: c_uint = 0xF;
pub const ORION_ACC_FIRST_SHIFT: c_int = 3;
pub const ORION_ACC_FIRST_MASK: c_uint = 0xF;
pub const ORION_TURN_OFF_SHIFT: c_int = 0;
pub const ORION_TURN_OFF_MASK: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devbus_read_params {
    pub bus_width: u32,
    pub badr_skew: u32,
    pub turn_off: u32,
    pub acc_first: u32,
    pub acc_next: u32,
    pub rd_setup: u32,
    pub rd_hold: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devbus_write_params {
    pub sync_enable: u32,
    pub wr_high: u32,
    pub wr_low: u32,
    pub ale_wr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devbus {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub tick_ps: c_ulong,
}

    static int get_timing_param_ps(struct devbus *devbus,
    struct device_node *node,
    const char *name,
    u32 *ticks)
    {
    u32 time_ps;
    int err;
    err = of_property_read_u32(node, name, &time_ps);
    if (err < 0) {
    dev_err(devbus.dev, "%pOF has no '%s' property\n",
    node, name);
    return err;
    }
// ticks = (time_ps + devbus->tick_ps - 1) / devbus->tick_ps;
    dev_dbg(devbus.dev, "%s: %u ps . 0x%x\n",
    name, time_ps, *ticks);
    return 0;
    }
    static int devbus_get_timing_params(struct devbus *devbus,
    struct device_node *node,
    struct devbus_read_params *r,
    struct devbus_write_params *w)
    {
    int err;
    err = of_property_read_u32(node, "devbus,bus-width", &r.bus_width);
    if (err < 0) {
    dev_err(devbus.dev,
    "%pOF has no 'devbus,bus-width' property\n",
    node);
    return err;
    }
//
// The bus width is encoded into the register as 0 for 8 bits,
// and 1 for 16 bits, so we do the necessary conversion here.
//
    if (r.bus_width == 8) {
    r.bus_width = 0;
    } else if (r.bus_width == 16) {
    r.bus_width = 1;
    } else {
    dev_err(devbus.dev, "invalid bus width %d\n", r.bus_width);
    return -EINVAL;
    }
    err = get_timing_param_ps(devbus, node, "devbus,badr-skew-ps",
    &r.badr_skew);
    if (err < 0)
    return err;
    err = get_timing_param_ps(devbus, node, "devbus,turn-off-ps",
    &r.turn_off);
    if (err < 0)
    return err;
    err = get_timing_param_ps(devbus, node, "devbus,acc-first-ps",
    &r.acc_first);
    if (err < 0)
    return err;
    err = get_timing_param_ps(devbus, node, "devbus,acc-next-ps",
    &r.acc_next);
    if (err < 0)
    return err;
    if (of_device_is_compatible(devbus.dev.of_node, "marvell,mvebu-devbus")) {
    err = get_timing_param_ps(devbus, node, "devbus,rd-setup-ps",
    &r.rd_setup);
    if (err < 0)
    return err;
    err = get_timing_param_ps(devbus, node, "devbus,rd-hold-ps",
    &r.rd_hold);
    if (err < 0)
    return err;
    err = of_property_read_u32(node, "devbus,sync-enable",
    &w.sync_enable);
    if (err < 0) {
    dev_err(devbus.dev,
    "%pOF has no 'devbus,sync-enable' property\n",
    node);
    return err;
    }
    }
    err = get_timing_param_ps(devbus, node, "devbus,ale-wr-ps",
    &w.ale_wr);
    if (err < 0)
    return err;
    err = get_timing_param_ps(devbus, node, "devbus,wr-low-ps",
    &w.wr_low);
    if (err < 0)
    return err;
    err = get_timing_param_ps(devbus, node, "devbus,wr-high-ps",
    &w.wr_high);
    if (err < 0)
    return err;
    return 0;
    }
    static void devbus_orion_set_timing_params(struct devbus *devbus,
    struct device_node *node,
    struct devbus_read_params *r,
    struct devbus_write_params *w)
    {
    u32 value;
//
// The hardware designers found it would be a good idea to
// split most of the values in the register into two fields:
// one containing all the low-order bits, and another one
// containing just the high-order bit. For all of those
// fields, we have to split the value into these two parts.
//
    value =	(r.turn_off   & ORION_TURN_OFF_MASK)  << ORION_TURN_OFF_SHIFT  |
    (r.acc_first  & ORION_ACC_FIRST_MASK) << ORION_ACC_FIRST_SHIFT |
    (r.acc_next   & ORION_ACC_NEXT_MASK)  << ORION_ACC_NEXT_SHIFT  |
    (w.ale_wr     & ORION_ALE_WR_MASK)    << ORION_ALE_WR_SHIFT    |
    (w.wr_low     & ORION_WR_LOW_MASK)    << ORION_WR_LOW_SHIFT    |
    (w.wr_high    & ORION_WR_HIGH_MASK)   << ORION_WR_HIGH_SHIFT   |
    r.bus_width                           << ORION_DEV_WIDTH_SHIFT |
    ((r.turn_off  & ORION_TURN_OFF_EXT_MASK)  ? ORION_TURN_OFF_EXT_BIT  : 0) |
    ((r.acc_first & ORION_ACC_FIRST_EXT_MASK) ? ORION_ACC_FIRST_EXT_BIT : 0) |
    ((r.acc_next  & ORION_ACC_NEXT_EXT_MASK)  ? ORION_ACC_NEXT_EXT_BIT  : 0) |
    ((w.ale_wr    & ORION_ALE_WR_EXT_MASK)    ? ORION_ALE_WR_EXT_BIT    : 0) |
    ((w.wr_low    & ORION_WR_LOW_EXT_MASK)    ? ORION_WR_LOW_EXT_BIT    : 0) |
    ((w.wr_high   & ORION_WR_HIGH_EXT_MASK)   ? ORION_WR_HIGH_EXT_BIT   : 0) |
    (r.badr_skew << ORION_BADR_SKEW_SHIFT) |
    ORION_RESERVED;
    writel(value, devbus.base);
    }
    static void devbus_armada_set_timing_params(struct devbus *devbus,
    struct device_node *node,
    struct devbus_read_params *r,
    struct devbus_write_params *w)
    {
    u32 value;
// Set read timings
    value = r.bus_width << ARMADA_DEV_WIDTH_SHIFT |
    r.badr_skew << ARMADA_BADR_SKEW_SHIFT |
    r.rd_hold   << ARMADA_RD_HOLD_SHIFT   |
    r.acc_next  << ARMADA_ACC_NEXT_SHIFT  |
    r.rd_setup  << ARMADA_RD_SETUP_SHIFT  |
    r.acc_first << ARMADA_ACC_FIRST_SHIFT |
    r.turn_off;
    dev_dbg(devbus.dev, "read parameters register 0x%p = 0x%x\n",
    devbus.base + ARMADA_READ_PARAM_OFFSET,
    value);
    writel(value, devbus.base + ARMADA_READ_PARAM_OFFSET);
// Set write timings
    value = w.sync_enable  << ARMADA_SYNC_ENABLE_SHIFT |
    w.wr_low       << ARMADA_WR_LOW_SHIFT      |
    w.wr_high      << ARMADA_WR_HIGH_SHIFT     |
    w.ale_wr;
    dev_dbg(devbus.dev, "write parameters register: 0x%p = 0x%x\n",
    devbus.base + ARMADA_WRITE_PARAM_OFFSET,
    value);
    writel(value, devbus.base + ARMADA_WRITE_PARAM_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn mvebu_devbus_probe(pdev: *mut platform_device) -> c_int {
    static int mvebu_devbus_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = pdev.dev.of_node;
    struct devbus_read_params r;
    struct devbus_write_params w;
    struct devbus *devbus;
    struct clk *clk;
    unsigned long rate;
    int err;
    devbus = devm_kzalloc(&pdev.dev, sizeof(struct devbus), GFP_KERNEL);
    if (!devbus)
    return -ENOMEM;
    devbus.dev = dev;
    devbus.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(devbus.base))
    return PTR_ERR(devbus.base);
    clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return PTR_ERR(clk);
//
// Obtain clock period in picoseconds,
// we need this in order to convert timing
// parameters from cycles to picoseconds.
//
    rate = clk_get_rate(clk) / 1000;
    devbus.tick_ps = 1000000000 / rate;
    dev_dbg(devbus.dev, "Setting timing parameter, tick is %lu ps\n",
    devbus.tick_ps);
    if (!of_property_read_bool(node, "devbus,keep-config")) {
// Read the Device Tree node
    err = devbus_get_timing_params(devbus, node, &r, &w);
    if (err < 0)
    return err;
// Set the new timing parameters
    if (of_device_is_compatible(node, "marvell,orion-devbus"))
    devbus_orion_set_timing_params(devbus, node, &r, &w);
    else
    devbus_armada_set_timing_params(devbus, node, &r, &w);
    }
//
// We need to create a child device explicitly from here to
// guarantee that the child will be probed after the timing
// parameters for the bus are written.
//
    err = of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    if (err < 0)
    return err;
    return 0;
    }
    static const struct of_device_id mvebu_devbus_of_match[] = {
    { .compatible = "marvell,mvebu-devbus" },
    { .compatible = "marvell,orion-devbus" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mvebu_devbus_of_match);
    static struct platform_driver mvebu_devbus_driver = {
    .probe		= mvebu_devbus_probe,
    .driver		= {
    .name	= "mvebu-devbus",
    .of_match_table = mvebu_devbus_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn mvebu_devbus_init() -> int __init {
    static int __init mvebu_devbus_init(void)
    {
    return platform_driver_register(&mvebu_devbus_driver);
    }
    module_init(mvebu_devbus_init);
    MODULE_AUTHOR("Ezequiel Garcia <ezequiel.garcia@free-electrons.com>");
    MODULE_DESCRIPTION("Marvell EBU SoC Device Bus controller");
