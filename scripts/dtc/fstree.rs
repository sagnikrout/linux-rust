//! Automatically rewritten from C to Rust
//! Source: scripts/dtc/fstree.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005.
//

    static struct node *read_fstree(const char *dirname)
    {
    DIR *d;
    struct dirent *de;
    struct stat st;
    struct node *tree;
    d = opendir(dirname);
    if (!d)
    die("Couldn't opendir() \"%s\": %s\n", dirname, strerror(errno));
    tree = build_node(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    while ((de = readdir(d)) != core::ptr::null_mut()) {
    char *tmpname;
    if (streq(de.d_name, ".")
    || streq(de.d_name, ".."))
    continue;
    tmpname = join_path(dirname, de.d_name);
    if (stat(tmpname, &st) < 0)
    die("stat(%s): %s\n", tmpname, strerror(errno));
    if (S_ISREG(st.st_mode)) {
    struct property *prop;
    FILE *pfile;
    pfile = fopen(tmpname, "rb");
    if (! pfile) {
    fprintf(stderr,
    "WARNING: Cannot open %s: %s\n",
    tmpname, strerror(errno));
    } else {
    prop = build_property(de.d_name,
    data_copy_file(pfile,
    st.st_size),
    core::ptr::null_mut());
    add_property(tree, prop);
    fclose(pfile);
    }
    } else if (S_ISDIR(st.st_mode)) {
    struct node *newchild;
    newchild = read_fstree(tmpname);
    newchild = name_node(newchild, xstrdup(de.d_name));
    add_child(tree, newchild);
    }
    free(tmpname);
    }
    closedir(d);
    return tree;
    }
    struct dt_info *dt_from_fs(const char *dirname)
    {
    struct node *tree;
    tree = read_fstree(dirname);
    tree = name_node(tree, "");
    return build_dt_info(DTSF_V1, core::ptr::null_mut(), tree, guess_boot_cpuid(tree));
    }
