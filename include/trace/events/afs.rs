//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/afs.h
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
// AFS tracepoints
//
// Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Define enums for tracing information.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_fs_operation {
    afs_FS_FetchData		= 130,	/* AFS Fetch file data */
    afs_FS_FetchACL			= 131,	/* AFS Fetch file ACL */
    afs_FS_FetchStatus		= 132,	/* AFS Fetch file status */
    afs_FS_StoreData		= 133,	/* AFS Store file data */
    afs_FS_StoreACL			= 134,	/* AFS Store file ACL */
    afs_FS_StoreStatus		= 135,	/* AFS Store file status */
    afs_FS_RemoveFile		= 136,	/* AFS Remove a file */
    afs_FS_CreateFile		= 137,	/* AFS Create a file */
    afs_FS_Rename			= 138,	/* AFS Rename or move a file or directory */
    afs_FS_Symlink			= 139,	/* AFS Create a symbolic link */
    afs_FS_Link			= 140,	/* AFS Create a hard link */
    afs_FS_MakeDir			= 141,	/* AFS Create a directory */
    afs_FS_RemoveDir		= 142,	/* AFS Remove a directory */
    afs_FS_GetVolumeInfo		= 148,	/* AFS Get information about a volume */
    afs_FS_GetVolumeStatus		= 149,	/* AFS Get volume status information */
    afs_FS_GetRootVolume		= 151,	/* AFS Get root volume name */
    afs_FS_SetLock			= 156,	/* AFS Request a file lock */
    afs_FS_ExtendLock		= 157,	/* AFS Extend a file lock */
    afs_FS_ReleaseLock		= 158,	/* AFS Release a file lock */
    afs_FS_Lookup			= 161,	/* AFS lookup file in directory */
    afs_FS_InlineBulkStatus		= 65536, /* AFS Fetch multiple file statuses with errors */
    afs_FS_FetchData64		= 65537, /* AFS Fetch file data */
    afs_FS_StoreData64		= 65538, /* AFS Store file data */
    afs_FS_GiveUpAllCallBacks	= 65539, /* AFS Give up all our callbacks on a server */
    afs_FS_GetCapabilities		= 65540, /* AFS Get FS server capabilities */

    yfs_FS_FetchData		= 130,	 /* YFS Fetch file data */
    yfs_FS_FetchACL			= 64131, /* YFS Fetch file ACL */
    yfs_FS_FetchStatus		= 64132, /* YFS Fetch file status */
    yfs_FS_StoreACL			= 64134, /* YFS Store file ACL */
    yfs_FS_StoreStatus		= 64135, /* YFS Store file status */
    yfs_FS_RemoveFile		= 64136, /* YFS Remove a file */
    yfs_FS_CreateFile		= 64137, /* YFS Create a file */
    yfs_FS_Rename			= 64138, /* YFS Rename or move a file or directory */
    yfs_FS_Symlink			= 64139, /* YFS Create a symbolic link */
    yfs_FS_Link			= 64140, /* YFS Create a hard link */
    yfs_FS_MakeDir			= 64141, /* YFS Create a directory */
    yfs_FS_RemoveDir		= 64142, /* YFS Remove a directory */
    yfs_FS_GetVolumeStatus		= 64149, /* YFS Get volume status information */
    yfs_FS_SetVolumeStatus		= 64150, /* YFS Set volume status information */
    yfs_FS_SetLock			= 64156, /* YFS Request a file lock */
    yfs_FS_ExtendLock		= 64157, /* YFS Extend a file lock */
    yfs_FS_ReleaseLock		= 64158, /* YFS Release a file lock */
    yfs_FS_Lookup			= 64161, /* YFS lookup file in directory */
    yfs_FS_FlushCPS			= 64165,
    yfs_FS_FetchOpaqueACL		= 64168,
    yfs_FS_WhoAmI			= 64170,
    yfs_FS_RemoveACL		= 64171,
    yfs_FS_RemoveFile2		= 64173,
    yfs_FS_StoreOpaqueACL2		= 64174,
    yfs_FS_Rename_Replace		= 64176,
    yfs_FS_Rename_NoReplace		= 64177,
    yfs_FS_Rename_Exchange		= 64187,
    yfs_FS_InlineBulkStatus		= 64536, /* YFS Fetch multiple file statuses with errors */
    yfs_FS_FetchData64		= 64537, /* YFS Fetch file data */
    yfs_FS_StoreData64		= 64538, /* YFS Store file data */
    yfs_FS_UpdateSymlink		= 64540,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_vl_operation {
    afs_VL_GetEntryByNameU	= 527,		/* AFS Get Vol Entry By Name operation ID */
    afs_VL_GetAddrsU	= 533,		/* AFS Get FS server addresses */
    afs_YFSVL_GetEndpoints	= 64002,	/* YFS Get FS & Vol server addresses */
    afs_YFSVL_GetCellName	= 64014,	/* YFS Get actual cell name */
    afs_VL_GetCapabilities	= 65537,	/* AFS Get VL server capabilities */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_cm_operation {
    afs_CB_CallBack			= 204,	/* AFS break callback promises */
    afs_CB_InitCallBackState	= 205,	/* AFS initialise callback state */
    afs_CB_Probe			= 206,	/* AFS probe client */
    afs_CB_GetLock			= 207,	/* AFS get contents of CM lock table */
    afs_CB_GetCE			= 208,	/* AFS get cache file description */
    afs_CB_GetXStatsVersion		= 209,	/* AFS get version of extended statistics */
    afs_CB_GetXStats		= 210,	/* AFS get contents of extended statistics data */
    afs_CB_InitCallBackState3	= 213,	/* AFS initialise callback state, version 3 */
    afs_CB_ProbeUuid		= 214,	/* AFS check the client hasn't rebooted */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum yfs_cm_operation {
    yfs_CB_Probe			= 206,	/* YFS probe client */
    yfs_CB_GetLock			= 207,	/* YFS get contents of CM lock table */
    yfs_CB_XStatsVersion		= 209,	/* YFS get version of extended statistics */
    yfs_CB_GetXStats		= 210,	/* YFS get contents of extended statistics data */
    yfs_CB_InitCallBackState3	= 213,	/* YFS initialise callback state, version 3 */
    yfs_CB_ProbeUuid		= 214,	/* YFS check the client hasn't rebooted */
    yfs_CB_GetServerPrefs		= 215,
    yfs_CB_GetCellServDV		= 216,
    yfs_CB_GetLocalCell		= 217,
    yfs_CB_GetCacheConfig		= 218,
    yfs_CB_GetCellByNum		= 65537,
    yfs_CB_TellMeAboutYourself	= 65538, /* get client capabilities */
    yfs_CB_CallBack			= 64204,
}

//
// Declare tracing information enums and their string mappings for display.
//

//
// Generate enums for tracing information.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_alist_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_call_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_cb_break_reason {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_cb_promise_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_cell_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_dir_invalid_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_edit_dir_op {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_edit_dir_reason {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_eproto_cause {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_estate_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_file_error {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_flock_event {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_flock_operation {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_io_error {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_rotate_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_server_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_vnode_invalid_trace {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum afs_volume_trace {

//
// Export enum symbols via userspace.
//

    afs_alist_traces;
    afs_call_traces;
    afs_cb_break_reasons;
    afs_cb_promise_traces;
    afs_cell_traces;
    afs_cm_operations;
    afs_dir_invalid_traces;
    afs_edit_dir_ops;
    afs_edit_dir_reasons;
    afs_eproto_causes;
    afs_estate_traces;
    afs_file_errors;
    afs_flock_operations;
    afs_flock_types;
    afs_fs_operations;
    afs_io_errors;
    afs_rotate_traces;
    afs_server_traces;
    afs_vnode_invalid_traces;
    afs_vl_operations;
    yfs_cm_operations;

//
// Now redefine the EM() and E_() macros to map the enums to the strings that
// will be printed in the output.
//

    TRACE_EVENT(afs_receive_data,
    TP_PROTO(struct afs_call *call, struct iov_iter *iter,
    bool want_more, int ret),

    TP_ARGS(call, iter, want_more, ret),

    TP_STRUCT__entry(
    __field(loff_t,			remain)
    __field(unsigned int,		call)
    __field(enum afs_call_state,	state)
    __field(unsigned short,		unmarshall)
    __field(bool,			want_more)
    __field(int,			ret)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->state	= call->state;
    __entry->unmarshall	= call->unmarshall;
    __entry->remain	= iov_iter_count(iter);
    __entry->want_more	= want_more;
    __entry->ret	= ret;
    ),

    TP_printk("c=%08x r=%llu u=%u w=%u s=%u ret=%d",
    __entry->call,
    __entry->remain,
    __entry->unmarshall,
    __entry->want_more,
    __entry->state,
    __entry->ret)
    );

    TRACE_EVENT(afs_notify_call,
    TP_PROTO(struct rxrpc_call *rxcall, struct afs_call *call),

    TP_ARGS(rxcall, call),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum afs_call_state,	state)
    __field(unsigned short,		unmarshall)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->state	= call->state;
    __entry->unmarshall	= call->unmarshall;
    ),

    TP_printk("c=%08x s=%u u=%u",
    __entry->call,
    __entry->state, __entry->unmarshall)
    );

    TRACE_EVENT(afs_cb_call,
    TP_PROTO(struct afs_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(u32,			op)
    __field(u16,			service_id)
    __field(u8,				security_ix)
    __field(u32,			enctype)
    ),

    TP_fast_assign(
    __entry->call	= call->debug_id;
    __entry->op		= call->operation_ID;
    __entry->service_id	= call->service_id;
    __entry->security_ix = call->security_ix;
    __entry->enctype	= call->enctype;
    ),

    TP_printk("c=%08x %s sv=%u sx=%u en=%u",
    __entry->call,
    __entry->service_id == 2501 ?
    __print_symbolic(__entry->op, yfs_cm_operations) :
    __print_symbolic(__entry->op, afs_cm_operations),
    __entry->service_id,
    __entry->security_ix,
    __entry->enctype)
    );

    TRACE_EVENT(afs_call,
    TP_PROTO(unsigned int call_debug_id, enum afs_call_trace op,
    int ref, int outstanding, const void *where),

    TP_ARGS(call_debug_id, op, ref, outstanding, where),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(int,			op)
    __field(int,			ref)
    __field(int,			outstanding)
    __field(const void *,		where)
    ),

    TP_fast_assign(
    __entry->call = call_debug_id;
    __entry->op = op;
    __entry->ref = ref;
    __entry->outstanding = outstanding;
    __entry->where = where;
    ),

    TP_printk("c=%08x %s r=%d o=%d sp=%pSR",
    __entry->call,
    __print_symbolic(__entry->op, afs_call_traces),
    __entry->ref,
    __entry->outstanding,
    __entry->where)
    );

    TRACE_EVENT(afs_make_fs_call,
    TP_PROTO(struct afs_call *call, const struct afs_fid *fid),

    TP_ARGS(call, fid),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum afs_fs_operation,	op)
    __field_struct(struct afs_fid,	fid)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->op = call->operation_ID;
    if (fid) {
    __entry->fid = *fid;
    } else {
    __entry->fid.vid = 0;
    __entry->fid.vnode = 0;
    __entry->fid.unique = 0;
    }
    ),

    TP_printk("c=%08x V=%llx i=%llx:%x %s",
    __entry->call,
    __entry->fid.vid,
    __entry->fid.vnode,
    __entry->fid.unique,
    __print_symbolic(__entry->op, afs_fs_operations))
    );

    TRACE_EVENT(afs_make_fs_calli,
    TP_PROTO(struct afs_call *call, const struct afs_fid *fid,
    unsigned int i),

    TP_ARGS(call, fid, i),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(unsigned int,		i)
    __field(enum afs_fs_operation,	op)
    __field_struct(struct afs_fid,	fid)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->i = i;
    __entry->op = call->operation_ID;
    if (fid) {
    __entry->fid = *fid;
    } else {
    __entry->fid.vid = 0;
    __entry->fid.vnode = 0;
    __entry->fid.unique = 0;
    }
    ),

    TP_printk("c=%08x V=%llx i=%llx:%x %s i=%u",
    __entry->call,
    __entry->fid.vid,
    __entry->fid.vnode,
    __entry->fid.unique,
    __print_symbolic(__entry->op, afs_fs_operations),
    __entry->i)
    );

    TRACE_EVENT(afs_make_fs_call1,
    TP_PROTO(struct afs_call *call, const struct afs_fid *fid,
    const struct qstr *name),

    TP_ARGS(call, fid, name),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum afs_fs_operation,	op)
    __field_struct(struct afs_fid,	fid)
    __array(char,			name, 24)
    ),

    TP_fast_assign(
    unsigned int __len = min_t(unsigned int, name->len, 23);
    __entry->call = call->debug_id;
    __entry->op = call->operation_ID;
    if (fid) {
    __entry->fid = *fid;
    } else {
    __entry->fid.vid = 0;
    __entry->fid.vnode = 0;
    __entry->fid.unique = 0;
    }
    memcpy(__entry->name, name->name, __len);
    __entry->name[__len] = 0;
    ),

    TP_printk("c=%08x V=%llx i=%llx:%x %s \"%s\"",
    __entry->call,
    __entry->fid.vid,
    __entry->fid.vnode,
    __entry->fid.unique,
    __print_symbolic(__entry->op, afs_fs_operations),
    __entry->name)
    );

    TRACE_EVENT(afs_make_fs_call2,
    TP_PROTO(struct afs_call *call, const struct afs_fid *fid,
    const struct qstr *name, const struct qstr *name2),

    TP_ARGS(call, fid, name, name2),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum afs_fs_operation,	op)
    __field_struct(struct afs_fid,	fid)
    __array(char,			name, 24)
    __array(char,			name2, 24)
    ),

    TP_fast_assign(
    unsigned int __len = min_t(unsigned int, name->len, 23);
    unsigned int __len2 = min_t(unsigned int, name2->len, 23);
    __entry->call = call->debug_id;
    __entry->op = call->operation_ID;
    if (fid) {
    __entry->fid = *fid;
    } else {
    __entry->fid.vid = 0;
    __entry->fid.vnode = 0;
    __entry->fid.unique = 0;
    }
    memcpy(__entry->name, name->name, __len);
    __entry->name[__len] = 0;
    memcpy(__entry->name2, name2->name, __len2);
    __entry->name2[__len2] = 0;
    ),

    TP_printk("c=%08x V=%llx i=%llx:%x %s \"%s\" \"%s\"",
    __entry->call,
    __entry->fid.vid,
    __entry->fid.vnode,
    __entry->fid.unique,
    __print_symbolic(__entry->op, afs_fs_operations),
    __entry->name,
    __entry->name2)
    );

    TRACE_EVENT(afs_make_vl_call,
    TP_PROTO(struct afs_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum afs_vl_operation,	op)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->op = call->operation_ID;
    ),

    TP_printk("c=%08x %s",
    __entry->call,
    __print_symbolic(__entry->op, afs_vl_operations))
    );

    TRACE_EVENT(afs_call_done,
    TP_PROTO(struct afs_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(struct rxrpc_call *,	rx_call)
    __field(int,			ret)
    __field(u32,			abort_code)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->rx_call = call->rxcall;
    __entry->ret = call->error;
    __entry->abort_code = call->abort_code;
    ),

    TP_printk("   c=%08x ret=%d ab=%d [%p]",
    __entry->call,
    __entry->ret,
    __entry->abort_code,
    __entry->rx_call)
    );

    TRACE_EVENT(afs_send_data,
    TP_PROTO(struct afs_call *call, struct msghdr *msg),

    TP_ARGS(call, msg),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(unsigned int,		flags)
    __field(loff_t,			offset)
    __field(loff_t,			count)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->flags = msg->msg_flags;
    __entry->offset = msg->msg_iter.xarray_start + msg->msg_iter.iov_offset;
    __entry->count = iov_iter_count(&msg->msg_iter);
    ),

    TP_printk(" c=%08x o=%llx n=%llx f=%x",
    __entry->call, __entry->offset, __entry->count,
    __entry->flags)
    );

    TRACE_EVENT(afs_sent_data,
    TP_PROTO(struct afs_call *call, struct msghdr *msg, int ret),

    TP_ARGS(call, msg, ret),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(int,			ret)
    __field(loff_t,			offset)
    __field(loff_t,			count)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->ret = ret;
    __entry->offset = msg->msg_iter.xarray_start + msg->msg_iter.iov_offset;
    __entry->count = iov_iter_count(&msg->msg_iter);
    ),

    TP_printk(" c=%08x o=%llx n=%llx r=%x",
    __entry->call, __entry->offset, __entry->count,
    __entry->ret)
    );

    TRACE_EVENT(afs_dir_check_failed,
    TP_PROTO(struct afs_vnode *vnode, loff_t off),

    TP_ARGS(vnode, off),

    TP_STRUCT__entry(
    __field(struct afs_vnode *,		vnode)
    __field(loff_t,			off)
    __field(loff_t,			i_size)
    ),

    TP_fast_assign(
    __entry->vnode = vnode;
    __entry->off = off;
    __entry->i_size = i_size_read(&vnode->netfs.inode);
    ),

    TP_printk("vn=%p %llx/%llx",
    __entry->vnode, __entry->off, __entry->i_size)
    );

    TRACE_EVENT(afs_call_state,
    TP_PROTO(struct afs_call *call,
    enum afs_call_state from,
    enum afs_call_state to,
    int ret, u32 remote_abort),

    TP_ARGS(call, from, to, ret, remote_abort),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum afs_call_state,	from)
    __field(enum afs_call_state,	to)
    __field(int,			ret)
    __field(u32,			abort)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->from = from;
    __entry->to = to;
    __entry->ret = ret;
    __entry->abort = remote_abort;
    ),

    TP_printk("c=%08x %u->%u r=%d ab=%d",
    __entry->call,
    __entry->from, __entry->to,
    __entry->ret, __entry->abort)
    );

    TRACE_EVENT(afs_lookup,
    TP_PROTO(struct afs_vnode *dvnode, const struct qstr *name,
    struct afs_fid *fid),

    TP_ARGS(dvnode, name, fid),

    TP_STRUCT__entry(
    __field_struct(struct afs_fid,	dfid)
    __field_struct(struct afs_fid,	fid)
    __array(char,			name, 24)
    ),

    TP_fast_assign(
    int __len = min_t(int, name->len, 23);
    __entry->dfid = dvnode->fid;
    __entry->fid = *fid;
    memcpy(__entry->name, name->name, __len);
    __entry->name[__len] = 0;
    ),

    TP_printk("d=%llx:%llx:%x \"%s\" f=%llx:%x",
    __entry->dfid.vid, __entry->dfid.vnode, __entry->dfid.unique,
    __entry->name,
    __entry->fid.vnode, __entry->fid.unique)
    );

    TRACE_EVENT(afs_edit_dir,
    TP_PROTO(struct afs_vnode *dvnode,
    enum afs_edit_dir_reason why,
    enum afs_edit_dir_op op,
    unsigned int block,
    unsigned int slot,
    unsigned int f_vnode,
    unsigned int f_unique,
    const char *name),

    TP_ARGS(dvnode, why, op, block, slot, f_vnode, f_unique, name),

    TP_STRUCT__entry(
    __field(unsigned int,		vnode)
    __field(unsigned int,		unique)
    __field(enum afs_edit_dir_reason,	why)
    __field(enum afs_edit_dir_op,	op)
    __field(unsigned int,		block)
    __field(unsigned short,		slot)
    __field(unsigned int,		f_vnode)
    __field(unsigned int,		f_unique)
    __array(char,			name, 24)
    ),

    TP_fast_assign(
    int __len = strlen(name);
    __len = min(__len, 23);
    __entry->vnode	= dvnode->fid.vnode;
    __entry->unique	= dvnode->fid.unique;
    __entry->why	= why;
    __entry->op		= op;
    __entry->block	= block;
    __entry->slot	= slot;
    __entry->f_vnode	= f_vnode;
    __entry->f_unique	= f_unique;
    memcpy(__entry->name, name, __len);
    __entry->name[__len] = 0;
    ),

    TP_printk("di=%x:%x %s %s %u[%u] fi=%x:%x \"%s\"",
    __entry->vnode, __entry->unique,
    __print_symbolic(__entry->why, afs_edit_dir_reasons),
    __print_symbolic(__entry->op, afs_edit_dir_ops),
    __entry->block, __entry->slot,
    __entry->f_vnode, __entry->f_unique,
    __entry->name)
    );

    TRACE_EVENT(afs_dir_invalid,
    TP_PROTO(const struct afs_vnode *dvnode, enum afs_dir_invalid_trace trace),

    TP_ARGS(dvnode, trace),

    TP_STRUCT__entry(
    __field(unsigned int,		vnode)
    __field(unsigned int,		unique)
    __field(enum afs_dir_invalid_trace,	trace)
    ),

    TP_fast_assign(
    __entry->vnode	= dvnode->fid.vnode;
    __entry->unique	= dvnode->fid.unique;
    __entry->trace	= trace;
    ),

    TP_printk("di=%x:%x %s",
    __entry->vnode, __entry->unique,
    __print_symbolic(__entry->trace, afs_dir_invalid_traces))
    );

    TRACE_EVENT(afs_cb_promise,
    TP_PROTO(const struct afs_vnode *vnode, enum afs_cb_promise_trace trace),

    TP_ARGS(vnode, trace),

    TP_STRUCT__entry(
    __field(unsigned int,		vnode)
    __field(unsigned int,		unique)
    __field(enum afs_cb_promise_trace,	trace)
    ),

    TP_fast_assign(
    __entry->vnode	= vnode->fid.vnode;
    __entry->unique	= vnode->fid.unique;
    __entry->trace	= trace;
    ),

    TP_printk("di=%x:%x %s",
    __entry->vnode, __entry->unique,
    __print_symbolic(__entry->trace, afs_cb_promise_traces))
    );

    TRACE_EVENT(afs_vnode_invalid,
    TP_PROTO(const struct afs_vnode *vnode, enum afs_vnode_invalid_trace trace),

    TP_ARGS(vnode, trace),

    TP_STRUCT__entry(
    __field(unsigned int,		vnode)
    __field(unsigned int,		unique)
    __field(enum afs_vnode_invalid_trace, trace)
    ),

    TP_fast_assign(
    __entry->vnode	= vnode->fid.vnode;
    __entry->unique	= vnode->fid.unique;
    __entry->trace	= trace;
    ),

    TP_printk("di=%x:%x %s",
    __entry->vnode, __entry->unique,
    __print_symbolic(__entry->trace, afs_vnode_invalid_traces))
    );

    TRACE_EVENT(afs_set_dv,
    TP_PROTO(const struct afs_vnode *dvnode, u64 new_dv),

    TP_ARGS(dvnode, new_dv),

    TP_STRUCT__entry(
    __field(unsigned int,		vnode)
    __field(unsigned int,		unique)
    __field(u64,			old_dv)
    __field(u64,			new_dv)
    ),

    TP_fast_assign(
    __entry->vnode	= dvnode->fid.vnode;
    __entry->unique	= dvnode->fid.unique;
    __entry->old_dv	= dvnode->status.data_version;
    __entry->new_dv	= new_dv;
    ),

    TP_printk("di=%x:%x dv=%llx -> dv=%llx",
    __entry->vnode, __entry->unique,
    __entry->old_dv, __entry->new_dv)
    );

    TRACE_EVENT(afs_dv_mismatch,
    TP_PROTO(const struct afs_vnode *dvnode, u64 before_dv, int delta, u64 new_dv),

    TP_ARGS(dvnode, before_dv, delta, new_dv),

    TP_STRUCT__entry(
    __field(unsigned int,		vnode)
    __field(unsigned int,		unique)
    __field(int,			delta)
    __field(u64,			before_dv)
    __field(u64,			new_dv)
    ),

    TP_fast_assign(
    __entry->vnode	= dvnode->fid.vnode;
    __entry->unique	= dvnode->fid.unique;
    __entry->delta	= delta;
    __entry->before_dv	= before_dv;
    __entry->new_dv	= new_dv;
    ),

    TP_printk("di=%x:%x xdv=%llx+%d dv=%llx",
    __entry->vnode, __entry->unique,
    __entry->before_dv, __entry->delta, __entry->new_dv)
    );

    TRACE_EVENT(afs_protocol_error,
    TP_PROTO(struct afs_call *call, enum afs_eproto_cause cause),

    TP_ARGS(call, cause),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(enum afs_eproto_cause,	cause)
    ),

    TP_fast_assign(
    __entry->call = call ? call->debug_id : 0;
    __entry->cause = cause;
    ),

    TP_printk("c=%08x %s",
    __entry->call,
    __print_symbolic(__entry->cause, afs_eproto_causes))
    );

    TRACE_EVENT(afs_io_error,
    TP_PROTO(unsigned int call, int error, enum afs_io_error where),

    TP_ARGS(call, error, where),

    TP_STRUCT__entry(
    __field(unsigned int,	call)
    __field(int,		error)
    __field(enum afs_io_error,	where)
    ),

    TP_fast_assign(
    __entry->call = call;
    __entry->error = error;
    __entry->where = where;
    ),

    TP_printk("c=%08x r=%d %s",
    __entry->call, __entry->error,
    __print_symbolic(__entry->where, afs_io_errors))
    );

    TRACE_EVENT(afs_file_error,
    TP_PROTO(struct afs_vnode *vnode, int error, enum afs_file_error where),

    TP_ARGS(vnode, error, where),

    TP_STRUCT__entry(
    __field_struct(struct afs_fid,	fid)
    __field(int,			error)
    __field(enum afs_file_error,	where)
    ),

    TP_fast_assign(
    __entry->fid = vnode->fid;
    __entry->error = error;
    __entry->where = where;
    ),

    TP_printk("%llx:%llx:%x r=%d %s",
    __entry->fid.vid, __entry->fid.vnode, __entry->fid.unique,
    __entry->error,
    __print_symbolic(__entry->where, afs_file_errors))
    );

    TRACE_EVENT(afs_bulkstat_error,
    TP_PROTO(struct afs_operation *op, struct afs_fid *fid, unsigned int index, s32 abort),

    TP_ARGS(op, fid, index, abort),

    TP_STRUCT__entry(
    __field_struct(struct afs_fid,	fid)
    __field(unsigned int,		op)
    __field(unsigned int,		index)
    __field(s32,			abort)
    ),

    TP_fast_assign(
    __entry->op = op->debug_id;
    __entry->fid = *fid;
    __entry->index = index;
    __entry->abort = abort;
    ),

    TP_printk("OP=%08x[%02x] %llx:%llx:%x a=%d",
    __entry->op, __entry->index,
    __entry->fid.vid, __entry->fid.vnode, __entry->fid.unique,
    __entry->abort)
    );

    TRACE_EVENT(afs_cm_no_server,
    TP_PROTO(struct afs_call *call, const struct sockaddr_rxrpc *srx),

    TP_ARGS(call, srx),

    TP_STRUCT__entry(
    __field(unsigned int,			call)
    __field(unsigned int,			op_id)
    __field_struct(struct sockaddr_rxrpc,	srx)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->op_id = call->operation_ID;
    memcpy(&__entry->srx, srx, sizeof(__entry->srx));
    ),

    TP_printk("c=%08x op=%u %pISpc",
    __entry->call, __entry->op_id, &__entry->srx.transport)
    );

    TRACE_EVENT(afs_cm_no_server_u,
    TP_PROTO(struct afs_call *call, const uuid_t *uuid),

    TP_ARGS(call, uuid),

    TP_STRUCT__entry(
    __field(unsigned int,			call)
    __field(unsigned int,			op_id)
    __field_struct(uuid_t,			uuid)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->op_id = call->operation_ID;
    memcpy(&__entry->uuid, uuid, sizeof(__entry->uuid));
    ),

    TP_printk("c=%08x op=%u %pU",
    __entry->call, __entry->op_id, &__entry->uuid)
    );

    TRACE_EVENT(afs_flock_ev,
    TP_PROTO(struct afs_vnode *vnode, struct file_lock *fl,
    enum afs_flock_event event, int error),

    TP_ARGS(vnode, fl, event, error),

    TP_STRUCT__entry(
    __field_struct(struct afs_fid,	fid)
    __field(enum afs_flock_event,	event)
    __field(enum afs_lock_state,	state)
    __field(int,			error)
    __field(unsigned int,		debug_id)
    ),

    TP_fast_assign(
    __entry->fid = vnode->fid;
    __entry->event = event;
    __entry->state = vnode->lock_state;
    __entry->error = error;
    __entry->debug_id = fl ? fl->fl_u.afs.debug_id : 0;
    ),

    TP_printk("%llx:%llx:%x %04x %s s=%s e=%d",
    __entry->fid.vid, __entry->fid.vnode, __entry->fid.unique,
    __entry->debug_id,
    __print_symbolic(__entry->event, afs_flock_events),
    __print_symbolic(__entry->state, afs_flock_states),
    __entry->error)
    );

    TRACE_EVENT(afs_flock_op,
    TP_PROTO(struct afs_vnode *vnode, struct file_lock *fl,
    enum afs_flock_operation op),

    TP_ARGS(vnode, fl, op),

    TP_STRUCT__entry(
    __field_struct(struct afs_fid,	fid)
    __field(loff_t,			from)
    __field(loff_t,			len)
    __field(enum afs_flock_operation,	op)
    __field(unsigned char,		type)
    __field(unsigned int,		flags)
    __field(unsigned int,		debug_id)
    ),

    TP_fast_assign(
    __entry->fid = vnode->fid;
    __entry->from = fl->fl_start;
    __entry->len = fl->fl_end - fl->fl_start + 1;
    __entry->op = op;
    __entry->type = fl->c.flc_type;
    __entry->flags = fl->c.flc_flags;
    __entry->debug_id = fl->fl_u.afs.debug_id;
    ),

    TP_printk("%llx:%llx:%x %04x %s t=%s R=%llx/%llx f=%x",
    __entry->fid.vid, __entry->fid.vnode, __entry->fid.unique,
    __entry->debug_id,
    __print_symbolic(__entry->op, afs_flock_operations),
    __print_symbolic(__entry->type, afs_flock_types),
    __entry->from, __entry->len, __entry->flags)
    );

    TRACE_EVENT(afs_reload_dir,
    TP_PROTO(struct afs_vnode *vnode),

    TP_ARGS(vnode),

    TP_STRUCT__entry(
    __field_struct(struct afs_fid,	fid)
    ),

    TP_fast_assign(
    __entry->fid = vnode->fid;
    ),

    TP_printk("%llx:%llx:%x",
    __entry->fid.vid, __entry->fid.vnode, __entry->fid.unique)
    );

    TRACE_EVENT(afs_silly_rename,
    TP_PROTO(struct afs_vnode *vnode, bool done),

    TP_ARGS(vnode, done),

    TP_STRUCT__entry(
    __field_struct(struct afs_fid,	fid)
    __field(bool,			done)
    ),

    TP_fast_assign(
    __entry->fid = vnode->fid;
    __entry->done = done;
    ),

    TP_printk("%llx:%llx:%x done=%u",
    __entry->fid.vid, __entry->fid.vnode, __entry->fid.unique,
    __entry->done)
    );

    TRACE_EVENT(afs_get_tree,
    TP_PROTO(struct afs_cell *cell, struct afs_volume *volume),

    TP_ARGS(cell, volume),

    TP_STRUCT__entry(
    __field(u64,			vid)
    __array(char,			cell, 24)
    __array(char,			volume, 24)
    ),

    TP_fast_assign(
    int __len;
    __entry->vid = volume->vid;
    __len = min_t(int, cell->name_len, 23);
    memcpy(__entry->cell, cell->name, __len);
    __entry->cell[__len] = 0;
    __len = min_t(int, volume->name_len, 23);
    memcpy(__entry->volume, volume->name, __len);
    __entry->volume[__len] = 0;
    ),

    TP_printk("--- MOUNT %s:%s %llx",
    __entry->cell, __entry->volume, __entry->vid)
    );

    TRACE_EVENT(afs_cb_v_break,
    TP_PROTO(afs_volid_t vid, unsigned int cb_v_break,
    enum afs_cb_break_reason reason),

    TP_ARGS(vid, cb_v_break, reason),

    TP_STRUCT__entry(
    __field(afs_volid_t,		vid)
    __field(unsigned int,		cb_v_break)
    __field(enum afs_cb_break_reason,	reason)
    ),

    TP_fast_assign(
    __entry->vid	= vid;
    __entry->cb_v_break	= cb_v_break;
    __entry->reason	= reason;
    ),

    TP_printk("%llx vb=%x %s",
    __entry->vid,
    __entry->cb_v_break,
    __print_symbolic(__entry->reason, afs_cb_break_reasons))
    );

    TRACE_EVENT(afs_cb_break,
    TP_PROTO(struct afs_fid *fid, unsigned int cb_break,
    enum afs_cb_break_reason reason, bool skipped),

    TP_ARGS(fid, cb_break, reason, skipped),

    TP_STRUCT__entry(
    __field_struct(struct afs_fid,	fid)
    __field(unsigned int,		cb_break)
    __field(enum afs_cb_break_reason,	reason)
    __field(bool,			skipped)
    ),

    TP_fast_assign(
    __entry->fid	= *fid;
    __entry->cb_break	= cb_break;
    __entry->reason	= reason;
    __entry->skipped	= skipped;
    ),

    TP_printk("%llx:%llx:%x b=%x s=%u %s",
    __entry->fid.vid, __entry->fid.vnode, __entry->fid.unique,
    __entry->cb_break,
    __entry->skipped,
    __print_symbolic(__entry->reason, afs_cb_break_reasons))
    );

    TRACE_EVENT(afs_cb_miss,
    TP_PROTO(struct afs_fid *fid, enum afs_cb_break_reason reason),

    TP_ARGS(fid, reason),

    TP_STRUCT__entry(
    __field_struct(struct afs_fid,	fid)
    __field(enum afs_cb_break_reason,	reason)
    ),

    TP_fast_assign(
    __entry->fid	= *fid;
    __entry->reason	= reason;
    ),

    TP_printk(" %llx:%llx:%x %s",
    __entry->fid.vid, __entry->fid.vnode, __entry->fid.unique,
    __print_symbolic(__entry->reason, afs_cb_break_reasons))
    );

    TRACE_EVENT(afs_server,
    TP_PROTO(unsigned int server_debug_id, int ref, int active,
    enum afs_server_trace reason),

    TP_ARGS(server_debug_id, ref, active, reason),

    TP_STRUCT__entry(
    __field(unsigned int,		server)
    __field(int,			ref)
    __field(int,			active)
    __field(int,			reason)
    ),

    TP_fast_assign(
    __entry->server = server_debug_id;
    __entry->ref = ref;
    __entry->active = active;
    __entry->reason = reason;
    ),

    TP_printk("s=%08x %s r=%d a=%d",
    __entry->server,
    __print_symbolic(__entry->reason, afs_server_traces),
    __entry->ref,
    __entry->active)
    );

    TRACE_EVENT(afs_volume,
    TP_PROTO(unsigned int debug_id, afs_volid_t vid, int ref,
    enum afs_volume_trace reason),

    TP_ARGS(debug_id, vid, ref, reason),

    TP_STRUCT__entry(
    __field(unsigned int,		debug_id)
    __field(afs_volid_t,		vid)
    __field(int,			ref)
    __field(enum afs_volume_trace,	reason)
    ),

    TP_fast_assign(
    __entry->debug_id	= debug_id;
    __entry->vid	= vid;
    __entry->ref	= ref;
    __entry->reason	= reason;
    ),

    TP_printk("V=%08x %s vid=%llx r=%d",
    __entry->debug_id,
    __print_symbolic(__entry->reason, afs_volume_traces),
    __entry->vid,
    __entry->ref)
    );

    TRACE_EVENT(afs_cell,
    TP_PROTO(unsigned int cell_debug_id, int ref, int active,
    enum afs_cell_trace reason),

    TP_ARGS(cell_debug_id, ref, active, reason),

    TP_STRUCT__entry(
    __field(unsigned int,		cell)
    __field(int,			ref)
    __field(int,			active)
    __field(int,			reason)
    ),

    TP_fast_assign(
    __entry->cell = cell_debug_id;
    __entry->ref = ref;
    __entry->active = active;
    __entry->reason = reason;
    ),

    TP_printk("L=%08x %s r=%d a=%d",
    __entry->cell,
    __print_symbolic(__entry->reason, afs_cell_traces),
    __entry->ref,
    __entry->active)
    );

    TRACE_EVENT(afs_alist,
    TP_PROTO(unsigned int alist_debug_id, int ref, enum afs_alist_trace reason),

    TP_ARGS(alist_debug_id, ref, reason),

    TP_STRUCT__entry(
    __field(unsigned int,		alist)
    __field(int,			ref)
    __field(int,			active)
    __field(int,			reason)
    ),

    TP_fast_assign(
    __entry->alist = alist_debug_id;
    __entry->ref = ref;
    __entry->reason = reason;
    ),

    TP_printk("AL=%08x %s r=%d",
    __entry->alist,
    __print_symbolic(__entry->reason, afs_alist_traces),
    __entry->ref)
    );

    TRACE_EVENT(afs_estate,
    TP_PROTO(unsigned int server_debug_id, unsigned int estate_debug_id,
    int ref, enum afs_estate_trace reason),

    TP_ARGS(server_debug_id, estate_debug_id, ref, reason),

    TP_STRUCT__entry(
    __field(unsigned int,		server)
    __field(unsigned int,		estate)
    __field(int,			ref)
    __field(int,			active)
    __field(int,			reason)
    ),

    TP_fast_assign(
    __entry->server = server_debug_id;
    __entry->estate = estate_debug_id;
    __entry->ref = ref;
    __entry->reason = reason;
    ),

    TP_printk("ES=%08x[%x] %s r=%d",
    __entry->server,
    __entry->estate,
    __print_symbolic(__entry->reason, afs_estate_traces),
    __entry->ref)
    );

    TRACE_EVENT(afs_fs_probe,
    TP_PROTO(struct afs_server *server, bool tx, struct afs_endpoint_state *estate,
    unsigned int addr_index, int error, s32 abort_code, unsigned int rtt_us),

    TP_ARGS(server, tx, estate, addr_index, error, abort_code, rtt_us),

    TP_STRUCT__entry(
    __field(unsigned int,		server)
    __field(unsigned int,		estate)
    __field(bool,			tx)
    __field(u16,			addr_index)
    __field(short,			error)
    __field(s32,			abort_code)
    __field(unsigned int,		rtt_us)
    __field_struct(struct sockaddr_rxrpc, srx)
    ),

    TP_fast_assign(
    struct afs_addr_list *alist = estate->addresses;
    __entry->server = server->debug_id;
    __entry->estate = estate->probe_seq;
    __entry->tx = tx;
    __entry->addr_index = addr_index;
    __entry->error = error;
    __entry->abort_code = abort_code;
    __entry->rtt_us = rtt_us;
    memcpy(&__entry->srx, rxrpc_kernel_remote_srx(alist->addrs[addr_index].peer),
    sizeof(__entry->srx));
    ),

    TP_printk("s=%08x %s pq=%x ax=%u e=%d ac=%d rtt=%d %pISpc",
    __entry->server, __entry->tx ? "tx" : "rx", __entry->estate,
    __entry->addr_index, __entry->error, __entry->abort_code, __entry->rtt_us,
    &__entry->srx.transport)
    );

    TRACE_EVENT(afs_vl_probe,
    TP_PROTO(struct afs_vlserver *server, bool tx, struct afs_addr_list *alist,
    unsigned int addr_index, int error, s32 abort_code, unsigned int rtt_us),

    TP_ARGS(server, tx, alist, addr_index, error, abort_code, rtt_us),

    TP_STRUCT__entry(
    __field(unsigned int,		server)
    __field(bool,			tx)
    __field(unsigned short,		flags)
    __field(u16,			addr_index)
    __field(short,			error)
    __field(s32,			abort_code)
    __field(unsigned int,		rtt_us)
    __field_struct(struct sockaddr_rxrpc, srx)
    ),

    TP_fast_assign(
    __entry->server = server->debug_id;
    __entry->tx = tx;
    __entry->addr_index = addr_index;
    __entry->error = error;
    __entry->abort_code = abort_code;
    __entry->rtt_us = rtt_us;
    memcpy(&__entry->srx, rxrpc_kernel_remote_srx(alist->addrs[addr_index].peer),
    sizeof(__entry->srx));
    ),

    TP_printk("vl=%08x %s ax=%u e=%d ac=%d rtt=%d %pISpc",
    __entry->server, __entry->tx ? "tx" : "rx", __entry->addr_index,
    __entry->error, __entry->abort_code, __entry->rtt_us,
    &__entry->srx.transport)
    );

    TRACE_EVENT(afs_rotate,
    TP_PROTO(struct afs_operation *op, enum afs_rotate_trace reason, unsigned int extra),

    TP_ARGS(op, reason, extra),

    TP_STRUCT__entry(
    __field(unsigned int,		op)
    __field(unsigned int,		flags)
    __field(unsigned int,		extra)
    __field(unsigned short,		iteration)
    __field(short,			server_index)
    __field(short,			addr_index)
    __field(enum afs_rotate_trace,	reason)
    ),

    TP_fast_assign(
    __entry->op = op->debug_id;
    __entry->flags = op->flags;
    __entry->iteration = op->nr_iterations;
    __entry->server_index = op->server_index;
    __entry->addr_index = op->addr_index;
    __entry->reason = reason;
    __entry->extra = extra;
    ),

    TP_printk("OP=%08x it=%02x %s fl=%x sx=%d ax=%d ext=%d",
    __entry->op,
    __entry->iteration,
    __print_symbolic(__entry->reason, afs_rotate_traces),
    __entry->flags,
    __entry->server_index,
    __entry->addr_index,
    __entry->extra)
    );

    TRACE_EVENT(afs_make_call,
    TP_PROTO(struct afs_call *call),

    TP_ARGS(call),

    TP_STRUCT__entry(
    __field(unsigned int,		call)
    __field(bool,			is_vl)
    __field(enum afs_fs_operation,	op)
    __field_struct(struct afs_fid,	fid)
    __field_struct(struct sockaddr_rxrpc, srx)
    ),

    TP_fast_assign(
    __entry->call = call->debug_id;
    __entry->op = call->operation_ID;
    __entry->fid = call->fid;
    memcpy(&__entry->srx, rxrpc_kernel_remote_srx(call->peer),
    sizeof(__entry->srx));
    __entry->srx.srx_service = call->service_id;
    __entry->is_vl = (__entry->srx.srx_service == VL_SERVICE ||
    __entry->srx.srx_service == YFS_VL_SERVICE);
    ),

    TP_printk("c=%08x %pISpc+%u %s %llx:%llx:%x",
    __entry->call,
    &__entry->srx.transport,
    __entry->srx.srx_service,
    __entry->is_vl ?
    __print_symbolic(__entry->op, afs_vl_operations) :
    __print_symbolic(__entry->op, afs_fs_operations),
    __entry->fid.vid,
    __entry->fid.vnode,
    __entry->fid.unique)
    );

    TRACE_EVENT(afs_read_recv,
    TP_PROTO(const struct afs_operation *op, const struct afs_call *call),

    TP_ARGS(op, call),

    TP_STRUCT__entry(
    __field(unsigned int,		rreq)
    __field(unsigned int,		sreq)
    __field(unsigned int,		op)
    __field(unsigned int,		op_flags)
    __field(unsigned int,		call)
    __field(enum afs_call_state,	call_state)
    ),

    TP_fast_assign(
    __entry->op = op->debug_id;
    __entry->sreq = op->fetch.subreq->debug_index;
    __entry->rreq = op->fetch.subreq->rreq->debug_id;
    __entry->op_flags = op->flags;
    __entry->call = call->debug_id;
    __entry->call_state = call->state;
    ),

    TP_printk("R=%08x[%x] OP=%08x c=%08x cs=%x of=%x",
    __entry->rreq, __entry->sreq,
    __entry->op,
    __entry->call, __entry->call_state,
    __entry->op_flags)
    );

// This part must be outside protection
