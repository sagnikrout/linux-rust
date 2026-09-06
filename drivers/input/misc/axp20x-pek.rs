//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/axp20x-pek.c
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


//
// axp20x power button driver.
//
// Copyright (C) 2013 Carlo Caione <carlo@caione.org>
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file "COPYING" in the main directory of this
// archive for more details.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axp20x_info {
    pub startup_time: *const axp20x_time,
    pub startup_mask: c_uint,
    pub shutdown_time: *const axp20x_time,
    pub shutdown_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axp20x_pek {
    pub axp20x: *mut axp20x_dev,
    pub input: *mut input_dev,
    pub info: *mut axp20x_info,
    pub irq_dbr: c_int,
    pub irq_dbf: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axp20x_time {
    pub time: c_uint,
    pub idx: c_uint,
}

    static const struct axp20x_time startup_time[] = {
    { .time = 128,  .idx = 0 },
    { .time = 1000, .idx = 2 },
    { .time = 3000, .idx = 1 },
    { .time = 2000, .idx = 3 },
    };
    static const struct axp20x_time axp221_startup_time[] = {
    { .time = 128,  .idx = 0 },
    { .time = 1000, .idx = 1 },
    { .time = 2000, .idx = 2 },
    { .time = 3000, .idx = 3 },
    };
    static const struct axp20x_time shutdown_time[] = {
    { .time = 4000,  .idx = 0 },
    { .time = 6000,  .idx = 1 },
    { .time = 8000,  .idx = 2 },
    { .time = 10000, .idx = 3 },
    };
    static const struct axp20x_info axp20x_info = {
    .startup_time = startup_time,
    .startup_mask = AXP20X_PEK_STARTUP_MASK,
    .shutdown_time = shutdown_time,
    .shutdown_mask = AXP20X_PEK_SHUTDOWN_MASK,
    };
    static const struct axp20x_info axp221_info = {
    .startup_time = axp221_startup_time,
    .startup_mask = AXP20X_PEK_STARTUP_MASK,
    .shutdown_time = shutdown_time,
    .shutdown_mask = AXP20X_PEK_SHUTDOWN_MASK,
    };
    static ssize_t axp20x_show_attr(struct device *dev,
    const struct axp20x_time *time,
    unsigned int mask, char *buf)
    {
    struct axp20x_pek *axp20x_pek = dev_get_drvdata(dev);
    unsigned int val;
    int ret, i;
    ret = regmap_read(axp20x_pek.axp20x.regmap, AXP20X_PEK_KEY, &val);
    if (ret != 0)
    return ret;
    val &= mask;
    val >>= ffs(mask) - 1;
    for (i = 0; i < 4; i++)
    if (val == time[i].idx)
    val = time[i].time;
    return sprintf(buf, "%u\n", val);
    }
    static ssize_t axp20x_show_attr_startup(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct axp20x_pek *axp20x_pek = dev_get_drvdata(dev);
    return axp20x_show_attr(dev, axp20x_pek.info.startup_time,
    axp20x_pek.info.startup_mask, buf);
    }
    static ssize_t axp20x_show_attr_shutdown(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct axp20x_pek *axp20x_pek = dev_get_drvdata(dev);
    return axp20x_show_attr(dev, axp20x_pek.info.shutdown_time,
    axp20x_pek.info.shutdown_mask, buf);
    }
    static ssize_t axp20x_store_attr(struct device *dev,
    const struct axp20x_time *time,
    unsigned int mask, const char *buf,
    size_t count)
    {
    struct axp20x_pek *axp20x_pek = dev_get_drvdata(dev);
    int ret, i;
    unsigned int val, idx = 0;
    let mut best_err: c_uint = UINT_MAX;
    ret = kstrtouint(buf, 10, &val);
    if (ret)
    return ret;
    for (i = 3; i >= 0; i--) {
    unsigned int err;
    err = abs(time[i].time - val);
    if (err < best_err) {
    best_err = err;
    idx = time[i].idx;
    }
    if (!err)
    break;
    }
    idx <<= ffs(mask) - 1;
    ret = regmap_update_bits(axp20x_pek.axp20x.regmap, AXP20X_PEK_KEY,
    mask, idx);
    if (ret != 0)
    return -EINVAL;
    return count;
    }
    static ssize_t axp20x_store_attr_startup(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct axp20x_pek *axp20x_pek = dev_get_drvdata(dev);
    return axp20x_store_attr(dev, axp20x_pek.info.startup_time,
    axp20x_pek.info.startup_mask, buf, count);
    }
    static ssize_t axp20x_store_attr_shutdown(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct axp20x_pek *axp20x_pek = dev_get_drvdata(dev);
    return axp20x_store_attr(dev, axp20x_pek.info.shutdown_time,
    axp20x_pek.info.shutdown_mask, buf, count);
    }
    static DEVICE_ATTR(startup, 0644, axp20x_show_attr_startup,
    axp20x_store_attr_startup);
    static DEVICE_ATTR(shutdown, 0644, axp20x_show_attr_shutdown,
    axp20x_store_attr_shutdown);
    static struct attribute *axp20x_attrs[] = {
    &dev_attr_startup.attr,
    &dev_attr_shutdown.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(axp20x);
#[no_mangle]
unsafe extern "C" fn axp20x_pek_irq(irq: c_int, pwr: *mut c_void) -> irqreturn_t {
    static irqreturn_t axp20x_pek_irq(int irq, void *pwr)
    {
    struct input_dev *idev = pwr;
    struct axp20x_pek *axp20x_pek = input_get_drvdata(idev);
//
// The power-button is connected to ground so a falling edge (dbf)
// means it is pressed.
//
    if (irq == axp20x_pek.irq_dbf)
    input_report_key(idev, KEY_POWER, true);
#[no_mangle]
pub unsafe extern "C" fn if(axp20x_pek->irq_dbr: irq ==) -> else {
    else if (irq == axp20x_pek.irq_dbr)
    input_report_key(idev, KEY_POWER, false);
    input_sync(idev);
    return IRQ_HANDLED;
    }
    static int axp20x_pek_probe_input_device(struct axp20x_pek *axp20x_pek,
    struct platform_device *pdev)
    {
    struct axp20x_dev *axp20x = axp20x_pek.axp20x;
    struct input_dev *idev;
    int error;
    axp20x_pek.irq_dbr = platform_get_irq_byname(pdev, "PEK_DBR");
    if (axp20x_pek.irq_dbr < 0)
    return axp20x_pek.irq_dbr;
    axp20x_pek.irq_dbr = regmap_irq_get_virq(axp20x.regmap_irqc,
    axp20x_pek.irq_dbr);
    axp20x_pek.irq_dbf = platform_get_irq_byname(pdev, "PEK_DBF");
    if (axp20x_pek.irq_dbf < 0)
    return axp20x_pek.irq_dbf;
    axp20x_pek.irq_dbf = regmap_irq_get_virq(axp20x.regmap_irqc,
    axp20x_pek.irq_dbf);
    axp20x_pek.input = devm_input_allocate_device(&pdev.dev);
    if (!axp20x_pek.input)
    return -ENOMEM;
    idev = axp20x_pek.input;
    idev.name = "axp20x-pek";
    idev.phys = "m1kbd/input2";
    idev.dev.parent = &pdev.dev;
    input_set_capability(idev, EV_KEY, KEY_POWER);
    input_set_drvdata(idev, axp20x_pek);
    error = devm_request_any_context_irq(&pdev.dev, axp20x_pek.irq_dbr,
    axp20x_pek_irq, 0,
    "axp20x-pek-dbr", idev);
    if (error < 0) {
    dev_err(&pdev.dev, "Failed to request dbr IRQ#%d: %d\n",
    axp20x_pek.irq_dbr, error);
    return error;
    }
    error = devm_request_any_context_irq(&pdev.dev, axp20x_pek.irq_dbf,
    axp20x_pek_irq, 0,
    "axp20x-pek-dbf", idev);
    if (error < 0) {
    dev_err(&pdev.dev, "Failed to request dbf IRQ#%d: %d\n",
    axp20x_pek.irq_dbf, error);
    return error;
    }
    error = input_register_device(idev);
    if (error) {
    dev_err(&pdev.dev, "Can't register input device: %d\n",
    error);
    return error;
    }
    device_init_wakeup(&pdev.dev, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp20x_pek_should_register_input(axp20x_pek: *mut axp20x_pek) -> bool {
    static bool axp20x_pek_should_register_input(struct axp20x_pek *axp20x_pek)
    {
    if (IS_ENABLED(CONFIG_INPUT_SOC_BUTTON_ARRAY) &&
    axp20x_pek.axp20x.variant == AXP288_ID) {
//
// On Cherry Trail platforms (hrv == 3), do not register the
// input device if there is an "INTCFD9" or "ACPI0011" gpio
// button ACPI device, as that handles the power button too,
// and otherwise we end up reporting all presses twice.
//
    if (soc_intel_is_cht() &&
    (acpi_dev_present("INTCFD9", core::ptr::null_mut(), -1) ||
    acpi_dev_present("ACPI0011", core::ptr::null_mut(), -1)))
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn axp20x_pek_probe(pdev: *mut platform_device) -> c_int {
    static int axp20x_pek_probe(struct platform_device *pdev)
    {
    struct axp20x_pek *axp20x_pek;
    const struct platform_device_id *match = platform_get_device_id(pdev);
    int error;
    if (!match) {
    dev_err(&pdev.dev, "Failed to get platform_device_id\n");
    return -EINVAL;
    }
    axp20x_pek = devm_kzalloc(&pdev.dev, sizeof(struct axp20x_pek),
    GFP_KERNEL);
    if (!axp20x_pek)
    return -ENOMEM;
    axp20x_pek.axp20x = dev_get_drvdata(pdev.dev.parent);
    if (axp20x_pek_should_register_input(axp20x_pek)) {
    error = axp20x_pek_probe_input_device(axp20x_pek, pdev);
    if (error)
    return error;
    }
    axp20x_pek.info = (struct axp20x_info *)match.driver_data;
    platform_set_drvdata(pdev, axp20x_pek);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp20x_pek_suspend(dev: *mut device) -> c_int {
    static int axp20x_pek_suspend(struct device *dev)
    {
    struct axp20x_pek *axp20x_pek = dev_get_drvdata(dev);
//
// As nested threaded IRQs are not automatically disabled during
// suspend, we must explicitly disable non-wakeup IRQs.
//
    if (device_may_wakeup(dev)) {
    enable_irq_wake(axp20x_pek.irq_dbf);
    enable_irq_wake(axp20x_pek.irq_dbr);
    } else {
    disable_irq(axp20x_pek.irq_dbf);
    disable_irq(axp20x_pek.irq_dbr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp20x_pek_resume(dev: *mut device) -> c_int {
    static int axp20x_pek_resume(struct device *dev)
    {
    struct axp20x_pek *axp20x_pek = dev_get_drvdata(dev);
    if (device_may_wakeup(dev)) {
    disable_irq_wake(axp20x_pek.irq_dbf);
    disable_irq_wake(axp20x_pek.irq_dbr);
    } else {
    enable_irq(axp20x_pek.irq_dbf);
    enable_irq(axp20x_pek.irq_dbr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn axp20x_pek_resume_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused axp20x_pek_resume_noirq(struct device *dev)
    {
    struct axp20x_pek *axp20x_pek = dev_get_drvdata(dev);
    if (axp20x_pek.axp20x.variant != AXP288_ID)
    return 0;
//
// Clear interrupts from button presses during suspend, to avoid
// a wakeup power-button press getting reported to userspace.
//
    regmap_write(axp20x_pek.axp20x.regmap,
    AXP20X_IRQ1_STATE + AXP288_IRQ_POKN / 8,
    BIT(AXP288_IRQ_POKN % 8));
    return 0;
    }
    static const struct dev_pm_ops axp20x_pek_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(axp20x_pek_suspend, axp20x_pek_resume)
    .resume_noirq = pm_sleep_ptr(axp20x_pek_resume_noirq),
    };
    static const struct platform_device_id axp_pek_id_match[] = {
    {
    .name = "axp20x-pek",
    .driver_data = (kernel_ulong_t)&axp20x_info,
    },
    {
    .name = "axp221-pek",
    .driver_data = (kernel_ulong_t)&axp221_info,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, axp_pek_id_match);
    static struct platform_driver axp20x_pek_driver = {
    .probe		= axp20x_pek_probe,
    .id_table	= axp_pek_id_match,
    .driver		= {
    .name		= "axp20x-pek",
    .pm		= pm_sleep_ptr(&axp20x_pek_pm_ops),
    .dev_groups	= axp20x_groups,
    },
    };
    module_platform_driver(axp20x_pek_driver);
    MODULE_DESCRIPTION("axp20x Power Button");
    MODULE_AUTHOR("Carlo Caione <carlo@caione.org>");
    MODULE_LICENSE("GPL");
