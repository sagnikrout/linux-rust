//! Automatically rewritten from C to Rust
//! Source: drivers/usb/dwc3/dwc3-pci.c
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
// dwc3-pci.c - PCI Specific glue layer
//
// Copyright (C) 2010-2011 Texas Instruments Incorporated - https://www.ti.com
//
// Authors: Felipe Balbi <balbi@ti.com>,
// Sebastian Andrzej Siewior <bigeasy@linutronix.de>
//

pub const PCI_DEVICE_ID_INTEL_CMLLP: c_uint = 0x02ee;
pub const PCI_DEVICE_ID_INTEL_CMLH: c_uint = 0x06ee;
pub const PCI_DEVICE_ID_INTEL_BXT: c_uint = 0x0aaa;
pub const PCI_DEVICE_ID_INTEL_BYT: c_uint = 0x0f37;
pub const PCI_DEVICE_ID_INTEL_MRFLD: c_uint = 0x119e;
pub const PCI_DEVICE_ID_INTEL_BXT_M: c_uint = 0x1aaa;
pub const PCI_DEVICE_ID_INTEL_BSW: c_uint = 0x22b7;
pub const PCI_DEVICE_ID_INTEL_GLK: c_uint = 0x31aa;
pub const PCI_DEVICE_ID_INTEL_ICLLP: c_uint = 0x34ee;
pub const PCI_DEVICE_ID_INTEL_TGPH: c_uint = 0x43ee;
pub const PCI_DEVICE_ID_INTEL_ADL: c_uint = 0x460e;
pub const PCI_DEVICE_ID_INTEL_ADLN: c_uint = 0x465e;
pub const PCI_DEVICE_ID_INTEL_EHL: c_uint = 0x4b7e;
pub const PCI_DEVICE_ID_INTEL_WCL: c_uint = 0x4d7e;
pub const PCI_DEVICE_ID_INTEL_JSP: c_uint = 0x4dee;
pub const PCI_DEVICE_ID_INTEL_ADL_PCH: c_uint = 0x51ee;
pub const PCI_DEVICE_ID_INTEL_ADLN_PCH: c_uint = 0x54ee;
pub const PCI_DEVICE_ID_INTEL_APL: c_uint = 0x5aaa;
pub const PCI_DEVICE_ID_INTEL_NVLS_PCH: c_uint = 0x6e6f;
pub const PCI_DEVICE_ID_INTEL_ARLH_PCH: c_uint = 0x777e;
pub const PCI_DEVICE_ID_INTEL_RPLS: c_uint = 0x7a61;
pub const PCI_DEVICE_ID_INTEL_MTL: c_uint = 0x7e7e;
pub const PCI_DEVICE_ID_INTEL_ADLS: c_uint = 0x7ae1;
pub const PCI_DEVICE_ID_INTEL_MTLM: c_uint = 0x7eb1;
pub const PCI_DEVICE_ID_INTEL_MTLP: c_uint = 0x7ec1;
pub const PCI_DEVICE_ID_INTEL_MTLS: c_uint = 0x7f6f;
pub const PCI_DEVICE_ID_INTEL_TGL: c_uint = 0x9a15;
pub const PCI_DEVICE_ID_INTEL_SPTLP: c_uint = 0x9d30;
pub const PCI_DEVICE_ID_INTEL_CNPLP: c_uint = 0x9dee;
pub const PCI_DEVICE_ID_INTEL_TGPLP: c_uint = 0xa0ee;
pub const PCI_DEVICE_ID_INTEL_SPTH: c_uint = 0xa130;
pub const PCI_DEVICE_ID_INTEL_KBP: c_uint = 0xa2b0;
pub const PCI_DEVICE_ID_INTEL_CNPH: c_uint = 0xa36e;
pub const PCI_DEVICE_ID_INTEL_CNPV: c_uint = 0xa3b0;
pub const PCI_DEVICE_ID_INTEL_RPL: c_uint = 0xa70e;
pub const PCI_DEVICE_ID_INTEL_NVLH: c_uint = 0xd37f;
pub const PCI_DEVICE_ID_INTEL_PTLH: c_uint = 0xe332;
pub const PCI_DEVICE_ID_INTEL_PTLH_PCH: c_uint = 0xe37e;
pub const PCI_DEVICE_ID_INTEL_PTLU: c_uint = 0xe432;
pub const PCI_DEVICE_ID_INTEL_PTLU_PCH: c_uint = 0xe47e;
pub const PCI_DEVICE_ID_AMD_MR: c_uint = 0x163a;

pub const PCI_INTEL_BXT_FUNC_PMU_PWR: c_int = 4;
pub const PCI_INTEL_BXT_STATE_D0: c_int = 0;
pub const PCI_INTEL_BXT_STATE_D3: c_int = 3;
pub const GP_RWBAR: c_int = 1;
pub const GP_RWREG1: c_uint = 0xa0;

//
// struct dwc3_pci - Driver private structure
// @dwc3: child dwc3 platform_device
// @pci: our link to PCI bus
// @guid: _DSM GUID
// @has_dsm_for_pm: true for devices which need to run _DSM on runtime PM
// @wakeup_work: work for asynchronous resume
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc3_pci {
    pub dwc3: *mut platform_device,
    pub pci: *mut pci_dev,
    pub guid: guid_t,
    pub has_dsm_for_pm:1: c_uint,
    pub wakeup_work: work_struct,
}

    let mut reset_gpios: static struct acpi_gpio_params = { 0, 0, false };
    let mut cs_gpios: static struct acpi_gpio_params = { 1, 0, false };
    static const struct acpi_gpio_mapping acpi_dwc3_byt_gpios[] = {
    { "reset-gpios", &reset_gpios, 1 },
    { "cs-gpios", &cs_gpios, 1 },
    { },
    };
    static struct gpiod_lookup_table platform_bytcr_gpios = {
    .dev_id		= "0000:00:16.0",
    .table		= {
    GPIO_LOOKUP("INT33FC:00", 54, "cs", GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP("INT33FC:02", 14, "reset", GPIO_ACTIVE_HIGH),
    {}
    },
    };
#[no_mangle]
unsafe extern "C" fn dwc3_byt_enable_ulpi_refclock(pci: *mut pci_dev) -> c_int {
    static int dwc3_byt_enable_ulpi_refclock(struct pci_dev *pci)
    {
    void __iomem	*reg;
    u32		value;
    reg = pcim_iomap(pci, GP_RWBAR, 0);
    if (!reg)
    return -ENOMEM;
    value = readl(reg + GP_RWREG1);
    if (!(value & GP_RWREG1_ULPI_REFCLK_DISABLE))
    goto unmap; /* ULPI refclk already enabled */
    value &= ~GP_RWREG1_ULPI_REFCLK_DISABLE;
    writel(value, reg + GP_RWREG1);
// This comes from the Intel Android x86 tree w/o any explanation
    msleep(100);
    unmap:
    pcim_iounmap(pci, reg);
    return 0;
    }
    static const struct property_entry dwc3_pci_intel_properties[] = {
    PROPERTY_ENTRY_STRING("dr_mode", "peripheral"),
    PROPERTY_ENTRY_BOOL("linux,sysdev_is_parent"),
    {}
    };
    static const struct property_entry dwc3_pci_intel_phy_charger_detect_properties[] = {
    PROPERTY_ENTRY_STRING("dr_mode", "peripheral"),
    PROPERTY_ENTRY_BOOL("snps,dis_u2_susphy_quirk"),
    PROPERTY_ENTRY_BOOL("linux,phy_charger_detect"),
    PROPERTY_ENTRY_BOOL("linux,sysdev_is_parent"),
    {}
    };
    static const struct property_entry dwc3_pci_intel_byt_properties[] = {
    PROPERTY_ENTRY_STRING("dr_mode", "peripheral"),
    PROPERTY_ENTRY_BOOL("snps,dis_u2_susphy_quirk"),
    PROPERTY_ENTRY_BOOL("linux,sysdev_is_parent"),
    {}
    };
//
// Intel Merrifield SoC uses these endpoints for tracing and they cannot
// be re-allocated if being used because the side band flow control signals
// are hard wired to certain endpoints:
// - 1 High BW Bulk IN (IN#1) (RTIT)
// - 1 1KB BW Bulk IN (IN#8) + 1 1KB BW Bulk OUT (Run Control) (OUT#8)
//
    static const u8 dwc3_pci_mrfld_reserved_endpoints[] = { 3, 16, 17 };
    static const struct property_entry dwc3_pci_mrfld_properties[] = {
    PROPERTY_ENTRY_STRING("dr_mode", "otg"),
    PROPERTY_ENTRY_STRING("linux,extcon-name", "mrfld_bcove_pwrsrc"),
    PROPERTY_ENTRY_BOOL("snps,dis_u3_susphy_quirk"),
    PROPERTY_ENTRY_BOOL("snps,dis_u2_susphy_quirk"),
    PROPERTY_ENTRY_U8_ARRAY("snps,reserved-endpoints", dwc3_pci_mrfld_reserved_endpoints),
    PROPERTY_ENTRY_BOOL("snps,usb2-gadget-lpm-disable"),
    PROPERTY_ENTRY_BOOL("linux,sysdev_is_parent"),
    {}
    };
    static const struct property_entry dwc3_pci_amd_properties[] = {
    PROPERTY_ENTRY_BOOL("snps,has-lpm-erratum"),
    PROPERTY_ENTRY_U8("snps,lpm-nyet-threshold", 0xf),
    PROPERTY_ENTRY_BOOL("snps,u2exit_lfps_quirk"),
    PROPERTY_ENTRY_BOOL("snps,u2ss_inp3_quirk"),
    PROPERTY_ENTRY_BOOL("snps,req_p1p2p3_quirk"),
    PROPERTY_ENTRY_BOOL("snps,del_p1p2p3_quirk"),
    PROPERTY_ENTRY_BOOL("snps,del_phy_power_chg_quirk"),
    PROPERTY_ENTRY_BOOL("snps,lfps_filter_quirk"),
    PROPERTY_ENTRY_BOOL("snps,rx_detect_poll_quirk"),
    PROPERTY_ENTRY_BOOL("snps,tx_de_emphasis_quirk"),
    PROPERTY_ENTRY_U8("snps,tx_de_emphasis", 1),
// FIXME these quirks should be removed when AMD NL tapes out
    PROPERTY_ENTRY_BOOL("snps,disable_scramble_quirk"),
    PROPERTY_ENTRY_BOOL("snps,dis_u3_susphy_quirk"),
    PROPERTY_ENTRY_BOOL("snps,dis_u2_susphy_quirk"),
    PROPERTY_ENTRY_BOOL("linux,sysdev_is_parent"),
    {}
    };
    static const struct property_entry dwc3_pci_mr_properties[] = {
    PROPERTY_ENTRY_STRING("dr_mode", "otg"),
    PROPERTY_ENTRY_BOOL("usb-role-switch"),
    PROPERTY_ENTRY_STRING("role-switch-default-mode", "host"),
    PROPERTY_ENTRY_BOOL("linux,sysdev_is_parent"),
    {}
    };
    static const struct software_node dwc3_pci_intel_swnode = {
    .properties = dwc3_pci_intel_properties,
    };
    static const struct software_node dwc3_pci_intel_phy_charger_detect_swnode = {
    .properties = dwc3_pci_intel_phy_charger_detect_properties,
    };
    static const struct software_node dwc3_pci_intel_byt_swnode = {
    .properties = dwc3_pci_intel_byt_properties,
    };
    static const struct software_node dwc3_pci_intel_mrfld_swnode = {
    .properties = dwc3_pci_mrfld_properties,
    };
    static const struct software_node dwc3_pci_amd_swnode = {
    .properties = dwc3_pci_amd_properties,
    };
    static const struct software_node dwc3_pci_amd_mr_swnode = {
    .properties = dwc3_pci_mr_properties,
    };
    static int dwc3_pci_quirks(struct dwc3_pci *dwc,
    const struct software_node *swnode)
    {
    struct pci_dev			*pdev = dwc.pci;
    if (pdev.vendor == PCI_VENDOR_ID_INTEL) {
    if (pdev.device == PCI_DEVICE_ID_INTEL_BXT ||
    pdev.device == PCI_DEVICE_ID_INTEL_BXT_M ||
    pdev.device == PCI_DEVICE_ID_INTEL_EHL) {
    guid_parse(PCI_INTEL_BXT_DSM_GUID, &dwc.guid);
    dwc.has_dsm_for_pm = true;
    }
    if (pdev.device == PCI_DEVICE_ID_INTEL_BYT) {
    struct gpio_desc *gpio;
    const char *bios_ver;
    int ret;
// On BYT the FW does not always enable the refclock
    ret = dwc3_byt_enable_ulpi_refclock(pdev);
    if (ret)
    return ret;
    ret = devm_acpi_dev_add_driver_gpios(&pdev.dev,
    acpi_dwc3_byt_gpios);
    if (ret)
    dev_dbg(&pdev.dev, "failed to add mapping table\n");
//
// A lot of BYT devices lack ACPI resource entries for
// the GPIOs. If the ACPI entry for the GPIO controller
// is present add a fallback mapping to the reference
// design GPIOs which all boards seem to use.
//
    if (acpi_dev_present("INT33FC", core::ptr::null_mut(), -1))
    gpiod_add_lookup_table(&platform_bytcr_gpios);
//
// These GPIOs will turn on the USB2 PHY. Note that we have to
// put the gpio descriptors again here because the phy driver
// might want to grab them, too.
//
    gpio = gpiod_get_optional(&pdev.dev, "cs", GPIOD_OUT_LOW);
    if (IS_ERR(gpio))
    return PTR_ERR(gpio);
    gpiod_set_value_cansleep(gpio, 1);
    gpiod_put(gpio);
    gpio = gpiod_get_optional(&pdev.dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(gpio))
    return PTR_ERR(gpio);
    if (gpio) {
    gpiod_set_value_cansleep(gpio, 1);
    gpiod_put(gpio);
    usleep_range(10000, 11000);
    }
//
// Make the pdev name predictable (only 1 DWC3 on BYT)
// and patch the phy dev-name into the lookup table so
// that the phy-driver can get the GPIOs.
//
    dwc.dwc3.id = PLATFORM_DEVID_NONE;
    platform_bytcr_gpios.dev_id = "dwc3.ulpi";
//
// Some Android tablets with a Crystal Cove PMIC
// (INT33FD), rely on the TUSB1211 phy for charger
// detection. These can be identified by them _not_
// using the standard ACPI battery and ac drivers.
//
    bios_ver = dmi_get_system_info(DMI_BIOS_VERSION);
    if (acpi_dev_present("INT33FD", "1", 2) &&
    acpi_quirk_skip_acpi_ac_and_battery() &&
// Lenovo Yoga Tablet 2 Pro 1380 uses LC824206XA instead
    !(bios_ver &&
    strstarts(bios_ver, "BLADE_21.X64.0005.R00.1504101516"))) {
    dev_info(&pdev.dev, "Using TUSB1211 phy for charger detection\n");
    swnode = &dwc3_pci_intel_phy_charger_detect_swnode;
    }
    }
    }
    return device_add_software_node(&dwc.dwc3.dev, swnode);
    }

#[no_mangle]
unsafe extern "C" fn dwc3_pci_resume_work(work: *mut work_struct) {
    static void dwc3_pci_resume_work(struct work_struct *work)
    {
    struct dwc3_pci *dwc = container_of(work, struct dwc3_pci, wakeup_work);
    struct platform_device *dwc3 = dwc.dwc3;
    int ret;
    ret = pm_runtime_get_sync(&dwc3.dev);
    if (ret < 0) {
    pm_runtime_put_sync_autosuspend(&dwc3.dev);
    return;
    }
    pm_runtime_put_sync_autosuspend(&dwc3.dev);
    }

#[no_mangle]
unsafe extern "C" fn dwc3_pci_probe(pci: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int dwc3_pci_probe(struct pci_dev *pci, const struct pci_device_id *id)
    {
    struct dwc3_pci		*dwc;
    struct resource		res[2];
    int			ret;
    struct device		*dev = &pci.dev;
    ret = pcim_enable_device(pci);
    if (ret) {
    dev_err(dev, "failed to enable pci device\n");
    return -ENODEV;
    }
    pci_set_master(pci);
    dwc = devm_kzalloc(dev, sizeof(*dwc), GFP_KERNEL);
    if (!dwc)
    return -ENOMEM;
    dwc.dwc3 = platform_device_alloc("dwc3", PLATFORM_DEVID_AUTO);
    if (!dwc.dwc3)
    return -ENOMEM;
    memset(res, 0x00, sizeof(struct resource) * ARRAY_SIZE(res));
    res[0].start	= pci_resource_start(pci, 0);
    res[0].end	= pci_resource_end(pci, 0);
    res[0].name	= "dwc_usb3";
    res[0].flags	= IORESOURCE_MEM;
    res[1].start	= pci.irq;
    res[1].name	= "dwc_usb3";
    res[1].flags	= IORESOURCE_IRQ;
    ret = platform_device_add_resources(dwc.dwc3, res, ARRAY_SIZE(res));
    if (ret) {
    dev_err(dev, "couldn't add resources to dwc3 device\n");
    goto err;
    }
    dwc.pci = pci;
    dwc.dwc3.dev.parent = dev;
    ACPI_COMPANION_SET(&dwc.dwc3.dev, ACPI_COMPANION(dev));
    ret = dwc3_pci_quirks(dwc, (void *)id.driver_data);
    if (ret)
    goto err;
    ret = platform_device_add(dwc.dwc3);
    if (ret) {
    dev_err(dev, "failed to register dwc3 device\n");
    goto err;
    }
    device_init_wakeup(dev, true);
    pci_set_drvdata(pci, dwc);
    pm_runtime_put(dev);

    INIT_WORK(&dwc.wakeup_work, dwc3_pci_resume_work);

    return 0;
    err:
    device_remove_software_node(&dwc.dwc3.dev);
    platform_device_put(dwc.dwc3);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_pci_remove(pci: *mut pci_dev) {
    static void dwc3_pci_remove(struct pci_dev *pci)
    {
    struct dwc3_pci		*dwc = pci_get_drvdata(pci);
    struct pci_dev		*pdev = dwc.pci;
    if (pdev.device == PCI_DEVICE_ID_INTEL_BYT)
    gpiod_remove_lookup_table(&platform_bytcr_gpios);

    cancel_work_sync(&dwc.wakeup_work);

    device_init_wakeup(&pci.dev, false);
    pm_runtime_get(&pci.dev);
    device_remove_software_node(&dwc.dwc3.dev);
    platform_device_unregister(dwc.dwc3);
    }
    static const struct pci_device_id dwc3_pci_id_table[] = {
    { PCI_DEVICE_DATA(INTEL, CMLLP, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, CMLH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, BXT, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, BYT, &dwc3_pci_intel_byt_swnode) },
    { PCI_DEVICE_DATA(INTEL, MRFLD, &dwc3_pci_intel_mrfld_swnode) },
    { PCI_DEVICE_DATA(INTEL, BXT_M, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, BSW, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, GLK, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, ICLLP, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, TGPH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, ADL, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, ADLN, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, EHL, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, WCL, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, JSP, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, ADL_PCH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, ADLN_PCH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, APL, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, NVLS_PCH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, ARLH_PCH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, RPLS, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, MTL, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, ADLS, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, MTLM, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, MTLP, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, MTLS, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, TGL, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, SPTLP, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, CNPLP, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, TGPLP, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, SPTH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, KBP, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, CNPH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, CNPV, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, RPL, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, NVLH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, PTLH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, PTLH_PCH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, PTLU, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(INTEL, PTLU_PCH, &dwc3_pci_intel_swnode) },
    { PCI_DEVICE_DATA(AMD, NL_USB, &dwc3_pci_amd_swnode) },
    { PCI_DEVICE_DATA(AMD, MR, &dwc3_pci_amd_mr_swnode) },
    {  }	/* Terminating Entry */
    };
    MODULE_DEVICE_TABLE(pci, dwc3_pci_id_table);

#[no_mangle]
unsafe extern "C" fn dwc3_pci_dsm(dwc: *mut dwc3_pci, param: c_int) -> c_int {
    static int dwc3_pci_dsm(struct dwc3_pci *dwc, int param)
    {
    union acpi_object *obj;
    union acpi_object tmp;
    let mut argv4: union acpi_object = ACPI_INIT_DSM_ARGV4(1, &tmp);
    if (!dwc.has_dsm_for_pm)
    return 0;
    tmp.type = ACPI_TYPE_INTEGER;
    tmp.integer.value = param;
    obj = acpi_evaluate_dsm(ACPI_HANDLE(&dwc.pci.dev), &dwc.guid,
    1, PCI_INTEL_BXT_FUNC_PMU_PWR, &argv4);
    if (!obj) {
    dev_err(&dwc.pci.dev, "failed to evaluate _DSM\n");
    return -EIO;
    }
    ACPI_FREE(obj);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn dwc3_pci_runtime_suspend(dev: *mut device) -> c_int {
    static int dwc3_pci_runtime_suspend(struct device *dev)
    {
    struct dwc3_pci		*dwc = dev_get_drvdata(dev);
    if (device_can_wakeup(dev))
    return dwc3_pci_dsm(dwc, PCI_INTEL_BXT_STATE_D3);
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn dwc3_pci_runtime_resume(dev: *mut device) -> c_int {
    static int dwc3_pci_runtime_resume(struct device *dev)
    {
    struct dwc3_pci		*dwc = dev_get_drvdata(dev);
    int			ret;
    ret = dwc3_pci_dsm(dwc, PCI_INTEL_BXT_STATE_D0);
    if (ret)
    return ret;
    queue_work(pm_wq, &dwc.wakeup_work);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn dwc3_pci_suspend(dev: *mut device) -> c_int {
    static int dwc3_pci_suspend(struct device *dev)
    {
    struct dwc3_pci		*dwc = dev_get_drvdata(dev);
    return dwc3_pci_dsm(dwc, PCI_INTEL_BXT_STATE_D3);
    }
#[no_mangle]
unsafe extern "C" fn dwc3_pci_resume(dev: *mut device) -> c_int {
    static int dwc3_pci_resume(struct device *dev)
    {
    struct dwc3_pci		*dwc = dev_get_drvdata(dev);
    return dwc3_pci_dsm(dwc, PCI_INTEL_BXT_STATE_D0);
    }

    static const struct dev_pm_ops dwc3_pci_dev_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(dwc3_pci_suspend, dwc3_pci_resume)
    SET_RUNTIME_PM_OPS(dwc3_pci_runtime_suspend, dwc3_pci_runtime_resume,
    core::ptr::null_mut())
    };
    static struct pci_driver dwc3_pci_driver = {
    .name		= "dwc3-pci",
    .id_table	= dwc3_pci_id_table,
    .probe		= dwc3_pci_probe,
    .remove		= dwc3_pci_remove,
    .driver		= {
    .pm	= &dwc3_pci_dev_pm_ops,
    }
    };
    MODULE_AUTHOR("Felipe Balbi <balbi@ti.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("DesignWare USB3 PCI Glue Layer");
    module_pci_driver(dwc3_pci_driver);
