//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i3c/master.h
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
// Copyright (C) 2018 Cadence Design Systems Inc.
//
// Author: Boris Brezillon <boris.brezillon@bootlin.com>
//

pub const I3C_HOT_JOIN_ADDR: c_uint = 0x2;
pub const I3C_BROADCAST_ADDR: c_uint = 0x7e;

// notifier actions. notifier call data is the struct i3c_bus
//
// struct i3c_i2c_dev_desc - Common part of the I3C/I2C device descriptor
// @node: node element used to insert the slot into the I2C or I3C device
// list
// @master: I3C master that instantiated this device. Will be used to do
// I2C/I3C transfers
// @master_priv: master private data assigned to the device. Can be used to
// add master specific information
//
// This structure is describing common I3C/I2C dev information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_i2c_dev_desc {
    pub node: list_head,
    pub master: *mut i3c_master_controller,
    pub master_priv: *mut c_void,
}

//
// struct i2c_dev_boardinfo - I2C device board information
// @node: used to insert the boardinfo object in the I2C boardinfo list
// @base: regular I2C board information
// @lvr: LVR (Legacy Virtual Register) needed by the I3C core to know about
// the I2C device limitations
//
// This structure is used to attach board-level information to an I2C device.
// Each I2C device connected on the I3C bus should have one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_dev_boardinfo {
    pub node: list_head,
    pub base: i2c_board_info,
    pub lvr: u8,
}

//
// struct i2c_dev_desc - I2C device descriptor
// @common: common part of the I2C device descriptor
// @dev: I2C device object registered to the I2C framework
// @addr: I2C device address
// @lvr: LVR (Legacy Virtual Register) needed by the I3C core to know about
// the I2C device limitations
//
// Each I2C device connected on the bus will have an i2c_dev_desc.
// This object is created by the core and later attached to the controller
// using &struct_i3c_master_controller->ops->attach_i2c_dev().
//
// &struct_i2c_dev_desc is the internal representation of an I2C device
// connected on an I3C bus. This object is also passed to all
// &struct_i3c_master_controller_ops hooks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_dev_desc {
    pub common: i3c_i2c_dev_desc,
    pub dev: *mut i2c_client,
    pub addr: u16,
    pub lvr: u8,
}

//
// struct i3c_ibi_slot - I3C IBI (In-Band Interrupt) slot
// @work: work associated to this slot. The IBI handler will be called from
// there
// @dev: the I3C device that has generated this IBI
// @len: length of the payload associated to this IBI
// @data: payload buffer
//
// An IBI slot is an object pre-allocated by the controller and used when an
// IBI comes in.
// Every time an IBI comes in, the I3C master driver should find a free IBI
// slot in its IBI slot pool, retrieve the IBI payload and queue the IBI using
// i3c_master_queue_ibi().
//
// How IBI slots are allocated is left to the I3C master driver, though, for
// simple kmalloc-based allocation, the generic IBI slot pool can be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_ibi_slot {
    pub work: work_struct,
    pub dev: *mut i3c_dev_desc,
    pub len: c_uint,
    pub data: *mut c_void,
}

//
// struct i3c_device_ibi_info - IBI information attached to a specific device
// @all_ibis_handled: used to be informed when no more IBIs are waiting to be
// processed. Used by i3c_device_disable_ibi() to wait for
// all IBIs to be dequeued
// @pending_ibis: count the number of pending IBIs. Each pending IBI has its
// work element queued to the controller workqueue
// @max_payload_len: maximum payload length for an IBI coming from this device.
// this value is specified when calling
// i3c_device_request_ibi() and should not change at run
// time. All messages IBIs exceeding this limit should be
// rejected by the master
// @num_slots: number of IBI slots reserved for this device
// @enabled: reflect the IBI status
// @wq: workqueue used to execute IBI handlers.
// @handler: IBI handler specified at i3c_device_request_ibi() call time. This
// handler will be called from the controller workqueue, and as such
// is allowed to sleep (though it is recommended to process the IBI
// as fast as possible to not stall processing of other IBIs queued
// on the same workqueue).
// New I3C messages can be sent from the IBI handler
//
// The &struct_i3c_device_ibi_info object is allocated when
// i3c_device_request_ibi() is called and attached to a specific device. This
// object is here to manage IBIs coming from a specific I3C device.
//
// Note that this structure is the generic view of the IBI management
// infrastructure. I3C master drivers may have their own internal
// representation which they can associate to the device using
// controller-private data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_device_ibi_info {
    pub all_ibis_handled: completion,
    pub pending_ibis: core::sync::atomic::AtomicI32,
    pub max_payload_len: c_uint,
    pub num_slots: c_uint,
    pub enabled: c_uint,
    pub wq: *mut workqueue_struct,
    pub payload): *const i3c_ibi_payload,
}

//
// struct i3c_dev_boardinfo - I3C device board information
// @node: used to insert the boardinfo object in the I3C boardinfo list
// @init_dyn_addr: initial dynamic address requested by the FW. We provide no
// guarantee that the device will end up using this address,
// but try our best to assign this specific address to the
// device
// @static_addr: static address the I3C device listen on before it's been
// assigned a dynamic address by the master. Will be used during
// bus initialization to assign it a specific dynamic address
// before starting DAA (Dynamic Address Assignment)
// @static_addr_method: Bitmap describing which methods of Dynamic Address
// Assignment from a Static Address are supported by this I3C Target.
// A value of 1 in a bit position indicates that the I3C target
// supports that method, and a value of 0 indicates that the I3C
// target does not support that method.
// Bit 0: SETDASA
// Bit 1: SETAASA
// All other bits are reserved.
// @pid: I3C Provisioned ID exposed by the device. This is a unique identifier
// that may be used to attach boardinfo to i3c_dev_desc when the device
// does not have a static address
// @fwnode: Firmware node (DT or ACPI) in case the device has been
// described in firmware
//
// This structure is used to attach board-level information to an I3C device.
// Not all I3C devices connected on the bus will have a boardinfo. It's only
// needed if you want to attach extra resources to a device or assign it a
// specific dynamic address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_dev_boardinfo {
    pub node: list_head,
    pub init_dyn_addr: u8,
    pub static_addr: u8,
    pub static_addr_method: u8,
    pub pid: u64,
    pub fwnode: *mut fwnode_handle,
}

//
// struct i3c_dev_desc - I3C device descriptor
// @common: common part of the I3C device descriptor
// @info: I3C device information. Will be automatically filled when you create
// your device with i3c_master_add_i3c_dev_locked()
// @ibi_lock: lock used to protect the &struct_i3c_device->ibi
// @ibi: IBI info attached to a device. Should be NULL until
// i3c_device_request_ibi() is called
// @dev: pointer to the I3C device object exposed to I3C device drivers. This
// should never be accessed from I3C master controller drivers. Only core
// code should manipulate it in when updating the dev <-> desc link or
// when propagating IBI events to the driver
// @boardinfo: pointer to the boardinfo attached to this I3C device
//
// Internal representation of an I3C device. This object is only used by the
// core and passed to I3C master controller drivers when they're requested to
// do some operations on the device.
// The core maintains the link between the internal I3C dev descriptor and the
// object exposed to the I3C device drivers (&struct_i3c_device).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_dev_desc {
    pub common: i3c_i2c_dev_desc,
    pub info: i3c_device_info,
    pub ibi_lock: mutex,
    pub ibi: *mut i3c_device_ibi_info,
    pub dev: *mut i3c_device,
    pub boardinfo: *const i3c_dev_boardinfo,
}

//
// struct i3c_device - I3C device object
// @dev: device object to register the I3C dev to the device model
// @desc: pointer to an i3c device descriptor object. This link is updated
// every time the I3C device is rediscovered with a different dynamic
// address assigned
// @bus: I3C bus this device is attached to
// @node: unregistered device list node, only for use by
// i3c_master_register_new_i3c_devs(), it is not protected by a lock
//
// I3C device object exposed to I3C device drivers. The takes care of linking
// this object to the relevant &struct_i3c_dev_desc one.
// All I3C devs on the I3C bus are represented, including I3C masters. For each
// of them, we have an instance of &struct i3c_device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_device {
    pub dev: device,
    pub desc: *mut i3c_dev_desc,
    pub bus: *mut i3c_bus,
    pub node: list_head,
}

//
// The I3C specification says the maximum number of devices connected on the
// bus is 11, but this number depends on external parameters like trace length,
// capacitive load per Device, and the types of Devices present on the Bus.
// I3C master can also have limitations, so this number is just here as a
// reference and should be adjusted on a per-controller/per-board basis.
//
pub const I3C_BUS_MAX_DEVS: c_int = 11;
// Taken from the I3C Spec V1.1.1, chapter 6.2. "Timing specification"
pub const I3C_BUS_I2C_FM_PLUS_SCL_MAX_RATE: c_int = 1000000;
pub const I3C_BUS_I2C_FM_SCL_MAX_RATE: c_int = 400000;
pub const I3C_BUS_I3C_SCL_MAX_RATE: c_int = 12900000;
pub const I3C_BUS_I3C_SCL_TYP_RATE: c_int = 12500000;
pub const I3C_BUS_TAVAL_MIN_NS: c_int = 1000;
pub const I3C_BUS_TBUF_MIXED_FM_MIN_NS: c_int = 1300;
pub const I3C_BUS_THIGH_MIXED_MAX_NS: c_int = 41;
pub const I3C_BUS_TIDLE_MIN_NS: c_int = 200000;
pub const I3C_BUS_TLOW_OD_MIN_NS: c_int = 200;
pub const I3C_BUS_THIGH_INIT_OD_MIN_NS: c_int = 200;
//
// enum i3c_bus_mode - I3C bus mode
// @I3C_BUS_MODE_PURE: only I3C devices are connected to the bus. No limitation
// expected
// @I3C_BUS_MODE_MIXED_FAST: I2C devices with 50ns spike filter are present on
// the bus. The only impact in this mode is that the
// high SCL pulse has to stay below 50ns to trick I2C
// devices when transmitting I3C frames
// @I3C_BUS_MODE_MIXED_LIMITED: I2C devices without 50ns spike filter are
// present on the bus. However they allow
// compliance up to the maximum SDR SCL clock
// frequency.
// @I3C_BUS_MODE_MIXED_SLOW: I2C devices without 50ns spike filter are present
// on the bus
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_bus_mode {
    I3C_BUS_MODE_PURE,
    I3C_BUS_MODE_MIXED_FAST,
    I3C_BUS_MODE_MIXED_LIMITED,
    I3C_BUS_MODE_MIXED_SLOW,
}

//
// enum i3c_open_drain_speed - I3C open-drain speed
// @I3C_OPEN_DRAIN_SLOW_SPEED: Slow open-drain speed for sending the first
// broadcast address. The first broadcast address at this speed
// will be visible to all devices on the I3C bus. I3C devices
// working in I2C mode will turn off their spike filter when
// switching into I3C mode.
// @I3C_OPEN_DRAIN_NORMAL_SPEED: Normal open-drain speed in I3C bus mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_open_drain_speed {
    I3C_OPEN_DRAIN_SLOW_SPEED,
    I3C_OPEN_DRAIN_NORMAL_SPEED,
}

//
// enum i3c_addr_slot_status - I3C address slot status
// @I3C_ADDR_SLOT_FREE: address is free
// @I3C_ADDR_SLOT_RSVD: address is reserved
// @I3C_ADDR_SLOT_I2C_DEV: address is assigned to an I2C device
// @I3C_ADDR_SLOT_I3C_DEV: address is assigned to an I3C device
// @I3C_ADDR_SLOT_STATUS_MASK: address slot mask
// @I3C_ADDR_SLOT_EXT_STATUS_MASK: address slot mask with extended information
// @I3C_ADDR_SLOT_EXT_DESIRED: the bitmask represents addresses that are preferred by some devices,
// such as the "assigned-address" property in a device tree source.
// On an I3C bus, addresses are assigned dynamically, and we need to know which
// addresses are free to use and which ones are already assigned.
//
// Addresses marked as reserved are those reserved by the I3C protocol
// (broadcast address, ...).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i3c_addr_slot_status {
    I3C_ADDR_SLOT_FREE,
    I3C_ADDR_SLOT_RSVD,
    I3C_ADDR_SLOT_I2C_DEV,
    I3C_ADDR_SLOT_I3C_DEV,
    I3C_ADDR_SLOT_STATUS_MASK = 3,
    I3C_ADDR_SLOT_EXT_STATUS_MASK = 7,
    I3C_ADDR_SLOT_EXT_DESIRED = BIT(2),
}

pub const I3C_ADDR_SLOT_STATUS_BITS: c_int = 4;
//
// struct i3c_bus - I3C bus object
// @cur_master: I3C master currently driving the bus. Since I3C is multi-master
// this can change over the time. Will be used to let a master
// know whether it needs to request bus ownership before sending
// a frame or not
// @id: bus ID. Assigned by the framework when register the bus
// @addrslots: a bitmap with 2-bits per-slot to encode the address status and
// ease the DAA (Dynamic Address Assignment) procedure (see
// &enum i3c_addr_slot_status)
// @mode: bus mode (see &enum i3c_bus_mode)
// @scl_rate.i3c: maximum rate for the clock signal when doing I3C SDR/priv
// transfers
// @scl_rate.i2c: maximum rate for the clock signal when doing I2C transfers
// @scl_rate: SCL signal rate for I3C and I2C mode
// @devs.i3c: contains a list of I3C device descriptors representing I3C
// devices connected on the bus and successfully attached to the
// I3C master
// @devs.i2c: contains a list of I2C device descriptors representing I2C
// devices connected on the bus and successfully attached to the
// I3C master
// @devs: 2 lists containing all I3C/I2C devices connected to the bus
// @lock: read/write lock on the bus. This is needed to protect against
// operations that have an impact on the whole bus and the devices
// connected to it. For example, when asking slaves to drop their
// dynamic address (RSTDAA CCC), we need to make sure no one is trying
// to send I3C frames to these devices.
// Note that this lock does not protect against concurrency between
// devices: several drivers can send different I3C/I2C frames through
// the same master in parallel. This is the responsibility of the
// master to guarantee that frames are actually sent sequentially and
// not interlaced
//
// The I3C bus is represented with its own object and not implicitly described
// by the I3C master to cope with the multi-master functionality, where one bus
// can be shared amongst several masters, each of them requesting bus ownership
// when they need to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_bus {
    pub cur_master: *mut i3c_dev_desc,
    pub id: c_int,
    pub BITS_PER_LONG]: *mut *mut unsigned long addrslots[((I2C_MAX_ADDR + 1)  I3C_ADDR_SLOT_STATUS_BITS) /,
    pub mode: i3c_bus_mode,
    pub i3c: c_ulong,
    pub i2c: c_ulong,
    pub scl_rate: },
    pub i3c: list_head,
    pub i2c: list_head,
    pub devs: },
    pub lock: rw_semaphore,
}

//
// struct i3c_master_controller_ops - I3C master methods
// @bus_init: hook responsible for the I3C bus initialization. You should at
// least call master_set_info() from there and set the bus mode.
// You can also put controller specific initialization in there.
// This method is mandatory.
// @bus_cleanup: cleanup everything done in
// &i3c_master_controller_ops->bus_init().
// This method is optional.
// @attach_i3c_dev: called every time an I3C device is attached to the bus. It
// can be after a DAA or when a device is statically declared
// by the FW, in which case it will only have a static address
// and the dynamic address will be 0.
// When this function is called, device information have not
// been retrieved yet.
// This is a good place to attach master controller specific
// data to I3C devices.
// This method is optional.
// @reattach_i3c_dev: called every time an I3C device has its addressed
// changed. It can be because the device has been powered
// down and has lost its address, or it can happen when a
// device had a static address and has been assigned a
// dynamic address with SETDASA.
// This method is optional.
// @detach_i3c_dev: called when an I3C device is detached from the bus. Usually
// happens when the master device is unregistered.
// This method is optional.
// @do_daa: do a DAA (Dynamic Address Assignment) procedure. This is procedure
// should send an ENTDAA CCC command and then add all devices
// discovered sure the DAA using i3c_master_add_i3c_dev_locked().
// Add devices added with i3c_master_add_i3c_dev_locked() will then be
// attached or re-attached to the controller.
// This method is mandatory.
// @supports_ccc_cmd: should return true if the CCC command is supported, false
// otherwise.
// This method is optional, if not provided the core assumes
// all CCC commands are supported.
// @send_ccc_cmd: send a CCC command
// This method is mandatory.
// @i3c_xfers: do one or several I3C SDR or HDR transfers.
// This method is mandatory.
// @attach_i2c_dev: called every time an I2C device is attached to the bus.
// This is a good place to attach master controller specific
// data to I2C devices.
// This method is optional.
// @detach_i2c_dev: called when an I2C device is detached from the bus. Usually
// happens when the master device is unregistered.
// This method is optional.
// @i2c_xfers: do one or several I2C transfers. Note that, unlike i3c
// transfers, the core does not guarantee that buffers attached to
// the transfers are DMA-safe. If drivers want to have DMA-safe
// buffers, they should use the i2c_get_dma_safe_msg_buf()
// and i2c_put_dma_safe_msg_buf() helpers provided by the I2C
// framework.
// This method is mandatory.
// @request_ibi: attach an IBI handler to an I3C device. This implies defining
// an IBI handler and the constraints of the IBI (maximum payload
// length and number of pre-allocated slots).
// Some controllers support less IBI-capable devices than regular
// devices, so this method might return -%EBUSY if there's no
// more space for an extra IBI registration
// This method is optional.
// @free_ibi: free an IBI previously requested with ->request_ibi(). The IBI
// should have been disabled with ->disable_irq() prior to that
// This method is mandatory only if ->request_ibi is not NULL.
// @enable_ibi: enable the IBI. Only valid if ->request_ibi() has been called
// prior to ->enable_ibi(). The controller should first enable
// the IBI on the controller end (for example, unmask the hardware
// IRQ) and then send the ENEC CCC command (with the IBI flag set)
// to the I3C device.
// This method is mandatory only if ->request_ibi is not NULL.
// @disable_ibi: disable an IBI. First send the DISEC CCC command with the IBI
// flag set and then deactivate the hardware IRQ on the
// controller end.
// This method is mandatory only if ->request_ibi is not NULL.
// @recycle_ibi_slot: recycle an IBI slot. Called every time an IBI has been
// processed by its handler. The IBI slot should be put back
// in the IBI slot pool so that the controller can re-use it
// for a future IBI
// This method is mandatory only if ->request_ibi is not
// NULL.
// @enable_hotjoin: enable hot join event detect.
// @disable_hotjoin: disable hot join event detect.
// @set_speed: adjust I3C open drain mode timing.
// @set_dev_nack_retry: configure device NACK retry count for the master
// controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_master_controller_ops {
    pub master): *mut *mut int (bus_init)(struct i3c_master_controller,
    pub master): *mut *mut void (bus_cleanup)(struct i3c_master_controller,
    pub dev): *mut *mut int (attach_i3c_dev)(struct i3c_dev_desc,
    pub old_dyn_addr): *mut *mut *mut int (reattach_i3c_dev)(struct i3c_dev_desc dev, u8,
    pub dev): *mut *mut void (detach_i3c_dev)(struct i3c_dev_desc,
    pub master): *mut *mut int (do_daa)(struct i3c_master_controller,
    pub cmd): *const i3c_ccc_cmd,
    pub cmd): *mut i3c_ccc_cmd,
    pub mode): int nxfers, enum i3c_xfer_mode,
    pub dev): *mut *mut int (attach_i2c_dev)(struct i2c_dev_desc,
    pub dev): *mut *mut void (detach_i2c_dev)(struct i2c_dev_desc,
    pub nxfers): *mut *mut i2c_msg xfers, int,
    pub req): *const i3c_ibi_setup,
    pub dev): *mut *mut void (free_ibi)(struct i3c_dev_desc,
    pub dev): *mut *mut int (enable_ibi)(struct i3c_dev_desc,
    pub dev): *mut *mut int (disable_ibi)(struct i3c_dev_desc,
    pub slot): *mut i3c_ibi_slot,
    pub master): *mut *mut int (enable_hotjoin)(struct i3c_master_controller,
    pub master): *mut *mut int (disable_hotjoin)(struct i3c_master_controller,
    pub speed): *mut *mut *mut int (set_speed)(struct i3c_master_controller master, enum i3c_open_drain_speed,
    pub dev_nack_retry_cnt): c_uint,
}

//
// struct i3c_master_controller - I3C master controller object
// @dev: device to be registered to the device-model
// @this: an I3C device object representing this master. This device will be
// added to the list of I3C devs available on the bus
// @i2c: I2C adapter used for backward compatibility. This adapter is
// registered to the I2C subsystem to be as transparent as possible to
// existing I2C drivers
// @ops: master operations. See &struct i3c_master_controller_ops
// @secondary: true if the master is a secondary master
// @init_done: true when the bus initialization is done
// @hotjoin: true if the master support hotjoin
// @rpm_allowed: true if Runtime PM allowed
// @rpm_ibi_allowed: true if IBI and Hot-Join allowed while runtime suspended
// @ibi_wakeup: IBI can wakeup the system
// @shutting_down: set to true when master begins shutdown or unregister
// @boardinfo.i3c: list of I3C  boardinfo objects
// @boardinfo.i2c: list of I2C boardinfo objects
// @boardinfo: board-level information attached to devices connected on the bus
// @bus: I3C bus exposed by this master
// @addr_method: Bitmap describing which methods of Address Assignment required
// to be run for discovering all the devices on the bus.
// Bit 0: SETDASA
// Bit 1: SETAASA
// All other bits are reserved.
// @wq: freezable workqueue which can be used by master
// drivers if they need to postpone operations that need to take place
// in a thread context. Typical examples are Hot Join processing which
// requires taking the bus lock in maintenance, which in turn, can only
// be done from a sleep-able context
// @hj_work: work item used to run DAA after a Hot-Join event is detected.
// Queued to @wq by i3c_master_queue_hotjoin()
// @reg_work: work item used to register newly discovered I3C devices with
// the driver model. Queued to @wq by i3c_master_do_daa_ext() so
// that device registration is deferred out of the DAA caller's
// context (notably the resume path), and is skipped if the
// controller is shutting down
// @dev_nack_retry_count: retry count when slave device nack
//
// A &struct i3c_master_controller has to be registered to the I3C subsystem
// through i3c_master_register(). None of &struct i3c_master_controller fields
// should be set manually, just pass appropriate values to
// i3c_master_register().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_master_controller {
    pub dev: device,
    pub this: *mut i3c_dev_desc,
    pub i2c: i2c_adapter,
    pub ops: *const i3c_master_controller_ops,
    pub 1: unsigned int secondary :,
    pub 1: unsigned int init_done :,
    pub 1: unsigned int hotjoin:,
    pub 1: unsigned int rpm_allowed:,
    pub 1: unsigned int rpm_ibi_allowed:,
    pub 1: unsigned int ibi_wakeup:,
    pub shutting_down: bool,
    pub i3c: list_head,
    pub i2c: list_head,
    pub boardinfo: },
    pub bus: i3c_bus,
    pub addr_method: u8,
    pub wq: *mut workqueue_struct,
    pub hj_work: work_struct,
    pub reg_work: work_struct,
    pub dev_nack_retry_count: c_uint,
}

//
// i3c_bus_for_each_i2cdev() - iterate over all I2C devices present on the bus
// @bus: the I3C bus
// @dev: an I2C device descriptor pointer updated to point to the current slot
// at each iteration of the loop
//
// Iterate over all I2C devs present on the bus.
//

//
// i3c_bus_for_each_i3cdev() - iterate over all I3C devices present on the bus
// @bus: the I3C bus
// @dev: and I3C device descriptor pointer updated to point to the current slot
// at each iteration of the loop
//
// Iterate over all I3C devs present on the bus.
//

//
// struct i3c_dma - DMA transfer and mapping descriptor
// @dev: device object of a device doing DMA
// @buf: destination/source buffer for DMA
// @len: length of transfer
// @map_len: length of DMA mapping
// @addr: mapped DMA address for a Host Controller Driver
// @dir: DMA direction
// @bounce_buf: an allocated bounce buffer if transfer needs it or NULL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_dma {
    pub dev: *mut device,
    pub buf: *mut c_void,
    pub len: usize,
    pub map_len: usize,
    pub addr: dma_addr_t,
    pub dir: dma_data_direction,
    pub bounce_buf: *mut c_void,
}

extern "C" {
    pub fn i3c_master_entdaa_locked(master: *mut i3c_master_controller) -> c_int;
}
extern "C" {
    pub fn i3c_master_defslvs_locked(master: *mut i3c_master_controller) -> c_int;
}
extern "C" {
    pub fn i3c_master_add_i3c_dev_locked(master: *mut i3c_master_controller, addr: u8);
}
extern "C" {
    pub fn i3c_master_do_daa(master: *mut i3c_master_controller) -> c_int;
}
extern "C" {
    pub fn i3c_master_do_daa_ext(master: *mut i3c_master_controller, rstdaa: bool) -> c_int;
}
extern "C" {
    pub fn i3c_master_dma_unmap_single(dma_xfer: *mut i3c_dma);
}
extern "C" {
    pub fn i3c_master_unregister(master: *mut i3c_master_controller);
}
extern "C" {
    pub fn i3c_master_enable_hotjoin(master: *mut i3c_master_controller) -> c_int;
}
extern "C" {
    pub fn i3c_master_disable_hotjoin(master: *mut i3c_master_controller) -> c_int;
}
extern "C" {
    pub fn i3c_master_queue_hotjoin(master: *mut i3c_master_controller);
}
//
// i3c_dev_get_master_data() - get master private data attached to an I3C
// device descriptor
// @dev: the I3C device descriptor to get private data from
//
// Return: the private data previously attached with i3c_dev_set_master_data()
// or NULL if no data has been attached to the device.
//
// i3c_dev_set_master_data() - attach master private data to an I3C device
// descriptor
// @dev: the I3C device descriptor to attach private data to
// @data: private data
//
// This functions allows a master controller to attach per-device private data
// which can then be retrieved with i3c_dev_get_master_data().
//
// i2c_dev_get_master_data() - get master private data attached to an I2C
// device descriptor
// @dev: the I2C device descriptor to get private data from
//
// Return: the private data previously attached with i2c_dev_set_master_data()
// or NULL if no data has been attached to the device.
//
// i2c_dev_set_master_data() - attach master private data to an I2C device
// descriptor
// @dev: the I2C device descriptor to attach private data to
// @data: private data
//
// This functions allows a master controller to attach per-device private data
// which can then be retrieved with i2c_device_get_master_data().
//
// i3c_dev_get_master() - get master used to communicate with a device
// @dev: I3C dev
//
// Return: the master controller driving @dev
//
// i2c_dev_get_master() - get master used to communicate with a device
// @dev: I2C dev
//
// Return: the master controller driving @dev
//
// i3c_master_get_bus() - get the bus attached to a master
// @master: master object
//
// Return: the I3C bus @master is connected to
//
extern "C" {
    pub fn i3c_generic_ibi_free_pool(pool: *mut i3c_generic_ibi_pool);
}
extern "C" {
    pub fn i3c_master_queue_ibi(dev: *mut i3c_dev_desc, slot: *mut i3c_ibi_slot);
}
extern "C" {
    pub fn i3c_master_has_wakeup_enabled_devs(master: *mut i3c_master_controller) -> bool;
}
extern "C" {
    pub fn i3c_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn i3c_unregister_notifier(nb: *mut notifier_block) -> c_int;
}
