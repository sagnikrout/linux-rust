//! Automatically rewritten from C to Rust
//! Source: sound/soc/soc-devres.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// soc-devres.c  --  ALSA SoC Audio Layer devres functions
//
// Copyright (C) 2013 Linaro Ltd

#[no_mangle]
unsafe extern "C" fn devm_component_release(dev: *mut device, res: *mut c_void) {
    static void devm_component_release(struct device *dev, void *res)
    {
    const struct snd_soc_component_driver **cmpnt_drv = res;
    snd_soc_unregister_component_by_driver(dev, *cmpnt_drv);
    }
//
// devm_snd_soc_register_component - resource managed component registration
// @dev: Device used to manage component
// @cmpnt_drv: Component driver
// @dai_drv: DAI driver
// @num_dai: Number of DAIs to register
//
// Register a component with automatic unregistration when the device is
// unregistered.
//
    int devm_snd_soc_register_component(struct device *dev,
    const struct snd_soc_component_driver *cmpnt_drv,
    struct snd_soc_dai_driver *dai_drv, int num_dai)
    {
    const struct snd_soc_component_driver **ptr;
    int ret;
    ptr = devres_alloc(devm_component_release, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return -ENOMEM;
    ret = snd_soc_register_component(dev, cmpnt_drv, dai_drv, num_dai);
    if (ret == 0) {
// ptr = cmpnt_drv;
    devres_add(dev, ptr);
    } else {
    devres_free(ptr);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_snd_soc_register_component);
//
// devm_snd_soc_register_card - resource managed card registration
// @dev: Device used to manage card
// @card: Card to register
//
// Register a card with automatic unregistration when the device is
// unregistered.
//
#[no_mangle]
pub unsafe extern "C" fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int {
    int devm_snd_soc_register_card(struct device *dev, struct snd_soc_card *card)
    {
    card.devres_dev = dev;
    return snd_soc_register_card(card);
    }
    EXPORT_SYMBOL_GPL(devm_snd_soc_register_card);

#[no_mangle]
unsafe extern "C" fn devm_dmaengine_pcm_release(dev: *mut device, res: *mut c_void) {
    static void devm_dmaengine_pcm_release(struct device *dev, void *res)
    {
    snd_dmaengine_pcm_unregister(*(struct device **)res);
    }
//
// devm_snd_dmaengine_pcm_register - resource managed dmaengine PCM registration
// @dev: The parent device for the PCM device
// @config: Platform specific PCM configuration
// @flags: Platform specific quirks
//
// Register a dmaengine based PCM device with automatic unregistration when the
// device is unregistered.
//
    int devm_snd_dmaengine_pcm_register(struct device *dev,
    const struct snd_dmaengine_pcm_config *config, unsigned int flags)
    {
    struct device **ptr;
    int ret;
    ptr = devres_alloc(devm_dmaengine_pcm_release, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return -ENOMEM;
    ret = snd_dmaengine_pcm_register(dev, config, flags);
    if (ret == 0) {
// ptr = dev;
    devres_add(dev, ptr);
    } else {
    devres_free(ptr);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(devm_snd_dmaengine_pcm_register);
