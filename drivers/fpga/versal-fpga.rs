//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/versal-fpga.c
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
// Copyright (C) 2019-2021 Xilinx, Inc.
//

    static int versal_fpga_ops_write_init(struct fpga_manager *mgr,
    struct fpga_image_info *info,
    const char *buf, size_t size)
    {
    return 0;
    }
    static int versal_fpga_ops_write(struct fpga_manager *mgr,
    const char *buf, size_t size)
    {
    let mut dma_addr: dma_addr_t = 0;
    char *kbuf;
    int ret;
    kbuf = dma_alloc_coherent(mgr.dev.parent, size, &dma_addr, GFP_KERNEL);
    if (!kbuf)
    return -ENOMEM;
    memcpy(kbuf, buf, size);
    ret = zynqmp_pm_load_pdi(PDI_SRC_DDR, dma_addr);
    dma_free_coherent(mgr.dev.parent, size, kbuf, dma_addr);
    return ret;
    }
    static const struct fpga_manager_ops versal_fpga_ops = {
    .write_init = versal_fpga_ops_write_init,
    .write = versal_fpga_ops_write,
    };
#[no_mangle]
unsafe extern "C" fn versal_fpga_probe(pdev: *mut platform_device) -> c_int {
    static int versal_fpga_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct fpga_manager *mgr;
    int ret;
    ret = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(44));
    if (ret < 0) {
    dev_err(dev, "no usable DMA configuration\n");
    return ret;
    }
    mgr = devm_fpga_mgr_register(dev, "Xilinx Versal FPGA Manager",
    &versal_fpga_ops, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(mgr);
    }
    static const struct of_device_id versal_fpga_of_match[] = {
    { .compatible = "xlnx,versal-fpga", },
    {},
    };
    MODULE_DEVICE_TABLE(of, versal_fpga_of_match);
    static struct platform_driver versal_fpga_driver = {
    .probe = versal_fpga_probe,
    .driver = {
    .name = "versal_fpga_manager",
    .of_match_table = versal_fpga_of_match,
    },
    };
    module_platform_driver(versal_fpga_driver);
    MODULE_AUTHOR("Nava kishore Manne <nava.manne@xilinx.com>");
    MODULE_AUTHOR("Appana Durga Kedareswara rao <appanad.durga.rao@xilinx.com>");
    MODULE_DESCRIPTION("Xilinx Versal FPGA Manager");
    MODULE_LICENSE("GPL");
