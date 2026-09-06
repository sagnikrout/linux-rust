//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/serial.h
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
// USB Serial Converter stuff
//
// Copyright (C) 1999 - 2012
// Greg Kroah-Hartman (greg@kroah.com)
//

// The maximum number of ports one device can grab at once
pub const MAX_NUM_PORTS: c_int = 16;
// USB serial flags
pub const USB_SERIAL_WRITE_BUSY: c_int = 0;
pub const USB_SERIAL_THROTTLED: c_int = 1;
//
// usb_serial_port: structure for the specific ports of a device.
// @serial: pointer back to the struct usb_serial owner of this port.
// @port: pointer to the corresponding tty_port for this port.
// @lock: spinlock to grab when updating portions of this structure.
// @minor: the minor number of the port
// @port_number: the struct usb_serial port number of this port (starts at 0)
// @interrupt_in_buffer: pointer to the interrupt in buffer for this port.
// @interrupt_in_urb: pointer to the interrupt in struct urb for this port.
// @interrupt_in_endpointAddress: endpoint address for the interrupt in pipe
// for this port.
// @interrupt_out_buffer: pointer to the interrupt out buffer for this port.
// @interrupt_out_size: the size of the interrupt_out_buffer, in bytes.
// @interrupt_out_urb: pointer to the interrupt out struct urb for this port.
// @interrupt_out_endpointAddress: endpoint address for the interrupt out pipe
// for this port.
// @bulk_in_buffer: pointer to the bulk in buffer for this port.
// @bulk_in_size: the size of the bulk_in_buffer, in bytes.
// @read_urb: pointer to the bulk in struct urb for this port.
// @bulk_in_endpointAddress: endpoint address for the bulk in pipe for this
// port.
// @bulk_in_buffers: pointers to the bulk in buffers for this port
// @read_urbs: pointers to the bulk in urbs for this port
// @read_urbs_free: status bitmap the for bulk in urbs
// @bulk_out_buffer: pointer to the bulk out buffer for this port.
// @bulk_out_size: the size of the bulk_out_buffer, in bytes.
// @write_urb: pointer to the bulk out struct urb for this port.
// @write_fifo: kfifo used to buffer outgoing data
// @bulk_out_buffers: pointers to the bulk out buffers for this port
// @write_urbs: pointers to the bulk out urbs for this port
// @write_urbs_free: status bitmap the for bulk out urbs
// @icount: interrupt counters
// @tx_bytes: number of bytes currently in host stack queues
// @bulk_out_endpointAddress: endpoint address for the bulk out pipe for this
// port.
// @flags: usb serial port flags
// @work: work queue entry for the line discipline waking up.
// @dev: pointer to the serial device
//
// This structure is used by the usb-serial core and drivers for the specific
// ports of a device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_serial_port {
    pub serial: *mut usb_serial,
    pub port: tty_port,
    pub lock: spinlock_t,
    pub minor: u32,
    pub port_number: u8,
    pub interrupt_in_buffer: *mut c_uchar,
    pub interrupt_in_urb: *mut urb,
    pub interrupt_in_endpointAddress: __u8,
    pub interrupt_out_buffer: *mut c_uchar,
    pub interrupt_out_size: c_int,
    pub interrupt_out_urb: *mut urb,
    pub interrupt_out_endpointAddress: __u8,
    pub bulk_in_buffer: *mut c_uchar,
    pub bulk_in_size: c_int,
    pub read_urb: *mut urb,
    pub bulk_in_endpointAddress: __u8,
    pub bulk_in_buffers: [*mut c_uchar; 2],
    pub read_urbs: [*mut urb; 2],
    pub read_urbs_free: c_ulong,
    pub bulk_out_buffer: *mut c_uchar,
    pub bulk_out_size: c_int,
    pub write_urb: *mut urb,
    pub write_fifo: kfifo,
    pub bulk_out_buffers: [*mut c_uchar; 2],
    pub write_urbs: [*mut urb; 2],
    pub write_urbs_free: c_ulong,
    pub bulk_out_endpointAddress: __u8,
    pub icount: async_icount,
    pub tx_bytes: c_int,
    pub flags: c_ulong,
    pub work: work_struct,
    pub /: *mut *mut unsigned long sysrq; / sysrq timeout,
    pub dev: device,
}

// get and set the port private data pointer helper functions
extern "C" {
    pub fn dev_get_drvdata(_arg: &port->dev) -> return;
}
//
// usb_serial - structure used by the usb-serial core for a device
// @dev: pointer to the struct usb_device for this device
// @type: pointer to the struct usb_serial_driver for this device
// @interface: pointer to the struct usb_interface for this device
// @sibling: pointer to the struct usb_interface of any sibling interface
// @suspend_count: number of suspended (sibling) interfaces
// @num_ports: the number of ports this device has
// @num_interrupt_in: number of interrupt in endpoints we have
// @num_interrupt_out: number of interrupt out endpoints we have
// @num_bulk_in: number of bulk in endpoints we have
// @num_bulk_out: number of bulk out endpoints we have
// @port: array of struct usb_serial_port structures for the different ports.
// @private: place to put any driver specific information that is needed.  The
// usb-serial driver is required to manage this data, the usb-serial core
// will not touch this.  Use usb_get_serial_data() and
// usb_set_serial_data() to access this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_serial {
    pub dev: *mut usb_device,
    pub type: *mut usb_serial_driver,
    pub interface: *mut usb_interface,
    pub sibling: *mut usb_interface,
    pub suspend_count: c_uint,
    pub disconnected:1: c_uchar,
    pub attached:1: c_uchar,
    pub minors_reserved:1: c_uchar,
    pub num_ports: c_uchar,
    pub num_port_pointers: c_uchar,
    pub num_interrupt_in: c_uchar,
    pub num_interrupt_out: c_uchar,
    pub num_bulk_in: c_uchar,
    pub num_bulk_out: c_uchar,
    pub port: [*mut usb_serial_port; MAX_NUM_PORTS],
    pub kref: kref,
    pub disc_mutex: mutex,
    pub private: *mut c_void,
}

// get and set the serial private data pointer helper functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_serial_endpoints {
    pub num_bulk_in: c_uchar,
    pub num_bulk_out: c_uchar,
    pub num_interrupt_in: c_uchar,
    pub num_interrupt_out: c_uchar,
    pub bulk_in: [*mut usb_endpoint_descriptor; MAX_NUM_PORTS],
    pub bulk_out: [*mut usb_endpoint_descriptor; MAX_NUM_PORTS],
    pub interrupt_in: [*mut usb_endpoint_descriptor; MAX_NUM_PORTS],
    pub interrupt_out: [*mut usb_endpoint_descriptor; MAX_NUM_PORTS],
}

//
// usb_serial_driver - describes a usb serial driver
// @description: pointer to a string that describes this driver.  This string
// used in the syslog messages when a device is inserted or removed.
// @id_table: pointer to a list of usb_device_id structures that define all
// of the devices this structure can support.
// @num_ports: the number of different ports this device will have.
// @num_bulk_in: minimum number of bulk-in endpoints
// @num_bulk_out: minimum number of bulk-out endpoints
// @num_interrupt_in: minimum number of interrupt-in endpoints
// @num_interrupt_out: minimum number of interrupt-out endpoints
// @bulk_in_size: minimum number of bytes to allocate for bulk-in buffer
// (0 = end-point size)
// @bulk_out_size: bytes to allocate for bulk-out buffer (0 = end-point size)
// @calc_num_ports: pointer to a function to determine how many ports this
// device has dynamically. It can also be used to verify the number of
// endpoints or to modify the port-endpoint mapping. It will be called
// after the probe() callback is called, but before attach().
// @probe: pointer to the driver's probe function.
// This will be called when the device is inserted into the system,
// but before the device has been fully initialized by the usb_serial
// subsystem.  Use this function to download any firmware to the device,
// or any other early initialization that might be needed.
// Return 0 to continue on with the initialization sequence.  Anything
// else will abort it.
// @attach: pointer to the driver's attach function.
// This will be called when the struct usb_serial structure is fully
// set up.  Do any local initialization of the device, or any private
// memory structure allocation at this point in time.
// @disconnect: pointer to the driver's disconnect function.  This will be
// called when the device is unplugged or unbound from the driver.
// @release: pointer to the driver's release function.  This will be called
// when the usb_serial data structure is about to be destroyed.
// @usb_driver: pointer to the struct usb_driver that controls this
// device.  This is necessary to allow dynamic ids to be added to
// the driver from sysfs.
//
// This structure is defines a USB Serial driver.  It provides all of
// the information that the USB serial core code needs.  If the function
// pointers are defined, then the USB serial core code will call them when
// the corresponding tty port functions are called.  If they are not
// called, the generic serial function will be used instead.
//
// The driver.owner field should be set to the module owner of this driver.
// The driver.name field should be set to the name of this driver (remember
// it will show up in sysfs, so it needs to be short and to the point.
// Using the module name is a good idea.)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_serial_driver {
    pub description: *const c_char,
    pub id_table: *const usb_device_id,
    pub driver_list: list_head,
    pub driver: device_driver,
    pub usb_driver: *mut usb_driver,
    pub dynids: usb_dynids,
    pub num_ports: c_uchar,
    pub num_bulk_in: c_uchar,
    pub num_bulk_out: c_uchar,
    pub num_interrupt_in: c_uchar,
    pub num_interrupt_out: c_uchar,
    pub bulk_in_size: usize,
    pub bulk_out_size: usize,
    pub id): *const *const *const int (probe)(struct usb_serial serial, struct usb_device_id,
    pub serial): *mut *mut int (attach)(struct usb_serial,
    pub epds): *mut usb_serial_endpoints,
    pub serial): *mut *mut void (disconnect)(struct usb_serial,
    pub serial): *mut *mut void (release)(struct usb_serial,
    pub port): *mut *mut int (port_probe)(struct usb_serial_port,
    pub port): *mut *mut void (port_remove)(struct usb_serial_port,
    pub message): *mut *mut *mut int (suspend)(struct usb_serial serial, pm_message_t,
    pub serial): *mut *mut int (resume)(struct usb_serial,
    pub serial): *mut *mut int (reset_resume)(struct usb_serial,
// serial function calls
// Called by console and by the tty layer
    pub port): *mut *mut *mut int (open)(struct tty_struct tty, struct usb_serial_port,
    pub port): *mut *mut void (close)(struct usb_serial_port,
    pub count): *const *const unsigned char buf, int,
// Called only by the tty layer
    pub tty): *mut *mut unsigned int (write_room)(struct tty_struct,
    pub arg): unsigned int cmd, unsigned long,
    pub ss): *mut *mut *mut void (get_serial)(struct tty_struct tty, struct serial_struct,
    pub ss): *mut *mut *mut int (set_serial)(struct tty_struct tty, struct serial_struct,
    pub old): *const ktermios,
    pub break_state): *mut *mut *mut int (break_ctl)(struct tty_struct tty, int,
    pub tty): *mut *mut unsigned int (chars_in_buffer)(struct tty_struct,
    pub timeout): *mut *mut *mut void (wait_until_sent)(struct tty_struct tty, long,
    pub port): *mut *mut bool (tx_empty)(struct usb_serial_port,
    pub tty): *mut *mut void (throttle)(struct tty_struct,
    pub tty): *mut *mut void (unthrottle)(struct tty_struct,
    pub tty): *mut *mut int (tiocmget)(struct tty_struct,
    pub clear): unsigned int set, unsigned int,
    pub arg): *mut *mut *mut int (tiocmiwait)(struct tty_struct tty, unsigned long,
    pub icount): *mut serial_icounter_struct,
// Called by the tty layer for port level work. There may or may not
    pub on): *mut *mut *mut void (dtr_rts)(struct usb_serial_port port, int,
    pub port): *mut *mut int (carrier_raised)(struct usb_serial_port,
// Called by the usb serial hooks to allow the user to rework the
    pub tty): *mut *mut void (init_termios)(struct tty_struct,
// USB events
    pub urb): *mut *mut void (read_int_callback)(struct urb,
    pub urb): *mut *mut void (write_int_callback)(struct urb,
    pub urb): *mut *mut void (read_bulk_callback)(struct urb,
    pub urb): *mut *mut void (write_bulk_callback)(struct urb,
// Called by the generic read bulk callback
    pub urb): *mut *mut void (process_read_urb)(struct urb,
// Called by the generic write implementation
    pub size): *mut *mut void dest, size_t,
}

extern "C" {
    pub fn usb_serial_deregister_drivers(serial_drivers[]: *const *const usb_serial_driver);
}
extern "C" {
    pub fn usb_serial_port_softint(port: *mut usb_serial_port);
}
extern "C" {
    pub fn usb_serial_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int;
}
extern "C" {
    pub fn usb_serial_resume(intf: *mut usb_interface) -> c_int;
}
// USB Serial console functions

extern "C" {
    pub fn usb_serial_console_init(minor: c_int);
}
extern "C" {
    pub fn usb_serial_console_exit();
}
extern "C" {
    pub fn usb_serial_console_disconnect(serial: *mut usb_serial);
}

// Functions needed by other parts of the usbserial core
extern "C" {
    pub fn usb_serial_put(serial: *mut usb_serial);
}
extern "C" {
    pub fn usb_serial_claim_interface(serial: *mut usb_serial, intf: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usb_serial_generic_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int;
}
extern "C" {
    pub fn usb_serial_generic_write_start(port: *mut usb_serial_port, mem_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn usb_serial_generic_close(port: *mut usb_serial_port);
}
extern "C" {
    pub fn usb_serial_generic_resume(serial: *mut usb_serial) -> c_int;
}
extern "C" {
    pub fn usb_serial_generic_write_room(tty: *mut tty_struct) -> c_uint;
}
extern "C" {
    pub fn usb_serial_generic_chars_in_buffer(tty: *mut tty_struct) -> c_uint;
}
extern "C" {
    pub fn usb_serial_generic_wait_until_sent(tty: *mut tty_struct, timeout: c_long);
}
extern "C" {
    pub fn usb_serial_generic_read_bulk_callback(urb: *mut urb);
}
extern "C" {
    pub fn usb_serial_generic_write_bulk_callback(urb: *mut urb);
}
extern "C" {
    pub fn usb_serial_generic_throttle(tty: *mut tty_struct);
}
extern "C" {
    pub fn usb_serial_generic_unthrottle(tty: *mut tty_struct);
}
extern "C" {
    pub fn usb_serial_generic_tiocmiwait(tty: *mut tty_struct, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn usb_serial_generic_get_icount(tty: *mut tty_struct, icount: *mut serial_icounter_struct) -> c_int;
}
extern "C" {
    pub fn usb_serial_generic_register() -> c_int;
}
extern "C" {
    pub fn usb_serial_generic_deregister();
}
extern "C" {
    pub fn usb_serial_generic_submit_read_urbs(port: *mut usb_serial_port, mem_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn usb_serial_generic_process_read_urb(urb: *mut urb);
}
extern "C" {
    pub fn usb_serial_generic_prepare_write_buffer(port: *mut usb_serial_port, dest: *mut c_void, size: usize) -> c_int;
}

extern "C" {
    pub fn usb_serial_handle_sysrq_char(port: *mut usb_serial_port, ch: c_uint) -> c_int;
}
extern "C" {
    pub fn usb_serial_handle_break(port: *mut usb_serial_port) -> c_int;
}

extern "C" {
    pub fn usb_serial_bus_register(device: *mut usb_serial_driver) -> c_int;
}
extern "C" {
    pub fn usb_serial_bus_deregister(device: *mut usb_serial_driver);
}
//
// Macro for reporting errors in write path to avoid infinite loop
// when port is used as a console.
//

//
// module_usb_serial_driver() - Helper macro for registering a USB Serial driver
// @__serial_drivers: list of usb_serial drivers to register
// @__ids: all device ids that @__serial_drivers bind to
//
// Helper macro for USB serial drivers which do not do anything special
// in module init/exit. This eliminates a lot of boilerplate. Each
// module may only use this macro once, and calling it replaces
// module_init() and module_exit()
//

