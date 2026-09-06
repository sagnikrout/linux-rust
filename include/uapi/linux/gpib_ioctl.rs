//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/gpib_ioctl.h
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
// copyright            : (C) 2002 by Frank Mori Hess
//

pub const GPIB_CODE: c_int = 160;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_board_type_ioctl {
    pub name: [c_char; 100],
}

// argument for read/write/command ioctls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_read_write_ioctl {
    pub buffer_ptr: __u64,
    pub requested_transfer_count: __u32,
    pub completed_transfer_count: __u32,
    pub cmd*/: *mut *mut __s32 end; / end flag return for reads, end io suppression request for,
    pub handle: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_open_dev_ioctl {
    pub handle: __u32,
    pub pad: __u32,
    pub sad: __s32,
    pub is_board: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_close_dev_ioctl {
    pub handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_serial_poll_ioctl {
    pub pad: __u32,
    pub sad: __s32,
    pub status_byte: __u8,
    pub /: *mut *mut __u8 padding[3]; / align to 32 bit boundary,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_eos_ioctl {
    pub eos: __s32,
    pub eos_flags: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_wait_ioctl {
    pub handle: __s32,
    pub wait_mask: __s32,
    pub clear_mask: __s32,
    pub set_mask: __s32,
    pub ibsta: __s32,
    pub pad: __s32,
    pub sad: __s32,
    pub usec_timeout: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_online_ioctl {
    pub init_data_ptr: __u64,
    pub init_data_length: __s32,
    pub online: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_spoll_bytes_ioctl {
    pub num_bytes: __u32,
    pub pad: __u32,
    pub sad: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_board_info_ioctl {
    pub pad: __u32,
    pub sad: __s32,
    pub parallel_poll_configuration: __s32,
    pub autopolling: __s32,
    pub is_system_controller: __s32,
    pub t1_delay: __u32,
    pub 1: unsigned ist :,
    pub 1: unsigned no_7_bit_eos :,
    pub /: *mut *mut unsigned padding :30; / align to 32 bit boundary,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_select_pci_ioctl {
    pub pci_bus: __s32,
    pub pci_slot: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_ppoll_config_ioctl {
    pub config: __u8,
    pub 1: unsigned set_ist :,
    pub 1: unsigned clear_ist :,
    pub /: *mut *mut unsigned padding :22; / align to 32 bit boundary,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_pad_ioctl {
    pub handle: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_sad_ioctl {
    pub handle: __u32,
    pub sad: __s32,
}

// select a piece of hardware to attach by its sysfs device path
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_select_device_path_ioctl {
    pub device_path: [c_char; 0x1000],
}

// update status byte and request service
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_request_service2 {
    pub status_byte: __u8,
    pub /: *mut *mut __u8 padding[3]; / align to 32 bit boundary,
    pub new_reason_for_service: __s32,
}

// Standard functions.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpib_ioctl {
    IBRD = _IOWR(GPIB_CODE, 100, struct gpib_read_write_ioctl),
    IBWRT = _IOWR(GPIB_CODE, 101, struct gpib_read_write_ioctl),
    IBCMD = _IOWR(GPIB_CODE, 102, struct gpib_read_write_ioctl),
    IBOPENDEV = _IOWR(GPIB_CODE, 3, struct gpib_open_dev_ioctl),
    IBCLOSEDEV = _IOW(GPIB_CODE, 4, struct gpib_close_dev_ioctl),
    IBWAIT = _IOWR(GPIB_CODE, 5, struct gpib_wait_ioctl),
    IBRPP = _IOWR(GPIB_CODE, 6, __u8),

    IBSIC = _IOW(GPIB_CODE, 9, __u32),
    IBSRE = _IOW(GPIB_CODE, 10, __s32),
    IBGTS = _IO(GPIB_CODE, 11),
    IBCAC = _IOW(GPIB_CODE, 12, __s32),
    IBLINES = _IOR(GPIB_CODE, 14, __s16),
    IBPAD = _IOW(GPIB_CODE, 15, struct gpib_pad_ioctl),
    IBSAD = _IOW(GPIB_CODE, 16, struct gpib_sad_ioctl),
    IBTMO = _IOW(GPIB_CODE, 17, __u32),
    IBRSP = _IOWR(GPIB_CODE, 18, struct gpib_serial_poll_ioctl),
    IBEOS = _IOW(GPIB_CODE, 19, struct gpib_eos_ioctl),
    IBRSV = _IOW(GPIB_CODE, 20, __u8),
    CFCBASE = _IOW(GPIB_CODE, 21, __u64),
    CFCIRQ = _IOW(GPIB_CODE, 22, __u32),
    CFCDMA = _IOW(GPIB_CODE, 23, __u32),
    CFCBOARDTYPE = _IOW(GPIB_CODE, 24, struct gpib_board_type_ioctl),

    IBMUTEX = _IOW(GPIB_CODE, 26, __s32),
    IBSPOLL_BYTES = _IOWR(GPIB_CODE, 27, struct gpib_spoll_bytes_ioctl),
    IBPPC = _IOW(GPIB_CODE, 28, struct gpib_ppoll_config_ioctl),
    IBBOARD_INFO = _IOR(GPIB_CODE, 29, struct gpib_board_info_ioctl),

    IBQUERY_BOARD_RSV = _IOR(GPIB_CODE, 31, __s32),
    IBSELECT_PCI = _IOWR(GPIB_CODE, 32, struct gpib_select_pci_ioctl),
    IBEVENT = _IOR(GPIB_CODE, 33, __s16),
    IBRSC = _IOW(GPIB_CODE, 34, __s32),
    IB_T1_DELAY = _IOW(GPIB_CODE, 35, __u32),
    IBLOC = _IO(GPIB_CODE, 36),

    IBAUTOSPOLL = _IOW(GPIB_CODE, 38, __s16),
    IBONL = _IOW(GPIB_CODE, 39, struct gpib_online_ioctl),
    IBPP2_SET = _IOW(GPIB_CODE, 40, __s16),
    IBPP2_GET = _IOR(GPIB_CODE, 41, __s16),
    IBSELECT_DEVICE_PATH = _IOW(GPIB_CODE, 43, struct gpib_select_device_path_ioctl),
// 44 was IBSELECT_SERIAL_NUMBER
    IBRSV2 = _IOW(GPIB_CODE, 45, struct gpib_request_service2)
}
