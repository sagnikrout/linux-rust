//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/power_supply.h
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
// Universal power supply monitor class
//
// Copyright © 2007  Anton Vorontsov <cbou@mail.ru>
// Copyright © 2004  Szabolcs Gyurko
// Copyright © 2003  Ian Molton <spyro@f2s.com>
//
// Modified: 2004, Oct     Szabolcs Gyurko
//

//
// All voltages, currents, charges, energies, time and temperatures in uV,
// µA, µAh, µWh, seconds and tenths of degree Celsius unless otherwise
// stated. It's driver's job to convert its raw values to units in which
// this class operates.
//
// For systems where the charger determines the maximum battery capacity
// the min and max fields should be used to present these values to user
// space. Unused/unknown fields will not appear in sysfs.
//
// What algorithm is the charger using?
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_supply_charge_type {
    POWER_SUPPLY_CHARGE_TYPE_UNKNOWN = 0,
    POWER_SUPPLY_CHARGE_TYPE_NONE,
    POWER_SUPPLY_CHARGE_TYPE_TRICKLE,	/* slow speed */
    POWER_SUPPLY_CHARGE_TYPE_FAST,		/* fast speed */
    POWER_SUPPLY_CHARGE_TYPE_STANDARD,	/* normal speed */
    POWER_SUPPLY_CHARGE_TYPE_ADAPTIVE,	/* dynamically adjusted speed */
    POWER_SUPPLY_CHARGE_TYPE_CUSTOM,	/* use CHARGE_CONTROL_* props */
    POWER_SUPPLY_CHARGE_TYPE_LONGLIFE,	/* slow speed, longer life */
    POWER_SUPPLY_CHARGE_TYPE_BYPASS,	/* bypassing the charger */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_supply_property {
// Properties of type `int'
    POWER_SUPPLY_PROP_STATUS = 0,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_CHARGE_TYPES,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_AUTHENTIC,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_CYCLE_COUNT,
    POWER_SUPPLY_PROP_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_VOLTAGE_MIN,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_AVG,
    POWER_SUPPLY_PROP_VOLTAGE_OCV,
    POWER_SUPPLY_PROP_VOLTAGE_BOOT,
    POWER_SUPPLY_PROP_CURRENT_MAX,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CURRENT_AVG,
    POWER_SUPPLY_PROP_CURRENT_BOOT,
    POWER_SUPPLY_PROP_POWER_NOW,
    POWER_SUPPLY_PROP_POWER_AVG,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_EMPTY_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_EMPTY,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_CHARGE_AVG,
    POWER_SUPPLY_PROP_CHARGE_COUNTER,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT,
    POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT_MAX,
    POWER_SUPPLY_PROP_CHARGE_CONTROL_START_THRESHOLD, /* in percents! */
    POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD, /* in percents! */
    POWER_SUPPLY_PROP_CHARGE_BEHAVIOUR,
    POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT,
    POWER_SUPPLY_PROP_INPUT_VOLTAGE_LIMIT,
    POWER_SUPPLY_PROP_INPUT_POWER_LIMIT,
    POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN,
    POWER_SUPPLY_PROP_ENERGY_EMPTY_DESIGN,
    POWER_SUPPLY_PROP_ENERGY_FULL,
    POWER_SUPPLY_PROP_ENERGY_EMPTY,
    POWER_SUPPLY_PROP_ENERGY_NOW,
    POWER_SUPPLY_PROP_ENERGY_AVG,
    POWER_SUPPLY_PROP_CAPACITY, /* in percents! */
    POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN, /* in percents! */
    POWER_SUPPLY_PROP_CAPACITY_ALERT_MAX, /* in percents! */
    POWER_SUPPLY_PROP_CAPACITY_ERROR_MARGIN, /* in percents! */
    POWER_SUPPLY_PROP_CAPACITY_LEVEL,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_TEMP_MAX,
    POWER_SUPPLY_PROP_TEMP_MIN,
    POWER_SUPPLY_PROP_TEMP_ALERT_MIN,
    POWER_SUPPLY_PROP_TEMP_ALERT_MAX,
    POWER_SUPPLY_PROP_TEMP_AMBIENT,
    POWER_SUPPLY_PROP_TEMP_AMBIENT_ALERT_MIN,
    POWER_SUPPLY_PROP_TEMP_AMBIENT_ALERT_MAX,
    POWER_SUPPLY_PROP_TIME_TO_EMPTY_NOW,
    POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG,
    POWER_SUPPLY_PROP_TIME_TO_FULL_NOW,
    POWER_SUPPLY_PROP_TIME_TO_FULL_AVG,
    POWER_SUPPLY_PROP_TYPE, /* use power_supply.type instead */
    POWER_SUPPLY_PROP_USB_TYPE,
    POWER_SUPPLY_PROP_SCOPE,
    POWER_SUPPLY_PROP_PRECHARGE_CURRENT,
    POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT,
    POWER_SUPPLY_PROP_CALIBRATE,
    POWER_SUPPLY_PROP_MANUFACTURE_YEAR,
    POWER_SUPPLY_PROP_MANUFACTURE_MONTH,
    POWER_SUPPLY_PROP_MANUFACTURE_DAY,
    POWER_SUPPLY_PROP_INTERNAL_RESISTANCE,
    POWER_SUPPLY_PROP_STATE_OF_HEALTH,
// Properties of type `const char *'
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
    POWER_SUPPLY_PROP_SERIAL_NUMBER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_supply_type {
    POWER_SUPPLY_TYPE_UNKNOWN = 0,
    POWER_SUPPLY_TYPE_BATTERY,
    POWER_SUPPLY_TYPE_UPS,
    POWER_SUPPLY_TYPE_MAINS,
    POWER_SUPPLY_TYPE_USB,			/* Standard Downstream Port */
    POWER_SUPPLY_TYPE_USB_DCP,		/* Dedicated Charging Port */
    POWER_SUPPLY_TYPE_USB_CDP,		/* Charging Downstream Port */
    POWER_SUPPLY_TYPE_USB_ACA,		/* Accessory Charger Adapters */
    POWER_SUPPLY_TYPE_USB_TYPE_C,		/* Type C Port */
    POWER_SUPPLY_TYPE_USB_PD,		/* Power Delivery Port */
    POWER_SUPPLY_TYPE_USB_PD_DRP,		/* PD Dual Role Port */
    POWER_SUPPLY_TYPE_APPLE_BRICK_ID,	/* Apple Charging Method */
    POWER_SUPPLY_TYPE_WIRELESS,		/* Wireless */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_supply_usb_type {
    POWER_SUPPLY_USB_TYPE_UNKNOWN = 0,
    POWER_SUPPLY_USB_TYPE_SDP,		/* Standard Downstream Port */
    POWER_SUPPLY_USB_TYPE_DCP,		/* Dedicated Charging Port */
    POWER_SUPPLY_USB_TYPE_CDP,		/* Charging Downstream Port */
    POWER_SUPPLY_USB_TYPE_ACA,		/* Accessory Charger Adapters */
    POWER_SUPPLY_USB_TYPE_C,		/* Type C Port */
    POWER_SUPPLY_USB_TYPE_PD,		/* Power Delivery Port */
    POWER_SUPPLY_USB_TYPE_PD_DRP,		/* PD Dual Role Port */
    POWER_SUPPLY_USB_TYPE_PD_PPS,		/* PD Programmable Power Supply */
// PD Standard Power Range Adjustable Voltage Supply
    POWER_SUPPLY_USB_TYPE_PD_SPR_AVS,
    POWER_SUPPLY_USB_TYPE_PD_PPS_SPR_AVS,	/* Supports both PD PPS + SPR AVS */
    POWER_SUPPLY_USB_TYPE_APPLE_BRICK_ID,	/* Apple Charging Method */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_supply_charge_behaviour {
    POWER_SUPPLY_CHARGE_BEHAVIOUR_AUTO = 0,
    POWER_SUPPLY_CHARGE_BEHAVIOUR_INHIBIT_CHARGE,
    POWER_SUPPLY_CHARGE_BEHAVIOUR_INHIBIT_CHARGE_AWAKE,
    POWER_SUPPLY_CHARGE_BEHAVIOUR_FORCE_DISCHARGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_supply_notifier_events {
    PSY_EVENT_PROP_CHANGED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union power_supply_propval {
    pub intval: c_int,
    pub strval: *const c_char,
}

// Run-time specific power supply configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_config {
    pub fwnode: *mut fwnode_handle,
// Driver private data
    pub drv_data: *mut c_void,
// Device specific sysfs attributes
    pub attr_grp: *const attribute_group,
    pub supplied_to: *mut c_char,
    pub num_supplicants: usize,
    pub no_wakeup_source: bool,
}

// Description of power supply
#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_desc {
    pub name: *const c_char,
    pub type: power_supply_type,
    pub charge_behaviours: u8,
    pub charge_types: u32,
    pub usb_types: u32,
    pub properties: *const power_supply_property,
    pub num_properties: usize,
//
// Functions for drivers implementing power supply class.
// These shouldn't be called directly by other drivers for accessing
// this power supply. Instead use power_supply_*() functions (for
// example power_supply_get_property()).
//
    pub val): *mut power_supply_propval,
    pub val): *const power_supply_propval,
//
// property_is_writeable() will be called during registration
// of power supply. If this happens during device probe then it must
// not access internal data of device (because probe did not end).
//
    pub psp): power_supply_property,
    pub psy): *mut *mut void (external_power_changed)(struct power_supply,
//
// Optional registration-time initialization. This runs in sleepable
// process context after driver data and any battery info are available,
// but before the device is added. The callback must not publish changes or
// start asynchronous activity that can access the power supply before
// registration completes. Return 0 on success or a negative errno on
// failure.
//
    pub psy): *mut *mut int (init)(struct power_supply,
//
// Set if thermal zone should not be created for this power supply.
// For example for virtual supplies forwarding calls to actual
// sensors or other supplies.
//
    pub no_thermal: bool,
// For APM emulation, think legacy userspace.
    pub use_for_apm: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_ext {
    pub name: *const *const c_char,
    pub charge_behaviours: u8,
    pub charge_types: u32,
    pub properties: *const power_supply_property,
    pub num_properties: usize,
    pub val): *mut power_supply_propval,
    pub val): *const power_supply_propval,
    pub psp): power_supply_property,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply {
    pub desc: *const power_supply_desc,
    pub supplied_to: *mut c_char,
    pub num_supplicants: usize,
    pub supplied_from: *mut c_char,
    pub num_supplies: usize,
// Driver private data
    pub drv_data: *mut c_void,
// private
    pub dev: device,
    pub changed_work: work_struct,
    pub deferred_register_work: delayed_work,
    pub changed_lock: spinlock_t,
    pub changed: bool,
    pub update_groups: bool,
    pub initialized: bool,
    pub removing: bool,
    pub use_cnt: core::sync::atomic::AtomicI32,
    pub battery_info: *mut power_supply_battery_info,
    pub /: *mut *mut rw_semaphore extensions_sem; / protects "extensions",
    pub extensions: list_head,

    pub tzd: *mut thermal_zone_device,
    pub tcd: *mut thermal_cooling_device,

    pub charging_or_full_trig: *mut led_trigger,
    pub online_trig: *mut led_trigger,
    pub charging_trig: *mut led_trigger,
    pub full_trig: *mut led_trigger,
    pub charging_blink_full_solid_trig: *mut led_trigger,
    pub charging_orange_full_green_trig: *mut led_trigger,

}

//
// This is recommended structure to specify static power supply parameters.
// Generic one, parametrizable for different power supplies. Power supply
// class itself does not use it, but that's what implementing most platform
// drivers, should try reuse for consistency.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_info {
    pub name: *const c_char,
    pub technology: c_int,
    pub voltage_max_design: c_int,
    pub voltage_min_design: c_int,
    pub charge_full_design: c_int,
    pub charge_empty_design: c_int,
    pub energy_full_design: c_int,
    pub energy_empty_design: c_int,
    pub use_for_apm: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_battery_ocv_table {
    pub /: *mut *mut int ocv; / microVolts,
    pub /: *mut *mut int capacity; / percent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_resistance_temp_table {
    pub /: *mut *mut int temp; / celsius,
    pub /: *mut *mut int resistance; / internal resistance percent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_vbat_ri_table {
    pub /: *mut *mut int vbat_uv; / Battery voltage in microvolt,
    pub /: *mut *mut int ri_uohm; / Internal resistance in microohm,
}

//
// struct power_supply_maintenance_charge_table - setting for maintenace charging
// @charge_current_max_ua: maintenance charging current that is used to keep
// the charge of the battery full as current is consumed after full charging.
// The corresponding charge_voltage_max_uv is used as a safeguard: when we
// reach this voltage the maintenance charging current is turned off. It is
// turned back on if we fall below this voltage.
// @charge_voltage_max_uv: maintenance charging voltage that is usually a bit
// lower than the constant_charge_voltage_max_uv. We can apply this settings
// charge_current_max_ua until we get back up to this voltage.
// @safety_timer_minutes: maintenance charging safety timer, with an expiry
// time in minutes. We will only use maintenance charging in this setting
// for a certain amount of time, then we will first move to the next
// maintenance charge current and voltage pair in respective array and wait
// for the next safety timer timeout, or, if we reached the last maintencance
// charging setting, disable charging until we reach
// charge_restart_voltage_uv and restart ordinary CC/CV charging from there.
// These timers should be chosen to align with the typical discharge curve
// for the battery.
//
// Ordinary CC/CV charging will stop charging when the charge current goes
// below charge_term_current_ua, and then restart it (if the device is still
// plugged into the charger) at charge_restart_voltage_uv. This happens in most
// consumer products because the power usage while connected to a charger is
// not zero, and devices are not manufactured to draw power directly from the
// charger: instead they will at all times dissipate the battery a little, like
// the power used in standby mode. This will over time give a charge graph
// such as this:
//
// Energy
// ^      ...        ...      ...      ...      ...      ...      ...
// |    .   .       .  .     .  .     .  .     .  .     .  .     .
// |  ..     .   ..     .  ..    .  ..    .  ..    .  ..    .  ..
// |.          ..        ..       ..       ..       ..       ..
// +-------------------------------------------------------------------> t
//
// Practically this means that the Li-ions are wandering back and forth in the
// battery and this causes degeneration of the battery anode and cathode.
// To prolong the life of the battery, maintenance charging is applied after
// reaching charge_term_current_ua to hold up the charge in the battery while
// consuming power, thus lowering the wear on the battery:
//
// Energy
// ^      .......................................
// |    .                                        ......................
// |  ..
// |.
// +-------------------------------------------------------------------> t
//
// Maintenance charging uses the voltages from this table: a table of settings
// is traversed using a slightly lower current and voltage than what is used for
// CC/CV charging. The maintenance charging will for safety reasons not go on
// indefinately: we lower the current and voltage with successive maintenance
// settings, then disable charging completely after we reach the last one,
// and after that we do not restart charging until we reach
// charge_restart_voltage_uv (see struct power_supply_battery_info) and restart
// ordinary CC/CV charging from there.
//
// As an example, a Samsung EB425161LA Lithium-Ion battery is CC/CV charged
// at 900mA to 4340mV, then maintenance charged at 600mA and 4150mV for up to
// 60 hours, then maintenance charged at 600mA and 4100mV for up to 200 hours.
// After this the charge cycle is restarted waiting for
// charge_restart_voltage_uv.
//
// For most mobile electronics this type of maintenance charging is enough for
// the user to disconnect the device and make use of it before both maintenance
// charging cycles are complete, if the current and voltage has been chosen
// appropriately. These need to be determined from battery discharge curves
// and expected standby current.
//
// If the voltage anyway drops to charge_restart_voltage_uv during maintenance
// charging, ordinary CC/CV charging is restarted. This can happen if the
// device is e.g. actively used during charging, so more current is drawn than
// the expected stand-by current. Also overvoltage protection will be applied
// as usual.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_maintenance_charge_table {
    pub charge_current_max_ua: c_int,
    pub charge_voltage_max_uv: c_int,
    pub charge_safety_timer_minutes: c_int,
}

pub const POWER_SUPPLY_OCV_TEMP_MAX: c_int = 20;
//
// struct power_supply_battery_info - information about batteries
// @technology: from the POWER_SUPPLY_TECHNOLOGY_* enum
// @energy_full_design_uwh: energy content when fully charged in microwatt
// hours
// @charge_full_design_uah: charge content when fully charged in microampere
// hours
// @voltage_min_design_uv: minimum voltage across the poles when the battery
// is at minimum voltage level in microvolts. If the voltage drops below this
// level the battery will need precharging when using CC/CV charging.
// @voltage_max_design_uv: voltage across the poles when the battery is fully
// charged in microvolts. This is the "nominal voltage" i.e. the voltage
// printed on the label of the battery.
// @tricklecharge_current_ua: the tricklecharge current used when trickle
// charging the battery in microamperes. This is the charging phase when the
// battery is completely empty and we need to carefully trickle in some
// charge until we reach the precharging voltage.
// @precharge_current_ua: current to use in the precharge phase in microamperes,
// the precharge rate is limited by limiting the current to this value.
// @precharge_voltage_max_uv: the maximum voltage allowed when precharging in
// microvolts. When we pass this voltage we will nominally switch over to the
// CC (constant current) charging phase defined by constant_charge_current_ua
// and constant_charge_voltage_max_uv.
// @charge_term_current_ua: when the current in the CV (constant voltage)
// charging phase drops below this value in microamperes the charging will
// terminate completely and not restart until the voltage over the battery
// poles reach charge_restart_voltage_uv unless we use maintenance charging.
// @charge_restart_voltage_uv: when the battery has been fully charged by
// CC/CV charging and charging has been disabled, and the voltage subsequently
// drops below this value in microvolts, the charging will be restarted
// (typically using CV charging).
// @overvoltage_limit_uv: If the voltage exceeds the nominal voltage
// voltage_max_design_uv and we reach this voltage level, all charging must
// stop and emergency procedures take place, such as shutting down the system
// in some cases.
// @constant_charge_current_max_ua: current in microamperes to use in the CC
// (constant current) charging phase. The charging rate is limited
// by this current. This is the main charging phase and as the current is
// constant into the battery the voltage slowly ascends to
// constant_charge_voltage_max_uv.
// @constant_charge_voltage_max_uv: voltage in microvolts signifying the end of
// the CC (constant current) charging phase and the beginning of the CV
// (constant voltage) charging phase.
// @maintenance_charge: an array of maintenance charging settings to be used
// after the main CC/CV charging phase is complete.
// @maintenance_charge_size: the number of maintenance charging settings in
// maintenance_charge.
// @alert_low_temp_charge_current_ua: The charging current to use if the battery
// enters low alert temperature, i.e. if the internal temperature is between
// temp_alert_min and temp_min. No matter the charging phase, this
// and alert_high_temp_charge_voltage_uv will be applied.
// @alert_low_temp_charge_voltage_uv: Same as alert_low_temp_charge_current_ua,
// but for the charging voltage.
// @alert_high_temp_charge_current_ua: The charging current to use if the
// battery enters high alert temperature, i.e. if the internal temperature is
// between temp_alert_max and temp_max. No matter the charging phase, this
// and alert_high_temp_charge_voltage_uv will be applied, usually lowering
// the charging current as an evasive manouver.
// @alert_high_temp_charge_voltage_uv: Same as
// alert_high_temp_charge_current_ua, but for the charging voltage.
// @factory_internal_resistance_uohm: the internal resistance of the battery
// at fabrication time, expressed in microohms. This resistance will vary
// depending on the lifetime and charge of the battery, so this is just a
// nominal ballpark figure. This internal resistance is given for the state
// when the battery is discharging.
// @factory_internal_resistance_charging_uohm: the internal resistance of the
// battery at fabrication time while charging, expressed in microohms.
// The charging process will affect the internal resistance of the battery
// so this value provides a better resistance under these circumstances.
// This resistance will vary depending on the lifetime and charge of the
// battery, so this is just a nominal ballpark figure.
// @ocv_temp: array indicating the open circuit voltage (OCV) capacity
// temperature indices. This is an array of temperatures in degrees Celsius
// indicating which capacity table to use for a certain temperature, since
// the capacity for reasons of chemistry will be different at different
// temperatures. Determining capacity is a multivariate problem and the
// temperature is the first variable we determine.
// @temp_ambient_alert_min: the battery will go outside of operating conditions
// when the ambient temperature goes below this temperature in degrees
// Celsius.
// @temp_ambient_alert_max: the battery will go outside of operating conditions
// when the ambient temperature goes above this temperature in degrees
// Celsius.
// @temp_alert_min: the battery should issue an alert if the internal
// temperature goes below this temperature in degrees Celsius.
// @temp_alert_max: the battery should issue an alert if the internal
// temperature goes above this temperature in degrees Celsius.
// @temp_min: the battery will go outside of operating conditions when
// the internal temperature goes below this temperature in degrees Celsius.
// Normally this means the system should shut down.
// @temp_max: the battery will go outside of operating conditions when
// the internal temperature goes above this temperature in degrees Celsius.
// Normally this means the system should shut down.
// @ocv_table: for each entry in ocv_temp there is a corresponding entry in
// ocv_table and a size for each entry in ocv_table_size. These arrays
// determine the capacity in percent in relation to the voltage in microvolts
// at the indexed temperature.
// @ocv_table_size: for each entry in ocv_temp this array is giving the size of
// each entry in the array of capacity arrays in ocv_table.
// @resist_table: this is a table that correlates a battery temperature to the
// expected internal resistance at this temperature. The resistance is given
// as a percentage of factory_internal_resistance_uohm. Knowing the
// resistance of the battery is usually necessary for calculating the open
// circuit voltage (OCV) that is then used with the ocv_table to calculate
// the capacity of the battery. The resist_table must be ordered descending
// by temperature: highest temperature with lowest resistance first, lowest
// temperature with highest resistance last.
// @resist_table_size: the number of items in the resist_table.
// @vbat2ri_discharging: this is a table that correlates Battery voltage (VBAT)
// to internal resistance (Ri). The resistance is given in microohm for the
// corresponding voltage in microvolts. The internal resistance is used to
// determine the open circuit voltage so that we can determine the capacity
// of the battery. These voltages to resistance tables apply when the battery
// is discharging. The table must be ordered descending by voltage: highest
// voltage first.
// @vbat2ri_discharging_size: the number of items in the vbat2ri_discharging
// table.
// @vbat2ri_charging: same function as vbat2ri_discharging but for the state
// when the battery is charging. Being under charge changes the battery's
// internal resistance characteristics so a separate table is needed.
// The table must be ordered descending by voltage: highest voltage first.
// @vbat2ri_charging_size: the number of items in the vbat2ri_charging
// table.
// @bti_resistance_ohm: The Battery Type Indicator (BIT) nominal resistance
// in ohms for this battery, if an identification resistor is mounted
// between a third battery terminal and ground. This scheme is used by a lot
// of mobile device batteries.
// @bti_resistance_tolerance: The tolerance in percent of the BTI resistance,
// for example 10 for +/- 10%, if the bti_resistance is set to 7000 and the
// tolerance is 10% we will detect a proper battery if the BTI resistance
// is between 6300 and 7700 Ohm.
//
// This is the recommended struct to manage static battery parameters,
// populated by power_supply_get_battery_info(). Most platform drivers should
// use these for consistency.
//
// Its field names must correspond to elements in enum power_supply_property.
// The default field value is -EINVAL or NULL for pointers.
//
// CC/CV CHARGING:
//
// The charging parameters here assume a CC/CV charging scheme. This method
// is most common with Lithium Ion batteries (other methods are possible) and
// looks as follows:
//
// ^ Battery voltage
// |                                               --- overvoltage_limit_uv
// |
// |                    ...................................................
// |                 .. constant_charge_voltage_max_uv
// |              ..
// |             .
// |            .
// |           .
// |          .
// |         .
// |     .. precharge_voltage_max_uv
// |  ..
// |. (trickle charging)
// +------------------------------------------------------------------> time
//
// ^ Current into the battery
// |
// |      ............. constant_charge_current_max_ua
// |      .            .
// |      .             .
// |      .              .
// |      .               .
// |      .                ..
// |      .                  ....
// |      .                       .....
// |    ... precharge_current_ua       .......  charge_term_current_ua
// |    .                                    .
// |.... tricklecharge_current_ua            .
// |                                         .
// +-----------------------------------------------------------------> time
//
// These diagrams are synchronized on time and the voltage and current
// follow each other.
//
// With CC/CV charging commence over time like this for an empty battery:
//
// 1. When the battery is completely empty it may need to be charged with
// an especially small current so that electrons just "trickle in",
// this is the tricklecharge_current_ua.
//
// 2. Next a small initial pre-charge current (precharge_current_ua)
// is applied if the voltage is below precharge_voltage_max_uv until we
// reach precharge_voltage_max_uv. CAUTION: in some texts this is referred
// to as "trickle charging" but the use in the Linux kernel is different
// see below!
//
// 3. Then the main charging current is applied, which is called the constant
// current (CC) phase. A current regulator is set up to allow
// constant_charge_current_max_ua of current to flow into the battery.
// The chemical reaction in the battery will make the voltage go up as
// charge goes into the battery. This current is applied until we reach
// the constant_charge_voltage_max_uv voltage.
//
// 4. At this voltage we switch over to the constant voltage (CV) phase. This
// means we allow current to go into the battery, but we keep the voltage
// fixed. This current will continue to charge the battery while keeping
// the voltage the same. A chemical reaction in the battery goes on
// storing energy without affecting the voltage. Over time the current
// will slowly drop and when we reach charge_term_current_ua we will
// end the constant voltage phase.
//
// After this the battery is fully charged, and if we do not support maintenance
// charging, the charging will not restart until power dissipation makes the
// voltage fall so that we reach charge_restart_voltage_uv and at this point
// we restart charging at the appropriate phase, usually this will be inside
// the CV phase.
//
// If we support maintenance charging the voltage is however kept high after
// the CV phase with a very low current. This is meant to let the same charge
// go in for usage while the charger is still connected, mainly for
// dissipation for the power consuming entity while connected to the
// charger.
//
// All charging MUST terminate if the overvoltage_limit_uv is ever reached.
// Overcharging Lithium Ion cells can be DANGEROUS and lead to fire or
// explosions.
//
// DETERMINING BATTERY CAPACITY:
//
// Several members of the struct deal with trying to determine the remaining
// capacity in the battery, usually as a percentage of charge. In practice
// many chargers uses a so-called fuel gauge or coloumb counter that measure
// how much charge goes into the battery and how much goes out (+/- leak
// consumption). This does not help if we do not know how much capacity the
// battery has to begin with, such as when it is first used or was taken out
// and charged in a separate charger. Therefore many capacity algorithms use
// the open circuit voltage with a look-up table to determine the rough
// capacity of the battery. The open circuit voltage can be conceptualized
// with an ideal voltage source (V) in series with an internal resistance (Ri)
// like this:
//
// +-------> IBAT >----------------+
// |                    ^          |
// [ ] Ri                |          |
// |                    | VBAT     |
// o <----------        |          |
// +|           ^        |         [ ] Rload
// .---.         |        |          |
// | V |         | OCV    |          |
// '---'         |        |          |
// |           |        |          |
// GND +-------------------------------+
//
// If we disconnect the load (here simplified as a fixed resistance Rload)
// and measure VBAT with a infinite impedance voltage meter we will get
// VBAT = OCV and this assumption is sometimes made even under load, assuming
// Rload is insignificant. However this will be of dubious quality because the
// load is rarely that small and Ri is strongly nonlinear depending on
// temperature and how much capacity is left in the battery due to the
// chemistry involved.
//
// In many practical applications we cannot just disconnect the battery from
// the load, so instead we often try to measure the instantaneous IBAT (the
// current out from the battery), estimate the Ri and thus calculate the
// voltage drop over Ri and compensate like this:
//
// OCV = VBAT - (IBAT * Ri)
//
// The tables vbat2ri_discharging and vbat2ri_charging are used to determine
// (by interpolation) the Ri from the VBAT under load. These curves are highly
// nonlinear and may need many datapoints but can be found in datasheets for
// some batteries. This gives the compensated open circuit voltage (OCV) for
// the battery even under load. Using this method will also compensate for
// temperature changes in the environment: this will also make the internal
// resistance change, and it will affect the VBAT under load, so correlating
// VBAT to Ri takes both remaining capacity and temperature into consideration.
//
// Alternatively a manufacturer can specify how the capacity of the battery
// is dependent on the battery temperature which is the main factor affecting
// Ri. As we know all checmical reactions are faster when it is warm and slower
// when it is cold. You can put in 1500mAh and only get 800mAh out before the
// voltage drops too low for example. This effect is also highly nonlinear and
// the purpose of the table resist_table: this will take a temperature and
// tell us how big percentage of Ri the specified temperature correlates to.
// Usually we have 100% of the factory_internal_resistance_uohm at 25 degrees
// Celsius.
//
// The power supply class itself doesn't use this struct as of now.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_supply_battery_info {
    pub technology: c_uint,
    pub energy_full_design_uwh: c_int,
    pub charge_full_design_uah: c_int,
    pub voltage_min_design_uv: c_int,
    pub voltage_max_design_uv: c_int,
    pub tricklecharge_current_ua: c_int,
    pub precharge_current_ua: c_int,
    pub precharge_voltage_max_uv: c_int,
    pub charge_term_current_ua: c_int,
    pub charge_restart_voltage_uv: c_int,
    pub overvoltage_limit_uv: c_int,
    pub constant_charge_current_max_ua: c_int,
    pub constant_charge_voltage_max_uv: c_int,
    pub maintenance_charge: *const power_supply_maintenance_charge_table,
    pub maintenance_charge_size: c_int,
    pub alert_low_temp_charge_current_ua: c_int,
    pub alert_low_temp_charge_voltage_uv: c_int,
    pub alert_high_temp_charge_current_ua: c_int,
    pub alert_high_temp_charge_voltage_uv: c_int,
    pub factory_internal_resistance_uohm: c_int,
    pub factory_internal_resistance_charging_uohm: c_int,
    pub ocv_temp: [c_int; POWER_SUPPLY_OCV_TEMP_MAX],
    pub temp_ambient_alert_min: c_int,
    pub temp_ambient_alert_max: c_int,
    pub temp_alert_min: c_int,
    pub temp_alert_max: c_int,
    pub temp_min: c_int,
    pub temp_max: c_int,
    pub ocv_table: [*const power_supply_battery_ocv_table; POWER_SUPPLY_OCV_TEMP_MAX],
    pub ocv_table_size: [c_int; POWER_SUPPLY_OCV_TEMP_MAX],
    pub resist_table: *const power_supply_resistance_temp_table,
    pub resist_table_size: c_int,
    pub vbat2ri_discharging: *const power_supply_vbat_ri_table,
    pub vbat2ri_discharging_size: c_int,
    pub vbat2ri_charging: *const power_supply_vbat_ri_table,
    pub vbat2ri_charging_size: c_int,
    pub bti_resistance_ohm: c_int,
    pub bti_resistance_tolerance: c_int,
}

extern "C" {
    pub fn power_supply_reg_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn power_supply_unreg_notifier(nb: *mut notifier_block);
}
extern "C" {
    pub fn power_supply_put_system_batteries(psys: *mut power_supply, count: c_int);
}
extern "C" {
    pub fn power_supply_put(psy: *mut power_supply);
}

// psys = NULL;

extern "C" {
    pub fn power_supply_changed(psy: *mut power_supply);
}
extern "C" {
    pub fn power_supply_am_i_supplied(psy: *mut power_supply) -> c_int;
}

extern "C" {
    pub fn power_supply_is_system_supplied() -> c_int;
}

extern "C" {
    pub fn power_supply_external_power_changed(psy: *mut power_supply);
}
extern "C" {
    pub fn power_supply_unregister(psy: *mut power_supply);
}
extern "C" {
    pub fn power_supply_powers(psy: *mut power_supply, dev: *mut device) -> c_int;
}

extern "C" {
    pub fn power_supply_for_each_psy(data: *mut c_void, psy: *mut *mut int (fn)(struct power_supply, data): *mut c_void) -> c_int;
}

extern "C" {
    pub fn power_supply_charge_behaviour_parse(available_behaviours: c_uint, buf: *const c_char) -> c_int;
}
extern "C" {
    pub fn power_supply_charge_types_parse(available_types: c_uint, buf: *const c_char) -> c_int;
}

