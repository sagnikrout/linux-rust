//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/hisi_powerkey.c
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
// Hisilicon PMIC powerkey driver
//
// Copyright (C) 2013 Hisilicon Ltd.
// Copyright (C) 2015, 2016 Linaro Ltd.
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file "COPYING" in the main directory of this
// archive for more details.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

// the held interrupt will trigger after 4 seconds

#[no_mangle]
unsafe extern "C" fn hi65xx_power_press_isr(irq: c_int, q: *mut c_void) -> irqreturn_t {
    static irqreturn_t hi65xx_power_press_isr(int irq, void *q)
    {
    struct input_dev *input = q;
    pm_wakeup_dev_event(input.dev.parent, MAX_HELD_TIME, true);
    input_report_key(input, KEY_POWER, 1);
    input_sync(input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hi65xx_power_release_isr(irq: c_int, q: *mut c_void) -> irqreturn_t {
    static irqreturn_t hi65xx_power_release_isr(int irq, void *q)
    {
    struct input_dev *input = q;
    pm_wakeup_event(input.dev.parent, MAX_HELD_TIME);
    input_report_key(input, KEY_POWER, 0);
    input_sync(input);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hi65xx_restart_toggle_isr(irq: c_int, q: *mut c_void) -> irqreturn_t {
    static irqreturn_t hi65xx_restart_toggle_isr(int irq, void *q)
    {
    struct input_dev *input = q;
    let mut value: c_int = test_bit(KEY_RESTART, input.key);
    pm_wakeup_event(input.dev.parent, MAX_HELD_TIME);
    input_report_key(input, KEY_RESTART, !value);
    input_sync(input);
    return IRQ_HANDLED;
    }
    static const struct {
    const char *name;
    irqreturn_t (*handler)(int irq, void *q);
    } hi65xx_irq_info[] = {
    { "down", hi65xx_power_press_isr },
    { "up", hi65xx_power_release_isr },
    { "hold 4s", hi65xx_restart_toggle_isr },
    };
#[no_mangle]
unsafe extern "C" fn hi65xx_powerkey_probe(pdev: *mut platform_device) -> c_int {
    static int hi65xx_powerkey_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct input_dev *input;
    int irq, i, error;
    input = devm_input_allocate_device(dev);
    if (!input) {
    dev_err(dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    input.phys = "hisi_on/input0";
    input.name = "HISI 65xx PowerOn Key";
    input_set_capability(input, EV_KEY, KEY_POWER);
    input_set_capability(input, EV_KEY, KEY_RESTART);
    for (i = 0; i < ARRAY_SIZE(hi65xx_irq_info); i++) {
    irq = platform_get_irq_byname(pdev, hi65xx_irq_info[i].name);
    if (irq < 0)
    return irq;
    error = devm_request_any_context_irq(dev, irq,
    hi65xx_irq_info[i].handler,
    IRQF_ONESHOT,
    hi65xx_irq_info[i].name,
    input);
    if (error < 0) {
    dev_err(dev, "couldn't request irq %s: %d\n",
    hi65xx_irq_info[i].name, error);
    return error;
    }
    }
    error = input_register_device(input);
    if (error) {
    dev_err(dev, "failed to register input device: %d\n", error);
    return error;
    }
    device_init_wakeup(dev, 1);
    return 0;
    }
    static struct platform_driver hi65xx_powerkey_driver = {
    .driver = {
    .name = "hi65xx-powerkey",
    },
    .probe = hi65xx_powerkey_probe,
    };
    module_platform_driver(hi65xx_powerkey_driver);
    MODULE_AUTHOR("Zhiliang Xue <xuezhiliang@huawei.com");
    MODULE_DESCRIPTION("Hisi PMIC Power key driver");
    MODULE_LICENSE("GPL v2");
