//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/9p.h
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

// Define EM() to export the enums to userspace via TRACE_DEFINE_ENUM()

// And also use EM/EMe to define helper enums -- once

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p9_fid_reftype {
    P9_FID_REFTYPE
    } __mode(byte);

//
// Now redefine the EM() and EMe() macros to map the enums to the strings
// that will be printed in the output.
//

    __print_symbolic(type, P9_MSG_T)

    __print_symbolic(type, P9_FID_REFTYPE)

    TRACE_EVENT(9p_client_req,
    TP_PROTO(struct p9_client *clnt, int8_t type, int tag),

    TP_ARGS(clnt, type, tag),

    TP_STRUCT__entry(
    __field(    void *,		clnt			     )
    __field(	__u8,		type			     )
    __field(	__u32,		tag			     )
    ),

    TP_fast_assign(
    __entry->clnt    =  clnt;
    __entry->type    =  type;
    __entry->tag     =  tag;
    ),

    TP_printk("client %lu request %s tag  %d",
    (long)__entry->clnt, show_9p_op(__entry->type),
    __entry->tag)
    );

    TRACE_EVENT(9p_client_res,
    TP_PROTO(struct p9_client *clnt, int8_t type, int tag, int err),

    TP_ARGS(clnt, type, tag, err),

    TP_STRUCT__entry(
    __field(    void *,		clnt			     )
    __field(	__u8,		type			     )
    __field(	__u32,		tag			     )
    __field(	__u32,		err			     )
    ),

    TP_fast_assign(
    __entry->clnt    =  clnt;
    __entry->type    =  type;
    __entry->tag     =  tag;
    __entry->err     =  err;
    ),

    TP_printk("client %lu response %s tag  %d err %d",
    (long)__entry->clnt, show_9p_op(__entry->type),
    __entry->tag, __entry->err)
    );

// dump 32 bytes of protocol data
pub const P9_PROTO_DUMP_SZ: c_int = 32;
    TRACE_EVENT(9p_protocol_dump,
    TP_PROTO(struct p9_client *clnt, struct p9_fcall *pdu),

    TP_ARGS(clnt, pdu),

    TP_STRUCT__entry(
    __field(	void *,		clnt				)
    __field(	__u8,		type				)
    __field(	__u16,		tag				)
    __dynamic_array(unsigned char, line,
    min_t(size_t, pdu->capacity, P9_PROTO_DUMP_SZ))
    ),

    TP_fast_assign(
    __entry->clnt   =  clnt;
    __entry->type   =  pdu->id;
    __entry->tag    =  pdu->tag;
    memcpy(__get_dynamic_array(line), pdu->sdata,
    __get_dynamic_array_len(line));
    ),
    TP_printk("clnt %lu %s(tag = %d)\n%*ph\n",
    (unsigned long)__entry->clnt, show_9p_op(__entry->type),
    __entry->tag, __get_dynamic_array_len(line),
    __get_dynamic_array(line))
    );


    TRACE_EVENT(9p_fid_ref,
    TP_PROTO(struct p9_fid *fid, __u8 type),

    TP_ARGS(fid, type),

    TP_STRUCT__entry(
    __field(	int,	fid		)
    __field(	int,	refcount	)
    __field(	__u8, type	)
    ),

    TP_fast_assign(
    __entry->fid = fid->fid;
    __entry->refcount = refcount_read(&fid->count);
    __entry->type = type;
    ),

    TP_printk("%s fid %d, refcount %d",
    show_9p_fid_reftype(__entry->type),
    __entry->fid, __entry->refcount)
    );

// This part must be outside protection
