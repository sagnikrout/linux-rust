//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ddbridge/ddbridge.h
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
//
// ddbridge.h: Digital Devices PCIe bridge driver
//
// Copyright (C) 2010-2017 Digital Devices GmbH
// Ralph Metzler <rmetzler@digitaldevices.de>
//

pub const DDB_MAX_I2C: c_int = 32;
pub const DDB_MAX_PORT: c_int = 32;
pub const DDB_MAX_INPUT: c_int = 64;
pub const DDB_MAX_OUTPUT: c_int = 32;
pub const DDB_MAX_LINK: c_int = 4;
pub const DDB_LINK_SHIFT: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_regset {
    pub base: u32,
    pub num: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_regmap {
    pub irq_base_i2c: u32,
    pub irq_base_idma: u32,
    pub irq_base_odma: u32,
    pub i2c: *const ddb_regset,
    pub i2c_buf: *const ddb_regset,
    pub idma: *const ddb_regset,
    pub idma_buf: *const ddb_regset,
    pub odma: *const ddb_regset,
    pub odma_buf: *const ddb_regset,
    pub input: *const ddb_regset,
    pub output: *const ddb_regset,
    pub channel: *const ddb_regset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_ids {
    pub vendor: u16,
    pub device: u16,
    pub subvendor: u16,
    pub subdevice: u16,
    pub hwid: u32,
    pub regmapid: u32,
    pub devid: u32,
    pub mac: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_info {
    pub type: c_int,
pub const DDB_NONE: c_int = 0;
pub const DDB_OCTOPUS: c_int = 1;
pub const DDB_OCTOPUS_CI: c_int = 2;
pub const DDB_OCTOPUS_MAX: c_int = 5;
pub const DDB_OCTOPUS_MAX_CT: c_int = 6;
pub const DDB_OCTOPUS_MCI: c_int = 9;
    pub name: *mut c_char,
    pub i2c_mask: u32,
    pub board_control: u32,
    pub board_control_2: u32,
    pub port_num: u8,
    pub led_num: u8,
    pub fan_num: u8,
    pub temp_num: u8,
    pub temp_bus: u8,
    pub /: *mut *mut u8 con_clock; / use a continuous clock,
    pub ts_quirks: u8,
pub const TS_QUIRK_SERIAL: c_int = 1;
pub const TS_QUIRK_REVERSED: c_int = 2;
pub const TS_QUIRK_ALT_OSC: c_int = 8;
    pub mci_ports: u8,
    pub mci_type: u8,
    pub tempmon_irq: u32,
    pub regmap: *const ddb_regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_dma {
    pub io: *mut c_void,
    pub regs: u32,
    pub bufregs: u32,
    pub pbuf: [dma_addr_t; DMA_MAX_BUFS],
    pub vbuf: [*mut u8; DMA_MAX_BUFS],
    pub num: u32,
    pub size: u32,
    pub div: u32,
    pub bufval: u32,
    pub work: work_struct,
    pub /: *mut *mut spinlock_t lock; / DMA lock,
    pub wq: wait_queue_head_t,
    pub running: c_int,
    pub stat: u32,
    pub ctrl: u32,
    pub cbuf: u32,
    pub coff: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_dvb {
    pub adap: *mut dvb_adapter,
    pub adap_registered: c_int,
    pub dev: *mut dvb_device,
    pub i2c_client: [*mut i2c_client; 1],
    pub fe: *mut dvb_frontend,
    pub fe2: *mut dvb_frontend,
    pub dmxdev: dmxdev,
    pub demux: dvb_demux,
    pub dvbnet: dvb_net,
    pub hw_frontend: dmx_frontend,
    pub mem_frontend: dmx_frontend,
    pub users: c_int,
    pub attached: u32,
    pub input: u8,
    pub tone: fe_sec_tone_mode,
    pub voltage: fe_sec_voltage,
    pub int): *mut *mut *mut int (i2c_gate_ctrl)(struct dvb_frontend ,,
    pub voltage): fe_sec_voltage,
    pub input): *mut *mut *mut int (set_input)(struct dvb_frontend fe, int,
    pub cmd): *mut dvb_diseqc_master_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_ci {
    pub en: dvb_ca_en50221,
    pub port: *mut ddb_port,
    pub nr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_io {
    pub port: *mut ddb_port,
    pub nr: u32,
    pub regs: u32,
    pub dma: *mut ddb_dma,
    pub redo: *mut ddb_io,
    pub redi: *mut ddb_io,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_i2c {
    pub dev: *mut ddb,
    pub nr: u32,
    pub regs: u32,
    pub link: u32,
    pub adap: i2c_adapter,
    pub rbuf: u32,
    pub wbuf: u32,
    pub bsize: u32,
    pub completion: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_port {
    pub dev: *mut ddb,
    pub nr: u32,
    pub pnr: u32,
    pub regs: u32,
    pub lnr: u32,
    pub i2c: *mut ddb_i2c,
    pub /: *mut *mut mutex i2c_gate_lock; / I2C access lock,
    pub class: u32,
pub const DDB_PORT_NONE: c_int = 0;
pub const DDB_PORT_CI: c_int = 1;
pub const DDB_PORT_TUNER: c_int = 2;
pub const DDB_PORT_LOOP: c_int = 3;
    pub name: *mut c_char,
    pub type_name: *mut c_char,
    pub type: u32,
pub const DDB_TUNER_DUMMY: c_uint = 0xffffffff;
pub const DDB_TUNER_NONE: c_int = 0;
pub const DDB_TUNER_DVBS_ST: c_int = 1;
pub const DDB_TUNER_DVBS_ST_AA: c_int = 2;
pub const DDB_TUNER_DVBCT_TR: c_int = 3;
pub const DDB_TUNER_DVBCT_ST: c_int = 4;
pub const DDB_CI_INTERNAL: c_int = 5;
pub const DDB_CI_EXTERNAL_SONY: c_int = 6;
pub const DDB_TUNER_DVBCT2_SONY_P: c_int = 7;
pub const DDB_TUNER_DVBC2T2_SONY_P: c_int = 8;
pub const DDB_TUNER_ISDBT_SONY_P: c_int = 9;
pub const DDB_TUNER_DVBS_STV0910_P: c_int = 10;
pub const DDB_TUNER_MXL5XX: c_int = 11;
pub const DDB_CI_EXTERNAL_XO2: c_int = 12;
pub const DDB_CI_EXTERNAL_XO2_B: c_int = 13;
pub const DDB_TUNER_DVBS_STV0910_PR: c_int = 14;
pub const DDB_TUNER_DVBC2T2I_SONY_P: c_int = 15;
pub const DDB_TUNER_XO2: c_int = 32;

pub const DDB_TUNER_MCI: c_int = 48;
    pub input: [*mut ddb_input; 2],
    pub output: *mut ddb_output,
    pub en: *mut dvb_ca_en50221,
    pub en_freedata: u8,
    pub dvb: [ddb_dvb; 2],
    pub gap: u32,
    pub obr: u32,
    pub creg: u8,
}

pub const CM_STARTUP_DELAY: c_int = 2;
pub const CM_AVERAGE: c_int = 20;
pub const CM_GAIN: c_int = 10;
pub const HW_LSB_SHIFT: c_int = 12;
pub const HW_LSB_MASK: c_uint = 0x1000;
pub const CM_IDLE: c_int = 0;
pub const CM_STARTUP: c_int = 1;
pub const CM_ADJUST: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_lnb {
    pub /: *mut *mut mutex lock; / lock lnb access,
    pub tone: u32,
    pub oldvoltage: [fe_sec_voltage; 4],
    pub voltage: [u32; 4],
    pub voltages: u32,
    pub fmode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_irq {
    pub ): *mut *mut void (handler)(void,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_link {
    pub dev: *mut ddb,
    pub info: *const ddb_info,
    pub nr: u32,
    pub regs: u32,
    pub /: *mut *mut spinlock_t lock; / lock link access,
    pub /: *mut *mut mutex flash_mutex; / lock flash access,
    pub lnb: ddb_lnb,
    pub bh_work: work_struct,
    pub ids: ddb_ids,
    pub /: *mut *mut spinlock_t temp_lock; / lock temp chip access,
    pub overtemperature_error: c_int,
    pub temp_tab: [u8; 11],
    pub irq: [ddb_irq; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb {
    pub pdev: *mut pci_dev,
    pub pfdev: *mut platform_device,
    pub dev: *mut device,
    pub msi: c_int,
    pub wq: *mut workqueue_struct,
    pub has_dma: u32,
    pub link: [ddb_link; DDB_MAX_LINK],
    pub regs: *mut unsigned char __iomem,
    pub regs_len: u32,
    pub port_num: u32,
    pub port: [ddb_port; DDB_MAX_PORT],
    pub i2c_num: u32,
    pub i2c: [ddb_i2c; DDB_MAX_I2C],
    pub input: [ddb_input; DDB_MAX_INPUT],
    pub output: [ddb_output; DDB_MAX_OUTPUT],
    pub adap: [dvb_adapter; DDB_MAX_INPUT],
    pub idma: [ddb_dma; DDB_MAX_INPUT],
    pub odma: [ddb_dma; DDB_MAX_OUTPUT],
    pub ddb_dev: *mut device,
    pub ddb_dev_users: u32,
    pub nr: u32,
    pub iobuf: [u8; 1028],
    pub leds: u8,
    pub ts_irq: u32,
    pub i2c_irq: u32,
    pub /: *mut *mut mutex mutex; / lock access to global ddb array,
    pub tsbuf: [u8; TS_CAPTURE_LEN],
}

//
extern "C" {
    pub fn ddbridge_flashread(dev: *mut ddb, link: u32, buf: *mut u8, addr: u32, len: u32) -> c_int;
}
//
// ddbridge-core.c
extern "C" {
    pub fn ddb_ports_detach(dev: *mut ddb);
}
extern "C" {
    pub fn ddb_ports_release(dev: *mut ddb);
}
extern "C" {
    pub fn ddb_buffers_free(dev: *mut ddb);
}
extern "C" {
    pub fn ddb_device_destroy(dev: *mut ddb);
}
extern "C" {
    pub fn ddb_irq_handler0(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ddb_irq_handler1(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ddb_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ddb_ports_init(dev: *mut ddb);
}
extern "C" {
    pub fn ddb_buffers_alloc(dev: *mut ddb) -> c_int;
}
extern "C" {
    pub fn ddb_ports_attach(dev: *mut ddb) -> c_int;
}
extern "C" {
    pub fn ddb_device_create(dev: *mut ddb) -> c_int;
}
extern "C" {
    pub fn ddb_init(dev: *mut ddb) -> c_int;
}
extern "C" {
    pub fn ddb_unmap(dev: *mut ddb);
}
extern "C" {
    pub fn ddb_exit_ddbridge(stage: c_int, error: c_int) -> c_int;
}
extern "C" {
    pub fn ddb_init_ddbridge() -> c_int;
}
