//! Automatically rewritten from C to Rust
//! Source: drivers/iio/multiplexer/iio-mux.c
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
// IIO multiplexer driver
//
// Copyright (C) 2017 Axentia Technologies AB
//
// Author: Peter Rosin <peda@axentia.se>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_ext_info_cache {
    pub data: *mut c_char,
    pub size: isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_child {
    pub ext_info_cache: *mut mux_ext_info_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux {
    pub cached_state: c_int,
    pub control: *mut mux_control,
    pub parent: *mut iio_channel,
    pub chan: *mut iio_chan_spec,
    pub ext_info: *mut iio_chan_spec_ext_info,
    pub child: *mut mux_child,
    pub delay_us: u32,
}

#[no_mangle]
unsafe extern "C" fn iio_mux_select(mux: *mut mux, idx: c_int) -> c_int {
    static int iio_mux_select(struct mux *mux, int idx)
    {
    struct mux_child *child = &mux.child[idx];
    struct iio_chan_spec const *chan = &mux.chan[idx];
    int ret;
    int i;
    ret = mux_control_select_delay(mux.control, chan.channel,
    mux.delay_us);
    if (ret < 0) {
    mux.cached_state = -1;
    return ret;
    }
    if (mux.cached_state == chan.channel)
    return 0;
    if (chan.ext_info) {
    for (i = 0; chan.ext_info[i].name; ++i) {
    const char *attr = chan.ext_info[i].name;
    struct mux_ext_info_cache *cache;
    cache = &child.ext_info_cache[i];
    if (cache.size < 0)
    continue;
    ret = iio_write_channel_ext_info(mux.parent, attr,
    cache.data,
    cache.size);
    if (ret < 0) {
    mux_control_deselect(mux.control);
    mux.cached_state = -1;
    return ret;
    }
    }
    }
    mux.cached_state = chan.channel;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iio_mux_deselect(mux: *mut mux) {
    static void iio_mux_deselect(struct mux *mux)
    {
    mux_control_deselect(mux.control);
    }
    static int mux_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct mux *mux = iio_priv(indio_dev);
    let mut idx: c_int = chan - mux.chan;
    int ret;
    ret = iio_mux_select(mux, idx);
    if (ret < 0)
    return ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = iio_read_channel_raw(mux.parent, val);
    break;
    case IIO_CHAN_INFO_SCALE:
    ret = iio_read_channel_scale(mux.parent, val, val2);
    break;
    default:
    ret = -EINVAL;
    }
    iio_mux_deselect(mux);
    return ret;
    }
    static int mux_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    struct mux *mux = iio_priv(indio_dev);
    let mut idx: c_int = chan - mux.chan;
    int ret;
    ret = iio_mux_select(mux, idx);
    if (ret < 0)
    return ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
// type = IIO_VAL_INT;
    ret = iio_read_avail_channel_raw(mux.parent, vals, length);
    break;
    default:
    ret = -EINVAL;
    }
    iio_mux_deselect(mux);
    return ret;
    }
    static int mux_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct mux *mux = iio_priv(indio_dev);
    let mut idx: c_int = chan - mux.chan;
    int ret;
    ret = iio_mux_select(mux, idx);
    if (ret < 0)
    return ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = iio_write_channel_raw(mux.parent, val);
    break;
    default:
    ret = -EINVAL;
    }
    iio_mux_deselect(mux);
    return ret;
    }
    static const struct iio_info mux_info = {
    .read_raw = mux_read_raw,
    .read_avail = mux_read_avail,
    .write_raw = mux_write_raw,
    };
    static ssize_t mux_read_ext_info(struct iio_dev *indio_dev, uintptr_t private,
    struct iio_chan_spec const *chan, char *buf)
    {
    struct mux *mux = iio_priv(indio_dev);
    let mut idx: c_int = chan - mux.chan;
    ssize_t ret;
    ret = iio_mux_select(mux, idx);
    if (ret < 0)
    return ret;
    ret = iio_read_channel_ext_info(mux.parent,
    mux.ext_info[private].name,
    buf);
    iio_mux_deselect(mux);
    return ret;
    }
    static ssize_t mux_write_ext_info(struct iio_dev *indio_dev, uintptr_t private,
    struct iio_chan_spec const *chan,
    const char *buf, size_t len)
    {
    struct device *dev = indio_dev.dev.parent;
    struct mux *mux = iio_priv(indio_dev);
    let mut idx: c_int = chan - mux.chan;
    char *new;
    ssize_t ret;
    if (len >= PAGE_SIZE)
    return -EINVAL;
    ret = iio_mux_select(mux, idx);
    if (ret < 0)
    return ret;
    new = devm_kmemdup(dev, buf, len + 1, GFP_KERNEL);
    if (!new) {
    iio_mux_deselect(mux);
    return -ENOMEM;
    }
    new[len] = 0;
    ret = iio_write_channel_ext_info(mux.parent,
    mux.ext_info[private].name,
    buf, len);
    if (ret < 0) {
    iio_mux_deselect(mux);
    devm_kfree(dev, new);
    return ret;
    }
    devm_kfree(dev, mux.child[idx].ext_info_cache[private].data);
    mux.child[idx].ext_info_cache[private].data = new;
    mux.child[idx].ext_info_cache[private].size = len;
    iio_mux_deselect(mux);
    return ret;
    }
    static int mux_configure_chan_ext_info(struct device *dev, struct mux *mux,
    int idx, int num_ext_info)
    {
    struct mux_child *child = &mux.child[idx];
    struct iio_chan_spec const *pchan = mux.parent.channel;
    int i;
    int ret;
    char *page __free(kfree) = kzalloc(PAGE_SIZE, GFP_KERNEL);
    if (!page)
    return -ENOMEM;
    child.ext_info_cache = devm_kcalloc(dev,
    num_ext_info,
    sizeof(*child.ext_info_cache),
    GFP_KERNEL);
    if (!child.ext_info_cache)
    return -ENOMEM;
    for (i = 0; i < num_ext_info; ++i) {
    child.ext_info_cache[i].size = -1;
    if (!pchan.ext_info[i].write)
    continue;
    if (!pchan.ext_info[i].read)
    continue;
    ret = iio_read_channel_ext_info(mux.parent,
    mux.ext_info[i].name,
    page);
    if (ret < 0) {
    dev_err(dev, "failed to get ext_info '%s'\n",
    pchan.ext_info[i].name);
    return ret;
    }
    if (ret >= PAGE_SIZE) {
    dev_err(dev, "too large ext_info '%s'\n",
    pchan.ext_info[i].name);
    return -EINVAL;
    }
    child.ext_info_cache[i].data = devm_kmemdup(dev, page, ret + 1,
    GFP_KERNEL);
    if (!child.ext_info_cache[i].data)
    return -ENOMEM;
    child.ext_info_cache[i].data[ret] = 0;
    child.ext_info_cache[i].size = ret;
    }
    return 0;
    }
    static int mux_configure_channel(struct device *dev, struct mux *mux, u32 state,
    const char *label, int idx)
    {
    struct iio_chan_spec *chan = &mux.chan[idx];
    struct iio_chan_spec const *pchan = mux.parent.channel;
    int num_ext_info;
    int ret;
    chan.indexed = 1;
    chan.output = pchan.output;
    chan.datasheet_name = label;
    chan.ext_info = mux.ext_info;
    ret = iio_get_channel_type(mux.parent, &chan.type);
    if (ret < 0) {
    dev_err(dev, "failed to get parent channel type\n");
    return ret;
    }
    if (iio_channel_has_info(pchan, IIO_CHAN_INFO_RAW))
    chan.info_mask_separate |= BIT(IIO_CHAN_INFO_RAW);
    if (iio_channel_has_info(pchan, IIO_CHAN_INFO_SCALE))
    chan.info_mask_separate |= BIT(IIO_CHAN_INFO_SCALE);
    if (iio_channel_has_available(pchan, IIO_CHAN_INFO_RAW))
    chan.info_mask_separate_available |= BIT(IIO_CHAN_INFO_RAW);
    if (state >= mux_control_states(mux.control)) {
    dev_err(dev, "too many channels\n");
    return -EINVAL;
    }
    chan.channel = state;
    num_ext_info = iio_get_channel_ext_info_count(mux.parent);
    if (num_ext_info)
    return mux_configure_chan_ext_info(dev, mux, idx, num_ext_info);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mux_probe(pdev: *mut platform_device) -> c_int {
    static int mux_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct iio_dev *indio_dev;
    struct iio_channel *parent;
    struct mux *mux;
    const char **labels;
    int all_children;
    int children;
    u32 state;
    int sizeof_ext_info;
    int sizeof_priv;
    int i;
    int ret;
    parent = devm_iio_channel_get(dev, "parent");
    if (IS_ERR(parent))
    return dev_err_probe(dev, PTR_ERR(parent),
    "failed to get parent channel\n");
    sizeof_ext_info = iio_get_channel_ext_info_count(parent);
    if (sizeof_ext_info) {
    sizeof_ext_info += 1; /* one extra entry for the sentinel */
    sizeof_ext_info *= sizeof(*mux.ext_info);
    }
    all_children = device_property_string_array_count(dev, "channels");
    if (all_children < 0)
    return all_children;
    labels = devm_kmalloc_array(dev, all_children, sizeof(*labels), GFP_KERNEL);
    if (!labels)
    return -ENOMEM;
    ret = device_property_read_string_array(dev, "channels", labels, all_children);
    if (ret < 0)
    return ret;
    children = 0;
    for (state = 0; state < all_children; state++) {
    if (*labels[state])
    children++;
    }
    if (children <= 0) {
    dev_err(dev, "not even a single child\n");
    return -EINVAL;
    }
    sizeof_priv = sizeof(*mux);
    sizeof_priv += sizeof(*mux.child) * children;
    sizeof_priv += sizeof(*mux.chan) * children;
    sizeof_priv += sizeof_ext_info;
    indio_dev = devm_iio_device_alloc(dev, sizeof_priv);
    if (!indio_dev)
    return -ENOMEM;
    mux = iio_priv(indio_dev);
    mux.child = (struct mux_child *)(mux + 1);
    mux.chan = (struct iio_chan_spec *)(mux.child + children);
    platform_set_drvdata(pdev, indio_dev);
    mux.parent = parent;
    mux.cached_state = -1;
    mux.delay_us = 0;
    device_property_read_u32(dev, "settle-time-us", &mux.delay_us);
    indio_dev.name = dev_name(dev);
    indio_dev.info = &mux_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = mux.chan;
    indio_dev.num_channels = children;
    if (sizeof_ext_info) {
    mux.ext_info = devm_kmemdup(dev,
    parent.channel.ext_info,
    sizeof_ext_info, GFP_KERNEL);
    if (!mux.ext_info)
    return -ENOMEM;
    for (i = 0; mux.ext_info[i].name; ++i) {
    if (parent.channel.ext_info[i].read)
    mux.ext_info[i].read = mux_read_ext_info;
    if (parent.channel.ext_info[i].write)
    mux.ext_info[i].write = mux_write_ext_info;
    mux.ext_info[i].private = i;
    }
    }
    mux.control = devm_mux_control_get(dev, core::ptr::null_mut());
    if (IS_ERR(mux.control))
    return dev_err_probe(dev, PTR_ERR(mux.control),
    "failed to get control-mux\n");
    i = 0;
    for (state = 0; state < all_children; state++) {
    if (!*labels[state])
    continue;
    ret = mux_configure_channel(dev, mux, state, labels[state], i++);
    if (ret < 0)
    return ret;
    }
    ret = devm_iio_device_register(dev, indio_dev);
    if (ret) {
    dev_err(dev, "failed to register iio device\n");
    return ret;
    }
    return 0;
    }
    static const struct of_device_id mux_match[] = {
    { .compatible = "io-channel-mux" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mux_match);
    static struct platform_driver mux_driver = {
    .probe = mux_probe,
    .driver = {
    .name = "iio-mux",
    .of_match_table = mux_match,
    },
    };
    module_platform_driver(mux_driver);
    MODULE_DESCRIPTION("IIO multiplexer driver");
    MODULE_AUTHOR("Peter Rosin <peda@axentia.se>");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_CONSUMER");
