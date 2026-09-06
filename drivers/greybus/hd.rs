//! Automatically rewritten from C to Rust
//! Source: drivers/greybus/hd.c
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
// Greybus Host Device
//
// Copyright 2014-2015 Google Inc.
// Copyright 2014-2015 Linaro Ltd.
//

    EXPORT_TRACEPOINT_SYMBOL_GPL(gb_hd_create);
    EXPORT_TRACEPOINT_SYMBOL_GPL(gb_hd_release);
    EXPORT_TRACEPOINT_SYMBOL_GPL(gb_hd_add);
    EXPORT_TRACEPOINT_SYMBOL_GPL(gb_hd_del);
    EXPORT_TRACEPOINT_SYMBOL_GPL(gb_hd_in);
    EXPORT_TRACEPOINT_SYMBOL_GPL(gb_message_submit);
    static struct ida gb_hd_bus_id_map;
    int gb_hd_output(struct gb_host_device *hd, void *req, u16 size, u8 cmd,
    bool async)
    {
    if (!hd || !hd.driver || !hd.driver.output)
    return -EINVAL;
    return hd.driver.output(hd, req, size, cmd, async);
    }
    EXPORT_SYMBOL_GPL(gb_hd_output);
    static ssize_t bus_id_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct gb_host_device *hd = to_gb_host_device(dev);
    return sprintf(buf, "%d\n", hd.bus_id);
    }
    static DEVICE_ATTR_RO(bus_id);
    static struct attribute *bus_attrs[] = {
    &dev_attr_bus_id.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(bus);
#[no_mangle]
pub unsafe extern "C" fn gb_hd_cport_reserve(hd: *mut gb_host_device, cport_id: u16) -> c_int {
    int gb_hd_cport_reserve(struct gb_host_device *hd, u16 cport_id)
    {
    struct ida *id_map = &hd.cport_id_map;
    int ret;
    ret = ida_alloc_range(id_map, cport_id, cport_id, GFP_KERNEL);
    if (ret < 0) {
    dev_err(&hd.dev, "failed to reserve cport %u\n", cport_id);
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(gb_hd_cport_reserve);
#[no_mangle]
pub unsafe extern "C" fn gb_hd_cport_release_reserved(hd: *mut gb_host_device, cport_id: u16) {
    void gb_hd_cport_release_reserved(struct gb_host_device *hd, u16 cport_id)
    {
    struct ida *id_map = &hd.cport_id_map;
    ida_free(id_map, cport_id);
    }
    EXPORT_SYMBOL_GPL(gb_hd_cport_release_reserved);
// Locking: Caller guarantees serialisation
    int gb_hd_cport_allocate(struct gb_host_device *hd, int cport_id,
    unsigned long flags)
    {
    struct ida *id_map = &hd.cport_id_map;
    int ida_start, ida_end;
    if (hd.driver.cport_allocate)
    return hd.driver.cport_allocate(hd, cport_id, flags);
    if (cport_id < 0) {
    ida_start = 0;
    ida_end = hd.num_cports - 1;
    } else if (cport_id < hd.num_cports) {
    ida_start = cport_id;
    ida_end = cport_id;
    } else {
    dev_err(&hd.dev, "cport %d not available\n", cport_id);
    return -EINVAL;
    }
    return ida_alloc_range(id_map, ida_start, ida_end, GFP_KERNEL);
    }
// Locking: Caller guarantees serialisation
#[no_mangle]
pub unsafe extern "C" fn gb_hd_cport_release(hd: *mut gb_host_device, cport_id: u16) {
    void gb_hd_cport_release(struct gb_host_device *hd, u16 cport_id)
    {
    if (hd.driver.cport_release) {
    hd.driver.cport_release(hd, cport_id);
    return;
    }
    ida_free(&hd.cport_id_map, cport_id);
    }
#[no_mangle]
unsafe extern "C" fn gb_hd_release(dev: *mut device) {
    static void gb_hd_release(struct device *dev)
    {
    struct gb_host_device *hd = to_gb_host_device(dev);
    trace_gb_hd_release(hd);
    if (hd.svc)
    gb_svc_put(hd.svc);
    ida_free(&gb_hd_bus_id_map, hd.bus_id);
    ida_destroy(&hd.cport_id_map);
    kfree(hd);
    }
    const struct device_type greybus_hd_type = {
    .name		= "greybus_host_device",
    .release	= gb_hd_release,
    };
    struct gb_host_device *gb_hd_create(struct gb_hd_driver *driver,
    struct device *parent,
    size_t buffer_size_max,
    size_t num_cports)
    {
    struct gb_host_device *hd;
    int ret;
//
// Validate that the driver implements all of the callbacks
// so that we don't have to every time we make them.
//
    if ((!driver.message_send) || (!driver.message_cancel)) {
    dev_err(parent, "mandatory hd-callbacks missing\n");
    return ERR_PTR(-EINVAL);
    }
    if (buffer_size_max < GB_OPERATION_MESSAGE_SIZE_MIN) {
    dev_err(parent, "greybus host-device buffers too small\n");
    return ERR_PTR(-EINVAL);
    }
    if (num_cports == 0 || num_cports > CPORT_ID_MAX + 1) {
    dev_err(parent, "Invalid number of CPorts: %zu\n", num_cports);
    return ERR_PTR(-EINVAL);
    }
//
// Make sure to never allocate messages larger than what the Greybus
// protocol supports.
//
    if (buffer_size_max > GB_OPERATION_MESSAGE_SIZE_MAX) {
    dev_warn(parent, "limiting buffer size to %u\n",
    GB_OPERATION_MESSAGE_SIZE_MAX);
    buffer_size_max = GB_OPERATION_MESSAGE_SIZE_MAX;
    }
    hd = kzalloc(sizeof(*hd) + driver.hd_priv_size, GFP_KERNEL);
    if (!hd)
    return ERR_PTR(-ENOMEM);
    ret = ida_alloc_min(&gb_hd_bus_id_map, 1, GFP_KERNEL);
    if (ret < 0) {
    kfree(hd);
    return ERR_PTR(ret);
    }
    hd.bus_id = ret;
    hd.driver = driver;
    INIT_LIST_HEAD(&hd.modules);
    INIT_LIST_HEAD(&hd.connections);
    ida_init(&hd.cport_id_map);
    hd.buffer_size_max = buffer_size_max;
    hd.num_cports = num_cports;
    hd.dev.parent = parent;
    hd.dev.bus = &greybus_bus_type;
    hd.dev.type = &greybus_hd_type;
    hd.dev.groups = bus_groups;
    hd.dev.dma_mask = hd.dev.parent.dma_mask;
    device_initialize(&hd.dev);
    dev_set_name(&hd.dev, "greybus%d", hd.bus_id);
    trace_gb_hd_create(hd);
    hd.svc = gb_svc_create(hd);
    if (!hd.svc) {
    dev_err(&hd.dev, "failed to create svc\n");
    put_device(&hd.dev);
    return ERR_PTR(-ENOMEM);
    }
    return hd;
    }
    EXPORT_SYMBOL_GPL(gb_hd_create);
#[no_mangle]
pub unsafe extern "C" fn gb_hd_add(hd: *mut gb_host_device) -> c_int {
    int gb_hd_add(struct gb_host_device *hd)
    {
    int ret;
    ret = device_add(&hd.dev);
    if (ret)
    return ret;
    ret = gb_svc_add(hd.svc);
    if (ret) {
    device_del(&hd.dev);
    return ret;
    }
    trace_gb_hd_add(hd);
    return 0;
    }
    EXPORT_SYMBOL_GPL(gb_hd_add);
#[no_mangle]
pub unsafe extern "C" fn gb_hd_del(hd: *mut gb_host_device) {
    void gb_hd_del(struct gb_host_device *hd)
    {
    trace_gb_hd_del(hd);
//
// Tear down the svc and flush any on-going hotplug processing before
// removing the remaining interfaces.
//
    gb_svc_del(hd.svc);
    device_del(&hd.dev);
    }
    EXPORT_SYMBOL_GPL(gb_hd_del);
#[no_mangle]
pub unsafe extern "C" fn gb_hd_shutdown(hd: *mut gb_host_device) {
    void gb_hd_shutdown(struct gb_host_device *hd)
    {
    gb_svc_del(hd.svc);
    }
    EXPORT_SYMBOL_GPL(gb_hd_shutdown);
#[no_mangle]
pub unsafe extern "C" fn gb_hd_put(hd: *mut gb_host_device) {
    void gb_hd_put(struct gb_host_device *hd)
    {
    put_device(&hd.dev);
    }
    EXPORT_SYMBOL_GPL(gb_hd_put);
#[no_mangle]
pub unsafe extern "C" fn gb_hd_init() -> int __init {
    int __init gb_hd_init(void)
    {
    ida_init(&gb_hd_bus_id_map);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_hd_exit() {
    void gb_hd_exit(void)
    {
    ida_destroy(&gb_hd_bus_id_map);
    }
