//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/panel/panel-samsung-s6e63m0-dsi.c
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
// DSI interface to the Samsung S6E63M0 panel.
// (C) 2019 Linus Walleij
//

pub const MCS_GLOBAL_PARAM: c_uint = 0xb0;

    static int s6e63m0_dsi_dcs_read(struct device *dev, void *trsp,
    const u8 cmd, u8 *data)
    {
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(dev);
    int ret;
    ret = mipi_dsi_dcs_read(dsi, cmd, data, 1);
    if (ret < 0) {
    dev_err(dev, "could not read DCS CMD %02x\n", cmd);
    return ret;
    }
    dev_dbg(dev, "DSI read CMD %02x = %02x\n", cmd, *data);
    return 0;
    }
    static int s6e63m0_dsi_dcs_write(struct device *dev, void *trsp,
    const u8 *data, size_t len)
    {
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(dev);
    const u8 *seqp = data;
    u8 cmd;
    u8 cmdwritten;
    int remain;
    int chunk;
    int ret;
    dev_dbg(dev, "DSI writing dcs seq: %*ph\n", (int)len, data);
// Pick out and skip past the DCS command
    cmd = *seqp;
    seqp++;
    cmdwritten = 0;
    remain = len - 1;
    chunk = remain;
// Send max S6E63M0_DSI_MAX_CHUNK bytes at a time
    if (chunk > S6E63M0_DSI_MAX_CHUNK)
    chunk = S6E63M0_DSI_MAX_CHUNK;
    ret = mipi_dsi_dcs_write(dsi, cmd, seqp, chunk);
    if (ret < 0) {
    dev_err(dev, "error sending DCS command seq cmd %02x\n", cmd);
    return ret;
    }
    cmdwritten += chunk;
    seqp += chunk;
    while (cmdwritten < remain) {
    chunk = remain - cmdwritten;
    if (chunk > S6E63M0_DSI_MAX_CHUNK)
    chunk = S6E63M0_DSI_MAX_CHUNK;
    ret = mipi_dsi_dcs_write(dsi, MCS_GLOBAL_PARAM, &cmdwritten, 1);
    if (ret < 0) {
    dev_err(dev, "error sending CMD %02x global param %02x\n",
    cmd, cmdwritten);
    return ret;
    }
    ret = mipi_dsi_dcs_write(dsi, cmd, seqp, chunk);
    if (ret < 0) {
    dev_err(dev, "error sending CMD %02x chunk\n", cmd);
    return ret;
    }
    cmdwritten += chunk;
    seqp += chunk;
    }
    dev_dbg(dev, "sent command %02x %02x bytes\n", cmd, cmdwritten);
    usleep_range(8000, 9000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s6e63m0_dsi_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int s6e63m0_dsi_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    int ret;
    dsi.lanes = 2;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.hs_rate = 349440000;
    dsi.lp_rate = 9600000;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO |
    MIPI_DSI_MODE_VIDEO_BURST;
    ret = s6e63m0_probe(dev, core::ptr::null_mut(), s6e63m0_dsi_dcs_read,
    s6e63m0_dsi_dcs_write, true);
    if (ret)
    return ret;
    ret = mipi_dsi_attach(dsi);
    if (ret < 0)
    s6e63m0_remove(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s6e63m0_dsi_remove(dsi: *mut mipi_dsi_device) {
    static void s6e63m0_dsi_remove(struct mipi_dsi_device *dsi)
    {
    mipi_dsi_detach(dsi);
    s6e63m0_remove(&dsi.dev);
    }
    static const struct of_device_id s6e63m0_dsi_of_match[] = {
    { .compatible = "samsung,s6e63m0" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, s6e63m0_dsi_of_match);
    static struct mipi_dsi_driver s6e63m0_dsi_driver = {
    .probe			= s6e63m0_dsi_probe,
    .remove			= s6e63m0_dsi_remove,
    .driver			= {
    .name		= "panel-samsung-s6e63m0",
    .of_match_table = s6e63m0_dsi_of_match,
    },
    };
    module_mipi_dsi_driver(s6e63m0_dsi_driver);
    MODULE_AUTHOR("Linus Walleij <linusw@kernel.org>");
    MODULE_DESCRIPTION("s6e63m0 LCD DSI Driver");
    MODULE_LICENSE("GPL v2");
