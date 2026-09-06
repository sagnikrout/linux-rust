//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/xilinx-spi.c
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
// Xilinx Spartan6 and 7 Series Slave Serial SPI Driver
//
// Copyright (C) 2017 DENX Software Engineering
//
// Anatolij Gustschin <agust@denx.de>
//
// Manage Xilinx FPGA firmware that is loaded over SPI using
// the slave serial configuration interface.
//

    static int xilinx_spi_write(struct xilinx_fpga_core *core, const char *buf,
    size_t count)
    {
    struct spi_device *spi = to_spi_device(core.dev);
    const char *fw_data = buf;
    const char *fw_data_end = fw_data + count;
    while (fw_data < fw_data_end) {
    size_t remaining, stride;
    int ret;
    remaining = fw_data_end - fw_data;
    stride = min_t(size_t, remaining, SZ_4K);
    ret = spi_write(spi, fw_data, stride);
    if (ret) {
    dev_err(core.dev, "SPI error in firmware write: %d\n",
    ret);
    return ret;
    }
    fw_data += stride;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xilinx_spi_probe(spi: *mut spi_device) -> c_int {
    static int xilinx_spi_probe(struct spi_device *spi)
    {
    struct xilinx_fpga_core *core;
    core = devm_kzalloc(&spi.dev, sizeof(*core), GFP_KERNEL);
    if (!core)
    return -ENOMEM;
    core.dev = &spi.dev;
    core.write = xilinx_spi_write;
    return xilinx_core_probe(core);
    }
    static const struct spi_device_id xilinx_spi_ids[] = {
    { "fpga-slave-serial" },
    { },
    };
    MODULE_DEVICE_TABLE(spi, xilinx_spi_ids);

    static const struct of_device_id xlnx_spi_of_match[] = {
    {
    .compatible = "xlnx,fpga-slave-serial",
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, xlnx_spi_of_match);

    static struct spi_driver xilinx_slave_spi_driver = {
    .driver = {
    .name = "xlnx-slave-spi",
    .of_match_table = of_match_ptr(xlnx_spi_of_match),
    },
    .probe = xilinx_spi_probe,
    .id_table = xilinx_spi_ids,
    };
    module_spi_driver(xilinx_slave_spi_driver)
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Anatolij Gustschin <agust@denx.de>");
    MODULE_DESCRIPTION("Load Xilinx FPGA firmware over SPI");
