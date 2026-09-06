//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/random.c
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


// Copyright (C) 2005 - 2008 Jeff Dike <jdike@{linux.intel,addtoit}.com>
// Much of this ripped from drivers/char/hw_random.c, see there for other
// copyright.
//
// This software may be used and distributed according to the terms
// of the GNU General Public License, incorporated herein by reference.
//

//
// core module information
//

// Changed at init time, in the non-modular case, and at module load
// time, in the module case.  Presumably, the module subsystem
// protects against a module being loaded twice at the same time.
//
    let mut random_fd: static int = -1;
    static struct hwrng hwrng;
    static DECLARE_COMPLETION(have_data);
#[no_mangle]
unsafe extern "C" fn rng_dev_read(rng: *mut hwrng, buf: *mut c_void, max: usize, block: bool) -> c_int {
    static int rng_dev_read(struct hwrng *rng, void *buf, size_t max, bool block)
    {
    int ret;
    for (;;) {
    ret = os_read_file(random_fd, buf, max);
    if (block && ret == -EAGAIN) {
    add_sigio_fd(random_fd);
    ret = wait_for_completion_killable(&have_data);
    ignore_sigio_fd(random_fd);
    deactivate_fd(random_fd, RANDOM_IRQ);
    if (ret < 0)
    break;
    } else {
    break;
    }
    }
    return ret != -EAGAIN ? ret : 0;
    }
#[no_mangle]
unsafe extern "C" fn random_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t random_interrupt(int irq, void *data)
    {
    complete(&have_data);
    return IRQ_HANDLED;
    }
//
// rng_init - initialize RNG module
//
#[no_mangle]
unsafe extern "C" fn rng_init() -> int __init {
    static int __init rng_init (void)
    {
    int err;
    err = os_open_file("/dev/random", of_read(OPENFLAGS()), 0);
    if (err < 0)
    goto out;
    random_fd = err;
    err = um_request_irq(RANDOM_IRQ, random_fd, IRQ_READ, random_interrupt,
    0, "random", core::ptr::null_mut());
    if (err < 0)
    goto err_out_cleanup_hw;
    sigio_broken();
    hwrng.name = RNG_MODULE_NAME;
    hwrng.read = rng_dev_read;
    err = hwrng_register(&hwrng);
    if (err) {
    pr_err(RNG_MODULE_NAME " registering failed (%d)\n", err);
    goto err_out_cleanup_hw;
    }
    out:
    return err;
    err_out_cleanup_hw:
    os_close_file(random_fd);
    random_fd = -1;
    goto out;
    }
//
// rng_cleanup - shutdown RNG module
//
#[no_mangle]
unsafe extern "C" fn cleanup() {
    static void cleanup(void)
    {
    free_irq_by_fd(random_fd);
    os_close_file(random_fd);
    }
#[no_mangle]
unsafe extern "C" fn rng_cleanup() -> void __exit {
    static void __exit rng_cleanup(void)
    {
    hwrng_unregister(&hwrng);
    os_close_file(random_fd);
    }
    module_init (rng_init);
    module_exit (rng_cleanup);
    __uml_exitcall(cleanup);
    MODULE_DESCRIPTION("UML Host Random Number Generator (RNG) driver");
    MODULE_LICENSE("GPL");
