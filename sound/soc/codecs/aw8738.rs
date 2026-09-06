//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/aw8738.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw8738_priv {
    pub gpiod_mode: *mut gpio_desc,
    pub mode: c_uint,
}

    static int aw8738_drv_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *c = snd_soc_dapm_to_component(w.dapm);
    struct aw8738_priv *aw = snd_soc_component_get_drvdata(c);
    int i;
    switch (event) {
    case SND_SOC_DAPM_POST_PMU:
    for (i = 0; i < aw.mode; i++) {
    gpiod_set_value_cansleep(aw.gpiod_mode, 0);
    udelay(2);
    gpiod_set_value_cansleep(aw.gpiod_mode, 1);
    udelay(2);
    }
    msleep(40);
    break;
    case SND_SOC_DAPM_PRE_PMD:
    gpiod_set_value_cansleep(aw.gpiod_mode, 0);
    usleep_range(1000, 2000);
    break;
    default:
    WARN(1, "Unexpected event");
    return -EINVAL;
    }
    return 0;
    }
    static const struct snd_soc_dapm_widget aw8738_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("IN"),
    SND_SOC_DAPM_OUT_DRV_E("DRV", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0, aw8738_drv_event,
    SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_OUTPUT("OUT"),
    };
    static const struct snd_soc_dapm_route aw8738_dapm_routes[] = {
    { "DRV", core::ptr::null_mut(), "IN" },
    { "OUT", core::ptr::null_mut(), "DRV" },
    };
    static const struct snd_soc_component_driver aw8738_component_driver = {
    .dapm_widgets = aw8738_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(aw8738_dapm_widgets),
    .dapm_routes = aw8738_dapm_routes,
    .num_dapm_routes = ARRAY_SIZE(aw8738_dapm_routes),
    };
#[no_mangle]
unsafe extern "C" fn aw8738_probe(pdev: *mut platform_device) -> c_int {
    static int aw8738_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct aw8738_priv *aw;
    int ret;
    aw = devm_kzalloc(dev, sizeof(*aw), GFP_KERNEL);
    if (!aw)
    return -ENOMEM;
    platform_set_drvdata(pdev, aw);
    aw.gpiod_mode = devm_gpiod_get(dev, "mode", GPIOD_OUT_LOW);
    if (IS_ERR(aw.gpiod_mode))
    return dev_err_probe(dev, PTR_ERR(aw.gpiod_mode),
    "Failed to get 'mode' gpio");
    ret = device_property_read_u32(dev, "awinic,mode", &aw.mode);
    if (ret)
    return -EINVAL;
    return devm_snd_soc_register_component(&pdev.dev,
    &aw8738_component_driver,
    core::ptr::null_mut(), 0);
    }

    static const struct of_device_id aw8738_of_match[] = {
    { .compatible = "awinic,aw8738" },
    { }
    };
    MODULE_DEVICE_TABLE(of, aw8738_of_match);

    static struct platform_driver aw8738_driver = {
    .probe	= aw8738_probe,
    .driver = {
    .name = "aw8738",
    .of_match_table = of_match_ptr(aw8738_of_match),
    },
    };
    module_platform_driver(aw8738_driver);
    MODULE_DESCRIPTION("Awinic AW8738 Amplifier Driver");
    MODULE_LICENSE("GPL v2");
