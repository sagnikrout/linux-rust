//! Automatically rewritten from C to Rust
//! Source: drivers/hv/mshv_portid_table.c
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
// Ports and connections are hypervisor struct used for inter-partition
// communication. Port represents the source and connection represents
// the destination. Partitions are responsible for managing the port and
// connection ids.
//
pub const PORTID_MIN: c_int = 1;

    static DEFINE_IDR(port_table_idr);
    void
    mshv_port_table_fini(void)
    {
    struct port_table_info *port_info;
    unsigned long i, tmp;
    idr_lock(&port_table_idr);
    if (!idr_is_empty(&port_table_idr)) {
    idr_for_each_entry_ul(&port_table_idr, port_info, tmp, i) {
    port_info = idr_remove(&port_table_idr, i);
    kfree_rcu(port_info, portbl_rcu);
    }
    }
    idr_unlock(&port_table_idr);
    }
    int
    mshv_portid_alloc(struct port_table_info *info)
    {
    int ret;
    idr_preload(GFP_KERNEL);
    idr_lock(&port_table_idr);
    ret = idr_alloc(&port_table_idr, info, PORTID_MIN,
    PORTID_MAX, GFP_NOWAIT);
    idr_unlock(&port_table_idr);
    idr_preload_end();
    return ret;
    }
    void
    mshv_portid_free(int port_id)
    {
    struct port_table_info *info;
    idr_lock(&port_table_idr);
    info = idr_remove(&port_table_idr, port_id);
    WARN_ON(!info);
    idr_unlock(&port_table_idr);
    kfree_rcu(info, portbl_rcu);
    }
    int
    mshv_portid_lookup(int port_id, struct port_table_info *info)
    {
    struct port_table_info *_info;
    let mut ret: c_int = -ENOENT;
    rcu_read_lock();
    _info = idr_find(&port_table_idr, port_id);
    rcu_read_unlock();
    if (_info) {
// info = *_info;
    ret = 0;
    }
    return ret;
    }
