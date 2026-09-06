//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/riowd.c
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
// riowd.c - driver for hw watchdog inside Super I/O of RIO
//
// Copyright (C) 2001, 2008 David S. Miller (davem@davemloft.net)
//

// RIO uses the NatSemi Super I/O power management logical device
// as its' watchdog.
//
// When the watchdog triggers, it asserts a line to the BBC (Boot Bus
// Controller) of the machine.  The BBC can only be configured to
// trigger a power-on reset when the signal is asserted.  The BBC
// can be configured to ignore the signal entirely as well.
//
// The only Super I/O device register we care about is at index
// 0x05 (WDTO_INDEX) which is the watchdog time-out in minutes (1-255).
// If set to zero, this disables the watchdog.  When set, the system
// must periodically (before watchdog expires) clear (set to zero) and
// re-set the watchdog else it will trigger.
//
// There are two other indexed watchdog registers inside this Super I/O
// logical device, but they are unused.  The first, at index 0x06 is
// the watchdog control and can be used to make the watchdog timer re-set
// when the PS/2 mouse or serial lines show activity.  The second, at
// index 0x07 is merely a sampling of the line from the watchdog to the
// BBC.
//
// The watchdog device generates no interrupts.
//
    MODULE_AUTHOR("David S. Miller <davem@davemloft.net>");
    MODULE_DESCRIPTION("Hardware watchdog driver for Sun RIO");
    MODULE_LICENSE("GPL");

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riowd {
    pub regs: *mut void __iomem,
    pub lock: spinlock_t,
}

    static struct riowd *riowd_device;
pub const WDTO_INDEX: c_uint = 0x05;
    static int riowd_timeout = 1;		/* in minutes */
    module_param(riowd_timeout, int, 0);
    MODULE_PARM_DESC(riowd_timeout, "Watchdog timeout in minutes");
#[no_mangle]
unsafe extern "C" fn riowd_writereg(p: *mut riowd, val: u8, index: c_int) {
    static void riowd_writereg(struct riowd *p, u8 val, int index)
    {
    unsigned long flags;
    spin_lock_irqsave(&p.lock, flags);
    writeb(index, p.regs + 0);
    writeb(val, p.regs + 1);
    spin_unlock_irqrestore(&p.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn riowd_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int riowd_open(struct inode *inode, struct file *filp)
    {
    stream_open(inode, filp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn riowd_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int riowd_release(struct inode *inode, struct file *filp)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn riowd_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long riowd_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
    {
    static const struct watchdog_info info = {
    .options		= WDIOF_SETTIMEOUT,
    .firmware_version	= 1,
    .identity		= DRIVER_NAME,
    };
    void __user *argp = (void __user *)arg;
    struct riowd *p = riowd_device;
    unsigned int options;
    int new_margin;
    switch (cmd) {
    case WDIOC_GETSUPPORT:
    if (copy_to_user(argp, &info, sizeof(info)))
    return -EFAULT;
    break;
    case WDIOC_GETSTATUS:
    case WDIOC_GETBOOTSTATUS:
    if (put_user(0, (int __user *)argp))
    return -EFAULT;
    break;
    case WDIOC_KEEPALIVE:
    riowd_writereg(p, riowd_timeout, WDTO_INDEX);
    break;
    case WDIOC_SETOPTIONS:
    if (copy_from_user(&options, argp, sizeof(options)))
    return -EFAULT;
    if (options & WDIOS_DISABLECARD)
    riowd_writereg(p, 0, WDTO_INDEX);
#[no_mangle]
pub unsafe extern "C" fn if(WDIOS_ENABLECARD: options &) -> else {
    else if (options & WDIOS_ENABLECARD)
    riowd_writereg(p, riowd_timeout, WDTO_INDEX);
    else
    return -EINVAL;
    break;
    case WDIOC_SETTIMEOUT:
    if (get_user(new_margin, (int __user *)argp))
    return -EFAULT;
    if ((new_margin < 60) || (new_margin > (255 * 60)))
    return -EINVAL;
    riowd_timeout = (new_margin + 59) / 60;
    riowd_writereg(p, riowd_timeout, WDTO_INDEX);
    fallthrough;
    case WDIOC_GETTIMEOUT:
    return put_user(riowd_timeout * 60, (int __user *)argp);
    default:
    return -EINVAL;
    }
    return 0;
    }
    static ssize_t riowd_write(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    struct riowd *p = riowd_device;
    if (count) {
    riowd_writereg(p, riowd_timeout, WDTO_INDEX);
    return 1;
    }
    return 0;
    }
    static const struct file_operations riowd_fops = {
    .owner =		THIS_MODULE,
    .unlocked_ioctl =	riowd_ioctl,
    .compat_ioctl	=	compat_ptr_ioctl,
    .open =			riowd_open,
    .write =		riowd_write,
    .release =		riowd_release,
    };
    static struct miscdevice riowd_miscdev = {
    .minor	= WATCHDOG_MINOR,
    .name	= "watchdog",
    .fops	= &riowd_fops
    };
#[no_mangle]
unsafe extern "C" fn riowd_probe(op: *mut platform_device) -> c_int {
    static int riowd_probe(struct platform_device *op)
    {
    struct riowd *p;
    let mut err: c_int = -EINVAL;
    if (riowd_device)
    goto out;
    err = -ENOMEM;
    p = devm_kzalloc(&op.dev, sizeof(*p), GFP_KERNEL);
    if (!p)
    goto out;
    spin_lock_init(&p.lock);
    p.regs = of_ioremap(&op.resource[0], 0, 2, DRIVER_NAME);
    if (!p.regs) {
    pr_err("Cannot map registers\n");
    goto out;
    }
// Make miscdev useable right away
    riowd_device = p;
    err = misc_register(&riowd_miscdev);
    if (err) {
    pr_err("Cannot register watchdog misc device\n");
    goto out_iounmap;
    }
    pr_info("Hardware watchdog [%i minutes], regs at %p\n",
    riowd_timeout, p.regs);
    platform_set_drvdata(op, p);
    return 0;
    out_iounmap:
    riowd_device = core::ptr::null_mut();
    of_iounmap(&op.resource[0], p.regs, 2);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn riowd_remove(op: *mut platform_device) {
    static void riowd_remove(struct platform_device *op)
    {
    struct riowd *p = platform_get_drvdata(op);
    misc_deregister(&riowd_miscdev);
    of_iounmap(&op.resource[0], p.regs, 2);
    }
    static const struct of_device_id riowd_match[] = {
    {
    .name = "pmc",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, riowd_match);
    static struct platform_driver riowd_driver = {
    .driver = {
    .name = DRIVER_NAME,
    .of_match_table = riowd_match,
    },
    .probe		= riowd_probe,
    .remove		= riowd_remove,
    };
    module_platform_driver(riowd_driver);
