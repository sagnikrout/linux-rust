//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i2c.h
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
// i2c.h - definitions for the Linux i2c bus interface
// Copyright (C) 1995-2000 Simon G. Vogl
// Copyright (C) 2013-2019 Wolfram Sang <wsa@kernel.org>
//
// With some changes from Kyösti Mälkki <kmalkki@cc.hut.fi> and
// Frodo Looijaard <frodol@dds.nl>
//

// --- General options ------------------------------------------------
// I2C Frequency Modes
pub const I2C_MAX_STANDARD_MODE_FREQ: c_int = 100000;
pub const I2C_MAX_FAST_MODE_FREQ: c_int = 400000;
pub const I2C_MAX_FAST_MODE_PLUS_FREQ: c_int = 1000000;
pub const I2C_MAX_TURBO_MODE_FREQ: c_int = 1400000;
pub const I2C_MAX_HIGH_SPEED_MODE_FREQ: c_int = 3400000;
pub const I2C_MAX_ULTRA_FAST_MODE_FREQ: c_int = 5000000;

// Return the Frequency mode string based on the bus frequency
//
// The master routines are the ones normally used to transmit data to devices
// on a bus (or read from them). Apart from two basic transfer functions to
// transmit one message at a time, a more complex version can be used to
// transmit an arbitrary number of messages without interruption.
// @count must be less than 64k since msg.len is u16.
//
// i2c_master_recv - issue a single I2C message in master receive mode
// @client: Handle to slave device
// @buf: Where to store data read from slave
// @count: How many bytes to read, must be less than 64k since msg.len is u16
//
// Returns negative errno, or else the number of bytes read.
//
extern "C" {
    pub fn i2c_transfer_buffer_flags(_arg: client, _arg: buf, _arg: count, _arg: I2C_M_RD) -> return;
}
//
// i2c_master_recv_dmasafe - issue a single I2C message in master receive mode
// using a DMA safe buffer
// @client: Handle to slave device
// @buf: Where to store data read from slave, must be safe to use with DMA
// @count: How many bytes to read, must be less than 64k since msg.len is u16
//
// Returns negative errno, or else the number of bytes read.
//
// i2c_master_send - issue a single I2C message in master transmit mode
// @client: Handle to slave device
// @buf: Data that will be written to the slave
// @count: How many bytes to write, must be less than 64k since msg.len is u16
//
// Returns negative errno, or else the number of bytes written.
//
extern "C" {
    pub fn i2c_transfer_buffer_flags(_arg: client, )buf: *mut (char, _arg: count, _arg: 0) -> return;
}
//
// i2c_master_send_dmasafe - issue a single I2C message in master transmit mode
// using a DMA safe buffer
// @client: Handle to slave device
// @buf: Data that will be written to the slave, must be safe to use with DMA
// @count: How many bytes to write, must be less than 64k since msg.len is u16
//
// Returns negative errno, or else the number of bytes written.
//
// Transfer num messages.
//
extern "C" {
    pub fn i2c_transfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
}
// Unlocked flavor
extern "C" {
    pub fn __i2c_transfer(adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: c_int) -> c_int;
}
// This is the very generalized SMBus access routine. You probably do not
// Unlocked flavor
// Now follow the 'nice' access routines. These also document the calling
extern "C" {
    pub fn i2c_smbus_pec(crc: u8, p: *mut u8, count: usize) -> u8;
}
extern "C" {
    pub fn i2c_smbus_read_byte(client: *const i2c_client) -> i32;
}
extern "C" {
    pub fn i2c_smbus_write_byte(client: *const i2c_client, value: u8) -> i32;
}
extern "C" {
    pub fn i2c_smbus_read_byte_data(client: *const i2c_client, command: u8) -> i32;
}
extern "C" {
    pub fn i2c_smbus_read_word_data(client: *const i2c_client, command: u8) -> i32;
}
extern "C" {
    pub fn i2c_smbus_write_word_data(_arg: client, _arg: command, _arg: swab16(value)) -> return;
}
// Returns the number of read bytes

//
// struct i2c_device_identity - i2c client device identification
// @manufacturer_id: 0 - 4095, database maintained by NXP
// @part_id: 0 - 511, according to manufacturer
// @die_revision: 0 - 7, according to manufacturer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_device_identity {
    pub manufacturer_id: u16,
pub const I2C_DEVICE_ID_NXP_SEMICONDUCTORS: c_int = 0;
pub const I2C_DEVICE_ID_NXP_SEMICONDUCTORS_1: c_int = 1;
pub const I2C_DEVICE_ID_NXP_SEMICONDUCTORS_2: c_int = 2;
pub const I2C_DEVICE_ID_NXP_SEMICONDUCTORS_3: c_int = 3;
pub const I2C_DEVICE_ID_RAMTRON_INTERNATIONAL: c_int = 4;
pub const I2C_DEVICE_ID_ANALOG_DEVICES: c_int = 5;
pub const I2C_DEVICE_ID_STMICROELECTRONICS: c_int = 6;
pub const I2C_DEVICE_ID_ON_SEMICONDUCTOR: c_int = 7;
pub const I2C_DEVICE_ID_SPRINTEK_CORPORATION: c_int = 8;
pub const I2C_DEVICE_ID_ESPROS_PHOTONICS_AG: c_int = 9;
pub const I2C_DEVICE_ID_FUJITSU_SEMICONDUCTOR: c_int = 10;
pub const I2C_DEVICE_ID_FLIR: c_int = 11;
pub const I2C_DEVICE_ID_O2MICRO: c_int = 12;
pub const I2C_DEVICE_ID_ATMEL: c_int = 13;
pub const I2C_DEVICE_ID_NONE: c_uint = 0xffff;
    pub part_id: u16,
    pub die_revision: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2c_alert_protocol {
    I2C_PROTOCOL_SMBUS_ALERT,
    I2C_PROTOCOL_SMBUS_HOST_NOTIFY,
}

//
// enum i2c_driver_flags - Flags for an I2C device driver
//
// @I2C_DRV_ACPI_WAIVE_D0_PROBE: Don't put the device in D0 state for probe
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2c_driver_flags {
    I2C_DRV_ACPI_WAIVE_D0_PROBE = BIT(0),
}

//
// struct i2c_driver - represent an I2C device driver
// @class: What kind of i2c device we instantiate (for detect)
// @probe: Callback for device binding
// @remove: Callback for device unbinding
// @shutdown: Callback for device shutdown
// @alert: Alert callback, for example for the SMBus alert protocol
// @command: Callback for bus-wide signaling (optional)
// @driver: Device driver model driver
// @id_table: List of I2C devices supported by this driver
// @detect: Callback for device detection
// @address_list: The I2C addresses to probe (for detect)
// @clients: List of detected clients we created (for i2c-core use only)
// @flags: A bitmask of flags defined in &enum i2c_driver_flags
//
// The driver.owner field should be set to the module owner of this driver.
// The driver.name field should be set to the name of this driver.
//
// For automatic device detection, both @detect and @address_list must
// be defined. @class should also be set, otherwise only devices forced
// with module parameters will be created. The detect function must
// fill at least the name field of the i2c_board_info structure it is
// handed upon successful detection, and possibly also the flags field.
//
// If @detect is missing, the driver will still work fine for enumerated
// devices. Detected devices simply won't be supported. This is expected
// for the many I2C/SMBus devices which can't be detected reliably, and
// the ones which can always be enumerated in practice.
//
// The i2c_client structure which is handed to the @detect callback is
// not a real i2c_client. It is initialized just enough so that you can
// call i2c_smbus_read_byte_data and friends on it. Don't do anything
// else with it. In particular, calling dev_dbg and friends on it is
// not allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_driver {
    pub class: c_uint,
// Standard driver model interfaces
    pub client): *mut *mut int (probe)(struct i2c_client,
    pub client): *mut *mut void (remove)(struct i2c_client,
// driver model interfaces that don't relate to enumeration
    pub client): *mut *mut void (shutdown)(struct i2c_client,
// Alert callback, for example for the SMBus alert protocol.
// The format and meaning of the data value depends on the protocol.
// For the SMBus alert protocol, there is a single bit of data passed
// as the alert response's low bit ("event flag").
// For the SMBus Host Notify protocol, the data corresponds to the
// 16-bit payload data reported by the slave device acting as master.
//
    pub data): c_uint,
// a ioctl like command that can be used to perform specific functions
// with the device.
//
    pub arg): *mut *mut *mut int (command)(struct i2c_client client, unsigned int cmd, void,
    pub driver: device_driver,
    pub id_table: *const i2c_device_id,
// Device detection callback for automatic device creation
    pub info): *mut *mut *mut int (detect)(struct i2c_client client, struct i2c_board_info,
    pub address_list: *const c_ushort,
    pub clients: list_head,
    pub flags: u32,
}

//
// struct i2c_client - represent an I2C slave device
// @flags: see I2C_CLIENT_* for possible flags
// @addr: Address used on the I2C bus connected to the parent adapter.
// @name: Indicates the type of the device, usually a chip name that's
// generic enough to hide second-sourcing and compatible revisions.
// @adapter: manages the bus segment hosting this I2C device
// @dev: Driver model device node for the slave.
// @init_irq: IRQ that was set at initialization
// @irq: indicates the IRQ generated by this device (if any)
// @detected: member of an i2c_driver.clients list or i2c-core's
// userspace_devices list
// @slave_cb: Callback when I2C slave mode of an adapter is used. The adapter
// calls it to pass on slave events to the slave driver.
// @devres_group_id: id of the devres group that will be created for resources
// acquired when probing this device.
// @debugfs: pointer to the debugfs subdirectory which the I2C core created
// for this client.
//
// An i2c_client identifies a single device (i.e. chip) connected to an
// i2c bus. The behaviour exposed to Linux is defined by the driver
// managing the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_client {
    pub /: *mut *mut unsigned short flags; / div., see below,
pub const I2C_CLIENT_PEC: c_uint = 0x04	/* Use Packet Error Checking */;
pub const I2C_CLIENT_TEN: c_uint = 0x10	/* we have a ten bit chip address */;
// Must equal I2C_M_TEN below
pub const I2C_CLIENT_SLAVE: c_uint = 0x20	/* we are the slave */;
pub const I2C_CLIENT_HOST_NOTIFY: c_uint = 0x40	/* We want to use I2C host notify */;
pub const I2C_CLIENT_WAKE: c_uint = 0x80	/* for board_info; true iff can wake */;
pub const I2C_CLIENT_SCCB: c_uint = 0x9000	/* Use Omnivision SCCB protocol */;
// Must match I2C_M_STOP|IGNORE_NAK
    pub /: *mut *mut unsigned short addr; / chip address - NOTE: 7bit,
// addresses are stored in the
// _LOWER_ 7 bits
    pub name: [c_char; I2C_NAME_SIZE],
    pub /: *mut *mut *mut i2c_adapter adapter; / the adapter we sit on,
    pub /: *mut *mut device dev; / the device structure,
    pub /: *mut *mut int init_irq; / irq set at initialization,
    pub /: *mut *mut int irq; / irq issued by device,
    pub detected: list_head,

    pub /: *mut *mut i2c_slave_cb_t slave_cb; / callback for slave mode,

    pub /: *mut *mut *mut void devres_group_id; / ID of probe devres group,
    pub /: *mut *mut *mut dentry debugfs; / per-client debugfs dir,
}

extern "C" {
    pub fn to_i2c_client(_arg: dev) -> return;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &client->dev) -> return;
}
// I2C slave support
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2c_slave_event {
    I2C_SLAVE_READ_REQUESTED,
    I2C_SLAVE_WRITE_REQUESTED,
    I2C_SLAVE_READ_PROCESSED,
    I2C_SLAVE_WRITE_RECEIVED,
    I2C_SLAVE_STOP,
}

extern "C" {
    pub fn i2c_slave_register(client: *mut i2c_client, slave_cb: i2c_slave_cb_t) -> c_int;
}
extern "C" {
    pub fn i2c_slave_unregister(client: *mut i2c_client) -> c_int;
}

extern "C" {
    pub fn i2c_detect_slave_mode(dev: *mut device) -> bool;
}

//
// struct i2c_board_info - template for device creation
// @type: chip type, to initialize i2c_client.name
// @flags: to initialize i2c_client.flags
// @addr: stored in i2c_client.addr
// @dev_name: Overrides the default <busnr>-<addr> dev_name if set
// @platform_data: stored in i2c_client.dev.platform_data
// @fwnode: device node supplied by the platform firmware
// @swnode: software node for the device
// @resources: resources associated with the device
// @num_resources: number of resources in the @resources array
// @irq: stored in i2c_client.irq
//
// I2C doesn't actually support hardware probing, although controllers and
// devices may be able to use I2C_SMBUS_QUICK to tell whether or not there's
// a device at a given address.  Drivers commonly need more information than
// that, such as chip type, configuration, associated IRQ, and so on.
//
// i2c_board_info is used to build tables of information listing I2C devices
// that are present.  This information is used to grow the driver model tree.
// For mainboards this is done statically using i2c_register_board_info();
// bus numbers identify adapters that aren't yet available.  For add-on boards,
// i2c_new_client_device() does this dynamically with the adapter already known.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_board_info {
    pub type: [c_char; I2C_NAME_SIZE],
    pub flags: c_ushort,
    pub addr: c_ushort,
    pub dev_name: *const c_char,
    pub platform_data: *mut c_void,
    pub fwnode: *mut fwnode_handle,
    pub swnode: *const software_node,
    pub resources: *const resource,
    pub num_resources: c_uint,
    pub irq: c_int,
}

//
// I2C_BOARD_INFO - macro used to list an i2c device and its address
// @dev_type: identifies the device type
// @dev_addr: the device's address on the bus.
//
// This macro initializes essential fields of a struct i2c_board_info,
// declaring what has been provided on a particular board.  Optional
// fields (such as associated irq, or device-specific platform_data)
// are provided using conventional syntax.
//

//
// Add-on boards should register/unregister their devices; e.g. a board
// with integrated I2C, a config eeprom, sensors, and a codec that's
// used in conjunction with the primary hardware.
//
// If you don't know the exact address of an I2C device, use this variant
// instead, which can probe for device presence in a list of possible
// addresses. The "probe" callback function is optional. If it is provided,
// it must return 1 on successful probe, 0 otherwise. If it is not provided,
// a default probing method is used.
//
// Common custom probe functions
extern "C" {
    pub fn i2c_probe_func_quick_read(adap: *mut i2c_adapter, addr: c_ushort) -> c_int;
}
extern "C" {
    pub fn i2c_unregister_device(client: *mut i2c_client);
}

// Mainboard arch_initcall() code should register all its I2C devices.
// This is done at arch_initcall time, before declaring any i2c adapters.
// Modules for add-on boards must use other calls.
//

//
// struct i2c_algorithm - represent I2C transfer methods
// @xfer: Transfer a given number of messages defined by the msgs array via
// the specified adapter.
// @xfer_atomic: Same as @xfer. Yet, only using atomic context so e.g. PMICs
// can be accessed very late before shutdown. Optional.
// @smbus_xfer: Issue SMBus transactions to the given I2C adapter. If this
// is not present, then the bus layer will try and convert the SMBus calls
// into I2C transfers instead.
// @smbus_xfer_atomic: Same as @smbus_xfer. Yet, only using atomic context
// so e.g. PMICs can be accessed very late before shutdown. Optional.
// @functionality: Return the flags that this algorithm/adapter pair supports
// from the ``I2C_FUNC_*`` flags.
// @reg_target: Register given client to local target mode of this adapter
// @unreg_target: Unregister given client from local target mode of this adapter
//
// @master_xfer: deprecated, use @xfer
// @master_xfer_atomic: deprecated, use @xfer_atomic
// @reg_slave: deprecated, use @reg_target
// @unreg_slave: deprecated, use @unreg_target
//
// i2c_algorithm is the interface to a class of hardware solutions which can
// be addressed using the same bus algorithms - i.e. bit-banging or the PCF8584
// to name two of the most common.
//
// The return codes from the ``xfer{_atomic}`` fields should indicate the
// type of error code that occurred during the transfer, as documented in the
// Kernel Documentation file Documentation/i2c/fault-codes.rst. Otherwise, the
// number of messages executed should be returned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_algorithm {
//
// If an adapter algorithm can't do I2C-level access, set xfer
// to NULL. If an adapter algorithm can do SMBus access, set
// smbus_xfer. If set to NULL, the SMBus protocol is simulated
// using common I2C messages.
//
    pub num): c_int,
    pub num): c_int,
}

// To determine what the adapter supports

//
// struct i2c_lock_operations - represent I2C locking operations
// @lock_bus: Get exclusive access to an I2C bus segment
// @trylock_bus: Try to get exclusive access to an I2C bus segment
// @unlock_bus: Release exclusive access to an I2C bus segment
//
// The main operations are wrapped by i2c_lock_bus and i2c_unlock_bus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_lock_operations {
    pub flags): *mut *mut *mut void (lock_bus)(struct i2c_adapter adapter, unsigned int,
    pub flags): *mut *mut *mut int (trylock_bus)(struct i2c_adapter adapter, unsigned int,
    pub flags): *mut *mut *mut void (unlock_bus)(struct i2c_adapter adapter, unsigned int,
}

//
// struct i2c_timings - I2C timing information
// @bus_freq_hz: the bus frequency in Hz
// @scl_rise_ns: time SCL signal takes to rise in ns; t(r) in the I2C specification
// @scl_fall_ns: time SCL signal takes to fall in ns; t(f) in the I2C specification
// @scl_int_delay_ns: time IP core additionally needs to setup SCL in ns
// @sda_fall_ns: time SDA signal takes to fall in ns; t(f) in the I2C specification
// @sda_hold_ns: time IP core additionally needs to hold SDA in ns
// @digital_filter_width_ns: width in ns of spikes on i2c lines that the IP core
// digital filter can filter out
// @analog_filter_cutoff_freq_hz: threshold frequency for the low pass IP core
// analog filter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_timings {
    pub bus_freq_hz: u32,
    pub scl_rise_ns: u32,
    pub scl_fall_ns: u32,
    pub scl_int_delay_ns: u32,
    pub sda_fall_ns: u32,
    pub sda_hold_ns: u32,
    pub digital_filter_width_ns: u32,
    pub analog_filter_cutoff_freq_hz: u32,
}

//
// struct i2c_bus_recovery_info - I2C bus recovery information
// @recover_bus: Recover routine. Either pass driver's recover_bus() routine, or
// i2c_generic_scl_recovery().
// @get_scl: This gets current value of SCL line. Mandatory for generic SCL
// recovery. Populated internally for generic GPIO recovery.
// @set_scl: This sets/clears the SCL line. Mandatory for generic SCL recovery.
// Populated internally for generic GPIO recovery.
// @get_sda: This gets current value of SDA line. This or set_sda() is mandatory
// for generic SCL recovery. Populated internally, if sda_gpio is a valid
// GPIO, for generic GPIO recovery.
// @set_sda: This sets/clears the SDA line. This or get_sda() is mandatory for
// generic SCL recovery. Populated internally, if sda_gpio is a valid GPIO,
// for generic GPIO recovery.
// @get_bus_free: Returns the bus free state as seen from the IP core in case it
// has a more complex internal logic than just reading SDA. Optional.
// @prepare_recovery: This will be called before starting recovery. Platform may
// configure padmux here for SDA/SCL line or something else they want.
// @unprepare_recovery: This will be called after completing recovery. Platform
// may configure padmux here for SDA/SCL line or something else they want.
// @scl_gpiod: gpiod of the SCL line. Only required for GPIO recovery.
// @sda_gpiod: gpiod of the SDA line. Only required for GPIO recovery.
// @pinctrl: pinctrl used by GPIO recovery to change the state of the I2C pins.
// Optional.
// @pins_default: default pinctrl state of SCL/SDA lines, when they are assigned
// to the I2C bus. Optional. Populated internally for GPIO recovery, if
// state with the name PINCTRL_STATE_DEFAULT is found and pinctrl is valid.
// @pins_gpio: recovery pinctrl state of SCL/SDA lines, when they are used as
// GPIOs. Optional. Populated internally for GPIO recovery, if this state
// is called "gpio" or "recovery" and pinctrl is valid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_bus_recovery_info {
    pub adap): *mut *mut int (recover_bus)(struct i2c_adapter,
    pub adap): *mut *mut int (get_scl)(struct i2c_adapter,
    pub val): *mut *mut *mut void (set_scl)(struct i2c_adapter adap, int,
    pub adap): *mut *mut int (get_sda)(struct i2c_adapter,
    pub val): *mut *mut *mut void (set_sda)(struct i2c_adapter adap, int,
    pub adap): *mut *mut int (get_bus_free)(struct i2c_adapter,
    pub adap): *mut *mut void (prepare_recovery)(struct i2c_adapter,
    pub adap): *mut *mut void (unprepare_recovery)(struct i2c_adapter,
// gpio recovery
    pub scl_gpiod: *mut gpio_desc,
    pub sda_gpiod: *mut gpio_desc,
    pub pinctrl: *mut pinctrl,
    pub pins_default: *mut pinctrl_state,
    pub pins_gpio: *mut pinctrl_state,
}

extern "C" {
    pub fn i2c_recover_bus(adap: *mut i2c_adapter) -> c_int;
}
// Generic recovery routines
extern "C" {
    pub fn i2c_generic_scl_recovery(adap: *mut i2c_adapter) -> c_int;
}
//
// struct i2c_adapter_quirks - describe flaws of an i2c adapter
// @flags: see I2C_AQ_* for possible flags and read below
// @max_num_msgs: maximum number of messages per transfer
// @max_write_len: maximum length of a write message
// @max_read_len: maximum length of a read message
// @max_comb_1st_msg_len: maximum length of the first msg in a combined message
// @max_comb_2nd_msg_len: maximum length of the second msg in a combined message
//
// Note about combined messages: Some I2C controllers can only send one message
// per transfer, plus something called combined message or write-then-read.
// This is (usually) a small write message followed by a read message and
// barely enough to access register based devices like EEPROMs. There is a flag
// to support this mode. It implies max_num_msg = 2 and does the length checks
// with max_comb_*_len because combined message mode usually has its own
// limitations. Because of HW implementations, some controllers can actually do
// write-then-anything or other variants. To support that, write-then-read has
// been broken out into smaller bits like write-first and read-second which can
// be combined as needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_adapter_quirks {
    pub flags: u64,
    pub max_num_msgs: c_int,
    pub max_write_len: u16,
    pub max_read_len: u16,
    pub max_comb_1st_msg_len: u16,
    pub max_comb_2nd_msg_len: u16,
}

// enforce max_num_msgs = 2 and use max_comb_*_len for length checks

// first combined message must be write

// second combined message must be read

// both combined messages must have the same target address

// convenience macro for typical write-then read case

// clock stretching is not supported

// message cannot have length of 0

// adapter cannot do repeated START

//
// i2c_adapter is the structure used to identify a physical i2c bus along
// with the access algorithms necessary to access it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_adapter {
    pub owner: *mut module,
    pub /: *mut *mut unsigned int class; / classes to allow probing for,
    pub /: *const *const *const i2c_algorithm algo; / the algorithm to access the bus,
    pub algo_data: *mut c_void,
// data fields that are valid for all devices
    pub lock_ops: *const i2c_lock_operations,
    pub bus_lock: rt_mutex,
    pub mux_lock: rt_mutex,
    pub /: *mut *mut int timeout; / in jiffies,
    pub retries: c_int,
    pub /: *mut *mut device dev; / the adapter device,
    pub /: *mut *mut unsigned long locked_flags; / owned by the I2C core,
pub const I2C_ALF_IS_SUSPENDED: c_int = 0;
pub const I2C_ALF_SUSPEND_REPORTED: c_int = 1;
    pub nr: c_int,
    pub name: [c_char; 48],
    pub dev_released: completion,
    pub userspace_clients_lock: mutex,
    pub userspace_clients: list_head,
    pub bus_recovery_info: *mut i2c_bus_recovery_info,
    pub quirks: *const i2c_adapter_quirks,
    pub host_notify_domain: *mut irq_domain,
    pub bus_regulator: *mut regulator,
    pub debugfs: *mut dentry,
// 7bit address space
    pub 7): DECLARE_BITMAP(addrs_in_instantiation, 1 <<,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &adap->dev) -> return;
}

extern "C" {
    pub fn to_i2c_adapter(_arg: parent) -> return;
}

extern "C" {
    pub fn i2c_for_each_dev(data: *mut c_void, dev: *mut *mut int (fn)(struct device, data): *mut c_void) -> c_int;
}
// Adapter locking functions, exported for shared pin cases

//
// i2c_lock_bus - Get exclusive access to an I2C bus segment
// @adapter: Target I2C bus segment
// @flags: I2C_LOCK_ROOT_ADAPTER locks the root i2c adapter, I2C_LOCK_SEGMENT
// locks only this branch in the adapter tree
//
// i2c_trylock_bus - Try to get exclusive access to an I2C bus segment
// @adapter: Target I2C bus segment
// @flags: I2C_LOCK_ROOT_ADAPTER tries to locks the root i2c adapter,
// I2C_LOCK_SEGMENT tries to lock only this branch in the adapter tree
//
// Return: true if the I2C bus segment is locked, false otherwise
//
// i2c_unlock_bus - Release exclusive access to an I2C bus segment
// @adapter: Target I2C bus segment
// @flags: I2C_LOCK_ROOT_ADAPTER unlocks the root i2c adapter, I2C_LOCK_SEGMENT
// unlocks only this branch in the adapter tree
//
// i2c_mark_adapter_suspended - Report suspended state of the adapter to the core
// @adap: Adapter to mark as suspended
//
// When using this helper to mark an adapter as suspended, the core will reject
// further transfers to this adapter. The usage of this helper is optional but
// recommended for devices having distinct handlers for system suspend and
// runtime suspend. More complex devices are free to implement custom solutions
// to reject transfers when suspended.
//
// i2c_mark_adapter_resumed - Report resumed state of the adapter to the core
// @adap: Adapter to mark as resumed
//
// When using this helper to mark an adapter as resumed, the core will allow
// further transfers to this adapter. See also further notes to
// @i2c_mark_adapter_suspended().
//
// i2c adapter classes (bitmask)

// Warn users that the adapter doesn't support classes anymore

// Internal numbers to terminate lists
pub const I2C_CLIENT_END: c_uint = 0xfffeU;
// Construct an I2C_CLIENT_END-terminated array of i2c addresses

// ----- functions exported by i2c.o
// administration...
//

extern "C" {
    pub fn i2c_add_adapter(adap: *mut i2c_adapter) -> c_int;
}
extern "C" {
    pub fn devm_i2c_add_adapter(dev: *mut device, adapter: *mut i2c_adapter) -> c_int;
}
extern "C" {
    pub fn i2c_del_adapter(adap: *mut i2c_adapter);
}
extern "C" {
    pub fn i2c_add_numbered_adapter(adap: *mut i2c_adapter) -> c_int;
}
extern "C" {
    pub fn i2c_register_driver(owner: *mut module, driver: *mut i2c_driver) -> c_int;
}
extern "C" {
    pub fn i2c_del_driver(driver: *mut i2c_driver);
}
// use a define to avoid include chaining to get THIS_MODULE

// call the i2c_client->command() of all attached clients with
// the given arguments
extern "C" {
    pub fn i2c_put_adapter(adap: *mut i2c_adapter);
}
extern "C" {
    pub fn i2c_adapter_depth(adapter: *mut i2c_adapter) -> c_uint;
}
extern "C" {
    pub fn i2c_parse_fw_timings(dev: *mut device, t: *mut i2c_timings, use_defaults: bool);
}
// Return the functionality mask
// Return 1 if adapter supports everything we need, 0 if not.
//
// i2c_check_quirks() - Function for checking the quirk flags in an i2c adapter
// @adap: i2c adapter
// @quirks: quirk flags
//
// Return: true if the adapter has all the specified quirk flags, false if not
//
// Return the adapter number for a specific adapter
//
// 10-bit address
// addr_1: 5'b11110 | addr[9:8] | (R/nW)
// addr_2: addr[7:0]
//
extern "C" {
    pub fn i2c_put_dma_safe_msg_buf(buf: *mut u8, msg: *mut i2c_msg, xferred: bool);
}
extern "C" {
    pub fn i2c_handle_smbus_host_notify(adap: *mut i2c_adapter, addr: c_ushort) -> c_int;
}
//
// module_i2c_driver() - Helper macro for registering a modular I2C driver
// @__i2c_driver: i2c_driver struct
//
// Helper macro for I2C drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

//
// builtin_i2c_driver() - Helper macro for registering a builtin I2C driver
// @__i2c_driver: i2c_driver struct
//
// Helper macro for I2C drivers which do not do anything special in their
// init. This eliminates a lot of boilerplate. Each driver may only
// use this macro once, and calling it replaces device_initcall().
//

// must call put_device() when done with returned i2c_client device
// must call put_device() when done with returned i2c_adapter device
// must call i2c_put_adapter() when done with returned i2c_adapter device

// must call put_device() when done with returned i2c_client device
extern "C" {
    pub fn i2c_find_device_by_fwnode(_arg: of_fwnode_handle(node)) -> return;
}
// must call put_device() when done with returned i2c_adapter device
extern "C" {
    pub fn i2c_find_adapter_by_fwnode(_arg: of_fwnode_handle(node)) -> return;
}
// must call i2c_put_adapter() when done with returned i2c_adapter device
extern "C" {
    pub fn i2c_get_adapter_by_fwnode(_arg: of_fwnode_handle(node)) -> return;
}

extern "C" {
    pub fn i2c_acpi_client_count(adev: *mut acpi_device) -> c_int;
}
extern "C" {
    pub fn i2c_acpi_find_bus_speed(dev: *mut device) -> u32;
}
extern "C" {
    pub fn i2c_acpi_waive_d0_probe(dev: *mut device) -> bool;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn i2c_acpi_new_device_by_fwnode(_arg: dev_fwnode(dev), _arg: index, _arg: info) -> return;
}
