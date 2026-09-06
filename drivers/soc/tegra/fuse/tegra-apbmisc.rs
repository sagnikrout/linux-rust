//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/fuse/tegra-apbmisc.c
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
// Copyright (c) 2014-2023, NVIDIA CORPORATION.  All rights reserved.
//

pub const FUSE_SKU_INFO: c_uint = 0x10;
pub const ERD_ERR_CONFIG: c_uint = 0x120c;
pub const ERD_MASK_INBAND_ERR: c_uint = 0x1;
pub const PMC_STRAPPING_OPT_A_RAM_CODE_SHIFT: c_int = 4;

    (0xf << PMC_STRAPPING_OPT_A_RAM_CODE_SHIFT)

    (0x3 << PMC_STRAPPING_OPT_A_RAM_CODE_SHIFT)

    static void __iomem *apbmisc_base;
    static bool long_ram_code;
    static u32 strapping;
    static u32 chipid;
#[no_mangle]
pub unsafe extern "C" fn tegra_read_chipid() -> u32 {
    u32 tegra_read_chipid(void)
    {
    WARN(!apbmisc_base, "Tegra APB MISC not yet available\n");
    if (!chipid)
    chipid = readl_relaxed(apbmisc_base + 4);
    return chipid;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_get_chip_id() -> u8 {
    u8 tegra_get_chip_id(void)
    {

    let mut soc_id: i32 = arm_smccc_get_soc_id_version();
    if (soc_id >= 0)
    return TEGRA_SMCCC_CHIP_ID(soc_id);

    return (tegra_read_chipid() >> 8) & 0xff;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_get_major_rev() -> u8 {
    u8 tegra_get_major_rev(void)
    {

    let mut soc_id: i32 = arm_smccc_get_soc_id_version();
    if (soc_id >= 0)
    return TEGRA_SMCCC_MAJOR_REV(soc_id);

    return (tegra_read_chipid() >> 4) & 0xf;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_get_minor_rev() -> u8 {
    u8 tegra_get_minor_rev(void)
    {

    let mut revision: i32 = arm_smccc_get_soc_id_revision();
    if (revision >= 0)
    return TEGRA_SMCCC_MINOR_REV(revision);

    return (tegra_read_chipid() >> 16) & 0xf;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_get_platform() -> u8 {
    u8 tegra_get_platform(void)
    {

    let mut revision: i32 = arm_smccc_get_soc_id_revision();
    if (revision >= 0)
    return TEGRA_SMCCC_PLATFORM(revision);

    return (tegra_read_chipid() >> 20) & 0xf;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_is_silicon() -> bool {
    bool tegra_is_silicon(void)
    {
    switch (tegra_get_chip_id()) {
    case TEGRA194:
    case TEGRA234:
    case TEGRA241:
    case TEGRA264:
    if (tegra_get_platform() == 0)
    return true;
    return false;
    }
//
// Chips prior to Tegra194 have a different way of determining whether
// they are silicon or not. Since we never supported simulation on the
// older Tegra chips, don't bother extracting the information and just
// report that we're running on silicon.
//
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_read_straps() -> u32 {
    u32 tegra_read_straps(void)
    {
    WARN(!apbmisc_base, "Tegra ABP MISC not yet available\n");
    return strapping;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_read_ram_code() -> u32 {
    u32 tegra_read_ram_code(void)
    {
    let mut straps: u32 = tegra_read_straps();
    if (long_ram_code)
    straps &= PMC_STRAPPING_OPT_A_RAM_CODE_MASK_LONG;
    else
    straps &= PMC_STRAPPING_OPT_A_RAM_CODE_MASK_SHORT;
    return straps >> PMC_STRAPPING_OPT_A_RAM_CODE_SHIFT;
    }
    EXPORT_SYMBOL_GPL(tegra_read_ram_code);
//
// The function sets ERD(Error Response Disable) bit.
// This allows to mask inband errors and always send an
// OKAY response from CBB to the master which caused error.
//
#[no_mangle]
pub unsafe extern "C" fn tegra194_miscreg_mask_serror() -> c_int {
    int tegra194_miscreg_mask_serror(void)
    {
    if (!apbmisc_base)
    return -EPROBE_DEFER;
    if (!of_machine_is_compatible("nvidia,tegra194")) {
    WARN(1, "Only supported for Tegra194 devices!\n");
    return -EOPNOTSUPP;
    }
    writel_relaxed(ERD_MASK_INBAND_ERR,
    apbmisc_base + ERD_ERR_CONFIG);
    return 0;
    }
    EXPORT_SYMBOL(tegra194_miscreg_mask_serror);
    static const struct of_device_id apbmisc_match[] __initconst = {
    { .compatible = "nvidia,tegra20-apbmisc", },
    { .compatible = "nvidia,tegra186-misc", },
    { .compatible = "nvidia,tegra194-misc", },
    { .compatible = "nvidia,tegra234-misc", },
    { .compatible = "nvidia,tegra264-misc", },
    {},
    };
#[no_mangle]
pub unsafe extern "C" fn tegra_init_revision() -> void __init {
    void __init tegra_init_revision(void)
    {
    u8 chip_id, minor_rev;
    chip_id = tegra_get_chip_id();
    minor_rev = tegra_get_minor_rev();
    switch (minor_rev) {
    case 1:
    tegra_sku_info.revision = TEGRA_REVISION_A01;
    break;
    case 2:
    tegra_sku_info.revision = TEGRA_REVISION_A02;
    break;
    case 3:
    if (chip_id == TEGRA20 && (tegra_fuse_read_spare(18) ||
    tegra_fuse_read_spare(19)))
    tegra_sku_info.revision = TEGRA_REVISION_A03p;
    else
    tegra_sku_info.revision = TEGRA_REVISION_A03;
    break;
    case 4:
    tegra_sku_info.revision = TEGRA_REVISION_A04;
    break;
    default:
    tegra_sku_info.revision = TEGRA_REVISION_UNKNOWN;
    }
    tegra_sku_info.sku_id = tegra_fuse_read_early(FUSE_SKU_INFO);
    tegra_sku_info.platform = tegra_get_platform();
    }
    static void tegra_init_apbmisc_resources(struct resource *apbmisc,
    struct resource *straps)
    {
    void __iomem *strapping_base;
    apbmisc_base = ioremap(apbmisc.start, resource_size(apbmisc));
    if (!apbmisc_base)
    pr_err("failed to map APBMISC registers\n");
    strapping_base = ioremap(straps.start, resource_size(straps));
    if (strapping_base) {
    strapping = readl_relaxed(strapping_base);
    iounmap(strapping_base);
    } else {
    pr_err("failed to map strapping options registers\n");
    }
    }
//
// tegra_init_apbmisc - Initializes Tegra APBMISC and Strapping registers.
//
// This is called during early init as some of the old 32-bit ARM code needs
// information from the APBMISC registers very early during boot.
//
#[no_mangle]
pub unsafe extern "C" fn tegra_init_apbmisc() -> void __init {
    void __init tegra_init_apbmisc(void)
    {
    struct resource apbmisc, straps;
    struct device_node *np;
    np = of_find_matching_node(core::ptr::null_mut(), apbmisc_match);
    if (!np) {
//
// Fall back to legacy initialization for 32-bit ARM only. All
// 64-bit ARM device tree files for Tegra are required to have
// an APBMISC node.
//
// This is for backwards-compatibility with old device trees
// that didn't contain an APBMISC node.
//
    if (IS_ENABLED(CONFIG_ARM) && soc_is_tegra()) {
// APBMISC registers (chip revision, ...)
    apbmisc.start = 0x70000800;
    apbmisc.end = 0x70000863;
    apbmisc.flags = IORESOURCE_MEM;
// strapping options
    if (of_machine_is_compatible("nvidia,tegra124")) {
    straps.start = 0x7000e864;
    straps.end = 0x7000e867;
    } else {
    straps.start = 0x70000008;
    straps.end = 0x7000000b;
    }
    straps.flags = IORESOURCE_MEM;
    pr_warn("Using APBMISC region %pR\n", &apbmisc);
    pr_warn("Using strapping options registers %pR\n",
    &straps);
    } else {
//
// At this point we're not running on Tegra, so play
// nice with multi-platform kernels.
//
    return;
    }
    } else {
//
// Extract information from the device tree if we've found a
// matching node.
//
    if (of_address_to_resource(np, 0, &apbmisc) < 0) {
    pr_err("failed to get APBMISC registers\n");
    goto put;
    }
    if (of_address_to_resource(np, 1, &straps) < 0) {
    pr_err("failed to get strapping options registers\n");
    goto put;
    }
    }
    tegra_init_apbmisc_resources(&apbmisc, &straps);
    long_ram_code = of_property_read_bool(np, "nvidia,long-ram-code");
    put:
    of_node_put(np);
    }

    static const struct acpi_device_id apbmisc_acpi_match[] = {
    { "NVDA2010" },
    { /* sentinel */ }
    };
#[no_mangle]
pub unsafe extern "C" fn tegra_acpi_init_apbmisc() {
    void tegra_acpi_init_apbmisc(void)
    {
    struct resource *resources[2] = { core::ptr::null_mut() };
    struct resource_entry *rentry;
    struct acpi_device *adev = core::ptr::null_mut();
    struct list_head resource_list;
    let mut rcount: c_int = 0;
    int ret;
    adev = acpi_dev_get_first_match_dev(apbmisc_acpi_match[0].id, core::ptr::null_mut(), -1);
    if (!adev)
    return;
    INIT_LIST_HEAD(&resource_list);
    ret = acpi_dev_get_memory_resources(adev, &resource_list);
    if (ret < 0) {
    pr_err("failed to get APBMISC memory resources");
    goto out_put_acpi_dev;
    }
//
// Get required memory resources.
//
// resources[0]: apbmisc.
// resources[1]: straps.
//
    resource_list_for_each_entry(rentry, &resource_list) {
    if (rcount >= ARRAY_SIZE(resources))
    break;
    resources[rcount++] = rentry.res;
    }
    if (!resources[0]) {
    pr_err("failed to get APBMISC registers\n");
    goto out_free_resource_list;
    }
    if (!resources[1]) {
    pr_err("failed to get strapping options registers\n");
    goto out_free_resource_list;
    }
    tegra_init_apbmisc_resources(resources[0], resources[1]);
    out_free_resource_list:
    acpi_dev_free_resource_list(&resource_list);
    out_put_acpi_dev:
    acpi_dev_put(adev);
    }

#[no_mangle]
pub unsafe extern "C" fn tegra_acpi_init_apbmisc() {
    void tegra_acpi_init_apbmisc(void)
    {
    }
