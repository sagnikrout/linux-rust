//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/leds.h
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
// Driver model for leds and led triggers
//
// Copyright (C) 2005 John Lenz <lenz@cs.wisc.edu>
// Copyright (C) 2005 Richard Purdie <rpurdie@openedhand.com>
//

//
// LED Core
//
// This is obsolete/useless. We now support variable maximum brightness.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_brightness {
    LED_OFF		= 0,
    LED_ON		= 1,
    LED_HALF	= 127,
    LED_FULL	= 255,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_default_state {
    LEDS_DEFSTATE_OFF	= 0,
    LEDS_DEFSTATE_ON	= 1,
    LEDS_DEFSTATE_KEEP	= 2,
}

//
// struct led_lookup_data - represents a single LED lookup entry
//
// @list: internal list of all LED lookup entries
// @provider: name of led_classdev providing the LED
// @dev_id: name of the device associated with this LED
// @con_id: name of the LED from the device's point of view
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_lookup_data {
    pub list: list_head,
    pub provider: *const c_char,
    pub dev_id: *const c_char,
    pub con_id: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_init_data {
// device fwnode handle
    pub fwnode: *mut fwnode_handle,
//
// default <color:function> tuple, for backward compatibility
// with in-driver hard-coded LED names used as a fallback when
// DT "label" property is absent; it should be set to NULL
// in new LED class drivers.
//
    pub default_label: *const c_char,
//
// string to be used for devicename section of LED class device
// either for label based LED name composition path or for fwnode
// based when devname_mandatory is true
//
    pub devicename: *const c_char,
//
// indicates if LED name should always comprise devicename section;
// only LEDs exposed by drivers of hot-pluggable devices should
// set it to true
//
    pub devname_mandatory: bool,
}

extern "C" {
    pub fn led_init_default_state_get(fwnode: *mut fwnode_handle) -> led_default_state;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_hw_trigger_type {
    pub dummy: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_classdev {
    pub name: *const c_char,
    pub brightness: c_uint,
    pub max_brightness: c_uint,
    pub color: c_uint,
    pub flags: c_int,
// Lower 16 bits reflect status

// Upper 16 bits reflect control information

// set_brightness_work / blink_timer flags, atomic, private.
    pub work_flags: c_ulong,
pub const LED_BLINK_SW: c_int = 0;
pub const LED_BLINK_ONESHOT: c_int = 1;
pub const LED_BLINK_ONESHOT_STOP: c_int = 2;
pub const LED_BLINK_INVERT: c_int = 3;
pub const LED_BLINK_BRIGHTNESS_CHANGE: c_int = 4;
pub const LED_BLINK_DISABLE: c_int = 5;
// Brightness off also disables hw-blinking so it is a separate action
pub const LED_SET_BRIGHTNESS_OFF: c_int = 6;
pub const LED_SET_BRIGHTNESS: c_int = 7;
pub const LED_SET_BLINK: c_int = 8;
// Set LED brightness level
// Must not sleep. Use brightness_set_blocking for drivers
// that can sleep while setting brightness.
//
    pub brightness): led_brightness,
//
// Set LED brightness level immediately - it can block the caller for
// the time required for accessing a LED device register.
//
    pub brightness): led_brightness,
// Get LED brightness level
    pub led_cdev): *mut *mut led_brightness (brightness_get)(struct led_classdev,
//
// Activate hardware accelerated blink, delays are in milliseconds
// and if both are zero then a sensible default should be chosen.
// The call should adjust the timings in that case and if it can't
// match the values specified exactly.
// Deactivate blinking again when the brightness is set to LED_OFF
// via the brightness_set() callback.
// For led_blink_set_nosleep() the LED core assumes that blink_set
// implementations, of drivers which do not use brightness_set_blocking,
// will not sleep. Therefor if brightness_set_blocking is not set
// this function must not sleep!
//
    pub delay_off): *mut c_ulong,
    pub repeat): *mut *mut led_pattern pattern, u32 len, int,
    pub led_cdev): *mut *mut int (pattern_clear)(struct led_classdev,
    pub dev: *mut device,
    pub groups: *const attribute_group,
    pub /: *mut *mut list_head node; / LED Device list,
    pub /: *const *const *const char default_trigger; / Trigger to use,
    pub blink_delay_off: unsigned long blink_delay_on,,
    pub blink_timer: timer_list,
    pub blink_brightness: c_int,
    pub new_blink_brightness: c_int,
    pub led_cdev): *mut *mut void (flash_resume)(struct led_classdev,
    pub /: *mut *mut *mut workqueue_wq; / LED workqueue,
    pub set_brightness_work: work_struct,
    pub delayed_set_value: c_int,
    pub delayed_delay_on: c_ulong,
    pub delayed_delay_off: c_ulong,

// Protects the trigger data below
    pub trigger_lock: rw_semaphore,
    pub trigger: *mut led_trigger,
    pub trig_list: list_head,
    pub trigger_data: *mut c_void,
// true if activated - deactivate routine uses it to do cleanup
    pub activated: bool,
// LEDs that have private triggers have this set
    pub trigger_type: *mut led_hw_trigger_type,
// Unique trigger name supported by LED set in hw control mode
    pub hw_control_trigger: *const c_char,
//
// Check if the LED driver supports the requested mode provided by the
// defined supported trigger to setup the LED to hw control mode.
//
// Return 0 on success. Return -EOPNOTSUPP when the passed flags are not
// supported and software fallback needs to be used.
// Return a negative error number on any other case  for check fail due
// to various reason like device not ready or timeouts.
//
    pub flags): c_ulong,
//
// Activate hardware control, LED driver will use the provided flags
// from the supported trigger and setup the LED to be driven by hardware
// following the requested mode from the trigger flags.
// Deactivate hardware blink control by setting brightness to LED_OFF via
// the brightness_set() callback.
//
// Return 0 on success, a negative error number on flags apply fail.
//
    pub flags): c_ulong,
//
// Get from the LED driver the current mode that the LED is set in hw
// control mode and put them in flags.
// Trigger can use this to get the initial state of a LED already set in
// hardware blink control.
//
// Return 0 on success, a negative error number on failing parsing the
// initial mode. Error from this function is NOT FATAL as the device
// may be in a not supported initial state by the attached LED trigger.
//
    pub flags): *mut c_ulong,
//
// Get the device this LED blinks in response to.
// e.g. for a PHY LED, it is the network device. If the LED is
// not yet associated to a device, return NULL.
//
    pub led_cdev): *mut *mut *mut device (hw_control_get_device)(led_classdev,

    pub brightness_hw_changed: c_int,
    pub brightness_hw_changed_kn: *mut kernfs_node,

// Ensures consistent access to the LED class device
    pub led_access: mutex,
}

//
// led_classdev_register_ext - register a new object of LED class with
// init data
// @parent: LED controller device this LED is driven by
// @led_cdev: the led_classdev structure for this device
// @init_data: the LED class device initialization data
//
// Register a new object of LED class, with name derived from init_data.
//
// Returns: 0 on success or negative error value on failure
//
// led_classdev_register - register a new object of LED class
// @parent: LED controller device this LED is driven by
// @led_cdev: the led_classdev structure for this device
//
// Register a new object of LED class, with name derived from the name property
// of passed led_cdev argument.
//
// Returns: 0 on success or negative error value on failure
//
extern "C" {
    pub fn led_classdev_register_ext(_arg: parent, _arg: led_cdev, _arg: NULL) -> return;
}
extern "C" {
    pub fn devm_led_classdev_register_ext(_arg: parent, _arg: led_cdev, _arg: NULL) -> return;
}
extern "C" {
    pub fn led_classdev_unregister(led_cdev: *mut led_classdev);
}
extern "C" {
    pub fn led_classdev_suspend(led_cdev: *mut led_classdev);
}
extern "C" {
    pub fn led_classdev_resume(led_cdev: *mut led_classdev);
}
extern "C" {
    pub fn led_add_lookup(led_lookup: *mut led_lookup_data);
}
extern "C" {
    pub fn led_remove_lookup(led_lookup: *mut led_lookup_data);
}
extern "C" {
    pub fn led_get(dev: *mut device, con_id: *mut c_char) -> *mut led_classdev __must_check;
}
extern "C" {
    pub fn devm_led_get(dev: *mut device, con_id: *mut c_char) -> *mut led_classdev __must_check;
}
extern "C" {
    pub fn led_put(led_cdev: *mut led_classdev);
}
//
// led_blink_set - set blinking with software fallback
// @led_cdev: the LED to start blinking
// @delay_on: the time it should be on (in ms)
// @delay_off: the time it should ble off (in ms)
//
// This function makes the LED blink, attempting to use the
// hardware acceleration if possible, but falling back to
// software blinking if there is no hardware blinking or if
// the LED refuses the passed values.
//
// This function may sleep!
//
// Note that if software blinking is active, simply calling
// led_cdev->brightness_set() will not stop the blinking,
// use led_set_brightness() instead.
//
// led_blink_set_nosleep - set blinking, guaranteed to not sleep
// @led_cdev: the LED to start blinking
// @delay_on: the time it should be on (in ms)
// @delay_off: the time it should ble off (in ms)
//
// This function makes the LED blink and is guaranteed to not sleep. Otherwise
// this is the same as led_blink_set(), see led_blink_set() for details.
//
// led_blink_set_oneshot - do a oneshot software blink
// @led_cdev: the LED to start blinking
// @delay_on: the time it should be on (in ms)
// @delay_off: the time it should ble off (in ms)
// @invert: blink off, then on, leaving the led on
//
// This function makes the LED blink one time for delay_on +
// delay_off time, ignoring the request if another one-shot
// blink is already in progress.
//
// If invert is set, led blinks for delay_off first, then for
// delay_on and leave the led on after the on-off cycle.
//
// This function is guaranteed not to sleep.
//
// led_set_brightness - set LED brightness
// @led_cdev: the LED to set
// @brightness: the brightness to set it to
//
// Set an LED's brightness, and, if necessary, cancel the
// software blink timer that implements blinking when the
// hardware doesn't. This function is guaranteed not to sleep.
//
extern "C" {
    pub fn led_set_brightness(led_cdev: *mut led_classdev, brightness: c_uint);
}
//
// led_set_brightness_sync - set LED brightness synchronously
// @led_cdev: the LED to set
// @value: the brightness to set it to
//
// Set an LED's brightness immediately. This function will block
// the caller for the time required for accessing device registers,
// and it can sleep.
//
// Returns: 0 on success or negative error value on failure
//
extern "C" {
    pub fn led_set_brightness_sync(led_cdev: *mut led_classdev, value: c_uint) -> c_int;
}
//
// led_mc_set_brightness - set mc LED color intensity values and brightness
// @led_cdev: the LED to set
// @intensity_value: array of per color intensity values to set
// @num_colors: amount of entries in intensity_value array
// @brightness: the brightness to set the LED to
//
// Set a multi-color LED's per color intensity values and brightness.
// If necessary, this cancels the software blink timer. This function is
// guaranteed not to sleep.
//
// Calling this function on a non multi-color led_classdev or with the wrong
// num_colors value is an error. In this case an error will be logged once
// and the call will do nothing.
//
// led_update_brightness - update LED brightness
// @led_cdev: the LED to query
//
// Get an LED's current brightness and update led_cdev->brightness
// member with the obtained value.
//
// Returns: 0 on success or negative error value on failure
//
extern "C" {
    pub fn led_update_brightness(led_cdev: *mut led_classdev) -> c_int;
}
//
// led_get_default_pattern - return default pattern
//
// @led_cdev: the LED to get default pattern for
// @size:     pointer for storing the number of elements in returned array,
// modified only if return != NULL
//
// Return:    Allocated array of integers with default pattern from device tree
// or NULL.  Caller is responsible for kfree().
//
// led_sysfs_disable - disable LED sysfs interface
// @led_cdev: the LED to set
//
// Disable the led_cdev's sysfs interface.
//
extern "C" {
    pub fn led_sysfs_disable(led_cdev: *mut led_classdev);
}
//
// led_sysfs_enable - enable LED sysfs interface
// @led_cdev: the LED to set
//
// Enable the led_cdev's sysfs interface.
//
extern "C" {
    pub fn led_sysfs_enable(led_cdev: *mut led_classdev);
}
//
// led_compose_name - compose LED class device name
// @dev: LED controller device object
// @init_data: the LED class device initialization data
// @led_classdev_name: composed LED class device name
//
// Create LED class device name basing on the provided init_data argument.
// The name can have <devicename:color:function> or <color:function>.
// form, depending on the init_data configuration.
//
// Returns: 0 on success or negative error value on failure
//
// led_get_color_name - get string representation of color ID
// @color_id: The LED_COLOR_ID_* constant
//
// Get the string name of a LED_COLOR_ID_* constant.
//
// Returns: A string constant or NULL on an invalid ID.
//
// led_sysfs_is_disabled - check if LED sysfs interface is disabled
// @led_cdev: the LED to query
//
// Returns: true if the led_cdev's sysfs interface is disabled.
//
// LED Triggers
//
// Registration functions for simple triggers

pub const TRIG_NAME_MAX: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_trigger {
// Trigger Properties
    pub name: *const c_char,
    pub led_cdev): *mut *mut int (activate)(struct led_classdev,
    pub led_cdev): *mut *mut void (deactivate)(struct led_classdev,
// Brightness set by led_trigger_event
    pub brightness: led_brightness,
// LED-private triggers have this set
    pub trigger_type: *mut led_hw_trigger_type,
// LEDs under control by this trigger (for simple triggers)
    pub leddev_list_lock: spinlock_t,
    pub led_cdevs: list_head,
// Link to next registered trigger
    pub next_trig: list_head,
    pub groups: *const attribute_group,
}

//
// Currently the attributes in struct led_trigger::groups are added directly to
// the LED device. As this might change in the future, the following
// macros abstract getting the LED device and its trigger_data from the dev
// parameter passed to the attribute accessor functions.
//

// Registration functions for complex triggers
extern "C" {
    pub fn led_trigger_register(trigger: *mut led_trigger) -> c_int;
}
extern "C" {
    pub fn led_trigger_unregister(trigger: *mut led_trigger);
}
extern "C" {
    pub fn led_trigger_unregister_simple(trigger: *mut led_trigger);
}
extern "C" {
    pub fn led_trigger_event(trigger: *mut led_trigger, event: led_brightness);
}
extern "C" {
    pub fn led_trigger_set_default(led_cdev: *mut led_classdev);
}
extern "C" {
    pub fn led_trigger_set(led_cdev: *mut led_classdev, trigger: *mut led_trigger) -> c_int;
}
extern "C" {
    pub fn led_trigger_remove(led_cdev: *mut led_classdev);
}

// Trigger has no members
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_trigger {
// Trigger inline empty functions
    pub 0: return,
    pub NULL: return,
    pub LED_OFF: return,

// Trigger specific enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_trigger_netdev_modes {
    TRIGGER_NETDEV_LINK = 0,
    TRIGGER_NETDEV_LINK_10,
    TRIGGER_NETDEV_LINK_100,
    TRIGGER_NETDEV_LINK_1000,
    TRIGGER_NETDEV_LINK_2500,
    TRIGGER_NETDEV_LINK_5000,
    TRIGGER_NETDEV_LINK_10000,
    TRIGGER_NETDEV_LINK_25000,
    TRIGGER_NETDEV_LINK_40000,
    TRIGGER_NETDEV_LINK_50000,
    TRIGGER_NETDEV_LINK_100000,
    TRIGGER_NETDEV_HALF_DUPLEX,
    TRIGGER_NETDEV_FULL_DUPLEX,
    TRIGGER_NETDEV_TX,
    TRIGGER_NETDEV_RX,
    TRIGGER_NETDEV_TX_ERR,
    TRIGGER_NETDEV_RX_ERR,

// Keep last
    __TRIGGER_NETDEV_MAX,
}

// Trigger specific functions

    pub write): void ledtrig_disk_activity(bool,

    pub ledtrig_mtd_activity(void): c_void,

    pub on): void ledtrig_flash_ctrl(bool,
    pub on): void ledtrig_torch_ctrl(bool,

    pub blank): void ledtrig_backlight_blank(bool,

//
// Generic LED platform data for describing LED names and default triggers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_info {
    pub name: *const c_char,
    pub default_trigger: *const c_char,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_platform_data {
    pub num_leds: c_int,
    pub leds: *mut led_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_properties {
    pub color: u32,
    pub color_present: bool,
    pub function: *const c_char,
    pub func_enum: u32,
    pub func_enum_present: bool,
    pub label: *const c_char,
}

// For the leds-gpio driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_led {
    pub name: *const c_char,
    pub default_trigger: *const c_char,

    pub gpio: unsigned,
    pub 1: unsigned active_low :,

    pub 1: unsigned retain_state_suspended :,
    pub 1: unsigned panic_indicator :,
    pub 2: unsigned default_state :,
    pub 1: unsigned retain_state_shutdown :,
// default_state should be one of LEDS_GPIO_DEFSTATE_(ON|OFF|KEEP)
    pub gpiod: *mut gpio_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_led_platform_data {
    pub num_leds: c_int,
    pub leds: *const gpio_led,

    pub gpio_blink_set: gpio_blink_set_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpu_led_event {
    CPU_LED_IDLE_START,	/* CPU enters idle */
    CPU_LED_IDLE_END,	/* CPU idle ends */
    CPU_LED_START,		/* Machine starts, especially resume */
    CPU_LED_STOP,		/* Machine stops, especially suspend */
    CPU_LED_HALTED,		/* Machine shutdown */
}

extern "C" {
    pub fn ledtrig_cpu(evt: cpu_led_event);
}

//
// struct led_pattern - pattern interval settings
// @delta_t: pattern interval delay, in milliseconds
// @brightness: pattern interval brightness
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct led_pattern {
    pub delta_t: u32,
    pub brightness: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_audio {
    LED_AUDIO_MUTE,		/* master mute LED */
    LED_AUDIO_MICMUTE,	/* mic mute LED */
    NUM_AUDIO_LEDS
}
