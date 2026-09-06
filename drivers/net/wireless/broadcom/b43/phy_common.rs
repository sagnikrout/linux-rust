//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/phy_common.h
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

// PHY register routing bits
pub const B43_PHYROUTE: c_uint = 0x0C00 /* PHY register routing bits mask */;
pub const B43_PHYROUTE_BASE: c_uint = 0x0000 /* Base registers */;
pub const B43_PHYROUTE_OFDM_GPHY: c_uint = 0x0400 /* OFDM register routing for G-PHYs */;
pub const B43_PHYROUTE_EXT_GPHY: c_uint = 0x0800 /* Extended G-PHY registers */;
pub const B43_PHYROUTE_N_BMODE: c_uint = 0x0C00 /* N-PHY BMODE registers */;
// CCK (B-PHY) registers.

// N-PHY registers.

// N-PHY BMODE registers.

// OFDM (A-PHY) registers.

// Extended G-PHY registers.

// Masks for the PHY versioning registers.
pub const B43_PHYVER_ANALOG: c_uint = 0xF000;
pub const B43_PHYVER_ANALOG_SHIFT: c_int = 12;
pub const B43_PHYVER_TYPE: c_uint = 0x0F00;
pub const B43_PHYVER_TYPE_SHIFT: c_int = 8;
pub const B43_PHYVER_VERSION: c_uint = 0x00FF;
// PHY writes need to be flushed if we reach limit
pub const B43_MAX_WRITES_IN_ROW: c_int = 24;
//
// enum b43_interference_mitigation - Interference Mitigation mode
//
// @B43_INTERFMODE_NONE:	Disabled
// @B43_INTERFMODE_NONWLAN:	Non-WLAN Interference Mitigation
// @B43_INTERFMODE_MANUALWLAN:	WLAN Interference Mitigation
// @B43_INTERFMODE_AUTOWLAN:	Automatic WLAN Interference Mitigation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_interference_mitigation {
    B43_INTERFMODE_NONE,
    B43_INTERFMODE_NONWLAN,
    B43_INTERFMODE_MANUALWLAN,
    B43_INTERFMODE_AUTOWLAN,
}

// Antenna identifiers
//
// enum b43_txpwr_result - Return value for the recalc_txpower PHY op.
//
// @B43_TXPWR_RES_NEED_ADJUST:	Values changed. Hardware adjustment is needed.
// @B43_TXPWR_RES_DONE:		No more work to do. Everything is done.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_txpwr_result {
    B43_TXPWR_RES_NEED_ADJUST,
    B43_TXPWR_RES_DONE,
}

//
// struct b43_phy_operations - Function pointers for PHY ops.
//
// @allocate:		Allocate and initialise the PHY data structures.
// Must not be NULL.
// @free:		Destroy and free the PHY data structures.
// Must not be NULL.
//
// @prepare_structs:	Prepare the PHY data structures.
// The data structures allocated in @allocate are
// initialized here.
// Must not be NULL.
// @prepare_hardware:	Prepare the PHY. This is called before b43_chip_init to
// do some early PHY hardware init.
// Can be NULL, if not required.
// @init:		Initialize the PHY.
// Must not be NULL.
// @exit:		Shutdown the PHY.
// Can be NULL, if not required.
//
// @phy_read:		Read from a PHY register.
// Must not be NULL.
// @phy_write:		Write to a PHY register.
// Must not be NULL.
// @phy_maskset:	Maskset a PHY register, taking shortcuts.
// If it is NULL, a generic algorithm is used.
// @radio_read:		Read from a Radio register.
// Must not be NULL.
// @radio_write:	Write to a Radio register.
// Must not be NULL.
//
// @supports_hwpctl:	Returns a boolean whether Hardware Power Control
// is supported or not.
// If NULL, hwpctl is assumed to be never supported.
// @software_rfkill:	Turn the radio ON or OFF.
// Possible state values are
// RFKILL_STATE_SOFT_BLOCKED or
// RFKILL_STATE_UNBLOCKED
// Must not be NULL.
// @switch_analog:	Turn the Analog on/off.
// Must not be NULL.
// @switch_channel:	Switch the radio to another channel.
// Must not be NULL.
// @get_default_chan:	Just returns the default channel number.
// Must not be NULL.
// @set_rx_antenna:	Set the antenna used for RX.
// Can be NULL, if not supported.
// @interf_mitigation:	Switch the Interference Mitigation mode.
// Can be NULL, if not supported.
//
// @recalc_txpower:	Recalculate the transmission power parameters.
// This callback has to recalculate the TX power settings,
// but does not need to write them to the hardware, yet.
// Returns enum b43_txpwr_result to indicate whether the hardware
// needs to be adjusted.
// If B43_TXPWR_NEED_ADJUST is returned, @adjust_txpower
// will be called later.
// If the parameter "ignore_tssi" is true, the TSSI values should
// be ignored and a recalculation of the power settings should be
// done even if the TSSI values did not change.
// This function may sleep, but should not.
// Must not be NULL.
// @adjust_txpower:	Write the previously calculated TX power settings
// (from @recalc_txpower) to the hardware.
// This function may sleep.
// Can be NULL, if (and ONLY if) @recalc_txpower _always_
// returns B43_TXPWR_RES_DONE.
//
// @pwork_15sec:	Periodic work. Called every 15 seconds.
// Can be NULL, if not required.
// @pwork_60sec:	Periodic work. Called every 60 seconds.
// Can be NULL, if not required.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_operations {
// Initialisation
    pub dev): *mut *mut int (allocate)(struct b43_wldev,
    pub dev): *mut *mut void (free)(struct b43_wldev,
    pub dev): *mut *mut void (prepare_structs)(struct b43_wldev,
    pub dev): *mut *mut int (prepare_hardware)(struct b43_wldev,
    pub dev): *mut *mut int (init)(struct b43_wldev,
    pub dev): *mut *mut void (exit)(struct b43_wldev,
// Register access
    pub reg): *mut *mut *mut u16 (phy_read)(struct b43_wldev dev, u16,
    pub value): *mut *mut *mut void (phy_write)(struct b43_wldev dev, u16 reg, u16,
    pub set): *mut *mut *mut void (phy_maskset)(struct b43_wldev dev, u16 reg, u16 mask, u16,
    pub reg): *mut *mut *mut u16 (radio_read)(struct b43_wldev dev, u16,
    pub value): *mut *mut *mut void (radio_write)(struct b43_wldev dev, u16 reg, u16,
// Radio
    pub dev): *mut *mut bool (supports_hwpctl)(struct b43_wldev,
    pub blocked): *mut *mut *mut void (software_rfkill)(struct b43_wldev dev, bool,
    pub on): *mut *mut *mut void (switch_analog)(struct b43_wldev dev, bool,
    pub new_channel): *mut *mut *mut int (switch_channel)(struct b43_wldev dev, unsigned int,
    pub dev): *mut *mut unsigned int (get_default_chan)(struct b43_wldev,
    pub antenna): *mut *mut *mut void (set_rx_antenna)(struct b43_wldev dev, int,
    pub new_mode): b43_interference_mitigation,
// Transmission power adjustment
    pub ignore_tssi): bool,
    pub dev): *mut *mut void (adjust_txpower)(struct b43_wldev,
// Misc
    pub dev): *mut *mut void (pwork_15sec)(struct b43_wldev,
    pub dev): *mut *mut void (pwork_60sec)(struct b43_wldev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy {
// Hardware operation callbacks.
    pub ops: *const b43_phy_operations,
// Most hardware context information is stored in the standard-
// specific data structures pointed to by the pointers below.
// Only one of them is valid (the currently enabled PHY).

// No union for debug build to force NULL derefs in buggy code.

// G-PHY specific information
    pub g: *mut b43_phy_g,
// N-PHY specific information
    pub n: *mut b43_phy_n,
// LP-PHY specific information
    pub lp: *mut b43_phy_lp,
// HT-PHY specific information
    pub ht: *mut b43_phy_ht,
// LCN-PHY specific information
    pub lcn: *mut b43_phy_lcn,
// AC-PHY specific information
    pub ac: *mut b43_phy_ac,
}

// Band support flags.
// Is GMODE (2 GHz mode) bit enabled?
// After power reset full init has to be performed
// Analog Type
// B43_PHYTYPE_
// PHY revision number.
// Count writes since last read
// Radio versioning
// Software state of the radio
// Desired TX power level (in dBm).
// This is set by the user and adjusted in b43_phy_xmitpower().
// Hardware Power Control enabled?
// The time (in absolute jiffies) when the next TX power output
// check is needed.
// Current channel
// PHY TX errors counter.

// PHY registers locked (w.r.t. firmware)
// Radio registers locked (w.r.t. firmware)

//
// b43_phy_allocate - Allocate PHY structs
// Allocate the PHY data structures, based on the current dev->phy.type
//
extern "C" {
    pub fn b43_phy_allocate(dev: *mut b43_wldev) -> c_int;
}
//
// b43_phy_free - Free PHY structs
//
extern "C" {
    pub fn b43_phy_free(dev: *mut b43_wldev);
}
//
// b43_phy_init - Initialise the PHY
//
extern "C" {
    pub fn b43_phy_init(dev: *mut b43_wldev) -> c_int;
}
//
// b43_phy_exit - Cleanup PHY
//
extern "C" {
    pub fn b43_phy_exit(dev: *mut b43_wldev);
}
//
// b43_has_hardware_pctl - Hardware Power Control supported?
// Returns a boolean, whether hardware power control is supported.
//
extern "C" {
    pub fn b43_has_hardware_pctl(dev: *mut b43_wldev) -> bool;
}
//
// b43_phy_read - 16bit PHY register read access
//
extern "C" {
    pub fn b43_phy_read(dev: *mut b43_wldev, reg: u16) -> u16;
}
//
// b43_phy_write - 16bit PHY register write access
//
extern "C" {
    pub fn b43_phy_write(dev: *mut b43_wldev, reg: u16, value: u16);
}
//
// b43_phy_copy - copy contents of 16bit PHY register to another
//
extern "C" {
    pub fn b43_phy_copy(dev: *mut b43_wldev, destreg: u16, srcreg: u16);
}
//
// b43_phy_mask - Mask a PHY register with a mask
//
extern "C" {
    pub fn b43_phy_mask(dev: *mut b43_wldev, offset: u16, mask: u16);
}
//
// b43_phy_set - OR a PHY register with a bitmap
//
extern "C" {
    pub fn b43_phy_set(dev: *mut b43_wldev, offset: u16, set: u16);
}
//
// b43_phy_maskset - Mask and OR a PHY register with a mask and bitmap
//
extern "C" {
    pub fn b43_phy_maskset(dev: *mut b43_wldev, offset: u16, mask: u16, set: u16);
}
//
// b43_radio_read - 16bit Radio register read access
//
extern "C" {
    pub fn b43_radio_read(dev: *mut b43_wldev, reg: u16) -> u16;
}

//
// b43_radio_write - 16bit Radio register write access
//
extern "C" {
    pub fn b43_radio_write(dev: *mut b43_wldev, reg: u16, value: u16);
}

//
// b43_radio_mask - Mask a 16bit radio register with a mask
//
extern "C" {
    pub fn b43_radio_mask(dev: *mut b43_wldev, offset: u16, mask: u16);
}
//
// b43_radio_set - OR a 16bit radio register with a bitmap
//
extern "C" {
    pub fn b43_radio_set(dev: *mut b43_wldev, offset: u16, set: u16);
}
//
// b43_radio_maskset - Mask and OR a radio register with a mask and bitmap
//
extern "C" {
    pub fn b43_radio_maskset(dev: *mut b43_wldev, offset: u16, mask: u16, set: u16);
}
//
// b43_radio_wait_value - Waits for a given value in masked register read
//
// b43_radio_lock - Lock firmware radio register access
//
extern "C" {
    pub fn b43_radio_lock(dev: *mut b43_wldev);
}
//
// b43_radio_unlock - Unlock firmware radio register access
//
extern "C" {
    pub fn b43_radio_unlock(dev: *mut b43_wldev);
}
//
// b43_phy_lock - Lock firmware PHY register access
//
extern "C" {
    pub fn b43_phy_lock(dev: *mut b43_wldev);
}
//
// b43_phy_unlock - Unlock firmware PHY register access
//
extern "C" {
    pub fn b43_phy_unlock(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_phy_put_into_reset(dev: *mut b43_wldev);
}
extern "C" {
    pub fn b43_phy_take_out_of_reset(dev: *mut b43_wldev);
}
//
// b43_switch_channel - Switch to another channel
//
extern "C" {
    pub fn b43_switch_channel(dev: *mut b43_wldev, new_channel: c_uint) -> c_int;
}
//
// b43_software_rfkill - Turn the radio ON or OFF in software.
//
extern "C" {
    pub fn b43_software_rfkill(dev: *mut b43_wldev, blocked: bool);
}
//
// b43_phy_txpower_check - Check TX power output.
//
// Compare the current TX power output to the desired power emission
// and schedule an adjustment in case it mismatches.
//
// @flags:	OR'ed enum b43_phy_txpower_check_flags flags.
// See the docs below.
//
extern "C" {
    pub fn b43_phy_txpower_check(dev: *mut b43_wldev, flags: c_uint);
}
//
// enum b43_phy_txpower_check_flags - Flags for b43_phy_txpower_check()
//
// @B43_TXPWR_IGNORE_TIME: Ignore the schedule time and force-redo
// the check now.
// @B43_TXPWR_IGNORE_TSSI: Redo the recalculation, even if the average
// TSSI did not change.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum b43_phy_txpower_check_flags {
    B43_TXPWR_IGNORE_TIME		= (1 << 0),
    B43_TXPWR_IGNORE_TSSI		= (1 << 1),
}

extern "C" {
    pub fn b43_phy_txpower_adjust_work(work: *mut work_struct);
}
//
// b43_phy_shm_tssi_read - Read the average of the last 4 TSSI from SHM.
//
// @shm_offset:		The SHM address to read the values from.
//
// Returns the average of the 4 TSSI values, or a negative error code.
//
extern "C" {
    pub fn b43_phy_shm_tssi_read(dev: *mut b43_wldev, shm_offset: u16) -> c_int;
}
//
// b43_phy_switch_analog_generic - Generic PHY operation for switching the Analog.
//
// It does the switching based on the PHY0 core register.
// Do _not_ call this directly. Only use it as a switch_analog callback
// for struct b43_phy_operations.
//
extern "C" {
    pub fn b43_phyop_switch_analog_generic(dev: *mut b43_wldev, on: bool);
}
extern "C" {
    pub fn b43_is_40mhz(dev: *mut b43_wldev) -> bool;
}
extern "C" {
    pub fn b43_phy_force_clock(dev: *mut b43_wldev, force: bool);
}
