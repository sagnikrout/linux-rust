//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/exar_wdt.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// exar_wdt.c - Driver for the watchdog present in some
// Exar/MaxLinear UART chips like the XR28V38x.
//
// (c) Copyright 2022 D. Müller <d.mueller@elsoft.ch>.
//

    static const unsigned short sio_config_ports[] = { 0x2e, 0x4e };
    static const unsigned char sio_enter_keys[] = { 0x67, 0x77, 0x87, 0xA0 };
pub const EXAR_EXIT_KEY: c_uint = 0xAA;
pub const EXAR_LDN: c_uint = 0x07;
pub const EXAR_DID: c_uint = 0x20;
pub const EXAR_VID: c_uint = 0x23;
pub const EXAR_WDT: c_uint = 0x26;
pub const EXAR_ACT: c_uint = 0x30;
pub const EXAR_RTBASE: c_uint = 0x60;
pub const EXAR_WDT_LDEV: c_uint = 0x08;
pub const EXAR_VEN_ID: c_uint = 0x13A8;
pub const EXAR_DEV_382: c_uint = 0x0382;
pub const EXAR_DEV_384: c_uint = 0x0384;
// WDT runtime registers
pub const WDT_CTRL: c_uint = 0x00;
pub const WDT_VAL: c_uint = 0x01;
pub const WDT_UNITS_10MS: c_uint = 0x0	/* the 10 millisec unit of the HW is not used */;
pub const WDT_UNITS_SEC: c_uint = 0x2;
pub const WDT_UNITS_MIN: c_uint = 0x4;
// default WDT control for WDTOUT signal activ / rearm by read
pub const EXAR_WDT_DEF_CONF: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wdt_pdev_node {
    pub list: list_head,
    pub pdev: *mut platform_device,
    pub name: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wdt_priv {
// the lock for WDT io operations
    pub io_lock: spinlock_t,
    pub wdt_res: resource,
    pub wdt_dev: watchdog_device,
    pub did: c_ushort,
    pub config_port: c_ushort,
    pub enter_key: c_uchar,
    pub unit: c_uchar,
    pub timeout: c_uchar,
}

pub const WATCHDOG_TIMEOUT: c_int = 60;
    let mut timeout: static int = WATCHDOG_TIMEOUT;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout,
    "Watchdog timeout in seconds. 1<=timeout<=15300, default="
    __MODULE_STRING(WATCHDOG_TIMEOUT) ".");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    static int exar_sio_enter(const unsigned short config_port,
    const unsigned char key)
    {
    if (!request_muxed_region(config_port, 2, DRV_NAME))
    return -EBUSY;
// write the ENTER-KEY twice
    outb(key, config_port);
    outb(key, config_port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exar_sio_exit(config_port: c_ushort) {
    static void exar_sio_exit(const unsigned short config_port)
    {
    outb(EXAR_EXIT_KEY, config_port);
    release_region(config_port, 2);
    }
    static unsigned char exar_sio_read(const unsigned short config_port,
    const unsigned char reg)
    {
    outb(reg, config_port);
    return inb(config_port + 1);
    }
    static void exar_sio_write(const unsigned short config_port,
    const unsigned char reg, const unsigned char val)
    {
    outb(reg, config_port);
    outb(val, config_port + 1);
    }
    static unsigned short exar_sio_read16(const unsigned short config_port,
    const unsigned char reg)
    {
    unsigned char msb, lsb;
    msb = exar_sio_read(config_port, reg);
    lsb = exar_sio_read(config_port, reg + 1);
    return (msb << 8) | lsb;
    }
#[no_mangle]
unsafe extern "C" fn exar_sio_select_wdt(config_port: c_ushort) {
    static void exar_sio_select_wdt(const unsigned short config_port)
    {
    exar_sio_write(config_port, EXAR_LDN, EXAR_WDT_LDEV);
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_arm(priv: *const wdt_priv) {
    static void exar_wdt_arm(const struct wdt_priv *priv)
    {
    let mut rt_base: c_ushort = priv.wdt_res.start;
// write timeout value twice to arm watchdog
    outb(priv.timeout, rt_base + WDT_VAL);
    outb(priv.timeout, rt_base + WDT_VAL);
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_disarm(priv: *const wdt_priv) {
    static void exar_wdt_disarm(const struct wdt_priv *priv)
    {
    let mut rt_base: c_ushort = priv.wdt_res.start;
//
// use two accesses with different values to make sure
// that a combination of a previous single access and
// the ones below with the same value are not falsely
// interpreted as "arm watchdog"
//
    outb(0xFF, rt_base + WDT_VAL);
    outb(0, rt_base + WDT_VAL);
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int exar_wdt_start(struct watchdog_device *wdog)
    {
    struct wdt_priv *priv = watchdog_get_drvdata(wdog);
    let mut rt_base: c_ushort = priv.wdt_res.start;
    spin_lock(&priv.io_lock);
    exar_wdt_disarm(priv);
    outb(priv.unit, rt_base + WDT_CTRL);
    exar_wdt_arm(priv);
    spin_unlock(&priv.io_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int exar_wdt_stop(struct watchdog_device *wdog)
    {
    struct wdt_priv *priv = watchdog_get_drvdata(wdog);
    spin_lock(&priv.io_lock);
    exar_wdt_disarm(priv);
    spin_unlock(&priv.io_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_keepalive(wdog: *mut watchdog_device) -> c_int {
    static int exar_wdt_keepalive(struct watchdog_device *wdog)
    {
    struct wdt_priv *priv = watchdog_get_drvdata(wdog);
    let mut rt_base: c_ushort = priv.wdt_res.start;
    spin_lock(&priv.io_lock);
// reading the WDT_VAL reg will feed the watchdog
    inb(rt_base + WDT_VAL);
    spin_unlock(&priv.io_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_set_timeout(wdog: *mut watchdog_device, t: c_uint) -> c_int {
    static int exar_wdt_set_timeout(struct watchdog_device *wdog, unsigned int t)
    {
    struct wdt_priv *priv = watchdog_get_drvdata(wdog);
    let mut unit_min: bool = false;
//
// if new timeout is bigger then 255 seconds, change the
// unit to minutes and round the timeout up to the next whole minute
//
    if (t > 255) {
    unit_min = true;
    t = DIV_ROUND_UP(t, 60);
    }
// save for later use in exar_wdt_start()
    priv.unit = unit_min ? WDT_UNITS_MIN : WDT_UNITS_SEC;
    priv.timeout = t;
    wdog.timeout = unit_min ? t * 60 : t;
    if (watchdog_hw_running(wdog))
    exar_wdt_start(wdog);
    return 0;
    }
    static const struct watchdog_info exar_wdt_info = {
    .options	= WDIOF_KEEPALIVEPING |
    WDIOF_SETTIMEOUT |
    WDIOF_MAGICCLOSE,
    .identity	= "Exar XR28V38x Watchdog",
    };
    static const struct watchdog_ops exar_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= exar_wdt_start,
    .stop		= exar_wdt_stop,
    .ping		= exar_wdt_keepalive,
    .set_timeout	= exar_wdt_set_timeout,
    };
    static int exar_wdt_config(struct watchdog_device *wdog,
    const unsigned char conf)
    {
    struct wdt_priv *priv = watchdog_get_drvdata(wdog);
    int ret;
    ret = exar_sio_enter(priv.config_port, priv.enter_key);
    if (ret)
    return ret;
    exar_sio_select_wdt(priv.config_port);
    exar_sio_write(priv.config_port, EXAR_WDT, conf);
    exar_sio_exit(priv.config_port);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_probe(pdev: *mut platform_device) -> int __init {
    static int __init exar_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct wdt_priv *priv = dev.platform_data;
    struct watchdog_device *wdt_dev = &priv.wdt_dev;
    struct resource *res;
    int ret;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res)
    return -ENXIO;
    spin_lock_init(&priv.io_lock);
    wdt_dev.info = &exar_wdt_info;
    wdt_dev.ops = &exar_wdt_ops;
    wdt_dev.min_timeout = 1;
    wdt_dev.max_timeout = 255 * 60;
    watchdog_init_timeout(wdt_dev, timeout, core::ptr::null_mut());
    watchdog_set_nowayout(wdt_dev, nowayout);
    watchdog_stop_on_reboot(wdt_dev);
    watchdog_stop_on_unregister(wdt_dev);
    watchdog_set_drvdata(wdt_dev, priv);
    ret = exar_wdt_config(wdt_dev, EXAR_WDT_DEF_CONF);
    if (ret)
    return ret;
    exar_wdt_set_timeout(wdt_dev, timeout);
// Make sure that the watchdog is not running
    exar_wdt_stop(wdt_dev);
    ret = devm_watchdog_register_device(dev, wdt_dev);
    if (ret)
    return ret;
    dev_info(dev, "XR28V%X WDT initialized. timeout=%d sec (nowayout=%d)\n",
    priv.did, timeout, nowayout);
    return 0;
    }
    static unsigned short __init exar_detect(const unsigned short config_port,
    const unsigned char key,
    unsigned short *rt_base)
    {
    int ret;
    let mut base: c_ushort = 0;
    unsigned short vid, did;
    ret = exar_sio_enter(config_port, key);
    if (ret)
    return 0;
    vid = exar_sio_read16(config_port, EXAR_VID);
    did = exar_sio_read16(config_port, EXAR_DID);
// check for the vendor and device IDs we currently know about
    if (vid == EXAR_VEN_ID &&
    (did == EXAR_DEV_382 ||
    did == EXAR_DEV_384)) {
    exar_sio_select_wdt(config_port);
// is device active?
    if (exar_sio_read(config_port, EXAR_ACT) == 0x01)
    base = exar_sio_read16(config_port, EXAR_RTBASE);
    }
    exar_sio_exit(config_port);
    if (base) {
    pr_debug("Found a XR28V%X WDT (conf: 0x%x / rt: 0x%04x)\n",
    did, config_port, base);
// rt_base = base;
    return did;
    }
    return 0;
    }
    static struct platform_driver exar_wdt_driver = {
    .driver = {
    .name = DRV_NAME,
    },
    };
    static LIST_HEAD(pdev_list);
#[no_mangle]
unsafe extern "C" fn exar_wdt_register(priv: *mut wdt_priv, idx: c_int) -> int __init {
    static int __init exar_wdt_register(struct wdt_priv *priv, const int idx)
    {
    struct wdt_pdev_node *n;
    n = kzalloc_obj(*n);
    if (!n)
    return -ENOMEM;
    INIT_LIST_HEAD(&n.list);
    scnprintf((char *)n.name, sizeof(n.name), DRV_NAME ".%d", idx);
    priv.wdt_res.name = n.name;
    n.pdev = platform_device_register_resndata(core::ptr::null_mut(), DRV_NAME, idx,
    &priv.wdt_res, 1,
    priv, sizeof(*priv));
    if (IS_ERR(n.pdev)) {
    let mut err: c_int = PTR_ERR(n.pdev);
    kfree(n);
    return err;
    }
    list_add_tail(&n.list, &pdev_list);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_unregister() {
    static void exar_wdt_unregister(void)
    {
    struct wdt_pdev_node *n, *t;
    list_for_each_entry_safe(n, t, &pdev_list, list) {
    platform_device_unregister(n.pdev);
    list_del(&n.list);
    kfree(n);
    }
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_init() -> int __init {
    static int __init exar_wdt_init(void)
    {
    int ret, i, j, idx = 0;
// search for active Exar watchdogs on all possible locations
    for (i = 0; i < ARRAY_SIZE(sio_config_ports); i++) {
    for (j = 0; j < ARRAY_SIZE(sio_enter_keys); j++) {
    unsigned short did, rt_base = 0;
    did = exar_detect(sio_config_ports[i],
    sio_enter_keys[j],
    &rt_base);
    if (did) {
    struct wdt_priv priv = {
    .wdt_res = DEFINE_RES_IO(rt_base, 2),
    .did = did,
    .config_port = sio_config_ports[i],
    .enter_key = sio_enter_keys[j],
    };
    ret = exar_wdt_register(&priv, idx);
    if (!ret)
    idx++;
    }
    }
    }
    if (!idx)
    return -ENODEV;
    ret = platform_driver_probe(&exar_wdt_driver, exar_wdt_probe);
    if (ret)
    exar_wdt_unregister();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exar_wdt_exit() -> void __exit {
    static void __exit exar_wdt_exit(void)
    {
    exar_wdt_unregister();
    platform_driver_unregister(&exar_wdt_driver);
    }
    module_init(exar_wdt_init);
    module_exit(exar_wdt_exit);
    MODULE_AUTHOR("David Müller <d.mueller@elsoft.ch>");
    MODULE_DESCRIPTION("Exar/MaxLinear Watchdog Driver");
    MODULE_LICENSE("GPL");
