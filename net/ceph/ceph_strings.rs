//! Automatically rewritten from C to Rust
//! Source: net/ceph/ceph_strings.c
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
// Ceph string constants
//

    const char *ceph_entity_type_name(int type)
    {
    switch (type) {
    case CEPH_ENTITY_TYPE_MDS: return "mds";
    case CEPH_ENTITY_TYPE_OSD: return "osd";
    case CEPH_ENTITY_TYPE_MON: return "mon";
    case CEPH_ENTITY_TYPE_CLIENT: return "client";
    case CEPH_ENTITY_TYPE_AUTH: return "auth";
    default: return "unknown";
    }
    }
    EXPORT_SYMBOL(ceph_entity_type_name);
    const char *ceph_auth_proto_name(int proto)
    {
    switch (proto) {
    case CEPH_AUTH_UNKNOWN:
    return "unknown";
    case CEPH_AUTH_NONE:
    return "none";
    case CEPH_AUTH_CEPHX:
    return "cephx";
    default:
    return "???";
    }
    }
    const char *ceph_con_mode_name(int mode)
    {
    switch (mode) {
    case CEPH_CON_MODE_UNKNOWN:
    return "unknown";
    case CEPH_CON_MODE_CRC:
    return "crc";
    case CEPH_CON_MODE_SECURE:
    return "secure";
    default:
    return "???";
    }
    }
    const char *ceph_osd_op_name(int op)
    {
    switch (op) {

    __CEPH_FORALL_OSD_OPS(GENERATE_CASE)

    default:
    return "???";
    }
    }
    const char *ceph_osd_watch_op_name(int o)
    {
    switch (o) {
    case CEPH_OSD_WATCH_OP_UNWATCH:
    return "unwatch";
    case CEPH_OSD_WATCH_OP_WATCH:
    return "watch";
    case CEPH_OSD_WATCH_OP_RECONNECT:
    return "reconnect";
    case CEPH_OSD_WATCH_OP_PING:
    return "ping";
    default:
    return "???";
    }
    }
    const char *ceph_osd_state_name(int s)
    {
    switch (s) {
    case CEPH_OSD_EXISTS:
    return "exists";
    case CEPH_OSD_UP:
    return "up";
    case CEPH_OSD_AUTOOUT:
    return "autoout";
    case CEPH_OSD_NEW:
    return "new";
    default:
    return "???";
    }
    }
