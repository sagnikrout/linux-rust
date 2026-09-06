//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/chsc.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// ioctl interface for /dev/chsc
//
// Copyright IBM Corp. 2008, 2012
// Author(s): Cornelia Huck <cornelia.huck@de.ibm.com>
//

pub const CHSC_SIZE: c_uint = 0x1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_async_header {
    pub length: __u16,
    pub code: __u16,
    pub cmd_dependend: __u32,
    pub 4: __u32 key :,
    pub 28: __u32 :,
    pub sid: subchannel_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_async_area {
    pub header: chsc_async_header,
    pub chsc_async_header)]: __u8 data[CHSC_SIZE - sizeof(struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_header {
    pub length: __u16,
    pub code: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_sync_area {
    pub header: chsc_header,
    pub chsc_header)]: __u8 data[CHSC_SIZE - sizeof(struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_response_struct {
    pub length: __u16,
    pub code: __u16,
    pub parms: __u32,
    pub sizeof(__u32)]: *mut *mut __u8 data[CHSC_SIZE - 2  sizeof(__u16) -,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_chp_cd {
    pub chpid: chp_id,
    pub m: c_int,
    pub fmt: c_int,
    pub cpcb: chsc_response_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_cu_cd {
    pub cun: __u16,
    pub cssid: __u8,
    pub m: c_int,
    pub fmt: c_int,
    pub cucb: chsc_response_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_sch_cud {
    pub schid: subchannel_id,
    pub fmt: c_int,
    pub scub: chsc_response_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_id {
    pub m: c_int,
    pub cssid: __u8,
    pub ssid: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_conf_info {
    pub id: conf_id,
    pub fmt: c_int,
    pub scid: chsc_response_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccl_parm_chpid {
    pub m: c_int,
    pub chp: chp_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccl_parm_cssids {
    pub f_cssid: __u8,
    pub l_cssid: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_comp_list {
    pub ctype: },
    pub fmt: c_int,
    pub chpid: ccl_parm_chpid,
    pub cssids: ccl_parm_cssids,
    pub req: },
    pub sccl: chsc_response_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_dcal {
    pub atype: },
    pub list_parm: [__u32; 2],
    pub fmt: c_int,
    pub req: },
    pub sdcal: chsc_response_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chsc_cpd_info {
    pub chpid: chp_id,
    pub m: c_int,
    pub fmt: c_int,
    pub rfmt: c_int,
    pub c: c_int,
    pub chpdb: chsc_response_struct,
}

