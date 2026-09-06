//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/hyperbus/rpc-if.c
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
// Linux driver for RPC-IF HyperFlash
//
// Copyright (C) 2019-2020 Cogent Embedded, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpcif_hyperbus {
    pub rpc: rpcif,
    pub ctlr: hyperbus_ctlr,
    pub hbdev: hyperbus_device,
}

    static const struct rpcif_op rpcif_op_tmpl = {
    .cmd = {
    .buswidth = 8,
    .ddr = true,
    },
    .ocmd = {
    .buswidth = 8,
    .ddr = true,
    },
    .addr = {
    .nbytes = 1,
    .buswidth = 8,
    .ddr = true,
    },
    .data = {
    .buswidth = 8,
    .ddr = true,
    },
    };
    static void rpcif_hb_prepare_read(struct rpcif *rpc, void *to,
    unsigned long from, ssize_t len)
    {
    let mut op: rpcif_op = rpcif_op_tmpl;
    op.cmd.opcode = HYPERBUS_RW_READ | HYPERBUS_AS_MEM;
    op.addr.val = from >> 1;
    op.dummy.buswidth = 1;
    op.dummy.ncycles = 15;
    op.data.dir = RPCIF_DATA_IN;
    op.data.nbytes = len;
    op.data.buf.in = to;
    rpcif_prepare(rpc.dev, &op, core::ptr::null_mut(), core::ptr::null_mut());
    }
    static void rpcif_hb_prepare_write(struct rpcif *rpc, unsigned long to,
    void *from, ssize_t len)
    {
    let mut op: rpcif_op = rpcif_op_tmpl;
    op.cmd.opcode = HYPERBUS_RW_WRITE | HYPERBUS_AS_MEM;
    op.addr.val = to >> 1;
    op.data.dir = RPCIF_DATA_OUT;
    op.data.nbytes = len;
    op.data.buf.out = from;
    rpcif_prepare(rpc.dev, &op, core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn rpcif_hb_read16(hbdev: *mut hyperbus_device, addr: c_ulong) -> u16 {
    static u16 rpcif_hb_read16(struct hyperbus_device *hbdev, unsigned long addr)
    {
    struct rpcif_hyperbus *hyperbus =
    container_of(hbdev, struct rpcif_hyperbus, hbdev);
    map_word data;
    rpcif_hb_prepare_read(&hyperbus.rpc, &data, addr, 2);
    rpcif_manual_xfer(hyperbus.rpc.dev);
    return data.x[0];
    }
    static void rpcif_hb_write16(struct hyperbus_device *hbdev, unsigned long addr,
    u16 data)
    {
    struct rpcif_hyperbus *hyperbus =
    container_of(hbdev, struct rpcif_hyperbus, hbdev);
    rpcif_hb_prepare_write(&hyperbus.rpc, addr, &data, 2);
    rpcif_manual_xfer(hyperbus.rpc.dev);
    }
    static void rpcif_hb_copy_from(struct hyperbus_device *hbdev, void *to,
    unsigned long from, ssize_t len)
    {
    struct rpcif_hyperbus *hyperbus =
    container_of(hbdev, struct rpcif_hyperbus, hbdev);
    rpcif_hb_prepare_read(&hyperbus.rpc, to, from, len);
    rpcif_dirmap_read(hyperbus.rpc.dev, from, len, to);
    }
    static const struct hyperbus_ops rpcif_hb_ops = {
    .read16 = rpcif_hb_read16,
    .write16 = rpcif_hb_write16,
    .copy_from = rpcif_hb_copy_from,
    };
#[no_mangle]
unsafe extern "C" fn rpcif_hb_probe(pdev: *mut platform_device) -> c_int {
    static int rpcif_hb_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rpcif_hyperbus *hyperbus;
    int error;
    hyperbus = devm_kzalloc(dev, sizeof(*hyperbus), GFP_KERNEL);
    if (!hyperbus)
    return -ENOMEM;
    error = rpcif_sw_init(&hyperbus.rpc, pdev.dev.parent);
    if (error)
    return error;
    platform_set_drvdata(pdev, hyperbus);
    pm_runtime_enable(hyperbus.rpc.dev);
    error = rpcif_hw_init(hyperbus.rpc.dev, true);
    if (error)
    goto out_disable_rpm;
    hyperbus.hbdev.map.size = hyperbus.rpc.size;
    hyperbus.hbdev.map.virt = hyperbus.rpc.dirmap;
    hyperbus.ctlr.dev = dev;
    hyperbus.ctlr.ops = &rpcif_hb_ops;
    hyperbus.hbdev.ctlr = &hyperbus.ctlr;
    hyperbus.hbdev.np = of_get_next_child(pdev.dev.parent.of_node, core::ptr::null_mut());
    error = hyperbus_register_device(&hyperbus.hbdev);
    if (error)
    goto out_disable_rpm;
    return 0;
    out_disable_rpm:
    pm_runtime_disable(hyperbus.rpc.dev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn rpcif_hb_remove(pdev: *mut platform_device) {
    static void rpcif_hb_remove(struct platform_device *pdev)
    {
    struct rpcif_hyperbus *hyperbus = platform_get_drvdata(pdev);
    hyperbus_unregister_device(&hyperbus.hbdev);
    pm_runtime_disable(hyperbus.rpc.dev);
    }
    static const struct platform_device_id rpc_if_hyperflash_id_table[] = {
    { .name = "rpc-if-hyperflash" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, rpc_if_hyperflash_id_table);
    static struct platform_driver rpcif_platform_driver = {
    .probe	= rpcif_hb_probe,
    .remove = rpcif_hb_remove,
    .id_table = rpc_if_hyperflash_id_table,
    .driver	= {
    .name	= "rpc-if-hyperflash",
    },
    };
    module_platform_driver(rpcif_platform_driver);
    MODULE_DESCRIPTION("Renesas RPC-IF HyperFlash driver");
    MODULE_LICENSE("GPL v2");
