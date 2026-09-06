//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/janz-cmodio.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Janz CMOD-IO MODULbus Carrier Board PCI Driver
//
// Copyright (c) 2010 Ira W. Snyder <iws@ovro.caltech.edu>
//
// Lots of inspiration and code was copied from drivers/mfd/sm501.c
//

// Size of each MODULbus module in PCI BAR4
pub const CMODIO_MODULBUS_SIZE: c_uint = 0x200;
// Maximum number of MODULbus modules on a CMOD-IO carrier board
pub const CMODIO_MAX_MODULES: c_int = 4;
// Module Parameters
    let mut num_modules: static unsigned int = CMODIO_MAX_MODULES;
    static char *modules[CMODIO_MAX_MODULES] = {
    "empty", "empty", "empty", "empty",
    };
    module_param_array(modules, charp, &num_modules, S_IRUGO);
    MODULE_PARM_DESC(modules, "MODULbus modules attached to the carrier board");
// Unique Device Id
    static unsigned int cmodio_id;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmodio_device {
// Parent PCI device
    pub pdev: *mut pci_dev,
// PLX control registers
    pub ctrl: *mut janz_cmodio_onboard_regs __iomem,
// hex switch position
    pub hex: u8,
// mfd-core API
    pub cells: [mfd_cell; CMODIO_MAX_MODULES],
    pub CMODIO_MAX_MODULES]: *mut *mut resource resources[3,
    pub pdata: [janz_platform_data; CMODIO_MAX_MODULES],
}

//
// Subdevices using the mfd-core API
//
    static int cmodio_setup_subdevice(struct cmodio_device *priv,
    char *name, unsigned int devno,
    unsigned int modno)
    {
    struct janz_platform_data *pdata;
    struct mfd_cell *cell;
    struct resource *res;
    struct pci_dev *pci;
    pci = priv.pdev;
    cell = &priv.cells[devno];
    res = &priv.resources[devno * 3];
    pdata = &priv.pdata[devno];
    cell.name = name;
    cell.resources = res;
    cell.num_resources = 3;
// Setup the subdevice ID -- must be unique
    cell.id = cmodio_id++;
// Add platform data
    pdata.modno = modno;
    cell.platform_data = pdata;
    cell.pdata_size = sizeof(*pdata);
// MODULbus registers -- PCI BAR3 is big-endian MODULbus access
    res.flags = IORESOURCE_MEM;
    res.parent = &pci.resource[3];
    res.start = pci.resource[3].start + (CMODIO_MODULBUS_SIZE * modno);
    res.end = res.start + CMODIO_MODULBUS_SIZE - 1;
    res++;
// PLX Control Registers -- PCI BAR4 is interrupt and other registers
    res.flags = IORESOURCE_MEM;
    res.parent = &pci.resource[4];
    res.start = pci.resource[4].start;
    res.end = pci.resource[4].end;
    res++;
//
// IRQ
//
// The start and end fields are used as an offset to the irq_base
// parameter passed into the mfd_add_devices() function call. All
// devices share the same IRQ.
//
    res.flags = IORESOURCE_IRQ;
    res.parent = core::ptr::null_mut();
    res.start = 0;
    res.end = 0;
    res++;
    return 0;
    }
// Probe each submodule using kernel parameters
#[no_mangle]
unsafe extern "C" fn cmodio_probe_submodules(priv: *mut cmodio_device) -> c_int {
    static int cmodio_probe_submodules(struct cmodio_device *priv)
    {
    struct pci_dev *pdev = priv.pdev;
    let mut num_probed: c_uint = 0;
    char *name;
    int i;
    for (i = 0; i < num_modules; i++) {
    name = modules[i];
    if (!strcmp(name, "") || !strcmp(name, "empty"))
    continue;
    dev_dbg(&priv.pdev.dev, "MODULbus %d: name %s\n", i, name);
    cmodio_setup_subdevice(priv, name, num_probed, i);
    num_probed++;
    }
// print an error message if no modules were probed
    if (num_probed == 0) {
    dev_err(&priv.pdev.dev, "no MODULbus modules specified, "
    "please set the ``modules'' kernel "
    "parameter according to your "
    "hardware configuration\n");
    return -ENODEV;
    }
    return mfd_add_devices(&pdev.dev, 0, priv.cells,
    num_probed, core::ptr::null_mut(), pdev.irq, core::ptr::null_mut());
    }
//
// SYSFS Attributes
//
    static ssize_t modulbus_number_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct cmodio_device *priv = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%x\n", priv.hex);
    }
    static DEVICE_ATTR_RO(modulbus_number);
    static struct attribute *cmodio_sysfs_attrs[] = {
    &dev_attr_modulbus_number.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group cmodio_sysfs_attr_group = {
    .attrs = cmodio_sysfs_attrs,
    };
//
// PCI Driver
//
    static int cmodio_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    struct cmodio_device *priv;
    int ret;
    priv = devm_kzalloc(&dev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    pci_set_drvdata(dev, priv);
    priv.pdev = dev;
// Hardware Initialization
    ret = pci_enable_device(dev);
    if (ret) {
    dev_err(&dev.dev, "unable to enable device\n");
    return ret;
    }
    pci_set_master(dev);
    ret = pci_request_regions(dev, DRV_NAME);
    if (ret) {
    dev_err(&dev.dev, "unable to request regions\n");
    goto out_pci_disable_device;
    }
// Onboard configuration registers
    priv.ctrl = pci_ioremap_bar(dev, 4);
    if (!priv.ctrl) {
    dev_err(&dev.dev, "unable to remap onboard regs\n");
    ret = -ENOMEM;
    goto out_pci_release_regions;
    }
// Read the hex switch on the carrier board
    priv.hex = ioread8(&priv.ctrl.int_enable);
// Add the MODULbus number (hex switch value) to the device's sysfs
    ret = sysfs_create_group(&dev.dev.kobj, &cmodio_sysfs_attr_group);
    if (ret) {
    dev_err(&dev.dev, "unable to create sysfs attributes\n");
    goto out_unmap_ctrl;
    }
//
// Disable all interrupt lines, each submodule will enable its
// own interrupt line if needed
//
    iowrite8(0xf, &priv.ctrl.int_disable);
// Register drivers for all submodules
    ret = cmodio_probe_submodules(priv);
    if (ret) {
    dev_err(&dev.dev, "unable to probe submodules\n");
    goto out_sysfs_remove_group;
    }
    return 0;
    out_sysfs_remove_group:
    sysfs_remove_group(&dev.dev.kobj, &cmodio_sysfs_attr_group);
    out_unmap_ctrl:
    iounmap(priv.ctrl);
    out_pci_release_regions:
    pci_release_regions(dev);
    out_pci_disable_device:
    pci_disable_device(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cmodio_pci_remove(dev: *mut pci_dev) {
    static void cmodio_pci_remove(struct pci_dev *dev)
    {
    struct cmodio_device *priv = pci_get_drvdata(dev);
    mfd_remove_devices(&dev.dev);
    sysfs_remove_group(&dev.dev.kobj, &cmodio_sysfs_attr_group);
    iounmap(priv.ctrl);
    pci_release_regions(dev);
    pci_disable_device(dev);
    }
pub const PCI_VENDOR_ID_JANZ: c_uint = 0x13c3;
// The list of devices that this module will support
    static const struct pci_device_id cmodio_pci_ids[] = {
    { PCI_VENDOR_ID_PLX, PCI_DEVICE_ID_PLX_9030, PCI_VENDOR_ID_JANZ, 0x0101 },
    { PCI_VENDOR_ID_PLX, PCI_DEVICE_ID_PLX_9050, PCI_VENDOR_ID_JANZ, 0x0100 },
    { PCI_VENDOR_ID_PLX, PCI_DEVICE_ID_PLX_9030, PCI_VENDOR_ID_JANZ, 0x0201 },
    { PCI_VENDOR_ID_PLX, PCI_DEVICE_ID_PLX_9030, PCI_VENDOR_ID_JANZ, 0x0202 },
    { PCI_VENDOR_ID_PLX, PCI_DEVICE_ID_PLX_9050, PCI_VENDOR_ID_JANZ, 0x0201 },
    { PCI_VENDOR_ID_PLX, PCI_DEVICE_ID_PLX_9050, PCI_VENDOR_ID_JANZ, 0x0202 },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, cmodio_pci_ids);
    static struct pci_driver cmodio_pci_driver = {
    .name     = DRV_NAME,
    .id_table = cmodio_pci_ids,
    .probe    = cmodio_pci_probe,
    .remove   = cmodio_pci_remove,
    };
    module_pci_driver(cmodio_pci_driver);
    MODULE_AUTHOR("Ira W. Snyder <iws@ovro.caltech.edu>");
    MODULE_DESCRIPTION("Janz CMOD-IO PCI MODULbus Carrier Board Driver");
    MODULE_LICENSE("GPL");
