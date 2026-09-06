//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/x86/lpss.c
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
// ACPI support for Intel Lynxpoint LPSS.
//
// Copyright (C) 2013, Intel Corporation
// Authors: Mika Westerberg <mika.westerberg@linux.intel.com>
// Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//

pub const LPSS_CLK_SIZE: c_uint = 0x04;
pub const LPSS_LTR_SIZE: c_uint = 0x18;
// Offsets relative to LPSS_PRIVATE_OFFSET

pub const LPSS_RESETS: c_uint = 0x04;

pub const LPSS_GENERAL: c_uint = 0x08;

pub const LPSS_SW_LTR: c_uint = 0x10;
pub const LPSS_AUTO_LTR: c_uint = 0x14;

pub const LPSS_LTR_SNOOP_MASK: c_uint = 0x0000FFFF;
pub const LPSS_LTR_SNOOP_LAT_1US: c_uint = 0x800;
pub const LPSS_LTR_SNOOP_LAT_32US: c_uint = 0xC00;
pub const LPSS_LTR_SNOOP_LAT_SHIFT: c_int = 5;
pub const LPSS_LTR_SNOOP_LAT_CUTOFF: c_int = 3000;
pub const LPSS_LTR_MAX_VAL: c_uint = 0x3FF;
pub const LPSS_TX_INT: c_uint = 0x20;

pub const LPSS_PRV_REG_COUNT: c_int = 9;
// LPSS Flags

//
// For some devices the DSDT AML code for another device turns off the device
// before our suspend handler runs, causing us to read/save all 1-s (0xffffffff)
// as ctx register values.
// Luckily these devices always use the same ctx register values, so we can
// work around this by saving the ctx registers once on activation.
//

    struct lpss_private_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpss_device_desc {
    pub flags: c_uint,
    pub clk_con_id: *const c_char,
    pub prv_offset: c_uint,
    pub prv_size_override: usize,
    pub properties: *const property_entry,
    pub pdata): *mut *mut void (setup)(struct lpss_private_data,
    pub resume_from_noirq: bool,
}

    static const struct lpss_device_desc lpss_dma_desc = {
    .flags = LPSS_CLK,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpss_private_data {
    pub adev: *mut acpi_device,
    pub mmio_base: *mut void __iomem,
    pub mmio_size: resource_size_t,
    pub fixed_clk_rate: c_uint,
    pub clk: *mut clk,
    pub dev_desc: *const lpss_device_desc,
    pub prv_reg_ctx: [u32; LPSS_PRV_REG_COUNT],
}

// Devices which need to be in D3 before lpss_iosf_enter_d3_state() proceeds
    let mut pmc_atom_d3_mask: static u32 = 0xfe000ffe;
// LPSS run time quirks
    static unsigned int lpss_quirks;
//
// LPSS_QUIRK_ALWAYS_POWER_ON: override power state for LPSS DMA device.
//
// The LPSS DMA controller has neither _PS0 nor _PS3 method. Moreover
// it can be powered off automatically whenever the last LPSS device goes down.
// In case of no power any access to the DMA controller will hang the system.
// The behaviour is reproduced on some HP laptops based on Intel BayTrail as
// well as on ASuS T100TA transformer.
//
// This quirk overrides power state of entire LPSS island to keep DMA powered
// on whenever we have at least one other device in use.
//

// UART Component Parameter Register
pub const LPSS_UART_CPR: c_uint = 0xF4;

#[no_mangle]
unsafe extern "C" fn lpss_uart_setup(pdata: *mut lpss_private_data) {
    static void lpss_uart_setup(struct lpss_private_data *pdata)
    {
    unsigned int offset;
    u32 val;
    offset = pdata.dev_desc.prv_offset + LPSS_TX_INT;
    val = readl(pdata.mmio_base + offset);
    writel(val | LPSS_TX_INT_MASK, pdata.mmio_base + offset);
    val = readl(pdata.mmio_base + LPSS_UART_CPR);
    if (!(val & LPSS_UART_CPR_AFCE)) {
    offset = pdata.dev_desc.prv_offset + LPSS_GENERAL;
    val = readl(pdata.mmio_base + offset);
    val |= LPSS_GENERAL_UART_RTS_OVRD;
    writel(val, pdata.mmio_base + offset);
    }
    }
#[no_mangle]
unsafe extern "C" fn lpss_deassert_reset(pdata: *mut lpss_private_data) {
    static void lpss_deassert_reset(struct lpss_private_data *pdata)
    {
    unsigned int offset;
    u32 val;
    offset = pdata.dev_desc.prv_offset + LPSS_RESETS;
    val = readl(pdata.mmio_base + offset);
    val |= LPSS_RESETS_RESET_APB | LPSS_RESETS_RESET_FUNC;
    writel(val, pdata.mmio_base + offset);
    }
//
// BYT PWM used for backlight control by the i915 driver on systems without
// the Crystal Cove PMIC.
//
    static struct pwm_lookup byt_pwm_lookup[] = {
    PWM_LOOKUP_WITH_MODULE("80860F09:00", 0, "0000:00:02.0",
    "pwm_soc_backlight", 0, PWM_POLARITY_NORMAL,
    "pwm-lpss-platform"),
    };
#[no_mangle]
unsafe extern "C" fn byt_pwm_setup(pdata: *mut lpss_private_data) {
    static void byt_pwm_setup(struct lpss_private_data *pdata)
    {
// Only call pwm_add_table for the first PWM controller
    if (acpi_dev_uid_match(pdata.adev, 1))
    pwm_add_table(byt_pwm_lookup, ARRAY_SIZE(byt_pwm_lookup));
    }
pub const LPSS_I2C_ENABLE: c_uint = 0x6c;
#[no_mangle]
unsafe extern "C" fn byt_i2c_setup(pdata: *mut lpss_private_data) {
    static void byt_i2c_setup(struct lpss_private_data *pdata)
    {
    let mut handle: acpi_handle = pdata.adev.handle;
    let mut shared_host: c_ulonglong = 0;
    acpi_status status;
    u64 uid;
// Expected to always be successful, but better safe then sorry
    if (!acpi_dev_uid_to_integer(pdata.adev, &uid) && uid) {
// Detect I2C bus shared with PUNIT and ignore its d3 status
    status = acpi_evaluate_integer(handle, "_SEM", core::ptr::null_mut(), &shared_host);
    if (ACPI_SUCCESS(status) && shared_host)
    pmc_atom_d3_mask &= ~(BIT_LPSS2_F1_I2C1 << (uid - 1));
    }
    lpss_deassert_reset(pdata);
    if (readl(pdata.mmio_base + pdata.dev_desc.prv_offset))
    pdata.fixed_clk_rate = 133000000;
    writel(0, pdata.mmio_base + LPSS_I2C_ENABLE);
    }
//
// BSW PWM1 is used for backlight control by the i915 driver
// BSW PWM2 is used for backlight control for fixed (etched into the glass)
// touch controls on some models. These touch-controls have specialized
// drivers which know they need the "pwm_soc_lpss_2" con-id.
//
    static struct pwm_lookup bsw_pwm_lookup[] = {
    PWM_LOOKUP_WITH_MODULE("80862288:00", 0, "0000:00:02.0",
    "pwm_soc_backlight", 0, PWM_POLARITY_NORMAL,
    "pwm-lpss-platform"),
    PWM_LOOKUP_WITH_MODULE("80862289:00", 0, core::ptr::null_mut(),
    "pwm_soc_lpss_2", 0, PWM_POLARITY_NORMAL,
    "pwm-lpss-platform"),
    };
#[no_mangle]
unsafe extern "C" fn bsw_pwm_setup(pdata: *mut lpss_private_data) {
    static void bsw_pwm_setup(struct lpss_private_data *pdata)
    {
// Only call pwm_add_table for the first PWM controller
    if (acpi_dev_uid_match(pdata.adev, 1))
    pwm_add_table(bsw_pwm_lookup, ARRAY_SIZE(bsw_pwm_lookup));
    }
    static const struct property_entry lpt_spi_properties[] = {
    PROPERTY_ENTRY_U32("intel,spi-pxa2xx-type", LPSS_LPT_SSP),
    { }
    };
    static const struct lpss_device_desc lpt_spi_dev_desc = {
    .flags = LPSS_CLK | LPSS_CLK_GATE | LPSS_CLK_DIVIDER | LPSS_LTR
    | LPSS_SAVE_CTX,
    .prv_offset = 0x800,
    .properties = lpt_spi_properties,
    };
    static const struct lpss_device_desc lpt_i2c_dev_desc = {
    .flags = LPSS_CLK | LPSS_CLK_GATE | LPSS_LTR | LPSS_SAVE_CTX,
    .prv_offset = 0x800,
    };
    static struct property_entry uart_properties[] = {
    PROPERTY_ENTRY_U32("reg-io-width", 4),
    PROPERTY_ENTRY_U32("reg-shift", 2),
    PROPERTY_ENTRY_BOOL("snps,uart-16550-compatible"),
    { },
    };
    static const struct lpss_device_desc lpt_uart_dev_desc = {
    .flags = LPSS_CLK | LPSS_CLK_GATE | LPSS_CLK_DIVIDER | LPSS_LTR
    | LPSS_SAVE_CTX,
    .clk_con_id = "baudclk",
    .prv_offset = 0x800,
    .setup = lpss_uart_setup,
    .properties = uart_properties,
    };
    static const struct lpss_device_desc lpt_sdio_dev_desc = {
    .flags = LPSS_LTR,
    .prv_offset = 0x1000,
    .prv_size_override = 0x1018,
    };
    static const struct lpss_device_desc byt_pwm_dev_desc = {
    .flags = LPSS_SAVE_CTX,
    .prv_offset = 0x800,
    .setup = byt_pwm_setup,
    };
    static const struct lpss_device_desc bsw_pwm_dev_desc = {
    .flags = LPSS_SAVE_CTX_ONCE | LPSS_NO_D3_DELAY,
    .prv_offset = 0x800,
    .setup = bsw_pwm_setup,
    .resume_from_noirq = true,
    };
    static const struct lpss_device_desc bsw_pwm2_dev_desc = {
    .flags = LPSS_SAVE_CTX_ONCE | LPSS_NO_D3_DELAY,
    .prv_offset = 0x800,
    .resume_from_noirq = true,
    };
    static const struct lpss_device_desc byt_uart_dev_desc = {
    .flags = LPSS_CLK | LPSS_CLK_GATE | LPSS_CLK_DIVIDER | LPSS_SAVE_CTX,
    .clk_con_id = "baudclk",
    .prv_offset = 0x800,
    .setup = lpss_uart_setup,
    .properties = uart_properties,
    };
    static const struct lpss_device_desc bsw_uart_dev_desc = {
    .flags = LPSS_CLK | LPSS_CLK_GATE | LPSS_CLK_DIVIDER | LPSS_SAVE_CTX
    | LPSS_NO_D3_DELAY,
    .clk_con_id = "baudclk",
    .prv_offset = 0x800,
    .setup = lpss_uart_setup,
    .properties = uart_properties,
    };
    static const struct property_entry byt_spi_properties[] = {
    PROPERTY_ENTRY_U32("intel,spi-pxa2xx-type", LPSS_BYT_SSP),
    { }
    };
    static const struct lpss_device_desc byt_spi_dev_desc = {
    .flags = LPSS_CLK | LPSS_CLK_GATE | LPSS_CLK_DIVIDER | LPSS_SAVE_CTX,
    .prv_offset = 0x400,
    .properties = byt_spi_properties,
    };
    static const struct lpss_device_desc byt_sdio_dev_desc = {
    .flags = LPSS_CLK,
    };
    static const struct lpss_device_desc byt_i2c_dev_desc = {
    .flags = LPSS_CLK | LPSS_SAVE_CTX,
    .prv_offset = 0x800,
    .setup = byt_i2c_setup,
    .resume_from_noirq = true,
    };
    static const struct lpss_device_desc bsw_i2c_dev_desc = {
    .flags = LPSS_CLK | LPSS_SAVE_CTX | LPSS_NO_D3_DELAY,
    .prv_offset = 0x800,
    .setup = byt_i2c_setup,
    .resume_from_noirq = true,
    };
    static const struct property_entry bsw_spi_properties[] = {
    PROPERTY_ENTRY_U32("intel,spi-pxa2xx-type", LPSS_BSW_SSP),
    PROPERTY_ENTRY_U32("num-cs", 2),
    { }
    };
    static const struct lpss_device_desc bsw_spi_dev_desc = {
    .flags = LPSS_CLK | LPSS_CLK_GATE | LPSS_CLK_DIVIDER | LPSS_SAVE_CTX
    | LPSS_NO_D3_DELAY,
    .prv_offset = 0x400,
    .setup = lpss_deassert_reset,
    .properties = bsw_spi_properties,
    };
    static const struct x86_cpu_id lpss_cpu_ids[] = {
    X86_MATCH_VFM(INTEL_ATOM_SILVERMONT,	core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ATOM_AIRMONT,	core::ptr::null_mut()),
    {}
    };

    static const struct acpi_device_id acpi_lpss_device_ids[] = {
// Generic LPSS devices
    { "INTL9C60", LPSS_ADDR(lpss_dma_desc) },
// Lynxpoint LPSS devices
    { "INT33C0", LPSS_ADDR(lpt_spi_dev_desc) },
    { "INT33C1", LPSS_ADDR(lpt_spi_dev_desc) },
    { "INT33C2", LPSS_ADDR(lpt_i2c_dev_desc) },
    { "INT33C3", LPSS_ADDR(lpt_i2c_dev_desc) },
    { "INT33C4", LPSS_ADDR(lpt_uart_dev_desc) },
    { "INT33C5", LPSS_ADDR(lpt_uart_dev_desc) },
    { "INT33C6", LPSS_ADDR(lpt_sdio_dev_desc) },
// BayTrail LPSS devices
    { "80860F09", LPSS_ADDR(byt_pwm_dev_desc) },
    { "80860F0A", LPSS_ADDR(byt_uart_dev_desc) },
    { "80860F0E", LPSS_ADDR(byt_spi_dev_desc) },
    { "80860F14", LPSS_ADDR(byt_sdio_dev_desc) },
    { "80860F41", LPSS_ADDR(byt_i2c_dev_desc) },
// Braswell LPSS devices
    { "80862286", LPSS_ADDR(lpss_dma_desc) },
    { "80862288", LPSS_ADDR(bsw_pwm_dev_desc) },
    { "80862289", LPSS_ADDR(bsw_pwm2_dev_desc) },
    { "8086228A", LPSS_ADDR(bsw_uart_dev_desc) },
    { "8086228E", LPSS_ADDR(bsw_spi_dev_desc) },
    { "808622C0", LPSS_ADDR(lpss_dma_desc) },
    { "808622C1", LPSS_ADDR(bsw_i2c_dev_desc) },
// Broadwell LPSS devices
    { "INT3430", LPSS_ADDR(lpt_spi_dev_desc) },
    { "INT3431", LPSS_ADDR(lpt_spi_dev_desc) },
    { "INT3432", LPSS_ADDR(lpt_i2c_dev_desc) },
    { "INT3433", LPSS_ADDR(lpt_i2c_dev_desc) },
    { "INT3434", LPSS_ADDR(lpt_uart_dev_desc) },
    { "INT3435", LPSS_ADDR(lpt_uart_dev_desc) },
    { "INT3436", LPSS_ADDR(lpt_sdio_dev_desc) },
    { }
    };

// LPSS main clock device.
    static struct platform_device *lpss_clk_dev;
#[no_mangle]
pub unsafe extern "C" fn lpt_register_clock_device() {
    static inline void lpt_register_clock_device(void)
    {
    lpss_clk_dev = platform_device_register_simple("clk-lpss-atom",
    PLATFORM_DEVID_NONE,
    core::ptr::null_mut(), 0);
    }
    static int register_device_clock(struct acpi_device *adev,
    struct lpss_private_data *pdata)
    {
    const struct lpss_device_desc *dev_desc = pdata.dev_desc;
    const char *devname = dev_name(&adev.dev);
    struct clk *clk;
    struct lpss_clk_data *clk_data;
    const char *parent, *clk_name;
    void __iomem *prv_base;
    if (!lpss_clk_dev)
    lpt_register_clock_device();
    if (IS_ERR(lpss_clk_dev))
    return PTR_ERR(lpss_clk_dev);
    clk_data = platform_get_drvdata(lpss_clk_dev);
    if (!clk_data)
    return -ENODEV;
    clk = clk_data.clk;
    if (!pdata.mmio_base
    || pdata.mmio_size < dev_desc.prv_offset + LPSS_CLK_SIZE)
    return -ENODATA;
    parent = clk_data.name;
    prv_base = pdata.mmio_base + dev_desc.prv_offset;
    if (pdata.fixed_clk_rate) {
    clk = clk_register_fixed_rate(core::ptr::null_mut(), devname, parent, 0,
    pdata.fixed_clk_rate);
    goto out;
    }
    if (dev_desc.flags & LPSS_CLK_GATE) {
    clk = clk_register_gate(core::ptr::null_mut(), devname, parent, 0,
    prv_base, 0, 0, core::ptr::null_mut());
    parent = devname;
    }
    if (dev_desc.flags & LPSS_CLK_DIVIDER) {
// Prevent division by zero
    if (!readl(prv_base))
    writel(LPSS_CLK_DIVIDER_DEF_MASK, prv_base);
    clk_name = kasprintf(GFP_KERNEL, "%s-div", devname);
    if (!clk_name)
    return -ENOMEM;
    clk = clk_register_fractional_divider(core::ptr::null_mut(), clk_name, parent,
    0, prv_base, 1, 15, 16, 15,
    CLK_FRAC_DIVIDER_POWER_OF_TWO_PS,
    core::ptr::null_mut());
    parent = clk_name;
    clk_name = kasprintf(GFP_KERNEL, "%s-update", devname);
    if (!clk_name) {
    kfree(parent);
    return -ENOMEM;
    }
    clk = clk_register_gate(core::ptr::null_mut(), clk_name, parent,
    CLK_SET_RATE_PARENT | CLK_SET_RATE_GATE,
    prv_base, 31, 0, core::ptr::null_mut());
    kfree(parent);
    kfree(clk_name);
    }
    out:
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    pdata.clk = clk;
    clk_register_clkdev(clk, dev_desc.clk_con_id, devname);
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpss_device_links {
    pub supplier_hid: *const c_char,
    pub supplier_uid: *const c_char,
    pub consumer_hid: *const c_char,
    pub consumer_uid: *const c_char,
    pub flags: u32,
    pub dep_missing_ids: *const dmi_system_id,
}

// Please keep this list sorted alphabetically by vendor and model
    static const struct dmi_system_id i2c1_dep_missing_dmi_ids[] = {
    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "ASUSTeK COMPUTER INC."),
    DMI_MATCH(DMI_PRODUCT_NAME, "T200TA"),
    },
    },
    {}
    };
//
// The _DEP method is used to identify dependencies but instead of creating
// device links for every handle in _DEP, only links in the following list are
// created. That is necessary because, in the general case, _DEP can refer to
// devices that might not have drivers, or that are on different buses, or where
// the supplier is not enumerated until after the consumer is probed.
//
    static const struct lpss_device_links lpss_device_links[] = {
// CHT External sdcard slot controller depends on PMIC I2C ctrl
    {"808622C1", "7", "80860F14", "3", DL_FLAG_PM_RUNTIME},
// CHT iGPU depends on PMIC I2C controller
    {"808622C1", "7", "LNXVIDEO", core::ptr::null_mut(), DL_FLAG_PM_RUNTIME},
// BYT iGPU depends on the Embedded Controller I2C controller (UID 1)
    {"80860F41", "1", "LNXVIDEO", core::ptr::null_mut(), DL_FLAG_PM_RUNTIME,
    i2c1_dep_missing_dmi_ids},
// BYT CR iGPU depends on PMIC I2C controller (UID 5 on CR)
    {"80860F41", "5", "LNXVIDEO", core::ptr::null_mut(), DL_FLAG_PM_RUNTIME},
// BYT iGPU depends on PMIC I2C controller (UID 7 on non CR)
    {"80860F41", "7", "LNXVIDEO", core::ptr::null_mut(), DL_FLAG_PM_RUNTIME},
    };
    static bool acpi_lpss_is_supplier(struct acpi_device *adev,
    const struct lpss_device_links *link)
    {
    return acpi_dev_hid_uid_match(adev, link.supplier_hid, link.supplier_uid);
    }
    static bool acpi_lpss_is_consumer(struct acpi_device *adev,
    const struct lpss_device_links *link)
    {
    return acpi_dev_hid_uid_match(adev, link.consumer_hid, link.consumer_uid);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_uid {
    pub hid: *const c_char,
    pub uid: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn match_hid_uid(dev: *mut device, data: *const c_void) -> c_int {
    static int match_hid_uid(struct device *dev, const void *data)
    {
    struct acpi_device *adev = ACPI_COMPANION(dev);
    const struct hid_uid *id = data;
    if (!adev)
    return 0;
    return acpi_dev_hid_uid_match(adev, id.hid, id.uid);
    }
    static struct device *acpi_lpss_find_device(const char *hid, const char *uid)
    {
    struct device *dev;
    struct hid_uid data = {
    .hid = hid,
    .uid = uid,
    };
    dev = bus_find_device(&platform_bus_type, core::ptr::null_mut(), &data, match_hid_uid);
    if (dev)
    return dev;
    return bus_find_device(&pci_bus_type, core::ptr::null_mut(), &data, match_hid_uid);
    }
    static void acpi_lpss_link_consumer(struct device *dev1,
    const struct lpss_device_links *link)
    {
    struct device *dev2;
    dev2 = acpi_lpss_find_device(link.consumer_hid, link.consumer_uid);
    if (!dev2)
    return;
    if ((link.dep_missing_ids && dmi_check_system(link.dep_missing_ids))
    || acpi_device_dep(ACPI_HANDLE(dev2), ACPI_HANDLE(dev1)))
    device_link_add(dev2, dev1, link.flags);
    put_device(dev2);
    }
    static void acpi_lpss_link_supplier(struct device *dev1,
    const struct lpss_device_links *link)
    {
    struct device *dev2;
    dev2 = acpi_lpss_find_device(link.supplier_hid, link.supplier_uid);
    if (!dev2)
    return;
    if ((link.dep_missing_ids && dmi_check_system(link.dep_missing_ids))
    || acpi_device_dep(ACPI_HANDLE(dev1), ACPI_HANDLE(dev2)))
    device_link_add(dev1, dev2, link.flags);
    put_device(dev2);
    }
    static void acpi_lpss_create_device_links(struct acpi_device *adev,
    struct platform_device *pdev)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(lpss_device_links); i++) {
    const struct lpss_device_links *link = &lpss_device_links[i];
    if (acpi_lpss_is_supplier(adev, link))
    acpi_lpss_link_consumer(&pdev.dev, link);
    if (acpi_lpss_is_consumer(adev, link))
    acpi_lpss_link_supplier(&pdev.dev, link);
    }
    }
    static int acpi_lpss_create_device(struct acpi_device *adev,
    const struct acpi_device_id *id)
    {
    const struct lpss_device_desc *dev_desc;
    struct lpss_private_data *pdata;
    struct resource_entry *rentry;
    struct list_head resource_list;
    struct platform_device *pdev;
    int ret;
    dev_desc = (const struct lpss_device_desc *)id.driver_data;
    if (!dev_desc)
    return -EINVAL;
    pdata = kzalloc_obj(*pdata);
    if (!pdata)
    return -ENOMEM;
    INIT_LIST_HEAD(&resource_list);
    ret = acpi_dev_get_memory_resources(adev, &resource_list);
    if (ret < 0)
    goto err_out;
    rentry = list_first_entry_or_null(&resource_list, struct resource_entry, node);
    if (rentry) {
    if (dev_desc.prv_size_override)
    pdata.mmio_size = dev_desc.prv_size_override;
    else
    pdata.mmio_size = resource_size(rentry.res);
    pdata.mmio_base = ioremap(rentry.res.start, pdata.mmio_size);
    }
    acpi_dev_free_resource_list(&resource_list);
    if (!pdata.mmio_base) {
// Avoid acpi_bus_attach() instantiating a pdev for this dev.
    adev.pnp.type.platform_id = 0;
    goto out_free;
    }
    pdata.adev = adev;
    pdata.dev_desc = dev_desc;
    if (dev_desc.setup)
    dev_desc.setup(pdata);
    if (dev_desc.flags & LPSS_CLK) {
    ret = register_device_clock(adev, pdata);
    if (ret)
    goto out_free;
    }
//
// This works around a known issue in ACPI tables where LPSS devices
// have _PS0 and _PS3 without _PSC (and no power resources), so
// acpi_bus_init_power() will assume that the BIOS has put them into D0.
//
    acpi_device_fix_up_power(adev);
    adev.driver_data = pdata;
    pdev = acpi_create_platform_device(adev, dev_desc.properties);
    if (IS_ERR_OR_NULL(pdev)) {
    adev.driver_data = core::ptr::null_mut();
    ret = PTR_ERR(pdev);
    goto err_out;
    }
    acpi_lpss_create_device_links(adev, pdev);
    return 1;
    out_free:
// Skip the device, but continue the namespace scan
    ret = 0;
    err_out:
    kfree(pdata);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __lpss_reg_read(pdata: *mut lpss_private_data, reg: c_uint) -> u32 {
    static u32 __lpss_reg_read(struct lpss_private_data *pdata, unsigned int reg)
    {
    return readl(pdata.mmio_base + pdata.dev_desc.prv_offset + reg);
    }
    static void __lpss_reg_write(u32 val, struct lpss_private_data *pdata,
    unsigned int reg)
    {
    writel(val, pdata.mmio_base + pdata.dev_desc.prv_offset + reg);
    }
#[no_mangle]
unsafe extern "C" fn lpss_reg_read(dev: *mut device, reg: c_uint, val: *mut u32) -> c_int {
    static int lpss_reg_read(struct device *dev, unsigned int reg, u32 *val)
    {
    struct acpi_device *adev = ACPI_COMPANION(dev);
    struct lpss_private_data *pdata;
    unsigned long flags;
    int ret;
    if (WARN_ON(!adev))
    return -ENODEV;
    spin_lock_irqsave(&dev.power.lock, flags);
    if (pm_runtime_suspended(dev)) {
    ret = -EAGAIN;
    goto out;
    }
    pdata = acpi_driver_data(adev);
    if (WARN_ON(!pdata || !pdata.mmio_base)) {
    ret = -ENODEV;
    goto out;
    }
// val = __lpss_reg_read(pdata, reg);
    ret = 0;
    out:
    spin_unlock_irqrestore(&dev.power.lock, flags);
    return ret;
    }
    static ssize_t lpss_ltr_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut ltr_value: u32 = 0;
    unsigned int reg;
    int ret;
    reg = strcmp(attr.attr.name, "auto_ltr") ? LPSS_SW_LTR : LPSS_AUTO_LTR;
    ret = lpss_reg_read(dev, reg, &ltr_value);
    if (ret)
    return ret;
    return sysfs_emit(buf, "%08x\n", ltr_value);
    }
    static ssize_t lpss_ltr_mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut ltr_mode: u32 = 0;
    char *outstr;
    int ret;
    ret = lpss_reg_read(dev, LPSS_GENERAL, &ltr_mode);
    if (ret)
    return ret;
    outstr = (ltr_mode & LPSS_GENERAL_LTR_MODE_SW) ? "sw" : "auto";
    return sprintf(buf, "%s\n", outstr);
    }
    static DEVICE_ATTR(auto_ltr, S_IRUSR, lpss_ltr_show, core::ptr::null_mut());
    static DEVICE_ATTR(sw_ltr, S_IRUSR, lpss_ltr_show, core::ptr::null_mut());
    static DEVICE_ATTR(ltr_mode, S_IRUSR, lpss_ltr_mode_show, core::ptr::null_mut());
    static struct attribute *lpss_attrs[] = {
    &dev_attr_auto_ltr.attr,
    &dev_attr_sw_ltr.attr,
    &dev_attr_ltr_mode.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group lpss_attr_group = {
    .attrs = lpss_attrs,
    .name = "lpss_ltr",
    };
#[no_mangle]
unsafe extern "C" fn acpi_lpss_set_ltr(dev: *mut device, val: i32) {
    static void acpi_lpss_set_ltr(struct device *dev, s32 val)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    u32 ltr_mode, ltr_val;
    ltr_mode = __lpss_reg_read(pdata, LPSS_GENERAL);
    if (val < 0) {
    if (ltr_mode & LPSS_GENERAL_LTR_MODE_SW) {
    ltr_mode &= ~LPSS_GENERAL_LTR_MODE_SW;
    __lpss_reg_write(ltr_mode, pdata, LPSS_GENERAL);
    }
    return;
    }
    ltr_val = __lpss_reg_read(pdata, LPSS_SW_LTR) & ~LPSS_LTR_SNOOP_MASK;
    if (val >= LPSS_LTR_SNOOP_LAT_CUTOFF) {
    ltr_val |= LPSS_LTR_SNOOP_LAT_32US;
    val = LPSS_LTR_MAX_VAL;
    } else if (val > LPSS_LTR_MAX_VAL) {
    ltr_val |= LPSS_LTR_SNOOP_LAT_32US | LPSS_LTR_SNOOP_REQ;
    val >>= LPSS_LTR_SNOOP_LAT_SHIFT;
    } else {
    ltr_val |= LPSS_LTR_SNOOP_LAT_1US | LPSS_LTR_SNOOP_REQ;
    }
    ltr_val |= val;
    __lpss_reg_write(ltr_val, pdata, LPSS_SW_LTR);
    if (!(ltr_mode & LPSS_GENERAL_LTR_MODE_SW)) {
    ltr_mode |= LPSS_GENERAL_LTR_MODE_SW;
    __lpss_reg_write(ltr_mode, pdata, LPSS_GENERAL);
    }
    }

//
// acpi_lpss_save_ctx() - Save the private registers of LPSS device
// @dev: LPSS device
// @pdata: pointer to the private data of the LPSS device
//
// Most LPSS devices have private registers which may loose their context when
// the device is powered down. acpi_lpss_save_ctx() saves those registers into
// prv_reg_ctx array.
//
    static void acpi_lpss_save_ctx(struct device *dev,
    struct lpss_private_data *pdata)
    {
    unsigned int i;
    for (i = 0; i < LPSS_PRV_REG_COUNT; i++) {
    let mut offset: c_ulong = i * sizeof(u32);
    pdata.prv_reg_ctx[i] = __lpss_reg_read(pdata, offset);
    dev_dbg(dev, "saving 0x%08x from LPSS reg at offset 0x%02lx\n",
    pdata.prv_reg_ctx[i], offset);
    }
    }
//
// acpi_lpss_restore_ctx() - Restore the private registers of LPSS device
// @dev: LPSS device
// @pdata: pointer to the private data of the LPSS device
//
// Restores the registers that were previously stored with acpi_lpss_save_ctx().
//
    static void acpi_lpss_restore_ctx(struct device *dev,
    struct lpss_private_data *pdata)
    {
    unsigned int i;
    for (i = 0; i < LPSS_PRV_REG_COUNT; i++) {
    let mut offset: c_ulong = i * sizeof(u32);
    __lpss_reg_write(pdata.prv_reg_ctx[i], pdata, offset);
    dev_dbg(dev, "restoring 0x%08x to LPSS reg at offset 0x%02lx\n",
    pdata.prv_reg_ctx[i], offset);
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_d3_to_d0_delay(pdata: *mut lpss_private_data) {
    static void acpi_lpss_d3_to_d0_delay(struct lpss_private_data *pdata)
    {
//
// The following delay is needed or the subsequent write operations may
// fail. The LPSS devices are actually PCI devices and the PCI spec
// expects 10ms delay before the device can be accessed after D3 to D0
// transition. However some platforms like BSW does not need this delay.
//
    unsigned int delay = 10;	/* default 10ms delay */
    if (pdata.dev_desc.flags & LPSS_NO_D3_DELAY)
    delay = 0;
    msleep(delay);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_activate(dev: *mut device) -> c_int {
    static int acpi_lpss_activate(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    int ret;
    ret = acpi_dev_resume(dev);
    if (ret)
    return ret;
    acpi_lpss_d3_to_d0_delay(pdata);
//
// This is called only on ->probe() stage where a device is either in
// known state defined by BIOS or most likely powered off. Due to this
// we have to deassert reset line to be sure that ->probe() will
// recognize the device.
//
    if (pdata.dev_desc.flags & (LPSS_SAVE_CTX | LPSS_SAVE_CTX_ONCE))
    lpss_deassert_reset(pdata);
    if (pdata.dev_desc.flags & LPSS_SAVE_CTX_ONCE)
    acpi_lpss_save_ctx(dev, pdata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_dismiss(dev: *mut device) {
    static void acpi_lpss_dismiss(struct device *dev)
    {
    acpi_dev_suspend(dev, false);
    }
// IOSF SB for LPSS island
pub const LPSS_IOSF_UNIT_LPIOEP: c_uint = 0xA0;
pub const LPSS_IOSF_UNIT_LPIO1: c_uint = 0xAB;
pub const LPSS_IOSF_UNIT_LPIO2: c_uint = 0xAC;
pub const LPSS_IOSF_PMCSR: c_uint = 0x84;
pub const LPSS_PMCSR_D0: c_int = 0;
pub const LPSS_PMCSR_D3hot: c_int = 3;

pub const LPSS_IOSF_GPIODEF0: c_uint = 0x154;

    static DEFINE_MUTEX(lpss_iosf_mutex);
    let mut lpss_iosf_d3_entered: static bool = true;
#[no_mangle]
unsafe extern "C" fn lpss_iosf_enter_d3_state() {
    static void lpss_iosf_enter_d3_state(void)
    {
    let mut value1: u32 = 0;
    let mut mask1: u32 = LPSS_GPIODEF0_DMA_D3_MASK | LPSS_GPIODEF0_DMA_LLP;
    let mut value2: u32 = LPSS_PMCSR_D3hot;
    let mut mask2: u32 = LPSS_PMCSR_Dx_MASK;
//
// PMC provides an information about actual status of the LPSS devices.
// Here we read the values related to LPSS power island, i.e. LPSS
// devices, excluding both LPSS DMA controllers, along with SCC domain.
//
    u32 func_dis, d3_sts_0, pmc_status;
    int ret;
    ret = pmc_atom_read(PMC_FUNC_DIS, &func_dis);
    if (ret)
    return;
    mutex_lock(&lpss_iosf_mutex);
    ret = pmc_atom_read(PMC_D3_STS_0, &d3_sts_0);
    if (ret)
    goto exit;
//
// Get the status of entire LPSS power island per device basis.
// Shutdown both LPSS DMA controllers if and only if all other devices
// are already in D3hot.
//
    pmc_status = (~(d3_sts_0 | func_dis)) & pmc_atom_d3_mask;
    if (pmc_status)
    goto exit;
    iosf_mbi_modify(LPSS_IOSF_UNIT_LPIO1, MBI_CFG_WRITE,
    LPSS_IOSF_PMCSR, value2, mask2);
    iosf_mbi_modify(LPSS_IOSF_UNIT_LPIO2, MBI_CFG_WRITE,
    LPSS_IOSF_PMCSR, value2, mask2);
    iosf_mbi_modify(LPSS_IOSF_UNIT_LPIOEP, MBI_CR_WRITE,
    LPSS_IOSF_GPIODEF0, value1, mask1);
    lpss_iosf_d3_entered = true;
    exit:
    mutex_unlock(&lpss_iosf_mutex);
    }
#[no_mangle]
unsafe extern "C" fn lpss_iosf_exit_d3_state() {
    static void lpss_iosf_exit_d3_state(void)
    {
    u32 value1 = LPSS_GPIODEF0_DMA1_D3 | LPSS_GPIODEF0_DMA2_D3 |
    LPSS_GPIODEF0_DMA_LLP;
    let mut mask1: u32 = LPSS_GPIODEF0_DMA_D3_MASK | LPSS_GPIODEF0_DMA_LLP;
    let mut value2: u32 = LPSS_PMCSR_D0;
    let mut mask2: u32 = LPSS_PMCSR_Dx_MASK;
    mutex_lock(&lpss_iosf_mutex);
    if (!lpss_iosf_d3_entered)
    goto exit;
    lpss_iosf_d3_entered = false;
    iosf_mbi_modify(LPSS_IOSF_UNIT_LPIOEP, MBI_CR_WRITE,
    LPSS_IOSF_GPIODEF0, value1, mask1);
    iosf_mbi_modify(LPSS_IOSF_UNIT_LPIO2, MBI_CFG_WRITE,
    LPSS_IOSF_PMCSR, value2, mask2);
    iosf_mbi_modify(LPSS_IOSF_UNIT_LPIO1, MBI_CFG_WRITE,
    LPSS_IOSF_PMCSR, value2, mask2);
    exit:
    mutex_unlock(&lpss_iosf_mutex);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_suspend(dev: *mut device, wakeup: bool) -> c_int {
    static int acpi_lpss_suspend(struct device *dev, bool wakeup)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    int ret;
    if (pdata.dev_desc.flags & LPSS_SAVE_CTX)
    acpi_lpss_save_ctx(dev, pdata);
    ret = acpi_dev_suspend(dev, wakeup);
//
// This call must be last in the sequence, otherwise PMC will return
// wrong status for devices being about to be powered off. See
// lpss_iosf_enter_d3_state() for further information.
//
    if (acpi_target_system_state() == ACPI_STATE_S0 &&
    lpss_quirks & LPSS_QUIRK_ALWAYS_POWER_ON && iosf_mbi_available())
    lpss_iosf_enter_d3_state();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_resume(dev: *mut device) -> c_int {
    static int acpi_lpss_resume(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    int ret;
//
// This call is kept first to be in symmetry with
// acpi_lpss_runtime_suspend() one.
//
    if (lpss_quirks & LPSS_QUIRK_ALWAYS_POWER_ON && iosf_mbi_available())
    lpss_iosf_exit_d3_state();
    ret = acpi_dev_resume(dev);
    if (ret)
    return ret;
    acpi_lpss_d3_to_d0_delay(pdata);
    if (pdata.dev_desc.flags & (LPSS_SAVE_CTX | LPSS_SAVE_CTX_ONCE))
    acpi_lpss_restore_ctx(dev, pdata);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn acpi_lpss_do_suspend_late(dev: *mut device) -> c_int {
    static int acpi_lpss_do_suspend_late(struct device *dev)
    {
    int ret;
    if (dev_pm_skip_suspend(dev))
    return 0;
    ret = pm_generic_suspend_late(dev);
    return ret ? ret : acpi_lpss_suspend(dev, device_may_wakeup(dev));
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_suspend_late(dev: *mut device) -> c_int {
    static int acpi_lpss_suspend_late(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    if (pdata.dev_desc.resume_from_noirq)
    return 0;
    return acpi_lpss_do_suspend_late(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_suspend_noirq(dev: *mut device) -> c_int {
    static int acpi_lpss_suspend_noirq(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    int ret;
    if (pdata.dev_desc.resume_from_noirq) {
//
// The driver's ->suspend_late callback will be invoked by
// acpi_lpss_do_suspend_late(), with the assumption that the
// driver really wanted to run that code in ->suspend_noirq, but
// it could not run after acpi_dev_suspend() and the driver
// expected the latter to be called in the "late" phase.
//
    ret = acpi_lpss_do_suspend_late(dev);
    if (ret)
    return ret;
    }
    return acpi_subsys_suspend_noirq(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_do_resume_early(dev: *mut device) -> c_int {
    static int acpi_lpss_do_resume_early(struct device *dev)
    {
    let mut ret: c_int = acpi_lpss_resume(dev);
    return ret ? ret : pm_generic_resume_early(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_resume_early(dev: *mut device) -> c_int {
    static int acpi_lpss_resume_early(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    if (pdata.dev_desc.resume_from_noirq)
    return 0;
    if (dev_pm_skip_resume(dev))
    return 0;
    return acpi_lpss_do_resume_early(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_resume_noirq(dev: *mut device) -> c_int {
    static int acpi_lpss_resume_noirq(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    int ret;
// Follow acpi_subsys_resume_noirq().
    if (dev_pm_skip_resume(dev))
    return 0;
    ret = pm_generic_resume_noirq(dev);
    if (ret)
    return ret;
    if (!pdata.dev_desc.resume_from_noirq)
    return 0;
//
// The driver's ->resume_early callback will be invoked by
// acpi_lpss_do_resume_early(), with the assumption that the driver
// really wanted to run that code in ->resume_noirq, but it could not
// run before acpi_dev_resume() and the driver expected the latter to be
// called in the "early" phase.
//
    return acpi_lpss_do_resume_early(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_do_restore_early(dev: *mut device) -> c_int {
    static int acpi_lpss_do_restore_early(struct device *dev)
    {
    let mut ret: c_int = acpi_lpss_resume(dev);
    return ret ? ret : pm_generic_restore_early(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_restore_early(dev: *mut device) -> c_int {
    static int acpi_lpss_restore_early(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    if (pdata.dev_desc.resume_from_noirq)
    return 0;
    return acpi_lpss_do_restore_early(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_restore_noirq(dev: *mut device) -> c_int {
    static int acpi_lpss_restore_noirq(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    int ret;
    ret = pm_generic_restore_noirq(dev);
    if (ret)
    return ret;
    if (!pdata.dev_desc.resume_from_noirq)
    return 0;
// This is analogous to what happens in acpi_lpss_resume_noirq().
    return acpi_lpss_do_restore_early(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_do_poweroff_late(dev: *mut device) -> c_int {
    static int acpi_lpss_do_poweroff_late(struct device *dev)
    {
    let mut ret: c_int = pm_generic_poweroff_late(dev);
    return ret ? ret : acpi_lpss_suspend(dev, device_may_wakeup(dev));
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_poweroff_late(dev: *mut device) -> c_int {
    static int acpi_lpss_poweroff_late(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    if (dev_pm_skip_suspend(dev))
    return 0;
    if (pdata.dev_desc.resume_from_noirq)
    return 0;
    return acpi_lpss_do_poweroff_late(dev);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_poweroff_noirq(dev: *mut device) -> c_int {
    static int acpi_lpss_poweroff_noirq(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    if (dev_pm_skip_suspend(dev))
    return 0;
    if (pdata.dev_desc.resume_from_noirq) {
// This is analogous to the acpi_lpss_suspend_noirq() case.
    let mut ret: c_int = acpi_lpss_do_poweroff_late(dev);
    if (ret)
    return ret;
    }
    return pm_generic_poweroff_noirq(dev);
    }

#[no_mangle]
unsafe extern "C" fn acpi_lpss_runtime_suspend(dev: *mut device) -> c_int {
    static int acpi_lpss_runtime_suspend(struct device *dev)
    {
    let mut ret: c_int = pm_generic_runtime_suspend(dev);
    return ret ? ret : acpi_lpss_suspend(dev, true);
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_runtime_resume(dev: *mut device) -> c_int {
    static int acpi_lpss_runtime_resume(struct device *dev)
    {
    let mut ret: c_int = acpi_lpss_resume(dev);
    return ret ? ret : pm_generic_runtime_resume(dev);
    }

    static struct dev_pm_domain acpi_lpss_pm_domain = {

    .activate = acpi_lpss_activate,
    .dismiss = acpi_lpss_dismiss,

    .ops = {

    .prepare = acpi_subsys_prepare,
    .complete = acpi_subsys_complete,
    .suspend = acpi_subsys_suspend,
    .suspend_late = acpi_lpss_suspend_late,
    .suspend_noirq = acpi_lpss_suspend_noirq,
    .resume_noirq = acpi_lpss_resume_noirq,
    .resume_early = acpi_lpss_resume_early,
    .freeze = acpi_subsys_freeze,
    .poweroff = acpi_subsys_poweroff,
    .poweroff_late = acpi_lpss_poweroff_late,
    .poweroff_noirq = acpi_lpss_poweroff_noirq,
    .restore_noirq = acpi_lpss_restore_noirq,
    .restore_early = acpi_lpss_restore_early,

    .runtime_suspend = acpi_lpss_runtime_suspend,
    .runtime_resume = acpi_lpss_runtime_resume,

    },
    };
    static int acpi_lpss_platform_notify(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct platform_device *pdev = to_platform_device(data);
    struct lpss_private_data *pdata;
    struct acpi_device *adev;
    const struct acpi_device_id *id;
    id = acpi_match_device(acpi_lpss_device_ids, &pdev.dev);
    if (!id || !id.driver_data)
    return 0;
    adev = ACPI_COMPANION(&pdev.dev);
    if (!adev)
    return 0;
    pdata = acpi_driver_data(adev);
    if (!pdata)
    return 0;
    if (pdata.mmio_base &&
    pdata.mmio_size < pdata.dev_desc.prv_offset + LPSS_LTR_SIZE) {
    dev_err(&pdev.dev, "MMIO size insufficient to access LTR\n");
    return 0;
    }
    switch (action) {
    case BUS_NOTIFY_BIND_DRIVER:
    dev_pm_domain_set(&pdev.dev, &acpi_lpss_pm_domain);
    break;
    case BUS_NOTIFY_DRIVER_NOT_BOUND:
    case BUS_NOTIFY_UNBOUND_DRIVER:
    dev_pm_domain_set(&pdev.dev, core::ptr::null_mut());
    break;
    case BUS_NOTIFY_ADD_DEVICE:
    dev_pm_domain_set(&pdev.dev, &acpi_lpss_pm_domain);
    if (pdata.dev_desc.flags & LPSS_LTR)
    return sysfs_create_group(&pdev.dev.kobj,
    &lpss_attr_group);
    break;
    case BUS_NOTIFY_DEL_DEVICE:
    if (pdata.dev_desc.flags & LPSS_LTR)
    sysfs_remove_group(&pdev.dev.kobj, &lpss_attr_group);
    dev_pm_domain_set(&pdev.dev, core::ptr::null_mut());
    break;
    default:
    break;
    }
    return 0;
    }
    static struct notifier_block acpi_lpss_nb = {
    .notifier_call = acpi_lpss_platform_notify,
    };
#[no_mangle]
unsafe extern "C" fn acpi_lpss_bind(dev: *mut device) {
    static void acpi_lpss_bind(struct device *dev)
    {
    struct lpss_private_data *pdata = acpi_driver_data(ACPI_COMPANION(dev));
    if (!pdata || !pdata.mmio_base || !(pdata.dev_desc.flags & LPSS_LTR))
    return;
    if (pdata.mmio_size >= pdata.dev_desc.prv_offset + LPSS_LTR_SIZE)
    dev.power.set_latency_tolerance = acpi_lpss_set_ltr;
    else
    dev_err(dev, "MMIO size insufficient to access LTR\n");
    }
#[no_mangle]
unsafe extern "C" fn acpi_lpss_unbind(dev: *mut device) {
    static void acpi_lpss_unbind(struct device *dev)
    {
    dev.power.set_latency_tolerance = core::ptr::null_mut();
    }
    static struct acpi_scan_handler lpss_handler = {
    .ids = acpi_lpss_device_ids,
    .attach = acpi_lpss_create_device,
    .bind = acpi_lpss_bind,
    .unbind = acpi_lpss_unbind,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_lpss_init() -> void __init {
    void __init acpi_lpss_init(void)
    {
    const struct x86_cpu_id *id;
    int ret;
    ret = lpss_atom_clk_init();
    if (ret)
    return;
    id = x86_match_cpu(lpss_cpu_ids);
    if (id)
    lpss_quirks |= LPSS_QUIRK_ALWAYS_POWER_ON;
    bus_register_notifier(&platform_bus_type, &acpi_lpss_nb);
    acpi_scan_add_handler(&lpss_handler);
    }

    static struct acpi_scan_handler lpss_handler = {
    .ids = acpi_lpss_device_ids,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_lpss_init() -> void __init {
    void __init acpi_lpss_init(void)
    {
    acpi_scan_add_handler(&lpss_handler);
    }
