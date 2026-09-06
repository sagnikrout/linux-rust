//! Automatically rewritten from C to Rust
//! Source: drivers/peci/device.c
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
// Copyright (c) 2018-2021 Intel Corporation

//
// PECI device can be removed using sysfs, but the removal can also happen as
// a result of controller being removed.
// Mutex is used to protect PECI device from being double-deleted.
//
    static DEFINE_MUTEX(peci_device_del_lock);

#[no_mangle]
unsafe extern "C" fn peci_get_revision(device: *mut peci_device, revision: *mut u8) -> c_int {
    static int peci_get_revision(struct peci_device *device, u8 *revision)
    {
    struct peci_request *req;
    u64 dib;
    req = peci_xfer_get_dib(device);
    if (IS_ERR(req))
    return PTR_ERR(req);
//
// PECI device may be in a state where it is unable to return a proper
// DIB, in which case it returns 0 as DIB value.
// Let's treat this as an error to avoid carrying on with the detection
// using invalid revision.
//
    dib = peci_request_dib_read(req);
    if (dib == 0) {
    peci_request_free(req);
    return -EIO;
    }
// revision = FIELD_GET(REVISION_NUM_MASK, dib);
    peci_request_free(req);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn peci_get_cpu_id(device: *mut peci_device, cpu_id: *mut u32) -> c_int {
    static int peci_get_cpu_id(struct peci_device *device, u32 *cpu_id)
    {
    struct peci_request *req;
    int ret;
    req = peci_xfer_pkg_cfg_readl(device, PECI_PCS_PKG_ID, PECI_PKG_ID_CPU_ID);
    if (IS_ERR(req))
    return PTR_ERR(req);
    ret = peci_request_status(req);
    if (ret)
    goto out_req_free;
// cpu_id = peci_request_data_readl(req);
    out_req_free:
    peci_request_free(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn peci_x86_cpu_family(sig: c_uint) -> c_uint {
    static unsigned int peci_x86_cpu_family(unsigned int sig)
    {
    unsigned int x86;
    x86 = (sig >> 8) & 0xf;
    if (x86 == 0xf)
    x86 += (sig >> 20) & 0xff;
    return x86;
    }
#[no_mangle]
unsafe extern "C" fn peci_x86_cpu_model(sig: c_uint) -> c_uint {
    static unsigned int peci_x86_cpu_model(unsigned int sig)
    {
    unsigned int fam, model;
    fam = peci_x86_cpu_family(sig);
    model = (sig >> 4) & 0xf;
    if (fam >= 0x6)
    model += ((sig >> 16) & 0xf) << 4;
    return model;
    }
#[no_mangle]
unsafe extern "C" fn peci_device_info_init(device: *mut peci_device) -> c_int {
    static int peci_device_info_init(struct peci_device *device)
    {
    u8 revision;
    u32 cpu_id;
    int ret;
    ret = peci_get_cpu_id(device, &cpu_id);
    if (ret)
    return ret;
    device.info.x86_vfm = IFM(peci_x86_cpu_family(cpu_id), peci_x86_cpu_model(cpu_id));
    ret = peci_get_revision(device, &revision);
    if (ret)
    return ret;
    device.info.peci_revision = revision;
    device.info.socket_id = device.addr - PECI_BASE_ADDR;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn peci_detect(controller: *mut peci_controller, addr: u8) -> c_int {
    static int peci_detect(struct peci_controller *controller, u8 addr)
    {
//
// PECI Ping is a command encoded by tx_len = 0, rx_len = 0.
// We expect correct Write FCS if the device at the target address
// is able to respond.
//
    let mut req: peci_request = { 0 };
    int ret;
    mutex_lock(&controller.bus_lock);
    ret = controller.ops.xfer(controller, addr, &req);
    mutex_unlock(&controller.bus_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn peci_addr_valid(addr: u8) -> bool {
    static bool peci_addr_valid(u8 addr)
    {
    return addr >= PECI_BASE_ADDR && addr < PECI_BASE_ADDR + PECI_DEVICE_NUM_MAX;
    }
#[no_mangle]
unsafe extern "C" fn peci_dev_exists(dev: *mut device, data: *mut c_void) -> c_int {
    static int peci_dev_exists(struct device *dev, void *data)
    {
    struct peci_device *device = to_peci_device(dev);
    u8 *addr = data;
    if (device.addr == *addr)
    return -EBUSY;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn peci_device_create(controller: *mut peci_controller, addr: u8) -> c_int {
    int peci_device_create(struct peci_controller *controller, u8 addr)
    {
    struct peci_device *device;
    int ret;
    if (!peci_addr_valid(addr))
    return -EINVAL;
// Check if we have already detected this device before.
    ret = device_for_each_child(&controller.dev, &addr, peci_dev_exists);
    if (ret)
    return 0;
    ret = peci_detect(controller, addr);
    if (ret) {
//
// Device not present or host state doesn't allow successful
// detection at this time.
//
    if (ret == -EIO || ret == -ETIMEDOUT)
    return 0;
    return ret;
    }
    device = kzalloc_obj(*device);
    if (!device)
    return -ENOMEM;
    device_initialize(&device.dev);
    device.addr = addr;
    device.dev.parent = &controller.dev;
    device.dev.bus = &peci_bus_type;
    device.dev.type = &peci_device_type;
    ret = peci_device_info_init(device);
    if (ret)
    goto err_put;
    ret = dev_set_name(&device.dev, "%d-%02x", controller.id, device.addr);
    if (ret)
    goto err_put;
    ret = device_add(&device.dev);
    if (ret)
    goto err_put;
    return 0;
    err_put:
    put_device(&device.dev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn peci_device_destroy(device: *mut peci_device) {
    void peci_device_destroy(struct peci_device *device)
    {
    mutex_lock(&peci_device_del_lock);
    if (!device.deleted) {
    device_unregister(&device.dev);
    device.deleted = true;
    }
    mutex_unlock(&peci_device_del_lock);
    }
    int __peci_driver_register(struct peci_driver *driver, struct module *owner,
    const char *mod_name)
    {
    driver.driver.bus = &peci_bus_type;
    driver.driver.owner = owner;
    driver.driver.mod_name = mod_name;
    if (!driver.probe) {
    pr_err("peci: trying to register driver without probe callback\n");
    return -EINVAL;
    }
    if (!driver.id_table) {
    pr_err("peci: trying to register driver without device id table\n");
    return -EINVAL;
    }
    return driver_register(&driver.driver);
    }
    EXPORT_SYMBOL_NS_GPL(__peci_driver_register, "PECI");
#[no_mangle]
pub unsafe extern "C" fn peci_driver_unregister(driver: *mut peci_driver) {
    void peci_driver_unregister(struct peci_driver *driver)
    {
    driver_unregister(&driver.driver);
    }
    EXPORT_SYMBOL_NS_GPL(peci_driver_unregister, "PECI");
#[no_mangle]
unsafe extern "C" fn peci_device_release(dev: *mut device) {
    static void peci_device_release(struct device *dev)
    {
    struct peci_device *device = to_peci_device(dev);
    kfree(device);
    }
    const struct device_type peci_device_type = {
    .groups		= peci_device_groups,
    .release	= peci_device_release,
    };
