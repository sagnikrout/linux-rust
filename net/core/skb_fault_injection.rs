//! Automatically rewritten from C to Rust
//! Source: net/core/skb_fault_injection.c
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

    static struct {
    struct fault_attr attr;
    char devname[IFNAMSIZ];
    bool filtered;
    } skb_realloc = {
    .attr = FAULT_ATTR_INITIALIZER,
    .filtered = false,
    };
#[no_mangle]
unsafe extern "C" fn should_fail_net_realloc_skb(skb: *mut sk_buff) -> bool {
    static bool should_fail_net_realloc_skb(struct sk_buff *skb)
    {
    struct net_device *net = skb.dev;
    if (skb_realloc.filtered &&
    strncmp(net.name, skb_realloc.devname, IFNAMSIZ))
// device name filter set, but names do not match
    return false;
    if (!should_fail(&skb_realloc.attr, 1))
    return false;
    return true;
    }
    ALLOW_ERROR_INJECTION(should_fail_net_realloc_skb, TRUE);
#[no_mangle]
pub unsafe extern "C" fn skb_might_realloc(skb: *mut sk_buff) {
    void skb_might_realloc(struct sk_buff *skb)
    {
    if (!should_fail_net_realloc_skb(skb))
    return;
    pskb_expand_head(skb, 0, 0, GFP_ATOMIC);
    }
    EXPORT_SYMBOL(skb_might_realloc);
#[no_mangle]
unsafe extern "C" fn fail_skb_realloc_setup(str: *mut c_char) -> int __init {
    static int __init fail_skb_realloc_setup(char *str)
    {
    return setup_fault_attr(&skb_realloc.attr, str);
    }
    __setup("fail_skb_realloc=", fail_skb_realloc_setup);
#[no_mangle]
unsafe extern "C" fn reset_settings() {
    static void reset_settings(void)
    {
    skb_realloc.filtered = false;
    memset(&skb_realloc.devname, 0, IFNAMSIZ);
    }
    static ssize_t devname_write(struct file *file, const char __user *buffer,
    size_t count, loff_t *ppos)
    {
    ssize_t ret;
    reset_settings();
    ret = simple_write_to_buffer(&skb_realloc.devname, IFNAMSIZ,
    ppos, buffer, count);
    if (ret < 0)
    return ret;
    skb_realloc.devname[IFNAMSIZ - 1] = '\0';
// Remove a possible \n at the end of devname
    strim(skb_realloc.devname);
    if (strnlen(skb_realloc.devname, IFNAMSIZ))
    skb_realloc.filtered = true;
    return count;
    }
    static ssize_t devname_read(struct file *file,
    char __user *buffer,
    size_t size, loff_t *ppos)
    {
    if (!skb_realloc.filtered)
    return 0;
    return simple_read_from_buffer(buffer, size, ppos, &skb_realloc.devname,
    strlen(skb_realloc.devname));
    }
    static const struct file_operations devname_ops = {
    .write = devname_write,
    .read = devname_read,
    };
#[no_mangle]
unsafe extern "C" fn fail_skb_realloc_debugfs() -> int __init {
    static int __init fail_skb_realloc_debugfs(void)
    {
    let mut mode: umode_t = S_IFREG | 0600;
    struct dentry *dir;
    dir = fault_create_debugfs_attr("fail_skb_realloc", core::ptr::null_mut(),
    &skb_realloc.attr);
    if (IS_ERR(dir))
    return PTR_ERR(dir);
    debugfs_create_file("devname", mode, dir, core::ptr::null_mut(), &devname_ops);
    return 0;
    }
    late_initcall(fail_skb_realloc_debugfs);
