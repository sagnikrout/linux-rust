//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/it87_wdt.c
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
// Watchdog Timer Driver
// for ITE IT87xx Environment Control - Low Pin Count Input / Output
//
// (c) Copyright 2007  Oliver Schuster <olivers137@aol.com>
//
// Based on softdog.c	by Alan Cox,
// 83977f_wdt.c	by Jose Goncalves,
// it87.c		by Chris Gauthron, Jean Delvare
//
// Data-sheets: Publicly available at the ITE website
// http://www.ite.com.tw
//
// Support of the watchdog timers, which are available on
// IT8607, IT8613, IT8620, IT8622, IT8625, IT8628, IT8655, IT8659,
// IT8665, IT8686, IT8702, IT8712, IT8716, IT8718, IT8720, IT8721,
// IT8726,	IT8728, IT8772, IT8783, IT8784 and IT8786.
//

// Defaults for Module Parameter
pub const DEFAULT_TIMEOUT: c_int = 60;
pub const DEFAULT_TESTMODE: c_int = 0;

// IO Ports
pub const REG: c_uint = 0x2e;
pub const VAL: c_uint = 0x2f;
// Logical device Numbers LDN
pub const EC: c_uint = 0x04;
pub const GPIO: c_uint = 0x07;
// Configuration Registers and Functions
pub const LDNREG: c_uint = 0x07;
pub const CHIPID: c_uint = 0x20;
pub const CHIPREV: c_uint = 0x22;
// Chip Id numbers
pub const NO_DEV_ID: c_uint = 0xffff;
pub const IT8607_ID: c_uint = 0x8607;
pub const IT8613_ID: c_uint = 0x8613;
pub const IT8620_ID: c_uint = 0x8620;
pub const IT8622_ID: c_uint = 0x8622;
pub const IT8625_ID: c_uint = 0x8625;
pub const IT8628_ID: c_uint = 0x8628;
pub const IT8655_ID: c_uint = 0x8655;
pub const IT8659_ID: c_uint = 0x8659;
pub const IT8665_ID: c_uint = 0x8665;
pub const IT8686_ID: c_uint = 0x8686;
pub const IT8702_ID: c_uint = 0x8702;
pub const IT8705_ID: c_uint = 0x8705;
pub const IT8712_ID: c_uint = 0x8712;
pub const IT8716_ID: c_uint = 0x8716;
pub const IT8718_ID: c_uint = 0x8718;
pub const IT8720_ID: c_uint = 0x8720;
pub const IT8721_ID: c_uint = 0x8721;
pub const IT8726_ID: c_uint = 0x8726	/* the data sheet suggest wrongly 0x8716 */;
pub const IT8728_ID: c_uint = 0x8728;
pub const IT8772_ID: c_uint = 0x8772;
pub const IT8783_ID: c_uint = 0x8783;
pub const IT8784_ID: c_uint = 0x8784;
pub const IT8786_ID: c_uint = 0x8786;
// Environment Controller Configuration Registers LDN=0x04
pub const SCR1: c_uint = 0xfa;
// Environment Controller Bits SCR1
pub const WDT_PWRGD: c_uint = 0x20;
// GPIO Configuration Registers LDN=0x07
pub const WDTCTRL: c_uint = 0x71;
pub const WDTCFG: c_uint = 0x72;
pub const WDTVALLSB: c_uint = 0x73;
pub const WDTVALMSB: c_uint = 0x74;
// GPIO Bits WDTCFG
pub const WDT_TOV1: c_uint = 0x80;
pub const WDT_KRST: c_uint = 0x40;
pub const WDT_TOVE: c_uint = 0x20;
pub const WDT_PWROK: c_uint = 0x10 /* not in it8721 */;
pub const WDT_INT_MASK: c_uint = 0x0f;
    static unsigned int max_units, chip_type;
    let mut timeout: static unsigned int = DEFAULT_TIMEOUT;
    let mut testmode: static int = DEFAULT_TESTMODE;
    let mut nowayout: static bool = DEFAULT_NOWAYOUT;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout, "Watchdog timeout in seconds, default="
    __MODULE_STRING(DEFAULT_TIMEOUT));
    module_param(testmode, int, 0);
    MODULE_PARM_DESC(testmode, "Watchdog test mode (1 = no reboot), default="
    __MODULE_STRING(DEFAULT_TESTMODE));
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started, default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT));
// Superio Chip
#[no_mangle]
pub unsafe extern "C" fn superio_enter() -> c_int {
    static inline int superio_enter(void)
    {
//
// Try to reserve REG and REG + 1 for exclusive access.
//
    if (!request_muxed_region(REG, 2, WATCHDOG_NAME))
    return -EBUSY;
    outb(0x87, REG);
    outb(0x01, REG);
    outb(0x55, REG);
    outb(0x55, REG);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn superio_exit() {
    static inline void superio_exit(void)
    {
    outb(0x02, REG);
    outb(0x02, VAL);
    release_region(REG, 2);
    }
#[no_mangle]
pub unsafe extern "C" fn superio_select(ldn: c_int) {
    static inline void superio_select(int ldn)
    {
    outb(LDNREG, REG);
    outb(ldn, VAL);
    }
#[no_mangle]
pub unsafe extern "C" fn superio_inb(reg: c_int) -> c_int {
    static inline int superio_inb(int reg)
    {
    outb(reg, REG);
    return inb(VAL);
    }
#[no_mangle]
pub unsafe extern "C" fn superio_outb(val: c_int, reg: c_int) {
    static inline void superio_outb(int val, int reg)
    {
    outb(reg, REG);
    outb(val, VAL);
    }
#[no_mangle]
pub unsafe extern "C" fn superio_inw(reg: c_int) -> c_int {
    static inline int superio_inw(int reg)
    {
    int val;
    outb(reg++, REG);
    val = inb(VAL) << 8;
    outb(reg, REG);
    val |= inb(VAL);
    return val;
    }
// Internal function, should be called after superio_select(GPIO)
#[no_mangle]
unsafe extern "C" fn _wdt_update_timeout(t: c_uint) {
    static void _wdt_update_timeout(unsigned int t)
    {
    let mut cfg: c_uchar = WDT_KRST;
    if (testmode)
    cfg = 0;
    if (t <= max_units)
    cfg |= WDT_TOV1;
    else
    t /= 60;
    if (chip_type != IT8721_ID)
    cfg |= WDT_PWROK;
    superio_outb(cfg, WDTCFG);
    superio_outb(t, WDTVALLSB);
    if (max_units > 255)
    superio_outb(t >> 8, WDTVALMSB);
    }
// Internal function, should be called after superio_select(GPIO)
#[no_mangle]
unsafe extern "C" fn _wdt_running() -> bool {
    static bool _wdt_running(void)
    {
    return superio_inb(WDTVALLSB) || (max_units > 255 && superio_inb(WDTVALMSB));
    }
#[no_mangle]
unsafe extern "C" fn wdt_update_timeout(t: c_uint) -> c_int {
    static int wdt_update_timeout(unsigned int t)
    {
    int ret;
    ret = superio_enter();
    if (ret)
    return ret;
    superio_select(GPIO);
    _wdt_update_timeout(t);
    superio_exit();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wdt_round_time(t: c_int) -> c_int {
    static int wdt_round_time(int t)
    {
    t += 59;
    t -= t % 60;
    return t;
    }
// watchdog timer handling
#[no_mangle]
unsafe extern "C" fn wdt_start(wdd: *mut watchdog_device) -> c_int {
    static int wdt_start(struct watchdog_device *wdd)
    {
    return wdt_update_timeout(wdd.timeout);
    }
#[no_mangle]
unsafe extern "C" fn wdt_stop(wdd: *mut watchdog_device) -> c_int {
    static int wdt_stop(struct watchdog_device *wdd)
    {
    return wdt_update_timeout(0);
    }
//
// wdt_set_timeout - set a new timeout value with watchdog ioctl
// @wdd: pointer to the watchdog_device structure
// @t: timeout value in seconds
//
// The hardware device has a 8 or 16 bit watchdog timer (depends on
// chip version) that can be configured to count seconds or minutes.
//
// Used within WDIOC_SETTIMEOUT watchdog device ioctl.
//
// Return: 0 if the timeout was set successfully, or a negative error code on
// failure.
//
#[no_mangle]
unsafe extern "C" fn wdt_set_timeout(wdd: *mut watchdog_device, t: c_uint) -> c_int {
    static int wdt_set_timeout(struct watchdog_device *wdd, unsigned int t)
    {
    let mut ret: c_int = 0;
    if (t > max_units)
    t = wdt_round_time(t);
    wdd.timeout = t;
    if (watchdog_hw_running(wdd))
    ret = wdt_update_timeout(t);
    return ret;
    }
    enum {
    IT87_WDT_OUTPUT_THROUGH_PWRGD	= BIT(0),
    };
    static const struct dmi_system_id it87_quirks[] = {
    {
// Qotom Q30900P (IT8786)
    .matches = {
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "QCML04"),
    },
    .driver_data = (void *)IT87_WDT_OUTPUT_THROUGH_PWRGD,
    },
    {}
    };
    static const struct watchdog_info ident = {
    .options = WDIOF_SETTIMEOUT | WDIOF_MAGICCLOSE | WDIOF_KEEPALIVEPING,
    .firmware_version = 1,
    .identity = WATCHDOG_NAME,
    };
    static const struct watchdog_ops wdt_ops = {
    .owner = THIS_MODULE,
    .start = wdt_start,
    .stop = wdt_stop,
    .set_timeout = wdt_set_timeout,
    };
    static struct watchdog_device wdt_dev = {
    .info = &ident,
    .ops = &wdt_ops,
    .min_timeout = 1,
    };
#[no_mangle]
unsafe extern "C" fn it87_wdt_init() -> int __init {
    static int __init it87_wdt_init(void)
    {
    const struct dmi_system_id *dmi_id;
    u8  chip_rev;
    u8 ctrl;
    let mut quirks: c_int = 0;
    int rc;
    rc = superio_enter();
    if (rc)
    return rc;
    chip_type = superio_inw(CHIPID);
    chip_rev  = superio_inb(CHIPREV) & 0x0f;
    superio_exit();
    dmi_id = dmi_first_match(it87_quirks);
    if (dmi_id)
    quirks = (long)dmi_id.driver_data;
    switch (chip_type) {
    case IT8702_ID:
    max_units = 255;
    break;
    case IT8712_ID:
    max_units = (chip_rev < 8) ? 255 : 65535;
    break;
    case IT8607_ID:
    case IT8613_ID:
    case IT8620_ID:
    case IT8622_ID:
    case IT8625_ID:
    case IT8628_ID:
    case IT8655_ID:
    case IT8659_ID:
    case IT8665_ID:
    case IT8686_ID:
    case IT8716_ID:
    case IT8718_ID:
    case IT8720_ID:
    case IT8721_ID:
    case IT8726_ID:
    case IT8728_ID:
    case IT8772_ID:
    case IT8783_ID:
    case IT8784_ID:
    case IT8786_ID:
    max_units = 65535;
    break;
    case IT8705_ID:
    pr_err("Unsupported Chip found, Chip %04x Revision %02x\n",
    chip_type, chip_rev);
    return -ENODEV;
    case NO_DEV_ID:
    pr_err("no device\n");
    return -ENODEV;
    default:
    pr_err("Unknown Chip found, Chip %04x Revision %04x\n",
    chip_type, chip_rev);
    return -ENODEV;
    }
    rc = superio_enter();
    if (rc)
    return rc;
    superio_select(GPIO);
    superio_outb(WDT_TOV1, WDTCFG);
    switch (chip_type) {
    case IT8784_ID:
    case IT8786_ID:
    ctrl = superio_inb(WDTCTRL);
    ctrl &= 0x08;
    superio_outb(ctrl, WDTCTRL);
    break;
    default:
    superio_outb(0x00, WDTCTRL);
    }
    if (quirks & IT87_WDT_OUTPUT_THROUGH_PWRGD) {
    superio_select(EC);
    ctrl = superio_inb(SCR1);
    if (!(ctrl & WDT_PWRGD)) {
    ctrl |= WDT_PWRGD;
    superio_outb(ctrl, SCR1);
    }
    }
// wdt already left running by firmware?
    if (_wdt_running()) {
    pr_info("Left running by firmware.\n");
    set_bit(WDOG_HW_RUNNING, &wdt_dev.status);
    }
    superio_exit();
    if (timeout < 1 || timeout > max_units * 60) {
    timeout = DEFAULT_TIMEOUT;
    pr_warn("Timeout value out of range, use default %d sec\n",
    DEFAULT_TIMEOUT);
    }
    if (timeout > max_units)
    timeout = wdt_round_time(timeout);
    wdt_dev.timeout = timeout;
    wdt_dev.max_timeout = max_units * 60;
    watchdog_stop_on_reboot(&wdt_dev);
    rc = watchdog_register_device(&wdt_dev);
    if (rc)
    return rc;
    pr_info("Chip IT%04x revision %d initialized. timeout=%d sec (nowayout=%d testmode=%d)\n",
    chip_type, chip_rev, timeout, nowayout, testmode);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn it87_wdt_exit() -> void __exit {
    static void __exit it87_wdt_exit(void)
    {
    watchdog_unregister_device(&wdt_dev);
    }
    module_init(it87_wdt_init);
    module_exit(it87_wdt_exit);
    MODULE_AUTHOR("Oliver Schuster");
    MODULE_DESCRIPTION("Hardware Watchdog Device Driver for IT87xx EC-LPC I/O");
    MODULE_LICENSE("GPL");
