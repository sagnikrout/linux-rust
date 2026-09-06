//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lsm_audit.h
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
// Common LSM logging functions
// Heavily borrowed from selinux/avc.h
//
// Author : Etienne BASSET  <etienne.basset@ensta.org>
//
// All credits to : Stephen Smalley
// All BUGS to : Etienne BASSET  <etienne.basset@ensta.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_network_audit {
    pub netif: c_int,
    pub sk: *const sock,
    pub family: u16,
    pub dport: __be16,
    pub sport: __be16,
    pub daddr: __be32,
    pub saddr: __be32,
    pub v4: },
    pub daddr: in6_addr,
    pub saddr: in6_addr,
    pub v6: },
    pub fam: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_ioctlop_audit {
    pub path: path,
    pub cmd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_ibpkey_audit {
    pub subnet_prefix: u64,
    pub pkey: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsm_ibendport_audit {
    pub dev_name: *const c_char,
    pub port: u8,
}

// Auxiliary data to use in generating the audit record.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_audit_data {
    pub type: c_char,
pub const LSM_AUDIT_DATA_PATH: c_int = 1;
pub const LSM_AUDIT_DATA_NET: c_int = 2;
pub const LSM_AUDIT_DATA_CAP: c_int = 3;
pub const LSM_AUDIT_DATA_IPC: c_int = 4;
pub const LSM_AUDIT_DATA_TASK: c_int = 5;
pub const LSM_AUDIT_DATA_KEY: c_int = 6;
pub const LSM_AUDIT_DATA_NONE: c_int = 7;
pub const LSM_AUDIT_DATA_KMOD: c_int = 8;
pub const LSM_AUDIT_DATA_INODE: c_int = 9;
pub const LSM_AUDIT_DATA_DENTRY: c_int = 10;
pub const LSM_AUDIT_DATA_IOCTL_OP: c_int = 11;
pub const LSM_AUDIT_DATA_FILE: c_int = 12;
pub const LSM_AUDIT_DATA_IBPKEY: c_int = 13;
pub const LSM_AUDIT_DATA_IBENDPORT: c_int = 14;
pub const LSM_AUDIT_DATA_LOCKDOWN: c_int = 15;
pub const LSM_AUDIT_DATA_NOTIFICATION: c_int = 16;
pub const LSM_AUDIT_DATA_ANONINODE: c_int = 17;
pub const LSM_AUDIT_DATA_NLMSGTYPE: c_int = 18;
    pub path: path,
    pub dentry: *mut dentry,
    pub inode: *mut inode,
    pub net: *mut lsm_network_audit,
    pub cap: c_int,
    pub ipc_id: c_int,
    pub tsk: *mut task_struct,

    pub key: key_serial_t,
    pub key_desc: *mut c_char,
    pub key_struct: },

    pub kmod_name: *mut c_char,
    pub op: *mut lsm_ioctlop_audit,
    pub file: *const file,
    pub ibpkey: *mut lsm_ibpkey_audit,
    pub ibendport: *mut lsm_ibendport_audit,
    pub reason: c_int,
    pub anonclass: *const c_char,
    pub nlmsg_type: u16,
    pub u: },
// this union contains LSM specific data

    pub smack_audit_data: *mut smack_audit_data,

    pub selinux_audit_data: *mut selinux_audit_data,

    pub apparmor_audit_data: *mut apparmor_audit_data,

}

