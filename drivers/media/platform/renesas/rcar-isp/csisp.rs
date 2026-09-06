//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/renesas/rcar-isp/csisp.c
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
// Copyright (C) 2021 Renesas Electronics Corp.
//
// Driver for Renesas R-Car ISP Channel Selector
//
// The ISP hardware is capable of more than just channel selection, features
// such as demosaicing, white balance control and color space conversion are
// also possible. These more advanced features are not supported by the driver
// due to lack of documentation.
//

pub const ISPINPUTSEL0_REG: c_uint = 0x0008;

pub const ISPSTART_REG: c_uint = 0x0014;
pub const ISPSTART_START: c_uint = 0xffff;
pub const ISPSTART_STOP: c_uint = 0x0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_isp_format {
    pub code: u32,
    pub datatype: c_uint,
    pub procmode: c_uint,
}

    static const struct rcar_isp_format rcar_isp_formats[] = {
    {
    .code = MEDIA_BUS_FMT_RGB888_1X24,
    .datatype = MIPI_CSI2_DT_RGB888,
    .procmode = 0x15
    }, {
    .code = MEDIA_BUS_FMT_Y10_1X10,
    .datatype = MIPI_CSI2_DT_RAW10,
    .procmode = 0x10,
    }, {
    .code = MEDIA_BUS_FMT_UYVY8_1X16,
    .datatype = MIPI_CSI2_DT_YUV422_8B,
    .procmode = 0x0c,
    }, {
    .code = MEDIA_BUS_FMT_YUYV8_1X16,
    .datatype = MIPI_CSI2_DT_YUV422_8B,
    .procmode = 0x0c,
    }, {
    .code = MEDIA_BUS_FMT_UYVY8_2X8,
    .datatype = MIPI_CSI2_DT_YUV422_8B,
    .procmode = 0x0c,
    }, {
    .code = MEDIA_BUS_FMT_YUYV10_2X10,
    .datatype = MIPI_CSI2_DT_YUV422_8B,
    .procmode = 0x0c,
    }, {
    .code = MEDIA_BUS_FMT_SBGGR8_1X8,
    .datatype = MIPI_CSI2_DT_RAW8,
    .procmode = 0x00,
    }, {
    .code = MEDIA_BUS_FMT_SGBRG8_1X8,
    .datatype = MIPI_CSI2_DT_RAW8,
    .procmode = 0x00,
    }, {
    .code = MEDIA_BUS_FMT_SGRBG8_1X8,
    .datatype = MIPI_CSI2_DT_RAW8,
    .procmode = 0x00,
    }, {
    .code = MEDIA_BUS_FMT_SRGGB8_1X8,
    .datatype = MIPI_CSI2_DT_RAW8,
    .procmode = 0x00,
    }, {
    .code = MEDIA_BUS_FMT_SBGGR10_1X10,
    .datatype = MIPI_CSI2_DT_RAW10,
    .procmode = 0x01,
    }, {
    .code = MEDIA_BUS_FMT_SGBRG10_1X10,
    .datatype = MIPI_CSI2_DT_RAW10,
    .procmode = 0x01,
    }, {
    .code = MEDIA_BUS_FMT_SGRBG10_1X10,
    .datatype = MIPI_CSI2_DT_RAW10,
    .procmode = 0x01,
    }, {
    .code = MEDIA_BUS_FMT_SRGGB10_1X10,
    .datatype = MIPI_CSI2_DT_RAW10,
    .procmode = 0x01,
    }, {
    .code = MEDIA_BUS_FMT_SBGGR12_1X12,
    .datatype = MIPI_CSI2_DT_RAW12,
    .procmode = 0x02,
    }, {
    .code = MEDIA_BUS_FMT_SGBRG12_1X12,
    .datatype = MIPI_CSI2_DT_RAW12,
    .procmode = 0x02,
    }, {
    .code = MEDIA_BUS_FMT_SGRBG12_1X12,
    .datatype = MIPI_CSI2_DT_RAW12,
    .procmode = 0x02,
    }, {
    .code = MEDIA_BUS_FMT_SRGGB12_1X12,
    .datatype = MIPI_CSI2_DT_RAW12,
    .procmode = 0x02,
    },
    };
    static const struct rcar_isp_format *risp_code_to_fmt(unsigned int code)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(rcar_isp_formats); i++) {
    if (rcar_isp_formats[i].code == code)
    return &rcar_isp_formats[i];
    }
    return core::ptr::null_mut();
    }
    enum rcar_isp_input {
    RISP_CSI_INPUT0,
    RISP_CSI_INPUT1,
    };
    enum rcar_isp_pads {
    RCAR_ISP_SINK,
    RCAR_ISP_PORT0,
    RCAR_ISP_PORT1,
    RCAR_ISP_PORT2,
    RCAR_ISP_PORT3,
    RCAR_ISP_PORT4,
    RCAR_ISP_PORT5,
    RCAR_ISP_PORT6,
    RCAR_ISP_PORT7,
    RCAR_ISP_NUM_PADS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_isp {
    pub dev: *mut device,
    pub csbase: *mut void __iomem,
    pub rstc: *mut reset_control,
    pub core: rcar_isp_core,
    pub csi_input: enum rcar_isp_input,
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; RCAR_ISP_NUM_PADS],
    pub notifier: v4l2_async_notifier,
    pub remote: *mut v4l2_subdev,
    pub remote_pad: c_uint,
    pub stream_count: c_int,
}

    static inline struct rcar_isp *sd_to_isp(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct rcar_isp, subdev);
    }
    static inline struct rcar_isp *notifier_to_isp(struct v4l2_async_notifier *n)
    {
    return container_of(n, struct rcar_isp, notifier);
    }
#[no_mangle]
unsafe extern "C" fn risp_write_cs(isp: *mut rcar_isp, offset: u32, value: u32) {
    static void risp_write_cs(struct rcar_isp *isp, u32 offset, u32 value)
    {
    iowrite32(value, isp.csbase + offset);
    }
#[no_mangle]
unsafe extern "C" fn risp_read_cs(isp: *mut rcar_isp, offset: u32) -> u32 {
    static u32 risp_read_cs(struct rcar_isp *isp, u32 offset)
    {
    return ioread32(isp.csbase + offset);
    }
#[no_mangle]
unsafe extern "C" fn risp_power_on(isp: *mut rcar_isp) -> c_int {
    static int risp_power_on(struct rcar_isp *isp)
    {
    int ret;
    ret = pm_runtime_resume_and_get(isp.dev);
    if (ret < 0)
    return ret;
    ret = reset_control_deassert(isp.rstc);
    if (ret < 0) {
    pm_runtime_put(isp.dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn risp_power_off(isp: *mut rcar_isp) {
    static void risp_power_off(struct rcar_isp *isp)
    {
    reset_control_assert(isp.rstc);
    pm_runtime_put(isp.dev);
    }
#[no_mangle]
unsafe extern "C" fn risp_start(isp: *mut rcar_isp, state: *mut v4l2_subdev_state) -> c_int {
    static int risp_start(struct rcar_isp *isp, struct v4l2_subdev_state *state)
    {
    const struct v4l2_mbus_framefmt *fmt;
    const struct rcar_isp_format *format;
    unsigned int vc;
    let mut sel_csi: u32 = 0;
    int ret;
    fmt = v4l2_subdev_state_get_format(state, RCAR_ISP_SINK);
    if (!fmt)
    return -EINVAL;
    format = risp_code_to_fmt(fmt.code);
    if (!format) {
    dev_err(isp.dev, "Unsupported bus format\n");
    return -EINVAL;
    }
    ret = risp_power_on(isp);
    if (ret) {
    dev_err(isp.dev, "Failed to power on ISP\n");
    return ret;
    }
// Select CSI-2 input source.
    if (isp.csi_input == RISP_CSI_INPUT1)
    sel_csi = ISPINPUTSEL0_SEL_CSI0;
    risp_write_cs(isp, ISPINPUTSEL0_REG,
    risp_read_cs(isp, ISPINPUTSEL0_REG) | sel_csi);
// Configure Channel Selector.
    for (vc = 0; vc < 4; vc++) {
    let mut ch: u8 = vc + 4;
    let mut dt: u8 = format.datatype;
    risp_write_cs(isp, ISPCS_FILTER_ID_CH_REG(ch), BIT(vc));
    risp_write_cs(isp, ISPCS_DT_CODE03_CH_REG(ch),
    ISPCS_DT_CODE03_EN3 | ISPCS_DT_CODE03_DT3(dt) |
    ISPCS_DT_CODE03_EN2 | ISPCS_DT_CODE03_DT2(dt) |
    ISPCS_DT_CODE03_EN1 | ISPCS_DT_CODE03_DT1(dt) |
    ISPCS_DT_CODE03_EN0 | ISPCS_DT_CODE03_DT0(dt));
    }
// Setup processing method.
    risp_write_cs(isp, ISPPROCMODE_DT_REG(format.datatype),
    ISPPROCMODE_DT_PROC_MODE_VCn(3, format.procmode) |
    ISPPROCMODE_DT_PROC_MODE_VCn(2, format.procmode) |
    ISPPROCMODE_DT_PROC_MODE_VCn(1, format.procmode) |
    ISPPROCMODE_DT_PROC_MODE_VCn(0, format.procmode));
// Start ISP.
    risp_write_cs(isp, ISPSTART_REG, ISPSTART_START);
    ret = v4l2_subdev_enable_streams(isp.remote, isp.remote_pad,
    BIT_ULL(0));
    if (ret)
    risp_power_off(isp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn risp_stop(isp: *mut rcar_isp) {
    static void risp_stop(struct rcar_isp *isp)
    {
    v4l2_subdev_disable_streams(isp.remote, isp.remote_pad, BIT_ULL(0));
// Stop ISP.
    risp_write_cs(isp, ISPSTART_REG, ISPSTART_STOP);
    risp_power_off(isp);
    }
    static int risp_enable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state, u32 source_pad,
    u64 source_streams_mask)
    {
    struct rcar_isp *isp = sd_to_isp(sd);
    let mut ret: c_int = 0;
    if (source_streams_mask != 1)
    return -EINVAL;
    if (!isp.remote)
    return -ENODEV;
    if (isp.stream_count == 0) {
    ret = risp_start(isp, state);
    if (ret)
    return ret;
    }
    isp.stream_count += 1;
    return ret;
    }
    static int risp_disable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state, u32 source_pad,
    u64 source_streams_mask)
    {
    struct rcar_isp *isp = sd_to_isp(sd);
    if (source_streams_mask != 1)
    return -EINVAL;
    if (!isp.remote)
    return -ENODEV;
    if (isp.stream_count == 1)
    risp_stop(isp);
    isp.stream_count -= 1;
    return 0;
    }
    static int risp_set_pad_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *framefmt;
    if (format.pad > RCAR_ISP_SINK)
    return v4l2_subdev_get_fmt(sd, state, format);
    if (!risp_code_to_fmt(format.format.code))
    format.format.code = rcar_isp_formats[0].code;
    for (unsigned int i = 0; i < RCAR_ISP_NUM_PADS; i++) {
    framefmt = v4l2_subdev_state_get_format(state, i);
// framefmt = format->format;
    }
    return 0;
    }
    static const struct v4l2_subdev_pad_ops risp_pad_ops = {
    .enable_streams = risp_enable_streams,
    .disable_streams = risp_disable_streams,
    .set_fmt = risp_set_pad_format,
    .get_fmt = v4l2_subdev_get_fmt,
    .link_validate = v4l2_subdev_link_validate_default,
    };
    static const struct v4l2_subdev_ops rcar_isp_subdev_ops = {
    .pad	= &risp_pad_ops,
    };
// -----------------------------------------------------------------------------
// Async handling and registration of subdevices and links
//
    static int risp_notify_bound(struct v4l2_async_notifier *notifier,
    struct v4l2_subdev *subdev,
    struct v4l2_async_connection *asd)
    {
    struct rcar_isp *isp = notifier_to_isp(notifier);
    int pad;
    pad = media_entity_get_fwnode_pad(&subdev.entity, asd.match.fwnode,
    MEDIA_PAD_FL_SOURCE);
    if (pad < 0) {
    dev_err(isp.dev, "Failed to find pad for %s\n", subdev.name);
    return pad;
    }
    isp.remote = subdev;
    isp.remote_pad = pad;
    dev_dbg(isp.dev, "Bound %s pad: %d\n", subdev.name, pad);
    return media_create_pad_link(&subdev.entity, pad,
    &isp.subdev.entity, 0,
    MEDIA_LNK_FL_ENABLED |
    MEDIA_LNK_FL_IMMUTABLE);
    }
    static void risp_notify_unbind(struct v4l2_async_notifier *notifier,
    struct v4l2_subdev *subdev,
    struct v4l2_async_connection *asd)
    {
    struct rcar_isp *isp = notifier_to_isp(notifier);
    isp.remote = core::ptr::null_mut();
    dev_dbg(isp.dev, "Unbind %s\n", subdev.name);
    }
    static const struct v4l2_async_notifier_operations risp_notify_ops = {
    .bound = risp_notify_bound,
    .unbind = risp_notify_unbind,
    };
#[no_mangle]
unsafe extern "C" fn risp_parse_dt(isp: *mut rcar_isp) -> c_int {
    static int risp_parse_dt(struct rcar_isp *isp)
    {
    struct v4l2_async_connection *asd;
    struct fwnode_handle *fwnode;
    struct fwnode_handle *ep;
    unsigned int id;
    int ret;
    for (id = 0; id < 2; id++) {
    ep = fwnode_graph_get_endpoint_by_id(dev_fwnode(isp.dev),
    0, id, 0);
    if (ep)
    break;
    }
    if (!ep) {
    dev_err(isp.dev, "Not connected to subdevice\n");
    return -EINVAL;
    }
    if (id == 1)
    isp.csi_input = RISP_CSI_INPUT1;
    fwnode = fwnode_graph_get_remote_endpoint(ep);
    fwnode_handle_put(ep);
    dev_dbg(isp.dev, "Found '%pOF'\n", to_of_node(fwnode));
    v4l2_async_subdev_nf_init(&isp.notifier, &isp.subdev);
    isp.notifier.ops = &risp_notify_ops;
    asd = v4l2_async_nf_add_fwnode(&isp.notifier, fwnode,
    struct v4l2_async_connection);
    fwnode_handle_put(fwnode);
    if (IS_ERR(asd))
    return PTR_ERR(asd);
    ret = v4l2_async_nf_register(&isp.notifier);
    if (ret)
    v4l2_async_nf_cleanup(&isp.notifier);
    return ret;
    }
// -----------------------------------------------------------------------------
// ISP Core connection
//
#[no_mangle]
unsafe extern "C" fn risp_cs_registered(sd: *mut v4l2_subdev) -> c_int {
    static int risp_cs_registered(struct v4l2_subdev *sd)
    {
    struct rcar_isp *isp = sd_to_isp(sd);
    return risp_core_registered(&isp.core, sd);
    }
    static const struct v4l2_subdev_internal_ops risp_cs_internal_ops = {
    .registered = risp_cs_registered,
    };
// -----------------------------------------------------------------------------
// Platform Device Driver
//
    static const struct media_entity_operations risp_entity_ops = {
    .link_validate = v4l2_subdev_link_validate,
    };
    static int risp_probe_resources(struct rcar_isp *isp,
    struct platform_device *pdev)
    {
    struct resource *res;
//
// For backward compatibility allow cs base to be the only reg if no
// reg-names are set in DT.
//
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "cs");
    if (!res)
    isp.csbase = devm_platform_ioremap_resource(pdev, 0);
    else
    isp.csbase = devm_ioremap_resource(&pdev.dev, res);
    if (IS_ERR(isp.csbase))
    return PTR_ERR(isp.csbase);
    isp.rstc = devm_reset_control_get_shared(&pdev.dev, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(isp.rstc);
    }
    static const struct of_device_id risp_of_id_table[] = {
    { .compatible = "renesas,r8a779a0-isp" },
    { .compatible = "renesas,r8a779g0-isp" },
// Keep above for compatibility with old DTB files.
    { .compatible = "renesas,rcar-gen4-isp" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, risp_of_id_table);
#[no_mangle]
unsafe extern "C" fn risp_probe(pdev: *mut platform_device) -> c_int {
    static int risp_probe(struct platform_device *pdev)
    {
    struct rcar_isp *isp;
    unsigned int i;
    int ret;
    isp = devm_kzalloc(&pdev.dev, sizeof(*isp), GFP_KERNEL);
    if (!isp)
    return -ENOMEM;
    isp.dev = &pdev.dev;
    ret = risp_probe_resources(isp, pdev);
    if (ret) {
    dev_err(isp.dev, "Failed to get resources\n");
    return ret;
    }
    platform_set_drvdata(pdev, isp);
    pm_runtime_enable(&pdev.dev);
    ret = risp_parse_dt(isp);
    if (ret)
    goto error_pm;
    isp.subdev.owner = THIS_MODULE;
    isp.subdev.dev = &pdev.dev;
    v4l2_subdev_init(&isp.subdev, &rcar_isp_subdev_ops);
    v4l2_set_subdevdata(&isp.subdev, &pdev.dev);
    snprintf(isp.subdev.name, sizeof(isp.subdev.name), "%s %s",
    KBUILD_MODNAME, dev_name(&pdev.dev));
    isp.subdev.flags = V4L2_SUBDEV_FL_HAS_DEVNODE;
    isp.subdev.entity.function = MEDIA_ENT_F_VID_MUX;
    isp.subdev.entity.ops = &risp_entity_ops;
    isp.pads[RCAR_ISP_SINK].flags = MEDIA_PAD_FL_SINK;
    for (i = RCAR_ISP_PORT0; i < RCAR_ISP_NUM_PADS; i++)
    isp.pads[i].flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&isp.subdev.entity, RCAR_ISP_NUM_PADS,
    isp.pads);
    if (ret)
    goto error_notifier;
    ret = v4l2_subdev_init_finalize(&isp.subdev);
    if (ret)
    goto error_notifier;
    ret = risp_core_probe(&isp.core, pdev, isp.csbase, isp.rstc);
    switch (ret) {
    case 0:
// The device have an ISP core.
    isp.subdev.internal_ops = &risp_cs_internal_ops;
    break;
    case -ENODEV:
// The device don't have an ISP core, that is OK.
    ret = 0;
    break;
    default:
// Something went wrong registering the ISP core.
    goto error_subdev;
    }
    ret = v4l2_async_register_subdev(&isp.subdev);
    if (ret < 0)
    goto error_core;
    dev_info(isp.dev, "Using CSI-2 input: %u\n", isp.csi_input);
    return 0;
    error_core:
    risp_core_remove(&isp.core);
    error_subdev:
    v4l2_subdev_cleanup(&isp.subdev);
    error_notifier:
    v4l2_async_nf_unregister(&isp.notifier);
    v4l2_async_nf_cleanup(&isp.notifier);
    error_pm:
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn risp_remove(pdev: *mut platform_device) {
    static void risp_remove(struct platform_device *pdev)
    {
    struct rcar_isp *isp = platform_get_drvdata(pdev);
    risp_core_remove(&isp.core);
    v4l2_async_nf_unregister(&isp.notifier);
    v4l2_async_nf_cleanup(&isp.notifier);
    v4l2_async_unregister_subdev(&isp.subdev);
    v4l2_subdev_cleanup(&isp.subdev);
    pm_runtime_disable(&pdev.dev);
    }
    static struct platform_driver rcar_isp_driver = {
    .driver = {
    .name = "rcar-isp",
    .suppress_bind_attrs = true,
    .of_match_table = risp_of_id_table,
    },
    .probe = risp_probe,
    .remove = risp_remove,
    };
    module_platform_driver(rcar_isp_driver);
    MODULE_AUTHOR("Niklas Söderlund <niklas.soderlund@ragnatech.se>");
    MODULE_DESCRIPTION("Renesas R-Car ISP Channel Selector driver");
    MODULE_LICENSE("GPL");
