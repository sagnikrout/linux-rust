//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/lcdc/imx-lcdc.c
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
// SPDX-FileCopyrightText: 2020 Marian Cichy <M.Cichy@pengutronix.de>

pub const IMX21LCDC_LSSAR: c_uint = 0x0000 /* LCDC Screen Start Address Register */;
pub const IMX21LCDC_LSR: c_uint = 0x0004 /* LCDC Size Register */;
pub const IMX21LCDC_LVPWR: c_uint = 0x0008 /* LCDC Virtual Page Width Register */;
pub const IMX21LCDC_LCPR: c_uint = 0x000C /* LCDC Cursor Position Register */;
pub const IMX21LCDC_LCWHB: c_uint = 0x0010 /* LCDC Cursor Width Height and Blink Register*/;
pub const IMX21LCDC_LCCMR: c_uint = 0x0014 /* LCDC Color Cursor Mapping Register */;
pub const IMX21LCDC_LPCR: c_uint = 0x0018 /* LCDC Panel Configuration Register */;
pub const IMX21LCDC_LHCR: c_uint = 0x001C /* LCDC Horizontal Configuration Register */;
pub const IMX21LCDC_LVCR: c_uint = 0x0020 /* LCDC Vertical Configuration Register */;
pub const IMX21LCDC_LPOR: c_uint = 0x0024 /* LCDC Panning Offset Register */;
pub const IMX21LCDC_LSCR: c_uint = 0x0028 /* LCDC Sharp Configuration Register */;
pub const IMX21LCDC_LPCCR: c_uint = 0x002C /* LCDC PWM Contrast Control Register */;
pub const IMX21LCDC_LDCR: c_uint = 0x0030 /* LCDC DMA Control Register */;
pub const IMX21LCDC_LRMCR: c_uint = 0x0034 /* LCDC Refresh Mode Control Register */;
pub const IMX21LCDC_LICR: c_uint = 0x0038 /* LCDC Interrupt Configuration Register */;
pub const IMX21LCDC_LIER: c_uint = 0x003C /* LCDC Interrupt Enable Register */;
pub const IMX21LCDC_LISR: c_uint = 0x0040 /* LCDC Interrupt Status Register */;
pub const IMX21LCDC_LGWSAR: c_uint = 0x0050 /* LCDC Graphic Window Start Address Register */;
pub const IMX21LCDC_LGWSR: c_uint = 0x0054 /* LCDC Graph Window Size Register */;
pub const IMX21LCDC_LGWVPWR: c_uint = 0x0058 /* LCDC Graphic Window Virtual Page Width Register */;
pub const IMX21LCDC_LGWPOR: c_uint = 0x005C /* LCDC Graphic Window Panning Offset Register */;
pub const IMX21LCDC_LGWPR: c_uint = 0x0060 /* LCDC Graphic Window Position Register */;
pub const IMX21LCDC_LGWCR: c_uint = 0x0064 /* LCDC Graphic Window Control Register */;
pub const IMX21LCDC_LGWDCR: c_uint = 0x0068 /* LCDC Graphic Window DMA Control Register */;
pub const IMX21LCDC_LAUSCR: c_uint = 0x0080 /* LCDC AUS Mode Control Register */;
pub const IMX21LCDC_LAUSCCR: c_uint = 0x0084 /* LCDC AUS Mode Cursor Control Register */;
pub const IMX21LCDC_BGLUT: c_uint = 0x0800 /* Background Lookup Table */;
pub const IMX21LCDC_GWLUT: c_uint = 0x0C00 /* Graphic Window Lookup Table */;

// Values HSYNC, VSYNC and Framesize Register

// Values for LPCR Register

pub const BPP_RGB565: c_uint = 0x05;
pub const BPP_XRGB8888: c_uint = 0x07;
pub const LCDC_MIN_XRES: c_int = 64;
pub const LCDC_MIN_YRES: c_int = 64;
pub const LCDC_MAX_XRES: c_int = 1024;
pub const LCDC_MAX_YRES: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_lcdc {
    pub drm: drm_device,
    pub pipe: drm_simple_display_pipe,
    pub connector: *mut drm_connector,
    pub base: *mut void __iomem,
    pub clk_ipg: *mut clk,
    pub clk_ahb: *mut clk,
    pub clk_per: *mut clk,
}

    static const u32 imx_lcdc_formats[] = {
    DRM_FORMAT_RGB565, DRM_FORMAT_XRGB8888,
    };
    static inline struct imx_lcdc *imx_lcdc_from_drmdev(struct drm_device *drm)
    {
    return container_of(drm, struct imx_lcdc, drm);
    }
#[no_mangle]
unsafe extern "C" fn imx_lcdc_get_format(drm_format: c_uint) -> c_uint {
    static unsigned int imx_lcdc_get_format(unsigned int drm_format)
    {
    switch (drm_format) {
    default:
    DRM_WARN("Format not supported - fallback to XRGB8888\n");
    fallthrough;
    case DRM_FORMAT_XRGB8888:
    return BPP_XRGB8888;
    case DRM_FORMAT_RGB565:
    return BPP_RGB565;
    }
    }
    static void imx_lcdc_update_hw_registers(struct drm_simple_display_pipe *pipe,
    struct drm_plane_state *old_state,
    bool mode_set)
    {
    struct drm_crtc *crtc = &pipe.crtc;
    struct drm_plane_state *new_state = pipe.plane.state;
    struct drm_framebuffer *fb = new_state.fb;
    struct imx_lcdc *lcdc = imx_lcdc_from_drmdev(pipe.crtc.dev);
    u32 lpcr, lvcr, lhcr;
    u32 framesize;
    dma_addr_t addr;
    addr = drm_fb_dma_get_gem_addr(fb, new_state, 0);
// The LSSAR register specifies the LCD screen start address (SSA).
    writel(addr, lcdc.base + IMX21LCDC_LSSAR);
    if (!mode_set)
    return;
// Disable PER clock to make register write possible
    if (old_state && old_state.crtc && old_state.crtc.enabled)
    clk_disable_unprepare(lcdc.clk_per);
// Framesize
    framesize = FIELD_PREP(IMX21LCDC_LSR_XMAX, crtc.mode.hdisplay >> 4) |
    FIELD_PREP(IMX21LCDC_LSR_YMAX, crtc.mode.vdisplay);
    writel(framesize, lcdc.base + IMX21LCDC_LSR);
// HSYNC
    lhcr = FIELD_PREP(IMX21LCDC_LHCR_HFPORCH, crtc.mode.hsync_start - crtc.mode.hdisplay - 1) |
    FIELD_PREP(IMX21LCDC_LHCR_HWIDTH, crtc.mode.hsync_end - crtc.mode.hsync_start - 1) |
    FIELD_PREP(IMX21LCDC_LHCR_HBPORCH, crtc.mode.htotal - crtc.mode.hsync_end - 3);
    writel(lhcr, lcdc.base + IMX21LCDC_LHCR);
// VSYNC
    lvcr = FIELD_PREP(IMX21LCDC_LVCR_VFPORCH, crtc.mode.vsync_start - crtc.mode.vdisplay) |
    FIELD_PREP(IMX21LCDC_LVCR_VWIDTH, crtc.mode.vsync_end - crtc.mode.vsync_start) |
    FIELD_PREP(IMX21LCDC_LVCR_VBPORCH, crtc.mode.vtotal - crtc.mode.vsync_end);
    writel(lvcr, lcdc.base + IMX21LCDC_LVCR);
    lpcr = readl(lcdc.base + IMX21LCDC_LPCR);
    lpcr &= ~IMX21LCDC_LPCR_BPIX;
    lpcr |= FIELD_PREP(IMX21LCDC_LPCR_BPIX, imx_lcdc_get_format(fb.format.format));
    writel(lpcr, lcdc.base + IMX21LCDC_LPCR);
// Virtual Page Width
    writel(new_state.fb.pitches[0] / 4, lcdc.base + IMX21LCDC_LVPWR);
// Enable PER clock
    if (new_state.crtc.enabled)
    clk_prepare_enable(lcdc.clk_per);
    }
    static void imx_lcdc_pipe_enable(struct drm_simple_display_pipe *pipe,
    struct drm_crtc_state *crtc_state,
    struct drm_plane_state *plane_state)
    {
    int ret;
    int clk_div;
    int bpp;
    struct imx_lcdc *lcdc = imx_lcdc_from_drmdev(pipe.crtc.dev);
    struct drm_display_mode *mode = &pipe.crtc.mode;
    struct drm_display_info *disp_info = &lcdc.connector.display_info;
    let mut hsync_pol: c_int = (mode.flags & DRM_MODE_FLAG_PHSYNC) ? 0 : 1;
    let mut vsync_pol: c_int = (mode.flags & DRM_MODE_FLAG_PVSYNC) ? 0 : 1;
    const int data_enable_pol =
    (disp_info.bus_flags & DRM_BUS_FLAG_DE_HIGH) ? 0 : 1;
    const int clk_pol =
    (disp_info.bus_flags & DRM_BUS_FLAG_PIXDATA_DRIVE_POSEDGE) ? 0 : 1;
    clk_div = DIV_ROUND_CLOSEST_ULL(clk_get_rate(lcdc.clk_per),
    mode.clock * 1000);
    bpp = imx_lcdc_get_format(plane_state.fb.format.format);
    writel(FIELD_PREP(IMX21LCDC_LPCR_PCD, clk_div - 1) |
    FIELD_PREP(IMX21LCDC_LPCR_LPPOL, hsync_pol) |
    FIELD_PREP(IMX21LCDC_LPCR_FLMPOL, vsync_pol) |
    FIELD_PREP(IMX21LCDC_LPCR_OEPOL, data_enable_pol) |
    FIELD_PREP(IMX21LCDC_LPCR_TFT, 1) |
    FIELD_PREP(IMX21LCDC_LPCR_COLOR, 1) |
    FIELD_PREP(IMX21LCDC_LPCR_PBSIZ, 3) |
    FIELD_PREP(IMX21LCDC_LPCR_BPIX, bpp) |
    FIELD_PREP(IMX21LCDC_LPCR_SCLKSEL, 1) |
    FIELD_PREP(IMX21LCDC_LPCR_PIXPOL, 0) |
    FIELD_PREP(IMX21LCDC_LPCR_CLKPOL, clk_pol),
    lcdc.base + IMX21LCDC_LPCR);
// 0px panning offset
    writel(0x00000000, lcdc.base + IMX21LCDC_LPOR);
// disable hardware cursor
    writel(readl(lcdc.base + IMX21LCDC_LCPR) & ~(IMX21LCDC_LCPR_CC0 | IMX21LCDC_LCPR_CC1),
    lcdc.base + IMX21LCDC_LCPR);
    ret = clk_prepare_enable(lcdc.clk_ipg);
    if (ret) {
    dev_err(pipe.crtc.dev.dev, "Cannot enable ipg clock: %pe\n", ERR_PTR(ret));
    return;
    }
    ret = clk_prepare_enable(lcdc.clk_ahb);
    if (ret) {
    dev_err(pipe.crtc.dev.dev, "Cannot enable ahb clock: %pe\n", ERR_PTR(ret));
    clk_disable_unprepare(lcdc.clk_ipg);
    return;
    }
    imx_lcdc_update_hw_registers(pipe, core::ptr::null_mut(), true);
// Enable VBLANK Interrupt
    writel(INTR_EOF, lcdc.base + IMX21LCDC_LIER);
    }
#[no_mangle]
unsafe extern "C" fn imx_lcdc_pipe_disable(pipe: *mut drm_simple_display_pipe) {
    static void imx_lcdc_pipe_disable(struct drm_simple_display_pipe *pipe)
    {
    struct imx_lcdc *lcdc = imx_lcdc_from_drmdev(pipe.crtc.dev);
    struct drm_crtc *crtc = &lcdc.pipe.crtc;
    struct drm_pending_vblank_event *event;
    clk_disable_unprepare(lcdc.clk_ahb);
    clk_disable_unprepare(lcdc.clk_ipg);
    if (pipe.crtc.enabled)
    clk_disable_unprepare(lcdc.clk_per);
    spin_lock_irq(&lcdc.drm.event_lock);
    event = crtc.state.event;
    if (event) {
    crtc.state.event = core::ptr::null_mut();
    drm_crtc_send_vblank_event(crtc, event);
    }
    spin_unlock_irq(&lcdc.drm.event_lock);
// Disable VBLANK Interrupt
    writel(0, lcdc.base + IMX21LCDC_LIER);
    }
    static int imx_lcdc_pipe_check(struct drm_simple_display_pipe *pipe,
    struct drm_plane_state *plane_state,
    struct drm_crtc_state *crtc_state)
    {
    const struct drm_display_mode *mode = &crtc_state.mode;
    const struct drm_display_mode *old_mode = &pipe.crtc.state.mode;
    if (mode.hdisplay < LCDC_MIN_XRES || mode.hdisplay > LCDC_MAX_XRES ||
    mode.vdisplay < LCDC_MIN_YRES || mode.vdisplay > LCDC_MAX_YRES ||
    mode.hdisplay % 0x10) { /* must be multiple of 16 */
    drm_err(pipe.crtc.dev, "unsupported display mode (%u x %u)\n",
    mode.hdisplay, mode.vdisplay);
    return -EINVAL;
    }
    crtc_state.mode_changed =
    old_mode.hdisplay != mode.hdisplay ||
    old_mode.vdisplay != mode.vdisplay;
    return 0;
    }
    static void imx_lcdc_pipe_update(struct drm_simple_display_pipe *pipe,
    struct drm_plane_state *old_state)
    {
    struct drm_crtc *crtc = &pipe.crtc;
    struct drm_pending_vblank_event *event = crtc.state.event;
    struct drm_plane_state *new_state = pipe.plane.state;
    struct drm_framebuffer *fb = new_state.fb;
    struct drm_framebuffer *old_fb = old_state.fb;
    struct drm_crtc *old_crtc = old_state.crtc;
    let mut mode_changed: bool = false;
    if (old_fb && old_fb.format != fb.format)
    mode_changed = true;
#[no_mangle]
pub unsafe extern "C" fn if(crtc: old_crtc !=) -> else {
    else if (old_crtc != crtc)
    mode_changed = true;
    imx_lcdc_update_hw_registers(pipe, old_state, mode_changed);
    if (event) {
    crtc.state.event = core::ptr::null_mut();
    spin_lock_irq(&crtc.dev.event_lock);
    if (crtc.state.active && drm_crtc_vblank_get(crtc) == 0)
    drm_crtc_arm_vblank_event(crtc, event);
    else
    drm_crtc_send_vblank_event(crtc, event);
    spin_unlock_irq(&crtc.dev.event_lock);
    }
    }
    static const struct drm_simple_display_pipe_funcs imx_lcdc_pipe_funcs = {
    .enable = imx_lcdc_pipe_enable,
    .disable = imx_lcdc_pipe_disable,
    .check = imx_lcdc_pipe_check,
    .update = imx_lcdc_pipe_update,
    };
    static const struct drm_mode_config_funcs imx_lcdc_mode_config_funcs = {
    .fb_create = drm_gem_fb_create_with_dirty,
    .atomic_check = drm_atomic_helper_check,
    .atomic_commit = drm_atomic_helper_commit,
    };
    static const struct drm_mode_config_helper_funcs imx_lcdc_mode_config_helpers = {
    .atomic_commit_tail = drm_atomic_helper_commit_tail_rpm,
    };
    DEFINE_DRM_GEM_DMA_FOPS(imx_lcdc_drm_fops);
    static struct drm_driver imx_lcdc_drm_driver = {
    .driver_features = DRIVER_GEM | DRIVER_MODESET | DRIVER_ATOMIC,
    .fops = &imx_lcdc_drm_fops,
    DRM_GEM_DMA_DRIVER_OPS_VMAP,
    DRM_FBDEV_DMA_DRIVER_OPS,
    .name = "imx-lcdc",
    .desc = "i.MX LCDC driver",
    };
    static const struct of_device_id imx_lcdc_of_dev_id[] = {
    {
    .compatible = "fsl,imx21-lcdc",
    },
    {
    .compatible = "fsl,imx25-lcdc",
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx_lcdc_of_dev_id);
#[no_mangle]
unsafe extern "C" fn imx_lcdc_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx_lcdc_irq_handler(int irq, void *arg)
    {
    struct imx_lcdc *lcdc = arg;
    struct drm_crtc *crtc = &lcdc.pipe.crtc;
    unsigned int status;
    status = readl(lcdc.base + IMX21LCDC_LISR);
    if (status & INTR_EOF) {
    drm_crtc_handle_vblank(crtc);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn imx_lcdc_probe(pdev: *mut platform_device) -> c_int {
    static int imx_lcdc_probe(struct platform_device *pdev)
    {
    struct imx_lcdc *lcdc;
    struct drm_device *drm;
    struct drm_bridge *bridge;
    int irq;
    int ret;
    struct device *dev = &pdev.dev;
    lcdc = devm_drm_dev_alloc(dev, &imx_lcdc_drm_driver,
    struct imx_lcdc, drm);
    if (IS_ERR(lcdc))
    return PTR_ERR(lcdc);
    drm = &lcdc.drm;
    lcdc.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(lcdc.base))
    return dev_err_probe(dev, PTR_ERR(lcdc.base), "Cannot get IO memory\n");
    bridge = devm_drm_of_get_bridge(dev, dev.of_node, 0, 0);
    if (IS_ERR(bridge))
    return dev_err_probe(dev, PTR_ERR(bridge), "Failed to find bridge\n");
// Get Clocks
    lcdc.clk_ipg = devm_clk_get(dev, "ipg");
    if (IS_ERR(lcdc.clk_ipg))
    return dev_err_probe(dev, PTR_ERR(lcdc.clk_ipg), "Failed to get %s clk\n", "ipg");
    lcdc.clk_ahb = devm_clk_get(dev, "ahb");
    if (IS_ERR(lcdc.clk_ahb))
    return dev_err_probe(dev, PTR_ERR(lcdc.clk_ahb), "Failed to get %s clk\n", "ahb");
    lcdc.clk_per = devm_clk_get(dev, "per");
    if (IS_ERR(lcdc.clk_per))
    return dev_err_probe(dev, PTR_ERR(lcdc.clk_per), "Failed to get %s clk\n", "per");
    ret = dma_set_mask_and_coherent(drm.dev, DMA_BIT_MASK(32));
    if (ret)
    return dev_err_probe(dev, ret, "Cannot set DMA Mask\n");
// Modeset init
    ret = drmm_mode_config_init(drm);
    if (ret)
    return dev_err_probe(dev, ret, "Cannot initialize mode configuration structure\n");
// CRTC, Plane, Encoder
    ret = drm_simple_display_pipe_init(drm, &lcdc.pipe,
    &imx_lcdc_pipe_funcs,
    imx_lcdc_formats,
    ARRAY_SIZE(imx_lcdc_formats), core::ptr::null_mut(), core::ptr::null_mut());
    if (ret < 0)
    return dev_err_probe(drm.dev, ret, "Cannot setup simple display pipe\n");
    ret = drm_vblank_init(drm, drm.mode_config.num_crtc);
    if (ret < 0)
    return dev_err_probe(drm.dev, ret, "Failed to initialize vblank\n");
    ret = drm_bridge_attach(&lcdc.pipe.encoder, bridge, core::ptr::null_mut(), DRM_BRIDGE_ATTACH_NO_CONNECTOR);
    if (ret)
    return dev_err_probe(drm.dev, ret, "Cannot attach bridge\n");
    lcdc.connector = drm_bridge_connector_init(drm, &lcdc.pipe.encoder);
    if (IS_ERR(lcdc.connector))
    return dev_err_probe(drm.dev, PTR_ERR(lcdc.connector), "Cannot init bridge connector\n");
//
// The LCDC controller does not have an enable bit. The
// controller starts directly when the clocks are enabled.
// If the clocks are enabled when the controller is not yet
// programmed with proper register values (enabled at the
// bootloader, for example) then it just goes into some undefined
// state.
// To avoid this issue, let's enable and disable LCDC IPG,
// PER and AHB clock so that we force some kind of 'reset'
// to the LCDC block.
//
    ret = clk_prepare_enable(lcdc.clk_ipg);
    if (ret)
    return dev_err_probe(dev, ret, "Cannot enable ipg clock\n");
    clk_disable_unprepare(lcdc.clk_ipg);
    ret = clk_prepare_enable(lcdc.clk_per);
    if (ret)
    return dev_err_probe(dev, ret, "Cannot enable per clock\n");
    clk_disable_unprepare(lcdc.clk_per);
    ret = clk_prepare_enable(lcdc.clk_ahb);
    if (ret)
    return dev_err_probe(dev, ret, "Cannot enable ahb clock\n");
    clk_disable_unprepare(lcdc.clk_ahb);
    drm.mode_config.min_width = LCDC_MIN_XRES;
    drm.mode_config.max_width = LCDC_MAX_XRES;
    drm.mode_config.min_height = LCDC_MIN_YRES;
    drm.mode_config.max_height = LCDC_MAX_YRES;
    drm.mode_config.preferred_depth = 16;
    drm.mode_config.funcs = &imx_lcdc_mode_config_funcs;
    drm.mode_config.helper_private = &imx_lcdc_mode_config_helpers;
    drm_mode_config_reset(drm);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    ret = irq;
    return ret;
    }
    ret = devm_request_irq(dev, irq, imx_lcdc_irq_handler, 0, "imx-lcdc", lcdc);
    if (ret < 0)
    return dev_err_probe(drm.dev, ret, "Failed to install IRQ handler\n");
    platform_set_drvdata(pdev, drm);
    ret = drm_dev_register(&lcdc.drm, 0);
    if (ret)
    return dev_err_probe(dev, ret, "Cannot register device\n");
    drm_client_setup(drm, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_lcdc_remove(pdev: *mut platform_device) {
    static void imx_lcdc_remove(struct platform_device *pdev)
    {
    struct drm_device *drm = platform_get_drvdata(pdev);
    drm_dev_unregister(drm);
    drm_atomic_helper_shutdown(drm);
    }
#[no_mangle]
unsafe extern "C" fn imx_lcdc_shutdown(pdev: *mut platform_device) {
    static void imx_lcdc_shutdown(struct platform_device *pdev)
    {
    drm_atomic_helper_shutdown(platform_get_drvdata(pdev));
    }
    static struct platform_driver imx_lcdc_driver = {
    .driver = {
    .name = "imx-lcdc",
    .of_match_table = imx_lcdc_of_dev_id,
    },
    .probe = imx_lcdc_probe,
    .remove = imx_lcdc_remove,
    .shutdown = imx_lcdc_shutdown,
    };
    module_platform_driver(imx_lcdc_driver);
    MODULE_AUTHOR("Marian Cichy <M.Cichy@pengutronix.de>");
    MODULE_DESCRIPTION("Freescale i.MX LCDC driver");
    MODULE_LICENSE("GPL");
