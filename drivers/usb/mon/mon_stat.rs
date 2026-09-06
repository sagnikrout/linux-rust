//! Automatically rewritten from C to Rust
//! Source: drivers/usb/mon/mon_stat.c
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


// SPDX-License-Identifier: GPL-2.0
//
// The USB Monitor, inspired by Dave Harding's USBMon.
//
// This is the 's' or 'stat' reader which debugs usbmon itself.
// Note that this code blows through locks, so make sure that
// /dbg/usbmon/0s is well protected from non-root users.
//

pub const STAT_BUF_SIZE: c_int = 80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snap {
    pub slen: c_int,
    pub str: [c_char; STAT_BUF_SIZE],
}

#[no_mangle]
unsafe extern "C" fn mon_stat_open(inode: *mut inode, file: *mut file) -> c_int {
    static int mon_stat_open(struct inode *inode, struct file *file)
    {
    struct mon_bus *mbus;
    struct snap *sp;
    sp = kmalloc_obj(struct snap);
    if (sp == core::ptr::null_mut())
    return -ENOMEM;
    mbus = inode.i_private;
    sp.slen = scnprintf(sp.str, STAT_BUF_SIZE,
    "nreaders %d events %u text_lost %u\n",
    mbus.nreaders, mbus.cnt_events, mbus.cnt_text_lost);
    file.private_data = sp;
    return 0;
    }
    static ssize_t mon_stat_read(struct file *file, char __user *buf,
    size_t nbytes, loff_t *ppos)
    {
    struct snap *sp = file.private_data;
    return simple_read_from_buffer(buf, nbytes, ppos, sp.str, sp.slen);
    }
#[no_mangle]
unsafe extern "C" fn mon_stat_release(inode: *mut inode, file: *mut file) -> c_int {
    static int mon_stat_release(struct inode *inode, struct file *file)
    {
    struct snap *sp = file.private_data;
    file.private_data = core::ptr::null_mut();
    kfree(sp);
    return 0;
    }
    const struct file_operations mon_fops_stat = {
    .owner =	THIS_MODULE,
    .open =		mon_stat_open,
    .read =		mon_stat_read,
// .write =	mon_stat_write,
// .poll =		mon_stat_poll,
// .unlocked_ioctl =	mon_stat_ioctl,
    .release =	mon_stat_release,
    };
