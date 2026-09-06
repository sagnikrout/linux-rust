//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/intel_quark_i2c_gpio.c
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
// Intel Quark MFD PCI driver for I2C & GPIO
//
// Copyright(c) 2014 Intel Corporation.
//
// Intel Quark PCI device for I2C and GPIO controller sharing the same
// PCI function. This PCI driver will split the 2 devices into their
// respective drivers.
//

// PCI BAR for register base address
pub const MFD_I2C_BAR: c_int = 0;
pub const MFD_GPIO_BAR: c_int = 1;
// ACPI _ADR value to match the child node

pub const INTEL_QUARK_IORES_MEM: c_int = 0;
pub const INTEL_QUARK_IORES_IRQ: c_int = 1;

// The Quark I2C controller source clock
pub const INTEL_QUARK_I2C_CLK_HZ: c_int = 33000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_quark_mfd {
    pub i2c_clk: *mut clk,
    pub i2c_clk_lookup: *mut clk_lookup,
}

    static const struct property_entry intel_quark_i2c_controller_standard_properties[] = {
    PROPERTY_ENTRY_U32("clock-frequency", I2C_MAX_STANDARD_MODE_FREQ),
    { }
    };
    static const struct software_node intel_quark_i2c_controller_standard_node = {
    .name = "intel-quark-i2c-controller",
    .properties = intel_quark_i2c_controller_standard_properties,
    };
    static const struct property_entry intel_quark_i2c_controller_fast_properties[] = {
    PROPERTY_ENTRY_U32("clock-frequency", I2C_MAX_FAST_MODE_FREQ),
    { }
    };
    static const struct software_node intel_quark_i2c_controller_fast_node = {
    .name = "intel-quark-i2c-controller",
    .properties = intel_quark_i2c_controller_fast_properties,
    };
    static const struct dmi_system_id dmi_platform_info[] = {
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "Galileo"),
    },
    .driver_data = (void *)&intel_quark_i2c_controller_standard_node,
    },
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "GalileoGen2"),
    },
    .driver_data = (void *)&intel_quark_i2c_controller_fast_node,
    },
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "SIMATIC IOT2000"),
    },
    .driver_data = (void *)&intel_quark_i2c_controller_fast_node,
    },
    {}
    };
// This is used as a place holder and will be modified at run-time
    static struct resource intel_quark_i2c_res[] = {
    [INTEL_QUARK_IORES_MEM] = {
    .flags = IORESOURCE_MEM,
    },
    [INTEL_QUARK_IORES_IRQ] = {
    .flags = IORESOURCE_IRQ,
    },
    };
    static struct mfd_cell_acpi_match intel_quark_acpi_match_i2c = {
    .adr = MFD_ACPI_MATCH_I2C,
    };
// This is used as a place holder and will be modified at run-time
    static struct resource intel_quark_gpio_res[] = {
    [INTEL_QUARK_IORES_MEM] = {
    .flags = IORESOURCE_MEM,
    },
    [INTEL_QUARK_IORES_IRQ] = {
    .flags = IORESOURCE_IRQ,
    },
    };
    static struct mfd_cell_acpi_match intel_quark_acpi_match_gpio = {
    .adr = MFD_ACPI_MATCH_GPIO,
    };
    static const struct software_node intel_quark_gpio_controller_node = {
    .name = "intel-quark-gpio-controller",
    };
    static const struct property_entry intel_quark_gpio_portA_properties[] = {
    PROPERTY_ENTRY_U32("reg", 0),
    PROPERTY_ENTRY_U32("snps,nr-gpios", 8),
    PROPERTY_ENTRY_U32("gpio-base", 8),
    { }
    };
    static const struct software_node intel_quark_gpio_portA_node = {
    .name = "portA",
    .parent = &intel_quark_gpio_controller_node,
    .properties = intel_quark_gpio_portA_properties,
    };
    static const struct software_node *intel_quark_gpio_node_group[] = {
    &intel_quark_gpio_controller_node,
    &intel_quark_gpio_portA_node,
    core::ptr::null_mut()
    };
    static struct mfd_cell intel_quark_mfd_cells[] = {
    [MFD_I2C_BAR] = {
    .id = MFD_I2C_BAR,
    .name = "i2c_designware",
    .acpi_match = &intel_quark_acpi_match_i2c,
    .num_resources = ARRAY_SIZE(intel_quark_i2c_res),
    .resources = intel_quark_i2c_res,
    .ignore_resource_conflicts = true,
    },
    [MFD_GPIO_BAR] = {
    .id = MFD_GPIO_BAR,
    .name = "gpio-dwapb",
    .acpi_match = &intel_quark_acpi_match_gpio,
    .num_resources = ARRAY_SIZE(intel_quark_gpio_res),
    .resources = intel_quark_gpio_res,
    .ignore_resource_conflicts = true,
    },
    };
    static const struct pci_device_id intel_quark_mfd_ids[] = {
    { PCI_VDEVICE(INTEL, 0x0934), },
    {},
    };
    MODULE_DEVICE_TABLE(pci, intel_quark_mfd_ids);
#[no_mangle]
unsafe extern "C" fn intel_quark_register_i2c_clk(dev: *mut device) -> c_int {
    static int intel_quark_register_i2c_clk(struct device *dev)
    {
    struct intel_quark_mfd *quark_mfd = dev_get_drvdata(dev);
    struct clk *i2c_clk;
    i2c_clk = clk_register_fixed_rate(dev,
    INTEL_QUARK_I2C_CONTROLLER_CLK, core::ptr::null_mut(),
    0, INTEL_QUARK_I2C_CLK_HZ);
    if (IS_ERR(i2c_clk))
    return PTR_ERR(i2c_clk);
    quark_mfd.i2c_clk = i2c_clk;
    quark_mfd.i2c_clk_lookup = clkdev_create(i2c_clk, core::ptr::null_mut(),
    INTEL_QUARK_I2C_CONTROLLER_CLK);
    if (!quark_mfd.i2c_clk_lookup) {
    clk_unregister(quark_mfd.i2c_clk);
    dev_err(dev, "Fixed clk register failed\n");
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_quark_unregister_i2c_clk(dev: *mut device) {
    static void intel_quark_unregister_i2c_clk(struct device *dev)
    {
    struct intel_quark_mfd *quark_mfd = dev_get_drvdata(dev);
    if (!quark_mfd.i2c_clk_lookup)
    return;
    clkdev_drop(quark_mfd.i2c_clk_lookup);
    clk_unregister(quark_mfd.i2c_clk);
    }
#[no_mangle]
unsafe extern "C" fn intel_quark_i2c_setup(pdev: *mut pci_dev) -> c_int {
    static int intel_quark_i2c_setup(struct pci_dev *pdev)
    {
    struct mfd_cell *cell = &intel_quark_mfd_cells[MFD_I2C_BAR];
    struct resource *res = intel_quark_i2c_res;
    const struct dmi_system_id *dmi_id;
    res[INTEL_QUARK_IORES_MEM].start = pci_resource_start(pdev, MFD_I2C_BAR);
    res[INTEL_QUARK_IORES_MEM].end = pci_resource_end(pdev, MFD_I2C_BAR);
    res[INTEL_QUARK_IORES_IRQ].start = pci_irq_vector(pdev, 0);
    res[INTEL_QUARK_IORES_IRQ].end = pci_irq_vector(pdev, 0);
// Normal mode by default
    cell.swnode = &intel_quark_i2c_controller_standard_node;
    dmi_id = dmi_first_match(dmi_platform_info);
    if (dmi_id)
    cell.swnode = (struct software_node *)dmi_id.driver_data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_quark_gpio_setup(pdev: *mut pci_dev) -> c_int {
    static int intel_quark_gpio_setup(struct pci_dev *pdev)
    {
    struct mfd_cell *cell = &intel_quark_mfd_cells[MFD_GPIO_BAR];
    struct resource *res = intel_quark_gpio_res;
    int ret;
    res[INTEL_QUARK_IORES_MEM].start = pci_resource_start(pdev, MFD_GPIO_BAR);
    res[INTEL_QUARK_IORES_MEM].end = pci_resource_end(pdev, MFD_GPIO_BAR);
    res[INTEL_QUARK_IORES_IRQ].start = pci_irq_vector(pdev, 0);
    res[INTEL_QUARK_IORES_IRQ].end = pci_irq_vector(pdev, 0);
    ret = software_node_register_node_group(intel_quark_gpio_node_group);
    if (ret)
    return ret;
    cell.swnode = &intel_quark_gpio_controller_node;
    return 0;
    }
    static int intel_quark_mfd_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct intel_quark_mfd *quark_mfd;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    quark_mfd = devm_kzalloc(&pdev.dev, sizeof(*quark_mfd), GFP_KERNEL);
    if (!quark_mfd)
    return -ENOMEM;
    dev_set_drvdata(&pdev.dev, quark_mfd);
    ret = intel_quark_register_i2c_clk(&pdev.dev);
    if (ret)
    return ret;
    pci_set_master(pdev);
// This driver only requires 1 IRQ vector
    ret = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_ALL_TYPES);
    if (ret < 0)
    goto err_unregister_i2c_clk;
    ret = intel_quark_i2c_setup(pdev);
    if (ret)
    goto err_free_irq_vectors;
    ret = intel_quark_gpio_setup(pdev);
    if (ret)
    goto err_free_irq_vectors;
    ret = mfd_add_devices(&pdev.dev, 0, intel_quark_mfd_cells,
    ARRAY_SIZE(intel_quark_mfd_cells), core::ptr::null_mut(), 0,
    core::ptr::null_mut());
    if (ret)
    goto err_unregister_gpio_node_group;
    return 0;
    err_unregister_gpio_node_group:
    software_node_unregister_node_group(intel_quark_gpio_node_group);
    err_free_irq_vectors:
    pci_free_irq_vectors(pdev);
    err_unregister_i2c_clk:
    intel_quark_unregister_i2c_clk(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn intel_quark_mfd_remove(pdev: *mut pci_dev) {
    static void intel_quark_mfd_remove(struct pci_dev *pdev)
    {
    mfd_remove_devices(&pdev.dev);
    software_node_unregister_node_group(intel_quark_gpio_node_group);
    pci_free_irq_vectors(pdev);
    intel_quark_unregister_i2c_clk(&pdev.dev);
    }
    static struct pci_driver intel_quark_mfd_driver = {
    .name		= "intel_quark_mfd_i2c_gpio",
    .id_table	= intel_quark_mfd_ids,
    .probe		= intel_quark_mfd_probe,
    .remove		= intel_quark_mfd_remove,
    };
    module_pci_driver(intel_quark_mfd_driver);
    MODULE_AUTHOR("Raymond Tan <raymond.tan@intel.com>");
    MODULE_DESCRIPTION("Intel Quark MFD PCI driver for I2C & GPIO");
    MODULE_LICENSE("GPL v2");
