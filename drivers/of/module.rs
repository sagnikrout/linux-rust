//! Automatically rewritten from C to Rust
//! Source: drivers/of/module.c
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
// Linux kernel module helpers.
//

#[no_mangle]
pub unsafe extern "C" fn of_modalias(np: *const device_node, str: *mut c_char, len: isize) -> isize {
    ssize_t of_modalias(const struct device_node *np, char *str, ssize_t len)
    {
    const char *compat;
    char *c;
    struct property *p;
    ssize_t csize;
    ssize_t tsize;
//
// Prevent a kernel oops in vsnprintf() -- it only allows passing a
// NULL ptr when the length is also 0. Also filter out the negative
// lengths...
//
    if ((len > 0 && !str) || len < 0)
    return -EINVAL;
// Name & Type
// %p eats all alphanum characters, so %c must be used here
    csize = snprintf(str, len, "of:N%pOFn%c%s", np, 'T',
    of_node_get_device_type(np));
    tsize = csize;
    if (csize >= len)
    csize = len > 0 ? len - 1 : 0;
    len -= csize;
    str += csize;
    of_property_for_each_string(np, "compatible", p, compat) {
    csize = snprintf(str, len, "C%s", compat);
    tsize += csize;
    if (csize >= len)
    continue;
    for (c = str; c; ) {
    c = strchr(c, ' ');
    if (c)
// c++ = '_';
    }
    len -= csize;
    str += csize;
    }
    return tsize;
    }
#[no_mangle]
pub unsafe extern "C" fn of_request_module(np: *const device_node) -> c_int {
    int of_request_module(const struct device_node *np)
    {
    char *str;
    ssize_t size;
    int ret;
    if (!np)
    return -ENODEV;
    size = of_modalias(np, core::ptr::null_mut(), 0);
    if (size < 0)
    return size;
// Reserve an additional byte for the trailing '\0'
    size++;
    str = kmalloc(size, GFP_KERNEL);
    if (!str)
    return -ENOMEM;
    of_modalias(np, str, size);
    str[size - 1] = '\0';
    ret = request_module(str);
    kfree(str);
    return ret;
    }
    EXPORT_SYMBOL_GPL(of_request_module);
