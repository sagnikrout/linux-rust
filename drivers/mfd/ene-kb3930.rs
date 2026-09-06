//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/ene-kb3930.c
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


// SPDX-License-Identifier: BSD-2-Clause OR GPL-2.0-or-later
//
// ENE KB3930 Embedded Controller Driver
//
// Copyright (C) 2020 Lubomir Rintel
//

// I2C registers that are multiplexing access to the EC RAM.
    enum {
    EC_DATA_IN	= 0x00,
    EC_RAM_OUT	= 0x80,
    EC_RAM_IN	= 0x81,
    };
// EC RAM registers.
    enum {
    EC_MODEL	= 0x30,
    EC_VERSION_MAJ	= 0x31,
    EC_VERSION_MIN	= 0x32,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kb3930 {
    pub client: *mut i2c_client,
    pub ram_regmap: *mut regmap,
    pub off_gpios: *mut gpio_descs,
}

    static struct kb3930 *kb3930_power_off;
pub const EC_GPIO_WAVE: c_int = 0;
pub const EC_GPIO_OFF_MODE: c_int = 1;
pub const EC_OFF_MODE_REBOOT: c_int = 0;
pub const EC_OFF_MODE_POWER: c_int = 1;
#[no_mangle]
unsafe extern "C" fn kb3930_off(ddata: *mut kb3930, off_mode: c_int) {
    static void kb3930_off(struct kb3930 *ddata, int off_mode)
    {
    gpiod_direction_output(ddata.off_gpios.desc[EC_GPIO_OFF_MODE],
    off_mode);
//
// This creates a 10 Hz wave on EC_GPIO_WAVE that signals a
// shutdown request to the EC. Once the EC detects it, it will
// proceed to turn the power off or reset the board depending on
// the value of EC_GPIO_OFF_MODE.
//
    while (1) {
    mdelay(50);
    gpiod_direction_output(ddata.off_gpios.desc[EC_GPIO_WAVE], 0);
    mdelay(50);
    gpiod_direction_output(ddata.off_gpios.desc[EC_GPIO_WAVE], 1);
    }
    }
    static int kb3930_restart(struct notifier_block *this,
    unsigned long mode, void *cmd)
    {
    kb3930_off(kb3930_power_off, EC_OFF_MODE_REBOOT);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn kb3930_pm_power_off() {
    static void kb3930_pm_power_off(void)
    {
    kb3930_off(kb3930_power_off, EC_OFF_MODE_POWER);
    }
    static struct notifier_block kb3930_restart_nb = {
    .notifier_call = kb3930_restart,
    };
    static const struct mfd_cell ariel_ec_cells[] = {
    { .name = "dell-wyse-ariel-led", },
    { .name = "dell-wyse-ariel-power", },
    };
    static int kb3930_ec_ram_reg_write(void *context, unsigned int reg,
    unsigned int val)
    {
    struct kb3930 *ddata = context;
    return i2c_smbus_write_word_data(ddata.client, EC_RAM_OUT,
    (val << 8) | reg);
    }
    static int kb3930_ec_ram_reg_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct kb3930 *ddata = context;
    int ret;
    ret = i2c_smbus_write_word_data(ddata.client, EC_RAM_IN, reg);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_read_word_data(ddata.client, EC_DATA_IN);
    if (ret < 0)
    return ret;
// val = ret >> 8;
    return 0;
    }
    static const struct regmap_config kb3930_ram_regmap_config = {
    .name = "ec_ram",
    .reg_bits = 8,
    .val_bits = 8,
    .reg_stride = 1,
    .max_register = 0xff,
    .reg_write = kb3930_ec_ram_reg_write,
    .reg_read = kb3930_ec_ram_reg_read,
    .fast_io = false,
    };
#[no_mangle]
unsafe extern "C" fn kb3930_probe(client: *mut i2c_client) -> c_int {
    static int kb3930_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device_node *np = dev.of_node;
    struct kb3930 *ddata;
    unsigned int model;
    int ret;
    ddata = devm_kzalloc(dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    kb3930_power_off = ddata;
    ddata.client = client;
    i2c_set_clientdata(client, ddata);
    ddata.ram_regmap = devm_regmap_init(dev, core::ptr::null_mut(), ddata,
    &kb3930_ram_regmap_config);
    if (IS_ERR(ddata.ram_regmap))
    return PTR_ERR(ddata.ram_regmap);
    ret = regmap_read(ddata.ram_regmap, EC_MODEL, &model);
    if (ret < 0)
    return ret;
// Currently we only support the cells present on Dell Ariel model.
    if (model != 'J') {
    dev_err(dev, "unknown board model: %02x\n", model);
    return -ENODEV;
    }
    ret = devm_mfd_add_devices(dev, PLATFORM_DEVID_AUTO,
    ariel_ec_cells,
    ARRAY_SIZE(ariel_ec_cells),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    return ret;
    if (of_device_is_system_power_controller(np)) {
    ddata.off_gpios =
    devm_gpiod_get_array_optional(dev, "off", GPIOD_IN);
    if (IS_ERR(ddata.off_gpios))
    return PTR_ERR(ddata.off_gpios);
    if (ddata.off_gpios && ddata.off_gpios.ndescs < 2) {
    dev_err(dev, "invalid off-gpios property\n");
    return -EINVAL;
    }
    }
    if (ddata.off_gpios) {
    register_restart_handler(&kb3930_restart_nb);
    if (!pm_power_off)
    pm_power_off = kb3930_pm_power_off;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kb3930_remove(client: *mut i2c_client) {
    static void kb3930_remove(struct i2c_client *client)
    {
    struct kb3930 *ddata = i2c_get_clientdata(client);
    if (ddata.off_gpios) {
    if (pm_power_off == kb3930_pm_power_off)
    pm_power_off = core::ptr::null_mut();
    unregister_restart_handler(&kb3930_restart_nb);
    }
    kb3930_power_off = core::ptr::null_mut();
    }
    static const struct of_device_id kb3930_dt_ids[] = {
    { .compatible = "ene,kb3930" },
    { }
    };
    MODULE_DEVICE_TABLE(of, kb3930_dt_ids);
    static struct i2c_driver kb3930_driver = {
    .probe = kb3930_probe,
    .remove = kb3930_remove,
    .driver = {
    .name = "ene-kb3930",
    .of_match_table = kb3930_dt_ids,
    },
    };
    module_i2c_driver(kb3930_driver);
    MODULE_AUTHOR("Lubomir Rintel <lkundrak@v3.sk>");
    MODULE_DESCRIPTION("ENE KB3930 Embedded Controller Driver");
    MODULE_LICENSE("Dual BSD/GPL");
