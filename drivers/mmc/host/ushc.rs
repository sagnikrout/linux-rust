//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/ushc.c
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
// USB SD Host Controller (USHC) controller driver.
//
// Copyright (C) 2010 Cambridge Silicon Radio Ltd.
//
// Notes:
// - Only version 2 devices are supported.
// - Version 2 devices only support SDIO cards/devices (R2 response is
// unsupported).
//
// References:
// [USHC] USB SD Host Controller specification (CS-118793-SP)
//

    enum ushc_request {
    USHC_GET_CAPS  = 0x00,
    USHC_HOST_CTRL = 0x01,
    USHC_PWR_CTRL  = 0x02,
    USHC_CLK_FREQ  = 0x03,
    USHC_EXEC_CMD  = 0x04,
    USHC_READ_RESP = 0x05,
    USHC_RESET     = 0x06,
    };
    enum ushc_request_type {
    USHC_GET_CAPS_TYPE  = USB_DIR_IN  | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    USHC_HOST_CTRL_TYPE = USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    USHC_PWR_CTRL_TYPE  = USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    USHC_CLK_FREQ_TYPE  = USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    USHC_EXEC_CMD_TYPE  = USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    USHC_READ_RESP_TYPE = USB_DIR_IN  | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    USHC_RESET_TYPE     = USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    };
pub const USHC_GET_CAPS_VERSION_MASK: c_uint = 0xff;

pub const USHC_PWR_CTRL_OFF: c_uint = 0x00;
pub const USHC_PWR_CTRL_3V3: c_uint = 0x01;
pub const USHC_PWR_CTRL_3V0: c_uint = 0x02;
pub const USHC_PWR_CTRL_1V8: c_uint = 0x03;

pub const USHC_READ_RESP_ERR_MASK: c_uint = 0x0f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ushc_cbw {
    pub signature: __u8,
    pub cmd_idx: __u8,
    pub block_size: __le16,
    pub arg: __le32,
    pub __attribute__((packed)): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ushc_csw {
    pub signature: __u8,
    pub status: __u8,
    pub response: __le32,
    pub __attribute__((packed)): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ushc_int_data {
    pub status: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ushc_data {
    pub usb_dev: *mut usb_device,
    pub mmc: *mut mmc_host,
    pub int_urb: *mut urb,
    pub int_data: *mut ushc_int_data,
    pub cbw_urb: *mut urb,
    pub cbw: *mut ushc_cbw,
    pub data_urb: *mut urb,
    pub csw_urb: *mut urb,
    pub csw: *mut ushc_csw,
    pub lock: spinlock_t,
    pub current_req: *mut mmc_request,
    pub caps: u32,
    pub host_ctrl: u16,
    pub flags: c_ulong,
    pub last_status: u8,
    pub clock_freq: c_int,
}

pub const DISCONNECTED: c_int = 0;
pub const INT_EN: c_int = 1;
pub const IGNORE_NEXT_INT: c_int = 2;
    static void data_callback(struct urb *urb);
#[no_mangle]
unsafe extern "C" fn ushc_hw_reset(ushc: *mut ushc_data) -> c_int {
    static int ushc_hw_reset(struct ushc_data *ushc)
    {
    return usb_control_msg(ushc.usb_dev, usb_sndctrlpipe(ushc.usb_dev, 0),
    USHC_RESET, USHC_RESET_TYPE,
    0, 0, core::ptr::null_mut(), 0, 100);
    }
#[no_mangle]
unsafe extern "C" fn ushc_hw_get_caps(ushc: *mut ushc_data) -> c_int {
    static int ushc_hw_get_caps(struct ushc_data *ushc)
    {
    int ret;
    int version;
    ret = usb_control_msg(ushc.usb_dev, usb_rcvctrlpipe(ushc.usb_dev, 0),
    USHC_GET_CAPS, USHC_GET_CAPS_TYPE,
    0, 0, &ushc.caps, sizeof(ushc.caps), 100);
    if (ret < 0)
    return ret;
    ushc.caps = le32_to_cpu(ushc.caps);
    version = ushc.caps & USHC_GET_CAPS_VERSION_MASK;
    if (version != 0x02) {
    dev_err(&ushc.usb_dev.dev, "controller version %d is not supported\n", version);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ushc_hw_set_host_ctrl(ushc: *mut ushc_data, mask: u16, val: u16) -> c_int {
    static int ushc_hw_set_host_ctrl(struct ushc_data *ushc, u16 mask, u16 val)
    {
    u16 host_ctrl;
    int ret;
    host_ctrl = (ushc.host_ctrl & ~mask) | val;
    ret = usb_control_msg(ushc.usb_dev, usb_sndctrlpipe(ushc.usb_dev, 0),
    USHC_HOST_CTRL, USHC_HOST_CTRL_TYPE,
    host_ctrl, 0, core::ptr::null_mut(), 0, 100);
    if (ret < 0)
    return ret;
    ushc.host_ctrl = host_ctrl;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn int_callback(urb: *mut urb) {
    static void int_callback(struct urb *urb)
    {
    struct ushc_data *ushc = urb.context;
    u8 status, last_status;
    if (urb.status < 0)
    return;
    status = ushc.int_data.status;
    last_status = ushc.last_status;
    ushc.last_status = status;
//
// Ignore the card interrupt status on interrupt transfers that
// were submitted while card interrupts where disabled.
//
// This avoid occasional spurious interrupts when enabling
// interrupts immediately after clearing the source on the card.
//
    if (!test_and_clear_bit(IGNORE_NEXT_INT, &ushc.flags)
    && test_bit(INT_EN, &ushc.flags)
    && status & USHC_INT_STATUS_SDIO_INT) {
    mmc_signal_sdio_irq(ushc.mmc);
    }
    if ((status ^ last_status) & USHC_INT_STATUS_CARD_PRESENT)
    mmc_detect_change(ushc.mmc, msecs_to_jiffies(100));
    if (!test_bit(INT_EN, &ushc.flags))
    set_bit(IGNORE_NEXT_INT, &ushc.flags);
    usb_submit_urb(ushc.int_urb, GFP_ATOMIC);
    }
#[no_mangle]
unsafe extern "C" fn cbw_callback(urb: *mut urb) {
    static void cbw_callback(struct urb *urb)
    {
    struct ushc_data *ushc = urb.context;
    if (urb.status != 0) {
    usb_unlink_urb(ushc.data_urb);
    usb_unlink_urb(ushc.csw_urb);
    }
    }
#[no_mangle]
unsafe extern "C" fn data_callback(urb: *mut urb) {
    static void data_callback(struct urb *urb)
    {
    struct ushc_data *ushc = urb.context;
    if (urb.status != 0)
    usb_unlink_urb(ushc.csw_urb);
    }
#[no_mangle]
unsafe extern "C" fn csw_callback(urb: *mut urb) {
    static void csw_callback(struct urb *urb)
    {
    struct ushc_data *ushc = urb.context;
    struct mmc_request *req = ushc.current_req;
    int status;
    status = ushc.csw.status;
    if (urb.status != 0) {
    req.cmd.error = urb.status;
    } else if (status & USHC_READ_RESP_ERR_CMD) {
    if (status & USHC_READ_RESP_ERR_CRC)
    req.cmd.error = -EIO;
    else
    req.cmd.error = -ETIMEDOUT;
    }
    if (req.data) {
    if (status & USHC_READ_RESP_ERR_DAT) {
    if (status & USHC_READ_RESP_ERR_CRC)
    req.data.error = -EIO;
    else
    req.data.error = -ETIMEDOUT;
    req.data.bytes_xfered = 0;
    } else {
    req.data.bytes_xfered = req.data.blksz * req.data.blocks;
    }
    }
    req.cmd.resp[0] = le32_to_cpu(ushc.csw.response);
    mmc_request_done(ushc.mmc, req);
    }
#[no_mangle]
unsafe extern "C" fn ushc_request(mmc: *mut mmc_host, req: *mut mmc_request) {
    static void ushc_request(struct mmc_host *mmc, struct mmc_request *req)
    {
    struct ushc_data *ushc = mmc_priv(mmc);
    int ret;
    unsigned long flags;
    spin_lock_irqsave(&ushc.lock, flags);
    if (test_bit(DISCONNECTED, &ushc.flags)) {
    ret = -ENODEV;
    goto out;
    }
// Version 2 firmware doesn't support the R2 response format.
    if (req.cmd.flags & MMC_RSP_136) {
    ret = -EINVAL;
    goto out;
    }
// The Astoria's data FIFOs don't work with clock speeds < 5MHz so
    limit commands with data to 6MHz or more. */
    if (req.data && ushc.clock_freq < 6000000) {
    ret = -EINVAL;
    goto out;
    }
    ushc.current_req = req;
// Start cmd with CBW.
    ushc.cbw.cmd_idx = cpu_to_le16(req.cmd.opcode);
    if (req.data)
    ushc.cbw.block_size = cpu_to_le16(req.data.blksz);
    else
    ushc.cbw.block_size = 0;
    ushc.cbw.arg = cpu_to_le32(req.cmd.arg);
    ret = usb_submit_urb(ushc.cbw_urb, GFP_ATOMIC);
    if (ret < 0)
    goto out;
// Submit data (if any).
    if (req.data) {
    struct mmc_data *data = req.data;
    int pipe;
    if (data.flags & MMC_DATA_READ)
    pipe = usb_rcvbulkpipe(ushc.usb_dev, 6);
    else
    pipe = usb_sndbulkpipe(ushc.usb_dev, 2);
    usb_fill_bulk_urb(ushc.data_urb, ushc.usb_dev, pipe,
    core::ptr::null_mut(), data.sg.length,
    data_callback, ushc);
    ushc.data_urb.num_sgs = 1;
    ushc.data_urb.sg = data.sg;
    ret = usb_submit_urb(ushc.data_urb, GFP_ATOMIC);
    if (ret < 0)
    goto out;
    }
// Submit CSW.
    ret = usb_submit_urb(ushc.csw_urb, GFP_ATOMIC);
    out:
    spin_unlock_irqrestore(&ushc.lock, flags);
    if (ret < 0) {
    usb_unlink_urb(ushc.cbw_urb);
    usb_unlink_urb(ushc.data_urb);
    req.cmd.error = ret;
    mmc_request_done(mmc, req);
    }
    }
#[no_mangle]
unsafe extern "C" fn ushc_set_power(ushc: *mut ushc_data, power_mode: c_uchar) -> c_int {
    static int ushc_set_power(struct ushc_data *ushc, unsigned char power_mode)
    {
    u16 voltage;
    switch (power_mode) {
    case MMC_POWER_OFF:
    voltage = USHC_PWR_CTRL_OFF;
    break;
    case MMC_POWER_UP:
    case MMC_POWER_ON:
    voltage = USHC_PWR_CTRL_3V3;
    break;
    default:
    return -EINVAL;
    }
    return usb_control_msg(ushc.usb_dev, usb_sndctrlpipe(ushc.usb_dev, 0),
    USHC_PWR_CTRL, USHC_PWR_CTRL_TYPE,
    voltage, 0, core::ptr::null_mut(), 0, 100);
    }
#[no_mangle]
unsafe extern "C" fn ushc_set_bus_width(ushc: *mut ushc_data, bus_width: c_int) -> c_int {
    static int ushc_set_bus_width(struct ushc_data *ushc, int bus_width)
    {
    return ushc_hw_set_host_ctrl(ushc, USHC_HOST_CTRL_4BIT,
    bus_width == 4 ? USHC_HOST_CTRL_4BIT : 0);
    }
#[no_mangle]
unsafe extern "C" fn ushc_set_bus_freq(ushc: *mut ushc_data, clk: c_int, enable_hs: bool) -> c_int {
    static int ushc_set_bus_freq(struct ushc_data *ushc, int clk, bool enable_hs)
    {
    int ret;
// Hardware can't detect interrupts while the clock is off.
    if (clk == 0)
    clk = 400000;
    ret = ushc_hw_set_host_ctrl(ushc, USHC_HOST_CTRL_HIGH_SPD,
    enable_hs ? USHC_HOST_CTRL_HIGH_SPD : 0);
    if (ret < 0)
    return ret;
    ret = usb_control_msg(ushc.usb_dev, usb_sndctrlpipe(ushc.usb_dev, 0),
    USHC_CLK_FREQ, USHC_CLK_FREQ_TYPE,
    clk & 0xffff, (clk >> 16) & 0xffff, core::ptr::null_mut(), 0, 100);
    if (ret < 0)
    return ret;
    ushc.clock_freq = clk;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ushc_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void ushc_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct ushc_data *ushc = mmc_priv(mmc);
    ushc_set_power(ushc, ios.power_mode);
    ushc_set_bus_width(ushc, 1 << ios.bus_width);
    ushc_set_bus_freq(ushc, ios.clock, ios.timing == MMC_TIMING_SD_HS);
    }
#[no_mangle]
unsafe extern "C" fn ushc_get_cd(mmc: *mut mmc_host) -> c_int {
    static int ushc_get_cd(struct mmc_host *mmc)
    {
    struct ushc_data *ushc = mmc_priv(mmc);
    return !!(ushc.last_status & USHC_INT_STATUS_CARD_PRESENT);
    }
#[no_mangle]
unsafe extern "C" fn ushc_enable_sdio_irq(mmc: *mut mmc_host, enable: c_int) {
    static void ushc_enable_sdio_irq(struct mmc_host *mmc, int enable)
    {
    struct ushc_data *ushc = mmc_priv(mmc);
    if (enable)
    set_bit(INT_EN, &ushc.flags);
    else
    clear_bit(INT_EN, &ushc.flags);
    }
#[no_mangle]
unsafe extern "C" fn ushc_clean_up(ushc: *mut ushc_data) {
    static void ushc_clean_up(struct ushc_data *ushc)
    {
    usb_free_urb(ushc.int_urb);
    usb_free_urb(ushc.csw_urb);
    usb_free_urb(ushc.data_urb);
    usb_free_urb(ushc.cbw_urb);
    kfree(ushc.int_data);
    kfree(ushc.cbw);
    kfree(ushc.csw);
    }
    static const struct mmc_host_ops ushc_ops = {
    .request         = ushc_request,
    .set_ios         = ushc_set_ios,
    .get_cd          = ushc_get_cd,
    .enable_sdio_irq = ushc_enable_sdio_irq,
    };
#[no_mangle]
unsafe extern "C" fn ushc_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int ushc_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    struct usb_device *usb_dev = interface_to_usbdev(intf);
    struct mmc_host *mmc;
    struct ushc_data *ushc;
    int ret;
    if (intf.cur_altsetting.desc.bNumEndpoints < 1)
    return -ENODEV;
    mmc = devm_mmc_alloc_host(&intf.dev, sizeof(*ushc));
    if (mmc == core::ptr::null_mut())
    return -ENOMEM;
    ushc = mmc_priv(mmc);
    usb_set_intfdata(intf, ushc);
    ushc.usb_dev = usb_dev;
    ushc.mmc = mmc;
    spin_lock_init(&ushc.lock);
    ret = ushc_hw_reset(ushc);
    if (ret < 0)
    goto err;
// Read capabilities.
    ret = ushc_hw_get_caps(ushc);
    if (ret < 0)
    goto err;
    mmc.ops = &ushc_ops;
    mmc.f_min = 400000;
    mmc.f_max = 50000000;
    mmc.ocr_avail = MMC_VDD_32_33 | MMC_VDD_33_34;
    mmc.caps = MMC_CAP_4_BIT_DATA | MMC_CAP_SDIO_IRQ;
    mmc.caps |= (ushc.caps & USHC_GET_CAPS_HIGH_SPD) ? MMC_CAP_SD_HIGHSPEED : 0;
    mmc.max_seg_size  = 512*511;
    mmc.max_segs      = 1;
    mmc.max_req_size  = 512*511;
    mmc.max_blk_size  = 512;
    mmc.max_blk_count = 511;
    ushc.int_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (ushc.int_urb == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err;
    }
    ushc.int_data = kzalloc_obj(struct ushc_int_data);
    if (ushc.int_data == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err;
    }
    usb_fill_int_urb(ushc.int_urb, ushc.usb_dev,
    usb_rcvintpipe(usb_dev,
    intf.cur_altsetting.endpoint[0].desc.bEndpointAddress),
    ushc.int_data, sizeof(struct ushc_int_data),
    int_callback, ushc,
    intf.cur_altsetting.endpoint[0].desc.bInterval);
    ushc.cbw_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (ushc.cbw_urb == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err;
    }
    ushc.cbw = kzalloc_obj(struct ushc_cbw);
    if (ushc.cbw == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err;
    }
    ushc.cbw.signature = USHC_CBW_SIGNATURE;
    usb_fill_bulk_urb(ushc.cbw_urb, ushc.usb_dev, usb_sndbulkpipe(usb_dev, 2),
    ushc.cbw, sizeof(struct ushc_cbw),
    cbw_callback, ushc);
    ushc.data_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (ushc.data_urb == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err;
    }
    ushc.csw_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (ushc.csw_urb == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err;
    }
    ushc.csw = kzalloc_obj(struct ushc_csw);
    if (ushc.csw == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto err;
    }
    usb_fill_bulk_urb(ushc.csw_urb, ushc.usb_dev, usb_rcvbulkpipe(usb_dev, 6),
    ushc.csw, sizeof(struct ushc_csw),
    csw_callback, ushc);
    ret = mmc_add_host(ushc.mmc);
    if (ret)
    goto err;
    ret = usb_submit_urb(ushc.int_urb, GFP_KERNEL);
    if (ret < 0) {
    mmc_remove_host(ushc.mmc);
    goto err;
    }
    return 0;
    err:
    ushc_clean_up(ushc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ushc_disconnect(intf: *mut usb_interface) {
    static void ushc_disconnect(struct usb_interface *intf)
    {
    struct ushc_data *ushc = usb_get_intfdata(intf);
    spin_lock_irq(&ushc.lock);
    set_bit(DISCONNECTED, &ushc.flags);
    spin_unlock_irq(&ushc.lock);
    usb_kill_urb(ushc.int_urb);
    usb_kill_urb(ushc.cbw_urb);
    usb_kill_urb(ushc.data_urb);
    usb_kill_urb(ushc.csw_urb);
    mmc_remove_host(ushc.mmc);
    ushc_clean_up(ushc);
    }
    static struct usb_device_id ushc_id_table[] = {
// CSR USB SD Host Controller
    { USB_DEVICE(0x0a12, 0x5d10) },
    { },
    };
    MODULE_DEVICE_TABLE(usb, ushc_id_table);
    static struct usb_driver ushc_driver = {
    .name       = "ushc",
    .id_table   = ushc_id_table,
    .probe      = ushc_probe,
    .disconnect = ushc_disconnect,
    };
    module_usb_driver(ushc_driver);
    MODULE_DESCRIPTION("USB SD Host Controller driver");
    MODULE_AUTHOR("David Vrabel <david.vrabel@csr.com>");
    MODULE_LICENSE("GPL");
