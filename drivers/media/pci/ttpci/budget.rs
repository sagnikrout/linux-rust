//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ttpci/budget.h
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

pub const TS_SIZE: c_int = 188;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct budget_info {
    pub name: *mut c_char,
    pub type: c_int,
}

// place to store all the necessary device information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct budget {
// devices
    pub dvb_dev: dvb_device,
    pub dvb_net: dvb_net,
    pub dev: *mut saa7146_dev,
    pub i2c_adap: i2c_adapter,
    pub card: *mut budget_info,
    pub grabbing: *mut c_uchar,
    pub pt: saa7146_pgtable,
    pub fidb_bh_work: work_struct,
    pub vpe_bh_work: work_struct,
    pub dmxdev: dmxdev,
    pub demux: dvb_demux,
    pub hw_frontend: dmx_frontend,
    pub mem_frontend: dmx_frontend,
    pub ci_present: c_int,
    pub video_port: c_int,
    pub buffer_width: u32,
    pub buffer_height: u32,
    pub buffer_size: u32,
    pub buffer_warning_threshold: u32,
    pub buffer_warnings: u32,
    pub buffer_warning_time: c_ulong,
    pub ttbp: u32,
    pub feeding: c_int,
    pub feedlock: spinlock_t,
    pub debilock: spinlock_t,
    pub dvb_adapter: dvb_adapter,
    pub dvb_frontend: *mut dvb_frontend,
    pub status): *mut *mut *mut int (read_fe_status)(struct dvb_frontend fe, enum fe_status,
    pub fe_synced: c_int,
    pub priv: *mut c_void,
}

pub const BUDGET_TT: c_int = 0;
pub const BUDGET_TT_HW_DISEQC: c_int = 1;
pub const BUDGET_PATCH: c_int = 3;
pub const BUDGET_FS_ACTIVY: c_int = 4;
pub const BUDGET_CIN1200S: c_int = 5;
pub const BUDGET_CIN1200C: c_int = 6;
pub const BUDGET_CIN1200T: c_int = 7;
pub const BUDGET_KNC1S: c_int = 8;
pub const BUDGET_KNC1C: c_int = 9;
pub const BUDGET_KNC1T: c_int = 10;
pub const BUDGET_KNC1SP: c_int = 11;
pub const BUDGET_KNC1CP: c_int = 12;
pub const BUDGET_KNC1TP: c_int = 13;
pub const BUDGET_TVSTAR: c_int = 14;
pub const BUDGET_CIN1200C_MK3: c_int = 15;
pub const BUDGET_KNC1C_MK3: c_int = 16;
pub const BUDGET_KNC1CP_MK3: c_int = 17;
pub const BUDGET_KNC1S2: c_int = 18;
pub const BUDGET_KNC1C_TDA10024: c_int = 19;
pub const BUDGET_VIDEO_PORTA: c_int = 0;
pub const BUDGET_VIDEO_PORTB: c_int = 1;
extern "C" {
    pub fn ttpci_budget_init_hooks(budget: *mut budget);
}
extern "C" {
    pub fn ttpci_budget_deinit(budget: *mut budget) -> c_int;
}
extern "C" {
    pub fn ttpci_budget_irq10_handler(dev: *mut saa7146_dev, isr: *mut u32);
}
extern "C" {
    pub fn ttpci_budget_set_video_port(dev: *mut saa7146_dev, video_port: c_int);
}
