//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/stackglue.h
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
// stackglue.h
//
// Glue to the underlying cluster stack.
//
// Copyright (C) 2007 Oracle.  All rights reserved.
//

// Needed for plock-related prototypes
//
// dlmconstants.h does not have a LOCAL flag.  We hope to remove it
// some day, but right now we need it.  Let's fake it.  This value is larger
// than any flag in dlmconstants.h.
//
pub const DLM_LKF_LOCAL: c_uint = 0x00100000;
//
// This shadows DLM_LOCKSPACE_LEN in fs/dlm/dlm_internal.h.  That probably
// wants to be in a public header.
//
pub const GROUP_NAME_MAX: c_int = 64;
// This shadows  OCFS2_CLUSTER_NAME_LEN
pub const CLUSTER_NAME_MAX: c_int = 16;
//
// ocfs2_protocol_version changes when ocfs2 does something different in
// its inter-node behavior.  See dlmglue.c for more information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_protocol_version {
    pub pv_major: u8,
    pub pv_minor: u8,
}

//
// The dlm_lockstatus struct includes lvb space, but the dlm_lksb struct only
// has a pointer to separately allocated lvb space.  This struct exists only to
// include in the lksb union to make space for a combined dlm_lksb and lvb.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsdlm_lksb_plus_lvb {
    pub lksb: dlm_lksb,
    pub lvb: [c_char; DLM_LVB_LEN],
}

//
// A union of all lock status structures.  We define it here so that the
// size of the union is known.  Lock status structures are embedded in
// ocfs2 inodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_dlm_lksb {
    pub lksb_o2dlm: dlm_lockstatus,
    pub lksb_fsdlm: dlm_lksb,
    pub padding: fsdlm_lksb_plus_lvb,
}

//
// The ocfs2_locking_protocol defines the handlers called on ocfs2's behalf.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_locking_protocol {
    pub lp_max_version: ocfs2_protocol_version,
    pub lksb): *mut *mut void (lp_lock_ast)(struct ocfs2_dlm_lksb,
    pub level): *mut *mut *mut void (lp_blocking_ast)(struct ocfs2_dlm_lksb lksb, int,
    pub error): *mut *mut *mut void (lp_unlock_ast)(struct ocfs2_dlm_lksb lksb, int,
}

//
// A cluster connection.  Mostly opaque to ocfs2, the connection holds
// state for the underlying stack.  ocfs2 does use cc_version to determine
// locking compatibility.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_cluster_connection {
    pub 1]: char cc_name[GROUP_NAME_MAX +,
    pub cc_namelen: c_int,
    pub 1]: char cc_cluster_name[CLUSTER_NAME_MAX +,
    pub cc_cluster_name_len: c_int,
    pub cc_version: ocfs2_protocol_version,
    pub cc_proto: *mut ocfs2_locking_protocol,
    pub recovery_data): *mut *mut void (cc_recovery_handler)(int node_num, void,
    pub cc_recovery_data: *mut c_void,
    pub cc_lockspace: *mut c_void,
    pub cc_private: *mut c_void,
}

//
// Each cluster stack implements the stack operations structure.  Not used
// in the ocfs2 code, the stackglue code translates generic cluster calls
// into stack operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_stack_operations {
//
// The fs code calls ocfs2_cluster_connect() to attach a new
// filesystem to the cluster stack.  The ->connect() op is passed
// an ocfs2_cluster_connection with the name and recovery field
// filled in.
//
// The stack must set up any notification mechanisms and create
// the filesystem lockspace in the DLM.  The lockspace should be
// stored on cc_lockspace.  Any other information can be stored on
// cc_private.
//
// ->connect() must not return until it is guaranteed that
//
// - Node down notifications for the filesystem will be received
// and passed to conn->cc_recovery_handler().
// - Locking requests for the filesystem will be processed.
//
    pub conn): *mut *mut int (connect)(struct ocfs2_cluster_connection,
//
// The fs code calls ocfs2_cluster_disconnect() when a filesystem
// no longer needs cluster services.  All DLM locks have been
// dropped, and recovery notification is being ignored by the
// fs code.  The stack must disengage from the DLM and discontinue
// recovery notification.
//
// Once ->disconnect() has returned, the connection structure will
// be freed.  Thus, a stack must not return from ->disconnect()
// until it will no longer reference the conn pointer.
//
// Once this call returns, the stack glue will be dropping this
// connection's reference on the module.
//
    pub conn): *mut *mut int (disconnect)(struct ocfs2_cluster_connection,
//
// ->this_node() returns the cluster's unique identifier for the
// local node.
//
    pub node): *mut c_uint,
//
// Call the underlying dlm lock function.  The ->dlm_lock()
// callback should convert the flags and mode as appropriate.
//
// ast and bast functions are not part of the call because the
// stack will likely want to wrap ast and bast calls before passing
// them to stack->sp_proto.  There is no astarg.  The lksb will
// be passed back to the ast and bast functions.  The caller can
// use this to find their object.
//
    pub namelen): c_uint,
//
// Call the underlying dlm unlock function.  The ->dlm_unlock()
// function should convert the flags as appropriate.
//
// The unlock ast is not passed, as the stack will want to wrap
// it before calling stack->sp_proto->lp_unlock_ast().  There is
// no astarg.  The lksb will be passed back to the unlock ast
// function.  The caller can use this to find their object.
//
    pub flags): u32,
//
// Return the status of the current lock status block.  The fs
// code should never dereference the union.  The ->lock_status()
// callback pulls out the stack-specific lksb, converts the status
// to a proper errno, and returns it.
//
    pub lksb): *mut *mut int (lock_status)(struct ocfs2_dlm_lksb,
//
// Return non-zero if the LVB is valid.
//
    pub lksb): *mut *mut int (lvb_valid)(struct ocfs2_dlm_lksb,
//
// Pull the lvb pointer off of the stack-specific lksb.
//
    pub lksb): *mut *mut *mut void (lock_lvb)(struct ocfs2_dlm_lksb,
//
// Cluster-aware posix locks
//
// This is NULL for stacks which do not support posix locks.
//
    pub fl): *mut file_lock,
//
// This is an optional debugging hook.  If provided, the
// stack can dump debugging information about this lock.
//
    pub lksb): *mut *mut void (dump_lksb)(struct ocfs2_dlm_lksb,
}

//
// Each stack plugin must describe itself by registering a
// ocfs2_stack_plugin structure.  This is only seen by stackglue and the
// stack driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocfs2_stack_plugin {
    pub sp_name: *mut c_char,
    pub sp_ops: *const ocfs2_stack_operations,
    pub sp_owner: *mut module,
// These are managed by the stackglue code.
    pub sp_list: list_head,
    pub sp_count: c_uint,
    pub sp_max_proto: ocfs2_protocol_version,
}

// Used by the filesystem
//
// Used by callers that don't store their stack name.  They must ensure
// all nodes have the same stack.
//
extern "C" {
    pub fn ocfs2_cluster_hangup(group: *const c_char, grouplen: c_int);
}
extern "C" {
    pub fn ocfs2_dlm_lock_status(lksb: *mut ocfs2_dlm_lksb) -> c_int;
}
extern "C" {
    pub fn ocfs2_dlm_lvb_valid(lksb: *mut ocfs2_dlm_lksb) -> c_int;
}
extern "C" {
    pub fn ocfs2_dlm_dump_lksb(lksb: *mut ocfs2_dlm_lksb);
}
extern "C" {
    pub fn ocfs2_stack_supports_plocks() -> c_int;
}
extern "C" {
    pub fn ocfs2_stack_glue_set_max_proto_version(max_proto: *mut ocfs2_protocol_version);
}
// Used by stack plugins
extern "C" {
    pub fn ocfs2_stack_glue_register(plugin: *mut ocfs2_stack_plugin) -> c_int;
}
extern "C" {
    pub fn ocfs2_stack_glue_unregister(plugin: *mut ocfs2_stack_plugin);
}
