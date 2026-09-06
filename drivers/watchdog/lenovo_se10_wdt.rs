//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/lenovo_se10_wdt.c
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
// WDT driver for Lenovo SE10.
//

pub const STATUS_PORT: c_uint = 0x6C;
pub const CMD_PORT: c_uint = 0x6C;
pub const DATA_PORT: c_uint = 0x68;
pub const OUTBUF_FULL: c_uint = 0x01;
pub const INBUF_EMPTY: c_uint = 0x02;
pub const CFG_LDN: c_uint = 0x07;
pub const CFG_BRAM_LDN: c_uint = 0x10 /* for BRAM Base */;
pub const CFG_PORT: c_uint = 0x2E;
pub const CFG_SIZE: c_int = 2;
pub const CMD_SIZE: c_int = 4;
pub const BRAM_SIZE: c_int = 2;
pub const UNLOCK_KEY: c_uint = 0x87;
pub const LOCK_KEY: c_uint = 0xAA;
pub const CUS_WDT_SWI: c_uint = 0x1A;
pub const CUS_WDT_CFG: c_uint = 0x1B;
pub const CUS_WDT_FEED: c_uint = 0xB0;
pub const CUS_WDT_CNT: c_uint = 0xB1;

// The timeout range is 1-255 seconds
pub const MIN_TIMEOUT: c_int = 1;
pub const MAX_TIMEOUT: c_int = 255;
pub const MAX_WAIT: c_int = 10;

    static unsigned short bram_base;
    static struct platform_device *se10_pdev;
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
pub struct se10_wdt {
    pub wdd: watchdog_device,
}

#[no_mangle]
unsafe extern "C" fn set_bram(offset: c_uchar, val: c_uchar) -> c_int {
    static int set_bram(unsigned char offset, unsigned char val)
    {
    if (!request_muxed_region(bram_base, BRAM_SIZE, DRVNAME))
    return -EBUSY;
    outb(offset, bram_base);
    outb(val, bram_base + 1);
    release_region(bram_base, BRAM_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wait_for_buffer(condition: c_int) {
    static void wait_for_buffer(int condition)
    {
    let mut loop: c_int = 0;
    while (1) {
    if (inb(STATUS_PORT) & condition || loop > MAX_WAIT)
    break;
    loop++;
    usleep_range(10, 125);
    }
    }
#[no_mangle]
unsafe extern "C" fn send_cmd(cmd: c_uchar) {
    static void send_cmd(unsigned char cmd)
    {
    wait_for_buffer(INBUF_EMPTY);
    outb(cmd, CMD_PORT);
    wait_for_buffer(INBUF_EMPTY);
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
unsafe extern "C" fn wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int wdt_start(struct watchdog_device *wdog)
    {
    return set_bram(CUS_WDT_SWI, 0x80);
    }
#[no_mangle]
unsafe extern "C" fn wdt_set_timeout(wdog: *mut watchdog_device, timeout: c_uint) -> c_int {
    static int wdt_set_timeout(struct watchdog_device *wdog, unsigned int timeout)
    {
    wdog.timeout = timeout;
    return set_bram(CUS_WDT_CFG, wdog.timeout);
    }
#[no_mangle]
unsafe extern "C" fn wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int wdt_stop(struct watchdog_device *wdog)
    {
    return set_bram(CUS_WDT_SWI, 0);
    }
#[no_mangle]
unsafe extern "C" fn wdt_get_time(wdog: *mut watchdog_device) -> c_uint {
    static unsigned int wdt_get_time(struct watchdog_device *wdog)
    {
    unsigned char time;
    if (!request_muxed_region(CMD_PORT, CMD_SIZE, DRVNAME))
    return -EBUSY;
    send_cmd(CUS_WDT_CNT);
    wait_for_buffer(OUTBUF_FULL);
    time = inb(DATA_PORT);
    release_region(CMD_PORT, CMD_SIZE);
    return time;
    }
#[no_mangle]
unsafe extern "C" fn wdt_ping(wdog: *mut watchdog_device) -> c_int {
    static int wdt_ping(struct watchdog_device *wdog)
    {
    if (!request_muxed_region(CMD_PORT, CMD_SIZE, DRVNAME))
    return -EBUSY;
    send_cmd(CUS_WDT_FEED);
    release_region(CMD_PORT, CMD_SIZE);
    return 0;
    }
    static const struct watchdog_info wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .identity = "Lenovo SE10 Watchdog",
    };
    static const struct watchdog_ops se10_wdt_ops = {
    .owner = THIS_MODULE,
    .start = wdt_start,
    .stop = wdt_stop,
    .ping = wdt_ping,
    .set_timeout = wdt_set_timeout,
    .get_timeleft = wdt_get_time,
    };
#[no_mangle]
unsafe extern "C" fn get_chipID() -> c_uint {
    static unsigned int get_chipID(void)
    {
    unsigned char msb, lsb;
    outb(UNLOCK_KEY, CFG_PORT);
    outb(0x01, CFG_PORT);
    outb(0x55, CFG_PORT);
    outb(0x55, CFG_PORT);
    msb = lpc_read(0x20);
    lsb = lpc_read(0x21);
    outb(LOCK_KEY, CFG_PORT);
    return (msb * 256 + lsb);
    }
#[no_mangle]
unsafe extern "C" fn se10_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int se10_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct se10_wdt *priv;
    unsigned int chip_id;
    int ret;
    if (!request_muxed_region(CFG_PORT, CFG_SIZE, DRVNAME))
    return -EBUSY;
    chip_id = get_chipID();
    if (chip_id != 0x5632 && chip_id != 0x5652) {
    release_region(CFG_PORT, CFG_SIZE);
    return -ENODEV;
    }
    lpc_write(CFG_LDN, CFG_BRAM_LDN);
    bram_base = (lpc_read(0x60) << 8) | lpc_read(0x61);
    release_region(CFG_PORT, CFG_SIZE);
    dev_info(dev, "Found Lenovo SE10 0x%x\n", chip_id);
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    watchdog_set_drvdata(&priv.wdd, priv);
    priv.wdd.parent = dev;
    priv.wdd.info = &wdt_info;
    priv.wdd.ops = &se10_wdt_ops;
    priv.wdd.timeout = WATCHDOG_TIMEOUT; /* Set default timeout */
    priv.wdd.min_timeout = MIN_TIMEOUT;
    priv.wdd.max_timeout = MAX_TIMEOUT;
    set_bram(CUS_WDT_CFG, WATCHDOG_TIMEOUT); /* Set time to default */
    watchdog_init_timeout(&priv.wdd, timeout, dev);
    watchdog_set_nowayout(&priv.wdd, nowayout);
    watchdog_stop_on_reboot(&priv.wdd);
    watchdog_stop_on_unregister(&priv.wdd);
    ret = devm_watchdog_register_device(dev, &priv.wdd);
    dev_dbg(&pdev.dev, "initialized. timeout=%d sec (nowayout=%d)\n",
    priv.wdd.timeout, nowayout);
    return ret;
    }
    static struct platform_driver se10_wdt_driver = {
    .driver = {
    .name = DRVNAME,
    },
    .probe  = se10_wdt_probe,
    };
#[no_mangle]
unsafe extern "C" fn se10_create_platform_device() -> c_int {
    static int se10_create_platform_device(void)
    {
    int err;
    se10_pdev = platform_device_alloc("lenovo-se10-wdt", -1);
    if (!se10_pdev)
    return -ENOMEM;
    err = platform_device_add(se10_pdev);
    if (err) {
    platform_device_put(se10_pdev);
    se10_pdev = core::ptr::null_mut();
    }
    return err;
    }
    static const struct dmi_system_id se10_dmi_table[] __initconst = {
    {
    .ident = "LENOVO-SE10",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "12NH"),
    },
    },
    {
    .ident = "LENOVO-SE10",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "12NJ"),
    },
    },
    {
    .ident = "LENOVO-SE10",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "12NK"),
    },
    },
    {
    .ident = "LENOVO-SE10",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "12NL"),
    },
    },
    {
    .ident = "LENOVO-SE10",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "12NM"),
    },
    },
    {
    .ident = "LENOVO-SE10-G2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "13LJ"),
    },
    },
    {
    .ident = "LENOVO-SE10-G2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "13LK"),
    },
    },
    {
    .ident = "LENOVO-SE10-G2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "13S1"),
    },
    },
    {
    .ident = "LENOVO-SE10-G2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "13S2"),
    },
    },
    {
    .ident = "LENOVO-SE10-G2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "13S3"),
    },
    },
    {
    .ident = "LENOVO-SE10-G2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "13S4"),
    },
    },
    {
    .ident = "LENOVO-SE10-G2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "13S5"),
    },
    },
    {
    .ident = "LENOVO-SE10-G2",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "13S6"),
    },
    },
    {}
    };
    MODULE_DEVICE_TABLE(dmi, se10_dmi_table);
#[no_mangle]
unsafe extern "C" fn se10_wdt_init() -> int __init {
    static int __init se10_wdt_init(void)
    {
    int err;
    if (!dmi_check_system(se10_dmi_table))
    return -ENODEV;
    err = platform_driver_register(&se10_wdt_driver);
    if (err)
    return err;
    err = se10_create_platform_device();
    if (err)
    platform_driver_unregister(&se10_wdt_driver);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn se10_wdt_exit() -> void __exit {
    static void __exit se10_wdt_exit(void)
    {
    if (se10_pdev)
    platform_device_unregister(se10_pdev);
    platform_driver_unregister(&se10_wdt_driver);
    }
    module_init(se10_wdt_init);
    module_exit(se10_wdt_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Ober<dober@lenovo.com>");
    MODULE_AUTHOR("Mark Pearson <mpearson-lenovo@squebb.ca>");
    MODULE_DESCRIPTION("WDT driver for Lenovo SE10");
