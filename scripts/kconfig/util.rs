//! Automatically rewritten from C to Rust
//! Source: scripts/kconfig/util.c
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
// Copyright (C) 2002-2005 Roman Zippel <zippel@linux-m68k.org>
// Copyright (C) 2002-2005 Sam Ravnborg <sam@ravnborg.org>
//

// hash table of all parsed Kconfig files
    static HASHTABLE_DEFINE(file_hashtable, 1U << 11);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file {
    pub node: hlist_node,
    struct {
    pub name: *const c_char,
    pub lineno: c_int,
    pub parent: },
    pub name: [c_char; ],
}

    static void die_duplicated_include(struct file *file,
    const char *parent, int lineno)
    {
    fprintf(stderr,
    "%s:%d: error: repeated inclusion of %s\n"
    "%s:%d: note: location of first inclusion of %s\n",
    parent, lineno, file.name,
    file.parent.name, file.parent.lineno, file.name);
    exit(1);
    }
// file already present in list? If not add it
    const char *file_lookup(const char *name,
    const char *parent_name, int parent_lineno)
    {
    const char *parent = core::ptr::null_mut();
    struct file *file;
    size_t len;
    let mut hash: c_int = hash_str(name);
    if (parent_name)
    parent = file_lookup(parent_name, core::ptr::null_mut(), 0);
    hash_for_each_possible(file_hashtable, file, node, hash)
    if (!strcmp(name, file.name)) {
    if (!parent_name)
    return file.name;
    die_duplicated_include(file, parent, parent_lineno);
    }
    len = strlen(name);
    file = xmalloc(sizeof(*file) + len + 1);
    memset(file, 0, sizeof(*file));
    memcpy(file.name, name, len);
    file.name[len] = '\0';
    file.parent.name = parent;
    file.parent.lineno = parent_lineno;
    hash_add(file_hashtable, &file.node, hash);
    str_printf(&autoconf_cmd, "\t%s \\\n", name);
    return file.name;
    }
// Allocate initial growable string
#[no_mangle]
pub unsafe extern "C" fn str_new() -> gstr {
    struct gstr str_new(void)
    {
    struct gstr gs;
    gs.s = xmalloc(sizeof(char) * 64);
    gs.len = 64;
    gs.max_width = 0;
    strcpy(gs.s, "\0");
    return gs;
    }
// Free storage for growable string
#[no_mangle]
pub unsafe extern "C" fn str_free(gs: *mut gstr) {
    void str_free(struct gstr *gs)
    {
    free(gs.s);
    gs.s = core::ptr::null_mut();
    gs.len = 0;
    }
// Append to growable string
#[no_mangle]
pub unsafe extern "C" fn str_append(gs: *mut gstr, s: *const c_char) {
    void str_append(struct gstr *gs, const char *s)
    {
    size_t l;
    if (s) {
    l = strlen(gs.s) + strlen(s) + 1;
    if (l > gs.len) {
    gs.s = xrealloc(gs.s, l);
    gs.len = l;
    }
    strcat(gs.s, s);
    }
    }
// Append printf formatted string to growable string
#[no_mangle]
pub unsafe extern "C" fn str_printf(gs: *mut gstr, fmt: *const c_char, ...) {
    void str_printf(struct gstr *gs, const char *fmt, ...)
    {
    va_list ap;
    char s[10000]; /* big enough... */
    va_start(ap, fmt);
    vsnprintf(s, sizeof(s), fmt, ap);
    str_append(gs, s);
    va_end(ap);
    }
// Retrieve value of growable string
    char *str_get(const struct gstr *gs)
    {
    return gs.s;
    }
