//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/exynos/exynos_drm_scaler.c
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
// Copyright (C) 2017 Samsung Electronics Co.Ltd
// Author:
// Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

pub const SCALER_MAX_CLK: c_int = 4;
pub const SCALER_AUTOSUSPEND_DELAY: c_int = 2000;
pub const SCALER_RESET_WAIT_RETRIES: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scaler_data {
    pub clk_name: [*const c_char; SCALER_MAX_CLK],
    pub num_clk: c_uint,
    pub formats: *const exynos_drm_ipp_formats,
    pub num_formats: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scaler_context {
    pub ipp: exynos_drm_ipp,
    pub drm_dev: *mut drm_device,
    pub dma_priv: *mut c_void,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub clock: [*mut clk; SCALER_MAX_CLK],
    pub task: *mut exynos_drm_ipp_task,
    pub scaler_data: *const scaler_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scaler_format {
    pub drm_fmt: u32,
    pub internal_fmt: u32,
    pub chroma_tile_w: u32,
    pub chroma_tile_h: u32,
}

    static const struct scaler_format scaler_formats[] = {
    { DRM_FORMAT_NV12, SCALER_YUV420_2P_UV, 8, 8 },
    { DRM_FORMAT_NV21, SCALER_YUV420_2P_VU, 8, 8 },
    { DRM_FORMAT_YUV420, SCALER_YUV420_3P, 8, 8 },
    { DRM_FORMAT_YUYV, SCALER_YUV422_1P_YUYV, 16, 16 },
    { DRM_FORMAT_UYVY, SCALER_YUV422_1P_UYVY, 16, 16 },
    { DRM_FORMAT_YVYU, SCALER_YUV422_1P_YVYU, 16, 16 },
    { DRM_FORMAT_NV16, SCALER_YUV422_2P_UV, 8, 16 },
    { DRM_FORMAT_NV61, SCALER_YUV422_2P_VU, 8, 16 },
    { DRM_FORMAT_YUV422, SCALER_YUV422_3P, 8, 16 },
    { DRM_FORMAT_NV24, SCALER_YUV444_2P_UV, 16, 16 },
    { DRM_FORMAT_NV42, SCALER_YUV444_2P_VU, 16, 16 },
    { DRM_FORMAT_YUV444, SCALER_YUV444_3P, 16, 16 },
    { DRM_FORMAT_RGB565, SCALER_RGB_565, 0, 0 },
    { DRM_FORMAT_XRGB1555, SCALER_ARGB1555, 0, 0 },
    { DRM_FORMAT_ARGB1555, SCALER_ARGB1555, 0, 0 },
    { DRM_FORMAT_XRGB4444, SCALER_ARGB4444, 0, 0 },
    { DRM_FORMAT_ARGB4444, SCALER_ARGB4444, 0, 0 },
    { DRM_FORMAT_XRGB8888, SCALER_ARGB8888, 0, 0 },
    { DRM_FORMAT_ARGB8888, SCALER_ARGB8888, 0, 0 },
    { DRM_FORMAT_RGBX8888, SCALER_RGBA8888, 0, 0 },
    { DRM_FORMAT_RGBA8888, SCALER_RGBA8888, 0, 0 },
    };
    static const struct scaler_format *scaler_get_format(u32 drm_fmt)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(scaler_formats); i++)
    if (scaler_formats[i].drm_fmt == drm_fmt)
    return &scaler_formats[i];
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn scaler_reset(scaler: *mut scaler_context) -> c_int {
    static inline int scaler_reset(struct scaler_context *scaler)
    {
    let mut retry: c_int = SCALER_RESET_WAIT_RETRIES;
    scaler_write(SCALER_CFG_SOFT_RESET, SCALER_CFG);
    do {
    cpu_relax();
    } while (--retry > 1 &&
    scaler_read(SCALER_CFG) & SCALER_CFG_SOFT_RESET);
    do {
    cpu_relax();
    scaler_write(1, SCALER_INT_EN);
    } while (--retry > 0 && scaler_read(SCALER_INT_EN) != 1);
    return retry ? 0 : -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn scaler_enable_int(scaler: *mut scaler_context) {
    static inline void scaler_enable_int(struct scaler_context *scaler)
    {
    u32 val;
    val = SCALER_INT_EN_TIMEOUT |
    SCALER_INT_EN_ILLEGAL_BLEND |
    SCALER_INT_EN_ILLEGAL_RATIO |
    SCALER_INT_EN_ILLEGAL_DST_HEIGHT |
    SCALER_INT_EN_ILLEGAL_DST_WIDTH |
    SCALER_INT_EN_ILLEGAL_DST_V_POS |
    SCALER_INT_EN_ILLEGAL_DST_H_POS |
    SCALER_INT_EN_ILLEGAL_DST_C_SPAN |
    SCALER_INT_EN_ILLEGAL_DST_Y_SPAN |
    SCALER_INT_EN_ILLEGAL_DST_CR_BASE |
    SCALER_INT_EN_ILLEGAL_DST_CB_BASE |
    SCALER_INT_EN_ILLEGAL_DST_Y_BASE |
    SCALER_INT_EN_ILLEGAL_DST_COLOR |
    SCALER_INT_EN_ILLEGAL_SRC_HEIGHT |
    SCALER_INT_EN_ILLEGAL_SRC_WIDTH |
    SCALER_INT_EN_ILLEGAL_SRC_CV_POS |
    SCALER_INT_EN_ILLEGAL_SRC_CH_POS |
    SCALER_INT_EN_ILLEGAL_SRC_YV_POS |
    SCALER_INT_EN_ILLEGAL_SRC_YH_POS |
    SCALER_INT_EN_ILLEGAL_DST_SPAN |
    SCALER_INT_EN_ILLEGAL_SRC_Y_SPAN |
    SCALER_INT_EN_ILLEGAL_SRC_CR_BASE |
    SCALER_INT_EN_ILLEGAL_SRC_CB_BASE |
    SCALER_INT_EN_ILLEGAL_SRC_Y_BASE |
    SCALER_INT_EN_ILLEGAL_SRC_COLOR |
    SCALER_INT_EN_FRAME_END;
    scaler_write(val, SCALER_INT_EN);
    }
    static inline void scaler_set_src_fmt(struct scaler_context *scaler,
    u32 src_fmt, u32 tile)
    {
    u32 val;
    val = SCALER_SRC_CFG_SET_COLOR_FORMAT(src_fmt) | (tile << 10);
    scaler_write(val, SCALER_SRC_CFG);
    }
    static inline void scaler_set_src_base(struct scaler_context *scaler,
    struct exynos_drm_ipp_buffer *src_buf)
    {
    static unsigned int bases[] = {
    SCALER_SRC_Y_BASE,
    SCALER_SRC_CB_BASE,
    SCALER_SRC_CR_BASE,
    };
    int i;
    for (i = 0; i < src_buf.format.num_planes; ++i)
    scaler_write(src_buf.dma_addr[i], bases[i]);
    }
    static inline void scaler_set_src_span(struct scaler_context *scaler,
    struct exynos_drm_ipp_buffer *src_buf)
    {
    u32 val;
    val = SCALER_SRC_SPAN_SET_Y_SPAN(src_buf.buf.pitch[0] /
    src_buf.format.cpp[0]);
    if (src_buf.format.num_planes > 1)
    val |= SCALER_SRC_SPAN_SET_C_SPAN(src_buf.buf.pitch[1]);
    scaler_write(val, SCALER_SRC_SPAN);
    }
    static inline void scaler_set_src_luma_chroma_pos(struct scaler_context *scaler,
    struct drm_exynos_ipp_task_rect *src_pos,
    const struct scaler_format *fmt)
    {
    u32 val;
    val = SCALER_SRC_Y_POS_SET_YH_POS(src_pos.x << 2);
    val |=  SCALER_SRC_Y_POS_SET_YV_POS(src_pos.y << 2);
    scaler_write(val, SCALER_SRC_Y_POS);
    val = SCALER_SRC_C_POS_SET_CH_POS(
    (src_pos.x * fmt.chroma_tile_w / 16) << 2);
    val |=  SCALER_SRC_C_POS_SET_CV_POS(
    (src_pos.y * fmt.chroma_tile_h / 16) << 2);
    scaler_write(val, SCALER_SRC_C_POS);
    }
    static inline void scaler_set_src_wh(struct scaler_context *scaler,
    struct drm_exynos_ipp_task_rect *src_pos)
    {
    u32 val;
    val = SCALER_SRC_WH_SET_WIDTH(src_pos.w);
    val |= SCALER_SRC_WH_SET_HEIGHT(src_pos.h);
    scaler_write(val, SCALER_SRC_WH);
    }
    static inline void scaler_set_dst_fmt(struct scaler_context *scaler,
    u32 dst_fmt)
    {
    u32 val;
    val = SCALER_DST_CFG_SET_COLOR_FORMAT(dst_fmt);
    scaler_write(val, SCALER_DST_CFG);
    }
    static inline void scaler_set_dst_base(struct scaler_context *scaler,
    struct exynos_drm_ipp_buffer *dst_buf)
    {
    static unsigned int bases[] = {
    SCALER_DST_Y_BASE,
    SCALER_DST_CB_BASE,
    SCALER_DST_CR_BASE,
    };
    int i;
    for (i = 0; i < dst_buf.format.num_planes; ++i)
    scaler_write(dst_buf.dma_addr[i], bases[i]);
    }
    static inline void scaler_set_dst_span(struct scaler_context *scaler,
    struct exynos_drm_ipp_buffer *dst_buf)
    {
    u32 val;
    val = SCALER_DST_SPAN_SET_Y_SPAN(dst_buf.buf.pitch[0] /
    dst_buf.format.cpp[0]);
    if (dst_buf.format.num_planes > 1)
    val |= SCALER_DST_SPAN_SET_C_SPAN(dst_buf.buf.pitch[1]);
    scaler_write(val, SCALER_DST_SPAN);
    }
    static inline void scaler_set_dst_luma_pos(struct scaler_context *scaler,
    struct drm_exynos_ipp_task_rect *dst_pos)
    {
    u32 val;
    val = SCALER_DST_WH_SET_WIDTH(dst_pos.w);
    val |= SCALER_DST_WH_SET_HEIGHT(dst_pos.h);
    scaler_write(val, SCALER_DST_WH);
    }
    static inline void scaler_set_dst_wh(struct scaler_context *scaler,
    struct drm_exynos_ipp_task_rect *dst_pos)
    {
    u32 val;
    val = SCALER_DST_POS_SET_H_POS(dst_pos.x);
    val |= SCALER_DST_POS_SET_V_POS(dst_pos.y);
    scaler_write(val, SCALER_DST_POS);
    }
    static inline void scaler_set_hv_ratio(struct scaler_context *scaler,
    unsigned int rotation,
    struct drm_exynos_ipp_task_rect *src_pos,
    struct drm_exynos_ipp_task_rect *dst_pos)
    {
    u32 val, h_ratio, v_ratio;
    if (drm_rotation_90_or_270(rotation)) {
    h_ratio = (src_pos.h << 16) / dst_pos.w;
    v_ratio = (src_pos.w << 16) / dst_pos.h;
    } else {
    h_ratio = (src_pos.w << 16) / dst_pos.w;
    v_ratio = (src_pos.h << 16) / dst_pos.h;
    }
    val = SCALER_H_RATIO_SET(h_ratio);
    scaler_write(val, SCALER_H_RATIO);
    val = SCALER_V_RATIO_SET(v_ratio);
    scaler_write(val, SCALER_V_RATIO);
    }
    static inline void scaler_set_rotation(struct scaler_context *scaler,
    unsigned int rotation)
    {
    let mut val: u32 = 0;
    if (rotation & DRM_MODE_ROTATE_90)
    val |= SCALER_ROT_CFG_SET_ROTMODE(SCALER_ROT_MODE_90);
#[no_mangle]
pub unsafe extern "C" fn if(DRM_MODE_ROTATE_180: rotation &) -> else {
    else if (rotation & DRM_MODE_ROTATE_180)
    val |= SCALER_ROT_CFG_SET_ROTMODE(SCALER_ROT_MODE_180);
#[no_mangle]
pub unsafe extern "C" fn if(DRM_MODE_ROTATE_270: rotation &) -> else {
    else if (rotation & DRM_MODE_ROTATE_270)
    val |= SCALER_ROT_CFG_SET_ROTMODE(SCALER_ROT_MODE_270);
    if (rotation & DRM_MODE_REFLECT_X)
    val |= SCALER_ROT_CFG_FLIP_X_EN;
    if (rotation & DRM_MODE_REFLECT_Y)
    val |= SCALER_ROT_CFG_FLIP_Y_EN;
    scaler_write(val, SCALER_ROT_CFG);
    }
    static inline void scaler_set_csc(struct scaler_context *scaler,
    const struct drm_format_info *fmt)
    {
    static const u32 csc_mtx[2][3][3] = {
    { /* YCbCr to RGB */
    {0x254, 0x000, 0x331},
    {0x254, 0xf38, 0xe60},
    {0x254, 0x409, 0x000},
    },
    { /* RGB to YCbCr */
    {0x084, 0x102, 0x032},
    {0xfb4, 0xf6b, 0x0e1},
    {0x0e1, 0xf44, 0xfdc},
    },
    };
    int i, j, dir;
    switch (fmt.format) {
    case DRM_FORMAT_RGB565:
    case DRM_FORMAT_XRGB1555:
    case DRM_FORMAT_ARGB1555:
    case DRM_FORMAT_XRGB4444:
    case DRM_FORMAT_ARGB4444:
    case DRM_FORMAT_XRGB8888:
    case DRM_FORMAT_ARGB8888:
    case DRM_FORMAT_RGBX8888:
    case DRM_FORMAT_RGBA8888:
    dir = 1;
    break;
    default:
    dir = 0;
    }
    for (i = 0; i < 3; i++)
    for (j = 0; j < 3; j++)
    scaler_write(csc_mtx[dir][i][j], SCALER_CSC_COEF(j, i));
    }
    static inline void scaler_set_timer(struct scaler_context *scaler,
    unsigned int timer, unsigned int divider)
    {
    u32 val;
    val = SCALER_TIMEOUT_CTRL_TIMER_ENABLE;
    val |= SCALER_TIMEOUT_CTRL_SET_TIMER_VALUE(timer);
    val |= SCALER_TIMEOUT_CTRL_SET_TIMER_DIV(divider);
    scaler_write(val, SCALER_TIMEOUT_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn scaler_start_hw(scaler: *mut scaler_context) {
    static inline void scaler_start_hw(struct scaler_context *scaler)
    {
    scaler_write(SCALER_CFG_START_CMD, SCALER_CFG);
    }
    static int scaler_commit(struct exynos_drm_ipp *ipp,
    struct exynos_drm_ipp_task *task)
    {
    struct scaler_context *scaler =
    container_of(ipp, struct scaler_context, ipp);
    struct drm_exynos_ipp_task_rect *src_pos = &task.src.rect;
    struct drm_exynos_ipp_task_rect *dst_pos = &task.dst.rect;
    const struct scaler_format *src_fmt, *dst_fmt;
    let mut ret: c_int = 0;
    src_fmt = scaler_get_format(task.src.buf.fourcc);
    dst_fmt = scaler_get_format(task.dst.buf.fourcc);
    ret = pm_runtime_resume_and_get(scaler.dev);
    if (ret < 0)
    return ret;
    if (scaler_reset(scaler))
    return -EIO;
    scaler.task = task;
    scaler_set_src_fmt(
    scaler, src_fmt.internal_fmt, task.src.buf.modifier != 0);
    scaler_set_src_base(scaler, &task.src);
    scaler_set_src_span(scaler, &task.src);
    scaler_set_src_luma_chroma_pos(scaler, src_pos, src_fmt);
    scaler_set_src_wh(scaler, src_pos);
    scaler_set_dst_fmt(scaler, dst_fmt.internal_fmt);
    scaler_set_dst_base(scaler, &task.dst);
    scaler_set_dst_span(scaler, &task.dst);
    scaler_set_dst_luma_pos(scaler, dst_pos);
    scaler_set_dst_wh(scaler, dst_pos);
    scaler_set_hv_ratio(scaler, task.transform.rotation, src_pos, dst_pos);
    scaler_set_rotation(scaler, task.transform.rotation);
    scaler_set_csc(scaler, task.src.format);
    scaler_set_timer(scaler, 0xffff, 0xf);
    scaler_enable_int(scaler);
    scaler_start_hw(scaler);
    return 0;
    }
    static const struct exynos_drm_ipp_funcs ipp_funcs = {
    .commit = scaler_commit,
    };
#[no_mangle]
pub unsafe extern "C" fn scaler_disable_int(scaler: *mut scaler_context) {
    static inline void scaler_disable_int(struct scaler_context *scaler)
    {
    scaler_write(0, SCALER_INT_EN);
    }
#[no_mangle]
pub unsafe extern "C" fn scaler_get_int_status(scaler: *mut scaler_context) -> u32 {
    static inline u32 scaler_get_int_status(struct scaler_context *scaler)
    {
    let mut val: u32 = scaler_read(SCALER_INT_STATUS);
    scaler_write(val, SCALER_INT_STATUS);
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn scaler_task_done(val: u32) -> c_int {
    static inline int scaler_task_done(u32 val)
    {
    return val & SCALER_INT_STATUS_FRAME_END ? 0 : -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn scaler_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t scaler_irq_handler(int irq, void *arg)
    {
    struct scaler_context *scaler = arg;
    let mut val: u32 = scaler_get_int_status(scaler);
    scaler_disable_int(scaler);
    if (scaler.task) {
    struct exynos_drm_ipp_task *task = scaler.task;
    scaler.task = core::ptr::null_mut();
    pm_runtime_mark_last_busy(scaler.dev);
    pm_runtime_put_autosuspend(scaler.dev);
    exynos_drm_ipp_task_done(task, scaler_task_done(val));
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn scaler_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int scaler_bind(struct device *dev, struct device *master, void *data)
    {
    struct scaler_context *scaler = dev_get_drvdata(dev);
    struct drm_device *drm_dev = data;
    struct exynos_drm_ipp *ipp = &scaler.ipp;
    scaler.drm_dev = drm_dev;
    ipp.drm_dev = drm_dev;
    exynos_drm_register_dma(drm_dev, dev, &scaler.dma_priv);
    exynos_drm_ipp_register(dev, ipp, &ipp_funcs,
    DRM_EXYNOS_IPP_CAP_CROP | DRM_EXYNOS_IPP_CAP_ROTATE |
    DRM_EXYNOS_IPP_CAP_SCALE | DRM_EXYNOS_IPP_CAP_CONVERT,
    scaler.scaler_data.formats,
    scaler.scaler_data.num_formats, "scaler");
    dev_info(dev, "The exynos scaler has been probed successfully\n");
    return 0;
    }
    static void scaler_unbind(struct device *dev, struct device *master,
    void *data)
    {
    struct scaler_context *scaler = dev_get_drvdata(dev);
    struct exynos_drm_ipp *ipp = &scaler.ipp;
    exynos_drm_ipp_unregister(dev, ipp);
    exynos_drm_unregister_dma(scaler.drm_dev, scaler.dev,
    &scaler.dma_priv);
    }
    static const struct component_ops scaler_component_ops = {
    .bind	= scaler_bind,
    .unbind = scaler_unbind,
    };
#[no_mangle]
unsafe extern "C" fn scaler_probe(pdev: *mut platform_device) -> c_int {
    static int scaler_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct scaler_context *scaler;
    int irq;
    int ret, i;
    scaler = devm_kzalloc(dev, sizeof(*scaler), GFP_KERNEL);
    if (!scaler)
    return -ENOMEM;
    scaler.scaler_data =
    (struct scaler_data *)of_device_get_match_data(dev);
    scaler.dev = dev;
    scaler.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(scaler.regs))
    return PTR_ERR(scaler.regs);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),	scaler_irq_handler,
    IRQF_ONESHOT, "drm_scaler", scaler);
    if (ret < 0) {
    dev_err(dev, "failed to request irq\n");
    return ret;
    }
    for (i = 0; i < scaler.scaler_data.num_clk; ++i) {
    scaler.clock[i] = devm_clk_get(dev,
    scaler.scaler_data.clk_name[i]);
    if (IS_ERR(scaler.clock[i])) {
    dev_err(dev, "failed to get clock\n");
    return PTR_ERR(scaler.clock[i]);
    }
    }
    pm_runtime_use_autosuspend(dev);
    pm_runtime_set_autosuspend_delay(dev, SCALER_AUTOSUSPEND_DELAY);
    pm_runtime_enable(dev);
    platform_set_drvdata(pdev, scaler);
    ret = component_add(dev, &scaler_component_ops);
    if (ret)
    goto err_ippdrv_register;
    return 0;
    err_ippdrv_register:
    pm_runtime_dont_use_autosuspend(dev);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scaler_remove(pdev: *mut platform_device) {
    static void scaler_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    component_del(dev, &scaler_component_ops);
    pm_runtime_dont_use_autosuspend(dev);
    pm_runtime_disable(dev);
    }
#[no_mangle]
unsafe extern "C" fn clk_disable_unprepare_wrapper(clk: *mut clk) -> c_int {
    static int clk_disable_unprepare_wrapper(struct clk *clk)
    {
    clk_disable_unprepare(clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scaler_clk_ctrl(scaler: *mut scaler_context, enable: bool) -> c_int {
    static int scaler_clk_ctrl(struct scaler_context *scaler, bool enable)
    {
    int (*clk_fun)(struct clk *clk), i;
    clk_fun = enable ? clk_prepare_enable : clk_disable_unprepare_wrapper;
    for (i = 0; i < scaler.scaler_data.num_clk; ++i)
    clk_fun(scaler.clock[i]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scaler_runtime_suspend(dev: *mut device) -> c_int {
    static int scaler_runtime_suspend(struct device *dev)
    {
    struct scaler_context *scaler = dev_get_drvdata(dev);
    return  scaler_clk_ctrl(scaler, false);
    }
#[no_mangle]
unsafe extern "C" fn scaler_runtime_resume(dev: *mut device) -> c_int {
    static int scaler_runtime_resume(struct device *dev)
    {
    struct scaler_context *scaler = dev_get_drvdata(dev);
    return  scaler_clk_ctrl(scaler, true);
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(scaler_pm_ops, scaler_runtime_suspend,
    scaler_runtime_resume, core::ptr::null_mut());
    static const struct drm_exynos_ipp_limit scaler_5420_two_pixel_hv_limits[] = {
    { IPP_SIZE_LIMIT(BUFFER, .h = { 16, SZ_8K }, .v = { 16, SZ_8K }) },
    { IPP_SIZE_LIMIT(AREA, .h.align = 2, .v.align = 2) },
    { IPP_SCALE_LIMIT(.h = { 65536 * 1 / 4, 65536 * 16 },
    .v = { 65536 * 1 / 4, 65536 * 16 }) },
    };
    static const struct drm_exynos_ipp_limit scaler_5420_two_pixel_h_limits[] = {
    { IPP_SIZE_LIMIT(BUFFER, .h = { 16, SZ_8K }, .v = { 16, SZ_8K }) },
    { IPP_SIZE_LIMIT(AREA, .h.align = 2, .v.align = 1) },
    { IPP_SCALE_LIMIT(.h = { 65536 * 1 / 4, 65536 * 16 },
    .v = { 65536 * 1 / 4, 65536 * 16 }) },
    };
    static const struct drm_exynos_ipp_limit scaler_5420_one_pixel_limits[] = {
    { IPP_SIZE_LIMIT(BUFFER, .h = { 16, SZ_8K }, .v = { 16, SZ_8K }) },
    { IPP_SCALE_LIMIT(.h = { 65536 * 1 / 4, 65536 * 16 },
    .v = { 65536 * 1 / 4, 65536 * 16 }) },
    };
    static const struct drm_exynos_ipp_limit scaler_5420_tile_limits[] = {
    { IPP_SIZE_LIMIT(BUFFER, .h = { 16, SZ_8K }, .v = { 16, SZ_8K })},
    { IPP_SIZE_LIMIT(AREA, .h.align = 16, .v.align = 16) },
    { IPP_SCALE_LIMIT(.h = {1, 1}, .v = {1, 1})},
    { }
    };

    IPP_SRCDST_MFORMAT(f, DRM_FORMAT_MOD_SAMSUNG_16_16_TILE, (l))
    static const struct exynos_drm_ipp_formats exynos5420_formats[] = {
// SCALER_YUV420_2P_UV
    { IPP_SRCDST_FORMAT(NV21, scaler_5420_two_pixel_hv_limits) },
// SCALER_YUV420_2P_VU
    { IPP_SRCDST_FORMAT(NV12, scaler_5420_two_pixel_hv_limits) },
// SCALER_YUV420_3P
    { IPP_SRCDST_FORMAT(YUV420, scaler_5420_two_pixel_hv_limits) },
// SCALER_YUV422_1P_YUYV
    { IPP_SRCDST_FORMAT(YUYV, scaler_5420_two_pixel_h_limits) },
// SCALER_YUV422_1P_UYVY
    { IPP_SRCDST_FORMAT(UYVY, scaler_5420_two_pixel_h_limits) },
// SCALER_YUV422_1P_YVYU
    { IPP_SRCDST_FORMAT(YVYU, scaler_5420_two_pixel_h_limits) },
// SCALER_YUV422_2P_UV
    { IPP_SRCDST_FORMAT(NV61, scaler_5420_two_pixel_h_limits) },
// SCALER_YUV422_2P_VU
    { IPP_SRCDST_FORMAT(NV16, scaler_5420_two_pixel_h_limits) },
// SCALER_YUV422_3P
    { IPP_SRCDST_FORMAT(YUV422, scaler_5420_two_pixel_h_limits) },
// SCALER_YUV444_2P_UV
    { IPP_SRCDST_FORMAT(NV42, scaler_5420_one_pixel_limits) },
// SCALER_YUV444_2P_VU
    { IPP_SRCDST_FORMAT(NV24, scaler_5420_one_pixel_limits) },
// SCALER_YUV444_3P
    { IPP_SRCDST_FORMAT(YUV444, scaler_5420_one_pixel_limits) },
// SCALER_RGB_565
    { IPP_SRCDST_FORMAT(RGB565, scaler_5420_one_pixel_limits) },
// SCALER_ARGB1555
    { IPP_SRCDST_FORMAT(XRGB1555, scaler_5420_one_pixel_limits) },
// SCALER_ARGB1555
    { IPP_SRCDST_FORMAT(ARGB1555, scaler_5420_one_pixel_limits) },
// SCALER_ARGB4444
    { IPP_SRCDST_FORMAT(XRGB4444, scaler_5420_one_pixel_limits) },
// SCALER_ARGB4444
    { IPP_SRCDST_FORMAT(ARGB4444, scaler_5420_one_pixel_limits) },
// SCALER_ARGB8888
    { IPP_SRCDST_FORMAT(XRGB8888, scaler_5420_one_pixel_limits) },
// SCALER_ARGB8888
    { IPP_SRCDST_FORMAT(ARGB8888, scaler_5420_one_pixel_limits) },
// SCALER_RGBA8888
    { IPP_SRCDST_FORMAT(RGBX8888, scaler_5420_one_pixel_limits) },
// SCALER_RGBA8888
    { IPP_SRCDST_FORMAT(RGBA8888, scaler_5420_one_pixel_limits) },
// SCALER_YUV420_2P_UV TILE
    { IPP_SRCDST_TILE_FORMAT(NV21, scaler_5420_tile_limits) },
// SCALER_YUV420_2P_VU TILE
    { IPP_SRCDST_TILE_FORMAT(NV12, scaler_5420_tile_limits) },
// SCALER_YUV420_3P TILE
    { IPP_SRCDST_TILE_FORMAT(YUV420, scaler_5420_tile_limits) },
// SCALER_YUV422_1P_YUYV TILE
    { IPP_SRCDST_TILE_FORMAT(YUYV, scaler_5420_tile_limits) },
    };
    static const struct scaler_data exynos5420_data = {
    .clk_name	= {"mscl"},
    .num_clk	= 1,
    .formats	= exynos5420_formats,
    .num_formats	= ARRAY_SIZE(exynos5420_formats),
    };
    static const struct scaler_data exynos5433_data = {
    .clk_name	= {"pclk", "aclk", "aclk_xiu"},
    .num_clk	= 3,
    .formats	= exynos5420_formats, /* intentional */
    .num_formats	= ARRAY_SIZE(exynos5420_formats),
    };
    static const struct of_device_id exynos_scaler_match[] = {
    {
    .compatible = "samsung,exynos5420-scaler",
    .data = &exynos5420_data,
    }, {
    .compatible = "samsung,exynos5433-scaler",
    .data = &exynos5433_data,
    }, {
    },
    };
    MODULE_DEVICE_TABLE(of, exynos_scaler_match);
    struct platform_driver scaler_driver = {
    .probe		= scaler_probe,
    .remove		= scaler_remove,
    .driver		= {
    .name	= "exynos-scaler",
    .pm	= pm_ptr(&scaler_pm_ops),
    .of_match_table = exynos_scaler_match,
    },
    };
