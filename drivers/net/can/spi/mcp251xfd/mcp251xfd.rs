//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/spi/mcp251xfd/mcp251xfd.h
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
// mcp251xfd - Microchip MCP251xFD Family CAN controller driver
//
// Copyright (c) 2019, 2020, 2021, 2023 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (c) 2019 Martin Sperl <kernel@martin.sperl.org>
//

// MPC251x registers
// CAN FD Controller Module SFR
pub const MCP251XFD_REG_CON: c_uint = 0x00;

pub const MCP251XFD_REG_CON_MODE_MIXED: c_int = 0;
pub const MCP251XFD_REG_CON_MODE_SLEEP: c_int = 1;
pub const MCP251XFD_REG_CON_MODE_INT_LOOPBACK: c_int = 2;
pub const MCP251XFD_REG_CON_MODE_LISTENONLY: c_int = 3;
pub const MCP251XFD_REG_CON_MODE_CONFIG: c_int = 4;
pub const MCP251XFD_REG_CON_MODE_EXT_LOOPBACK: c_int = 5;
pub const MCP251XFD_REG_CON_MODE_CAN2_0: c_int = 6;
pub const MCP251XFD_REG_CON_MODE_RESTRICTED: c_int = 7;

pub const MCP251XFD_REG_CON_WFT_T00FILTER: c_uint = 0x0;
pub const MCP251XFD_REG_CON_WFT_T01FILTER: c_uint = 0x1;
pub const MCP251XFD_REG_CON_WFT_T10FILTER: c_uint = 0x2;
pub const MCP251XFD_REG_CON_WFT_T11FILTER: c_uint = 0x3;

pub const MCP251XFD_REG_NBTCFG: c_uint = 0x04;

pub const MCP251XFD_REG_DBTCFG: c_uint = 0x08;

pub const MCP251XFD_REG_TDC: c_uint = 0x0c;

pub const MCP251XFD_REG_TDC_TDCMOD_AUTO: c_int = 2;
pub const MCP251XFD_REG_TDC_TDCMOD_MANUAL: c_int = 1;
pub const MCP251XFD_REG_TDC_TDCMOD_DISABLED: c_int = 0;

pub const MCP251XFD_REG_TBC: c_uint = 0x10;
pub const MCP251XFD_REG_TSCON: c_uint = 0x14;

pub const MCP251XFD_REG_VEC: c_uint = 0x18;

pub const MCP251XFD_REG_INT: c_uint = 0x1c;

// These IRQ flags must be cleared by SW in the CAN_INT register

pub const MCP251XFD_REG_RXIF: c_uint = 0x20;
pub const MCP251XFD_REG_TXIF: c_uint = 0x24;
pub const MCP251XFD_REG_RXOVIF: c_uint = 0x28;
pub const MCP251XFD_REG_TXATIF: c_uint = 0x2c;
pub const MCP251XFD_REG_TXREQ: c_uint = 0x30;
pub const MCP251XFD_REG_TREC: c_uint = 0x34;

pub const MCP251XFD_REG_BDIAG0: c_uint = 0x38;

pub const MCP251XFD_REG_BDIAG1: c_uint = 0x3c;

pub const MCP251XFD_REG_TEFCON: c_uint = 0x40;

pub const MCP251XFD_REG_TEFSTA: c_uint = 0x44;

pub const MCP251XFD_REG_TEFUA: c_uint = 0x48;
pub const MCP251XFD_REG_TXQCON: c_uint = 0x50;

pub const MCP251XFD_REG_TXQCON_PLSIZE_8: c_int = 0;
pub const MCP251XFD_REG_TXQCON_PLSIZE_12: c_int = 1;
pub const MCP251XFD_REG_TXQCON_PLSIZE_16: c_int = 2;
pub const MCP251XFD_REG_TXQCON_PLSIZE_20: c_int = 3;
pub const MCP251XFD_REG_TXQCON_PLSIZE_24: c_int = 4;
pub const MCP251XFD_REG_TXQCON_PLSIZE_32: c_int = 5;
pub const MCP251XFD_REG_TXQCON_PLSIZE_48: c_int = 6;
pub const MCP251XFD_REG_TXQCON_PLSIZE_64: c_int = 7;

pub const MCP251XFD_REG_TXQCON_TXAT_UNLIMITED: c_int = 3;
pub const MCP251XFD_REG_TXQCON_TXAT_THREE_SHOT: c_int = 1;
pub const MCP251XFD_REG_TXQCON_TXAT_ONE_SHOT: c_int = 0;

pub const MCP251XFD_REG_TXQSTA: c_uint = 0x54;

pub const MCP251XFD_REG_TXQUA: c_uint = 0x58;

pub const MCP251XFD_REG_FIFOCON_PLSIZE_8: c_int = 0;
pub const MCP251XFD_REG_FIFOCON_PLSIZE_12: c_int = 1;
pub const MCP251XFD_REG_FIFOCON_PLSIZE_16: c_int = 2;
pub const MCP251XFD_REG_FIFOCON_PLSIZE_20: c_int = 3;
pub const MCP251XFD_REG_FIFOCON_PLSIZE_24: c_int = 4;
pub const MCP251XFD_REG_FIFOCON_PLSIZE_32: c_int = 5;
pub const MCP251XFD_REG_FIFOCON_PLSIZE_48: c_int = 6;
pub const MCP251XFD_REG_FIFOCON_PLSIZE_64: c_int = 7;

pub const MCP251XFD_REG_FIFOCON_TXAT_ONE_SHOT: c_int = 0;
pub const MCP251XFD_REG_FIFOCON_TXAT_THREE_SHOT: c_int = 1;
pub const MCP251XFD_REG_FIFOCON_TXAT_UNLIMITED: c_int = 3;

// RAM
pub const MCP251XFD_RAM_START: c_uint = 0x400;

// Message Object

// MCP2517/18FD SFR
pub const MCP251XFD_REG_OSC: c_uint = 0xe00;

pub const MCP251XFD_REG_OSC_CLKODIV_10: c_int = 3;
pub const MCP251XFD_REG_OSC_CLKODIV_4: c_int = 2;
pub const MCP251XFD_REG_OSC_CLKODIV_2: c_int = 1;
pub const MCP251XFD_REG_OSC_CLKODIV_1: c_int = 0;

pub const MCP251XFD_REG_IOCON: c_uint = 0xe04;

pub const MCP251XFD_REG_CRC: c_uint = 0xe08;

pub const MCP251XFD_REG_ECCCON: c_uint = 0xe0c;

pub const MCP251XFD_REG_ECCSTAT: c_uint = 0xe10;

pub const MCP251XFD_REG_DEVID: c_uint = 0xe14	/* MCP2518FD only */;

// SPI commands
pub const MCP251XFD_SPI_INSTRUCTION_RESET: c_uint = 0x0000;
pub const MCP251XFD_SPI_INSTRUCTION_WRITE: c_uint = 0x2000;
pub const MCP251XFD_SPI_INSTRUCTION_READ: c_uint = 0x3000;
pub const MCP251XFD_SPI_INSTRUCTION_WRITE_CRC: c_uint = 0xa000;
pub const MCP251XFD_SPI_INSTRUCTION_READ_CRC: c_uint = 0xb000;
pub const MCP251XFD_SPI_INSTRUCTION_WRITE_CRC_SAFE: c_uint = 0xc000;

pub const MCP251XFD_SYSCLOCK_HZ_MAX: c_int = 40000000;
pub const MCP251XFD_SYSCLOCK_HZ_MIN: c_int = 1000000;
pub const MCP251XFD_SPICLOCK_HZ_MAX: c_int = 20000000;
pub const MCP251XFD_TIMESTAMP_WORK_DELAY_SEC: c_int = 45;
pub const MCP251XFD_OSC_PLL_MULTIPLIER: c_int = 10;

// Misc
pub const MCP251XFD_NAPI_WEIGHT: c_int = 32;
pub const MCP251XFD_SOFTRESET_RETRIES_MAX: c_int = 3;
pub const MCP251XFD_READ_CRC_RETRIES_MAX: c_int = 3;
pub const MCP251XFD_ECC_CNT_MAX: c_int = 2;
pub const MCP251XFD_SANITIZE_SPI: c_int = 1;
pub const MCP251XFD_SANITIZE_CAN: c_int = 1;
// FIFO and Ring

// Silence TX MAB overflow warnings

// Use CRC to access registers

// Use CRC to access RX/TEF-RAM

// Use CRC to access TX-RAM

// Enable ECC for RAM

// Use Half Duplex SPI transfers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_hw_tef_obj {
    pub id: u32,
    pub flags: u32,
    pub ts: u32,
}

// The tx_obj_raw version is used in spi async, i.e. without
// regmap. We have to take care of endianness ourselves.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_hw_tx_obj_can {
    pub id: u32,
    pub flags: u32,
    pub data)]: u8 data[sizeof_field(struct can_frame,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_hw_tx_obj_canfd {
    pub id: u32,
    pub flags: u32,
    pub data)]: u8 data[sizeof_field(struct canfd_frame,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_hw_rx_obj_can {
    pub id: u32,
    pub flags: u32,
    pub ts: u32,
    pub data)]: u8 data[sizeof_field(struct can_frame,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_hw_rx_obj_canfd {
    pub id: u32,
    pub flags: u32,
    pub ts: u32,
    pub data)]: u8 data[sizeof_field(struct canfd_frame,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mcp251xfd_tx_obj_load_buf {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub cmd: mcp251xfd_buf_cmd,
    pub hw_tx_obj: mcp251xfd_hw_tx_obj_raw,
    pub nocrc: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub cmd: mcp251xfd_buf_cmd_crc,
    pub hw_tx_obj: mcp251xfd_hw_tx_obj_raw,
    pub crc: __be16,
    pub crc: },
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union mcp251xfd_write_reg_buf {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub cmd: mcp251xfd_buf_cmd,
    pub data: [u8; 4],
    pub nocrc: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub cmd: mcp251xfd_buf_cmd_crc,
    pub data: [u8; 4],
    pub crc: __be16,
    pub crc: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub cmd: mcp251xfd_buf_cmd,
    pub data: [u8; 1],
    pub crc: __be16,
    pub safe: },
    pub ____cacheline_aligned: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_tx_obj {
    pub msg: spi_message,
    pub xfer: [spi_transfer; 2],
    pub buf: mcp251xfd_tx_obj_load_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_tef_ring {
    pub head: c_uint,
    pub tail: c_uint,
// u8 obj_num equals tx_ring->obj_num
// u8 obj_size equals sizeof(struct mcp251xfd_hw_tef_obj)
// u8 obj_num_shift_to_u8 equals tx_ring->obj_num_shift_to_u8
    pub irq_enable_buf: mcp251xfd_write_reg_buf,
    pub irq_enable_xfer: spi_transfer,
    pub irq_enable_msg: spi_message,
    pub uinc_buf: mcp251xfd_write_reg_buf,
    pub uinc_irq_disable_buf: mcp251xfd_write_reg_buf,
    pub uinc_xfer: [spi_transfer; MCP251XFD_TX_OBJ_NUM_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_tx_ring {
    pub head: c_uint,
    pub tail: c_uint,
    pub base: u16,
    pub nr: u8,
    pub fifo_nr: u8,
    pub obj_num: u8,
    pub obj_num_shift_to_u8: u8,
    pub obj_size: u8,
    pub obj: [mcp251xfd_tx_obj; MCP251XFD_TX_OBJ_NUM_MAX],
    pub rts_buf: mcp251xfd_write_reg_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_rx_ring {
    pub head: c_uint,
    pub tail: c_uint,
// timestamp of the last valid received CAN frame
    pub last_valid: u64,
    pub base: u16,
    pub nr: u8,
    pub fifo_nr: u8,
    pub obj_num: u8,
    pub obj_num_shift_to_u8: u8,
    pub obj_size: u8,
    pub irq_enable_buf: mcp251xfd_write_reg_buf,
    pub irq_enable_xfer: spi_transfer,
    pub irq_enable_msg: spi_message,
    pub uinc_buf: mcp251xfd_write_reg_buf,
    pub uinc_irq_disable_buf: mcp251xfd_write_reg_buf,
    pub uinc_xfer: [spi_transfer; MCP251XFD_FIFO_DEPTH],
    pub obj: [mcp251xfd_hw_rx_obj_canfd; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_ecc {
    pub ecc_stat: u32,
    pub cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_regs_status {
    pub intf: u32,
    pub rxif: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcp251xfd_model {
    MCP251XFD_MODEL_MCP2517FD = 0x2517,
    MCP251XFD_MODEL_MCP2518FD = 0x2518,
    MCP251XFD_MODEL_MCP251863 = 0x251863,
    MCP251XFD_MODEL_MCP251XFD = 0xffffffff,	/* autodetect model */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_devtype_data {
    pub model: mcp251xfd_model,
    pub quirks: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcp251xfd_flags {
    MCP251XFD_FLAGS_DOWN,
    MCP251XFD_FLAGS_FD_MODE,

    __MCP251XFD_FLAGS_SIZE__
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp251xfd_priv {
    pub can: can_priv,
    pub offload: can_rx_offload,
    pub ndev: *mut net_device,
    pub /: *mut *mut *mut regmap map_reg; / register access,
    pub /: *mut *mut *mut regmap map_rx; / RX/TEF RAM access,
    pub map_nocrc: *mut regmap,
    pub map_buf_nocrc_rx: *mut mcp251xfd_map_buf_nocrc,
    pub map_buf_nocrc_tx: *mut mcp251xfd_map_buf_nocrc,
    pub map_crc: *mut regmap,
    pub map_buf_crc_rx: *mut mcp251xfd_map_buf_crc,
    pub map_buf_crc_tx: *mut mcp251xfd_map_buf_crc,
    pub spi: *mut spi_device,
    pub spi_max_speed_hz_orig: u32,
    pub spi_max_speed_hz_fast: u32,
    pub spi_max_speed_hz_slow: u32,
    pub tef: [mcp251xfd_tef_ring; MCP251XFD_FIFO_TEF_NUM],
    pub rx: [*mut mcp251xfd_rx_ring; MCP251XFD_FIFO_RX_NUM],
    pub tx: [mcp251xfd_tx_ring; MCP251XFD_FIFO_TX_NUM],
    pub wq: *mut workqueue_struct,
    pub tx_work: work_struct,
    pub tx_work_obj: *mut mcp251xfd_tx_obj,
    pub __MCP251XFD_FLAGS_SIZE__): DECLARE_BITMAP(flags,,
    pub rx_ring_num: u8,
    pub rx_obj_num: u8,
    pub rx_obj_num_coalesce_irq: u8,
    pub tx_obj_num_coalesce_irq: u8,
    pub rx_coalesce_usecs_irq: u32,
    pub tx_coalesce_usecs_irq: u32,
    pub rx_irq_timer: hrtimer,
    pub tx_irq_timer: hrtimer,
    pub ecc: mcp251xfd_ecc,
    pub regs_status: mcp251xfd_regs_status,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub timestamp: delayed_work,
    pub rx_int: *mut gpio_desc,
    pub clk: *mut clk,
    pub pll_enable: bool,
    pub xstbyen: bool,
    pub reg_vdd: *mut regulator,
    pub reg_xceiver: *mut regulator,
    pub devtype_data: mcp251xfd_devtype_data,
    pub bec: can_berr_counter,
    pub gc: gpio_chip,
}

// listen-only mode works like FD mode
extern "C" {
    pub fn cpu_to_be16(_arg: MCP251XFD_SPI_INSTRUCTION_RESET) -> return;
}
extern "C" {
    pub fn regmap_reg_in_range(_arg: reg, _arg: &range) -> return;
}
// Number of u32 for RAM access, number of u8 otherwise.
extern "C" {
    pub fn regmap_read(_arg: priv->map_reg, _arg: MCP251XFD_REG_TBC, _arg: ts_raw) -> return;
}
// tx_tail = FIELD_GET(MCP251XFD_REG_FIFOSTA_FIFOCI_MASK, fifo_sta);
extern "C" {
    pub fn min_t(_arg: u8, _arg: len, mcp251xfd_get_tef_tail(priv): priv->tx->obj_num -) -> return;
}
// nr = (addr - mcp251xfd_get_tx_obj_addr(tx_ring, 0))
extern "C" {
    pub fn min_t(_arg: u8, _arg: len, mcp251xfd_get_rx_tail(ring): ring->obj_num -) -> return;
}

extern "C" {
    pub fn mcp251xfd_chip_fifo_init(priv: *const mcp251xfd_priv) -> c_int;
}
extern "C" {
    pub fn mcp251xfd_crc16_compute(data: *const c_void, data_size: usize) -> u16;
}
extern "C" {
    pub fn mcp251xfd_ethtool_init(priv: *mut mcp251xfd_priv);
}
extern "C" {
    pub fn mcp251xfd_regmap_init(priv: *mut mcp251xfd_priv) -> c_int;
}
extern "C" {
    pub fn mcp251xfd_ring_init(priv: *mut mcp251xfd_priv) -> c_int;
}
extern "C" {
    pub fn mcp251xfd_ring_free(priv: *mut mcp251xfd_priv);
}
extern "C" {
    pub fn mcp251xfd_ring_alloc(priv: *mut mcp251xfd_priv) -> c_int;
}
extern "C" {
    pub fn mcp251xfd_handle_rxif(priv: *mut mcp251xfd_priv) -> c_int;
}
extern "C" {
    pub fn mcp251xfd_handle_tefif(priv: *mut mcp251xfd_priv) -> c_int;
}
extern "C" {
    pub fn mcp251xfd_timestamp_init(priv: *mut mcp251xfd_priv);
}
extern "C" {
    pub fn mcp251xfd_timestamp_start(priv: *mut mcp251xfd_priv);
}
extern "C" {
    pub fn mcp251xfd_timestamp_stop(priv: *mut mcp251xfd_priv);
}
extern "C" {
    pub fn mcp251xfd_tx_obj_write_sync(work: *mut work_struct);
}

extern "C" {
    pub fn mcp251xfd_dump(priv: *const mcp251xfd_priv);
}

