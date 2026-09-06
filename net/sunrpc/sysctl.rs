//! Automatically rewritten from C to Rust
//! Source: net/sunrpc/sysctl.c
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
//
// linux/net/sunrpc/sysctl.c
//
// Sysctl interface to sunrpc module.
//
// I would prefer to register the sunrpc table below sys/net, but that's
// impossible at the moment.
//

//
// Declare the debug flags here
//
    unsigned int	rpc_debug;
    EXPORT_SYMBOL_GPL(rpc_debug);
    unsigned int	nfs_debug;
    EXPORT_SYMBOL_GPL(nfs_debug);
    unsigned int	nfsd_debug;
    EXPORT_SYMBOL_GPL(nfsd_debug);
    unsigned int	nlm_debug;
    EXPORT_SYMBOL_GPL(nlm_debug);

    static int proc_do_xprt(const struct ctl_table *table, int write,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    char tmpbuf[256];
    ssize_t len;
    if (write || *ppos) {
// lenp = 0;
    return 0;
    }
    len = svc_print_xprts(tmpbuf, sizeof(tmpbuf));
    len = memory_read_from_buffer(buffer, *lenp, ppos, tmpbuf, len);
    if (len < 0) {
// lenp = 0;
    return -EINVAL;
    }
// lenp = len;
    return 0;
    }
    static int
    proc_dodebug(const struct ctl_table *table, int write, void *buffer, size_t *lenp,
    loff_t *ppos)
    {
    char		tmpbuf[20], *s = core::ptr::null_mut();
    char *p;
    unsigned int	value;
    size_t		left, len;
    if ((*ppos && !write) || !*lenp) {
// lenp = 0;
    return 0;
    }
    left = *lenp;
    if (write) {
    p = buffer;
    while (left && isspace(*p)) {
    left--;
    p++;
    }
    if (!left)
    goto done;
    if (left > sizeof(tmpbuf) - 1)
    return -EINVAL;
    memcpy(tmpbuf, p, left);
    tmpbuf[left] = '\0';
    value = simple_strtol(tmpbuf, &s, 0);
    if (s) {
    left -= (s - tmpbuf);
    if (left && !isspace(*s))
    return -EINVAL;
    while (left && isspace(*s)) {
    left--;
    s++;
    }
    } else
    left = 0;
// (unsigned int *) table->data = value;
// Display the RPC tasks on writing to rpc_debug
    if (strcmp(table.procname, "rpc_debug") == 0)
    rpc_show_tasks(&init_net);
    } else {
    len = sprintf(tmpbuf, "0x%04x", *(unsigned int *) table.data);
    if (len > left)
    len = left;
    memcpy(buffer, tmpbuf, len);
    if ((left -= len) > 0) {
// ((char *)buffer + len) = '\n';
    left--;
    }
    }
    done:
// lenp -= left;
// ppos += *lenp;
    return 0;
    }
    static struct ctl_table_header *sunrpc_table_header;
    static struct ctl_table debug_table[] = {
    {
    .procname	= "rpc_debug",
    .data		= &rpc_debug,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dodebug
    },
    {
    .procname	= "nfs_debug",
    .data		= &nfs_debug,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dodebug
    },
    {
    .procname	= "nfsd_debug",
    .data		= &nfsd_debug,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dodebug
    },
    {
    .procname	= "nlm_debug",
    .data		= &nlm_debug,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dodebug
    },
    {
    .procname	= "transports",
    .maxlen		= 256,
    .mode		= 0444,
    .proc_handler	= proc_do_xprt,
    },
    };
    void
    rpc_register_sysctl(void)
    {
    if (!sunrpc_table_header)
    sunrpc_table_header = register_sysctl("sunrpc", debug_table);
    }
    void
    rpc_unregister_sysctl(void)
    {
    if (sunrpc_table_header) {
    unregister_sysctl_table(sunrpc_table_header);
    sunrpc_table_header = core::ptr::null_mut();
    }
    }
