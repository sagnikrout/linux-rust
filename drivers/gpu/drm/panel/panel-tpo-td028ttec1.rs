//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-tpo-td028ttec1.c
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
// Toppoly TD028TTEC1 Panel Driver
//
// Copyright (C) 2019 Texas Instruments Incorporated
//
// Based on the omapdrm-specific panel-tpo-td028ttec1 driver
//
// Copyright (C) 2008 Nokia Corporation
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//
// Neo 1973 code (jbt6k74.c):
// Copyright (C) 2006-2007 OpenMoko, Inc.
// Author: Harald Welte <laforge@openmoko.org>
//
// Ported and adapted from Neo 1973 U-Boot by:
// H. Nikolaus Schaller <hns@goldelico.com>
//

pub const JBT_COMMAND: c_uint = 0x000;
pub const JBT_DATA: c_uint = 0x100;
pub const JBT_REG_SLEEP_IN: c_uint = 0x10;
pub const JBT_REG_SLEEP_OUT: c_uint = 0x11;
pub const JBT_REG_DISPLAY_OFF: c_uint = 0x28;
pub const JBT_REG_DISPLAY_ON: c_uint = 0x29;
pub const JBT_REG_RGB_FORMAT: c_uint = 0x3a;
pub const JBT_REG_QUAD_RATE: c_uint = 0x3b;
pub const JBT_REG_POWER_ON_OFF: c_uint = 0xb0;
pub const JBT_REG_BOOSTER_OP: c_uint = 0xb1;
pub const JBT_REG_BOOSTER_MODE: c_uint = 0xb2;
pub const JBT_REG_BOOSTER_FREQ: c_uint = 0xb3;
pub const JBT_REG_OPAMP_SYSCLK: c_uint = 0xb4;
pub const JBT_REG_VSC_VOLTAGE: c_uint = 0xb5;
pub const JBT_REG_VCOM_VOLTAGE: c_uint = 0xb6;
pub const JBT_REG_EXT_DISPL: c_uint = 0xb7;
pub const JBT_REG_OUTPUT_CONTROL: c_uint = 0xb8;
pub const JBT_REG_DCCLK_DCEV: c_uint = 0xb9;
pub const JBT_REG_DISPLAY_MODE1: c_uint = 0xba;
pub const JBT_REG_DISPLAY_MODE2: c_uint = 0xbb;
pub const JBT_REG_DISPLAY_MODE: c_uint = 0xbc;
pub const JBT_REG_ASW_SLEW: c_uint = 0xbd;
pub const JBT_REG_DUMMY_DISPLAY: c_uint = 0xbe;
pub const JBT_REG_DRIVE_SYSTEM: c_uint = 0xbf;
pub const JBT_REG_SLEEP_OUT_FR_A: c_uint = 0xc0;
pub const JBT_REG_SLEEP_OUT_FR_B: c_uint = 0xc1;
pub const JBT_REG_SLEEP_OUT_FR_C: c_uint = 0xc2;
pub const JBT_REG_SLEEP_IN_LCCNT_D: c_uint = 0xc3;
pub const JBT_REG_SLEEP_IN_LCCNT_E: c_uint = 0xc4;
pub const JBT_REG_SLEEP_IN_LCCNT_F: c_uint = 0xc5;
pub const JBT_REG_SLEEP_IN_LCCNT_G: c_uint = 0xc6;
pub const JBT_REG_GAMMA1_FINE_1: c_uint = 0xc7;
pub const JBT_REG_GAMMA1_FINE_2: c_uint = 0xc8;
pub const JBT_REG_GAMMA1_INCLINATION: c_uint = 0xc9;
pub const JBT_REG_GAMMA1_BLUE_OFFSET: c_uint = 0xca;
pub const JBT_REG_BLANK_CONTROL: c_uint = 0xcf;
pub const JBT_REG_BLANK_TH_TV: c_uint = 0xd0;
pub const JBT_REG_CKV_ON_OFF: c_uint = 0xd1;
pub const JBT_REG_CKV_1_2: c_uint = 0xd2;
pub const JBT_REG_OEV_TIMING: c_uint = 0xd3;
pub const JBT_REG_ASW_TIMING_1: c_uint = 0xd4;
pub const JBT_REG_ASW_TIMING_2: c_uint = 0xd5;
pub const JBT_REG_HCLOCK_VGA: c_uint = 0xec;
pub const JBT_REG_HCLOCK_QVGA: c_uint = 0xed;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct td028ttec1_panel {
    pub panel: drm_panel,
    pub spi: *mut spi_device,
}

    static int
    jbt_ret_write_0(struct td028ttec1_panel *lcd, u8 reg, int *err)
    {
    struct spi_device *spi = lcd.spi;
    let mut tx_buf: u16 = JBT_COMMAND | reg;
    int ret;
    if (err && *err)
    return *err;
    ret = spi_write(spi, (u8 *)&tx_buf, sizeof(tx_buf));
    if (ret < 0) {
    dev_err(&spi.dev, "%s: SPI write failed: %d\n", __func__, ret);
    if (err)
// err = ret;
    }
    return ret;
    }
    static int noinline_for_stack
    jbt_reg_write_1(struct td028ttec1_panel *lcd,
    u8 reg, u8 data, int *err)
    {
    struct spi_device *spi = lcd.spi;
    u16 tx_buf[2];
    int ret;
    if (err && *err)
    return *err;
    tx_buf[0] = JBT_COMMAND | reg;
    tx_buf[1] = JBT_DATA | data;
    ret = spi_write(spi, (u8 *)tx_buf, sizeof(tx_buf));
    if (ret < 0) {
    dev_err(&spi.dev, "%s: SPI write failed: %d\n", __func__, ret);
    if (err)
// err = ret;
    }
    return ret;
    }
    static int noinline_for_stack
    jbt_reg_write_2(struct td028ttec1_panel *lcd,
    u8 reg, u16 data, int *err)
    {
    struct spi_device *spi = lcd.spi;
    u16 tx_buf[3];
    int ret;
    if (err && *err)
    return *err;
    tx_buf[0] = JBT_COMMAND | reg;
    tx_buf[1] = JBT_DATA | (data >> 8);
    tx_buf[2] = JBT_DATA | (data & 0xff);
    ret = spi_write(spi, (u8 *)tx_buf, sizeof(tx_buf));
    if (ret < 0) {
    dev_err(&spi.dev, "%s: SPI write failed: %d\n", __func__, ret);
    if (err)
// err = ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn td028ttec1_prepare(panel: *mut drm_panel) -> c_int {
    static int td028ttec1_prepare(struct drm_panel *panel)
    {
    struct td028ttec1_panel *lcd = to_td028ttec1_device(panel);
    unsigned int i;
    let mut ret: c_int = 0;
// Three times command zero
    for (i = 0; i < 3; ++i) {
    jbt_ret_write_0(lcd, 0x00, &ret);
    usleep_range(1000, 2000);
    }
// deep standby out
    jbt_reg_write_1(lcd, JBT_REG_POWER_ON_OFF, 0x17, &ret);
// RGB I/F on, RAM write off, QVGA through, SIGCON enable
    jbt_reg_write_1(lcd, JBT_REG_DISPLAY_MODE, 0x80, &ret);
// Quad mode off
    jbt_reg_write_1(lcd, JBT_REG_QUAD_RATE, 0x00, &ret);
// AVDD on, XVDD on
    jbt_reg_write_1(lcd, JBT_REG_POWER_ON_OFF, 0x16, &ret);
// Output control
    jbt_reg_write_2(lcd, JBT_REG_OUTPUT_CONTROL, 0xfff9, &ret);
// Sleep mode off
    jbt_ret_write_0(lcd, JBT_REG_SLEEP_OUT, &ret);
// at this point we have like 50% grey
// initialize register set
    jbt_reg_write_1(lcd, JBT_REG_DISPLAY_MODE1, 0x01, &ret);
    jbt_reg_write_1(lcd, JBT_REG_DISPLAY_MODE2, 0x00, &ret);
    jbt_reg_write_1(lcd, JBT_REG_RGB_FORMAT, 0x60, &ret);
    jbt_reg_write_1(lcd, JBT_REG_DRIVE_SYSTEM, 0x10, &ret);
    jbt_reg_write_1(lcd, JBT_REG_BOOSTER_OP, 0x56, &ret);
    jbt_reg_write_1(lcd, JBT_REG_BOOSTER_MODE, 0x33, &ret);
    jbt_reg_write_1(lcd, JBT_REG_BOOSTER_FREQ, 0x11, &ret);
    jbt_reg_write_1(lcd, JBT_REG_BOOSTER_FREQ, 0x11, &ret);
    jbt_reg_write_1(lcd, JBT_REG_OPAMP_SYSCLK, 0x02, &ret);
    jbt_reg_write_1(lcd, JBT_REG_VSC_VOLTAGE, 0x2b, &ret);
    jbt_reg_write_1(lcd, JBT_REG_VCOM_VOLTAGE, 0x40, &ret);
    jbt_reg_write_1(lcd, JBT_REG_EXT_DISPL, 0x03, &ret);
    jbt_reg_write_1(lcd, JBT_REG_DCCLK_DCEV, 0x04, &ret);
//
// default of 0x02 in JBT_REG_ASW_SLEW responsible for 72Hz requirement
// to avoid red / blue flicker
//
    jbt_reg_write_1(lcd, JBT_REG_ASW_SLEW, 0x04, &ret);
    jbt_reg_write_1(lcd, JBT_REG_DUMMY_DISPLAY, 0x00, &ret);
    jbt_reg_write_1(lcd, JBT_REG_SLEEP_OUT_FR_A, 0x11, &ret);
    jbt_reg_write_1(lcd, JBT_REG_SLEEP_OUT_FR_B, 0x11, &ret);
    jbt_reg_write_1(lcd, JBT_REG_SLEEP_OUT_FR_C, 0x11, &ret);
    jbt_reg_write_2(lcd, JBT_REG_SLEEP_IN_LCCNT_D, 0x2040, &ret);
    jbt_reg_write_2(lcd, JBT_REG_SLEEP_IN_LCCNT_E, 0x60c0, &ret);
    jbt_reg_write_2(lcd, JBT_REG_SLEEP_IN_LCCNT_F, 0x1020, &ret);
    jbt_reg_write_2(lcd, JBT_REG_SLEEP_IN_LCCNT_G, 0x60c0, &ret);
    jbt_reg_write_2(lcd, JBT_REG_GAMMA1_FINE_1, 0x5533, &ret);
    jbt_reg_write_1(lcd, JBT_REG_GAMMA1_FINE_2, 0x00, &ret);
    jbt_reg_write_1(lcd, JBT_REG_GAMMA1_INCLINATION, 0x00, &ret);
    jbt_reg_write_1(lcd, JBT_REG_GAMMA1_BLUE_OFFSET, 0x00, &ret);
    jbt_reg_write_2(lcd, JBT_REG_HCLOCK_VGA, 0x1f0, &ret);
    jbt_reg_write_1(lcd, JBT_REG_BLANK_CONTROL, 0x02, &ret);
    jbt_reg_write_2(lcd, JBT_REG_BLANK_TH_TV, 0x0804, &ret);
    jbt_reg_write_1(lcd, JBT_REG_CKV_ON_OFF, 0x01, &ret);
    jbt_reg_write_2(lcd, JBT_REG_CKV_1_2, 0x0000, &ret);
    jbt_reg_write_2(lcd, JBT_REG_OEV_TIMING, 0x0d0e, &ret);
    jbt_reg_write_2(lcd, JBT_REG_ASW_TIMING_1, 0x11a4, &ret);
    jbt_reg_write_1(lcd, JBT_REG_ASW_TIMING_2, 0x0e, &ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn td028ttec1_enable(panel: *mut drm_panel) -> c_int {
    static int td028ttec1_enable(struct drm_panel *panel)
    {
    struct td028ttec1_panel *lcd = to_td028ttec1_device(panel);
    return jbt_ret_write_0(lcd, JBT_REG_DISPLAY_ON, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn td028ttec1_disable(panel: *mut drm_panel) -> c_int {
    static int td028ttec1_disable(struct drm_panel *panel)
    {
    struct td028ttec1_panel *lcd = to_td028ttec1_device(panel);
    jbt_ret_write_0(lcd, JBT_REG_DISPLAY_OFF, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn td028ttec1_unprepare(panel: *mut drm_panel) -> c_int {
    static int td028ttec1_unprepare(struct drm_panel *panel)
    {
    struct td028ttec1_panel *lcd = to_td028ttec1_device(panel);
    jbt_reg_write_2(lcd, JBT_REG_OUTPUT_CONTROL, 0x8002, core::ptr::null_mut());
    jbt_ret_write_0(lcd, JBT_REG_SLEEP_IN, core::ptr::null_mut());
    jbt_reg_write_1(lcd, JBT_REG_POWER_ON_OFF, 0x00, core::ptr::null_mut());
    return 0;
    }
    static const struct drm_display_mode td028ttec1_mode = {
    .clock = 22153,
    .hdisplay = 480,
    .hsync_start = 480 + 24,
    .hsync_end = 480 + 24 + 8,
    .htotal = 480 + 24 + 8 + 8,
    .vdisplay = 640,
    .vsync_start = 640 + 4,
    .vsync_end = 640 + 4 + 2,
    .vtotal = 640 + 4 + 2 + 2,
    .type = DRM_MODE_TYPE_DRIVER | DRM_MODE_TYPE_PREFERRED,
    .flags = DRM_MODE_FLAG_NHSYNC | DRM_MODE_FLAG_NVSYNC,
    .width_mm = 43,
    .height_mm = 58,
    };
    static int td028ttec1_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev, &td028ttec1_mode);
    if (!mode)
    return -ENOMEM;
    drm_mode_set_name(mode);
    drm_mode_probed_add(connector, mode);
    connector.display_info.width_mm = td028ttec1_mode.width_mm;
    connector.display_info.height_mm = td028ttec1_mode.height_mm;
//
// FIXME: According to the datasheet sync signals are sampled on the
// rising edge of the clock, but the code running on the OpenMoko Neo
// FreeRunner and Neo 1973 indicates sampling on the falling edge. This
// should be tested on a real device.
//
    connector.display_info.bus_flags = DRM_BUS_FLAG_DE_HIGH
    | DRM_BUS_FLAG_SYNC_SAMPLE_NEGEDGE
    | DRM_BUS_FLAG_PIXDATA_SAMPLE_POSEDGE;
    return 1;
    }
    static const struct drm_panel_funcs td028ttec1_funcs = {
    .prepare = td028ttec1_prepare,
    .enable = td028ttec1_enable,
    .disable = td028ttec1_disable,
    .unprepare = td028ttec1_unprepare,
    .get_modes = td028ttec1_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn td028ttec1_probe(spi: *mut spi_device) -> c_int {
    static int td028ttec1_probe(struct spi_device *spi)
    {
    struct td028ttec1_panel *lcd;
    int ret;
    lcd = devm_drm_panel_alloc(&spi.dev, struct td028ttec1_panel, panel,
    &td028ttec1_funcs,
    DRM_MODE_CONNECTOR_DPI);
    if (IS_ERR(lcd))
    return PTR_ERR(lcd);
    spi_set_drvdata(spi, lcd);
    lcd.spi = spi;
    spi.mode = SPI_MODE_3;
    spi.bits_per_word = 9;
    ret = spi_setup(spi);
    if (ret < 0) {
    dev_err(&spi.dev, "failed to setup SPI: %d\n", ret);
    return ret;
    }
    ret = drm_panel_of_backlight(&lcd.panel);
    if (ret)
    return ret;
    drm_panel_add(&lcd.panel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn td028ttec1_remove(spi: *mut spi_device) {
    static void td028ttec1_remove(struct spi_device *spi)
    {
    struct td028ttec1_panel *lcd = spi_get_drvdata(spi);
    drm_panel_remove(&lcd.panel);
    drm_panel_disable(&lcd.panel);
    drm_panel_unprepare(&lcd.panel);
    }
    static const struct of_device_id td028ttec1_of_match[] = {
    { .compatible = "tpo,td028ttec1", },
// DT backward compatibility.
    { .compatible = "toppoly,td028ttec1", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, td028ttec1_of_match);
    static const struct spi_device_id td028ttec1_ids[] = {
    { "td028ttec1", 0 },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(spi, td028ttec1_ids);
    static struct spi_driver td028ttec1_driver = {
    .probe		= td028ttec1_probe,
    .remove		= td028ttec1_remove,
    .id_table	= td028ttec1_ids,
    .driver		= {
    .name   = "panel-tpo-td028ttec1",
    .of_match_table = td028ttec1_of_match,
    },
    };
    module_spi_driver(td028ttec1_driver);
    MODULE_AUTHOR("H. Nikolaus Schaller <hns@goldelico.com>");
    MODULE_DESCRIPTION("Toppoly TD028TTEC1 panel driver");
    MODULE_LICENSE("GPL");
