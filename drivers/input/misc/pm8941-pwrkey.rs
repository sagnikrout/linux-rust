//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/pm8941-pwrkey.c
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
// Copyright (c) 2010-2011, 2020-2021, The Linux Foundation. All rights reserved.
// Copyright (c) 2014, Sony Mobile Communications Inc.
//

pub const PON_REV2: c_uint = 0x01;
pub const PON_SUBTYPE: c_uint = 0x05;
pub const PON_SUBTYPE_PRIMARY: c_uint = 0x01;
pub const PON_SUBTYPE_SECONDARY: c_uint = 0x02;
pub const PON_SUBTYPE_1REG: c_uint = 0x03;
pub const PON_SUBTYPE_GEN2_PRIMARY: c_uint = 0x04;
pub const PON_SUBTYPE_GEN2_SECONDARY: c_uint = 0x05;
pub const PON_SUBTYPE_GEN3_PBS: c_uint = 0x08;
pub const PON_SUBTYPE_GEN3_HLOS: c_uint = 0x09;
pub const PON_RT_STS: c_uint = 0x10;

pub const PON_PS_HOLD_RST_CTL: c_uint = 0x5a;
pub const PON_PS_HOLD_RST_CTL2: c_uint = 0x5b;

pub const PON_PS_HOLD_TYPE_MASK: c_uint = 0x0f;
pub const PON_PS_HOLD_TYPE_WARM_RESET: c_int = 1;
pub const PON_PS_HOLD_TYPE_SHUTDOWN: c_int = 4;
pub const PON_PS_HOLD_TYPE_HARD_RESET: c_int = 7;
pub const PON_PULL_CTL: c_uint = 0x70;

pub const PON_DBC_CTL: c_uint = 0x71;
pub const PON_DBC_DELAY_MASK_GEN1: c_uint = 0x7;
pub const PON_DBC_DELAY_MASK_GEN2: c_uint = 0xf;
pub const PON_DBC_SHIFT_GEN1: c_int = 6;
pub const PON_DBC_SHIFT_GEN2: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8941_data {
    pub pull_up_bit: c_uint,
    pub status_bit: c_uint,
    pub supports_ps_hold_poff_config: bool,
    pub supports_debounce_config: bool,
    pub has_pon_pbs: bool,
    pub wakeup_source_default: bool,
    pub name: *const c_char,
    pub phys: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8941_pwrkey {
    pub dev: *mut device,
    pub irq: c_int,
    pub baseaddr: u32,
    pub pon_pbs_baseaddr: u32,
    pub regmap: *mut regmap,
    pub input: *mut input_dev,
    pub revision: c_uint,
    pub subtype: c_uint,
    pub reboot_notifier: notifier_block,
    pub code: u32,
    pub sw_debounce_time_us: u32,
    pub sw_debounce_end_time: ktime_t,
    pub last_status: bool,
    pub data: *const pm8941_data,
}

    static int pm8941_reboot_notify(struct notifier_block *nb,
    unsigned long code, void *unused)
    {
    struct pm8941_pwrkey *pwrkey = container_of(nb, struct pm8941_pwrkey,
    reboot_notifier);
    unsigned int enable_reg;
    unsigned int reset_type;
    int error;
// PMICs with revision 0 have the enable bit in same register as ctrl
    if (pwrkey.revision == 0)
    enable_reg = PON_PS_HOLD_RST_CTL;
    else
    enable_reg = PON_PS_HOLD_RST_CTL2;
    error = regmap_update_bits(pwrkey.regmap,
    pwrkey.baseaddr + enable_reg,
    PON_PS_HOLD_ENABLE,
    0);
    if (error)
    dev_err(pwrkey.dev,
    "unable to clear ps hold reset enable: %d\n",
    error);
//
// Updates of PON_PS_HOLD_ENABLE requires 3 sleep cycles between
// writes.
//
    usleep_range(100, 1000);
    switch (code) {
    case SYS_HALT:
    case SYS_POWER_OFF:
    reset_type = PON_PS_HOLD_TYPE_SHUTDOWN;
    break;
    case SYS_RESTART:
    default:
    if (reboot_mode == REBOOT_WARM)
    reset_type = PON_PS_HOLD_TYPE_WARM_RESET;
    else
    reset_type = PON_PS_HOLD_TYPE_HARD_RESET;
    break;
    }
    error = regmap_update_bits(pwrkey.regmap,
    pwrkey.baseaddr + PON_PS_HOLD_RST_CTL,
    PON_PS_HOLD_TYPE_MASK,
    reset_type);
    if (error)
    dev_err(pwrkey.dev, "unable to set ps hold reset type: %d\n",
    error);
    error = regmap_update_bits(pwrkey.regmap,
    pwrkey.baseaddr + enable_reg,
    PON_PS_HOLD_ENABLE,
    PON_PS_HOLD_ENABLE);
    if (error)
    dev_err(pwrkey.dev, "unable to re-set enable: %d\n", error);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn pm8941_pwrkey_irq(irq: c_int, _data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm8941_pwrkey_irq(int irq, void *_data)
    {
    struct pm8941_pwrkey *pwrkey = _data;
    unsigned int sts;
    int err;
    if (pwrkey.sw_debounce_time_us) {
    if (ktime_before(ktime_get(), pwrkey.sw_debounce_end_time)) {
    dev_dbg(pwrkey.dev,
    "ignoring key event received before debounce end %lld us\n",
    ktime_to_us(pwrkey.sw_debounce_end_time));
    return IRQ_HANDLED;
    }
    }
    err = regmap_read(pwrkey.regmap, pwrkey.baseaddr + PON_RT_STS, &sts);
    if (err)
    return IRQ_HANDLED;
    sts &= pwrkey.data.status_bit;
    if (pwrkey.sw_debounce_time_us && !sts)
    pwrkey.sw_debounce_end_time = ktime_add_us(ktime_get(),
    pwrkey.sw_debounce_time_us);
//
// Simulate a press event in case a release event occurred without a
// corresponding press event.
//
    if (!pwrkey.last_status && !sts) {
    input_report_key(pwrkey.input, pwrkey.code, 1);
    input_sync(pwrkey.input);
    }
    pwrkey.last_status = sts;
    input_report_key(pwrkey.input, pwrkey.code, sts);
    input_sync(pwrkey.input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm8941_pwrkey_sw_debounce_init(pwrkey: *mut pm8941_pwrkey) -> c_int {
    static int pm8941_pwrkey_sw_debounce_init(struct pm8941_pwrkey *pwrkey)
    {
    unsigned int val, addr, mask;
    int error;
    if (pwrkey.data.has_pon_pbs && !pwrkey.pon_pbs_baseaddr) {
    dev_err(pwrkey.dev,
    "PON_PBS address missing, can't read HW debounce time\n");
    return 0;
    }
    if (pwrkey.pon_pbs_baseaddr)
    addr = pwrkey.pon_pbs_baseaddr + PON_DBC_CTL;
    else
    addr = pwrkey.baseaddr + PON_DBC_CTL;
    error = regmap_read(pwrkey.regmap, addr, &val);
    if (error)
    return error;
    if (pwrkey.subtype >= PON_SUBTYPE_GEN2_PRIMARY)
    mask = 0xf;
    else
    mask = 0x7;
    pwrkey.sw_debounce_time_us =
    2 * USEC_PER_SEC / (1 << (mask - (val & mask)));
    dev_dbg(pwrkey.dev, "SW debounce time = %u us\n",
    pwrkey.sw_debounce_time_us);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8941_pwrkey_suspend(dev: *mut device) -> c_int {
    static int pm8941_pwrkey_suspend(struct device *dev)
    {
    struct pm8941_pwrkey *pwrkey = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(pwrkey.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8941_pwrkey_resume(dev: *mut device) -> c_int {
    static int pm8941_pwrkey_resume(struct device *dev)
    {
    struct pm8941_pwrkey *pwrkey = dev_get_drvdata(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(pwrkey.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pm8941_pwr_key_pm_ops,
    pm8941_pwrkey_suspend, pm8941_pwrkey_resume);
#[no_mangle]
unsafe extern "C" fn pm8941_pwrkey_probe(pdev: *mut platform_device) -> c_int {
    static int pm8941_pwrkey_probe(struct platform_device *pdev)
    {
    struct pm8941_pwrkey *pwrkey;
    bool pull_up, wakeup;
    struct device *parent;
    struct device_node *regmap_node;
    const __be32 *addr;
    u32 req_delay, mask, delay_shift;
    int error;
    if (of_property_read_u32(pdev.dev.of_node, "debounce", &req_delay))
    req_delay = 15625;
    if (req_delay > 2000000 || req_delay == 0) {
    dev_err(&pdev.dev, "invalid debounce time: %u\n", req_delay);
    return -EINVAL;
    }
    pull_up = of_property_read_bool(pdev.dev.of_node, "bias-pull-up");
    pwrkey = devm_kzalloc(&pdev.dev, sizeof(*pwrkey), GFP_KERNEL);
    if (!pwrkey)
    return -ENOMEM;
    pwrkey.dev = &pdev.dev;
    pwrkey.data = of_device_get_match_data(&pdev.dev);
    parent = pdev.dev.parent;
    regmap_node = pdev.dev.of_node;
    pwrkey.regmap = dev_get_regmap(parent, core::ptr::null_mut());
    if (!pwrkey.regmap) {
    regmap_node = parent.of_node;
//
// We failed to get regmap for parent. Let's see if we are
// a child of pon node and read regmap and reg from its
// parent.
//
    pwrkey.regmap = dev_get_regmap(parent.parent, core::ptr::null_mut());
    if (!pwrkey.regmap) {
    dev_err(&pdev.dev, "failed to locate regmap\n");
    return -ENODEV;
    }
    }
    addr = of_get_address(regmap_node, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (!addr) {
    dev_err(&pdev.dev, "reg property missing\n");
    return -EINVAL;
    }
    pwrkey.baseaddr = be32_to_cpup(addr);
    if (pwrkey.data.has_pon_pbs) {
// PON_PBS base address is optional
    addr = of_get_address(regmap_node, 1, core::ptr::null_mut(), core::ptr::null_mut());
    if (addr)
    pwrkey.pon_pbs_baseaddr = be32_to_cpup(addr);
    }
    pwrkey.irq = platform_get_irq(pdev, 0);
    if (pwrkey.irq < 0)
    return pwrkey.irq;
    error = regmap_read(pwrkey.regmap, pwrkey.baseaddr + PON_REV2,
    &pwrkey.revision);
    if (error) {
    dev_err(&pdev.dev, "failed to read revision: %d\n", error);
    return error;
    }
    error = regmap_read(pwrkey.regmap, pwrkey.baseaddr + PON_SUBTYPE,
    &pwrkey.subtype);
    if (error) {
    dev_err(&pdev.dev, "failed to read subtype: %d\n", error);
    return error;
    }
    error = of_property_read_u32(pdev.dev.of_node, "linux,code",
    &pwrkey.code);
    if (error) {
    dev_dbg(&pdev.dev,
    "no linux,code assuming power (%d)\n", error);
    pwrkey.code = KEY_POWER;
    }
    pwrkey.input = devm_input_allocate_device(&pdev.dev);
    if (!pwrkey.input) {
    dev_dbg(&pdev.dev, "unable to allocate input device\n");
    return -ENOMEM;
    }
    input_set_capability(pwrkey.input, EV_KEY, pwrkey.code);
    pwrkey.input.name = pwrkey.data.name;
    pwrkey.input.phys = pwrkey.data.phys;
    if (pwrkey.data.supports_debounce_config) {
    if (pwrkey.subtype >= PON_SUBTYPE_GEN2_PRIMARY) {
    mask = PON_DBC_DELAY_MASK_GEN2;
    delay_shift = PON_DBC_SHIFT_GEN2;
    } else {
    mask = PON_DBC_DELAY_MASK_GEN1;
    delay_shift = PON_DBC_SHIFT_GEN1;
    }
    req_delay = (req_delay << delay_shift) / USEC_PER_SEC;
    req_delay = ilog2(req_delay);
    error = regmap_update_bits(pwrkey.regmap,
    pwrkey.baseaddr + PON_DBC_CTL,
    mask,
    req_delay);
    if (error) {
    dev_err(&pdev.dev, "failed to set debounce: %d\n",
    error);
    return error;
    }
    }
    error = pm8941_pwrkey_sw_debounce_init(pwrkey);
    if (error)
    return error;
    if (pwrkey.data.pull_up_bit) {
    error = regmap_update_bits(pwrkey.regmap,
    pwrkey.baseaddr + PON_PULL_CTL,
    pwrkey.data.pull_up_bit,
    pull_up ? pwrkey.data.pull_up_bit :
    0);
    if (error) {
    dev_err(&pdev.dev, "failed to set pull: %d\n", error);
    return error;
    }
    }
    error = devm_request_threaded_irq(&pdev.dev, pwrkey.irq,
    core::ptr::null_mut(), pm8941_pwrkey_irq,
    IRQF_ONESHOT,
    pwrkey.data.name, pwrkey);
    if (error) {
    dev_err(&pdev.dev, "failed requesting IRQ: %d\n", error);
    return error;
    }
    error = input_register_device(pwrkey.input);
    if (error) {
    dev_err(&pdev.dev, "failed to register input device: %d\n",
    error);
    return error;
    }
    if (pwrkey.data.supports_ps_hold_poff_config) {
    pwrkey.reboot_notifier.notifier_call = pm8941_reboot_notify;
    error = register_reboot_notifier(&pwrkey.reboot_notifier);
    if (error) {
    dev_err(&pdev.dev, "failed to register reboot notifier: %d\n",
    error);
    return error;
    }
    }
    wakeup = pwrkey.data.wakeup_source_default ||
    of_property_read_bool(pdev.dev.of_node, "wakeup-source");
    platform_set_drvdata(pdev, pwrkey);
    device_init_wakeup(&pdev.dev, wakeup);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8941_pwrkey_remove(pdev: *mut platform_device) {
    static void pm8941_pwrkey_remove(struct platform_device *pdev)
    {
    struct pm8941_pwrkey *pwrkey = platform_get_drvdata(pdev);
    if (pwrkey.data.supports_ps_hold_poff_config)
    unregister_reboot_notifier(&pwrkey.reboot_notifier);
    }
    static const struct pm8941_data pwrkey_data = {
    .pull_up_bit = PON_KPDPWR_PULL_UP,
    .status_bit = PON_KPDPWR_N_SET,
    .name = "pm8941_pwrkey",
    .phys = "pm8941_pwrkey/input0",
    .supports_ps_hold_poff_config = true,
    .supports_debounce_config = true,
    .has_pon_pbs = false,
    .wakeup_source_default = true,
    };
    static const struct pm8941_data resin_data = {
    .pull_up_bit = PON_RESIN_PULL_UP,
    .status_bit = PON_RESIN_N_SET,
    .name = "pm8941_resin",
    .phys = "pm8941_resin/input0",
    .supports_ps_hold_poff_config = true,
    .supports_debounce_config = true,
    .has_pon_pbs = false,
    .wakeup_source_default = false,
    };
    static const struct pm8941_data pon_gen3_pwrkey_data = {
    .status_bit = PON_GEN3_KPDPWR_N_SET,
    .name = "pmic_pwrkey",
    .phys = "pmic_pwrkey/input0",
    .supports_ps_hold_poff_config = false,
    .supports_debounce_config = false,
    .has_pon_pbs = true,
    .wakeup_source_default = true,
    };
    static const struct pm8941_data pon_gen3_resin_data = {
    .status_bit = PON_GEN3_RESIN_N_SET,
    .name = "pmic_resin",
    .phys = "pmic_resin/input0",
    .supports_ps_hold_poff_config = false,
    .supports_debounce_config = false,
    .has_pon_pbs = true,
    .wakeup_source_default = false,
    };
    static const struct of_device_id pm8941_pwr_key_id_table[] = {
    { .compatible = "qcom,pm8941-pwrkey", .data = &pwrkey_data },
    { .compatible = "qcom,pm8941-resin", .data = &resin_data },
    { .compatible = "qcom,pmk8350-pwrkey", .data = &pon_gen3_pwrkey_data },
    { .compatible = "qcom,pmk8350-resin", .data = &pon_gen3_resin_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, pm8941_pwr_key_id_table);
    static struct platform_driver pm8941_pwrkey_driver = {
    .probe = pm8941_pwrkey_probe,
    .remove = pm8941_pwrkey_remove,
    .driver = {
    .name = "pm8941-pwrkey",
    .pm = pm_sleep_ptr(&pm8941_pwr_key_pm_ops),
    .of_match_table = of_match_ptr(pm8941_pwr_key_id_table),
    },
    };
    module_platform_driver(pm8941_pwrkey_driver);
    MODULE_DESCRIPTION("PM8941 Power Key driver");
    MODULE_LICENSE("GPL v2");
