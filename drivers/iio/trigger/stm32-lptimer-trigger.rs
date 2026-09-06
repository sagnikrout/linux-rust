//! Automatically rewritten from C to Rust
//! Source: drivers/iio/trigger/stm32-lptimer-trigger.c
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
//
// STM32 Low-Power Timer Trigger driver
//
// Copyright (C) STMicroelectronics 2017
//
// Author: Fabrice Gasnier <fabrice.gasnier@st.com>.
//
// Inspired by Benjamin Gaignard's stm32-timer-trigger driver
//

// Maximum triggers + one trailing null entry to indicate the end of array
pub const MAX_TRIGGERS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_lptim_cfg {
    pub (*triggers)[MAX_TRIGGERS]: *const *const c_char,
    pub nb_triggers: c_uint,
}

// List Low-Power Timer triggers for H7, MP13, MP15
    static const char * const stm32_lptim_triggers[][MAX_TRIGGERS] = {
    { LPTIM1_OUT,},
    { LPTIM2_OUT,},
    { LPTIM3_OUT,},
    };
// List Low-Power Timer triggers for STM32MP25
    static const char * const stm32mp25_lptim_triggers[][MAX_TRIGGERS] = {
    { LPTIM1_CH1, LPTIM1_CH2, },
    { LPTIM2_CH1, LPTIM2_CH2, },
    { LPTIM3_CH1,},
    { LPTIM4_CH1,},
    { LPTIM5_OUT,},
    };
    static const struct stm32_lptim_cfg stm32mp15_lptim_cfg = {
    .triggers = stm32_lptim_triggers,
    .nb_triggers = ARRAY_SIZE(stm32_lptim_triggers),
    };
    static const struct stm32_lptim_cfg stm32mp25_lptim_cfg = {
    .triggers = stm32mp25_lptim_triggers,
    .nb_triggers = ARRAY_SIZE(stm32mp25_lptim_triggers),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_lptim_trigger {
    pub dev: *mut device,
    pub triggers: *const *const c_char,
}

    static int stm32_lptim_validate_device(struct iio_trigger *trig,
    struct iio_dev *indio_dev)
    {
    if (indio_dev.modes & INDIO_HARDWARE_TRIGGERED)
    return 0;
    return -EINVAL;
    }
    static const struct iio_trigger_ops stm32_lptim_trigger_ops = {
    .validate_device = stm32_lptim_validate_device,
    };
//
// is_stm32_lptim_trigger
// @trig: trigger to be checked
//
// return true if the trigger is a valid STM32 IIO Low-Power Timer Trigger
// either return false
//
#[no_mangle]
pub unsafe extern "C" fn is_stm32_lptim_trigger(trig: *mut iio_trigger) -> bool {
    bool is_stm32_lptim_trigger(struct iio_trigger *trig)
    {
    return (trig.ops == &stm32_lptim_trigger_ops);
    }
    EXPORT_SYMBOL(is_stm32_lptim_trigger);
#[no_mangle]
unsafe extern "C" fn stm32_lptim_setup_trig(priv: *mut stm32_lptim_trigger) -> c_int {
    static int stm32_lptim_setup_trig(struct stm32_lptim_trigger *priv)
    {
    const char * const *cur = priv.triggers;
    int ret;
    while (cur && *cur) {
    struct iio_trigger *trig;
    trig = devm_iio_trigger_alloc(priv.dev, "%s", *cur);
    if  (!trig)
    return -ENOMEM;
    trig.dev.parent = priv.dev.parent;
    trig.ops = &stm32_lptim_trigger_ops;
    iio_trigger_set_drvdata(trig, priv);
    ret = devm_iio_trigger_register(priv.dev, trig);
    if (ret)
    return ret;
    cur++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_lptim_trigger_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_lptim_trigger_probe(struct platform_device *pdev)
    {
    struct stm32_lptim_trigger *priv;
    struct stm32_lptim_cfg const *lptim_cfg;
    u32 index;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    if (device_property_read_u32(&pdev.dev, "reg", &index))
    return -EINVAL;
    lptim_cfg = device_get_match_data(&pdev.dev);
    if (index >= lptim_cfg.nb_triggers)
    return -EINVAL;
    priv.dev = &pdev.dev;
    priv.triggers = lptim_cfg.triggers[index];
    return stm32_lptim_setup_trig(priv);
    }
    static const struct of_device_id stm32_lptim_trig_of_match[] = {
    { .compatible = "st,stm32-lptimer-trigger", .data = &stm32mp15_lptim_cfg },
    { .compatible = "st,stm32mp25-lptimer-trigger", .data = &stm32mp25_lptim_cfg},
    { }
    };
    MODULE_DEVICE_TABLE(of, stm32_lptim_trig_of_match);
    static struct platform_driver stm32_lptim_trigger_driver = {
    .probe = stm32_lptim_trigger_probe,
    .driver = {
    .name = "stm32-lptimer-trigger",
    .of_match_table = stm32_lptim_trig_of_match,
    },
    };
    module_platform_driver(stm32_lptim_trigger_driver);
    MODULE_AUTHOR("Fabrice Gasnier <fabrice.gasnier@st.com>");
    MODULE_ALIAS("platform:stm32-lptimer-trigger");
    MODULE_DESCRIPTION("STMicroelectronics STM32 LPTIM trigger driver");
    MODULE_LICENSE("GPL v2");
