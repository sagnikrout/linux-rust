//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-rpc-if.c
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
// RPC-IF SPI/QSPI/Octa driver
//
// Copyright (C) 2018 ~ 2019 Renesas Solutions Corp.
// Copyright (C) 2019 Macronix International Co., Ltd.
// Copyright (C) 2019 - 2020 Cogent Embedded, Inc.
//

    static void rpcif_spi_mem_prepare(struct spi_device *spi_dev,
    const struct spi_mem_op *spi_op,
    u64 *offs, size_t *len)
    {
    struct rpcif *rpc = spi_controller_get_devdata(spi_dev.controller);
    let mut rpc_op: rpcif_op = { };
    rpc_op.cmd.opcode = spi_op.cmd.opcode;
    rpc_op.cmd.buswidth = spi_op.cmd.buswidth;
    if (spi_op.addr.nbytes) {
    rpc_op.addr.buswidth = spi_op.addr.buswidth;
    rpc_op.addr.nbytes = spi_op.addr.nbytes;
    rpc_op.addr.val = spi_op.addr.val;
    }
    if (spi_op.dummy.nbytes) {
    rpc_op.dummy.buswidth = spi_op.dummy.buswidth;
    rpc_op.dummy.ncycles  = spi_op.dummy.nbytes * 8 /
    spi_op.dummy.buswidth;
    }
    if (spi_op.data.nbytes || (offs && len)) {
    rpc_op.data.buswidth = spi_op.data.buswidth;
    rpc_op.data.nbytes = spi_op.data.nbytes;
    switch (spi_op.data.dir) {
    case SPI_MEM_DATA_IN:
    rpc_op.data.dir = RPCIF_DATA_IN;
    rpc_op.data.buf.in = spi_op.data.buf.in;
    break;
    case SPI_MEM_DATA_OUT:
    rpc_op.data.dir = RPCIF_DATA_OUT;
    rpc_op.data.buf.out = spi_op.data.buf.out;
    break;
    case SPI_MEM_NO_DATA:
    rpc_op.data.dir = RPCIF_NO_DATA;
    break;
    }
    } else	{
    rpc_op.data.dir = RPCIF_NO_DATA;
    }
    rpcif_prepare(rpc.dev, &rpc_op, offs, len);
    }
    static bool rpcif_spi_mem_supports_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    if (!spi_mem_default_supports_op(mem, op))
    return false;
    if (op.data.buswidth > 4 || op.addr.buswidth > 4 ||
    op.dummy.buswidth > 4 || op.cmd.buswidth > 4 ||
    op.addr.nbytes > 4)
    return false;
    return true;
    }
    static ssize_t xspi_spi_mem_dirmap_write(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len, const void *buf)
    {
    struct rpcif *rpc = spi_controller_get_devdata(desc.mem.spi.controller);
    if (offs + desc.info.offset + len > U32_MAX)
    return -EINVAL;
    rpcif_spi_mem_prepare(desc.mem.spi, desc.info.op_tmpl, &offs, &len);
    return xspi_dirmap_write(rpc.dev, offs, len, buf);
    }
    static ssize_t rpcif_spi_mem_dirmap_read(struct spi_mem_dirmap_desc *desc,
    u64 offs, size_t len, void *buf)
    {
    struct rpcif *rpc =
    spi_controller_get_devdata(desc.mem.spi.controller);
    if (offs + desc.info.offset + len > U32_MAX)
    return -EINVAL;
    rpcif_spi_mem_prepare(desc.mem.spi, desc.info.op_tmpl, &offs, &len);
    return rpcif_dirmap_read(rpc.dev, offs, len, buf);
    }
#[no_mangle]
unsafe extern "C" fn rpcif_spi_mem_dirmap_create(desc: *mut spi_mem_dirmap_desc) -> c_int {
    static int rpcif_spi_mem_dirmap_create(struct spi_mem_dirmap_desc *desc)
    {
    struct rpcif *rpc =
    spi_controller_get_devdata(desc.mem.spi.controller);
    if (desc.info.offset + desc.info.length > U32_MAX)
    return -EINVAL;
    if (!rpcif_spi_mem_supports_op(desc.mem, desc.info.op_tmpl))
    return -EOPNOTSUPP;
    if (!rpc.dirmap)
    return -EOPNOTSUPP;
    if (!rpc.xspi && desc.info.op_tmpl.data.dir != SPI_MEM_DATA_IN)
    return -EOPNOTSUPP;
    return 0;
    }
    static int rpcif_spi_mem_exec_op(struct spi_mem *mem,
    const struct spi_mem_op *op)
    {
    struct rpcif *rpc =
    spi_controller_get_devdata(mem.spi.controller);
    rpcif_spi_mem_prepare(mem.spi, op, core::ptr::null_mut(), core::ptr::null_mut());
    return rpcif_manual_xfer(rpc.dev);
    }
    static const struct spi_controller_mem_ops rpcif_spi_mem_ops = {
    .supports_op	= rpcif_spi_mem_supports_op,
    .exec_op	= rpcif_spi_mem_exec_op,
    .dirmap_create	= rpcif_spi_mem_dirmap_create,
    .dirmap_read	= rpcif_spi_mem_dirmap_read,
    .dirmap_write	= xspi_spi_mem_dirmap_write,
    };
#[no_mangle]
unsafe extern "C" fn rpcif_spi_probe(pdev: *mut platform_device) -> c_int {
    static int rpcif_spi_probe(struct platform_device *pdev)
    {
    struct device *parent = pdev.dev.parent;
    struct spi_controller *ctlr;
    struct rpcif *rpc;
    int error;
    ctlr = devm_spi_alloc_host(&pdev.dev, sizeof(*rpc));
    if (!ctlr)
    return -ENOMEM;
    rpc = spi_controller_get_devdata(ctlr);
    error = rpcif_sw_init(rpc, parent);
    if (error)
    return error;
    platform_set_drvdata(pdev, ctlr);
    ctlr.dev.of_node = parent.of_node;
    pm_runtime_set_autosuspend_delay(rpc.dev, 200);
    pm_runtime_use_autosuspend(rpc.dev);
    pm_runtime_enable(rpc.dev);
    ctlr.num_chipselect = 1;
    ctlr.mem_ops = &rpcif_spi_mem_ops;
    ctlr.bits_per_word_mask = SPI_BPW_MASK(8);
    ctlr.mode_bits = SPI_CPOL | SPI_CPHA | SPI_TX_QUAD | SPI_RX_QUAD;
    ctlr.flags = SPI_CONTROLLER_HALF_DUPLEX;
    error = rpcif_hw_init(rpc.dev, false);
    if (error)
    goto out_disable_rpm;
    error = spi_register_controller(ctlr);
    if (error) {
    dev_err(&pdev.dev, "spi_register_controller failed\n");
    goto out_disable_rpm;
    }
    return 0;
    out_disable_rpm:
    pm_runtime_dont_use_autosuspend(rpc.dev);
    pm_runtime_disable(rpc.dev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn rpcif_spi_remove(pdev: *mut platform_device) {
    static void rpcif_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *ctlr = platform_get_drvdata(pdev);
    struct rpcif *rpc = spi_controller_get_devdata(ctlr);
    spi_unregister_controller(ctlr);
    pm_runtime_dont_use_autosuspend(rpc.dev);
    pm_runtime_disable(rpc.dev);
    }
#[no_mangle]
unsafe extern "C" fn rpcif_spi_suspend(dev: *mut device) -> c_int {
    static int rpcif_spi_suspend(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    return spi_controller_suspend(ctlr);
    }
#[no_mangle]
unsafe extern "C" fn rpcif_spi_resume(dev: *mut device) -> c_int {
    static int rpcif_spi_resume(struct device *dev)
    {
    struct spi_controller *ctlr = dev_get_drvdata(dev);
    struct rpcif *rpc = spi_controller_get_devdata(ctlr);
    int ret;
    ret = rpcif_hw_init(rpc.dev, false);
    if (ret)
    return ret;
    return spi_controller_resume(ctlr);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(rpcif_spi_pm_ops, rpcif_spi_suspend, rpcif_spi_resume);
    static const struct platform_device_id rpc_if_spi_id_table[] = {
    { .name = "rpc-if-spi" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, rpc_if_spi_id_table);
    static struct platform_driver rpcif_spi_driver = {
    .probe	= rpcif_spi_probe,
    .remove = rpcif_spi_remove,
    .id_table = rpc_if_spi_id_table,
    .driver = {
    .name	= "rpc-if-spi",
    .pm	= pm_sleep_ptr(&rpcif_spi_pm_ops),
    },
    };
    module_platform_driver(rpcif_spi_driver);
    MODULE_DESCRIPTION("Renesas RPC-IF SPI driver");
    MODULE_LICENSE("GPL v2");
