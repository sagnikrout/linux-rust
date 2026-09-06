//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/amlogic/c3/isp/c3-isp-dev.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2024 Amlogic, Inc. All rights reserved
//

#[no_mangle]
pub unsafe extern "C" fn c3_isp_read(isp: *mut c3_isp_device, reg: u32) -> u32 {
    u32 c3_isp_read(struct c3_isp_device *isp, u32 reg)
    {
    return readl(isp.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn c3_isp_write(isp: *mut c3_isp_device, reg: u32, val: u32) {
    void c3_isp_write(struct c3_isp_device *isp, u32 reg, u32 val)
    {
    writel(val, isp.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn c3_isp_update_bits(isp: *mut c3_isp_device, reg: u32, mask: u32, val: u32) {
    void c3_isp_update_bits(struct c3_isp_device *isp, u32 reg, u32 mask, u32 val)
    {
    u32 orig, tmp;
    orig = c3_isp_read(isp, reg);
    tmp = orig & ~mask;
    tmp |= val & mask;
    if (tmp != orig)
    c3_isp_write(isp, reg, tmp);
    }
// PM runtime suspend
#[no_mangle]
unsafe extern "C" fn c3_isp_runtime_suspend(dev: *mut device) -> c_int {
    static int c3_isp_runtime_suspend(struct device *dev)
    {
    struct c3_isp_device *isp = dev_get_drvdata(dev);
    clk_bulk_disable_unprepare(isp.info.clock_num, isp.clks);
    return 0;
    }
// PM runtime resume
#[no_mangle]
unsafe extern "C" fn c3_isp_runtime_resume(dev: *mut device) -> c_int {
    static int c3_isp_runtime_resume(struct device *dev)
    {
    struct c3_isp_device *isp = dev_get_drvdata(dev);
    return clk_bulk_prepare_enable(isp.info.clock_num, isp.clks);
    }
    static const struct dev_pm_ops c3_isp_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    RUNTIME_PM_OPS(c3_isp_runtime_suspend,
    c3_isp_runtime_resume, core::ptr::null_mut())
    };
// IRQ handling
#[no_mangle]
unsafe extern "C" fn c3_isp_irq_handler(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t c3_isp_irq_handler(int irq, void *dev)
    {
    struct c3_isp_device *isp = dev;
    u32 status;
// Get irq status and clear irq status
    status = c3_isp_read(isp, ISP_TOP_RO_IRQ_STAT);
    c3_isp_write(isp, ISP_TOP_IRQ_CLR, status);
    if (status & ISP_TOP_RO_IRQ_STAT_FRM_END_MASK) {
    c3_isp_stats_isr(isp);
    c3_isp_params_isr(isp);
    c3_isp_captures_isr(isp);
    isp.frm_sequence++;
    }
    if (status & ISP_TOP_RO_IRQ_STAT_FRM_RST_MASK)
    c3_isp_core_queue_sof(isp);
    return IRQ_HANDLED;
    }
// Subdev notifier register
    static int c3_isp_notify_bound(struct v4l2_async_notifier *notifier,
    struct v4l2_subdev *sd,
    struct v4l2_async_connection *asc)
    {
    struct c3_isp_device *isp =
    container_of(notifier, struct c3_isp_device, notifier);
    struct media_pad *sink =
    &isp.core.sd.entity.pads[C3_ISP_CORE_PAD_SINK_VIDEO];
    return v4l2_create_fwnode_links_to_pad(sd, sink, MEDIA_LNK_FL_ENABLED |
    MEDIA_LNK_FL_IMMUTABLE);
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_notify_complete(notifier: *mut v4l2_async_notifier) -> c_int {
    static int c3_isp_notify_complete(struct v4l2_async_notifier *notifier)
    {
    struct c3_isp_device *isp =
    container_of(notifier, struct c3_isp_device, notifier);
    return v4l2_device_register_subdev_nodes(&isp.v4l2_dev);
    }
    static const struct v4l2_async_notifier_operations c3_isp_notify_ops = {
    .bound = c3_isp_notify_bound,
    .complete = c3_isp_notify_complete,
    };
#[no_mangle]
unsafe extern "C" fn c3_isp_async_nf_register(isp: *mut c3_isp_device) -> c_int {
    static int c3_isp_async_nf_register(struct c3_isp_device *isp)
    {
    struct v4l2_async_connection *asc;
    struct fwnode_handle *ep;
    int ret;
    v4l2_async_nf_init(&isp.notifier, &isp.v4l2_dev);
    ep = fwnode_graph_get_endpoint_by_id(dev_fwnode(isp.dev), 0, 0,
    FWNODE_GRAPH_ENDPOINT_NEXT);
    if (!ep)
    return -ENOTCONN;
    asc = v4l2_async_nf_add_fwnode_remote(&isp.notifier, ep,
    struct v4l2_async_connection);
    fwnode_handle_put(ep);
    if (IS_ERR(asc))
    return PTR_ERR(asc);
    isp.notifier.ops = &c3_isp_notify_ops;
    ret = v4l2_async_nf_register(&isp.notifier);
    if (ret)
    v4l2_async_nf_cleanup(&isp.notifier);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_async_nf_unregister(isp: *mut c3_isp_device) {
    static void c3_isp_async_nf_unregister(struct c3_isp_device *isp)
    {
    v4l2_async_nf_unregister(&isp.notifier);
    v4l2_async_nf_cleanup(&isp.notifier);
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_media_register(isp: *mut c3_isp_device) -> c_int {
    static int c3_isp_media_register(struct c3_isp_device *isp)
    {
    struct media_device *media_dev = &isp.media_dev;
    struct v4l2_device *v4l2_dev = &isp.v4l2_dev;
    int ret;
// Initialize media device
    strscpy(media_dev.model, C3_ISP_DRIVER_NAME, sizeof(media_dev.model));
    media_dev.dev = isp.dev;
    media_device_init(media_dev);
// Initialize v4l2 device
    v4l2_dev.mdev = media_dev;
    strscpy(v4l2_dev.name, C3_ISP_DRIVER_NAME, sizeof(v4l2_dev.name));
    ret = v4l2_device_register(isp.dev, v4l2_dev);
    if (ret)
    goto err_media_dev_cleanup;
    ret = media_device_register(&isp.media_dev);
    if (ret) {
    dev_err(isp.dev, "Failed to register media device: %d\n", ret);
    goto err_unreg_v4l2_dev;
    }
    return 0;
    err_unreg_v4l2_dev:
    v4l2_device_unregister(&isp.v4l2_dev);
    err_media_dev_cleanup:
    media_device_cleanup(media_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_media_unregister(isp: *mut c3_isp_device) {
    static void c3_isp_media_unregister(struct c3_isp_device *isp)
    {
    media_device_unregister(&isp.media_dev);
    v4l2_device_unregister(&isp.v4l2_dev);
    media_device_cleanup(&isp.media_dev);
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_remove_links(isp: *mut c3_isp_device) {
    static void c3_isp_remove_links(struct c3_isp_device *isp)
    {
    unsigned int i;
    media_entity_remove_links(&isp.core.sd.entity);
    for (i = 0; i < C3_ISP_NUM_RSZ; i++)
    media_entity_remove_links(&isp.resizers[i].sd.entity);
    for (i = 0; i < C3_ISP_NUM_CAP_DEVS; i++)
    media_entity_remove_links(&isp.caps[i].vdev.entity);
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_create_links(isp: *mut c3_isp_device) -> c_int {
    static int c3_isp_create_links(struct c3_isp_device *isp)
    {
    unsigned int i;
    int ret;
    for (i = 0; i < C3_ISP_NUM_RSZ; i++) {
    ret = media_create_pad_link(&isp.resizers[i].sd.entity,
    C3_ISP_RSZ_PAD_SOURCE,
    &isp.caps[i].vdev.entity, 0,
    MEDIA_LNK_FL_ENABLED |
    MEDIA_LNK_FL_IMMUTABLE);
    if (ret) {
    dev_err(isp.dev,
    "Failed to link rsz %u and cap %u\n", i, i);
    goto err_remove_links;
    }
    ret = media_create_pad_link(&isp.core.sd.entity,
    C3_ISP_CORE_PAD_SOURCE_VIDEO_0 + i,
    &isp.resizers[i].sd.entity,
    C3_ISP_RSZ_PAD_SINK,
    MEDIA_LNK_FL_ENABLED);
    if (ret) {
    dev_err(isp.dev,
    "Failed to link core and rsz %u\n", i);
    goto err_remove_links;
    }
    }
    ret = media_create_pad_link(&isp.core.sd.entity,
    C3_ISP_CORE_PAD_SOURCE_STATS,
    &isp.stats.vdev.entity,
    0, MEDIA_LNK_FL_ENABLED);
    if (ret) {
    dev_err(isp.dev, "Failed to link core and stats\n");
    goto err_remove_links;
    }
    ret = media_create_pad_link(&isp.params.vdev.entity, 0,
    &isp.core.sd.entity,
    C3_ISP_CORE_PAD_SINK_PARAMS,
    MEDIA_LNK_FL_ENABLED);
    if (ret) {
    dev_err(isp.dev, "Failed to link params and core\n");
    goto err_remove_links;
    }
    return 0;
    err_remove_links:
    c3_isp_remove_links(isp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_videos_register(isp: *mut c3_isp_device) -> c_int {
    static int c3_isp_videos_register(struct c3_isp_device *isp)
    {
    int ret;
    ret = c3_isp_captures_register(isp);
    if (ret)
    return ret;
    ret = c3_isp_stats_register(isp);
    if (ret)
    goto err_captures_unregister;
    ret = c3_isp_params_register(isp);
    if (ret)
    goto err_stats_unregister;
    ret = c3_isp_create_links(isp);
    if (ret)
    goto err_params_unregister;
    return 0;
    err_params_unregister:
    c3_isp_params_unregister(isp);
    err_stats_unregister:
    c3_isp_stats_unregister(isp);
    err_captures_unregister:
    c3_isp_captures_unregister(isp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_videos_unregister(isp: *mut c3_isp_device) {
    static void c3_isp_videos_unregister(struct c3_isp_device *isp)
    {
    c3_isp_remove_links(isp);
    c3_isp_params_unregister(isp);
    c3_isp_stats_unregister(isp);
    c3_isp_captures_unregister(isp);
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_get_clocks(isp: *mut c3_isp_device) -> c_int {
    static int c3_isp_get_clocks(struct c3_isp_device *isp)
    {
    const struct c3_isp_info *info = isp.info;
    for (unsigned int i = 0; i < info.clock_num; i++)
    isp.clks[i].id = info.clocks[i];
    return devm_clk_bulk_get(isp.dev, info.clock_num, isp.clks);
    }
#[no_mangle]
unsafe extern "C" fn c3_isp_probe(pdev: *mut platform_device) -> c_int {
    static int c3_isp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct c3_isp_device *isp;
    int irq;
    int ret;
    isp = devm_kzalloc(dev, sizeof(*isp), GFP_KERNEL);
    if (!isp)
    return -ENOMEM;
    isp.info = of_device_get_match_data(dev);
    isp.dev = dev;
    isp.base = devm_platform_ioremap_resource_byname(pdev, "isp");
    if (IS_ERR(isp.base))
    return dev_err_probe(dev, PTR_ERR(isp.base),
    "Failed to ioremap resource\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = c3_isp_get_clocks(isp);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to get clocks\n");
    platform_set_drvdata(pdev, isp);
    pm_runtime_enable(dev);
    ret = c3_isp_media_register(isp);
    if (ret)
    goto err_runtime_disable;
    ret = c3_isp_core_register(isp);
    if (ret)
    goto err_v4l2_unregister;
    ret = c3_isp_resizers_register(isp);
    if (ret)
    goto err_core_unregister;
    ret = c3_isp_async_nf_register(isp);
    if (ret)
    goto err_resizers_unregister;
    ret = devm_request_irq(dev, irq,
    c3_isp_irq_handler, IRQF_SHARED,
    dev_driver_string(dev), isp);
    if (ret)
    goto err_nf_unregister;
    ret = c3_isp_videos_register(isp);
    if (ret)
    goto err_nf_unregister;
    return 0;
    err_nf_unregister:
    c3_isp_async_nf_unregister(isp);
    err_resizers_unregister:
    c3_isp_resizers_unregister(isp);
    err_core_unregister:
    c3_isp_core_unregister(isp);
    err_v4l2_unregister:
    c3_isp_media_unregister(isp);
    err_runtime_disable:
    pm_runtime_disable(dev);
    return ret;
    };
#[no_mangle]
unsafe extern "C" fn c3_isp_remove(pdev: *mut platform_device) {
    static void c3_isp_remove(struct platform_device *pdev)
    {
    struct c3_isp_device *isp = platform_get_drvdata(pdev);
    c3_isp_videos_unregister(isp);
    c3_isp_async_nf_unregister(isp);
    c3_isp_core_unregister(isp);
    c3_isp_resizers_unregister(isp);
    c3_isp_media_unregister(isp);
    pm_runtime_disable(isp.dev);
    };
    static const struct c3_isp_info isp_info = {
    .clocks = {"vapb", "isp0"},
    .clock_num = 2
    };
    static const struct of_device_id c3_isp_of_match[] = {
    { .compatible = "amlogic,c3-isp",
    .data = &isp_info },
    { },
    };
    MODULE_DEVICE_TABLE(of, c3_isp_of_match);
    static struct platform_driver c3_isp_driver = {
    .probe = c3_isp_probe,
    .remove = c3_isp_remove,
    .driver = {
    .name = "c3-isp",
    .of_match_table = c3_isp_of_match,
    .pm = pm_ptr(&c3_isp_pm_ops),
    },
    };
    module_platform_driver(c3_isp_driver);
    MODULE_AUTHOR("Keke Li <keke.li@amlogic.com>");
    MODULE_DESCRIPTION("Amlogic C3 ISP pipeline");
    MODULE_LICENSE("GPL");
