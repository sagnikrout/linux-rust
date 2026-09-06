//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-designware-pcidrv.c
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
// Synopsys DesignWare I2C adapter driver (master only).
//
// Based on the TI DAVINCI I2C adapter driver.
//
// Copyright (C) 2006 Texas Instruments.
// Copyright (C) 2007 MontaVista Software Inc.
// Copyright (C) 2009 Provigent Ltd.
// Copyright (C) 2011, 2015, 2016 Intel Corporation.
//

    enum dw_pci_ctl_id_t {
    medfield,
    merrifield,
    baytrail,
    cherrytrail,
    haswell,
    elkhartlake,
    navi_amd,
    };
//
// This is a legacy structure to describe the hardware counters
// to configure signal timings on the bus. For Device Tree platforms
// one should use the respective properties and for ACPI there is
// a set of ACPI methods that provide these counters. No new
// platform should use this structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_scl_sda_cfg {
    pub ss_hcnt: u16,
    pub fs_hcnt: u16,
    pub ss_lcnt: u16,
    pub fs_lcnt: u16,
    pub sda_hold_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_pci_controller {
    pub bus_num: u32,
    pub flags: u32,
    pub scl_sda_cfg: *mut dw_scl_sda_cfg,
    pub c): *mut *mut *mut int (setup)(struct pci_dev pdev, struct dw_pci_controller,
    pub dev): *mut *mut u32 (get_clk_rate_khz)(struct dw_i2c_dev,
}

// Merrifield HCNT/LCNT/SDA hold time
    static struct dw_scl_sda_cfg mrfld_config = {
    .ss_hcnt = 0x2f8,
    .fs_hcnt = 0x87,
    .ss_lcnt = 0x37b,
    .fs_lcnt = 0x10a,
    };
// BayTrail HCNT/LCNT/SDA hold time
    static struct dw_scl_sda_cfg byt_config = {
    .ss_hcnt = 0x200,
    .fs_hcnt = 0x55,
    .ss_lcnt = 0x200,
    .fs_lcnt = 0x99,
    .sda_hold_time = 0x6,
    };
// Haswell HCNT/LCNT/SDA hold time
    static struct dw_scl_sda_cfg hsw_config = {
    .ss_hcnt = 0x01b0,
    .fs_hcnt = 0x48,
    .ss_lcnt = 0x01fb,
    .fs_lcnt = 0xa0,
    .sda_hold_time = 0x9,
    };
// NAVI-AMD HCNT/LCNT/SDA hold time
    static struct dw_scl_sda_cfg navi_amd_config = {
    .ss_hcnt = 0x1ae,
    .ss_lcnt = 0x23a,
    .sda_hold_time = 0x9,
    };
#[no_mangle]
unsafe extern "C" fn mfld_get_clk_rate_khz(dev: *mut dw_i2c_dev) -> u32 {
    static u32 mfld_get_clk_rate_khz(struct dw_i2c_dev *dev)
    {
    return 25000;
    }
#[no_mangle]
unsafe extern "C" fn mfld_setup(pdev: *mut pci_dev, c: *mut dw_pci_controller) -> c_int {
    static int mfld_setup(struct pci_dev *pdev, struct dw_pci_controller *c)
    {
    struct dw_i2c_dev *dev = pci_get_drvdata(pdev);
    switch (pdev.device) {
    case 0x0817:
    dev.timings.bus_freq_hz = I2C_MAX_STANDARD_MODE_FREQ;
    fallthrough;
    case 0x0818:
    case 0x0819:
    c.bus_num = pdev.device - 0x817 + 3;
    return 0;
    case 0x082C:
    case 0x082D:
    case 0x082E:
    c.bus_num = pdev.device - 0x82C + 0;
    return 0;
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn mrfld_setup(pdev: *mut pci_dev, c: *mut dw_pci_controller) -> c_int {
    static int mrfld_setup(struct pci_dev *pdev, struct dw_pci_controller *c)
    {
//
// On Intel Merrifield the user visible i2c buses are enumerated
// [1..7]. So, we add 1 to shift the default range. Besides that the
// first PCI slot provides 4 functions, that's why we have to add 0 to
// the first slot and 4 to the next one.
//
    switch (PCI_SLOT(pdev.devfn)) {
    case 8:
    c.bus_num = PCI_FUNC(pdev.devfn) + 0 + 1;
    return 0;
    case 9:
    c.bus_num = PCI_FUNC(pdev.devfn) + 4 + 1;
    return 0;
    }
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn ehl_get_clk_rate_khz(dev: *mut dw_i2c_dev) -> u32 {
    static u32 ehl_get_clk_rate_khz(struct dw_i2c_dev *dev)
    {
    return 100000;
    }
#[no_mangle]
unsafe extern "C" fn navi_amd_get_clk_rate_khz(dev: *mut dw_i2c_dev) -> u32 {
    static u32 navi_amd_get_clk_rate_khz(struct dw_i2c_dev *dev)
    {
    return 100000;
    }
#[no_mangle]
unsafe extern "C" fn navi_amd_setup(pdev: *mut pci_dev, c: *mut dw_pci_controller) -> c_int {
    static int navi_amd_setup(struct pci_dev *pdev, struct dw_pci_controller *c)
    {
    struct dw_i2c_dev *dev = pci_get_drvdata(pdev);
    dev.flags |= MODEL_AMD_NAVI_GPU | ACCESS_POLLING;
    dev.timings.bus_freq_hz = I2C_MAX_STANDARD_MODE_FREQ;
    return 0;
    }
    static struct dw_pci_controller dw_pci_controllers[] = {
    [medfield] = {
    .bus_num = -1,
    .setup = mfld_setup,
    .get_clk_rate_khz = mfld_get_clk_rate_khz,
    },
    [merrifield] = {
    .bus_num = -1,
    .scl_sda_cfg = &mrfld_config,
    .setup = mrfld_setup,
    },
    [baytrail] = {
    .bus_num = -1,
    .scl_sda_cfg = &byt_config,
    },
    [haswell] = {
    .bus_num = -1,
    .scl_sda_cfg = &hsw_config,
    },
    [cherrytrail] = {
    .bus_num = -1,
    .scl_sda_cfg = &byt_config,
    },
    [elkhartlake] = {
    .bus_num = -1,
    .get_clk_rate_khz = ehl_get_clk_rate_khz,
    },
    [navi_amd] = {
    .bus_num = -1,
    .scl_sda_cfg = &navi_amd_config,
    .setup =  navi_amd_setup,
    .get_clk_rate_khz = navi_amd_get_clk_rate_khz,
    },
    };
    static const struct property_entry dgpu_properties[] = {
// USB-C doesn't power the system
    PROPERTY_ENTRY_U8("scope", POWER_SUPPLY_SCOPE_DEVICE),
    {}
    };
    static const struct software_node dgpu_node = {
    .properties = dgpu_properties,
    };
    static int i2c_dw_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct device *device = &pdev.dev;
    struct dw_i2c_dev *dev;
    struct i2c_adapter *adap;
    int r;
    struct dw_pci_controller *controller;
    struct dw_scl_sda_cfg *cfg;
    if (id.driver_data >= ARRAY_SIZE(dw_pci_controllers))
    return dev_err_probe(device, -EINVAL, "Invalid driver data %ld\n",
    id.driver_data);
    controller = &dw_pci_controllers[id.driver_data];
    r = pcim_enable_device(pdev);
    if (r)
    return dev_err_probe(device, r, "Failed to enable I2C PCI device\n");
    pci_set_master(pdev);
    r = pcim_iomap_regions(pdev, 1 << 0, pci_name(pdev));
    if (r)
    return dev_err_probe(device, r, "I/O memory remapping failed\n");
    dev = devm_kzalloc(device, sizeof(*dev), GFP_KERNEL);
    if (!dev)
    return -ENOMEM;
    r = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_ALL_TYPES);
    if (r < 0)
    return r;
    dev.get_clk_rate_khz = controller.get_clk_rate_khz;
    dev.base = pcim_iomap_table(pdev)[0];
    dev.dev = device;
    dev.irq = pci_irq_vector(pdev, 0);
    dev.flags |= controller.flags;
    pci_set_drvdata(pdev, dev);
    if (controller.setup) {
    r = controller.setup(pdev, controller);
    if (r)
    return r;
    }
    r = i2c_dw_fw_parse_and_configure(dev);
    if (r)
    return r;
    i2c_dw_configure(dev);
    if (controller.scl_sda_cfg) {
    cfg = controller.scl_sda_cfg;
    dev.ss_hcnt = cfg.ss_hcnt;
    dev.fs_hcnt = cfg.fs_hcnt;
    dev.ss_lcnt = cfg.ss_lcnt;
    dev.fs_lcnt = cfg.fs_lcnt;
    dev.sda_hold_time = cfg.sda_hold_time;
    }
    adap = &dev.adapter;
    adap.owner = THIS_MODULE;
    adap.class = 0;
    adap.nr = controller.bus_num;
    r = i2c_dw_probe(dev);
    if (r)
    return r;
    if ((dev.flags & MODEL_MASK) == MODEL_AMD_NAVI_GPU) {
    dev.slave = i2c_new_ccgx_ucsi(&dev.adapter, dev.irq, &dgpu_node);
    if (IS_ERR(dev.slave)) {
    i2c_del_adapter(&dev.adapter);
    return dev_err_probe(device, PTR_ERR(dev.slave),
    "register UCSI failed\n");
    }
    }
    pm_runtime_set_autosuspend_delay(device, 1000);
    pm_runtime_use_autosuspend(device);
    pm_runtime_put_autosuspend(device);
    pm_runtime_allow(device);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_dw_pci_remove(pdev: *mut pci_dev) {
    static void i2c_dw_pci_remove(struct pci_dev *pdev)
    {
    struct dw_i2c_dev *dev = pci_get_drvdata(pdev);
    struct device *device = &pdev.dev;
    i2c_dw_disable(dev);
    pm_runtime_forbid(device);
    pm_runtime_get_noresume(device);
    i2c_del_adapter(&dev.adapter);
    }
    static const struct pci_device_id i2c_designware_pci_ids[] = {
// Medfield
    { PCI_VDEVICE(INTEL, 0x0817), .driver_data = medfield },
    { PCI_VDEVICE(INTEL, 0x0818), .driver_data = medfield },
    { PCI_VDEVICE(INTEL, 0x0819), .driver_data = medfield },
    { PCI_VDEVICE(INTEL, 0x082C), .driver_data = medfield },
    { PCI_VDEVICE(INTEL, 0x082D), .driver_data = medfield },
    { PCI_VDEVICE(INTEL, 0x082E), .driver_data = medfield },
// Merrifield
    { PCI_VDEVICE(INTEL, 0x1195), .driver_data = merrifield },
    { PCI_VDEVICE(INTEL, 0x1196), .driver_data = merrifield },
// Baytrail
    { PCI_VDEVICE(INTEL, 0x0F41), .driver_data = baytrail },
    { PCI_VDEVICE(INTEL, 0x0F42), .driver_data = baytrail },
    { PCI_VDEVICE(INTEL, 0x0F43), .driver_data = baytrail },
    { PCI_VDEVICE(INTEL, 0x0F44), .driver_data = baytrail },
    { PCI_VDEVICE(INTEL, 0x0F45), .driver_data = baytrail },
    { PCI_VDEVICE(INTEL, 0x0F46), .driver_data = baytrail },
    { PCI_VDEVICE(INTEL, 0x0F47), .driver_data = baytrail },
// Haswell
    { PCI_VDEVICE(INTEL, 0x9c61), .driver_data = haswell },
    { PCI_VDEVICE(INTEL, 0x9c62), .driver_data = haswell },
// Braswell / Cherrytrail
    { PCI_VDEVICE(INTEL, 0x22C1), .driver_data = cherrytrail },
    { PCI_VDEVICE(INTEL, 0x22C2), .driver_data = cherrytrail },
    { PCI_VDEVICE(INTEL, 0x22C3), .driver_data = cherrytrail },
    { PCI_VDEVICE(INTEL, 0x22C4), .driver_data = cherrytrail },
    { PCI_VDEVICE(INTEL, 0x22C5), .driver_data = cherrytrail },
    { PCI_VDEVICE(INTEL, 0x22C6), .driver_data = cherrytrail },
    { PCI_VDEVICE(INTEL, 0x22C7), .driver_data = cherrytrail },
// Elkhart Lake (PSE I2C)
    { PCI_VDEVICE(INTEL, 0x4bb9), .driver_data = elkhartlake },
    { PCI_VDEVICE(INTEL, 0x4bba), .driver_data = elkhartlake },
    { PCI_VDEVICE(INTEL, 0x4bbb), .driver_data = elkhartlake },
    { PCI_VDEVICE(INTEL, 0x4bbc), .driver_data = elkhartlake },
    { PCI_VDEVICE(INTEL, 0x4bbd), .driver_data = elkhartlake },
    { PCI_VDEVICE(INTEL, 0x4bbe), .driver_data = elkhartlake },
    { PCI_VDEVICE(INTEL, 0x4bbf), .driver_data = elkhartlake },
    { PCI_VDEVICE(INTEL, 0x4bc0), .driver_data = elkhartlake },
// AMD NAVI
    { PCI_VDEVICE(ATI,  0x7314), .driver_data = navi_amd },
    { PCI_VDEVICE(ATI,  0x73a4), .driver_data = navi_amd },
    { PCI_VDEVICE(ATI,  0x73e4), .driver_data = navi_amd },
    { PCI_VDEVICE(ATI,  0x73c4), .driver_data = navi_amd },
    { PCI_VDEVICE(ATI,  0x7444), .driver_data = navi_amd },
    { PCI_VDEVICE(ATI,  0x7464), .driver_data = navi_amd },
    { }
    };
    MODULE_DEVICE_TABLE(pci, i2c_designware_pci_ids);
#[no_mangle]
unsafe extern "C" fn i2c_dw_pci_shutdown(pdev: *mut pci_dev) {
    static void i2c_dw_pci_shutdown(struct pci_dev *pdev)
    {
    struct dw_i2c_dev *i_dev;
    i_dev = pci_get_drvdata(pdev);
    if (!i_dev)
    return;
    pm_runtime_disable(&pdev.dev);
    if (!pm_runtime_status_suspended(&pdev.dev))
    i2c_dw_shutdown(i_dev);
    }
    static struct pci_driver dw_i2c_driver = {
    .name		= DRIVER_NAME,
    .probe		= i2c_dw_pci_probe,
    .remove		= i2c_dw_pci_remove,
    .shutdown	= i2c_dw_pci_shutdown,
    .driver         = {
    .pm	= pm_ptr(&i2c_dw_dev_pm_ops),
    },
    .id_table	= i2c_designware_pci_ids,
    };
    module_pci_driver(dw_i2c_driver);
    MODULE_AUTHOR("Baruch Siach <baruch@tkos.co.il>");
    MODULE_DESCRIPTION("Synopsys DesignWare PCI I2C bus adapter");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("I2C_DW");
    MODULE_IMPORT_NS("I2C_DW_COMMON");
