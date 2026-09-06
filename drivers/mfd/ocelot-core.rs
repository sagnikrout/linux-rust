//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/ocelot-core.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Core driver for the Ocelot chip family.
//
// The VSC7511, 7512, 7513, and 7514 can be controlled internally via an
// on-chip MIPS processor, or externally via SPI, I2C, PCIe. This core driver is
// intended to be the bus-agnostic glue between, for example, the SPI bus and
// the child devices.
//
// Copyright 2021-2022 Innovative Advantage Inc.
//
// Author: Colin Foster <colin.foster@in-advantage.com>
//

pub const REG_GCB_SOFT_RST: c_uint = 0x0008;

pub const VSC7512_MIIM0_RES_START: c_uint = 0x7107009c;
pub const VSC7512_MIIM1_RES_START: c_uint = 0x710700c0;
pub const VSC7512_MIIM_RES_SIZE: c_uint = 0x00000024;
pub const VSC7512_PHY_RES_START: c_uint = 0x710700f0;
pub const VSC7512_PHY_RES_SIZE: c_uint = 0x00000004;
pub const VSC7512_GPIO_RES_START: c_uint = 0x71070034;
pub const VSC7512_GPIO_RES_SIZE: c_uint = 0x0000006c;
pub const VSC7512_SIO_CTRL_RES_START: c_uint = 0x710700f8;
pub const VSC7512_SIO_CTRL_RES_SIZE: c_uint = 0x00000100;
pub const VSC7512_HSIO_RES_START: c_uint = 0x710d0000;
pub const VSC7512_HSIO_RES_SIZE: c_uint = 0x00000128;
pub const VSC7512_ANA_RES_START: c_uint = 0x71880000;
pub const VSC7512_ANA_RES_SIZE: c_uint = 0x00010000;
pub const VSC7512_QS_RES_START: c_uint = 0x71080000;
pub const VSC7512_QS_RES_SIZE: c_uint = 0x00000100;
pub const VSC7512_QSYS_RES_START: c_uint = 0x71800000;
pub const VSC7512_QSYS_RES_SIZE: c_uint = 0x00200000;
pub const VSC7512_REW_RES_START: c_uint = 0x71030000;
pub const VSC7512_REW_RES_SIZE: c_uint = 0x00010000;
pub const VSC7512_SYS_RES_START: c_uint = 0x71010000;
pub const VSC7512_SYS_RES_SIZE: c_uint = 0x00010000;
pub const VSC7512_S0_RES_START: c_uint = 0x71040000;
pub const VSC7512_S1_RES_START: c_uint = 0x71050000;
pub const VSC7512_S2_RES_START: c_uint = 0x71060000;
pub const VCAP_RES_SIZE: c_uint = 0x00000400;
pub const VSC7512_PORT_0_RES_START: c_uint = 0x711e0000;
pub const VSC7512_PORT_1_RES_START: c_uint = 0x711f0000;
pub const VSC7512_PORT_2_RES_START: c_uint = 0x71200000;
pub const VSC7512_PORT_3_RES_START: c_uint = 0x71210000;
pub const VSC7512_PORT_4_RES_START: c_uint = 0x71220000;
pub const VSC7512_PORT_5_RES_START: c_uint = 0x71230000;
pub const VSC7512_PORT_6_RES_START: c_uint = 0x71240000;
pub const VSC7512_PORT_7_RES_START: c_uint = 0x71250000;
pub const VSC7512_PORT_8_RES_START: c_uint = 0x71260000;
pub const VSC7512_PORT_9_RES_START: c_uint = 0x71270000;
pub const VSC7512_PORT_10_RES_START: c_uint = 0x71280000;
pub const VSC7512_PORT_RES_SIZE: c_uint = 0x00010000;
pub const VSC7512_GCB_RST_SLEEP_US: c_int = 100;
pub const VSC7512_GCB_RST_TIMEOUT_US: c_int = 100000;
#[no_mangle]
unsafe extern "C" fn ocelot_gcb_chip_rst_status(ddata: *mut ocelot_ddata) -> c_int {
    static int ocelot_gcb_chip_rst_status(struct ocelot_ddata *ddata)
    {
    int val, err;
    err = regmap_read(ddata.gcb_regmap, REG_GCB_SOFT_RST, &val);
    if (err)
    return err;
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn ocelot_chip_reset(dev: *mut device) -> c_int {
    int ocelot_chip_reset(struct device *dev)
    {
    struct ocelot_ddata *ddata = dev_get_drvdata(dev);
    int ret, val;
//
// Reset the entire chip here to put it into a completely known state.
// Other drivers may want to reset their own subsystems. The register
// self-clears, so one write is all that is needed and wait for it to
// clear.
//
    ret = regmap_write(ddata.gcb_regmap, REG_GCB_SOFT_RST, BIT_SOFT_CHIP_RST);
    if (ret)
    return ret;
    return readx_poll_timeout(ocelot_gcb_chip_rst_status, ddata, val, !val,
    VSC7512_GCB_RST_SLEEP_US, VSC7512_GCB_RST_TIMEOUT_US);
    }
    EXPORT_SYMBOL_NS(ocelot_chip_reset, "MFD_OCELOT");
    static const struct resource vsc7512_miim0_resources[] = {
    DEFINE_RES_REG_NAMED(VSC7512_MIIM0_RES_START, VSC7512_MIIM_RES_SIZE, "gcb_miim0"),
    DEFINE_RES_REG_NAMED(VSC7512_PHY_RES_START, VSC7512_PHY_RES_SIZE, "gcb_phy"),
    };
    static const struct resource vsc7512_miim1_resources[] = {
    DEFINE_RES_REG_NAMED(VSC7512_MIIM1_RES_START, VSC7512_MIIM_RES_SIZE, "gcb_miim1"),
    };
    static const struct resource vsc7512_pinctrl_resources[] = {
    DEFINE_RES_REG_NAMED(VSC7512_GPIO_RES_START, VSC7512_GPIO_RES_SIZE, "gcb_gpio"),
    };
    static const struct resource vsc7512_sgpio_resources[] = {
    DEFINE_RES_REG_NAMED(VSC7512_SIO_CTRL_RES_START, VSC7512_SIO_CTRL_RES_SIZE, "gcb_sio"),
    };
    static const struct resource vsc7512_serdes_resources[] = {
    DEFINE_RES_REG_NAMED(VSC7512_HSIO_RES_START, VSC7512_HSIO_RES_SIZE, "hsio"),
    };
    static const struct resource vsc7512_switch_resources[] = {
    DEFINE_RES_REG_NAMED(VSC7512_ANA_RES_START, VSC7512_ANA_RES_SIZE, "ana"),
    DEFINE_RES_REG_NAMED(VSC7512_HSIO_RES_START, VSC7512_HSIO_RES_SIZE, "hsio"),
    DEFINE_RES_REG_NAMED(VSC7512_QS_RES_START, VSC7512_QS_RES_SIZE, "qs"),
    DEFINE_RES_REG_NAMED(VSC7512_QSYS_RES_START, VSC7512_QSYS_RES_SIZE, "qsys"),
    DEFINE_RES_REG_NAMED(VSC7512_REW_RES_START, VSC7512_REW_RES_SIZE, "rew"),
    DEFINE_RES_REG_NAMED(VSC7512_SYS_RES_START, VSC7512_SYS_RES_SIZE, "sys"),
    DEFINE_RES_REG_NAMED(VSC7512_S0_RES_START, VCAP_RES_SIZE, "s0"),
    DEFINE_RES_REG_NAMED(VSC7512_S1_RES_START, VCAP_RES_SIZE, "s1"),
    DEFINE_RES_REG_NAMED(VSC7512_S2_RES_START, VCAP_RES_SIZE, "s2"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_0_RES_START, VSC7512_PORT_RES_SIZE, "port0"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_1_RES_START, VSC7512_PORT_RES_SIZE, "port1"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_2_RES_START, VSC7512_PORT_RES_SIZE, "port2"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_3_RES_START, VSC7512_PORT_RES_SIZE, "port3"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_4_RES_START, VSC7512_PORT_RES_SIZE, "port4"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_5_RES_START, VSC7512_PORT_RES_SIZE, "port5"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_6_RES_START, VSC7512_PORT_RES_SIZE, "port6"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_7_RES_START, VSC7512_PORT_RES_SIZE, "port7"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_8_RES_START, VSC7512_PORT_RES_SIZE, "port8"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_9_RES_START, VSC7512_PORT_RES_SIZE, "port9"),
    DEFINE_RES_REG_NAMED(VSC7512_PORT_10_RES_START, VSC7512_PORT_RES_SIZE, "port10")
    };
    static const struct mfd_cell vsc7512_devs[] = {
    {
    .name = "ocelot-pinctrl",
    .of_compatible = "mscc,ocelot-pinctrl",
    .num_resources = ARRAY_SIZE(vsc7512_pinctrl_resources),
    .resources = vsc7512_pinctrl_resources,
    }, {
    .name = "ocelot-sgpio",
    .of_compatible = "mscc,ocelot-sgpio",
    .num_resources = ARRAY_SIZE(vsc7512_sgpio_resources),
    .resources = vsc7512_sgpio_resources,
    }, {
    .name = "ocelot-miim0",
    .of_compatible = "mscc,ocelot-miim",
    .of_reg = VSC7512_MIIM0_RES_START,
    .use_of_reg = true,
    .num_resources = ARRAY_SIZE(vsc7512_miim0_resources),
    .resources = vsc7512_miim0_resources,
    }, {
    .name = "ocelot-miim1",
    .of_compatible = "mscc,ocelot-miim",
    .of_reg = VSC7512_MIIM1_RES_START,
    .use_of_reg = true,
    .num_resources = ARRAY_SIZE(vsc7512_miim1_resources),
    .resources = vsc7512_miim1_resources,
    }, {
    .name = "ocelot-serdes",
    .of_compatible = "mscc,vsc7514-serdes",
    .num_resources = ARRAY_SIZE(vsc7512_serdes_resources),
    .resources = vsc7512_serdes_resources,
    }, {
    .name = "ocelot-ext-switch",
    .of_compatible = "mscc,vsc7512-switch",
    .num_resources = ARRAY_SIZE(vsc7512_switch_resources),
    .resources = vsc7512_switch_resources,
    },
    };
    static void ocelot_core_try_add_regmap(struct device *dev,
    const struct resource *res)
    {
    if (dev_get_regmap(dev, res.name))
    return;
    ocelot_spi_init_regmap(dev, res);
    }
    static void ocelot_core_try_add_regmaps(struct device *dev,
    const struct mfd_cell *cell)
    {
    int i;
    for (i = 0; i < cell.num_resources; i++)
    ocelot_core_try_add_regmap(dev, &cell.resources[i]);
    }
#[no_mangle]
pub unsafe extern "C" fn ocelot_core_init(dev: *mut device) -> c_int {
    int ocelot_core_init(struct device *dev)
    {
    int i, ndevs;
    ndevs = ARRAY_SIZE(vsc7512_devs);
    for (i = 0; i < ndevs; i++)
    ocelot_core_try_add_regmaps(dev, &vsc7512_devs[i]);
    return devm_mfd_add_devices(dev, PLATFORM_DEVID_AUTO, vsc7512_devs, ndevs, core::ptr::null_mut(), 0, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_NS(ocelot_core_init, "MFD_OCELOT");
    MODULE_DESCRIPTION("Externally Controlled Ocelot Chip Driver");
    MODULE_AUTHOR("Colin Foster <colin.foster@in-advantage.com>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("MFD_OCELOT_SPI");
