//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-s35390a.c
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
// Seiko Instruments S-35390A RTC Driver
//
// Copyright (c) 2007 Byron Bradley
//

pub const S35390A_CMD_STATUS1: c_int = 0;
pub const S35390A_CMD_STATUS2: c_int = 1;
pub const S35390A_CMD_TIME1: c_int = 2;
pub const S35390A_CMD_TIME2: c_int = 3;
pub const S35390A_CMD_INT2_REG1: c_int = 5;
pub const S35390A_CMD_FREE_REG: c_int = 7;
pub const S35390A_BYTE_YEAR: c_int = 0;
pub const S35390A_BYTE_MONTH: c_int = 1;
pub const S35390A_BYTE_DAY: c_int = 2;
pub const S35390A_BYTE_WDAY: c_int = 3;
pub const S35390A_BYTE_HOURS: c_int = 4;
pub const S35390A_BYTE_MINS: c_int = 5;
pub const S35390A_BYTE_SECS: c_int = 6;
pub const S35390A_ALRM_BYTE_WDAY: c_int = 0;
pub const S35390A_ALRM_BYTE_HOURS: c_int = 1;
pub const S35390A_ALRM_BYTE_MINS: c_int = 2;
// flags for STATUS1

// flag for STATUS2

// INT2 pin output mode
pub const S35390A_INT2_MODE_MASK: c_uint = 0x0E;
pub const S35390A_INT2_MODE_NOINTR: c_uint = 0x00;

    static const struct i2c_device_id s35390a_id[] = {
    { .name = "s35390a" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, s35390a_id);
    static const __maybe_unused struct of_device_id s35390a_of_match[] = {
    { .compatible = "sii,s35390a" },
    { }
    };
    MODULE_DEVICE_TABLE(of, s35390a_of_match);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s35390a {
    pub client: [*mut i2c_client; 8],
    pub twentyfourhour: c_int,
}

#[no_mangle]
unsafe extern "C" fn s35390a_set_reg(s35390a: *mut s35390a, reg: c_int, buf: *mut u8, len: c_int) -> c_int {
    static int s35390a_set_reg(struct s35390a *s35390a, int reg, u8  *buf, int len)
    {
    struct i2c_client *client = s35390a.client[reg];
    struct i2c_msg msg[] = {
    {
    .addr = client.addr,
    .len = len,
    .buf = buf
    },
    };
    if ((i2c_transfer(client.adapter, msg, 1)) != 1)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s35390a_get_reg(s35390a: *mut s35390a, reg: c_int, buf: *mut u8, len: c_int) -> c_int {
    static int s35390a_get_reg(struct s35390a *s35390a, int reg, u8 *buf, int len)
    {
    struct i2c_client *client = s35390a.client[reg];
    struct i2c_msg msg[] = {
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = len,
    .buf = buf
    },
    };
    if ((i2c_transfer(client.adapter, msg, 1)) != 1)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s35390a_init(s35390a: *mut s35390a) -> c_int {
    static int s35390a_init(struct s35390a *s35390a)
    {
    u8 buf;
    int ret;
    let mut initcount: unsigned = 0;
//
// At least one of POC and BLD are set, so reinitialise chip. Keeping
// this information in the hardware to know later that the time isn't
// valid is unfortunately not possible because POC and BLD are cleared
// on read. So the reset is best done now.
//
// The 24H bit is kept over reset, so set it already here.
//
    initialize:
    buf = S35390A_FLAG_RESET | S35390A_FLAG_24H;
    ret = s35390a_set_reg(s35390a, S35390A_CMD_STATUS1, &buf, 1);
    if (ret < 0)
    return ret;
    ret = s35390a_get_reg(s35390a, S35390A_CMD_STATUS1, &buf, 1);
    if (ret < 0)
    return ret;
    if (buf & (S35390A_FLAG_POC | S35390A_FLAG_BLD)) {
// Try up to five times to reset the chip
    if (initcount < 5) {
    ++initcount;
    goto initialize;
    } else
    return -EIO;
    }
    return 1;
    }
//
// Returns <0 on error, 0 if rtc is setup fine and 1 if the chip was reset.
// To keep the information if an irq is pending, pass the value read from
// STATUS1 to the caller.
//
#[no_mangle]
unsafe extern "C" fn s35390a_read_status(s35390a: *mut s35390a, status1: *mut c_char) -> c_int {
    static int s35390a_read_status(struct s35390a *s35390a, char *status1)
    {
    int ret;
    ret = s35390a_get_reg(s35390a, S35390A_CMD_STATUS1, status1, 1);
    if (ret < 0)
    return ret;
    if (*status1 & S35390A_FLAG_POC) {
//
// Do not communicate for 0.5 seconds since the power-on
// detection circuit is in operation.
//
    msleep(500);
    return 1;
    } else if (*status1 & S35390A_FLAG_BLD)
    return 1;
//
// If both POC and BLD are unset everything is fine.
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s35390a_disable_test_mode(s35390a: *mut s35390a) -> c_int {
    static int s35390a_disable_test_mode(struct s35390a *s35390a)
    {
    u8 buf[1];
    if (s35390a_get_reg(s35390a, S35390A_CMD_STATUS2, buf, sizeof(buf)) < 0)
    return -EIO;
    if (!(buf[0] & S35390A_FLAG_TEST))
    return 0;
    buf[0] &= ~S35390A_FLAG_TEST;
    return s35390a_set_reg(s35390a, S35390A_CMD_STATUS2, buf, sizeof(buf));
    }
#[no_mangle]
unsafe extern "C" fn s35390a_hr2reg(s35390a: *mut s35390a, hour: c_int) -> c_char {
    static char s35390a_hr2reg(struct s35390a *s35390a, int hour)
    {
    if (s35390a.twentyfourhour)
    return bin2bcd(hour);
    if (hour < 12)
    return bin2bcd(hour);
    return 0x40 | bin2bcd(hour - 12);
    }
#[no_mangle]
unsafe extern "C" fn s35390a_reg2hr(s35390a: *mut s35390a, reg: c_char) -> c_int {
    static int s35390a_reg2hr(struct s35390a *s35390a, char reg)
    {
    unsigned hour;
    if (s35390a.twentyfourhour)
    return bcd2bin(reg & 0x3f);
    hour = bcd2bin(reg & 0x3f);
    if (reg & 0x40)
    hour += 12;
    return hour;
    }
#[no_mangle]
unsafe extern "C" fn s35390a_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int s35390a_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct s35390a	*s35390a = i2c_get_clientdata(client);
    int i;
    u8 buf[7], status;
    dev_dbg(&client.dev, "%s: tm is secs=%d, mins=%d, hours=%d mday=%d, "
    "mon=%d, year=%d, wday=%d\n", __func__, tm.tm_sec,
    tm.tm_min, tm.tm_hour, tm.tm_mday, tm.tm_mon, tm.tm_year,
    tm.tm_wday);
    if (s35390a_read_status(s35390a, &status) == 1)
    s35390a_init(s35390a);
    buf[S35390A_BYTE_YEAR] = bin2bcd(tm.tm_year - 100);
    buf[S35390A_BYTE_MONTH] = bin2bcd(tm.tm_mon + 1);
    buf[S35390A_BYTE_DAY] = bin2bcd(tm.tm_mday);
    buf[S35390A_BYTE_WDAY] = bin2bcd(tm.tm_wday);
    buf[S35390A_BYTE_HOURS] = s35390a_hr2reg(s35390a, tm.tm_hour);
    buf[S35390A_BYTE_MINS] = bin2bcd(tm.tm_min);
    buf[S35390A_BYTE_SECS] = bin2bcd(tm.tm_sec);
// This chip expects the bits of each byte to be in reverse order
    for (i = 0; i < 7; ++i)
    buf[i] = bitrev8(buf[i]);
    return s35390a_set_reg(s35390a, S35390A_CMD_TIME1, buf, sizeof(buf));
    }
#[no_mangle]
unsafe extern "C" fn s35390a_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int s35390a_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct s35390a *s35390a = i2c_get_clientdata(client);
    u8 buf[7], status;
    int i, err;
    if (s35390a_read_status(s35390a, &status) == 1)
    return -EINVAL;
    err = s35390a_get_reg(s35390a, S35390A_CMD_TIME1, buf, sizeof(buf));
    if (err < 0)
    return err;
// This chip returns the bits of each byte in reverse order
    for (i = 0; i < 7; ++i)
    buf[i] = bitrev8(buf[i]);
    tm.tm_sec = bcd2bin(buf[S35390A_BYTE_SECS]);
    tm.tm_min = bcd2bin(buf[S35390A_BYTE_MINS]);
    tm.tm_hour = s35390a_reg2hr(s35390a, buf[S35390A_BYTE_HOURS]);
    tm.tm_wday = bcd2bin(buf[S35390A_BYTE_WDAY]);
    tm.tm_mday = bcd2bin(buf[S35390A_BYTE_DAY]);
    tm.tm_mon = bcd2bin(buf[S35390A_BYTE_MONTH]) - 1;
    tm.tm_year = bcd2bin(buf[S35390A_BYTE_YEAR]) + 100;
    dev_dbg(&client.dev, "%s: tm is secs=%d, mins=%d, hours=%d, mday=%d, "
    "mon=%d, year=%d, wday=%d\n", __func__, tm.tm_sec,
    tm.tm_min, tm.tm_hour, tm.tm_mday, tm.tm_mon, tm.tm_year,
    tm.tm_wday);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s35390a_rtc_set_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int s35390a_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct s35390a *s35390a = i2c_get_clientdata(client);
    u8 buf[3], sts = 0;
    int err, i;
    dev_dbg(&client.dev, "%s: alm is secs=%d, mins=%d, hours=%d mday=%d, "\
    "mon=%d, year=%d, wday=%d\n", __func__, alm.time.tm_sec,
    alm.time.tm_min, alm.time.tm_hour, alm.time.tm_mday,
    alm.time.tm_mon, alm.time.tm_year, alm.time.tm_wday);
// disable interrupt (which deasserts the irq line)
    err = s35390a_set_reg(s35390a, S35390A_CMD_STATUS2, &sts, sizeof(sts));
    if (err < 0)
    return err;
// clear pending interrupt (in STATUS1 only), if any
    err = s35390a_get_reg(s35390a, S35390A_CMD_STATUS1, &sts, sizeof(sts));
    if (err < 0)
    return err;
    if (alm.enabled)
    sts = S35390A_INT2_MODE_ALARM;
    else
    sts = S35390A_INT2_MODE_NOINTR;
// set interrupt mode
    err = s35390a_set_reg(s35390a, S35390A_CMD_STATUS2, &sts, sizeof(sts));
    if (err < 0)
    return err;
    if (alm.time.tm_wday != -1)
    buf[S35390A_ALRM_BYTE_WDAY] = bin2bcd(alm.time.tm_wday) | 0x80;
    else
    buf[S35390A_ALRM_BYTE_WDAY] = 0;
    buf[S35390A_ALRM_BYTE_HOURS] = s35390a_hr2reg(s35390a,
    alm.time.tm_hour) | 0x80;
    buf[S35390A_ALRM_BYTE_MINS] = bin2bcd(alm.time.tm_min) | 0x80;
    if (alm.time.tm_hour >= 12)
    buf[S35390A_ALRM_BYTE_HOURS] |= 0x40;
    for (i = 0; i < 3; ++i)
    buf[i] = bitrev8(buf[i]);
    err = s35390a_set_reg(s35390a, S35390A_CMD_INT2_REG1, buf,
    sizeof(buf));
    return err;
    }
#[no_mangle]
unsafe extern "C" fn s35390a_rtc_read_alarm(dev: *mut device, alm: *mut rtc_wkalrm) -> c_int {
    static int s35390a_rtc_read_alarm(struct device *dev, struct rtc_wkalrm *alm)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct s35390a *s35390a = i2c_get_clientdata(client);
    u8 buf[3], sts;
    int i, err;
    err = s35390a_get_reg(s35390a, S35390A_CMD_STATUS2, &sts, sizeof(sts));
    if (err < 0)
    return err;
    if ((sts & S35390A_INT2_MODE_MASK) != S35390A_INT2_MODE_ALARM) {
//
// When the alarm isn't enabled, the register to configure
// the alarm time isn't accessible.
//
    alm.enabled = 0;
    return 0;
    } else {
    alm.enabled = 1;
    }
    err = s35390a_get_reg(s35390a, S35390A_CMD_INT2_REG1, buf, sizeof(buf));
    if (err < 0)
    return err;
// This chip returns the bits of each byte in reverse order
    for (i = 0; i < 3; ++i)
    buf[i] = bitrev8(buf[i]);
//
// B0 of the three matching registers is an enable flag. Iff it is set
// the configured value is used for matching.
//
    if (buf[S35390A_ALRM_BYTE_WDAY] & 0x80)
    alm.time.tm_wday =
    bcd2bin(buf[S35390A_ALRM_BYTE_WDAY] & ~0x80);
    if (buf[S35390A_ALRM_BYTE_HOURS] & 0x80)
    alm.time.tm_hour =
    s35390a_reg2hr(s35390a,
    buf[S35390A_ALRM_BYTE_HOURS] & ~0x80);
    if (buf[S35390A_ALRM_BYTE_MINS] & 0x80)
    alm.time.tm_min = bcd2bin(buf[S35390A_ALRM_BYTE_MINS] & ~0x80);
// alarm triggers always at s=0
    alm.time.tm_sec = 0;
    dev_dbg(&client.dev, "%s: alm is mins=%d, hours=%d, wday=%d\n",
    __func__, alm.time.tm_min, alm.time.tm_hour,
    alm.time.tm_wday);
    return 0;
    }
    static int s35390a_rtc_ioctl(struct device *dev, unsigned int cmd,
    unsigned long arg)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct s35390a *s35390a = i2c_get_clientdata(client);
    u8 sts;
    int err;
    switch (cmd) {
    case RTC_VL_READ:
// s35390a_reset set lowvoltage flag and init RTC if needed
    err = s35390a_read_status(s35390a, &sts);
    if (err < 0)
    return err;
    if (copy_to_user((void __user *)arg, &err, sizeof(int)))
    return -EFAULT;
    break;
    case RTC_VL_CLR:
// update flag and clear register
    err = s35390a_init(s35390a);
    if (err < 0)
    return err;
    break;
    default:
    return -ENOIOCTLCMD;
    }
    return 0;
    }
    static const struct rtc_class_ops s35390a_rtc_ops = {
    .read_time	= s35390a_rtc_read_time,
    .set_time	= s35390a_rtc_set_time,
    .set_alarm	= s35390a_rtc_set_alarm,
    .read_alarm	= s35390a_rtc_read_alarm,
    .ioctl          = s35390a_rtc_ioctl,
    };
    static int s35390a_nvmem_read(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct s35390a *s35390a = priv;
// The offset is ignored because the NVMEM region is only 1 byte
    return s35390a_get_reg(s35390a, S35390A_CMD_FREE_REG, val, bytes);
    }
    static int s35390a_nvmem_write(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct s35390a *s35390a = priv;
    return s35390a_set_reg(s35390a, S35390A_CMD_FREE_REG, val, bytes);
    }
#[no_mangle]
unsafe extern "C" fn s35390a_probe(client: *mut i2c_client) -> c_int {
    static int s35390a_probe(struct i2c_client *client)
    {
    int err, err_read;
    unsigned int i;
    struct s35390a *s35390a;
    struct rtc_device *rtc;
    u8 buf, status1;
    struct device *dev = &client.dev;
    struct nvmem_config nvmem_cfg = {
    .name = "s35390a_nvram",
    .type = NVMEM_TYPE_BATTERY_BACKED,
    .word_size = 1,
    .stride = 1,
    .size = 1,
    .reg_read = s35390a_nvmem_read,
    .reg_write = s35390a_nvmem_write,
    };
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -ENODEV;
    s35390a = devm_kzalloc(dev, sizeof(struct s35390a), GFP_KERNEL);
    if (!s35390a)
    return -ENOMEM;
    s35390a.client[0] = client;
    i2c_set_clientdata(client, s35390a);
// This chip uses multiple addresses, use dummy devices for them
    for (i = 1; i < 8; ++i) {
    s35390a.client[i] = devm_i2c_new_dummy_device(dev,
    client.adapter,
    client.addr + i);
    if (IS_ERR(s35390a.client[i])) {
    dev_err(dev, "Address %02x unavailable\n",
    client.addr + i);
    return PTR_ERR(s35390a.client[i]);
    }
    }
    rtc = devm_rtc_allocate_device(dev);
    if (IS_ERR(rtc))
    return PTR_ERR(rtc);
    err_read = s35390a_read_status(s35390a, &status1);
    if (err_read < 0) {
    dev_err(dev, "error resetting chip\n");
    return err_read;
    }
    if (status1 & S35390A_FLAG_24H)
    s35390a.twentyfourhour = 1;
    else
    s35390a.twentyfourhour = 0;
    if (status1 & S35390A_FLAG_INT2) {
// disable alarm (and maybe test mode)
    buf = 0;
    err = s35390a_set_reg(s35390a, S35390A_CMD_STATUS2, &buf, 1);
    if (err < 0) {
    dev_err(dev, "error disabling alarm");
    return err;
    }
    } else {
    err = s35390a_disable_test_mode(s35390a);
    if (err < 0) {
    dev_err(dev, "error disabling test mode\n");
    return err;
    }
    }
    device_set_wakeup_capable(dev, 1);
    rtc.ops = &s35390a_rtc_ops;
    rtc.range_min = RTC_TIMESTAMP_BEGIN_2000;
    rtc.range_max = RTC_TIMESTAMP_END_2099;
    set_bit(RTC_FEATURE_ALARM_RES_MINUTE, rtc.features);
    clear_bit(RTC_FEATURE_UPDATE_INTERRUPT, rtc.features);
    if (status1 & S35390A_FLAG_INT2)
    rtc_update_irq(rtc, 1, RTC_AF);
    nvmem_cfg.priv = s35390a;
    err = devm_rtc_nvmem_register(rtc, &nvmem_cfg);
    if (err)
    return err;
    return devm_rtc_register_device(rtc);
    }
    static struct i2c_driver s35390a_driver = {
    .driver		= {
    .name	= "rtc-s35390a",
    .of_match_table = of_match_ptr(s35390a_of_match),
    },
    .probe		= s35390a_probe,
    .id_table	= s35390a_id,
    };
    module_i2c_driver(s35390a_driver);
    MODULE_AUTHOR("Byron Bradley <byron.bbradley@gmail.com>");
    MODULE_DESCRIPTION("S35390A RTC driver");
    MODULE_LICENSE("GPL");
