//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/smipcie/smipcie.h
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
// SMI PCIe driver for DVBSky cards.
//
// Copyright (C) 2014 Max nibble <nibble.max@gmail.com>
//

// -------- Register Base --------
pub const MSI_CONTROL_REG_BASE: c_uint = 0x0800;
pub const SYSTEM_CONTROL_REG_BASE: c_uint = 0x0880;
pub const PCIE_EP_DEBUG_REG_BASE: c_uint = 0x08C0;
pub const IR_CONTROL_REG_BASE: c_uint = 0x0900;
pub const I2C_A_CONTROL_REG_BASE: c_uint = 0x0940;
pub const I2C_B_CONTROL_REG_BASE: c_uint = 0x0980;
pub const ATV_PORTA_CONTROL_REG_BASE: c_uint = 0x09C0;
pub const DTV_PORTA_CONTROL_REG_BASE: c_uint = 0x0A00;
pub const AES_PORTA_CONTROL_REG_BASE: c_uint = 0x0A80;
pub const DMA_PORTA_CONTROL_REG_BASE: c_uint = 0x0AC0;
pub const ATV_PORTB_CONTROL_REG_BASE: c_uint = 0x0B00;
pub const DTV_PORTB_CONTROL_REG_BASE: c_uint = 0x0B40;
pub const AES_PORTB_CONTROL_REG_BASE: c_uint = 0x0BC0;
pub const DMA_PORTB_CONTROL_REG_BASE: c_uint = 0x0C00;
pub const UART_A_REGISTER_BASE: c_uint = 0x0C40;
pub const UART_B_REGISTER_BASE: c_uint = 0x0C80;
pub const GPS_CONTROL_REG_BASE: c_uint = 0x0CC0;
pub const DMA_PORTC_CONTROL_REG_BASE: c_uint = 0x0D00;
pub const DMA_PORTD_CONTROL_REG_BASE: c_uint = 0x0D00;
pub const AES_RANDOM_DATA_BASE: c_uint = 0x0D80;
pub const AES_KEY_IN_BASE: c_uint = 0x0D90;
pub const RANDOM_DATA_LIB_BASE: c_uint = 0x0E00;
pub const IR_DATA_BUFFER_BASE: c_uint = 0x0F00;
pub const PORTA_TS_BUFFER_BASE: c_uint = 0x1000;
pub const PORTA_I2S_BUFFER_BASE: c_uint = 0x1400;
pub const PORTB_TS_BUFFER_BASE: c_uint = 0x1800;
pub const PORTB_I2S_BUFFER_BASE: c_uint = 0x1C00;
// -------- MSI control and state register --------

// -------- Hybird Controller System Control register --------

pub const rbPaMSMask: c_uint = 0x07;
pub const rbPaMSDtvNoGpio: c_uint = 0x00 /*[2:0], DTV Simple mode */;
pub const rbPaMSDtv4bitGpio: c_uint = 0x01 /*[2:0], DTV TS2 Serial mode)*/;
pub const rbPaMSDtv7bitGpio: c_uint = 0x02 /*[2:0], DTV TS0 Serial mode*/;
pub const rbPaMS8bitGpio: c_uint = 0x03 /*[2:0], GPIO mode selected;(8bit GPIO)*/;
pub const rbPaMSAtv: c_uint = 0x04 /*[2:0], 3'b1xx: ATV mode select*/;
pub const rbPbMSMask: c_uint = 0x38;
pub const rbPbMSDtvNoGpio: c_uint = 0x00 /*[5:3], DTV Simple mode */;
pub const rbPbMSDtv4bitGpio: c_uint = 0x08 /*[5:3], DTV TS2 Serial mode*/;
pub const rbPbMSDtv7bitGpio: c_uint = 0x10 /*[5:3], DTV TS0 Serial mode*/;
pub const rbPbMS8bitGpio: c_uint = 0x18 /*[5:3], GPIO mode selected;(8bit GPIO)*/;
pub const rbPbMSAtv: c_uint = 0x20 /*[5:3], 3'b1xx: ATV mode select*/;
pub const rbPaAESEN: c_uint = 0x40 /*[6], port A AES enable bit*/;
pub const rbPbAESEN: c_uint = 0x80 /*[7], port B AES enable bit*/;

// -------- IR Control register --------

pub const rbIRen: c_uint = 0x80;
pub const rbIRhighidle: c_uint = 0x10;
pub const rbIRlowidle: c_uint = 0x00;
pub const rbIRVld: c_uint = 0x04;
// -------- I2C A control and state register --------

// -------- I2C B control and state register --------

// -------- Digital TV control register, Port A --------

// -------- DMA Control Register, Port A  --------

// -------- Digital TV control register, Port B --------

// -------- AES control register, Port B --------

// -------- DMA Control Register, Port B  --------

// -------- Macro define of 24 interrupt resource --------

// software I2C bit mask
pub const SW_I2C_MSK_MODE: c_uint = 0x01;
pub const SW_I2C_MSK_CLK_OUT: c_uint = 0x02;
pub const SW_I2C_MSK_DAT_OUT: c_uint = 0x04;
pub const SW_I2C_MSK_CLK_EN: c_uint = 0x08;
pub const SW_I2C_MSK_DAT_EN: c_uint = 0x10;
pub const SW_I2C_MSK_DAT_IN: c_uint = 0x40;
pub const SW_I2C_MSK_CLK_IN: c_uint = 0x80;
pub const SMI_VID: c_uint = 0x1ADE;
pub const SMI_PID: c_uint = 0x3038;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smi_cfg_info {
pub const SMI_DVBSKY_S952: c_int = 0;
pub const SMI_DVBSKY_S950: c_int = 1;
pub const SMI_DVBSKY_T9580: c_int = 2;
pub const SMI_DVBSKY_T982: c_int = 3;
pub const SMI_TECHNOTREND_S2_4200: c_int = 4;
    pub type: c_int,
    pub name: *mut c_char,
pub const SMI_TS_NULL: c_int = 0;
pub const SMI_TS_DMA_SINGLE: c_int = 1;
pub const SMI_TS_DMA_BOTH: c_int = 3;
// SMI_TS_NULL: not use;
// SMI_TS_DMA_SINGLE: use DMA 0 only;
// SMI_TS_DMA_BOTH:use DMA 0 and 1.
    pub ts_0: c_int,
    pub ts_1: c_int,
pub const DVBSKY_FE_NULL: c_int = 0;
pub const DVBSKY_FE_M88RS6000: c_int = 1;
pub const DVBSKY_FE_M88DS3103: c_int = 2;
pub const DVBSKY_FE_SIT2: c_int = 3;
    pub fe_0: c_int,
    pub fe_1: c_int,
    pub rc_map: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smi_rc {
    pub dev: *mut smi_dev,
    pub rc_dev: *mut rc_dev,
    pub input_phys: [c_char; 64],
    pub device_name: [c_char; 64],
    pub irData: [u8; 256],
    pub users: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smi_port {
    pub dev: *mut smi_dev,
    pub idx: c_int,
    pub enable: c_int,
    pub fe_type: c_int,
// regs
    pub DMA_CHAN0_ADDR_LOW: u32,
    pub DMA_CHAN0_ADDR_HI: u32,
    pub DMA_CHAN0_TRANS_STATE: u32,
    pub DMA_CHAN0_CONTROL: u32,
    pub DMA_CHAN1_ADDR_LOW: u32,
    pub DMA_CHAN1_ADDR_HI: u32,
    pub DMA_CHAN1_TRANS_STATE: u32,
    pub DMA_CHAN1_CONTROL: u32,
    pub DMA_MANAGEMENT: u32,
// dma
    pub dma_addr: [dma_addr_t; 2],
    pub cpu_addr: [*mut u8; 2],
    pub _dmaInterruptCH0: u32,
    pub _dmaInterruptCH1: u32,
    pub _int_status: u32,
    pub bh_work: work_struct,
// dvb
    pub hw_frontend: dmx_frontend,
    pub mem_frontend: dmx_frontend,
    pub dmxdev: dmxdev,
    pub dvb_adapter: dvb_adapter,
    pub demux: dvb_demux,
    pub dvbnet: dvb_net,
    pub users: c_int,
    pub fe: *mut dvb_frontend,
// frontend i2c module
    pub i2c_client_demod: *mut i2c_client,
    pub i2c_client_tuner: *mut i2c_client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smi_dev {
    pub nr: c_int,
    pub info: *mut smi_cfg_info,
// pcie
    pub pci_dev: *mut pci_dev,
    pub lmmio: *mut u32 __iomem,
// ts port
    pub ts_port: [smi_port; 2],
// i2c
    pub i2c_bus: [i2c_adapter; 2],
    pub i2c_bit: [i2c_algo_bit_data; 2],
// ir
    pub ir: smi_rc,
}

extern "C" {
    pub fn smi_ir_irq(ir: *mut smi_rc, int_status: u32) -> c_int;
}
extern "C" {
    pub fn smi_ir_start(ir: *mut smi_rc);
}
extern "C" {
    pub fn smi_ir_exit(dev: *mut smi_dev);
}
extern "C" {
    pub fn smi_ir_init(dev: *mut smi_dev) -> c_int;
}
