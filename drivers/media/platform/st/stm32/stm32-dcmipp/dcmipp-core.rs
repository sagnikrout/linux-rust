//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/st/stm32/stm32-dcmipp/dcmipp-core.c
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
// Driver for STM32 Digital Camera Memory Interface Pixel Processor
//
// Copyright (C) STMicroelectronics SA 2023
// Authors: Hugues Fruchet <hugues.fruchet@foss.st.com>
// Alain Volmat <alain.volmat@foss.st.com>
// for STMicroelectronics.
//

    .src_ent = src,						\
    .src_pad = srcpad,					\
    .sink_ent = sink,					\
    .sink_pad = sinkpad,					\
    .flags = link_flags,					\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcmipp_device {
// The platform device
    pub pdev: platform_device,
    pub dev: *mut device,
// Hardware resources
    pub regs: *mut void __iomem,
    pub mclk: *mut clk,
    pub kclk: *mut clk,
// The pipeline configuration
    pub pipe_cfg: *const dcmipp_pipeline_config,
// The Associated media_device parent
    pub mdev: media_device,
// Internal v4l2 parent device
    pub v4l2_dev: v4l2_device,
// Entities
    pub entity: *mut dcmipp_ent_device,
    pub notifier: v4l2_async_notifier,
}

    static inline struct dcmipp_device *
    notifier_to_dcmipp(struct v4l2_async_notifier *n)
    {
    return container_of(n, struct dcmipp_device, notifier);
    }
// Structure which describes individual configuration for each entity
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcmipp_ent_config {
    pub name: *const c_char,
    struct dcmipp_ent_device *(*init)
    (struct device *dev, const char *entity_name,
    pub regs): *mut *mut v4l2_device v4l2_dev, void __iomem,
    pub ved): *mut *mut void (release)(struct dcmipp_ent_device,
}

// Structure which describes links between entities
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcmipp_ent_link {
    pub src_ent: c_uint,
    pub src_pad: u16,
    pub sink_ent: c_uint,
    pub sink_pad: u16,
    pub flags: u32,
}

// Structure which describes the whole topology
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcmipp_pipeline_config {
    pub ents: *const dcmipp_ent_config,
    pub num_ents: usize,
    pub links: *const dcmipp_ent_link,
    pub num_links: usize,
    pub hw_revision: u32,
    pub has_csi2: bool,
    pub needs_mclk: bool,
}

// --------------------------------------------------------------------------
// Topology Configuration
//
    static const struct dcmipp_ent_config stm32mp13_ent_config[] = {
    {
    .name = "dcmipp_input",
    .init = dcmipp_inp_ent_init,
    .release = dcmipp_inp_ent_release,
    },
    {
    .name = "dcmipp_dump_postproc",
    .init = dcmipp_byteproc_ent_init,
    .release = dcmipp_byteproc_ent_release,
    },
    {
    .name = "dcmipp_dump_capture",
    .init = dcmipp_bytecap_ent_init,
    .release = dcmipp_bytecap_ent_release,
    },
    };
pub const ID_INPUT: c_int = 0;
pub const ID_DUMP_BYTEPROC: c_int = 1;
pub const ID_DUMP_CAPTURE: c_int = 2;
    static const struct dcmipp_ent_link stm32mp13_ent_links[] = {
    DCMIPP_ENT_LINK(ID_INPUT, 1, ID_DUMP_BYTEPROC, 0,
    MEDIA_LNK_FL_ENABLED | MEDIA_LNK_FL_IMMUTABLE),
    DCMIPP_ENT_LINK(ID_DUMP_BYTEPROC, 1, ID_DUMP_CAPTURE,  0,
    MEDIA_LNK_FL_ENABLED | MEDIA_LNK_FL_IMMUTABLE),
    };
pub const DCMIPP_STM32MP13_VERR: c_uint = 0x10;
    static const struct dcmipp_pipeline_config stm32mp13_pipe_cfg = {
    .ents		= stm32mp13_ent_config,
    .num_ents	= ARRAY_SIZE(stm32mp13_ent_config),
    .links		= stm32mp13_ent_links,
    .num_links	= ARRAY_SIZE(stm32mp13_ent_links),
    .hw_revision	= DCMIPP_STM32MP13_VERR
    };
    static const struct dcmipp_ent_config stm32mp25_ent_config[] = {
    {
    .name = "dcmipp_input",
    .init = dcmipp_inp_ent_init,
    .release = dcmipp_inp_ent_release,
    },
    {
    .name = "dcmipp_dump_postproc",
    .init = dcmipp_byteproc_ent_init,
    .release = dcmipp_byteproc_ent_release,
    },
    {
    .name = "dcmipp_dump_capture",
    .init = dcmipp_bytecap_ent_init,
    .release = dcmipp_bytecap_ent_release,
    },
    };
    static const struct dcmipp_ent_link stm32mp25_ent_links[] = {
    DCMIPP_ENT_LINK(ID_INPUT, 1, ID_DUMP_BYTEPROC, 0,
    MEDIA_LNK_FL_ENABLED | MEDIA_LNK_FL_IMMUTABLE),
    DCMIPP_ENT_LINK(ID_DUMP_BYTEPROC, 1, ID_DUMP_CAPTURE,  0,
    MEDIA_LNK_FL_ENABLED | MEDIA_LNK_FL_IMMUTABLE),
    };
pub const DCMIPP_STM32MP25_VERR: c_uint = 0x30;
    static const struct dcmipp_pipeline_config stm32mp25_pipe_cfg = {
    .ents		= stm32mp25_ent_config,
    .num_ents	= ARRAY_SIZE(stm32mp25_ent_config),
    .links		= stm32mp25_ent_links,
    .num_links	= ARRAY_SIZE(stm32mp25_ent_links),
    .hw_revision    = DCMIPP_STM32MP25_VERR,
    .has_csi2	= true,
    .needs_mclk	= true
    };

    (f) == MEDIA_LNK_FL_ENABLED ? "ENABLED" :\
    (f) == MEDIA_LNK_FL_IMMUTABLE ? "IMMUTABLE" :\
    (f) == (MEDIA_LNK_FL_ENABLED |\
    MEDIA_LNK_FL_IMMUTABLE) ?\
    "ENABLED, IMMUTABLE" :\
    "UNKNOWN")
#[no_mangle]
unsafe extern "C" fn dcmipp_create_links(dcmipp: *mut dcmipp_device) -> c_int {
    static int dcmipp_create_links(struct dcmipp_device *dcmipp)
    {
    unsigned int i;
    int ret;
// Initialize the links between entities
    for (i = 0; i < dcmipp.pipe_cfg.num_links; i++) {
    const struct dcmipp_ent_link *link =
    &dcmipp.pipe_cfg.links[i];
    struct dcmipp_ent_device *ved_src =
    dcmipp.entity[link.src_ent];
    struct dcmipp_ent_device *ved_sink =
    dcmipp.entity[link.sink_ent];
    dev_dbg(dcmipp.dev, "Create link \"%s\":%d . %d:\"%s\" [%s]\n",
    dcmipp.pipe_cfg.ents[link.src_ent].name,
    link.src_pad, link.sink_pad,
    dcmipp.pipe_cfg.ents[link.sink_ent].name,
    LINK_FLAG_TO_STR(link.flags));
    ret = media_create_pad_link(ved_src.ent, link.src_pad,
    ved_sink.ent, link.sink_pad,
    link.flags);
    if (ret)
    return ret;
    }
    return 0;
    }
    static int dcmipp_graph_init(struct dcmipp_device *dcmipp);
#[no_mangle]
unsafe extern "C" fn dcmipp_create_subdevs(dcmipp: *mut dcmipp_device) -> c_int {
    static int dcmipp_create_subdevs(struct dcmipp_device *dcmipp)
    {
    int ret, i;
// Call all subdev inits
    for (i = 0; i < dcmipp.pipe_cfg.num_ents; i++) {
    const char *name = dcmipp.pipe_cfg.ents[i].name;
    dev_dbg(dcmipp.dev, "add subdev %s\n", name);
    dcmipp.entity[i] =
    dcmipp.pipe_cfg.ents[i].init(dcmipp.dev, name,
    &dcmipp.v4l2_dev,
    dcmipp.regs);
    if (IS_ERR(dcmipp.entity[i])) {
    dev_err(dcmipp.dev, "failed to init subdev %s\n",
    name);
    ret = PTR_ERR(dcmipp.entity[i]);
    goto err_init_entity;
    }
    }
// Initialize links
    ret = dcmipp_create_links(dcmipp);
    if (ret)
    goto err_init_entity;
    ret = dcmipp_graph_init(dcmipp);
    if (ret < 0)
    goto err_init_entity;
    return 0;
    err_init_entity:
    while (i-- > 0)
    dcmipp.pipe_cfg.ents[i].release(dcmipp.entity[i]);
    return ret;
    }
    static const struct of_device_id dcmipp_of_match[] = {
    { .compatible = "st,stm32mp13-dcmipp", .data = &stm32mp13_pipe_cfg },
    { .compatible = "st,stm32mp25-dcmipp", .data = &stm32mp25_pipe_cfg },
    { /* end node */ },
    };
    MODULE_DEVICE_TABLE(of, dcmipp_of_match);
#[no_mangle]
unsafe extern "C" fn dcmipp_irq_thread(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t dcmipp_irq_thread(int irq, void *arg)
    {
    struct dcmipp_device *dcmipp = arg;
    struct dcmipp_ent_device *ved;
    unsigned int i;
// Call irq thread of each entities of pipeline
    for (i = 0; i < dcmipp.pipe_cfg.num_ents; i++) {
    ved = dcmipp.entity[i];
    if (ved.thread_fn && ved.handler_ret == IRQ_WAKE_THREAD)
    ved.thread_fn(irq, ved);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn dcmipp_irq_callback(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t dcmipp_irq_callback(int irq, void *arg)
    {
    struct dcmipp_device *dcmipp = arg;
    struct dcmipp_ent_device *ved;
    let mut ret: irqreturn_t = IRQ_HANDLED;
    unsigned int i;
// Call irq handler of each entities of pipeline
    for (i = 0; i < dcmipp.pipe_cfg.num_ents; i++) {
    ved = dcmipp.entity[i];
    if (ved.handler)
    ved.handler_ret = ved.handler(irq, ved);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ved->thread_fn) -> else {
    else if (ved.thread_fn)
    ved.handler_ret = IRQ_WAKE_THREAD;
    else
    ved.handler_ret = IRQ_HANDLED;
    if (ved.handler_ret != IRQ_HANDLED)
    ret = ved.handler_ret;
    }
    return ret;
    }
    static int dcmipp_graph_notify_bound(struct v4l2_async_notifier *notifier,
    struct v4l2_subdev *subdev,
    struct v4l2_async_connection *asd)
    {
    struct dcmipp_device *dcmipp = notifier_to_dcmipp(notifier);
    let mut ret: c_int = -EINVAL;
    int src_pad, i;
    struct dcmipp_ent_device *sink;
    let mut vep: v4l2_fwnode_endpoint = { 0 };
    struct fwnode_handle *ep;
    enum v4l2_mbus_type supported_types[] = {
    V4L2_MBUS_PARALLEL, V4L2_MBUS_BT656, V4L2_MBUS_CSI2_DPHY
    };
    dev_dbg(dcmipp.dev, "Subdev \"%s\" bound\n", subdev.name);
//
// Link this sub-device to DCMIPP, it could be
// a parallel camera sensor or a CSI-2 to parallel bridge
//
    src_pad = media_entity_get_fwnode_pad(&subdev.entity,
    subdev.fwnode,
    MEDIA_PAD_FL_SOURCE);
// Get bus characteristics from devicetree
    ep = fwnode_graph_get_endpoint_by_id(dev_fwnode(dcmipp.dev), 0, 0,
    FWNODE_GRAPH_ENDPOINT_NEXT);
    if (!ep) {
    dev_err(dcmipp.dev, "Could not find the endpoint\n");
    return -ENODEV;
    }
// Check for supported MBUS type
    for (i = 0; i < ARRAY_SIZE(supported_types); i++) {
// Only MP25 supports CSI input
    if (supported_types[i] == V4L2_MBUS_CSI2_DPHY &&
    !dcmipp.pipe_cfg.has_csi2)
    continue;
    vep.bus_type = supported_types[i];
    ret = v4l2_fwnode_endpoint_parse(ep, &vep);
    if (!ret)
    break;
    }
    fwnode_handle_put(ep);
    if (ret) {
    dev_err(dcmipp.dev, "Could not parse the endpoint\n");
    return ret;
    }
    if (vep.bus_type != V4L2_MBUS_CSI2_DPHY &&
    vep.bus.parallel.bus_width == 0) {
    dev_err(dcmipp.dev, "Invalid parallel interface bus-width\n");
    return -ENODEV;
    }
// Only 8 bits bus width supported with BT656 bus
    if (vep.bus_type == V4L2_MBUS_BT656 &&
    vep.bus.parallel.bus_width != 8) {
    dev_err(dcmipp.dev, "BT656 bus conflicts with %u bits bus width (8 bits required)\n",
    vep.bus.parallel.bus_width);
    return -ENODEV;
    }
// Connect input device to the dcmipp_input subdev
    sink = dcmipp.entity[ID_INPUT];
    if (vep.bus_type != V4L2_MBUS_CSI2_DPHY) {
    sink.bus.flags = vep.bus.parallel.flags;
    sink.bus.bus_width = vep.bus.parallel.bus_width;
    sink.bus.data_shift = vep.bus.parallel.data_shift;
    }
    sink.bus_type = vep.bus_type;
    ret = media_create_pad_link(&subdev.entity, src_pad, sink.ent, 0,
    MEDIA_LNK_FL_IMMUTABLE |
    MEDIA_LNK_FL_ENABLED);
    if (ret) {
    dev_err(dcmipp.dev, "Failed to create media pad link with subdev \"%s\"\n",
    subdev.name);
    return ret;
    }
    dev_dbg(dcmipp.dev, "DCMIPP is now linked to \"%s\"\n", subdev.name);
    return 0;
    }
    static void dcmipp_graph_notify_unbind(struct v4l2_async_notifier *notifier,
    struct v4l2_subdev *sd,
    struct v4l2_async_connection *asd)
    {
    struct dcmipp_device *dcmipp = notifier_to_dcmipp(notifier);
    dev_dbg(dcmipp.dev, "Removing %s\n", sd.name);
    }
#[no_mangle]
unsafe extern "C" fn dcmipp_graph_notify_complete(notifier: *mut v4l2_async_notifier) -> c_int {
    static int dcmipp_graph_notify_complete(struct v4l2_async_notifier *notifier)
    {
    struct dcmipp_device *dcmipp = notifier_to_dcmipp(notifier);
    int ret;
// Register the media device
    ret = media_device_register(&dcmipp.mdev);
    if (ret) {
    dev_err(dcmipp.mdev.dev,
    "media device register failed (err=%d)\n", ret);
    return ret;
    }
// Expose all subdev's nodes
    ret = v4l2_device_register_subdev_nodes(&dcmipp.v4l2_dev);
    if (ret) {
    dev_err(dcmipp.mdev.dev,
    "dcmipp subdev nodes registration failed (err=%d)\n",
    ret);
    media_device_unregister(&dcmipp.mdev);
    return ret;
    }
    dev_dbg(dcmipp.dev, "Notify complete !\n");
    return 0;
    }
    static const struct v4l2_async_notifier_operations dcmipp_graph_notify_ops = {
    .bound = dcmipp_graph_notify_bound,
    .unbind = dcmipp_graph_notify_unbind,
    .complete = dcmipp_graph_notify_complete,
    };
#[no_mangle]
unsafe extern "C" fn dcmipp_graph_init(dcmipp: *mut dcmipp_device) -> c_int {
    static int dcmipp_graph_init(struct dcmipp_device *dcmipp)
    {
    struct v4l2_async_connection *asd;
    struct fwnode_handle *ep;
    int ret;
    ep = fwnode_graph_get_endpoint_by_id(dev_fwnode(dcmipp.dev), 0, 0,
    FWNODE_GRAPH_ENDPOINT_NEXT);
    if (!ep) {
    dev_err(dcmipp.dev, "Failed to get next endpoint\n");
    return -EINVAL;
    }
    v4l2_async_nf_init(&dcmipp.notifier, &dcmipp.v4l2_dev);
    asd = v4l2_async_nf_add_fwnode_remote(&dcmipp.notifier, ep,
    struct v4l2_async_connection);
    fwnode_handle_put(ep);
    if (IS_ERR(asd)) {
    dev_err(dcmipp.dev, "Failed to add fwnode remote subdev\n");
    return PTR_ERR(asd);
    }
    dcmipp.notifier.ops = &dcmipp_graph_notify_ops;
    ret = v4l2_async_nf_register(&dcmipp.notifier);
    if (ret < 0) {
    dev_err(dcmipp.dev, "Failed to register notifier\n");
    v4l2_async_nf_cleanup(&dcmipp.notifier);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcmipp_probe(pdev: *mut platform_device) -> c_int {
    static int dcmipp_probe(struct platform_device *pdev)
    {
    struct dcmipp_device *dcmipp;
    struct clk *kclk, *mclk;
    const struct dcmipp_pipeline_config *pipe_cfg;
    struct reset_control *rstc;
    int irq;
    int ret;
    dcmipp = devm_kzalloc(&pdev.dev, sizeof(*dcmipp), GFP_KERNEL);
    if (!dcmipp)
    return -ENOMEM;
    dcmipp.dev = &pdev.dev;
    pipe_cfg = device_get_match_data(dcmipp.dev);
    if (!pipe_cfg) {
    dev_err(&pdev.dev, "Can't get device data\n");
    return -ENODEV;
    }
    dcmipp.pipe_cfg = pipe_cfg;
    platform_set_drvdata(pdev, dcmipp);
// Get hardware resources from devicetree
    rstc = devm_reset_control_get_exclusive(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rstc))
    return dev_err_probe(&pdev.dev, PTR_ERR(rstc),
    "Could not get reset control\n");
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    dcmipp.regs = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(dcmipp.regs)) {
    dev_err(&pdev.dev, "Could not map registers\n");
    return PTR_ERR(dcmipp.regs);
    }
    ret = devm_request_threaded_irq(&pdev.dev, irq, dcmipp_irq_callback,
    dcmipp_irq_thread, IRQF_ONESHOT,
    dev_name(&pdev.dev), dcmipp);
    if (ret) {
    dev_err(&pdev.dev, "Unable to request irq %d\n", irq);
    return ret;
    }
// Reset device
    ret = reset_control_assert(rstc);
    if (ret) {
    dev_err(&pdev.dev, "Failed to assert the reset line\n");
    return ret;
    }
    usleep_range(3000, 5000);
    ret = reset_control_deassert(rstc);
    if (ret) {
    dev_err(&pdev.dev, "Failed to deassert the reset line\n");
    return ret;
    }
//
// In case of the DCMIPP has only 1 clock (such as on MP13), the
// clock might not be named.
//
    kclk = devm_clk_get(&pdev.dev,
    dcmipp.pipe_cfg.needs_mclk ? "kclk" : core::ptr::null_mut());
    if (IS_ERR(kclk))
    return dev_err_probe(&pdev.dev, PTR_ERR(kclk),
    "Unable to get kclk\n");
    dcmipp.kclk = kclk;
    if (dcmipp.pipe_cfg.needs_mclk) {
    mclk = devm_clk_get(&pdev.dev, "mclk");
    if (IS_ERR(mclk))
    return dev_err_probe(&pdev.dev, PTR_ERR(mclk),
    "Unable to get mclk\n");
    dcmipp.mclk = mclk;
    }
    dcmipp.entity = devm_kcalloc(&pdev.dev, dcmipp.pipe_cfg.num_ents,
    sizeof(*dcmipp.entity), GFP_KERNEL);
    if (!dcmipp.entity)
    return -ENOMEM;
// Register the v4l2 struct
    ret = v4l2_device_register(&pdev.dev, &dcmipp.v4l2_dev);
    if (ret) {
    dev_err(&pdev.dev,
    "v4l2 device register failed (err=%d)\n", ret);
    return ret;
    }
// Link the media device within the v4l2_device
    dcmipp.v4l2_dev.mdev = &dcmipp.mdev;
// Initialize media device
    strscpy(dcmipp.mdev.model, DCMIPP_MDEV_MODEL_NAME,
    sizeof(dcmipp.mdev.model));
    dcmipp.mdev.hw_revision = pipe_cfg.hw_revision;
    dcmipp.mdev.dev = &pdev.dev;
    media_device_init(&dcmipp.mdev);
// Initialize subdevs
    ret = dcmipp_create_subdevs(dcmipp);
    if (ret) {
    media_device_cleanup(&dcmipp.mdev);
    v4l2_device_unregister(&dcmipp.v4l2_dev);
    return ret;
    }
    pm_runtime_enable(dcmipp.dev);
    dev_info(&pdev.dev, "Probe done");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcmipp_remove(pdev: *mut platform_device) {
    static void dcmipp_remove(struct platform_device *pdev)
    {
    struct dcmipp_device *dcmipp = platform_get_drvdata(pdev);
    unsigned int i;
    pm_runtime_disable(&pdev.dev);
    v4l2_async_nf_unregister(&dcmipp.notifier);
    v4l2_async_nf_cleanup(&dcmipp.notifier);
    for (i = 0; i < dcmipp.pipe_cfg.num_ents; i++)
    dcmipp.pipe_cfg.ents[i].release(dcmipp.entity[i]);
    media_device_unregister(&dcmipp.mdev);
    media_device_cleanup(&dcmipp.mdev);
    v4l2_device_unregister(&dcmipp.v4l2_dev);
    }
#[no_mangle]
unsafe extern "C" fn dcmipp_runtime_suspend(dev: *mut device) -> c_int {
    static int dcmipp_runtime_suspend(struct device *dev)
    {
    struct dcmipp_device *dcmipp = dev_get_drvdata(dev);
    clk_disable_unprepare(dcmipp.kclk);
    clk_disable_unprepare(dcmipp.mclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcmipp_runtime_resume(dev: *mut device) -> c_int {
    static int dcmipp_runtime_resume(struct device *dev)
    {
    struct dcmipp_device *dcmipp = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(dcmipp.mclk);
    if (ret) {
    dev_err(dev, "%s: Failed to prepare_enable mclk\n", __func__);
    return ret;
    }
    ret = clk_prepare_enable(dcmipp.kclk);
    if (ret) {
    clk_disable_unprepare(dcmipp.mclk);
    dev_err(dev, "%s: Failed to prepare_enable kclk\n", __func__);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dcmipp_suspend(dev: *mut device) -> c_int {
    static int dcmipp_suspend(struct device *dev)
    {
// disable clock
    pm_runtime_force_suspend(dev);
// change pinctrl state
    pinctrl_pm_select_sleep_state(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcmipp_resume(dev: *mut device) -> c_int {
    static int dcmipp_resume(struct device *dev)
    {
// restore pinctl default state
    pinctrl_pm_select_default_state(dev);
// clock enable
    return pm_runtime_force_resume(dev);
    }
    static const struct dev_pm_ops dcmipp_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(dcmipp_suspend, dcmipp_resume)
    RUNTIME_PM_OPS(dcmipp_runtime_suspend, dcmipp_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver dcmipp_pdrv = {
    .probe		= dcmipp_probe,
    .remove		= dcmipp_remove,
    .driver		= {
    .name	= DCMIPP_PDEV_NAME,
    .of_match_table = dcmipp_of_match,
    .pm = pm_ptr(&dcmipp_pm_ops),
    },
    };
    module_platform_driver(dcmipp_pdrv);
    MODULE_AUTHOR("Hugues Fruchet <hugues.fruchet@foss.st.com>");
    MODULE_AUTHOR("Alain Volmat <alain.volmat@foss.st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STM32 Digital Camera Memory Interface with Pixel Processor driver");
    MODULE_LICENSE("GPL");
