//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/rc/ite-cir.h
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
// Driver for ITE Tech Inc. IT8712F/IT8512F CIR
//
// Copyright (C) 2010 Juan Jesús García de Soria <skandalfo@gmail.com>
//
// platform driver name to register

// FIFO sizes
pub const ITE_TX_FIFO_LEN: c_int = 32;
pub const ITE_RX_FIFO_LEN: c_int = 32;
// interrupt types
pub const ITE_IRQ_TX_FIFO: c_int = 1;
pub const ITE_IRQ_RX_FIFO: c_int = 2;
pub const ITE_IRQ_RX_FIFO_OVERRUN: c_int = 4;
// forward declaration
// struct for storing the parameters of different recognized devices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ite_dev_params {
// model of the device
    pub model: *const c_char,
// size of the I/O region
    pub io_region_size: c_int,
// IR pnp I/O resource number
    pub io_rsrc_no: c_int,
// hw-specific operation function pointers; most of these must be
// called while holding the spin lock, except for the TX FIFO length
// one
// get pending interrupt causes
    pub dev): *mut *mut int (get_irq_causes) (struct ite_dev,
// enable rx
    pub dev): *mut *mut void (enable_rx) (struct ite_dev,
// make rx enter the idle state; keep listening for a pulse, but stop
// streaming space bytes
    pub dev): *mut *mut void (idle_rx) (struct ite_dev,
// disable rx completely
    pub dev): *mut *mut void (disable_rx) (struct ite_dev,
// read bytes from RX FIFO; return read count
    pub buf_size): *mut *mut *mut *mut int (get_rx_bytes) (struct ite_dev dev, u8 buf, int,
// enable tx FIFO space available interrupt
    pub dev): *mut *mut void (enable_tx_interrupt) (struct ite_dev,
// disable tx FIFO space available interrupt
    pub dev): *mut *mut void (disable_tx_interrupt) (struct ite_dev,
// get number of full TX FIFO slots
    pub dev): *mut *mut int (get_tx_used_slots) (struct ite_dev,
// put a byte to the TX FIFO
    pub value): *mut *mut *mut void (put_tx_byte) (struct ite_dev dev, u8,
// disable hardware completely
    pub dev): *mut *mut void (disable) (struct ite_dev,
// initialize the hardware
    pub dev): *mut *mut void (init_hardware) (struct ite_dev,
// set the carrier parameters
    pub pulse_width_bits): u8 allowance_bits, u8,
}

// ITE CIR device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ite_dev {
    pub pdev: *mut pnp_dev,
    pub rdev: *mut rc_dev,
// sync data
    pub lock: spinlock_t,
    pub transmitting: bool,
// transmit support
    pub tx_ended: wait_queue_head_t tx_queue,,
// rx low carrier frequency, in Hz, 0 means no demodulation
    pub rx_low_carrier_freq: c_uint,
// tx high carrier frequency, in Hz, 0 means no demodulation
    pub rx_high_carrier_freq: c_uint,
// tx carrier frequency, in Hz
    pub tx_carrier_freq: c_uint,
// duty cycle, 0-100
    pub tx_duty_cycle: c_int,
// hardware I/O settings
    pub cir_addr: c_ulong,
    pub cir_irq: c_int,
// overridable copy of model parameters
    pub params: *const ite_dev_params,
}

// common values for all kinds of hardware
// baud rate divisor default
pub const ITE_BAUDRATE_DIVISOR: c_int = 1;
// low-speed carrier frequency limits (Hz)
pub const ITE_LCF_MIN_CARRIER_FREQ: c_int = 27000;
pub const ITE_LCF_MAX_CARRIER_FREQ: c_int = 58000;
// high-speed carrier frequency limits (Hz)
pub const ITE_HCF_MIN_CARRIER_FREQ: c_int = 400000;
pub const ITE_HCF_MAX_CARRIER_FREQ: c_int = 500000;
// default carrier freq for when demodulator is off (Hz)
pub const ITE_DEFAULT_CARRIER_FREQ: c_int = 38000;
// convert bits to us

//
// n in RDCR produces a tolerance of +/- n * 6.25% around the center
// carrier frequency...
//
// From two limit frequencies, L (low) and H (high), we can get both the
// center frequency F = (L + H) / 2 and the variation from the center
// frequency A = (H - L) / (H + L). We can use this in order to honor the
// s_rx_carrier_range() call in ir-core. We'll suppose that any request
// setting L=0 means we must shut down the demodulator.
//
pub const ITE_RXDCR_PER_10000_STEP: c_int = 625;
// high speed carrier freq values
pub const ITE_CFQ_400: c_uint = 0x03;
pub const ITE_CFQ_450: c_uint = 0x08;
pub const ITE_CFQ_480: c_uint = 0x0b;
pub const ITE_CFQ_500: c_uint = 0x0d;
// values for pulse widths
pub const ITE_TXMPW_A: c_uint = 0x02;
pub const ITE_TXMPW_B: c_uint = 0x03;
pub const ITE_TXMPW_C: c_uint = 0x04;
pub const ITE_TXMPW_D: c_uint = 0x05;
pub const ITE_TXMPW_E: c_uint = 0x06;
// values for demodulator carrier range allowance
pub const ITE_RXDCR_DEFAULT: c_uint = 0x01	/* default carrier range */;
pub const ITE_RXDCR_MAX: c_uint = 0x07	/* default carrier range */;
// DR TX bits
pub const ITE_TX_PULSE: c_uint = 0x00;
pub const ITE_TX_SPACE: c_uint = 0x80;
pub const ITE_TX_MAX_RLE: c_uint = 0x80;
pub const ITE_TX_RLE_MASK: c_uint = 0x7f;
//
// IT8712F
//
// hardware data obtained from:
//
// IT8712F
// Environment Control - Low Pin Count Input / Output
// (EC - LPC I/O)
// Preliminary Specification V0. 81
//
// register offsets
pub const IT87_DR: c_uint = 0x00	/* data register */;
pub const IT87_IER: c_uint = 0x01	/* interrupt enable register */;
pub const IT87_RCR: c_uint = 0x02	/* receiver control register */;
pub const IT87_TCR1: c_uint = 0x03	/* transmitter control register 1 */;
pub const IT87_TCR2: c_uint = 0x04	/* transmitter control register 2 */;
pub const IT87_TSR: c_uint = 0x05	/* transmitter status register */;
pub const IT87_RSR: c_uint = 0x06	/* receiver status register */;
pub const IT87_BDLR: c_uint = 0x05	/* baud rate divisor low byte register */;
pub const IT87_BDHR: c_uint = 0x06	/* baud rate divisor high byte register */;
pub const IT87_IIR: c_uint = 0x07	/* interrupt identification register */;
pub const IT87_IOREG_LENGTH: c_uint = 0x08	/* length of register file */;
// IER bits
pub const IT87_TLDLIE: c_uint = 0x01	/* transmitter low data interrupt enable */;
pub const IT87_RDAIE: c_uint = 0x02	/* receiver data available interrupt enable */;
pub const IT87_RFOIE: c_uint = 0x04	/* receiver FIFO overrun interrupt enable */;
pub const IT87_IEC: c_uint = 0x08	/* interrupt enable control */;
pub const IT87_BR: c_uint = 0x10	/* baud rate register enable */;
pub const IT87_RESET: c_uint = 0x20	/* reset */;
// RCR bits
pub const IT87_RXDCR: c_uint = 0x07	/* receiver demodulation carrier range mask */;
pub const IT87_RXACT: c_uint = 0x08	/* receiver active */;
pub const IT87_RXEND: c_uint = 0x10	/* receiver demodulation enable */;
pub const IT87_RXEN: c_uint = 0x20	/* receiver enable */;
pub const IT87_HCFS: c_uint = 0x40	/* high-speed carrier frequency select */;
pub const IT87_RDWOS: c_uint = 0x80	/* receiver data without sync */;
// TCR1 bits
pub const IT87_TXMPM: c_uint = 0x03	/* transmitter modulation pulse mode mask */;
pub const IT87_TXMPM_DEFAULT: c_uint = 0x00	/* modulation pulse mode default */;
pub const IT87_TXENDF: c_uint = 0x04	/* transmitter deferral */;
pub const IT87_TXRLE: c_uint = 0x08	/* transmitter run length enable */;
pub const IT87_FIFOTL: c_uint = 0x30	/* FIFO level threshold mask */;
pub const IT87_FIFOTL_DEFAULT: c_uint = 0x20	/* FIFO level threshold default;
// 0x00 -> 1, 0x10 -> 7, 0x20 -> 17,
// 0x30 -> 25
pub const IT87_ILE: c_uint = 0x40	/* internal loopback enable */;
pub const IT87_FIFOCLR: c_uint = 0x80	/* FIFO clear bit */;
// TCR2 bits
pub const IT87_TXMPW: c_uint = 0x07	/* transmitter modulation pulse width mask */;
pub const IT87_TXMPW_DEFAULT: c_uint = 0x04	/* default modulation pulse width */;
pub const IT87_CFQ: c_uint = 0xf8	/* carrier frequency mask */;

// TSR bits
pub const IT87_TXFBC: c_uint = 0x3f	/* transmitter FIFO byte count mask */;
// RSR bits
pub const IT87_RXFBC: c_uint = 0x3f	/* receiver FIFO byte count mask */;
pub const IT87_RXFTO: c_uint = 0x80	/* receiver FIFO time-out */;
// IIR bits
pub const IT87_IP: c_uint = 0x01	/* interrupt pending */;
pub const IT87_II: c_uint = 0x06	/* interrupt identification mask */;
pub const IT87_II_NOINT: c_uint = 0x00	/* no interrupt */;
pub const IT87_II_TXLDL: c_uint = 0x02	/* transmitter low data level */;
pub const IT87_II_RXDS: c_uint = 0x04	/* receiver data stored */;
pub const IT87_II_RXFO: c_uint = 0x06	/* receiver FIFO overrun */;
//
// IT8512E/F
//
// Hardware data obtained from:
//
// IT8512E/F
// Embedded Controller
// Preliminary Specification V0.4.1
//
// Note that the CIR registers are not directly available to the host, because
// they only are accessible to the integrated microcontroller. Thus, in order
// use it, some kind of bridging is required. As the bridging may depend on
// the controller firmware in use, we are going to use the PNP ID in order to
// determine the strategy and ports available. See after these generic
// IT8512E/F register definitions for register definitions for those
// strategies.
//
// register offsets
pub const IT85_C0DR: c_uint = 0x00	/* data register */;
pub const IT85_C0MSTCR: c_uint = 0x01	/* master control register */;
pub const IT85_C0IER: c_uint = 0x02	/* interrupt enable register */;
pub const IT85_C0IIR: c_uint = 0x03	/* interrupt identification register */;
pub const IT85_C0CFR: c_uint = 0x04	/* carrier frequency register */;
pub const IT85_C0RCR: c_uint = 0x05	/* receiver control register */;
pub const IT85_C0TCR: c_uint = 0x06	/* transmitter control register */;
pub const IT85_C0SCK: c_uint = 0x07	/* slow clock control register */;
pub const IT85_C0BDLR: c_uint = 0x08	/* baud rate divisor low byte register */;
pub const IT85_C0BDHR: c_uint = 0x09	/* baud rate divisor high byte register */;
pub const IT85_C0TFSR: c_uint = 0x0a	/* transmitter FIFO status register */;
pub const IT85_C0RFSR: c_uint = 0x0b	/* receiver FIFO status register */;
pub const IT85_C0WCL: c_uint = 0x0d	/* wakeup code length register */;
pub const IT85_C0WCR: c_uint = 0x0e	/* wakeup code read/write register */;
pub const IT85_C0WPS: c_uint = 0x0f	/* wakeup power control/status register */;
pub const IT85_IOREG_LENGTH: c_uint = 0x10	/* length of register file */;
// C0MSTCR bits
pub const IT85_RESET: c_uint = 0x01	/* reset */;
pub const IT85_FIFOCLR: c_uint = 0x02	/* FIFO clear bit */;
pub const IT85_FIFOTL: c_uint = 0x0c	/* FIFO level threshold mask */;
pub const IT85_FIFOTL_DEFAULT: c_uint = 0x08	/* FIFO level threshold default;
// 0x00 -> 1, 0x04 -> 7, 0x08 -> 17,
// 0x0c -> 25
pub const IT85_ILE: c_uint = 0x10	/* internal loopback enable */;
pub const IT85_ILSEL: c_uint = 0x20	/* internal loopback select */;
// C0IER bits
pub const IT85_TLDLIE: c_uint = 0x01	/* TX low data level interrupt enable */;
pub const IT85_RDAIE: c_uint = 0x02	/* RX data available interrupt enable */;
pub const IT85_RFOIE: c_uint = 0x04	/* RX FIFO overrun interrupt enable */;
pub const IT85_IEC: c_uint = 0x80	/* interrupt enable function control */;
// C0IIR bits
pub const IT85_TLDLI: c_uint = 0x01	/* transmitter low data level interrupt */;
pub const IT85_RDAI: c_uint = 0x02	/* receiver data available interrupt */;
pub const IT85_RFOI: c_uint = 0x04	/* receiver FIFO overrun interrupt */;
pub const IT85_NIP: c_uint = 0x80	/* no interrupt pending */;
// C0CFR bits
pub const IT85_CFQ: c_uint = 0x1f	/* carrier frequency mask */;
pub const IT85_HCFS: c_uint = 0x20	/* high speed carrier frequency select */;
// C0RCR bits
pub const IT85_RXDCR: c_uint = 0x07	/* receiver demodulation carrier range mask */;
pub const IT85_RXACT: c_uint = 0x08	/* receiver active */;
pub const IT85_RXEND: c_uint = 0x10	/* receiver demodulation enable */;
pub const IT85_RDWOS: c_uint = 0x20	/* receiver data without sync */;
pub const IT85_RXEN: c_uint = 0x80	/* receiver enable */;
// C0TCR bits
pub const IT85_TXMPW: c_uint = 0x07	/* transmitter modulation pulse width mask */;
pub const IT85_TXMPW_DEFAULT: c_uint = 0x04	/* default modulation pulse width */;
pub const IT85_TXMPM: c_uint = 0x18	/* transmitter modulation pulse mode mask */;
pub const IT85_TXMPM_DEFAULT: c_uint = 0x00	/* modulation pulse mode default */;
pub const IT85_TXENDF: c_uint = 0x20	/* transmitter deferral */;
pub const IT85_TXRLE: c_uint = 0x40	/* transmitter run length enable */;
// C0SCK bits
pub const IT85_SCKS: c_uint = 0x01	/* slow clock select */;
pub const IT85_TXDCKG: c_uint = 0x02	/* TXD clock gating */;
pub const IT85_DLL1P8E: c_uint = 0x04	/* DLL 1.8432M enable */;
pub const IT85_DLLTE: c_uint = 0x08	/* DLL test enable */;
pub const IT85_BRCM: c_uint = 0x70	/* baud rate count mode */;
pub const IT85_DLLOCK: c_uint = 0x80	/* DLL lock */;
// C0TFSR bits
pub const IT85_TXFBC: c_uint = 0x3f	/* transmitter FIFO count mask */;
// C0RFSR bits
pub const IT85_RXFBC: c_uint = 0x3f	/* receiver FIFO count mask */;
pub const IT85_RXFTO: c_uint = 0x80	/* receiver FIFO time-out */;
// C0WCL bits
pub const IT85_WCL: c_uint = 0x3f	/* wakeup code length mask */;
// C0WPS bits
pub const IT85_CIRPOSIE: c_uint = 0x01	/* power on/off status interrupt enable */;
pub const IT85_CIRPOIS: c_uint = 0x02	/* power on/off interrupt status */;
pub const IT85_CIRPOII: c_uint = 0x04	/* power on/off interrupt identification */;
pub const IT85_RCRST: c_uint = 0x10	/* wakeup code reading counter reset bit */;
pub const IT85_WCRST: c_uint = 0x20	/* wakeup code writing counter reset bit */;
//
// ITE8708
//
// Hardware data obtained from hacked driver for IT8512 in this forum post:
//
// http://ubuntuforums.org/showthread.php?t=1028640
//
// Although there's no official documentation for that driver, analysis would
// suggest that it maps the 16 registers of IT8512 onto two 8-register banks,
// selectable by a single bank-select bit that's mapped onto both banks. The
// IT8512 registers are mapped in a different order, so that the first bank
// maps the ones that are used more often, and two registers that share a
// reserved high-order bit are placed at the same offset in both banks in
// order to reuse the reserved bit as the bank select bit.
//
// register offsets
// mapped onto both banks
pub const IT8708_BANKSEL: c_uint = 0x07	/* bank select register */;
pub const IT8708_HRAE: c_uint = 0x80	/* high registers access enable */;
// mapped onto the low bank
pub const IT8708_C0DR: c_uint = 0x00	/* data register */;
pub const IT8708_C0MSTCR: c_uint = 0x01	/* master control register */;
pub const IT8708_C0IER: c_uint = 0x02	/* interrupt enable register */;
pub const IT8708_C0IIR: c_uint = 0x03	/* interrupt identification register */;
pub const IT8708_C0RFSR: c_uint = 0x04	/* receiver FIFO status register */;
pub const IT8708_C0RCR: c_uint = 0x05	/* receiver control register */;
pub const IT8708_C0TFSR: c_uint = 0x06	/* transmitter FIFO status register */;
pub const IT8708_C0TCR: c_uint = 0x07	/* transmitter control register */;
// mapped onto the high bank
pub const IT8708_C0BDLR: c_uint = 0x01	/* baud rate divisor low byte register */;
pub const IT8708_C0BDHR: c_uint = 0x02	/* baud rate divisor high byte register */;
pub const IT8708_C0CFR: c_uint = 0x04	/* carrier frequency register */;
// registers whose bank mapping we don't know, since they weren't being used
// in the hacked driver... most probably they belong to the high bank too,
// since they fit in the holes the other registers leave
pub const IT8708_C0SCK: c_uint = 0x03	/* slow clock control register */;
pub const IT8708_C0WCL: c_uint = 0x05	/* wakeup code length register */;
pub const IT8708_C0WCR: c_uint = 0x06	/* wakeup code read/write register */;
pub const IT8708_C0WPS: c_uint = 0x07	/* wakeup power control/status register */;
pub const IT8708_IOREG_LENGTH: c_uint = 0x08	/* length of register file */;
// two more registers that are defined in the hacked driver, but can't be
// found in the data sheets; no idea what they are or how they are accessed,
// since the hacked driver doesn't seem to use them
pub const IT8708_CSCRR: c_uint = 0x00;
pub const IT8708_CGPINTR: c_uint = 0x01;
// CSCRR bits
pub const IT8708_CSCRR_SCRB: c_uint = 0x3f;
pub const IT8708_CSCRR_PM: c_uint = 0x80;
// CGPINTR bits
pub const IT8708_CGPINT: c_uint = 0x01;
//
// ITE8709
//
// Hardware interfacing data obtained from the original lirc_ite8709 driver.
// Verbatim from its sources:
//
// The ITE8709 device seems to be the combination of IT8512 superIO chip and
// a specific firmware running on the IT8512's embedded micro-controller.
// In addition of the embedded micro-controller, the IT8512 chip contains a
// CIR module and several other modules. A few modules are directly accessible
// by the host CPU, but most of them are only accessible by the
// micro-controller. The CIR module is only accessible by the
// micro-controller.
//
// The battery-backed SRAM module is accessible by the host CPU and the
// micro-controller. So one of the MC's firmware role is to act as a bridge
// between the host CPU and the CIR module. The firmware implements a kind of
// communication protocol using the SRAM module as a shared memory. The IT8512
// specification is publicly available on ITE's web site, but the
// communication protocol is not, so it was reverse-engineered.
//
// register offsets
pub const IT8709_RAM_IDX: c_uint = 0x00	/* index into the SRAM module bytes */;
pub const IT8709_RAM_VAL: c_uint = 0x01	/* read/write data to the indexed byte */;
pub const IT8709_IOREG_LENGTH: c_uint = 0x02	/* length of register file */;
// register offsets inside the SRAM module
pub const IT8709_MODE: c_uint = 0x1a	/* request/ack byte */;
pub const IT8709_REG_IDX: c_uint = 0x1b	/* index of the CIR register to access */;
pub const IT8709_REG_VAL: c_uint = 0x1c	/* value read/to be written */;
pub const IT8709_IIR: c_uint = 0x1e	/* interrupt identification register */;
pub const IT8709_RFSR: c_uint = 0x1f	/* receiver FIFO status register */;
pub const IT8709_FIFO: c_uint = 0x20	/* start of in RAM RX FIFO copy */;
// MODE values
pub const IT8709_IDLE: c_uint = 0x00;
pub const IT8709_WRITE: c_uint = 0x01;
pub const IT8709_READ: c_uint = 0x02;
