//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_rt_dmic.c
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
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2024 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.
//
// soc_sdw_rt_dmic - Helpers to handle Realtek SDW DMIC from generic machine driver
//

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_rt_dmic_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_component *component;
    struct sdw_slave *sdw_peripheral = core::ptr::null_mut();
    char *mic_name;
    let mut rt1320_dmic_num: c_int = 0, part_id, i;
    component = dai.component;
//
// rt715-sdca (aka rt714) is a special case that uses different name in card->components
// and component->name_prefix.
//
    if (!strcmp(component.name_prefix, "rt714"))
    mic_name = devm_kasprintf(card.dev, GFP_KERNEL, "rt715-sdca");
    else
    mic_name = devm_kasprintf(card.dev, GFP_KERNEL, "%s", component.name_prefix);
    if (!mic_name)
    return -ENOMEM;
//
// If there is any rt1320/rt1321 DMIC belonging to this card, try to count the `cfg-mics`
// to be used in card->components.
// Note: The rt1320 drivers register the peripheral dev to component->dev, so get the
// sdw_peripheral from component->dev.
//
    if (is_sdw_slave(component.dev))
    sdw_peripheral = dev_to_sdw_dev(component.dev);
    if (sdw_peripheral &&
    (sdw_peripheral.id.part_id == 0x1320 || sdw_peripheral.id.part_id == 0x1321)) {
    part_id = sdw_peripheral.id.part_id;
//
// This rtd init callback is called once, so count the rt1320/rt1321 with SDCA
// function SmartMic type in this card.
//
    for_each_card_components(card, component) {
    if (!is_sdw_slave(component.dev))
    continue;
    sdw_peripheral = dev_to_sdw_dev(component.dev);
    if (sdw_peripheral.id.part_id != part_id)
    continue;
    for (i = 0; i < sdw_peripheral.sdca_data.num_functions; i++) {
    if (sdw_peripheral.sdca_data.function[i].type ==
    SDCA_FUNCTION_TYPE_SMART_MIC) {
    rt1320_dmic_num++;
    break;
    }
    }
    }
    card.components = devm_kasprintf(card.dev, GFP_KERNEL,
    "%s mic:%s cfg-mics:%d", card.components,
    mic_name, rt1320_dmic_num);
    } else {
    card.components = devm_kasprintf(card.dev, GFP_KERNEL,
    "%s mic:%s", card.components,
    mic_name);
    }
    if (!card.components)
    return -ENOMEM;
    dev_dbg(card.dev, "card.components: %s\n", card.components);
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_rt_dmic_rtd_init, "SND_SOC_SDW_UTILS");
