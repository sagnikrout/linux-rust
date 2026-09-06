//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/pci/vfio_pci.c
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
// Copyright (c) 2021, NVIDIA CORPORATION & AFFILIATES. All rights reserved
//
// Copyright (C) 2012 Red Hat, Inc.  All rights reserved.
// Author: Alex Williamson <alex.williamson@redhat.com>
//
// Derived from original vfio:
// Copyright 2010 Cisco Systems, Inc.  All rights reserved.
// Author: Tom Lyon, pugs@cisco.com
//

    static char ids[1024] __initdata;
    module_param_string(ids, ids, sizeof(ids), 0);
    MODULE_PARM_DESC(ids, "Initial PCI IDs to add to the vfio driver, format is \"vendor:device[:subvendor[:subdevice[:class[:class_mask]]]]\" and multiple comma separated entries can be specified");
    static bool nointxmask;
    module_param_named(nointxmask, nointxmask, bool, S_IRUGO | S_IWUSR);
    MODULE_PARM_DESC(nointxmask,
    "Disable support for PCI 2.3 style INTx masking.  If this resolves problems for specific devices, report lspci -vvvxxx to linux-pci@vger.kernel.org so the device can be fixed automatically via the broken_intx_masking flag.");

    static bool disable_vga;
    module_param(disable_vga, bool, S_IRUGO);
    MODULE_PARM_DESC(disable_vga, "Disable VGA resource access through vfio-pci");

    static bool disable_idle_d3;
    module_param(disable_idle_d3, bool, S_IRUGO | S_IWUSR);
    MODULE_PARM_DESC(disable_idle_d3,
    "Disable using the PCI D3 low power state for idle, unused devices");
    static bool enable_sriov;

    module_param(enable_sriov, bool, 0644);
    MODULE_PARM_DESC(enable_sriov, "Enable support for SR-IOV configuration.  Enabling SR-IOV on a PF typically requires support of the userspace PF driver, enabling VFs without such support may result in non-functional VFs or PF.");

    static bool disable_denylist;
    module_param(disable_denylist, bool, 0444);
    MODULE_PARM_DESC(disable_denylist, "Disable use of device denylist. Disabling the denylist allows binding to devices with known errata that may lead to exploitable stability or security issues when accessed by untrusted users.");
#[no_mangle]
unsafe extern "C" fn vfio_pci_dev_in_denylist(pdev: *mut pci_dev) -> bool {
    static bool vfio_pci_dev_in_denylist(struct pci_dev *pdev)
    {
    switch (pdev.vendor) {
    case PCI_VENDOR_ID_INTEL:
    switch (pdev.device) {
    case PCI_DEVICE_ID_INTEL_QAT_C3XXX:
    case PCI_DEVICE_ID_INTEL_QAT_C3XXX_VF:
    case PCI_DEVICE_ID_INTEL_QAT_C62X:
    case PCI_DEVICE_ID_INTEL_QAT_C62X_VF:
    case PCI_DEVICE_ID_INTEL_QAT_DH895XCC:
    case PCI_DEVICE_ID_INTEL_QAT_DH895XCC_VF:
    case PCI_DEVICE_ID_INTEL_DSA_SPR0:
    case PCI_DEVICE_ID_INTEL_IAX_SPR0:
    return true;
    default:
    return false;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn vfio_pci_is_denylisted(pdev: *mut pci_dev) -> bool {
    static bool vfio_pci_is_denylisted(struct pci_dev *pdev)
    {
    if (!vfio_pci_dev_in_denylist(pdev))
    return false;
    if (disable_denylist) {
    pci_warn(pdev,
    "device denylist disabled - allowing device %04x:%04x.\n",
    pdev.vendor, pdev.device);
    return false;
    }
    pci_warn(pdev, "%04x:%04x exists in vfio-pci device denylist, driver probing disallowed.\n",
    pdev.vendor, pdev.device);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn vfio_pci_open_device(core_vdev: *mut vfio_device) -> c_int {
    static int vfio_pci_open_device(struct vfio_device *core_vdev)
    {
    struct vfio_pci_core_device *vdev =
    container_of(core_vdev, struct vfio_pci_core_device, vdev);
    struct pci_dev *pdev = vdev.pdev;
    int ret;
    ret = vfio_pci_core_enable(vdev);
    if (ret)
    return ret;
    if (vfio_pci_is_intel_display(pdev)) {
    ret = vfio_pci_igd_init(vdev);
    if (ret && ret != -ENODEV) {
    pci_warn(pdev, "Failed to setup Intel IGD regions\n");
    vfio_pci_core_disable(vdev);
    return ret;
    }
    }
    vfio_pci_core_finish_enable(vdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vfio_pci_init_dev(core_vdev: *mut vfio_device) -> c_int {
    static int vfio_pci_init_dev(struct vfio_device *core_vdev)
    {
    struct vfio_pci_core_device *vdev =
    container_of(core_vdev, struct vfio_pci_core_device, vdev);
//
// These behaviors originated in vfio-pci and moved into
// vfio-pci-core when the driver was split; vfio-pci remains the
// only driver that toggles them.  Latch our module parameters per
// device at init time so that later parameter changes do not
// affect already-initialized devices.
//
    vdev.nointxmask = nointxmask;
    vdev.disable_idle_d3 = disable_idle_d3;

    vdev.disable_vga = disable_vga;

    return vfio_pci_core_init_dev(core_vdev);
    }
    static const struct vfio_device_ops vfio_pci_ops = {
    .name		= "vfio-pci",
    .init		= vfio_pci_init_dev,
    .release	= vfio_pci_core_release_dev,
    .open_device	= vfio_pci_open_device,
    .close_device	= vfio_pci_core_close_device,
    .ioctl		= vfio_pci_core_ioctl,
    .get_region_info_caps = vfio_pci_ioctl_get_region_info,
    .device_feature = vfio_pci_core_ioctl_feature,
    .read		= vfio_pci_core_read,
    .write		= vfio_pci_core_write,
    .mmap		= vfio_pci_core_mmap,
    .request	= vfio_pci_core_request,
    .match		= vfio_pci_core_match,
    .match_token_uuid = vfio_pci_core_match_token_uuid,
    .bind_iommufd	= vfio_iommufd_physical_bind,
    .unbind_iommufd	= vfio_iommufd_physical_unbind,
    .attach_ioas	= vfio_iommufd_physical_attach_ioas,
    .detach_ioas	= vfio_iommufd_physical_detach_ioas,
    .pasid_attach_ioas	= vfio_iommufd_physical_pasid_attach_ioas,
    .pasid_detach_ioas	= vfio_iommufd_physical_pasid_detach_ioas,
    };
    static const struct vfio_pci_device_ops vfio_pci_dev_ops = {
    .get_dmabuf_phys = vfio_pci_core_get_dmabuf_phys,
    };
#[no_mangle]
unsafe extern "C" fn vfio_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int vfio_pci_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct vfio_pci_core_device *vdev;
    int ret;
    if (vfio_pci_is_denylisted(pdev))
    return -EINVAL;
    vdev = vfio_alloc_device(vfio_pci_core_device, vdev, &pdev.dev,
    &vfio_pci_ops);
    if (IS_ERR(vdev))
    return PTR_ERR(vdev);
    dev_set_drvdata(&pdev.dev, vdev);
    vdev.pci_ops = &vfio_pci_dev_ops;
    ret = vfio_pci_core_register_device(vdev);
    if (ret)
    goto out_put_vdev;
    return 0;
    out_put_vdev:
    vfio_put_device(&vdev.vdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vfio_pci_remove(pdev: *mut pci_dev) {
    static void vfio_pci_remove(struct pci_dev *pdev)
    {
    struct vfio_pci_core_device *vdev = dev_get_drvdata(&pdev.dev);
    vfio_pci_core_unregister_device(vdev);
    vfio_put_device(&vdev.vdev);
    }
#[no_mangle]
unsafe extern "C" fn vfio_pci_sriov_configure(pdev: *mut pci_dev, nr_virtfn: c_int) -> c_int {
    static int vfio_pci_sriov_configure(struct pci_dev *pdev, int nr_virtfn)
    {
    struct vfio_pci_core_device *vdev = dev_get_drvdata(&pdev.dev);
    if (!enable_sriov)
    return -ENOENT;
    return vfio_pci_core_sriov_configure(vdev, nr_virtfn);
    }
    static const struct pci_device_id vfio_pci_table[] = {
    { PCI_DRIVER_OVERRIDE_DEVICE_VFIO(PCI_ANY_ID, PCI_ANY_ID) }, /* match all by default */
    {}
    };
    MODULE_DEVICE_TABLE(pci, vfio_pci_table);
    static struct pci_driver vfio_pci_driver = {
    .name			= "vfio-pci",
    .id_table		= vfio_pci_table,
    .probe			= vfio_pci_probe,
    .remove			= vfio_pci_remove,
    .sriov_configure	= vfio_pci_sriov_configure,
    .err_handler		= &vfio_pci_core_err_handlers,
    .driver_managed_dma	= true,
    };
#[no_mangle]
unsafe extern "C" fn vfio_pci_fill_ids() -> void __init {
    static void __init vfio_pci_fill_ids(void)
    {
    char *p, *id;
    int rc;
// no ids passed actually
    if (ids[0] == '\0')
    return;
// add ids specified in the module parameter
    p = ids;
    while ((id = strsep(&p, ","))) {
    unsigned int vendor, device, subvendor = PCI_ANY_ID,
    subdevice = PCI_ANY_ID, class = 0, class_mask = 0;
    int fields;
    if (!strlen(id))
    continue;
    fields = sscanf(id, "%x:%x:%x:%x:%x:%x",
    &vendor, &device, &subvendor, &subdevice,
    &class, &class_mask);
    if (fields < 2) {
    pr_warn("invalid id string \"%s\"\n", id);
    continue;
    }
    rc = pci_add_dynid(&vfio_pci_driver, vendor, device,
    subvendor, subdevice, class, class_mask, 0);
    if (rc)
    pr_warn("failed to add dynamic id [%04x:%04x[%04x:%04x]] class %#08x/%08x (%d)\n",
    vendor, device, subvendor, subdevice,
    class, class_mask, rc);
    else
    pr_info("add [%04x:%04x[%04x:%04x]] class %#08x/%08x\n",
    vendor, device, subvendor, subdevice,
    class, class_mask);
    }
    }
#[no_mangle]
unsafe extern "C" fn vfio_pci_init() -> int __init {
    static int __init vfio_pci_init(void)
    {
    int ret;
// Register and scan for devices
    ret = pci_register_driver(&vfio_pci_driver);
    if (ret)
    return ret;
    vfio_pci_fill_ids();
    if (disable_denylist)
    pr_warn("device denylist disabled.\n");
    return 0;
    }
    module_init(vfio_pci_init);
#[no_mangle]
unsafe extern "C" fn vfio_pci_cleanup() -> void __exit {
    static void __exit vfio_pci_cleanup(void)
    {
    pci_unregister_driver(&vfio_pci_driver);
    }
    module_exit(vfio_pci_cleanup);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
