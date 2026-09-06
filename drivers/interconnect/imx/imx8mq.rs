//! Automatically rewritten from C to Rust
//! Source: drivers/interconnect/imx/imx8mq.c
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
// Interconnect framework driver for i.MX8MQ SoC
//
// Copyright (c) 2019-2020, NXP
//

    static const struct imx_icc_node_adj_desc imx8mq_dram_adj = {
    .bw_mul = 1,
    .bw_div = 4,
    .phandle_name = "fsl,ddrc",
    };
    static const struct imx_icc_node_adj_desc imx8mq_noc_adj = {
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
    DEFINE_BUS_INTERCONNECT("NOC", IMX8MQ_ICN_NOC, &imx8mq_noc_adj,
    IMX8MQ_ICS_DRAM, IMX8MQ_ICN_MAIN),
    DEFINE_BUS_SLAVE("DRAM", IMX8MQ_ICS_DRAM, &imx8mq_dram_adj),
    DEFINE_BUS_SLAVE("OCRAM", IMX8MQ_ICS_OCRAM, core::ptr::null_mut()),
    DEFINE_BUS_MASTER("A53", IMX8MQ_ICM_A53, IMX8MQ_ICN_NOC),
// VPUMIX
    DEFINE_BUS_MASTER("VPU", IMX8MQ_ICM_VPU, IMX8MQ_ICN_VIDEO),
    DEFINE_BUS_INTERCONNECT("PL301_VIDEO", IMX8MQ_ICN_VIDEO, core::ptr::null_mut(), IMX8MQ_ICN_NOC),
// GPUMIX
    DEFINE_BUS_MASTER("GPU", IMX8MQ_ICM_GPU, IMX8MQ_ICN_GPU),
    DEFINE_BUS_INTERCONNECT("PL301_GPU", IMX8MQ_ICN_GPU, core::ptr::null_mut(), IMX8MQ_ICN_NOC),
// DISPMIX (only for DCSS)
    DEFINE_BUS_MASTER("DC", IMX8MQ_ICM_DCSS, IMX8MQ_ICN_DCSS),
    DEFINE_BUS_INTERCONNECT("PL301_DC", IMX8MQ_ICN_DCSS, core::ptr::null_mut(), IMX8MQ_ICN_NOC),
// USBMIX
    DEFINE_BUS_MASTER("USB1", IMX8MQ_ICM_USB1, IMX8MQ_ICN_USB),
    DEFINE_BUS_MASTER("USB2", IMX8MQ_ICM_USB2, IMX8MQ_ICN_USB),
    DEFINE_BUS_INTERCONNECT("PL301_USB", IMX8MQ_ICN_USB, core::ptr::null_mut(), IMX8MQ_ICN_NOC),
// PL301_DISPLAY (IPs other than DCSS, inside SUPERMIX)
    DEFINE_BUS_MASTER("CSI1", IMX8MQ_ICM_CSI1, IMX8MQ_ICN_DISPLAY),
    DEFINE_BUS_MASTER("CSI2", IMX8MQ_ICM_CSI2, IMX8MQ_ICN_DISPLAY),
    DEFINE_BUS_MASTER("LCDIF", IMX8MQ_ICM_LCDIF, IMX8MQ_ICN_DISPLAY),
    DEFINE_BUS_INTERCONNECT("PL301_DISPLAY", IMX8MQ_ICN_DISPLAY, core::ptr::null_mut(), IMX8MQ_ICN_MAIN),
// AUDIO
    DEFINE_BUS_MASTER("SDMA2", IMX8MQ_ICM_SDMA2, IMX8MQ_ICN_AUDIO),
    DEFINE_BUS_INTERCONNECT("PL301_AUDIO", IMX8MQ_ICN_AUDIO, core::ptr::null_mut(), IMX8MQ_ICN_DISPLAY),
// ENET
    DEFINE_BUS_MASTER("ENET", IMX8MQ_ICM_ENET, IMX8MQ_ICN_ENET),
    DEFINE_BUS_INTERCONNECT("PL301_ENET", IMX8MQ_ICN_ENET, core::ptr::null_mut(), IMX8MQ_ICN_MAIN),
// OTHER
    DEFINE_BUS_MASTER("SDMA1", IMX8MQ_ICM_SDMA1, IMX8MQ_ICN_MAIN),
    DEFINE_BUS_MASTER("NAND", IMX8MQ_ICM_NAND, IMX8MQ_ICN_MAIN),
    DEFINE_BUS_MASTER("USDHC1", IMX8MQ_ICM_USDHC1, IMX8MQ_ICN_MAIN),
    DEFINE_BUS_MASTER("USDHC2", IMX8MQ_ICM_USDHC2, IMX8MQ_ICN_MAIN),
    DEFINE_BUS_MASTER("PCIE1", IMX8MQ_ICM_PCIE1, IMX8MQ_ICN_MAIN),
    DEFINE_BUS_MASTER("PCIE2", IMX8MQ_ICM_PCIE2, IMX8MQ_ICN_MAIN),
    DEFINE_BUS_INTERCONNECT("PL301_MAIN", IMX8MQ_ICN_MAIN, core::ptr::null_mut(),
    IMX8MQ_ICN_NOC, IMX8MQ_ICS_OCRAM),
    };
#[no_mangle]
unsafe extern "C" fn imx8mq_icc_probe(pdev: *mut platform_device) -> c_int {
    static int imx8mq_icc_probe(struct platform_device *pdev)
    {
    return imx_icc_register(pdev, nodes, ARRAY_SIZE(nodes), core::ptr::null_mut());
    }
    static struct platform_driver imx8mq_icc_driver = {
    .probe = imx8mq_icc_probe,
    .remove = imx_icc_unregister,
    .driver = {
    .name = "imx8mq-interconnect",
    .sync_state = icc_sync_state,
    },
    };
    module_platform_driver(imx8mq_icc_driver);
    MODULE_ALIAS("platform:imx8mq-interconnect");
    MODULE_AUTHOR("Leonard Crestez <leonard.crestez@nxp.com>");
    MODULE_DESCRIPTION("Interconnect framework driver for i.MX8MQ SoC");
    MODULE_LICENSE("GPL v2");
