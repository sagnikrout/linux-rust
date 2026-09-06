//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gpio/driver.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union gpio_irq_fwspec {
    pub fwspec: irq_fwspec,

    pub msiinfo: msi_alloc_info_t,

}

//
// struct gpio_irq_chip - GPIO interrupt controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_irq_chip {
//
// @chip:
//
// GPIO IRQ chip implementation, provided by GPIO driver.
//
    pub chip: *mut irq_chip,
//
// @domain:
//
// Interrupt translation domain; responsible for mapping between GPIO
// hwirq number and Linux IRQ number.
//
    pub domain: *mut irq_domain,

//
// @fwnode:
//
// Firmware node corresponding to this gpiochip/irqchip, necessary
// for hierarchical irqdomain support.
//
    pub fwnode: *mut fwnode_handle,
//
// @parent_domain:
//
// If non-NULL, will be set as the parent of this GPIO interrupt
// controller's IRQ domain to establish a hierarchical interrupt
// domain. The presence of this will activate the hierarchical
// interrupt support.
//
    pub parent_domain: *mut irq_domain,
//
// @child_to_parent_hwirq:
//
// This callback translates a child hardware IRQ offset to a parent
// hardware IRQ offset on a hierarchical interrupt chip. The child
// hardware IRQs correspond to the GPIO index 0..ngpio-1 (see the
// ngpio field of struct gpio_chip) and the corresponding parent
// hardware IRQ and type (such as IRQ_TYPE_*) shall be returned by
// the driver. The driver can calculate this from an offset or using
// a lookup table or whatever method is best for this chip. Return
// 0 on successful translation in the driver.
//
// If some ranges of hardware IRQs do not have a corresponding parent
// HWIRQ, return -EINVAL, but also make sure to fill in @valid_mask and
// @need_valid_mask to make these GPIO lines unavailable for
// translation.
//
    pub parent_type): *mut c_uint,
//
// @populate_parent_alloc_arg :
//
// This optional callback allocates and populates the specific struct
// for the parent's IRQ domain. If this is not specified, then
// &gpiochip_populate_parent_fwspec_twocell will be used. A four-cell
// variant named &gpiochip_populate_parent_fwspec_fourcell is also
// available.
//
    pub parent_type): c_uint,
//
// @child_offset_to_irq:
//
// This optional callback is used to translate the child's GPIO line
// offset on the GPIO chip to an IRQ number for the GPIO to_irq()
// callback. If this is not specified, then a default callback will be
// provided that returns the line offset.
//
    pub pin): c_uint,
//
// @child_irq_domain_ops:
//
// The IRQ domain operations that will be used for this GPIO IRQ
// chip. If no operations are provided, then default callbacks will
// be populated to setup the IRQ hierarchy. Some drivers need to
// supply their own translate function.
//
    pub child_irq_domain_ops: irq_domain_ops,

//
// @handler:
//
// The IRQ handler to use (often a predefined IRQ core function) for
// GPIO IRQs, provided by GPIO driver.
//
    pub handler: irq_flow_handler_t,
//
// @default_type:
//
// Default IRQ triggering type applied during GPIO driver
// initialization, provided by GPIO driver.
//
    pub default_type: c_uint,
//
// @lock_key:
//
// Per GPIO IRQ chip lockdep class for IRQ lock.
//
    pub lock_key: *mut lock_class_key,
//
// @request_key:
//
// Per GPIO IRQ chip lockdep class for IRQ request.
//
    pub request_key: *mut lock_class_key,
//
// @parent_handler:
//
// The interrupt handler for the GPIO chip's parent interrupts, may be
// NULL if the parent interrupts are nested rather than cascaded.
//
    pub parent_handler: irq_flow_handler_t,
//
// @parent_handler_data:
//
// If @per_parent_data is false, @parent_handler_data is a
// single pointer used as the data associated with every
// parent interrupt.
//
    pub parent_handler_data: *mut c_void,
//
// @parent_handler_data_array:
//
// If @per_parent_data is true, @parent_handler_data_array is
// an array of @num_parents pointers, and is used to associate
// different data for each parent. This cannot be NULL if
// @per_parent_data is true.
//
    pub parent_handler_data_array: *mut c_void,
}

//
// @num_parents:
//
// The number of interrupt parents of a GPIO chip.
//
// @parents:
//
// A list of interrupt parents of a GPIO chip. This is owned by the
// driver, so the core will only reference this list, not modify it.
//
// @map:
//
// A list of interrupt parents for each line of a GPIO chip.
//
// @threaded:
//
// True if set the interrupt handling uses nested threads.
//
// @per_parent_data:
//
// True if parent_handler_data_array describes a @num_parents
// sized array to be used as parent data.
//
// @initialized:
//
// Flag to track GPIO chip irq member's initialization.
// This flag will make sure GPIO chip irq members are not used
// before they are initialized.
//
// @domain_is_allocated_externally:
//
// True it the irq_domain was allocated outside of gpiolib, in which
// case gpiolib won't free the irq_domain itself.
//
// @init_hw: optional routine to initialize hardware before
// an IRQ chip will be added. This is quite useful when
// a particular driver wants to clear IRQ related registers
// in order to avoid undesired events.
//
// @init_valid_mask: optional routine to initialize @valid_mask, to be
// used if not all GPIO lines are valid interrupts. Sometimes some
// lines just cannot fire interrupts, and this routine, when defined,
// is passed a bitmap in "valid_mask" and it will have ngpios
// bits from 0..(ngpios-1) set to "1" as in valid. The callback can
// then directly set some bits to "0" if they cannot be used for
// interrupts.
//
// @valid_mask:
//
// If not %NULL, holds bitmask of GPIOs which are valid to be included
// in IRQ domain of the chip.
//
// @first:
//
// Required for static IRQ allocation. If set,
// irq_domain_create_simple() will allocate and map all IRQs
// during initialization.
//
// @irq_enable:
//
// Store old irq_chip irq_enable callback
//
// @irq_disable:
//
// Store old irq_chip irq_disable callback
//
// @irq_unmask:
//
// Store old irq_chip irq_unmask callback
//
// @irq_mask:
//
// Store old irq_chip irq_mask callback
//
// struct gpio_chip - abstract a GPIO controller
// @label: a functional name for the GPIO device, such as a part
// number or the name of the SoC IP-block implementing it.
// @gpiodev: the internal state holder, opaque struct
// @parent: optional parent device providing the GPIOs
// @fwnode: optional fwnode providing this controller's properties
// @owner: helps prevent removal of modules exporting active GPIOs
// @request: optional hook for chip-specific activation, such as
// enabling module power and clock; may sleep; must return 0 on success
// or negative error number on failure
// @free: optional hook for chip-specific deactivation, such as
// disabling module power and clock; may sleep
// @get_direction: returns direction for signal "offset", 0=out, 1=in,
// (same as GPIO_LINE_DIRECTION_OUT / GPIO_LINE_DIRECTION_IN),
// or negative error. It is recommended to always implement this
// function, even on input-only or output-only gpio chips.
// @direction_input: configures signal "offset" as input, returns 0 on success
// or a negative error number. This can be omitted on input-only or
// output-only gpio chips.
// @direction_output: configures signal "offset" as output, returns 0 on
// success or a negative error number. This can be omitted on input-only
// or output-only gpio chips.
// @get: returns value for signal "offset", 0=low, 1=high, or negative error.
// the low and high values are defined as physical low on the line
// in/out to the connector such as a physical pad, pin or rail. The GPIO
// library has internal logic to handle lines that are active low, such
// as indicated by overstrike or #name in a schematic, and the driver
// should not try to second-guess the logic value of a line.
// @get_multiple: reads values for multiple signals defined by "mask" and
// stores them in "bits", returns 0 on success or negative error
// @set: assigns output value for signal "offset", returns 0 on success or
// negative error value. The output value follows the same semantic
// rules as for @get.
// @set_multiple: assigns output values for multiple signals defined by
// "mask", returns 0 on success or negative error value
// @set_config: optional hook for all kinds of settings. Uses the same
// packed config format as generic pinconf. Must return 0 on success and
// a negative error number on failure.
// @to_irq: optional hook supporting non-static gpiod_to_irq() mappings;
// implementation may not sleep
// @dbg_show: optional routine to show contents in debugfs; default code
// will be used when this is omitted, but custom code can show extra
// state (such as pullup/pulldown configuration).
// @init_valid_mask: optional routine to initialize @valid_mask, to be used if
// not all GPIOs are valid.
// @add_pin_ranges: optional routine to initialize pin ranges, to be used when
// requires special mapping of the pins that provides GPIO functionality.
// It is called after adding GPIO chip and before adding IRQ chip.
// @en_hw_timestamp: Dependent on GPIO chip, an optional routine to
// enable hardware timestamp.
// @dis_hw_timestamp: Dependent on GPIO chip, an optional routine to
// disable hardware timestamp.
// @base: identifies the first GPIO number handled by this chip;
// or, if negative during registration, requests dynamic ID allocation.
// DEPRECATION: providing anything non-negative and nailing the base
// offset of GPIO chips is deprecated. Please pass -1 as base to
// let gpiolib select the chip base in all possible cases. We want to
// get rid of the static GPIO number space in the long run.
// @ngpio: the number of GPIOs handled by this controller; the last GPIO
// handled is (base + ngpio - 1).
// @offset: when multiple gpio chips belong to the same device this
// can be used as offset within the device so friendly names can
// be properly assigned.
// @names: if set, must be an array of strings to use as alternative
// names for the GPIOs in this chip. Any entry in the array
// may be NULL if there is no alias for the GPIO, however the
// array must be @ngpio entries long.
// @can_sleep: flag must be set iff get()/set() methods sleep, as they
// must while accessing GPIO expander chips over I2C or SPI. This
// implies that if the chip supports IRQs, these IRQs need to be threaded
// as the chip access may sleep when e.g. reading out the IRQ status
// registers.
//
// A gpio_chip can help platforms abstract various sources of GPIOs so
// they can all be accessed through a common programming interface.
// Example sources would be SOC controllers, FPGAs, multifunction
// chips, dedicated GPIO expanders, and so on.
//
// Each chip controls a number of signals, identified in method calls
// by "offset" values in the range 0..(@ngpio - 1).  When those signals
// are referenced through calls like gpio_get_value(gpio), the offset
// is calculated by subtracting @base from the gpio number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_chip {
    pub label: *const c_char,
    pub gpiodev: *mut gpio_device,
    pub parent: *mut device,
    pub fwnode: *mut fwnode_handle,
    pub owner: *mut module,
    pub offset): c_uint,
    pub offset): c_uint,
    pub offset): c_uint,
    pub offset): c_uint,
    pub value): unsigned int offset, int,
    pub offset): c_uint,
    pub bits): *mut c_ulong,
    pub value): unsigned int offset, int,
    pub bits): *mut c_ulong,
    pub config): c_ulong,
    pub offset): c_uint,
    pub gc): *mut gpio_chip,
    pub ngpios): c_uint,
    pub gc): *mut *mut int (add_pin_ranges)(struct gpio_chip,
    pub flags): c_ulong,
    pub flags): c_ulong,
    pub base: c_int,
    pub ngpio: u16,
    pub offset: u16,
    pub names: *const *const c_char,
    pub can_sleep: bool,

//
// With CONFIG_GPIOLIB_IRQCHIP we get an irqchip inside the gpiolib
// to handle IRQs for most practical cases.
//
// @irq:
//
// Integrates interrupt chip functionality with the GPIO chip. Can be
// used to handle IRQs for most practical cases.
//
    pub irq: gpio_irq_chip,

//
// If CONFIG_OF_GPIO is enabled, then all GPIO controllers described in
// the device tree automatically may have an OF translation
//
// @of_gpio_n_cells:
//
// Number of cells used to form the GPIO specifier. The standard is 2
// cells:
//
// gpios = <&gpio offset flags>;
//
// some complex GPIO controllers instantiate more than one chip per
// device tree node and have 3 cells:
//
// gpios = <&gpio instance offset flags>;
//
// Legacy GPIO controllers may even have 1 cell:
//
// gpios = <&gpio offset>;
//
    pub of_gpio_n_cells: c_uint,
//
// @of_node_instance_match:
//
// Determine if a chip is the right instance. Must be implemented by
// any driver using more than one gpio_chip per device tree node.
// Returns true if gc is the instance indicated by i (which is the
// first cell in the phandles for GPIO lines and gpio-ranges).
//
    pub i): *mut *mut *mut bool (of_node_instance_match)(struct gpio_chip gc, unsigned int,
//
// @of_xlate:
//
// Callback to translate a device tree GPIO specifier into a chip-
// relative GPIO number and flags.
//
    pub flags): *const *const of_phandle_args gpiospec, u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _gpiochip_for_each_data {
    pub label: *const c_char,
    pub i: *mut c_uint,
}

// _data.i = 0;
//
// for_each_hwgpio_in_range - Iterates over all GPIOs in a given range
// @_chip: Chip to iterate over.
// @_i: Loop counter.
// @_base: First GPIO in the ranger.
// @_size: Amount of GPIOs to check starting from @base.
// @_label: Place to store the address of the label if the GPIO is requested.
// Set to NULL for unused GPIOs.
//

//
// for_each_hwgpio - Iterates over all GPIOs for given chip.
// @_chip: Chip to iterate over.
// @_i: Loop counter.
// @_label: Place to store the address of the label if the GPIO is requested.
// Set to NULL for unused GPIOs.
//

//
// for_each_requested_gpio_in_range - iterates over requested GPIOs in a given range
// @_chip:	the chip to query
// @_i:		loop variable
// @_base:	first GPIO in the range
// @_size:	amount of GPIOs to check starting from @base
// @_label:	label of current GPIO
//

// Iterates over all requested GPIO of the given @chip

// add/remove chips
//
// gpiochip_add_data() - register a gpio_chip
// @gc: the chip to register, with gc->base initialized
// @data: driver-private data associated with this chip
//
// Context: potentially before irqs will work
//
// When gpiochip_add_data() is called very early during boot, so that GPIOs
// can be freely used, the gc->parent device must be registered before
// the gpio framework's arch_initcall().  Otherwise sysfs initialization
// for GPIOs will fail rudely.
//
// gpiochip_add_data() must only be called after gpiolib initialization,
// i.e. after core_initcall().
//
// If gc->base is negative, this requests dynamic assignment of
// a range of valid GPIOs.
//
// Returns:
// A negative errno if the chip can't be registered, such as because the
// gc->base is invalid or already associated with a different chip.
// Otherwise it returns zero as a success code.
//

extern "C" {
    pub fn gpiochip_remove(gc: *mut gpio_chip);
}
extern "C" {
    pub fn gpio_device_put(gdev: *mut gpio_device);
}
extern "C" {
    pub fn gpiochip_line_is_irq(gc: *mut gpio_chip, offset: c_uint) -> bool;
}
extern "C" {
    pub fn gpiochip_reqres_irq(gc: *mut gpio_chip, offset: c_uint) -> c_int;
}
extern "C" {
    pub fn gpiochip_relres_irq(gc: *mut gpio_chip, offset: c_uint);
}
extern "C" {
    pub fn gpiochip_disable_irq(gc: *mut gpio_chip, offset: c_uint);
}
extern "C" {
    pub fn gpiochip_enable_irq(gc: *mut gpio_chip, offset: c_uint);
}
// irq_data versions of the above
extern "C" {
    pub fn gpiochip_irq_reqres(data: *mut irq_data) -> c_int;
}
extern "C" {
    pub fn gpiochip_irq_relres(data: *mut irq_data);
}
// Paste this in your irq_chip structure

// Yes, dropping const is ugly, but it isn't like we have a choice
// Line status inquiry for drivers
extern "C" {
    pub fn gpiochip_line_is_open_drain(gc: *mut gpio_chip, offset: c_uint) -> bool;
}
extern "C" {
    pub fn gpiochip_line_is_open_source(gc: *mut gpio_chip, offset: c_uint) -> bool;
}
// Sleep persistence inquiry for drivers
extern "C" {
    pub fn gpiochip_line_is_persistent(gc: *mut gpio_chip, offset: c_uint) -> bool;
}
extern "C" {
    pub fn gpiochip_line_is_valid(gc: *const gpio_chip, offset: c_uint) -> bool;
}
// get driver data

extern "C" {
    pub fn gpiochip_generic_request(gc: *mut gpio_chip, offset: c_uint) -> c_int;
}
extern "C" {
    pub fn gpiochip_generic_free(gc: *mut gpio_chip, offset: c_uint);
}
//
// struct gpio_pin_range - pin range controlled by a gpio chip
// @node: list for maintaining set of pin ranges, used internally
// @pctldev: pinctrl device which handles corresponding pins
// @range: actual range of pins controlled by a gpio controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_pin_range {
    pub node: list_head,
    pub pctldev: *mut pinctrl_dev,
    pub range: pinctrl_gpio_range,
}

extern "C" {
    pub fn gpiochip_remove_pin_ranges(gc: *mut gpio_chip);
}

extern "C" {
    pub fn gpiochip_free_own_desc(desc: *mut gpio_desc);
}

// lock/unlock as IRQ
extern "C" {
    pub fn gpiochip_lock_as_irq(gc: *mut gpio_chip, offset: c_uint) -> c_int;
}
extern "C" {
    pub fn gpiochip_unlock_as_irq(gc: *mut gpio_chip, offset: c_uint);
}
// struct gpio_device getters
extern "C" {
    pub fn gpio_device_get_base(gdev: *mut gpio_device) -> c_int;
}

// GPIO can never have been requested
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

