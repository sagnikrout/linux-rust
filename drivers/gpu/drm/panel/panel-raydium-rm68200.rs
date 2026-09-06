//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-raydium-rm68200.c
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
// Copyright (C) STMicroelectronics SA 2017
//
// Authors: Philippe Cornu <philippe.cornu@st.com>
// Yannick Fertre <yannick.fertre@st.com>
//

// Manufacturer Command Set
pub const MCS_CMD_MODE_SW: c_uint = 0xFE /* CMD Mode Switch */;
pub const MCS_CMD1_UCS: c_uint = 0x00 /* User Command Set (UCS = CMD1) */;
pub const MCS_CMD2_P0: c_uint = 0x01 /* Manufacture Command Set Page0 (CMD2 P0) */;
pub const MCS_CMD2_P1: c_uint = 0x02 /* Manufacture Command Set Page1 (CMD2 P1) */;
pub const MCS_CMD2_P2: c_uint = 0x03 /* Manufacture Command Set Page2 (CMD2 P2) */;
pub const MCS_CMD2_P3: c_uint = 0x04 /* Manufacture Command Set Page3 (CMD2 P3) */;
// CMD2 P0 commands (Display Options and Power)
pub const MCS_STBCTR: c_uint = 0x12 /* TE1 Output Setting Zig-Zag Connection */;
pub const MCS_SGOPCTR: c_uint = 0x16 /* Source Bias Current */;
pub const MCS_SDCTR: c_uint = 0x1A /* Source Output Delay Time */;
pub const MCS_INVCTR: c_uint = 0x1B /* Inversion Type */;
pub const MCS_EXT_PWR_IC: c_uint = 0x24 /* External PWR IC Control */;
pub const MCS_SETAVDD: c_uint = 0x27 /* PFM Control for AVDD Output */;
pub const MCS_SETAVEE: c_uint = 0x29 /* PFM Control for AVEE Output */;
pub const MCS_BT2CTR: c_uint = 0x2B /* DDVDL Charge Pump Control */;
pub const MCS_BT3CTR: c_uint = 0x2F /* VGH Charge Pump Control */;
pub const MCS_BT4CTR: c_uint = 0x34 /* VGL Charge Pump Control */;
pub const MCS_VCMCTR: c_uint = 0x46 /* VCOM Output Level Control */;
pub const MCS_SETVGN: c_uint = 0x52 /* VG M/S N Control */;
pub const MCS_SETVGP: c_uint = 0x54 /* VG M/S P Control */;
pub const MCS_SW_CTRL: c_uint = 0x5F /* Interface Control for PFM and MIPI */;
// CMD2 P2 commands (GOA Timing Control) - no description in datasheet
pub const GOA_VSTV1: c_uint = 0x00;
pub const GOA_VSTV2: c_uint = 0x07;
pub const GOA_VCLK1: c_uint = 0x0E;
pub const GOA_VCLK2: c_uint = 0x17;
pub const GOA_VCLK_OPT1: c_uint = 0x20;
pub const GOA_BICLK1: c_uint = 0x2A;
pub const GOA_BICLK2: c_uint = 0x37;
pub const GOA_BICLK3: c_uint = 0x44;
pub const GOA_BICLK4: c_uint = 0x4F;
pub const GOA_BICLK_OPT1: c_uint = 0x5B;
pub const GOA_BICLK_OPT2: c_uint = 0x60;
pub const MCS_GOA_GPO1: c_uint = 0x6D;
pub const MCS_GOA_GPO2: c_uint = 0x71;
pub const MCS_GOA_EQ: c_uint = 0x74;
pub const MCS_GOA_CLK_GALLON: c_uint = 0x7C;
pub const MCS_GOA_FS_SEL0: c_uint = 0x7E;
pub const MCS_GOA_FS_SEL1: c_uint = 0x87;
pub const MCS_GOA_FS_SEL2: c_uint = 0x91;
pub const MCS_GOA_FS_SEL3: c_uint = 0x9B;
pub const MCS_GOA_BS_SEL0: c_uint = 0xAC;
pub const MCS_GOA_BS_SEL1: c_uint = 0xB5;
pub const MCS_GOA_BS_SEL2: c_uint = 0xBF;
pub const MCS_GOA_BS_SEL3: c_uint = 0xC9;
pub const MCS_GOA_BS_SEL4: c_uint = 0xD3;
// CMD2 P3 commands (Gamma)
pub const MCS_GAMMA_VP: c_uint = 0x60 /* Gamma VP1~VP16 */;
pub const MCS_GAMMA_VN: c_uint = 0x70 /* Gamma VN1~VN16 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rm68200 {
    pub dev: *mut device,
    pub panel: drm_panel,
    pub reset_gpio: *mut gpio_desc,
    pub supply: *mut regulator,
}

    static const struct drm_display_mode default_mode = {
    .clock = 54000,
    .hdisplay = 720,
    .hsync_start = 720 + 48,
    .hsync_end = 720 + 48 + 9,
    .htotal = 720 + 48 + 9 + 48,
    .vdisplay = 1280,
    .vsync_start = 1280 + 12,
    .vsync_end = 1280 + 12 + 5,
    .vtotal = 1280 + 12 + 5 + 12,
    .flags = 0,
    .width_mm = 68,
    .height_mm = 122,
    };
    static inline struct rm68200 *panel_to_rm68200(struct drm_panel *panel)
    {
    return container_of(panel, struct rm68200, panel);
    }
    static void rm68200_dcs_write_buf(struct rm68200 *ctx, const void *data,
    size_t len)
    {
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(ctx.dev);
    int err;
    err = mipi_dsi_dcs_write_buffer(dsi, data, len);
    if (err < 0)
    dev_err_ratelimited(ctx.dev, "MIPI DSI DCS write buffer failed: %d\n", err);
    }
#[no_mangle]
unsafe extern "C" fn rm68200_dcs_write_cmd(ctx: *mut rm68200, cmd: u8, value: u8) {
    static void rm68200_dcs_write_cmd(struct rm68200 *ctx, u8 cmd, u8 value)
    {
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(ctx.dev);
    int err;
    err = mipi_dsi_dcs_write(dsi, cmd, &value, 1);
    if (err < 0)
    dev_err_ratelimited(ctx.dev, "MIPI DSI DCS write failed: %d\n", err);
    }

    ({								\
    static const u8 d[] = { seq };				\
    \
    rm68200_dcs_write_buf(ctx, d, ARRAY_SIZE(d));		\
    })
//
// This panel is not able to auto-increment all cmd addresses so for some of
// them, we need to send them one by one...
//

    ({								\
    static const u8 d[] = { seq };				\
    unsigned int i;						\
    \
    for (i = 0; i < ARRAY_SIZE(d) ; i++)			\
    rm68200_dcs_write_cmd(ctx, cmd + i, d[i]);	\
    })
#[no_mangle]
unsafe extern "C" fn rm68200_init_sequence(ctx: *mut rm68200) {
    static void rm68200_init_sequence(struct rm68200 *ctx)
    {
// Enter CMD2 with page 0
    dcs_write_seq(ctx, MCS_CMD_MODE_SW, MCS_CMD2_P0);
    dcs_write_cmd_seq(ctx, MCS_EXT_PWR_IC, 0xC0, 0x53, 0x00);
    dcs_write_seq(ctx, MCS_BT2CTR, 0xE5);
    dcs_write_seq(ctx, MCS_SETAVDD, 0x0A);
    dcs_write_seq(ctx, MCS_SETAVEE, 0x0A);
    dcs_write_seq(ctx, MCS_SGOPCTR, 0x52);
    dcs_write_seq(ctx, MCS_BT3CTR, 0x53);
    dcs_write_seq(ctx, MCS_BT4CTR, 0x5A);
    dcs_write_seq(ctx, MCS_INVCTR, 0x00);
    dcs_write_seq(ctx, MCS_STBCTR, 0x0A);
    dcs_write_seq(ctx, MCS_SDCTR, 0x06);
    dcs_write_seq(ctx, MCS_VCMCTR, 0x56);
    dcs_write_seq(ctx, MCS_SETVGN, 0xA0, 0x00);
    dcs_write_seq(ctx, MCS_SETVGP, 0xA0, 0x00);
    dcs_write_seq(ctx, MCS_SW_CTRL, 0x11); /* 2 data lanes, see doc */
    dcs_write_seq(ctx, MCS_CMD_MODE_SW, MCS_CMD2_P2);
    dcs_write_seq(ctx, GOA_VSTV1, 0x05);
    dcs_write_seq(ctx, 0x02, 0x0B);
    dcs_write_seq(ctx, 0x03, 0x0F);
    dcs_write_seq(ctx, 0x04, 0x7D, 0x00, 0x50);
    dcs_write_cmd_seq(ctx, GOA_VSTV2, 0x05, 0x16, 0x0D, 0x11, 0x7D, 0x00,
    0x50);
    dcs_write_cmd_seq(ctx, GOA_VCLK1, 0x07, 0x08, 0x01, 0x02, 0x00, 0x7D,
    0x00, 0x85, 0x08);
    dcs_write_cmd_seq(ctx, GOA_VCLK2, 0x03, 0x04, 0x05, 0x06, 0x00, 0x7D,
    0x00, 0x85, 0x08);
    dcs_write_seq(ctx, GOA_VCLK_OPT1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00);
    dcs_write_cmd_seq(ctx, GOA_BICLK1, 0x07, 0x08);
    dcs_write_seq(ctx, 0x2D, 0x01);
    dcs_write_seq(ctx, 0x2F, 0x02, 0x00, 0x40, 0x05, 0x08, 0x54, 0x7D,
    0x00);
    dcs_write_cmd_seq(ctx, GOA_BICLK2, 0x03, 0x04, 0x05, 0x06, 0x00);
    dcs_write_seq(ctx, 0x3D, 0x40);
    dcs_write_seq(ctx, 0x3F, 0x05, 0x08, 0x54, 0x7D, 0x00);
    dcs_write_seq(ctx, GOA_BICLK3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00);
    dcs_write_seq(ctx, GOA_BICLK4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00);
    dcs_write_seq(ctx, 0x58, 0x00, 0x00, 0x00);
    dcs_write_seq(ctx, GOA_BICLK_OPT1, 0x00, 0x00, 0x00, 0x00, 0x00);
    dcs_write_seq(ctx, GOA_BICLK_OPT2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    dcs_write_seq(ctx, MCS_GOA_GPO1, 0x00, 0x00, 0x00, 0x00);
    dcs_write_seq(ctx, MCS_GOA_GPO2, 0x00, 0x20, 0x00);
    dcs_write_seq(ctx, MCS_GOA_EQ, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
    0x00, 0x00);
    dcs_write_seq(ctx, MCS_GOA_CLK_GALLON, 0x00, 0x00);
    dcs_write_cmd_seq(ctx, MCS_GOA_FS_SEL0, 0xBF, 0x02, 0x06, 0x14, 0x10,
    0x16, 0x12, 0x08, 0x3F);
    dcs_write_cmd_seq(ctx, MCS_GOA_FS_SEL1, 0x3F, 0x3F, 0x3F, 0x3F, 0x0C,
    0x0A, 0x0E, 0x3F, 0x3F, 0x00);
    dcs_write_cmd_seq(ctx, MCS_GOA_FS_SEL2, 0x04, 0x3F, 0x3F, 0x3F, 0x3F,
    0x05, 0x01, 0x3F, 0x3F, 0x0F);
    dcs_write_cmd_seq(ctx, MCS_GOA_FS_SEL3, 0x0B, 0x0D, 0x3F, 0x3F, 0x3F,
    0x3F);
    dcs_write_cmd_seq(ctx, 0xA2, 0x3F, 0x09, 0x13, 0x17, 0x11, 0x15);
    dcs_write_cmd_seq(ctx, 0xA9, 0x07, 0x03, 0x3F);
    dcs_write_cmd_seq(ctx, MCS_GOA_BS_SEL0, 0x3F, 0x05, 0x01, 0x17, 0x13,
    0x15, 0x11, 0x0F, 0x3F);
    dcs_write_cmd_seq(ctx, MCS_GOA_BS_SEL1, 0x3F, 0x3F, 0x3F, 0x3F, 0x0B,
    0x0D, 0x09, 0x3F, 0x3F, 0x07);
    dcs_write_cmd_seq(ctx, MCS_GOA_BS_SEL2, 0x03, 0x3F, 0x3F, 0x3F, 0x3F,
    0x02, 0x06, 0x3F, 0x3F, 0x08);
    dcs_write_cmd_seq(ctx, MCS_GOA_BS_SEL3, 0x0C, 0x0A, 0x3F, 0x3F, 0x3F,
    0x3F, 0x3F, 0x0E, 0x10, 0x14);
    dcs_write_cmd_seq(ctx, MCS_GOA_BS_SEL4, 0x12, 0x16, 0x00, 0x04, 0x3F);
    dcs_write_seq(ctx, 0xDC, 0x02);
    dcs_write_seq(ctx, 0xDE, 0x12);
    dcs_write_seq(ctx, MCS_CMD_MODE_SW, 0x0E); /* No documentation */
    dcs_write_seq(ctx, 0x01, 0x75);
    dcs_write_seq(ctx, MCS_CMD_MODE_SW, MCS_CMD2_P3);
    dcs_write_cmd_seq(ctx, MCS_GAMMA_VP, 0x00, 0x0C, 0x12, 0x0E, 0x06,
    0x12, 0x0E, 0x0B, 0x15, 0x0B, 0x10, 0x07, 0x0F,
    0x12, 0x0C, 0x00);
    dcs_write_cmd_seq(ctx, MCS_GAMMA_VN, 0x00, 0x0C, 0x12, 0x0E, 0x06,
    0x12, 0x0E, 0x0B, 0x15, 0x0B, 0x10, 0x07, 0x0F,
    0x12, 0x0C, 0x00);
// Exit CMD2
    dcs_write_seq(ctx, MCS_CMD_MODE_SW, MCS_CMD1_UCS);
    }
#[no_mangle]
unsafe extern "C" fn rm68200_unprepare(panel: *mut drm_panel) -> c_int {
    static int rm68200_unprepare(struct drm_panel *panel)
    {
    struct rm68200 *ctx = panel_to_rm68200(panel);
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(ctx.dev);
    int ret;
    ret = mipi_dsi_dcs_set_display_off(dsi);
    if (ret)
    dev_warn(panel.dev, "failed to set display off: %d\n", ret);
    ret = mipi_dsi_dcs_enter_sleep_mode(dsi);
    if (ret)
    dev_warn(panel.dev, "failed to enter sleep mode: %d\n", ret);
    msleep(120);
    if (ctx.reset_gpio) {
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    msleep(20);
    }
    regulator_disable(ctx.supply);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rm68200_prepare(panel: *mut drm_panel) -> c_int {
    static int rm68200_prepare(struct drm_panel *panel)
    {
    struct rm68200 *ctx = panel_to_rm68200(panel);
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(ctx.dev);
    int ret;
    ret = regulator_enable(ctx.supply);
    if (ret < 0) {
    dev_err(ctx.dev, "failed to enable supply: %d\n", ret);
    return ret;
    }
    if (ctx.reset_gpio) {
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    msleep(20);
    gpiod_set_value_cansleep(ctx.reset_gpio, 0);
    msleep(100);
    }
    rm68200_init_sequence(ctx);
    ret = mipi_dsi_dcs_exit_sleep_mode(dsi);
    if (ret)
    return ret;
    msleep(125);
    ret = mipi_dsi_dcs_set_display_on(dsi);
    if (ret)
    return ret;
    msleep(20);
    return 0;
    }
    static int rm68200_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev, &default_mode);
    if (!mode) {
    dev_err(panel.dev, "failed to add mode %ux%u@%u\n",
    default_mode.hdisplay, default_mode.vdisplay,
    drm_mode_vrefresh(&default_mode));
    return -ENOMEM;
    }
    drm_mode_set_name(mode);
    mode.type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED;
    drm_mode_probed_add(connector, mode);
    connector.display_info.width_mm = mode.width_mm;
    connector.display_info.height_mm = mode.height_mm;
    return 1;
    }
    static const struct drm_panel_funcs rm68200_drm_funcs = {
    .unprepare = rm68200_unprepare,
    .prepare = rm68200_prepare,
    .get_modes = rm68200_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn rm68200_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int rm68200_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct rm68200 *ctx;
    int ret;
    ctx = devm_drm_panel_alloc(dev, struct rm68200, panel,
    &rm68200_drm_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    ctx.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.reset_gpio)) {
    ret = PTR_ERR(ctx.reset_gpio);
    dev_err(dev, "cannot get reset GPIO: %d\n", ret);
    return ret;
    }
    ctx.supply = devm_regulator_get(dev, "power");
    if (IS_ERR(ctx.supply)) {
    ret = PTR_ERR(ctx.supply);
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "cannot get regulator: %d\n", ret);
    return ret;
    }
    mipi_dsi_set_drvdata(dsi, ctx);
    ctx.dev = dev;
    dsi.lanes = 2;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_MODE_LPM | MIPI_DSI_CLOCK_NON_CONTINUOUS;
    ret = drm_panel_of_backlight(&ctx.panel);
    if (ret)
    return ret;
    drm_panel_add(&ctx.panel);
    ret = mipi_dsi_attach(dsi);
    if (ret < 0) {
    dev_err(dev, "mipi_dsi_attach() failed: %d\n", ret);
    drm_panel_remove(&ctx.panel);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rm68200_remove(dsi: *mut mipi_dsi_device) {
    static void rm68200_remove(struct mipi_dsi_device *dsi)
    {
    struct rm68200 *ctx = mipi_dsi_get_drvdata(dsi);
    mipi_dsi_detach(dsi);
    drm_panel_remove(&ctx.panel);
    }
    static const struct of_device_id raydium_rm68200_of_match[] = {
    { .compatible = "raydium,rm68200" },
    { }
    };
    MODULE_DEVICE_TABLE(of, raydium_rm68200_of_match);
    static struct mipi_dsi_driver raydium_rm68200_driver = {
    .probe = rm68200_probe,
    .remove = rm68200_remove,
    .driver = {
    .name = "panel-raydium-rm68200",
    .of_match_table = raydium_rm68200_of_match,
    },
    };
    module_mipi_dsi_driver(raydium_rm68200_driver);
    MODULE_AUTHOR("Philippe Cornu <philippe.cornu@st.com>");
    MODULE_AUTHOR("Yannick Fertre <yannick.fertre@st.com>");
    MODULE_DESCRIPTION("DRM Driver for Raydium RM68200 MIPI DSI panel");
    MODULE_LICENSE("GPL v2");
