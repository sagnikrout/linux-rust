//! Automatically rewritten from C to Rust
//! Source: drivers/pnp/core.c
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
// core.c - contains all core device and protocol registration functions
//
// Copyright 2002 Adam Belay <ambx1@neo.rr.com>
//

    static LIST_HEAD(pnp_protocols);
    LIST_HEAD(pnp_global);
    DEFINE_MUTEX(pnp_lock);
//
// ACPI or PNPBIOS should tell us about all platform devices, so we can
// skip some blind probes.  ISAPNP typically enumerates only plug-in ISA
// devices, not built-in things like COM ports.
//
    int pnp_platform_devices;
    EXPORT_SYMBOL(pnp_platform_devices);
#[no_mangle]
unsafe extern "C" fn pnp_remove_protocol(protocol: *mut pnp_protocol) {
    static void pnp_remove_protocol(struct pnp_protocol *protocol)
    {
    mutex_lock(&pnp_lock);
    list_del(&protocol.protocol_list);
    mutex_unlock(&pnp_lock);
    }
//
// pnp_register_protocol - adds a pnp protocol to the pnp layer
// @protocol: pointer to the corresponding pnp_protocol structure
//
// Ex protocols: ISAPNP, PNPBIOS, etc
//
#[no_mangle]
pub unsafe extern "C" fn pnp_register_protocol(protocol: *mut pnp_protocol) -> c_int {
    int pnp_register_protocol(struct pnp_protocol *protocol)
    {
    struct list_head *pos;
    int nodenum, ret;
    INIT_LIST_HEAD(&protocol.devices);
    INIT_LIST_HEAD(&protocol.cards);
    nodenum = 0;
    mutex_lock(&pnp_lock);
// assign the lowest unused number
    list_for_each(pos, &pnp_protocols) {
    struct pnp_protocol *cur = to_pnp_protocol(pos);
    if (cur.number == nodenum) {
    pos = &pnp_protocols;
    nodenum++;
    }
    }
    protocol.number = nodenum;
    dev_set_name(&protocol.dev, "pnp%d", nodenum);
    list_add_tail(&protocol.protocol_list, &pnp_protocols);
    mutex_unlock(&pnp_lock);
    ret = device_register(&protocol.dev);
    if (ret)
    pnp_remove_protocol(protocol);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pnp_free_ids(dev: *mut pnp_dev) {
    static void pnp_free_ids(struct pnp_dev *dev)
    {
    struct pnp_id *id;
    struct pnp_id *next;
    id = dev.id;
    while (id) {
    next = id.next;
    kfree(id);
    id = next;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_free_resource(pnp_res: *mut pnp_resource) {
    void pnp_free_resource(struct pnp_resource *pnp_res)
    {
    list_del(&pnp_res.list);
    kfree(pnp_res);
    }
#[no_mangle]
pub unsafe extern "C" fn pnp_free_resources(dev: *mut pnp_dev) {
    void pnp_free_resources(struct pnp_dev *dev)
    {
    struct pnp_resource *pnp_res, *tmp;
    list_for_each_entry_safe(pnp_res, tmp, &dev.resources, list) {
    pnp_free_resource(pnp_res);
    }
    }
#[no_mangle]
unsafe extern "C" fn pnp_release_device(dmdev: *mut device) {
    static void pnp_release_device(struct device *dmdev)
    {
    struct pnp_dev *dev = to_pnp_dev(dmdev);
    pnp_free_ids(dev);
    pnp_free_resources(dev);
    pnp_free_options(dev);
    kfree(dev);
    }
    struct pnp_dev *pnp_alloc_dev(struct pnp_protocol *protocol, int id,
    const char *pnpid)
    {
    struct pnp_dev *dev;
    struct pnp_id *dev_id;
    dev = kzalloc_obj(struct pnp_dev);
    if (!dev)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&dev.resources);
    INIT_LIST_HEAD(&dev.options);
    dev.protocol = protocol;
    dev.number = id;
    dev.dma_mask = DMA_BIT_MASK(24);
    dev.dev.parent = &dev.protocol.dev;
    dev.dev.bus = &pnp_bus_type;
    dev.dev.dma_mask = &dev.dma_mask;
    dev.dev.coherent_dma_mask = dev.dma_mask;
    dev.dev.release = &pnp_release_device;
    dev_id = pnp_add_id(dev, pnpid);
    if (!dev_id) {
    kfree(dev);
    return core::ptr::null_mut();
    }
    dev_set_name(&dev.dev, "%02x:%02x", dev.protocol.number, dev.number);
    return dev;
    }
#[no_mangle]
unsafe extern "C" fn pnp_delist_device(dev: *mut pnp_dev) {
    static void pnp_delist_device(struct pnp_dev *dev)
    {
    mutex_lock(&pnp_lock);
    list_del(&dev.global_list);
    list_del(&dev.protocol_list);
    mutex_unlock(&pnp_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __pnp_add_device(dev: *mut pnp_dev) -> c_int {
    int __pnp_add_device(struct pnp_dev *dev)
    {
    int ret;
    pnp_fixup_device(dev);
    dev.status = PNP_READY;
    mutex_lock(&pnp_lock);
    list_add_tail(&dev.global_list, &pnp_global);
    list_add_tail(&dev.protocol_list, &dev.protocol.devices);
    mutex_unlock(&pnp_lock);
    ret = device_register(&dev.dev);
    if (ret)
    pnp_delist_device(dev);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: dev->protocol->can_wakeup) -> else {
    else if (dev.protocol.can_wakeup)
    device_set_wakeup_capable(&dev.dev,
    dev.protocol.can_wakeup(dev));
    return ret;
    }
//
// pnp_add_device - adds a pnp device to the pnp layer
// @dev: pointer to dev to add
//
// adds to driver model, name database, fixups, interface, etc.
//
#[no_mangle]
pub unsafe extern "C" fn pnp_add_device(dev: *mut pnp_dev) -> c_int {
    int pnp_add_device(struct pnp_dev *dev)
    {
    int ret;
    char buf[128];
    let mut len: c_int = 0;
    struct pnp_id *id;
    if (dev.card)
    return -EINVAL;
    ret = __pnp_add_device(dev);
    if (ret)
    return ret;
    buf[0] = '\0';
    for (id = dev.id; id; id = id.next)
    len += scnprintf(buf + len, sizeof(buf) - len, " %s", id.id);
    dev_dbg(&dev.dev, "%s device, IDs%s (%s)\n", dev.protocol.name, buf,
    dev.active ? "active" : "disabled");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pnp_init() -> int __init {
    static int __init pnp_init(void)
    {
    return bus_register(&pnp_bus_type);
    }
    subsys_initcall(pnp_init);
    int pnp_debug;

    module_param_named(debug, pnp_debug, int, 0644);
