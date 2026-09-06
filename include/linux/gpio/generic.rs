//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gpio/generic.h
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
// struct gpio_generic_chip_config - Generic GPIO chip configuration data
// @dev: Parent device of the new GPIO chip (compulsory).
// @sz: Size (width) of the MMIO registers in bytes, typically 1, 2 or 4.
// @dat: MMIO address for the register to READ the value of the GPIO lines, it
// is expected that a 1 in the corresponding bit in this register means
// the line is asserted.
// @set: MMIO address for the register to SET the value of the GPIO lines, it
// is expected that we write the line with 1 in this register to drive
// the GPIO line high.
// @clr: MMIO address for the register to CLEAR the value of the GPIO lines,
// it is expected that we write the line with 1 in this register to
// drive the GPIO line low. It is allowed to leave this address as NULL,
// in that case the SET register will be assumed to also clear the GPIO
// lines, by actively writing the line with 0.
// @dirout: MMIO address for the register to set the line as OUTPUT. It is
// assumed that setting a line to 1 in this register will turn that
// line into an output line. Conversely, setting the line to 0 will
// turn that line into an input.
// @dirin: MMIO address for the register to set this line as INPUT. It is
// assumed that setting a line to 1 in this register will turn that
// line into an input line. Conversely, setting the line to 0 will
// turn that line into an output.
// @flags: Different flags that will affect the behaviour of the device, such
// as endianness etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_generic_chip_config {
    pub dev: *mut device,
    pub sz: c_ulong,
    pub dat: *mut void __iomem,
    pub set: *mut void __iomem,
    pub clr: *mut void __iomem,
    pub dirout: *mut void __iomem,
    pub dirin: *mut void __iomem,
    pub flags: c_ulong,
}

//
// struct gpio_generic_chip - Generic GPIO chip implementation.
// @gc: The underlying struct gpio_chip object, implementing low-level GPIO
// chip routines.
// @read_reg: reader function for generic GPIO
// @write_reg: writer function for generic GPIO
// @be_bits: if the generic GPIO has big endian bit order (bit 31 is
// representing line 0, bit 30 is line 1 ... bit 0 is line 31) this
// is set to true by the generic GPIO core. It is for internal
// housekeeping only.
// @reg_dat: data (in) register for generic GPIO
// @reg_set: output set register (out=high) for generic GPIO
// @reg_clr: output clear register (out=low) for generic GPIO
// @reg_dir_out: direction out setting register for generic GPIO
// @reg_dir_in: direction in setting register for generic GPIO
// @dir_unreadable: indicates that the direction register(s) cannot be read and
// we need to rely on out internal state tracking.
// @pinctrl: the generic GPIO uses a pin control backend.
// @bits: number of register bits used for a generic GPIO
// i.e. <register width> * 8
// @lock: used to lock chip->sdata. Also, this is needed to keep
// shadowed and real data registers writes together.
// @sdata: shadowed data register for generic GPIO to clear/set bits safely.
// @sdir: shadowed direction register for generic GPIO to clear/set direction
// safely. A "1" in this word means the line is set as output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_generic_chip {
    pub gc: gpio_chip,
    pub reg): *mut *mut unsigned long (read_reg)(void __iomem,
    pub data): *mut *mut *mut void (write_reg)(void __iomem reg, unsigned long,
    pub be_bits: bool,
    pub reg_dat: *mut void __iomem,
    pub reg_set: *mut void __iomem,
    pub reg_clr: *mut void __iomem,
    pub reg_dir_out: *mut void __iomem,
    pub reg_dir_in: *mut void __iomem,
    pub dir_unreadable: bool,
    pub pinctrl: bool,
    pub bits: c_int,
    pub lock: raw_spinlock_t,
    pub sdata: c_ulong,
    pub sdir: c_ulong,
}

extern "C" {
    pub fn container_of(_arg: gc, gpio_generic_chip: struct, _arg: gc) -> return;
}
//
// gpio_generic_chip_set() - Set the GPIO line value of the generic GPIO chip.
// @chip: Generic GPIO chip to use.
// @offset: Hardware offset of the line to set.
// @value: New GPIO line value.
//
// Some modules using the generic GPIO chip, need to set line values in their
// direction setters but they don't have access to the gpio-mmio symbols so
// they use the function pointer in struct gpio_chip directly. This is not
// optimal and can lead to crashes at run-time in some instances. This wrapper
// provides a safe interface for users.
//
// Returns: 0 on success, negative error number of failure.
//
// gpio_generic_read_reg() - Read a register using the underlying callback.
// @chip: Generic GPIO chip to use.
// @reg: Register to read.
//
// Returns: value read from register.
//
// gpio_generic_write_reg() - Write a register using the underlying callback.
// @chip: Generic GPIO chip to use.
// @reg: Register to write to.
// @val: New value to write.
//

