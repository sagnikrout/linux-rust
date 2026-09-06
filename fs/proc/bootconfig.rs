//! Automatically rewritten from C to Rust
//! Source: fs/proc/bootconfig.c
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
// /proc/bootconfig - Extra boot configuration
//

    static char *saved_boot_config;
#[no_mangle]
unsafe extern "C" fn boot_config_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int boot_config_proc_show(struct seq_file *m, void *v)
    {
    if (saved_boot_config)
    seq_puts(m, saved_boot_config);
    return 0;
    }
// Rest size of buffer

// Return the needed total length if @size is 0
#[no_mangle]
unsafe extern "C" fn copy_xbc_key_value_list(dst: *mut c_char, size: usize) -> int __init {
    static int __init copy_xbc_key_value_list(char *dst, size_t size)
    {
    struct xbc_node *leaf, *vnode;
    char *key, *end = dst + size;
    const char *val;
    char q;
    let mut ret: c_int = 0;
    key = kzalloc(XBC_KEYLEN_MAX, GFP_KERNEL);
    if (!key)
    return -ENOMEM;
    xbc_for_each_key_value(leaf, val) {
    ret = xbc_node_compose_key(leaf, key, XBC_KEYLEN_MAX);
    if (ret < 0)
    break;
    ret = snprintf(dst, rest(dst, end), "%s = ", key);
    if (ret < 0)
    break;
    dst += ret;
    vnode = xbc_node_get_child(leaf);
    if (vnode) {
    xbc_array_for_each_value(vnode, val) {
    if (strchr(val, '"'))
    q = '\'';
    else
    q = '"';
    ret = snprintf(dst, rest(dst, end), "%c%s%c%s",
    q, val, q, xbc_node_is_array(vnode) ? ", " : "\n");
    if (ret < 0)
    goto out;
    dst += ret;
    }
    } else {
    ret = snprintf(dst, rest(dst, end), "\"\"\n");
    if (ret < 0)
    break;
    dst += ret;
    }
    }
    if (cmdline_has_extra_options() && ret >= 0 && boot_command_line[0]) {
    ret = snprintf(dst, rest(dst, end), "# Parameters from bootloader:\n# %s\n",
    boot_command_line);
    if (ret > 0)
    dst += ret;
    }
    out:
    kfree(key);
    return ret < 0 ? ret : dst - (end - size);
    }
#[no_mangle]
unsafe extern "C" fn proc_boot_config_init() -> int __init {
    static int __init proc_boot_config_init(void)
    {
    int len;
    len = copy_xbc_key_value_list(core::ptr::null_mut(), 0);
    if (len < 0)
    return len;
    if (len > 0) {
    saved_boot_config = kzalloc(len + 1, GFP_KERNEL);
    if (!saved_boot_config)
    return -ENOMEM;
    len = copy_xbc_key_value_list(saved_boot_config, len + 1);
    if (len < 0) {
    kfree(saved_boot_config);
    return len;
    }
    }
    proc_create_single("bootconfig", 0, core::ptr::null_mut(), boot_config_proc_show);
    return 0;
    }
    fs_initcall(proc_boot_config_init);
