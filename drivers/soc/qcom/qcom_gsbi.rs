//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/qcom_gsbi.c
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
// Copyright (c) 2014, The Linux foundation. All rights reserved.
//

pub const GSBI_CTRL_REG: c_uint = 0x0000;
pub const GSBI_PROTOCOL_SHIFT: c_int = 4;
pub const MAX_GSBI: c_int = 12;
pub const TCSR_ADM_CRCI_BASE: c_uint = 0x70;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crci_config {
    pub num_rows: u32,
    pub (*array)[MAX_GSBI]: *const u32,
}

    static const u32 crci_ipq8064[][MAX_GSBI] = {
    {
    0x000003, 0x00000c, 0x000030, 0x0000c0,
    0x000300, 0x000c00, 0x003000, 0x00c000,
    0x030000, 0x0c0000, 0x300000, 0xc00000
    },
    {
    0x000003, 0x00000c, 0x000030, 0x0000c0,
    0x000300, 0x000c00, 0x003000, 0x00c000,
    0x030000, 0x0c0000, 0x300000, 0xc00000
    },
    };
    static const struct crci_config config_ipq8064 = {
    .num_rows = ARRAY_SIZE(crci_ipq8064),
    .array = crci_ipq8064,
    };
    static const unsigned int crci_apq8064[][MAX_GSBI] = {
    {
    0x001800, 0x006000, 0x000030, 0x0000c0,
    0x000300, 0x000400, 0x000000, 0x000000,
    0x000000, 0x000000, 0x000000, 0x000000
    },
    {
    0x000000, 0x000000, 0x000000, 0x000000,
    0x000000, 0x000020, 0x0000c0, 0x000000,
    0x000000, 0x000000, 0x000000, 0x000000
    },
    };
    static const struct crci_config config_apq8064 = {
    .num_rows = ARRAY_SIZE(crci_apq8064),
    .array = crci_apq8064,
    };
    static const unsigned int crci_msm8960[][MAX_GSBI] = {
    {
    0x000003, 0x00000c, 0x000030, 0x0000c0,
    0x000300, 0x000400, 0x000000, 0x000000,
    0x000000, 0x000000, 0x000000, 0x000000
    },
    {
    0x000000, 0x000000, 0x000000, 0x000000,
    0x000000, 0x000020, 0x0000c0, 0x000300,
    0x001800, 0x006000, 0x000000, 0x000000
    },
    };
    static const struct crci_config config_msm8960 = {
    .num_rows = ARRAY_SIZE(crci_msm8960),
    .array = crci_msm8960,
    };
    static const unsigned int crci_msm8660[][MAX_GSBI] = {
    {	/* ADM 0 - B */
    0x000003, 0x00000c, 0x000030, 0x0000c0,
    0x000300, 0x000c00, 0x003000, 0x00c000,
    0x030000, 0x0c0000, 0x300000, 0xc00000
    },
    {	/* ADM 0 - B */
    0x000003, 0x00000c, 0x000030, 0x0000c0,
    0x000300, 0x000c00, 0x003000, 0x00c000,
    0x030000, 0x0c0000, 0x300000, 0xc00000
    },
    {	/* ADM 1 - A */
    0x000003, 0x00000c, 0x000030, 0x0000c0,
    0x000300, 0x000c00, 0x003000, 0x00c000,
    0x030000, 0x0c0000, 0x300000, 0xc00000
    },
    {	/* ADM 1 - B */
    0x000003, 0x00000c, 0x000030, 0x0000c0,
    0x000300, 0x000c00, 0x003000, 0x00c000,
    0x030000, 0x0c0000, 0x300000, 0xc00000
    },
    };
    static const struct crci_config config_msm8660 = {
    .num_rows = ARRAY_SIZE(crci_msm8660),
    .array = crci_msm8660,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsbi_info {
    pub hclk: *mut clk,
    pub mode: u32,
    pub crci: u32,
    pub tcsr: *mut regmap,
}

    static const struct of_device_id tcsr_dt_match[] __maybe_unused = {
    { .compatible = "qcom,tcsr-ipq8064", .data = &config_ipq8064},
    { .compatible = "qcom,tcsr-apq8064", .data = &config_apq8064},
    { .compatible = "qcom,tcsr-msm8960", .data = &config_msm8960},
    { .compatible = "qcom,tcsr-msm8660", .data = &config_msm8660},
    { },
    };
#[no_mangle]
unsafe extern "C" fn gsbi_probe(pdev: *mut platform_device) -> c_int {
    static int gsbi_probe(struct platform_device *pdev)
    {
    struct device_node *node = pdev.dev.of_node;
    struct device_node *tcsr_node;
    const struct of_device_id *match;
    void __iomem *base;
    struct gsbi_info *gsbi;
    int i;
    u32 mask, gsbi_num;
    const struct crci_config *config = core::ptr::null_mut();
    gsbi = devm_kzalloc(&pdev.dev, sizeof(*gsbi), GFP_KERNEL);
    if (!gsbi)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
// get the tcsr node and setup the config and regmap
    gsbi.tcsr = syscon_regmap_lookup_by_phandle(node, "syscon-tcsr");
    if (!IS_ERR(gsbi.tcsr)) {
    tcsr_node = of_parse_phandle(node, "syscon-tcsr", 0);
    if (tcsr_node) {
    match = of_match_node(tcsr_dt_match, tcsr_node);
    if (match)
    config = match.data;
    else
    dev_warn(&pdev.dev, "no matching TCSR\n");
    of_node_put(tcsr_node);
    }
    }
    if (of_property_read_u32(node, "cell-index", &gsbi_num)) {
    dev_err(&pdev.dev, "missing cell-index\n");
    return -EINVAL;
    }
    if (gsbi_num < 1 || gsbi_num > MAX_GSBI) {
    dev_err(&pdev.dev, "invalid cell-index\n");
    return -EINVAL;
    }
    if (of_property_read_u32(node, "qcom,mode", &gsbi.mode)) {
    dev_err(&pdev.dev, "missing mode configuration\n");
    return -EINVAL;
    }
// not required, so default to 0 if not present
    of_property_read_u32(node, "qcom,crci", &gsbi.crci);
    dev_info(&pdev.dev, "GSBI port protocol: %d crci: %d\n",
    gsbi.mode, gsbi.crci);
    gsbi.hclk = devm_clk_get_enabled(&pdev.dev, "iface");
    if (IS_ERR(gsbi.hclk))
    return PTR_ERR(gsbi.hclk);
    writel_relaxed((gsbi.mode << GSBI_PROTOCOL_SHIFT) | gsbi.crci,
    base + GSBI_CTRL_REG);
//
// modify tcsr to reflect mode and ADM CRCI mux
// Each gsbi contains a pair of bits, one for RX and one for TX
// SPI mode requires both bits cleared, otherwise they are set
//
    if (config) {
    for (i = 0; i < config.num_rows; i++) {
    mask = config.array[i][gsbi_num - 1];
    if (gsbi.mode == GSBI_PROT_SPI)
    regmap_update_bits(gsbi.tcsr,
    TCSR_ADM_CRCI_BASE + 4 * i, mask, 0);
    else
    regmap_update_bits(gsbi.tcsr,
    TCSR_ADM_CRCI_BASE + 4 * i, mask, mask);
    }
    }
// make sure the gsbi control write is not reordered
    wmb();
    platform_set_drvdata(pdev, gsbi);
    return of_platform_populate(node, core::ptr::null_mut(), core::ptr::null_mut(), &pdev.dev);
    }
    static const struct of_device_id gsbi_dt_match[] = {
    { .compatible = "qcom,gsbi-v1.0.0", },
    { },
    };
    MODULE_DEVICE_TABLE(of, gsbi_dt_match);
    static struct platform_driver gsbi_driver = {
    .driver = {
    .name		= "gsbi",
    .of_match_table	= gsbi_dt_match,
    },
    .probe = gsbi_probe,
    };
    module_platform_driver(gsbi_driver);
    MODULE_AUTHOR("Andy Gross <agross@codeaurora.org>");
    MODULE_DESCRIPTION("QCOM GSBI driver");
    MODULE_LICENSE("GPL v2");
