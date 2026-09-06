//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/rc/ene_ir.h
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
// driver for ENE KB3926 B/C/D/E/F CIR (also known as ENE0XXX)
//
// Copyright (C) 2010 Maxim Levitsky <maximlevitsky@gmail.com>
//

// hardware address

pub const ENE_IO_SIZE: c_int = 4;
// 8 bytes of samples, divided in 2 packets
pub const ENE_FW_SAMPLE_BUFFER: c_uint = 0xF8F0	/* sample buffer */;
pub const ENE_FW_SAMPLE_SPACE: c_uint = 0x80	/* sample is space */;
pub const ENE_FW_PACKET_SIZE: c_int = 4;
// first firmware flag register
pub const ENE_FW1: c_uint = 0xF8F8  /* flagr */;
pub const ENE_FW1_ENABLE: c_uint = 0x01	/* enable fw processing */;
pub const ENE_FW1_TXIRQ: c_uint = 0x02	/* TX interrupt pending */;
pub const ENE_FW1_HAS_EXTRA_BUF: c_uint = 0x04	/* fw uses extra buffer*/;
pub const ENE_FW1_EXTRA_BUF_HND: c_uint = 0x08	/* extra buffer handshake bit*/;
pub const ENE_FW1_LED_ON: c_uint = 0x10	/* turn on a led */;
pub const ENE_FW1_WPATTERN: c_uint = 0x20	/* enable wake pattern */;
pub const ENE_FW1_WAKE: c_uint = 0x40	/* enable wake from S3 */;
pub const ENE_FW1_IRQ: c_uint = 0x80	/* enable interrupt */;
// second firmware flag register
pub const ENE_FW2: c_uint = 0xF8F9  /* flagw */;
pub const ENE_FW2_BUF_WPTR: c_uint = 0x01	/* which half of the buffer to read */;
pub const ENE_FW2_RXIRQ: c_uint = 0x04	/* RX IRQ pending*/;
pub const ENE_FW2_GP0A: c_uint = 0x08	/* Use GPIO0A for demodulated input */;
pub const ENE_FW2_EMMITER1_CONN: c_uint = 0x10	/* TX emmiter 1 connected */;
pub const ENE_FW2_EMMITER2_CONN: c_uint = 0x20	/* TX emmiter 2 connected */;
pub const ENE_FW2_FAN_INPUT: c_uint = 0x40	/* fan input used for demodulated data*/;
pub const ENE_FW2_LEARNING: c_uint = 0x80	/* hardware supports learning and TX */;
// firmware RX pointer for new style buffer
pub const ENE_FW_RX_POINTER: c_uint = 0xF8FA;
// high parts of samples for fan input (8 samples)
pub const ENE_FW_SMPL_BUF_FAN: c_uint = 0xF8FB;
pub const ENE_FW_SMPL_BUF_FAN_PLS: c_uint = 0x8000	/* combined sample is pulse */;
pub const ENE_FW_SMPL_BUF_FAN_MSK: c_uint = 0x0FFF  /* combined sample maximum value */;

// transmitter ports
pub const ENE_GPIOFS1: c_uint = 0xFC01;
pub const ENE_GPIOFS1_GPIO0D: c_uint = 0x20	/* enable tx output on GPIO0D */;
pub const ENE_GPIOFS8: c_uint = 0xFC08;
pub const ENE_GPIOFS8_GPIO41: c_uint = 0x02	/* enable tx output on GPIO40 */;
// IRQ registers block (for revision B)
pub const ENEB_IRQ: c_uint = 0xFD09	/* IRQ number */;
pub const ENEB_IRQ_UNK1: c_uint = 0xFD17	/* unknown setting = 1 */;
pub const ENEB_IRQ_STATUS: c_uint = 0xFD80	/* irq status */;
pub const ENEB_IRQ_STATUS_IR: c_uint = 0x20	/* IR irq */;
// fan as input settings
pub const ENE_FAN_AS_IN1: c_uint = 0xFE30  /* fan init reg 1 */;
pub const ENE_FAN_AS_IN1_EN: c_uint = 0xCD;
pub const ENE_FAN_AS_IN2: c_uint = 0xFE31  /* fan init reg 2 */;
pub const ENE_FAN_AS_IN2_EN: c_uint = 0x03;
// IRQ registers block (for revision C,D)
pub const ENE_IRQ: c_uint = 0xFE9B	/* new irq settings register */;
pub const ENE_IRQ_MASK: c_uint = 0x0F	/* irq number mask */;
pub const ENE_IRQ_UNK_EN: c_uint = 0x10	/* always enabled */;
pub const ENE_IRQ_STATUS: c_uint = 0x20	/* irq status and ACK */;
// CIR Config register #1
pub const ENE_CIRCFG: c_uint = 0xFEC0;
pub const ENE_CIRCFG_RX_EN: c_uint = 0x01	/* RX enable */;
pub const ENE_CIRCFG_RX_IRQ: c_uint = 0x02	/* Enable hardware interrupt */;
pub const ENE_CIRCFG_REV_POL: c_uint = 0x04	/* Input polarity reversed */;
pub const ENE_CIRCFG_CARR_DEMOD: c_uint = 0x08	/* Enable carrier demodulator */;
pub const ENE_CIRCFG_TX_EN: c_uint = 0x10	/* TX enable */;
pub const ENE_CIRCFG_TX_IRQ: c_uint = 0x20	/* Send interrupt on TX done */;
pub const ENE_CIRCFG_TX_POL_REV: c_uint = 0x40	/* TX polarity reversed */;
pub const ENE_CIRCFG_TX_CARR: c_uint = 0x80	/* send TX carrier or not */;
// CIR config register #2
pub const ENE_CIRCFG2: c_uint = 0xFEC1;
pub const ENE_CIRCFG2_RLC: c_uint = 0x00;
pub const ENE_CIRCFG2_RC5: c_uint = 0x01;
pub const ENE_CIRCFG2_RC6: c_uint = 0x02;
pub const ENE_CIRCFG2_NEC: c_uint = 0x03;
pub const ENE_CIRCFG2_CARR_DETECT: c_uint = 0x10	/* Enable carrier detection */;
pub const ENE_CIRCFG2_GPIO0A: c_uint = 0x20	/* Use GPIO0A instead of GPIO40 for input */;
pub const ENE_CIRCFG2_FAST_SAMPL1: c_uint = 0x40	/* Fast leading pulse detection for RC6 */;
pub const ENE_CIRCFG2_FAST_SAMPL2: c_uint = 0x80	/* Fast data detection for RC6 */;
// Knobs for protocol decoding - will document when/if will use them
pub const ENE_CIRPF: c_uint = 0xFEC2;
pub const ENE_CIRHIGH: c_uint = 0xFEC3;
pub const ENE_CIRBIT: c_uint = 0xFEC4;
pub const ENE_CIRSTART: c_uint = 0xFEC5;
pub const ENE_CIRSTART2: c_uint = 0xFEC6;
// Actual register which contains RLC RX data - read by firmware
pub const ENE_CIRDAT_IN: c_uint = 0xFEC7;
// RLC configuration - sample period (1us resolution) + idle mode
pub const ENE_CIRRLC_CFG: c_uint = 0xFEC8;
pub const ENE_CIRRLC_CFG_OVERFLOW: c_uint = 0x80	/* interrupt on overflows if set */;
pub const ENE_DEFAULT_SAMPLE_PERIOD: c_int = 50;
// Two byte RLC TX buffer
pub const ENE_CIRRLC_OUT0: c_uint = 0xFEC9;
pub const ENE_CIRRLC_OUT1: c_uint = 0xFECA;
pub const ENE_CIRRLC_OUT_PULSE: c_uint = 0x80	/* Transmitted sample is pulse */;
pub const ENE_CIRRLC_OUT_MASK: c_uint = 0x7F;
// Carrier detect setting
// Low nibble  - number of carrier pulses to average
// High nibble - number of initial carrier pulses to discard
//
pub const ENE_CIRCAR_PULS: c_uint = 0xFECB;
// detected RX carrier period (resolution: 500 ns)
pub const ENE_CIRCAR_PRD: c_uint = 0xFECC;
pub const ENE_CIRCAR_PRD_VALID: c_uint = 0x80	/* data valid content valid */;
// detected RX carrier pulse width (resolution: 500 ns)
pub const ENE_CIRCAR_HPRD: c_uint = 0xFECD;
// TX period (resolution: 500 ns, minimum 2)
pub const ENE_CIRMOD_PRD: c_uint = 0xFECE;
pub const ENE_CIRMOD_PRD_POL: c_uint = 0x80	/* TX carrier polarity*/;
pub const ENE_CIRMOD_PRD_MAX: c_uint = 0x7F	/* 15.87 kHz */;
pub const ENE_CIRMOD_PRD_MIN: c_uint = 0x02	/* 1 Mhz */;
// TX pulse width (resolution: 500 ns)
pub const ENE_CIRMOD_HPRD: c_uint = 0xFECF;
// Hardware versions
pub const ENE_ECHV: c_uint = 0xFF00	/* hardware revision */;
pub const ENE_PLLFRH: c_uint = 0xFF16;
pub const ENE_PLLFRL: c_uint = 0xFF17;
pub const ENE_DEFAULT_PLL_FREQ: c_int = 1000;
pub const ENE_ECSTS: c_uint = 0xFF1D;
pub const ENE_ECSTS_RSRVD: c_uint = 0x04;
pub const ENE_ECVER_MAJOR: c_uint = 0xFF1E	/* chip version */;
pub const ENE_ECVER_MINOR: c_uint = 0xFF1F;
pub const ENE_HW_VER_OLD: c_uint = 0xFD00;
//

pub const ENE_IRQ_RX: c_int = 1;
pub const ENE_IRQ_TX: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ene_device {
    pub pnp_dev: *mut pnp_dev,
    pub rdev: *mut rc_dev,
// hw IO settings
    pub hw_io: c_long,
    pub irq: c_int,
    pub hw_lock: spinlock_t,
// HW features
    pub /: *mut *mut int hw_revision; / hardware revision,
    pub input*/: *mut *mut bool hw_use_gpio_0a; / gpio0a is demodulated,
    pub /: *mut *mut bool hw_extra_buffer; / hardware has 'extra buffer',
    pub /: *mut *mut bool hw_fan_input; / fan input is IR data source,
    pub /: *mut *mut bool hw_learning_and_tx_capable; / learning & tx capable,
    pub pll_freq: c_int,
    pub buffer_len: c_int,
// Extra RX buffer location
    pub extra_buf1_address: c_int,
    pub extra_buf1_len: c_int,
    pub extra_buf2_address: c_int,
    pub extra_buf2_len: c_int,
// HW state
    pub /: *mut *mut int r_pointer; / pointer to next sample to read,
    pub /: *mut *mut int w_pointer; / pointer to next sample hw will write,
    pub rx*/: *mut *mut bool rx_fan_input_inuse; / is fan input in use for,
    pub /: *mut *mut int tx_reg; / current reg used for TX,
    pub /: *mut *mut u8 saved_conf1; / saved FEC0 reg,
    pub /: *mut *mut unsigned int tx_sample; / current sample for TX,
    pub /: *mut *mut bool tx_sample_pulse; / current sample is pulse,
// TX buffer
    pub buffer*/: *mut *mut *mut unsigned tx_buffer; / input samples,
    pub /: *mut *mut int tx_pos; / position in that buffer,
    pub /: *mut *mut int tx_len; / current len of tx buffer,
    pub /: *mut *mut int tx_done; / done transmitting,
// one more sample pending
    pub /: *mut *mut completion tx_complete; / TX completion,
    pub tx_sim_timer: timer_list,
// TX settings
    pub tx_period: c_int,
    pub tx_duty_cycle: c_int,
    pub transmitter_mask: c_int,
// RX settings
    pub /: *mut *mut bool learning_mode_enabled; / learning input enabled,
    pub /: *mut *mut bool carrier_detect_enabled; / carrier detect enabled,
    pub rx_period_adjust: c_int,
    pub rx_enabled: bool,
}

extern "C" {
    pub fn ene_irq_status(dev: *mut ene_device) -> static int;
}
extern "C" {
    pub fn ene_rx_read_hw_pointer(dev: *mut ene_device) -> static void;
}
