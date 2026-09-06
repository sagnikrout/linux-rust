//! Automatically rewritten from C to Rust
//! Source: drivers/peci/cpu.c
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
// Copyright (c) 2021 Intel Corporation

//
// peci_temp_read() - read the maximum die temperature from PECI target device
// @device: PECI device to which request is going to be sent
// @temp_raw: where to store the read temperature
//
// It uses GetTemp PECI command.
//
// Return: 0 if succeeded, other values in case errors.
//
#[no_mangle]
pub unsafe extern "C" fn peci_temp_read(device: *mut peci_device, temp_raw: *mut i16) -> c_int {
    int peci_temp_read(struct peci_device *device, s16 *temp_raw)
    {
    struct peci_request *req;
    req = peci_xfer_get_temp(device);
    if (IS_ERR(req))
    return PTR_ERR(req);
// temp_raw = peci_request_temp_read(req);
    peci_request_free(req);
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(peci_temp_read, "PECI_CPU");
//
// peci_pcs_read() - read PCS register
// @device: PECI device to which request is going to be sent
// @index: PCS index
// @param: PCS parameter
// @data: where to store the read data
//
// It uses RdPkgConfig PECI command.
//
// Return: 0 if succeeded, other values in case errors.
//
#[no_mangle]
pub unsafe extern "C" fn peci_pcs_read(device: *mut peci_device, index: u8, param: u16, data: *mut u32) -> c_int {
    int peci_pcs_read(struct peci_device *device, u8 index, u16 param, u32 *data)
    {
    struct peci_request *req;
    int ret;
    req = peci_xfer_pkg_cfg_readl(device, index, param);
    if (IS_ERR(req))
    return PTR_ERR(req);
    ret = peci_request_status(req);
    if (ret)
    goto out_req_free;
// data = peci_request_data_readl(req);
    out_req_free:
    peci_request_free(req);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(peci_pcs_read, "PECI_CPU");
//
// peci_pci_local_read() - read 32-bit memory location using raw address
// @device: PECI device to which request is going to be sent
// @bus: bus
// @dev: device
// @func: function
// @reg: register
// @data: where to store the read data
//
// It uses RdPCIConfigLocal PECI command.
//
// Return: 0 if succeeded, other values in case errors.
//
    int peci_pci_local_read(struct peci_device *device, u8 bus, u8 dev, u8 func,
    u16 reg, u32 *data)
    {
    struct peci_request *req;
    int ret;
    req = peci_xfer_pci_cfg_local_readl(device, bus, dev, func, reg);
    if (IS_ERR(req))
    return PTR_ERR(req);
    ret = peci_request_status(req);
    if (ret)
    goto out_req_free;
// data = peci_request_data_readl(req);
    out_req_free:
    peci_request_free(req);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(peci_pci_local_read, "PECI_CPU");
//
// peci_ep_pci_local_read() - read 32-bit memory location using raw address
// @device: PECI device to which request is going to be sent
// @seg: PCI segment
// @bus: bus
// @dev: device
// @func: function
// @reg: register
// @data: where to store the read data
//
// Like &peci_pci_local_read, but it uses RdEndpointConfig PECI command.
//
// Return: 0 if succeeded, other values in case errors.
//
    int peci_ep_pci_local_read(struct peci_device *device, u8 seg,
    u8 bus, u8 dev, u8 func, u16 reg, u32 *data)
    {
    struct peci_request *req;
    int ret;
    req = peci_xfer_ep_pci_cfg_local_readl(device, seg, bus, dev, func, reg);
    if (IS_ERR(req))
    return PTR_ERR(req);
    ret = peci_request_status(req);
    if (ret)
    goto out_req_free;
// data = peci_request_data_readl(req);
    out_req_free:
    peci_request_free(req);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(peci_ep_pci_local_read, "PECI_CPU");
//
// peci_mmio_read() - read 32-bit memory location using 64-bit bar offset address
// @device: PECI device to which request is going to be sent
// @bar: PCI bar
// @seg: PCI segment
// @bus: bus
// @dev: device
// @func: function
// @address: 64-bit MMIO address
// @data: where to store the read data
//
// It uses RdEndpointConfig PECI command.
//
// Return: 0 if succeeded, other values in case errors.
//
    int peci_mmio_read(struct peci_device *device, u8 bar, u8 seg,
    u8 bus, u8 dev, u8 func, u64 address, u32 *data)
    {
    struct peci_request *req;
    int ret;
    req = peci_xfer_ep_mmio64_readl(device, bar, seg, bus, dev, func, address);
    if (IS_ERR(req))
    return PTR_ERR(req);
    ret = peci_request_status(req);
    if (ret)
    goto out_req_free;
// data = peci_request_data_readl(req);
    out_req_free:
    peci_request_free(req);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(peci_mmio_read, "PECI_CPU");
    static const char * const peci_adev_types[] = {
    "cputemp",
    "dimmtemp",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peci_cpu {
    pub device: *mut peci_device,
    pub id: *const peci_device_id,
}

#[no_mangle]
unsafe extern "C" fn adev_release(dev: *mut device) {
    static void adev_release(struct device *dev)
    {
    struct auxiliary_device *adev = to_auxiliary_dev(dev);
    kfree(adev.name);
    kfree(adev);
    }
    static struct auxiliary_device *adev_alloc(struct peci_cpu *priv, int idx)
    {
    struct peci_controller *controller = to_peci_controller(priv.device.dev.parent);
    struct auxiliary_device *adev;
    const char *name;
    int ret;
    adev = kzalloc_obj(*adev);
    if (!adev)
    return ERR_PTR(-ENOMEM);
    name = kasprintf(GFP_KERNEL, "%s.%s", peci_adev_types[idx], (const char *)priv.id.data);
    if (!name) {
    ret = -ENOMEM;
    goto free_adev;
    }
    adev.name = name;
    adev.dev.parent = &priv.device.dev;
    adev.dev.release = adev_release;
    adev.id = (controller.id << 16) | (priv.device.addr);
    ret = auxiliary_device_init(adev);
    if (ret)
    goto free_name;
    return adev;
    free_name:
    kfree(name);
    free_adev:
    kfree(adev);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn unregister_adev(_adev: *mut c_void) {
    static void unregister_adev(void *_adev)
    {
    struct auxiliary_device *adev = _adev;
    auxiliary_device_delete(adev);
    auxiliary_device_uninit(adev);
    }
#[no_mangle]
unsafe extern "C" fn devm_adev_add(dev: *mut device, idx: c_int) -> c_int {
    static int devm_adev_add(struct device *dev, int idx)
    {
    struct peci_cpu *priv = dev_get_drvdata(dev);
    struct auxiliary_device *adev;
    int ret;
    adev = adev_alloc(priv, idx);
    if (IS_ERR(adev))
    return PTR_ERR(adev);
    ret = auxiliary_device_add(adev);
    if (ret) {
    auxiliary_device_uninit(adev);
    return ret;
    }
    ret = devm_add_action_or_reset(&priv.device.dev, unregister_adev, adev);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn peci_cpu_add_adevices(priv: *mut peci_cpu) {
    static void peci_cpu_add_adevices(struct peci_cpu *priv)
    {
    struct device *dev = &priv.device.dev;
    int ret, i;
    for (i = 0; i < ARRAY_SIZE(peci_adev_types); i++) {
    ret = devm_adev_add(dev, i);
    if (ret) {
    dev_warn(dev, "Failed to register PECI auxiliary: %s, ret = %d\n",
    peci_adev_types[i], ret);
    continue;
    }
    }
    }
    static int
    peci_cpu_probe(struct peci_device *device, const struct peci_device_id *id)
    {
    struct device *dev = &device.dev;
    struct peci_cpu *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    dev_set_drvdata(dev, priv);
    priv.device = device;
    priv.id = id;
    peci_cpu_add_adevices(priv);
    return 0;
    }
    static const struct peci_device_id peci_cpu_device_ids[] = {
    { /* Haswell Xeon */
    .x86_vfm = INTEL_HASWELL_X,
    .data	= "hsx",
    },
    { /* Broadwell Xeon */
    .x86_vfm = INTEL_BROADWELL_X,
    .data	= "bdx",
    },
    { /* Broadwell Xeon D */
    .x86_vfm = INTEL_BROADWELL_D,
    .data	= "bdxd",
    },
    { /* Skylake Xeon */
    .x86_vfm = INTEL_SKYLAKE_X,
    .data	= "skx",
    },
    { /* Icelake Xeon */
    .x86_vfm = INTEL_ICELAKE_X,
    .data	= "icx",
    },
    { /* Icelake Xeon D */
    .x86_vfm = INTEL_ICELAKE_D,
    .data	= "icxd",
    },
    { /* Sapphire Rapids Xeon */
    .x86_vfm = INTEL_SAPPHIRERAPIDS_X,
    .data	= "spr",
    },
    { /* Emerald Rapids Xeon */
    .x86_vfm  = INTEL_EMERALDRAPIDS_X,
    .data  = "emr",
    },
    { }
    };
    MODULE_DEVICE_TABLE(peci, peci_cpu_device_ids);
    static struct peci_driver peci_cpu_driver = {
    .probe		= peci_cpu_probe,
    .id_table	= peci_cpu_device_ids,
    .driver		= {
    .name		= "peci-cpu",
    },
    };
    module_peci_driver(peci_cpu_driver);
    MODULE_AUTHOR("Iwona Winiarska <iwona.winiarska@intel.com>");
    MODULE_DESCRIPTION("PECI CPU driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PECI");
