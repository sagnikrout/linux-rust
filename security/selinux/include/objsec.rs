//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/include/objsec.h
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
// Security-Enhanced Linux (SELinux) security module
//
// This file contains the SELinux security data structures for kernel objects.
//
// Author(s):  Stephen Smalley, <stephen.smalley.work@gmail.com>
// Chris Vance, <cvance@nai.com>
// Wayne Salamon, <wsalamon@nai.com>
// James Morris <jmorris@redhat.com>
//
// Copyright (C) 2001,2002 Networks Associates Technology, Inc.
// Copyright (C) 2003 Red Hat, Inc., James Morris <jmorris@redhat.com>
// Copyright (C) 2016 Mellanox Technologies
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avdc_entry {
    pub /: *mut *mut u32 isid; / inode SID,
    pub /: *mut *mut av_decision avd; / av decision,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred_security_struct {
    pub /: *mut *mut u32 osid; / SID prior to last execve,
    pub /: *mut *mut u32 sid; / current SID,
    pub /: *mut *mut u32 exec_sid; / exec SID,
    pub /: *mut *mut u32 create_sid; / fscreate SID,
    pub /: *mut *mut u32 keycreate_sid; / keycreate SID,
    pub /: *mut *mut u32 sockcreate_sid; / fscreate SID,
    pub __randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_security_struct {

    pub /: *mut *mut u32 sid; / current SID for cached entries,
    pub /: *mut *mut u32 seqno; / AVC sequence number,
    pub /: *mut *mut unsigned int dir_spot; / dir cache index to check first,
    pub /: *mut *mut avdc_entry dir[TSEC_AVDC_DIR_SIZE]; / dir entries,
    pub /: *mut *mut bool permissive_neveraudit; / permissive and neveraudit,
    pub avdcache: },
    pub __randomize_layout: },
    pub avc_policy_seqno()): tsec->avdcache.seqno ==,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum label_initialized {
    LABEL_INVALID, /* invalid or not initialized */
    LABEL_INITIALIZED, /* initialized */
    LABEL_PENDING
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode_security_struct {
    pub /: *mut *mut *mut inode inode; / back pointer to inode object,
    pub /: *mut *mut list_head list; / list of inode_security_struct,
    pub /: *mut *mut u32 task_sid; / SID of creating task,
    pub /: *mut *mut u32 sid; / SID of this object,
    pub /: *mut *mut u16 sclass; / security class of this object,
    pub /: *mut *mut unsigned char initialized; / initialization flag,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_security_struct {
    pub /: *mut *mut u32 sid; / SID of open file description,
    pub /: *mut *mut u32 fown_sid; / SID of file owner (for SIGIO),
    pub /: *mut *mut u32 isid; / SID of inode at the time of file open,
    pub /: *mut *mut u32 pseqno; / Policy seqno at the time of file open,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct backing_file_security_struct {
    pub /: *mut *mut u32 uf_sid; / associated user file fsec->sid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct superblock_security_struct {
    pub /: *mut *mut u32 sid; / SID of file system superblock,
    pub /: *mut *mut u32 def_sid; / default SID for labeling,
    pub /: *mut *mut u32 mntpoint_sid; / SECURITY_FS_USE_MNTPOINT context for files,
    pub /: *mut *mut u32 creator_sid; / SID of privileged process,
    pub /: *mut *mut unsigned short behavior; / labeling behavior,
    pub /: *mut *mut unsigned short flags; / which mount options were specified,
    pub lock: mutex,
    pub isec_head: list_head,
    pub isec_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_security_struct {
    pub /: *mut *mut u32 sid; / SID of message,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_security_struct {
    pub /: *mut *mut u16 sclass; / security class of this object,
    pub /: *mut *mut u32 sid; / SID of IPC resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netif_security_struct {
    pub /: *const *const *const net ns; / network namespace,
    pub /: *mut *mut int ifindex; / device index,
    pub /: *mut *mut u32 sid; / SID for this interface,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netnode_security_struct {
    pub /: *mut *mut __be32 ipv4; / IPv4 node address,
    pub /: *mut *mut in6_addr ipv6; / IPv6 node address,
    pub addr: },
    pub /: *mut *mut u32 sid; / SID for this node,
    pub /: *mut *mut u16 family; / address family,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netport_security_struct {
    pub /: *mut *mut u32 sid; / SID for this node,
    pub /: *mut *mut u16 port; / port number,
    pub /: *mut *mut u8 protocol; / transport protocol,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_security_struct {

    pub nlbl_state: },
    pub /: *mut *mut *mut netlbl_lsm_secattr nlbl_secattr; / NetLabel sec attributes,

    pub /: *mut *mut u32 sid; / SID of this object,
    pub /: *mut *mut u32 peer_sid; / SID of peer,
    pub /: *mut *mut u16 sclass; / sock security class,
    pub sctp_assoc_state: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tun_security_struct {
    pub /: *mut *mut u32 sid; / SID for the tun device sockets,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_security_struct {
    pub /: *mut *mut u32 sid; / SID of key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_security_struct {
    pub /: *mut *mut u32 sid; / SID of the queue pair or MAD agent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_security_struct {
    pub /: *mut *mut u64 subnet_prefix; / Port subnet prefix,
    pub /: *mut *mut u16 pkey; / PKey number,
    pub /: *mut *mut u32 sid; / SID of pkey,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_security_struct {
    pub /: *mut *mut u32 sid; / SID of bpf obj creator,
    pub /: *mut *mut u32 perms; / permissions for allowed bpf token commands,
    pub /: *mut *mut u32 grantor_sid; / SID of token grantor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_security_struct {
    pub /: *mut *mut u32 sid; / SID of perf_event obj creator,
}

//
// get the subjective security ID of the current task
//

