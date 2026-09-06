//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-raspberrypi-touchscreen.c
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


//
// Copyright © 2016-2017 Broadcom
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// Portions of this file (derived from panel-simple.c) are:
//
// Copyright (C) 2013, NVIDIA Corporation.  All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sub license,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Raspberry Pi 7" touchscreen panel driver.
//
// The 7" touchscreen consists of a DPI LCD panel, a Toshiba
// TC358762XBG DSI-DPI bridge, and an I2C-connected Atmel ATTINY88-MUR
// controlling power management, the LCD PWM, and initial register
// setup of the Tohsiba.
//
// This driver controls the TC358762 and ATTINY88, presenting a DSI
// device with a drm_panel.
//

// I2C registers of the Atmel microcontroller.
    enum REG_ADDR {
    REG_ID = 0x80,
    REG_PORTA, /* BIT(2) for horizontal flip, BIT(3) for vertical flip */
    REG_PORTB,
    REG_PORTC,
    REG_PORTD,
    REG_POWERON,
    REG_PWM,
    REG_DDRA,
    REG_DDRB,
    REG_DDRC,
    REG_DDRD,
    REG_TEST,
    REG_WR_ADDRL,
    REG_WR_ADDRH,
    REG_READH,
    REG_READL,
    REG_WRITEH,
    REG_WRITEL,
    REG_ID2,
    };
// DSI D-PHY Layer Registers
pub const D0W_DPHYCONTTX: c_uint = 0x0004;
pub const CLW_DPHYCONTRX: c_uint = 0x0020;
pub const D0W_DPHYCONTRX: c_uint = 0x0024;
pub const D1W_DPHYCONTRX: c_uint = 0x0028;
pub const COM_DPHYCONTRX: c_uint = 0x0038;
pub const CLW_CNTRL: c_uint = 0x0040;
pub const D0W_CNTRL: c_uint = 0x0044;
pub const D1W_CNTRL: c_uint = 0x0048;
pub const DFTMODE_CNTRL: c_uint = 0x0054;
// DSI PPI Layer Registers
pub const PPI_STARTPPI: c_uint = 0x0104;
pub const PPI_BUSYPPI: c_uint = 0x0108;
pub const PPI_LINEINITCNT: c_uint = 0x0110;
pub const PPI_LPTXTIMECNT: c_uint = 0x0114;
pub const PPI_CLS_ATMR: c_uint = 0x0140;
pub const PPI_D0S_ATMR: c_uint = 0x0144;
pub const PPI_D1S_ATMR: c_uint = 0x0148;
pub const PPI_D0S_CLRSIPOCOUNT: c_uint = 0x0164;
pub const PPI_D1S_CLRSIPOCOUNT: c_uint = 0x0168;
pub const CLS_PRE: c_uint = 0x0180;
pub const D0S_PRE: c_uint = 0x0184;
pub const D1S_PRE: c_uint = 0x0188;
pub const CLS_PREP: c_uint = 0x01A0;
pub const D0S_PREP: c_uint = 0x01A4;
pub const D1S_PREP: c_uint = 0x01A8;
pub const CLS_ZERO: c_uint = 0x01C0;
pub const D0S_ZERO: c_uint = 0x01C4;
pub const D1S_ZERO: c_uint = 0x01C8;
pub const PPI_CLRFLG: c_uint = 0x01E0;
pub const PPI_CLRSIPO: c_uint = 0x01E4;
pub const HSTIMEOUT: c_uint = 0x01F0;
pub const HSTIMEOUTENABLE: c_uint = 0x01F4;
// DSI Protocol Layer Registers
pub const DSI_STARTDSI: c_uint = 0x0204;
pub const DSI_BUSYDSI: c_uint = 0x0208;
pub const DSI_LANEENABLE: c_uint = 0x0210;

pub const DSI_LANESTATUS0: c_uint = 0x0214;
pub const DSI_LANESTATUS1: c_uint = 0x0218;
pub const DSI_INTSTATUS: c_uint = 0x0220;
pub const DSI_INTMASK: c_uint = 0x0224;
pub const DSI_INTCLR: c_uint = 0x0228;
pub const DSI_LPTXTO: c_uint = 0x0230;
pub const DSI_MODE: c_uint = 0x0260;
pub const DSI_PAYLOAD0: c_uint = 0x0268;
pub const DSI_PAYLOAD1: c_uint = 0x026C;
pub const DSI_SHORTPKTDAT: c_uint = 0x0270;
pub const DSI_SHORTPKTREQ: c_uint = 0x0274;
pub const DSI_BTASTA: c_uint = 0x0278;
pub const DSI_BTACLR: c_uint = 0x027C;
// DSI General Registers
pub const DSIERRCNT: c_uint = 0x0300;
pub const DSISIGMOD: c_uint = 0x0304;
// DSI Application Layer Registers
pub const APLCTRL: c_uint = 0x0400;
pub const APLSTAT: c_uint = 0x0404;
pub const APLERR: c_uint = 0x0408;
pub const PWRMOD: c_uint = 0x040C;
pub const RDPKTLN: c_uint = 0x0410;
pub const PXLFMT: c_uint = 0x0414;
pub const MEMWRCMD: c_uint = 0x0418;
// LCDC/DPI Host Registers
pub const LCDCTRL: c_uint = 0x0420;
pub const HSR: c_uint = 0x0424;
pub const HDISPR: c_uint = 0x0428;
pub const VSR: c_uint = 0x042C;
pub const VDISPR: c_uint = 0x0430;
pub const VFUEN: c_uint = 0x0434;
// DBI-B Host Registers
pub const DBIBCTRL: c_uint = 0x0440;
// SPI Master Registers
pub const SPICMR: c_uint = 0x0450;
pub const SPITCR: c_uint = 0x0454;
// System Controller Registers
pub const SYSSTAT: c_uint = 0x0460;
pub const SYSCTRL: c_uint = 0x0464;
pub const SYSPLL1: c_uint = 0x0468;
pub const SYSPLL2: c_uint = 0x046C;
pub const SYSPLL3: c_uint = 0x0470;
pub const SYSPMCTRL: c_uint = 0x047C;
// GPIO Registers
pub const GPIOC: c_uint = 0x0480;
pub const GPIOO: c_uint = 0x0484;
pub const GPIOI: c_uint = 0x0488;
// I2C Registers
pub const I2CCLKCTRL: c_uint = 0x0490;
// Chip/Rev Registers
pub const IDREG: c_uint = 0x04A0;
// Debug Registers
pub const WCMDQUEUE: c_uint = 0x0500;
pub const RCMDQUEUE: c_uint = 0x0504;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_touchscreen {
    pub base: drm_panel,
    pub dsi: *mut mipi_dsi_device,
    pub i2c: *mut i2c_client,
}

    static const struct drm_display_mode rpi_touchscreen_modes[] = {
    {
// Modeline comes from the Raspberry Pi firmware, with HFP=1
// plugged in and clock re-computed from that.
//
    .clock = 25979400 / 1000,
    .hdisplay = 800,
    .hsync_start = 800 + 1,
    .hsync_end = 800 + 1 + 2,
    .htotal = 800 + 1 + 2 + 46,
    .vdisplay = 480,
    .vsync_start = 480 + 7,
    .vsync_end = 480 + 7 + 2,
    .vtotal = 480 + 7 + 2 + 21,
    },
    };
    static struct rpi_touchscreen *panel_to_ts(struct drm_panel *panel)
    {
    return container_of(panel, struct rpi_touchscreen, base);
    }
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_i2c_read(ts: *mut rpi_touchscreen, reg: u8) -> c_int {
    static int rpi_touchscreen_i2c_read(struct rpi_touchscreen *ts, u8 reg)
    {
    return i2c_smbus_read_byte_data(ts.i2c, reg);
    }
    static void rpi_touchscreen_i2c_write(struct rpi_touchscreen *ts,
    u8 reg, u8 val)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(ts.i2c, reg, val);
    if (ret)
    dev_err(&ts.i2c.dev, "I2C write failed: %d\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_write(ts: *mut rpi_touchscreen, reg: u16, val: u32) -> c_int {
    static int rpi_touchscreen_write(struct rpi_touchscreen *ts, u16 reg, u32 val)
    {
    u8 msg[] = {
    reg,
    reg >> 8,
    val,
    val >> 8,
    val >> 16,
    val >> 24,
    };
    mipi_dsi_generic_write(ts.dsi, msg, sizeof(msg));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_disable(panel: *mut drm_panel) -> c_int {
    static int rpi_touchscreen_disable(struct drm_panel *panel)
    {
    struct rpi_touchscreen *ts = panel_to_ts(panel);
    rpi_touchscreen_i2c_write(ts, REG_PWM, 0);
    rpi_touchscreen_i2c_write(ts, REG_POWERON, 0);
    udelay(1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_noop(panel: *mut drm_panel) -> c_int {
    static int rpi_touchscreen_noop(struct drm_panel *panel)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_prepare(panel: *mut drm_panel) -> c_int {
    static int rpi_touchscreen_prepare(struct drm_panel *panel)
    {
    struct rpi_touchscreen *ts = panel_to_ts(panel);
    int i;
    rpi_touchscreen_i2c_write(ts, REG_POWERON, 1);
// Wait for nPWRDWN to go low to indicate poweron is done.
    for (i = 0; i < 100; i++) {
    if (rpi_touchscreen_i2c_read(ts, REG_PORTB) & 1)
    break;
    }
    rpi_touchscreen_write(ts, DSI_LANEENABLE,
    DSI_LANEENABLE_CLOCK |
    DSI_LANEENABLE_D0);
    rpi_touchscreen_write(ts, PPI_D0S_CLRSIPOCOUNT, 0x05);
    rpi_touchscreen_write(ts, PPI_D1S_CLRSIPOCOUNT, 0x05);
    rpi_touchscreen_write(ts, PPI_D0S_ATMR, 0x00);
    rpi_touchscreen_write(ts, PPI_D1S_ATMR, 0x00);
    rpi_touchscreen_write(ts, PPI_LPTXTIMECNT, 0x03);
    rpi_touchscreen_write(ts, SPICMR, 0x00);
    rpi_touchscreen_write(ts, LCDCTRL, 0x00100150);
    rpi_touchscreen_write(ts, SYSCTRL, 0x040f);
    msleep(100);
    rpi_touchscreen_write(ts, PPI_STARTPPI, 0x01);
    rpi_touchscreen_write(ts, DSI_STARTDSI, 0x01);
    msleep(100);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_enable(panel: *mut drm_panel) -> c_int {
    static int rpi_touchscreen_enable(struct drm_panel *panel)
    {
    struct rpi_touchscreen *ts = panel_to_ts(panel);
// Turn on the backlight.
    rpi_touchscreen_i2c_write(ts, REG_PWM, 255);
// Default to the same orientation as the closed source
// firmware used for the panel.  Runtime rotation
// configuration will be supported using VC4's plane
// orientation bits.
//
    rpi_touchscreen_i2c_write(ts, REG_PORTA, BIT(2));
    return 0;
    }
    static int rpi_touchscreen_get_modes(struct drm_panel *panel,
    struct drm_connector *connector)
    {
    unsigned int i, num = 0;
    let mut bus_format: static u32 = MEDIA_BUS_FMT_RGB888_1X24;
    for (i = 0; i < ARRAY_SIZE(rpi_touchscreen_modes); i++) {
    const struct drm_display_mode *m = &rpi_touchscreen_modes[i];
    struct drm_display_mode *mode;
    mode = drm_mode_duplicate(connector.dev, m);
    if (!mode) {
    dev_err(panel.dev, "failed to add mode %ux%u@%u\n",
    m.hdisplay, m.vdisplay,
    drm_mode_vrefresh(m));
    continue;
    }
    mode.type |= DRM_MODE_TYPE_DRIVER;
    if (i == 0)
    mode.type |= DRM_MODE_TYPE_PREFERRED;
    drm_mode_set_name(mode);
    drm_mode_probed_add(connector, mode);
    num++;
    }
    connector.display_info.bpc = 8;
    connector.display_info.width_mm = 154;
    connector.display_info.height_mm = 86;
    drm_display_info_set_bus_formats(&connector.display_info,
    &bus_format, 1);
    return num;
    }
    static const struct drm_panel_funcs rpi_touchscreen_funcs = {
    .disable = rpi_touchscreen_disable,
    .unprepare = rpi_touchscreen_noop,
    .prepare = rpi_touchscreen_prepare,
    .enable = rpi_touchscreen_enable,
    .get_modes = rpi_touchscreen_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_probe(i2c: *mut i2c_client) -> c_int {
    static int rpi_touchscreen_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct rpi_touchscreen *ts;
    struct device_node *endpoint, *dsi_host_node;
    struct mipi_dsi_host *host;
    int ver;
    struct mipi_dsi_device_info info = {
    .type = RPI_DSI_DRIVER_NAME,
    .channel = 0,
    .node = core::ptr::null_mut(),
    };
    ts = devm_drm_panel_alloc(dev, __typeof(*ts), base,
    &rpi_touchscreen_funcs,
    DRM_MODE_CONNECTOR_DSI);
    if (IS_ERR(ts))
    return PTR_ERR(ts);
    i2c_set_clientdata(i2c, ts);
    ts.i2c = i2c;
    ver = rpi_touchscreen_i2c_read(ts, REG_ID);
    if (ver < 0) {
    dev_err(dev, "Atmel I2C read failed: %d\n", ver);
    return -ENODEV;
    }
    switch (ver) {
    case 0xde: /* ver 1 */
    case 0xc3: /* ver 2 */
    break;
    default:
    dev_err(dev, "Unknown Atmel firmware revision: 0x%02x\n", ver);
    return -ENODEV;
    }
// Turn off at boot, so we can cleanly sequence powering on.
    rpi_touchscreen_i2c_write(ts, REG_POWERON, 0);
// Look up the DSI host.  It needs to probe before we do.
    endpoint = of_graph_get_endpoint_by_regs(dev.of_node, 0, -1);
    if (!endpoint)
    return -ENODEV;
    dsi_host_node = of_graph_get_remote_port_parent(endpoint);
    if (!dsi_host_node)
    goto error;
    host = of_find_mipi_dsi_host_by_node(dsi_host_node);
    of_node_put(dsi_host_node);
    if (!host) {
    of_node_put(endpoint);
    return -EPROBE_DEFER;
    }
    info.node = of_graph_get_remote_port(endpoint);
    if (!info.node)
    goto error;
    of_node_put(endpoint);
    ts.dsi = mipi_dsi_device_register_full(host, &info);
    if (IS_ERR(ts.dsi)) {
    dev_err(dev, "DSI device registration failed: %ld\n",
    PTR_ERR(ts.dsi));
    return PTR_ERR(ts.dsi);
    }
// This appears last, as it's what will unblock the DSI host
// driver's component bind function.
//
    drm_panel_add(&ts.base);
    return 0;
    error:
    of_node_put(endpoint);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_remove(i2c: *mut i2c_client) {
    static void rpi_touchscreen_remove(struct i2c_client *i2c)
    {
    struct rpi_touchscreen *ts = i2c_get_clientdata(i2c);
    mipi_dsi_detach(ts.dsi);
    drm_panel_remove(&ts.base);
    mipi_dsi_device_unregister(ts.dsi);
    }
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_dsi_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int rpi_touchscreen_dsi_probe(struct mipi_dsi_device *dsi)
    {
    int ret;
    dsi.mode_flags = (MIPI_DSI_MODE_VIDEO |
    MIPI_DSI_MODE_VIDEO_SYNC_PULSE |
    MIPI_DSI_MODE_LPM);
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.lanes = 1;
    ret = mipi_dsi_attach(dsi);
    if (ret)
    dev_err(&dsi.dev, "failed to attach dsi to host: %d\n", ret);
    return ret;
    }
    static struct mipi_dsi_driver rpi_touchscreen_dsi_driver = {
    .driver.name = RPI_DSI_DRIVER_NAME,
    .probe = rpi_touchscreen_dsi_probe,
    };
    static const struct of_device_id rpi_touchscreen_of_ids[] = {
    { .compatible = "raspberrypi,7inch-touchscreen-panel" },
    { } /* sentinel */
    };
    MODULE_DEVICE_TABLE(of, rpi_touchscreen_of_ids);
    static struct i2c_driver rpi_touchscreen_driver = {
    .driver = {
    .name = "rpi_touchscreen",
    .of_match_table = rpi_touchscreen_of_ids,
    },
    .probe = rpi_touchscreen_probe,
    .remove = rpi_touchscreen_remove,
    };
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_init() -> int __init {
    static int __init rpi_touchscreen_init(void)
    {
    mipi_dsi_driver_register(&rpi_touchscreen_dsi_driver);
    return i2c_add_driver(&rpi_touchscreen_driver);
    }
    module_init(rpi_touchscreen_init);
#[no_mangle]
unsafe extern "C" fn rpi_touchscreen_exit() -> void __exit {
    static void __exit rpi_touchscreen_exit(void)
    {
    i2c_del_driver(&rpi_touchscreen_driver);
    mipi_dsi_driver_unregister(&rpi_touchscreen_dsi_driver);
    }
    module_exit(rpi_touchscreen_exit);
    MODULE_AUTHOR("Eric Anholt <eric@anholt.net>");
    MODULE_DESCRIPTION("Raspberry Pi 7-inch touchscreen driver");
    MODULE_LICENSE("GPL v2");
