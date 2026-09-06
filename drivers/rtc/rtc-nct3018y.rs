//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-nct3018y.c
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
// Copyright (c) 2022 Nuvoton Technology Corporation

pub const NCT3018Y_REG_SC: c_uint = 0x00 /* seconds */;
pub const NCT3018Y_REG_SCA: c_uint = 0x01 /* alarm */;
pub const NCT3018Y_REG_MN: c_uint = 0x02;
pub const NCT3018Y_REG_MNA: c_uint = 0x03 /* alarm */;
pub const NCT3018Y_REG_HR: c_uint = 0x04;
pub const NCT3018Y_REG_HRA: c_uint = 0x05 /* alarm */;
pub const NCT3018Y_REG_DW: c_uint = 0x06;
pub const NCT3018Y_REG_DM: c_uint = 0x07;
pub const NCT3018Y_REG_MO: c_uint = 0x08;
pub const NCT3018Y_REG_YR: c_uint = 0x09;
pub const NCT3018Y_REG_CTRL: c_uint = 0x0A /* timer control */;
pub const NCT3018Y_REG_ST: c_uint = 0x0B /* status */;
pub const NCT3018Y_REG_CLKO: c_uint = 0x0C /* clock out */;
pub const NCT3018Y_REG_PART: c_uint = 0x21 /* part info */;

pub const NCT3018Y_REG_BAT_MASK: c_uint = 0x07;
pub const NCT3018Y_REG_CLKO_F_MASK: c_uint = 0x03 /* frequenc mask */;
pub const NCT3018Y_REG_CLKO_CKE: c_uint = 0x80 /* clock out enabled */;
pub const NCT3018Y_REG_PART_NCT3018Y: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nct3018y {
    pub rtc: *mut rtc_device,
    pub client: *mut i2c_client,
    pub part_num: c_int,

    pub clkout_hw: clk_hw,

}

#[no_mangle]
unsafe extern "C" fn nct3018y_set_alarm_mode(client: *mut i2c_client, on: bool) -> c_int {
    static int nct3018y_set_alarm_mode(struct i2c_client *client, bool on)
    {
    int err, flags;
    dev_dbg(&client.dev, "%s:on:%d\n", __func__, on);
    flags =  i2c_smbus_read_byte_data(client, NCT3018Y_REG_CTRL);
    if (flags < 0) {
    dev_dbg(&client.dev,
    "Failed to read NCT3018Y_REG_CTRL\n");
    return flags;
    }
    if (on)
    flags |= NCT3018Y_BIT_AIE;
    else
    flags &= ~NCT3018Y_BIT_AIE;
    flags |= NCT3018Y_BIT_CIE;
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_CTRL, flags);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_CTRL\n");
    return err;
    }
    flags =  i2c_smbus_read_byte_data(client, NCT3018Y_REG_ST);
    if (flags < 0) {
    dev_dbg(&client.dev,
    "Failed to read NCT3018Y_REG_ST\n");
    return flags;
    }
    flags &= ~(NCT3018Y_BIT_AF);
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_ST, flags);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_ST\n");
    return err;
    }
    return 0;
    }
    static int nct3018y_get_alarm_mode(struct i2c_client *client, unsigned char *alarm_enable,
    unsigned char *alarm_flag)
    {
    int flags;
    if (alarm_enable) {
    dev_dbg(&client.dev, "%s:NCT3018Y_REG_CTRL\n", __func__);
    flags =  i2c_smbus_read_byte_data(client, NCT3018Y_REG_CTRL);
    if (flags < 0)
    return flags;
// alarm_enable = flags & NCT3018Y_BIT_AIE;
    dev_dbg(&client.dev, "%s:alarm_enable:%x\n", __func__, *alarm_enable);
    }
    if (alarm_flag) {
    dev_dbg(&client.dev, "%s:NCT3018Y_REG_ST\n", __func__);
    flags =  i2c_smbus_read_byte_data(client, NCT3018Y_REG_ST);
    if (flags < 0)
    return flags;
// alarm_flag = flags & NCT3018Y_BIT_AF;
    dev_dbg(&client.dev, "%s:alarm_flag:%x\n", __func__, *alarm_flag);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t nct3018y_irq(int irq, void *dev_id)
    {
    struct nct3018y *nct3018y = i2c_get_clientdata(dev_id);
    struct i2c_client *client = nct3018y.client;
    int err;
    unsigned char alarm_flag;
    unsigned char alarm_enable;
    dev_dbg(&client.dev, "%s:irq:%d\n", __func__, irq);
    err = nct3018y_get_alarm_mode(nct3018y.client, &alarm_enable, &alarm_flag);
    if (err)
    return IRQ_NONE;
    if (alarm_flag) {
    dev_dbg(&client.dev, "%s:alarm flag:%x\n",
    __func__, alarm_flag);
    rtc_update_irq(nct3018y.rtc, 1, RTC_IRQF | RTC_AF);
    nct3018y_set_alarm_mode(nct3018y.client, 0);
    dev_dbg(&client.dev, "%s:IRQ_HANDLED\n", __func__);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
//
// In the routines that deal directly with the nct3018y hardware, we use
// rtc_time -- month 0-11, hour 0-23, yr = calendar year-epoch.
//
#[no_mangle]
unsafe extern "C" fn nct3018y_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int nct3018y_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    unsigned char buf[10];
    int err;
    err = i2c_smbus_read_i2c_block_data(client, NCT3018Y_REG_ST, 1, buf);
    if (err < 0)
    return err;
    if (!buf[0]) {
    dev_dbg(&client.dev, " voltage <=1.7, date/time is not reliable.\n");
    return -EINVAL;
    }
    err = i2c_smbus_read_i2c_block_data(client, NCT3018Y_REG_SC, sizeof(buf), buf);
    if (err < 0)
    return err;
    tm.tm_sec = bcd2bin(buf[0] & 0x7F);
    tm.tm_min = bcd2bin(buf[2] & 0x7F);
    tm.tm_hour = bcd2bin(buf[4] & 0x3F);
    tm.tm_wday = buf[6] & 0x07;
    tm.tm_mday = bcd2bin(buf[7] & 0x3F);
    tm.tm_mon = bcd2bin(buf[8] & 0x1F) - 1;
    tm.tm_year = bcd2bin(buf[9]) + 100;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int nct3018y_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct nct3018y *nct3018y = dev_get_drvdata(dev);
    unsigned char buf[4] = {0};
    int err, flags;
    let mut restore_flags: c_int = 0;
    flags = i2c_smbus_read_byte_data(client, NCT3018Y_REG_CTRL);
    if (flags < 0) {
    dev_dbg(&client.dev, "Failed to read NCT3018Y_REG_CTRL.\n");
    return flags;
    }
// Check and set TWO bit
    if (nct3018y.part_num == NCT3018Y_REG_PART_NCT3018Y && !(flags & NCT3018Y_BIT_TWO)) {
    restore_flags = 1;
    flags |= NCT3018Y_BIT_TWO;
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_CTRL, flags);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_CTRL.\n");
    return err;
    }
    }
    buf[0] = bin2bcd(tm.tm_sec);
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_SC, buf[0]);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_SC\n");
    return err;
    }
    buf[0] = bin2bcd(tm.tm_min);
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_MN, buf[0]);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_MN\n");
    return err;
    }
    buf[0] = bin2bcd(tm.tm_hour);
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_HR, buf[0]);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_HR\n");
    return err;
    }
    buf[0] = tm.tm_wday & 0x07;
    buf[1] = bin2bcd(tm.tm_mday);
    buf[2] = bin2bcd(tm.tm_mon + 1);
    buf[3] = bin2bcd(tm.tm_year - 100);
    err = i2c_smbus_write_i2c_block_data(client, NCT3018Y_REG_DW,
    sizeof(buf), buf);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write for day and mon and year\n");
    return -EIO;
    }
// Restore TWO bit
    if (restore_flags) {
    if (nct3018y.part_num == NCT3018Y_REG_PART_NCT3018Y)
    flags &= ~NCT3018Y_BIT_TWO;
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_CTRL, flags);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_CTRL.\n");
    return err;
    }
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_rtc_read_alarm(dev: *mut device, tm: *mut rtc_wkalrm) -> c_int {
    static int nct3018y_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    unsigned char buf[5];
    int err;
    err = i2c_smbus_read_i2c_block_data(client, NCT3018Y_REG_SCA,
    sizeof(buf), buf);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to read date\n");
    return -EIO;
    }
    dev_dbg(&client.dev, "%s: raw data is sec=%02x, min=%02x hr=%02x\n",
    __func__, buf[0], buf[2], buf[4]);
    tm.time.tm_sec = bcd2bin(buf[0] & 0x7F);
    tm.time.tm_min = bcd2bin(buf[2] & 0x7F);
    tm.time.tm_hour = bcd2bin(buf[4] & 0x3F);
    err = nct3018y_get_alarm_mode(client, &tm.enabled, &tm.pending);
    if (err < 0)
    return err;
    dev_dbg(&client.dev, "%s:s=%d m=%d, hr=%d, enabled=%d, pending=%d\n",
    __func__, tm.time.tm_sec, tm.time.tm_min,
    tm.time.tm_hour, tm.enabled, tm.pending);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_rtc_set_alarm(dev: *mut device, tm: *mut rtc_wkalrm) -> c_int {
    static int nct3018y_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    int err;
    dev_dbg(dev, "%s, sec=%d, min=%d hour=%d tm.enabled:%d\n",
    __func__, tm.time.tm_sec, tm.time.tm_min, tm.time.tm_hour,
    tm.enabled);
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_SCA, bin2bcd(tm.time.tm_sec));
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_SCA\n");
    return err;
    }
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_MNA, bin2bcd(tm.time.tm_min));
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_MNA\n");
    return err;
    }
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_HRA, bin2bcd(tm.time.tm_hour));
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_HRA\n");
    return err;
    }
    return nct3018y_set_alarm_mode(client, tm.enabled);
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_irq_enable(dev: *mut device, enabled: c_uint) -> c_int {
    static int nct3018y_irq_enable(struct device *dev, unsigned int enabled)
    {
    dev_dbg(dev, "%s: alarm enable=%d\n", __func__, enabled);
    return nct3018y_set_alarm_mode(to_i2c_client(dev), enabled);
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_ioctl(dev: *mut device, cmd: c_uint, arg: c_ulong) -> c_int {
    static int nct3018y_ioctl(struct device *dev, unsigned int cmd, unsigned long arg)
    {
    struct i2c_client *client = to_i2c_client(dev);
    int status, flags = 0;
    switch (cmd) {
    case RTC_VL_READ:
    status = i2c_smbus_read_byte_data(client, NCT3018Y_REG_ST);
    if (status < 0)
    return status;
    if (!(status & NCT3018Y_REG_BAT_MASK))
    flags |= RTC_VL_DATA_INVALID;
    return put_user(flags, (unsigned int __user *)arg);
    default:
    return -ENOIOCTLCMD;
    }
    }

//
// Handling of the clkout
//

    static const int clkout_rates[] = {
    32768,
    1024,
    32,
    1,
    };
    static unsigned long nct3018y_clkout_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct nct3018y *nct3018y = clkout_hw_to_nct3018y(hw);
    struct i2c_client *client = nct3018y.client;
    int flags;
    flags = i2c_smbus_read_byte_data(client, NCT3018Y_REG_CLKO);
    if (flags < 0)
    return 0;
    flags &= NCT3018Y_REG_CLKO_F_MASK;
    return clkout_rates[flags];
    }
    static int nct3018y_clkout_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(clkout_rates); i++)
    if (clkout_rates[i] <= req.rate) {
    req.rate = clkout_rates[i];
    return 0;
    }
    req.rate = clkout_rates[0];
    return 0;
    }
    static int nct3018y_clkout_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct nct3018y *nct3018y = clkout_hw_to_nct3018y(hw);
    struct i2c_client *client = nct3018y.client;
    int i, flags;
    flags = i2c_smbus_read_byte_data(client, NCT3018Y_REG_CLKO);
    if (flags < 0)
    return flags;
    for (i = 0; i < ARRAY_SIZE(clkout_rates); i++)
    if (clkout_rates[i] == rate) {
    flags &= ~NCT3018Y_REG_CLKO_F_MASK;
    flags |= i;
    return i2c_smbus_write_byte_data(client, NCT3018Y_REG_CLKO, flags);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_clkout_control(hw: *mut clk_hw, enable: bool) -> c_int {
    static int nct3018y_clkout_control(struct clk_hw *hw, bool enable)
    {
    struct nct3018y *nct3018y = clkout_hw_to_nct3018y(hw);
    struct i2c_client *client = nct3018y.client;
    int flags;
    flags = i2c_smbus_read_byte_data(client, NCT3018Y_REG_CLKO);
    if (flags < 0)
    return flags;
    if (enable)
    flags |= NCT3018Y_REG_CLKO_CKE;
    else
    flags &= ~NCT3018Y_REG_CLKO_CKE;
    return i2c_smbus_write_byte_data(client, NCT3018Y_REG_CLKO, flags);
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_clkout_prepare(hw: *mut clk_hw) -> c_int {
    static int nct3018y_clkout_prepare(struct clk_hw *hw)
    {
    return nct3018y_clkout_control(hw, 1);
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_clkout_unprepare(hw: *mut clk_hw) {
    static void nct3018y_clkout_unprepare(struct clk_hw *hw)
    {
    nct3018y_clkout_control(hw, 0);
    }
#[no_mangle]
unsafe extern "C" fn nct3018y_clkout_is_prepared(hw: *mut clk_hw) -> c_int {
    static int nct3018y_clkout_is_prepared(struct clk_hw *hw)
    {
    struct nct3018y *nct3018y = clkout_hw_to_nct3018y(hw);
    struct i2c_client *client = nct3018y.client;
    int flags;
    flags = i2c_smbus_read_byte_data(client, NCT3018Y_REG_CLKO);
    if (flags < 0)
    return flags;
    return flags & NCT3018Y_REG_CLKO_CKE;
    }
    static const struct clk_ops nct3018y_clkout_ops = {
    .prepare = nct3018y_clkout_prepare,
    .unprepare = nct3018y_clkout_unprepare,
    .is_prepared = nct3018y_clkout_is_prepared,
    .recalc_rate = nct3018y_clkout_recalc_rate,
    .determine_rate = nct3018y_clkout_determine_rate,
    .set_rate = nct3018y_clkout_set_rate,
    };
    static struct clk *nct3018y_clkout_register_clk(struct nct3018y *nct3018y)
    {
    struct i2c_client *client = nct3018y.client;
    struct device_node *node = client.dev.of_node;
    let mut init: clk_init_data = {};
    struct clk *clk;
    init.name = "nct3018y-clkout";
    init.ops = &nct3018y_clkout_ops;
    init.flags = 0;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
    nct3018y.clkout_hw.init = &init;
// optional override of the clockname
    of_property_read_string(node, "clock-output-names", &init.name);
// register the clock
    clk = devm_clk_register(&client.dev, &nct3018y.clkout_hw);
    if (!IS_ERR(clk))
    of_clk_add_provider(node, of_clk_src_simple_get, clk);
    return clk;
    }

    static const struct rtc_class_ops nct3018y_rtc_ops = {
    .read_time	= nct3018y_rtc_read_time,
    .set_time	= nct3018y_rtc_set_time,
    .read_alarm	= nct3018y_rtc_read_alarm,
    .set_alarm	= nct3018y_rtc_set_alarm,
    .alarm_irq_enable = nct3018y_irq_enable,
    .ioctl		= nct3018y_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn nct3018y_probe(client: *mut i2c_client) -> c_int {
    static int nct3018y_probe(struct i2c_client *client)
    {
    struct nct3018y *nct3018y;
    int err, flags;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C |
    I2C_FUNC_SMBUS_BYTE |
    I2C_FUNC_SMBUS_BLOCK_DATA))
    return -ENODEV;
    nct3018y = devm_kzalloc(&client.dev, sizeof(struct nct3018y),
    GFP_KERNEL);
    if (!nct3018y)
    return -ENOMEM;
    i2c_set_clientdata(client, nct3018y);
    nct3018y.client = client;
    device_set_wakeup_capable(&client.dev, 1);
    flags = i2c_smbus_read_byte_data(client, NCT3018Y_REG_CTRL);
    if (flags < 0) {
    dev_dbg(&client.dev, "%s: read error\n", __func__);
    return flags;
    } else if (flags & NCT3018Y_BIT_TWO) {
    dev_dbg(&client.dev, "%s: NCT3018Y_BIT_TWO is set\n", __func__);
    }
    nct3018y.part_num = i2c_smbus_read_byte_data(client, NCT3018Y_REG_PART);
    if (nct3018y.part_num < 0) {
    dev_dbg(&client.dev, "Failed to read NCT3018Y_REG_PART.\n");
    return nct3018y.part_num;
    } else {
    nct3018y.part_num &= 0x03; /* Part number is corresponding to bit 0 and 1 */
    if (nct3018y.part_num == NCT3018Y_REG_PART_NCT3018Y) {
    flags = NCT3018Y_BIT_HF;
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_CTRL, flags);
    if (err < 0) {
    dev_dbg(&client.dev, "Unable to write NCT3018Y_REG_CTRL.\n");
    return err;
    }
    }
    }
    flags = 0;
    err = i2c_smbus_write_byte_data(client, NCT3018Y_REG_ST, flags);
    if (err < 0) {
    dev_dbg(&client.dev, "%s: write error\n", __func__);
    return err;
    }
    nct3018y.rtc = devm_rtc_allocate_device(&client.dev);
    if (IS_ERR(nct3018y.rtc))
    return PTR_ERR(nct3018y.rtc);
    nct3018y.rtc.ops = &nct3018y_rtc_ops;
    nct3018y.rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    nct3018y.rtc.range_max = RTC_TIMESTAMP_END_2099;
    if (client.irq > 0) {
    err = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), nct3018y_irq,
    IRQF_ONESHOT | IRQF_TRIGGER_FALLING,
    "nct3018y", client);
    if (err) {
    dev_dbg(&client.dev, "unable to request IRQ %d\n", client.irq);
    return err;
    }
    } else {
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, nct3018y.rtc.features);
    clear_bit(RTC_FEATURE_ALARM, nct3018y.rtc.features);
    }

// register clk in common clk framework
    nct3018y_clkout_register_clk(nct3018y);

    return devm_rtc_register_device(nct3018y.rtc);
    }
    static const struct i2c_device_id nct3018y_id[] = {
    { .name = "nct3018y" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, nct3018y_id);
    static const struct of_device_id nct3018y_of_match[] = {
    { .compatible = "nuvoton,nct3018y" },
    {}
    };
    MODULE_DEVICE_TABLE(of, nct3018y_of_match);
    static struct i2c_driver nct3018y_driver = {
    .driver		= {
    .name	= "rtc-nct3018y",
    .of_match_table = nct3018y_of_match,
    },
    .probe		= nct3018y_probe,
    .id_table	= nct3018y_id,
    };
    module_i2c_driver(nct3018y_driver);
    MODULE_AUTHOR("Medad CChien <ctcchien@nuvoton.com>");
    MODULE_AUTHOR("Mia Lin <mimi05633@gmail.com>");
    MODULE_DESCRIPTION("Nuvoton NCT3018Y RTC driver");
    MODULE_LICENSE("GPL");
