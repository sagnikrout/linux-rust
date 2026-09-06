//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tiny/ili9225.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// DRM driver for Ilitek ILI9225 panels
//
// Copyright 2017 David Lechner <david@lechnology.com>
//
// Some code copied from mipi-dbi.c
// Copyright 2016 Noralf Trønnes
//

pub const ILI9225_DRIVER_READ_CODE: c_uint = 0x00;
pub const ILI9225_DRIVER_OUTPUT_CONTROL: c_uint = 0x01;
pub const ILI9225_LCD_AC_DRIVING_CONTROL: c_uint = 0x02;
pub const ILI9225_ENTRY_MODE: c_uint = 0x03;
pub const ILI9225_DISPLAY_CONTROL_1: c_uint = 0x07;
pub const ILI9225_BLANK_PERIOD_CONTROL_1: c_uint = 0x08;
pub const ILI9225_FRAME_CYCLE_CONTROL: c_uint = 0x0b;
pub const ILI9225_INTERFACE_CONTROL: c_uint = 0x0c;
pub const ILI9225_OSCILLATION_CONTROL: c_uint = 0x0f;
pub const ILI9225_POWER_CONTROL_1: c_uint = 0x10;
pub const ILI9225_POWER_CONTROL_2: c_uint = 0x11;
pub const ILI9225_POWER_CONTROL_3: c_uint = 0x12;
pub const ILI9225_POWER_CONTROL_4: c_uint = 0x13;
pub const ILI9225_POWER_CONTROL_5: c_uint = 0x14;
pub const ILI9225_VCI_RECYCLING: c_uint = 0x15;
pub const ILI9225_RAM_ADDRESS_SET_1: c_uint = 0x20;
pub const ILI9225_RAM_ADDRESS_SET_2: c_uint = 0x21;
pub const ILI9225_WRITE_DATA_TO_GRAM: c_uint = 0x22;
pub const ILI9225_SOFTWARE_RESET: c_uint = 0x28;
pub const ILI9225_GATE_SCAN_CONTROL: c_uint = 0x30;
pub const ILI9225_VERTICAL_SCROLL_1: c_uint = 0x31;
pub const ILI9225_VERTICAL_SCROLL_2: c_uint = 0x32;
pub const ILI9225_VERTICAL_SCROLL_3: c_uint = 0x33;
pub const ILI9225_PARTIAL_DRIVING_POS_1: c_uint = 0x34;
pub const ILI9225_PARTIAL_DRIVING_POS_2: c_uint = 0x35;
pub const ILI9225_HORIZ_WINDOW_ADDR_1: c_uint = 0x36;
pub const ILI9225_HORIZ_WINDOW_ADDR_2: c_uint = 0x37;
pub const ILI9225_VERT_WINDOW_ADDR_1: c_uint = 0x38;
pub const ILI9225_VERT_WINDOW_ADDR_2: c_uint = 0x39;
pub const ILI9225_GAMMA_CONTROL_1: c_uint = 0x50;
pub const ILI9225_GAMMA_CONTROL_2: c_uint = 0x51;
pub const ILI9225_GAMMA_CONTROL_3: c_uint = 0x52;
pub const ILI9225_GAMMA_CONTROL_4: c_uint = 0x53;
pub const ILI9225_GAMMA_CONTROL_5: c_uint = 0x54;
pub const ILI9225_GAMMA_CONTROL_6: c_uint = 0x55;
pub const ILI9225_GAMMA_CONTROL_7: c_uint = 0x56;
pub const ILI9225_GAMMA_CONTROL_8: c_uint = 0x57;
pub const ILI9225_GAMMA_CONTROL_9: c_uint = 0x58;
pub const ILI9225_GAMMA_CONTROL_10: c_uint = 0x59;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ili9225_device {
    pub dbidev: mipi_dbi_dev,
    pub plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
}

    static struct ili9225_device *to_ili9225_device(struct drm_device *dev)
    {
    return container_of(drm_to_mipi_dbi_dev(dev), struct ili9225_device, dbidev);
    }
#[no_mangle]
pub unsafe extern "C" fn ili9225_command(dbi: *mut mipi_dbi, cmd: u8, data: u16) -> c_int {
    static inline int ili9225_command(struct mipi_dbi *dbi, u8 cmd, u16 data)
    {
    u8 par[2] = { data >> 8, data & 0xff };
    return mipi_dbi_command_buf(dbi, cmd, par, 2);
    }
    static void ili9225_fb_dirty(struct iosys_map *src, struct drm_framebuffer *fb,
    struct drm_rect *rect, struct drm_format_conv_state *fmtcnv_state)
    {
    struct mipi_dbi_dev *dbidev = drm_to_mipi_dbi_dev(fb.dev);
    let mut height: c_uint = rect.y2 - rect.y1;
    let mut width: c_uint = rect.x2 - rect.x1;
    struct mipi_dbi *dbi = &dbidev.dbi;
    let mut swap: bool = dbi.swap_bytes;
    u16 x_start, y_start;
    u16 x1, x2, y1, y2;
    let mut ret: c_int = 0;
    bool full;
    void *tr;
    full = width == fb.width && height == fb.height;
    DRM_DEBUG_KMS("Flushing [FB:%d] " DRM_RECT_FMT "\n", fb.base.id, DRM_RECT_ARG(rect));
    if (!dbi.dc || !full || swap ||
    fb.format.format == DRM_FORMAT_XRGB8888) {
    tr = dbidev.tx_buf;
    ret = mipi_dbi_buf_copy(tr, src, fb, rect, swap, fmtcnv_state);
    if (ret)
    goto err_msg;
    } else {
    tr = src.vaddr; /* TODO: Use mapping abstraction properly */
    }
    switch (dbidev.rotation) {
    default:
    x1 = rect.x1;
    x2 = rect.x2 - 1;
    y1 = rect.y1;
    y2 = rect.y2 - 1;
    x_start = x1;
    y_start = y1;
    break;
    case 90:
    x1 = rect.y1;
    x2 = rect.y2 - 1;
    y1 = fb.width - rect.x2;
    y2 = fb.width - rect.x1 - 1;
    x_start = x1;
    y_start = y2;
    break;
    case 180:
    x1 = fb.width - rect.x2;
    x2 = fb.width - rect.x1 - 1;
    y1 = fb.height - rect.y2;
    y2 = fb.height - rect.y1 - 1;
    x_start = x2;
    y_start = y2;
    break;
    case 270:
    x1 = fb.height - rect.y2;
    x2 = fb.height - rect.y1 - 1;
    y1 = rect.x1;
    y2 = rect.x2 - 1;
    x_start = x2;
    y_start = y1;
    break;
    }
    ili9225_command(dbi, ILI9225_HORIZ_WINDOW_ADDR_1, x2);
    ili9225_command(dbi, ILI9225_HORIZ_WINDOW_ADDR_2, x1);
    ili9225_command(dbi, ILI9225_VERT_WINDOW_ADDR_1, y2);
    ili9225_command(dbi, ILI9225_VERT_WINDOW_ADDR_2, y1);
    ili9225_command(dbi, ILI9225_RAM_ADDRESS_SET_1, x_start);
    ili9225_command(dbi, ILI9225_RAM_ADDRESS_SET_2, y_start);
    ret = mipi_dbi_command_buf(dbi, ILI9225_WRITE_DATA_TO_GRAM, tr,
    width * height * 2);
    err_msg:
    if (ret)
    dev_err_once(fb.dev.dev, "Failed to update display %d\n", ret);
    }
    static const u32 ili9225_plane_formats[] = {
    DRM_MIPI_DBI_PLANE_FORMATS,
    };
    static const u64 ili9225_plane_format_modifiers[] = {
    DRM_MIPI_DBI_PLANE_FORMAT_MODIFIERS,
    };
    static void ili9225_plane_helper_atomic_update(struct drm_plane *plane,
    struct drm_atomic_commit *state)
    {
    struct drm_device *drm = plane.dev;
    struct drm_plane_state *plane_state = plane.state;
    struct drm_shadow_plane_state *shadow_plane_state = to_drm_shadow_plane_state(plane_state);
    struct drm_framebuffer *fb = plane_state.fb;
    struct drm_plane_state *old_plane_state = drm_atomic_get_old_plane_state(state, plane);
    struct drm_rect rect;
    int idx;
    if (!plane_state.fb)
    return;
    if (!drm_dev_enter(drm, &idx))
    return;
    if (drm_atomic_helper_damage_merged(old_plane_state, plane_state, &rect))
    ili9225_fb_dirty(&shadow_plane_state.data[0], fb, &rect,
    &shadow_plane_state.fmtcnv_state);
    drm_dev_exit(idx);
    }
    static const struct drm_plane_helper_funcs ili9225_plane_helper_funcs = {
    DRM_GEM_SHADOW_PLANE_HELPER_FUNCS,
    .atomic_check = drm_mipi_dbi_plane_helper_atomic_check,
    .atomic_update = ili9225_plane_helper_atomic_update,
    };
    static const struct drm_plane_funcs ili9225_plane_funcs = {
    DRM_MIPI_DBI_PLANE_FUNCS,
    .destroy = drm_plane_cleanup,
    };
    static void ili9225_crtc_helper_atomic_enable(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    struct drm_device *drm = crtc.dev;
    struct ili9225_device *ili9225 = to_ili9225_device(drm);
    struct mipi_dbi_dev *dbidev = &ili9225.dbidev;
    struct device *dev = drm.dev;
    struct mipi_dbi *dbi = &dbidev.dbi;
    int ret, idx;
    u8 am_id;
    if (!drm_dev_enter(drm, &idx))
    return;
    DRM_DEBUG_KMS("\n");
    mipi_dbi_hw_reset(dbi);
//
// There don't seem to be two example init sequences that match, so
// using the one from the popular Arduino library for this display.
// https://github.com/Nkawu/TFT_22_ILI9225/blob/master/src/TFT_22_ILI9225.cpp
//
    ret = ili9225_command(dbi, ILI9225_POWER_CONTROL_1, 0x0000);
    if (ret) {
    DRM_DEV_ERROR(dev, "Error sending command %d\n", ret);
    goto out_exit;
    }
    ili9225_command(dbi, ILI9225_POWER_CONTROL_2, 0x0000);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_3, 0x0000);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_4, 0x0000);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_5, 0x0000);
    msleep(40);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_2, 0x0018);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_3, 0x6121);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_4, 0x006f);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_5, 0x495f);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_1, 0x0800);
    msleep(10);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_2, 0x103b);
    msleep(50);
    switch (dbidev.rotation) {
    default:
    am_id = 0x30;
    break;
    case 90:
    am_id = 0x18;
    break;
    case 180:
    am_id = 0x00;
    break;
    case 270:
    am_id = 0x28;
    break;
    }
    ili9225_command(dbi, ILI9225_DRIVER_OUTPUT_CONTROL, 0x011c);
    ili9225_command(dbi, ILI9225_LCD_AC_DRIVING_CONTROL, 0x0100);
    ili9225_command(dbi, ILI9225_ENTRY_MODE, 0x1000 | am_id);
    ili9225_command(dbi, ILI9225_DISPLAY_CONTROL_1, 0x0000);
    ili9225_command(dbi, ILI9225_BLANK_PERIOD_CONTROL_1, 0x0808);
    ili9225_command(dbi, ILI9225_FRAME_CYCLE_CONTROL, 0x1100);
    ili9225_command(dbi, ILI9225_INTERFACE_CONTROL, 0x0000);
    ili9225_command(dbi, ILI9225_OSCILLATION_CONTROL, 0x0d01);
    ili9225_command(dbi, ILI9225_VCI_RECYCLING, 0x0020);
    ili9225_command(dbi, ILI9225_RAM_ADDRESS_SET_1, 0x0000);
    ili9225_command(dbi, ILI9225_RAM_ADDRESS_SET_2, 0x0000);
    ili9225_command(dbi, ILI9225_GATE_SCAN_CONTROL, 0x0000);
    ili9225_command(dbi, ILI9225_VERTICAL_SCROLL_1, 0x00db);
    ili9225_command(dbi, ILI9225_VERTICAL_SCROLL_2, 0x0000);
    ili9225_command(dbi, ILI9225_VERTICAL_SCROLL_3, 0x0000);
    ili9225_command(dbi, ILI9225_PARTIAL_DRIVING_POS_1, 0x00db);
    ili9225_command(dbi, ILI9225_PARTIAL_DRIVING_POS_2, 0x0000);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_1, 0x0000);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_2, 0x0808);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_3, 0x080a);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_4, 0x000a);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_5, 0x0a08);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_6, 0x0808);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_7, 0x0000);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_8, 0x0a00);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_9, 0x0710);
    ili9225_command(dbi, ILI9225_GAMMA_CONTROL_10, 0x0710);
    ili9225_command(dbi, ILI9225_DISPLAY_CONTROL_1, 0x0012);
    msleep(50);
    ili9225_command(dbi, ILI9225_DISPLAY_CONTROL_1, 0x1017);
    out_exit:
    drm_dev_exit(idx);
    }
    static void ili9225_crtc_helper_atomic_disable(struct drm_crtc *crtc,
    struct drm_atomic_commit *state)
    {
    struct drm_device *drm = crtc.dev;
    struct ili9225_device *ili9225 = to_ili9225_device(drm);
    struct mipi_dbi_dev *dbidev = &ili9225.dbidev;
    struct mipi_dbi *dbi = &dbidev.dbi;
    DRM_DEBUG_KMS("\n");
//
// This callback is not protected by drm_dev_enter/exit since we want to
// turn off the display on regular driver unload. It's highly unlikely
// that the underlying SPI controller is gone should this be called after
// unplug.
//
    ili9225_command(dbi, ILI9225_DISPLAY_CONTROL_1, 0x0000);
    msleep(50);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_2, 0x0007);
    msleep(50);
    ili9225_command(dbi, ILI9225_POWER_CONTROL_1, 0x0a02);
    }
    static const struct drm_crtc_helper_funcs ili9225_crtc_helper_funcs = {
    .mode_valid = drm_mipi_dbi_crtc_helper_mode_valid,
    .atomic_check = drm_mipi_dbi_crtc_helper_atomic_check,
    .atomic_enable = ili9225_crtc_helper_atomic_enable,
    .atomic_disable = ili9225_crtc_helper_atomic_disable,
    };
    static const struct drm_crtc_funcs ili9225_crtc_funcs = {
    DRM_MIPI_DBI_CRTC_FUNCS,
    .destroy = drm_crtc_cleanup,
    };
    static const struct drm_encoder_funcs ili9225_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
    static const struct drm_connector_helper_funcs ili9225_connector_helper_funcs = {
    DRM_MIPI_DBI_CONNECTOR_HELPER_FUNCS,
    };
    static const struct drm_connector_funcs ili9225_connector_funcs = {
    DRM_MIPI_DBI_CONNECTOR_FUNCS,
    .destroy = drm_connector_cleanup,
    };
    static const struct drm_mode_config_helper_funcs ili9225_mode_config_helper_funcs = {
    DRM_MIPI_DBI_MODE_CONFIG_HELPER_FUNCS,
    };
    static const struct drm_mode_config_funcs ili9225_mode_config_funcs = {
    DRM_MIPI_DBI_MODE_CONFIG_FUNCS,
    };
    static int ili9225_dbi_command(struct mipi_dbi *dbi, u8 *cmd, u8 *par,
    size_t num)
    {
    struct spi_device *spi = dbi.spi;
    let mut bpw: c_uint = 8;
    u32 speed_hz;
    int ret;
    spi_bus_lock(spi.controller);
    gpiod_set_value_cansleep(dbi.dc, 0);
    speed_hz = mipi_dbi_spi_cmd_max_speed(spi, 1);
    ret = mipi_dbi_spi_transfer(spi, speed_hz, 8, cmd, 1);
    spi_bus_unlock(spi.controller);
    if (ret || !num)
    return ret;
    if (*cmd == ILI9225_WRITE_DATA_TO_GRAM && !dbi.swap_bytes)
    bpw = 16;
    spi_bus_lock(spi.controller);
    gpiod_set_value_cansleep(dbi.dc, 1);
    speed_hz = mipi_dbi_spi_cmd_max_speed(spi, num);
    ret = mipi_dbi_spi_transfer(spi, speed_hz, bpw, par, num);
    spi_bus_unlock(spi.controller);
    return ret;
    }
    static const struct drm_display_mode ili9225_mode = {
    DRM_SIMPLE_MODE(176, 220, 35, 44),
    };
    DEFINE_DRM_GEM_DMA_FOPS(ili9225_fops);
    static const struct drm_driver ili9225_driver = {
    .driver_features	= DRIVER_GEM | DRIVER_MODESET | DRIVER_ATOMIC,
    .fops			= &ili9225_fops,
    DRM_GEM_DMA_DRIVER_OPS_VMAP,
    DRM_FBDEV_DMA_DRIVER_OPS,
    .name			= "ili9225",
    .desc			= "Ilitek ILI9225",
    .major			= 1,
    .minor			= 0,
    };
    static const struct of_device_id ili9225_of_match[] = {
    { .compatible = "vot,v220hf01a-t" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ili9225_of_match);
    static const struct spi_device_id ili9225_id[] = {
    { "v220hf01a-t", 0 },
    { },
    };
    MODULE_DEVICE_TABLE(spi, ili9225_id);
#[no_mangle]
unsafe extern "C" fn ili9225_probe(spi: *mut spi_device) -> c_int {
    static int ili9225_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct ili9225_device *ili9225;
    struct mipi_dbi_dev *dbidev;
    struct drm_device *drm;
    struct mipi_dbi *dbi;
    struct gpio_desc *rs;
    struct drm_plane *plane;
    struct drm_crtc *crtc;
    struct drm_encoder *encoder;
    struct drm_connector *connector;
    let mut rotation: u32 = 0;
    int ret;
    ili9225 = devm_drm_dev_alloc(dev, &ili9225_driver, struct ili9225_device, dbidev.drm);
    if (IS_ERR(ili9225))
    return PTR_ERR(ili9225);
    dbidev = &ili9225.dbidev;
    dbi = &dbidev.dbi;
    drm = &dbidev.drm;
    dbi.reset = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(dbi.reset))
    return dev_err_probe(dev, PTR_ERR(dbi.reset), "Failed to get GPIO 'reset'\n");
    rs = devm_gpiod_get(dev, "rs", GPIOD_OUT_LOW);
    if (IS_ERR(rs))
    return dev_err_probe(dev, PTR_ERR(rs), "Failed to get GPIO 'rs'\n");
    device_property_read_u32(dev, "rotation", &rotation);
    ret = mipi_dbi_spi_init(spi, dbi, rs);
    if (ret)
    return ret;
// override the command function set in  mipi_dbi_spi_init()
    dbi.command = ili9225_dbi_command;
    ret = drm_mipi_dbi_dev_init(dbidev, &ili9225_mode, ili9225_plane_formats[0],
    rotation, 0);
    if (ret)
    return ret;
    ret = drmm_mode_config_init(drm);
    if (ret)
    return ret;
    drm.mode_config.min_width = dbidev.mode.hdisplay;
    drm.mode_config.max_width = dbidev.mode.hdisplay;
    drm.mode_config.min_height = dbidev.mode.vdisplay;
    drm.mode_config.max_height = dbidev.mode.vdisplay;
    drm.mode_config.funcs = &ili9225_mode_config_funcs;
    drm.mode_config.preferred_depth = 16;
    drm.mode_config.helper_private = &ili9225_mode_config_helper_funcs;
    plane = &ili9225.plane;
    ret = drm_universal_plane_init(drm, plane, 0, &ili9225_plane_funcs,
    ili9225_plane_formats, ARRAY_SIZE(ili9225_plane_formats),
    ili9225_plane_format_modifiers,
    DRM_PLANE_TYPE_PRIMARY, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_plane_helper_add(plane, &ili9225_plane_helper_funcs);
    drm_plane_enable_fb_damage_clips(plane);
    crtc = &ili9225.crtc;
    ret = drm_crtc_init_with_planes(drm, crtc, plane, core::ptr::null_mut(), &ili9225_crtc_funcs, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_crtc_helper_add(crtc, &ili9225_crtc_helper_funcs);
    encoder = &ili9225.encoder;
    ret = drm_encoder_init(drm, encoder, &ili9225_encoder_funcs, DRM_MODE_ENCODER_NONE, core::ptr::null_mut());
    if (ret)
    return ret;
    encoder.possible_crtcs = drm_crtc_mask(crtc);
    connector = &ili9225.connector;
    ret = drm_connector_init(drm, connector, &ili9225_connector_funcs,
    DRM_MODE_CONNECTOR_SPI);
    if (ret)
    return ret;
    drm_connector_helper_add(connector, &ili9225_connector_helper_funcs);
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
unsafe extern "C" fn ili9225_remove(spi: *mut spi_device) {
    static void ili9225_remove(struct spi_device *spi)
    {
    struct drm_device *drm = spi_get_drvdata(spi);
    drm_dev_unplug(drm);
    drm_atomic_helper_shutdown(drm);
    }
#[no_mangle]
unsafe extern "C" fn ili9225_shutdown(spi: *mut spi_device) {
    static void ili9225_shutdown(struct spi_device *spi)
    {
    drm_atomic_helper_shutdown(spi_get_drvdata(spi));
    }
    static struct spi_driver ili9225_spi_driver = {
    .driver = {
    .name = "ili9225",
    .of_match_table = ili9225_of_match,
    },
    .id_table = ili9225_id,
    .probe = ili9225_probe,
    .remove = ili9225_remove,
    .shutdown = ili9225_shutdown,
    };
    module_spi_driver(ili9225_spi_driver);
    MODULE_DESCRIPTION("Ilitek ILI9225 DRM driver");
    MODULE_AUTHOR("David Lechner <david@lechnology.com>");
    MODULE_LICENSE("GPL");
