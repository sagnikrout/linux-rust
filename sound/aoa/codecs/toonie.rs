//! Automatically rewritten from C to Rust
//! Source: sound/aoa/codecs/toonie.c
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
// Apple Onboard Audio driver for Toonie codec
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//
// This is a driver for the toonie codec chip. This chip is present
// on the Mac Mini and is nothing but a DAC.
//

    MODULE_AUTHOR("Johannes Berg <johannes@sipsolutions.net>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("toonie codec driver for snd-aoa");

#[repr(C)]
#[derive(Copy, Clone)]
pub struct toonie {
    pub codec: aoa_codec,
}

#[no_mangle]
unsafe extern "C" fn toonie_dev_register(dev: *mut snd_device) -> c_int {
    static int toonie_dev_register(struct snd_device *dev)
    {
    return 0;
    }
    static const struct snd_device_ops ops = {
    .dev_register = toonie_dev_register,
    };
    static struct transfer_info toonie_transfers[] = {
// This thing *only* has analog output,
// the rates are taken from Info.plist
// from Darwin.
    {
    .formats = SNDRV_PCM_FMTBIT_S16_BE |
    SNDRV_PCM_FMTBIT_S24_BE,
    .rates = SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_44100 |
    SNDRV_PCM_RATE_48000 |
    SNDRV_PCM_RATE_88200 |
    SNDRV_PCM_RATE_96000,
    },
    {}
    };
    static int toonie_usable(struct codec_info_item *cii,
    struct transfer_info *ti,
    struct transfer_info *out)
    {
    return 1;
    }

#[no_mangle]
unsafe extern "C" fn toonie_suspend(cii: *mut codec_info_item, state: pm_message_t) -> c_int {
    static int toonie_suspend(struct codec_info_item *cii, pm_message_t state)
    {
// can we turn it off somehow?
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn toonie_resume(cii: *mut codec_info_item) -> c_int {
    static int toonie_resume(struct codec_info_item *cii)
    {
    return 0;
    }

    static struct codec_info toonie_codec_info = {
    .transfers = toonie_transfers,
    .sysclock_factor = 256,
    .bus_factor = 64,
    .owner = THIS_MODULE,
    .usable = toonie_usable,

    .suspend = toonie_suspend,
    .resume = toonie_resume,

    };
#[no_mangle]
unsafe extern "C" fn toonie_init_codec(codec: *mut aoa_codec) -> c_int {
    static int toonie_init_codec(struct aoa_codec *codec)
    {
    struct toonie *toonie = codec_to_toonie(codec);
// nothing connected? what a joke!
    if (toonie.codec.connected != 1)
    return -ENOTCONN;
    if (aoa_snd_device_new(SNDRV_DEV_CODEC, toonie, &ops)) {
    printk(KERN_ERR PFX "failed to create toonie snd device!\n");
    return -ENODEV;
    }
    if (toonie.codec.soundbus_dev.attach_codec(toonie.codec.soundbus_dev,
    aoa_get_card(),
    &toonie_codec_info, toonie)) {
    printk(KERN_ERR PFX "error creating toonie pcm\n");
    snd_device_free(aoa_get_card(), toonie);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn toonie_exit_codec(codec: *mut aoa_codec) {
    static void toonie_exit_codec(struct aoa_codec *codec)
    {
    struct toonie *toonie = codec_to_toonie(codec);
    if (!toonie.codec.soundbus_dev) {
    printk(KERN_ERR PFX "toonie_exit_codec called without soundbus_dev!\n");
    return;
    }
    toonie.codec.soundbus_dev.detach_codec(toonie.codec.soundbus_dev, toonie);
    }
    static struct toonie *toonie;
#[no_mangle]
unsafe extern "C" fn toonie_init() -> int __init {
    static int __init toonie_init(void)
    {
    toonie = kzalloc_obj(struct toonie);
    if (!toonie)
    return -ENOMEM;
    strscpy(toonie.codec.name, "toonie");
    toonie.codec.owner = THIS_MODULE;
    toonie.codec.init = toonie_init_codec;
    toonie.codec.exit = toonie_exit_codec;
    if (aoa_codec_register(&toonie.codec)) {
    kfree(toonie);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn toonie_exit() -> void __exit {
    static void __exit toonie_exit(void)
    {
    aoa_codec_unregister(&toonie.codec);
    kfree(toonie);
    }
    module_init(toonie_init);
    module_exit(toonie_exit);
