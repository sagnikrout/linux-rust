//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/intel/hda-bus.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Keyon Jie <yang.jie@linux.intel.com>

#[no_mangle]
unsafe extern "C" fn update_codec_wake_enable(bus: *mut hdac_bus, addr: c_uint, link_power: bool) {
    static void update_codec_wake_enable(struct hdac_bus *bus, unsigned int addr, bool link_power)
    {
    let mut mask: c_uint = snd_hdac_chip_readw(bus, WAKEEN);
    if (link_power)
    mask &= ~BIT(addr);
    else
    mask |= BIT(addr);
    snd_hdac_chip_updatew(bus, WAKEEN, STATESTS_INT_MASK, mask);
    }
#[no_mangle]
unsafe extern "C" fn sof_hda_bus_link_power(codec: *mut hdac_device, enable: bool) {
    static void sof_hda_bus_link_power(struct hdac_device *codec, bool enable)
    {
    struct hdac_bus *bus = codec.bus;
    let mut oldstate: bool = test_bit(codec.addr, &bus.codec_powered);
    snd_hdac_ext_bus_link_power(codec, enable);
    if (enable == oldstate)
    return;
//
// Both codec driver and controller can hold references to
// display power. To avoid unnecessary power-up/down cycles,
// controller doesn't immediately release its reference.
//
// If the codec driver powers down the link, release
// the controller reference as well.
//
    if (codec.addr == HDA_IDISP_ADDR && !enable)
    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, false);
// WAKEEN needs to be set for disabled links
    update_codec_wake_enable(bus, codec.addr, enable);
    }
    static const struct hdac_bus_ops bus_core_ops = {
    .command = snd_hdac_bus_send_cmd,
    .get_response = snd_hdac_bus_get_response,
    .link_power = sof_hda_bus_link_power,
    };

//
// This can be used for both with/without hda link support.
//
#[no_mangle]
pub unsafe extern "C" fn sof_hda_bus_init(sdev: *mut snd_sof_dev, dev: *mut device) {
    void sof_hda_bus_init(struct snd_sof_dev *sdev, struct device *dev)
    {
    struct hdac_bus *bus = sof_to_bus(sdev);

    const struct sof_intel_dsp_desc *chip = get_chip_info(sdev.pdata);
    snd_hdac_ext_bus_init(bus, dev, &bus_core_ops, sof_hda_ext_ops);
    if (chip && chip.hw_ip_version >= SOF_INTEL_ACE_2_0)
    bus.use_pio_for_commands = true;

    snd_hdac_ext_bus_init(bus, dev, core::ptr::null_mut(), core::ptr::null_mut());

    memset(bus, 0, sizeof(*bus));
    bus.dev = dev;
    INIT_LIST_HEAD(&bus.stream_list);
    bus.irq = -1;
//
// There is only one HDA bus atm. keep the index as 0.
// Need to fix when there are more than one HDA bus.
//
    bus.idx = 0;
    spin_lock_init(&bus.reg_lock);

    }
    EXPORT_SYMBOL_NS(sof_hda_bus_init, "SND_SOC_SOF_INTEL_HDA_COMMON");
#[no_mangle]
pub unsafe extern "C" fn sof_hda_bus_exit(sdev: *mut snd_sof_dev) {
    void sof_hda_bus_exit(struct snd_sof_dev *sdev)
    {

    struct hdac_bus *bus = sof_to_bus(sdev);
    snd_hdac_ext_bus_exit(bus);

    }
    EXPORT_SYMBOL_NS(sof_hda_bus_exit, "SND_SOC_SOF_INTEL_HDA_COMMON");
