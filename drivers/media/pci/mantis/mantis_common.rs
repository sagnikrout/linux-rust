//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/mantis/mantis_common.h
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
//

pub const MANTIS_ERROR: c_int = 0;
pub const MANTIS_NOTICE: c_int = 1;
pub const MANTIS_INFO: c_int = 2;
pub const MANTIS_DEBUG: c_int = 3;
pub const MANTIS_TMG: c_int = 9;

pub const MANTIS_TS_188: c_int = 0;
pub const MANTIS_TS_204: c_int = 1;
pub const TWINHAN_TECHNOLOGIES: c_uint = 0x1822;
pub const MANTIS: c_uint = 0x4e35;
pub const TECHNISAT: c_uint = 0x1ae4;
pub const TERRATEC: c_uint = 0x153b;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mantis_i2c_mode {
    MANTIS_PAGE_MODE = 0,
    MANTIS_BYTE_MODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mantis_hwconfig {
    pub model_name: *mut c_char,
    pub dev_type: *mut c_char,
    pub ts_size: u32,
    pub baud_rate: mantis_baud,
    pub parity: mantis_parity,
    pub bytes: u32,
    pub dev_id): *mut *mut irqreturn_t (irq_handler)(int irq, void,
    pub fe): *mut *mut *mut int (frontend_init)(struct mantis_pci mantis, struct dvb_frontend,
    pub power: u8,
    pub reset: u8,
    pub i2c_mode: mantis_i2c_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mantis_pci_drvdata {
    pub hwconfig: *mut mantis_hwconfig,
    pub rc_map_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mantis_pci {
    pub verbose: c_uint,
// PCI stuff
    pub vendor_id: u16,
    pub device_id: u16,
    pub subsystem_vendor: u16,
    pub subsystem_device: u16,
    pub latency: u8,
    pub pdev: *mut pci_dev,
    pub mantis_addr: c_ulong,
    pub mmio: *mut void __iomem,
    pub irq: u8,
    pub revision: u8,
    pub num: c_uint,
// RISC Core
    pub busy_block: u32,
    pub last_block: u32,
    pub buf_cpu: *mut u8,
    pub buf_dma: dma_addr_t,
    pub risc_cpu: *mut __le32,
    pub risc_dma: dma_addr_t,
    pub bh_work: work_struct,
    pub intmask_lock: spinlock_t,
    pub adapter: i2c_adapter,
    pub i2c_rc: c_int,
    pub i2c_wq: wait_queue_head_t,
    pub i2c_lock: mutex,
// DVB stuff
    pub dvb_adapter: dvb_adapter,
    pub fe: *mut dvb_frontend,
    pub demux: dvb_demux,
    pub dmxdev: dmxdev,
    pub fe_hw: dmx_frontend,
    pub fe_mem: dmx_frontend,
    pub dvbnet: dvb_net,
    pub feeds: u8,
    pub hwconfig: *mut mantis_hwconfig,
    pub mantis_int_stat: u32,
    pub mantis_int_mask: u32,
// board specific
    pub mac_address: [u8; 8],
    pub sub_vendor_id: u32,
    pub sub_device_id: u32,
// A12 A13 A14
    pub gpio_status: u32,
    pub gpif_status: u32,
    pub mantis_ca: *mut mantis_ca,
    pub uart_work: work_struct,
    pub rc: *mut rc_dev,
    pub device_name: [c_char; 80],
    pub input_phys: [c_char; 80],
    pub rc_map_name: *mut c_char,
}

