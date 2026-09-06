//! Automatically rewritten from C to Rust
//! Source: drivers/cdx/controller/cdx_rpmsg.c
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
// Platform driver for CDX bus.
//
// Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
//

    static struct rpmsg_device_id cdx_rpmsg_id_table[] = {
    { .name = "mcdi_ipc" },
    { },
    };
    MODULE_DEVICE_TABLE(rpmsg, cdx_rpmsg_id_table);
    int cdx_rpmsg_send(struct cdx_mcdi *cdx_mcdi,
    const struct cdx_dword *hdr, size_t hdr_len,
    const struct cdx_dword *sdu, size_t sdu_len)
    {
    unsigned char *send_buf;
    int ret;
    send_buf = kzalloc(hdr_len + sdu_len, GFP_KERNEL);
    if (!send_buf)
    return -ENOMEM;
    memcpy(send_buf, hdr, hdr_len);
    memcpy(send_buf + hdr_len, sdu, sdu_len);
    ret = rpmsg_send(cdx_mcdi.ept, send_buf, hdr_len + sdu_len);
    kfree(send_buf);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdx_attach_to_rproc(pdev: *mut platform_device) -> c_int {
    static int cdx_attach_to_rproc(struct platform_device *pdev)
    {
    struct device_node *r5_core_node;
    struct cdx_controller *cdx_c;
    struct cdx_mcdi *cdx_mcdi;
    struct device *dev;
    struct rproc *rp;
    int ret;
    dev = &pdev.dev;
    cdx_c = platform_get_drvdata(pdev);
    cdx_mcdi = cdx_c.priv;
    r5_core_node = of_parse_phandle(dev.of_node, "xlnx,rproc", 0);
    if (!r5_core_node) {
    dev_err(&pdev.dev, "xlnx,rproc: invalid phandle\n");
    return -EINVAL;
    }
    rp = rproc_get_by_phandle(r5_core_node.phandle);
    if (!rp) {
    ret = -EPROBE_DEFER;
    goto pdev_err;
    }
// Attach to remote processor
    ret = rproc_boot(rp);
    if (ret) {
    dev_err(&pdev.dev, "Failed to attach to remote processor\n");
    rproc_put(rp);
    goto pdev_err;
    }
    cdx_mcdi.r5_rproc = rp;
    pdev_err:
    of_node_put(r5_core_node);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cdx_detach_to_r5(pdev: *mut platform_device) {
    static void cdx_detach_to_r5(struct platform_device *pdev)
    {
    struct cdx_controller *cdx_c;
    struct cdx_mcdi *cdx_mcdi;
    cdx_c = platform_get_drvdata(pdev);
    cdx_mcdi = cdx_c.priv;
    rproc_detach(cdx_mcdi.r5_rproc);
    rproc_put(cdx_mcdi.r5_rproc);
    }
    static int cdx_rpmsg_cb(struct rpmsg_device *rpdev, void *data,
    int len, void *priv, u32 src)
    {
    struct cdx_controller *cdx_c = dev_get_drvdata(&rpdev.dev);
    struct cdx_mcdi *cdx_mcdi = cdx_c.priv;
    if (len > MCDI_BUF_LEN)
    return -EINVAL;
    cdx_mcdi_process_cmd(cdx_mcdi, (struct cdx_dword *)data, len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdx_rpmsg_post_probe_work(work: *mut work_struct) {
    static void cdx_rpmsg_post_probe_work(struct work_struct *work)
    {
    struct cdx_controller *cdx_c;
    struct cdx_mcdi *cdx_mcdi;
    cdx_mcdi = container_of(work, struct cdx_mcdi, work);
    cdx_c = dev_get_drvdata(&cdx_mcdi.rpdev.dev);
    cdx_rpmsg_post_probe(cdx_c);
    }
#[no_mangle]
unsafe extern "C" fn cdx_rpmsg_probe(rpdev: *mut rpmsg_device) -> c_int {
    static int cdx_rpmsg_probe(struct rpmsg_device *rpdev)
    {
    let mut chinfo: rpmsg_channel_info = {0};
    struct cdx_controller *cdx_c;
    struct cdx_mcdi *cdx_mcdi;
    cdx_c = (struct cdx_controller *)cdx_rpmsg_id_table[0].driver_data;
    cdx_mcdi = cdx_c.priv;
    chinfo.src = RPMSG_ADDR_ANY;
    chinfo.dst = rpdev.dst;
    strscpy(chinfo.name, cdx_rpmsg_id_table[0].name, sizeof(chinfo.name));
    cdx_mcdi.ept = rpmsg_create_ept(rpdev, cdx_rpmsg_cb, core::ptr::null_mut(), chinfo);
    if (!cdx_mcdi.ept) {
    dev_err_probe(&rpdev.dev, -ENXIO,
    "Failed to create ept for channel %s\n",
    chinfo.name);
    return -EINVAL;
    }
    cdx_mcdi.rpdev = rpdev;
    dev_set_drvdata(&rpdev.dev, cdx_c);
    schedule_work(&cdx_mcdi.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cdx_rpmsg_remove(rpdev: *mut rpmsg_device) {
    static void cdx_rpmsg_remove(struct rpmsg_device *rpdev)
    {
    struct cdx_controller *cdx_c = dev_get_drvdata(&rpdev.dev);
    struct cdx_mcdi *cdx_mcdi = cdx_c.priv;
    flush_work(&cdx_mcdi.work);
    cdx_rpmsg_pre_remove(cdx_c);
    rpmsg_destroy_ept(cdx_mcdi.ept);
    dev_set_drvdata(&rpdev.dev, core::ptr::null_mut());
    }
    static struct rpmsg_driver cdx_rpmsg_driver = {
    .drv.name = KBUILD_MODNAME,
    .id_table = cdx_rpmsg_id_table,
    .probe = cdx_rpmsg_probe,
    .remove = cdx_rpmsg_remove,
    .callback = cdx_rpmsg_cb,
    };
#[no_mangle]
pub unsafe extern "C" fn cdx_setup_rpmsg(pdev: *mut platform_device) -> c_int {
    int cdx_setup_rpmsg(struct platform_device *pdev)
    {
    struct cdx_controller *cdx_c;
    struct cdx_mcdi *cdx_mcdi;
    int ret;
// Attach to remote processor
    ret = cdx_attach_to_rproc(pdev);
    if (ret)
    return ret;
    cdx_c = platform_get_drvdata(pdev);
    cdx_mcdi = cdx_c.priv;
// Register RPMsg driver
    cdx_rpmsg_id_table[0].driver_data = (kernel_ulong_t)cdx_c;
    INIT_WORK(&cdx_mcdi.work, cdx_rpmsg_post_probe_work);
    ret = register_rpmsg_driver(&cdx_rpmsg_driver);
    if (ret) {
    dev_err(&pdev.dev,
    "Failed to register cdx RPMsg driver: %d\n", ret);
    cdx_detach_to_r5(pdev);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cdx_destroy_rpmsg(pdev: *mut platform_device) {
    void cdx_destroy_rpmsg(struct platform_device *pdev)
    {
    unregister_rpmsg_driver(&cdx_rpmsg_driver);
    cdx_detach_to_r5(pdev);
    }
