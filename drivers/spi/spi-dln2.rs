//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-dln2.c
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
// Driver for the Diolan DLN-2 USB-SPI adapter
//
// Copyright (c) 2014 Intel Corporation
//

pub const DLN2_SPI_MODULE_ID: c_uint = 0x02;

// SPI commands

pub const DLN2_SPI_MAX_XFER_SIZE: c_int = 256;

pub const DLN2_TRANSFERS_WAIT_COMPLETE: c_int = 1;
pub const DLN2_TRANSFERS_CANCEL: c_int = 0;
pub const DLN2_RPM_AUTOSUSPEND_TIMEOUT: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dln2_spi {
    pub pdev: *mut platform_device,
    pub host: *mut spi_controller,
    pub port: u8,
//
// This buffer will be used mainly for read/write operations. Since
// they're quite large, we cannot use the stack. Protection is not
// needed because all SPI communication is serialized by the SPI core.
//
    pub buf: *mut c_void,
    pub bpw: u8,
    pub speed: u32,
    pub mode: u16,
    pub cs: u8,
}

//
// Enable/Disable SPI module. The disable command will wait for transfers to
// complete first.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_enable(dln2: *mut dln2_spi, enable: bool) -> c_int {
    static int dln2_spi_enable(struct dln2_spi *dln2, bool enable)
    {
    u16 cmd;
    struct {
    u8 port;
    u8 wait_for_completion;
    } tx;
    let mut len: unsigned = sizeof(tx);
    tx.port = dln2.port;
    if (enable) {
    cmd = DLN2_SPI_ENABLE;
    len -= sizeof(tx.wait_for_completion);
    } else {
    tx.wait_for_completion = DLN2_TRANSFERS_WAIT_COMPLETE;
    cmd = DLN2_SPI_DISABLE;
    }
    return dln2_transfer_tx(dln2.pdev, cmd, &tx, len);
    }
//
// Select/unselect multiple CS lines. The selected lines will be automatically
// toggled LOW/HIGH by the board firmware during transfers, provided they're
// enabled first.
//
// Ex: cs_mask = 0x03 -> CS0 & CS1 will be selected and the next WR/RD operation
// will toggle the lines LOW/HIGH automatically.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_cs_set(dln2: *mut dln2_spi, cs_mask: u8) -> c_int {
    static int dln2_spi_cs_set(struct dln2_spi *dln2, u8 cs_mask)
    {
    struct {
    u8 port;
    u8 cs;
    } tx;
    tx.port = dln2.port;
//
// According to Diolan docs, "a slave device can be selected by changing
// the corresponding bit value to 0". The rest must be set to 1. Hence
// the bitwise NOT in front.
//
    tx.cs = ~cs_mask;
    return dln2_transfer_tx(dln2.pdev, DLN2_SPI_SET_SS, &tx, sizeof(tx));
    }
//
// Select one CS line. The other lines will be un-selected.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_cs_set_one(dln2: *mut dln2_spi, cs: u8) -> c_int {
    static int dln2_spi_cs_set_one(struct dln2_spi *dln2, u8 cs)
    {
    return dln2_spi_cs_set(dln2, BIT(cs));
    }
//
// Enable/disable CS lines for usage. The module has to be disabled first.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_cs_enable(dln2: *mut dln2_spi, cs_mask: u8, enable: bool) -> c_int {
    static int dln2_spi_cs_enable(struct dln2_spi *dln2, u8 cs_mask, bool enable)
    {
    struct {
    u8 port;
    u8 cs;
    } tx;
    u16 cmd;
    tx.port = dln2.port;
    tx.cs = cs_mask;
    cmd = enable ? DLN2_SPI_SS_MULTI_ENABLE : DLN2_SPI_SS_MULTI_DISABLE;
    return dln2_transfer_tx(dln2.pdev, cmd, &tx, sizeof(tx));
    }
#[no_mangle]
unsafe extern "C" fn dln2_spi_cs_enable_all(dln2: *mut dln2_spi, enable: bool) -> c_int {
    static int dln2_spi_cs_enable_all(struct dln2_spi *dln2, bool enable)
    {
    let mut cs_mask: u8 = GENMASK(dln2.host.num_chipselect - 1, 0);
    return dln2_spi_cs_enable(dln2, cs_mask, enable);
    }
#[no_mangle]
unsafe extern "C" fn dln2_spi_get_cs_num(dln2: *mut dln2_spi, cs_num: *mut u16) -> c_int {
    static int dln2_spi_get_cs_num(struct dln2_spi *dln2, u16 *cs_num)
    {
    int ret;
    struct {
    u8 port;
    } tx;
    struct {
    __le16 cs_count;
    } rx;
    let mut rx_len: unsigned = sizeof(rx);
    tx.port = dln2.port;
    ret = dln2_transfer(dln2.pdev, DLN2_SPI_GET_SS_COUNT, &tx, sizeof(tx),
    &rx, &rx_len);
    if (ret < 0)
    return ret;
    if (rx_len < sizeof(rx))
    return -EPROTO;
// cs_num = le16_to_cpu(rx.cs_count);
    dev_dbg(&dln2.pdev.dev, "cs_num = %d\n", *cs_num);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dln2_spi_get_speed(dln2: *mut dln2_spi, cmd: u16, freq: *mut u32) -> c_int {
    static int dln2_spi_get_speed(struct dln2_spi *dln2, u16 cmd, u32 *freq)
    {
    int ret;
    struct {
    u8 port;
    } tx;
    struct {
    __le32 speed;
    } rx;
    let mut rx_len: unsigned = sizeof(rx);
    tx.port = dln2.port;
    ret = dln2_transfer(dln2.pdev, cmd, &tx, sizeof(tx), &rx, &rx_len);
    if (ret < 0)
    return ret;
    if (rx_len < sizeof(rx))
    return -EPROTO;
// freq = le32_to_cpu(rx.speed);
    return 0;
    }
//
// Get bus min/max frequencies.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_get_speed_range(dln2: *mut dln2_spi, fmin: *mut u32, fmax: *mut u32) -> c_int {
    static int dln2_spi_get_speed_range(struct dln2_spi *dln2, u32 *fmin, u32 *fmax)
    {
    int ret;
    ret = dln2_spi_get_speed(dln2, DLN2_SPI_GET_MIN_FREQUENCY, fmin);
    if (ret < 0)
    return ret;
    ret = dln2_spi_get_speed(dln2, DLN2_SPI_GET_MAX_FREQUENCY, fmax);
    if (ret < 0)
    return ret;
    dev_dbg(&dln2.pdev.dev, "freq_min = %d, freq_max = %d\n",
// fmin, *fmax);
    return 0;
    }
//
// Set the bus speed. The module will automatically round down to the closest
// available frequency and returns it. The module has to be disabled first.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_set_speed(dln2: *mut dln2_spi, speed: u32) -> c_int {
    static int dln2_spi_set_speed(struct dln2_spi *dln2, u32 speed)
    {
    int ret;
    struct {
    u8 port;
    __le32 speed;
    } __packed tx;
    struct {
    __le32 speed;
    } rx;
    let mut rx_len: c_int = sizeof(rx);
    tx.port = dln2.port;
    tx.speed = cpu_to_le32(speed);
    ret = dln2_transfer(dln2.pdev, DLN2_SPI_SET_FREQUENCY, &tx, sizeof(tx),
    &rx, &rx_len);
    if (ret < 0)
    return ret;
    if (rx_len < sizeof(rx))
    return -EPROTO;
    return 0;
    }
//
// Change CPOL & CPHA. The module has to be disabled first.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_set_mode(dln2: *mut dln2_spi, mode: u8) -> c_int {
    static int dln2_spi_set_mode(struct dln2_spi *dln2, u8 mode)
    {
    struct {
    u8 port;
    u8 mode;
    } tx;
    tx.port = dln2.port;
    tx.mode = mode;
    return dln2_transfer_tx(dln2.pdev, DLN2_SPI_SET_MODE, &tx, sizeof(tx));
    }
//
// Change frame size. The module has to be disabled first.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_set_bpw(dln2: *mut dln2_spi, bpw: u8) -> c_int {
    static int dln2_spi_set_bpw(struct dln2_spi *dln2, u8 bpw)
    {
    struct {
    u8 port;
    u8 bpw;
    } tx;
    tx.port = dln2.port;
    tx.bpw = bpw;
    return dln2_transfer_tx(dln2.pdev, DLN2_SPI_SET_FRAME_SIZE,
    &tx, sizeof(tx));
    }
    static int dln2_spi_get_supported_frame_sizes(struct dln2_spi *dln2,
    u32 *bpw_mask)
    {
    int ret;
    struct {
    u8 port;
    } tx;
    struct {
    u8 count;
    u8 frame_sizes[36];
    } *rx = dln2.buf;
    let mut rx_len: unsigned = sizeof(*rx);
    int i;
    tx.port = dln2.port;
    ret = dln2_transfer(dln2.pdev, DLN2_SPI_GET_SUPPORTED_FRAME_SIZES,
    &tx, sizeof(tx), rx, &rx_len);
    if (ret < 0)
    return ret;
    if (rx_len < sizeof(*rx))
    return -EPROTO;
    if (rx.count > ARRAY_SIZE(rx.frame_sizes))
    return -EPROTO;
// bpw_mask = 0;
    for (i = 0; i < rx.count; i++)
// bpw_mask |= BIT(rx->frame_sizes[i] - 1);
    dev_dbg(&dln2.pdev.dev, "bpw_mask = 0x%X\n", *bpw_mask);
    return 0;
    }
//
// Copy the data to DLN2 buffer and change the byte order to LE, requested by
// DLN2 module. SPI core makes sure that the data length is a multiple of word
// size.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_copy_to_buf(dln2_buf: *mut u8, src: *const u8, len: u16, bpw: u8) -> c_int {
    static int dln2_spi_copy_to_buf(u8 *dln2_buf, const u8 *src, u16 len, u8 bpw)
    {

    memcpy(dln2_buf, src, len);

    if (bpw <= 8) {
    memcpy(dln2_buf, src, len);
    } else if (bpw <= 16) {
    __le16 *d = (__le16 *)dln2_buf;
    u16 *s = (u16 *)src;
    len = len / 2;
    while (len--)
// d++ = cpu_to_le16p(s++);
    } else {
    __le32 *d = (__le32 *)dln2_buf;
    u32 *s = (u32 *)src;
    len = len / 4;
    while (len--)
// d++ = cpu_to_le32p(s++);
    }

    return 0;
    }
//
// Copy the data from DLN2 buffer and convert to CPU byte order since the DLN2
// buffer is LE ordered. SPI core makes sure that the data length is a multiple
// of word size. The RX dln2_buf is 2 byte aligned so, for BE, we have to make
// sure we avoid unaligned accesses for 32 bit case.
//
#[no_mangle]
unsafe extern "C" fn dln2_spi_copy_from_buf(dest: *mut u8, dln2_buf: *const u8, len: u16, bpw: u8) -> c_int {
    static int dln2_spi_copy_from_buf(u8 *dest, const u8 *dln2_buf, u16 len, u8 bpw)
    {

    memcpy(dest, dln2_buf, len);

    if (bpw <= 8) {
    memcpy(dest, dln2_buf, len);
    } else if (bpw <= 16) {
    u16 *d = (u16 *)dest;
    __le16 *s = (__le16 *)dln2_buf;
    len = len / 2;
    while (len--)
// d++ = le16_to_cpup(s++);
    } else {
    u32 *d = (u32 *)dest;
    __le32 *s = (__le32 *)dln2_buf;
    len = len / 4;
    while (len--)
// d++ = get_unaligned_le32(s++);
    }

    return 0;
    }
//
// Perform one write operation.
//
    static int dln2_spi_write_one(struct dln2_spi *dln2, const u8 *data,
    u16 data_len, u8 attr)
    {
    struct {
    u8 port;
    __le16 size;
    u8 attr;
    u8 buf[DLN2_SPI_MAX_XFER_SIZE];
    } __packed *tx = dln2.buf;
    unsigned tx_len;
    BUILD_BUG_ON(sizeof(*tx) > DLN2_SPI_BUF_SIZE);
    if (data_len > DLN2_SPI_MAX_XFER_SIZE)
    return -EINVAL;
    tx.port = dln2.port;
    tx.size = cpu_to_le16(data_len);
    tx.attr = attr;
    dln2_spi_copy_to_buf(tx.buf, data, data_len, dln2.bpw);
    tx_len = sizeof(*tx) + data_len - DLN2_SPI_MAX_XFER_SIZE;
    return dln2_transfer_tx(dln2.pdev, DLN2_SPI_WRITE, tx, tx_len);
    }
//
// Perform one read operation.
//
    static int dln2_spi_read_one(struct dln2_spi *dln2, u8 *data,
    u16 data_len, u8 attr)
    {
    int ret;
    struct {
    u8 port;
    __le16 size;
    u8 attr;
    } __packed tx;
    struct {
    __le16 size;
    u8 buf[DLN2_SPI_MAX_XFER_SIZE];
    } __packed *rx = dln2.buf;
    let mut rx_len: unsigned = sizeof(*rx);
    BUILD_BUG_ON(sizeof(*rx) > DLN2_SPI_BUF_SIZE);
    if (data_len > DLN2_SPI_MAX_XFER_SIZE)
    return -EINVAL;
    tx.port = dln2.port;
    tx.size = cpu_to_le16(data_len);
    tx.attr = attr;
    ret = dln2_transfer(dln2.pdev, DLN2_SPI_READ, &tx, sizeof(tx),
    rx, &rx_len);
    if (ret < 0)
    return ret;
    if (rx_len < sizeof(rx.size) + data_len)
    return -EPROTO;
    if (le16_to_cpu(rx.size) != data_len)
    return -EPROTO;
    dln2_spi_copy_from_buf(data, rx.buf, data_len, dln2.bpw);
    return 0;
    }
//
// Perform one write & read operation.
//
    static int dln2_spi_read_write_one(struct dln2_spi *dln2, const u8 *tx_data,
    u8 *rx_data, u16 data_len, u8 attr)
    {
    int ret;
    struct {
    u8 port;
    __le16 size;
    u8 attr;
    u8 buf[DLN2_SPI_MAX_XFER_SIZE];
    } __packed *tx;
    struct {
    __le16 size;
    u8 buf[DLN2_SPI_MAX_XFER_SIZE];
    } __packed *rx;
    unsigned tx_len, rx_len;
    BUILD_BUG_ON(sizeof(*tx) > DLN2_SPI_BUF_SIZE ||
    sizeof(*rx) > DLN2_SPI_BUF_SIZE);
    if (data_len > DLN2_SPI_MAX_XFER_SIZE)
    return -EINVAL;
//
// Since this is a pseudo full-duplex communication, we're perfectly
// safe to use the same buffer for both tx and rx. When DLN2 sends the
// response back, with the rx data, we don't need the tx buffer anymore.
//
    tx = dln2.buf;
    rx = dln2.buf;
    tx.port = dln2.port;
    tx.size = cpu_to_le16(data_len);
    tx.attr = attr;
    dln2_spi_copy_to_buf(tx.buf, tx_data, data_len, dln2.bpw);
    tx_len = sizeof(*tx) + data_len - DLN2_SPI_MAX_XFER_SIZE;
    rx_len = sizeof(*rx);
    ret = dln2_transfer(dln2.pdev, DLN2_SPI_READ_WRITE, tx, tx_len,
    rx, &rx_len);
    if (ret < 0)
    return ret;
    if (rx_len < sizeof(rx.size) + data_len)
    return -EPROTO;
    if (le16_to_cpu(rx.size) != data_len)
    return -EPROTO;
    dln2_spi_copy_from_buf(rx_data, rx.buf, data_len, dln2.bpw);
    return 0;
    }
//
// Read/Write wrapper. It will automatically split an operation into multiple
// single ones due to device buffer constraints.
//
    static int dln2_spi_rdwr(struct dln2_spi *dln2, const u8 *tx_data,
    u8 *rx_data, u16 data_len, u8 attr)
    {
    int ret;
    u16 len;
    u8 temp_attr;
    let mut remaining: u16 = data_len;
    u16 offset;
    do {
    if (remaining > DLN2_SPI_MAX_XFER_SIZE) {
    len = DLN2_SPI_MAX_XFER_SIZE;
    temp_attr = DLN2_SPI_ATTR_LEAVE_SS_LOW;
    } else {
    len = remaining;
    temp_attr = attr;
    }
    offset = data_len - remaining;
    if (tx_data && rx_data) {
    ret = dln2_spi_read_write_one(dln2,
    tx_data + offset,
    rx_data + offset,
    len, temp_attr);
    } else if (tx_data) {
    ret = dln2_spi_write_one(dln2,
    tx_data + offset,
    len, temp_attr);
    } else if (rx_data) {
    ret = dln2_spi_read_one(dln2,
    rx_data + offset,
    len, temp_attr);
    } else {
    return -EINVAL;
    }
    if (ret < 0)
    return ret;
    remaining -= len;
    } while (remaining);
    return 0;
    }
    static int dln2_spi_prepare_message(struct spi_controller *host,
    struct spi_message *message)
    {
    int ret;
    struct dln2_spi *dln2 = spi_controller_get_devdata(host);
    struct spi_device *spi = message.spi;
    if (dln2.cs != spi_get_chipselect(spi, 0)) {
    ret = dln2_spi_cs_set_one(dln2, spi_get_chipselect(spi, 0));
    if (ret < 0)
    return ret;
    dln2.cs = spi_get_chipselect(spi, 0);
    }
    return 0;
    }
    static int dln2_spi_transfer_setup(struct dln2_spi *dln2, u32 speed,
    u8 bpw, u8 mode)
    {
    int ret;
    bool bus_setup_change;
    bus_setup_change = dln2.speed != speed || dln2.mode != mode ||
    dln2.bpw != bpw;
    if (!bus_setup_change)
    return 0;
    ret = dln2_spi_enable(dln2, false);
    if (ret < 0)
    return ret;
    if (dln2.speed != speed) {
    ret = dln2_spi_set_speed(dln2, speed);
    if (ret < 0)
    return ret;
    dln2.speed = speed;
    }
    if (dln2.mode != mode) {
    ret = dln2_spi_set_mode(dln2, mode & 0x3);
    if (ret < 0)
    return ret;
    dln2.mode = mode;
    }
    if (dln2.bpw != bpw) {
    ret = dln2_spi_set_bpw(dln2, bpw);
    if (ret < 0)
    return ret;
    dln2.bpw = bpw;
    }
    return dln2_spi_enable(dln2, true);
    }
    static int dln2_spi_transfer_one(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct dln2_spi *dln2 = spi_controller_get_devdata(host);
    int status;
    let mut attr: u8 = 0;
    status = dln2_spi_transfer_setup(dln2, xfer.speed_hz,
    xfer.bits_per_word,
    spi.mode);
    if (status < 0) {
    dev_err(&dln2.pdev.dev, "Cannot setup transfer\n");
    return status;
    }
    if (!xfer.cs_change && !spi_transfer_is_last(host, xfer))
    attr = DLN2_SPI_ATTR_LEAVE_SS_LOW;
    status = dln2_spi_rdwr(dln2, xfer.tx_buf, xfer.rx_buf,
    xfer.len, attr);
    if (status < 0)
    dev_err(&dln2.pdev.dev, "write/read failed!\n");
    return status;
    }
#[no_mangle]
unsafe extern "C" fn dln2_spi_probe(pdev: *mut platform_device) -> c_int {
    static int dln2_spi_probe(struct platform_device *pdev)
    {
    struct spi_controller *host;
    struct dln2_spi *dln2;
    struct dln2_platform_data *pdata = dev_get_platdata(&pdev.dev);
    int ret;
    host = devm_spi_alloc_host(&pdev.dev, sizeof(*dln2));
    if (!host)
    return -ENOMEM;
    platform_set_drvdata(pdev, host);
    dln2 = spi_controller_get_devdata(host);
    dln2.buf = devm_kmalloc(&pdev.dev, DLN2_SPI_BUF_SIZE, GFP_KERNEL);
    if (!dln2.buf)
    return -ENOMEM;
    dln2.host = host;
    dln2.pdev = pdev;
    dln2.port = pdata.port;
// cs/mode can never be 0xff, so the first transfer will set them
    dln2.cs = 0xff;
    dln2.mode = 0xff;
// disable SPI module before continuing with the setup
    ret = dln2_spi_enable(dln2, false);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to disable SPI module\n");
    return ret;
    }
    ret = dln2_spi_get_cs_num(dln2, &host.num_chipselect);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to get number of CS pins\n");
    return ret;
    }
    ret = dln2_spi_get_speed_range(dln2,
    &host.min_speed_hz,
    &host.max_speed_hz);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to read bus min/max freqs\n");
    return ret;
    }
    ret = dln2_spi_get_supported_frame_sizes(dln2,
    &host.bits_per_word_mask);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to read supported frame sizes\n");
    return ret;
    }
    ret = dln2_spi_cs_enable_all(dln2, true);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to enable CS pins\n");
    return ret;
    }
    host.bus_num = -1;
    host.mode_bits = SPI_CPOL | SPI_CPHA;
    host.prepare_message = dln2_spi_prepare_message;
    host.transfer_one = dln2_spi_transfer_one;
    host.auto_runtime_pm = true;
// enable SPI module, we're good to go
    ret = dln2_spi_enable(dln2, true);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to enable SPI module\n");
    return ret;
    }
    pm_runtime_set_autosuspend_delay(&pdev.dev,
    DLN2_RPM_AUTOSUSPEND_TIMEOUT);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ret = spi_register_controller(host);
    if (ret < 0) {
    dev_err(&pdev.dev, "Failed to register host\n");
    goto exit_register;
    }
    return ret;
    exit_register:
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    if (dln2_spi_enable(dln2, false) < 0)
    dev_err(&pdev.dev, "Failed to disable SPI module\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dln2_spi_remove(pdev: *mut platform_device) {
    static void dln2_spi_remove(struct platform_device *pdev)
    {
    struct spi_controller *host = platform_get_drvdata(pdev);
    struct dln2_spi *dln2 = spi_controller_get_devdata(host);
    spi_unregister_controller(host);
    pm_runtime_disable(&pdev.dev);
    if (dln2_spi_enable(dln2, false) < 0)
    dev_err(&pdev.dev, "Failed to disable SPI module\n");
    }
#[no_mangle]
unsafe extern "C" fn dln2_spi_suspend(dev: *mut device) -> c_int {
    static int dln2_spi_suspend(struct device *dev)
    {
    int ret;
    struct spi_controller *host = dev_get_drvdata(dev);
    struct dln2_spi *dln2 = spi_controller_get_devdata(host);
    ret = spi_controller_suspend(host);
    if (ret < 0)
    return ret;
    if (!pm_runtime_suspended(dev)) {
    ret = dln2_spi_enable(dln2, false);
    if (ret < 0)
    return ret;
    }
//
// USB power may be cut off during sleep. Resetting the following
// parameters will force the board to be set up before first transfer.
//
    dln2.cs = 0xff;
    dln2.speed = 0;
    dln2.bpw = 0;
    dln2.mode = 0xff;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dln2_spi_resume(dev: *mut device) -> c_int {
    static int dln2_spi_resume(struct device *dev)
    {
    int ret;
    struct spi_controller *host = dev_get_drvdata(dev);
    struct dln2_spi *dln2 = spi_controller_get_devdata(host);
    if (!pm_runtime_suspended(dev)) {
    ret = dln2_spi_cs_enable_all(dln2, true);
    if (ret < 0)
    return ret;
    ret = dln2_spi_enable(dln2, true);
    if (ret < 0)
    return ret;
    }
    return spi_controller_resume(host);
    }
#[no_mangle]
unsafe extern "C" fn dln2_spi_runtime_suspend(dev: *mut device) -> c_int {
    static int dln2_spi_runtime_suspend(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct dln2_spi *dln2 = spi_controller_get_devdata(host);
    return dln2_spi_enable(dln2, false);
    }
#[no_mangle]
unsafe extern "C" fn dln2_spi_runtime_resume(dev: *mut device) -> c_int {
    static int dln2_spi_runtime_resume(struct device *dev)
    {
    struct spi_controller *host = dev_get_drvdata(dev);
    struct dln2_spi *dln2 = spi_controller_get_devdata(host);
    return  dln2_spi_enable(dln2, true);
    }
    static const struct dev_pm_ops dln2_spi_pm = {
    SYSTEM_SLEEP_PM_OPS(dln2_spi_suspend, dln2_spi_resume)
    RUNTIME_PM_OPS(dln2_spi_runtime_suspend, dln2_spi_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver spi_dln2_driver = {
    .driver = {
    .name	= "dln2-spi",
    .pm	= pm_ptr(&dln2_spi_pm),
    },
    .probe		= dln2_spi_probe,
    .remove		= dln2_spi_remove,
    };
    module_platform_driver(spi_dln2_driver);
    MODULE_DESCRIPTION("Driver for the Diolan DLN2 SPI host interface");
    MODULE_AUTHOR("Laurentiu Palcu <laurentiu.palcu@intel.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:dln2-spi");
