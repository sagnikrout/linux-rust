//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-novatek-nt35950.c
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
// Novatek NT35950 DriverIC panels driver
//
// Copyright (c) 2021 AngeloGioacchino Del Regno
// <angelogioacchino.delregno@somainline.org>
//

pub const MCS_CMD_MAUCCTR: c_uint = 0xf0 /* Manufacturer command enable */;
pub const MCS_PARAM_SCALER_FUNCTION: c_uint = 0x58 /* Scale-up function */;
pub const MCS_PARAM_SCALEUP_MODE: c_uint = 0xc9;
pub const MCS_SCALEUP_SIMPLE: c_uint = 0x0;

// VESA Display Stream Compression param
pub const MCS_PARAM_VESA_DSC_ON: c_uint = 0x03;
// Data Compression mode
pub const MCS_PARAM_DATA_COMPRESSION: c_uint = 0x90;
pub const MCS_DATA_COMPRESSION_NONE: c_uint = 0x00;
pub const MCS_DATA_COMPRESSION_FBC: c_uint = 0x02;
pub const MCS_DATA_COMPRESSION_DSC: c_uint = 0x03;
// Display Output control
pub const MCS_PARAM_DISP_OUTPUT_CTRL: c_uint = 0xb4;

// VESA Display Stream Compression setting
pub const MCS_PARAM_VESA_DSC_SETTING: c_uint = 0xc0;
// SubPixel Rendering (SPR)
pub const MCS_PARAM_SPR_EN: c_uint = 0xe3;
pub const MCS_PARAM_SPR_MODE: c_uint = 0xef;
pub const MCS_SPR_MODE_YYG_RAINBOW_RGB: c_uint = 0x01;
pub const NT35950_VREG_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nt35950 {
    pub panel: drm_panel,
    pub connector: *mut drm_connector,
    pub dsi: [*mut mipi_dsi_device; 2],
    pub vregs: [regulator_bulk_data; NT35950_VREG_MAX],
    pub reset_gpio: *mut gpio_desc,
    pub desc: *const nt35950_panel_desc,
    pub cur_mode: c_int,
    pub last_page: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nt35950_panel_mode {
    pub mode: drm_display_mode,
    pub enable_sram: bool,
    pub is_video_mode: bool,
    pub scaler_on: u8,
    pub scaler_mode: u8,
    pub compression: u8,
    pub spr_en: u8,
    pub spr_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nt35950_panel_desc {
    pub model_name: *const c_char,
    pub dsi_info: mipi_dsi_device_info,
    pub mode_data: *const nt35950_panel_mode,
    pub is_dual_dsi: bool,
    pub num_lanes: u8,
    pub num_modes: u8,
}

    static inline struct nt35950 *to_nt35950(struct drm_panel *panel)
    {
    return container_of(panel, struct nt35950, panel);
    }
#[no_mangle]
unsafe extern "C" fn nt35950_reset(nt: *mut nt35950) {
    static void nt35950_reset(struct nt35950 *nt)
    {
    gpiod_set_value_cansleep(nt.reset_gpio, 1);
    usleep_range(12000, 13000);
    gpiod_set_value_cansleep(nt.reset_gpio, 0);
    usleep_range(300, 400);
    gpiod_set_value_cansleep(nt.reset_gpio, 1);
    usleep_range(12000, 13000);
    }
//
// nt35950_set_cmd2_page - Select manufacturer control (CMD2) page
// @dsi_ctx: context for mipi_dsi functions
// @nt:   Main driver structure
// @page: Page number (0-7)
//
    static void nt35950_set_cmd2_page(struct mipi_dsi_multi_context *dsi_ctx,
    struct nt35950 *nt, u8 page)
    {
    const u8 mauc_cmd2_page[] = { MCS_CMD_MAUCCTR, 0x55, 0xaa, 0x52,
    0x08, page };
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, mauc_cmd2_page,
    ARRAY_SIZE(mauc_cmd2_page));
    if (!dsi_ctx.accum_err)
    nt.last_page = page;
    }
//
// nt35950_set_data_compression - Set data compression mode
// @dsi_ctx: context for mipi_dsi functions
// @nt:        Main driver structure
// @comp_mode: Compression mode
//
    static void nt35950_set_data_compression(struct mipi_dsi_multi_context *dsi_ctx,
    struct nt35950 *nt, u8 comp_mode)
    {
    u8 cmd_data_compression[] = { MCS_PARAM_DATA_COMPRESSION, comp_mode };
    u8 cmd_vesa_dsc_on[] = { MCS_PARAM_VESA_DSC_ON, !!comp_mode };
    u8 cmd_vesa_dsc_setting[] = { MCS_PARAM_VESA_DSC_SETTING, 0x03 };
    let mut last_page: u8 = nt.last_page;
// Set CMD2 Page 0 if we're not there yet
    if (last_page != 0)
    nt35950_set_cmd2_page(dsi_ctx, nt, 0);
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd_data_compression,
    ARRAY_SIZE(cmd_data_compression));
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd_vesa_dsc_on,
    ARRAY_SIZE(cmd_vesa_dsc_on));
// Set the vesa dsc setting on Page 4
    nt35950_set_cmd2_page(dsi_ctx, nt, 4);
// Display Stream Compression setting, always 0x03
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd_vesa_dsc_setting,
    ARRAY_SIZE(cmd_vesa_dsc_setting));
// Get back to the previously set page
    nt35950_set_cmd2_page(dsi_ctx, nt, last_page);
    }
//
// nt35950_set_scaler - Enable/disable resolution upscaling
// @dsi_ctx: context for mipi_dsi functions
// @scale_up:  Scale up function control
//
    static void nt35950_set_scaler(struct mipi_dsi_multi_context *dsi_ctx,
    u8 scale_up)
    {
    u8 cmd_scaler[] = { MCS_PARAM_SCALER_FUNCTION, scale_up };
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd_scaler,
    ARRAY_SIZE(cmd_scaler));
    }
//
// nt35950_set_scale_mode - Resolution upscaling mode
// @dsi_ctx: context for mipi_dsi functions
// @mode: Scaler mode (MCS_DATA_COMPRESSION_*)
//
    static void nt35950_set_scale_mode(struct mipi_dsi_multi_context *dsi_ctx,
    u8 mode)
    {
    u8 cmd_scaler[] = { MCS_PARAM_SCALEUP_MODE, mode };
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd_scaler,
    ARRAY_SIZE(cmd_scaler));
    }
//
// nt35950_inject_black_image - Display a completely black image
// @dsi_ctx: context for mipi_dsi functions
//
// After IC setup, the attached panel may show random data
// due to driveric behavior changes (resolution, compression,
// scaling, etc). This function, called after parameters setup,
// makes the driver ic to output a completely black image to
// the display.
// It makes sense to push a black image before sending the sleep-out
// and display-on commands.
//
#[no_mangle]
unsafe extern "C" fn nt35950_inject_black_image(dsi_ctx: *mut mipi_dsi_multi_context) {
    static void nt35950_inject_black_image(struct mipi_dsi_multi_context *dsi_ctx)
    {
    const u8 cmd0_black_img[] = { 0x6f, 0x01 };
    const u8 cmd1_black_img[] = { 0xf3, 0x10 };
    u8 cmd_test[] = { 0xff, 0xaa, 0x55, 0xa5, 0x80 };
// Enable test command
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd_test, ARRAY_SIZE(cmd_test));
// Send a black image
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd0_black_img,
    ARRAY_SIZE(cmd0_black_img));
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd1_black_img,
    ARRAY_SIZE(cmd1_black_img));
// Disable test command
    cmd_test[ARRAY_SIZE(cmd_test) - 1] = 0x00;
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd_test, ARRAY_SIZE(cmd_test));
    }
//
// nt35950_set_dispout - Set Display Output register parameters
// @nt:    Main driver structure
// @dsi_ctx: context for mipi_dsi functions
//
    static void nt35950_set_dispout(struct mipi_dsi_multi_context *dsi_ctx,
    struct nt35950 *nt)
    {
    u8 cmd_dispout[] = { MCS_PARAM_DISP_OUTPUT_CTRL, 0x00 };
    const struct nt35950_panel_mode *mode_data = nt.desc.mode_data;
    if (mode_data[nt.cur_mode].is_video_mode)
    cmd_dispout[1] |= MCS_DISP_OUT_VIDEO_MODE;
    if (mode_data[nt.cur_mode].enable_sram)
    cmd_dispout[1] |= MCS_DISP_OUT_SRAM_EN;
    mipi_dsi_dcs_write_buffer_multi(dsi_ctx, cmd_dispout,
    ARRAY_SIZE(cmd_dispout));
    }
#[no_mangle]
unsafe extern "C" fn nt35950_get_current_mode(nt: *mut nt35950) -> c_int {
    static int nt35950_get_current_mode(struct nt35950 *nt)
    {
    struct drm_connector *connector = nt.connector;
    struct drm_crtc_state *crtc_state;
    int i;
// Return the default (first) mode if no info available yet
    if (!connector.state || !connector.state.crtc)
    return 0;
    crtc_state = connector.state.crtc.state;
    for (i = 0; i < nt.desc.num_modes; i++) {
    if (drm_mode_match(&crtc_state.mode,
    &nt.desc.mode_data[i].mode,
    DRM_MODE_MATCH_TIMINGS | DRM_MODE_MATCH_CLOCK))
    return i;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nt35950_on(nt: *mut nt35950) -> c_int {
    static int nt35950_on(struct nt35950 *nt)
    {
    const struct nt35950_panel_mode *mode_data = nt.desc.mode_data;
    struct mipi_dsi_device *dsi = nt.dsi[0];
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    nt.cur_mode = nt35950_get_current_mode(nt);
    nt.dsi[0].mode_flags |= MIPI_DSI_MODE_LPM;
    nt.dsi[1].mode_flags |= MIPI_DSI_MODE_LPM;
    nt35950_set_cmd2_page(&dsi_ctx, nt, 0);
    nt35950_set_data_compression(&dsi_ctx, nt, mode_data[nt.cur_mode].compression);
    nt35950_set_scale_mode(&dsi_ctx, mode_data[nt.cur_mode].scaler_mode);
    nt35950_set_scaler(&dsi_ctx, mode_data[nt.cur_mode].scaler_on);
    nt35950_set_dispout(&dsi_ctx, nt);
    mipi_dsi_dcs_set_tear_on_multi(&dsi_ctx, MIPI_DSI_DCS_TEAR_MODE_VBLANK);
    mipi_dsi_dcs_set_tear_scanline_multi(&dsi_ctx, 0);
// CMD2 Page 1
    nt35950_set_cmd2_page(&dsi_ctx, nt, 1);
// Unknown command
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, 0xd4, 0x88, 0x88);
// CMD2 Page 7
    nt35950_set_cmd2_page(&dsi_ctx, nt, 7);
// Enable SubPixel Rendering
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, MCS_PARAM_SPR_EN, 0x01);
// SPR Mode: YYG Rainbow-RGB
    mipi_dsi_dcs_write_seq_multi(&dsi_ctx, MCS_PARAM_SPR_MODE,
    MCS_SPR_MODE_YYG_RAINBOW_RGB);
// CMD3
    nt35950_inject_black_image(&dsi_ctx);
    mipi_dsi_dcs_exit_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 120);
    mipi_dsi_dcs_set_display_on_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 120);
    if (dsi_ctx.accum_err)
    return dsi_ctx.accum_err;
    nt.dsi[0].mode_flags &= ~MIPI_DSI_MODE_LPM;
    nt.dsi[1].mode_flags &= ~MIPI_DSI_MODE_LPM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nt35950_off(nt: *mut nt35950) {
    static void nt35950_off(struct nt35950 *nt)
    {
    struct mipi_dsi_device *dsi = nt.dsi[0];
    let mut dsi_ctx: mipi_dsi_multi_context = { .dsi = dsi };
    mipi_dsi_dcs_set_display_off_multi(&dsi_ctx);
    mipi_dsi_usleep_range(&dsi_ctx, 10000, 11000);
    mipi_dsi_dcs_enter_sleep_mode_multi(&dsi_ctx);
    mipi_dsi_msleep(&dsi_ctx, 150);
    nt.dsi[0].mode_flags |= MIPI_DSI_MODE_LPM;
    nt.dsi[1].mode_flags |= MIPI_DSI_MODE_LPM;
    }
#[no_mangle]
unsafe extern "C" fn nt35950_sharp_init_vregs(nt: *mut nt35950, dev: *mut device) -> c_int {
    static int nt35950_sharp_init_vregs(struct nt35950 *nt, struct device *dev)
    {
    int ret;
    nt.vregs[0].supply = "vddio";
    nt.vregs[1].supply = "avdd";
    nt.vregs[2].supply = "avee";
    nt.vregs[3].supply = "dvdd";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(nt.vregs),
    nt.vregs);
    if (ret < 0)
    return ret;
    ret = regulator_is_supported_voltage(nt.vregs[0].consumer,
    1750000, 1950000);
    if (!ret)
    return -EINVAL;
    ret = regulator_is_supported_voltage(nt.vregs[1].consumer,
    5200000, 5900000);
    if (!ret)
    return -EINVAL;
// AVEE is negative: -5.90V to -5.20V
    ret = regulator_is_supported_voltage(nt.vregs[2].consumer,
    5200000, 5900000);
    if (!ret)
    return -EINVAL;
    ret = regulator_is_supported_voltage(nt.vregs[3].consumer,
    1300000, 1400000);
    if (!ret)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nt35950_prepare(panel: *mut drm_panel) -> c_int {
    static int nt35950_prepare(struct drm_panel *panel)
    {
    struct nt35950 *nt = to_nt35950(panel);
    int ret;
    ret = regulator_enable(nt.vregs[0].consumer);
    if (ret)
    return ret;
    usleep_range(2000, 5000);
    ret = regulator_enable(nt.vregs[3].consumer);
    if (ret)
    goto end;
    usleep_range(15000, 18000);
    ret = regulator_enable(nt.vregs[1].consumer);
    if (ret)
    goto end;
    ret = regulator_enable(nt.vregs[2].consumer);
    if (ret)
    goto end;
    usleep_range(12000, 13000);
    nt35950_reset(nt);
    ret = nt35950_on(nt);
    end:
    if (ret < 0) {
    regulator_bulk_disable(ARRAY_SIZE(nt.vregs), nt.vregs);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nt35950_unprepare(panel: *mut drm_panel) -> c_int {
    static int nt35950_unprepare(struct drm_panel *panel)
    {
    struct nt35950 *nt = to_nt35950(panel);
    nt35950_off(nt);
    gpiod_set_value_cansleep(nt.reset_gpio, 0);
    regulator_bulk_disable(ARRAY_SIZE(nt.vregs), nt.vregs);
    return 0;
    }
    static int nt35950_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct nt35950 *nt = to_nt35950(panel);
    int i;
    for (i = 0; i < nt.desc.num_modes; i++) {
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev,
    &nt.desc.mode_data[i].mode);
    if (!mode)
    return -ENOMEM;
    drm_mode_set_name(mode);
    mode.type |= DRM_MODE_TYPE_DRIVER;
    if (nt.desc.num_modes == 1)
    mode.type |= DRM_MODE_TYPE_PREFERRED;
    drm_mode_probed_add(connector, mode);
    }
    connector.display_info.bpc = 8;
    connector.display_info.height_mm = nt.desc.mode_data[0].mode.height_mm;
    connector.display_info.width_mm = nt.desc.mode_data[0].mode.width_mm;
    nt.connector = connector;
    return nt.desc.num_modes;
    }
    static const struct drm_panel_funcs nt35950_panel_funcs = {
    .prepare = nt35950_prepare,
    .unprepare = nt35950_unprepare,
    .get_modes = nt35950_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn nt35950_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int nt35950_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct device_node *dsi_r;
    struct mipi_dsi_host *dsi_r_host;
    struct nt35950 *nt;
    const struct mipi_dsi_device_info *info;
    int i, num_dsis = 1, ret;
    nt = devm_drm_panel_alloc(dev, struct nt35950, panel, &nt35950_panel_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(nt))
    return PTR_ERR(nt);
    ret = nt35950_sharp_init_vregs(nt, dev);
    if (ret)
    return dev_err_probe(dev, ret, "Regulator init failure.\n");
    nt.desc = of_device_get_match_data(dev);
    if (!nt.desc)
    return -ENODEV;
    nt.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_ASIS);
    if (IS_ERR(nt.reset_gpio)) {
    return dev_err_probe(dev, PTR_ERR(nt.reset_gpio),
    "Failed to get reset gpio\n");
    }
// If the panel is connected on two DSIs then DSI0 left, DSI1 right
    if (nt.desc.is_dual_dsi) {
    info = &nt.desc.dsi_info;
    dsi_r = of_graph_get_remote_node(dsi.dev.of_node, 1, -1);
    if (!dsi_r) {
    dev_err(dev, "Cannot get secondary DSI node.\n");
    return -ENODEV;
    }
    dsi_r_host = of_find_mipi_dsi_host_by_node(dsi_r);
    of_node_put(dsi_r);
    if (!dsi_r_host)
    return dev_err_probe(dev, -EPROBE_DEFER, "Cannot get secondary DSI host\n");
    nt.dsi[1] = mipi_dsi_device_register_full(dsi_r_host, info);
    if (IS_ERR(nt.dsi[1])) {
    dev_err(dev, "Cannot get secondary DSI node\n");
    return PTR_ERR(nt.dsi[1]);
    }
    num_dsis++;
    }
    nt.dsi[0] = dsi;
    mipi_dsi_set_drvdata(dsi, nt);
    ret = drm_panel_of_backlight(&nt.panel);
    if (ret) {
    if (num_dsis == 2)
    mipi_dsi_device_unregister(nt.dsi[1]);
    return dev_err_probe(dev, ret, "Failed to get backlight\n");
    }
    drm_panel_add(&nt.panel);
    for (i = 0; i < num_dsis; i++) {
    nt.dsi[i].lanes = nt.desc.num_lanes;
    nt.dsi[i].format = MIPI_DSI_FMT_RGB888;
    nt.dsi[i].mode_flags = MIPI_DSI_CLOCK_NON_CONTINUOUS |
    MIPI_DSI_MODE_LPM;
    if (nt.desc.mode_data[0].is_video_mode)
    nt.dsi[i].mode_flags |= MIPI_DSI_MODE_VIDEO;
    ret = mipi_dsi_attach(nt.dsi[i]);
    if (ret < 0) {
// If we fail to attach to either host, we're done
    if (num_dsis == 2)
    mipi_dsi_device_unregister(nt.dsi[1]);
    return dev_err_probe(dev, ret,
    "Cannot attach to DSI%d host.\n", i);
    }
    }
// Make sure to set RESX LOW before starting the power-on sequence
    gpiod_set_value_cansleep(nt.reset_gpio, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nt35950_remove(dsi: *mut mipi_dsi_device) {
    static void nt35950_remove(struct mipi_dsi_device *dsi)
    {
    struct nt35950 *nt = mipi_dsi_get_drvdata(dsi);
    int ret;
    ret = mipi_dsi_detach(nt.dsi[0]);
    if (ret < 0)
    dev_err(&dsi.dev,
    "Failed to detach from DSI0 host: %d\n", ret);
    if (nt.dsi[1]) {
    ret = mipi_dsi_detach(nt.dsi[1]);
    if (ret < 0)
    dev_err(&dsi.dev,
    "Failed to detach from DSI1 host: %d\n", ret);
    mipi_dsi_device_unregister(nt.dsi[1]);
    }
    drm_panel_remove(&nt.panel);
    }
    static const struct nt35950_panel_mode sharp_ls055d1sx04_modes[] = {
    {
// 1920x1080 60Hz no compression
    .mode = {
    .clock = 214537,
    .hdisplay = 1080,
    .hsync_start = 1080 + 400,
    .hsync_end = 1080 + 400 + 40,
    .htotal = 1080 + 400 + 40 + 300,
    .vdisplay = 1920,
    .vsync_start = 1920 + 12,
    .vsync_end = 1920 + 12 + 2,
    .vtotal = 1920 + 12 + 2 + 10,
    .width_mm = 68,
    .height_mm = 121,
    },
    .compression = MCS_DATA_COMPRESSION_NONE,
    .enable_sram = true,
    .is_video_mode = false,
    .scaler_on = 1,
    .scaler_mode = MCS_SCALEUP_DUPLICATE,
    },
// TODO: Add 2160x3840 60Hz when DSC is supported
    };
    static const struct nt35950_panel_desc sharp_ls055d1sx04 = {
    .model_name = "Sharp LS055D1SX04",
    .dsi_info = {
    .type = "LS055D1SX04",
    .channel = 0,
    .node = core::ptr::null_mut(),
    },
    .mode_data = sharp_ls055d1sx04_modes,
    .num_modes = ARRAY_SIZE(sharp_ls055d1sx04_modes),
    .is_dual_dsi = true,
    .num_lanes = 4,
    };
    static const struct of_device_id nt35950_of_match[] = {
    { .compatible = "sharp,ls055d1sx04", .data = &sharp_ls055d1sx04 },
    {  }
    };
    MODULE_DEVICE_TABLE(of, nt35950_of_match);
    static struct mipi_dsi_driver nt35950_driver = {
    .probe = nt35950_probe,
    .remove = nt35950_remove,
    .driver = {
    .name = "panel-novatek-nt35950",
    .of_match_table = nt35950_of_match,
    },
    };
    module_mipi_dsi_driver(nt35950_driver);
    MODULE_AUTHOR("AngeloGioacchino Del Regno <angelogioacchino.delregno@somainline.org>");
    MODULE_DESCRIPTION("Novatek NT35950 DriverIC panels driver");
    MODULE_LICENSE("GPL v2");
