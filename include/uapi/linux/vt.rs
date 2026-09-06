//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/vt.h
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
// These constants are also useful for user-level apps (e.g., VC
// resizing).
//

// Note: the ioctl VT_GETSTATE does not work for
// 0x56 is 'V', to avoid collision with termios and kd
pub const VT_OPENQRY: c_uint = 0x5600	/* find available vt */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_mode {
    pub /: *mut *mut __u8 mode; / vt mode,
    pub /: *mut *mut __u8 waitv; / if set, hang on writes if not active,
    pub /: *mut *mut __s16 relsig; / signal to raise on release req,
    pub /: *mut *mut __s16 acqsig; / signal to raise on acquisition,
    pub /: *mut *mut __s16 frsig; / unused (set to 0),
}

pub const VT_GETMODE: c_uint = 0x5601	/* get mode of active vt */;
pub const VT_SETMODE: c_uint = 0x5602	/* set mode of active vt */;
pub const VT_AUTO: c_uint = 0x00	/* auto vt switching */;
pub const VT_PROCESS: c_uint = 0x01	/* process controls switching */;
pub const VT_ACKACQ: c_uint = 0x02	/* acknowledge switch */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_stat {
    pub /: *mut *mut __u16 v_active; / active vt,
    pub /: *mut *mut __u16 v_signal; / signal to send,
    pub /: *mut *mut __u16 v_state; / vt bitmask,
}

pub const VT_GETSTATE: c_uint = 0x5603	/* get global vt state info */;
pub const VT_SENDSIG: c_uint = 0x5604	/* signal to send to bitmask of vts */;
pub const VT_RELDISP: c_uint = 0x5605	/* release display */;
pub const VT_ACTIVATE: c_uint = 0x5606	/* make vt active */;
pub const VT_WAITACTIVE: c_uint = 0x5607	/* wait for vt active */;
pub const VT_DISALLOCATE: c_uint = 0x5608  /* free memory associated to vt */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_sizes {
    pub /: *mut *mut __u16 v_rows; / number of rows,
    pub /: *mut *mut __u16 v_cols; / number of columns,
    pub /: *mut *mut __u16 v_scrollsize; / number of lines of scrollback,
}

pub const VT_RESIZE: c_uint = 0x5609	/* set kernel's idea of screensize */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_consize {
    pub /: *mut *mut __u16 v_rows; / number of rows,
    pub /: *mut *mut __u16 v_cols; / number of columns,
    pub /: *mut *mut __u16 v_vlin; / number of pixel rows on screen,
    pub /: *mut *mut __u16 v_clin; / number of pixel rows per character,
    pub /: *mut *mut __u16 v_vcol; / number of pixel columns on screen,
    pub /: *mut *mut __u16 v_ccol; / number of pixel columns per character,
}

pub const VT_RESIZEX: c_uint = 0x560A  /* set kernel's idea of screensize + more */;
pub const VT_LOCKSWITCH: c_uint = 0x560B  /* disallow vt switching */;
pub const VT_UNLOCKSWITCH: c_uint = 0x560C  /* allow vt switching */;
pub const VT_GETHIFONTMASK: c_uint = 0x560D  /* return hi font mask */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_event {
    pub event: __u32,
pub const VT_EVENT_SWITCH: c_uint = 0x0001	/* Console switch */;
pub const VT_EVENT_BLANK: c_uint = 0x0002	/* Screen blank */;
pub const VT_EVENT_UNBLANK: c_uint = 0x0004	/* Screen unblank */;
pub const VT_EVENT_RESIZE: c_uint = 0x0008	/* Resize display */;
pub const VT_MAX_EVENT: c_uint = 0x000F;
    pub /: *mut *mut __u32 oldev; / Old console,
    pub /: *mut *mut __u32 newev; / New console (if changing),
    pub /: *mut *mut __u32 pad[4]; / Padding for expansion,
}

pub const VT_WAITEVENT: c_uint = 0x560E	/* Wait for an event */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_setactivate {
    pub console: __u32,
    pub mode: vt_mode,
}

pub const VT_SETACTIVATE: c_uint = 0x560F	/* Activate and set the mode of a console */;
// get console size and cursor position
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_consizecsrpos {
    pub /: *mut *mut __u16 con_rows; / number of console rows,
    pub /: *mut *mut __u16 con_cols; / number of console columns,
    pub /: *mut *mut __u16 csr_row; / current cursor's row,
    pub /: *mut *mut __u16 csr_col; / current cursor's column,
}

