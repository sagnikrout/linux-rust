//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/phy_g.h
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
// OFDM PHY registers are defined in the A-PHY header.

// CCK (B) PHY Registers

pub const B43_PHY_PGACTL_LPF: c_uint = 0x1000	/* Low pass filter (?) */;
pub const B43_PHY_PGACTL_LOWBANDW: c_uint = 0x0040	/* Low bandwidth flag */;
pub const B43_PHY_PGACTL_UNKNOWN: c_uint = 0xEFA0;

// Extended G-PHY Registers

pub const B43_PHY_GTABOFF: c_uint = 0x03FF	/* G-PHY table offset (see below) */;
pub const B43_PHY_GTABNR: c_uint = 0xFC00	/* G-PHY table number (see below) */;
pub const B43_PHY_GTABNR_SHIFT: c_int = 10;

pub const B43_PHY_RFOVERVAL_EXTLNA: c_uint = 0x8000;
pub const B43_PHY_RFOVERVAL_LNA: c_uint = 0x7000;
pub const B43_PHY_RFOVERVAL_LNA_SHIFT: c_int = 12;
pub const B43_PHY_RFOVERVAL_PGA: c_uint = 0x0F00;
pub const B43_PHY_RFOVERVAL_PGA_SHIFT: c_int = 8;
pub const B43_PHY_RFOVERVAL_UNK: c_uint = 0x0010	/* Unknown, always set. */;
pub const B43_PHY_RFOVERVAL_TRSWRX: c_uint = 0x00E0;
pub const B43_PHY_RFOVERVAL_BW: c_uint = 0x0003	/* Bandwidth flags */;
pub const B43_PHY_RFOVERVAL_BW_LPF: c_uint = 0x0001	/* Low Pass Filter */;
pub const B43_PHY_RFOVERVAL_BW_LBW: c_uint = 0x0002	/* Low Bandwidth (when set), high when unset */;

// G-PHY table numbers

extern "C" {
    pub fn b43_gtab_read(dev: *mut b43_wldev, table: u16, offset: u16) -> u16;
}
extern "C" {
    pub fn b43_gtab_write(dev: *mut b43_wldev, table: u16, offset: u16, value: u16);
}
// Returns the boolean whether "TX Magnification" is enabled.

// Card uses the loopback gain stuff

// Radio Attenuation (RF Attenuation)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_rfatt {
    pub /: *mut *mut u8 att; / Attenuation value,
    pub /: *mut *mut bool with_padmix; / Flag, PAD Mixer enabled.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_rfatt_list {
// Attenuation values list
    pub list: *const b43_rfatt,
    pub len: u8,
// Minimum/Maximum attenuation values
    pub min_val: u8,
    pub max_val: u8,
}

// Returns true, if the values are the same.
// Baseband Attenuation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_bbatt {
    pub /: *mut *mut u8 att; / Attenuation value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_bbatt_list {
// Attenuation values list
    pub list: *const b43_bbatt,
    pub len: u8,
// Minimum/Maximum attenuation values
    pub min_val: u8,
    pub max_val: u8,
}

// Returns true, if the values are the same.
// tx_control bits.
pub const B43_TXCTL_PA3DB: c_uint = 0x40	/* PA Gain 3dB */;
pub const B43_TXCTL_PA2DB: c_uint = 0x20	/* PA Gain 2dB */;
pub const B43_TXCTL_TXMIX: c_uint = 0x10	/* TX Mixer Gain */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_g {
// ACI (adjacent channel interference) flags.
    pub aci_enable: bool,
    pub aci_wlan_automatic: bool,
    pub aci_hw_rssi: bool,
// Radio switched on/off
    pub radio_on: bool,
// Values saved when turning the radio off.
// They are needed when turning it on again.
    pub valid: bool,
    pub rfover: u16,
    pub rfoverval: u16,
    pub radio_off_context: },
    pub minlowsig: [u16; 2],
    pub minlowsigpos: [u16; 2],
// Pointer to the table used to convert a
// TSSI value to dBm-Q5.2
    pub tssi2dbm: *const i8,
// tssi2dbm is kmalloc()ed. Only used for free()ing.
    pub dyn_tssi_tbl: bool,
// Target idle TSSI
    pub tgt_idle_tssi: c_int,
// Current idle TSSI
    pub cur_idle_tssi: c_int,
// The current average TSSI.
    pub average_tssi: u8,
// Current TX power level attenuation control values
    pub bbatt: b43_bbatt,
    pub rfatt: b43_rfatt,
    pub /: *mut *mut u8 tx_control; / B43_TXCTL_XXX,
// The calculated attenuation deltas that are used later
// when adjusting the actual power output.
    pub bbatt_delta: c_int,
    pub rfatt_delta: c_int,
// LocalOscillator control values.
    pub lo_control: *mut b43_txpower_lo_control,
// Values from b43_calc_loopback_gain()
    pub /: *mut *mut s16 max_lb_gain; / Maximum Loopback gain in hdB,
    pub /: *mut *mut s16 trsw_rx_gain; / TRSW RX gain in hdB,
    pub /: *mut *mut s16 lna_lod_gain; / LNA lod,
    pub /: *mut *mut s16 lna_gain; / LNA,
    pub /: *mut *mut s16 pga_gain; / PGA,
// Current Interference Mitigation mode
    pub interfmode: c_int,
// Stack of saved values from the Interference Mitigation code.
// Each value in the stack is laid out as follows:
// bit 0-11:  offset
// bit 12-15: register ID
// bit 16-32: value
// register ID is: 0x1 PHY, 0x2 Radio, 0x3 ILT
//
pub const B43_INTERFSTACK_SIZE: c_int = 26;
    pub structure: u32 interfstack[B43_INTERFSTACK_SIZE]; //FIXME: use a data,
// Saved values from the NRSSI Slope calculation
    pub nrssi: [i16; 2],
    pub nrssislope: i32,
// In memory nrssi lookup table.
    pub nrssi_lt: [i8; 64],
    pub lofcal: u16,
    pub rename?: u16 initval; //FIXME,
// The device does address auto increment for the OFDM tables.
// We cache the previously used address here and omit the address
// write on the next table access, if possible.
    pub /: *mut *mut u16 ofdmtab_addr; / The address currently set in hardware.,
    pub ofdmtab_addr_direction: },
}
