//! Automatically rewritten from C to Rust
//! Source: sound/hda/core/ext/controller.c
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
// hdac-ext-controller.c - HD-audio extended controller functions.
//
// Copyright (C) 2014-2015 Intel Corp
// Author: Jeeja KP <jeeja.kp@intel.com>
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//

//
// processing pipe helpers - these helpers are useful for dealing with HDA
// new capability of processing pipelines
//
// snd_hdac_ext_bus_ppcap_enable - enable/disable processing pipe capability
// @bus: the pointer to HDAC bus object
// @enable: flag to turn on/off the capability
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_ppcap_enable(bus: *mut hdac_bus, enable: bool) {
    void snd_hdac_ext_bus_ppcap_enable(struct hdac_bus *bus, bool enable)
    {
    if (!bus.ppcap) {
    dev_err(bus.dev, "Address of PP capability is core::ptr::null_mut()");
    return;
    }
    if (enable)
    snd_hdac_updatel(bus.ppcap, AZX_REG_PP_PPCTL,
    AZX_PPCTL_GPROCEN, AZX_PPCTL_GPROCEN);
    else
    snd_hdac_updatel(bus.ppcap, AZX_REG_PP_PPCTL,
    AZX_PPCTL_GPROCEN, 0);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_ppcap_enable);
//
// snd_hdac_ext_bus_ppcap_int_enable - ppcap interrupt enable/disable
// @bus: the pointer to HDAC bus object
// @enable: flag to enable/disable interrupt
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_ppcap_int_enable(bus: *mut hdac_bus, enable: bool) {
    void snd_hdac_ext_bus_ppcap_int_enable(struct hdac_bus *bus, bool enable)
    {
    if (!bus.ppcap) {
    dev_err(bus.dev, "Address of PP capability is core::ptr::null_mut()\n");
    return;
    }
    if (enable)
    snd_hdac_updatel(bus.ppcap, AZX_REG_PP_PPCTL,
    AZX_PPCTL_PIE, AZX_PPCTL_PIE);
    else
    snd_hdac_updatel(bus.ppcap, AZX_REG_PP_PPCTL,
    AZX_PPCTL_PIE, 0);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_ppcap_int_enable);
//
// Multilink helpers - these helpers are useful for dealing with HDA
// new multilink capability
//
// snd_hdac_ext_bus_get_ml_capabilities - get multilink capability
// @bus: the pointer to HDAC bus object
//
// This will parse all links and read the mlink capabilities and add them
// in hlink_list of extended hdac bus
// Note: this will be freed on bus exit by driver
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_get_ml_capabilities(bus: *mut hdac_bus) -> c_int {
    int snd_hdac_ext_bus_get_ml_capabilities(struct hdac_bus *bus)
    {
    int idx;
    u32 link_count;
    struct hdac_ext_link *hlink;
    u32 leptr;
    link_count = readl(bus.mlcap + AZX_REG_ML_MLCD) + 1;
    dev_dbg(bus.dev, "In %s Link count: %d\n", __func__, link_count);
    for (idx = 0; idx < link_count; idx++) {
    hlink = kzalloc_obj(*hlink);
    if (!hlink)
    return -ENOMEM;
    hlink.index = idx;
    hlink.bus = bus;
    hlink.ml_addr = bus.mlcap + AZX_ML_BASE +
    (AZX_ML_INTERVAL * idx);
    hlink.lcaps  = readl(hlink.ml_addr + AZX_REG_ML_LCAP);
    hlink.lsdiid = readw(hlink.ml_addr + AZX_REG_ML_LSDIID);
    hlink.slcount = FIELD_GET(AZX_ML_HDA_LCAP_SLCOUNT, hlink.lcaps) + 1;
    if (hdac_ext_link_alt(hlink)) {
    leptr = readl(hlink.ml_addr + AZX_REG_ML_LEPTR);
    hlink.id = FIELD_GET(AZX_REG_ML_LEPTR_ID, leptr);
    }
// since link in On, update the ref
    hlink.ref_count = 1;
    list_add_tail(&hlink.list, &bus.hlink_list);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_get_ml_capabilities);
//
// snd_hdac_ext_link_free_all- free hdac extended link objects
//
// @bus: the pointer to HDAC bus object
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_link_free_all(bus: *mut hdac_bus) {
    void snd_hdac_ext_link_free_all(struct hdac_bus *bus)
    {
    struct hdac_ext_link *hlink;
    while (!list_empty(&bus.hlink_list)) {
    hlink = list_first_entry(&bus.hlink_list, struct hdac_ext_link, list);
    list_del(&hlink.list);
    kfree(hlink);
    }
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_link_free_all);
    struct hdac_ext_link *snd_hdac_ext_bus_get_hlink_by_id(struct hdac_bus *bus, u32 id)
    {
    struct hdac_ext_link *hlink;
    list_for_each_entry(hlink, &bus.hlink_list, list)
    if (hdac_ext_link_alt(hlink) && hlink.id == id)
    return hlink;
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_get_hlink_by_id);
//
// snd_hdac_ext_bus_get_hlink_by_addr - get hlink at specified address
// @bus: hlink's parent bus device
// @addr: codec device address
//
// Returns hlink object or NULL if matching hlink is not found.
//
    struct hdac_ext_link *snd_hdac_ext_bus_get_hlink_by_addr(struct hdac_bus *bus, int addr)
    {
    struct hdac_ext_link *hlink;
    list_for_each_entry(hlink, &bus.hlink_list, list)
    if (hlink.lsdiid & (0x1 << addr))
    return hlink;
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_get_hlink_by_addr);
//
// snd_hdac_ext_bus_get_hlink_by_name - get hlink based on codec name
// @bus: the pointer to HDAC bus object
// @codec_name: codec name
//
    struct hdac_ext_link *snd_hdac_ext_bus_get_hlink_by_name(struct hdac_bus *bus,
    const char *codec_name)
    {
    int bus_idx, addr;
    if (sscanf(codec_name, "ehdaudio%dD%d", &bus_idx, &addr) != 2)
    return core::ptr::null_mut();
    if (bus.idx != bus_idx)
    return core::ptr::null_mut();
    if (addr < 0 || addr > 31)
    return core::ptr::null_mut();
    return snd_hdac_ext_bus_get_hlink_by_addr(bus, addr);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_get_hlink_by_name);
#[no_mangle]
unsafe extern "C" fn check_hdac_link_power_active(hlink: *mut hdac_ext_link, enable: bool) -> c_int {
    static int check_hdac_link_power_active(struct hdac_ext_link *hlink, bool enable)
    {
    int timeout;
    u32 val;
    let mut mask: c_int = (1 << AZX_ML_LCTL_CPA_SHIFT);
    udelay(3);
    timeout = 150;
    do {
    val = readl(hlink.ml_addr + AZX_REG_ML_LCTL);
    if (enable) {
    if (((val & mask) >> AZX_ML_LCTL_CPA_SHIFT))
    return 0;
    } else {
    if (!((val & mask) >> AZX_ML_LCTL_CPA_SHIFT))
    return 0;
    }
    udelay(3);
    } while (--timeout);
    return -EIO;
    }
//
// snd_hdac_ext_bus_link_power_up -power up hda link
// @hlink: HD-audio extended link
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power_up(hlink: *mut hdac_ext_link) -> c_int {
    int snd_hdac_ext_bus_link_power_up(struct hdac_ext_link *hlink)
    {
    snd_hdac_updatel(hlink.ml_addr, AZX_REG_ML_LCTL,
    AZX_ML_LCTL_SPA, AZX_ML_LCTL_SPA);
    return check_hdac_link_power_active(hlink, true);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_link_power_up);
//
// snd_hdac_ext_bus_link_power_down -power down hda link
// @hlink: HD-audio extended link
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power_down(hlink: *mut hdac_ext_link) -> c_int {
    int snd_hdac_ext_bus_link_power_down(struct hdac_ext_link *hlink)
    {
    snd_hdac_updatel(hlink.ml_addr, AZX_REG_ML_LCTL, AZX_ML_LCTL_SPA, 0);
    return check_hdac_link_power_active(hlink, false);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_link_power_down);
//
// snd_hdac_ext_bus_link_power_up_all -power up all hda link
// @bus: the pointer to HDAC bus object
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power_up_all(bus: *mut hdac_bus) -> c_int {
    int snd_hdac_ext_bus_link_power_up_all(struct hdac_bus *bus)
    {
    struct hdac_ext_link *hlink = core::ptr::null_mut();
    int ret;
    list_for_each_entry(hlink, &bus.hlink_list, list) {
    ret = snd_hdac_ext_bus_link_power_up(hlink);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_link_power_up_all);
//
// snd_hdac_ext_bus_link_power_down_all -power down all hda link
// @bus: the pointer to HDAC bus object
//
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power_down_all(bus: *mut hdac_bus) -> c_int {
    int snd_hdac_ext_bus_link_power_down_all(struct hdac_bus *bus)
    {
    struct hdac_ext_link *hlink = core::ptr::null_mut();
    int ret;
    list_for_each_entry(hlink, &bus.hlink_list, list) {
    ret = snd_hdac_ext_bus_link_power_down(hlink);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_link_power_down_all);
//
// snd_hdac_ext_bus_link_set_stream_id - maps stream id to link output
// @link: HD-audio ext link to set up
// @stream: stream id
//
    void snd_hdac_ext_bus_link_set_stream_id(struct hdac_ext_link *link,
    int stream)
    {
    snd_hdac_updatew(link.ml_addr, AZX_REG_ML_LOSIDV, (1 << stream), 1 << stream);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_link_set_stream_id);
//
// snd_hdac_ext_bus_link_clear_stream_id - maps stream id to link output
// @link: HD-audio ext link to set up
// @stream: stream id
//
    void snd_hdac_ext_bus_link_clear_stream_id(struct hdac_ext_link *link,
    int stream)
    {
    snd_hdac_updatew(link.ml_addr, AZX_REG_ML_LOSIDV, (1 << stream), 0);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_link_clear_stream_id);
    int snd_hdac_ext_bus_link_get(struct hdac_bus *bus,
    struct hdac_ext_link *hlink)
    {
    unsigned long codec_mask;
    let mut ret: c_int = 0;
    guard(mutex)(&bus.lock);
//
// if we move from 0 to 1, count will be 1 so power up this link
// as well, also check the dma status and trigger that
//
    if (++hlink.ref_count == 1) {
    if (!bus.cmd_dma_state) {
    snd_hdac_bus_init_cmd_io(bus);
    bus.cmd_dma_state = true;
    }
    ret = snd_hdac_ext_bus_link_power_up(hlink);
//
// clear the register to invalidate all the output streams
//
    snd_hdac_updatew(hlink.ml_addr, AZX_REG_ML_LOSIDV,
    AZX_ML_LOSIDV_STREAM_MASK, 0);
//
// wait for 521usec for codec to report status
// HDA spec section 4.3 - Codec Discovery
//
    udelay(521);
    codec_mask = snd_hdac_chip_readw(bus, STATESTS);
    dev_dbg(bus.dev, "codec_mask = 0x%lx\n", codec_mask);
    snd_hdac_chip_writew(bus, STATESTS, codec_mask);
    if (!bus.codec_mask)
    bus.codec_mask = codec_mask;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_link_get);
    int snd_hdac_ext_bus_link_put(struct hdac_bus *bus,
    struct hdac_ext_link *hlink)
    {
    let mut ret: c_int = 0;
    struct hdac_ext_link *hlink_tmp;
    let mut link_up: bool = false;
    guard(mutex)(&bus.lock);
//
// if we move from 1 to 0, count will be 0
// so power down this link as well
//
    if (--hlink.ref_count == 0) {
    ret = snd_hdac_ext_bus_link_power_down(hlink);
//
// now check if all links are off, if so turn off
// cmd dma as well
//
    list_for_each_entry(hlink_tmp, &bus.hlink_list, list) {
    if (hlink_tmp.ref_count) {
    link_up = true;
    break;
    }
    }
    if (!link_up) {
    snd_hdac_bus_stop_cmd_io(bus);
    bus.cmd_dma_state = false;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_link_put);
#[no_mangle]
unsafe extern "C" fn hdac_ext_codec_link_up(codec: *mut hdac_device) {
    static void hdac_ext_codec_link_up(struct hdac_device *codec)
    {
    const char *devname = dev_name(&codec.dev);
    struct hdac_ext_link *hlink =
    snd_hdac_ext_bus_get_hlink_by_name(codec.bus, devname);
    if (hlink)
    snd_hdac_ext_bus_link_get(codec.bus, hlink);
    }
#[no_mangle]
unsafe extern "C" fn hdac_ext_codec_link_down(codec: *mut hdac_device) {
    static void hdac_ext_codec_link_down(struct hdac_device *codec)
    {
    const char *devname = dev_name(&codec.dev);
    struct hdac_ext_link *hlink =
    snd_hdac_ext_bus_get_hlink_by_name(codec.bus, devname);
    if (hlink)
    snd_hdac_ext_bus_link_put(codec.bus, hlink);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power(codec: *mut hdac_device, enable: bool) {
    void snd_hdac_ext_bus_link_power(struct hdac_device *codec, bool enable)
    {
    struct hdac_bus *bus = codec.bus;
    let mut oldstate: bool = test_bit(codec.addr, &bus.codec_powered);
    if (enable == oldstate)
    return;
    snd_hdac_bus_link_power(codec, enable);
    if (enable)
    hdac_ext_codec_link_up(codec);
    else
    hdac_ext_codec_link_down(codec);
    }
    EXPORT_SYMBOL_GPL(snd_hdac_ext_bus_link_power);
