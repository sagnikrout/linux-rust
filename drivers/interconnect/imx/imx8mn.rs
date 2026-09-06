//! Automatically rewritten from C to Rust
//! Source: drivers/interconnect/imx/imx8mn.c
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
// Interconnect framework driver for i.MX8MN SoC
//
// Copyright (c) 2019-2020, NXP
//

    static const struct imx_icc_node_adj_desc imx8mn_dram_adj = {
    .bw_mul = 1,
    .bw_div = 4,
    .phandle_name = "fsl,ddrc",
    };
    static const struct imx_icc_node_adj_desc imx8mn_noc_adj = {
    .bw_mul = 1,
    .bw_div = 4,
    .main_noc = true,
    };
//
// Describe bus masters, slaves and connections between them
//
// This is a simplified subset of the bus diagram, there are several other
// PL301 nics which are skipped/merged into PL301_MAIN
//
    static struct imx_icc_node_desc nodes[] = {
    DEFINE_BUS_INTERCONNECT("NOC", IMX8MN_ICN_NOC, &imx8mn_noc_adj,
    IMX8MN_ICS_DRAM, IMX8MN_ICN_MAIN),
    DEFINE_BUS_SLAVE("DRAM", IMX8MN_ICS_DRAM, &imx8mn_dram_adj),
    DEFINE_BUS_SLAVE("OCRAM", IMX8MN_ICS_OCRAM, core::ptr::null_mut()),
    DEFINE_BUS_MASTER("A53", IMX8MN_ICM_A53, IMX8MN_ICN_NOC),
// GPUMIX
    DEFINE_BUS_MASTER("GPU", IMX8MN_ICM_GPU, IMX8MN_ICN_GPU),
    DEFINE_BUS_INTERCONNECT("PL301_GPU", IMX8MN_ICN_GPU, core::ptr::null_mut(), IMX8MN_ICN_NOC),
// DISPLAYMIX
    DEFINE_BUS_MASTER("CSI1", IMX8MN_ICM_CSI1, IMX8MN_ICN_MIPI),
    DEFINE_BUS_MASTER("CSI2", IMX8MN_ICM_CSI2, IMX8MN_ICN_MIPI),
    DEFINE_BUS_MASTER("ISI", IMX8MN_ICM_ISI, IMX8MN_ICN_MIPI),
    DEFINE_BUS_MASTER("LCDIF", IMX8MN_ICM_LCDIF, IMX8MN_ICN_MIPI),
    DEFINE_BUS_INTERCONNECT("PL301_MIPI", IMX8MN_ICN_MIPI, core::ptr::null_mut(), IMX8MN_ICN_NOC),
// USB goes straight to NOC
    DEFINE_BUS_MASTER("USB", IMX8MN_ICM_USB, IMX8MN_ICN_NOC),
// Audio
    DEFINE_BUS_MASTER("SDMA2", IMX8MN_ICM_SDMA2, IMX8MN_ICN_AUDIO),
    DEFINE_BUS_MASTER("SDMA3", IMX8MN_ICM_SDMA3, IMX8MN_ICN_AUDIO),
    DEFINE_BUS_INTERCONNECT("PL301_AUDIO", IMX8MN_ICN_AUDIO, core::ptr::null_mut(), IMX8MN_ICN_MAIN),
// Ethernet
    DEFINE_BUS_MASTER("ENET", IMX8MN_ICM_ENET, IMX8MN_ICN_ENET),
    DEFINE_BUS_INTERCONNECT("PL301_ENET", IMX8MN_ICN_ENET, core::ptr::null_mut(), IMX8MN_ICN_MAIN),
// Other
    DEFINE_BUS_MASTER("SDMA1", IMX8MN_ICM_SDMA1, IMX8MN_ICN_MAIN),
    DEFINE_BUS_MASTER("NAND", IMX8MN_ICM_NAND, IMX8MN_ICN_MAIN),
    DEFINE_BUS_MASTER("USDHC1", IMX8MN_ICM_USDHC1, IMX8MN_ICN_MAIN),
    DEFINE_BUS_MASTER("USDHC2", IMX8MN_ICM_USDHC2, IMX8MN_ICN_MAIN),
    DEFINE_BUS_MASTER("USDHC3", IMX8MN_ICM_USDHC3, IMX8MN_ICN_MAIN),
    DEFINE_BUS_INTERCONNECT("PL301_MAIN", IMX8MN_ICN_MAIN, core::ptr::null_mut(),
    IMX8MN_ICN_NOC, IMX8MN_ICS_OCRAM),
    };
#[no_mangle]
unsafe extern "C" fn imx8mn_icc_probe(pdev: *mut platform_device) -> c_int {
    static int imx8mn_icc_probe(struct platform_device *pdev)
    {
    return imx_icc_register(pdev, nodes, ARRAY_SIZE(nodes), core::ptr::null_mut());
    }
    static struct platform_driver imx8mn_icc_driver = {
    .probe = imx8mn_icc_probe,
    .remove = imx_icc_unregister,
    .driver = {
    .name = "imx8mn-interconnect",
    },
    };
    module_platform_driver(imx8mn_icc_driver);
    MODULE_ALIAS("platform:imx8mn-interconnect");
    MODULE_AUTHOR("Leonard Crestez <leonard.crestez@nxp.com>");
    MODULE_DESCRIPTION("Interconnect framework driver for i.MX8MN SoC");
    MODULE_LICENSE("GPL v2");
