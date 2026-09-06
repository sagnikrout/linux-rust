//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/lenovo_se30g2_se60_wdt.c
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
// WDT driver for Lenovo SE30G2 & SE60.
//

pub const CFG_PORT: c_uint = 0x2E;
pub const CFG_SIZE: c_int = 2;
pub const CFG_LDN: c_uint = 0x07;
pub const CFG_BRAM_LDN: c_uint = 0x10;
pub const BRAM_SIZE: c_int = 2;
pub const BRAM_WDT_REG: c_uint = 0x48;

// The timeout range is 1-255 seconds
pub const MIN_TIMEOUT: c_int = 1;
pub const MAX_TIMEOUT: c_int = 255;

    static unsigned short bram_base;
    static struct platform_device *se_30g2_60_pdev;
    static int timeout; /* in seconds */
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout,
    "Watchdog timeout in seconds. 1 <= timeout <= 255, default="
    __MODULE_STRING(WATCHDOG_TIMEOUT) ".");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct se_30g2_60_wdt {
    pub wdd: watchdog_device,
}

#[no_mangle]
unsafe extern "C" fn enter_pnp_mode() {
    static void enter_pnp_mode(void)
    {
    outb(0x87, CFG_PORT);
    outb(0x01, CFG_PORT);
    outb(0x55, CFG_PORT);
    outb(0x55, CFG_PORT);
    }
#[no_mangle]
unsafe extern "C" fn exit_pnp_mode() {
    static void exit_pnp_mode(void)
    {
    outb(0x2, CFG_PORT);
    outb(0x2, CFG_PORT + 1);
    }
#[no_mangle]
unsafe extern "C" fn lpc_write(index: c_uchar, data: c_uchar) {
    static void lpc_write(unsigned char index, unsigned char data)
    {
    outb(index, CFG_PORT);
    outb(data, CFG_PORT + 1);
    }
#[no_mangle]
unsafe extern "C" fn lpc_read(index: c_uchar) -> c_uchar {
    static unsigned char lpc_read(unsigned char index)
    {
    outb(index, CFG_PORT);
    return inb(CFG_PORT + 1);
    }
#[no_mangle]
unsafe extern "C" fn lpc_chip_id() -> c_ushort {
    static unsigned short lpc_chip_id(void)
    {
    unsigned char msb, lsb;
    msb = lpc_read(0x20);
    lsb = lpc_read(0x21);
    return (msb << 8 | lsb);
    }
#[no_mangle]
unsafe extern "C" fn bram_write(reg: c_uchar, val: c_uchar) {
    static void bram_write(unsigned char reg, unsigned char val)
    {
    outb(reg | 0x80, bram_base);
    outb(val, bram_base + 1);
    }
#[no_mangle]
unsafe extern "C" fn bram_read(reg: c_uchar) -> c_uchar {
    static unsigned char bram_read(unsigned char reg)
    {
    unsigned char val;
    outb(reg | 0x80, bram_base);
    val = inb(bram_base + 1);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn wdt_read(val: *mut c_ushort) -> c_int {
    static int wdt_read(unsigned short *val)
    {
    if (!request_muxed_region(bram_base, BRAM_SIZE, DRVNAME))
    return -EACCES;
// val = ((bram_read(BRAM_WDT_REG) & 0xFF) << 8);
// val |= bram_read(BRAM_WDT_REG + 1) & 0xFF;
    release_region(bram_base, BRAM_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wdt_write(val: c_ushort) -> c_int {
    static int wdt_write(unsigned short val)
    {
    if (!request_muxed_region(bram_base, BRAM_SIZE, DRVNAME))
    return -EACCES;
    bram_write(BRAM_WDT_REG, (val >> 8) & 0xFF);
    bram_write(BRAM_WDT_REG + 1, val & 0xFF);
    release_region(bram_base, BRAM_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int wdt_start(struct watchdog_device *wdog)
    {
    return wdt_write(wdog.timeout);
    }
#[no_mangle]
unsafe extern "C" fn wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int wdt_stop(struct watchdog_device *wdog)
    {
    return wdt_write(0);
    }
#[no_mangle]
unsafe extern "C" fn wdt_ping(wdog: *mut watchdog_device) -> c_int {
    static int wdt_ping(struct watchdog_device *wdog)
    {
    return wdt_write(wdog.timeout);
    }
#[no_mangle]
unsafe extern "C" fn wdt_get_timeleft(wdog: *mut watchdog_device) -> c_uint {
    static unsigned int wdt_get_timeleft(struct watchdog_device *wdog)
    {
    unsigned short val;
    int err;
    err = wdt_read(&val);
    return err ? 0 : val;
    }
    static const struct watchdog_info wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "Lenovo SE30G2 SE60 WDOG",
    };
    static const struct watchdog_ops se_30g2_60_wdt_ops = {
    .owner = THIS_MODULE,
    .start = wdt_start,
    .stop = wdt_stop,
    .ping = wdt_ping,
    .get_timeleft = wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn se_30g2_60_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int se_30g2_60_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct se_30g2_60_wdt *priv;
    unsigned int chip_id;
    int ret;
    if (!request_muxed_region(CFG_PORT, CFG_SIZE, DRVNAME))
    return -EBUSY;
// identify the chip
    enter_pnp_mode();
    chip_id = lpc_chip_id();
    if (chip_id != 0x5782) {
    exit_pnp_mode();
    release_region(CFG_PORT, CFG_SIZE);
    return -ENODEV;
    }
// probe the BRAM base address
    lpc_write(CFG_LDN, CFG_BRAM_LDN);
    bram_base = (lpc_read(0x60) << 8) | lpc_read(0x61);
    exit_pnp_mode();
    release_region(CFG_PORT, CFG_SIZE);
    dev_info(dev, "Found Lenovo SE30G2 SE60 0x%x\n", chip_id);
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    watchdog_set_drvdata(&priv.wdd, priv);
    priv.wdd.parent = dev;
    priv.wdd.info = &wdt_info;
    priv.wdd.ops = &se_30g2_60_wdt_ops;
    priv.wdd.timeout = WATCHDOG_TIMEOUT; /* Set default timeout */
    priv.wdd.min_timeout = MIN_TIMEOUT;
    priv.wdd.max_timeout = MAX_TIMEOUT;
    watchdog_init_timeout(&priv.wdd, timeout, dev);
    watchdog_set_nowayout(&priv.wdd, nowayout);
    watchdog_stop_on_reboot(&priv.wdd);
    watchdog_stop_on_unregister(&priv.wdd);
    ret = devm_watchdog_register_device(dev, &priv.wdd);
    dev_dbg(&pdev.dev, "initialized. timeout=%d sec (nowayout=%d)\n",
    priv.wdd.timeout, nowayout);
    return ret;
    }
    static struct platform_driver se_30g2_60_wdt_driver = {
    .driver = {
    .name = DRVNAME,
    },
    .probe = se_30g2_60_wdt_probe,
    };
#[no_mangle]
unsafe extern "C" fn se_30g2_60_create_device(id: *const dmi_system_id) -> c_int {
    static int se_30g2_60_create_device(const struct dmi_system_id *id)
    {
    int err;
    se_30g2_60_pdev = platform_device_alloc("lenovo-se30g2-se60-wdt", -1);
    if (!se_30g2_60_pdev)
    return -ENOMEM;
    err = platform_device_add(se_30g2_60_pdev);
    if (err) {
    platform_device_put(se_30g2_60_pdev);
    se_30g2_60_pdev = core::ptr::null_mut();
    }
    return err;
    }
    static const struct dmi_system_id se_30g2_60[] __initconst = {
    {
    .ident = "LENOVO-SE30G2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_BOARD_NAME, "33BD"),
    },
    .callback = se_30g2_60_create_device,
    },
    {
    .ident = "LENOVO-SE60",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_BOARD_NAME, "33BF"),
    },
    .callback = se_30g2_60_create_device,
    },
    {}
    };
    MODULE_DEVICE_TABLE(dmi, se_30g2_60);
#[no_mangle]
unsafe extern "C" fn se_30g2_60_wdt_init() -> int __init {
    static int __init se_30g2_60_wdt_init(void)
    {
    int err;
    if (!dmi_check_system(se_30g2_60))
    return -ENODEV;
    err = platform_driver_register(&se_30g2_60_wdt_driver);
    if (err && se_30g2_60_pdev) {
    platform_device_unregister(se_30g2_60_pdev);
    se_30g2_60_pdev = core::ptr::null_mut();
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn se_30g2_60_wdt_exit() -> void __exit {
    static void __exit se_30g2_60_wdt_exit(void)
    {
    if (se_30g2_60_pdev)
    platform_device_unregister(se_30g2_60_pdev);
    platform_driver_unregister(&se_30g2_60_wdt_driver);
    }
    module_init(se_30g2_60_wdt_init);
    module_exit(se_30g2_60_wdt_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Mark Pearson <mpearson-lenovo@squebb.ca>");
    MODULE_DESCRIPTION("WDT driver for Lenovo SE30G2 & SE60");
