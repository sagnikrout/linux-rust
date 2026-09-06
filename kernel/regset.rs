//! Automatically rewritten from C to Rust
//! Source: kernel/regset.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }



// SPDX-License-Identifier: GPL-2.0-only

    static int __regset_get(struct task_struct *target,
    const struct user_regset *regset,
    unsigned int size,
    void **data)
    {
    void *p = *data, *to_free = core::ptr::null_mut();
    int res;
    if (!regset.regset_get)
    return -EOPNOTSUPP;
    if (size > regset.n * regset.size)
    size = regset.n * regset.size;
    if (!p) {
    to_free = p = kvzalloc(size, GFP_KERNEL);
    if (!p)
    return -ENOMEM;
    }
    res = regset.regset_get(target, regset,
    (struct membuf){.p = p, .left = size});
    if (res < 0) {
    kvfree(to_free);
    return res;
    }
// data = p;
    return size - res;
    }
    int regset_get(struct task_struct *target,
    const struct user_regset *regset,
    unsigned int size,
    void *data)
    {
    return __regset_get(target, regset, size, &data);
    }
    EXPORT_SYMBOL(regset_get);
    int regset_get_alloc(struct task_struct *target,
    const struct user_regset *regset,
    unsigned int size,
    void **data)
    {
// data = NULL;
    return __regset_get(target, regset, size, data);
    }
    EXPORT_SYMBOL(regset_get_alloc);
//
// copy_regset_to_user - fetch a thread's user_regset data into user memory
// @target:	thread to be examined
// @view:	&struct user_regset_view describing user thread machine state
// @setno:	index in @view->regsets
// @offset:	offset into the regset data, in bytes
// @size:	amount of data to copy, in bytes
// @data:	user-mode pointer to copy into
//
    int copy_regset_to_user(struct task_struct *target,
    const struct user_regset_view *view,
    unsigned int setno,
    unsigned int offset, unsigned int size,
    void __user *data)
    {
    const struct user_regset *regset = &view.regsets[setno];
    void *buf;
    int ret;
    ret = regset_get_alloc(target, regset, size, &buf);
    if (ret > 0)
    ret = copy_to_user(data, buf, ret) ? -EFAULT : 0;
    kvfree(buf);
    return ret;
    }
