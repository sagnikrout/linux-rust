//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/88pm886.c
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

    static const struct regmap_config pm886_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = PM886_REG_RTC_SPARE6,
    };
    static const struct regmap_config pm886_regmap_battery_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = PM886_REG_CLS_CONFIG1,
    };
    static const struct regmap_irq pm886_regmap_irqs[] = {
    REGMAP_IRQ_REG(PM886_IRQ_ONKEY, 0, PM886_INT_ENA1_ONKEY),
    };
    static const struct regmap_irq_chip pm886_regmap_irq_chip = {
    .name = "88pm886",
    .irqs = pm886_regmap_irqs,
    .num_irqs = ARRAY_SIZE(pm886_regmap_irqs),
    .num_regs = 4,
    .status_base = PM886_REG_INT_STATUS1,
    .ack_base = PM886_REG_INT_STATUS1,
    .unmask_base = PM886_REG_INT_ENA_1,
    };
    static const struct resource pm886_onkey_resources[] = {
    DEFINE_RES_IRQ_NAMED(PM886_IRQ_ONKEY, "88pm886-onkey"),
    };
    static const struct mfd_cell pm886_devs[] = {
    MFD_CELL_NAME("88pm886-gpadc"),
    MFD_CELL_RES("88pm886-onkey", pm886_onkey_resources),
    MFD_CELL_NAME("88pm886-regulator"),
    MFD_CELL_NAME("88pm886-rtc"),
    };
#[no_mangle]
unsafe extern "C" fn pm886_power_off_handler(sys_off_data: *mut sys_off_data) -> c_int {
    static int pm886_power_off_handler(struct sys_off_data *sys_off_data)
    {
    struct pm886_chip *chip = sys_off_data.cb_data;
    struct regmap *regmap = chip.regmap;
    struct device *dev = &chip.client.dev;
    int err;
    err = regmap_update_bits(regmap, PM886_REG_MISC_CONFIG1, PM886_SW_PDOWN, PM886_SW_PDOWN);
    if (err) {
    dev_err(dev, "Failed to power off the device: %d\n", err);
    return NOTIFY_BAD;
    }
    return NOTIFY_DONE;
    }
    static int pm886_setup_irq(struct pm886_chip *chip,
    struct regmap_irq_chip_data **irq_data)
    {
    struct regmap *regmap = chip.regmap;
    struct device *dev = &chip.client.dev;
    int err;
// Set interrupt clearing mode to clear on write.
    err = regmap_update_bits(regmap, PM886_REG_MISC_CONFIG2,
    PM886_INT_INV | PM886_INT_CLEAR | PM886_INT_MASK_MODE,
    PM886_INT_WC);
    if (err) {
    dev_err(dev, "Failed to set interrupt clearing mode: %d\n", err);
    return err;
    }
    err = devm_regmap_add_irq_chip(dev, regmap, chip.client.irq,
    IRQF_ONESHOT, 0, &pm886_regmap_irq_chip,
    irq_data);
    if (err) {
    dev_err(dev, "Failed to request IRQ: %d\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm886_probe(client: *mut i2c_client) -> c_int {
    static int pm886_probe(struct i2c_client *client)
    {
    struct regmap *regmap, *regmap_battery;
    struct regmap_irq_chip_data *irq_data;
    struct device *dev = &client.dev;
    struct i2c_client *battery_page;
    struct pm886_chip *chip;
    unsigned int chip_id;
    int err;
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.client = client;
    chip.chip_id = (uintptr_t)device_get_match_data(dev);
    i2c_set_clientdata(client, chip);
    regmap = devm_regmap_init_i2c(client, &pm886_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap), "Failed to initialize regmap\n");
    chip.regmap = regmap;
    err = regmap_read(regmap, PM886_REG_ID, &chip_id);
    if (err)
    return dev_err_probe(dev, err, "Failed to read chip ID\n");
    if (chip.chip_id != chip_id)
    return dev_err_probe(dev, -EINVAL, "Unsupported chip: 0x%x\n", chip_id);
    battery_page = devm_i2c_new_dummy_device(dev, client.adapter,
    client.addr + PM886_PAGE_OFFSET_BATTERY);
    if (IS_ERR(battery_page))
    return dev_err_probe(dev, PTR_ERR(battery_page),
    "Failed to initialize battery page\n");
    regmap_battery = devm_regmap_init_i2c(battery_page, &pm886_regmap_battery_config);
    if (IS_ERR(regmap_battery))
    return dev_err_probe(dev, PTR_ERR(regmap_battery),
    "Failed to initialize battery regmap\n");
    chip.regmap_battery = regmap_battery;
    err = pm886_setup_irq(chip, &irq_data);
    if (err)
    return err;
    err = devm_mfd_add_devices(dev, PLATFORM_DEVID_NONE, pm886_devs, ARRAY_SIZE(pm886_devs),
    core::ptr::null_mut(), 0, regmap_irq_get_domain(irq_data));
    if (err)
    return dev_err_probe(dev, err, "Failed to add devices\n");
    err = devm_register_power_off_handler(dev, pm886_power_off_handler, chip);
    if (err)
    return dev_err_probe(dev, err, "Failed to register power off handler\n");
    if (device_property_read_bool(dev, "wakeup-source")) {
    err = devm_device_init_wakeup(dev);
    if (err)
    return dev_err_probe(dev, err, "Failed to init wakeup\n");
    }
    return 0;
    }
    static const struct of_device_id pm886_of_match[] = {
    { .compatible = "marvell,88pm886-a1", .data = (void *)PM886_A1_CHIP_ID },
    { }
    };
    MODULE_DEVICE_TABLE(of, pm886_of_match);
    static struct i2c_driver pm886_i2c_driver = {
    .driver = {
    .name = "88pm886",
    .of_match_table = pm886_of_match,
    },
    .probe = pm886_probe,
    };
    module_i2c_driver(pm886_i2c_driver);
    MODULE_DESCRIPTION("Marvell 88PM886 PMIC driver");
    MODULE_AUTHOR("Karel Balej <balejk@matfyz.cz>");
    MODULE_LICENSE("GPL");
