//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ssm2305.c
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
// Analog Devices SSM2305 Amplifier Driver
//
// Copyright (C) 2018 Pengutronix, Marco Felsch <kernel@pengutronix.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssm2305 {
// shutdown gpio
    pub gpiod_shutdown: *mut gpio_desc,
}

    static int ssm2305_power_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kctrl, int event)
    {
    struct snd_soc_component *c = snd_soc_dapm_to_component(w.dapm);
    struct ssm2305 *data = snd_soc_component_get_drvdata(c);
    gpiod_set_value_cansleep(data.gpiod_shutdown,
    SND_SOC_DAPM_EVENT_ON(event));
    return 0;
    }
    static const struct snd_soc_dapm_widget ssm2305_dapm_widgets[] = {
// Stereo input/output
    SND_SOC_DAPM_INPUT("L_IN"),
    SND_SOC_DAPM_INPUT("R_IN"),
    SND_SOC_DAPM_OUTPUT("L_OUT"),
    SND_SOC_DAPM_OUTPUT("R_OUT"),
    SND_SOC_DAPM_SUPPLY("Power", SND_SOC_NOPM, 0, 0, ssm2305_power_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    };
    static const struct snd_soc_dapm_route ssm2305_dapm_routes[] = {
    { "L_OUT", core::ptr::null_mut(), "L_IN" },
    { "R_OUT", core::ptr::null_mut(), "R_IN" },
    { "L_IN", core::ptr::null_mut(), "Power" },
    { "R_IN", core::ptr::null_mut(), "Power" },
    };
    static const struct snd_soc_component_driver ssm2305_component_driver = {
    .dapm_widgets		= ssm2305_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(ssm2305_dapm_widgets),
    .dapm_routes		= ssm2305_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(ssm2305_dapm_routes),
    };
#[no_mangle]
unsafe extern "C" fn ssm2305_probe(pdev: *mut platform_device) -> c_int {
    static int ssm2305_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ssm2305 *priv;
// Allocate the private data
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
// Get shutdown gpio
    priv.gpiod_shutdown = devm_gpiod_get(dev, "shutdown",
    GPIOD_OUT_LOW);
    if (IS_ERR(priv.gpiod_shutdown))
    return dev_err_probe(dev, PTR_ERR(priv.gpiod_shutdown),
    "Failed to get 'shutdown' gpio\n");
    return devm_snd_soc_register_component(dev, &ssm2305_component_driver,
    core::ptr::null_mut(), 0);
    }

    static const struct of_device_id ssm2305_of_match[] = {
    { .compatible = "adi,ssm2305", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ssm2305_of_match);

    static struct platform_driver ssm2305_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = of_match_ptr(ssm2305_of_match),
    },
    .probe = ssm2305_probe,
    };
    module_platform_driver(ssm2305_driver);
    MODULE_DESCRIPTION("ASoC SSM2305 amplifier driver");
    MODULE_AUTHOR("Marco Felsch <m.felsch@pengutronix.de>");
    MODULE_LICENSE("GPL v2");
