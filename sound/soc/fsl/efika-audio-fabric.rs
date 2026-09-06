//! Automatically rewritten from C to Rust
//! Source: sound/soc/fsl/efika-audio-fabric.c
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
// Efika driver for the PSC of the Freescale MPC52xx
// configured as AC97 interface
//
// Copyright 2008 Jon Smirl, Digispeaker
// Author: Jon Smirl <jonsmirl@gmail.com>
//

    SND_SOC_DAILINK_DEFS(analog,
    DAILINK_COMP_ARRAY(COMP_CPU("mpc5200-psc-ac97.0")),
    DAILINK_COMP_ARRAY(COMP_CODEC("stac9766-codec",
    "stac9766-hifi-analog")),
    DAILINK_COMP_ARRAY(COMP_PLATFORM("mpc5200-pcm-audio")));
    SND_SOC_DAILINK_DEFS(iec958,
    DAILINK_COMP_ARRAY(COMP_CPU("mpc5200-psc-ac97.1")),
    DAILINK_COMP_ARRAY(COMP_CODEC("stac9766-codec",
    "stac9766-hifi-IEC958")),
    DAILINK_COMP_ARRAY(COMP_PLATFORM("mpc5200-pcm-audio")));
    static struct snd_soc_dai_link efika_fabric_dai[] = {
    {
    .name = "AC97",
    .stream_name = "AC97 Analog",
    SND_SOC_DAILINK_REG(analog),
    },
    {
    .name = "AC97",
    .stream_name = "AC97 IEC958",
    SND_SOC_DAILINK_REG(iec958),
    },
    };
    static struct snd_soc_card card = {
    .name = "Efika",
    .owner = THIS_MODULE,
    .dai_link = efika_fabric_dai,
    .num_links = ARRAY_SIZE(efika_fabric_dai),
    };
#[no_mangle]
unsafe extern "C" fn efika_fabric_init() -> __init int {
    static __init int efika_fabric_init(void)
    {
    struct platform_device *pdev;
    int rc;
    if (!of_machine_is_compatible("bplan,efika"))
    return -ENODEV;
    pdev = platform_device_alloc("soc-audio", 1);
    if (!pdev) {
    pr_err("efika_fabric_init: platform_device_alloc() failed\n");
    return -ENODEV;
    }
    platform_set_drvdata(pdev, &card);
    rc = platform_device_add(pdev);
    if (rc) {
    pr_err("efika_fabric_init: platform_device_add() failed\n");
    platform_device_put(pdev);
    return -ENODEV;
    }
    return 0;
    }
    module_init(efika_fabric_init);
    MODULE_AUTHOR("Jon Smirl <jonsmirl@gmail.com>");
    MODULE_DESCRIPTION(DRV_NAME ": mpc5200 Efika fabric driver");
    MODULE_LICENSE("GPL");
