//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/alim7101_wdt.c
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
// ALi M7101 PMU Computer Watchdog Timer driver
//
// Based on w83877f_wdt.c by Scott Jennings <linuxdrivers@oro.net>
// and the Cobalt kernel WDT timer driver by Tim Hockin
// <thockin@cobaltnet.com>
//
// (c)2002 Steve Hill <steve@navaho.co.uk>
//
// This WDT driver is different from most other Linux WDT
// drivers in that the driver will ping the watchdog by itself,
// because this particular WDT has a very short timeout (1.6
// seconds) and it would be insane to count on any userspace
// daemon always getting scheduled within that time frame.
//
// Additions:
// Aug 23, 2004 - Added use_gpio module parameter for use on revision a1d PMUs
// found on very old cobalt hardware.
// -- Mike Waychison <michael.waychison@sun.com>
//

pub const WDT_ENABLE: c_uint = 0x9C;
pub const WDT_DISABLE: c_uint = 0x8C;
pub const ALI_7101_WDT: c_uint = 0x92;
pub const ALI_7101_GPIO: c_uint = 0x7D;
pub const ALI_7101_GPIO_O: c_uint = 0x7E;
pub const ALI_WDT_ARM: c_uint = 0x01;
//
// We're going to use a 1 second timeout.
// If we reset the watchdog every ~250ms we should be safe.

//
// We must not require too good response from the userspace daemon.
// Here we require the userspace daemon to send us a heartbeat
// char to /dev/watchdog every 30 seconds.
//

// in seconds, will be multiplied by HZ to get seconds to wait for a ping
    let mut timeout: static int = WATCHDOG_TIMEOUT;
    module_param(timeout, int, 0);
    MODULE_PARM_DESC(timeout,
    "Watchdog timeout in seconds. (1<=timeout<=3600, default="
    __MODULE_STRING(WATCHDOG_TIMEOUT) ")");
    static int use_gpio; /* Use the pic (for a1d revision alim7101) */
    module_param(use_gpio, int, 0);
    MODULE_PARM_DESC(use_gpio,
    "Use the gpio watchdog (required by old cobalt boards).");
    static void wdt_timer_ping(struct timer_list *);
    static DEFINE_TIMER(timer, wdt_timer_ping);
    static unsigned long next_heartbeat;
    static unsigned long wdt_is_open;
    static char wdt_expect_close;
    static struct pci_dev *alim7101_pmu;
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
//
// Whack the dog
//
#[no_mangle]
unsafe extern "C" fn wdt_timer_ping(unused: *mut timer_list) {
    static void wdt_timer_ping(struct timer_list *unused)
    {
// If we got a heartbeat pulse within the WDT_US_INTERVAL
// we agree to ping the WDT
//
    char tmp;
    if (time_before(jiffies, next_heartbeat)) {
// Ping the WDT (this is actually a disarm/arm sequence)
    pci_read_config_byte(alim7101_pmu, 0x92, &tmp);
    pci_write_config_byte(alim7101_pmu,
    ALI_7101_WDT, (tmp & ~ALI_WDT_ARM));
    pci_write_config_byte(alim7101_pmu,
    ALI_7101_WDT, (tmp | ALI_WDT_ARM));
    if (use_gpio) {
    pci_read_config_byte(alim7101_pmu,
    ALI_7101_GPIO_O, &tmp);
    pci_write_config_byte(alim7101_pmu,
    ALI_7101_GPIO_O, tmp | 0x20);
    pci_write_config_byte(alim7101_pmu,
    ALI_7101_GPIO_O, tmp & ~0x20);
    }
    } else {
    pr_warn("Heartbeat lost! Will not ping the watchdog\n");
    }
// Re-set the timer interval
    mod_timer(&timer, jiffies + WDT_INTERVAL);
    }
//
// Utility routines
//
#[no_mangle]
unsafe extern "C" fn wdt_change(writeval: c_int) {
    static void wdt_change(int writeval)
    {
    char tmp;
    pci_read_config_byte(alim7101_pmu, ALI_7101_WDT, &tmp);
    if (writeval == WDT_ENABLE) {
    pci_write_config_byte(alim7101_pmu,
    ALI_7101_WDT, (tmp | ALI_WDT_ARM));
    if (use_gpio) {
    pci_read_config_byte(alim7101_pmu,
    ALI_7101_GPIO_O, &tmp);
    pci_write_config_byte(alim7101_pmu,
    ALI_7101_GPIO_O, tmp & ~0x20);
    }
    } else {
    pci_write_config_byte(alim7101_pmu,
    ALI_7101_WDT, (tmp & ~ALI_WDT_ARM));
    if (use_gpio) {
    pci_read_config_byte(alim7101_pmu,
    ALI_7101_GPIO_O, &tmp);
    pci_write_config_byte(alim7101_pmu,
    ALI_7101_GPIO_O, tmp | 0x20);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn wdt_startup() {
    static void wdt_startup(void)
    {
    next_heartbeat = jiffies + (timeout * HZ);
// We must enable before we kick off the timer in case the timer
    occurs as we ping it */
    wdt_change(WDT_ENABLE);
// Start the timer
    mod_timer(&timer, jiffies + WDT_INTERVAL);
    pr_info("Watchdog timer is now enabled\n");
    }
#[no_mangle]
unsafe extern "C" fn wdt_turnoff() {
    static void wdt_turnoff(void)
    {
// Stop the timer
    timer_delete_sync(&timer);
    wdt_change(WDT_DISABLE);
    pr_info("Watchdog timer is now disabled...\n");
    }
#[no_mangle]
unsafe extern "C" fn wdt_keepalive() {
    static void wdt_keepalive(void)
    {
// user land ping
    next_heartbeat = jiffies + (timeout * HZ);
    }
//
// /dev/watchdog handling
//
    static ssize_t fop_write(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
// See if we got the magic character 'V' and reload the timer
    if (count) {
    if (!nowayout) {
    size_t ofs;
// note: just in case someone wrote the magic character
// five months ago...
    wdt_expect_close = 0;
// now scan
    for (ofs = 0; ofs != count; ofs++) {
    char c;
    if (get_user(c, buf + ofs))
    return -EFAULT;
    if (c == 'V')
    wdt_expect_close = 42;
    }
    }
// someone wrote to us, we should restart timer
    wdt_keepalive();
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn fop_open(inode: *mut inode, file: *mut file) -> c_int {
    static int fop_open(struct inode *inode, struct file *file)
    {
// Just in case we're already talking to someone...
    if (test_and_set_bit(0, &wdt_is_open))
    return -EBUSY;
// Good, fire up the show
    wdt_startup();
    return stream_open(inode, file);
    }
#[no_mangle]
unsafe extern "C" fn fop_close(inode: *mut inode, file: *mut file) -> c_int {
    static int fop_close(struct inode *inode, struct file *file)
    {
    if (wdt_expect_close == 42)
    wdt_turnoff();
    else {
// wim: shouldn't there be a: timer_delete(&timer);
    pr_crit("device file closed unexpectedly. Will not stop the WDT!\n");
    }
    clear_bit(0, &wdt_is_open);
    wdt_expect_close = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fop_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long fop_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    void __user *argp = (void __user *)arg;
    int __user *p = argp;
    static const struct watchdog_info ident = {
    .options = WDIOF_KEEPALIVEPING | WDIOF_SETTIMEOUT
    | WDIOF_MAGICCLOSE,
    .firmware_version = 1,
    .identity = "ALiM7101",
    };
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    return copy_to_user(argp, &ident, sizeof(ident)) ? -EFAULT : 0;
    case WDIOC_GETSTATUS:
    case WDIOC_GETBOOTSTATUS:
    return put_user(0, p);
    case WDIOC_SETOPTIONS:
    {
    int new_options, retval = -EINVAL;
    if (get_user(new_options, p))
    return -EFAULT;
    if (new_options & WDIOS_DISABLECARD) {
    wdt_turnoff();
    retval = 0;
    }
    if (new_options & WDIOS_ENABLECARD) {
    wdt_startup();
    retval = 0;
    }
    return retval;
    }
    case WDIOC_KEEPALIVE:
    wdt_keepalive();
    return 0;
    case WDIOC_SETTIMEOUT:
    {
    int new_timeout;
    if (get_user(new_timeout, p))
    return -EFAULT;
// arbitrary upper limit
    if (new_timeout < 1 || new_timeout > 3600)
    return -EINVAL;
    timeout = new_timeout;
    wdt_keepalive();
    }
    fallthrough;
    case WDIOC_GETTIMEOUT:
    return put_user(timeout, p);
    default:
    return -ENOTTY;
    }
    }
    static const struct file_operations wdt_fops = {
    .owner		=	THIS_MODULE,
    .write		=	fop_write,
    .open		=	fop_open,
    .release	=	fop_close,
    .unlocked_ioctl	=	fop_ioctl,
    .compat_ioctl	= 	compat_ptr_ioctl,
    };
    static struct miscdevice wdt_miscdev = {
    .minor	=	WATCHDOG_MINOR,
    .name	=	"watchdog",
    .fops	=	&wdt_fops,
    };
    static int wdt_restart_handle(struct notifier_block *this, unsigned long mode,
    void *cmd)
    {
//
// Cobalt devices have no way of rebooting themselves other
// than getting the watchdog to pull reset, so we restart the
// watchdog on reboot with no heartbeat.
//
    wdt_change(WDT_ENABLE);
// loop until the watchdog fires
    while (true)
    ;
    return NOTIFY_DONE;
    }
    static struct notifier_block wdt_restart_handler = {
    .notifier_call = wdt_restart_handle,
    .priority = 128,
    };
//
// Notifier for system down
//
    static int wdt_notify_sys(struct notifier_block *this,
    unsigned long code, void *unused)
    {
    if (code == SYS_DOWN || code == SYS_HALT)
    wdt_turnoff();
    return NOTIFY_DONE;
    }
//
// The WDT needs to learn about soft shutdowns in order to
// turn the timebomb registers off.
//
    static struct notifier_block wdt_notifier = {
    .notifier_call = wdt_notify_sys,
    };
#[no_mangle]
unsafe extern "C" fn alim7101_wdt_unload() -> void __exit {
    static void __exit alim7101_wdt_unload(void)
    {
    wdt_turnoff();
// Deregister
    misc_deregister(&wdt_miscdev);
    unregister_reboot_notifier(&wdt_notifier);
    unregister_restart_handler(&wdt_restart_handler);
    pci_dev_put(alim7101_pmu);
    }
#[no_mangle]
unsafe extern "C" fn alim7101_wdt_init() -> int __init {
    static int __init alim7101_wdt_init(void)
    {
    let mut rc: c_int = -EBUSY;
    struct pci_dev *ali1543_south;
    char tmp;
    pr_info("Steve Hill <steve@navaho.co.uk>\n");
    alim7101_pmu = pci_get_device(PCI_VENDOR_ID_AL, PCI_DEVICE_ID_AL_M7101,
    core::ptr::null_mut());
    if (!alim7101_pmu) {
    pr_info("ALi M7101 PMU not present - WDT not set\n");
    return -EBUSY;
    }
// Set the WDT in the PMU to 1 second
    pci_write_config_byte(alim7101_pmu, ALI_7101_WDT, 0x02);
    ali1543_south = pci_get_device(PCI_VENDOR_ID_AL, PCI_DEVICE_ID_AL_M1533,
    core::ptr::null_mut());
    if (!ali1543_south) {
    pr_info("ALi 1543 South-Bridge not present - WDT not set\n");
    goto err_out;
    }
    pci_read_config_byte(ali1543_south, 0x5e, &tmp);
    pci_dev_put(ali1543_south);
    if ((tmp & 0x1e) == 0x00) {
    if (!use_gpio) {
    pr_info("Detected old alim7101 revision 'a1d'.  If this is a cobalt board, set the 'use_gpio' module parameter.\n");
    goto err_out;
    }
    nowayout = 1;
    } else if ((tmp & 0x1e) != 0x12 && (tmp & 0x1e) != 0x00) {
    pr_info("ALi 1543 South-Bridge does not have the correct revision number (???1001?) - WDT not set\n");
    goto err_out;
    }
    if (timeout < 1 || timeout > 3600) {
// arbitrary upper limit
    timeout = WATCHDOG_TIMEOUT;
    pr_info("timeout value must be 1 <= x <= 3600, using %d\n",
    timeout);
    }
    rc = register_reboot_notifier(&wdt_notifier);
    if (rc) {
    pr_err("cannot register reboot notifier (err=%d)\n", rc);
    goto err_out;
    }
    rc = register_restart_handler(&wdt_restart_handler);
    if (rc) {
    pr_err("cannot register restart handler (err=%d)\n", rc);
    goto err_out_reboot;
    }
    rc = misc_register(&wdt_miscdev);
    if (rc) {
    pr_err("cannot register miscdev on minor=%d (err=%d)\n",
    wdt_miscdev.minor, rc);
    goto err_out_restart;
    }
    if (nowayout)
    __module_get(THIS_MODULE);
    pr_info("WDT driver for ALi M7101 initialised. timeout=%d sec (nowayout=%d)\n",
    timeout, nowayout);
    return 0;
    err_out_restart:
    unregister_restart_handler(&wdt_restart_handler);
    err_out_reboot:
    unregister_reboot_notifier(&wdt_notifier);
    err_out:
    pci_dev_put(alim7101_pmu);
    return rc;
    }
    module_init(alim7101_wdt_init);
    module_exit(alim7101_wdt_unload);
    static const struct pci_device_id alim7101_pci_tbl[] __used = {
    { PCI_DEVICE(PCI_VENDOR_ID_AL, PCI_DEVICE_ID_AL_M1533) },
    { PCI_DEVICE(PCI_VENDOR_ID_AL, PCI_DEVICE_ID_AL_M7101) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, alim7101_pci_tbl);
    MODULE_AUTHOR("Steve Hill");
    MODULE_DESCRIPTION("ALi M7101 PMU Computer Watchdog Timer driver");
    MODULE_LICENSE("GPL");
