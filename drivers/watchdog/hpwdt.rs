//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/hpwdt.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// HPE WatchDog Driver
// based on
//
// SoftDog	0.05:	A Software Watchdog Device
//
// (c) Copyright 2018 Hewlett Packard Enterprise Development LP
// Thomas Mingarelli <thomas.mingarelli@hpe.com>
//

pub const HPWDT_MAX_TICKS: c_int = 65535;

pub const DEFAULT_MARGIN: c_int = 30;
pub const PRETIMEOUT_SEC: c_int = 9;
    static unsigned int soft_margin = DEFAULT_MARGIN;	/* in seconds */
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    let mut pretimeout: static bool = IS_ENABLED(CONFIG_HPWDT_NMI_DECODING);
    let mut kdumptimeout: static int = -1;
    static void __iomem *pci_mem_addr;		/* the PCI-memory address */
    static unsigned long __iomem *hpwdt_nmistat;
    static unsigned long __iomem *hpwdt_timer_reg;
    static unsigned long __iomem *hpwdt_timer_con;
    static const struct pci_device_id hpwdt_devices[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_COMPAQ, 0xB203) },	/* iLO2 */
    { PCI_DEVICE(PCI_VENDOR_ID_HP, 0x3306) },	/* iLO3 */
    { PCI_DEVICE(PCI_VENDOR_ID_HP_3PAR, 0x0389) },	/* PCtrl */
    {0},			/* terminate list */
    };
    MODULE_DEVICE_TABLE(pci, hpwdt_devices);
    static const struct pci_device_id hpwdt_blacklist[] = {
    { PCI_DEVICE_SUB(PCI_VENDOR_ID_HP, 0x3306, PCI_VENDOR_ID_HP, 0x1979) }, /* auxilary iLO */
    { PCI_DEVICE_SUB(PCI_VENDOR_ID_HP, 0x3306, PCI_VENDOR_ID_HP_3PAR, 0x0289) },  /* CL */
    {0},			/* terminate list */
    };
    static struct watchdog_device hpwdt_dev;
//
// Watchdog operations
//
#[no_mangle]
unsafe extern "C" fn hpwdt_hw_is_running() -> c_int {
    static int hpwdt_hw_is_running(void)
    {
    return ioread8(hpwdt_timer_con) & 0x01;
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_start(wdd: *mut watchdog_device) -> c_int {
    static int hpwdt_start(struct watchdog_device *wdd)
    {
    let mut control: c_int = 0x81 | (pretimeout ? 0x4 : 0);
    let mut reload: c_int = SECS_TO_TICKS(min(wdd.timeout, wdd.max_hw_heartbeat_ms/1000));
    dev_dbg(wdd.parent, "start watchdog 0x%08x:0x%08x:0x%02x\n", wdd.timeout, reload, control);
    iowrite16(reload, hpwdt_timer_reg);
    iowrite8(control, hpwdt_timer_con);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_stop() {
    static void hpwdt_stop(void)
    {
    unsigned long data;
    pr_debug("stop  watchdog\n");
    data = ioread8(hpwdt_timer_con);
    data &= 0xFE;
    iowrite8(data, hpwdt_timer_con);
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_stop_core(wdd: *mut watchdog_device) -> c_int {
    static int hpwdt_stop_core(struct watchdog_device *wdd)
    {
    hpwdt_stop();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_ping_ticks(val: c_int) {
    static void hpwdt_ping_ticks(int val)
    {
    val = min(val, HPWDT_MAX_TICKS);
    iowrite16(val, hpwdt_timer_reg);
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_ping(wdd: *mut watchdog_device) -> c_int {
    static int hpwdt_ping(struct watchdog_device *wdd)
    {
    let mut reload: c_int = SECS_TO_TICKS(min(wdd.timeout, wdd.max_hw_heartbeat_ms/1000));
    dev_dbg(wdd.parent, "ping  watchdog 0x%08x:0x%08x\n", wdd.timeout, reload);
    hpwdt_ping_ticks(reload);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_gettimeleft(wdd: *mut watchdog_device) -> c_uint {
    static unsigned int hpwdt_gettimeleft(struct watchdog_device *wdd)
    {
    return TICKS_TO_SECS(ioread16(hpwdt_timer_reg));
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_settimeout(wdd: *mut watchdog_device, val: c_uint) -> c_int {
    static int hpwdt_settimeout(struct watchdog_device *wdd, unsigned int val)
    {
    dev_dbg(wdd.parent, "set_timeout = %d\n", val);
    wdd.timeout = val;
    if (val <= wdd.pretimeout) {
    dev_dbg(wdd.parent, "pretimeout < timeout. Setting to zero\n");
    wdd.pretimeout = 0;
    pretimeout = false;
    if (watchdog_active(wdd))
    hpwdt_start(wdd);
    }
    hpwdt_ping(wdd);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn hpwdt_set_pretimeout(wdd: *mut watchdog_device, req: c_uint) -> c_int {
    static int hpwdt_set_pretimeout(struct watchdog_device *wdd, unsigned int req)
    {
    let mut val: c_uint = 0;
    dev_dbg(wdd.parent, "set_pretimeout = %d\n", req);
    if (req) {
    val = PRETIMEOUT_SEC;
    if (val >= wdd.timeout)
    return -EINVAL;
    }
    if (val != req)
    dev_dbg(wdd.parent, "Rounding pretimeout to: %d\n", val);
    wdd.pretimeout = val;
    pretimeout = !!val;
    if (watchdog_active(wdd))
    hpwdt_start(wdd);
    return 0;
    }

//
// NMI Handler
//
#[no_mangle]
unsafe extern "C" fn hpwdt_pretimeout(ulReason: c_uint, regs: *mut pt_regs) -> c_int {
    static int hpwdt_pretimeout(unsigned int ulReason, struct pt_regs *regs)
    {
    let mut nmistat: u8 = ioread8(hpwdt_nmistat);
    let mut mynmi: bool = (nmistat & (NMISTAT_EWDOG | NMISTAT_RTRAP)) != 0;
    static char panic_msg_default[] =
    "00: An NMI occurred. Depending on your system the reason "
    "for the NMI is logged in any one of the following resources:\n"
    "1. Integrated Management Log (IML)\n"
    "2. OA Syslog\n"
    "3. OA Forward Progress Log\n"
    "4. iLO Event Log";
    static char panic_msg_uv[] =
    "00: A watchdog NMI occurred.";
    char *panic_msg = is_uv_system() ? panic_msg_uv : panic_msg_default;
    if (ulReason == NMI_UNKNOWN && !mynmi)
    return NMI_DONE;
    if (kdumptimeout < 0)
    hpwdt_stop();
#[no_mangle]
pub unsafe extern "C" fn if(0: kdumptimeout ==) -> else {
    else if (kdumptimeout == 0)
    ;
    else {
    let mut val: c_uint = max((unsigned int)kdumptimeout, hpwdt_dev.timeout);
    hpwdt_ping_ticks(SECS_TO_TICKS(val));
    }
    hex_byte_pack(panic_msg, nmistat);
    nmi_panic(regs, panic_msg);
    return NMI_HANDLED;
    }

    static const struct watchdog_info ident = {
    .options = WDIOF_PRETIMEOUT    |
    WDIOF_SETTIMEOUT    |
    WDIOF_KEEPALIVEPING |
    WDIOF_MAGICCLOSE,
    .identity = "HPE iLO2+ HW Watchdog Timer",
    };
//
// Kernel interfaces
//
    static const struct watchdog_ops hpwdt_ops = {
    .owner		= THIS_MODULE,
    .start		= hpwdt_start,
    .stop		= hpwdt_stop_core,
    .ping		= hpwdt_ping,
    .set_timeout	= hpwdt_settimeout,
    .get_timeleft	= hpwdt_gettimeleft,

    .set_pretimeout	= hpwdt_set_pretimeout,

    };
    static struct watchdog_device hpwdt_dev = {
    .info		= &ident,
    .ops		= &hpwdt_ops,
    .min_timeout	= 1,
    .timeout	= DEFAULT_MARGIN,
    .pretimeout	= PRETIMEOUT_SEC,
    .max_hw_heartbeat_ms	= HPWDT_MAX_TIMER * 1000,
    };
//
// Init & Exit
//
#[no_mangle]
unsafe extern "C" fn hpwdt_init_nmi_decoding(dev: *mut pci_dev) -> c_int {
    static int hpwdt_init_nmi_decoding(struct pci_dev *dev)
    {

    int retval;
//
// Only one function can register for NMI_UNKNOWN
//
    retval = register_nmi_handler(NMI_UNKNOWN, hpwdt_pretimeout, 0, "hpwdt");
    if (retval)
    goto error;
    retval = register_nmi_handler(NMI_SERR, hpwdt_pretimeout, 0, "hpwdt");
    if (retval)
    goto error1;
    retval = register_nmi_handler(NMI_IO_CHECK, hpwdt_pretimeout, 0, "hpwdt");
    if (retval)
    goto error2;
    dev_info(&dev.dev,
    "HPE Watchdog Timer Driver: NMI decoding initialized\n");
    return 0;
    error2:
    unregister_nmi_handler(NMI_SERR, "hpwdt");
    error1:
    unregister_nmi_handler(NMI_UNKNOWN, "hpwdt");
    error:
    dev_warn(&dev.dev,
    "Unable to register a die notifier (err=%d).\n",
    retval);
    return retval;

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_exit_nmi_decoding() {
    static void hpwdt_exit_nmi_decoding(void)
    {

    unregister_nmi_handler(NMI_UNKNOWN, "hpwdt");
    unregister_nmi_handler(NMI_SERR, "hpwdt");
    unregister_nmi_handler(NMI_IO_CHECK, "hpwdt");

    }
    static int hpwdt_init_one(struct pci_dev *dev,
    const struct pci_device_id *ent)
    {
    int retval;
//
// First let's find out if we are on an iLO2+ server. We will
// not run on a legacy ASM box.
// So we only support the G5 ProLiant servers and higher.
//
    if (dev.subsystem_vendor != PCI_VENDOR_ID_HP &&
    dev.subsystem_vendor != PCI_VENDOR_ID_HP_3PAR) {
    dev_warn(&dev.dev,
    "This server does not have an iLO2+ ASIC.\n");
    return -ENODEV;
    }
    if (pci_match_id(hpwdt_blacklist, dev)) {
    dev_dbg(&dev.dev, "Not supported on this device\n");
    return -ENODEV;
    }
    if (pci_enable_device(dev)) {
    dev_warn(&dev.dev,
    "Not possible to enable PCI Device: 0x%x:0x%x.\n",
    ent.vendor, ent.device);
    return -ENODEV;
    }
    pci_mem_addr = pci_iomap(dev, 1, 0x80);
    if (!pci_mem_addr) {
    dev_warn(&dev.dev,
    "Unable to detect the iLO2+ server memory.\n");
    retval = -ENOMEM;
    goto error_pci_iomap;
    }
    hpwdt_nmistat	= pci_mem_addr + 0x6e;
    hpwdt_timer_reg = pci_mem_addr + 0x70;
    hpwdt_timer_con = pci_mem_addr + 0x72;
// Have the core update running timer until user space is ready
    if (hpwdt_hw_is_running()) {
    dev_info(&dev.dev, "timer is running\n");
    set_bit(WDOG_HW_RUNNING, &hpwdt_dev.status);
    }
// Initialize NMI Decoding functionality
    retval = hpwdt_init_nmi_decoding(dev);
    if (retval != 0)
    goto error_init_nmi_decoding;
    watchdog_stop_on_unregister(&hpwdt_dev);
    watchdog_set_nowayout(&hpwdt_dev, nowayout);
    watchdog_init_timeout(&hpwdt_dev, soft_margin, core::ptr::null_mut());
    if (is_kdump_kernel()) {
    pretimeout = false;
    kdumptimeout = 0;
    }
    if (pretimeout && hpwdt_dev.timeout <= PRETIMEOUT_SEC) {
    dev_warn(&dev.dev, "timeout <= pretimeout. Setting pretimeout to zero\n");
    pretimeout = false;
    }
    hpwdt_dev.pretimeout = pretimeout ? PRETIMEOUT_SEC : 0;
    kdumptimeout = min(kdumptimeout, HPWDT_MAX_TIMER);
    hpwdt_dev.parent = &dev.dev;
    retval = watchdog_register_device(&hpwdt_dev);
    if (retval < 0)
    goto error_wd_register;
    dev_info(&dev.dev, "HPE Watchdog Timer Driver: Version: %s\n",
    HPWDT_VERSION);
    dev_info(&dev.dev, "timeout: %d seconds (nowayout=%d)\n",
    hpwdt_dev.timeout, nowayout);
    dev_info(&dev.dev, "pretimeout: %s.\n",
    pretimeout ? "on" : "off");
    dev_info(&dev.dev, "kdumptimeout: %d.\n", kdumptimeout);
    return 0;
    error_wd_register:
    hpwdt_exit_nmi_decoding();
    error_init_nmi_decoding:
    pci_iounmap(dev, pci_mem_addr);
    error_pci_iomap:
    pci_disable_device(dev);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_exit(dev: *mut pci_dev) {
    static void hpwdt_exit(struct pci_dev *dev)
    {
    watchdog_unregister_device(&hpwdt_dev);
    hpwdt_exit_nmi_decoding();
    pci_iounmap(dev, pci_mem_addr);
    pci_disable_device(dev);
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_suspend(dev: *mut device) -> c_int {
    static int hpwdt_suspend(struct device *dev)
    {
    if (watchdog_active(&hpwdt_dev))
    hpwdt_stop();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hpwdt_resume(dev: *mut device) -> c_int {
    static int hpwdt_resume(struct device *dev)
    {
    if (watchdog_active(&hpwdt_dev))
    hpwdt_start(&hpwdt_dev);
    return 0;
    }
    static const struct dev_pm_ops hpwdt_pm_ops = {
    LATE_SYSTEM_SLEEP_PM_OPS(hpwdt_suspend, hpwdt_resume)
    };
    static struct pci_driver hpwdt_driver = {
    .name = "hpwdt",
    .id_table = hpwdt_devices,
    .probe = hpwdt_init_one,
    .remove = hpwdt_exit,
    .driver = {
    .name = "hpwdt",
    .pm = &hpwdt_pm_ops,
    }
    };
    MODULE_AUTHOR("Tom Mingarelli");
    MODULE_DESCRIPTION("hpe watchdog driver");
    MODULE_LICENSE("GPL");
    MODULE_VERSION(HPWDT_VERSION);
    module_param(soft_margin, int, 0);
    MODULE_PARM_DESC(soft_margin, "Watchdog timeout in seconds");
    module_param_named(timeout, soft_margin, int, 0);
    MODULE_PARM_DESC(timeout, "Alias of soft_margin");
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
    module_param(kdumptimeout, int, 0444);
    MODULE_PARM_DESC(kdumptimeout, "Timeout applied for crash kernel transition in seconds");

    module_param(pretimeout, bool, 0);
    MODULE_PARM_DESC(pretimeout, "Watchdog pretimeout enabled");

    module_pci_driver(hpwdt_driver);
