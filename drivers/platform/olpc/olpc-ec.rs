//! Automatically rewritten from C to Rust
//! Source: drivers/platform/olpc/olpc-ec.c
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
// Generic driver for the OLPC Embedded Controller.
//
// Author: Andres Salomon <dilinger@queued.net>
//
// Copyright (C) 2011-2012 One Laptop per Child Foundation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_cmd_desc {
    pub cmd: u8,
    pub outbuf: *mut *mut u8 inbuf,,
    pub outlen: size_t inlen,,
    pub err: c_int,
    pub finished: completion,
    pub node: list_head,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct olpc_ec_priv {
    pub drv: *mut olpc_ec_driver,
    pub version: u8,
    pub worker: work_struct,
    pub cmd_lock: mutex,
// DCON regulator
    pub dcon_enabled: bool,
// Pending EC commands
    pub cmd_q: list_head,
    pub cmd_q_lock: spinlock_t,
    pub dbgfs_dir: *mut dentry,
//
// EC event mask to be applied during suspend (defining wakeup
// sources).
//
    pub ec_wakeup_mask: u16,
//
// Running an EC command while suspending means we don't always finish
// the command before the machine suspends.  This means that the EC
// is expecting the command protocol to finish, but we after a period
// of time (while the OS is asleep) the EC times out and restarts its
// idle loop.  Meanwhile, the OS wakes up, thinks it's still in the
// middle of the command protocol, starts throwing random things at
// the EC... and everyone's uphappy.
//
    pub suspended: bool,
}

    static struct olpc_ec_driver *ec_driver;
    static struct olpc_ec_priv *ec_priv;
    static void *ec_cb_arg;
#[no_mangle]
pub unsafe extern "C" fn olpc_ec_driver_register(drv: *mut olpc_ec_driver, arg: *mut c_void) {
    void olpc_ec_driver_register(struct olpc_ec_driver *drv, void *arg)
    {
    ec_driver = drv;
    ec_cb_arg = arg;
    }
    EXPORT_SYMBOL_GPL(olpc_ec_driver_register);
#[no_mangle]
unsafe extern "C" fn olpc_ec_worker(w: *mut work_struct) {
    static void olpc_ec_worker(struct work_struct *w)
    {
    struct olpc_ec_priv *ec = container_of(w, struct olpc_ec_priv, worker);
    struct ec_cmd_desc *desc = core::ptr::null_mut();
    unsigned long flags;
// Grab the first pending command from the queue
    spin_lock_irqsave(&ec.cmd_q_lock, flags);
    if (!list_empty(&ec.cmd_q)) {
    desc = list_first_entry(&ec.cmd_q, struct ec_cmd_desc, node);
    list_del(&desc.node);
    }
    spin_unlock_irqrestore(&ec.cmd_q_lock, flags);
// Do we actually have anything to do?
    if (!desc)
    return;
// Protect the EC hw with a mutex; only run one cmd at a time
    mutex_lock(&ec.cmd_lock);
    desc.err = ec_driver.ec_cmd(desc.cmd, desc.inbuf, desc.inlen,
    desc.outbuf, desc.outlen, ec_cb_arg);
    mutex_unlock(&ec.cmd_lock);
// Finished, wake up olpc_ec_cmd()
    complete(&desc.finished);
// Run the worker thread again in case there are more cmds pending
    schedule_work(&ec.worker);
    }
//
// Throw a cmd descripter onto the list.  We now have SMP OLPC machines, so
// locking is pretty critical.
//
    static void queue_ec_descriptor(struct ec_cmd_desc *desc,
    struct olpc_ec_priv *ec)
    {
    unsigned long flags;
    INIT_LIST_HEAD(&desc.node);
    spin_lock_irqsave(&ec.cmd_q_lock, flags);
    list_add_tail(&desc.node, &ec.cmd_q);
    spin_unlock_irqrestore(&ec.cmd_q_lock, flags);
    schedule_work(&ec.worker);
    }
#[no_mangle]
pub unsafe extern "C" fn olpc_ec_cmd(cmd: u8, inbuf: *mut u8, inlen: usize, outbuf: *mut u8, outlen: usize) -> c_int {
    int olpc_ec_cmd(u8 cmd, u8 *inbuf, size_t inlen, u8 *outbuf, size_t outlen)
    {
    struct olpc_ec_priv *ec = ec_priv;
    struct ec_cmd_desc desc;
// Driver not yet registered.
    if (!ec_driver)
    return -EPROBE_DEFER;
    if (WARN_ON(!ec_driver.ec_cmd))
    return -ENODEV;
    if (!ec)
    return -ENOMEM;
// Suspending in the middle of a command hoses things really badly
    if (WARN_ON(ec.suspended))
    return -EBUSY;
    might_sleep();
    desc.cmd = cmd;
    desc.inbuf = inbuf;
    desc.outbuf = outbuf;
    desc.inlen = inlen;
    desc.outlen = outlen;
    desc.err = 0;
    init_completion(&desc.finished);
    queue_ec_descriptor(&desc, ec);
// Timeouts must be handled in the platform-specific EC hook
    wait_for_completion(&desc.finished);
// The worker thread dequeues the cmd; no need to do anything here
    return desc.err;
    }
    EXPORT_SYMBOL_GPL(olpc_ec_cmd);
#[no_mangle]
pub unsafe extern "C" fn olpc_ec_wakeup_set(value: u16) {
    void olpc_ec_wakeup_set(u16 value)
    {
    struct olpc_ec_priv *ec = ec_priv;
    if (WARN_ON(!ec))
    return;
    ec.ec_wakeup_mask |= value;
    }
    EXPORT_SYMBOL_GPL(olpc_ec_wakeup_set);
#[no_mangle]
pub unsafe extern "C" fn olpc_ec_wakeup_clear(value: u16) {
    void olpc_ec_wakeup_clear(u16 value)
    {
    struct olpc_ec_priv *ec = ec_priv;
    if (WARN_ON(!ec))
    return;
    ec.ec_wakeup_mask &= ~value;
    }
    EXPORT_SYMBOL_GPL(olpc_ec_wakeup_clear);
#[no_mangle]
pub unsafe extern "C" fn olpc_ec_mask_write(bits: u16) -> c_int {
    int olpc_ec_mask_write(u16 bits)
    {
    struct olpc_ec_priv *ec = ec_priv;
    if (WARN_ON(!ec))
    return -ENODEV;
// EC version 0x5f adds support for wide SCI mask
    if (ec.version >= 0x5f) {
    let mut ec_word: __be16 = cpu_to_be16(bits);
    return olpc_ec_cmd(EC_WRITE_EXT_SCI_MASK, (void *)&ec_word, 2, core::ptr::null_mut(), 0);
    } else {
    let mut ec_byte: u8 = bits & 0xff;
    return olpc_ec_cmd(EC_WRITE_SCI_MASK, &ec_byte, 1, core::ptr::null_mut(), 0);
    }
    }
    EXPORT_SYMBOL_GPL(olpc_ec_mask_write);
//
// Returns true if the compile and runtime configurations allow for EC events
// to wake the system.
//
#[no_mangle]
pub unsafe extern "C" fn olpc_ec_wakeup_available() -> bool {
    bool olpc_ec_wakeup_available(void)
    {
    if (WARN_ON(!ec_driver))
    return false;
    return ec_driver.wakeup_available;
    }
    EXPORT_SYMBOL_GPL(olpc_ec_wakeup_available);
#[no_mangle]
pub unsafe extern "C" fn olpc_ec_sci_query(sci_value: *mut u16) -> c_int {
    int olpc_ec_sci_query(u16 *sci_value)
    {
    struct olpc_ec_priv *ec = ec_priv;
    int ret;
    if (WARN_ON(!ec))
    return -ENODEV;
// EC version 0x5f adds support for wide SCI mask
    if (ec.version >= 0x5f) {
    __be16 ec_word;
    ret = olpc_ec_cmd(EC_EXT_SCI_QUERY, core::ptr::null_mut(), 0, (void *)&ec_word, 2);
    if (ret == 0)
// sci_value = be16_to_cpu(ec_word);
    } else {
    u8 ec_byte;
    ret = olpc_ec_cmd(EC_SCI_QUERY, core::ptr::null_mut(), 0, &ec_byte, 1);
    if (ret == 0)
// sci_value = ec_byte;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(olpc_ec_sci_query);

//
// debugfs support for "generic commands", to allow sending
// arbitrary EC commands from userspace.
//

    static DEFINE_MUTEX(ec_dbgfs_lock);
    static unsigned char ec_dbgfs_resp[EC_MAX_CMD_REPLY];
    static unsigned int ec_dbgfs_resp_bytes;
    static ssize_t ec_dbgfs_cmd_write(struct file *file, const char __user *buf,
    size_t size, loff_t *ppos)
    {
    int i, m;
    unsigned char ec_cmd[EC_MAX_CMD_ARGS];
    unsigned int ec_cmd_int[EC_MAX_CMD_ARGS];
    char cmdbuf[64] = "";
    int ec_cmd_bytes;
    mutex_lock(&ec_dbgfs_lock);
    size = simple_write_to_buffer(cmdbuf, sizeof(cmdbuf), ppos, buf, size);
    m = sscanf(cmdbuf, "%x:%u %x %x %x %x %x", &ec_cmd_int[0],
    &ec_dbgfs_resp_bytes, &ec_cmd_int[1], &ec_cmd_int[2],
    &ec_cmd_int[3], &ec_cmd_int[4], &ec_cmd_int[5]);
    if (m < 2 || ec_dbgfs_resp_bytes > EC_MAX_CMD_REPLY) {
// reset to prevent overflow on read
    ec_dbgfs_resp_bytes = 0;
    pr_debug("olpc-ec: bad ec cmd:  cmd:response-count [arg1 [arg2 ...]]\n");
    size = -EINVAL;
    goto out;
    }
// convert scanf'd ints to char
    ec_cmd_bytes = m - 2;
    for (i = 0; i <= ec_cmd_bytes; i++)
    ec_cmd[i] = ec_cmd_int[i];
    pr_debug("olpc-ec: debugfs cmd 0x%02x with %d args %5ph, want %d returns\n",
    ec_cmd[0], ec_cmd_bytes, ec_cmd + 1,
    ec_dbgfs_resp_bytes);
    olpc_ec_cmd(ec_cmd[0], (ec_cmd_bytes == 0) ? core::ptr::null_mut() : &ec_cmd[1],
    ec_cmd_bytes, ec_dbgfs_resp, ec_dbgfs_resp_bytes);
    pr_debug("olpc-ec: response %8ph (%d bytes expected)\n",
    ec_dbgfs_resp, ec_dbgfs_resp_bytes);
    out:
    mutex_unlock(&ec_dbgfs_lock);
    return size;
    }
    static ssize_t ec_dbgfs_cmd_read(struct file *file, char __user *buf,
    size_t size, loff_t *ppos)
    {
    unsigned int i, r;
    char *rp;
    char respbuf[64];
    mutex_lock(&ec_dbgfs_lock);
    rp = respbuf;
    rp += sprintf(rp, "%02x", ec_dbgfs_resp[0]);
    for (i = 1; i < ec_dbgfs_resp_bytes; i++)
    rp += sprintf(rp, ", %02x", ec_dbgfs_resp[i]);
    mutex_unlock(&ec_dbgfs_lock);
    rp += sprintf(rp, "\n");
    r = rp - respbuf;
    return simple_read_from_buffer(buf, size, ppos, respbuf, r);
    }
    static const struct file_operations ec_dbgfs_ops = {
    .write = ec_dbgfs_cmd_write,
    .read = ec_dbgfs_cmd_read,
    };
    static struct dentry *olpc_ec_setup_debugfs(void)
    {
    struct dentry *dbgfs_dir;
    dbgfs_dir = debugfs_create_dir("olpc-ec", core::ptr::null_mut());
    debugfs_create_file("cmd", 0600, dbgfs_dir, core::ptr::null_mut(), &ec_dbgfs_ops);
    return dbgfs_dir;
    }

    static struct dentry *olpc_ec_setup_debugfs(void)
    {
    return core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn olpc_ec_set_dcon_power(ec: *mut olpc_ec_priv, state: bool) -> c_int {
    static int olpc_ec_set_dcon_power(struct olpc_ec_priv *ec, bool state)
    {
    let mut ec_byte: c_uchar = state;
    int ret;
    if (ec.dcon_enabled == state)
    return 0;
    ret = olpc_ec_cmd(EC_DCON_POWER_MODE, &ec_byte, 1, core::ptr::null_mut(), 0);
    if (ret)
    return ret;
    ec.dcon_enabled = state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dcon_regulator_enable(rdev: *mut regulator_dev) -> c_int {
    static int dcon_regulator_enable(struct regulator_dev *rdev)
    {
    struct olpc_ec_priv *ec = rdev_get_drvdata(rdev);
    return olpc_ec_set_dcon_power(ec, true);
    }
#[no_mangle]
unsafe extern "C" fn dcon_regulator_disable(rdev: *mut regulator_dev) -> c_int {
    static int dcon_regulator_disable(struct regulator_dev *rdev)
    {
    struct olpc_ec_priv *ec = rdev_get_drvdata(rdev);
    return olpc_ec_set_dcon_power(ec, false);
    }
#[no_mangle]
unsafe extern "C" fn dcon_regulator_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int dcon_regulator_is_enabled(struct regulator_dev *rdev)
    {
    struct olpc_ec_priv *ec = rdev_get_drvdata(rdev);
    return ec.dcon_enabled ? 1 : 0;
    }
    static const struct regulator_ops dcon_regulator_ops = {
    .enable		= dcon_regulator_enable,
    .disable	= dcon_regulator_disable,
    .is_enabled	= dcon_regulator_is_enabled,
    };
    static const struct regulator_desc dcon_desc = {
    .name		= "dcon",
    .id		= 0,
    .ops		= &dcon_regulator_ops,
    .type		= REGULATOR_VOLTAGE,
    .owner		= THIS_MODULE,
    .enable_time	= 25000,
    };
#[no_mangle]
unsafe extern "C" fn olpc_ec_probe(pdev: *mut platform_device) -> c_int {
    static int olpc_ec_probe(struct platform_device *pdev)
    {
    struct olpc_ec_priv *ec;
    let mut config: regulator_config = { };
    struct regulator_dev *regulator;
    int err;
    if (!ec_driver)
    return -ENODEV;
    ec = kzalloc_obj(*ec);
    if (!ec)
    return -ENOMEM;
    ec.drv = ec_driver;
    INIT_WORK(&ec.worker, olpc_ec_worker);
    mutex_init(&ec.cmd_lock);
    INIT_LIST_HEAD(&ec.cmd_q);
    spin_lock_init(&ec.cmd_q_lock);
    ec_priv = ec;
    platform_set_drvdata(pdev, ec);
// get the EC revision
    err = olpc_ec_cmd(EC_FIRMWARE_REV, core::ptr::null_mut(), 0, &ec.version, 1);
    if (err)
    goto error;
    config.dev = pdev.dev.parent;
    config.driver_data = ec;
    ec.dcon_enabled = true;
    regulator = devm_regulator_register(&pdev.dev, &dcon_desc, &config);
    if (IS_ERR(regulator)) {
    dev_err(&pdev.dev, "failed to register DCON regulator\n");
    err = PTR_ERR(regulator);
    goto error;
    }
    ec.dbgfs_dir = olpc_ec_setup_debugfs();
    return 0;
    error:
    ec_priv = core::ptr::null_mut();
    kfree(ec);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn olpc_ec_suspend(dev: *mut device) -> c_int {
    static int olpc_ec_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct olpc_ec_priv *ec = platform_get_drvdata(pdev);
    let mut err: c_int = 0;
    olpc_ec_mask_write(ec.ec_wakeup_mask);
    if (ec_driver.suspend)
    err = ec_driver.suspend(pdev);
    if (!err)
    ec.suspended = true;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn olpc_ec_resume(dev: *mut device) -> c_int {
    static int olpc_ec_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct olpc_ec_priv *ec = platform_get_drvdata(pdev);
    ec.suspended = false;
    return ec_driver.resume ? ec_driver.resume(pdev) : 0;
    }
    static const struct dev_pm_ops olpc_ec_pm_ops = {
    .suspend_late = olpc_ec_suspend,
    .resume_early = olpc_ec_resume,
    };
    static struct platform_driver olpc_ec_plat_driver = {
    .probe = olpc_ec_probe,
    .driver = {
    .name = "olpc-ec",
    .pm = &olpc_ec_pm_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn olpc_ec_init_module() -> int __init {
    static int __init olpc_ec_init_module(void)
    {
    return platform_driver_register(&olpc_ec_plat_driver);
    }
    arch_initcall(olpc_ec_init_module);
