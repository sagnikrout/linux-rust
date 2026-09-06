//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regmap.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Register map access API
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

//
// regmap_mdio address encoding. IEEE 802.3ae clause 45 addresses consist of a
// device address and a register address.
//
pub const REGMAP_MDIO_C45_DEVAD_SHIFT: c_int = 16;

//
// regmap.reg_shift indicates by how much we must shift registers prior to
// performing any operation. It's a signed value, positive numbers means
// downshifting the register's address, while negative numbers means upshifting.
//

//
// The supported cache types, the default is no cache.  Any new caches should
// usually use the maple tree cache unless they specifically require that there
// are never any allocations at runtime in which case they should use the sparse
// flat cache.  The rbtree cache *may* have some performance advantage for very
// low end systems that make heavy use of cache syncs but is mainly legacy.
// These caches are sparse and entries will be initialized from hardware if no
// default has been provided.
// The non-sparse flat cache is provided for compatibility with existing users
// and will zero-initialize cache entries for which no defaults are provided.
// New users should use the sparse flat cache.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regcache_type {
    REGCACHE_NONE,
    REGCACHE_RBTREE,
    REGCACHE_FLAT,
    REGCACHE_MAPLE,
    REGCACHE_FLAT_S,
}

//
// struct reg_default - Default value for a register.
//
// @reg: Register address.
// @def: Register default value.
//
// We use an array of structs rather than a simple array as many modern devices
// have very sparse register maps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

//
// struct reg_sequence - An individual write from a sequence of writes.
//
// @reg: Register address.
// @def: Register value.
// @delay_us: Delay to be applied after the register write in microseconds
//
// Register/value pairs for sequences of writes with an optional delay in
// microseconds to be applied after each write.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
    pub delay_us: c_uint,
}

//
// regmap_read_poll_timeout - Poll until a condition is met or a timeout occurs
//
// @map: Regmap to read from
// @addr: Address to poll
// @val: Unsigned integer variable to read the value into
// @cond: Break condition (usually involving @val)
// @sleep_us: Maximum time to sleep between reads in us (0 tight-loops). Please
// read usleep_range() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
//
// This is modelled after the readx_poll_timeout macros in linux/iopoll.h.
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout or the regmap_read
// error return value in case of a error read. In the two former cases,
// the last read value at @addr is stored in @val. Must not be called
// from atomic context if sleep_us or timeout_us are used.
//

//
// regmap_read_poll_timeout_atomic - Poll until a condition is met or a timeout occurs
//
// @map: Regmap to read from
// @addr: Address to poll
// @val: Unsigned integer variable to read the value into
// @cond: Break condition (usually involving @val)
// @delay_us: Time to udelay between reads in us (0 tight-loops). Please
// read udelay() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
//
// This is modelled after the readx_poll_timeout_atomic macros in linux/iopoll.h.
//
// Note: In general regmap cannot be used in atomic context. If you want to use
// this macro then first setup your regmap for atomic use (flat or no cache
// and MMIO regmap).
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout or the regmap_read
// error return value in case of a error read. In the two former cases,
// the last read value at @addr is stored in @val.
//

//
// regmap_field_read_poll_timeout - Poll until a condition is met or timeout
//
// @field: Regmap field to read from
// @val: Unsigned integer variable to read the value into
// @cond: Break condition (usually involving @val)
// @sleep_us: Maximum time to sleep between reads in us (0 tight-loops). Please
// read usleep_range() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
//
// This is modelled after the readx_poll_timeout macros in linux/iopoll.h.
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout or the regmap_field_read
// error return value in case of a error read. In the two former cases,
// the last read value at @addr is stored in @val. Must not be called
// from atomic context if sleep_us or timeout_us are used.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum regmap_endian {
// Unspecified -> 0 -> Backwards compatible default
    REGMAP_ENDIAN_DEFAULT = 0,
    REGMAP_ENDIAN_BIG,
    REGMAP_ENDIAN_LITTLE,
    REGMAP_ENDIAN_NATIVE,
}

//
// struct regmap_range - A register range, used for access related checks
// (readable/writeable/volatile/precious checks)
//
// @range_min: address of first register
// @range_max: address of last register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_range {
    pub range_min: c_uint,
    pub range_max: c_uint,
}

//
// struct regmap_access_table - A table of register ranges for access checks
//
// @yes_ranges : pointer to an array of regmap ranges used as "yes ranges"
// @n_yes_ranges: size of the above array
// @no_ranges: pointer to an array of regmap ranges used as "no ranges"
// @n_no_ranges: size of the above array
//
// A table of ranges including some yes ranges and some no ranges.
// If a register belongs to a no_range, the corresponding check function
// will return false. If a register belongs to a yes range, the corresponding
// check function will return true. "no_ranges" are searched first.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_access_table {
    pub yes_ranges: *const regmap_range,
    pub n_yes_ranges: c_uint,
    pub no_ranges: *const regmap_range,
    pub n_no_ranges: c_uint,
}

extern "C" {
    pub fn void(: *mut *mut regmap_lock)(void) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut regmap_unlock)(void) -> typedef;
}
//
// struct regmap_config - Configuration for the register map of a device.
//
// @name: Optional name of the regmap. Useful when a device has multiple
// register regions.
//
// @reg_bits: Number of bits in a register address, mandatory.
// @reg_stride: The register address stride. Valid register addresses are a
// multiple of this value. If set to 0, a value of 1 will be
// used.
// @reg_shift: The number of bits to shift the register before performing any
// operations. Any positive number will be downshifted, and negative
// values will be upshifted
// @reg_base: Value to be added to every register address before performing any
// operation.
// @pad_bits: Number of bits of padding between register and value.
// @val_bits: Number of bits in a register value, mandatory.
//
// @writeable_reg: Optional callback returning true if the register
// can be written to. If this field is NULL but wr_table
// (see below) is not, the check is performed on such table
// (a register is writeable if it belongs to one of the ranges
// specified by wr_table).
// @readable_reg: Optional callback returning true if the register
// can be read from. If this field is NULL but rd_table
// (see below) is not, the check is performed on such table
// (a register is readable if it belongs to one of the ranges
// specified by rd_table).
// @volatile_reg: Optional callback returning true if the register
// value can't be cached. If this field is NULL but
// volatile_table (see below) is not, the check is performed on
// such table (a register is volatile if it belongs to one of
// the ranges specified by volatile_table).
// @precious_reg: Optional callback returning true if the register
// should not be read outside of a call from the driver
// (e.g., a clear on read interrupt status register). If this
// field is NULL but precious_table (see below) is not, the
// check is performed on such table (a register is precious if
// it belongs to one of the ranges specified by precious_table).
// @writeable_noinc_reg: Optional callback returning true if the register
// supports multiple write operations without incrementing
// the register number. If this field is NULL but
// wr_noinc_table (see below) is not, the check is
// performed on such table (a register is no increment
// writeable if it belongs to one of the ranges specified
// by wr_noinc_table).
// @readable_noinc_reg: Optional callback returning true if the register
// supports multiple read operations without incrementing
// the register number. If this field is NULL but
// rd_noinc_table (see below) is not, the check is
// performed on such table (a register is no increment
// readable if it belongs to one of the ranges specified
// by rd_noinc_table).
// @reg_read:	  Optional callback that if filled will be used to perform
// all the reads from the registers. Should only be provided for
// devices whose read operation cannot be represented as a simple
// read operation on a bus such as SPI, I2C, etc. Most of the
// devices do not need this.
// @reg_write:	  Same as above for writing.
// @reg_update_bits: Optional callback that if filled will be used to perform
// all the update_bits(rmw) operation. Should only be provided
// if the function require special handling with lock and reg
// handling and the operation cannot be represented as a simple
// update_bits operation on a bus such as SPI, I2C, etc.
// @read: Optional callback that if filled will be used to perform all the
// bulk reads from the registers. Data is returned in the buffer used
// to transmit data.
// @write: Same as above for writing.
// @max_raw_read: Max raw read size that can be used on the device.
// @max_raw_write: Max raw write size that can be used on the device.
// @can_sleep:	  Optional, specifies whether regmap operations can sleep.
// @fast_io:	  Register IO is fast. Use a spinlock instead of a mutex
// to perform locking. This field is ignored if custom lock/unlock
// functions are used (see fields lock/unlock of struct regmap_config).
// This field is a duplicate of a similar file in
// 'struct regmap_bus' and serves exact same purpose.
// Use it only for "no-bus" cases.
// @io_port:	  Support IO port accessors. Makes sense only when MMIO vs. IO port
// access can be distinguished.
// @disable_locking: This regmap is either protected by external means or
// is guaranteed not to be accessed from multiple threads.
// Don't use any locking mechanisms.
// @lock:	  Optional lock callback (overrides regmap's default lock
// function, based on spinlock or mutex).
// @unlock:	  As above for unlocking.
// @lock_arg:	  This field is passed as the only argument of lock/unlock
// functions (ignored in case regular lock/unlock functions
// are not overridden).
// @max_register: Optional, specifies the maximum valid register address.
// @max_register_is_0: Optional, specifies that zero value in @max_register
// should be taken into account. This is a workaround to
// apply handling of @max_register for regmap that contains
// only one register.
// @wr_table:     Optional, points to a struct regmap_access_table specifying
// valid ranges for write access.
// @rd_table:     As above, for read access.
// @volatile_table: As above, for volatile registers.
// @precious_table: As above, for precious registers.
// @wr_noinc_table: As above, for no increment writeable registers.
// @rd_noinc_table: As above, for no increment readable registers.
// @reg_defaults: Power on reset values for registers (for use with
// register cache support).
// @num_reg_defaults: Number of elements in reg_defaults.
// @reg_default_cb: Optional callback to return default values for registers
// not listed in reg_defaults. This is only used for
// REGCACHE_FLAT population; drivers must ensure the readable_reg
// writeable_reg callbacks are defined to handle holes.
//
// @read_flag_mask: Mask to be set in the top bytes of the register when doing
// a read.
// @write_flag_mask: Mask to be set in the top bytes of the register when doing
// a write. If both read_flag_mask and write_flag_mask are
// empty and zero_flag_mask is not set the regmap_bus default
// masks are used.
// @zero_flag_mask: If set, read_flag_mask and write_flag_mask are used even
// if they are both empty.
// @use_relaxed_mmio: If set, MMIO R/W operations will not use memory barriers.
// This can avoid load on devices which don't require strict
// orderings, but drivers should carefully add any explicit
// memory barriers when they may require them.
// @use_single_read: If set, converts the bulk read operation into a series of
// single read operations. This is useful for a device that
// does not support  bulk read.
// @use_single_write: If set, converts the bulk write operation into a series of
// single write operations. This is useful for a device that
// does not support bulk write.
// @can_multi_write: If set, the device supports the multi write mode of bulk
// write operations, if clear multi write requests will be
// split into individual write operations
//
// @cache_type: The actual cache type.
// @reg_defaults_raw: Power on reset values for registers (for use with
// register cache support).
// @num_reg_defaults_raw: Number of elements in reg_defaults_raw.
// @use_hwlock: Indicate if a hardware spinlock should be used.
// @use_raw_spinlock: Indicate if a raw spinlock should be used.
// @hwlock_id: Specify the hardware spinlock id.
// @hwlock_mode: The hardware spinlock mode, should be HWLOCK_IRQSTATE,
// HWLOCK_IRQ or 0.
// @reg_format_endian: Endianness for formatted register addresses. If this is
// DEFAULT, the @reg_format_endian_default value from the
// regmap bus is used.
// @val_format_endian: Endianness for formatted register values. If this is
// DEFAULT, the @reg_format_endian_default value from the
// regmap bus is used.
//
// @ranges: Array of configuration entries for virtual address ranges.
// @num_ranges: Number of range configuration entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub reg_shift: c_int,
    pub reg_base: c_uint,
    pub pad_bits: c_int,
    pub val_bits: c_int,
    pub reg): *mut *mut *mut bool (writeable_reg)(struct device dev, unsigned int,
    pub reg): *mut *mut *mut bool (readable_reg)(struct device dev, unsigned int,
    pub reg): *mut *mut *mut bool (volatile_reg)(struct device dev, unsigned int,
    pub reg): *mut *mut *mut bool (precious_reg)(struct device dev, unsigned int,
    pub reg): *mut *mut *mut bool (writeable_noinc_reg)(struct device dev, unsigned int,
    pub reg): *mut *mut *mut bool (readable_noinc_reg)(struct device dev, unsigned int,
    pub val): *mut *mut *mut int (reg_read)(void context, unsigned int reg, unsigned int,
    pub val): *mut *mut *mut int (reg_write)(void context, unsigned int reg, unsigned int,
    pub val): unsigned int mask, unsigned int,
// Bulk read/write
    pub val_size): *mut *mut void val_buf, size_t,
    pub count): *const *const *const *const int (write)(void context, void data, size_t,
    pub max_raw_read: usize,
    pub max_raw_write: usize,
    pub can_sleep: bool,
    pub fast_io: bool,
    pub io_port: bool,
    pub disable_locking: bool,
    pub lock: regmap_lock,
    pub unlock: regmap_unlock,
    pub lock_arg: *mut c_void,
    pub max_register: c_uint,
    pub max_register_is_0: bool,
    pub wr_table: *const regmap_access_table,
    pub rd_table: *const regmap_access_table,
    pub volatile_table: *const regmap_access_table,
    pub precious_table: *const regmap_access_table,
    pub wr_noinc_table: *const regmap_access_table,
    pub rd_noinc_table: *const regmap_access_table,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub def): *mut c_uint,
    pub cache_type: regcache_type,
    pub reg_defaults_raw: *const c_void,
    pub num_reg_defaults_raw: c_uint,
    pub read_flag_mask: c_ulong,
    pub write_flag_mask: c_ulong,
    pub zero_flag_mask: bool,
    pub use_single_read: bool,
    pub use_single_write: bool,
    pub use_relaxed_mmio: bool,
    pub can_multi_write: bool,
    pub use_hwlock: bool,
    pub use_raw_spinlock: bool,
    pub hwlock_id: c_uint,
    pub hwlock_mode: c_uint,
    pub reg_format_endian: regmap_endian,
    pub val_format_endian: regmap_endian,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_uint,
}

//
// struct regmap_range_cfg - Configuration for indirectly accessed or paged
// registers.
//
// @name: Descriptive name for diagnostics
//
// @range_min: Address of the lowest register address in virtual range.
// @range_max: Address of the highest register in virtual range.
//
// @selector_reg: Register with selector field.
// @selector_mask: Bit mask for selector value.
// @selector_shift: Bit shift for selector value.
//
// @window_start: Address of first (lowest) register in data window.
// @window_len: Number of registers in data window.
//
// Registers, mapped to this virtual range, are accessed in two steps:
// 1. page selector register update;
// 2. access through data window registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_range_cfg {
    pub name: *const c_char,
// Registers of virtual address range
    pub range_min: c_uint,
    pub range_max: c_uint,
// Page selector for indirect addressing
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_int,
// Data window (per each page)
    pub window_start: c_uint,
    pub window_len: c_uint,
}

//
// struct regmap_sdw_mbq_cfg - Configuration for Multi-Byte Quantities
//
// @mbq_size: Callback returning the actual size of the given register.
// @deferrable: Callback returning true if the hardware can defer
// transactions to the given register. Deferral should
// only be used by SDCA parts and typically which controls
// are deferrable will be specified in either as a hard
// coded list or from the DisCo tables in the platform
// firmware.
//
// @timeout_us: The time in microseconds after which waiting for a deferred
// transaction should time out.
// @retry_us: The time in microseconds between polls of the function busy
// status whilst waiting for an opportunity to retry a deferred
// transaction.
//
// Provides additional configuration required for SoundWire MBQ register maps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_sdw_mbq_cfg {
    pub reg): *mut *mut *mut int (mbq_size)(struct device dev, unsigned int,
    pub reg): *mut *mut *mut bool (deferrable)(struct device dev, unsigned int,
    pub timeout_us: c_ulong,
    pub retry_us: c_ulong,
}

extern "C" {
    pub fn void(context: *mut *mut regmap_hw_free_context)(void) -> typedef;
}
//
// struct regmap_bus - Description of a hardware bus for the register map
// infrastructure.
//
// @fast_io: Register IO is fast. Use a spinlock instead of a mutex
// to perform locking. This field is ignored if custom lock/unlock
// functions are used (see fields lock/unlock of
// struct regmap_config).
// @free_on_exit: kfree this on exit of regmap
// @write: Write operation.
// @gather_write: Write operation with split register/value, return -ENOTSUPP
// if not implemented  on a given device.
// @async_write: Write operation which completes asynchronously, optional and
// must serialise with respect to non-async I/O.
// @reg_write: Write a single register value to the given register address. This
// write operation has to complete when returning from the function.
// @reg_noinc_write: Write multiple register values to the same register. This
// write operation has to complete when returning from the function.
// @reg_update_bits: Update bits operation to be used against volatile
// registers, intended for devices supporting some mechanism
// for setting clearing bits without having to
// read/modify/write.
// @read: Read operation.  Data is returned in the buffer used to transmit
// data.
// @reg_read: Read a single register value from a given register address.
// @reg_noinc_read: Read multiple register values from the same register. This
// read operation has to complete when returning from the function.
// @free_context: Free context.
// @async_alloc: Allocate a regmap_async() structure.
// @read_flag_mask: Mask to be set in the top byte of the register when doing
// a read.
// @reg_format_endian_default: Default endianness for formatted register
// addresses. Used when the regmap_config specifies DEFAULT. If this is
// DEFAULT, BIG is assumed.
// @val_format_endian_default: Default endianness for formatted register
// values. Used when the regmap_config specifies DEFAULT. If this is
// DEFAULT, BIG is assumed.
// @max_raw_read: Max raw read size that can be used on the bus.
// @max_raw_write: Max raw write size that can be used on the bus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_bus {
    pub fast_io: bool,
    pub free_on_exit: bool,
    pub write: regmap_hw_write,
    pub gather_write: regmap_hw_gather_write,
    pub async_write: regmap_hw_async_write,
    pub reg_write: regmap_hw_reg_write,
    pub reg_noinc_write: regmap_hw_reg_noinc_write,
    pub reg_update_bits: regmap_hw_reg_update_bits,
    pub read: regmap_hw_read,
    pub reg_read: regmap_hw_reg_read,
    pub reg_noinc_read: regmap_hw_reg_noinc_read,
    pub free_context: regmap_hw_free_context,
    pub async_alloc: regmap_hw_async_alloc,
    pub read_flag_mask: u8,
    pub reg_format_endian_default: regmap_endian,
    pub val_format_endian_default: regmap_endian,
    pub max_raw_read: usize,
    pub max_raw_write: usize,
}

//
// __regmap_init functions.
//
// These functions take a lock key and name parameter, and should not be called
// directly. Instead, use the regmap_init macros that generate a key and name
// for each call.
//
// Wrapper for regmap_init macros to include a unique lockdep key and name
// for each call. No-op if CONFIG_LOCKDEP is not set.
//
// @fn: Real function to call (in the form __[*_]regmap_init[_*])
// @name: Config variable name (#config in the calling macro)
//

//
// regmap_init() - Initialise register map
//
// @dev: Device that will be interacted with
// @bus: Bus-specific callbacks to use with device
// @bus_context: Data passed to bus-specific callbacks
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.  This function should generally not be called
// directly, it should be called by bus-specific init functions.
//

//
// regmap_init_i2c() - Initialise register map
//
// @i2c: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_mdio() - Initialise register map
//
// @mdio_dev: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_sccb() - Initialise register map
//
// @i2c: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_slimbus() - Initialise register map
//
// @slimbus: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_spi() - Initialise register map
//
// @dev: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_spmi_base() - Create regmap for the Base register space
//
// @dev:	SPMI device that will be interacted with
// @config:	Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_spmi_ext() - Create regmap for Ext register space
//
// @dev:	Device that will be interacted with
// @config:	Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_w1() - Initialise register map
//
// @w1_dev: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_mmio_clk() - Initialise register map with register clock
//
// @dev: Device that will be interacted with
// @clk_id: register clock consumer ID
// @regs: Pointer to memory-mapped IO region
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap. Implies 'fast_io'.
//

//
// regmap_init_mmio() - Initialise register map
//
// @dev: Device that will be interacted with
// @regs: Pointer to memory-mapped IO region
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap. Implies 'fast_io'.
//

//
// regmap_init_ac97() - Initialise AC'97 register map
//
// @ac97: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

extern "C" {
    pub fn regmap_ac97_default_volatile(dev: *mut device, reg: c_uint) -> bool;
}
//
// regmap_init_sdw() - Initialise register map
//
// @sdw: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_sdw_mbq() - Initialise register map
//
// @sdw: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_sdw_mbq_cfg() - Initialise MBQ SDW register map with config
//
// @dev: &struct device that will be interacted with
// @sdw: Soundwire device that will be interacted with
// @config: Configuration for register map
// @mbq_config: Properties for the MBQ registers
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap. The regmap will be automatically freed by the
// device management code.
//

//
// regmap_init_i3c() - Initialise register map
//
// @i3c: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// regmap_init_spi_avmm() - Initialize register map for Intel SPI Slave
// to AVMM Bus Bridge
//
// @spi: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.
//

//
// regmap_init_fsi() - Initialise register map
//
// @fsi_dev: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer to
// a struct regmap.
//

//
// devm_regmap_init() - Initialise managed register map
//
// @dev: Device that will be interacted with
// @bus: Bus-specific callbacks to use with device
// @bus_context: Data passed to bus-specific callbacks
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  This function should generally not be called
// directly, it should be called by bus-specific init functions.  The
// map will be automatically freed by the device management code.
//

//
// devm_regmap_init_i2c() - Initialise managed register map
//
// @i2c: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_mdio() - Initialise managed register map
//
// @mdio_dev: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_sccb() - Initialise managed register map
//
// @i2c: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_spi() - Initialise register map
//
// @dev: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The map will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_spmi_base() - Create managed regmap for Base register space
//
// @dev:	SPMI device that will be interacted with
// @config:	Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_spmi_ext() - Create managed regmap for Ext register space
//
// @dev:	SPMI device that will be interacted with
// @config:	Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_w1() - Initialise managed register map
//
// @w1_dev: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_mmio_clk() - Initialise managed register map with clock
//
// @dev: Device that will be interacted with
// @clk_id: register clock consumer ID
// @regs: Pointer to memory-mapped IO region
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code. Implies 'fast_io'.
//

//
// devm_regmap_init_mmio() - Initialise managed register map
//
// @dev: Device that will be interacted with
// @regs: Pointer to memory-mapped IO region
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code. Implies 'fast_io'.
//

//
// devm_regmap_init_ac97() - Initialise AC'97 register map
//
// @ac97: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_sdw() - Initialise managed register map
//
// @sdw: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap. The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_sdw_mbq() - Initialise managed register map
//
// @sdw: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap. The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_sdw_mbq_cfg() - Initialise managed MBQ SDW register map with config
//
// @dev: Device that will be interacted with
// @sdw: SoundWire Device that will be interacted with
// @config: Configuration for register map
// @mbq_config: Properties for the MBQ registers
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap. The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_slimbus() - Initialise managed register map
//
// @slimbus: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap. The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_i3c() - Initialise managed register map
//
// @i3c: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_spi_avmm() - Initialize register map for Intel SPI Slave
// to AVMM Bus Bridge
//
// @spi: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The map will be automatically freed by the
// device management code.
//

//
// devm_regmap_init_fsi() - Initialise managed register map
//
// @fsi_dev: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//

extern "C" {
    pub fn regmap_mmio_attach_clk(map: *mut regmap, clk: *mut clk) -> c_int;
}
extern "C" {
    pub fn regmap_mmio_detach_clk(map: *mut regmap);
}
extern "C" {
    pub fn regmap_exit(map: *mut regmap);
}
extern "C" {
    pub fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
}
extern "C" {
    pub fn regmap_write_async(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
}
extern "C" {
    pub fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn regmap_read_bypassed(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn regmap_update_bits_base(_arg: map, _arg: reg, _arg: mask, _arg: val, _arg: NULL, _arg: false, _arg: false) -> return;
}
extern "C" {
    pub fn regmap_update_bits_base(_arg: map, _arg: reg, _arg: mask, _arg: val, _arg: NULL, _arg: true, _arg: false) -> return;
}
extern "C" {
    pub fn regmap_update_bits_base(_arg: map, _arg: reg, _arg: mask, _arg: val, _arg: NULL, _arg: false, _arg: true) -> return;
}
// def = 0;
extern "C" {
    pub fn regmap_get_val_bytes(map: *mut regmap) -> c_int;
}
extern "C" {
    pub fn regmap_get_max_register(map: *mut regmap) -> c_int;
}
extern "C" {
    pub fn regmap_get_reg_stride(map: *mut regmap) -> c_int;
}
extern "C" {
    pub fn regmap_might_sleep(map: *mut regmap) -> bool;
}
extern "C" {
    pub fn regmap_async_complete(map: *mut regmap) -> c_int;
}
extern "C" {
    pub fn regmap_can_raw_write(map: *mut regmap) -> bool;
}
extern "C" {
    pub fn regmap_get_raw_read_max(map: *mut regmap) -> usize;
}
extern "C" {
    pub fn regmap_get_raw_write_max(map: *mut regmap) -> usize;
}
extern "C" {
    pub fn regcache_sort_defaults(defaults: *mut reg_default, ndefaults: c_uint);
}
extern "C" {
    pub fn regcache_sync(map: *mut regmap) -> c_int;
}
extern "C" {
    pub fn regcache_cache_only(map: *mut regmap, enable: bool);
}
extern "C" {
    pub fn regcache_cache_bypass(map: *mut regmap, enable: bool);
}
extern "C" {
    pub fn regcache_mark_dirty(map: *mut regmap);
}
extern "C" {
    pub fn regcache_reg_cached(map: *mut regmap, reg: c_uint) -> bool;
}
extern "C" {
    pub fn regmap_update_bits_base(_arg: map, _arg: reg, _arg: bits, _arg: 0, _arg: NULL, _arg: false, _arg: false) -> return;
}
extern "C" {
    pub fn regmap_set_bits(_arg: map, _arg: reg, _arg: bits) -> return;
}
extern "C" {
    pub fn regmap_clear_bits(_arg: map, _arg: reg, _arg: bits) -> return;
}
extern "C" {
    pub fn regmap_test_bits(map: *mut regmap, reg: c_uint, bits: c_uint) -> c_int;
}
//
// struct reg_field - Description of an register field
//
// @reg: Offset of the register within the regmap bank
// @lsb: lsb of the register field.
// @msb: msb of the register field.
// @id_size: port size if it has some ports
// @id_offset: address offset for each ports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_field {
    pub reg: c_uint,
    pub lsb: c_uint,
    pub msb: c_uint,
    pub id_size: c_uint,
    pub id_offset: c_uint,
}

extern "C" {
    pub fn regmap_field_free(field: *mut regmap_field);
}
extern "C" {
    pub fn devm_regmap_field_free(dev: *mut device, field: *mut regmap_field);
}
extern "C" {
    pub fn regmap_field_bulk_free(field: *mut regmap_field);
}
extern "C" {
    pub fn regmap_field_read(field: *mut regmap_field, val: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn regmap_field_update_bits_base(_arg: field, _arg: ~0, _arg: val, _arg: NULL, _arg: false, _arg: true) -> return;
}
extern "C" {
    pub fn regmap_field_test_bits(field: *mut regmap_field, bits: c_uint) -> c_int;
}
//
// struct regmap_irq_type - IRQ type definitions.
//
// @type_reg_offset: Offset register for the irq type setting.
// @type_reg_mask: Device interrupt mask
// @type_rising_val: Register value to configure RISING type irq.
// @type_falling_val: Register value to configure FALLING type irq.
// @type_level_low_val: Register value to configure LEVEL_LOW type irq.
// @type_level_high_val: Register value to configure LEVEL_HIGH type irq.
// @types_supported: logical OR of IRQ_TYPE_* flags indicating supported types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_irq_type {
    pub type_reg_offset: c_uint,
    pub type_reg_mask: c_uint,
    pub type_rising_val: c_uint,
    pub type_falling_val: c_uint,
    pub type_level_low_val: c_uint,
    pub type_level_high_val: c_uint,
    pub types_supported: c_uint,
}

//
// struct regmap_irq - Description of an IRQ for the generic regmap irq_chip.
//
// @reg_offset: Offset of the status/mask register within the bank
// @mask:       Mask used to flag/control the register.
// @type:	IRQ trigger type setting details if supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_irq {
    pub reg_offset: c_uint,
    pub mask: c_uint,
    pub type: regmap_irq_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_irq_sub_irq_map {
    pub num_regs: c_uint,
    pub offset: *mut c_uint,
}

//
// struct regmap_irq_chip - Description of a generic regmap irq_chip.
//
// @name:        Descriptive name for IRQ controller.
// @domain_suffix: Name suffix to be appended to end of IRQ domain name. Needed
// when multiple regmap-IRQ controllers are created from same
// device.
//
// @main_status: Base main status register address. For chips which have
// interrupts arranged in separate sub-irq blocks with own IRQ
// registers and which have a main IRQ registers indicating
// sub-irq blocks with unhandled interrupts. For such chips fill
// sub-irq register information in status_base, mask_base and
// ack_base.
// @num_main_status_bits: Should be given to chips where number of meaningfull
// main status bits differs from num_regs.
// @sub_reg_offsets: arrays of mappings from main register bits to sub irq
// registers. First item in array describes the registers
// for first main status bit. Second array for second bit etc.
// Offset is given as sub register status offset to
// status_base. Should contain num_regs arrays.
// Can be provided for chips with more complex mapping than
// 1.st bit to 1.st sub-reg, 2.nd bit to 2.nd sub-reg, ...
// @num_main_regs: Number of 'main status' irq registers for chips which have
// main_status set.
//
// @status_base: Base status register address.
// @mask_base:   Base mask register address. Mask bits are set to 1 when an
// interrupt is masked, 0 when unmasked.
// @unmask_base:  Base unmask register address. Unmask bits are set to 1 when
// an interrupt is unmasked and 0 when masked.
// @ack_base:    Base ack address. If zero then the chip is clear on read.
// Using zero value is possible with @use_ack bit.
// @wake_base:   Base address for wake enables.  If zero unsupported.
// @config_base: Base address for IRQ type config regs. If null unsupported.
// @irq_reg_stride:  Stride to use for chips where registers are not contiguous.
// @init_ack_masked: Ack all masked interrupts once during initalization.
// @mask_unmask_non_inverted: Controls mask bit inversion for chips that set
// both @mask_base and @unmask_base. If false, mask and unmask bits are
// inverted (which is deprecated behavior); if true, bits will not be
// inverted and the registers keep their normal behavior. Note that if
// you use only one of @mask_base or @unmask_base, this flag has no
// effect and is unnecessary. Any new drivers that set both @mask_base
// and @unmask_base should set this to true to avoid relying on the
// deprecated behavior.
// @use_ack:     Use @ack register even if it is zero.
// @ack_invert:  Inverted ack register: cleared bits for ack.
// @clear_ack:  Use this to set 1 and 0 or vice-versa to clear interrupts.
// @status_invert: Inverted status register: cleared bits are active interrupts.
// @status_is_level: Status register is actuall signal level: Xor status
// register with previous value to get active interrupts.
// @wake_invert: Inverted wake register: cleared bits are wake disabled.
// @type_in_mask: Use the mask registers for controlling irq type. Use this if
// the hardware provides separate bits for rising/falling edge
// or low/high level interrupts and they should be combined into
// a single logical interrupt. Use &struct regmap_irq_type data
// to define the mask bit for each irq type.
// @clear_on_unmask: For chips with interrupts cleared on read: read the status
// registers before unmasking interrupts to clear any bits
// set when they were masked.
// @runtime_pm:  Hold a runtime PM lock on the device when accessing it.
// @no_status: No status register: all interrupts assumed generated by device.
//
// @num_regs:    Number of registers in each control bank.
//
// @irqs:        Descriptors for individual IRQs.  Interrupt numbers are
// assigned based on the index in the array of the interrupt.
// @num_irqs:    Number of descriptors.
// @num_config_bases:	Number of config base registers.
// @num_config_regs:	Number of config registers for each config base register.
//
// @handle_pre_irq:  Driver specific callback to handle interrupt from device
// before regmap_irq_handler process the interrupts.
// @handle_post_irq: Driver specific callback to handle interrupt from device
// after handling the interrupts in regmap_irq_handler().
// @handle_mask_sync: Callback used to handle IRQ mask syncs. The index will be
// in the range [0, num_regs)
// @set_type_config: Callback used for configuring irq types.
// @get_irq_reg: Callback for mapping (base register, index) pairs to register
// addresses. The base register will be one of @status_base,
// @mask_base, etc., @main_status, or any of @config_base.
// The index will be in the range [0, num_main_regs[ for the
// main status base, [0, num_config_regs[ for any config
// register base, and [0, num_regs[ for any other base.
// If unspecified then regmap_irq_get_irq_reg_linear() is used.
// @irq_reqres:	 Callback function to request IRQ resources (may be %NULL)
// @irq_relres:  Callback function to release IRQ resources (may be %NULL)
// @irq_drv_data:    Driver specific IRQ data which is passed as parameter when
// driver specific pre/post interrupt handler is called.
//
// This is not intended to handle every possible interrupt controller, but
// it should handle a substantial proportion of those that are found in the
// wild.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_irq_chip {
    pub name: *const c_char,
    pub domain_suffix: *const c_char,
    pub main_status: c_uint,
    pub num_main_status_bits: c_uint,
    pub sub_reg_offsets: *const regmap_irq_sub_irq_map,
    pub num_main_regs: c_int,
    pub status_base: c_uint,
    pub mask_base: c_uint,
    pub unmask_base: c_uint,
    pub ack_base: c_uint,
    pub wake_base: c_uint,
    pub config_base: *const c_uint,
    pub irq_reg_stride: c_uint,
    pub init_ack_masked:1: c_uint,
    pub mask_unmask_non_inverted:1: c_uint,
    pub use_ack:1: c_uint,
    pub ack_invert:1: c_uint,
    pub clear_ack:1: c_uint,
    pub status_invert:1: c_uint,
    pub status_is_level:1: c_uint,
    pub wake_invert:1: c_uint,
    pub type_in_mask:1: c_uint,
    pub clear_on_unmask:1: c_uint,
    pub runtime_pm:1: c_uint,
    pub no_status:1: c_uint,
    pub num_regs: c_int,
    pub irqs: *const regmap_irq,
    pub num_irqs: c_int,
    pub num_config_bases: c_int,
    pub num_config_regs: c_int,
    pub irq_drv_data): *mut *mut int (handle_pre_irq)(void,
    pub irq_drv_data): *mut *mut int (handle_post_irq)(void,
    pub irq_drv_data): *mut unsigned int mask_buf, void,
    pub irq_drv_data): *mut c_void,
    pub index): unsigned int base, int,
    pub hwirq): *mut *mut *mut int (irq_reqres)(void irq_drv_data, irq_hw_number_t,
    pub hwirq): *mut *mut *mut void (irq_relres)(void irq_drv_data, irq_hw_number_t,
    pub irq_drv_data: *mut c_void,
}

extern "C" {
    pub fn regmap_del_irq_chip(irq: c_int, data: *mut regmap_irq_chip_data);
}
extern "C" {
    pub fn regmap_irq_chip_get_base(data: *mut regmap_irq_chip_data) -> c_int;
}
extern "C" {
    pub fn regmap_irq_get_virq(data: *mut regmap_irq_chip_data, irq: c_int) -> c_int;
}

//
// These stubs should only ever be called by generic code which has
// regmap based facilities, if they ever get called at runtime
// something is going wrong and something probably needs to select
// REGMAP.
//

