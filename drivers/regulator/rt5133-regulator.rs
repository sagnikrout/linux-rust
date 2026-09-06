//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rt5133-regulator.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Richtek Technology Corp.
// Author: ChiYuan Huang <cy_huang@richtek.com>
// Author: ShihChia Chang <jeff_chang@richtek.com>

pub const RT5133_REG_CHIP_INFO: c_uint = 0x00;
pub const RT5133_REG_RST_CTRL: c_uint = 0x06;
pub const RT5133_REG_BASE_CTRL: c_uint = 0x09;
pub const RT5133_REG_GPIO_CTRL: c_uint = 0x0B;
pub const RT5133_REG_BASE_EVT: c_uint = 0x10;
pub const RT5133_REG_LDO_PGB_STAT: c_uint = 0x15;
pub const RT5133_REG_BASE_MASK: c_uint = 0x16;
pub const RT5133_REG_LDO_SHDN: c_uint = 0x19;
pub const RT5133_REG_LDO_ON: c_uint = 0x1A;
pub const RT5133_REG_LDO_OFF: c_uint = 0x1B;
pub const RT5133_REG_LDO1_CTRL1: c_uint = 0x20;
pub const RT5133_REG_LDO1_CTRL2: c_uint = 0x21;
pub const RT5133_REG_LDO1_CTRL3: c_uint = 0x22;
pub const RT5133_REG_LDO2_CTRL1: c_uint = 0x24;
pub const RT5133_REG_LDO2_CTRL2: c_uint = 0x25;
pub const RT5133_REG_LDO2_CTRL3: c_uint = 0x26;
pub const RT5133_REG_LDO3_CTRL1: c_uint = 0x28;
pub const RT5133_REG_LDO3_CTRL2: c_uint = 0x29;
pub const RT5133_REG_LDO3_CTRL3: c_uint = 0x2A;
pub const RT5133_REG_LDO4_CTRL1: c_uint = 0x2C;
pub const RT5133_REG_LDO4_CTRL2: c_uint = 0x2D;
pub const RT5133_REG_LDO4_CTRL3: c_uint = 0x2E;
pub const RT5133_REG_LDO5_CTRL1: c_uint = 0x30;
pub const RT5133_REG_LDO5_CTRL2: c_uint = 0x31;
pub const RT5133_REG_LDO5_CTRL3: c_uint = 0x32;
pub const RT5133_REG_LDO6_CTRL1: c_uint = 0x34;
pub const RT5133_REG_LDO6_CTRL2: c_uint = 0x35;
pub const RT5133_REG_LDO6_CTRL3: c_uint = 0x36;
pub const RT5133_REG_LDO7_CTRL1: c_uint = 0x38;
pub const RT5133_REG_LDO7_CTRL2: c_uint = 0x39;
pub const RT5133_REG_LDO7_CTRL3: c_uint = 0x3A;
pub const RT5133_REG_LDO8_CTRL1: c_uint = 0x3C;
pub const RT5133_REG_LDO8_CTRL2: c_uint = 0x3D;
pub const RT5133_REG_LDO8_CTRL3: c_uint = 0x3E;
pub const RT5133_REG_LDO8_CTRL4: c_uint = 0x3F;

pub const RT5133_RESET_CODE: c_uint = 0xB1;

pub const RT5133_GPIO_NR: c_int = 3;

pub const RT5133_LDO_PGB_EVT_SHIFT: c_int = 16;

pub const RT5133_LDO_OC_EVT_SHIFT: c_int = 8;

pub const RT5133_INTR_BYTE_NR: c_int = 3;
pub const RT5133_MAX_I2C_BLOCK_SIZE: c_int = 1;
pub const RT5133_CRC8_POLYNOMIAL: c_uint = 0x7;
pub const RT5133_I2C_ADDR_LEN: c_int = 1;
pub const RT5133_PREDATA_LEN: c_int = 2;
pub const RT5133_I2C_CRC_LEN: c_int = 1;
pub const RT5133_REG_ADDR_LEN: c_int = 1;
pub const RT5133_I2C_DUMMY_LEN: c_int = 1;

    enum {
    RT5133_REGULATOR_BASE = 0,
    RT5133_REGULATOR_LDO1,
    RT5133_REGULATOR_LDO2,
    RT5133_REGULATOR_LDO3,
    RT5133_REGULATOR_LDO4,
    RT5133_REGULATOR_LDO5,
    RT5133_REGULATOR_LDO6,
    RT5133_REGULATOR_LDO7,
    RT5133_REGULATOR_LDO8,
    RT5133_REGULATOR_MAX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chip_data {
    pub regulators: *const regulator_desc,
    pub vendor_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5133_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub enable_gpio: *mut gpio_desc,
    pub rdev: [*mut regulator_dev; RT5133_REGULATOR_MAX],
    pub gc: gpio_chip,
    pub cdata: *const chip_data,
    pub gpio_output_flag: c_uint,
    pub crc8_tbls: [u8; CRC8_TABLE_SIZE],
}

    static const unsigned int vout_type1_tables[] = {
    1800000, 2500000, 2700000, 2800000, 2900000, 3000000, 3100000, 3200000
    };
    static const unsigned int vout_type2_tables[] = {
    1700000, 1800000, 1900000, 2500000, 2700000, 2800000, 2900000, 3000000
    };
    static const unsigned int vout_type3_tables[] = {
    900000, 950000, 1000000, 1050000, 1100000, 1150000, 1200000, 1800000
    };
    static const unsigned int vout_type4_tables[] = {
    855000, 900000, 950000, 1000000, 1040000, 1090000, 1140000, 1710000
    };
    static const struct regulator_ops rt5133_regulator_ops = {
    .list_voltage = regulator_list_voltage_table,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .set_active_discharge = regulator_set_active_discharge_regmap,
    };
    static const struct regulator_ops rt5133_base_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    };

    {\
    .name = #_name,\
    .id = RT5133_REGULATOR_##_name,\
    .of_match = of_match_ptr(#_node_name),\
    .regulators_node = of_match_ptr("regulators"),\
    .supply_name = _supply,\
    .type = REGULATOR_VOLTAGE,\
    .owner = THIS_MODULE,\
    .ops = &rt5133_regulator_ops,\
    .n_voltages = ARRAY_SIZE(vtables),\
    .volt_table = vtables,\
    .enable_reg = RT5133_REG_##_name##_CTRL1,\
    .enable_mask = RT5133_LDO_ENABLE_MASK,\
    .vsel_reg = RT5133_REG_##_name##_CTRL2,\
    .vsel_mask = RT5133_LDO_VSEL_MASK,\
    .active_discharge_reg = RT5133_REG_##_name##_CTRL3,\
    .active_discharge_mask = RT5133_LDO_AD_MASK,\
    }
    static const struct regulator_desc rt5133_regulators[] = {
// For digital part, base current control
    {
    .name = "base",
    .id = RT5133_REGULATOR_BASE,
    .of_match = of_match_ptr("base"),
    .regulators_node = of_match_ptr("regulators"),
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .ops = &rt5133_base_regulator_ops,
    .enable_reg = RT5133_REG_BASE_CTRL,
    .enable_mask = RT5133_FOFF_BASE_MASK,
    .enable_is_inverted = true,
    },
    RT5133_REGULATOR_DESC(LDO1, ldo1, vout_type1_tables, "base"),
    RT5133_REGULATOR_DESC(LDO2, ldo2, vout_type1_tables, "base"),
    RT5133_REGULATOR_DESC(LDO3, ldo3, vout_type2_tables, "base"),
    RT5133_REGULATOR_DESC(LDO4, ldo4, vout_type2_tables, "base"),
    RT5133_REGULATOR_DESC(LDO5, ldo5, vout_type2_tables, "base"),
    RT5133_REGULATOR_DESC(LDO6, ldo6, vout_type2_tables, "base"),
    RT5133_REGULATOR_DESC(LDO7, ldo7, vout_type3_tables, "vin"),
    RT5133_REGULATOR_DESC(LDO8, ldo8, vout_type3_tables, "vin"),
    };
    static const struct regulator_desc rt5133a_regulators[] = {
// For digital part, base current control
    {
    .name = "base",
    .id = RT5133_REGULATOR_BASE,
    .of_match = of_match_ptr("base"),
    .regulators_node = of_match_ptr("regulators"),
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .ops = &rt5133_base_regulator_ops,
    .enable_reg = RT5133_REG_BASE_CTRL,
    .enable_mask = RT5133_FOFF_BASE_MASK,
    .enable_is_inverted = true,
    },
    RT5133_REGULATOR_DESC(LDO1, ldo1, vout_type1_tables, "base"),
    RT5133_REGULATOR_DESC(LDO2, ldo2, vout_type1_tables, "base"),
    RT5133_REGULATOR_DESC(LDO3, ldo3, vout_type2_tables, "base"),
    RT5133_REGULATOR_DESC(LDO4, ldo4, vout_type2_tables, "base"),
    RT5133_REGULATOR_DESC(LDO5, ldo5, vout_type2_tables, "base"),
    RT5133_REGULATOR_DESC(LDO6, ldo6, vout_type2_tables, "base"),
    RT5133_REGULATOR_DESC(LDO7, ldo7, vout_type3_tables, "vin"),
    RT5133_REGULATOR_DESC(LDO8, ldo8, vout_type4_tables, "vin"),
    };
    static const struct chip_data regulator_data[] = {
    { rt5133_regulators, 0x70},
    { rt5133a_regulators, 0x80},
    };
    static int rt5133_gpio_direction_output(struct gpio_chip *gpio,
    unsigned int offset, int value)
    {
    struct rt5133_priv *priv = gpiochip_get_data(gpio);
    if (offset >= RT5133_GPIO_NR)
    return -EINVAL;
    return regmap_update_bits(priv.regmap, RT5133_REG_GPIO_CTRL,
    BIT(7 - offset) | BIT(3 - offset),
    value ? BIT(7 - offset) | BIT(3 - offset) : 0);
    }
#[no_mangle]
unsafe extern "C" fn rt5133_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int rt5133_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct rt5133_priv *priv = gpiochip_get_data(chip);
    return !!(priv.gpio_output_flag & BIT(offset));
    }
#[no_mangle]
unsafe extern "C" fn rt5133_get_gpioen_mask(offset: c_uint, mask: *mut c_uint) -> c_int {
    static int rt5133_get_gpioen_mask(unsigned int offset, unsigned int *mask)
    {
    if (offset >= RT5133_GPIO_NR)
    return -EINVAL;
// mask = (BIT(7 - offset) | BIT(3 - offset));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt5133_gpio_set(chip: *mut gpio_chip, offset: c_uint, set_val: c_int) -> c_int {
    static int rt5133_gpio_set(struct gpio_chip *chip, unsigned int offset, int set_val)
    {
    struct rt5133_priv *priv = gpiochip_get_data(chip);
    let mut mask: c_uint = 0, val = 0, next_flag = priv.gpio_output_flag;
    let mut ret: c_int = 0;
    ret = rt5133_get_gpioen_mask(offset, &mask);
    if (ret) {
    dev_err(priv.dev, "%s get gpion en mask failed, offset(%d)\n", __func__, offset);
    return ret;
    }
    val = set_val ? mask : 0;
    if (set_val)
    next_flag |= BIT(offset);
    else
    next_flag &= ~BIT(offset);
    ret = regmap_update_bits(priv.regmap, RT5133_REG_GPIO_CTRL, mask, val);
    if (ret) {
    dev_err(priv.dev, "Failed to set gpio [%d] val %d\n", offset,
    set_val);
    return ret;
    }
    priv.gpio_output_flag = next_flag;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt5133_intr_handler(irq_number: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t rt5133_intr_handler(int irq_number, void *data)
    {
    struct rt5133_priv *priv = data;
    let mut intr_evts: u32 = 0, handle_evts;
    int i, ret;
    ret = regmap_bulk_read(priv.regmap, RT5133_REG_BASE_EVT, &intr_evts,
    RT5133_INTR_BYTE_NR);
    if (ret) {
    dev_err(priv.dev, "%s, read event failed\n", __func__);
    return IRQ_NONE;
    }
    handle_evts = intr_evts & RT5133_BASE_EVT_MASK;
//
// VREF_EVT is a special case, if base off
// this event will also be trigger. Skip it
//
    if (handle_evts & ~RT5133_VREF_EVT_MASK)
    dev_dbg(priv.dev, "base event occurred [0x%02x]\n",
    handle_evts);
    handle_evts = (intr_evts & RT5133_LDO_OC_EVT_MASK) >>
    RT5133_LDO_OC_EVT_SHIFT;
    for (i = RT5133_REGULATOR_LDO1; i < RT5133_REGULATOR_MAX && handle_evts; i++) {
    if (!(handle_evts & BIT(i - 1)))
    continue;
    regulator_notifier_call_chain(priv.rdev[i],
    REGULATOR_EVENT_OVER_CURRENT,
    &i);
    }
    handle_evts = (intr_evts & RT5133_LDO_PGB_EVT_MASK) >>
    RT5133_LDO_PGB_EVT_SHIFT;
    for (i = RT5133_REGULATOR_LDO1; i < RT5133_REGULATOR_MAX && handle_evts; i++) {
    if (!(handle_evts & BIT(i - 1)))
    continue;
    regulator_notifier_call_chain(priv.rdev[i],
    REGULATOR_EVENT_FAIL, &i);
    }
    ret = regmap_bulk_write(priv.regmap, RT5133_REG_BASE_EVT, &intr_evts,
    RT5133_INTR_BYTE_NR);
    if (ret)
    dev_err(priv.dev, "%s, clear event failed\n", __func__);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rt5133_enable_interrupts(irq_no: c_int, priv: *mut rt5133_priv) -> c_int {
    static int rt5133_enable_interrupts(int irq_no, struct rt5133_priv *priv)
    {
    let mut mask: u32 = RT5133_INTR_CLR_MASK;
    int ret;
// Force to write clear all events
    ret = regmap_bulk_write(priv.regmap, RT5133_REG_BASE_EVT, &mask,
    RT5133_INTR_BYTE_NR);
    if (ret) {
    dev_err(priv.dev, "Failed to clear all interrupts\n");
    return ret;
    }
// Unmask all interrupts
    mask = 0;
    ret = regmap_bulk_write(priv.regmap, RT5133_REG_BASE_MASK, &mask,
    RT5133_INTR_BYTE_NR);
    if (ret) {
    dev_err(priv.dev, "Failed to unmask all interrupts\n");
    return ret;
    }
    return devm_request_threaded_irq(priv.dev, irq_no, core::ptr::null_mut(),
    rt5133_intr_handler, IRQF_ONESHOT,
    dev_name(priv.dev), priv);
    }
    static int rt5133_regmap_hw_read(void *context, const void *reg_buf,
    size_t reg_size, void *val_buf,
    size_t val_size)
    {
    struct rt5133_priv *priv = context;
    struct i2c_client *client = to_i2c_client(priv.dev);
    let mut reg: u8 = *(u8 *)reg_buf, crc;
    u8 *buf;
    let mut buf_len: c_int = RT5133_PREDATA_LEN + val_size + RT5133_I2C_CRC_LEN;
    int read_len, ret;
    buf = kzalloc(buf_len, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    buf[0] = I2C_ADDR_XLATE_8BIT(client.addr, I2C_SMBUS_READ);
    buf[1] = reg;
    read_len = val_size + RT5133_I2C_CRC_LEN;
    ret = i2c_smbus_read_i2c_block_data(client, reg, read_len,
    buf + RT5133_PREDATA_LEN);
    if (ret < 0)
    goto out_read_err;
    if (ret != read_len) {
    ret = -EIO;
    goto out_read_err;
    }
    crc = crc8(priv.crc8_tbls, buf, RT5133_PREDATA_LEN + val_size, 0);
    if (crc != buf[RT5133_PREDATA_LEN + val_size]) {
    ret = -EIO;
    goto out_read_err;
    }
    memcpy(val_buf, buf + RT5133_PREDATA_LEN, val_size);
    dev_dbg(priv.dev, "%s, reg = 0x%02x, data = 0x%02x\n", __func__, reg, *(u8 *)val_buf);
    out_read_err:
    kfree(buf);
    return (ret < 0) ? ret : 0;
    }
#[no_mangle]
unsafe extern "C" fn rt5133_regmap_hw_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int rt5133_regmap_hw_write(void *context, const void *data, size_t count)
    {
    struct rt5133_priv *priv = context;
    struct i2c_client *client = to_i2c_client(priv.dev);
    let mut reg: u8 = *(u8 *)data, crc;
    u8 *buf;
    int buf_len = RT5133_I2C_ADDR_LEN + count + RT5133_I2C_CRC_LEN +
    RT5133_I2C_DUMMY_LEN;
    int write_len, ret;
    buf = kzalloc(buf_len, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    buf[0] = I2C_ADDR_XLATE_8BIT(client.addr, I2C_SMBUS_WRITE);
    buf[1] = reg;
    memcpy(buf + RT5133_PREDATA_LEN, data + RT5133_REG_ADDR_LEN,
    count - RT5133_REG_ADDR_LEN);
    crc = crc8(priv.crc8_tbls, buf, RT5133_I2C_ADDR_LEN + count, 0);
    buf[RT5133_I2C_ADDR_LEN + count] = crc;
    write_len = count - RT5133_REG_ADDR_LEN + RT5133_I2C_CRC_LEN +
    RT5133_I2C_DUMMY_LEN;
    ret = i2c_smbus_write_i2c_block_data(client, reg, write_len,
    buf + RT5133_PREDATA_LEN);
    dev_dbg(priv.dev, "%s, reg = 0x%02x, data = 0x%02x\n", __func__, reg,
// (u8 *)(buf + RT5133_PREDATA_LEN));
    kfree(buf);
    return ret;
    }
    static const struct regmap_bus rt5133_regmap_bus = {
    .read = rt5133_regmap_hw_read,
    .write = rt5133_regmap_hw_write,
// Due to crc, the block read/write length has the limit
    .max_raw_read = RT5133_MAX_I2C_BLOCK_SIZE,
    .max_raw_write = RT5133_MAX_I2C_BLOCK_SIZE,
    };
#[no_mangle]
unsafe extern "C" fn rt5133_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rt5133_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case RT5133_REG_CHIP_INFO:
    case RT5133_REG_BASE_EVT...RT5133_REG_LDO_PGB_STAT:
    case RT5133_REG_LDO_ON...RT5133_REG_LDO_OFF:
    case RT5133_REG_LDO1_CTRL1:
    case RT5133_REG_LDO2_CTRL1:
    case RT5133_REG_LDO3_CTRL1:
    case RT5133_REG_LDO4_CTRL1:
    case RT5133_REG_LDO5_CTRL1:
    case RT5133_REG_LDO6_CTRL1:
    case RT5133_REG_LDO7_CTRL1:
    case RT5133_REG_LDO8_CTRL1:
    return true;
    default:
    return false;
    };
    }
    static const struct regmap_config rt5133_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RT5133_REG_LDO8_CTRL4,
    .cache_type = REGCACHE_FLAT,
    .num_reg_defaults_raw = RT5133_REG_LDO8_CTRL4 + 1,
    .volatile_reg = rt5133_is_volatile_reg,
    };
#[no_mangle]
unsafe extern "C" fn rt5133_chip_reset(priv: *mut rt5133_priv) -> c_int {
    static int rt5133_chip_reset(struct rt5133_priv *priv)
    {
    int ret;
    ret = regmap_write(priv.regmap, RT5133_REG_RST_CTRL,
    RT5133_RESET_CODE);
    if (ret)
    return ret;
// Wait for register reset to take effect
    udelay(2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt5133_validate_vendor_info(priv: *mut rt5133_priv) -> c_int {
    static int rt5133_validate_vendor_info(struct rt5133_priv *priv)
    {
    let mut val: c_uint = 0;
    int i, ret;
    ret = regmap_read(priv.regmap, RT5133_REG_CHIP_INFO, &val);
    if (ret)
    return ret;
    for (i = 0; i < ARRAY_SIZE(regulator_data); i++) {
    if ((val & RT5133_VENDOR_ID_MASK) ==
    regulator_data[i].vendor_id){
    priv.cdata = &regulator_data[i];
    break;
    }
    }
    if (!priv.cdata) {
    dev_err(priv.dev, "Failed to find regulator match version\n");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt5133_parse_dt(priv: *mut rt5133_priv) -> c_int {
    static int rt5133_parse_dt(struct rt5133_priv *priv)
    {
    let mut val: c_uint = 0;
    let mut ret: c_int = 0;
    if (!device_property_read_bool(priv.dev, "richtek,oc-shutdown-all"))
    val = 0;
    else
    val = 1 << RT5133_OCSHDN_ALL_SHIFT;
    ret = regmap_update_bits(priv.regmap, RT5133_REG_LDO_SHDN,
    RT5133_OCSHDN_ALL_MASK, val);
    if (ret)
    return ret;
    if (!device_property_read_bool(priv.dev, "richtek,pgb-shutdown-all"))
    val = 0;
    else
    val = 1 << RT5133_PGBSHDN_ALL_SHIFT;
    return regmap_update_bits(priv.regmap, RT5133_REG_LDO_SHDN,
    RT5133_PGBSHDN_ALL_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn rt5133_probe(i2c: *mut i2c_client) -> c_int {
    static int rt5133_probe(struct i2c_client *i2c)
    {
    struct rt5133_priv *priv;
    let mut config: regulator_config = {0};
    int i, ret;
    priv = devm_kzalloc(&i2c.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &i2c.dev;
    crc8_populate_msb(priv.crc8_tbls, RT5133_CRC8_POLYNOMIAL);
    priv.enable_gpio = devm_gpiod_get_optional(&i2c.dev, "enable",
    GPIOD_OUT_HIGH);
    if (IS_ERR(priv.enable_gpio))
    dev_err(&i2c.dev, "Failed to request HWEN gpio, check if default en=high\n");
    priv.regmap = devm_regmap_init(&i2c.dev, &rt5133_regmap_bus, priv,
    &rt5133_regmap_config);
    if (IS_ERR(priv.regmap)) {
    dev_err(&i2c.dev, "Failed to register regmap\n");
    return PTR_ERR(priv.regmap);
    }
    ret = rt5133_validate_vendor_info(priv);
    if (ret) {
    dev_err(&i2c.dev, "Failed to check vendor info [%d]\n", ret);
    return ret;
    }
    ret = rt5133_chip_reset(priv);
    if (ret) {
    dev_err(&i2c.dev, "Failed to execute sw reset\n");
    return ret;
    }
    config.dev = &i2c.dev;
    config.driver_data = priv;
    config.regmap = priv.regmap;
    for (i = 0; i < RT5133_REGULATOR_MAX; i++) {
    priv.rdev[i] = devm_regulator_register(&i2c.dev,
    priv.cdata.regulators + i,
    &config);
    if (IS_ERR(priv.rdev[i])) {
    dev_err(&i2c.dev,
    "Failed to register [%d] regulator\n", i);
    return PTR_ERR(priv.rdev[i]);
    }
    }
    ret = rt5133_parse_dt(priv);
    if (ret) {
    dev_err(&i2c.dev, "%s, Failed to parse dt\n", __func__);
    return ret;
    }
    priv.gc.label = dev_name(&i2c.dev);
    priv.gc.parent = &i2c.dev;
    priv.gc.base = -1;
    priv.gc.ngpio = RT5133_GPIO_NR;
    priv.gc.set = rt5133_gpio_set;
    priv.gc.get = rt5133_gpio_get;
    priv.gc.direction_output = rt5133_gpio_direction_output;
    priv.gc.can_sleep = true;
    ret = devm_gpiochip_add_data(&i2c.dev, &priv.gc, priv);
    if (ret)
    return ret;
    ret = rt5133_enable_interrupts(i2c.irq, priv);
    if (ret) {
    dev_err(&i2c.dev, "enable interrupt failed\n");
    return ret;
    }
    i2c_set_clientdata(i2c, priv);
    return ret;
    }
    static const struct of_device_id __maybe_unused rt5133_of_match_table[] = {
    { .compatible = "richtek,rt5133", },
    { }
    };
    MODULE_DEVICE_TABLE(of, rt5133_of_match_table);
    static struct i2c_driver rt5133_driver = {
    .driver = {
    .name = "rt5133",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = rt5133_of_match_table,
    },
    .probe = rt5133_probe,
    };
    module_i2c_driver(rt5133_driver);
    MODULE_DESCRIPTION("RT5133 Regulator Driver");
    MODULE_LICENSE("GPL v2");
