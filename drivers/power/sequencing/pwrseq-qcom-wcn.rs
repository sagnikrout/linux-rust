//! Automatically rewritten from C to Rust
//! Source: drivers/power/sequencing/pwrseq-qcom-wcn.c
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
// Copyright (C) 2024 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwrseq_qcom_wcn_pdata {
    pub vregs: *const *const c_char,
    pub num_vregs: usize,
    pub pwup_delay_ms: c_uint,
    pub gpio_enable_delay_ms: c_uint,
    pub targets: *const pwrseq_target_data,
    pub /: *mut *mut bool has_vddio; / separate VDD IO regulator,
    pub dev): *mut *mut *mut int (match)(struct pwrseq_device pwrseq, struct device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwrseq_qcom_wcn_ctx {
    pub pwrseq: *mut pwrseq_device,
    pub of_node: *mut device_node,
    pub pdata: *const pwrseq_qcom_wcn_pdata,
    pub regs: *mut regulator_bulk_data,
    pub vddio: *mut regulator,
    pub bt_gpio: *mut gpio_desc,
    pub wlan_gpio: *mut gpio_desc,
    pub xo_clk_gpio: *mut gpio_desc,
    pub clk: *mut clk,
    pub last_gpio_enable_jf: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_ensure_gpio_delay(ctx: *mut pwrseq_qcom_wcn_ctx) {
    static void pwrseq_qcom_wcn_ensure_gpio_delay(struct pwrseq_qcom_wcn_ctx *ctx)
    {
    unsigned long diff_jiffies;
    unsigned int diff_msecs;
    if (!ctx.pdata.gpio_enable_delay_ms)
    return;
    diff_jiffies = jiffies - ctx.last_gpio_enable_jf;
    diff_msecs = jiffies_to_msecs(diff_jiffies);
    if (diff_msecs < ctx.pdata.gpio_enable_delay_ms)
    msleep(ctx.pdata.gpio_enable_delay_ms - diff_msecs);
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_vddio_enable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_vddio_enable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    return regulator_enable(ctx.vddio);
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_vddio_disable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_vddio_disable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    return regulator_disable(ctx.vddio);
    }
    static const struct pwrseq_unit_data pwrseq_qcom_wcn_vddio_unit_data = {
    .name = "vddio-enable",
    .enable = pwrseq_qcom_wcn_vddio_enable,
    .disable = pwrseq_qcom_wcn_vddio_disable,
    };
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_vregs_enable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_vregs_enable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    return regulator_bulk_enable(ctx.pdata.num_vregs, ctx.regs);
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_vregs_disable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_vregs_disable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    return regulator_bulk_disable(ctx.pdata.num_vregs, ctx.regs);
    }
    static const struct pwrseq_unit_data pwrseq_qcom_wcn_vregs_unit_data = {
    .name = "regulators-enable",
    .enable = pwrseq_qcom_wcn_vregs_enable,
    .disable = pwrseq_qcom_wcn_vregs_disable,
    };
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_clk_enable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_clk_enable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    return clk_prepare_enable(ctx.clk);
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_clk_disable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_clk_disable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    clk_disable_unprepare(ctx.clk);
    return 0;
    }
    static const struct pwrseq_unit_data pwrseq_qcom_wcn_clk_unit_data = {
    .name = "clock-enable",
    .enable = pwrseq_qcom_wcn_clk_enable,
    .disable = pwrseq_qcom_wcn_clk_disable,
    };
    static const struct pwrseq_unit_data *pwrseq_qcom_wcn3990_unit_deps[] = {
    &pwrseq_qcom_wcn_vddio_unit_data,
    &pwrseq_qcom_wcn_vregs_unit_data,
    core::ptr::null_mut(),
    };
    static const struct pwrseq_unit_data pwrseq_qcom_wcn3990_unit_data = {
    .name = "clock-enable",
    .deps = pwrseq_qcom_wcn3990_unit_deps,
    .enable = pwrseq_qcom_wcn_clk_enable,
    .disable = pwrseq_qcom_wcn_clk_disable,
    };
    static const struct pwrseq_unit_data *pwrseq_qcom_wcn_unit_deps[] = {
    &pwrseq_qcom_wcn_vregs_unit_data,
    &pwrseq_qcom_wcn_clk_unit_data,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn6855_clk_assert(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn6855_clk_assert(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    if (!ctx.xo_clk_gpio)
    return 0;
    msleep(1);
    gpiod_set_value_cansleep(ctx.xo_clk_gpio, 1);
    usleep_range(100, 200);
    return 0;
    }
    static const struct pwrseq_unit_data pwrseq_qcom_wcn6855_xo_clk_assert = {
    .name = "xo-clk-assert",
    .enable = pwrseq_qcom_wcn6855_clk_assert,
    };
    static const struct pwrseq_unit_data *pwrseq_qcom_wcn6855_unit_deps[] = {
    &pwrseq_qcom_wcn_vregs_unit_data,
    &pwrseq_qcom_wcn_clk_unit_data,
    &pwrseq_qcom_wcn6855_xo_clk_assert,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_bt_enable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_bt_enable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    pwrseq_qcom_wcn_ensure_gpio_delay(ctx);
    gpiod_set_value_cansleep(ctx.bt_gpio, 1);
    ctx.last_gpio_enable_jf = jiffies;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_bt_disable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_bt_disable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    gpiod_set_value_cansleep(ctx.bt_gpio, 0);
    return 0;
    }
    static const struct pwrseq_unit_data pwrseq_qcom_wcn_bt_unit_data = {
    .name = "bluetooth-enable",
    .deps = pwrseq_qcom_wcn_unit_deps,
    .enable = pwrseq_qcom_wcn_bt_enable,
    .disable = pwrseq_qcom_wcn_bt_disable,
    };
    static const struct pwrseq_unit_data pwrseq_qcom_wcn6855_bt_unit_data = {
    .name = "bluetooth-enable",
    .deps = pwrseq_qcom_wcn6855_unit_deps,
    .enable = pwrseq_qcom_wcn_bt_enable,
    .disable = pwrseq_qcom_wcn_bt_disable,
    };
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_wlan_enable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_wlan_enable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    pwrseq_qcom_wcn_ensure_gpio_delay(ctx);
    gpiod_set_value_cansleep(ctx.wlan_gpio, 1);
    ctx.last_gpio_enable_jf = jiffies;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_wlan_disable(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_wlan_disable(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    gpiod_set_value_cansleep(ctx.wlan_gpio, 0);
    return 0;
    }
    static const struct pwrseq_unit_data pwrseq_qcom_wcn_wlan_unit_data = {
    .name = "wlan-enable",
    .deps = pwrseq_qcom_wcn_unit_deps,
    .enable = pwrseq_qcom_wcn_wlan_enable,
    .disable = pwrseq_qcom_wcn_wlan_disable,
    };
    static const struct pwrseq_unit_data pwrseq_qcom_wcn6855_wlan_unit_data = {
    .name = "wlan-enable",
    .deps = pwrseq_qcom_wcn6855_unit_deps,
    .enable = pwrseq_qcom_wcn_wlan_enable,
    .disable = pwrseq_qcom_wcn_wlan_disable,
    };
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_pwup_delay(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn_pwup_delay(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    if (ctx.pdata.pwup_delay_ms)
    msleep(ctx.pdata.pwup_delay_ms);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn6855_xo_clk_deassert(pwrseq: *mut pwrseq_device) -> c_int {
    static int pwrseq_qcom_wcn6855_xo_clk_deassert(struct pwrseq_device *pwrseq)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    if (ctx.xo_clk_gpio) {
    usleep_range(2000, 5000);
    gpiod_set_value_cansleep(ctx.xo_clk_gpio, 0);
    }
    return pwrseq_qcom_wcn_pwup_delay(pwrseq);
    }
    static const struct pwrseq_target_data pwrseq_qcom_wcn_bt_target_data = {
    .name = "bluetooth",
    .unit = &pwrseq_qcom_wcn_bt_unit_data,
    .post_enable = pwrseq_qcom_wcn_pwup_delay,
    };
    static const struct pwrseq_target_data pwrseq_qcom_wcn_wlan_target_data = {
    .name = "wlan",
    .unit = &pwrseq_qcom_wcn_wlan_unit_data,
    .post_enable = pwrseq_qcom_wcn_pwup_delay,
    };
// There are no separate BT and WLAN enablement pins
    static const struct pwrseq_target_data pwrseq_qcom_wcn3990_bt_target_data = {
    .name = "bluetooth",
    .unit = &pwrseq_qcom_wcn3990_unit_data,
    };
    static const struct pwrseq_target_data pwrseq_qcom_wcn3990_wlan_target_data = {
    .name = "wlan",
    .unit = &pwrseq_qcom_wcn3990_unit_data,
    };
    static const struct pwrseq_target_data pwrseq_qcom_wcn6855_bt_target_data = {
    .name = "bluetooth",
    .unit = &pwrseq_qcom_wcn6855_bt_unit_data,
    .post_enable = pwrseq_qcom_wcn6855_xo_clk_deassert,
    };
    static const struct pwrseq_target_data pwrseq_qcom_wcn6855_wlan_target_data = {
    .name = "wlan",
    .unit = &pwrseq_qcom_wcn6855_wlan_unit_data,
    .post_enable = pwrseq_qcom_wcn6855_xo_clk_deassert,
    };
    static const struct pwrseq_target_data *pwrseq_qcom_wcn_targets[] = {
    &pwrseq_qcom_wcn_bt_target_data,
    &pwrseq_qcom_wcn_wlan_target_data,
    core::ptr::null_mut()
    };
    static const struct pwrseq_target_data *pwrseq_qcom_wcn3990_targets[] = {
    &pwrseq_qcom_wcn3990_bt_target_data,
    &pwrseq_qcom_wcn3990_wlan_target_data,
    core::ptr::null_mut()
    };
    static const struct pwrseq_target_data *pwrseq_qcom_wcn6855_targets[] = {
    &pwrseq_qcom_wcn6855_bt_target_data,
    &pwrseq_qcom_wcn6855_wlan_target_data,
    core::ptr::null_mut()
    };
    static const char *const pwrseq_qca6390_vregs[] = {
    "vddio",
    "vddaon",
    "vddpmu",
    "vddrfa0p95",
    "vddrfa1p3",
    "vddrfa1p9",
    "vddpcie1p3",
    "vddpcie1p9",
    };
    static const struct pwrseq_qcom_wcn_pdata pwrseq_qca6390_of_data = {
    .vregs = pwrseq_qca6390_vregs,
    .num_vregs = ARRAY_SIZE(pwrseq_qca6390_vregs),
    .pwup_delay_ms = 60,
    .gpio_enable_delay_ms = 100,
    .targets = pwrseq_qcom_wcn_targets,
    };
    static const char *const pwrseq_wcn3990_vregs[] = {
// vddio is handled separately
    "vddxo",
    "vddrf",
    "vddch0",
    "vddch1",
    };
    static int pwrseq_qcom_wcn3990_match(struct pwrseq_device *pwrseq,
    struct device *dev);
    static const struct pwrseq_qcom_wcn_pdata pwrseq_wcn3990_of_data = {
    .vregs = pwrseq_wcn3990_vregs,
    .num_vregs = ARRAY_SIZE(pwrseq_wcn3990_vregs),
    .pwup_delay_ms = 50,
    .targets = pwrseq_qcom_wcn3990_targets,
    .has_vddio = true,
    .match = pwrseq_qcom_wcn3990_match,
    };
    static const char *const pwrseq_wcn6750_vregs[] = {
    "vddaon",
    "vddasd",
    "vddpmu",
    "vddrfa0p8",
    "vddrfa1p2",
    "vddrfa1p7",
    "vddrfa2p2",
    };
    static const struct pwrseq_qcom_wcn_pdata pwrseq_wcn6750_of_data = {
    .vregs = pwrseq_wcn6750_vregs,
    .num_vregs = ARRAY_SIZE(pwrseq_wcn6750_vregs),
    .pwup_delay_ms = 50,
    .gpio_enable_delay_ms = 5,
    .targets = pwrseq_qcom_wcn_targets,
    };
    static const char *const pwrseq_wcn6855_vregs[] = {
    "vddio",
    "vddaon",
    "vddpmu",
    "vddpmumx",
    "vddpmucx",
    "vddrfa0p95",
    "vddrfa1p3",
    "vddrfa1p9",
    "vddpcie1p3",
    "vddpcie1p9",
    };
    static const struct pwrseq_qcom_wcn_pdata pwrseq_wcn6855_of_data = {
    .vregs = pwrseq_wcn6855_vregs,
    .num_vregs = ARRAY_SIZE(pwrseq_wcn6855_vregs),
    .pwup_delay_ms = 50,
    .gpio_enable_delay_ms = 5,
    .targets = pwrseq_qcom_wcn6855_targets,
    };
    static const char *const pwrseq_wcn7850_vregs[] = {
    "vdd",
    "vddio",
    "vddio1p2",
    "vddaon",
    "vdddig",
    "vddrfa1p2",
    "vddrfa1p8",
    };
    static const struct pwrseq_qcom_wcn_pdata pwrseq_wcn7850_of_data = {
    .vregs = pwrseq_wcn7850_vregs,
    .num_vregs = ARRAY_SIZE(pwrseq_wcn7850_vregs),
    .pwup_delay_ms = 50,
    .targets = pwrseq_qcom_wcn_targets,
    };
    static int pwrseq_qcom_wcn_match_regulator(struct pwrseq_device *pwrseq,
    struct device *dev,
    const char *name)
    {
    struct pwrseq_qcom_wcn_ctx *ctx = pwrseq_device_get_drvdata(pwrseq);
    struct device_node *dev_node = dev.of_node;
//
// The PMU supplies power to the Bluetooth and WLAN modules. both
// consume the PMU AON output so check the presence of the
// 'vddaon-supply' property and whether it leads us to the right
// device.
//
    if (!of_property_present(dev_node, name))
    return PWRSEQ_NO_MATCH;
    struct device_node *reg_node __free(device_node) =
    of_parse_phandle(dev_node, name, 0);
    if (!reg_node)
    return PWRSEQ_NO_MATCH;
//
// `reg_node` is the PMU AON regulator, its parent is the `regulators`
// node and finally its grandparent is the PMU device node that we're
// looking for.
//
    if (!reg_node.parent || !reg_node.parent.parent ||
    reg_node.parent.parent != ctx.of_node)
    return PWRSEQ_NO_MATCH;
    return PWRSEQ_MATCH_OK;
    }
    static int pwrseq_qcom_wcn_match(struct pwrseq_device *pwrseq,
    struct device *dev)
    {
    return pwrseq_qcom_wcn_match_regulator(pwrseq, dev, "vddaon-supply");
    }
    static int pwrseq_qcom_wcn3990_match(struct pwrseq_device *pwrseq,
    struct device *dev)
    {
    int ret;
// BT device
    ret = pwrseq_qcom_wcn_match_regulator(pwrseq, dev, "vddio-supply");
    if (ret == PWRSEQ_MATCH_OK)
    return ret;
// WiFi device match
    return pwrseq_qcom_wcn_match_regulator(pwrseq, dev, "vdd-1.8-xo-supply");
    }
#[no_mangle]
unsafe extern "C" fn pwrseq_qcom_wcn_probe(pdev: *mut platform_device) -> c_int {
    static int pwrseq_qcom_wcn_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pwrseq_qcom_wcn_ctx *ctx;
    struct pwrseq_config config;
    int i, ret;
    ctx = devm_kzalloc(dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.of_node = dev.of_node;
    ctx.pdata = device_get_match_data(dev);
    if (!ctx.pdata)
    return dev_err_probe(dev, -ENODEV,
    "Failed to obtain platform data\n");
    ctx.regs = devm_kcalloc(dev, ctx.pdata.num_vregs,
    sizeof(*ctx.regs), GFP_KERNEL);
    if (!ctx.regs)
    return -ENOMEM;
    for (i = 0; i < ctx.pdata.num_vregs; i++)
    ctx.regs[i].supply = ctx.pdata.vregs[i];
    ret = devm_regulator_bulk_get(dev, ctx.pdata.num_vregs, ctx.regs);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "Failed to get all regulators\n");
    if (ctx.pdata.has_vddio) {
    ctx.vddio = devm_regulator_get(dev, "vddio");
    if (IS_ERR(ctx.vddio))
    return dev_err_probe(dev, PTR_ERR(ctx.vddio), "Failed to get VDDIO\n");
    }
    ctx.bt_gpio = devm_gpiod_get_optional(dev, "bt-enable", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.bt_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.bt_gpio),
    "Failed to get the Bluetooth enable GPIO\n");
//
// FIXME: This should actually be GPIOD_OUT_LOW, but doing so would
// cause the WLAN power to be toggled, resulting in PCIe link down.
// Since the PCIe controller driver is not handling link down currently,
// the device becomes unusable. So we need to keep this workaround until
// the link down handling is implemented in the controller driver.
//
    ctx.wlan_gpio = devm_gpiod_get_optional(dev, "wlan-enable",
    GPIOD_ASIS);
    if (IS_ERR(ctx.wlan_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.wlan_gpio),
    "Failed to get the WLAN enable GPIO\n");
    ctx.xo_clk_gpio = devm_gpiod_get_optional(dev, "xo-clk",
    GPIOD_OUT_LOW);
    if (IS_ERR(ctx.xo_clk_gpio))
    return dev_err_probe(dev, PTR_ERR(ctx.xo_clk_gpio),
    "Failed to get the XO_CLK GPIO\n");
//
// Set direction to output but keep the current value in order to not
// disable the WLAN module accidentally if it's already powered on.
//
    gpiod_direction_output(ctx.wlan_gpio,
    gpiod_get_value_cansleep(ctx.wlan_gpio));
    ctx.clk = devm_clk_get_optional(dev, core::ptr::null_mut());
    if (IS_ERR(ctx.clk))
    return dev_err_probe(dev, PTR_ERR(ctx.clk),
    "Failed to get the reference clock\n");
    memset(&config, 0, sizeof(config));
    config.parent = dev;
    config.owner = THIS_MODULE;
    config.drvdata = ctx;
    config.match = ctx.pdata.match ? : pwrseq_qcom_wcn_match;
    config.targets = ctx.pdata.targets;
    ctx.pwrseq = devm_pwrseq_device_register(dev, &config);
    if (IS_ERR(ctx.pwrseq))
    return dev_err_probe(dev, PTR_ERR(ctx.pwrseq),
    "Failed to register the power sequencer\n");
    return 0;
    }
    static const struct of_device_id pwrseq_qcom_wcn_of_match[] = {
    {
    .compatible = "qcom,wcn3950-pmu",
    .data = &pwrseq_wcn3990_of_data,
    },
    {
    .compatible = "qcom,wcn3988-pmu",
    .data = &pwrseq_wcn3990_of_data,
    },
    {
    .compatible = "qcom,wcn3990-pmu",
    .data = &pwrseq_wcn3990_of_data,
    },
    {
    .compatible = "qcom,wcn3991-pmu",
    .data = &pwrseq_wcn3990_of_data,
    },
    {
    .compatible = "qcom,wcn3998-pmu",
    .data = &pwrseq_wcn3990_of_data,
    },
    {
    .compatible = "qcom,qca6390-pmu",
    .data = &pwrseq_qca6390_of_data,
    },
    {
    .compatible = "qcom,wcn6855-pmu",
    .data = &pwrseq_wcn6855_of_data,
    },
    {
    .compatible = "qcom,wcn7850-pmu",
    .data = &pwrseq_wcn7850_of_data,
    },
    {
    .compatible = "qcom,wcn6750-pmu",
    .data = &pwrseq_wcn6750_of_data,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, pwrseq_qcom_wcn_of_match);
    static struct platform_driver pwrseq_qcom_wcn_driver = {
    .driver = {
    .name = "pwrseq-qcom_wcn",
    .of_match_table = pwrseq_qcom_wcn_of_match,
    },
    .probe = pwrseq_qcom_wcn_probe,
    };
    module_platform_driver(pwrseq_qcom_wcn_driver);
    MODULE_AUTHOR("Bartosz Golaszewski <bartosz.golaszewski@linaro.org>");
    MODULE_DESCRIPTION("Qualcomm WCN PMU power sequencing driver");
    MODULE_LICENSE("GPL");
