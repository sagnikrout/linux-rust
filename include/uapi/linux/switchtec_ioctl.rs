//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/switchtec_ioctl.h
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
// Microsemi Switchtec PCIe Driver
// Copyright (c) 2017, Microsemi Corporation
//
// This program is free software; you can redistribute it and/or modify it
// under the terms and conditions of the GNU General Public License,
// version 2, as published by the Free Software Foundation.
//
// This program is distributed in the hope it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//

pub const SWITCHTEC_IOCTL_PART_CFG0: c_int = 0;
pub const SWITCHTEC_IOCTL_PART_CFG1: c_int = 1;
pub const SWITCHTEC_IOCTL_PART_IMG0: c_int = 2;
pub const SWITCHTEC_IOCTL_PART_IMG1: c_int = 3;
pub const SWITCHTEC_IOCTL_PART_NVLOG: c_int = 4;
pub const SWITCHTEC_IOCTL_PART_VENDOR0: c_int = 5;
pub const SWITCHTEC_IOCTL_PART_VENDOR1: c_int = 6;
pub const SWITCHTEC_IOCTL_PART_VENDOR2: c_int = 7;
pub const SWITCHTEC_IOCTL_PART_VENDOR3: c_int = 8;
pub const SWITCHTEC_IOCTL_PART_VENDOR4: c_int = 9;
pub const SWITCHTEC_IOCTL_PART_VENDOR5: c_int = 10;
pub const SWITCHTEC_IOCTL_PART_VENDOR6: c_int = 11;
pub const SWITCHTEC_IOCTL_PART_VENDOR7: c_int = 12;
pub const SWITCHTEC_IOCTL_PART_BL2_0: c_int = 13;
pub const SWITCHTEC_IOCTL_PART_BL2_1: c_int = 14;
pub const SWITCHTEC_IOCTL_PART_MAP_0: c_int = 15;
pub const SWITCHTEC_IOCTL_PART_MAP_1: c_int = 16;
pub const SWITCHTEC_IOCTL_PART_KEY_0: c_int = 17;
pub const SWITCHTEC_IOCTL_PART_KEY_1: c_int = 18;
pub const SWITCHTEC_NUM_PARTITIONS_GEN3: c_int = 13;
pub const SWITCHTEC_NUM_PARTITIONS_GEN4: c_int = 19;
// obsolete: for compatibility with old userspace software

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_ioctl_flash_info {
    pub flash_length: __u64,
    pub num_partitions: __u32,
    pub padding: __u32,
}

pub const SWITCHTEC_IOCTL_PART_ACTIVE: c_int = 1;
pub const SWITCHTEC_IOCTL_PART_RUNNING: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_ioctl_flash_part_info {
    pub flash_partition: __u32,
    pub address: __u32,
    pub length: __u32,
    pub active: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_ioctl_event_summary_legacy {
    pub global: __u64,
    pub part_bitmap: __u64,
    pub local_part: __u32,
    pub padding: __u32,
    pub part: [__u32; 48],
    pub pff: [__u32; 48],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_ioctl_event_summary {
    pub global: __u64,
    pub part_bitmap: __u64,
    pub local_part: __u32,
    pub padding: __u32,
    pub part: [__u32; 48],
    pub pff: [__u32; 255],
}

pub const SWITCHTEC_IOCTL_EVENT_STACK_ERROR: c_int = 0;
pub const SWITCHTEC_IOCTL_EVENT_PPU_ERROR: c_int = 1;
pub const SWITCHTEC_IOCTL_EVENT_ISP_ERROR: c_int = 2;
pub const SWITCHTEC_IOCTL_EVENT_SYS_RESET: c_int = 3;
pub const SWITCHTEC_IOCTL_EVENT_FW_EXC: c_int = 4;
pub const SWITCHTEC_IOCTL_EVENT_FW_NMI: c_int = 5;
pub const SWITCHTEC_IOCTL_EVENT_FW_NON_FATAL: c_int = 6;
pub const SWITCHTEC_IOCTL_EVENT_FW_FATAL: c_int = 7;
pub const SWITCHTEC_IOCTL_EVENT_TWI_MRPC_COMP: c_int = 8;
pub const SWITCHTEC_IOCTL_EVENT_TWI_MRPC_COMP_ASYNC: c_int = 9;
pub const SWITCHTEC_IOCTL_EVENT_CLI_MRPC_COMP: c_int = 10;
pub const SWITCHTEC_IOCTL_EVENT_CLI_MRPC_COMP_ASYNC: c_int = 11;
pub const SWITCHTEC_IOCTL_EVENT_GPIO_INT: c_int = 12;
pub const SWITCHTEC_IOCTL_EVENT_PART_RESET: c_int = 13;
pub const SWITCHTEC_IOCTL_EVENT_MRPC_COMP: c_int = 14;
pub const SWITCHTEC_IOCTL_EVENT_MRPC_COMP_ASYNC: c_int = 15;
pub const SWITCHTEC_IOCTL_EVENT_DYN_PART_BIND_COMP: c_int = 16;
pub const SWITCHTEC_IOCTL_EVENT_AER_IN_P2P: c_int = 17;
pub const SWITCHTEC_IOCTL_EVENT_AER_IN_VEP: c_int = 18;
pub const SWITCHTEC_IOCTL_EVENT_DPC: c_int = 19;
pub const SWITCHTEC_IOCTL_EVENT_CTS: c_int = 20;
pub const SWITCHTEC_IOCTL_EVENT_HOTPLUG: c_int = 21;
pub const SWITCHTEC_IOCTL_EVENT_IER: c_int = 22;
pub const SWITCHTEC_IOCTL_EVENT_THRESH: c_int = 23;
pub const SWITCHTEC_IOCTL_EVENT_POWER_MGMT: c_int = 24;
pub const SWITCHTEC_IOCTL_EVENT_TLP_THROTTLING: c_int = 25;
pub const SWITCHTEC_IOCTL_EVENT_FORCE_SPEED: c_int = 26;
pub const SWITCHTEC_IOCTL_EVENT_CREDIT_TIMEOUT: c_int = 27;
pub const SWITCHTEC_IOCTL_EVENT_LINK_STATE: c_int = 28;
pub const SWITCHTEC_IOCTL_EVENT_GFMS: c_int = 29;
pub const SWITCHTEC_IOCTL_EVENT_INTERCOMM_REQ_NOTIFY: c_int = 30;
pub const SWITCHTEC_IOCTL_EVENT_UEC: c_int = 31;
pub const SWITCHTEC_IOCTL_MAX_EVENTS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_ioctl_event_ctl {
    pub event_id: __u32,
    pub index: __s32,
    pub flags: __u32,
    pub occurred: __u32,
    pub count: __u32,
    pub data: [__u32; 5],
}

pub const SWITCHTEC_IOCTL_PFF_VEP: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_ioctl_pff_port {
    pub pff: __u32,
    pub partition: __u32,
    pub port: __u32,
}

