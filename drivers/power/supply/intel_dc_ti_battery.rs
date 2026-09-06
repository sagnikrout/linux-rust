//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/intel_dc_ti_battery.c
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
// Battery driver for the coulomb-counter of the Intel Dollar Cove TI PMIC
//
// Note the Intel Dollar Cove TI PMIC coulomb-counter is not a full-featured
// autonomous fuel-gauge. It is intended to work together with an always on
// micro-controller monitoring it.
//
// Since Linux does not monitor coulomb-counter changes while the device
// is off or suspended, voltage based capacity estimation from
// the adc-battery-helper code is used.
//
// Copyright (C) 2024 Hans de Goede <hansg@kernel.org>
//
// Register definitions and calibration code was taken from
// kernel/drivers/platform/x86/dc_ti_cc.c from the Acer A1-840 Android kernel
// which has the following copyright header:
//
// Copyright (C) 2014 Intel Corporation
// Author: Ramakrishna Pallala <ramakrishna.pallala@intel.com>
//
// dc_ti_cc.c is part of the Acer A1-840 Android kernel source-code archive
// named: "App. Guide_Acer_20151221_A_A.zip"
// which is distributed by Acer from the Acer A1-840 support page:
// https://www.acer.com/us-en/support/product-support/A1-840/downloads
//

pub const DC_TI_PMIC_VERSION_REG: c_uint = 0x00;
pub const PMIC_VERSION_A0: c_uint = 0xC0;
pub const PMIC_VERSION_A1: c_uint = 0xC1;
pub const DC_TI_CC_CNTL_REG: c_uint = 0x60;

pub const DC_TI_SMPL_CTR0_REG: c_uint = 0x69;
pub const DC_TI_SMPL_CTR1_REG: c_uint = 0x68;
pub const DC_TI_SMPL_CTR2_REG: c_uint = 0x67;
pub const DC_TI_CC_OFFSET_HI_REG: c_uint = 0x61;
pub const CC_OFFSET_HI_MASK: c_uint = 0x3F;
pub const DC_TI_CC_OFFSET_LO_REG: c_uint = 0x62;
pub const DC_TI_SW_OFFSET_REG: c_uint = 0x6C;
pub const DC_TI_CC_ACC3_REG: c_uint = 0x63;
pub const DC_TI_CC_ACC2_REG: c_uint = 0x64;
pub const DC_TI_CC_ACC1_REG: c_uint = 0x65;
pub const DC_TI_CC_ACC0_REG: c_uint = 0x66;
pub const DC_TI_CC_INTG1_REG: c_uint = 0x6A;
pub const DC_TI_CC_INTG1_MASK: c_uint = 0x3F;
pub const DC_TI_CC_INTG0_REG: c_uint = 0x6B;
pub const DC_TI_EEPROM_ACCESS_CONTROL: c_uint = 0x88;
pub const EEPROM_UNLOCK: c_uint = 0xDA;
pub const EEPROM_LOCK: c_uint = 0x00;
pub const DC_TI_EEPROM_CC_GAIN_REG: c_uint = 0xF4;

pub const PMIC_VERSION_A0_TRIM_REV: c_int = 3;
pub const PMIC_VERSION_A1_MIN_TRIM_REV: c_int = 1;
pub const DC_TI_EEPROM_CC_OFFSET_REG: c_uint = 0xFD;
pub const DC_TI_EEPROM_CTRL: c_uint = 0xFE;
pub const EEPROM_BANK0_SEL: c_uint = 0x01;
pub const EEPROM_BANK1_SEL: c_uint = 0x02;
pub const SMPL_INTVL_US: c_int = 15000;

pub const SLEEP_SLACK_US: c_int = 2500;
// CC gain correction is in 0.0025 increments
pub const CC_GAIN_STEP: c_int = 25;
pub const CC_GAIN_DIV: c_int = 10000;
// CC offset is in 0.5 units per 250ms (default sample interval)
pub const CC_OFFSET_DIV: c_int = 2;
pub const CC_OFFSET_SMPL_INTVL_MS: c_int = 250;
// CC accumulator scale is 366.2 ųCoulumb / unit

    ((acc) * (3662 * MSEC_PER_SEC / 10) / ((smpl_ctr) * SMPL_INTVL_MS))

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_ti_battery_chip {
// Must be the first member see adc-battery-helper documentation
    pub helper: adc_battery_helper,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub vbat_channel: *mut iio_channel,
    pub psy: *mut power_supply,
    pub cc_gain: c_int,
    pub cc_offset: c_int,
}

#[no_mangle]
unsafe extern "C" fn dc_ti_battery_get_voltage_and_current_now(psy: *mut power_supply, volt: *mut c_int, curr: *mut c_int) -> c_int {
    static int dc_ti_battery_get_voltage_and_current_now(struct power_supply *psy, int *volt, int *curr)
    {
    struct dc_ti_battery_chip *chip = power_supply_get_drvdata(psy);
    ktime_t ktime;
    s64 sleep_usec;
    unsigned int reg_val;
    s32 acc, smpl_ctr;
    int ret;
//
// Enable coulomb-counter before reading Vbat from ADC, so that the CC
// samples are from the same time period as the Vbat reading.
//
    ret = regmap_write(chip.regmap, DC_TI_CC_CNTL_REG,
    CC_CNTL_SMPL_INTVL_15MS | CC_CNTL_CC_OFFSET_EN | CC_CNTL_CC_CTR_EN);
    if (ret)
    goto out_err;
    ktime = ktime_get();
// Read Vbat, convert IIO mV to power-supply ųV
    ret = iio_read_channel_processed_scale(chip.vbat_channel, volt, 1000);
    if (ret < 0)
    goto out_err;
    ktime = ktime_sub(ktime_get(), ktime);
// Sleep at least 3 sample-times + slack to get 3+ CC samples
    sleep_usec = 3 * SMPL_INTVL_US + SLEEP_SLACK_US - ktime_to_us(ktime);
    if (sleep_usec > 0 && sleep_usec < 1000000)
    usleep_range(sleep_usec, sleep_usec + SLEEP_SLACK_US);
//
// The PMIC latches the coulomb- and sample-counters upon reading the
// CC_ACC0 register. Reading multiple registers at once is not supported.
//
// Step 1: Read CC_ACC0 - CC_ACC3
//
    ret = regmap_read(chip.regmap, DC_TI_CC_ACC0_REG, &reg_val);
    if (ret)
    goto out_err;
    acc = reg_val;
    ret = regmap_read(chip.regmap, DC_TI_CC_ACC1_REG, &reg_val);
    if (ret)
    goto out_err;
    acc |= reg_val << 8;
    ret = regmap_read(chip.regmap, DC_TI_CC_ACC2_REG, &reg_val);
    if (ret)
    goto out_err;
    acc |= reg_val << 16;
    ret = regmap_read(chip.regmap, DC_TI_CC_ACC3_REG, &reg_val);
    if (ret)
    goto out_err;
    acc |= reg_val << 24;
// Step 2: Read SMPL_CTR0 - SMPL_CTR2
    ret = regmap_read(chip.regmap, DC_TI_SMPL_CTR0_REG, &reg_val);
    if (ret)
    goto out_err;
    smpl_ctr = reg_val;
    ret = regmap_read(chip.regmap, DC_TI_SMPL_CTR1_REG, &reg_val);
    if (ret)
    goto out_err;
    smpl_ctr |= reg_val << 8;
    ret = regmap_read(chip.regmap, DC_TI_SMPL_CTR2_REG, &reg_val);
    if (ret)
    goto out_err;
    smpl_ctr |= reg_val << 16;
// Disable the coulumb-counter again
    ret = regmap_write(chip.regmap, DC_TI_CC_CNTL_REG,
    CC_CNTL_SMPL_INTVL_15MS | CC_CNTL_CC_OFFSET_EN);
    if (ret)
    goto out_err;
// Apply calibration
    acc -= chip.cc_offset * smpl_ctr * SMPL_INTVL_MS /
    (CC_OFFSET_SMPL_INTVL_MS * CC_OFFSET_DIV);
    acc = acc * (CC_GAIN_DIV - chip.cc_gain * CC_GAIN_STEP) / CC_GAIN_DIV;
// curr = CC_ACC_TO_UA(acc, smpl_ctr);
    return 0;
    out_err:
    dev_err(chip.dev, "IO-error %d communicating with PMIC\n", ret);
    return ret;
    }
    static const struct power_supply_desc dc_ti_battery_psy_desc = {
    .name		= "intel_dc_ti_battery",
    .type		= POWER_SUPPLY_TYPE_BATTERY,
    .get_property	= adc_battery_helper_get_property,
    .external_power_changed	= adc_battery_helper_external_power_changed,
    .properties	= adc_battery_helper_properties,
    .num_properties	= ADC_HELPER_NUM_PROPERTIES,
    };
#[no_mangle]
unsafe extern "C" fn dc_ti_battery_hw_init(chip: *mut dc_ti_battery_chip) -> c_int {
    static int dc_ti_battery_hw_init(struct dc_ti_battery_chip *chip)
    {
    u8 pmic_version, cc_trim_rev;
    unsigned int reg_val;
    int ret;
// Set sample rate to 15 ms and calibrate the coulomb-counter
    ret = regmap_write(chip.regmap, DC_TI_CC_CNTL_REG,
    CC_CNTL_SMPL_INTVL_15MS | CC_CNTL_CC_OFFSET_EN |
    CC_CNTL_CC_CAL_EN | CC_CNTL_CC_CTR_EN);
    if (ret)
    goto out;
    fsleep(CALIBRATION_TIME_US);
// Disable coulomb-counter it is only used while getting the current
    ret = regmap_write(chip.regmap, DC_TI_CC_CNTL_REG,
    CC_CNTL_SMPL_INTVL_15MS | CC_CNTL_CC_OFFSET_EN);
    if (ret)
    goto out;
    ret = regmap_read(chip.regmap, DC_TI_PMIC_VERSION_REG, &reg_val);
    if (ret)
    goto out;
    pmic_version = reg_val;
//
// As per the PMIC vendor (TI), the calibration offset and gain err
// values are stored in EEPROM Bank 0 and Bank 1 of the PMIC.
// We need to read the stored offset and gain margins and need
// to apply the corrections to the raw coulomb counter value.
//
// Unlock the EEPROM Access
    ret = regmap_write(chip.regmap, DC_TI_EEPROM_ACCESS_CONTROL, EEPROM_UNLOCK);
    if (ret)
    goto out;
// Select Bank 1 to read CC GAIN Err correction
    ret = regmap_write(chip.regmap, DC_TI_EEPROM_CTRL, EEPROM_BANK1_SEL);
    if (ret)
    goto out;
    ret = regmap_read(chip.regmap, DC_TI_EEPROM_CC_GAIN_REG, &reg_val);
    if (ret)
    goto out;
    cc_trim_rev = FIELD_GET(CC_TRIM_REVISION, reg_val);
    dev_dbg(chip.dev, "pmic-ver 0x%02x trim-rev %d\n", pmic_version, cc_trim_rev);
    if (!(pmic_version == PMIC_VERSION_A0 && cc_trim_rev == PMIC_VERSION_A0_TRIM_REV) &&
    !(pmic_version == PMIC_VERSION_A1 && cc_trim_rev >= PMIC_VERSION_A1_MIN_TRIM_REV)) {
    dev_dbg(chip.dev, "unsupported trim-revision, using uncalibrated CC values\n");
    goto out_relock;
    }
    chip.cc_gain = 1 - (int)FIELD_GET(CC_GAIN_CORRECTION, reg_val);
// Select Bank 0 to read CC OFFSET Correction
    ret = regmap_write(chip.regmap, DC_TI_EEPROM_CTRL, EEPROM_BANK0_SEL);
    if (ret)
    goto out_relock;
    ret = regmap_read(chip.regmap, DC_TI_EEPROM_CC_OFFSET_REG, &reg_val);
    if (ret)
    goto out_relock;
    chip.cc_offset = (s8)reg_val;
    dev_dbg(chip.dev, "cc-offset %d cc-gain %d\n", chip.cc_offset, chip.cc_gain);
    out_relock:
// Re-lock the EEPROM Access
    regmap_write(chip.regmap, DC_TI_EEPROM_ACCESS_CONTROL, EEPROM_LOCK);
    out:
    if (ret)
    dev_err(chip.dev, "IO-error %d initializing PMIC\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dc_ti_battery_probe(pdev: *mut platform_device) -> c_int {
    static int dc_ti_battery_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct intel_soc_pmic *pmic = dev_get_drvdata(dev.parent);
    let mut psy_cfg: power_supply_config = {};
    struct fwnode_reference_args args;
    struct gpio_desc *charge_finished;
    struct dc_ti_battery_chip *chip;
    int ret;
// On most devices with a Dollar Cove TI the battery is handled by ACPI
    if (!acpi_quirk_skip_acpi_ac_and_battery())
    return -ENODEV;
// ACPI glue code adds a "monitored-battery" fwnode, wait for this
    ret = fwnode_property_get_reference_args(dev_fwnode(dev), "monitored-battery",
    core::ptr::null_mut(), 0, 0, &args);
    if (ret) {
    dev_dbg(dev, "fwnode_property_get_ref() ret %d\n", ret);
    return dev_err_probe(dev, -EPROBE_DEFER, "Waiting for monitored-battery fwnode\n");
    }
    fwnode_handle_put(args.fwnode);
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dev = dev;
    chip.regmap = pmic.regmap;
    chip.vbat_channel = devm_iio_channel_get(dev, "VBAT");
    if (IS_ERR(chip.vbat_channel)) {
    dev_dbg(dev, "devm_iio_channel_get() ret %ld\n", PTR_ERR(chip.vbat_channel));
    return dev_err_probe(dev, -EPROBE_DEFER, "Waiting for VBAT IIO channel\n");
    }
    charge_finished = devm_gpiod_get_optional(dev, "charged", GPIOD_IN);
    if (IS_ERR(charge_finished))
    return dev_err_probe(dev, PTR_ERR(charge_finished), "Getting charged GPIO\n");
    ret = dc_ti_battery_hw_init(chip);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, chip);
    psy_cfg.drv_data = chip;
    chip.psy = devm_power_supply_register(dev, &dc_ti_battery_psy_desc, &psy_cfg);
    if (IS_ERR(chip.psy))
    return PTR_ERR(chip.psy);
    return adc_battery_helper_init(&chip.helper, chip.psy,
    dc_ti_battery_get_voltage_and_current_now,
    charge_finished);
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(dc_ti_battery_pm_ops, adc_battery_helper_suspend,
    adc_battery_helper_resume, core::ptr::null_mut());
    static struct platform_driver dc_ti_battery_driver = {
    .driver = {
    .name = DEV_NAME,
    .pm = pm_sleep_ptr(&dc_ti_battery_pm_ops),
    },
    .probe = dc_ti_battery_probe,
    };
    module_platform_driver(dc_ti_battery_driver);
    MODULE_ALIAS("platform:" DEV_NAME);
    MODULE_AUTHOR("Hans de Goede <hansg@kernel.org>");
    MODULE_DESCRIPTION("Intel Dollar Cove (TI) battery driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
