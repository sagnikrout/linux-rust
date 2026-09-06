//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-usbio.c
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
// Copyright (c) 2025 Intel Corporation.
// Copyright (c) 2025 Red Hat, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbio_i2c {
    pub adap: i2c_adapter,
    pub adev: *mut auxiliary_device,
    pub rwbuf: *mut usbio_i2c_rw,
    pub quirks: c_ulong,
    pub speed: u32,
    pub txbuf_len: u16,
    pub rxbuf_len: u16,
}

    static const struct acpi_device_id usbio_i2c_acpi_hids[] = {
    { "INTC1008" }, /* MTL */
    { "INTC10B3" }, /* ARL */
    { "INTC10B6" }, /* LNL */
    { "INTC10D2" }, /* MTL-CVF */
    { "INTC10E3" }, /* PTL */
    { "INTC1118" }, /* NVL */
    { }
    };
    static const u32 usbio_i2c_speeds[] = {
    I2C_MAX_STANDARD_MODE_FREQ,
    I2C_MAX_FAST_MODE_FREQ,
    I2C_MAX_FAST_MODE_PLUS_FREQ,
    I2C_MAX_HIGH_SPEED_MODE_FREQ
    };
#[no_mangle]
unsafe extern "C" fn usbio_i2c_uninit(adap: *mut i2c_adapter, msg: *mut i2c_msg) {
    static void usbio_i2c_uninit(struct i2c_adapter *adap, struct i2c_msg *msg)
    {
    struct usbio_i2c *i2c = i2c_get_adapdata(adap);
    struct usbio_i2c_uninit ubuf;
    ubuf.busid = i2c.adev.id;
    ubuf.config = cpu_to_le16(msg.addr);
    usbio_bulk_msg(i2c.adev, USBIO_PKTTYPE_I2C, USBIO_I2CCMD_UNINIT, true,
    &ubuf, sizeof(ubuf), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn usbio_i2c_init(adap: *mut i2c_adapter, msg: *mut i2c_msg) -> c_int {
    static int usbio_i2c_init(struct i2c_adapter *adap, struct i2c_msg *msg)
    {
    struct usbio_i2c *i2c = i2c_get_adapdata(adap);
    struct usbio_i2c_init ibuf;
    void *reply_buf;
    u16 reply_len;
    int ret;
    ibuf.busid = i2c.adev.id;
    ibuf.config = cpu_to_le16(msg.addr);
    ibuf.speed = cpu_to_le32(i2c.speed);
    if (i2c.quirks & USBIO_QUIRK_I2C_NO_INIT_ACK) {
    reply_buf = core::ptr::null_mut();
    reply_len = 0;
    } else {
    reply_buf = &ibuf;
    reply_len = sizeof(ibuf);
    }
    ret = usbio_bulk_msg(i2c.adev, USBIO_PKTTYPE_I2C, USBIO_I2CCMD_INIT, true,
    &ibuf, sizeof(ibuf), reply_buf, reply_len);
    if (ret != sizeof(ibuf))
    return (ret < 0) ? ret : -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbio_i2c_read(adap: *mut i2c_adapter, msg: *mut i2c_msg) -> c_int {
    static int usbio_i2c_read(struct i2c_adapter *adap, struct i2c_msg *msg)
    {
    struct usbio_i2c *i2c = i2c_get_adapdata(adap);
    let mut rxchunk: u16 = i2c.rxbuf_len - I2C_RW_OVERHEAD;
    struct usbio_i2c_rw *rbuf = i2c.rwbuf;
    int ret;
    rbuf.busid = i2c.adev.id;
    rbuf.config = cpu_to_le16(msg.addr);
    rbuf.size = cpu_to_le16(msg.len);
    if (msg.len > rxchunk) {
// Need to split the input buffer
    let mut len: u16 = 0;
    do {
    if (msg.len - len < rxchunk)
    rxchunk = msg.len - len;
    ret = usbio_bulk_msg(i2c.adev, USBIO_PKTTYPE_I2C,
    USBIO_I2CCMD_READ, true,
    rbuf, len == 0 ? sizeof(*rbuf) : 0,
    rbuf, sizeof(*rbuf) + rxchunk);
    if (ret < 0)
    return ret;
    memcpy(&msg.buf[len], rbuf.data, rxchunk);
    len += rxchunk;
    } while (msg.len > len);
    return 0;
    }
    ret = usbio_bulk_msg(i2c.adev, USBIO_PKTTYPE_I2C, USBIO_I2CCMD_READ, true,
    rbuf, sizeof(*rbuf), rbuf, sizeof(*rbuf) + msg.len);
    if (ret != sizeof(*rbuf) + msg.len)
    return (ret < 0) ? ret : -EIO;
    memcpy(msg.buf, rbuf.data, msg.len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbio_i2c_write(adap: *mut i2c_adapter, msg: *mut i2c_msg) -> c_int {
    static int usbio_i2c_write(struct i2c_adapter *adap, struct i2c_msg *msg)
    {
    struct usbio_i2c *i2c = i2c_get_adapdata(adap);
    let mut txchunk: u16 = i2c.txbuf_len - I2C_RW_OVERHEAD;
    struct usbio_i2c_rw *wbuf = i2c.rwbuf;
    int ret;
    if (msg.len > txchunk) {
// Need to split the output buffer
    let mut len: u16 = 0;
    do {
    wbuf.busid = i2c.adev.id;
    wbuf.config = cpu_to_le16(msg.addr);
    if (i2c.quirks & USBIO_QUIRK_I2C_USE_CHUNK_LEN)
    wbuf.size = cpu_to_le16(txchunk);
    else
    wbuf.size = cpu_to_le16(msg.len);
    memcpy(wbuf.data, &msg.buf[len], txchunk);
    len += txchunk;
    ret = usbio_bulk_msg(i2c.adev, USBIO_PKTTYPE_I2C,
    USBIO_I2CCMD_WRITE, msg.len == len,
    wbuf, sizeof(*wbuf) + txchunk,
    wbuf, sizeof(*wbuf));
    if (ret < 0)
    return ret;
    if (msg.len - len < txchunk)
    txchunk = msg.len - len;
    } while (msg.len > len);
    return 0;
    }
    wbuf.busid = i2c.adev.id;
    wbuf.config = cpu_to_le16(msg.addr);
    wbuf.size = cpu_to_le16(msg.len);
    memcpy(wbuf.data, msg.buf, msg.len);
    ret = usbio_bulk_msg(i2c.adev, USBIO_PKTTYPE_I2C, USBIO_I2CCMD_WRITE, true,
    wbuf, sizeof(*wbuf) + msg.len, wbuf, sizeof(*wbuf));
    if (ret != sizeof(*wbuf) || le16_to_cpu(wbuf.size) != msg.len)
    return (ret < 0) ? ret : -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbio_i2c_xfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int {
    static int usbio_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs, int num)
    {
    struct usbio_i2c *i2c = i2c_get_adapdata(adap);
    int ret;
    usbio_acquire(i2c.adev);
    ret = usbio_i2c_init(adap, msgs);
    if (ret)
    goto out_release;
    for (int i = 0; i < num; ret = ++i) {
    if (msgs[i].flags & I2C_M_RD)
    ret = usbio_i2c_read(adap, &msgs[i]);
    else
    ret = usbio_i2c_write(adap, &msgs[i]);
    if (ret)
    break;
    }
    usbio_i2c_uninit(adap, msgs);
    out_release:
    usbio_release(i2c.adev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn usbio_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 usbio_i2c_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_adapter_quirks usbio_i2c_quirks = {
    .flags = I2C_AQ_NO_ZERO_LEN | I2C_AQ_NO_REP_START,
    .max_read_len = SZ_4K,
    .max_write_len = SZ_4K,
    };
    static const struct i2c_adapter_quirks usbio_i2c_quirks_max_rw_len52 = {
    .flags = I2C_AQ_NO_ZERO_LEN | I2C_AQ_NO_REP_START,
    .max_read_len = 52,
    .max_write_len = 52,
    };
    static const struct i2c_algorithm usbio_i2c_algo = {
    .master_xfer = usbio_i2c_xfer,
    .functionality = usbio_i2c_func,
    };
    static int usbio_i2c_probe(struct auxiliary_device *adev,
    const struct auxiliary_device_id *adev_id)
    {
    struct usbio_i2c_bus_desc *i2c_desc;
    struct device *dev = &adev.dev;
    struct usbio_i2c *i2c;
    u32 max_speed;
    int ret;
    i2c_desc = dev_get_platdata(dev);
    if (!i2c_desc)
    return -EINVAL;
    i2c = devm_kzalloc(dev, sizeof(*i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    i2c.adev = adev;
    usbio_acpi_bind(i2c.adev, usbio_i2c_acpi_hids);
    usbio_get_txrxbuf_len(i2c.adev, &i2c.txbuf_len, &i2c.rxbuf_len);
    i2c.rwbuf = devm_kzalloc(dev, max(i2c.txbuf_len, i2c.rxbuf_len), GFP_KERNEL);
    if (!i2c.rwbuf)
    return -ENOMEM;
    i2c.quirks = usbio_get_quirks(i2c.adev);
    max_speed = usbio_i2c_speeds[i2c_desc.caps & USBIO_I2C_BUS_MODE_CAP_MASK];
    if (max_speed < I2C_MAX_FAST_MODE_FREQ &&
    (i2c.quirks & USBIO_QUIRK_I2C_ALLOW_400KHZ))
    max_speed = I2C_MAX_FAST_MODE_FREQ;
    i2c.speed = i2c_acpi_find_bus_speed(dev);
    if (!i2c.speed)
    i2c.speed = I2C_MAX_STANDARD_MODE_FREQ;
#[no_mangle]
pub unsafe extern "C" fn if(max_speed: i2c->speed >) -> else {
    dev_warn(dev, "Invalid speed %u adjusting to bus max %u\n",
    i2c.speed, max_speed);
    i2c.speed = max_speed;
    }
    i2c.adap.owner = THIS_MODULE;
    i2c.adap.class = I2C_CLASS_HWMON;
    i2c.adap.dev.parent = dev;
    i2c.adap.algo = &usbio_i2c_algo;
    if (i2c.quirks & USBIO_QUIRK_I2C_MAX_RW_LEN_52)
    i2c.adap.quirks = &usbio_i2c_quirks_max_rw_len52;
    else
    i2c.adap.quirks = &usbio_i2c_quirks;
    snprintf(i2c.adap.name, sizeof(i2c.adap.name), "%s.%d",
    USBIO_I2C_CLIENT, i2c.adev.id);
    device_set_node(&i2c.adap.dev, dev_fwnode(&adev.dev));
    auxiliary_set_drvdata(adev, i2c);
    i2c_set_adapdata(&i2c.adap, i2c);
    ret = i2c_add_adapter(&i2c.adap);
    if (ret)
    return ret;
    if (has_acpi_companion(&i2c.adap.dev))
    acpi_dev_clear_dependencies(ACPI_COMPANION(&i2c.adap.dev));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usbio_i2c_remove(adev: *mut auxiliary_device) {
    static void usbio_i2c_remove(struct auxiliary_device *adev)
    {
    struct usbio_i2c *i2c = auxiliary_get_drvdata(adev);
    i2c_del_adapter(&i2c.adap);
    }
    static const struct auxiliary_device_id usbio_i2c_id_table[] = {
    { "usbio.usbio-i2c" },
    { }
    };
    MODULE_DEVICE_TABLE(auxiliary, usbio_i2c_id_table);
    static struct auxiliary_driver usbio_i2c_driver = {
    .name = USBIO_I2C_CLIENT,
    .probe = usbio_i2c_probe,
    .remove = usbio_i2c_remove,
    .id_table = usbio_i2c_id_table
    };
    module_auxiliary_driver(usbio_i2c_driver);
    MODULE_DESCRIPTION("Intel USBIO I2C driver");
    MODULE_AUTHOR("Israel Cepeda <israel.a.cepeda.lopez@intel.com>");
    MODULE_AUTHOR("Hans de Goede <hansg@kernel.org>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("USBIO");
