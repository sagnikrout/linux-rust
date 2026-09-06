//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dc/dc-fg.c
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
// Copyright 2024 NXP
//

pub const FGSTCTRL: c_uint = 0x8;

pub const HTCFG1: c_uint = 0xc;

pub const HTCFG2: c_uint = 0x10;

pub const VTCFG1: c_uint = 0x14;

pub const VTCFG2: c_uint = 0x18;

pub const PKICKCONFIG: c_uint = 0x2c;
pub const SKICKCONFIG: c_uint = 0x30;

pub const PACFG: c_uint = 0x54;
pub const SACFG: c_uint = 0x58;

pub const FGINCTRL: c_uint = 0x5c;
pub const FGINCTRLPANIC: c_uint = 0x60;

pub const FGCCR: c_uint = 0x64;

pub const FGENABLE: c_uint = 0x68;

pub const FGSLR: c_uint = 0x6c;

pub const FGTIMESTAMP: c_uint = 0x74;

pub const FGCHSTAT: c_uint = 0x78;

pub const FGCHSTATCLR: c_uint = 0x7c;

    enum dc_fg_syncmode {
    FG_SYNCMODE_OFF,	/* No side-by-side synchronization. */
    };
    enum dc_fg_dm {
    FG_DM_CONSTCOL = 0x1,	/* Constant Color Background is shown. */
    FG_DM_SEC_ON_TOP = 0x5,	/* Both inputs overlaid with secondary on top. */
    };
    static const struct dc_subdev_info dc_fg_info[] = {
    { .reg_start = 0x5618b800, .id = 0, },
    { .reg_start = 0x5618d400, .id = 1, },
    };
    static const struct regmap_range dc_fg_regmap_write_ranges[] = {
    regmap_reg_range(FGSTCTRL, VTCFG2),
    regmap_reg_range(PKICKCONFIG, SKICKCONFIG),
    regmap_reg_range(PACFG, FGSLR),
    regmap_reg_range(FGCHSTATCLR, FGCHSTATCLR),
    };
    static const struct regmap_range dc_fg_regmap_read_ranges[] = {
    regmap_reg_range(FGSTCTRL, VTCFG2),
    regmap_reg_range(PKICKCONFIG, SKICKCONFIG),
    regmap_reg_range(PACFG, FGENABLE),
    regmap_reg_range(FGTIMESTAMP, FGCHSTAT),
    };
    static const struct regmap_access_table dc_fg_regmap_write_table = {
    .yes_ranges = dc_fg_regmap_write_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_fg_regmap_write_ranges),
    };
    static const struct regmap_access_table dc_fg_regmap_read_table = {
    .yes_ranges = dc_fg_regmap_read_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_fg_regmap_read_ranges),
    };
    static const struct regmap_config dc_fg_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .fast_io = true,
    .wr_table = &dc_fg_regmap_write_table,
    .rd_table = &dc_fg_regmap_read_table,
    .max_register = FGCHSTATCLR,
    };
#[no_mangle]
pub unsafe extern "C" fn dc_fg_enable_shden(fg: *mut dc_fg) {
    static inline void dc_fg_enable_shden(struct dc_fg *fg)
    {
    regmap_write_bits(fg.reg, FGSTCTRL, SHDEN, SHDEN);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_syncmode(fg: *mut dc_fg, mode: enum dc_fg_syncmode) {
    static inline void dc_fg_syncmode(struct dc_fg *fg, enum dc_fg_syncmode mode)
    {
    regmap_write_bits(fg.reg, FGSTCTRL, FGSYNCMODE_MASK, FGSYNCMODE(mode));
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_cfg_videomode(fg: *mut dc_fg, m: *mut drm_display_mode) {
    void dc_fg_cfg_videomode(struct dc_fg *fg, struct drm_display_mode *m)
    {
    u32 hact, htotal, hsync, hsbp;
    u32 vact, vtotal, vsync, vsbp;
    u32 kick_row, kick_col;
    int ret;
    hact = m.crtc_hdisplay;
    htotal = m.crtc_htotal;
    hsync = m.crtc_hsync_end - m.crtc_hsync_start;
    hsbp = m.crtc_htotal - m.crtc_hsync_start;
    vact = m.crtc_vdisplay;
    vtotal = m.crtc_vtotal;
    vsync = m.crtc_vsync_end - m.crtc_vsync_start;
    vsbp = m.crtc_vtotal - m.crtc_vsync_start;
// video mode
    regmap_write(fg.reg, HTCFG1, HACT(hact)   | HTOTAL(htotal));
    regmap_write(fg.reg, HTCFG2, HSYNC(hsync) | HSBP(hsbp) | HSEN);
    regmap_write(fg.reg, VTCFG1, VACT(vact)   | VTOTAL(vtotal));
    regmap_write(fg.reg, VTCFG2, VSYNC(vsync) | VSBP(vsbp) | VSEN);
    kick_col = hact + 1;
    kick_row = vact;
// pkickconfig
    regmap_write(fg.reg, PKICKCONFIG, COL(kick_col) | ROW(kick_row) | EN);
// skikconfig
    regmap_write(fg.reg, SKICKCONFIG, COL(kick_col) | ROW(kick_row) | EN);
// primary and secondary area position configuration
    regmap_write(fg.reg, PACFG, STARTX(0) | STARTY(0));
    regmap_write(fg.reg, SACFG, STARTX(0) | STARTY(0));
// alpha
    regmap_write_bits(fg.reg, FGINCTRL,      ENPRIMALPHA | ENSECALPHA, 0);
    regmap_write_bits(fg.reg, FGINCTRLPANIC, ENPRIMALPHA | ENSECALPHA, 0);
// constant color is green(used in panic mode)
    regmap_write(fg.reg, FGCCR, CCGREEN(0x3ff));
    ret = clk_set_rate(fg.clk_disp, m.clock * HZ_PER_KHZ);
    if (ret < 0)
    dev_err(fg.dev, "failed to set display clock rate: %d\n", ret);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_displaymode(fg: *mut dc_fg, mode: enum dc_fg_dm) {
    static inline void dc_fg_displaymode(struct dc_fg *fg, enum dc_fg_dm mode)
    {
    regmap_write_bits(fg.reg, FGINCTRL, FGDM_MASK, mode);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_panic_displaymode(fg: *mut dc_fg, mode: enum dc_fg_dm) {
    static inline void dc_fg_panic_displaymode(struct dc_fg *fg, enum dc_fg_dm mode)
    {
    regmap_write_bits(fg.reg, FGINCTRLPANIC, FGDM_MASK, mode);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_enable(fg: *mut dc_fg) {
    void dc_fg_enable(struct dc_fg *fg)
    {
    regmap_write(fg.reg, FGENABLE, FGEN);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_disable(fg: *mut dc_fg) {
    void dc_fg_disable(struct dc_fg *fg)
    {
    regmap_write(fg.reg, FGENABLE, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_shdtokgen(fg: *mut dc_fg) {
    void dc_fg_shdtokgen(struct dc_fg *fg)
    {
    regmap_write(fg.reg, FGSLR, SHDTOKGEN);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_get_frame_index(fg: *mut dc_fg) -> u32 {
    u32 dc_fg_get_frame_index(struct dc_fg *fg)
    {
    u32 val;
    regmap_read(fg.reg, FGTIMESTAMP, &val);
    return FRAMEINDEX(val);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_get_line_index(fg: *mut dc_fg) -> u32 {
    u32 dc_fg_get_line_index(struct dc_fg *fg)
    {
    u32 val;
    regmap_read(fg.reg, FGTIMESTAMP, &val);
    return LINEINDEX(val);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_wait_for_frame_index_moving(fg: *mut dc_fg) -> bool {
    bool dc_fg_wait_for_frame_index_moving(struct dc_fg *fg)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(100);
    u32 frame_index, last_frame_index;
    frame_index = dc_fg_get_frame_index(fg);
    do {
    last_frame_index = frame_index;
    frame_index = dc_fg_get_frame_index(fg);
    } while (last_frame_index == frame_index &&
    time_before(jiffies, timeout));
    return last_frame_index != frame_index;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_secondary_requests_to_read_empty_fifo(fg: *mut dc_fg) -> bool {
    bool dc_fg_secondary_requests_to_read_empty_fifo(struct dc_fg *fg)
    {
    u32 val;
    regmap_read(fg.reg, FGCHSTAT, &val);
    return !!(val & SFIFOEMPTY);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_secondary_clear_channel_status(fg: *mut dc_fg) {
    void dc_fg_secondary_clear_channel_status(struct dc_fg *fg)
    {
    regmap_write(fg.reg, FGCHSTATCLR, CLRSECSTAT);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_wait_for_secondary_syncup(fg: *mut dc_fg) -> c_int {
    int dc_fg_wait_for_secondary_syncup(struct dc_fg *fg)
    {
    unsigned int val;
    return regmap_read_poll_timeout(fg.reg, FGCHSTAT, val,
    val & SECSYNCSTAT, 5, 100000);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_enable_clock(fg: *mut dc_fg) {
    void dc_fg_enable_clock(struct dc_fg *fg)
    {
    int ret;
    ret = clk_prepare_enable(fg.clk_disp);
    if (ret)
    dev_err(fg.dev, "failed to enable display clock: %d\n", ret);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_disable_clock(fg: *mut dc_fg) {
    void dc_fg_disable_clock(struct dc_fg *fg)
    {
    clk_disable_unprepare(fg.clk_disp);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_check_clock(fg: *mut dc_fg, clk_khz: c_int) -> enum drm_mode_status {
    enum drm_mode_status dc_fg_check_clock(struct dc_fg *fg, int clk_khz)
    {
    unsigned long rounded_rate;
    rounded_rate = clk_round_rate(fg.clk_disp, clk_khz * HZ_PER_KHZ);
    if (rounded_rate != clk_khz * HZ_PER_KHZ)
    return MODE_NOCLOCK;
    return MODE_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_fg_init(fg: *mut dc_fg) {
    void dc_fg_init(struct dc_fg *fg)
    {
    dc_fg_enable_shden(fg);
    dc_fg_syncmode(fg, FG_SYNCMODE_OFF);
    dc_fg_displaymode(fg, FG_DM_SEC_ON_TOP);
    dc_fg_panic_displaymode(fg, FG_DM_CONSTCOL);
    }
#[no_mangle]
unsafe extern "C" fn dc_fg_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int dc_fg_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct dc_drm_device *dc_drm = data;
    struct resource *res;
    void __iomem *base;
    struct dc_fg *fg;
    int id;
    fg = devm_kzalloc(dev, sizeof(*fg), GFP_KERNEL);
    if (!fg)
    return -ENOMEM;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    fg.reg = devm_regmap_init_mmio(dev, base, &dc_fg_regmap_config);
    if (IS_ERR(fg.reg))
    return PTR_ERR(fg.reg);
    fg.clk_disp = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(fg.clk_disp))
    return dev_err_probe(dev, PTR_ERR(fg.clk_disp),
    "failed to get display clock\n");
    id = dc_subdev_get_id(dc_fg_info, ARRAY_SIZE(dc_fg_info), res);
    if (id < 0) {
    dev_err(dev, "failed to get instance number: %d\n", id);
    return id;
    }
    fg.dev = dev;
    dc_drm.fg[id] = fg;
    return 0;
    }
    static const struct component_ops dc_fg_ops = {
    .bind = dc_fg_bind,
    };
#[no_mangle]
unsafe extern "C" fn dc_fg_probe(pdev: *mut platform_device) -> c_int {
    static int dc_fg_probe(struct platform_device *pdev)
    {
    int ret;
    ret = component_add(&pdev.dev, &dc_fg_ops);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_fg_remove(pdev: *mut platform_device) {
    static void dc_fg_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &dc_fg_ops);
    }
    static const struct of_device_id dc_fg_dt_ids[] = {
    { .compatible = "fsl,imx8qxp-dc-framegen" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dc_fg_dt_ids);
    struct platform_driver dc_fg_driver = {
    .probe = dc_fg_probe,
    .remove = dc_fg_remove,
    .driver = {
    .name = "imx8-dc-framegen",
    .suppress_bind_attrs = true,
    .of_match_table = dc_fg_dt_ids,
    },
    };
