//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/lenovo_se30_wdt.c
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
// WDT driver for Lenovo SE30 device
//

pub const IOREGION_LENGTH: c_int = 4;
pub const WATCHDOG_TIMEOUT: c_int = 60;
pub const MIN_TIMEOUT: c_int = 1;
pub const MAX_TIMEOUT: c_int = 255;
pub const MAX_WAIT: c_int = 10;
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

pub const LNV_SE30_ID: c_uint = 0x0110;
pub const CHIPID_MASK: c_uint = 0xFFF0;
pub const CHIPID_REG: c_uint = 0x20;
pub const SIO_REG: c_uint = 0x2e;
pub const LDN_REG: c_uint = 0x07;
pub const UNLOCK_KEY: c_uint = 0x87;
pub const LOCK_KEY: c_uint = 0xAA;
pub const LD_NUM_SHM: c_uint = 0x0F;
pub const LD_BASE_ADDR: c_uint = 0xF8;
pub const WDT_MODULE: c_uint = 0x10;
pub const WDT_CFG_INDEX: c_uint = 0x15 /* WD configuration register */;
pub const WDT_CNT_INDEX: c_uint = 0x16 /* WD timer count register */;
pub const WDT_CFG_RESET: c_uint = 0x2;
// Host Interface WIN2 offset definition
pub const SHM_WIN_SIZE: c_uint = 0xFF;
pub const SHM_WIN_MOD_OFFSET: c_uint = 0x01;
pub const SHM_WIN_CMD_OFFSET: c_uint = 0x02;
pub const SHM_WIN_SEL_OFFSET: c_uint = 0x03;
pub const SHM_WIN_CTL_OFFSET: c_uint = 0x04;
pub const VAL_SHM_WIN_CTRL_WR: c_uint = 0x40;
pub const VAL_SHM_WIN_CTRL_RD: c_uint = 0x80;
pub const SHM_WIN_ID_OFFSET: c_uint = 0x08;
pub const SHM_WIN_DAT_OFFSET: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nct6692_reg {
    pub mod: c_uchar,
    pub cmd: c_uchar,
    pub sel: c_uchar,
    pub idx: c_uint,
}

// Watchdog is based on NCT6692 device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lenovo_se30_wdt {
    pub shm_base_addr: *mut unsigned char __iomem,
    pub wdt_cfg: nct6692_reg,
    pub wdt_cnt: nct6692_reg,
    pub wdt: watchdog_device,
}

#[no_mangle]
pub unsafe extern "C" fn superio_outb(ioreg: c_int, reg: c_int, val: c_int) {
    static inline void superio_outb(int ioreg, int reg, int val)
    {
    outb(reg, ioreg);
    outb(val, ioreg + 1);
    }
#[no_mangle]
pub unsafe extern "C" fn superio_inb(ioreg: c_int, reg: c_int) -> c_int {
    static inline int superio_inb(int ioreg, int reg)
    {
    outb(reg, ioreg);
    return inb(ioreg + 1);
    }
#[no_mangle]
pub unsafe extern "C" fn superio_enter(key: c_int, addr: c_int, name: *const c_char) -> c_int {
    static inline int superio_enter(int key, int addr, const char *name)
    {
    if (!request_muxed_region(addr, 2, name)) {
    pr_err("I/O address 0x%04x already in use\n", addr);
    return -EBUSY;
    }
    outb(key, addr); /* Enter extended function mode */
    outb(key, addr); /* Again according to manual */
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn superio_exit(key: c_int, addr: c_int) {
    static inline void superio_exit(int key, int addr)
    {
    outb(key, addr); /* Leave extended function mode */
    release_region(addr, 2);
    }
    static int shm_get_ready(unsigned char __iomem *shm_base_addr,
    const struct nct6692_reg *reg)
    {
    unsigned char pre_id, new_id;
    let mut loop: c_int = 0;
    iowrite8(reg.mod, shm_base_addr + SHM_WIN_MOD_OFFSET);
    iowrite8(reg.cmd, shm_base_addr + SHM_WIN_CMD_OFFSET);
    iowrite8(reg.sel, shm_base_addr + SHM_WIN_SEL_OFFSET);
    pre_id = ioread8(shm_base_addr + SHM_WIN_ID_OFFSET);
    iowrite8(VAL_SHM_WIN_CTRL_RD, shm_base_addr + SHM_WIN_CTL_OFFSET);
// Loop checking when interface is ready
    while (loop < MAX_WAIT) {
    new_id = ioread8(shm_base_addr + SHM_WIN_ID_OFFSET);
    if (new_id != pre_id)
    return 0;
    loop++;
    usleep_range(10, 125);
    }
    return -ETIMEDOUT;
    }
    static int read_shm_win(unsigned char __iomem *shm_base_addr,
    const struct nct6692_reg *reg,
    unsigned char idx_offset,
    unsigned char *data)
    {
    let mut err: c_int = shm_get_ready(shm_base_addr, reg);
    if (err)
    return err;
// data = ioread8(shm_base_addr + SHM_WIN_DAT_OFFSET + reg->idx + idx_offset);
    return 0;
    }
    static int write_shm_win(unsigned char __iomem *shm_base_addr,
    const struct nct6692_reg *reg,
    unsigned char idx_offset,
    unsigned char val)
    {
    let mut err: c_int = shm_get_ready(shm_base_addr, reg);
    if (err)
    return err;
    iowrite8(val, shm_base_addr + SHM_WIN_DAT_OFFSET + reg.idx + idx_offset);
    iowrite8(VAL_SHM_WIN_CTRL_WR, shm_base_addr + SHM_WIN_CTL_OFFSET);
    err = shm_get_ready(shm_base_addr, reg);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn lenovo_se30_wdt_enable(data: *mut lenovo_se30_wdt, timeout: c_uint) -> c_int {
    static int lenovo_se30_wdt_enable(struct lenovo_se30_wdt *data, unsigned int timeout)
    {
    if (timeout) {
    let mut err: c_int = write_shm_win(data.shm_base_addr, &data.wdt_cfg, 0, WDT_CFG_RESET);
    if (err)
    return err;
    }
    return write_shm_win(data.shm_base_addr, &data.wdt_cnt, 0, timeout);
    }
#[no_mangle]
unsafe extern "C" fn lenovo_se30_wdt_start(wdog: *mut watchdog_device) -> c_int {
    static int lenovo_se30_wdt_start(struct watchdog_device *wdog)
    {
    struct lenovo_se30_wdt *data = watchdog_get_drvdata(wdog);
    return lenovo_se30_wdt_enable(data, wdog.timeout);
    }
#[no_mangle]
unsafe extern "C" fn lenovo_se30_wdt_stop(wdog: *mut watchdog_device) -> c_int {
    static int lenovo_se30_wdt_stop(struct watchdog_device *wdog)
    {
    struct lenovo_se30_wdt *data = watchdog_get_drvdata(wdog);
    return lenovo_se30_wdt_enable(data, 0);
    }
#[no_mangle]
unsafe extern "C" fn lenovo_se30_wdt_get_timeleft(wdog: *mut watchdog_device) -> c_uint {
    static unsigned int lenovo_se30_wdt_get_timeleft(struct watchdog_device *wdog)
    {
    struct lenovo_se30_wdt *data = watchdog_get_drvdata(wdog);
    unsigned char timeleft;
    int err;
    err = read_shm_win(data.shm_base_addr, &data.wdt_cnt, 0, &timeleft);
    if (err)
    return 0;
    return timeleft;
    }
#[no_mangle]
unsafe extern "C" fn lenovo_se30_wdt_ping(wdt: *mut watchdog_device) -> c_int {
    static int lenovo_se30_wdt_ping(struct watchdog_device *wdt)
    {
    struct lenovo_se30_wdt *data = watchdog_get_drvdata(wdt);
    let mut err: c_int = 0;
//
// Device does not support refreshing WDT_TIMER_REG register when
// the watchdog is active.  Need to disable, feed and enable again
//
    err = lenovo_se30_wdt_enable(data, 0);
    if (err)
    return err;
    err = write_shm_win(data.shm_base_addr, &data.wdt_cnt, 0, wdt.timeout);
    if (!err)
    err = lenovo_se30_wdt_enable(data, wdt.timeout);
    return err;
    }
    static const struct watchdog_info lenovo_se30_wdt_info = {
    .options	= WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    .identity	= "Lenovo SE30 watchdog",
    };
    static const struct watchdog_ops lenovo_se30_wdt_ops = {
    .owner		= THIS_MODULE,
    .start		= lenovo_se30_wdt_start,
    .stop		= lenovo_se30_wdt_stop,
    .ping		= lenovo_se30_wdt_ping,
    .get_timeleft	= lenovo_se30_wdt_get_timeleft,
    };
#[no_mangle]
unsafe extern "C" fn lenovo_se30_wdt_probe(pdev: *mut platform_device) -> c_int {
    static int lenovo_se30_wdt_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct lenovo_se30_wdt *priv;
    unsigned long base_phys;
    unsigned short val;
    int err;
    err = superio_enter(UNLOCK_KEY, SIO_REG, LNV_SE30_NAME);
    if (err)
    return err;
    val = superio_inb(SIO_REG, CHIPID_REG) << 8;
    val |= superio_inb(SIO_REG, CHIPID_REG + 1);
    if ((val & CHIPID_MASK) != LNV_SE30_ID) {
    superio_exit(LOCK_KEY, SIO_REG);
    return -ENODEV;
    }
    superio_outb(SIO_REG, LDN_REG, LD_NUM_SHM);
    base_phys = (superio_inb(SIO_REG, LD_BASE_ADDR) |
    (superio_inb(SIO_REG, LD_BASE_ADDR + 1) << 8) |
    (superio_inb(SIO_REG, LD_BASE_ADDR + 2) << 16) |
    (superio_inb(SIO_REG, LD_BASE_ADDR + 3) << 24)) &
    0xFFFFFFFF;
    superio_exit(LOCK_KEY, SIO_REG);
    if (base_phys == 0xFFFFFFFF || base_phys == 0)
    return -ENODEV;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    if (!devm_request_mem_region(dev, base_phys, SHM_WIN_SIZE, LNV_SE30_NAME))
    return -EBUSY;
    priv.shm_base_addr = devm_ioremap(dev, base_phys, SHM_WIN_SIZE);
    if (!priv.shm_base_addr)
    return -ENOMEM;
    priv.wdt_cfg.mod = WDT_MODULE;
    priv.wdt_cfg.idx = WDT_CFG_INDEX;
    priv.wdt_cnt.mod = WDT_MODULE;
    priv.wdt_cnt.idx = WDT_CNT_INDEX;
    priv.wdt.ops = &lenovo_se30_wdt_ops;
    priv.wdt.info = &lenovo_se30_wdt_info;
    priv.wdt.timeout = WATCHDOG_TIMEOUT; /* Set default timeout */
    priv.wdt.min_timeout = MIN_TIMEOUT;
    priv.wdt.max_timeout = MAX_TIMEOUT;
    priv.wdt.parent = dev;
    watchdog_init_timeout(&priv.wdt, timeout, dev);
    watchdog_set_drvdata(&priv.wdt, priv);
    watchdog_set_nowayout(&priv.wdt, nowayout);
    watchdog_stop_on_reboot(&priv.wdt);
    watchdog_stop_on_unregister(&priv.wdt);
    return devm_watchdog_register_device(dev, &priv.wdt);
    }
    static struct platform_device *pdev;
    static struct platform_driver lenovo_se30_wdt_driver = {
    .driver = {
    .name = LNV_SE30_NAME,
    },
    .probe  = lenovo_se30_wdt_probe,
    };
#[no_mangle]
unsafe extern "C" fn lenovo_se30_create_platform_device(id: *const dmi_system_id) -> c_int {
    static int lenovo_se30_create_platform_device(const struct dmi_system_id *id)
    {
    int err;
    pdev = platform_device_alloc(LNV_SE30_NAME, -1);
    if (!pdev)
    return -ENOMEM;
    err = platform_device_add(pdev);
    if (err)
    platform_device_put(pdev);
    return err;
    }
    static const struct dmi_system_id lenovo_se30_wdt_dmi_table[] __initconst = {
    {
    .ident = "LENOVO-SE30",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "11NA"),
    },
    .callback = lenovo_se30_create_platform_device,
    },
    {
    .ident = "LENOVO-SE30",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "11NB"),
    },
    .callback = lenovo_se30_create_platform_device,
    },
    {
    .ident = "LENOVO-SE30",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "11NC"),
    },
    .callback = lenovo_se30_create_platform_device,
    },
    {
    .ident = "LENOVO-SE30",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "11NH"),
    },
    .callback = lenovo_se30_create_platform_device,
    },
    {
    .ident = "LENOVO-SE30",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "11NJ"),
    },
    .callback = lenovo_se30_create_platform_device,
    },
    {
    .ident = "LENOVO-SE30",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "LENOVO"),
    DMI_MATCH(DMI_PRODUCT_NAME, "11NK"),
    },
    .callback = lenovo_se30_create_platform_device,
    },
    {}
    };
    MODULE_DEVICE_TABLE(dmi, lenovo_se30_wdt_dmi_table);
#[no_mangle]
unsafe extern "C" fn lenovo_se30_wdt_init() -> int __init {
    static int __init lenovo_se30_wdt_init(void)
    {
    if (!dmi_check_system(lenovo_se30_wdt_dmi_table))
    return -ENODEV;
    return platform_driver_register(&lenovo_se30_wdt_driver);
    }
#[no_mangle]
unsafe extern "C" fn lenovo_se30_wdt_exit() -> void __exit {
    static void __exit lenovo_se30_wdt_exit(void)
    {
    if (pdev)
    platform_device_unregister(pdev);
    platform_driver_unregister(&lenovo_se30_wdt_driver);
    }
    module_init(lenovo_se30_wdt_init);
    module_exit(lenovo_se30_wdt_exit);
    MODULE_AUTHOR("Mark Pearson <mpearson-lenovo@squebb.ca>");
    MODULE_AUTHOR("David Ober <dober@lenovo.com>");
    MODULE_DESCRIPTION("Lenovo SE30 watchdog driver");
    MODULE_LICENSE("GPL");
