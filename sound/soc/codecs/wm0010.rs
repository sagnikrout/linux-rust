//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/wm0010.c
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
// wm0010.c  --  WM0010 DSP Driver
//
// Copyright 2012 Wolfson Microelectronics PLC.
//
// Authors: Mark Brown <broonie@opensource.wolfsonmicro.com>
// Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
// Scott Ling <sl@opensource.wolfsonmicro.com>
//

pub const DEVICE_ID_WM0010: c_int = 10;
// We only support v1 of the .dfw INFO record
pub const INFO_VERSION: c_int = 1;
    enum dfw_cmd {
    DFW_CMD_FUSE = 0x01,
    DFW_CMD_CODE_HDR,
    DFW_CMD_CODE_DATA,
    DFW_CMD_PLL,
    DFW_CMD_INFO = 0xff
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfw_binrec {
    pub command: u8,
    pub length:24: u32,
    pub address: u32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfw_inforec {
    pub info_version: u8,
    pub tool_major_version: u8,
    pub tool_minor_version: u8,
    pub dsp_target: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfw_pllrec {
    pub command: u8,
    pub length:24: u32,
    pub address: u32,
    pub clkctrl1: u32,
    pub clkctrl2: u32,
    pub clkctrl3: u32,
    pub ldetctrl: u32,
    pub uart_div: u32,
    pub spi_div: u32,
    pub __packed: },
    static struct pll_clock_map {
    pub max_sysclk: c_int,
    pub max_pll_spi_speed: c_int,
    pub pll_clkctrl1: u32,
    } pll_clock_map[] = {			   /* Dividers */
    { 22000000, 26000000, 0x00201f11 }, /* 2,32,2  */
    { 18000000, 26000000, 0x00203f21 }, /* 2,64,4  */
    { 14000000, 26000000, 0x00202620 }, /* 1,39,4  */
    { 10000000, 22000000, 0x00203120 }, /* 1,50,4  */
    {  6500000, 22000000, 0x00204520 }, /* 1,70,4  */
    {  5500000, 22000000, 0x00103f10 }, /* 1,64,2  */
}

    enum wm0010_state {
    WM0010_POWER_OFF,
    WM0010_OUT_OF_RESET,
    WM0010_BOOTROM,
    WM0010_STAGE2,
    WM0010_FIRMWARE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm0010_priv {
    pub component: *mut snd_soc_component,
    pub lock: mutex,
    pub dev: *mut device,
    pub pdata: wm0010_pdata,
    pub reset: *mut gpio_desc,
    pub core_supplies: [regulator_bulk_data; 2],
    pub dbvdd: *mut regulator,
    pub sysclk: c_int,
    pub state: enum wm0010_state,
    pub boot_failed: bool,
    pub ready: bool,
    pub pll_running: bool,
    pub max_spi_freq: c_int,
    pub board_max_spi_speed: c_int,
    pub pll_clkctrl1: u32,
    pub irq_lock: spinlock_t,
    pub irq: c_int,
    pub boot_completion: completion,
}

    static const struct snd_soc_dapm_widget wm0010_dapm_widgets[] = {
    SND_SOC_DAPM_SUPPLY("CLKIN",  SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0),
    };
    static const struct snd_soc_dapm_route wm0010_dapm_routes[] = {
    { "SDI2 Capture", core::ptr::null_mut(), "SDI1 Playback" },
    { "SDI1 Capture", core::ptr::null_mut(), "SDI2 Playback" },
    { "SDI1 Capture", core::ptr::null_mut(), "CLKIN" },
    { "SDI2 Capture", core::ptr::null_mut(), "CLKIN" },
    { "SDI1 Playback", core::ptr::null_mut(), "CLKIN" },
    { "SDI2 Playback", core::ptr::null_mut(), "CLKIN" },
    };
    static const char *wm0010_state_to_str(enum wm0010_state state)
    {
    static const char * const state_to_str[] = {
    "Power off",
    "Out of reset",
    "Boot ROM",
    "Stage2",
    "Firmware"
    };
    if (state < 0 || state >= ARRAY_SIZE(state_to_str))
    return "null";
    return state_to_str[state];
    }
// Called with wm0010->lock held
#[no_mangle]
unsafe extern "C" fn wm0010_halt(component: *mut snd_soc_component) {
    static void wm0010_halt(struct snd_soc_component *component)
    {
    struct wm0010_priv *wm0010 = snd_soc_component_get_drvdata(component);
    enum wm0010_state state;
// Fetch the wm0010 state
    scoped_guard(spinlock_irqsave, &wm0010.irq_lock)
    state = wm0010.state;
    switch (state) {
    case WM0010_POWER_OFF:
// If there's nothing to do, bail out
    return;
    case WM0010_OUT_OF_RESET:
    case WM0010_BOOTROM:
    case WM0010_STAGE2:
    case WM0010_FIRMWARE:
// Remember to put chip back into reset
    gpiod_set_value_cansleep(wm0010.reset, 1);
// Disable the regulators
    regulator_disable(wm0010.dbvdd);
    regulator_bulk_disable(ARRAY_SIZE(wm0010.core_supplies),
    wm0010.core_supplies);
    break;
    }
    scoped_guard(spinlock_irqsave, &wm0010.irq_lock)
    wm0010.state = WM0010_POWER_OFF;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm0010_boot_xfer {
    pub list: list_head,
    pub component: *mut snd_soc_component,
    pub done: *mut completion,
    pub m: spi_message,
    pub t: spi_transfer,
}

// Called with wm0010->lock held
#[no_mangle]
unsafe extern "C" fn wm0010_mark_boot_failure(wm0010: *mut wm0010_priv) {
    static void wm0010_mark_boot_failure(struct wm0010_priv *wm0010)
    {
    enum wm0010_state state;
    scoped_guard(spinlock_irqsave, &wm0010.irq_lock)
    state = wm0010.state;
    dev_err(wm0010.dev, "Failed to transition from `%s' state to `%s' state\n",
    wm0010_state_to_str(state), wm0010_state_to_str(state + 1));
    wm0010.boot_failed = true;
    }
#[no_mangle]
unsafe extern "C" fn wm0010_boot_xfer_complete(data: *mut c_void) {
    static void wm0010_boot_xfer_complete(void *data)
    {
    struct wm0010_boot_xfer *xfer = data;
    struct snd_soc_component *component = xfer.component;
    struct wm0010_priv *wm0010 = snd_soc_component_get_drvdata(component);
    u32 *out32 = xfer.t.rx_buf;
    int i;
    if (xfer.m.status != 0) {
    dev_err(component.dev, "SPI transfer failed: %d\n",
    xfer.m.status);
    wm0010_mark_boot_failure(wm0010);
    if (xfer.done)
    complete(xfer.done);
    return;
    }
    for (i = 0; i < xfer.t.len / 4; i++) {
    dev_dbg(component.dev, "%d: %04x\n", i, out32[i]);
    switch (be32_to_cpu(out32[i])) {
    case 0xe0e0e0e0:
    dev_err(component.dev,
    "%d: ROM error reported in stage 2\n", i);
    wm0010_mark_boot_failure(wm0010);
    break;
    case 0x55555555:
    if (wm0010.state < WM0010_STAGE2)
    break;
    dev_err(component.dev,
    "%d: ROM bootloader running in stage 2\n", i);
    wm0010_mark_boot_failure(wm0010);
    break;
    case 0x0fed0000:
    dev_dbg(component.dev, "Stage2 loader running\n");
    break;
    case 0x0fed0007:
    dev_dbg(component.dev, "CODE_HDR packet received\n");
    break;
    case 0x0fed0008:
    dev_dbg(component.dev, "CODE_DATA packet received\n");
    break;
    case 0x0fed0009:
    dev_dbg(component.dev, "Download complete\n");
    break;
    case 0x0fed000c:
    dev_dbg(component.dev, "Application start\n");
    break;
    case 0x0fed000e:
    dev_dbg(component.dev, "PLL packet received\n");
    wm0010.pll_running = true;
    break;
    case 0x0fed0025:
    dev_err(component.dev, "Device reports image too long\n");
    wm0010_mark_boot_failure(wm0010);
    break;
    case 0x0fed002c:
    dev_err(component.dev, "Device reports bad SPI packet\n");
    wm0010_mark_boot_failure(wm0010);
    break;
    case 0x0fed0031:
    dev_err(component.dev, "Device reports SPI read overflow\n");
    wm0010_mark_boot_failure(wm0010);
    break;
    case 0x0fed0032:
    dev_err(component.dev, "Device reports SPI underclock\n");
    wm0010_mark_boot_failure(wm0010);
    break;
    case 0x0fed0033:
    dev_err(component.dev, "Device reports bad header packet\n");
    wm0010_mark_boot_failure(wm0010);
    break;
    case 0x0fed0034:
    dev_err(component.dev, "Device reports invalid packet type\n");
    wm0010_mark_boot_failure(wm0010);
    break;
    case 0x0fed0035:
    dev_err(component.dev, "Device reports data before header error\n");
    wm0010_mark_boot_failure(wm0010);
    break;
    case 0x0fed0038:
    dev_err(component.dev, "Device reports invalid PLL packet\n");
    break;
    case 0x0fed003a:
    dev_err(component.dev, "Device reports packet alignment error\n");
    wm0010_mark_boot_failure(wm0010);
    break;
    default:
    dev_err(component.dev, "Unrecognised return 0x%x\n",
    be32_to_cpu(out32[i]));
    wm0010_mark_boot_failure(wm0010);
    break;
    }
    if (wm0010.boot_failed)
    break;
    }
    if (xfer.done)
    complete(xfer.done);
    }
#[no_mangle]
unsafe extern "C" fn byte_swap_64(data_in: *mut u64, data_out: *mut u64, len: u32) {
    static void byte_swap_64(u64 *data_in, u64 *data_out, u32 len)
    {
    int i;
    for (i = 0; i < len / 8; i++)
    data_out[i] = swab64(data_in[i]);
    }
#[no_mangle]
unsafe extern "C" fn wm0010_firmware_load(name: *const c_char, component: *mut snd_soc_component) -> c_int {
    static int wm0010_firmware_load(const char *name, struct snd_soc_component *component)
    {
    struct spi_device *spi = to_spi_device(component.dev);
    struct wm0010_priv *wm0010 = snd_soc_component_get_drvdata(component);
    struct list_head xfer_list;
    struct wm0010_boot_xfer *xfer;
    int ret;
    DECLARE_COMPLETION_ONSTACK(done);
    const struct dfw_binrec *rec;
    const struct dfw_inforec *inforec;
    u64 *img;
    u8 *out, dsp;
    u32 len, offset;
    INIT_LIST_HEAD(&xfer_list);
    const struct firmware *fw __free(firmware) = core::ptr::null_mut();
    ret = request_firmware(&fw, name, component.dev);
    if (ret != 0) {
    dev_err(component.dev, "Failed to request application(%s): %d\n",
    name, ret);
    return ret;
    }
    rec = (const struct dfw_binrec *)fw.data;
    inforec = (const struct dfw_inforec *)rec.data;
    offset = 0;
    dsp = inforec.dsp_target;
    wm0010.boot_failed = false;
    if (WARN_ON(!list_empty(&xfer_list)))
    return -EINVAL;
// First record should be INFO
    if (rec.command != DFW_CMD_INFO) {
    dev_err(component.dev, "First record not INFO\r\n");
    return -EINVAL;
    }
    if (inforec.info_version != INFO_VERSION) {
    dev_err(component.dev,
    "Unsupported version (%02d) of INFO record\r\n",
    inforec.info_version);
    return -EINVAL;
    }
    dev_dbg(component.dev, "Version v%02d INFO record found\r\n",
    inforec.info_version);
// Check it's a DSP file
    if (dsp != DEVICE_ID_WM0010) {
    dev_err(component.dev, "Not a WM0010 firmware file.\r\n");
    return -EINVAL;
    }
// Skip the info record as we don't need to send it
    offset += ((rec.length) + 8);
    rec = (void *)&rec.data[rec.length];
    while (offset < fw.size) {
    dev_dbg(component.dev,
    "Packet: command %d, data length = 0x%x\r\n",
    rec.command, rec.length);
    len = rec.length + 8;
    xfer = kzalloc_obj(*xfer);
    if (!xfer) {
    ret = -ENOMEM;
    goto abort;
    }
    xfer.component = component;
    list_add_tail(&xfer.list, &xfer_list);
    out = kzalloc(len, GFP_KERNEL | GFP_DMA);
    if (!out) {
    ret = -ENOMEM;
    goto abort;
    }
    xfer.t.rx_buf = out;
    img = kzalloc(len, GFP_KERNEL | GFP_DMA);
    if (!img) {
    ret = -ENOMEM;
    goto abort;
    }
    xfer.t.tx_buf = img;
    byte_swap_64((u64 *)&rec.command, img, len);
    spi_message_init(&xfer.m);
    xfer.m.complete = wm0010_boot_xfer_complete;
    xfer.m.context = xfer;
    xfer.t.len = len;
    xfer.t.bits_per_word = 8;
    if (!wm0010.pll_running) {
    xfer.t.speed_hz = wm0010.sysclk / 6;
    } else {
    xfer.t.speed_hz = wm0010.max_spi_freq;
    if (wm0010.board_max_spi_speed &&
    (wm0010.board_max_spi_speed < wm0010.max_spi_freq))
    xfer.t.speed_hz = wm0010.board_max_spi_speed;
    }
// Store max usable spi frequency for later use
    wm0010.max_spi_freq = xfer.t.speed_hz;
    spi_message_add_tail(&xfer.t, &xfer.m);
    offset += ((rec.length) + 8);
    rec = (void *)&rec.data[rec.length];
    if (offset >= fw.size) {
    dev_dbg(component.dev, "All transfers scheduled\n");
    xfer.done = &done;
    }
    ret = spi_async(spi, &xfer.m);
    if (ret != 0) {
    dev_err(component.dev, "Write failed: %d\n", ret);
    goto abort;
    }
    if (wm0010.boot_failed) {
    dev_dbg(component.dev, "Boot fail!\n");
    ret = -EINVAL;
    goto abort;
    }
    }
    wait_for_completion(&done);
    ret = 0;
    abort:
    while (!list_empty(&xfer_list)) {
    xfer = list_first_entry(&xfer_list, struct wm0010_boot_xfer,
    list);
    kfree(xfer.t.rx_buf);
    kfree(xfer.t.tx_buf);
    list_del(&xfer.list);
    kfree(xfer);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wm0010_stage2_load(component: *mut snd_soc_component) -> c_int {
    static int wm0010_stage2_load(struct snd_soc_component *component)
    {
    struct spi_device *spi = to_spi_device(component.dev);
    struct wm0010_priv *wm0010 = snd_soc_component_get_drvdata(component);
    struct spi_message m;
    struct spi_transfer t;
    int i;
    let mut ret: c_int = 0;
    const struct firmware *fw __free(firmware) = core::ptr::null_mut();
    ret = request_firmware(&fw, "wm0010_stage2.bin", component.dev);
    if (ret != 0) {
    dev_err(component.dev, "Failed to request stage2 loader: %d\n",
    ret);
    return ret;
    }
    dev_dbg(component.dev, "Downloading %zu byte stage 2 loader\n", fw.size);
// Copy to local buffer first as vmalloc causes problems for dma
    u32 *img __free(kfree) =
    kmemdup(&fw.data[0], fw.size, GFP_KERNEL | GFP_DMA);
    if (!img)
    return -ENOMEM;
    u8 *out __free(kfree) =
    kzalloc(fw.size, GFP_KERNEL | GFP_DMA);
    if (!out)
    return -ENOMEM;
    spi_message_init(&m);
    memset(&t, 0, sizeof(t));
    t.rx_buf = out;
    t.tx_buf = img;
    t.len = fw.size;
    t.bits_per_word = 8;
    t.speed_hz = wm0010.sysclk / 10;
    spi_message_add_tail(&t, &m);
    dev_dbg(component.dev, "Starting initial download at %dHz\n",
    t.speed_hz);
    ret = spi_sync(spi, &m);
    if (ret != 0) {
    dev_err(component.dev, "Initial download failed: %d\n", ret);
    return ret;
    }
// Look for errors from the boot ROM
    for (i = 0; i < fw.size; i++) {
    if (out[i] != 0x55) {
    dev_err(component.dev, "Boot ROM error: %x in %d\n",
    out[i], i);
    wm0010_mark_boot_failure(wm0010);
    return -EBUSY;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wm0010_boot(component: *mut snd_soc_component) -> c_int {
    static int wm0010_boot(struct snd_soc_component *component)
    {
    struct spi_device *spi = to_spi_device(component.dev);
    struct wm0010_priv *wm0010 = snd_soc_component_get_drvdata(component);
    unsigned long flags;
    int ret;
    struct spi_message m;
    struct spi_transfer t;
    struct dfw_pllrec pll_rec;
    u32 *p, len;
    u64 *img_swap;
    u8 *out;
    int i;
    spin_lock_irqsave(&wm0010.irq_lock, flags);
    if (wm0010.state != WM0010_POWER_OFF)
    dev_warn(wm0010.dev, "DSP already powered up!\n");
    spin_unlock_irqrestore(&wm0010.irq_lock, flags);
    if (wm0010.sysclk > 26000000) {
    dev_err(component.dev, "Max DSP clock frequency is 26MHz\n");
    ret = -ECANCELED;
    goto err;
    }
    mutex_lock(&wm0010.lock);
    wm0010.pll_running = false;
    dev_dbg(component.dev, "max_spi_freq: %d\n", wm0010.max_spi_freq);
    ret = regulator_bulk_enable(ARRAY_SIZE(wm0010.core_supplies),
    wm0010.core_supplies);
    if (ret != 0) {
    dev_err(&spi.dev, "Failed to enable core supplies: %d\n",
    ret);
    mutex_unlock(&wm0010.lock);
    goto err;
    }
    ret = regulator_enable(wm0010.dbvdd);
    if (ret != 0) {
    dev_err(&spi.dev, "Failed to enable DBVDD: %d\n", ret);
    goto err_core;
    }
// Release reset
    gpiod_set_value_cansleep(wm0010.reset, 0);
    spin_lock_irqsave(&wm0010.irq_lock, flags);
    wm0010.state = WM0010_OUT_OF_RESET;
    spin_unlock_irqrestore(&wm0010.irq_lock, flags);
    if (!wait_for_completion_timeout(&wm0010.boot_completion,
    msecs_to_jiffies(20)))
    dev_err(component.dev, "Failed to get interrupt from DSP\n");
    spin_lock_irqsave(&wm0010.irq_lock, flags);
    wm0010.state = WM0010_BOOTROM;
    spin_unlock_irqrestore(&wm0010.irq_lock, flags);
    ret = wm0010_stage2_load(component);
    if (ret)
    goto abort;
    if (!wait_for_completion_timeout(&wm0010.boot_completion,
    msecs_to_jiffies(20)))
    dev_err(component.dev, "Failed to get interrupt from DSP loader.\n");
    spin_lock_irqsave(&wm0010.irq_lock, flags);
    wm0010.state = WM0010_STAGE2;
    spin_unlock_irqrestore(&wm0010.irq_lock, flags);
// Only initialise PLL if max_spi_freq initialised
    if (wm0010.max_spi_freq) {
// Initialise a PLL record
    memset(&pll_rec, 0, sizeof(pll_rec));
    pll_rec.command = DFW_CMD_PLL;
    pll_rec.length = (sizeof(pll_rec) - 8);
// On wm0010 only the CLKCTRL1 value is used
    pll_rec.clkctrl1 = wm0010.pll_clkctrl1;
    ret = -ENOMEM;
    len = pll_rec.length + 8;
    out = kzalloc(len, GFP_KERNEL | GFP_DMA);
    if (!out)
    goto abort;
    img_swap = kzalloc(len, GFP_KERNEL | GFP_DMA);
    if (!img_swap)
    goto abort_out;
// We need to re-order for 0010
    byte_swap_64((u64 *)&pll_rec, img_swap, len);
    spi_message_init(&m);
    memset(&t, 0, sizeof(t));
    t.rx_buf = out;
    t.tx_buf = img_swap;
    t.len = len;
    t.bits_per_word = 8;
    t.speed_hz = wm0010.sysclk / 6;
    spi_message_add_tail(&t, &m);
    ret = spi_sync(spi, &m);
    if (ret) {
    dev_err(component.dev, "First PLL write failed: %d\n", ret);
    goto abort_swap;
    }
// Use a second send of the message to get the return status
    ret = spi_sync(spi, &m);
    if (ret) {
    dev_err(component.dev, "Second PLL write failed: %d\n", ret);
    goto abort_swap;
    }
    p = (u32 *)out;
// Look for PLL active code from the DSP
    for (i = 0; i < len / 4; i++) {
    if (*p == 0x0e00ed0f) {
    dev_dbg(component.dev, "PLL packet received\n");
    wm0010.pll_running = true;
    break;
    }
    p++;
    }
    kfree(img_swap);
    kfree(out);
    } else
    dev_dbg(component.dev, "Not enabling DSP PLL.");
    ret = wm0010_firmware_load("wm0010.dfw", component);
    if (ret != 0)
    goto abort;
    spin_lock_irqsave(&wm0010.irq_lock, flags);
    wm0010.state = WM0010_FIRMWARE;
    spin_unlock_irqrestore(&wm0010.irq_lock, flags);
    mutex_unlock(&wm0010.lock);
    return 0;
    abort_swap:
    kfree(img_swap);
    abort_out:
    kfree(out);
    abort:
// Put the chip back into reset
    wm0010_halt(component);
    mutex_unlock(&wm0010.lock);
    return ret;
    err_core:
    mutex_unlock(&wm0010.lock);
    regulator_bulk_disable(ARRAY_SIZE(wm0010.core_supplies),
    wm0010.core_supplies);
    err:
    return ret;
    }
    static int wm0010_set_bias_level(struct snd_soc_component *component,
    enum snd_soc_bias_level level)
    {
    struct wm0010_priv *wm0010 = snd_soc_component_get_drvdata(component);
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(component);
    switch (level) {
    case SND_SOC_BIAS_ON:
    if (snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_PREPARE)
    wm0010_boot(component);
    break;
    case SND_SOC_BIAS_PREPARE:
    break;
    case SND_SOC_BIAS_STANDBY:
    if (snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_PREPARE) {
    scoped_guard(mutex, &wm0010.lock)
    wm0010_halt(component);
    }
    break;
    case SND_SOC_BIAS_OFF:
    break;
    }
    return 0;
    }
    static int wm0010_set_sysclk(struct snd_soc_component *component, int source,
    int clk_id, unsigned int freq, int dir)
    {
    struct wm0010_priv *wm0010 = snd_soc_component_get_drvdata(component);
    unsigned int i;
    wm0010.sysclk = freq;
    if (freq < pll_clock_map[ARRAY_SIZE(pll_clock_map)-1].max_sysclk) {
    wm0010.max_spi_freq = 0;
    } else {
    for (i = 0; i < ARRAY_SIZE(pll_clock_map); i++)
    if (freq >= pll_clock_map[i].max_sysclk) {
    wm0010.max_spi_freq = pll_clock_map[i].max_pll_spi_speed;
    wm0010.pll_clkctrl1 = pll_clock_map[i].pll_clkctrl1;
    break;
    }
    }
    return 0;
    }
    static int wm0010_probe(struct snd_soc_component *component);
    static const struct snd_soc_component_driver soc_component_dev_wm0010 = {
    .probe			= wm0010_probe,
    .set_bias_level		= wm0010_set_bias_level,
    .set_sysclk		= wm0010_set_sysclk,
    .dapm_widgets		= wm0010_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(wm0010_dapm_widgets),
    .dapm_routes		= wm0010_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(wm0010_dapm_routes),
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };

    SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE |\
    SNDRV_PCM_FMTBIT_S32_LE)
    static struct snd_soc_dai_driver wm0010_dai[] = {
    {
    .name = "wm0010-sdi1",
    .playback = {
    .stream_name = "SDI1 Playback",
    .channels_min = 1,
    .channels_max = 2,
    .rates = WM0010_RATES,
    .formats = WM0010_FORMATS,
    },
    .capture = {
    .stream_name = "SDI1 Capture",
    .channels_min = 1,
    .channels_max = 2,
    .rates = WM0010_RATES,
    .formats = WM0010_FORMATS,
    },
    },
    {
    .name = "wm0010-sdi2",
    .playback = {
    .stream_name = "SDI2 Playback",
    .channels_min = 1,
    .channels_max = 2,
    .rates = WM0010_RATES,
    .formats = WM0010_FORMATS,
    },
    .capture = {
    .stream_name = "SDI2 Capture",
    .channels_min = 1,
    .channels_max = 2,
    .rates = WM0010_RATES,
    .formats = WM0010_FORMATS,
    },
    },
    };
#[no_mangle]
unsafe extern "C" fn wm0010_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t wm0010_irq(int irq, void *data)
    {
    struct wm0010_priv *wm0010 = data;
    switch (wm0010.state) {
    case WM0010_OUT_OF_RESET:
    case WM0010_BOOTROM:
    case WM0010_STAGE2:
    scoped_guard(spinlock, &wm0010.irq_lock)
    complete(&wm0010.boot_completion);
    return IRQ_HANDLED;
    default:
    return IRQ_NONE;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn wm0010_probe(component: *mut snd_soc_component) -> c_int {
    static int wm0010_probe(struct snd_soc_component *component)
    {
    struct wm0010_priv *wm0010 = snd_soc_component_get_drvdata(component);
    wm0010.component = component;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wm0010_spi_probe(spi: *mut spi_device) -> c_int {
    static int wm0010_spi_probe(struct spi_device *spi)
    {
    int ret;
    int trigger;
    int irq;
    struct wm0010_priv *wm0010;
    wm0010 = devm_kzalloc(&spi.dev, sizeof(*wm0010),
    GFP_KERNEL);
    if (!wm0010)
    return -ENOMEM;
    mutex_init(&wm0010.lock);
    spin_lock_init(&wm0010.irq_lock);
    spi_set_drvdata(spi, wm0010);
    wm0010.dev = &spi.dev;
    if (dev_get_platdata(&spi.dev))
    memcpy(&wm0010.pdata, dev_get_platdata(&spi.dev),
    sizeof(wm0010.pdata));
    init_completion(&wm0010.boot_completion);
    wm0010.core_supplies[0].supply = "AVDD";
    wm0010.core_supplies[1].supply = "DCVDD";
    ret = devm_regulator_bulk_get(wm0010.dev, ARRAY_SIZE(wm0010.core_supplies),
    wm0010.core_supplies);
    if (ret != 0) {
    dev_err(wm0010.dev, "Failed to obtain core supplies: %d\n",
    ret);
    return ret;
    }
    wm0010.dbvdd = devm_regulator_get(wm0010.dev, "DBVDD");
    if (IS_ERR(wm0010.dbvdd)) {
    ret = PTR_ERR(wm0010.dbvdd);
    dev_err(wm0010.dev, "Failed to obtain DBVDD: %d\n", ret);
    return ret;
    }
    wm0010.reset = devm_gpiod_get(wm0010.dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(wm0010.reset))
    return dev_err_probe(wm0010.dev, PTR_ERR(wm0010.reset),
    "could not get RESET GPIO\n");
    gpiod_set_consumer_name(wm0010.reset, "wm0010 reset");
    wm0010.state = WM0010_POWER_OFF;
    irq = spi.irq;
    if (wm0010.pdata.irq_flags)
    trigger = wm0010.pdata.irq_flags;
    else
    trigger = IRQF_TRIGGER_FALLING;
    trigger |= IRQF_ONESHOT;
    ret = request_threaded_irq(irq, core::ptr::null_mut(), wm0010_irq, trigger,
    "wm0010", wm0010);
    if (ret) {
    dev_err(wm0010.dev, "Failed to request IRQ %d: %d\n",
    irq, ret);
    return ret;
    }
    wm0010.irq = irq;
    ret = irq_set_irq_wake(irq, 1);
    if (ret) {
    dev_err(wm0010.dev, "Failed to set IRQ %d as wake source: %d\n",
    irq, ret);
    goto free_irq;
    }
    if (spi.max_speed_hz)
    wm0010.board_max_spi_speed = spi.max_speed_hz;
    else
    wm0010.board_max_spi_speed = 0;
    ret = devm_snd_soc_register_component(&spi.dev,
    &soc_component_dev_wm0010, wm0010_dai,
    ARRAY_SIZE(wm0010_dai));
    if (ret < 0)
    goto disable_irq_wake;
    return 0;
    disable_irq_wake:
    irq_set_irq_wake(wm0010.irq, 0);
    free_irq:
    if (wm0010.irq)
    free_irq(wm0010.irq, wm0010);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wm0010_spi_remove(spi: *mut spi_device) {
    static void wm0010_spi_remove(struct spi_device *spi)
    {
    struct wm0010_priv *wm0010 = spi_get_drvdata(spi);
    gpiod_set_value_cansleep(wm0010.reset, 1);
    irq_set_irq_wake(wm0010.irq, 0);
    if (wm0010.irq)
    free_irq(wm0010.irq, wm0010);
    }
    static struct spi_driver wm0010_spi_driver = {
    .driver = {
    .name	= "wm0010",
    },
    .probe		= wm0010_spi_probe,
    .remove		= wm0010_spi_remove,
    };
    module_spi_driver(wm0010_spi_driver);
    MODULE_DESCRIPTION("ASoC WM0010 driver");
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE("wm0010.dfw");
    MODULE_FIRMWARE("wm0010_stage2.bin");
