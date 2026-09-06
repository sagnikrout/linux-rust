//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/qcom-pm8008.c
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
// Copyright (c) 2021, The Linux Foundation. All rights reserved.
//

pub const I2C_INTR_STATUS_BASE: c_uint = 0x0550;
pub const INT_RT_STS_OFFSET: c_uint = 0x10;
pub const INT_SET_TYPE_OFFSET: c_uint = 0x11;
pub const INT_POL_HIGH_OFFSET: c_uint = 0x12;
pub const INT_POL_LOW_OFFSET: c_uint = 0x13;
pub const INT_LATCHED_CLR_OFFSET: c_uint = 0x14;
pub const INT_EN_SET_OFFSET: c_uint = 0x15;
pub const INT_EN_CLR_OFFSET: c_uint = 0x16;
pub const INT_LATCHED_STS_OFFSET: c_uint = 0x18;
    enum {
    PM8008_MISC,
    PM8008_TEMP_ALARM,
    PM8008_GPIO1,
    PM8008_GPIO2,
    PM8008_NUM_PERIPHS,
    };
pub const PM8008_PERIPH_0_BASE: c_uint = 0x900;
pub const PM8008_PERIPH_1_BASE: c_uint = 0x2400;
pub const PM8008_PERIPH_2_BASE: c_uint = 0xc000;
pub const PM8008_PERIPH_3_BASE: c_uint = 0xc100;

// PM8008 IRQ numbers
pub const PM8008_IRQ_MISC_UVLO: c_int = 0;
pub const PM8008_IRQ_MISC_OVLO: c_int = 1;
pub const PM8008_IRQ_MISC_OTST2: c_int = 2;
pub const PM8008_IRQ_MISC_OTST3: c_int = 3;
pub const PM8008_IRQ_MISC_LDO_OCP: c_int = 4;
pub const PM8008_IRQ_TEMP_ALARM: c_int = 5;
pub const PM8008_IRQ_GPIO1: c_int = 6;
pub const PM8008_IRQ_GPIO2: c_int = 7;
    enum {
    SET_TYPE_INDEX,
    POLARITY_HI_INDEX,
    POLARITY_LO_INDEX,
    };
    static const unsigned int pm8008_config_regs[] = {
    INT_SET_TYPE_OFFSET,
    INT_POL_HIGH_OFFSET,
    INT_POL_LOW_OFFSET,
    };

    [_irq] = {					\
    .reg_offset = (_off),			\
    .mask = (_mask),			\
    .type = {				\
    .type_reg_offset = (_off),	\
    .types_supported = (_types),	\
    },					\
    }
    static const struct regmap_irq pm8008_irqs[] = {
    _IRQ(PM8008_IRQ_MISC_UVLO,    PM8008_MISC,	BIT(0), IRQ_TYPE_EDGE_RISING),
    _IRQ(PM8008_IRQ_MISC_OVLO,    PM8008_MISC,	BIT(1), IRQ_TYPE_EDGE_RISING),
    _IRQ(PM8008_IRQ_MISC_OTST2,   PM8008_MISC,	BIT(2), IRQ_TYPE_EDGE_RISING),
    _IRQ(PM8008_IRQ_MISC_OTST3,   PM8008_MISC,	BIT(3), IRQ_TYPE_EDGE_RISING),
    _IRQ(PM8008_IRQ_MISC_LDO_OCP, PM8008_MISC,	BIT(4), IRQ_TYPE_EDGE_RISING),
    _IRQ(PM8008_IRQ_TEMP_ALARM,   PM8008_TEMP_ALARM,BIT(0), IRQ_TYPE_SENSE_MASK),
    _IRQ(PM8008_IRQ_GPIO1,	      PM8008_GPIO1,	BIT(0), IRQ_TYPE_SENSE_MASK),
    _IRQ(PM8008_IRQ_GPIO2,	      PM8008_GPIO2,	BIT(0), IRQ_TYPE_SENSE_MASK),
    };
    static const unsigned int pm8008_periph_base[] = {
    PM8008_PERIPH_0_BASE,
    PM8008_PERIPH_1_BASE,
    PM8008_PERIPH_2_BASE,
    PM8008_PERIPH_3_BASE,
    };
    static unsigned int pm8008_get_irq_reg(struct regmap_irq_chip_data *data,
    unsigned int base, int index)
    {
// Simple linear addressing for the main status register
    if (base == I2C_INTR_STATUS_BASE)
    return base + index;
    return pm8008_periph_base[index] + base;
    }
    static int pm8008_set_type_config(unsigned int **buf, unsigned int type,
    const struct regmap_irq *irq_data, int idx,
    void *irq_drv_data)
    {
    switch (type) {
    case IRQ_TYPE_EDGE_FALLING:
    case IRQ_TYPE_LEVEL_LOW:
    buf[POLARITY_HI_INDEX][idx] &= ~irq_data.mask;
    buf[POLARITY_LO_INDEX][idx] |= irq_data.mask;
    break;
    case IRQ_TYPE_EDGE_RISING:
    case IRQ_TYPE_LEVEL_HIGH:
    buf[POLARITY_HI_INDEX][idx] |= irq_data.mask;
    buf[POLARITY_LO_INDEX][idx] &= ~irq_data.mask;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    buf[POLARITY_HI_INDEX][idx] |= irq_data.mask;
    buf[POLARITY_LO_INDEX][idx] |= irq_data.mask;
    break;
    default:
    return -EINVAL;
    }
    if (type & IRQ_TYPE_EDGE_BOTH)
    buf[SET_TYPE_INDEX][idx] |= irq_data.mask;
    else
    buf[SET_TYPE_INDEX][idx] &= ~irq_data.mask;
    return 0;
    }
    static const struct regmap_irq_chip pm8008_irq_chip = {
    .name			= "pm8008",
    .main_status		= I2C_INTR_STATUS_BASE,
    .num_main_regs		= 1,
    .irqs			= pm8008_irqs,
    .num_irqs		= ARRAY_SIZE(pm8008_irqs),
    .num_regs		= PM8008_NUM_PERIPHS,
    .status_base		= INT_LATCHED_STS_OFFSET,
    .mask_base		= INT_EN_CLR_OFFSET,
    .unmask_base		= INT_EN_SET_OFFSET,
    .mask_unmask_non_inverted = true,
    .ack_base		= INT_LATCHED_CLR_OFFSET,
    .config_base		= pm8008_config_regs,
    .num_config_bases	= ARRAY_SIZE(pm8008_config_regs),
    .num_config_regs	= PM8008_NUM_PERIPHS,
    .set_type_config	= pm8008_set_type_config,
    .get_irq_reg		= pm8008_get_irq_reg,
    };
    static const struct regmap_config qcom_mfd_regmap_cfg = {
    .name		= "primary",
    .reg_bits	= 16,
    .val_bits	= 8,
    .max_register	= 0xffff,
    };
    static const struct regmap_config pm8008_regmap_cfg_2 = {
    .name		= "secondary",
    .reg_bits	= 16,
    .val_bits	= 8,
    .max_register	= 0xffff,
    };
    static const struct resource pm8008_temp_res[] = {
    DEFINE_RES_MEM(PM8008_TEMP_ALARM_ADDR, 0x100),
    DEFINE_RES_IRQ(PM8008_IRQ_TEMP_ALARM),
    };
    static const struct mfd_cell pm8008_cells[] = {
    MFD_CELL_NAME("pm8008-regulator"),
    MFD_CELL_RES("qpnp-temp-alarm", pm8008_temp_res),
    MFD_CELL_NAME("pm8008-gpio"),
    };
#[no_mangle]
unsafe extern "C" fn devm_irq_domain_fwnode_release(data: *mut c_void) {
    static void devm_irq_domain_fwnode_release(void *data)
    {
    struct fwnode_handle *fwnode = data;
    irq_domain_free_fwnode(fwnode);
    }
#[no_mangle]
unsafe extern "C" fn pm8008_probe(client: *mut i2c_client) -> c_int {
    static int pm8008_probe(struct i2c_client *client)
    {
    struct regmap_irq_chip_data *irq_data;
    struct device *dev = &client.dev;
    struct regmap *regmap, *regmap2;
    struct fwnode_handle *fwnode;
    struct i2c_client *dummy;
    struct gpio_desc *reset;
    char *name;
    int ret;
    dummy = devm_i2c_new_dummy_device(dev, client.adapter, client.addr + 1);
    if (IS_ERR(dummy)) {
    ret = PTR_ERR(dummy);
    dev_err(dev, "failed to claim second address: %d\n", ret);
    return ret;
    }
    regmap2 = devm_regmap_init_i2c(dummy, &qcom_mfd_regmap_cfg);
    if (IS_ERR(regmap2))
    return PTR_ERR(regmap2);
    ret = regmap_attach_dev(dev, regmap2, &pm8008_regmap_cfg_2);
    if (ret)
    return ret;
// Default regmap must be attached last.
    regmap = devm_regmap_init_i2c(client, &qcom_mfd_regmap_cfg);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(reset))
    return PTR_ERR(reset);
//
// The PMIC does not appear to require a post-reset delay, but wait
// for a millisecond for now anyway.
//
    usleep_range(1000, 2000);
    name = devm_kasprintf(dev, GFP_KERNEL, "%pOF-internal", dev.of_node);
    if (!name)
    return -ENOMEM;
    name = strreplace(name, '/', ':');
    fwnode = irq_domain_alloc_named_fwnode(name);
    if (!fwnode)
    return -ENOMEM;
    ret = devm_add_action_or_reset(dev, devm_irq_domain_fwnode_release, fwnode);
    if (ret)
    return ret;
    ret = devm_regmap_add_irq_chip_fwnode(dev, fwnode, regmap, client.irq,
    IRQF_SHARED, 0, &pm8008_irq_chip, &irq_data);
    if (ret) {
    dev_err(dev, "failed to add IRQ chip: %d\n", ret);
    return ret;
    }
// Needed by GPIO driver.
    dev_set_drvdata(dev, regmap_irq_get_domain(irq_data));
    return devm_mfd_add_devices(dev, PLATFORM_DEVID_AUTO, pm8008_cells,
    ARRAY_SIZE(pm8008_cells), core::ptr::null_mut(), 0,
    regmap_irq_get_domain(irq_data));
    }
    static const struct of_device_id pm8008_match[] = {
    { .compatible = "qcom,pm8008", },
    { },
    };
    MODULE_DEVICE_TABLE(of, pm8008_match);
    static struct i2c_driver pm8008_mfd_driver = {
    .driver = {
    .name = "pm8008",
    .of_match_table = pm8008_match,
    },
    .probe = pm8008_probe,
    };
    module_i2c_driver(pm8008_mfd_driver);
    MODULE_DESCRIPTION("QCOM PM8008 Power Management IC driver");
    MODULE_LICENSE("GPL v2");
