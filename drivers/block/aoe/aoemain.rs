//! Automatically rewritten from C to Rust
//! Source: drivers/block/aoe/aoemain.c
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


// Copyright (c) 2012 Coraid, Inc.  See COPYING for GPL terms.
//
// aoemain.c
// Module initialization routines, discover timer
//

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Sam Hopkins <sah@coraid.com>");
    MODULE_DESCRIPTION("AoE block/char driver for 2.6.2 and newer 2.6 kernels");
    MODULE_VERSION(VERSION);
    static struct timer_list timer;
    struct workqueue_struct *aoe_wq;
#[no_mangle]
unsafe extern "C" fn discover_timer(t: *mut timer_list) {
    static void discover_timer(struct timer_list *t)
    {
    mod_timer(t, jiffies + HZ * 60); /* one minute */
    aoecmd_cfg(0xffff, 0xff);
    }
    static void __exit
    aoe_exit(void)
    {
    timer_delete_sync(&timer);
    aoenet_exit();
    unregister_blkdev(AOE_MAJOR, DEVICE_NAME);
    aoecmd_exit();
    aoechr_exit();
    aoedev_exit();
    aoeblk_exit();		/* free cache after de-allocating bufs */
    destroy_workqueue(aoe_wq);
    }
    static int __init
    aoe_init(void)
    {
    int ret;
    aoe_wq = alloc_workqueue("aoe_wq", WQ_PERCPU, 0);
    if (!aoe_wq)
    return -ENOMEM;
    ret = aoedev_init();
    if (ret)
    goto dev_fail;
    ret = aoechr_init();
    if (ret)
    goto chr_fail;
    ret = aoeblk_init();
    if (ret)
    goto blk_fail;
    ret = aoenet_init();
    if (ret)
    goto net_fail;
    ret = aoecmd_init();
    if (ret)
    goto cmd_fail;
    ret = register_blkdev(AOE_MAJOR, DEVICE_NAME);
    if (ret < 0) {
    printk(KERN_ERR "aoe: can't register major\n");
    goto blkreg_fail;
    }
    printk(KERN_INFO "aoe: AoE v%s initialised.\n", VERSION);
    timer_setup(&timer, discover_timer, 0);
    discover_timer(&timer);
    return 0;
    blkreg_fail:
    aoecmd_exit();
    cmd_fail:
    aoenet_exit();
    net_fail:
    aoeblk_exit();
    blk_fail:
    aoechr_exit();
    chr_fail:
    aoedev_exit();
    dev_fail:
    destroy_workqueue(aoe_wq);
    printk(KERN_INFO "aoe: initialisation failure.\n");
    return ret;
    }
    module_init(aoe_init);
    module_exit(aoe_exit);
