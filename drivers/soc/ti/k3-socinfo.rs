//! Automatically rewritten from C to Rust
//! Source: drivers/soc/ti/k3-socinfo.c
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
// TI K3 SoC info driver
//
// Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com
//

pub const CTRLMMR_WKUP_JTAGID_REG: c_int = 0;
//
// Bits:
// 31-28 VARIANT	Device variant
// 27-12 PARTNO	Part number
// 11-1  MFG		Indicates TI as manufacturer (0x17)
// 0			Always 1
//

pub const CTRLMMR_WKUP_JTAGID_MFG_TI: c_uint = 0x17;
pub const JTAG_ID_PARTNO_AM65X: c_uint = 0xBB5A;
pub const JTAG_ID_PARTNO_J721E: c_uint = 0xBB64;
pub const JTAG_ID_PARTNO_J7200: c_uint = 0xBB6D;
pub const JTAG_ID_PARTNO_AM64X: c_uint = 0xBB38;
pub const JTAG_ID_PARTNO_J721S2: c_uint = 0xBB75;
pub const JTAG_ID_PARTNO_AM62X: c_uint = 0xBB7E;
pub const JTAG_ID_PARTNO_J784S4: c_uint = 0xBB80;
pub const JTAG_ID_PARTNO_AM62AX: c_uint = 0xBB8D;
pub const JTAG_ID_PARTNO_AM62PX: c_uint = 0xBB9D;
pub const JTAG_ID_PARTNO_J722S: c_uint = 0xBBA0;
pub const JTAG_ID_PARTNO_AM62LX: c_uint = 0xBBA7;
    static const struct k3_soc_id {
    unsigned int id;
    const char *family_name;
    } k3_soc_ids[] = {
    { JTAG_ID_PARTNO_AM65X, "AM65X" },
    { JTAG_ID_PARTNO_J721E, "J721E" },
    { JTAG_ID_PARTNO_J7200, "J7200" },
    { JTAG_ID_PARTNO_AM64X, "AM64X" },
    { JTAG_ID_PARTNO_J721S2, "J721S2"},
    { JTAG_ID_PARTNO_AM62X, "AM62X" },
    { JTAG_ID_PARTNO_J784S4, "J784S4" },
    { JTAG_ID_PARTNO_AM62AX, "AM62AX" },
    { JTAG_ID_PARTNO_AM62PX, "AM62PX" },
    { JTAG_ID_PARTNO_J722S, "J722S" },
    { JTAG_ID_PARTNO_AM62LX, "AM62LX" },
    };
    static const char * const j721e_rev_string_map[] = {
    "1.0", "1.1", "2.0",
    };
    static const char * const am62lx_rev_string_map[] = {
    "1.0", "1.1",
    };
    static const char * const am62p_gpsw_rev_string_map[] = {
    "1.0", "1.1", "1.2",
    };
    static int
    k3_chipinfo_get_gpsw_variant(struct device *dev)
    {
    let mut gpsw_val: u32 = 0;
    int ret;
    ret = nvmem_cell_read_u32(dev, "gpsw1", &gpsw_val);
    if (ret)
    return ret;
    return gpsw_val & GP_SW1_ADR_MASK;
    }
    static int
    k3_chipinfo_partno_to_names(unsigned int partno,
    struct soc_device_attribute *soc_dev_attr)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(k3_soc_ids); i++)
    if (partno == k3_soc_ids[i].id) {
    soc_dev_attr.family = k3_soc_ids[i].family_name;
    return 0;
    }
    return -ENODEV;
    }
    static int
    k3_chipinfo_variant_to_sr(struct device *dev, unsigned int partno,
    unsigned int variant, struct soc_device_attribute *soc_dev_attr)
    {
    let mut gpsw_variant: c_int = 0;
    switch (partno) {
    case JTAG_ID_PARTNO_J721E:
    if (variant >= ARRAY_SIZE(j721e_rev_string_map))
    goto err_unknown_variant;
    soc_dev_attr.revision = kasprintf(GFP_KERNEL, "SR%s",
    j721e_rev_string_map[variant]);
    break;
    case JTAG_ID_PARTNO_AM62LX:
    if (variant >= ARRAY_SIZE(am62lx_rev_string_map))
    goto err_unknown_variant;
    soc_dev_attr.revision = kasprintf(GFP_KERNEL, "SR%s",
    am62lx_rev_string_map[variant]);
    break;
    case JTAG_ID_PARTNO_AM62PX:
// Check GP_SW1 for silicon revision
    gpsw_variant = k3_chipinfo_get_gpsw_variant(dev);
    if (gpsw_variant == -EPROBE_DEFER)
    return gpsw_variant;
    if (gpsw_variant < 0 || gpsw_variant >= ARRAY_SIZE(am62p_gpsw_rev_string_map)) {
    dev_warn(dev, "Failed to get silicon variant (%d), set SR1.0\n",
    gpsw_variant);
    gpsw_variant = 0;
    }
    soc_dev_attr.revision = kasprintf(GFP_KERNEL, "SR%s",
    am62p_gpsw_rev_string_map[gpsw_variant]);
    break;
    default:
    variant++;
    soc_dev_attr.revision = kasprintf(GFP_KERNEL, "SR%x.0",
    variant);
    }
    if (!soc_dev_attr.revision)
    return -ENOMEM;
    return 0;
    err_unknown_variant:
    return -ENODEV;
    }
    static const struct regmap_config k3_chipinfo_regmap_cfg = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
#[no_mangle]
unsafe extern "C" fn k3_chipinfo_probe(pdev: *mut platform_device) -> c_int {
    static int k3_chipinfo_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct soc_device_attribute *soc_dev_attr;
    struct device *dev = &pdev.dev;
    struct soc_device *soc_dev;
    struct regmap *regmap;
    void __iomem *base;
    u32 partno_id;
    u32 variant;
    u32 jtag_id;
    u32 mfg;
    int ret;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = devm_regmap_init_mmio(dev, base, &k3_chipinfo_regmap_cfg);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    ret = regmap_read(regmap, CTRLMMR_WKUP_JTAGID_REG, &jtag_id);
    if (ret < 0)
    return ret;
    mfg = (jtag_id & CTRLMMR_WKUP_JTAGID_MFG_MASK) >>
    CTRLMMR_WKUP_JTAGID_MFG_SHIFT;
    if (mfg != CTRLMMR_WKUP_JTAGID_MFG_TI) {
    dev_err(dev, "Invalid MFG SoC\n");
    return -ENODEV;
    }
    variant = (jtag_id & CTRLMMR_WKUP_JTAGID_VARIANT_MASK) >>
    CTRLMMR_WKUP_JTAGID_VARIANT_SHIFT;
    partno_id = (jtag_id & CTRLMMR_WKUP_JTAGID_PARTNO_MASK) >>
    CTRLMMR_WKUP_JTAGID_PARTNO_SHIFT;
    soc_dev_attr = kzalloc_obj(*soc_dev_attr);
    if (!soc_dev_attr)
    return -ENOMEM;
    ret = k3_chipinfo_partno_to_names(partno_id, soc_dev_attr);
    if (ret) {
    dev_err(dev, "Unknown SoC JTAGID[0x%08X]: %d\n", jtag_id, ret);
    goto err;
    }
    ret = k3_chipinfo_variant_to_sr(dev, partno_id, variant, soc_dev_attr);
    if (ret) {
    dev_err(dev, "Unknown SoC SR[0x%08X]: %d\n", jtag_id, ret);
    goto err;
    }
    node = of_find_node_by_path("/");
    of_property_read_string(node, "model", &soc_dev_attr.machine);
    of_node_put(node);
    soc_dev = soc_device_register(soc_dev_attr);
    if (IS_ERR(soc_dev)) {
    ret = PTR_ERR(soc_dev);
    goto err_free_rev;
    }
    dev_info(dev, "Family:%s rev:%s JTAGID[0x%08x] Detected\n",
    soc_dev_attr.family,
    soc_dev_attr.revision, jtag_id);
    return 0;
    err_free_rev:
    kfree(soc_dev_attr.revision);
    err:
    kfree(soc_dev_attr);
    return ret;
    }
    static const struct of_device_id k3_chipinfo_of_match[] = {
    { .compatible = "ti,am654-chipid", },
    { /* sentinel */ },
    };
    static struct platform_driver k3_chipinfo_driver = {
    .driver = {
    .name = "k3-chipinfo",
    .of_match_table = k3_chipinfo_of_match,
    },
    .probe = k3_chipinfo_probe,
    };
#[no_mangle]
unsafe extern "C" fn k3_chipinfo_init() -> int __init {
    static int __init k3_chipinfo_init(void)
    {
    return platform_driver_register(&k3_chipinfo_driver);
    }
    subsys_initcall(k3_chipinfo_init);
