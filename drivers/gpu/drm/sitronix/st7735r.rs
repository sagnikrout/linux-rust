//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/sitronix/st7735r.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// DRM driver for display panels connected to a Sitronix ST7715R or ST7735R
// display controller in SPI mode.
//
// Copyright 2017 David Lechner <david@lechnology.com>
// Copyright (C) 2019 Glider bvba
//

pub const ST7735R_FRMCTR1: c_uint = 0xb1;
pub const ST7735R_FRMCTR2: c_uint = 0xb2;
pub const ST7735R_FRMCTR3: c_uint = 0xb3;
pub const ST7735R_INVCTR: c_uint = 0xb4;
pub const ST7735R_PWCTR1: c_uint = 0xc0;
pub const ST7735R_PWCTR2: c_uint = 0xc1;
pub const ST7735R_PWCTR3: c_uint = 0xc2;
pub const ST7735R_PWCTR4: c_uint = 0xc3;
pub const ST7735R_PWCTR5: c_uint = 0xc4;
pub const ST7735R_VMCTR1: c_uint = 0xc5;
pub const ST7735R_GAMCTRP1: c_uint = 0xe0;
pub const ST7735R_GAMCTRN1: c_uint = 0xe1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st7735r_cfg {
    pub mode: drm_display_mode,
    pub left_offset: c_uint,
    pub top_offset: c_uint,
    pub write_only:1: c_uint,
    pub /: *mut *mut unsigned int rgb:1; / RGB (vs. BGR),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st7735r_device {
    pub /: *mut *mut mipi_dbi_dev dbidev; / Must be first for .release(),
    pub cfg: *const st7735r_cfg,
    pub plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
}

    static struct st7735r_device *to_st7735r_device(struct drm_device *drm)
    {
    return container_of(drm_to_mipi_dbi_dev(drm), struct st7735r_device, dbidev);
    }
    static const u32 st7735r_plane_formats[] = {
    DRM_MIPI_DBI_PLANE_FORMATS,
    };
    static const u64 st7735r_plane_format_modifiers[] = {
    DRM_MIPI_DBI_PLANE_FORMAT_MODIFIERS,
    };
    static const struct drm_plane_helper_funcs st7735r_plane_helper_funcs = {
    DRM_MIPI_DBI_PLANE_HELPER_FUNCS,
    };
    static const struct drm_plane_funcs st7735r_plane_funcs = {
    DRM_MIPI_DBI_PLANE_FUNCS,
    .destroy = drm_plane_cleanup,
    };
    static void st7735r_crtc_helper_atomic_enable(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    struct drm_device *drm = crtc.dev;
    struct st7735r_device *st7735r = to_st7735r_device(drm);
    struct mipi_dbi_dev *dbidev = &st7735r.dbidev;
    struct mipi_dbi *dbi = &dbidev.dbi;
    int ret, idx;
    u8 addr_mode;
    if (!drm_dev_enter(drm, &idx))
    return;
    DRM_DEBUG_KMS("\n");
    ret = mipi_dbi_poweron_reset(dbidev);
    if (ret)
    goto out_exit;
    msleep(150);
    mipi_dbi_command(dbi, MIPI_DCS_EXIT_SLEEP_MODE);
    msleep(500);
    mipi_dbi_command(dbi, ST7735R_FRMCTR1, 0x01, 0x2c, 0x2d);
    mipi_dbi_command(dbi, ST7735R_FRMCTR2, 0x01, 0x2c, 0x2d);
    mipi_dbi_command(dbi, ST7735R_FRMCTR3, 0x01, 0x2c, 0x2d, 0x01, 0x2c,
    0x2d);
    mipi_dbi_command(dbi, ST7735R_INVCTR, 0x07);
    mipi_dbi_command(dbi, ST7735R_PWCTR1, 0xa2, 0x02, 0x84);
    mipi_dbi_command(dbi, ST7735R_PWCTR2, 0xc5);
    mipi_dbi_command(dbi, ST7735R_PWCTR3, 0x0a, 0x00);
    mipi_dbi_command(dbi, ST7735R_PWCTR4, 0x8a, 0x2a);
    mipi_dbi_command(dbi, ST7735R_PWCTR5, 0x8a, 0xee);
    mipi_dbi_command(dbi, ST7735R_VMCTR1, 0x0e);
    mipi_dbi_command(dbi, MIPI_DCS_EXIT_INVERT_MODE);
    switch (dbidev.rotation) {
    default:
    addr_mode = ST7735R_MX | ST7735R_MY;
    break;
    case 90:
    addr_mode = ST7735R_MX | ST7735R_MV;
    break;
    case 180:
    addr_mode = 0;
    break;
    case 270:
    addr_mode = ST7735R_MY | ST7735R_MV;
    break;
    }
    if (st7735r.cfg.rgb)
    addr_mode |= ST7735R_RGB;
    mipi_dbi_command(dbi, MIPI_DCS_SET_ADDRESS_MODE, addr_mode);
    mipi_dbi_command(dbi, MIPI_DCS_SET_PIXEL_FORMAT,
    MIPI_DCS_PIXEL_FMT_16BIT);
    mipi_dbi_command(dbi, ST7735R_GAMCTRP1, 0x02, 0x1c, 0x07, 0x12, 0x37,
    0x32, 0x29, 0x2d, 0x29, 0x25, 0x2b, 0x39, 0x00, 0x01,
    0x03, 0x10);
    mipi_dbi_command(dbi, ST7735R_GAMCTRN1, 0x03, 0x1d, 0x07, 0x06, 0x2e,
    0x2c, 0x29, 0x2d, 0x2e, 0x2e, 0x37, 0x3f, 0x00, 0x00,
    0x02, 0x10);
    mipi_dbi_command(dbi, MIPI_DCS_SET_DISPLAY_ON);
    msleep(100);
    mipi_dbi_command(dbi, MIPI_DCS_ENTER_NORMAL_MODE);
    msleep(20);
    backlight_enable(dbidev.backlight);
    out_exit:
    drm_dev_exit(idx);
    }
    static const struct drm_crtc_helper_funcs st7735r_crtc_helper_funcs = {
    DRM_MIPI_DBI_CRTC_HELPER_FUNCS,
    .atomic_enable = st7735r_crtc_helper_atomic_enable,
    };
    static const struct drm_crtc_funcs st7735r_crtc_funcs = {
    DRM_MIPI_DBI_CRTC_FUNCS,
    .destroy = drm_crtc_cleanup,
    };
    static const struct drm_encoder_funcs st7735r_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
    static const struct drm_connector_helper_funcs st7735r_connector_helper_funcs = {
    DRM_MIPI_DBI_CONNECTOR_HELPER_FUNCS,
    };
    static const struct drm_connector_funcs st7735r_connector_funcs = {
    DRM_MIPI_DBI_CONNECTOR_FUNCS,
    .destroy = drm_connector_cleanup,
    };
    static const struct drm_mode_config_helper_funcs st7735r_mode_config_helper_funcs = {
    DRM_MIPI_DBI_MODE_CONFIG_HELPER_FUNCS,
    };
    static const struct drm_mode_config_funcs st7735r_mode_config_funcs = {
    DRM_MIPI_DBI_MODE_CONFIG_FUNCS,
    };
    static const struct st7735r_cfg jd_t18003_t01_cfg = {
    .mode		= { DRM_SIMPLE_MODE(128, 160, 28, 35) },
// Cannot read from Adafruit 1.8" display via SPI
    .write_only	= true,
    };
    static const struct st7735r_cfg rh128128t_cfg = {
    .mode		= { DRM_SIMPLE_MODE(128, 128, 25, 26) },
    .left_offset	= 2,
    .top_offset	= 3,
    .rgb		= true,
    };
    DEFINE_DRM_GEM_DMA_FOPS(st7735r_fops);
    static const struct drm_driver st7735r_driver = {
    .driver_features	= DRIVER_GEM | DRIVER_MODESET | DRIVER_ATOMIC,
    .fops			= &st7735r_fops,
    DRM_GEM_DMA_DRIVER_OPS_VMAP,
    DRM_FBDEV_DMA_DRIVER_OPS,
    .debugfs_init		= mipi_dbi_debugfs_init,
    .name			= "st7735r",
    .desc			= "Sitronix ST7735R",
    .major			= 1,
    .minor			= 0,
    };
    static const struct of_device_id st7735r_of_match[] = {
    { .compatible = "jianda,jd-t18003-t01", .data = &jd_t18003_t01_cfg },
    { .compatible = "okaya,rh128128t", .data = &rh128128t_cfg },
    { },
    };
    MODULE_DEVICE_TABLE(of, st7735r_of_match);
    static const struct spi_device_id st7735r_id[] = {
    { "jd-t18003-t01", (uintptr_t)&jd_t18003_t01_cfg },
    { "rh128128t", (uintptr_t)&rh128128t_cfg },
    { },
    };
    MODULE_DEVICE_TABLE(spi, st7735r_id);
#[no_mangle]
unsafe extern "C" fn st7735r_probe(spi: *mut spi_device) -> c_int {
    static int st7735r_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    const struct st7735r_cfg *cfg;
    struct mipi_dbi_dev *dbidev;
    struct st7735r_device *st7735r;
    struct drm_device *drm;
    struct mipi_dbi *dbi;
    struct gpio_desc *dc;
    struct drm_plane *plane;
    struct drm_crtc *crtc;
    struct drm_encoder *encoder;
    struct drm_connector *connector;
    let mut rotation: u32 = 0;
    int ret;
    cfg = device_get_match_data(&spi.dev);
    if (!cfg)
    cfg = (void *)spi_get_device_id(spi).driver_data;
    st7735r = devm_drm_dev_alloc(dev, &st7735r_driver, struct st7735r_device, dbidev.drm);
    if (IS_ERR(st7735r))
    return PTR_ERR(st7735r);
    dbidev = &st7735r.dbidev;
    st7735r.cfg = cfg;
    dbi = &dbidev.dbi;
    drm = &dbidev.drm;
    dbi.reset = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(dbi.reset))
    return dev_err_probe(dev, PTR_ERR(dbi.reset), "Failed to get GPIO 'reset'\n");
    dc = devm_gpiod_get(dev, "dc", GPIOD_OUT_LOW);
    if (IS_ERR(dc))
    return dev_err_probe(dev, PTR_ERR(dc), "Failed to get GPIO 'dc'\n");
    dbidev.backlight = devm_of_find_backlight(dev);
    if (IS_ERR(dbidev.backlight))
    return PTR_ERR(dbidev.backlight);
    device_property_read_u32(dev, "rotation", &rotation);
    ret = mipi_dbi_spi_init(spi, dbi, dc);
    if (ret)
    return ret;
    if (cfg.write_only)
    dbi.read_commands = core::ptr::null_mut();
    dbidev.left_offset = cfg.left_offset;
    dbidev.top_offset = cfg.top_offset;
    ret = drm_mipi_dbi_dev_init(dbidev, &cfg.mode, st7735r_plane_formats[0], rotation, 0);
    if (ret)
    return ret;
    ret = drmm_mode_config_init(drm);
    if (ret)
    return ret;
    drm.mode_config.min_width = dbidev.mode.hdisplay;
    drm.mode_config.max_width = dbidev.mode.hdisplay;
    drm.mode_config.min_height = dbidev.mode.vdisplay;
    drm.mode_config.max_height = dbidev.mode.vdisplay;
    drm.mode_config.funcs = &st7735r_mode_config_funcs;
    drm.mode_config.preferred_depth = 16;
    drm.mode_config.helper_private = &st7735r_mode_config_helper_funcs;
    plane = &st7735r.plane;
    ret = drm_universal_plane_init(drm, plane, 0, &st7735r_plane_funcs,
    st7735r_plane_formats, ARRAY_SIZE(st7735r_plane_formats),
    st7735r_plane_format_modifiers,
    DRM_PLANE_TYPE_PRIMARY, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_plane_helper_add(plane, &st7735r_plane_helper_funcs);
    drm_plane_enable_fb_damage_clips(plane);
    crtc = &st7735r.crtc;
    ret = drm_crtc_init_with_planes(drm, crtc, plane, core::ptr::null_mut(), &st7735r_crtc_funcs, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_crtc_helper_add(crtc, &st7735r_crtc_helper_funcs);
    encoder = &st7735r.encoder;
    ret = drm_encoder_init(drm, encoder, &st7735r_encoder_funcs, DRM_MODE_ENCODER_NONE, core::ptr::null_mut());
    if (ret)
    return ret;
    encoder.possible_crtcs = drm_crtc_mask(crtc);
    connector = &st7735r.connector;
    ret = drm_connector_init(drm, connector, &st7735r_connector_funcs,
    DRM_MODE_CONNECTOR_SPI);
    if (ret)
    return ret;
    drm_connector_helper_add(connector, &st7735r_connector_helper_funcs);
    ret = drm_connector_attach_encoder(connector, encoder);
    if (ret)
    return ret;
    drm_mode_config_reset(drm);
    ret = drm_dev_register(drm, 0);
    if (ret)
    return ret;
    spi_set_drvdata(spi, drm);
    drm_client_setup(drm, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st7735r_remove(spi: *mut spi_device) {
    static void st7735r_remove(struct spi_device *spi)
    {
    struct drm_device *drm = spi_get_drvdata(spi);
    drm_dev_unplug(drm);
    drm_atomic_helper_shutdown(drm);
    }
#[no_mangle]
unsafe extern "C" fn st7735r_shutdown(spi: *mut spi_device) {
    static void st7735r_shutdown(struct spi_device *spi)
    {
    drm_atomic_helper_shutdown(spi_get_drvdata(spi));
    }
    static struct spi_driver st7735r_spi_driver = {
    .driver = {
    .name = "st7735r",
    .of_match_table = st7735r_of_match,
    },
    .id_table = st7735r_id,
    .probe = st7735r_probe,
    .remove = st7735r_remove,
    .shutdown = st7735r_shutdown,
    };
    module_spi_driver(st7735r_spi_driver);
    MODULE_DESCRIPTION("Sitronix ST7735R DRM driver");
    MODULE_AUTHOR("David Lechner <david@lechnology.com>");
    MODULE_LICENSE("GPL");
