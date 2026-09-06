//! Automatically rewritten from C to Rust
//! Source: fs/smb/client/cifsroot.c
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
// SMB root file system support
//
// Copyright (c) 2019 Paulo Alcantara <palcantara@suse.de>
//

    "vers=1.0,cifsacl,mfsymlinks,rsize=1048576,wsize=65536,uid=0,gid=0," \
    "hard,rootfs"
    static char root_dev[2048] __initdata = "";
    static char root_opts[1024] __initdata = DEFAULT_MNT_OPTS;
#[no_mangle]
unsafe extern "C" fn parse_srvaddr(start: *mut c_char, end: *mut c_char) -> __be32 __init {
    static __be32 __init parse_srvaddr(char *start, char *end)
    {
// TODO: ipv6 support
    char addr[sizeof("aaa.bbb.ccc.ddd")];
    let mut i: c_int = 0;
    while (start < end && i < sizeof(addr) - 1) {
    if (isdigit(*start) || *start == '.')
    addr[i++] = *start;
    start++;
    }
    addr[i] = '\0';
    return in_aton(addr);
    }
// cifsroot=//<server-ip>/<share>[,options]
#[no_mangle]
unsafe extern "C" fn cifs_root_setup(line: *mut c_char) -> int __init {
    static int __init cifs_root_setup(char *line)
    {
    char *s;
    int len;
    let mut srvaddr: __be32 = htonl(INADDR_NONE);
    ROOT_DEV = Root_CIFS;
    if (strlen(line) > 3 && line[0] == '/' && line[1] == '/') {
    s = strchr(&line[2], '/');
    if (!s || s[1] == '\0')
    return 1;
// make s point to ',' or '\0' at end of line
    s = strchrnul(s, ',');
// len is strlen(unc) + '\0'
    len = s - line + 1;
    if (len > sizeof(root_dev)) {
    pr_err("Root-CIFS: UNC path too long\n");
    return 1;
    }
    strscpy(root_dev, line, len);
    srvaddr = parse_srvaddr(&line[2], s);
    if (*s) {
    int n = snprintf(root_opts,
    sizeof(root_opts), "%s,%s",
    DEFAULT_MNT_OPTS, s + 1);
    if (n >= sizeof(root_opts)) {
    pr_err("Root-CIFS: mount options string too long\n");
    root_opts[sizeof(root_opts)-1] = '\0';
    return 1;
    }
    }
    }
    root_server_addr = srvaddr;
    return 1;
    }
    __setup("cifsroot=", cifs_root_setup);
#[no_mangle]
pub unsafe extern "C" fn cifs_root_data(dev: *mut c_char, opts: *mut c_char) -> int __init {
    int __init cifs_root_data(char **dev, char **opts)
    {
    if (!root_dev[0] || root_server_addr == htonl(INADDR_NONE)) {
    pr_err("Root-CIFS: no SMB server address\n");
    return -1;
    }
// dev = root_dev;
// opts = root_opts;
    return 0;
    }
