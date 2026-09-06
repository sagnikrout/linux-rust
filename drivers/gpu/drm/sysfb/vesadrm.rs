//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/sysfb/vesadrm.c
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

pub const DRIVER_MAJOR: c_int = 1;
pub const DRIVER_MINOR: c_int = 0;
pub const VESADRM_GAMMA_LUT_SIZE: c_int = 256;
    static const struct drm_format_info *vesadrm_get_format_si(struct drm_device *dev,
    const struct screen_info *si)
    {
    static const struct drm_sysfb_format formats[] = {
    { PIXEL_FORMAT_XRGB1555, DRM_FORMAT_XRGB1555, },
    { PIXEL_FORMAT_RGB565, DRM_FORMAT_RGB565, },
    { PIXEL_FORMAT_RGB888, DRM_FORMAT_RGB888, },
    { PIXEL_FORMAT_XRGB8888, DRM_FORMAT_XRGB8888, },
    { PIXEL_FORMAT_XBGR8888, DRM_FORMAT_XBGR8888, },
    { PIXEL_FORMAT_C8, DRM_FORMAT_C8, },
    };
    struct pixel_format pixel;
    int ret;
    ret = screen_info_pixel_format(si, &pixel);
    if (ret)
    return core::ptr::null_mut();
    return drm_sysfb_get_format(dev, formats, ARRAY_SIZE(formats), &pixel);
    }
//
// VESA device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vesadrm_device {
    pub sysfb: drm_sysfb_device,

// VESA Protected Mode interface
    struct {
    pub PrimaryPalette: *const u8,
    pub pmi: },

    void (*cmap_write)(struct vesadrm_device *vesa, unsigned int index,
    pub blue): u16 red, u16 green, u16,
// modesetting
    pub formats: [u32; DRM_SYSFB_PLANE_NFORMATS(1)],
    pub primary_plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
}

    static struct vesadrm_device *to_vesadrm_device(struct drm_device *dev)
    {
    return container_of(to_drm_sysfb_device(dev), struct vesadrm_device, sysfb);
    }
//
// Color LUT
//
    static void vesadrm_vga_cmap_write(struct vesadrm_device *vesa, unsigned int index,
    u16 red, u16 green, u16 blue)
    {
    let mut i8: u8 = index;
    let mut r8: u8 = red >> 8;
    let mut g8: u8 = green >> 8;
    let mut b8: u8 = blue >> 8;
    outb_p(i8, VGA_PEL_IW);
    outb_p(r8, VGA_PEL_D);
    outb_p(g8, VGA_PEL_D);
    outb_p(b8, VGA_PEL_D);
    }

    static void vesadrm_pmi_cmap_write(struct vesadrm_device *vesa, unsigned int index,
    u16 red, u16 green, u16 blue)
    {
    let mut i32: u32 = index;
    struct {
    u8 b8;
    u8 g8;
    u8 r8;
    u8 x8;
    } PaletteEntry = {
    blue >> 8,
    green >> 8,
    red >> 8,
    0x00,
    };
    __asm__ __volatile__ (
    "call *(%%esi)"
    : /* no return value */
    : "a" (0x4f09),
    "b" (0),
    "c" (1),
    "d" (i32),
    "D" (&PaletteEntry),
    "S" (&vesa.pmi.PrimaryPalette));
    }

    static void vesadrm_set_color_lut(struct drm_crtc *crtc, unsigned int index,
    u16 red, u16 green, u16 blue)
    {
    struct drm_device *dev = crtc.dev;
    struct vesadrm_device *vesa = to_vesadrm_device(dev);
    let mut i8: u8 = index & 0xff;
    if (drm_WARN_ON_ONCE(dev, index != i8))
    return; /* driver bug */
    vesa.cmap_write(vesa, i8, red, green, blue);
    }
    static void vesadrm_fill_gamma_lut(struct vesadrm_device *vesa,
    const struct drm_format_info *format)
    {
    struct drm_device *dev = &vesa.sysfb.dev;
    struct drm_crtc *crtc = &vesa.crtc;
    switch (format.format) {
    case DRM_FORMAT_XRGB1555:
    drm_crtc_fill_gamma_555(crtc, vesadrm_set_color_lut);
    break;
    case DRM_FORMAT_RGB565:
    drm_crtc_fill_gamma_565(crtc, vesadrm_set_color_lut);
    break;
    case DRM_FORMAT_RGB888:
    case DRM_FORMAT_XRGB8888:
    case DRM_FORMAT_BGRX8888:
    drm_crtc_fill_gamma_888(crtc, vesadrm_set_color_lut);
    break;
    default:
    drm_warn_once(dev, "Unsupported format %p4cc for gamma correction\n",
    &format.format);
    break;
    }
    }
    static void vesadrm_load_gamma_lut(struct vesadrm_device *vesa,
    const struct drm_format_info *format,
    struct drm_color_lut *lut)
    {
    struct drm_device *dev = &vesa.sysfb.dev;
    struct drm_crtc *crtc = &vesa.crtc;
    switch (format.format) {
    case DRM_FORMAT_XRGB1555:
    drm_crtc_load_gamma_555_from_888(crtc, lut, vesadrm_set_color_lut);
    break;
    case DRM_FORMAT_RGB565:
    drm_crtc_load_gamma_565_from_888(crtc, lut, vesadrm_set_color_lut);
    break;
    case DRM_FORMAT_RGB888:
    case DRM_FORMAT_XRGB8888:
    case DRM_FORMAT_BGRX8888:
    drm_crtc_load_gamma_888(crtc, lut, vesadrm_set_color_lut);
    break;
    default:
    drm_warn_once(dev, "Unsupported format %p4cc for gamma correction\n",
    &format.format);
    break;
    }
    }
    static void vesadrm_fill_palette_lut(struct vesadrm_device *vesa,
    const struct drm_format_info *format)
    {
    struct drm_device *dev = &vesa.sysfb.dev;
    struct drm_crtc *crtc = &vesa.crtc;
    switch (format.format) {
    case DRM_FORMAT_C8:
    drm_crtc_fill_palette_8(crtc, vesadrm_set_color_lut);
    break;
    case DRM_FORMAT_RGB332:
    drm_crtc_fill_palette_332(crtc, vesadrm_set_color_lut);
    break;
    default:
    drm_warn_once(dev, "Unsupported format %p4cc for palette\n",
    &format.format);
    break;
    }
    }
    static void vesadrm_load_palette_lut(struct vesadrm_device *vesa,
    const struct drm_format_info *format,
    struct drm_color_lut *lut)
    {
    struct drm_device *dev = &vesa.sysfb.dev;
    struct drm_crtc *crtc = &vesa.crtc;
    switch (format.format) {
    case DRM_FORMAT_C8:
    drm_crtc_load_palette_8(crtc, lut, vesadrm_set_color_lut);
    break;
    default:
    drm_warn_once(dev, "Unsupported format %p4cc for gamma correction\n",
    &format.format);
    break;
    }
    }
//
// Modesetting
//
    static const u64 vesadrm_primary_plane_format_modifiers[] = {
    DRM_SYSFB_PLANE_FORMAT_MODIFIERS,
    };
    static int vesadrm_primary_plane_helper_atomic_check(struct drm_plane *plane,
    struct drm_atomic_commit *new_state)
    {
    struct drm_sysfb_device *sysfb = to_drm_sysfb_device(plane.dev);
    struct drm_plane_state *new_plane_state = drm_atomic_get_new_plane_state(new_state, plane);
    struct drm_framebuffer *new_fb = new_plane_state.fb;
    struct drm_crtc_state *new_crtc_state;
    struct drm_sysfb_crtc_state *new_sysfb_crtc_state;
    int ret;
    ret = drm_sysfb_plane_helper_atomic_check(plane, new_state);
    if (ret)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !new_plane_state->visible) -> else {
    else if (!new_plane_state.visible)
    return 0;
//
// Fix up format conversion for specific cases
//
    switch (sysfb.fb_format.format) {
    case DRM_FORMAT_C8:
    new_crtc_state = drm_atomic_get_new_crtc_state(new_state, new_plane_state.crtc);
    new_sysfb_crtc_state = to_drm_sysfb_crtc_state(new_crtc_state);
    switch (new_fb.format.format) {
    case DRM_FORMAT_XRGB8888:
//
// Reduce XRGB8888 to RGB332. Each resulting pixel is an index
// into the C8 hardware palette, which stores RGB332 colors.
//
    if (new_sysfb_crtc_state.format.format != DRM_FORMAT_RGB332) {
    new_sysfb_crtc_state.format =
    drm_format_info(DRM_FORMAT_RGB332);
    new_crtc_state.color_mgmt_changed = true;
    }
    break;
    case DRM_FORMAT_C8:
//
// Restore original output. Emulation of XRGB8888 set RBG332
// output format and hardware palette. This needs to be undone
// when we switch back to DRM_FORMAT_C8.
//
    if (new_sysfb_crtc_state.format.format == DRM_FORMAT_RGB332) {
    new_sysfb_crtc_state.format = sysfb.fb_format;
    new_crtc_state.color_mgmt_changed = true;
    }
    break;
    }
    break;
    }
    return 0;
    }
    static const struct drm_plane_helper_funcs vesadrm_primary_plane_helper_funcs = {
    .begin_fb_access = drm_sysfb_plane_helper_begin_fb_access,
    .end_fb_access = drm_gem_end_shadow_fb_access,
    .atomic_check = vesadrm_primary_plane_helper_atomic_check,
    .atomic_update = drm_sysfb_plane_helper_atomic_update,
    .atomic_disable = drm_sysfb_plane_helper_atomic_disable,
    .get_scanout_buffer = drm_sysfb_plane_helper_get_scanout_buffer,
    };
    static const struct drm_plane_funcs vesadrm_primary_plane_funcs = {
    DRM_SYSFB_PLANE_FUNCS,
    .destroy = drm_plane_cleanup,
    };
    static void vesadrm_crtc_helper_atomic_flush(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    struct drm_device *dev = crtc.dev;
    struct drm_sysfb_device *sysfb = to_drm_sysfb_device(dev);
    struct vesadrm_device *vesa = to_vesadrm_device(dev);
    struct drm_crtc_state *crtc_state = drm_atomic_get_new_crtc_state(state, crtc);
    struct drm_sysfb_crtc_state *sysfb_crtc_state = to_drm_sysfb_crtc_state(crtc_state);
//
// The gamma LUT has to be reloaded after changing the primary
// plane's color format.
//
    if (crtc_state.enable && crtc_state.color_mgmt_changed) {
    switch (sysfb.fb_format.format) {
//
// Index formats
//
    case DRM_FORMAT_C8:
    if (sysfb_crtc_state.format.format == DRM_FORMAT_RGB332) {
    vesadrm_fill_palette_lut(vesa, sysfb_crtc_state.format);
    } else if (crtc.state.gamma_lut) {
    vesadrm_load_palette_lut(vesa,
    sysfb_crtc_state.format,
    crtc_state.gamma_lut.data);
    } else {
    vesadrm_fill_palette_lut(vesa, sysfb_crtc_state.format);
    }
    break;
//
// Component formats
//
    default:
    if (sysfb_crtc_state.format == sysfb.fb_format) {
    if (crtc_state.gamma_lut)
    vesadrm_load_gamma_lut(vesa,
    sysfb_crtc_state.format,
    crtc_state.gamma_lut.data);
    else
    vesadrm_fill_gamma_lut(vesa, sysfb_crtc_state.format);
    } else {
    vesadrm_fill_gamma_lut(vesa, sysfb_crtc_state.format);
    }
    break;
    }
    }
    }
    static const struct drm_crtc_helper_funcs vesadrm_crtc_helper_funcs = {
    DRM_SYSFB_CRTC_HELPER_FUNCS,
    .atomic_flush = vesadrm_crtc_helper_atomic_flush,
    };
    static const struct drm_crtc_funcs vesadrm_crtc_funcs = {
    DRM_SYSFB_CRTC_FUNCS,
    .destroy = drm_crtc_cleanup,
    };
    static const struct drm_encoder_funcs vesadrm_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
    static const struct drm_connector_helper_funcs vesadrm_connector_helper_funcs = {
    DRM_SYSFB_CONNECTOR_HELPER_FUNCS,
    };
    static const struct drm_connector_funcs vesadrm_connector_funcs = {
    DRM_SYSFB_CONNECTOR_FUNCS,
    .destroy = drm_connector_cleanup,
    };
    static const struct drm_mode_config_funcs vesadrm_mode_config_funcs = {
    DRM_SYSFB_MODE_CONFIG_FUNCS,
    };
//
// Init / Cleanup
//
    static struct vesadrm_device *vesadrm_device_create(struct drm_driver *drv,
    struct platform_device *pdev)
    {
    const struct sysfb_display_info *dpy;
    const struct screen_info *si;
    const struct drm_format_info *format;
    int width, height, stride;
    s64 vsize;
    struct resource resbuf;
    struct resource *res;
    struct vesadrm_device *vesa;
    struct drm_sysfb_device *sysfb;
    struct drm_device *dev;
    struct resource *mem = core::ptr::null_mut();
    void __iomem *screen_base;
    struct drm_plane *primary_plane;
    struct drm_crtc *crtc;
    struct drm_encoder *encoder;
    struct drm_connector *connector;
    unsigned long max_width, max_height;
    size_t nformats;
    int ret;
    dpy = dev_get_platdata(&pdev.dev);
    if (!dpy)
    return ERR_PTR(-ENODEV);
    si = &dpy.screen;
    if (screen_info_video_type(si) != VIDEO_TYPE_VLFB)
    return ERR_PTR(-ENODEV);
//
// VESA DRM driver
//
    vesa = devm_drm_dev_alloc(&pdev.dev, drv, struct vesadrm_device, sysfb.dev);
    if (IS_ERR(vesa))
    return ERR_CAST(vesa);
    sysfb = &vesa.sysfb;
    dev = &sysfb.dev;
    platform_set_drvdata(pdev, dev);
//
// Hardware settings
//
    format = vesadrm_get_format_si(dev, si);
    if (!format)
    return ERR_PTR(-EINVAL);
    width = drm_sysfb_get_width_si(dev, si);
    if (width < 0)
    return ERR_PTR(width);
    height = drm_sysfb_get_height_si(dev, si);
    if (height < 0)
    return ERR_PTR(height);
    res = drm_sysfb_get_memory_si(dev, si, &resbuf);
    if (!res)
    return ERR_PTR(-EINVAL);
    stride = drm_sysfb_get_stride_si(dev, si, format, width, height, resource_size(res));
    if (stride < 0)
    return ERR_PTR(stride);
    vsize = drm_sysfb_get_visible_size_si(dev, si, height, stride, resource_size(res));
    if (vsize < 0)
    return ERR_PTR(vsize);
    drm_dbg(dev, "framebuffer format=%p4cc, size=%dx%d, stride=%d bytes\n",
    &format.format, width, height, stride);
    if (!__screen_info_vbe_mode_nonvga(si)) {
    vesa.cmap_write = vesadrm_vga_cmap_write;
    } else {

    let mut pmi_base: phys_addr_t = __screen_info_vesapm_info_base(si);
    if (pmi_base) {
    const u16 *pmi_addr = phys_to_virt(pmi_base);
    vesa.pmi.PrimaryPalette = (u8 *)pmi_addr + pmi_addr[2];
    vesa.cmap_write = vesadrm_pmi_cmap_write;
    } else

    if (format.is_color_indexed)
    drm_warn(dev, "hardware palette is unchangeable, colors may be incorrect\n");
    }

    if (drm_edid_header_is_valid(dpy.edid.dummy) == 8)
    sysfb.edid = dpy.edid.dummy;

    sysfb.fb_mode = drm_sysfb_mode(width, height, 0, 0);
    sysfb.fb_format = format;
    sysfb.fb_pitch = stride;
    if (vesa.cmap_write)
    sysfb.fb_gamma_lut_size = VESADRM_GAMMA_LUT_SIZE;
//
// Memory management
//
    ret = devm_aperture_acquire_for_platform_device(pdev, res.start, vsize);
    if (ret) {
    drm_err(dev, "could not acquire memory range %pr: %d\n", res, ret);
    return ERR_PTR(ret);
    }
    drm_dbg(dev, "using I/O memory framebuffer at %pr\n", res);
    mem = devm_request_mem_region(&pdev.dev, res.start, vsize, drv.name);
    if (!mem) {
//
// We cannot make this fatal. Sometimes this comes from magic
// spaces our resource handlers simply don't know about. Use
// the I/O-memory resource as-is and try to map that instead.
//
    drm_warn(dev, "could not acquire memory region %pr\n", res);
    mem = res;
    }
    screen_base = devm_ioremap_wc(&pdev.dev, mem.start, resource_size(mem));
    if (!screen_base)
    return ERR_PTR(-ENOMEM);
    iosys_map_set_vaddr_iomem(&sysfb.fb_addr, screen_base);
//
// Modesetting
//
    ret = drmm_mode_config_init(dev);
    if (ret)
    return ERR_PTR(ret);
    max_width = max_t(unsigned long, width, DRM_SHADOW_PLANE_MAX_WIDTH);
    max_height = max_t(unsigned long, height, DRM_SHADOW_PLANE_MAX_HEIGHT);
    dev.mode_config.min_width = width;
    dev.mode_config.max_width = max_width;
    dev.mode_config.min_height = height;
    dev.mode_config.max_height = max_height;
    dev.mode_config.preferred_depth = format.depth;
    dev.mode_config.funcs = &vesadrm_mode_config_funcs;
// Primary plane
    nformats = drm_sysfb_build_fourcc_list(dev, &format.format, 1,
    vesa.formats, ARRAY_SIZE(vesa.formats));
    primary_plane = &vesa.primary_plane;
    ret = drm_universal_plane_init(dev, primary_plane, 0, &vesadrm_primary_plane_funcs,
    vesa.formats, nformats,
    vesadrm_primary_plane_format_modifiers,
    DRM_PLANE_TYPE_PRIMARY, core::ptr::null_mut());
    if (ret)
    return ERR_PTR(ret);
    drm_plane_helper_add(primary_plane, &vesadrm_primary_plane_helper_funcs);
    drm_plane_enable_fb_damage_clips(primary_plane);
// CRTC
    crtc = &vesa.crtc;
    ret = drm_crtc_init_with_planes(dev, crtc, primary_plane, core::ptr::null_mut(),
    &vesadrm_crtc_funcs, core::ptr::null_mut());
    if (ret)
    return ERR_PTR(ret);
    drm_crtc_helper_add(crtc, &vesadrm_crtc_helper_funcs);
    if (sysfb.fb_gamma_lut_size) {
    ret = drm_mode_crtc_set_gamma_size(crtc, sysfb.fb_gamma_lut_size);
    if (!ret)
    drm_crtc_enable_color_mgmt(crtc, 0, false, sysfb.fb_gamma_lut_size);
    }
// Encoder
    encoder = &vesa.encoder;
    ret = drm_encoder_init(dev, encoder, &vesadrm_encoder_funcs,
    DRM_MODE_ENCODER_NONE, core::ptr::null_mut());
    if (ret)
    return ERR_PTR(ret);
    encoder.possible_crtcs = drm_crtc_mask(crtc);
// Connector
    connector = &vesa.connector;
    ret = drm_connector_init(dev, connector, &vesadrm_connector_funcs,
    DRM_MODE_CONNECTOR_Unknown);
    if (ret)
    return ERR_PTR(ret);
    drm_connector_helper_add(connector, &vesadrm_connector_helper_funcs);
    drm_connector_set_panel_orientation_with_quirk(connector,
    DRM_MODE_PANEL_ORIENTATION_UNKNOWN,
    width, height);
    if (sysfb.edid)
    drm_connector_attach_edid_property(connector);
    ret = drm_connector_attach_encoder(connector, encoder);
    if (ret)
    return ERR_PTR(ret);
    drm_mode_config_reset(dev);
    return vesa;
    }
//
// DRM driver
//
    DEFINE_DRM_GEM_FOPS(vesadrm_fops);
    static struct drm_driver vesadrm_driver = {
    DRM_GEM_SHMEM_DRIVER_OPS,
    DRM_FBDEV_SHMEM_DRIVER_OPS,
    .name			= DRIVER_NAME,
    .desc			= DRIVER_DESC,
    .major			= DRIVER_MAJOR,
    .minor			= DRIVER_MINOR,
    .driver_features	= DRIVER_ATOMIC | DRIVER_GEM | DRIVER_MODESET,
    .fops			= &vesadrm_fops,
    };
//
// Platform driver
//
#[no_mangle]
unsafe extern "C" fn vesadrm_pm_suspend(dev: *mut device) -> c_int {
    static int vesadrm_pm_suspend(struct device *dev)
    {
    struct drm_device *drm = dev_get_drvdata(dev);
    return drm_mode_config_helper_suspend(drm);
    }
#[no_mangle]
unsafe extern "C" fn vesadrm_pm_resume(dev: *mut device) -> c_int {
    static int vesadrm_pm_resume(struct device *dev)
    {
    struct drm_device *drm = dev_get_drvdata(dev);
    return drm_mode_config_helper_resume(drm);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(vesadrm_pm_ops, vesadrm_pm_suspend, vesadrm_pm_resume);
#[no_mangle]
unsafe extern "C" fn vesadrm_probe(pdev: *mut platform_device) -> c_int {
    static int vesadrm_probe(struct platform_device *pdev)
    {
    struct vesadrm_device *vesa;
    struct drm_sysfb_device *sysfb;
    struct drm_device *dev;
    int ret;
    vesa = vesadrm_device_create(&vesadrm_driver, pdev);
    if (IS_ERR(vesa))
    return PTR_ERR(vesa);
    sysfb = &vesa.sysfb;
    dev = &sysfb.dev;
    ret = drm_dev_register(dev, 0);
    if (ret)
    return ret;
    drm_client_setup(dev, sysfb.fb_format);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vesadrm_remove(pdev: *mut platform_device) {
    static void vesadrm_remove(struct platform_device *pdev)
    {
    struct drm_device *dev = platform_get_drvdata(pdev);
    drm_dev_unplug(dev);
    }
    static struct platform_driver vesadrm_platform_driver = {
    .driver = {
    .name = "vesa-framebuffer",
    .pm = pm_sleep_ptr(&vesadrm_pm_ops),
    },
    .probe = vesadrm_probe,
    .remove = vesadrm_remove,
    };
    module_platform_driver(vesadrm_platform_driver);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
