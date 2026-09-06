//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/iosf_mbi.h
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
// Intel OnChip System Fabric MailBox access support
//

pub const MBI_MCR_OFFSET: c_uint = 0xD0;
pub const MBI_MDR_OFFSET: c_uint = 0xD4;
pub const MBI_MCRX_OFFSET: c_uint = 0xD8;
pub const MBI_RD_MASK: c_uint = 0xFEFFFFFF;

pub const MBI_MASK_HI: c_uint = 0xFFFFFF00;
pub const MBI_MASK_LO: c_uint = 0x000000FF;
pub const MBI_ENABLE: c_uint = 0xF0;
// IOSF SB read/write opcodes
pub const MBI_MMIO_READ: c_uint = 0x00;
pub const MBI_MMIO_WRITE: c_uint = 0x01;
pub const MBI_CFG_READ: c_uint = 0x04;
pub const MBI_CFG_WRITE: c_uint = 0x05;
pub const MBI_CR_READ: c_uint = 0x06;
pub const MBI_CR_WRITE: c_uint = 0x07;
pub const MBI_REG_READ: c_uint = 0x10;
pub const MBI_REG_WRITE: c_uint = 0x11;
pub const MBI_ESRAM_READ: c_uint = 0x12;
pub const MBI_ESRAM_WRITE: c_uint = 0x13;
// Baytrail available units
pub const BT_MBI_UNIT_AUNIT: c_uint = 0x00;
pub const BT_MBI_UNIT_SMC: c_uint = 0x01;
pub const BT_MBI_UNIT_CPU: c_uint = 0x02;
pub const BT_MBI_UNIT_BUNIT: c_uint = 0x03;
pub const BT_MBI_UNIT_PMC: c_uint = 0x04;
pub const BT_MBI_UNIT_GFX: c_uint = 0x06;
pub const BT_MBI_UNIT_SMI: c_uint = 0x0C;
pub const BT_MBI_UNIT_CCK: c_uint = 0x14;
pub const BT_MBI_UNIT_USB: c_uint = 0x43;
pub const BT_MBI_UNIT_SATA: c_uint = 0xA3;
pub const BT_MBI_UNIT_PCIE: c_uint = 0xA6;
// Quark available units
pub const QRK_MBI_UNIT_HBA: c_uint = 0x00;
pub const QRK_MBI_UNIT_HB: c_uint = 0x03;
pub const QRK_MBI_UNIT_RMU: c_uint = 0x04;
pub const QRK_MBI_UNIT_MM: c_uint = 0x05;
pub const QRK_MBI_UNIT_SOC: c_uint = 0x31;
// Action values for the pmic_bus_access_notifier functions
pub const MBI_PMIC_BUS_ACCESS_BEGIN: c_int = 1;
pub const MBI_PMIC_BUS_ACCESS_END: c_int = 2;

extern "C" {
    pub fn iosf_mbi_available() -> bool;
}
//
// iosf_mbi_read() - MailBox Interface read command
// @port:	port indicating subunit being accessed
// @opcode:	port specific read or write opcode
// @offset:	register address offset
// @mdr:	register data to be read
//
// Locking is handled by spinlock - cannot sleep.
// Return: Nonzero on error
//
extern "C" {
    pub fn iosf_mbi_read(port: u8, opcode: u8, offset: u32, mdr: *mut u32) -> c_int;
}
//
// iosf_mbi_write() - MailBox unmasked write command
// @port:	port indicating subunit being accessed
// @opcode:	port specific read or write opcode
// @offset:	register address offset
// @mdr:	register data to be written
//
// Locking is handled by spinlock - cannot sleep.
// Return: Nonzero on error
//
extern "C" {
    pub fn iosf_mbi_write(port: u8, opcode: u8, offset: u32, mdr: u32) -> c_int;
}
//
// iosf_mbi_modify() - MailBox masked write command
// @port:	port indicating subunit being accessed
// @opcode:	port specific read or write opcode
// @offset:	register address offset
// @mdr:	register data being modified
// @mask:	mask indicating bits in mdr to be modified
//
// Locking is handled by spinlock - cannot sleep.
// Return: Nonzero on error
//
extern "C" {
    pub fn iosf_mbi_modify(port: u8, opcode: u8, offset: u32, mdr: u32, mask: u32) -> c_int;
}
//
// iosf_mbi_punit_acquire() - Acquire access to the P-Unit
//
// One some systems the P-Unit accesses the PMIC to change various voltages
// through the same bus as other kernel drivers use for e.g. battery monitoring.
//
// If a driver sends requests to the P-Unit which require the P-Unit to access
// the PMIC bus while another driver is also accessing the PMIC bus various bad
// things happen.
//
// Call this function before sending requests to the P-Unit which may make it
// access the PMIC, be it through iosf_mbi* functions or through other means.
// This function will block all kernel access to the PMIC I2C bus, so that the
// P-Unit can safely access the PMIC over the shared I2C bus.
//
// Note on these systems the i2c-bus driver will request a semaphore from the
// P-Unit for exclusive access to the PMIC bus when i2c drivers are accessing
// it, but this does not appear to be sufficient, we still need to avoid making
// certain P-Unit requests during the access window to avoid problems.
//
// This function locks a mutex, as such it may sleep.
//
extern "C" {
    pub fn iosf_mbi_punit_acquire();
}
//
// iosf_mbi_punit_release() - Release access to the P-Unit
//
extern "C" {
    pub fn iosf_mbi_punit_release();
}
//
// iosf_mbi_block_punit_i2c_access() - Block P-Unit accesses to the PMIC bus
//
// Call this function to block P-Unit access to the PMIC I2C bus, so that the
// kernel can safely access the PMIC over the shared I2C bus.
//
// This function acquires the P-Unit bus semaphore and notifies
// pmic_bus_access_notifier listeners that they may no longer access the
// P-Unit in a way which may cause it to access the shared I2C bus.
//
// Note this function may be called multiple times and the bus will not
// be released until iosf_mbi_unblock_punit_i2c_access() has been called the
// same amount of times.
//
// Return: Nonzero on error
//
extern "C" {
    pub fn iosf_mbi_block_punit_i2c_access() -> c_int;
}
//
// iosf_mbi_unblock_punit_i2c_access() - Release PMIC I2C bus block
//
// Release i2c access block gotten through iosf_mbi_block_punit_i2c_access().
//
extern "C" {
    pub fn iosf_mbi_unblock_punit_i2c_access();
}
//
// iosf_mbi_register_pmic_bus_access_notifier - Register PMIC bus notifier
//
// This function can be used by drivers which may need to acquire P-Unit
// managed resources from interrupt context, where iosf_mbi_punit_acquire()
// can not be used.
//
// This function allows a driver to register a notifier to get notified (in a
// process context) before other drivers start accessing the PMIC bus.
//
// This allows the driver to acquire any resources, which it may need during
// the window the other driver is accessing the PMIC, before hand.
//
// @nb: notifier_block to register
//
extern "C" {
    pub fn iosf_mbi_register_pmic_bus_access_notifier(nb: *mut notifier_block) -> c_int;
}
//
// iosf_mbi_unregister_pmic_bus_access_notifier_unlocked - Unregister PMIC bus
// notifier, unlocked
//
// Like iosf_mbi_unregister_pmic_bus_access_notifier(), but for use when the
// caller has already called iosf_mbi_punit_acquire() itself.
//
// @nb: notifier_block to unregister
//
// iosf_mbi_assert_punit_acquired - Assert that the P-Unit has been acquired.
//
extern "C" {
    pub fn iosf_mbi_assert_punit_acquired();
}

