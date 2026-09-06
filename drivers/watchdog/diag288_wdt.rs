//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/diag288_wdt.c
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
// Watchdog driver for z/VM and LPAR using the diag 288 interface.
//
// Under z/VM, expiration of the watchdog will send a "system restart" command
// to CP.
//
// The command can be altered using the module parameter "cmd". This is
// not recommended because it's only supported on z/VM but not with LPAR.
//
// On LPAR, the watchdog will always trigger a system restart. The module
// parameter cmd is meaningless here.
//
// Copyright IBM Corp. 2004, 2013
// Author(s): Arnd Bergmann (arndb@de.ibm.com)
// Philipp Hachtmann (phacht@de.ibm.com)
//

pub const MAX_CMDLEN: c_int = 240;

    static char wdt_cmd[MAX_CMDLEN] = DEFAULT_CMD;
    static bool conceal_on;
    let mut nowayout_info: static bool = WATCHDOG_NOWAYOUT;
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Arnd Bergmann <arndb@de.ibm.com>");
    MODULE_AUTHOR("Philipp Hachtmann <phacht@de.ibm.com>");
    MODULE_DESCRIPTION("System z diag288  Watchdog Timer");
    module_param_string(cmd, wdt_cmd, MAX_CMDLEN, 0644);
    MODULE_PARM_DESC(cmd, "CP command that is run when the watchdog triggers (z/VM only)");
    module_param_named(conceal, conceal_on, bool, 0644);
    MODULE_PARM_DESC(conceal, "Enable the CONCEAL CP option while the watchdog is active (z/VM only)");
    module_param_named(nowayout, nowayout_info, bool, 0444);
    MODULE_PARM_DESC(nowayout, "Watchdog cannot be stopped once started (default = CONFIG_WATCHDOG_NOWAYOUT)");
    MODULE_ALIAS("vmwatchdog");
    static char *cmd_buf;
    static int diag288(unsigned int func, unsigned int timeout,
    unsigned long action, unsigned int len)
    {
    diag_stat_inc(DIAG_STAT_X288);
    return __diag288(func, timeout, action, len);
    }
#[no_mangle]
unsafe extern "C" fn diag288_str(func: c_uint, timeout: c_uint, cmd: *mut c_char) -> c_int {
    static int diag288_str(unsigned int func, unsigned int timeout, char *cmd)
    {
    ssize_t len;
    len = strscpy(cmd_buf, cmd, MAX_CMDLEN);
    if (len < 0)
    return len;
    ASCEBC(cmd_buf, MAX_CMDLEN);
    EBC_TOUPPER(cmd_buf, MAX_CMDLEN);
    return diag288(func, timeout, virt_to_phys(cmd_buf), len);
    }
#[no_mangle]
unsafe extern "C" fn wdt_start(dev: *mut watchdog_device) -> c_int {
    static int wdt_start(struct watchdog_device *dev)
    {
    int ret;
    unsigned int func;
    if (machine_is_vm()) {
    func = conceal_on ? (WDT_FUNC_INIT | WDT_FUNC_CONCEAL)
    : WDT_FUNC_INIT;
    ret = diag288_str(func, dev.timeout, wdt_cmd);
    WARN_ON(ret != 0);
    } else {
    ret = diag288(WDT_FUNC_INIT, dev.timeout, LPARWDT_RESTART, 0);
    }
    if (ret) {
    pr_err("The watchdog cannot be activated\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wdt_stop(dev: *mut watchdog_device) -> c_int {
    static int wdt_stop(struct watchdog_device *dev)
    {
    return diag288(WDT_FUNC_CANCEL, 0, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn wdt_ping(dev: *mut watchdog_device) -> c_int {
    static int wdt_ping(struct watchdog_device *dev)
    {
    int ret;
    unsigned int func;
    if (machine_is_vm()) {
//
// It seems to be ok to z/VM to use the init function to
// retrigger the watchdog. On LPAR WDT_FUNC_CHANGE must
// be used when the watchdog is running.
//
    func = conceal_on ? (WDT_FUNC_INIT | WDT_FUNC_CONCEAL)
    : WDT_FUNC_INIT;
    ret = diag288_str(func, dev.timeout, wdt_cmd);
    WARN_ON(ret != 0);
    } else {
    ret = diag288(WDT_FUNC_CHANGE, dev.timeout, 0, 0);
    }
    if (ret)
    pr_err("The watchdog timer cannot be started or reset\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wdt_set_timeout(dev: *mut *mut watchdog_device, new_to: c_uint) -> c_int {
    static int wdt_set_timeout(struct watchdog_device * dev, unsigned int new_to)
    {
    dev.timeout = new_to;
    return wdt_ping(dev);
    }
    static const struct watchdog_ops wdt_ops = {
    .owner = THIS_MODULE,
    .start = wdt_start,
    .stop = wdt_stop,
    .ping = wdt_ping,
    .set_timeout = wdt_set_timeout,
    };
    static const struct watchdog_info wdt_info = {
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    .firmware_version = 0,
    .identity = "z Watchdog",
    };
    static struct watchdog_device wdt_dev = {
    .parent = core::ptr::null_mut(),
    .info = &wdt_info,
    .ops = &wdt_ops,
    .bootstatus = 0,
    .timeout = WDT_DEFAULT_TIMEOUT,
    .min_timeout = MIN_INTERVAL,
    .max_timeout = MAX_INTERVAL,
    };
#[no_mangle]
unsafe extern "C" fn diag288_init() -> int __init {
    static int __init diag288_init(void)
    {
    watchdog_set_nowayout(&wdt_dev, nowayout_info);
    if (machine_is_vm()) {
    cmd_buf = kmalloc(MAX_CMDLEN, GFP_KERNEL);
    if (!cmd_buf) {
    pr_err("The watchdog cannot be initialized\n");
    return -ENOMEM;
    }
    }
    return watchdog_register_device(&wdt_dev);
    }
#[no_mangle]
unsafe extern "C" fn diag288_exit() -> void __exit {
    static void __exit diag288_exit(void)
    {
    watchdog_unregister_device(&wdt_dev);
    kfree(cmd_buf);
    }
    module_cpu_feature_match(S390_CPU_FEATURE_D288, diag288_init);
    module_exit(diag288_exit);
