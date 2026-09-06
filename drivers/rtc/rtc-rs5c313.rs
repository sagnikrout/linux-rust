//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-rs5c313.c
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


//
// Ricoh RS5C313 RTC device/driver
// Copyright (C) 2007 Nobuhiro Iwamatsu
//
// 2005-09-19 modified by kogiidena
//
// Based on the old drivers/char/rs5c313_rtc.c  by:
// Copyright (C) 2000 Philipp Rumpf <prumpf@tux.org>
// Copyright (C) 1999 Tetsuya Okada & Niibe Yutaka
//
// Based on code written by Paul Gortmaker.
// Copyright (C) 1996 Paul Gortmaker
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Based on other minimal char device drivers, like Alan's
// watchdog, Ted's random, etc. etc.
//
// 1.07	Paul Gortmaker.
// 1.08	Miquel van Smoorenburg: disallow certain things on the
// DEC Alpha as the CMOS clock is also used for other things.
// 1.09	Nikita Schmidt: epoch support and some Alpha cleanup.
// 1.09a	Pete Zaitcev: Sun SPARC
// 1.09b	Jeff Garzik: Modularize, init cleanup
// 1.09c	Jeff Garzik: SMP cleanup
// 1.10    Paul Barton-Davis: add support for async I/O
// 1.10a	Andrea Arcangeli: Alpha updates
// 1.10b	Andrew Morton: SMP lock fix
// 1.10c	Cesar Barros: SMP locking fixes and cleanup
// 1.10d	Paul Gortmaker: delete paranoia check in rtc_exit
// 1.10e	Maciej W. Rozycki: Handle DECstation's year weirdness.
// 1.11    Takashi Iwai: Kernel access functions
// rtc_register/rtc_unregister/rtc_control
// 1.11a   Daniele Bellucci: Audit create_proc_read_entry in rtc_init
// 1.12	Venkatesh Pallipadi: Hooks for emulating rtc on HPET base-timer
// CONFIG_HPET_EMULATE_RTC
// 1.13	Nobuhiro Iwamatsu: Update driver.
//

//
// LANDISK dependence part of RS5C313
//
pub const SCSMR1: c_uint = 0xFFE00000;
pub const SCSCR1: c_uint = 0xFFE00008;
pub const SCSMR1_CA: c_uint = 0x80;
pub const SCSCR1_CKE: c_uint = 0x03;
pub const SCSPTR1: c_uint = 0xFFE0001C;
pub const SCSPTR1_EIO: c_uint = 0x80;
pub const SCSPTR1_SPB1IO: c_uint = 0x08;
pub const SCSPTR1_SPB1DT: c_uint = 0x04;
pub const SCSPTR1_SPB0IO: c_uint = 0x02;
pub const SCSPTR1_SPB0DT: c_uint = 0x01;

// RICOH RS5C313 CE port
pub const RS5C313_CE: c_uint = 0xB0000003;
// RICOH RS5C313 CE port bit
pub const RS5C313_CE_RTCCE: c_uint = 0x02;
// SCSPTR1 data
    unsigned char scsptr1_data;

#[no_mangle]
unsafe extern "C" fn rs5c313_init_port() {
    static void rs5c313_init_port(void)
    {
// Set SCK as I/O port and Initialize SCSPTR1 data & I/O port.
    __raw_writeb(__raw_readb(SCSMR1) & ~SCSMR1_CA, SCSMR1);
    __raw_writeb(__raw_readb(SCSCR1) & ~SCSCR1_CKE, SCSCR1);
// And Initialize SCL for RS5C313 clock
    scsptr1_data = __raw_readb(SCSPTR1) | SCL;	/* SCL:H */
    __raw_writeb(scsptr1_data, SCSPTR1);
    scsptr1_data = __raw_readb(SCSPTR1) | SCL_OEN;	/* SCL output enable */
    __raw_writeb(scsptr1_data, SCSPTR1);
    RS5C313_CEDISABLE;	/* CE:L */
    }
#[no_mangle]
unsafe extern "C" fn rs5c313_write_data(data: c_uchar) {
    static void rs5c313_write_data(unsigned char data)
    {
    int i;
    for (i = 0; i < 8; i++) {
// SDA:Write Data
    scsptr1_data = (scsptr1_data & ~SDA) |
    ((((0x80 >> i) & data) >> (7 - i)) << 2);
    __raw_writeb(scsptr1_data, SCSPTR1);
    if (i == 0) {
    scsptr1_data |= SDA_OEN;	/* SDA:output enable */
    __raw_writeb(scsptr1_data, SCSPTR1);
    }
    ndelay(700);
    scsptr1_data &= ~SCL;	/* SCL:L */
    __raw_writeb(scsptr1_data, SCSPTR1);
    ndelay(700);
    scsptr1_data |= SCL;	/* SCL:H */
    __raw_writeb(scsptr1_data, SCSPTR1);
    }
    scsptr1_data &= ~SDA_OEN;	/* SDA:output disable */
    __raw_writeb(scsptr1_data, SCSPTR1);
    }
#[no_mangle]
unsafe extern "C" fn rs5c313_read_data() -> c_uchar {
    static unsigned char rs5c313_read_data(void)
    {
    int i;
    let mut data: c_uchar = 0;
    for (i = 0; i < 8; i++) {
    ndelay(700);
// SDA:Read Data
    data |= ((__raw_readb(SCSPTR1) & SDA) >> 2) << (7 - i);
    scsptr1_data &= ~SCL;	/* SCL:L */
    __raw_writeb(scsptr1_data, SCSPTR1);
    ndelay(700);
    scsptr1_data |= SCL;	/* SCL:H */
    __raw_writeb(scsptr1_data, SCSPTR1);
    }
    return data & 0x0F;
    }

//
// machine independence part of RS5C313
//
// RICOH RS5C313 address
pub const RS5C313_ADDR_SEC: c_uint = 0x00;
pub const RS5C313_ADDR_SEC10: c_uint = 0x01;
pub const RS5C313_ADDR_MIN: c_uint = 0x02;
pub const RS5C313_ADDR_MIN10: c_uint = 0x03;
pub const RS5C313_ADDR_HOUR: c_uint = 0x04;
pub const RS5C313_ADDR_HOUR10: c_uint = 0x05;
pub const RS5C313_ADDR_WEEK: c_uint = 0x06;
pub const RS5C313_ADDR_INTINTVREG: c_uint = 0x07;
pub const RS5C313_ADDR_DAY: c_uint = 0x08;
pub const RS5C313_ADDR_DAY10: c_uint = 0x09;
pub const RS5C313_ADDR_MON: c_uint = 0x0A;
pub const RS5C313_ADDR_MON10: c_uint = 0x0B;
pub const RS5C313_ADDR_YEAR: c_uint = 0x0C;
pub const RS5C313_ADDR_YEAR10: c_uint = 0x0D;
pub const RS5C313_ADDR_CNTREG: c_uint = 0x0E;
pub const RS5C313_ADDR_TESTREG: c_uint = 0x0F;
// RICOH RS5C313 control register
pub const RS5C313_CNTREG_ADJ_BSY: c_uint = 0x01;
pub const RS5C313_CNTREG_WTEN_XSTP: c_uint = 0x02;
pub const RS5C313_CNTREG_12_24: c_uint = 0x04;
pub const RS5C313_CNTREG_CTFG: c_uint = 0x08;
// RICOH RS5C313 test register
pub const RS5C313_TESTREG_TEST: c_uint = 0x01;
// RICOH RS5C313 control bit
pub const RS5C313_CNTBIT_READ: c_uint = 0x40;
pub const RS5C313_CNTBIT_AD: c_uint = 0x20;
pub const RS5C313_CNTBIT_DT: c_uint = 0x10;
#[no_mangle]
unsafe extern "C" fn rs5c313_read_reg(addr: c_uchar) -> c_uchar {
    static unsigned char rs5c313_read_reg(unsigned char addr)
    {
    rs5c313_write_data(addr | RS5C313_CNTBIT_READ | RS5C313_CNTBIT_AD);
    return rs5c313_read_data();
    }
#[no_mangle]
unsafe extern "C" fn rs5c313_write_reg(addr: c_uchar, data: c_uchar) {
    static void rs5c313_write_reg(unsigned char addr, unsigned char data)
    {
    data &= 0x0f;
    rs5c313_write_data(addr | RS5C313_CNTBIT_AD);
    rs5c313_write_data(data | RS5C313_CNTBIT_DT);
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn rs5c313_read_cntreg() -> c_uchar {
    static inline unsigned char rs5c313_read_cntreg(void)
    {
    return rs5c313_read_reg(RS5C313_ADDR_CNTREG);
    }
#[no_mangle]
pub unsafe extern "C" fn rs5c313_write_cntreg(data: c_uchar) {
    static inline void rs5c313_write_cntreg(unsigned char data)
    {
    rs5c313_write_reg(RS5C313_ADDR_CNTREG, data);
    }
#[no_mangle]
pub unsafe extern "C" fn rs5c313_write_intintvreg(data: c_uchar) {
    static inline void rs5c313_write_intintvreg(unsigned char data)
    {
    rs5c313_write_reg(RS5C313_ADDR_INTINTVREG, data);
    }
#[no_mangle]
unsafe extern "C" fn rs5c313_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rs5c313_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    int data;
    int cnt;
    cnt = 0;
    while (1) {
    RS5C313_CEENABLE;	/* CE:H */
// Initialize control reg. 24 hour
    rs5c313_write_cntreg(0x04);
    if (!(rs5c313_read_cntreg() & RS5C313_CNTREG_ADJ_BSY))
    break;
    RS5C313_CEDISABLE;
    ndelay(700);	/* CE:L */
    if (cnt++ > 100) {
    dev_err(dev, "%s: timeout error\n", __func__);
    return -EIO;
    }
    }
    data = rs5c313_read_reg(RS5C313_ADDR_SEC);
    data |= (rs5c313_read_reg(RS5C313_ADDR_SEC10) << 4);
    tm.tm_sec = bcd2bin(data);
    data = rs5c313_read_reg(RS5C313_ADDR_MIN);
    data |= (rs5c313_read_reg(RS5C313_ADDR_MIN10) << 4);
    tm.tm_min = bcd2bin(data);
    data = rs5c313_read_reg(RS5C313_ADDR_HOUR);
    data |= (rs5c313_read_reg(RS5C313_ADDR_HOUR10) << 4);
    tm.tm_hour = bcd2bin(data);
    data = rs5c313_read_reg(RS5C313_ADDR_DAY);
    data |= (rs5c313_read_reg(RS5C313_ADDR_DAY10) << 4);
    tm.tm_mday = bcd2bin(data);
    data = rs5c313_read_reg(RS5C313_ADDR_MON);
    data |= (rs5c313_read_reg(RS5C313_ADDR_MON10) << 4);
    tm.tm_mon = bcd2bin(data) - 1;
    data = rs5c313_read_reg(RS5C313_ADDR_YEAR);
    data |= (rs5c313_read_reg(RS5C313_ADDR_YEAR10) << 4);
    tm.tm_year = bcd2bin(data);
    if (tm.tm_year < 70)
    tm.tm_year += 100;
    data = rs5c313_read_reg(RS5C313_ADDR_WEEK);
    tm.tm_wday = bcd2bin(data);
    RS5C313_CEDISABLE;
    ndelay(700);		/* CE:L */
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rs5c313_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int rs5c313_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    int data;
    int cnt;
    cnt = 0;
// busy check.
    while (1) {
    RS5C313_CEENABLE;	/* CE:H */
// Initialize control reg. 24 hour
    rs5c313_write_cntreg(0x04);
    if (!(rs5c313_read_cntreg() & RS5C313_CNTREG_ADJ_BSY))
    break;
    RS5C313_MISCOP;
    RS5C313_CEDISABLE;
    ndelay(700);	/* CE:L */
    if (cnt++ > 100) {
    dev_err(dev, "%s: timeout error\n", __func__);
    return -EIO;
    }
    }
    data = bin2bcd(tm.tm_sec);
    rs5c313_write_reg(RS5C313_ADDR_SEC, data);
    rs5c313_write_reg(RS5C313_ADDR_SEC10, (data >> 4));
    data = bin2bcd(tm.tm_min);
    rs5c313_write_reg(RS5C313_ADDR_MIN, data);
    rs5c313_write_reg(RS5C313_ADDR_MIN10, (data >> 4));
    data = bin2bcd(tm.tm_hour);
    rs5c313_write_reg(RS5C313_ADDR_HOUR, data);
    rs5c313_write_reg(RS5C313_ADDR_HOUR10, (data >> 4));
    data = bin2bcd(tm.tm_mday);
    rs5c313_write_reg(RS5C313_ADDR_DAY, data);
    rs5c313_write_reg(RS5C313_ADDR_DAY10, (data >> 4));
    data = bin2bcd(tm.tm_mon + 1);
    rs5c313_write_reg(RS5C313_ADDR_MON, data);
    rs5c313_write_reg(RS5C313_ADDR_MON10, (data >> 4));
    data = bin2bcd(tm.tm_year % 100);
    rs5c313_write_reg(RS5C313_ADDR_YEAR, data);
    rs5c313_write_reg(RS5C313_ADDR_YEAR10, (data >> 4));
    data = bin2bcd(tm.tm_wday);
    rs5c313_write_reg(RS5C313_ADDR_WEEK, data);
    RS5C313_CEDISABLE;	/* CE:H */
    ndelay(700);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rs5c313_check_xstp_bit() {
    static void rs5c313_check_xstp_bit(void)
    {
    struct rtc_time tm;
    int cnt;
    RS5C313_CEENABLE;	/* CE:H */
    if (rs5c313_read_cntreg() & RS5C313_CNTREG_WTEN_XSTP) {
// INT interval reg. OFF
    rs5c313_write_intintvreg(0x00);
// Initialize control reg. 24 hour & adjust
    rs5c313_write_cntreg(0x07);
// busy check.
    for (cnt = 0; cnt < 100; cnt++) {
    if (!(rs5c313_read_cntreg() & RS5C313_CNTREG_ADJ_BSY))
    break;
    RS5C313_MISCOP;
    }
    memset(&tm, 0, sizeof(struct rtc_time));
    tm.tm_mday	= 1;
    tm.tm_mon	= 1 - 1;
    tm.tm_year	= 2000 - 1900;
    rs5c313_rtc_set_time(core::ptr::null_mut(), &tm);
    pr_err("invalid value, resetting to 1 Jan 2000\n");
    }
    RS5C313_CEDISABLE;
    ndelay(700);		/* CE:L */
    }
    static const struct rtc_class_ops rs5c313_rtc_ops = {
    .read_time = rs5c313_rtc_read_time,
    .set_time = rs5c313_rtc_set_time,
    };
#[no_mangle]
unsafe extern "C" fn rs5c313_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int rs5c313_rtc_probe(struct platform_device *pdev)
    {
    struct rtc_device *rtc;
    rs5c313_init_port();
    rs5c313_check_xstp_bit();
    rtc = devm_rtc_device_register(&pdev.dev, "rs5c313", &rs5c313_rtc_ops,
    THIS_MODULE);
    return PTR_ERR_OR_ZERO(rtc);
    }
    static struct platform_driver rs5c313_rtc_platform_driver = {
    .driver         = {
    .name   = DRV_NAME,
    },
    .probe	= rs5c313_rtc_probe,
    };
    module_platform_driver(rs5c313_rtc_platform_driver);
    MODULE_AUTHOR("kogiidena , Nobuhiro Iwamatsu <iwamatsu@nigauri.org>");
    MODULE_DESCRIPTION("Ricoh RS5C313 RTC device driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRV_NAME);
