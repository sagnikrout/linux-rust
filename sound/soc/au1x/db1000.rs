//! Automatically rewritten from C to Rust
//! Source: sound/soc/au1x/db1000.c
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
// DB1000/DB1500/DB1100 ASoC audio fabric support code.
//
// (c) 2011 Manuel Lauss <manuel.lauss@googlemail.com>
//

    SND_SOC_DAILINK_DEFS(hifi,
    DAILINK_COMP_ARRAY(COMP_CPU("alchemy-ac97c")),
    DAILINK_COMP_ARRAY(COMP_CODEC("ac97-codec", "ac97-hifi")),
    DAILINK_COMP_ARRAY(COMP_PLATFORM("alchemy-pcm-dma.0")));
    static struct snd_soc_dai_link db1000_ac97_dai = {
    .name		= "AC97",
    .stream_name	= "AC97 HiFi",
    SND_SOC_DAILINK_REG(hifi),
    };
    static struct snd_soc_card db1000_ac97 = {
    .name		= "DB1000_AC97",
    .owner		= THIS_MODULE,
    .dai_link	= &db1000_ac97_dai,
    .num_links	= 1,
    };
#[no_mangle]
unsafe extern "C" fn db1000_audio_probe(pdev: *mut platform_device) -> c_int {
    static int db1000_audio_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card = &db1000_ac97;
    card.dev = &pdev.dev;
    return devm_snd_soc_register_card(&pdev.dev, card);
    }
    static struct platform_driver db1000_audio_driver = {
    .driver	= {
    .name	= "db1000-audio",
    .pm	= &snd_soc_pm_ops,
    },
    .probe		= db1000_audio_probe,
    };
    module_platform_driver(db1000_audio_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("DB1000/DB1500/DB1100 ASoC audio");
    MODULE_AUTHOR("Manuel Lauss");
