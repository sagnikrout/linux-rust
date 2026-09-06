//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap/lcd_mipid.c
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
// LCD driver for MIPI DBI-C / DCS compatible LCDs
//
// Copyright (C) 2006 Nokia Corporation
// Author: Imre Deak <imre.deak@nokia.com>
//

pub const MIPID_CMD_READ_DISP_ID: c_uint = 0x04;
pub const MIPID_CMD_READ_RED: c_uint = 0x06;
pub const MIPID_CMD_READ_GREEN: c_uint = 0x07;
pub const MIPID_CMD_READ_BLUE: c_uint = 0x08;
pub const MIPID_CMD_READ_DISP_STATUS: c_uint = 0x09;
pub const MIPID_CMD_RDDSDR: c_uint = 0x0F;
pub const MIPID_CMD_SLEEP_IN: c_uint = 0x10;
pub const MIPID_CMD_SLEEP_OUT: c_uint = 0x11;
pub const MIPID_CMD_DISP_OFF: c_uint = 0x28;
pub const MIPID_CMD_DISP_ON: c_uint = 0x29;

    panel)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipid_device {
    pub enabled: c_int,
    pub revision: c_int,
    pub saved_bklight_level: c_uint,
    pub jiffies: *mut *mut unsigned long hw_guard_end; / next value of,
    when we can issue the
    next sleep in/out command */
    pub /: *mut *mut unsigned long hw_guard_wait; / max guard time in jiffies,
    pub reset: *mut gpio_desc,
    pub fbdev: *mut omapfb_device,
    pub spi: *mut spi_device,
    pub mutex: mutex,
    pub panel: lcd_panel,
    pub esd_work: delayed_work,
    pub m): *mut *mut void (esd_check)(struct mipid_device,
}

    static void mipid_transfer(struct mipid_device *md, int cmd, const u8 *wbuf,
    int wlen, u8 *rbuf, int rlen)
    {
    struct spi_message	m;
    struct spi_transfer	*x, xfer[4];
    u16			w;
    int			r;
    BUG_ON(md.spi == core::ptr::null_mut());
    spi_message_init(&m);
    memset(xfer, 0, sizeof(xfer));
    x = &xfer[0];
    cmd &=  0xff;
    x.tx_buf		= &cmd;
    x.bits_per_word	= 9;
    x.len			= 2;
    spi_message_add_tail(x, &m);
    if (wlen) {
    x++;
    x.tx_buf		= wbuf;
    x.len			= wlen;
    x.bits_per_word	= 9;
    spi_message_add_tail(x, &m);
    }
    if (rlen) {
    x++;
    x.rx_buf	= &w;
    x.len		= 1;
    spi_message_add_tail(x, &m);
    if (rlen > 1) {
// Arrange for the extra clock before the first
// data bit.
//
    x.bits_per_word = 9;
    x.len		 = 2;
    x++;
    x.rx_buf	 = &rbuf[1];
    x.len		 = rlen - 1;
    spi_message_add_tail(x, &m);
    }
    }
    r = spi_sync(md.spi, &m);
    if (r < 0)
    dev_dbg(&md.spi.dev, "spi_sync %d\n", r);
    if (rlen)
    rbuf[0] = w & 0xff;
    }
#[no_mangle]
pub unsafe extern "C" fn mipid_cmd(md: *mut mipid_device, cmd: c_int) {
    static inline void mipid_cmd(struct mipid_device *md, int cmd)
    {
    mipid_transfer(md, cmd, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    }
    static inline void mipid_write(struct mipid_device *md,
    int reg, const u8 *buf, int len)
    {
    mipid_transfer(md, reg, buf, len, core::ptr::null_mut(), 0);
    }
    static inline void mipid_read(struct mipid_device *md,
    int reg, u8 *buf, int len)
    {
    mipid_transfer(md, reg, core::ptr::null_mut(), 0, buf, len);
    }
#[no_mangle]
unsafe extern "C" fn set_data_lines(md: *mut mipid_device, data_lines: c_int) {
    static void set_data_lines(struct mipid_device *md, int data_lines)
    {
    u16 par;
    switch (data_lines) {
    case 16:
    par = 0x150;
    break;
    case 18:
    par = 0x160;
    break;
    case 24:
    par = 0x170;
    break;
    }
    mipid_write(md, 0x3a, (u8 *)&par, 2);
    }
#[no_mangle]
unsafe extern "C" fn send_init_string(md: *mut mipid_device) {
    static void send_init_string(struct mipid_device *md)
    {
    u16 initpar[] = { 0x0102, 0x0100, 0x0100 };
    mipid_write(md, 0xc2, (u8 *)initpar, sizeof(initpar));
    set_data_lines(md, md.panel.data_lines);
    }
#[no_mangle]
unsafe extern "C" fn hw_guard_start(md: *mut mipid_device, guard_msec: c_int) {
    static void hw_guard_start(struct mipid_device *md, int guard_msec)
    {
    md.hw_guard_wait = msecs_to_jiffies(guard_msec);
    md.hw_guard_end = jiffies + md.hw_guard_wait;
    }
#[no_mangle]
unsafe extern "C" fn hw_guard_wait(md: *mut mipid_device) {
    static void hw_guard_wait(struct mipid_device *md)
    {
    let mut wait: c_ulong = md.hw_guard_end - jiffies;
    if ((long)wait > 0 && time_before_eq(wait,  md.hw_guard_wait)) {
    set_current_state(TASK_UNINTERRUPTIBLE);
    schedule_timeout(wait);
    }
    }
#[no_mangle]
unsafe extern "C" fn set_sleep_mode(md: *mut mipid_device, on: c_int) {
    static void set_sleep_mode(struct mipid_device *md, int on)
    {
    int cmd, sleep_time = 50;
    if (on)
    cmd = MIPID_CMD_SLEEP_IN;
    else
    cmd = MIPID_CMD_SLEEP_OUT;
    hw_guard_wait(md);
    mipid_cmd(md, cmd);
    hw_guard_start(md, 120);
//
// When we enable the panel, it seems we _have_ to sleep
// 120 ms before sending the init string. When disabling the
// panel we'll sleep for the duration of 2 frames, so that the
// controller can still provide the PCLK,HS,VS signals.
//
    if (!on)
    sleep_time = 120;
    msleep(sleep_time);
    }
#[no_mangle]
unsafe extern "C" fn set_display_state(md: *mut mipid_device, enabled: c_int) {
    static void set_display_state(struct mipid_device *md, int enabled)
    {
    let mut cmd: c_int = enabled ? MIPID_CMD_DISP_ON : MIPID_CMD_DISP_OFF;
    mipid_cmd(md, cmd);
    }
#[no_mangle]
unsafe extern "C" fn mipid_set_bklight_level(panel: *mut lcd_panel, level: c_uint) -> c_int {
    static int mipid_set_bklight_level(struct lcd_panel *panel, unsigned int level)
    {
    struct mipid_device *md = to_mipid_device(panel);
    struct mipid_platform_data *pd = md.spi.dev.platform_data;
    if (pd.get_bklight_max == core::ptr::null_mut() || pd.set_bklight_level == core::ptr::null_mut())
    return -ENODEV;
    if (level > pd.get_bklight_max(pd))
    return -EINVAL;
    if (!md.enabled) {
    md.saved_bklight_level = level;
    return 0;
    }
    pd.set_bklight_level(pd, level);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mipid_get_bklight_level(panel: *mut lcd_panel) -> c_uint {
    static unsigned int mipid_get_bklight_level(struct lcd_panel *panel)
    {
    struct mipid_device *md = to_mipid_device(panel);
    struct mipid_platform_data *pd = md.spi.dev.platform_data;
    if (pd.get_bklight_level == core::ptr::null_mut())
    return -ENODEV;
    return pd.get_bklight_level(pd);
    }
#[no_mangle]
unsafe extern "C" fn mipid_get_bklight_max(panel: *mut lcd_panel) -> c_uint {
    static unsigned int mipid_get_bklight_max(struct lcd_panel *panel)
    {
    struct mipid_device *md = to_mipid_device(panel);
    struct mipid_platform_data *pd = md.spi.dev.platform_data;
    if (pd.get_bklight_max == core::ptr::null_mut())
    return -ENODEV;
    return pd.get_bklight_max(pd);
    }
#[no_mangle]
unsafe extern "C" fn mipid_get_caps(panel: *mut lcd_panel) -> c_ulong {
    static unsigned long mipid_get_caps(struct lcd_panel *panel)
    {
    return OMAPFB_CAPS_SET_BACKLIGHT;
    }
#[no_mangle]
unsafe extern "C" fn read_first_pixel(md: *mut mipid_device) -> u16 {
    static u16 read_first_pixel(struct mipid_device *md)
    {
    u16 pixel;
    u8 red, green, blue;
    mutex_lock(&md.mutex);
    mipid_read(md, MIPID_CMD_READ_RED, &red, 1);
    mipid_read(md, MIPID_CMD_READ_GREEN, &green, 1);
    mipid_read(md, MIPID_CMD_READ_BLUE, &blue, 1);
    mutex_unlock(&md.mutex);
    switch (md.panel.data_lines) {
    case 16:
    pixel = ((red >> 1) << 11) | (green << 5) | (blue >> 1);
    break;
    case 24:
// 24 bit -> 16 bit
    pixel = ((red >> 3) << 11) | ((green >> 2) << 5) |
    (blue >> 3);
    break;
    default:
    pixel = 0;
    BUG();
    }
    return pixel;
    }
#[no_mangle]
unsafe extern "C" fn mipid_run_test(panel: *mut lcd_panel, test_num: c_int) -> c_int {
    static int mipid_run_test(struct lcd_panel *panel, int test_num)
    {
    struct mipid_device *md = to_mipid_device(panel);
    static const u16 test_values[4] = {
    0x0000, 0xffff, 0xaaaa, 0x5555,
    };
    int i;
    if (test_num != MIPID_TEST_RGB_LINES)
    return MIPID_TEST_INVALID;
    for (i = 0; i < ARRAY_SIZE(test_values); i++) {
    int delay;
    unsigned long tmo;
    omapfb_write_first_pixel(md.fbdev, test_values[i]);
    tmo = jiffies + msecs_to_jiffies(100);
    delay = 25;
    while (1) {
    u16 pixel;
    msleep(delay);
    pixel = read_first_pixel(md);
    if (pixel == test_values[i])
    break;
    if (time_after(jiffies, tmo)) {
    dev_err(&md.spi.dev,
    "MIPI LCD RGB I/F test failed: "
    "expecting %04x, got %04x\n",
    test_values[i], pixel);
    return MIPID_TEST_FAILED;
    }
    delay = 10;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls041y3_esd_recover(md: *mut mipid_device) {
    static void ls041y3_esd_recover(struct mipid_device *md)
    {
    dev_err(&md.spi.dev, "performing LCD ESD recovery\n");
    set_sleep_mode(md, 1);
    set_sleep_mode(md, 0);
    }
#[no_mangle]
unsafe extern "C" fn ls041y3_esd_check_mode1(md: *mut mipid_device) {
    static void ls041y3_esd_check_mode1(struct mipid_device *md)
    {
    u8 state1, state2;
    mipid_read(md, MIPID_CMD_RDDSDR, &state1, 1);
    set_sleep_mode(md, 0);
    mipid_read(md, MIPID_CMD_RDDSDR, &state2, 1);
    dev_dbg(&md.spi.dev, "ESD mode 1 state1 %02x state2 %02x\n",
    state1, state2);
// Each sleep out command will trigger a self diagnostic and flip
// Bit6 if the test passes.
//
    if (!((state1 ^ state2) & (1 << 6)))
    ls041y3_esd_recover(md);
    }
#[no_mangle]
unsafe extern "C" fn ls041y3_esd_check_mode2(md: *mut mipid_device) {
    static void ls041y3_esd_check_mode2(struct mipid_device *md)
    {
    int i;
    u8 rbuf[2];
    static const struct {
    int	cmd;
    int	wlen;
    u16	wbuf[3];
    } *rd, rd_ctrl[7] = {
    { 0xb0, 4, { 0x0101, 0x01fe, } },
    { 0xb1, 4, { 0x01de, 0x0121, } },
    { 0xc2, 4, { 0x0100, 0x0100, } },
    { 0xbd, 2, { 0x0100, } },
    { 0xc2, 4, { 0x01fc, 0x0103, } },
    { 0xb4, 0, },
    { 0x00, 0, },
    };
    rd = rd_ctrl;
    for (i = 0; i < 3; i++, rd++)
    mipid_write(md, rd.cmd, (u8 *)rd.wbuf, rd.wlen);
    udelay(10);
    mipid_read(md, rd.cmd, rbuf, 2);
    rd++;
    for (i = 0; i < 3; i++, rd++) {
    udelay(10);
    mipid_write(md, rd.cmd, (u8 *)rd.wbuf, rd.wlen);
    }
    dev_dbg(&md.spi.dev, "ESD mode 2 state %02x\n", rbuf[1]);
    if (rbuf[1] == 0x00)
    ls041y3_esd_recover(md);
    }
#[no_mangle]
unsafe extern "C" fn ls041y3_esd_check(md: *mut mipid_device) {
    static void ls041y3_esd_check(struct mipid_device *md)
    {
    ls041y3_esd_check_mode1(md);
    if (md.revision >= 0x88)
    ls041y3_esd_check_mode2(md);
    }
#[no_mangle]
unsafe extern "C" fn mipid_esd_start_check(md: *mut mipid_device) {
    static void mipid_esd_start_check(struct mipid_device *md)
    {
    if (md.esd_check != core::ptr::null_mut())
    schedule_delayed_work(&md.esd_work,
    MIPID_ESD_CHECK_PERIOD);
    }
#[no_mangle]
unsafe extern "C" fn mipid_esd_stop_check(md: *mut mipid_device) {
    static void mipid_esd_stop_check(struct mipid_device *md)
    {
    if (md.esd_check != core::ptr::null_mut())
    cancel_delayed_work_sync(&md.esd_work);
    }
#[no_mangle]
unsafe extern "C" fn mipid_esd_work(work: *mut work_struct) {
    static void mipid_esd_work(struct work_struct *work)
    {
    struct mipid_device *md = container_of(work, struct mipid_device,
    esd_work.work);
    mutex_lock(&md.mutex);
    md.esd_check(md);
    mutex_unlock(&md.mutex);
    mipid_esd_start_check(md);
    }
#[no_mangle]
unsafe extern "C" fn mipid_enable(panel: *mut lcd_panel) -> c_int {
    static int mipid_enable(struct lcd_panel *panel)
    {
    struct mipid_device *md = to_mipid_device(panel);
    mutex_lock(&md.mutex);
    if (md.enabled) {
    mutex_unlock(&md.mutex);
    return 0;
    }
    set_sleep_mode(md, 0);
    md.enabled = 1;
    send_init_string(md);
    set_display_state(md, 1);
    mipid_set_bklight_level(panel, md.saved_bklight_level);
    mipid_esd_start_check(md);
    mutex_unlock(&md.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mipid_disable(panel: *mut lcd_panel) {
    static void mipid_disable(struct lcd_panel *panel)
    {
    struct mipid_device *md = to_mipid_device(panel);
//
// A final ESD work might be called before returning,
// so do this without holding the lock.
//
    mipid_esd_stop_check(md);
    mutex_lock(&md.mutex);
    if (!md.enabled) {
    mutex_unlock(&md.mutex);
    return;
    }
    md.saved_bklight_level = mipid_get_bklight_level(panel);
    mipid_set_bklight_level(panel, 0);
    set_display_state(md, 0);
    set_sleep_mode(md, 1);
    md.enabled = 0;
    mutex_unlock(&md.mutex);
    }
#[no_mangle]
unsafe extern "C" fn panel_enabled(md: *mut mipid_device) -> c_int {
    static int panel_enabled(struct mipid_device *md)
    {
    u32 disp_status;
    int enabled;
    mipid_read(md, MIPID_CMD_READ_DISP_STATUS, (u8 *)&disp_status, 4);
    disp_status = __be32_to_cpu(disp_status);
    enabled = (disp_status & (1 << 17)) && (disp_status & (1 << 10));
    dev_dbg(&md.spi.dev,
    "LCD panel %senabled by bootloader (status 0x%04x)\n",
    enabled ? "" : "not ", disp_status);
    return enabled;
    }
    static int mipid_init(struct lcd_panel *panel,
    struct omapfb_device *fbdev)
    {
    struct mipid_device *md = to_mipid_device(panel);
    md.fbdev = fbdev;
    INIT_DELAYED_WORK(&md.esd_work, mipid_esd_work);
    mutex_init(&md.mutex);
    md.enabled = panel_enabled(md);
    if (md.enabled)
    mipid_esd_start_check(md);
    else
    md.saved_bklight_level = mipid_get_bklight_level(panel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mipid_cleanup(panel: *mut lcd_panel) {
    static void mipid_cleanup(struct lcd_panel *panel)
    {
    struct mipid_device *md = to_mipid_device(panel);
    if (md.enabled)
    mipid_esd_stop_check(md);
    }
    static const struct lcd_panel mipid_panel = {
    .config		= OMAP_LCDC_PANEL_TFT,
    .bpp		= 16,
    .x_res		= 800,
    .y_res		= 480,
    .pixel_clock	= 21940,
    .hsw		= 50,
    .hfp		= 20,
    .hbp		= 15,
    .vsw		= 2,
    .vfp		= 1,
    .vbp		= 3,
    .init			= mipid_init,
    .cleanup		= mipid_cleanup,
    .enable			= mipid_enable,
    .disable		= mipid_disable,
    .get_caps		= mipid_get_caps,
    .set_bklight_level	= mipid_set_bklight_level,
    .get_bklight_level	= mipid_get_bklight_level,
    .get_bklight_max	= mipid_get_bklight_max,
    .run_test		= mipid_run_test,
    };
#[no_mangle]
unsafe extern "C" fn mipid_detect(md: *mut mipid_device) -> c_int {
    static int mipid_detect(struct mipid_device *md)
    {
    struct mipid_platform_data *pdata;
    u8 display_id[3];
    pdata = md.spi.dev.platform_data;
    if (pdata == core::ptr::null_mut()) {
    dev_err(&md.spi.dev, "missing platform data\n");
    return -ENOENT;
    }
    mipid_read(md, MIPID_CMD_READ_DISP_ID, display_id, 3);
    dev_dbg(&md.spi.dev, "MIPI display ID: %02x%02x%02x\n",
    display_id[0], display_id[1], display_id[2]);
    switch (display_id[0]) {
    case 0x45:
    md.panel.name = "lph8923";
    break;
    case 0x83:
    md.panel.name = "ls041y3";
    md.esd_check = ls041y3_esd_check;
    break;
    default:
    md.panel.name = "unknown";
    dev_err(&md.spi.dev, "invalid display ID\n");
    return -ENODEV;
    }
    md.revision = display_id[1];
    md.panel.data_lines = pdata.data_lines;
    pr_info("omapfb: %s rev %02x LCD detected, %d data lines\n",
    md.panel.name, md.revision, md.panel.data_lines);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mipid_spi_probe(spi: *mut spi_device) -> c_int {
    static int mipid_spi_probe(struct spi_device *spi)
    {
    struct mipid_device *md;
    int r;
    md = kzalloc_obj(*md);
    if (md == core::ptr::null_mut()) {
    dev_err(&spi.dev, "out of memory\n");
    return -ENOMEM;
    }
// This will de-assert RESET if active
    md.reset = gpiod_get(&spi.dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(md.reset))
    return dev_err_probe(&spi.dev, PTR_ERR(md.reset),
    "no reset GPIO line\n");
    spi.mode = SPI_MODE_0;
    md.spi = spi;
    dev_set_drvdata(&spi.dev, md);
    md.panel = mipid_panel;
    r = mipid_detect(md);
    if (r < 0)
    goto free_md;
    omapfb_register_panel(&md.panel);
    return 0;
    free_md:
    kfree(md);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn mipid_spi_remove(spi: *mut spi_device) {
    static void mipid_spi_remove(struct spi_device *spi)
    {
    struct mipid_device *md = dev_get_drvdata(&spi.dev);
// Asserts RESET
    gpiod_set_value(md.reset, 1);
    mipid_disable(&md.panel);
    kfree(md);
    }
    static struct spi_driver mipid_spi_driver = {
    .driver = {
    .name	= MIPID_MODULE_NAME,
    },
    .probe	= mipid_spi_probe,
    .remove	= mipid_spi_remove,
    };
    module_spi_driver(mipid_spi_driver);
    MODULE_DESCRIPTION("MIPI display driver");
    MODULE_LICENSE("GPL");
