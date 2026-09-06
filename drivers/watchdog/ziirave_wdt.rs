//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/ziirave_wdt.c
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
// Copyright (C) 2015 Zodiac Inflight Innovations
//
// Author: Martyn Welch <martyn.welch@collabora.co.uk>
//
// Based on twl4030_wdt.c by Timo Kokkonen <timo.t.kokkonen at nokia.com>:
//
// Copyright (C) Nokia Corporation
//

pub const ZIIRAVE_TIMEOUT_MIN: c_int = 3;
pub const ZIIRAVE_TIMEOUT_MAX: c_int = 255;
pub const ZIIRAVE_TIMEOUT_DEFAULT: c_int = 30;
pub const ZIIRAVE_PING_VALUE: c_uint = 0x0;
pub const ZIIRAVE_STATE_INITIAL: c_uint = 0x0;
pub const ZIIRAVE_STATE_OFF: c_uint = 0x1;
pub const ZIIRAVE_STATE_ON: c_uint = 0x2;

    static char *ziirave_reasons[] = {"power cycle", "hw watchdog", core::ptr::null_mut(), core::ptr::null_mut(),
    "host request", core::ptr::null_mut(), "illegal configuration",
    "illegal instruction", "illegal trap",
    "unknown"};
pub const ZIIRAVE_WDT_FIRM_VER_MAJOR: c_uint = 0x1;
pub const ZIIRAVE_WDT_BOOT_VER_MAJOR: c_uint = 0x3;
pub const ZIIRAVE_WDT_RESET_REASON: c_uint = 0x5;
pub const ZIIRAVE_WDT_STATE: c_uint = 0x6;
pub const ZIIRAVE_WDT_TIMEOUT: c_uint = 0x7;
pub const ZIIRAVE_WDT_TIME_LEFT: c_uint = 0x8;
pub const ZIIRAVE_WDT_PING: c_uint = 0x9;
pub const ZIIRAVE_WDT_RESET_DURATION: c_uint = 0xa;
pub const ZIIRAVE_FIRM_PKT_TOTAL_SIZE: c_int = 20;
pub const ZIIRAVE_FIRM_PKT_DATA_SIZE: c_int = 16;

pub const ZIIRAVE_FIRM_PAGE_SIZE: c_int = 128;
// Received and ready for next Download packet.
pub const ZIIRAVE_FIRM_DOWNLOAD_ACK: c_int = 1;
// Firmware commands
pub const ZIIRAVE_CMD_DOWNLOAD_START: c_uint = 0x10;
pub const ZIIRAVE_CMD_DOWNLOAD_END: c_uint = 0x11;
pub const ZIIRAVE_CMD_DOWNLOAD_SET_READ_ADDR: c_uint = 0x12;
pub const ZIIRAVE_CMD_DOWNLOAD_READ_BYTE: c_uint = 0x13;
pub const ZIIRAVE_CMD_RESET_PROCESSOR: c_uint = 0x0b;
pub const ZIIRAVE_CMD_JUMP_TO_BOOTLOADER: c_uint = 0x0c;
pub const ZIIRAVE_CMD_DOWNLOAD_PACKET: c_uint = 0x0e;
pub const ZIIRAVE_CMD_JUMP_TO_BOOTLOADER_MAGIC: c_int = 1;
pub const ZIIRAVE_CMD_RESET_PROCESSOR_MAGIC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ziirave_wdt_rev {
    pub major: c_uchar,
    pub minor: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ziirave_wdt_data {
    pub sysfs_mutex: mutex,
    pub wdd: watchdog_device,
    pub bootloader_rev: ziirave_wdt_rev,
    pub firmware_rev: ziirave_wdt_rev,
    pub reset_reason: c_int,
}

    static int wdt_timeout;
    module_param(wdt_timeout, int, 0);
    MODULE_PARM_DESC(wdt_timeout, "Watchdog timeout in seconds");
    static int reset_duration;
    module_param(reset_duration, int, 0);
    MODULE_PARM_DESC(reset_duration,
    "Watchdog reset pulse duration in milliseconds");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    static int ziirave_wdt_revision(struct i2c_client *client,
    struct ziirave_wdt_rev *rev, u8 command)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(client, command);
    if (ret < 0)
    return ret;
    rev.major = ret;
    ret = i2c_smbus_read_byte_data(client, command + 1);
    if (ret < 0)
    return ret;
    rev.minor = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ziirave_wdt_set_state(wdd: *mut watchdog_device, state: c_int) -> c_int {
    static int ziirave_wdt_set_state(struct watchdog_device *wdd, int state)
    {
    struct i2c_client *client = to_i2c_client(wdd.parent);
    return i2c_smbus_write_byte_data(client, ZIIRAVE_WDT_STATE, state);
    }
#[no_mangle]
unsafe extern "C" fn ziirave_wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int ziirave_wdt_start(struct watchdog_device *wdd)
    {
    return ziirave_wdt_set_state(wdd, ZIIRAVE_STATE_ON);
    }
#[no_mangle]
unsafe extern "C" fn ziirave_wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int ziirave_wdt_stop(struct watchdog_device *wdd)
    {
    return ziirave_wdt_set_state(wdd, ZIIRAVE_STATE_OFF);
    }
#[no_mangle]
unsafe extern "C" fn ziirave_wdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int ziirave_wdt_ping(struct watchdog_device *wdd)
    {
    struct i2c_client *client = to_i2c_client(wdd.parent);
    return i2c_smbus_write_byte_data(client, ZIIRAVE_WDT_PING,
    ZIIRAVE_PING_VALUE);
    }
    static int ziirave_wdt_set_timeout(struct watchdog_device *wdd,
    unsigned int timeout)
    {
    struct i2c_client *client = to_i2c_client(wdd.parent);
    int ret;
    ret = i2c_smbus_write_byte_data(client, ZIIRAVE_WDT_TIMEOUT, timeout);
    if (!ret)
    wdd.timeout = timeout;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ziirave_wdt_get_timeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int ziirave_wdt_get_timeleft(struct watchdog_device *wdd)
    {
    struct i2c_client *client = to_i2c_client(wdd.parent);
    int ret;
    ret = i2c_smbus_read_byte_data(client, ZIIRAVE_WDT_TIME_LEFT);
    if (ret < 0)
    ret = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ziirave_firm_read_ack(wdd: *mut watchdog_device) -> c_int {
    static int ziirave_firm_read_ack(struct watchdog_device *wdd)
    {
    struct i2c_client *client = to_i2c_client(wdd.parent);
    int ret;
    ret = i2c_smbus_read_byte(client);
    if (ret < 0) {
    dev_err(&client.dev, "Failed to read status byte\n");
    return ret;
    }
    let mut ret: return = = ZIIRAVE_FIRM_DOWNLOAD_ACK ? 0 : -EIO;
    }
#[no_mangle]
unsafe extern "C" fn ziirave_firm_set_read_addr(wdd: *mut watchdog_device, addr: u32) -> c_int {
    static int ziirave_firm_set_read_addr(struct watchdog_device *wdd, u32 addr)
    {
    struct i2c_client *client = to_i2c_client(wdd.parent);
    let mut addr16: u16 = (u16)addr / 2;
    u8 address[2];
    put_unaligned_le16(addr16, address);
    return i2c_smbus_write_block_data(client,
    ZIIRAVE_CMD_DOWNLOAD_SET_READ_ADDR,
    sizeof(address), address);
    }
#[no_mangle]
unsafe extern "C" fn ziirave_firm_addr_readonly(addr: u32) -> bool {
    static bool ziirave_firm_addr_readonly(u32 addr)
    {
    return addr < ZIIRAVE_FIRM_FLASH_MEMORY_START ||
    addr > ZIIRAVE_FIRM_FLASH_MEMORY_END;
    }
//
// ziirave_firm_write_pkt() - Build and write a firmware packet
//
// A packet to send to the firmware is composed by following bytes:
// Length | Addr0 | Addr1 | Data0 .. Data15 | Checksum |
// Where,
// Length: A data byte containing the length of the data.
// Addr0: Low byte of the address.
// Addr1: High byte of the address.
// Data0 .. Data15: Array of 16 bytes of data.
// Checksum: Checksum byte to verify data integrity.
//
    static int __ziirave_firm_write_pkt(struct watchdog_device *wdd,
    u32 addr, const u8 *data, u8 len)
    {
    let mut addr16: u16 = (u16)addr / 2;
    struct i2c_client *client = to_i2c_client(wdd.parent);
    u8 i, checksum = 0, packet[ZIIRAVE_FIRM_PKT_TOTAL_SIZE];
    int ret;
// Check max data size
    if (len > ZIIRAVE_FIRM_PKT_DATA_SIZE) {
    dev_err(&client.dev, "Firmware packet too long (%d)\n",
    len);
    return -EMSGSIZE;
    }
//
// Ignore packets that are targeting program memory outisde of
// app partition, since they will be ignored by the
// bootloader. At the same time, we need to make sure we'll
// allow zero length packet that will be sent as the last step
// of firmware update
//
    if (len && ziirave_firm_addr_readonly(addr))
    return 0;
// Packet length
    packet[0] = len;
// Packet address
    put_unaligned_le16(addr16, packet + 1);
    memcpy(packet + 3, data, len);
    memset(packet + 3 + len, 0, ZIIRAVE_FIRM_PKT_DATA_SIZE - len);
// Packet checksum
    for (i = 0; i < len + 3; i++)
    checksum += packet[i];
    packet[ZIIRAVE_FIRM_PKT_TOTAL_SIZE - 1] = checksum;
    ret = i2c_smbus_write_block_data(client, ZIIRAVE_CMD_DOWNLOAD_PACKET,
    sizeof(packet), packet);
    if (ret) {
    dev_err(&client.dev,
    "Failed to send DOWNLOAD_PACKET: %d\n", ret);
    return ret;
    }
    ret = ziirave_firm_read_ack(wdd);
    if (ret)
    dev_err(&client.dev,
    "Failed to write firmware packet at address 0x%04x: %d\n",
    addr, ret);
    return ret;
    }
    static int ziirave_firm_write_pkt(struct watchdog_device *wdd,
    u32 addr, const u8 *data, u8 len)
    {
    const u8 max_write_len = ZIIRAVE_FIRM_PAGE_SIZE -
    (addr - ALIGN_DOWN(addr, ZIIRAVE_FIRM_PAGE_SIZE));
    int ret;
    if (len > max_write_len) {
//
// If data crossed page boundary we need to split this
// write in two
//
    ret = __ziirave_firm_write_pkt(wdd, addr, data, max_write_len);
    if (ret)
    return ret;
    addr += max_write_len;
    data += max_write_len;
    len  -= max_write_len;
    }
    return __ziirave_firm_write_pkt(wdd, addr, data, len);
    }
    static int ziirave_firm_verify(struct watchdog_device *wdd,
    const struct firmware *fw)
    {
    struct i2c_client *client = to_i2c_client(wdd.parent);
    const struct ihex_binrec *rec;
    int i, ret;
    u8 data[ZIIRAVE_FIRM_PKT_DATA_SIZE];
    for (rec = (void *)fw.data; rec; rec = ihex_next_binrec(rec)) {
    let mut len: u16 = be16_to_cpu(rec.len);
    let mut addr: u32 = be32_to_cpu(rec.addr);
    if (len > sizeof(data))
    return -EINVAL;
    if (ziirave_firm_addr_readonly(addr))
    continue;
    ret = ziirave_firm_set_read_addr(wdd, addr);
    if (ret) {
    dev_err(&client.dev,
    "Failed to send SET_READ_ADDR command: %d\n",
    ret);
    return ret;
    }
    for (i = 0; i < len; i++) {
    ret = i2c_smbus_read_byte_data(client,
    ZIIRAVE_CMD_DOWNLOAD_READ_BYTE);
    if (ret < 0) {
    dev_err(&client.dev,
    "Failed to READ DATA: %d\n", ret);
    return ret;
    }
    data[i] = ret;
    }
    if (memcmp(data, rec.data, len)) {
    dev_err(&client.dev,
    "Firmware mismatch at address 0x%04x\n", addr);
    return -EINVAL;
    }
    }
    return 0;
    }
    static int ziirave_firm_upload(struct watchdog_device *wdd,
    const struct firmware *fw)
    {
    struct i2c_client *client = to_i2c_client(wdd.parent);
    const struct ihex_binrec *rec;
    int ret;
    ret = i2c_smbus_write_byte_data(client,
    ZIIRAVE_CMD_JUMP_TO_BOOTLOADER,
    ZIIRAVE_CMD_JUMP_TO_BOOTLOADER_MAGIC);
    if (ret) {
    dev_err(&client.dev, "Failed to jump to bootloader\n");
    return ret;
    }
    msleep(500);
    ret = i2c_smbus_write_byte(client, ZIIRAVE_CMD_DOWNLOAD_START);
    if (ret) {
    dev_err(&client.dev, "Failed to start download\n");
    return ret;
    }
    ret = ziirave_firm_read_ack(wdd);
    if (ret) {
    dev_err(&client.dev, "No ACK for start download\n");
    return ret;
    }
    msleep(500);
    for (rec = (void *)fw.data; rec; rec = ihex_next_binrec(rec)) {
    ret = ziirave_firm_write_pkt(wdd, be32_to_cpu(rec.addr),
    rec.data, be16_to_cpu(rec.len));
    if (ret)
    return ret;
    }
//
// Finish firmware download process by sending a zero length
// payload
//
    ret = ziirave_firm_write_pkt(wdd, 0, core::ptr::null_mut(), 0);
    if (ret) {
    dev_err(&client.dev, "Failed to send EMPTY packet: %d\n", ret);
    return ret;
    }
// This sleep seems to be required
    msleep(20);
// Start firmware verification
    ret = ziirave_firm_verify(wdd, fw);
    if (ret) {
    dev_err(&client.dev,
    "Failed to verify firmware: %d\n", ret);
    return ret;
    }
// End download operation
    ret = i2c_smbus_write_byte(client, ZIIRAVE_CMD_DOWNLOAD_END);
    if (ret) {
    dev_err(&client.dev,
    "Failed to end firmware download: %d\n", ret);
    return ret;
    }
// Reset the processor
    ret = i2c_smbus_write_byte_data(client,
    ZIIRAVE_CMD_RESET_PROCESSOR,
    ZIIRAVE_CMD_RESET_PROCESSOR_MAGIC);
    if (ret) {
    dev_err(&client.dev,
    "Failed to reset the watchdog: %d\n", ret);
    return ret;
    }
    msleep(500);
    return 0;
    }
    static const struct watchdog_info ziirave_wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .identity = "RAVE Switch Watchdog",
    };
    static const struct watchdog_ops ziirave_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= ziirave_wdt_start,
    .stop		= ziirave_wdt_stop,
    .ping		= ziirave_wdt_ping,
    .set_timeout	= ziirave_wdt_set_timeout,
    .get_timeleft	= ziirave_wdt_get_timeleft,
    };
    static ssize_t ziirave_wdt_sysfs_show_firm(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev.parent);
    struct ziirave_wdt_data *w_priv = i2c_get_clientdata(client);
    int ret;
    ret = mutex_lock_interruptible(&w_priv.sysfs_mutex);
    if (ret)
    return ret;
    ret = sysfs_emit(buf, "02.%02u.%02u\n",
    w_priv.firmware_rev.major,
    w_priv.firmware_rev.minor);
    mutex_unlock(&w_priv.sysfs_mutex);
    return ret;
    }
    static DEVICE_ATTR(firmware_version, S_IRUGO, ziirave_wdt_sysfs_show_firm,
    core::ptr::null_mut());
    static ssize_t ziirave_wdt_sysfs_show_boot(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev.parent);
    struct ziirave_wdt_data *w_priv = i2c_get_clientdata(client);
    int ret;
    ret = mutex_lock_interruptible(&w_priv.sysfs_mutex);
    if (ret)
    return ret;
    ret = sysfs_emit(buf, "01.%02u.%02u\n",
    w_priv.bootloader_rev.major,
    w_priv.bootloader_rev.minor);
    mutex_unlock(&w_priv.sysfs_mutex);
    return ret;
    }
    static DEVICE_ATTR(bootloader_version, S_IRUGO, ziirave_wdt_sysfs_show_boot,
    core::ptr::null_mut());
    static ssize_t ziirave_wdt_sysfs_show_reason(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct i2c_client *client = to_i2c_client(dev.parent);
    struct ziirave_wdt_data *w_priv = i2c_get_clientdata(client);
    int ret;
    ret = mutex_lock_interruptible(&w_priv.sysfs_mutex);
    if (ret)
    return ret;
    ret = sysfs_emit(buf, "%s\n", ziirave_reasons[w_priv.reset_reason]);
    mutex_unlock(&w_priv.sysfs_mutex);
    return ret;
    }
    static DEVICE_ATTR(reset_reason, S_IRUGO, ziirave_wdt_sysfs_show_reason,
    core::ptr::null_mut());
    static ssize_t ziirave_wdt_sysfs_store_firm(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct i2c_client *client = to_i2c_client(dev.parent);
    struct ziirave_wdt_data *w_priv = i2c_get_clientdata(client);
    const struct firmware *fw;
    int err;
    err = request_ihex_firmware(&fw, ZIIRAVE_FW_NAME, dev);
    if (err) {
    dev_err(&client.dev, "Failed to request ihex firmware\n");
    return err;
    }
    err = mutex_lock_interruptible(&w_priv.sysfs_mutex);
    if (err)
    goto release_firmware;
    err = ziirave_firm_upload(&w_priv.wdd, fw);
    if (err) {
    dev_err(&client.dev, "The firmware update failed: %d\n", err);
    goto unlock_mutex;
    }
// Update firmware version
    err = ziirave_wdt_revision(client, &w_priv.firmware_rev,
    ZIIRAVE_WDT_FIRM_VER_MAJOR);
    if (err) {
    dev_err(&client.dev, "Failed to read firmware version: %d\n",
    err);
    goto unlock_mutex;
    }
    dev_info(&client.dev,
    "Firmware updated to version 02.%02u.%02u\n",
    w_priv.firmware_rev.major, w_priv.firmware_rev.minor);
// Restore the watchdog timeout
    err = ziirave_wdt_set_timeout(&w_priv.wdd, w_priv.wdd.timeout);
    if (err)
    dev_err(&client.dev, "Failed to set timeout: %d\n", err);
    unlock_mutex:
    mutex_unlock(&w_priv.sysfs_mutex);
    release_firmware:
    release_firmware(fw);
    return err ? err : count;
    }
    static DEVICE_ATTR(update_firmware, S_IWUSR, core::ptr::null_mut(),
    ziirave_wdt_sysfs_store_firm);
    static struct attribute *ziirave_wdt_attrs[] = {
    &dev_attr_firmware_version.attr,
    &dev_attr_bootloader_version.attr,
    &dev_attr_reset_reason.attr,
    &dev_attr_update_firmware.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(ziirave_wdt);
#[no_mangle]
unsafe extern "C" fn ziirave_wdt_init_duration(client: *mut i2c_client) -> c_int {
    static int ziirave_wdt_init_duration(struct i2c_client *client)
    {
    int ret;
    if (!reset_duration) {
// See if the reset pulse duration is provided in an of_node
    if (!client.dev.of_node)
    ret = -ENODEV;
    else
    ret = of_property_read_u32(client.dev.of_node,
    "reset-duration-ms",
    &reset_duration);
    if (ret) {
    dev_info(&client.dev,
    "No reset pulse duration specified, using default\n");
    return 0;
    }
    }
    if (reset_duration < 1 || reset_duration > 255)
    return -EINVAL;
    dev_info(&client.dev, "Setting reset duration to %dms",
    reset_duration);
    return i2c_smbus_write_byte_data(client, ZIIRAVE_WDT_RESET_DURATION,
    reset_duration);
    }
#[no_mangle]
unsafe extern "C" fn ziirave_wdt_probe(client: *mut i2c_client) -> c_int {
    static int ziirave_wdt_probe(struct i2c_client *client)
    {
    int ret;
    struct ziirave_wdt_data *w_priv;
    int val;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE |
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_WRITE_BLOCK_DATA))
    return -ENODEV;
    w_priv = devm_kzalloc(&client.dev, sizeof(*w_priv), GFP_KERNEL);
    if (!w_priv)
    return -ENOMEM;
    mutex_init(&w_priv.sysfs_mutex);
    w_priv.wdd.info = &ziirave_wdt_info;
    w_priv.wdd.ops = &ziirave_wdt_ops;
    w_priv.wdd.min_timeout = ZIIRAVE_TIMEOUT_MIN;
    w_priv.wdd.max_timeout = ZIIRAVE_TIMEOUT_MAX;
    w_priv.wdd.parent = &client.dev;
    w_priv.wdd.groups = ziirave_wdt_groups;
    watchdog_init_timeout(&w_priv.wdd, wdt_timeout, &client.dev);
//
// The default value set in the watchdog should be perfectly valid, so
// pass that in if we haven't provided one via the module parameter or
// of property.
//
    if (w_priv.wdd.timeout == 0) {
    val = i2c_smbus_read_byte_data(client, ZIIRAVE_WDT_TIMEOUT);
    if (val < 0) {
    dev_err(&client.dev, "Failed to read timeout\n");
    return val;
    }
    if (val > ZIIRAVE_TIMEOUT_MAX ||
    val < ZIIRAVE_TIMEOUT_MIN)
    val = ZIIRAVE_TIMEOUT_DEFAULT;
    w_priv.wdd.timeout = val;
    }
    ret = ziirave_wdt_set_timeout(&w_priv.wdd, w_priv.wdd.timeout);
    if (ret) {
    dev_err(&client.dev, "Failed to set timeout\n");
    return ret;
    }
    dev_info(&client.dev, "Timeout set to %ds\n", w_priv.wdd.timeout);
    watchdog_set_nowayout(&w_priv.wdd, nowayout);
    i2c_set_clientdata(client, w_priv);
// If in unconfigured state, set to stopped
    val = i2c_smbus_read_byte_data(client, ZIIRAVE_WDT_STATE);
    if (val < 0) {
    dev_err(&client.dev, "Failed to read state\n");
    return val;
    }
    if (val == ZIIRAVE_STATE_INITIAL)
    ziirave_wdt_stop(&w_priv.wdd);
    ret = ziirave_wdt_init_duration(client);
    if (ret) {
    dev_err(&client.dev, "Failed to init duration\n");
    return ret;
    }
    ret = ziirave_wdt_revision(client, &w_priv.firmware_rev,
    ZIIRAVE_WDT_FIRM_VER_MAJOR);
    if (ret) {
    dev_err(&client.dev, "Failed to read firmware version\n");
    return ret;
    }
    dev_info(&client.dev,
    "Firmware version: 02.%02u.%02u\n",
    w_priv.firmware_rev.major, w_priv.firmware_rev.minor);
    ret = ziirave_wdt_revision(client, &w_priv.bootloader_rev,
    ZIIRAVE_WDT_BOOT_VER_MAJOR);
    if (ret) {
    dev_err(&client.dev, "Failed to read bootloader version\n");
    return ret;
    }
    dev_info(&client.dev,
    "Bootloader version: 01.%02u.%02u\n",
    w_priv.bootloader_rev.major, w_priv.bootloader_rev.minor);
    w_priv.reset_reason = i2c_smbus_read_byte_data(client,
    ZIIRAVE_WDT_RESET_REASON);
    if (w_priv.reset_reason < 0) {
    dev_err(&client.dev, "Failed to read reset reason\n");
    return w_priv.reset_reason;
    }
    if (w_priv.reset_reason >= ARRAY_SIZE(ziirave_reasons) ||
    !ziirave_reasons[w_priv.reset_reason]) {
    dev_err(&client.dev, "Invalid reset reason\n");
    return -ENODEV;
    }
    ret = watchdog_register_device(&w_priv.wdd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ziirave_wdt_remove(client: *mut i2c_client) {
    static void ziirave_wdt_remove(struct i2c_client *client)
    {
    struct ziirave_wdt_data *w_priv = i2c_get_clientdata(client);
    watchdog_unregister_device(&w_priv.wdd);
    }
    static const struct i2c_device_id ziirave_wdt_id[] = {
    { .name = "rave-wdt" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ziirave_wdt_id);
    static const struct of_device_id zrv_wdt_of_match[] = {
    { .compatible = "zii,rave-wdt", },
    { },
    };
    MODULE_DEVICE_TABLE(of, zrv_wdt_of_match);
    static struct i2c_driver ziirave_wdt_driver = {
    .driver = {
    .name = "ziirave_wdt",
    .of_match_table = zrv_wdt_of_match,
    },
    .probe = ziirave_wdt_probe,
    .remove = ziirave_wdt_remove,
    .id_table = ziirave_wdt_id,
    };
    module_i2c_driver(ziirave_wdt_driver);
    MODULE_AUTHOR("Martyn Welch <martyn.welch@collabora.co.uk");
    MODULE_DESCRIPTION("Zodiac Aerospace RAVE Switch Watchdog Processor Driver");
    MODULE_LICENSE("GPL");
