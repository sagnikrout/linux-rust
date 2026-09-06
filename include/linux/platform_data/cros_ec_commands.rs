//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/cros_ec_commands.h
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
// Host communication command constants for ChromeOS EC
//
// Copyright (C) 2012 Google, Inc
//
// NOTE: This file is auto-generated from ChromeOS EC Open Source code from
// https://chromium.googlesource.com/chromiumos/platform/ec/+/master/include/ec_commands.h
//
// Host communication command constants for Chrome EC

// Macro flag: #define BUILD_ASSERT(_cond)
//
// Current version of this protocol
//
// TODO(crosbug.com/p/11223): This is effectively useless; protocol is
// determined in other ways.  Remove this once the kernel code no longer
// depends on it.
//
pub const EC_PROTO_VERSION: c_uint = 0x00000002;
// Command version mask

// I/O addresses for ACPI commands
pub const EC_LPC_ADDR_ACPI_DATA: c_uint = 0x62;
pub const EC_LPC_ADDR_ACPI_CMD: c_uint = 0x66;
// I/O addresses for host command
pub const EC_LPC_ADDR_HOST_DATA: c_uint = 0x200;
pub const EC_LPC_ADDR_HOST_CMD: c_uint = 0x204;
// I/O addresses for host command args and params
// Protocol version 2
pub const EC_LPC_ADDR_HOST_ARGS: c_uint = 0x800  /* And 0x801, 0x802, 0x803 */;
pub const EC_LPC_ADDR_HOST_PARAM: c_uint = 0x804  /* For version 2 params; size is;
// EC_PROTO2_MAX_PARAM_SIZE
//
// Protocol version 3
pub const EC_LPC_ADDR_HOST_PACKET: c_uint = 0x800  /* Offset of version 3 packet */;
pub const EC_LPC_HOST_PACKET_SIZE: c_uint = 0x100  /* Max size of version 3 packet */;
//
// The actual block is 0x800-0x8ff, but some BIOSes think it's 0x880-0x8ff
// and they tell the kernel that so we have to think of it as two parts.
//
// Other BIOSes report only the I/O port region spanned by the Microchip
// MEC series EC; an attempt to address a larger region may fail.
//
pub const EC_HOST_CMD_REGION0: c_uint = 0x800;
pub const EC_HOST_CMD_REGION1: c_uint = 0x880;
pub const EC_HOST_CMD_REGION_SIZE: c_uint = 0x80;
pub const EC_HOST_CMD_MEC_REGION_SIZE: c_uint = 0x8;
// EC command register bit functions

pub const EC_LPC_ADDR_MEMMAP: c_uint = 0x900;

// The offset address of each type of data in mapped memory.
pub const EC_MEMMAP_TEMP_SENSOR: c_uint = 0x00 /* Temp sensors 0x00 - 0x0f */;
pub const EC_MEMMAP_FAN: c_uint = 0x10 /* Fan speeds 0x10 - 0x17 */;
pub const EC_MEMMAP_TEMP_SENSOR_B: c_uint = 0x18 /* More temp sensors 0x18 - 0x1f */;
pub const EC_MEMMAP_ID: c_uint = 0x20 /* 0x20 == 'E', 0x21 == 'C' */;
pub const EC_MEMMAP_ID_VERSION: c_uint = 0x22 /* Version of data in 0x20 - 0x2f */;
pub const EC_MEMMAP_THERMAL_VERSION: c_uint = 0x23 /* Version of data in 0x00 - 0x1f */;
pub const EC_MEMMAP_BATTERY_VERSION: c_uint = 0x24 /* Version of data in 0x40 - 0x7f */;
pub const EC_MEMMAP_SWITCHES_VERSION: c_uint = 0x25 /* Version of data in 0x30 - 0x33 */;
pub const EC_MEMMAP_EVENTS_VERSION: c_uint = 0x26 /* Version of data in 0x34 - 0x3f */;
pub const EC_MEMMAP_HOST_CMD_FLAGS: c_uint = 0x27 /* Host cmd interface flags (8 bits) */;
// Unused 0x28 - 0x2f
pub const EC_MEMMAP_SWITCHES: c_uint = 0x30	/* 8 bits */;
// Unused 0x31 - 0x33
pub const EC_MEMMAP_HOST_EVENTS: c_uint = 0x34 /* 64 bits */;
// Battery values are all 32 bits, unless otherwise noted.
pub const EC_MEMMAP_BATT_VOLT: c_uint = 0x40 /* Battery Present Voltage */;
pub const EC_MEMMAP_BATT_RATE: c_uint = 0x44 /* Battery Present Rate */;
pub const EC_MEMMAP_BATT_CAP: c_uint = 0x48 /* Battery Remaining Capacity */;
pub const EC_MEMMAP_BATT_FLAG: c_uint = 0x4c /* Battery State, see below (8-bit) */;
pub const EC_MEMMAP_BATT_COUNT: c_uint = 0x4d /* Battery Count (8-bit) */;
pub const EC_MEMMAP_BATT_INDEX: c_uint = 0x4e /* Current Battery Data Index (8-bit) */;
// Unused 0x4f
pub const EC_MEMMAP_BATT_DCAP: c_uint = 0x50 /* Battery Design Capacity */;
pub const EC_MEMMAP_BATT_DVLT: c_uint = 0x54 /* Battery Design Voltage */;
pub const EC_MEMMAP_BATT_LFCC: c_uint = 0x58 /* Battery Last Full Charge Capacity */;
pub const EC_MEMMAP_BATT_CCNT: c_uint = 0x5c /* Battery Cycle Count */;
// Strings are all 8 bytes (EC_MEMMAP_TEXT_MAX)
pub const EC_MEMMAP_BATT_MFGR: c_uint = 0x60 /* Battery Manufacturer String */;
pub const EC_MEMMAP_BATT_MODEL: c_uint = 0x68 /* Battery Model Number String */;
pub const EC_MEMMAP_BATT_SERIAL: c_uint = 0x70 /* Battery Serial Number String */;
pub const EC_MEMMAP_BATT_TYPE: c_uint = 0x78 /* Battery Type String */;
pub const EC_MEMMAP_ALS: c_uint = 0x80 /* ALS readings in lux (2 X 16 bits) */;
// Unused 0x84 - 0x8f
pub const EC_MEMMAP_ACC_STATUS: c_uint = 0x90 /* Accelerometer status (8 bits )*/;
// Unused 0x91
pub const EC_MEMMAP_ACC_DATA: c_uint = 0x92 /* Accelerometers data 0x92 - 0x9f */;
// 0x92: Lid Angle if available, LID_ANGLE_UNRELIABLE otherwise
// 0x94 - 0x99: 1st Accelerometer
// 0x9a - 0x9f: 2nd Accelerometer
pub const EC_MEMMAP_GYRO_DATA: c_uint = 0xa0 /* Gyroscope data 0xa0 - 0xa5 */;
// Unused 0xa6 - 0xdf
//
// ACPI is unable to access memory mapped data at or above this offset due to
// limitations of the ACPI protocol. Do not place data in the range 0xe0 - 0xfe
// which might be needed by ACPI.
//
pub const EC_MEMMAP_NO_ACPI: c_uint = 0xe0;
// Define the format of the accelerometer mapped memory status byte.
pub const EC_MEMMAP_ACC_STATUS_SAMPLE_ID_MASK: c_uint = 0x0f;

// Number of temp sensors at EC_MEMMAP_TEMP_SENSOR
pub const EC_TEMP_SENSOR_ENTRIES: c_int = 16;
//
// Number of temp sensors at EC_MEMMAP_TEMP_SENSOR_B.
//
// Valid only if EC_MEMMAP_THERMAL_VERSION returns >= 2.
//
pub const EC_TEMP_SENSOR_B_ENTRIES: c_int = 8;
// Special values for mapped temperature sensors
pub const EC_TEMP_SENSOR_NOT_PRESENT: c_uint = 0xff;
pub const EC_TEMP_SENSOR_ERROR: c_uint = 0xfe;
pub const EC_TEMP_SENSOR_NOT_POWERED: c_uint = 0xfd;
pub const EC_TEMP_SENSOR_NOT_CALIBRATED: c_uint = 0xfc;
//
// The offset of temperature value stored in mapped memory.  This allows
// reporting a temperature range of 200K to 454K = -73C to 181C.
//
pub const EC_TEMP_SENSOR_OFFSET: c_int = 200;
//
// Number of ALS readings at EC_MEMMAP_ALS
//
pub const EC_ALS_ENTRIES: c_int = 2;
//
// The default value a temperature sensor will return when it is present but
// has not been read this boot.  This is a reasonable number to avoid
// triggering alarms on the host.
//

pub const EC_FAN_SPEED_NOT_PRESENT: c_uint = 0xffff  /* Entry not present */;
pub const EC_FAN_SPEED_STALLED: c_uint = 0xfffe  /* Fan stalled */;
// Battery bit flags at EC_MEMMAP_BATT_FLAG.
pub const EC_BATT_FLAG_AC_PRESENT: c_uint = 0x01;
pub const EC_BATT_FLAG_BATT_PRESENT: c_uint = 0x02;
pub const EC_BATT_FLAG_DISCHARGING: c_uint = 0x04;
pub const EC_BATT_FLAG_CHARGING: c_uint = 0x08;
pub const EC_BATT_FLAG_LEVEL_CRITICAL: c_uint = 0x10;
// Set if some of the static/dynamic data is invalid (or outdated).
pub const EC_BATT_FLAG_INVALID_DATA: c_uint = 0x20;
// Switch flags at EC_MEMMAP_SWITCHES
pub const EC_SWITCH_LID_OPEN: c_uint = 0x01;
pub const EC_SWITCH_POWER_BUTTON_PRESSED: c_uint = 0x02;
pub const EC_SWITCH_WRITE_PROTECT_DISABLED: c_uint = 0x04;
// Was recovery requested via keyboard; now unused.
pub const EC_SWITCH_IGNORE1: c_uint = 0x08;
// Recovery requested via dedicated signal (from servo board)
pub const EC_SWITCH_DEDICATED_RECOVERY: c_uint = 0x10;
// Was fake developer mode switch; now unused.  Remove in next refactor.
pub const EC_SWITCH_IGNORE0: c_uint = 0x20;
// Host command interface flags
// Host command interface supports LPC args (LPC interface only)
pub const EC_HOST_CMD_FLAG_LPC_ARGS_SUPPORTED: c_uint = 0x01;
// Host command interface supports version 3 protocol
pub const EC_HOST_CMD_FLAG_VERSION_3: c_uint = 0x02;
// Wireless switch flags

pub const EC_WIRELESS_SWITCH_WLAN: c_uint = 0x01  /* WLAN radio */;
pub const EC_WIRELESS_SWITCH_BLUETOOTH: c_uint = 0x02  /* Bluetooth radio */;
pub const EC_WIRELESS_SWITCH_WWAN: c_uint = 0x04  /* WWAN power */;
pub const EC_WIRELESS_SWITCH_WLAN_POWER: c_uint = 0x08  /* WLAN power */;
//
// ACPI commands
//
// These are valid ONLY on the ACPI command/data port.
//
// ACPI Read Embedded Controller
//
// This reads from ACPI memory space on the EC (EC_ACPI_MEM_*).
//
// Use the following sequence:
//
// - Write EC_CMD_ACPI_READ to EC_LPC_ADDR_ACPI_CMD
// - Wait for EC_LPC_CMDR_PENDING bit to clear
// - Write address to EC_LPC_ADDR_ACPI_DATA
// - Wait for EC_LPC_CMDR_DATA bit to set
// - Read value from EC_LPC_ADDR_ACPI_DATA
//
pub const EC_CMD_ACPI_READ: c_uint = 0x0080;
//
// ACPI Write Embedded Controller
//
// This reads from ACPI memory space on the EC (EC_ACPI_MEM_*).
//
// Use the following sequence:
//
// - Write EC_CMD_ACPI_WRITE to EC_LPC_ADDR_ACPI_CMD
// - Wait for EC_LPC_CMDR_PENDING bit to clear
// - Write address to EC_LPC_ADDR_ACPI_DATA
// - Wait for EC_LPC_CMDR_PENDING bit to clear
// - Write value to EC_LPC_ADDR_ACPI_DATA
//
pub const EC_CMD_ACPI_WRITE: c_uint = 0x0081;
//
// ACPI Burst Enable Embedded Controller
//
// This enables burst mode on the EC to allow the host to issue several
// commands back-to-back. While in this mode, writes to mapped multi-byte
// data are locked out to ensure data consistency.
//
pub const EC_CMD_ACPI_BURST_ENABLE: c_uint = 0x0082;
//
// ACPI Burst Disable Embedded Controller
//
// This disables burst mode on the EC and stops preventing EC writes to mapped
// multi-byte data.
//
pub const EC_CMD_ACPI_BURST_DISABLE: c_uint = 0x0083;
//
// ACPI Query Embedded Controller
//
// This clears the lowest-order bit in the currently pending host events, and
// sets the result code to the 1-based index of the bit (event 0x00000001 = 1,
// event 0x80000000 = 32), or 0 if no event was pending.
//
pub const EC_CMD_ACPI_QUERY_EVENT: c_uint = 0x0084;
// Valid addresses in ACPI memory space, for read/write commands
// Memory space version; set to EC_ACPI_MEM_VERSION_CURRENT
pub const EC_ACPI_MEM_VERSION: c_uint = 0x00;
//
// Test location; writing value here updates test compliment byte to (0xff -
// value).
//
pub const EC_ACPI_MEM_TEST: c_uint = 0x01;
// Test compliment; writes here are ignored.
pub const EC_ACPI_MEM_TEST_COMPLIMENT: c_uint = 0x02;
// Keyboard backlight brightness percent (0 - 100)
pub const EC_ACPI_MEM_KEYBOARD_BACKLIGHT: c_uint = 0x03;
// DPTF Target Fan Duty (0-100, 0xff for auto/none)
pub const EC_ACPI_MEM_FAN_DUTY: c_uint = 0x04;
//
// DPTF temp thresholds. Any of the EC's temp sensors can have up to two
// independent thresholds attached to them. The current value of the ID
// register determines which sensor is affected by the THRESHOLD and COMMIT
// registers. The THRESHOLD register uses the same EC_TEMP_SENSOR_OFFSET scheme
// as the memory-mapped sensors. The COMMIT register applies those settings.
//
// The spec does not mandate any way to read back the threshold settings
// themselves, but when a threshold is crossed the AP needs a way to determine
// which sensor(s) are responsible. Each reading of the ID register clears and
// returns one sensor ID that has crossed one of its threshold (in either
// direction) since the last read. A value of 0xFF means "no new thresholds
// have tripped". Setting or enabling the thresholds for a sensor will clear
// the unread event count for that sensor.
//
pub const EC_ACPI_MEM_TEMP_ID: c_uint = 0x05;
pub const EC_ACPI_MEM_TEMP_THRESHOLD: c_uint = 0x06;
pub const EC_ACPI_MEM_TEMP_COMMIT: c_uint = 0x07;
//
// Here are the bits for the COMMIT register:
// bit 0 selects the threshold index for the chosen sensor (0/1)
// bit 1 enables/disables the selected threshold (0 = off, 1 = on)
// Each write to the commit register affects one threshold.
//

//
// Example:
//
// Set the thresholds for sensor 2 to 50 C and 60 C:
// write 2 to [0x05]      --  select temp sensor 2
// write 0x7b to [0x06]   --  C_TO_K(50) - EC_TEMP_SENSOR_OFFSET
// write 0x2 to [0x07]    --  enable threshold 0 with this value
// write 0x85 to [0x06]   --  C_TO_K(60) - EC_TEMP_SENSOR_OFFSET
// write 0x3 to [0x07]    --  enable threshold 1 with this value
//
// Disable the 60 C threshold, leaving the 50 C threshold unchanged:
// write 2 to [0x05]      --  select temp sensor 2
// write 0x1 to [0x07]    --  disable threshold 1
//
// DPTF battery charging current limit
pub const EC_ACPI_MEM_CHARGING_LIMIT: c_uint = 0x08;
// Charging limit is specified in 64 mA steps
pub const EC_ACPI_MEM_CHARGING_LIMIT_STEP_MA: c_int = 64;
// Value to disable DPTF battery charging limit
pub const EC_ACPI_MEM_CHARGING_LIMIT_DISABLED: c_uint = 0xff;
//
// Report device orientation
// Bits       Definition
// 3:1        Device DPTF Profile Number (DDPN)
// 0   = Reserved for backward compatibility (indicates no valid
// profile number. Host should fall back to using TBMD).
// 1..7 = DPTF Profile number to indicate to host which table needs
// to be loaded.
// 0         Tablet Mode Device Indicator (TBMD)
//
pub const EC_ACPI_MEM_DEVICE_ORIENTATION: c_uint = 0x09;
pub const EC_ACPI_MEM_TBMD_SHIFT: c_int = 0;
pub const EC_ACPI_MEM_TBMD_MASK: c_uint = 0x1;
pub const EC_ACPI_MEM_DDPN_SHIFT: c_int = 1;
pub const EC_ACPI_MEM_DDPN_MASK: c_uint = 0x7;
//
// Report device features. Uses the same format as the host command, except:
//
// bit 0 (EC_FEATURE_LIMITED) changes meaning from "EC code has a limited set
// of features", which is of limited interest when the system is already
// interpreting ACPI bytecode, to "EC_FEATURES[0-7] is not supported". Since
// these are supported, it defaults to 0.
// This allows detecting the presence of this field since older versions of
// the EC codebase would simply return 0xff to that unknown address. Check
// FEATURES0 != 0xff (or FEATURES0[0] == 0) to make sure that the other bits
// are valid.
//
pub const EC_ACPI_MEM_DEVICE_FEATURES0: c_uint = 0x0a;
pub const EC_ACPI_MEM_DEVICE_FEATURES1: c_uint = 0x0b;
pub const EC_ACPI_MEM_DEVICE_FEATURES2: c_uint = 0x0c;
pub const EC_ACPI_MEM_DEVICE_FEATURES3: c_uint = 0x0d;
pub const EC_ACPI_MEM_DEVICE_FEATURES4: c_uint = 0x0e;
pub const EC_ACPI_MEM_DEVICE_FEATURES5: c_uint = 0x0f;
pub const EC_ACPI_MEM_DEVICE_FEATURES6: c_uint = 0x10;
pub const EC_ACPI_MEM_DEVICE_FEATURES7: c_uint = 0x11;
pub const EC_ACPI_MEM_BATTERY_INDEX: c_uint = 0x12;
//
// USB Port Power. Each bit indicates whether the corresponding USB ports' power
// is enabled (1) or disabled (0).
// bit 0 USB port ID 0
// ...
// bit 7 USB port ID 7
//
pub const EC_ACPI_MEM_USB_PORT_POWER: c_uint = 0x13;
//
// ACPI addresses 0x20 - 0xff map to EC_MEMMAP offset 0x00 - 0xdf.  This data
// is read-only from the AP.  Added in EC_ACPI_MEM_VERSION 2.
//
pub const EC_ACPI_MEM_MAPPED_BEGIN: c_uint = 0x20;
pub const EC_ACPI_MEM_MAPPED_SIZE: c_uint = 0xe0;
// Current version of ACPI memory address space
pub const EC_ACPI_MEM_VERSION_CURRENT: c_int = 2;
//
// This header file is used in coreboot both in C and ACPI code.  The ACPI code
// is pre-processed to handle constants but the ASL compiler is unable to
// handle actual C code so keep it separate.
//
// Attributes for EC request and response packets.  Just defining __packed
// results in inefficient assembly code on ARM, if the structure is actually
// 32-bit aligned, as it should be for all buffers.
//
// Be very careful when adding these to existing structures.  They will round
// up the structure size to the specified boundary.
//
// Also be very careful to make that if a structure is included in some other
// parent structure that the alignment will still be true given the packing of
// the parent structure.  This is particularly important if the sub-structure
// will be passed as a pointer to another function, since that function will
// not know about the misaligment caused by the parent structure's packing.
//
// Also be very careful using __packed - particularly when nesting non-packed
// structures inside packed ones.  In fact, DO NOT use __packed directly;
// always use one of these attributes.
//
// Once everything is annotated properly, the following search strings should
// not return ANY matches in this file other than right here:
//
// "__packed" - generates inefficient code; all sub-structs must also be packed
//
// "struct [^_]" - all structs should be annotated, except for structs that are
// members of other structs/unions (and their original declarations should be
// annotated).
//
// Packed structures make no assumption about alignment, so they do inefficient
// byte-wise reads.
//

// Macro flag: #define __ec_todo_unpacked
// LPC command status byte masks
// EC has written a byte in the data register and host hasn't read it yet
pub const EC_LPC_STATUS_TO_HOST: c_uint = 0x01;
// Host has written a command/data byte and the EC hasn't read it yet
pub const EC_LPC_STATUS_FROM_HOST: c_uint = 0x02;
// EC is processing a command
pub const EC_LPC_STATUS_PROCESSING: c_uint = 0x04;
// Last write to EC was a command, not data
pub const EC_LPC_STATUS_LAST_CMD: c_uint = 0x08;
// EC is in burst mode
pub const EC_LPC_STATUS_BURST_MODE: c_uint = 0x10;
// SCI event is pending (requesting SCI query)
pub const EC_LPC_STATUS_SCI_PENDING: c_uint = 0x20;
// SMI event is pending (requesting SMI query)
pub const EC_LPC_STATUS_SMI_PENDING: c_uint = 0x40;
// (reserved)
pub const EC_LPC_STATUS_RESERVED: c_uint = 0x80;
//
// EC is busy.  This covers both the EC processing a command, and the host has
// written a new command but the EC hasn't picked it up yet.
//

//
// Host command response codes (16-bit).  Note that response codes should be
// stored in a uint16_t rather than directly in a value of this type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_status {
    EC_RES_SUCCESS = 0,
    EC_RES_INVALID_COMMAND = 1,
    EC_RES_ERROR = 2,
    EC_RES_INVALID_PARAM = 3,
    EC_RES_ACCESS_DENIED = 4,
    EC_RES_INVALID_RESPONSE = 5,
    EC_RES_INVALID_VERSION = 6,
    EC_RES_INVALID_CHECKSUM = 7,
    EC_RES_IN_PROGRESS = 8,		/* Accepted, command in progress */
    EC_RES_UNAVAILABLE = 9,		/* No response available */
    EC_RES_TIMEOUT = 10,		/* We got a timeout */
    EC_RES_OVERFLOW = 11,		/* Table / data overflow */
    EC_RES_INVALID_HEADER = 12,     /* Header contains invalid data */
    EC_RES_REQUEST_TRUNCATED = 13,  /* Didn't get the entire request */
    EC_RES_RESPONSE_TOO_BIG = 14,   /* Response was too big to handle */
    EC_RES_BUS_ERROR = 15,		/* Communications bus error */
    EC_RES_BUSY = 16,		/* Up but too busy.  Should retry */
    EC_RES_INVALID_HEADER_VERSION = 17,  /* Header version invalid */
    EC_RES_INVALID_HEADER_CRC = 18,      /* Header CRC invalid */
    EC_RES_INVALID_DATA_CRC = 19,        /* Data CRC invalid */
    EC_RES_DUP_UNAVAILABLE = 20,         /* Can't resend response */
}

//
// Host event codes.  Note these are 1-based, not 0-based, because ACPI query
// EC command uses code 0 to mean "no event pending".  We explicitly specify
// each value in the enum listing so they won't change if we delete/insert an
// item or rearrange the list (it needs to be stable across platforms, not
// just within a single compiled instance).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_event_code {
    EC_HOST_EVENT_LID_CLOSED = 1,
    EC_HOST_EVENT_LID_OPEN = 2,
    EC_HOST_EVENT_POWER_BUTTON = 3,
    EC_HOST_EVENT_AC_CONNECTED = 4,
    EC_HOST_EVENT_AC_DISCONNECTED = 5,
    EC_HOST_EVENT_BATTERY_LOW = 6,
    EC_HOST_EVENT_BATTERY_CRITICAL = 7,
    EC_HOST_EVENT_BATTERY = 8,
    EC_HOST_EVENT_THERMAL_THRESHOLD = 9,
// Event generated by a device attached to the EC
    EC_HOST_EVENT_DEVICE = 10,
    EC_HOST_EVENT_THERMAL = 11,
    EC_HOST_EVENT_USB_CHARGER = 12,
    EC_HOST_EVENT_KEY_PRESSED = 13,
//
// EC has finished initializing the host interface.  The host can check
// for this event following sending a EC_CMD_REBOOT_EC command to
// determine when the EC is ready to accept subsequent commands.
//
    EC_HOST_EVENT_INTERFACE_READY = 14,
// Keyboard recovery combo has been pressed
    EC_HOST_EVENT_KEYBOARD_RECOVERY = 15,

// Shutdown due to thermal overload
    EC_HOST_EVENT_THERMAL_SHUTDOWN = 16,
// Shutdown due to battery level too low
    EC_HOST_EVENT_BATTERY_SHUTDOWN = 17,

// Suggest that the AP throttle itself
    EC_HOST_EVENT_THROTTLE_START = 18,
// Suggest that the AP resume normal speed
    EC_HOST_EVENT_THROTTLE_STOP = 19,

// Hang detect logic detected a hang and host event timeout expired
    EC_HOST_EVENT_HANG_DETECT = 20,
// Hang detect logic detected a hang and warm rebooted the AP
    EC_HOST_EVENT_HANG_REBOOT = 21,

// PD MCU triggering host event
    EC_HOST_EVENT_PD_MCU = 22,

// Battery Status flags have changed
    EC_HOST_EVENT_BATTERY_STATUS = 23,

// EC encountered a panic, triggering a reset
    EC_HOST_EVENT_PANIC = 24,

// Keyboard fastboot combo has been pressed
    EC_HOST_EVENT_KEYBOARD_FASTBOOT = 25,

// EC RTC event occurred
    EC_HOST_EVENT_RTC = 26,

// Emulate MKBP event
    EC_HOST_EVENT_MKBP = 27,

// EC desires to change state of host-controlled USB mux
    EC_HOST_EVENT_USB_MUX = 28,

// TABLET/LAPTOP mode or detachable base attach/detach event
    EC_HOST_EVENT_MODE_CHANGE = 29,

// Keyboard recovery combo with hardware reinitialization
    EC_HOST_EVENT_KEYBOARD_RECOVERY_HW_REINIT = 30,

// WoV
    EC_HOST_EVENT_WOV = 31,

//
// The high bit of the event mask is not used as a host event code.  If
// it reads back as set, then the entire event mask should be
// considered invalid by the host.  This can happen when reading the
// raw event status via EC_MEMMAP_HOST_EVENTS but the LPC interface is
// not initialized on the EC, or improperly configured on the host.
//
    EC_HOST_EVENT_INVALID = 32
}

// Host event mask

//
// struct ec_lpc_host_args - Arguments at EC_LPC_ADDR_HOST_ARGS
// @flags: The host argument flags.
// @command_version: Command version.
// @data_size: The length of data.
// @checksum: Checksum; sum of command + flags + command_version + data_size +
// all params/response data bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_lpc_host_args {
    pub flags: u8,
    pub command_version: u8,
    pub data_size: u8,
    pub checksum: u8,
    pub __ec_align4: },
// Flags for ec_lpc_host_args.flags
//
// Args are from host.  Data area at EC_LPC_ADDR_HOST_PARAM contains command
// params.
//
// If EC gets a command and this flag is not set, this is an old-style command.
// Command version is 0 and params from host are at EC_LPC_ADDR_OLD_PARAM with
// unknown length.  EC must respond with an old-style response (that is,
// without setting EC_HOST_ARGS_FLAG_TO_HOST).
//
pub const EC_HOST_ARGS_FLAG_FROM_HOST: c_uint = 0x01;
//
// Args are from EC.  Data area at EC_LPC_ADDR_HOST_PARAM contains response.
//
// If EC responds to a command and this flag is not set, this is an old-style
// response.  Command version is 0 and response data from EC is at
// EC_LPC_ADDR_OLD_PARAM with unknown length.
//
pub const EC_HOST_ARGS_FLAG_TO_HOST: c_uint = 0x02;
//
// Byte codes returned by EC over SPI interface.
//
// These can be used by the AP to debug the EC interface, and to determine
// when the EC is not in a state where it will ever get around to responding
// to the AP.
//
// Example of sequence of bytes read from EC for a current good transfer:
// 1. -                  - AP asserts chip select (CS#)
// 2. EC_SPI_OLD_READY   - AP sends first byte(s) of request
// 3. -                  - EC starts handling CS# interrupt
// 4. EC_SPI_RECEIVING   - AP sends remaining byte(s) of request
// 5. EC_SPI_PROCESSING  - EC starts processing request; AP is clocking in
// bytes looking for EC_SPI_FRAME_START
// 6. -                  - EC finishes processing and sets up response
// 7. EC_SPI_FRAME_START - AP reads frame byte
// 8. (response packet)  - AP reads response packet
// 9. EC_SPI_PAST_END    - Any additional bytes read by AP
// 10 -                  - AP deasserts chip select
// 11 -                  - EC processes CS# interrupt and sets up DMA for
// next request
//
// If the AP is waiting for EC_SPI_FRAME_START and sees any value other than
// the following byte values:
// EC_SPI_OLD_READY
// EC_SPI_RX_READY
// EC_SPI_RECEIVING
// EC_SPI_PROCESSING
//
// Then the EC found an error in the request, or was not ready for the request
// and lost data.  The AP should give up waiting for EC_SPI_FRAME_START,
// because the EC is unable to tell when the AP is done sending its request.
//
// Framing byte which precedes a response packet from the EC.  After sending a
// request, the AP will clock in bytes until it sees the framing byte, then
// clock in the response packet.
//
pub const EC_SPI_FRAME_START: c_uint = 0xec;
//
// Padding bytes which are clocked out after the end of a response packet.
//
pub const EC_SPI_PAST_END: c_uint = 0xed;
//
// EC is ready to receive, and has ignored the byte sent by the AP.  EC expects
// that the AP will send a valid packet header (starting with
// EC_COMMAND_PROTOCOL_3) in the next 32 bytes.
//
pub const EC_SPI_RX_READY: c_uint = 0xf8;
//
// EC has started receiving the request from the AP, but hasn't started
// processing it yet.
//
pub const EC_SPI_RECEIVING: c_uint = 0xf9;
// EC has received the entire request from the AP and is processing it.
pub const EC_SPI_PROCESSING: c_uint = 0xfa;
//
// EC received bad data from the AP, such as a packet header with an invalid
// length.  EC will ignore all data until chip select deasserts.
//
pub const EC_SPI_RX_BAD_DATA: c_uint = 0xfb;
//
// EC received data from the AP before it was ready.  That is, the AP asserted
// chip select and started clocking data before the EC was ready to receive it.
// EC will ignore all data until chip select deasserts.
//
pub const EC_SPI_NOT_READY: c_uint = 0xfc;
//
// EC was ready to receive a request from the AP.  EC has treated the byte sent
// by the AP as part of a request packet, or (for old-style ECs) is processing
// a fully received packet but is not ready to respond yet.
//
pub const EC_SPI_OLD_READY: c_uint = 0xfd;
//
// Protocol version 2 for I2C and SPI send a request this way:
//
// 0	EC_CMD_VERSION0 + (command version)
// 1	Command number
// 2	Length of params = N
// 3..N+2	Params, if any
// N+3	8-bit checksum of bytes 0..N+2
//
// The corresponding response is:
//
// 0	Result code (EC_RES_*)
// 1	Length of params = M
// 2..M+1	Params, if any
// M+2	8-bit checksum of bytes 0..M+1
//
pub const EC_PROTO2_REQUEST_HEADER_BYTES: c_int = 3;
pub const EC_PROTO2_REQUEST_TRAILER_BYTES: c_int = 1;

pub const EC_PROTO2_RESPONSE_HEADER_BYTES: c_int = 2;
pub const EC_PROTO2_RESPONSE_TRAILER_BYTES: c_int = 1;

// Parameter length was limited by the LPC interface
pub const EC_PROTO2_MAX_PARAM_SIZE: c_uint = 0xfc;
// Maximum request and response packet sizes for protocol version 2

//
// Value written to legacy command port / prefix byte to indicate protocol
// 3+ structs are being used.  Usage is bus-dependent.
//
pub const EC_COMMAND_PROTOCOL_3: c_uint = 0xda;
pub const EC_HOST_REQUEST_VERSION: c_int = 3;
//
// struct ec_host_request - Version 3 request from host.
// @struct_version: Should be 3. The EC will return EC_RES_INVALID_HEADER if it
// receives a header with a version it doesn't know how to
// parse.
// @checksum: Checksum of request and data; sum of all bytes including checksum
// should total to 0.
// @command: Command to send (EC_CMD_...)
// @command_version: Command version.
// @reserved: Unused byte in current protocol version; set to 0.
// @data_len: Length of data which follows this header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_host_request {
    pub struct_version: u8,
    pub checksum: u8,
    pub command: u16,
    pub command_version: u8,
    pub reserved: u8,
    pub data_len: u16,
    pub __ec_align4: },
pub const EC_HOST_RESPONSE_VERSION: c_int = 3;
//
// struct ec_host_response - Version 3 response from EC.
// @struct_version: Struct version (=3).
// @checksum: Checksum of response and data; sum of all bytes including
// checksum should total to 0.
// @result: EC's response to the command (separate from communication failure)
// @data_len: Length of data which follows this header.
// @reserved: Unused bytes in current protocol version; set to 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_host_response {
    pub struct_version: u8,
    pub checksum: u8,
    pub result: u16,
    pub data_len: u16,
    pub reserved: u16,
    pub __ec_align4: },
//
// Host command protocol V4.
//
// Packets always start with a request or response header.  They are followed
// by data_len bytes of data.  If the data_crc_present flag is set, the data
// bytes are followed by a CRC-8 of that data, using x^8 + x^2 + x + 1
// polynomial.
//
// Host algorithm when sending a request q:
//
// 101) tries_left=(some value, e.g. 3);
// 102) q.seq_num++
// 103) q.seq_dup=0
// 104) Calculate q.header_crc.
// 105) Send request q to EC.
// 106) Wait for response r.  Go to 201 if received or 301 if timeout.
//
// 201) If r.struct_version != 4, go to 301.
// 202) If r.header_crc mismatches calculated CRC for r header, go to 301.
// 203) If r.data_crc_present and r.data_crc mismatches, go to 301.
// 204) If r.seq_num != q.seq_num, go to 301.
// 205) If r.seq_dup == q.seq_dup, return success.
// 207) If r.seq_dup == 1, go to 301.
// 208) Return error.
//
// 301) If --tries_left <= 0, return error.
// 302) If q.seq_dup == 1, go to 105.
// 303) q.seq_dup = 1
// 304) Go to 104.
//
// EC algorithm when receiving a request q.
// EC has response buffer r, error buffer e.
//
// 101) If q.struct_version != 4, set e.result = EC_RES_INVALID_HEADER_VERSION
// and go to 301
// 102) If q.header_crc mismatches calculated CRC, set e.result =
// EC_RES_INVALID_HEADER_CRC and go to 301
// 103) If q.data_crc_present, calculate data CRC.  If that mismatches the CRC
// byte at the end of the packet, set e.result = EC_RES_INVALID_DATA_CRC
// and go to 301.
// 104) If q.seq_dup == 0, go to 201.
// 105) If q.seq_num != r.seq_num, go to 201.
// 106) If q.seq_dup == r.seq_dup, go to 205, else go to 203.
//
// 201) Process request q into response r.
// 202) r.seq_num = q.seq_num
// 203) r.seq_dup = q.seq_dup
// 204) Calculate r.header_crc
// 205) If r.data_len > 0 and data is no longer available, set e.result =
// EC_RES_DUP_UNAVAILABLE and go to 301.
// 206) Send response r.
//
// 301) e.seq_num = q.seq_num
// 302) e.seq_dup = q.seq_dup
// 303) Calculate e.header_crc.
// 304) Send error response e.
//
// Version 4 request from host
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_host_request4 {
//
// bits 0-3: struct_version: Structure version (=4)
// bit    4: is_response: Is response (=0)
// bits 5-6: seq_num: Sequence number
// bit    7: seq_dup: Sequence duplicate flag
//
    pub fields0: u8,
//
// bits 0-4: command_version: Command version
// bits 5-6: Reserved (set 0, ignore on read)
// bit    7: data_crc_present: Is data CRC present after data
//
    pub fields1: u8,
// Command code (EC_CMD_*)
    pub command: u16,
// Length of data which follows this header (not including data CRC)
    pub data_len: u16,
// Reserved (set 0, ignore on read)
    pub reserved: u8,
// CRC-8 of above fields, using x^8 + x^2 + x + 1 polynomial
    pub header_crc: u8,
    pub __ec_align4: },
// Version 4 response from EC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_host_response4 {
//
// bits 0-3: struct_version: Structure version (=4)
// bit    4: is_response: Is response (=1)
// bits 5-6: seq_num: Sequence number
// bit    7: seq_dup: Sequence duplicate flag
//
    pub fields0: u8,
//
// bits 0-6: Reserved (set 0, ignore on read)
// bit    7: data_crc_present: Is data CRC present after data
//
    pub fields1: u8,
// Result code (EC_RES_*)
    pub result: u16,
// Length of data which follows this header (not including data CRC)
    pub data_len: u16,
// Reserved (set 0, ignore on read)
    pub reserved: u8,
// CRC-8 of above fields, using x^8 + x^2 + x + 1 polynomial
    pub header_crc: u8,
    pub __ec_align4: },
// Fields in fields0 byte
pub const EC_PACKET4_0_STRUCT_VERSION_MASK: c_uint = 0x0f;
pub const EC_PACKET4_0_IS_RESPONSE_MASK: c_uint = 0x10;
pub const EC_PACKET4_0_SEQ_NUM_SHIFT: c_int = 5;
pub const EC_PACKET4_0_SEQ_NUM_MASK: c_uint = 0x60;
pub const EC_PACKET4_0_SEQ_DUP_MASK: c_uint = 0x80;
// Fields in fields1 byte
pub const EC_PACKET4_1_COMMAND_VERSION_MASK: c_uint = 0x1f  /* (request only) */;
pub const EC_PACKET4_1_DATA_CRC_PRESENT_MASK: c_uint = 0x80;
//
// Notes on commands:
//
// Each command is an 16-bit command value.  Commands which take params or
// return response data specify structures for that data.  If no structure is
// specified, the command does not input or output data, respectively.
// Parameter/response length is implicit in the structs.  Some underlying
// communication protocols (I2C, SPI) may add length or checksum headers, but
// those are implementation-dependent and not defined here.
//
// All commands MUST be #defined to be 4-digit UPPER CASE hex values
// (e.g., 0x00AB, not 0xab) for CONFIG_HOSTCMD_SECTION_SORTED to work.
//
// General / test commands
//
// Get protocol version, used to deal with non-backward compatible protocol
// changes.
//
pub const EC_CMD_PROTO_VERSION: c_uint = 0x0000;
//
// struct ec_response_proto_version - Response to the proto version command.
// @version: The protocol version.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_proto_version {
    pub version: u32,
    pub __ec_align4: },
//
// Hello.  This is a simple command to test the EC is responsive to
// commands.
//
pub const EC_CMD_HELLO: c_uint = 0x0001;
//
// struct ec_params_hello - Parameters to the hello command.
// @in_data: Pass anything here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_hello {
    pub in_data: u32,
    pub __ec_align4: },
//
// struct ec_response_hello - Response to the hello command.
// @out_data: Output will be in_data + 0x01020304.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_hello {
    pub out_data: u32,
    pub __ec_align4: },
// Get version number
pub const EC_CMD_GET_VERSION: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_current_image {
    EC_IMAGE_UNKNOWN = 0,
    EC_IMAGE_RO,
    EC_IMAGE_RW
}

//
// struct ec_response_get_version - Response to the get version command.
// @version_string_ro: Null-terminated RO firmware version string.
// @version_string_rw: Null-terminated RW firmware version string.
// @reserved: Unused bytes; was previously RW-B firmware version string.
// @current_image: One of ec_current_image.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_version {
    pub version_string_ro: [c_char; 32],
    pub version_string_rw: [c_char; 32],
    pub reserved: [c_char; 32],
    pub current_image: u32,
    pub __ec_align4: },
// Read test
pub const EC_CMD_READ_TEST: c_uint = 0x0003;
//
// struct ec_params_read_test - Parameters for the read test command.
// @offset: Starting value for read buffer.
// @size: Size to read in bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_read_test {
    pub offset: u32,
    pub size: u32,
    pub __ec_align4: },
//
// struct ec_response_read_test - Response to the read test command.
// @data: Data returned by the read test command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_read_test {
    pub data: [u32; 32],
    pub __ec_align4: },
//
// Get build information
//
// Response is null-terminated string.
//
pub const EC_CMD_GET_BUILD_INFO: c_uint = 0x0004;
// Get chip info
pub const EC_CMD_GET_CHIP_INFO: c_uint = 0x0005;
//
// struct ec_response_get_chip_info - Response to the get chip info command.
// @vendor: Null-terminated string for chip vendor.
// @name: Null-terminated string for chip name.
// @revision: Null-terminated string for chip mask version.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_chip_info {
    pub vendor: [c_char; 32],
    pub name: [c_char; 32],
    pub revision: [c_char; 32],
    pub __ec_align4: },
// Get board HW version
pub const EC_CMD_GET_BOARD_VERSION: c_uint = 0x0006;
//
// struct ec_response_board_version - Response to the board version command.
// @board_version: A monotonously incrementing number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_board_version {
    pub board_version: u16,
    pub __ec_align2: },
//
// Read memory-mapped data.
//
// This is an alternate interface to memory-mapped data for bus protocols
// which don't support direct-mapped memory - I2C, SPI, etc.
//
// Response is params.size bytes of data.
//
pub const EC_CMD_READ_MEMMAP: c_uint = 0x0007;
//
// struct ec_params_read_memmap - Parameters for the read memory map command.
// @offset: Offset in memmap (EC_MEMMAP_*).
// @size: Size to read in bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_read_memmap {
    pub offset: u8,
    pub size: u8,
    pub __ec_align1: },
// Read versions supported for a command
pub const EC_CMD_GET_CMD_VERSIONS: c_uint = 0x0008;
//
// struct ec_params_get_cmd_versions - Parameters for the get command versions.
// @cmd: Command to check.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_get_cmd_versions {
    pub cmd: u8,
    pub __ec_align1: },
//
// struct ec_params_get_cmd_versions_v1 - Parameters for the get command
// versions (v1)
// @cmd: Command to check.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_get_cmd_versions_v1 {
    pub cmd: u16,
    pub __ec_align2: },
//
// struct ec_response_get_cmd_versions - Response to the get command versions.
// @version_mask: Mask of supported versions; use EC_VER_MASK() to compare with
// a desired version.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_cmd_versions {
    pub version_mask: u32,
    pub __ec_align4: },
//
// Check EC communications status (busy). This is needed on i2c/spi but not
// on lpc since it has its own out-of-band busy indicator.
//
// lpc must read the status from the command register. Attempting this on
// lpc will overwrite the args/parameter space and corrupt its data.
//
pub const EC_CMD_GET_COMMS_STATUS: c_uint = 0x0009;
// Avoid using ec_status which is for return values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_comms_status {
    EC_COMMS_STATUS_PROCESSING	= BIT(0),	/* Processing cmd */
}

//
// struct ec_response_get_comms_status - Response to the get comms status
// command.
// @flags: Mask of enum ec_comms_status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_comms_status {
    pub /: *mut *mut uint32_t flags; / Mask of enum ec_comms_status,
    pub __ec_align4: },
// Fake a variety of responses, purely for testing purposes.
pub const EC_CMD_TEST_PROTOCOL: c_uint = 0x000A;
// Tell the EC what to send back to us.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_test_protocol {
    pub ec_result: u32,
    pub ret_len: u32,
    pub buf: [u8; 32],
    pub __ec_align4: },
// Here it comes...
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_test_protocol {
    pub buf: [u8; 32],
    pub __ec_align4: },
// Get protocol information
pub const EC_CMD_GET_PROTOCOL_INFO: c_uint = 0x000B;
// Flags for ec_response_get_protocol_info.flags
// EC_RES_IN_PROGRESS may be returned if a command is slow

//
// struct ec_response_get_protocol_info - Response to the get protocol info.
// @protocol_versions: Bitmask of protocol versions supported (1 << n means
// version n).
// @max_request_packet_size: Maximum request packet size in bytes.
// @max_response_packet_size: Maximum response packet size in bytes.
// @flags: see EC_PROTOCOL_INFO_
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_protocol_info {
// Fields which exist if at least protocol version 3 supported
    pub protocol_versions: u32,
    pub max_request_packet_size: u16,
    pub max_response_packet_size: u16,
    pub flags: u32,
    pub __ec_align4: },
//
// Get/Set miscellaneous values
// The upper byte of .flags tells what to do (nothing means "get")
pub const EC_GSV_SET: c_uint = 0x80000000;
//
// The lower three bytes of .flags identifies the parameter, if that has
// meaning for an individual command.
//
pub const EC_GSV_PARAM_MASK: c_uint = 0x00ffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_get_set_value {
    pub flags: u32,
    pub value: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_set_value {
    pub flags: u32,
    pub value: u32,
    pub __ec_align4: },
// More than one command can use these structs to get/set parameters.
pub const EC_CMD_GSV_PAUSE_IN_S5: c_uint = 0x000C;
//
// List the features supported by the firmware
pub const EC_CMD_GET_FEATURES: c_uint = 0x000D;
// Supported features
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_feature_code {
//
// This image contains a limited set of features. Another image
// in RW partition may support more features.
//
    EC_FEATURE_LIMITED = 0,
//
// Commands for probing/reading/writing/erasing the flash in the
// EC are present.
//
    EC_FEATURE_FLASH = 1,
//
// Can control the fan speed directly.
//
    EC_FEATURE_PWM_FAN = 2,
//
// Can control the intensity of the keyboard backlight.
//
    EC_FEATURE_PWM_KEYB = 3,
//
// Support Google lightbar, introduced on Pixel.
//
    EC_FEATURE_LIGHTBAR = 4,
// Control of LEDs
    EC_FEATURE_LED = 5,
// Exposes an interface to control gyro and sensors.
// The host goes through the EC to access these sensors.
// In addition, the EC may provide composite sensors, like lid angle.
//
    EC_FEATURE_MOTION_SENSE = 6,
// The keyboard is controlled by the EC
    EC_FEATURE_KEYB = 7,
// The AP can use part of the EC flash as persistent storage.
    EC_FEATURE_PSTORE = 8,
// The EC monitors BIOS port 80h, and can return POST codes.
    EC_FEATURE_PORT80 = 9,
//
// Thermal management: include TMP specific commands.
// Higher level than direct fan control.
//
    EC_FEATURE_THERMAL = 10,
// Can switch the screen backlight on/off
    EC_FEATURE_BKLIGHT_SWITCH = 11,
// Can switch the wifi module on/off
    EC_FEATURE_WIFI_SWITCH = 12,
// Monitor host events, through for example SMI or SCI
    EC_FEATURE_HOST_EVENTS = 13,
// The EC exposes GPIO commands to control/monitor connected devices.
    EC_FEATURE_GPIO = 14,
// The EC can send i2c messages to downstream devices.
    EC_FEATURE_I2C = 15,
// Command to control charger are included
    EC_FEATURE_CHARGER = 16,
// Simple battery support.
    EC_FEATURE_BATTERY = 17,
//
// Support Smart battery protocol
// (Common Smart Battery System Interface Specification)
//
    EC_FEATURE_SMART_BATTERY = 18,
// EC can detect when the host hangs.
    EC_FEATURE_HANG_DETECT = 19,
// Report power information, for pit only
    EC_FEATURE_PMU = 20,
// Another Cros EC device is present downstream of this one
    EC_FEATURE_SUB_MCU = 21,
// Support USB Power delivery (PD) commands
    EC_FEATURE_USB_PD = 22,
// Control USB multiplexer, for audio through USB port for instance.
    EC_FEATURE_USB_MUX = 23,
// Motion Sensor code has an internal software FIFO
    EC_FEATURE_MOTION_SENSE_FIFO = 24,
// Support temporary secure vstore
    EC_FEATURE_VSTORE = 25,
// EC decides on USB-C SS mux state, muxes configured by host
    EC_FEATURE_USBC_SS_MUX_VIRTUAL = 26,
// EC has RTC feature that can be controlled by host commands
    EC_FEATURE_RTC = 27,
// The MCU exposes a Fingerprint sensor
    EC_FEATURE_FINGERPRINT = 28,
// The MCU exposes a Touchpad
    EC_FEATURE_TOUCHPAD = 29,
// The MCU has RWSIG task enabled
    EC_FEATURE_RWSIG = 30,
// EC has device events support
    EC_FEATURE_DEVICE_EVENT = 31,
// EC supports the unified wake masks for LPC/eSPI systems
    EC_FEATURE_UNIFIED_WAKE_MASKS = 32,
// EC supports 64-bit host events
    EC_FEATURE_HOST_EVENT64 = 33,
// EC runs code in RAM (not in place, a.k.a. XIP)
    EC_FEATURE_EXEC_IN_RAM = 34,
// EC supports CEC commands
    EC_FEATURE_CEC = 35,
// EC supports tight sensor timestamping.
    EC_FEATURE_MOTION_SENSE_TIGHT_TIMESTAMPS = 36,
//
// EC supports tablet mode detection aligned to Chrome and allows
// setting of threshold by host command using
// MOTIONSENSE_CMD_TABLET_MODE_LID_ANGLE.
//
    EC_FEATURE_REFINED_TABLET_MODE_HYSTERESIS = 37,
// The MCU is a System Companion Processor (SCP).
    EC_FEATURE_SCP = 39,
// The MCU is an Integrated Sensor Hub
    EC_FEATURE_ISH = 40,
// New TCPMv2 TYPEC_ prefaced commands supported
    EC_FEATURE_TYPEC_CMD = 41,
//
// The EC will wait for direction from the AP to enter Type-C alternate
// modes or USB4.
//
    EC_FEATURE_TYPEC_REQUIRE_AP_MODE_ENTRY = 42,
//
// The EC will wait for an acknowledge from the AP after setting the
// mux.
//
    EC_FEATURE_TYPEC_MUX_REQUIRE_AP_ACK = 43,
//
// The EC supports entering and residing in S4.
//
    EC_FEATURE_S4_RESIDENCY = 44,
//
// The EC supports the AP directing mux sets for the board.
//
    EC_FEATURE_TYPEC_AP_MUX_SET = 45,
//
// The EC supports the AP composing VDMs for us to send.
//
    EC_FEATURE_TYPEC_AP_VDM_SEND = 46,
//
// The EC supports system safe mode panic recovery.
//
    EC_FEATURE_SYSTEM_SAFE_MODE = 47,
//
// The EC will reboot on runtime assertion failures.
//
    EC_FEATURE_ASSERT_REBOOTS = 48,
//
// The EC image is built with tokenized logging enabled.
//
    EC_FEATURE_TOKENIZED_LOGGING = 49,
//
// The EC supports triggering an STB dump.
//
    EC_FEATURE_AMD_STB_DUMP = 50,
//
// The EC supports memory dump commands.
//
    EC_FEATURE_MEMORY_DUMP = 51,
//
// The EC supports DP2.1 capability
//
    EC_FEATURE_TYPEC_DP2_1 = 52,
//
// The MCU is System Companion Processor Core 1
//
    EC_FEATURE_SCP_C1 = 53,
//
// The EC supports UCSI PPM.
//
    EC_FEATURE_UCSI_PPM = 54,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_features {
    pub flags: [u32; 2],
    pub __ec_align4: },
//
// Get the board's SKU ID from EC
pub const EC_CMD_GET_SKU_ID: c_uint = 0x000E;
// Set SKU ID from AP
pub const EC_CMD_SET_SKU_ID: c_uint = 0x000F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_sku_id_info {
    pub sku_id: u32,
    pub __ec_align4: },
//
// Flash commands
// Get flash info
pub const EC_CMD_FLASH_INFO: c_uint = 0x0010;
pub const EC_VER_FLASH_INFO: c_int = 2;
//
// struct ec_response_flash_info - Response to the flash info command.
// @flash_size: Usable flash size in bytes.
// @write_block_size: Write block size. Write offset and size must be a
// multiple of this.
// @erase_block_size: Erase block size. Erase offset and size must be a
// multiple of this.
// @protect_block_size: Protection block size. Protection offset and size
// must be a multiple of this.
//
// Version 0 returns these fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_flash_info {
    pub flash_size: u32,
    pub write_block_size: u32,
    pub erase_block_size: u32,
    pub protect_block_size: u32,
    pub __ec_align4: },
//
// Flags for version 1+ flash info command
// EC flash erases bits to 0 instead of 1.
//

//
// Flash must be selected for read/write/erase operations to succeed.  This may
// be necessary on a chip where write/erase can be corrupted by other board
// activity, or where the chip needs to enable some sort of programming voltage,
// or where the read/write/erase operations require cleanly suspending other
// chip functionality.
//

//
// struct ec_response_flash_info_1 - Response to the flash info v1 command.
// @flash_size: Usable flash size in bytes.
// @write_block_size: Write block size. Write offset and size must be a
// multiple of this.
// @erase_block_size: Erase block size. Erase offset and size must be a
// multiple of this.
// @protect_block_size: Protection block size. Protection offset and size
// must be a multiple of this.
// @write_ideal_size: Ideal write size in bytes.  Writes will be fastest if
// size is exactly this and offset is a multiple of this.
// For example, an EC may have a write buffer which can do
// half-page operations if data is aligned, and a slower
// word-at-a-time write mode.
// @flags: Flags; see EC_FLASH_INFO_
//
// Version 1 returns the same initial fields as version 0, with additional
// fields following.
//
// gcc anonymous structs don't seem to get along with the __packed directive;
// if they did we'd define the version 0 structure as a sub-structure of this
// one.
//
// Version 2 supports flash banks of different sizes:
// The caller specified the number of banks it has preallocated
// (num_banks_desc)
// The EC returns the number of banks describing the flash memory.
// It adds banks descriptions up to num_banks_desc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_flash_info_1 {
// Version 0 fields; see above for description
    pub flash_size: u32,
    pub write_block_size: u32,
    pub erase_block_size: u32,
    pub protect_block_size: u32,
// Version 1 adds these fields:
    pub write_ideal_size: u32,
    pub flags: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_flash_info_2 {
// Number of banks to describe
    pub num_banks_desc: u16,
// Reserved; set 0; ignore on read
    pub reserved: [u8; 2],
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_flash_bank {
// Number of sector is in this bank.
    pub count: u16,
// Size in power of 2 of each sector (8 --> 256 bytes)
    pub size_exp: u8,
// Minimal write size for the sectors in this bank
    pub write_size_exp: u8,
// Erase size for the sectors in this bank
    pub erase_size_exp: u8,
// Size for write protection, usually identical to erase size.
    pub protect_size_exp: u8,
// Reserved; set 0; ignore on read
    pub reserved: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_flash_info_2 {
// Total flash in the EC.
    pub flash_size: u32,
// Flags; see EC_FLASH_INFO_*
    pub flags: u32,
// Maximum size to use to send data to write to the EC.
    pub write_ideal_size: u32,
// Number of banks present in the EC.
    pub num_banks_total: u16,
// Number of banks described in banks array.
    pub num_banks_desc: u16,
    pub banks: [ec_flash_bank; ],
    pub __ec_align4: },
//
// Read flash
//
// Response is params.size bytes of data.
//
pub const EC_CMD_FLASH_READ: c_uint = 0x0011;
//
// struct ec_params_flash_read - Parameters for the flash read command.
// @offset: Byte offset to read.
// @size: Size to read in bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_flash_read {
    pub offset: u32,
    pub size: u32,
    pub __ec_align4: },
// Write flash
pub const EC_CMD_FLASH_WRITE: c_uint = 0x0012;
pub const EC_VER_FLASH_WRITE: c_int = 1;
// Version 0 of the flash command supported only 64 bytes of data
pub const EC_FLASH_WRITE_VER0_SIZE: c_int = 64;
//
// struct ec_params_flash_write - Parameters for the flash write command.
// @offset: Byte offset to write.
// @size: Size to write in bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_flash_write {
    pub offset: u32,
    pub size: u32,
// Followed by data to write
    pub __ec_align4: },
// Erase flash
pub const EC_CMD_FLASH_ERASE: c_uint = 0x0013;
//
// struct ec_params_flash_erase - Parameters for the flash erase command, v0.
// @offset: Byte offset to erase.
// @size: Size to erase in bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_flash_erase {
    pub offset: u32,
    pub size: u32,
    pub __ec_align4: },
//
// v1 add async erase:
// subcommands can returns:
// EC_RES_SUCCESS : erased (see ERASE_SECTOR_ASYNC case below).
// EC_RES_INVALID_PARAM : offset/size are not aligned on a erase boundary.
// EC_RES_ERROR : other errors.
// EC_RES_BUSY : an existing erase operation is in progress.
// EC_RES_ACCESS_DENIED: Trying to erase running image.
//
// When ERASE_SECTOR_ASYNC returns EC_RES_SUCCESS, the operation is just
// properly queued. The user must call ERASE_GET_RESULT subcommand to get
// the proper result.
// When ERASE_GET_RESULT returns EC_RES_BUSY, the caller must wait and send
// ERASE_GET_RESULT again to get the result of ERASE_SECTOR_ASYNC.
// ERASE_GET_RESULT command may timeout on EC where flash access is not
// permitted while erasing. (For instance, STM32F4).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_flash_erase_cmd {
    FLASH_ERASE_SECTOR,     /* Erase and wait for result */
    FLASH_ERASE_SECTOR_ASYNC,  /* Erase and return immediately. */
    FLASH_ERASE_GET_RESULT,  /* Ask for last erase result */
}

//
// struct ec_params_flash_erase_v1 - Parameters for the flash erase command, v1.
// @cmd: One of ec_flash_erase_cmd.
// @reserved: Pad byte; currently always contains 0.
// @flag: No flags defined yet; set to 0.
// @params: Same as v0 parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_flash_erase_v1 {
    pub cmd: u8,
    pub reserved: u8,
    pub flag: u16,
    pub params: ec_params_flash_erase,
    pub __ec_align4: },
//
// Get/set flash protection.
//
// If mask!=0, sets/clear the requested bits of flags.  Depending on the
// firmware write protect GPIO, not all flags will take effect immediately;
// some flags require a subsequent hard reset to take effect.  Check the
// returned flags bits to see what actually happened.
//
// If mask=0, simply returns the current flags state.
//
pub const EC_CMD_FLASH_PROTECT: c_uint = 0x0015;

// Flags for flash protection
// RO flash code protected when the EC boots

//
// RO flash code protected now.  If this bit is set, at-boot status cannot
// be changed.
//

// Entire flash code protected now, until reboot.

// Flash write protect GPIO is asserted now

// Error - at least one bank of flash is stuck locked, and cannot be unlocked

//
// Error - flash protection is in inconsistent state.  At least one bank of
// flash which should be protected is not protected.  Usually fixed by
// re-requesting the desired flags, or by a hard reset if that fails.
//

// Entire flash code protected when the EC boots

// RW flash code protected when the EC boots

// RW flash code protected now.

// Rollback information flash region protected when the EC boots

// Rollback information flash region protected now

//
// struct ec_params_flash_protect - Parameters for the flash protect command.
// @mask: Bits in flags to apply.
// @flags: New flags to apply.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_flash_protect {
    pub mask: u32,
    pub flags: u32,
    pub __ec_align4: },
//
// struct ec_response_flash_protect - Response to the flash protect command.
// @flags: Current value of flash protect flags.
// @valid_flags: Flags which are valid on this platform. This allows the
// caller to distinguish between flags which aren't set vs. flags
// which can't be set on this platform.
// @writable_flags: Flags which can be changed given the current protection
// state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_flash_protect {
    pub flags: u32,
    pub valid_flags: u32,
    pub writable_flags: u32,
    pub __ec_align4: },
//
// Note: commands 0x14 - 0x19 version 0 were old commands to get/set flash
// write protect.  These commands may be reused with version > 0.
//
// Get the region offset/size
pub const EC_CMD_FLASH_REGION_INFO: c_uint = 0x0016;
pub const EC_VER_FLASH_REGION_INFO: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_flash_region {
// Region which holds read-only EC image
    EC_FLASH_REGION_RO = 0,
//
// Region which holds active RW image. 'Active' is different from
// 'running'. Active means 'scheduled-to-run'. Since RO image always
// scheduled to run, active/non-active applies only to RW images (for
// the same reason 'update' applies only to RW images. It's a state of
// an image on a flash. Running image can be RO, RW_A, RW_B but active
// image can only be RW_A or RW_B. In recovery mode, an active RW image
// doesn't enter 'running' state but it's still active on a flash.
//
    EC_FLASH_REGION_ACTIVE,
//
// Region which should be write-protected in the factory (a superset of
// EC_FLASH_REGION_RO)
//
    EC_FLASH_REGION_WP_RO,
// Region which holds updatable (non-active) RW image
    EC_FLASH_REGION_UPDATE,
// Number of regions
    EC_FLASH_REGION_COUNT,
}

//
// 'RW' is vague if there are multiple RW images; we mean the active one,
// so the old constant is deprecated.
//

//
// struct ec_params_flash_region_info - Parameters for the flash region info
// command.
// @region: Flash region; see EC_FLASH_REGION_
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_flash_region_info {
    pub region: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_flash_region_info {
    pub offset: u32,
    pub size: u32,
    pub __ec_align4: },
// Read/write VbNvContext
pub const EC_CMD_VBNV_CONTEXT: c_uint = 0x0017;
pub const EC_VER_VBNV_CONTEXT: c_int = 1;
pub const EC_VBNV_BLOCK_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_vbnvcontext_op {
    EC_VBNV_CONTEXT_OP_READ,
    EC_VBNV_CONTEXT_OP_WRITE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_vbnvcontext {
    pub op: u32,
    pub block: [u8; EC_VBNV_BLOCK_SIZE],
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_vbnvcontext {
    pub block: [u8; EC_VBNV_BLOCK_SIZE],
    pub __ec_align4: },
// Get SPI flash information
pub const EC_CMD_FLASH_SPI_INFO: c_uint = 0x0018;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_flash_spi_info {
// JEDEC info from command 0x9F (manufacturer, memory type, size)
    pub jedec: [u8; 3],
// Pad byte; currently always contains 0
    pub reserved0: u8,
// Manufacturer / device ID from command 0x90
    pub mfr_dev_id: [u8; 2],
// Status registers from command 0x05 and 0x35
    pub sr2: uint8_t sr1,,
    pub __ec_align1: },
// Select flash during flash operations
pub const EC_CMD_FLASH_SELECT: c_uint = 0x0019;
//
// struct ec_params_flash_select - Parameters for the flash select command.
// @select: 1 to select flash, 0 to deselect flash
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_flash_select {
    pub select: u8,
    pub __ec_align4: },
//
// PWM commands
// Get fan target RPM
pub const EC_CMD_PWM_GET_FAN_TARGET_RPM: c_uint = 0x0020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pwm_get_fan_rpm {
    pub rpm: u32,
    pub __ec_align4: },
// Set target fan RPM
pub const EC_CMD_PWM_SET_FAN_TARGET_RPM: c_uint = 0x0021;
// Version 0 of input params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pwm_set_fan_target_rpm_v0 {
    pub rpm: u32,
    pub __ec_align4: },
// Version 1 of input params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pwm_set_fan_target_rpm_v1 {
    pub rpm: u32,
    pub fan_idx: u8,
    pub __ec_align_size1: },
// Get keyboard backlight
// OBSOLETE - Use EC_CMD_PWM_SET_DUTY
pub const EC_CMD_PWM_GET_KEYBOARD_BACKLIGHT: c_uint = 0x0022;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pwm_get_keyboard_backlight {
    pub percent: u8,
    pub enabled: u8,
    pub __ec_align1: },
// Set keyboard backlight
// OBSOLETE - Use EC_CMD_PWM_SET_DUTY
pub const EC_CMD_PWM_SET_KEYBOARD_BACKLIGHT: c_uint = 0x0023;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pwm_set_keyboard_backlight {
    pub percent: u8,
    pub __ec_align1: },
// Set target fan PWM duty cycle
pub const EC_CMD_PWM_SET_FAN_DUTY: c_uint = 0x0024;
// Version 0 of input params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pwm_set_fan_duty_v0 {
    pub percent: u32,
    pub __ec_align4: },
// Version 1 of input params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pwm_set_fan_duty_v1 {
    pub percent: u32,
    pub fan_idx: u8,
    pub __ec_align_size1: },
pub const EC_CMD_PWM_SET_DUTY: c_uint = 0x0025;
// 16 bit duty cycle, 0xffff = 100%
pub const EC_PWM_MAX_DUTY: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_pwm_type {
// All types, indexed by board-specific enum pwm_channel
    EC_PWM_TYPE_GENERIC = 0,
// Keyboard backlight
    EC_PWM_TYPE_KB_LIGHT,
// Display backlight
    EC_PWM_TYPE_DISPLAY_LIGHT,
    EC_PWM_TYPE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pwm_set_duty {
    pub /: *mut *mut uint16_t duty; / Duty cycle, EC_PWM_MAX_DUTY = 100%,
    pub /: *mut *mut uint8_t pwm_type; / ec_pwm_type,
    pub /: *mut *mut uint8_t index; / Type-specific index, or 0 if unique,
    pub __ec_align4: },
pub const EC_CMD_PWM_GET_DUTY: c_uint = 0x0026;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pwm_get_duty {
    pub /: *mut *mut uint8_t pwm_type; / ec_pwm_type,
    pub /: *mut *mut uint8_t index; / Type-specific index, or 0 if unique,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pwm_get_duty {
    pub /: *mut *mut uint16_t duty; / Duty cycle, EC_PWM_MAX_DUTY = 100%,
    pub __ec_align2: },
pub const EC_CMD_PWM_GET_FAN_DUTY: c_uint = 0x0027;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pwm_get_fan_duty {
    pub fan_idx: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pwm_get_fan_duty {
    pub /: *mut *mut uint32_t percent; / Percentage of duty cycle, ranging from 0 ~ 100,
    pub __ec_align4: },
//
// Lightbar commands. This looks worse than it is. Since we only use one HOST
// command to say "talk to the lightbar", we put the "and tell it to do X" part
// into a subcommand. We'll make separate structs for subcommands with
// different input args, so that we know how much to expect.
//
pub const EC_CMD_LIGHTBAR_CMD: c_uint = 0x0028;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rgb_s {
    pub b: uint8_t r, g,,
    pub __ec_todo_unpacked: },
pub const LB_BATTERY_LEVELS: c_int = 4;
//
// List of tweakable parameters. NOTE: It's __packed so it can be sent in a
// host command, but the alignment is the same regardless. Keep it that way.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_params_v0 {
// Timing
    pub google_ramp_up: i32,
    pub google_ramp_down: i32,
    pub s3s0_ramp_up: i32,
    pub /: *mut *mut int32_t s0_tick_delay[2]; / AC=0/1,
    pub /: *mut *mut int32_t s0a_tick_delay[2]; / AC=0/1,
    pub s0s3_ramp_down: i32,
    pub s3_sleep_for: i32,
    pub s3_ramp_up: i32,
    pub s3_ramp_down: i32,
// Oscillation
    pub new_s0: u8,
    pub /: *mut *mut uint8_t osc_min[2]; / AC=0/1,
    pub /: *mut *mut uint8_t osc_max[2]; / AC=0/1,
    pub /: *mut *mut uint8_t w_ofs[2]; / AC=0/1,
// Brightness limits based on the backlight and AC.
    pub /: *mut *mut uint8_t bright_bl_off_fixed[2]; / AC=0/1,
    pub /: *mut *mut uint8_t bright_bl_on_min[2]; / AC=0/1,
    pub /: *mut *mut uint8_t bright_bl_on_max[2]; / AC=0/1,
// Battery level thresholds
    pub 1]: uint8_t battery_threshold[LB_BATTERY_LEVELS -,
// Map [AC][battery_level] to color index
    pub /: *mut *mut uint8_t s0_idx[2][LB_BATTERY_LEVELS]; / AP is running,
    pub /: *mut *mut uint8_t s3_idx[2][LB_BATTERY_LEVELS]; / AP is sleeping,
// Color palette
    pub /: *mut *mut rgb_s color[8]; / 0-3 are Google colors,
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_params_v1 {
// Timing
    pub google_ramp_up: i32,
    pub google_ramp_down: i32,
    pub s3s0_ramp_up: i32,
    pub /: *mut *mut int32_t s0_tick_delay[2]; / AC=0/1,
    pub /: *mut *mut int32_t s0a_tick_delay[2]; / AC=0/1,
    pub s0s3_ramp_down: i32,
    pub s3_sleep_for: i32,
    pub s3_ramp_up: i32,
    pub s3_ramp_down: i32,
    pub s5_ramp_up: i32,
    pub s5_ramp_down: i32,
    pub tap_tick_delay: i32,
    pub tap_gate_delay: i32,
    pub tap_display_time: i32,
// Tap-for-battery params
    pub tap_pct_red: u8,
    pub tap_pct_green: u8,
    pub tap_seg_min_on: u8,
    pub tap_seg_max_on: u8,
    pub tap_seg_osc: u8,
    pub tap_idx: [u8; 3],
// Oscillation
    pub /: *mut *mut uint8_t osc_min[2]; / AC=0/1,
    pub /: *mut *mut uint8_t osc_max[2]; / AC=0/1,
    pub /: *mut *mut uint8_t w_ofs[2]; / AC=0/1,
// Brightness limits based on the backlight and AC.
    pub /: *mut *mut uint8_t bright_bl_off_fixed[2]; / AC=0/1,
    pub /: *mut *mut uint8_t bright_bl_on_min[2]; / AC=0/1,
    pub /: *mut *mut uint8_t bright_bl_on_max[2]; / AC=0/1,
// Battery level thresholds
    pub 1]: uint8_t battery_threshold[LB_BATTERY_LEVELS -,
// Map [AC][battery_level] to color index
    pub /: *mut *mut uint8_t s0_idx[2][LB_BATTERY_LEVELS]; / AP is running,
    pub /: *mut *mut uint8_t s3_idx[2][LB_BATTERY_LEVELS]; / AP is sleeping,
// s5: single color pulse on inhibited power-up
    pub s5_idx: u8,
// Color palette
    pub /: *mut *mut rgb_s color[8]; / 0-3 are Google colors,
    pub __ec_todo_packed: },
// Lightbar command params v2
// crbug.com/467716
//
// lightbar_parms_v1 was too big for i2c, therefore in v2, we split them up by
// logical groups to make it more manageable ( < 120 bytes).
//
// NOTE: Each of these groups must be less than 120 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_params_v2_timing {
// Timing
    pub google_ramp_up: i32,
    pub google_ramp_down: i32,
    pub s3s0_ramp_up: i32,
    pub /: *mut *mut int32_t s0_tick_delay[2]; / AC=0/1,
    pub /: *mut *mut int32_t s0a_tick_delay[2]; / AC=0/1,
    pub s0s3_ramp_down: i32,
    pub s3_sleep_for: i32,
    pub s3_ramp_up: i32,
    pub s3_ramp_down: i32,
    pub s5_ramp_up: i32,
    pub s5_ramp_down: i32,
    pub tap_tick_delay: i32,
    pub tap_gate_delay: i32,
    pub tap_display_time: i32,
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_params_v2_tap {
// Tap-for-battery params
    pub tap_pct_red: u8,
    pub tap_pct_green: u8,
    pub tap_seg_min_on: u8,
    pub tap_seg_max_on: u8,
    pub tap_seg_osc: u8,
    pub tap_idx: [u8; 3],
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_params_v2_oscillation {
// Oscillation
    pub /: *mut *mut uint8_t osc_min[2]; / AC=0/1,
    pub /: *mut *mut uint8_t osc_max[2]; / AC=0/1,
    pub /: *mut *mut uint8_t w_ofs[2]; / AC=0/1,
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_params_v2_brightness {
// Brightness limits based on the backlight and AC.
    pub /: *mut *mut uint8_t bright_bl_off_fixed[2]; / AC=0/1,
    pub /: *mut *mut uint8_t bright_bl_on_min[2]; / AC=0/1,
    pub /: *mut *mut uint8_t bright_bl_on_max[2]; / AC=0/1,
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_params_v2_thresholds {
// Battery level thresholds
    pub 1]: uint8_t battery_threshold[LB_BATTERY_LEVELS -,
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_params_v2_colors {
// Map [AC][battery_level] to color index
    pub /: *mut *mut uint8_t s0_idx[2][LB_BATTERY_LEVELS]; / AP is running,
    pub /: *mut *mut uint8_t s3_idx[2][LB_BATTERY_LEVELS]; / AP is sleeping,
// s5: single color pulse on inhibited power-up
    pub s5_idx: u8,
// Color palette
    pub /: *mut *mut rgb_s color[8]; / 0-3 are Google colors,
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_params_v3 {
//
// Number of LEDs reported by the EC.
// May be less than the actual number of LEDs in the lightbar.
//
    pub reported_led_num: u8,
    pub __ec_todo_packed: },
// Lightbar program.
pub const EC_LB_PROG_LEN: c_int = 192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_program {
    pub size: u8,
    pub data: [u8; EC_LB_PROG_LEN],
    pub __ec_todo_unpacked: },
//
// Lightbar program for large sequences. Sequences are sent in pieces, with
// increasing offset. The sequences are still limited by the amount reserved in
// EC RAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lightbar_program_ex {
    pub size: u8,
    pub offset: u16,
    pub data: [u8; ],
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_lightbar {
    pub /: *mut *mut uint8_t cmd; / Command (see enum lightbar_command),
//
// The following commands have no args:
//
// dump, off, on, init, get_seq, get_params_v0, get_params_v1,
// version, get_brightness, get_demo, suspend, resume,
// get_params_v2_timing, get_params_v2_tap, get_params_v2_osc,
// get_params_v2_bright, get_params_v2_thlds,
// get_params_v2_colors
//
// Don't use an empty struct, because C++ hates that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub num: u8,
    pub demo: } set_brightness, seq,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub value: uint8_t ctrl, reg,,
    pub reg: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub blue: uint8_t led, red, green,,
    pub set_rgb: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub led: u8,
    pub get_rgb: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub enable: u8,
    pub manual_suspend_ctrl: },
    pub set_params_v0: lightbar_params_v0,
    pub set_params_v1: lightbar_params_v1,
    pub set_v2par_timing: lightbar_params_v2_timing,
    pub set_v2par_tap: lightbar_params_v2_tap,
    pub set_v2par_osc: lightbar_params_v2_oscillation,
    pub set_v2par_bright: lightbar_params_v2_brightness,
    pub set_v2par_thlds: lightbar_params_v2_thresholds,
    pub set_v2par_colors: lightbar_params_v2_colors,
    pub set_program: lightbar_program,
    pub set_program_ex: lightbar_program_ex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_lightbar {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub reg: u8,
    pub ic0: u8,
    pub ic1: u8,
    pub vals: [}; 23],
    pub dump: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub num: u8,
    pub get_demo: } get_seq, get_brightness,,
    pub get_params_v0: lightbar_params_v0,
    pub get_params_v1: lightbar_params_v1,
    pub get_params_v2_timing: lightbar_params_v2_timing,
    pub get_params_v2_tap: lightbar_params_v2_tap,
    pub get_params_v2_osc: lightbar_params_v2_oscillation,
    pub get_params_v2_bright: lightbar_params_v2_brightness,
    pub get_params_v2_thlds: lightbar_params_v2_thresholds,
    pub get_params_v2_colors: lightbar_params_v2_colors,
    pub get_params_v3: lightbar_params_v3,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub num: u32,
    pub flags: u32,
    pub version: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub blue: uint8_t red, green,,
    pub get_rgb: },
//
// The following commands have no response:
//
// off, on, init, set_brightness, seq, reg, set_rgb, demo,
// set_params_v0, set_params_v1, set_program,
// manual_suspend_ctrl, suspend, resume, set_v2par_timing,
// set_v2par_tap, set_v2par_osc, set_v2par_bright,
// set_v2par_thlds, set_v2par_colors
//
}

// Lightbar commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lightbar_command {
    LIGHTBAR_CMD_DUMP = 0,
    LIGHTBAR_CMD_OFF = 1,
    LIGHTBAR_CMD_ON = 2,
    LIGHTBAR_CMD_INIT = 3,
    LIGHTBAR_CMD_SET_BRIGHTNESS = 4,
    LIGHTBAR_CMD_SEQ = 5,
    LIGHTBAR_CMD_REG = 6,
    LIGHTBAR_CMD_SET_RGB = 7,
    LIGHTBAR_CMD_GET_SEQ = 8,
    LIGHTBAR_CMD_DEMO = 9,
    LIGHTBAR_CMD_GET_PARAMS_V0 = 10,
    LIGHTBAR_CMD_SET_PARAMS_V0 = 11,
    LIGHTBAR_CMD_VERSION = 12,
    LIGHTBAR_CMD_GET_BRIGHTNESS = 13,
    LIGHTBAR_CMD_GET_RGB = 14,
    LIGHTBAR_CMD_GET_DEMO = 15,
    LIGHTBAR_CMD_GET_PARAMS_V1 = 16,
    LIGHTBAR_CMD_SET_PARAMS_V1 = 17,
    LIGHTBAR_CMD_SET_PROGRAM = 18,
    LIGHTBAR_CMD_MANUAL_SUSPEND_CTRL = 19,
    LIGHTBAR_CMD_SUSPEND = 20,
    LIGHTBAR_CMD_RESUME = 21,
    LIGHTBAR_CMD_GET_PARAMS_V2_TIMING = 22,
    LIGHTBAR_CMD_SET_PARAMS_V2_TIMING = 23,
    LIGHTBAR_CMD_GET_PARAMS_V2_TAP = 24,
    LIGHTBAR_CMD_SET_PARAMS_V2_TAP = 25,
    LIGHTBAR_CMD_GET_PARAMS_V2_OSCILLATION = 26,
    LIGHTBAR_CMD_SET_PARAMS_V2_OSCILLATION = 27,
    LIGHTBAR_CMD_GET_PARAMS_V2_BRIGHTNESS = 28,
    LIGHTBAR_CMD_SET_PARAMS_V2_BRIGHTNESS = 29,
    LIGHTBAR_CMD_GET_PARAMS_V2_THRESHOLDS = 30,
    LIGHTBAR_CMD_SET_PARAMS_V2_THRESHOLDS = 31,
    LIGHTBAR_CMD_GET_PARAMS_V2_COLORS = 32,
    LIGHTBAR_CMD_SET_PARAMS_V2_COLORS = 33,
    LIGHTBAR_CMD_GET_PARAMS_V3 = 34,
    LIGHTBAR_CMD_SET_PROGRAM_EX = 35,
    LIGHTBAR_NUM_CMDS
}

//
// LED control commands
pub const EC_CMD_LED_CONTROL: c_uint = 0x0029;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_led_id {
// LED to indicate battery state of charge
    EC_LED_ID_BATTERY_LED = 0,
//
// LED to indicate system power state (on or in suspend).
// May be on power button or on C-panel.
//
    EC_LED_ID_POWER_LED,
// LED on power adapter or its plug
    EC_LED_ID_ADAPTER_LED,
// LED to indicate left side
    EC_LED_ID_LEFT_LED,
// LED to indicate right side
    EC_LED_ID_RIGHT_LED,
// LED to indicate recovery mode with HW_REINIT
    EC_LED_ID_RECOVERY_HW_REINIT_LED,
// LED to indicate sysrq debug mode.
    EC_LED_ID_SYSRQ_DEBUG_LED,

    EC_LED_ID_COUNT
}

// LED control flags

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_led_colors {
    EC_LED_COLOR_RED = 0,
    EC_LED_COLOR_GREEN,
    EC_LED_COLOR_BLUE,
    EC_LED_COLOR_YELLOW,
    EC_LED_COLOR_WHITE,
    EC_LED_COLOR_AMBER,

    EC_LED_COLOR_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_led_control {
    pub /: *mut *mut uint8_t led_id; / Which LED to control,
    pub /: *mut *mut uint8_t flags; / Control flags,
    pub brightness: [u8; EC_LED_COLOR_COUNT],
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_led_control {
//
// Available brightness value range.
//
// Range 0 means color channel not present.
// Range 1 means on/off control.
// Other values means the LED is control by PWM.
//
    pub brightness_range: [u8; EC_LED_COLOR_COUNT],
    pub __ec_align1: },
//
// Verified boot commands
//
// Note: command code 0x29 version 0 was VBOOT_CMD in Link EVT; it may be
// reused for other purposes with version > 0.
//
// Verified boot hash command
pub const EC_CMD_VBOOT_HASH: c_uint = 0x002A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_vboot_hash {
    pub /: *mut *mut uint8_t cmd; / enum ec_vboot_hash_cmd,
    pub /: *mut *mut uint8_t hash_type; / enum ec_vboot_hash_type,
    pub /: *mut *mut uint8_t nonce_size; / Nonce size; may be 0,
    pub /: *mut *mut uint8_t reserved0; / Reserved; set 0,
    pub /: *mut *mut uint32_t offset; / Offset in flash to hash,
    pub /: *mut *mut uint32_t size; / Number of bytes to hash,
    pub /: *mut *mut uint8_t nonce_data[64]; / Nonce data; ignored if nonce_size=0,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_vboot_hash {
    pub /: *mut *mut uint8_t status; / enum ec_vboot_hash_status,
    pub /: *mut *mut uint8_t hash_type; / enum ec_vboot_hash_type,
    pub /: *mut *mut uint8_t digest_size; / Size of hash digest in bytes,
    pub /: *mut *mut uint8_t reserved0; / Ignore; will be 0,
    pub /: *mut *mut uint32_t offset; / Offset in flash which was hashed,
    pub /: *mut *mut uint32_t size; / Number of bytes hashed,
    pub /: *mut *mut uint8_t hash_digest[64]; / Hash digest data,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_vboot_hash_cmd {
    EC_VBOOT_HASH_GET = 0,       /* Get current hash status */
    EC_VBOOT_HASH_ABORT = 1,     /* Abort calculating current hash */
    EC_VBOOT_HASH_START = 2,     /* Start computing a new hash */
    EC_VBOOT_HASH_RECALC = 3,    /* Synchronously compute a new hash */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_vboot_hash_type {
    EC_VBOOT_HASH_TYPE_SHA256 = 0, /* SHA-256 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_vboot_hash_status {
    EC_VBOOT_HASH_STATUS_NONE = 0, /* No hash (not started, or aborted) */
    EC_VBOOT_HASH_STATUS_DONE = 1, /* Finished computing a hash */
    EC_VBOOT_HASH_STATUS_BUSY = 2, /* Busy computing a hash */
}

//
// Special values for offset for EC_VBOOT_HASH_START and EC_VBOOT_HASH_RECALC.
// If one of these is specified, the EC will automatically update offset and
// size to the correct values for the specified image (RO or RW).
//
pub const EC_VBOOT_HASH_OFFSET_RO: c_uint = 0xfffffffe;
pub const EC_VBOOT_HASH_OFFSET_ACTIVE: c_uint = 0xfffffffd;
pub const EC_VBOOT_HASH_OFFSET_UPDATE: c_uint = 0xfffffffc;
//
// 'RW' is vague if there are multiple RW images; we mean the active one,
// so the old constant is deprecated.
//

//
// Motion sense commands. We'll make separate structs for sub-commands with
// different input args, so that we know how much to expect.
//
pub const EC_CMD_MOTION_SENSE_CMD: c_uint = 0x002B;
// Motion sense commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum motionsense_command {
//
// Dump command returns all motion sensor data including motion sense
// module flags and individual sensor flags.
//
    MOTIONSENSE_CMD_DUMP = 0,

//
// Info command returns data describing the details of a given sensor,
// including enum motionsensor_type, enum motionsensor_location, and
// enum motionsensor_chip.
//
    MOTIONSENSE_CMD_INFO = 1,

//
// EC Rate command is a setter/getter command for the EC sampling rate
// in milliseconds.
// It is per sensor, the EC run sample task  at the minimum of all
// sensors EC_RATE.
// For sensors without hardware FIFO, EC_RATE should be equals to 1/ODR
// to collect all the sensor samples.
// For sensor with hardware FIFO, EC_RATE is used as the maximal delay
// to process of all motion sensors in milliseconds.
//
    MOTIONSENSE_CMD_EC_RATE = 2,

//
// Sensor ODR command is a setter/getter command for the output data
// rate of a specific motion sensor in millihertz.
//
    MOTIONSENSE_CMD_SENSOR_ODR = 3,

//
// Sensor range command is a setter/getter command for the range of
// a specified motion sensor in +/-G's or +/- deg/s.
//
    MOTIONSENSE_CMD_SENSOR_RANGE = 4,

//
// Setter/getter command for the keyboard wake angle. When the lid
// angle is greater than this value, keyboard wake is disabled in S3,
// and when the lid angle goes less than this value, keyboard wake is
// enabled. Note, the lid angle measurement is an approximate,
// un-calibrated value, hence the wake angle isn't exact.
//
    MOTIONSENSE_CMD_KB_WAKE_ANGLE = 5,

//
// Returns a single sensor data.
//
    MOTIONSENSE_CMD_DATA = 6,

//
// Return sensor fifo info.
//
    MOTIONSENSE_CMD_FIFO_INFO = 7,

//
// Insert a flush element in the fifo and return sensor fifo info.
// The host can use that element to synchronize its operation.
//
    MOTIONSENSE_CMD_FIFO_FLUSH = 8,

//
// Return a portion of the fifo.
//
    MOTIONSENSE_CMD_FIFO_READ = 9,

//
// Perform low level calibration.
// On sensors that support it, ask to do offset calibration.
//
    MOTIONSENSE_CMD_PERFORM_CALIB = 10,

//
// Sensor Offset command is a setter/getter command for the offset
// used for calibration.
// The offsets can be calculated by the host, or via
// PERFORM_CALIB command.
//
    MOTIONSENSE_CMD_SENSOR_OFFSET = 11,

//
// List available activities for a MOTION sensor.
// Indicates if they are enabled or disabled.
//
    MOTIONSENSE_CMD_LIST_ACTIVITIES = 12,

//
// Activity management
// Enable/Disable activity recognition.
//
    MOTIONSENSE_CMD_SET_ACTIVITY = 13,

//
// Lid Angle
//
    MOTIONSENSE_CMD_LID_ANGLE = 14,

//
// Allow the FIFO to trigger interrupt via MKBP events.
// By default the FIFO does not send interrupt to process the FIFO
// until the AP is ready or it is coming from a wakeup sensor.
//
    MOTIONSENSE_CMD_FIFO_INT_ENABLE = 15,

//
// Spoof the readings of the sensors.  The spoofed readings can be set
// to arbitrary values, or will lock to the last read actual values.
//
    MOTIONSENSE_CMD_SPOOF = 16,

// Set lid angle for tablet mode detection.
    MOTIONSENSE_CMD_TABLET_MODE_LID_ANGLE = 17,

//
// Sensor Scale command is a setter/getter command for the calibration
// scale.
//
    MOTIONSENSE_CMD_SENSOR_SCALE = 18,

//
// Activity management
// Retrieve current status of given activity.
//
    MOTIONSENSE_CMD_GET_ACTIVITY = 20,

// Number of motionsense sub-commands.
    MOTIONSENSE_NUM_CMDS
}

// List of motion sensor types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum motionsensor_type {
    MOTIONSENSE_TYPE_ACCEL = 0,
    MOTIONSENSE_TYPE_GYRO = 1,
    MOTIONSENSE_TYPE_MAG = 2,
    MOTIONSENSE_TYPE_PROX = 3,
    MOTIONSENSE_TYPE_LIGHT = 4,
    MOTIONSENSE_TYPE_ACTIVITY = 5,
    MOTIONSENSE_TYPE_BARO = 6,
    MOTIONSENSE_TYPE_SYNC = 7,
    MOTIONSENSE_TYPE_MAX,
}

// List of motion sensor locations.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum motionsensor_location {
    MOTIONSENSE_LOC_BASE = 0,
    MOTIONSENSE_LOC_LID = 1,
    MOTIONSENSE_LOC_CAMERA = 2,
    MOTIONSENSE_LOC_MAX,
}

// List of motion sensor chips.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum motionsensor_chip {
    MOTIONSENSE_CHIP_KXCJ9 = 0,
    MOTIONSENSE_CHIP_LSM6DS0 = 1,
    MOTIONSENSE_CHIP_BMI160 = 2,
    MOTIONSENSE_CHIP_SI1141 = 3,
    MOTIONSENSE_CHIP_SI1142 = 4,
    MOTIONSENSE_CHIP_SI1143 = 5,
    MOTIONSENSE_CHIP_KX022 = 6,
    MOTIONSENSE_CHIP_L3GD20H = 7,
    MOTIONSENSE_CHIP_BMA255 = 8,
    MOTIONSENSE_CHIP_BMP280 = 9,
    MOTIONSENSE_CHIP_OPT3001 = 10,
    MOTIONSENSE_CHIP_BH1730 = 11,
    MOTIONSENSE_CHIP_GPIO = 12,
    MOTIONSENSE_CHIP_LIS2DH = 13,
    MOTIONSENSE_CHIP_LSM6DSM = 14,
    MOTIONSENSE_CHIP_LIS2DE = 15,
    MOTIONSENSE_CHIP_LIS2MDL = 16,
    MOTIONSENSE_CHIP_LSM6DS3 = 17,
    MOTIONSENSE_CHIP_LSM6DSO = 18,
    MOTIONSENSE_CHIP_LNG2DM = 19,
    MOTIONSENSE_CHIP_MAX,
}

// List of orientation positions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum motionsensor_orientation {
    MOTIONSENSE_ORIENTATION_LANDSCAPE = 0,
    MOTIONSENSE_ORIENTATION_PORTRAIT = 1,
    MOTIONSENSE_ORIENTATION_UPSIDE_DOWN_PORTRAIT = 2,
    MOTIONSENSE_ORIENTATION_UPSIDE_DOWN_LANDSCAPE = 3,
    MOTIONSENSE_ORIENTATION_UNKNOWN = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_activity_data {
    pub /: *mut *mut uint8_t activity; / motionsensor_activity,
    pub state: u8,
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_motion_sensor_data {
// Flags for each sensor.
    pub flags: u8,
// Sensor number the data comes from.
    pub sensor_num: u8,
// Each sensor is up to 3-axis.
    pub data: [i16; 3],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_packed {
    pub reserved: u16,
    pub timestamp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub activity_data: ec_response_activity_data,
    pub add_info: [i16; 2],
}

// Note: used in ec_response_get_next_data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_motion_sense_fifo_info {
// Size of the fifo
    pub size: u16,
// Amount of space used in the fifo
    pub count: u16,
// Timestamp recorded in us.
// aka accurate timestamp when host event was triggered.
//
    pub timestamp: u32,
// Total amount of vector lost
    pub total_lost: u16,
// Lost events since the last fifo_info, per sensors
    pub lost: [u16; ],
    pub __ec_todo_packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_motion_sense_fifo_data {
    pub number_data: u32,
    pub data: [ec_response_motion_sensor_data; ],
    pub __ec_todo_packed: },
// List supported activity recognition
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum motionsensor_activity {
    MOTIONSENSE_ACTIVITY_RESERVED = 0,
    MOTIONSENSE_ACTIVITY_SIG_MOTION = 1,
    MOTIONSENSE_ACTIVITY_DOUBLE_TAP = 2,
    MOTIONSENSE_ACTIVITY_ORIENTATION = 3,
    MOTIONSENSE_ACTIVITY_BODY_DETECTION = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_motion_sense_activity {
    pub sensor_num: u8,
    pub /: *mut *mut uint8_t activity; / one of enum motionsensor_activity,
    pub /: *mut *mut uint8_t enable; / 1: enable, 0: disable,
    pub reserved: u8,
    pub /: *mut *mut uint16_t parameters[3]; / activity dependent parameters,
    pub __ec_todo_unpacked: },
// Module flag masks used for the dump sub-command.

// Sensor flag masks used for the dump sub-command.

//
// Flush entry for synchronization.
// data contains time stamp
//

//
// Send this value for the data element to only perform a read. If you
// send any other value, the EC will interpret it as data to set and will
// return the actual value set.
//

pub const EC_MOTION_SENSE_INVALID_CALIB_TEMP: c_uint = 0x8000;
// MOTIONSENSE_CMD_SENSOR_OFFSET subcommand flag
// Set Calibration information

// Default Scale value, factor 1.

pub const LID_ANGLE_UNRELIABLE: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum motionsense_spoof_mode {
// Disable spoof mode.
    MOTIONSENSE_SPOOF_MODE_DISABLE = 0,

// Enable spoof mode, but use provided component values.
    MOTIONSENSE_SPOOF_MODE_CUSTOM,

// Enable spoof mode, but use the current sensor values.
    MOTIONSENSE_SPOOF_MODE_LOCK_CURRENT,

// Query the current spoof mode status for the sensor.
    MOTIONSENSE_SPOOF_MODE_QUERY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_motion_sense {
    pub cmd: u8,
// Used for MOTIONSENSE_CMD_DUMP.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
//
// Maximal number of sensor the host is expecting.
// 0 means the host is only interested in the number
// of sensors controlled by the EC.
//
    pub max_sensor_count: u8,
    pub dump: },
//
// Used for MOTIONSENSE_CMD_KB_WAKE_ANGLE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
// Data to set or EC_MOTION_SENSE_NO_VALUE to read.
// kb_wake_angle: angle to wakup AP.
//
    pub data: i16,
    pub kb_wake_angle: },
//
// Used for MOTIONSENSE_CMD_INFO, MOTIONSENSE_CMD_DATA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub sensor_num: u8,
    pub list_activities: } info, info_3, data, fifo_flush,,
//
// Used for MOTIONSENSE_CMD_PERFORM_CALIB:
// Allow entering/exiting the calibration mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub sensor_num: u8,
    pub enable: u8,
    pub perform_calib: },
//
// Used for MOTIONSENSE_CMD_EC_RATE, MOTIONSENSE_CMD_SENSOR_ODR
// and MOTIONSENSE_CMD_SENSOR_RANGE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub sensor_num: u8,
// Rounding flag, true for round-up, false for down.
    pub roundup: u8,
    pub reserved: u16,
// Data to set or EC_MOTION_SENSE_NO_VALUE to read.
    pub data: i32,
    pub sensor_range: } ec_rate, sensor_odr,,
// Used for MOTIONSENSE_CMD_SENSOR_OFFSET
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_packed {
    pub sensor_num: u8,
//
// bit 0: If set (MOTION_SENSE_SET_OFFSET), set
// the calibration information in the EC.
// If unset, just retrieve calibration information.
//
    pub flags: u16,
//
// Temperature at calibration, in units of 0.01 C
// 0x8000: invalid / unknown.
// 0x0: 0C
// 0x7fff: +327.67C
//
    pub temp: i16,
//
// Offset for calibration.
// Unit:
// Accelerometer: 1/1024 g
// Gyro:          1/1024 deg/s
// Compass:       1/16 uT
//
    pub offset: [i16; 3],
    pub sensor_offset: },
// Used for MOTIONSENSE_CMD_SENSOR_SCALE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_packed {
    pub sensor_num: u8,
//
// bit 0: If set (MOTION_SENSE_SET_OFFSET), set
// the calibration information in the EC.
// If unset, just retrieve calibration information.
//
    pub flags: u16,
//
// Temperature at calibration, in units of 0.01 C
// 0x8000: invalid / unknown.
// 0x0: 0C
// 0x7fff: +327.67C
//
    pub temp: i16,
//
// Scale for calibration:
// By default scale is 1, it is encoded on 16bits:
// 1 = BIT(15)
// ~2 = 0xFFFF
// ~0 = 0.
//
    pub scale: [u16; 3],
    pub sensor_scale: },
// Used for MOTIONSENSE_CMD_FIFO_INFO
// (no params)
// Used for MOTIONSENSE_CMD_FIFO_READ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
//
// Number of expected vector to return.
// EC may return less or 0 if none available.
//
    pub max_data_vector: u32,
    pub fifo_read: },
// Used for MOTIONSENSE_CMD_SET_ACTIVITY
    pub set_activity: ec_motion_sense_activity,
// Used for MOTIONSENSE_CMD_LID_ANGLE
// (no params)
// Used for MOTIONSENSE_CMD_FIFO_INT_ENABLE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
//
// 1: enable, 0 disable fifo,
// EC_MOTION_SENSE_NO_VALUE return value.
//
    pub enable: i8,
    pub fifo_int_enable: },
// Used for MOTIONSENSE_CMD_SPOOF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_packed {
    pub sensor_id: u8,
// See enum motionsense_spoof_mode.
    pub spoof_enable: u8,
// Ignored, used for alignment.
    pub reserved: u8,
// Individual component values to spoof.
    pub components: [i16; 3],
    pub spoof: },
// Used for MOTIONSENSE_CMD_TABLET_MODE_LID_ANGLE.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
//
// Lid angle threshold for switching between tablet and
// clamshell mode.
//
    pub lid_angle: i16,
//
// Hysteresis degree to prevent fluctuations between
// clamshell and tablet mode if lid angle keeps
// changing around the threshold. Lid motion driver will
// use lid_angle + hys_degree to trigger tablet mode and
// lid_angle - hys_degree to trigger clamshell mode.
//
    pub hys_degree: i16,
    pub tablet_mode_threshold: },
// Used for MOTIONSENSE_CMD_GET_ACTIVITY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub sensor_num: u8,
    pub /: *mut *mut uint8_t activity; / enum motionsensor_activity,
    pub get_activity: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_motion_sense {
// Used for MOTIONSENSE_CMD_DUMP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
// Flags representing the motion sensor module.
    pub module_flags: u8,
// Number of sensors managed directly by the EC.
    pub sensor_count: u8,
//
// Sensor data is truncated if response_max is too small
// for holding all the data.
//
    pub sensor): DECLARE_FLEX_ARRAY(struct ec_response_motion_sensor_data,,
    pub dump: },
// Used for MOTIONSENSE_CMD_INFO.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
// Should be element of enum motionsensor_type.
    pub type: u8,
// Should be element of enum motionsensor_location.
    pub location: u8,
// Should be element of enum motionsensor_chip.
    pub chip: u8,
    pub info: },
// Used for MOTIONSENSE_CMD_INFO version 3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
// Should be element of enum motionsensor_type.
    pub type: u8,
// Should be element of enum motionsensor_location.
    pub location: u8,
// Should be element of enum motionsensor_chip.
    pub chip: u8,
// Minimum sensor sampling frequency
    pub min_frequency: u32,
// Maximum sensor sampling frequency
    pub max_frequency: u32,
// Max number of sensor events that could be in fifo
    pub fifo_max_event_count: u32,
    pub info_3: },
// Used for MOTIONSENSE_CMD_DATA
    pub data: ec_response_motion_sensor_data,
//
// Used for MOTIONSENSE_CMD_EC_RATE, MOTIONSENSE_CMD_SENSOR_ODR,
// MOTIONSENSE_CMD_SENSOR_RANGE,
// MOTIONSENSE_CMD_KB_WAKE_ANGLE,
// MOTIONSENSE_CMD_FIFO_INT_ENABLE and
// MOTIONSENSE_CMD_SPOOF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
// Current value of the parameter queried.
    pub ret: i32,
    pub spoof: fifo_int_enable,,
//
// Used for MOTIONSENSE_CMD_SENSOR_OFFSET,
// PERFORM_CALIB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub temp: i16,
    pub offset: [i16; 3],
    pub perform_calib: } sensor_offset,,
// Used for MOTIONSENSE_CMD_SENSOR_SCALE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub temp: i16,
    pub scale: [u16; 3],
    pub sensor_scale: },
    pub fifo_flush: ec_response_motion_sense_fifo_info fifo_info,,
    pub fifo_read: ec_response_motion_sense_fifo_data,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_packed {
    pub reserved: u16,
    pub enabled: u32,
    pub disabled: u32,
    pub list_activities: },
// No params for set activity
// Used for MOTIONSENSE_CMD_LID_ANGLE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
//
// Angle between 0 and 360 degree if available,
// LID_ANGLE_UNRELIABLE otherwise.
//
    pub value: u16,
    pub lid_angle: },
// Used for MOTIONSENSE_CMD_TABLET_MODE_LID_ANGLE.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
//
// Lid angle threshold for switching between tablet and
// clamshell mode.
//
    pub lid_angle: u16,
// Hysteresis degree.
    pub hys_degree: u16,
    pub tablet_mode_threshold: },
// USED for MOTIONSENSE_CMD_GET_ACTIVITY.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub state: u8,
    pub get_activity: },
}

//
// Force lid open command
// Make lid event always open
pub const EC_CMD_FORCE_LID_OPEN: c_uint = 0x002C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_force_lid_open {
    pub enabled: u8,
    pub __ec_align1: },
//
// Configure the behavior of the power button
pub const EC_CMD_CONFIG_POWER_BUTTON: c_uint = 0x002D;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_config_power_button_flags {
// Enable/Disable power button pulses for x86 devices
    EC_POWER_BUTTON_ENABLE_PULSE = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_config_power_button {
// See enum ec_config_power_button_flags
    pub flags: u8,
    pub __ec_align1: },
//
// USB charging control commands
// Set USB port charging mode
pub const EC_CMD_USB_CHARGE_SET_MODE: c_uint = 0x0030;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_charge_set_mode {
    pub usb_port_id: u8,
    pub mode:7: u8,
    pub inhibit_charge:1: u8,
    pub __ec_align1: },
//
// Persistent storage for host
// Maximum bytes that can be read/written in a single command
pub const EC_PSTORE_SIZE_MAX: c_int = 64;
// Get persistent storage info
pub const EC_CMD_PSTORE_INFO: c_uint = 0x0040;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pstore_info {
// Persistent storage size, in bytes
    pub pstore_size: u32,
// Access size; read/write offset and size must be a multiple of this
    pub access_size: u32,
    pub __ec_align4: },
//
// Read persistent storage
//
// Response is params.size bytes of data.
//
pub const EC_CMD_PSTORE_READ: c_uint = 0x0041;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pstore_read {
    pub /: *mut *mut uint32_t offset; / Byte offset to read,
    pub /: *mut *mut uint32_t size; / Size to read in bytes,
    pub __ec_align4: },
// Write persistent storage
pub const EC_CMD_PSTORE_WRITE: c_uint = 0x0042;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pstore_write {
    pub /: *mut *mut uint32_t offset; / Byte offset to write,
    pub /: *mut *mut uint32_t size; / Size to write in bytes,
    pub data: [u8; EC_PSTORE_SIZE_MAX],
    pub __ec_align4: },
//
// Real-time clock
// RTC params and response structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_rtc {
    pub time: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_rtc {
    pub time: u32,
    pub __ec_align4: },
// These use ec_response_rtc
pub const EC_CMD_RTC_GET_VALUE: c_uint = 0x0044;
pub const EC_CMD_RTC_GET_ALARM: c_uint = 0x0045;
// These all use ec_params_rtc
pub const EC_CMD_RTC_SET_VALUE: c_uint = 0x0046;
pub const EC_CMD_RTC_SET_ALARM: c_uint = 0x0047;
// Pass as time param to SET_ALARM to clear the current alarm
pub const EC_RTC_ALARM_CLEAR: c_int = 0;
//
// Port80 log access
// Maximum entries that can be read/written in a single command
pub const EC_PORT80_SIZE_MAX: c_int = 32;
// Get last port80 code from previous boot
pub const EC_CMD_PORT80_LAST_BOOT: c_uint = 0x0048;
pub const EC_CMD_PORT80_READ: c_uint = 0x0048;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_port80_subcmd {
    EC_PORT80_GET_INFO = 0,
    EC_PORT80_READ_BUFFER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_port80_read {
    pub subcmd: u16,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub offset: u32,
    pub num_entries: u32,
    pub read_buffer: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_port80_read {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub writes: u32,
    pub history_size: u32,
    pub last_boot: u32,
    pub get_info: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub codes: [u16; EC_PORT80_SIZE_MAX],
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_port80_last_boot {
    pub code: u16,
    pub __ec_align2: },
//
// Temporary secure storage for host verified boot use
// Number of bytes in a vstore slot
pub const EC_VSTORE_SLOT_SIZE: c_int = 64;
// Maximum number of vstore slots
pub const EC_VSTORE_SLOT_MAX: c_int = 32;
// Get persistent storage info
pub const EC_CMD_VSTORE_INFO: c_uint = 0x0049;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_vstore_info {
// Indicates which slots are locked
    pub slot_locked: u32,
// Total number of slots available
    pub slot_count: u8,
    pub __ec_align_size1: },
//
// Read temporary secure storage
//
// Response is EC_VSTORE_SLOT_SIZE bytes of data.
//
pub const EC_CMD_VSTORE_READ: c_uint = 0x004A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_vstore_read {
    pub /: *mut *mut uint8_t slot; / Slot to read from,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_vstore_read {
    pub data: [u8; EC_VSTORE_SLOT_SIZE],
    pub __ec_align1: },
//
// Write temporary secure storage and lock it.
//
pub const EC_CMD_VSTORE_WRITE: c_uint = 0x004B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_vstore_write {
    pub /: *mut *mut uint8_t slot; / Slot to write to,
    pub data: [u8; EC_VSTORE_SLOT_SIZE],
    pub __ec_align1: },
//
// Thermal engine commands. Note that there are two implementations. We'll
// reuse the command number, but the data and behavior is incompatible.
// Version 0 is what originally shipped on Link.
// Version 1 separates the CPU thermal limits from the fan control.
//
pub const EC_CMD_THERMAL_SET_THRESHOLD: c_uint = 0x0050;
pub const EC_CMD_THERMAL_GET_THRESHOLD: c_uint = 0x0051;
// The version 0 structs are opaque. You have to know what they are for
// the get/set commands to make any sense.
//
// Version 0 - set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_thermal_set_threshold {
    pub sensor_type: u8,
    pub threshold_id: u8,
    pub value: u16,
    pub __ec_align2: },
// Version 0 - get
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_thermal_get_threshold {
    pub sensor_type: u8,
    pub threshold_id: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_thermal_get_threshold {
    pub value: u16,
    pub __ec_align2: },
// The version 1 structs are visible.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_temp_thresholds {
    EC_TEMP_THRESH_WARN = 0,
    EC_TEMP_THRESH_HIGH,
    EC_TEMP_THRESH_HALT,

    EC_TEMP_THRESH_COUNT
}

//
// Thermal configuration for one temperature sensor. Temps are in degrees K.
// Zero values will be silently ignored by the thermal task.
//
// Set 'temp_host' value allows thermal task to trigger some event with 1 degree
// hysteresis.
// For example,
// temp_host[EC_TEMP_THRESH_HIGH] = 300 K
// temp_host_release[EC_TEMP_THRESH_HIGH] = 0 K
// EC will throttle ap when temperature >= 301 K, and release throttling when
// temperature <= 299 K.
//
// Set 'temp_host_release' value allows thermal task has a custom hysteresis.
// For example,
// temp_host[EC_TEMP_THRESH_HIGH] = 300 K
// temp_host_release[EC_TEMP_THRESH_HIGH] = 295 K
// EC will throttle ap when temperature >= 301 K, and release throttling when
// temperature <= 294 K.
//
// Note that this structure is a sub-structure of
// ec_params_thermal_set_threshold_v1, but maintains its alignment there.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_thermal_config {
    pub /: *mut *mut uint32_t temp_host[EC_TEMP_THRESH_COUNT]; / levels of hotness,
    pub /: *mut *mut uint32_t temp_host_release[EC_TEMP_THRESH_COUNT]; / release levels,
    pub /: *mut *mut uint32_t temp_fan_off; / no active cooling needed,
    pub /: *mut *mut uint32_t temp_fan_max; / max active cooling needed,
    pub __ec_align4: },
// Version 1 - get config for one sensor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_thermal_get_threshold_v1 {
    pub sensor_num: u32,
    pub __ec_align4: },
// This returns a struct ec_thermal_config
//
// Version 1 - set config for one sensor.
// Use read-modify-write for best results!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_thermal_set_threshold_v1 {
    pub sensor_num: u32,
    pub cfg: ec_thermal_config,
    pub __ec_align4: },
// This returns no data
//
// Set or get fan control mode
pub const EC_CMD_THERMAL_AUTO_FAN_CTRL: c_uint = 0x0052;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_auto_fan_ctrl_cmd {
    EC_AUTO_FAN_CONTROL_CMD_SET = 0,
    EC_AUTO_FAN_CONTROL_CMD_GET,
}

// Version 1 of input params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_auto_fan_ctrl_v1 {
    pub fan_idx: u8,
    pub __ec_align1: },
// Version 2 of input params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_auto_fan_ctrl_v2 {
    pub fan_idx: u8,
    pub /: *mut *mut uint8_t cmd; / enum ec_auto_fan_ctrl_cmd,
    pub bool: *mut *mut uint8_t set_auto; / only used with EC_AUTO_FAN_CONTROL_CMD_SET -,
//
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_auto_fan_control {
    pub /: *mut *mut uint8_t is_auto; / bool,
    pub __ec_align1: },
// Get/Set TMP006 calibration data
pub const EC_CMD_TMP006_GET_CALIBRATION: c_uint = 0x0053;
pub const EC_CMD_TMP006_SET_CALIBRATION: c_uint = 0x0054;
//
// The original TMP006 calibration only needed four params, but now we need
// more. Since the algorithm is nothing but magic numbers anyway, we'll leave
// the params opaque. The v1 "get" response will include the algorithm number
// and how many params it requires. That way we can change the EC code without
// needing to update this file. We can also use a different algorithm on each
// sensor.
//
// This is the same struct for both v0 and v1.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_tmp006_get_calibration {
    pub index: u8,
    pub __ec_align1: },
// Version 0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_tmp006_get_calibration_v0 {
    pub s0: float,
    pub b0: float,
    pub b1: float,
    pub b2: float,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_tmp006_set_calibration_v0 {
    pub index: u8,
    pub reserved: [u8; 3],
    pub s0: float,
    pub b0: float,
    pub b1: float,
    pub b2: float,
    pub __ec_align4: },
// Version 1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_tmp006_get_calibration_v1 {
    pub algorithm: u8,
    pub num_params: u8,
    pub reserved: [u8; 2],
    pub val: [float; ],
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_tmp006_set_calibration_v1 {
    pub index: u8,
    pub algorithm: u8,
    pub num_params: u8,
    pub reserved: u8,
    pub val: [float; ],
    pub __ec_align4: },
// Read raw TMP006 data
pub const EC_CMD_TMP006_GET_RAW: c_uint = 0x0055;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_tmp006_get_raw {
    pub index: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_tmp006_get_raw {
    pub /: *mut *mut int32_t t; / In 1/100 K,
    pub /: *mut *mut int32_t v; / In nV,
    pub __ec_align4: },
//
// MKBP - Matrix KeyBoard Protocol
//
// Read key state
//
// Returns raw data for keyboard cols; see ec_response_mkbp_info.cols for
// expected response size.
//
// NOTE: This has been superseded by EC_CMD_MKBP_GET_NEXT_EVENT.  If you wish
// to obtain the instantaneous state, use EC_CMD_MKBP_INFO with the type
// EC_MKBP_INFO_CURRENT and event EC_MKBP_EVENT_KEY_MATRIX.
//
pub const EC_CMD_MKBP_STATE: c_uint = 0x0060;
//
// Provide information about various MKBP things.  See enum ec_mkbp_info_type.
//
pub const EC_CMD_MKBP_INFO: c_uint = 0x0061;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_mkbp_info {
    pub rows: u32,
    pub cols: u32,
// Formerly "switches", which was 0.
    pub reserved: u8,
    pub __ec_align_size1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_mkbp_info {
    pub info_type: u8,
    pub event_type: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_mkbp_info_type {
//
// Info about the keyboard matrix: number of rows and columns.
//
// Returns struct ec_response_mkbp_info.
//
    EC_MKBP_INFO_KBD = 0,

//
// For buttons and switches, info about which specifically are
// supported.  event_type must be set to one of the values in enum
// ec_mkbp_event.
//
// For EC_MKBP_EVENT_BUTTON and EC_MKBP_EVENT_SWITCH, returns a 4 byte
// bitmask indicating which buttons or switches are present.  See the
// bit inidices below.
//
    EC_MKBP_INFO_SUPPORTED = 1,

//
// Instantaneous state of buttons and switches.
//
// event_type must be set to one of the values in enum ec_mkbp_event.
//
// For EC_MKBP_EVENT_KEY_MATRIX, returns uint8_t key_matrix[13]
// indicating the current state of the keyboard matrix.
//
// For EC_MKBP_EVENT_HOST_EVENT, return uint32_t host_event, the raw
// event state.
//
// For EC_MKBP_EVENT_BUTTON, returns uint32_t buttons, indicating the
// state of supported buttons.
//
// For EC_MKBP_EVENT_SWITCH, returns uint32_t switches, indicating the
// state of supported switches.
//
    EC_MKBP_INFO_CURRENT = 2,
}

// Simulate key press
pub const EC_CMD_MKBP_SIMULATE_KEY: c_uint = 0x0062;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_mkbp_simulate_key {
    pub col: u8,
    pub row: u8,
    pub pressed: u8,
    pub __ec_align1: },
pub const EC_CMD_GET_KEYBOARD_ID: c_uint = 0x0063;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_keyboard_id {
    pub keyboard_id: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum keyboard_id {
    KEYBOARD_ID_UNSUPPORTED = 0,
    KEYBOARD_ID_UNREADABLE = 0xffffffff,
}

// Configure keyboard scanning
pub const EC_CMD_MKBP_SET_CONFIG: c_uint = 0x0064;
pub const EC_CMD_MKBP_GET_CONFIG: c_uint = 0x0065;
// flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mkbp_config_flags {
    EC_MKBP_FLAGS_ENABLE = 1,	/* Enable keyboard scanning */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mkbp_config_valid {
    EC_MKBP_VALID_SCAN_PERIOD		= BIT(0),
    EC_MKBP_VALID_POLL_TIMEOUT		= BIT(1),
    EC_MKBP_VALID_MIN_POST_SCAN_DELAY	= BIT(3),
    EC_MKBP_VALID_OUTPUT_SETTLE		= BIT(4),
    EC_MKBP_VALID_DEBOUNCE_DOWN		= BIT(5),
    EC_MKBP_VALID_DEBOUNCE_UP		= BIT(6),
    EC_MKBP_VALID_FIFO_MAX_DEPTH		= BIT(7),
}

//
// Configuration for our key scanning algorithm.
//
// Note that this is used as a sub-structure of
// ec_{params/response}_mkbp_get_config.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_mkbp_config {
    pub /: *mut *mut uint32_t valid_mask; / valid fields,
    pub /: *mut *mut uint8_t flags; / some flags (enum mkbp_config_flags),
    pub /: *mut *mut uint8_t valid_flags; / which flags are valid,
    pub /: *mut *mut uint16_t scan_period_us; / period between start of scans,
// revert to interrupt mode after no activity for this long
    pub poll_timeout_us: u32,
//
// minimum post-scan relax time. Once we finish a scan we check
// the time until we are due to start the next one. If this time is
// shorter this field, we use this instead.
//
    pub min_post_scan_delay_us: u16,
// delay between setting up output and waiting for it to settle
    pub output_settle_us: u16,
    pub /: *mut *mut uint16_t debounce_down_us; / time for debounce on key down,
    pub /: *mut *mut uint16_t debounce_up_us; / time for debounce on key up,
// maximum depth to allow for fifo (0 = no keyscan output)
    pub fifo_max_depth: u8,
    pub __ec_align_size1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_mkbp_set_config {
    pub config: ec_mkbp_config,
    pub __ec_align_size1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_mkbp_get_config {
    pub config: ec_mkbp_config,
    pub __ec_align_size1: },
// Run the key scan emulation
pub const EC_CMD_KEYSCAN_SEQ_CTRL: c_uint = 0x0066;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_keyscan_seq_cmd {
    EC_KEYSCAN_SEQ_STATUS = 0,	/* Get status information */
    EC_KEYSCAN_SEQ_CLEAR = 1,	/* Clear sequence */
    EC_KEYSCAN_SEQ_ADD = 2,		/* Add item to sequence */
    EC_KEYSCAN_SEQ_START = 3,	/* Start running sequence */
    EC_KEYSCAN_SEQ_COLLECT = 4,	/* Collect sequence summary data */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_collect_flags {
//
// Indicates this scan was processed by the EC. Due to timing, some
// scans may be skipped.
//
    EC_KEYSCAN_SEQ_FLAG_DONE	= BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_collect_item {
    pub /: *mut *mut uint8_t flags; / some flags (enum ec_collect_flags),
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_keyscan_seq_ctrl {
    pub /: *mut *mut uint8_t cmd; / Command to send (enum ec_keyscan_seq_cmd),
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align1 {
    pub /: *mut *mut uint8_t active; / still active,
    pub /: *mut *mut uint8_t num_items; / number of items,
// Current item being presented
    pub cur_item: u8,
    pub status: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
//
// Absolute time for this scan, measured from the
// start of the sequence.
//
    pub time_us: u32,
    pub /: *mut *mut uint8_t scan[0]; / keyscan data,
    pub add: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align1 {
    pub /: *mut *mut uint8_t start_item; / First item to return,
    pub /: *mut *mut uint8_t num_items; / Number of items to return,
    pub collect: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_result_keyscan_seq_ctrl {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub /: *mut *mut uint8_t num_items; / Number of items,
// Data for each item
    pub item: [ec_collect_item; 0],
    pub collect: },
}

//
// Get the next pending MKBP event.
//
// Returns EC_RES_UNAVAILABLE if there is no event pending.
//
pub const EC_CMD_GET_NEXT_EVENT: c_uint = 0x0067;
pub const EC_MKBP_HAS_MORE_EVENTS_SHIFT: c_int = 7;
//
// We use the most significant bit of the event type to indicate to the host
// that the EC has more MKBP events available to provide.
//

// The mask to apply to get the raw event type

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_mkbp_event {
// Keyboard matrix changed. The event data is the new matrix state.
    EC_MKBP_EVENT_KEY_MATRIX = 0,

// New host event. The event data is 4 bytes of host event flags.
    EC_MKBP_EVENT_HOST_EVENT = 1,

// New Sensor FIFO data. The event data is fifo_info structure.
    EC_MKBP_EVENT_SENSOR_FIFO = 2,

// The state of the non-matrixed buttons have changed.
    EC_MKBP_EVENT_BUTTON = 3,

// The state of the switches have changed.
    EC_MKBP_EVENT_SWITCH = 4,

// New Fingerprint sensor event, the event data is fp_events bitmap.
    EC_MKBP_EVENT_FINGERPRINT = 5,

//
// Sysrq event: send emulated sysrq. The event data is sysrq,
// corresponding to the key to be pressed.
//
    EC_MKBP_EVENT_SYSRQ = 6,

//
// New 64-bit host event.
// The event data is 8 bytes of host event flags.
//
    EC_MKBP_EVENT_HOST_EVENT64 = 7,

// Notify the AP that something happened on CEC
    EC_MKBP_EVENT_CEC_EVENT = 8,

// Send an incoming CEC message to the AP
    EC_MKBP_EVENT_CEC_MESSAGE = 9,

// Peripheral device charger event
    EC_MKBP_EVENT_PCHG = 12,

// Number of MKBP events
    EC_MKBP_EVENT_COUNT,
}

// Unaligned
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
// For aligning the fifo_info
    pub reserved: [u8; 3],
    pub info: ec_response_motion_sense_fifo_info,
    pub sensor_fifo: },
    pub buttons: u32,
    pub switches: u32,
    pub fp_events: u32,
    pub sysrq: u32,
// CEC events from enum mkbp_cec_event
    pub cec_events: u32,
}

// Unaligned
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
// For aligning the fifo_info
    pub reserved: [u8; 3],
    pub info: ec_response_motion_sense_fifo_info,
    pub sensor_fifo: },
    pub buttons: u32,
    pub switches: u32,
    pub fp_events: u32,
    pub sysrq: u32,
// CEC events from enum mkbp_cec_event
    pub cec_events: u32,
    pub cec_message: [u8; 16],
}

// Unaligned
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
// For aligning the fifo_info
    pub reserved: [u8; 3],
    pub info: ec_response_motion_sense_fifo_info,
    pub sensor_fifo: },
    pub buttons: u32,
    pub switches: u32,
    pub fp_events: u32,
    pub sysrq: u32,
// CEC events from enum mkbp_cec_event
    pub cec_events: u32,
    pub cec_message: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_next_event {
    pub event_type: u8,
// Followed by event data if any
    pub data: ec_response_get_next_data,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_next_event_v1 {
    pub event_type: u8,
// Followed by event data if any
    pub data: ec_response_get_next_data_v1,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_get_next_event_v3 {
    pub event_type: u8,
// Followed by event data if any
    pub data: ec_response_get_next_data_v3,
    pub __ec_align1: },
// Bit indices for buttons and switches.
// Buttons
pub const EC_MKBP_POWER_BUTTON: c_int = 0;
pub const EC_MKBP_VOL_UP: c_int = 1;
pub const EC_MKBP_VOL_DOWN: c_int = 2;
pub const EC_MKBP_RECOVERY: c_int = 3;
pub const EC_MKBP_BRI_UP: c_int = 4;
pub const EC_MKBP_BRI_DOWN: c_int = 5;
pub const EC_MKBP_SCREEN_LOCK: c_int = 6;
// Switches
pub const EC_MKBP_LID_OPEN: c_int = 0;
pub const EC_MKBP_TABLET_MODE: c_int = 1;
pub const EC_MKBP_BASE_ATTACHED: c_int = 2;
pub const EC_MKBP_FRONT_PROXIMITY: c_int = 3;
// Run keyboard factory test scanning
pub const EC_CMD_KEYBOARD_FACTORY_TEST: c_uint = 0x0068;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_keyboard_factory_test {
    pub /: *mut *mut uint16_t shorted; / Keyboard pins are shorted,
    pub __ec_align2: },
// Fingerprint events in 'fp_events' for EC_MKBP_EVENT_FINGERPRINT

pub const EC_MKBP_FP_ENROLL_PROGRESS_OFFSET: c_int = 4;

pub const EC_MKBP_FP_MATCH_IDX_OFFSET: c_int = 12;
pub const EC_MKBP_FP_MATCH_IDX_MASK: c_uint = 0x0000F000;

// code given by EC_MKBP_FP_ERRCODE() when EC_MKBP_FP_ENROLL is set
pub const EC_MKBP_FP_ERR_ENROLL_OK: c_int = 0;
pub const EC_MKBP_FP_ERR_ENROLL_LOW_QUALITY: c_int = 1;
pub const EC_MKBP_FP_ERR_ENROLL_IMMOBILE: c_int = 2;
pub const EC_MKBP_FP_ERR_ENROLL_LOW_COVERAGE: c_int = 3;
pub const EC_MKBP_FP_ERR_ENROLL_INTERNAL: c_int = 5;
// Can be used to detect if image was usable for enrollment or not.
pub const EC_MKBP_FP_ERR_ENROLL_PROBLEM_MASK: c_int = 1;
// code given by EC_MKBP_FP_ERRCODE() when EC_MKBP_FP_MATCH is set
pub const EC_MKBP_FP_ERR_MATCH_NO: c_int = 0;
pub const EC_MKBP_FP_ERR_MATCH_NO_INTERNAL: c_int = 6;
pub const EC_MKBP_FP_ERR_MATCH_NO_TEMPLATES: c_int = 7;
pub const EC_MKBP_FP_ERR_MATCH_NO_LOW_QUALITY: c_int = 2;
pub const EC_MKBP_FP_ERR_MATCH_NO_LOW_COVERAGE: c_int = 4;
pub const EC_MKBP_FP_ERR_MATCH_YES: c_int = 1;
pub const EC_MKBP_FP_ERR_MATCH_YES_UPDATED: c_int = 3;
pub const EC_MKBP_FP_ERR_MATCH_YES_UPDATE_FAILED: c_int = 5;
//
// Temperature sensor commands
// Read temperature sensor info
pub const EC_CMD_TEMP_SENSOR_GET_INFO: c_uint = 0x0070;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_temp_sensor_get_info {
    pub id: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_temp_sensor_get_info {
    pub sensor_name: [c_char; 32],
    pub sensor_type: u8,
    pub __ec_align1: },
//
// Note: host commands 0x80 - 0x87 are reserved to avoid conflict with ACPI
// commands accidentally sent to the wrong interface.  See the ACPI section
// below.
//
// Host event commands
// Obsolete. New implementation should use EC_CMD_HOST_EVENT instead
//
// Host event mask params and response structures, shared by all of the host
// event commands below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_host_event_mask {
    pub mask: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_host_event_mask {
    pub mask: u32,
    pub __ec_align4: },
// These all use ec_response_host_event_mask
pub const EC_CMD_HOST_EVENT_GET_B: c_uint = 0x0087;
pub const EC_CMD_HOST_EVENT_GET_SMI_MASK: c_uint = 0x0088;
pub const EC_CMD_HOST_EVENT_GET_SCI_MASK: c_uint = 0x0089;
pub const EC_CMD_HOST_EVENT_GET_WAKE_MASK: c_uint = 0x008D;
// These all use ec_params_host_event_mask
pub const EC_CMD_HOST_EVENT_SET_SMI_MASK: c_uint = 0x008A;
pub const EC_CMD_HOST_EVENT_SET_SCI_MASK: c_uint = 0x008B;
pub const EC_CMD_HOST_EVENT_CLEAR: c_uint = 0x008C;
pub const EC_CMD_HOST_EVENT_SET_WAKE_MASK: c_uint = 0x008E;
pub const EC_CMD_HOST_EVENT_CLEAR_B: c_uint = 0x008F;
//
// Unified host event programming interface - Should be used by newer versions
// of BIOS/OS to program host events and masks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_host_event {
// Action requested by host - one of enum ec_host_event_action.
    pub action: u8,
//
// Mask type that the host requested the action on - one of
// enum ec_host_event_mask_type.
//
    pub mask_type: u8,
// Set to 0, ignore on read
    pub reserved: u16,
// Value to be used in case of set operations.
    pub value: u64,
    pub __ec_align4: },
//
// Response structure returned by EC_CMD_HOST_EVENT.
// Update the value on a GET request. Set to 0 on GET/CLEAR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_host_event {
// Mask value in case of get operation
    pub value: u64,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_host_event_action {
//
// params.value is ignored. Value of mask_type populated
// in response.value
//
    EC_HOST_EVENT_GET,

// Bits in params.value are set
    EC_HOST_EVENT_SET,

// Bits in params.value are cleared
    EC_HOST_EVENT_CLEAR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_host_event_mask_type {

// Main host event copy
    EC_HOST_EVENT_MAIN,

// Copy B of host events
    EC_HOST_EVENT_B,

// SCI Mask
    EC_HOST_EVENT_SCI_MASK,

// SMI Mask
    EC_HOST_EVENT_SMI_MASK,

// Mask of events that should be always reported in hostevents
    EC_HOST_EVENT_ALWAYS_REPORT_MASK,

// Active wake mask
    EC_HOST_EVENT_ACTIVE_WAKE_MASK,

// Lazy wake mask for S0ix
    EC_HOST_EVENT_LAZY_WAKE_MASK_S0IX,

// Lazy wake mask for S3
    EC_HOST_EVENT_LAZY_WAKE_MASK_S3,

// Lazy wake mask for S5
    EC_HOST_EVENT_LAZY_WAKE_MASK_S5,
}

pub const EC_CMD_HOST_EVENT: c_uint = 0x00A4;
//
// Switch commands
// Enable/disable LCD backlight
pub const EC_CMD_SWITCH_ENABLE_BKLIGHT: c_uint = 0x0090;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_switch_enable_backlight {
    pub enabled: u8,
    pub __ec_align1: },
// Enable/disable WLAN/Bluetooth
pub const EC_CMD_SWITCH_ENABLE_WIRELESS: c_uint = 0x0091;
pub const EC_VER_SWITCH_ENABLE_WIRELESS: c_int = 1;
// Version 0 params; no response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_switch_enable_wireless_v0 {
    pub enabled: u8,
    pub __ec_align1: },
// Version 1 params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_switch_enable_wireless_v1 {
// Flags to enable now
    pub now_flags: u8,
// Which flags to copy from now_flags
    pub now_mask: u8,
//
// Flags to leave enabled in S3, if they're on at the S0->S3
// transition.  (Other flags will be disabled by the S0->S3
// transition.)
//
    pub suspend_flags: u8,
// Which flags to copy from suspend_flags
    pub suspend_mask: u8,
    pub __ec_align1: },
// Version 1 response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_switch_enable_wireless_v1 {
// Flags to enable now
    pub now_flags: u8,
// Flags to leave enabled in S3
    pub suspend_flags: u8,
    pub __ec_align1: },
//
// GPIO commands. Only available on EC if write protect has been disabled.
// Set GPIO output value
pub const EC_CMD_GPIO_SET: c_uint = 0x0092;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_gpio_set {
    pub name: [c_char; 32],
    pub val: u8,
    pub __ec_align1: },
// Get GPIO value
pub const EC_CMD_GPIO_GET: c_uint = 0x0093;
// Version 0 of input params and response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_gpio_get {
    pub name: [c_char; 32],
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_gpio_get {
    pub val: u8,
    pub __ec_align1: },
// Version 1 of input params and response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_gpio_get_v1 {
    pub subcmd: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align1 {
    pub name: [c_char; 32],
    pub get_value_by_name: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align1 {
    pub index: u8,
    pub get_info: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_gpio_get_v1 {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align1 {
    pub val: u8,
    pub get_count: } get_value_by_name,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub val: u8,
    pub name: [c_char; 32],
    pub flags: u32,
    pub get_info: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_get_subcmd {
    EC_GPIO_GET_BY_NAME = 0,
    EC_GPIO_GET_COUNT = 1,
    EC_GPIO_GET_INFO = 2,
}

//
// I2C commands. Only available when flash write protect is unlocked.
//
// CAUTION: These commands are deprecated, and are not supported anymore in EC
// builds >= 8398.0.0 (see crosbug.com/p/23570).
//
// Use EC_CMD_I2C_PASSTHRU instead.
//
// Read I2C bus
pub const EC_CMD_I2C_READ: c_uint = 0x0094;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_i2c_read {
    pub /: *mut *mut uint16_t addr; / 8-bit address (7-bit shifted << 1),
    pub /: *mut *mut uint8_t read_size; / Either 8 or 16.,
    pub port: u8,
    pub offset: u8,
    pub __ec_align_size1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_i2c_read {
    pub data: u16,
    pub __ec_align2: },
// Write I2C bus
pub const EC_CMD_I2C_WRITE: c_uint = 0x0095;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_i2c_write {
    pub data: u16,
    pub /: *mut *mut uint16_t addr; / 8-bit address (7-bit shifted << 1),
    pub /: *mut *mut uint8_t write_size; / Either 8 or 16.,
    pub port: u8,
    pub offset: u8,
    pub __ec_align_size1: },
//
// Charge state commands. Only available when flash write protect unlocked.
// Force charge state machine to stop charging the battery or force it to
// discharge the battery.
//
pub const EC_CMD_CHARGE_CONTROL: c_uint = 0x0096;
pub const EC_VER_CHARGE_CONTROL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_charge_control_mode {
    CHARGE_CONTROL_NORMAL = 0,
    CHARGE_CONTROL_IDLE,
    CHARGE_CONTROL_DISCHARGE,
// Add no more entry below.
    CHARGE_CONTROL_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_charge_control_cmd {
    EC_CHARGE_CONTROL_CMD_SET = 0,
    EC_CHARGE_CONTROL_CMD_GET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_charge_control_flag {
    EC_CHARGE_CONTROL_FLAG_NO_IDLE = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_charge_control {
    pub /: *mut *mut uint32_t mode; / enum charge_control_mode,
// Below are the fields added in V2.
    pub /: *mut *mut uint8_t cmd; / enum ec_charge_control_cmd.,
    pub /: *mut *mut uint8_t flags; / enum ec_charge_control_flag (v3+),
//
// Lower and upper thresholds for battery sustainer. This struct isn't
// named to avoid tainting foreign projects' name spaces.
//
// If charge mode is explicitly set (e.g. DISCHARGE), battery sustainer
// will be disabled. To disable battery sustainer, set mode=NORMAL,
// lower=-1, upper=-1.
//
    pub /: *mut *mut int8_t lower; / Display SoC in percentage.,
    pub /: *mut *mut int8_t upper; / Display SoC in percentage.,
    pub sustain_soc: },
    pub __ec_align4: },
// Added in v2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_charge_control {
    pub /: *mut *mut uint32_t mode; / enum charge_control_mode,
    pub lower: i8,
    pub upper: i8,
    pub sustain_soc: },
    pub /: *mut *mut uint8_t flags; / enum ec_charge_control_flag (v3+),
    pub reserved: u8,
    pub __ec_align4: },
//
// Snapshot console output buffer for use by EC_CMD_CONSOLE_READ.
pub const EC_CMD_CONSOLE_SNAPSHOT: c_uint = 0x0097;
//
// Read data from the saved snapshot. If the subcmd parameter is
// CONSOLE_READ_NEXT, this will return data starting from the beginning of
// the latest snapshot. If it is CONSOLE_READ_RECENT, it will start from the
// end of the previous snapshot.
//
// The params are only looked at in version >= 1 of this command. Prior
// versions will just default to CONSOLE_READ_NEXT behavior.
//
// Response is null-terminated string.  Empty string, if there is no more
// remaining output.
//
pub const EC_CMD_CONSOLE_READ: c_uint = 0x0098;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_console_read_subcmd {
    CONSOLE_READ_NEXT = 0,
    CONSOLE_READ_RECENT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_console_read_v1 {
    pub /: *mut *mut uint8_t subcmd; / enum ec_console_read_subcmd,
    pub __ec_align1: },
//
// Cut off battery power immediately or after the host has shut down.
//
// return EC_RES_INVALID_COMMAND if unsupported by a board/battery.
// EC_RES_SUCCESS if the command was successful.
// EC_RES_ERROR if the cut off command failed.
//
pub const EC_CMD_BATTERY_CUT_OFF: c_uint = 0x0099;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_battery_cutoff {
    pub flags: u8,
    pub __ec_align1: },
//
// USB port mux control.
//
// Switch USB mux or return to automatic switching.
//
pub const EC_CMD_USB_MUX: c_uint = 0x009A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_mux {
    pub mux: u8,
    pub __ec_align1: },
//
// LDOs / FETs control.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_ldo_state {
    EC_LDO_STATE_OFF = 0,	/* the LDO / FET is shut down */
    EC_LDO_STATE_ON = 1,	/* the LDO / FET is ON / providing power */
}

//
// Switch on/off a LDO.
//
pub const EC_CMD_LDO_SET: c_uint = 0x009B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_ldo_set {
    pub index: u8,
    pub state: u8,
    pub __ec_align1: },
//
// Get LDO state.
//
pub const EC_CMD_LDO_GET: c_uint = 0x009C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_ldo_get {
    pub index: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_ldo_get {
    pub state: u8,
    pub __ec_align1: },
//
// Power info.
//
// Get power info.
//
pub const EC_CMD_POWER_INFO: c_uint = 0x009D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_power_info {
    pub usb_dev_type: u32,
    pub voltage_ac: u16,
    pub voltage_system: u16,
    pub current_system: u16,
    pub usb_current_limit: u16,
    pub __ec_align4: },
//
// I2C passthru command
pub const EC_CMD_I2C_PASSTHRU: c_uint = 0x009E;
// Read data; if not present, message is a write

// Mask for address
pub const EC_I2C_ADDR_MASK: c_uint = 0x3ff;

// Any error

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_i2c_passthru_msg {
    pub /: *mut *mut uint16_t addr_flags; / I2C slave address (7 or 10 bits) and flags,
    pub /: *mut *mut uint16_t len; / Number of bytes to read or write,
    pub __ec_align2: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_i2c_passthru {
    pub /: *mut *mut uint8_t port; / I2C port number,
    pub /: *mut *mut uint8_t num_msgs; / Number of messages,
    pub msg: [ec_params_i2c_passthru_msg; ],
// Data to write for all messages is concatenated here
    pub __ec_align2: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_i2c_passthru {
    pub /: *mut *mut uint8_t i2c_status; / Status flags (EC_I2C_STATUS_...),
    pub /: *mut *mut uint8_t num_msgs; / Number of messages processed,
    pub /: *mut *mut uint8_t data[]; / Data read by messages concatenated here,
    pub __ec_align1: },
//
// AP hang detect
pub const EC_CMD_HANG_DETECT: c_uint = 0x009F;
pub const EC_HANG_DETECT_MIN_TIMEOUT: c_int = 5;
pub const EC_HANG_DETECT_MAX_TIMEOUT: c_int = 65535;
// EC hang detect commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_hang_detect_cmds {
// Reload AP hang detect timer.
    EC_HANG_DETECT_CMD_RELOAD = 0x0,

// Stop AP hang detect timer.
    EC_HANG_DETECT_CMD_CANCEL = 0x1,

// Configure watchdog with given reboot timeout and
// cancel currently running AP hang detect timer.
//
    EC_HANG_DETECT_CMD_SET_TIMEOUT = 0x2,

// Get last hang status - whether the AP boot was clear or not
    EC_HANG_DETECT_CMD_GET_STATUS = 0x3,

// Clear last hang status. Called when AP is rebooting/shutting down
// gracefully.
//
    EC_HANG_DETECT_CMD_CLEAR_STATUS = 0x4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_hang_detect {
    pub /: *mut *mut uint16_t command; / enum ec_hang_detect_cmds,
// Timeout in seconds before generating reboot
    pub reboot_timeout_sec: u16,
    pub __ec_align2: },
// Status codes that describe whether AP has boot normally or the hang has been
// detected and EC has reset AP
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_hang_detect_status {
    EC_HANG_DETECT_AP_BOOT_NORMAL = 0x0,
    EC_HANG_DETECT_AP_BOOT_EC_WDT = 0x1,
    EC_HANG_DETECT_AP_BOOT_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_hang_detect {
    pub /: *mut *mut uint8_t status; / enum ec_hang_detect_status,
    pub __ec_align1: },
//
// Commands for battery charging
//
// This is the single catch-all host command to exchange data regarding the
// charge state machine (v2 and up).
//
pub const EC_CMD_CHARGE_STATE: c_uint = 0x00A0;
// Subcommands for this host command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charge_state_command {
    CHARGE_STATE_CMD_GET_STATE,
    CHARGE_STATE_CMD_GET_PARAM,
    CHARGE_STATE_CMD_SET_PARAM,
    CHARGE_STATE_NUM_CMDS
}

//
// Known param numbers are defined here. Ranges are reserved for board-specific
// params, which are handled by the particular implementations.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum charge_state_params {
    CS_PARAM_CHG_VOLTAGE,	      /* charger voltage limit */
    CS_PARAM_CHG_CURRENT,	      /* charger current limit */
    CS_PARAM_CHG_INPUT_CURRENT,   /* charger input current limit */
    CS_PARAM_CHG_STATUS,	      /* charger-specific status */
    CS_PARAM_CHG_OPTION,	      /* charger-specific options */
    CS_PARAM_LIMIT_POWER,	      /*
// Check if power is limited due to
// low battery and / or a weak external
// charger. READ ONLY.
//
// How many so far?
    CS_NUM_BASE_PARAMS,

// Range for CONFIG_CHARGER_PROFILE_OVERRIDE params
    CS_PARAM_CUSTOM_PROFILE_MIN = 0x10000,
    CS_PARAM_CUSTOM_PROFILE_MAX = 0x1ffff,

// Range for CONFIG_CHARGE_STATE_DEBUG params
    CS_PARAM_DEBUG_MIN = 0x20000,
    CS_PARAM_DEBUG_CTL_MODE = 0x20000,
    CS_PARAM_DEBUG_MANUAL_MODE,
    CS_PARAM_DEBUG_SEEMS_DEAD,
    CS_PARAM_DEBUG_SEEMS_DISCONNECTED,
    CS_PARAM_DEBUG_BATT_REMOVED,
    CS_PARAM_DEBUG_MANUAL_CURRENT,
    CS_PARAM_DEBUG_MANUAL_VOLTAGE,
    CS_PARAM_DEBUG_MAX = 0x2ffff,

// Other custom param ranges go here...
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_charge_state {
    pub /: *mut *mut uint8_t cmd; / enum charge_state_command,
// get_state has no args
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub /: *mut *mut uint32_t param; / enum charge_state_param,
    pub get_param: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_todo_unpacked {
    pub /: *mut *mut uint32_t param; / param to set,
    pub /: *mut *mut uint32_t value; / value to set,
    pub set_param: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_charge_state {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align4 {
    pub ac: c_int,
    pub chg_voltage: c_int,
    pub chg_current: c_int,
    pub chg_input_current: c_int,
    pub batt_state_of_charge: c_int,
    pub get_state: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align4 {
    pub value: u32,
    pub get_param: },
// set_param returns no args
}

//
// Set maximum battery charging current.
//
pub const EC_CMD_CHARGE_CURRENT_LIMIT: c_uint = 0x00A1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_current_limit {
    pub /: *mut *mut uint32_t limit; / in mA,
    pub __ec_align4: },
//
// Set maximum external voltage / current.
//
pub const EC_CMD_EXTERNAL_POWER_LIMIT: c_uint = 0x00A2;
// Command v0 is used only on Spring and is obsolete + unsupported
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_external_power_limit_v1 {
    pub /: *mut *mut uint16_t current_lim; / in mA, or EC_POWER_LIMIT_NONE to clear limit,
    pub /: *mut *mut uint16_t voltage_lim; / in mV, or EC_POWER_LIMIT_NONE to clear limit,
    pub __ec_align2: },
pub const EC_POWER_LIMIT_NONE: c_uint = 0xffff;
//
// Set maximum voltage & current of a dedicated charge port
//
pub const EC_CMD_OVERRIDE_DEDICATED_CHARGER_LIMIT: c_uint = 0x00A3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_dedicated_charger_limit {
    pub /: *mut *mut uint16_t current_lim; / in mA,
    pub /: *mut *mut uint16_t voltage_lim; / in mV,
    pub __ec_align2: },
//
// Hibernate/Deep Sleep Commands
// Set the delay before going into hibernation.
pub const EC_CMD_HIBERNATION_DELAY: c_uint = 0x00A8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_hibernation_delay {
//
// Seconds to wait in G3 before hibernate.  Pass in 0 to read the
// current settings without changing them.
//
    pub seconds: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_hibernation_delay {
//
// The current time in seconds in which the system has been in the G3
// state.  This value is reset if the EC transitions out of G3.
//
    pub time_g3: u32,
//
// The current time remaining in seconds until the EC should hibernate.
// This value is also reset if the EC transitions out of G3.
//
    pub time_remaining: u32,
//
// The current time in seconds that the EC should wait in G3 before
// hibernating.
//
    pub hibernate_delay: u32,
    pub __ec_align4: },
// Inform the EC when entering a sleep state
pub const EC_CMD_HOST_SLEEP_EVENT: c_uint = 0x00A9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum host_sleep_event {
    HOST_SLEEP_EVENT_S3_SUSPEND   = 1,
    HOST_SLEEP_EVENT_S3_RESUME    = 2,
    HOST_SLEEP_EVENT_S0IX_SUSPEND = 3,
    HOST_SLEEP_EVENT_S0IX_RESUME  = 4,
// S3 suspend with additional enabled wake sources
    HOST_SLEEP_EVENT_S3_WAKEABLE_SUSPEND = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_host_sleep_event {
    pub sleep_event: u8,
    pub __ec_align1: },
//
// Use a default timeout value (CONFIG_SLEEP_TIMEOUT_MS) for detecting sleep
// transition failures
//
pub const EC_HOST_SLEEP_TIMEOUT_DEFAULT: c_int = 0;
// Disable timeout detection for this sleep transition
pub const EC_HOST_SLEEP_TIMEOUT_INFINITE: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_host_sleep_event_v1 {
// The type of sleep being entered or exited.
    pub sleep_event: u8,
// Padding
    pub reserved: u8,
// Parameters that apply for suspend messages.
//
// The timeout in milliseconds between when this message
// is received and when the EC will declare sleep
// transition failure if the sleep signal is not
// asserted.
//
    pub sleep_timeout_ms: u16,
    pub suspend_params: },
// No parameters for non-suspend messages.
}

// A timeout occurred when this bit is set
pub const EC_HOST_RESUME_SLEEP_TIMEOUT: c_uint = 0x80000000;
//
// The mask defining which bits correspond to the number of sleep transitions,
// as well as the maximum number of suspend line transitions that will be
// reported back to the host.
//
pub const EC_HOST_RESUME_SLEEP_TRANSITIONS_MASK: c_uint = 0x7FFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_host_sleep_event_v1 {
// Response fields that apply for resume messages.
//
// The number of sleep power signal transitions that
// occurred since the suspend message. The high bit
// indicates a timeout occurred.
//
    pub sleep_transitions: u32,
    pub resume_response: },
// No response fields for non-resume messages.
}

//
// Device events
pub const EC_CMD_DEVICE_EVENT: c_uint = 0x00AA;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_device_event {
    EC_DEVICE_EVENT_TRACKPAD,
    EC_DEVICE_EVENT_DSP,
    EC_DEVICE_EVENT_WIFI,
    EC_DEVICE_EVENT_WLC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_device_event_param {
// Get and clear pending device events
    EC_DEVICE_EVENT_PARAM_GET_CURRENT_EVENTS,
// Get device event mask
    EC_DEVICE_EVENT_PARAM_GET_ENABLED_EVENTS,
// Set device event mask
    EC_DEVICE_EVENT_PARAM_SET_ENABLED_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_device_event {
    pub event_mask: u32,
    pub param: u8,
    pub __ec_align_size1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_device_event {
    pub event_mask: u32,
    pub __ec_align4: },
//
// Smart battery pass-through
// Get / Set 16-bit smart battery registers
pub const EC_CMD_SB_READ_WORD: c_uint = 0x00B0;
pub const EC_CMD_SB_WRITE_WORD: c_uint = 0x00B1;
// Get / Set string smart battery parameters
// formatted as SMBUS "block".
//
pub const EC_CMD_SB_READ_BLOCK: c_uint = 0x00B2;
pub const EC_CMD_SB_WRITE_BLOCK: c_uint = 0x00B3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_sb_rd {
    pub reg: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_sb_rd_word {
    pub value: u16,
    pub __ec_align2: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_sb_wr_word {
    pub reg: u8,
    pub value: u16,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_sb_rd_block {
    pub data: [u8; 32],
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_sb_wr_block {
    pub reg: u8,
    pub data: [u16; 32],
    pub __ec_align1: },
//
// Battery vendor parameters
//
// Get or set vendor-specific parameters in the battery. Implementations may
// differ between boards or batteries. On a set operation, the response
// contains the actual value set, which may be rounded or clipped from the
// requested value.
//
pub const EC_CMD_BATTERY_VENDOR_PARAM: c_uint = 0x00B4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_battery_vendor_param_mode {
    BATTERY_VENDOR_PARAM_MODE_GET = 0,
    BATTERY_VENDOR_PARAM_MODE_SET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_battery_vendor_param {
    pub param: u32,
    pub value: u32,
    pub mode: u8,
    pub __ec_align_size1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_battery_vendor_param {
    pub value: u32,
    pub __ec_align4: },
//
// Smart Battery Firmware Update Commands
//
pub const EC_CMD_SB_FW_UPDATE: c_uint = 0x00B5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_sb_fw_update_subcmd {
    EC_SB_FW_UPDATE_PREPARE  = 0x0,
    EC_SB_FW_UPDATE_INFO     = 0x1, /*query sb info */
    EC_SB_FW_UPDATE_BEGIN    = 0x2, /*check if protected */
    EC_SB_FW_UPDATE_WRITE    = 0x3, /*check if protected */
    EC_SB_FW_UPDATE_END      = 0x4,
    EC_SB_FW_UPDATE_STATUS   = 0x5,
    EC_SB_FW_UPDATE_PROTECT  = 0x6,
    EC_SB_FW_UPDATE_MAX      = 0x7,
}

pub const SB_FW_UPDATE_CMD_WRITE_BLOCK_SIZE: c_int = 32;
pub const SB_FW_UPDATE_CMD_STATUS_SIZE: c_int = 2;
pub const SB_FW_UPDATE_CMD_INFO_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_sb_fw_update_header {
    pub /: *mut *mut uint16_t subcmd; / enum ec_sb_fw_update_subcmd,
    pub /: *mut *mut uint16_t fw_id; / firmware id,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_sb_fw_update {
    pub hdr: ec_sb_fw_update_header,
// EC_SB_FW_UPDATE_PREPARE  = 0x0
// EC_SB_FW_UPDATE_INFO     = 0x1
// EC_SB_FW_UPDATE_BEGIN    = 0x2
// EC_SB_FW_UPDATE_END      = 0x4
// EC_SB_FW_UPDATE_STATUS   = 0x5
// EC_SB_FW_UPDATE_PROTECT  = 0x6
// Those have no args
// EC_SB_FW_UPDATE_WRITE    = 0x3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align4 {
    pub data: [u8; SB_FW_UPDATE_CMD_WRITE_BLOCK_SIZE],
    pub write: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_sb_fw_update {
// EC_SB_FW_UPDATE_INFO     = 0x1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align1 {
    pub data: [u8; SB_FW_UPDATE_CMD_INFO_SIZE],
    pub info: },
// EC_SB_FW_UPDATE_STATUS   = 0x5
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ec_align1 {
    pub data: [u8; SB_FW_UPDATE_CMD_STATUS_SIZE],
    pub status: },
}

//
// Entering Verified Boot Mode Command
// Default mode is VBOOT_MODE_NORMAL if EC did not receive this command.
// Valid Modes are: normal, developer, and recovery.
//
pub const EC_CMD_ENTERING_MODE: c_uint = 0x00B6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_entering_mode {
    pub vboot_mode: c_int,
    pub __ec_align4: },
pub const VBOOT_MODE_NORMAL: c_int = 0;
pub const VBOOT_MODE_DEVELOPER: c_int = 1;
pub const VBOOT_MODE_RECOVERY: c_int = 2;
//
// I2C passthru protection command: Protects I2C tunnels against access on
// certain addresses (board-specific).
//
pub const EC_CMD_I2C_PASSTHRU_PROTECT: c_uint = 0x00B7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_i2c_passthru_protect_subcmd {
    EC_CMD_I2C_PASSTHRU_PROTECT_STATUS = 0x0,
    EC_CMD_I2C_PASSTHRU_PROTECT_ENABLE = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_i2c_passthru_protect {
    pub subcmd: u8,
    pub /: *mut *mut uint8_t port; / I2C port number,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_i2c_passthru_protect {
    pub /: *mut *mut uint8_t status; / Status flags (0: unlocked, 1: locked),
    pub __ec_align1: },
//
// HDMI CEC commands
//
// These commands are for sending and receiving message via HDMI CEC
//
pub const EC_CEC_MAX_PORTS: c_int = 16;
pub const MAX_CEC_MSG_LEN: c_int = 16;
//
// Helper macros for packing/unpacking cec_events.
// bits[27:0] : bitmask of events from enum mkbp_cec_event
// bits[31:28]: port number
//

// CEC message from the AP to be written on the CEC bus
pub const EC_CMD_CEC_WRITE_MSG: c_uint = 0x00B8;
//
// struct ec_params_cec_write - Message to write to the CEC bus
// @msg: message content to write to the CEC bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_cec_write {
    pub msg: [u8; MAX_CEC_MSG_LEN],
    pub __ec_align1: },
//
// struct ec_params_cec_write_v1 - Message to write to the CEC bus
// @port: CEC port to write the message on
// @msg_len: length of msg in bytes
// @msg: message content to write to the CEC bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_cec_write_v1 {
    pub port: u8,
    pub msg_len: u8,
    pub msg: [u8; MAX_CEC_MSG_LEN],
    pub __ec_align1: },
// CEC message read from a CEC bus reported back to the AP
pub const EC_CMD_CEC_READ_MSG: c_uint = 0x00B9;
//
// struct ec_params_cec_read - Read a message from the CEC bus
// @port: CEC port to read a message on
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_cec_read {
    pub port: u8,
    pub __ec_align1: },
//
// struct ec_response_cec_read - Message read from the CEC bus
// @msg_len: length of msg in bytes
// @msg: message content read from the CEC bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_cec_read {
    pub msg_len: u8,
    pub msg: [u8; MAX_CEC_MSG_LEN],
    pub __ec_align1: },
// Set various CEC parameters
pub const EC_CMD_CEC_SET: c_uint = 0x00BA;
//
// struct ec_params_cec_set - CEC parameters set
// @cmd: parameter type, can be CEC_CMD_ENABLE or CEC_CMD_LOGICAL_ADDRESS
// @port: CEC port to set the parameter on
// @val: in case cmd is CEC_CMD_ENABLE, this field can be 0 to disable CEC
// or 1 to enable CEC functionality, in case cmd is
// CEC_CMD_LOGICAL_ADDRESS, this field encodes the requested logical
// address between 0 and 15 or 0xff to unregister
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_cec_set {
    pub /: *mut *mut uint8_t cmd : 4; / enum cec_command,
    pub 4: uint8_t port :,
    pub val: u8,
    pub __ec_align1: },
// Read various CEC parameters
pub const EC_CMD_CEC_GET: c_uint = 0x00BB;
//
// struct ec_params_cec_get - CEC parameters get
// @cmd: parameter type, can be CEC_CMD_ENABLE or CEC_CMD_LOGICAL_ADDRESS
// @port: CEC port to get the parameter on
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_cec_get {
    pub /: *mut *mut uint8_t cmd : 4; / enum cec_command,
    pub 4: uint8_t port :,
    pub __ec_align1: },
//
// struct ec_response_cec_get - CEC parameters get response
// @val: in case cmd was CEC_CMD_ENABLE, this field will 0 if CEC is
// disabled or 1 if CEC functionality is enabled,
// in case cmd was CEC_CMD_LOGICAL_ADDRESS, this will encode the
// configured logical address between 0 and 15 or 0xff if unregistered
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_cec_get {
    pub val: u8,
    pub __ec_align1: },
// Get the number of CEC ports
pub const EC_CMD_CEC_PORT_COUNT: c_uint = 0x00C1;
//
// struct ec_response_cec_port_count - CEC port count response
// @port_count: number of CEC ports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_cec_port_count {
    pub port_count: u8,
    pub __ec_align1: },
// CEC parameters command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cec_command {
// CEC reading, writing and events enable
    CEC_CMD_ENABLE,
// CEC logical address
    CEC_CMD_LOGICAL_ADDRESS,
}

// Events from CEC to AP
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mkbp_cec_event {
// Outgoing message was acknowledged by a follower
    EC_MKBP_CEC_SEND_OK			= BIT(0),
// Outgoing message was not acknowledged
    EC_MKBP_CEC_SEND_FAILED			= BIT(1),
// Incoming message can be read out by AP
    EC_MKBP_CEC_HAVE_DATA			= BIT(2),
}

//
// Commands for audio codec.
pub const EC_CMD_EC_CODEC: c_uint = 0x00BC;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_subcmd {
    EC_CODEC_GET_CAPABILITIES = 0x0,
    EC_CODEC_GET_SHM_ADDR = 0x1,
    EC_CODEC_SET_SHM_ADDR = 0x2,
    EC_CODEC_SUBCMD_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_cap {
    EC_CODEC_CAP_WOV_AUDIO_SHM = 0,
    EC_CODEC_CAP_WOV_LANG_SHM = 1,
    EC_CODEC_CAP_LAST = 32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_shm_id {
    EC_CODEC_SHM_ID_WOV_AUDIO = 0x0,
    EC_CODEC_SHM_ID_WOV_LANG = 0x1,
    EC_CODEC_SHM_ID_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_shm_type {
    EC_CODEC_SHM_TYPE_EC_RAM = 0x0,
    EC_CODEC_SHM_TYPE_SYSTEM_RAM = 0x1,
}

    pub shm_id: u8,
    pub reserved: [u8; 3],
}

//
// Commands for DMIC on audio codec.
pub const EC_CMD_EC_CODEC_DMIC: c_uint = 0x00BD;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_dmic_subcmd {
    EC_CODEC_DMIC_GET_MAX_GAIN = 0x0,
    EC_CODEC_DMIC_SET_GAIN_IDX = 0x1,
    EC_CODEC_DMIC_GET_GAIN_IDX = 0x2,
    EC_CODEC_DMIC_SUBCMD_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_dmic_channel {
    EC_CODEC_DMIC_CHANNEL_0 = 0x0,
    EC_CODEC_DMIC_CHANNEL_1 = 0x1,
    EC_CODEC_DMIC_CHANNEL_2 = 0x2,
    EC_CODEC_DMIC_CHANNEL_3 = 0x3,
    EC_CODEC_DMIC_CHANNEL_4 = 0x4,
    EC_CODEC_DMIC_CHANNEL_5 = 0x5,
    EC_CODEC_DMIC_CHANNEL_6 = 0x6,
    EC_CODEC_DMIC_CHANNEL_7 = 0x7,
    EC_CODEC_DMIC_CHANNEL_COUNT,
}

//
// Commands for I2S RX on audio codec.
pub const EC_CMD_EC_CODEC_I2S_RX: c_uint = 0x00BE;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_i2s_rx_subcmd {
    EC_CODEC_I2S_RX_ENABLE = 0x0,
    EC_CODEC_I2S_RX_DISABLE = 0x1,
    EC_CODEC_I2S_RX_SET_SAMPLE_DEPTH = 0x2,
    EC_CODEC_I2S_RX_SET_DAIFMT = 0x3,
    EC_CODEC_I2S_RX_SET_BCLK = 0x4,
    EC_CODEC_I2S_RX_RESET = 0x5,
    EC_CODEC_I2S_RX_SUBCMD_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_i2s_rx_sample_depth {
    EC_CODEC_I2S_RX_SAMPLE_DEPTH_16 = 0x0,
    EC_CODEC_I2S_RX_SAMPLE_DEPTH_24 = 0x1,
    EC_CODEC_I2S_RX_SAMPLE_DEPTH_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_i2s_rx_daifmt {
    EC_CODEC_I2S_RX_DAIFMT_I2S = 0x0,
    EC_CODEC_I2S_RX_DAIFMT_RIGHT_J = 0x1,
    EC_CODEC_I2S_RX_DAIFMT_LEFT_J = 0x2,
    EC_CODEC_I2S_RX_DAIFMT_COUNT,
}

//
// Commands for WoV on audio codec.
pub const EC_CMD_EC_CODEC_WOV: c_uint = 0x00BF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_codec_wov_subcmd {
    EC_CODEC_WOV_SET_LANG = 0x0,
    EC_CODEC_WOV_SET_LANG_SHM = 0x1,
    EC_CODEC_WOV_GET_LANG = 0x2,
    EC_CODEC_WOV_ENABLE = 0x3,
    EC_CODEC_WOV_DISABLE = 0x4,
    EC_CODEC_WOV_READ_AUDIO = 0x5,
    EC_CODEC_WOV_READ_AUDIO_SHM = 0x6,
    EC_CODEC_WOV_SUBCMD_COUNT,
}

//
// @hash is SHA256 of the whole language model.
// @total_len indicates the length of whole language model.
// @offset is the cursor from the beginning of the model.
// @buf is the packet buffer.
// @len denotes how many bytes in the buf.
//
// System commands
//
// TODO(crosbug.com/p/23747): This is a confusing name, since it doesn't
// necessarily reboot the EC.  Rename to "image" or something similar?
//
pub const EC_CMD_REBOOT_EC: c_uint = 0x00D2;
// Command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_reboot_cmd {
    EC_REBOOT_CANCEL = 0,        /* Cancel a pending reboot */
    EC_REBOOT_JUMP_RO = 1,       /* Jump to RO without rebooting */
    EC_REBOOT_JUMP_RW = 2,       /* Jump to active RW without rebooting */
// (command 3 was jump to RW-B)
    EC_REBOOT_COLD = 4,          /* Cold-reboot */
    EC_REBOOT_DISABLE_JUMP = 5,  /* Disable jump until next reboot */
    EC_REBOOT_HIBERNATE = 6,     /* Hibernate EC */
    EC_REBOOT_HIBERNATE_CLEAR_AP_OFF = 7, /* and clears AP_OFF flag */
    EC_REBOOT_COLD_AP_OFF = 8,   /* Cold-reboot and don't boot AP */
}

// Flags for ec_params_reboot_ec.reboot_flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_reboot_ec {
    pub /: *mut *mut uint8_t cmd; / enum ec_reboot_cmd,
    pub /: *mut *mut *mut uint8_t flags; / See EC_REBOOT_FLAG_,
    pub __ec_align1: },
//
// Get information on last EC panic.
//
// Returns variable-length platform-dependent panic information.  See panic.h
// for details.
//
pub const EC_CMD_GET_PANIC_INFO: c_uint = 0x00D3;
//
// Special commands
//
// These do not follow the normal rules for commands.  See each command for
// details.
//
// Reboot NOW
//
// This command will work even when the EC LPC interface is busy, because the
// reboot command is processed at interrupt level.  Note that when the EC
// reboots, the host will reboot too, so there is no response to this command.
//
// Use EC_CMD_REBOOT_EC to reboot the EC more politely.
//
pub const EC_CMD_REBOOT: c_uint = 0x00D1  /* Think "die" */;
//
// Resend last response (not supported on LPC).
//
// Returns EC_RES_UNAVAILABLE if there is no response available - for example,
// there was no previous command, or the previous command's response was too
// big to save.
//
pub const EC_CMD_RESEND_RESPONSE: c_uint = 0x00DB;
//
// This header byte on a command indicate version 0. Any header byte less
// than this means that we are talking to an old EC which doesn't support
// versioning. In that case, we assume version 0.
//
// Header bytes greater than this indicate a later version. For example,
// EC_CMD_VERSION0 + 1 means we are using version 1.
//
// The old EC interface must not use commands 0xdc or higher.
//
pub const EC_CMD_VERSION0: c_uint = 0x00DC;
//
// PD commands
//
// These commands are for PD MCU communication.
//
// EC to PD MCU exchange status command
pub const EC_CMD_PD_EXCHANGE_STATUS: c_uint = 0x0100;
pub const EC_VER_PD_EXCHANGE_STATUS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_charge_state {
    PD_CHARGE_NO_CHANGE = 0, /* Don't change charge state */
    PD_CHARGE_NONE,          /* No charging allowed */
    PD_CHARGE_5V,            /* 5V charging only */
    PD_CHARGE_MAX            /* Charge at max voltage */
}

// Status of EC being sent to PD

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pd_status {
    pub /: *mut *mut uint8_t status; / EC status,
    pub /: *mut *mut int8_t batt_soc; / battery state of charge,
    pub /: *mut *mut uint8_t charge_state; / charging state (from enum pd_charge_state),
    pub __ec_align1: },
// Status of PD being sent back to EC

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pd_status {
    pub /: *mut *mut uint32_t curr_lim_ma; / input current limit,
    pub /: *mut *mut uint16_t status; / PD MCU status,
    pub /: *mut *mut int8_t active_charge_port; / active charging port,
    pub __ec_align_size1: },
// AP to PD MCU host event status command, cleared on read
pub const EC_CMD_PD_HOST_EVENT_STATUS: c_uint = 0x0104;
// PD MCU host event status bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_host_event_status {
    pub /: *mut *mut uint32_t status; / PD MCU host event status,
    pub __ec_align4: },
// Set USB type-C port role and muxes
pub const EC_CMD_USB_PD_CONTROL: c_uint = 0x0101;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_pd_control_role {
    USB_PD_CTRL_ROLE_NO_CHANGE = 0,
    USB_PD_CTRL_ROLE_TOGGLE_ON = 1, /* == AUTO */
    USB_PD_CTRL_ROLE_TOGGLE_OFF = 2,
    USB_PD_CTRL_ROLE_FORCE_SINK = 3,
    USB_PD_CTRL_ROLE_FORCE_SOURCE = 4,
    USB_PD_CTRL_ROLE_FREEZE = 5,
    USB_PD_CTRL_ROLE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_pd_control_mux {
    USB_PD_CTRL_MUX_NO_CHANGE = 0,
    USB_PD_CTRL_MUX_NONE = 1,
    USB_PD_CTRL_MUX_USB = 2,
    USB_PD_CTRL_MUX_DP = 3,
    USB_PD_CTRL_MUX_DOCK = 4,
    USB_PD_CTRL_MUX_AUTO = 5,
    USB_PD_CTRL_MUX_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_pd_control_swap {
    USB_PD_CTRL_SWAP_NONE = 0,
    USB_PD_CTRL_SWAP_DATA = 1,
    USB_PD_CTRL_SWAP_POWER = 2,
    USB_PD_CTRL_SWAP_VCONN = 3,
    USB_PD_CTRL_SWAP_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_control {
    pub port: u8,
    pub role: u8,
    pub mux: u8,
    pub swap: u8,
    pub __ec_align1: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_usb_pd_control {
    pub enabled: u8,
    pub role: u8,
    pub polarity: u8,
    pub state: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_usb_pd_control_v1 {
    pub enabled: u8,
    pub role: u8,
    pub polarity: u8,
    pub state: [c_char; 32],
    pub __ec_align1: },
// Values representing usbc PD CC state

// Active/Passive Cable

// Optical/Non-optical cable

// 3rd Gen TBT device (or AMA)/2nd gen tbt Adapter

// Active Link Uni-Direction

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_usb_pd_control_v2 {
    pub enabled: u8,
    pub role: u8,
    pub polarity: u8,
    pub state: [c_char; 32],
    pub /: *mut *mut uint8_t cc_state; / enum pd_cc_states representing cc state,
    pub /: *mut *mut uint8_t dp_mode; / Current DP pin mode (MODE_DP_PIN_[A-E]),
    pub /: *mut *mut uint8_t reserved; / Reserved for future use,
    pub /: *mut *mut *mut uint8_t control_flags; / USB_PD_CTRL_flags,
    pub /: *mut *mut *mut uint8_t cable_speed; / TBT_SS_ cable speed,
    pub /: *mut *mut *mut uint8_t cable_gen; / TBT_GEN3_ cable rounded support,
    pub __ec_align1: },
pub const EC_CMD_USB_PD_PORTS: c_uint = 0x0102;
// Maximum number of PD ports on a device, num_ports will be <= this
pub const EC_USB_PD_MAX_PORTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_usb_pd_ports {
    pub num_ports: u8,
    pub __ec_align1: },
pub const EC_CMD_USB_PD_POWER_INFO: c_uint = 0x0103;
pub const PD_POWER_CHARGING_PORT: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_power_info {
    pub port: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_chg_type {
    USB_CHG_TYPE_NONE,
    USB_CHG_TYPE_PD,
    USB_CHG_TYPE_C,
    USB_CHG_TYPE_PROPRIETARY,
    USB_CHG_TYPE_BC12_DCP,
    USB_CHG_TYPE_BC12_CDP,
    USB_CHG_TYPE_BC12_SDP,
    USB_CHG_TYPE_OTHER,
    USB_CHG_TYPE_VBUS,
    USB_CHG_TYPE_UNKNOWN,
    USB_CHG_TYPE_DEDICATED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_power_roles {
    USB_PD_PORT_POWER_DISCONNECTED,
    USB_PD_PORT_POWER_SOURCE,
    USB_PD_PORT_POWER_SINK,
    USB_PD_PORT_POWER_SINK_NOT_CHARGING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_chg_measures {
    pub voltage_max: u16,
    pub voltage_now: u16,
    pub current_max: u16,
    pub current_lim: u16,
    pub __ec_align2: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_usb_pd_power_info {
    pub role: u8,
    pub type: u8,
    pub dualrole: u8,
    pub reserved1: u8,
    pub meas: usb_chg_measures,
    pub max_power: u32,
    pub __ec_align4: },
//
// This command will return the number of USB PD charge port + the number
// of dedicated port present.
// EC_CMD_USB_PD_PORTS does NOT include the dedicated ports
//
pub const EC_CMD_CHARGE_PORT_COUNT: c_uint = 0x0105;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_charge_port_count {
    pub port_count: u8,
    pub __ec_align1: },
// Write USB-PD device FW
pub const EC_CMD_USB_PD_FW_UPDATE: c_uint = 0x0110;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_pd_fw_update_cmds {
    USB_PD_FW_REBOOT,
    USB_PD_FW_FLASH_ERASE,
    USB_PD_FW_FLASH_WRITE,
    USB_PD_FW_ERASE_SIG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_fw_update {
    pub dev_id: u16,
    pub cmd: u8,
    pub port: u8,
    pub /: *mut *mut uint32_t size; / Size to write in bytes,
// Followed by data to write
    pub __ec_align4: },
// Write USB-PD Accessory RW_HASH table entry
pub const EC_CMD_USB_PD_RW_HASH_ENTRY: c_uint = 0x0111;
// RW hash is first 20 bytes of SHA-256 of RW section
pub const PD_RW_HASH_SIZE: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_rw_hash_entry {
    pub dev_id: u16,
    pub dev_rw_hash: [u8; PD_RW_HASH_SIZE],
    pub /*: *mut uint8_t reserved;,
// For alignment of current_image
// TODO(rspangler) but it's not aligned!
// Should have been reserved[2].
//
    pub /: *mut *mut uint32_t current_image; / One of ec_current_image,
    pub __ec_align1: },
// Read USB-PD Accessory info
pub const EC_CMD_USB_PD_DEV_INFO: c_uint = 0x0112;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_info_request {
    pub port: u8,
    pub __ec_align1: },
// Read USB-PD Device discovery info
pub const EC_CMD_USB_PD_DISCOVERY: c_uint = 0x0113;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_discovery_entry {
    pub /: *mut *mut uint16_t vid; / USB-IF VID,
    pub /: *mut *mut uint16_t pid; / USB-IF PID,
    pub /: *mut *mut uint8_t ptype; / product type (hub,periph,cable,ama),
    pub __ec_align_size1: },
// Override default charge behavior
pub const EC_CMD_PD_CHARGE_PORT_OVERRIDE: c_uint = 0x0114;
// Negative port parameters have special meaning
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_pd_override_ports {
    OVERRIDE_DONT_CHARGE = -2,
    OVERRIDE_OFF = -1,
// [0, CONFIG_USB_PD_PORT_COUNT): Port#
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_charge_port_override {
    pub /: *mut *mut int16_t override_port; / Override port#,
    pub __ec_align2: },
//
// Read (and delete) one entry of PD event log.
// TODO(crbug.com/751742): Make this host command more generic to accommodate
// future non-PD logs that use the same internal EC event_log.
//
pub const EC_CMD_PD_GET_LOG_ENTRY: c_uint = 0x0115;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pd_log {
    pub /: *mut *mut uint32_t timestamp; / relative timestamp in milliseconds,
    pub /: *mut *mut uint8_t type; / event type : see PD_EVENT_xx below,
    pub /: *mut *mut uint8_t size_port; / [7:5] port number [4:0] payload size in bytes,
    pub /: *mut *mut uint16_t data; / type-defined data payload,
    pub /: *mut *mut uint8_t payload[]; / optional additional data payload: 0..16 bytes,
    pub __ec_align4: },
// The timestamp is the microsecond counter shifted to get about a ms.

pub const PD_LOG_SIZE_MASK: c_uint = 0x1f;
pub const PD_LOG_PORT_MASK: c_uint = 0xe0;
pub const PD_LOG_PORT_SHIFT: c_int = 5;

// PD event log : entry types
// PD MCU events
pub const PD_EVENT_MCU_BASE: c_uint = 0x00;

// Reserved for custom board event

// PD generic accessory events
pub const PD_EVENT_ACC_BASE: c_uint = 0x20;

// PD power supply events
pub const PD_EVENT_PS_BASE: c_uint = 0x40;

// PD video dongles events
pub const PD_EVENT_VIDEO_BASE: c_uint = 0x60;

// Returned in the "type" field, when there is no entry available
pub const PD_EVENT_NO_ENTRY: c_uint = 0xff;
//
// PD_EVENT_MCU_CHARGE event definition :
// the payload is "struct usb_chg_measures"
// the data field contains the port state flags as defined below :
//
// Port partner is a dual role device

// Port is the pending override port

// Port is the override port

// Charger type
pub const CHARGE_FLAGS_TYPE_SHIFT: c_int = 3;

// Power delivery role

//
// PD_EVENT_PS_FAULT data field flags definition :
//
pub const PS_FAULT_OCP: c_int = 1;
pub const PS_FAULT_FAST_OCP: c_int = 2;
pub const PS_FAULT_OVP: c_int = 3;
pub const PS_FAULT_DISCH: c_int = 4;
//
// PD_EVENT_VIDEO_CODEC payload is "struct mcdp_info".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcdp_version {
    pub major: u8,
    pub minor: u8,
    pub build: u16,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcdp_info {
    pub family: [u8; 2],
    pub chipid: [u8; 2],
    pub irom: mcdp_version,
    pub fw: mcdp_version,
    pub __ec_align4: },
// struct mcdp_info field decoding

// Get/Set USB-PD Alternate mode info
pub const EC_CMD_USB_PD_GET_AMODE: c_uint = 0x0116;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_get_mode_request {
    pub /: *mut *mut uint16_t svid_idx; / SVID index to get,
    pub /: *mut *mut uint8_t port; / port,
    pub __ec_align_size1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_get_mode_response {
    pub /: *mut *mut uint16_t svid; / SVID,
    pub /: *mut *mut uint16_t opos; / Object Position,
    pub /: *mut *mut uint32_t vdo[6]; / Mode VDOs,
    pub __ec_align4: },
pub const EC_CMD_USB_PD_SET_AMODE: c_uint = 0x0117;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_mode_cmd {
    PD_EXIT_MODE = 0,
    PD_ENTER_MODE = 1,
// Not a command.  Do NOT remove.
    PD_MODE_CMD_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_set_mode_request {
    pub /: *mut *mut uint32_t cmd; / enum pd_mode_cmd,
    pub /: *mut *mut uint16_t svid; / SVID to set,
    pub /: *mut *mut uint8_t opos; / Object Position,
    pub /: *mut *mut uint8_t port; / port,
    pub __ec_align4: },
// Ask the PD MCU to record a log of a requested type
pub const EC_CMD_PD_WRITE_LOG_ENTRY: c_uint = 0x0118;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pd_write_log_entry {
    pub /: *mut *mut uint8_t type; / event type : see PD_EVENT_xx above,
    pub /: *mut *mut uint8_t port; / port#, or 0 for events unrelated to a given port,
    pub __ec_align1: },
// Control USB-PD chip
pub const EC_CMD_PD_CONTROL: c_uint = 0x0119;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_pd_control_cmd {
    PD_SUSPEND = 0,      /* Suspend the PD chip (EC: stop talking to PD) */
    PD_RESUME,           /* Resume the PD chip (EC: start talking to PD) */
    PD_RESET,            /* Force reset the PD chip */
    PD_CONTROL_DISABLE,  /* Disable further calls to this command */
    PD_CHIP_ON,          /* Power on the PD chip */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pd_control {
    pub /: *mut *mut uint8_t chip; / chip id,
    pub subcmd: u8,
    pub __ec_align1: },
// Get info about USB-C SS muxes
pub const EC_CMD_USB_PD_MUX_INFO: c_uint = 0x011A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_mux_info {
    pub /: *mut *mut uint8_t port; / USB-C port number,
    pub __ec_align1: },
// Flags representing mux state

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_usb_pd_mux_info {
    pub /: *mut *mut *mut uint8_t flags; / USB_PD_MUX_-encoded USB mux state,
    pub __ec_align1: },
pub const EC_CMD_PD_CHIP_INFO: c_uint = 0x011B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pd_chip_info {
    pub /: *mut *mut uint8_t port; / USB-C port number,
    pub /: *mut *mut uint8_t renew; / Force renewal,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pd_chip_info {
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_id: u16,
    pub fw_version_string: [u8; 8],
    pub fw_version_number: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pd_chip_info_v1 {
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_id: u16,
    pub fw_version_string: [u8; 8],
    pub fw_version_number: u64,
}

// Run RW signature verification and get status
pub const EC_CMD_RWSIG_CHECK_STATUS: c_uint = 0x011C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_rwsig_check_status {
    pub status: u32,
    pub __ec_align4: },
// For controlling RWSIG task
pub const EC_CMD_RWSIG_ACTION: c_uint = 0x011D;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rwsig_action {
    RWSIG_ACTION_ABORT = 0,		/* Abort RWSIG and prevent jumping */
    RWSIG_ACTION_CONTINUE = 1,	/* Jump to RW immediately */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_rwsig_action {
    pub action: u32,
    pub __ec_align4: },
// Run verification on a slot
pub const EC_CMD_EFS_VERIFY: c_uint = 0x011E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_efs_verify {
    pub /: *mut *mut uint8_t region; / enum ec_flash_region,
    pub __ec_align1: },
//
// Retrieve info from Cros Board Info store. Response is based on the data
// type. Integers return a uint32. Strings return a string, using the response
// size to determine how big it is.
//
pub const EC_CMD_GET_CROS_BOARD_INFO: c_uint = 0x011F;
//
// Write info into Cros Board Info on EEPROM. Write fails if the board has
// hardware write-protect enabled.
//
pub const EC_CMD_SET_CROS_BOARD_INFO: c_uint = 0x0120;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cbi_data_tag {
    CBI_TAG_BOARD_VERSION = 0, /* uint32_t or smaller */
    CBI_TAG_OEM_ID = 1,        /* uint32_t or smaller */
    CBI_TAG_SKU_ID = 2,        /* uint32_t or smaller */
    CBI_TAG_DRAM_PART_NUM = 3, /* variable length ascii, nul terminated. */
    CBI_TAG_OEM_NAME = 4,      /* variable length ascii, nul terminated. */
    CBI_TAG_MODEL_ID = 5,      /* uint32_t or smaller */
    CBI_TAG_COUNT,
}

//
// Flags to control read operation
//
// RELOAD:  Invalidate cache and read data from EEPROM. Useful to verify
// write was successful without reboot.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_get_cbi {
    pub /: *mut *mut uint32_t tag; / enum cbi_data_tag,
    pub /: *mut *mut *mut uint32_t flag; / CBI_GET_,
    pub __ec_align4: },
//
// Flags to control write behavior.
//
// NO_SYNC: Makes EC update data in RAM but skip writing to EEPROM. It's
// useful when writing multiple fields in a row.
// INIT:    Need to be set when creating a new CBI from scratch. All fields
// will be initialized to zero first.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_set_cbi {
    pub /: *mut *mut uint32_t tag; / enum cbi_data_tag,
    pub /: *mut *mut *mut uint32_t flag; / CBI_SET_,
    pub /: *mut *mut uint32_t size; / Data size,
    pub /: *mut *mut uint8_t data[]; / For string and raw data,
    pub __ec_align1: },
//
// Information about resets of the AP by the EC and the EC's own uptime.
//
pub const EC_CMD_GET_UPTIME_INFO: c_uint = 0x0121;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_uptime_info {
//
// Number of milliseconds since the last EC boot. Sysjump resets
// typically do not restart the EC's time_since_boot epoch.
//
// WARNING: The EC's sense of time is much less accurate than the AP's
// sense of time, in both phase and frequency.  This timebase is similar
// to CLOCK_MONOTONIC_RAW, but with 1% or more frequency error.
//
    pub time_since_ec_boot_ms: u32,
//
// Number of times the AP was reset by the EC since the last EC boot.
// Note that the AP may be held in reset by the EC during the initial
// boot sequence, such that the very first AP boot may count as more
// than one here.
//
    pub ap_resets_since_ec_boot: u32,
//
// The set of flags which describe the EC's most recent reset.  See
// include/system.h RESET_FLAG_* for details.
//
    pub ec_reset_flags: u32,
// Empty log entries have both the cause and timestamp set to zero.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_reset_log_entry {
//
// See include/chipset.h: enum chipset_{reset,shutdown}_reason
// for details.
//
    pub reset_cause: u16,
// Reserved for protocol growth.
    pub reserved: u16,
//
// The time of the reset's assertion, in milliseconds since the
// last EC boot, in the same epoch as time_since_ec_boot_ms.
// Set to zero if the log entry is empty.
//
    pub reset_time_ms: u32,
    pub recent_ap_reset: [}; 4],
    pub __ec_align4: },
//
// Add entropy to the device secret (stored in the rollback region).
//
// Depending on the chip, the operation may take a long time (e.g. to erase
// flash), so the commands are asynchronous.
//
pub const EC_CMD_ADD_ENTROPY: c_uint = 0x0122;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum add_entropy_action {
// Add entropy to the current secret.
    ADD_ENTROPY_ASYNC = 0,
//
// Add entropy, and also make sure that the previous secret is erased.
// (this can be implemented by adding entropy multiple times until
// all rolback blocks have been overwritten).
//
    ADD_ENTROPY_RESET_ASYNC = 1,
// Read back result from the previous operation.
    ADD_ENTROPY_GET_RESULT = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_rollback_add_entropy {
    pub action: u8,
    pub __ec_align1: },
//
// Perform a single read of a given ADC channel.
//
pub const EC_CMD_ADC_READ: c_uint = 0x0123;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_adc_read {
    pub adc_channel: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_adc_read {
    pub adc_value: i32,
    pub __ec_align4: },
//
// Read back rollback info
//
pub const EC_CMD_ROLLBACK_INFO: c_uint = 0x0124;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_rollback_info {
    pub /: *mut *mut int32_t id; / Incrementing number to indicate which region to use.,
    pub rollback_min_version: i32,
    pub rw_rollback_version: i32,
    pub __ec_align4: },
// Issue AP reset
pub const EC_CMD_AP_RESET: c_uint = 0x0125;
//
// Get the number of peripheral charge ports
//
pub const EC_CMD_PCHG_COUNT: c_uint = 0x0134;
pub const EC_PCHG_MAX_PORTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pchg_count {
    pub port_count: u8,
    pub __ec_align1: },
//
// Get the status of a peripheral charge port
//
pub const EC_CMD_PCHG: c_uint = 0x0135;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pchg {
    pub port: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pchg {
    pub /: *mut *mut uint32_t error; / enum pchg_error,
    pub /: *mut *mut uint8_t state; / enum pchg_state state,
    pub battery_percentage: u8,
    pub unused0: u8,
    pub unused1: u8,
// Fields added in version 1
    pub fw_version: u32,
    pub dropped_event_count: u32,
    pub __ec_align2: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pchg_state {
// Charger is reset and not initialized.
    PCHG_STATE_RESET = 0,
// Charger is initialized or disabled.
    PCHG_STATE_INITIALIZED,
// Charger is enabled and ready to detect a device.
    PCHG_STATE_ENABLED,
// Device is in proximity.
    PCHG_STATE_DETECTED,
// Device is being charged.
    PCHG_STATE_CHARGING,
// Device is fully charged. It implies DETECTED (& not charging).
    PCHG_STATE_FULL,
// In download (a.k.a. firmware update) mode
    PCHG_STATE_DOWNLOAD,
// In download mode. Ready for receiving data.
    PCHG_STATE_DOWNLOADING,
// Device is ready for data communication.
    PCHG_STATE_CONNECTED,
// Put no more entry below
    PCHG_STATE_COUNT,
}

//
// Update firmware of peripheral chip
//
pub const EC_CMD_PCHG_UPDATE: c_uint = 0x0136;
// Port number is encoded in bit[28:31].
pub const EC_MKBP_PCHG_PORT_SHIFT: c_int = 28;
// Utility macro for converting MKBP event to port number.

// Utility macro for extracting event bits.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec_pchg_update_cmd {
// Reset chip to normal mode.
    EC_PCHG_UPDATE_CMD_RESET_TO_NORMAL = 0,
// Reset and put a chip in update (a.k.a. download) mode.
    EC_PCHG_UPDATE_CMD_OPEN,
// Write a block of data containing FW image.
    EC_PCHG_UPDATE_CMD_WRITE,
// Close update session.
    EC_PCHG_UPDATE_CMD_CLOSE,
// End of commands
    EC_PCHG_UPDATE_CMD_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_pchg_update {
// PCHG port number
    pub port: u8,
// enum ec_pchg_update_cmd
    pub cmd: u8,
// Padding
    pub reserved0: u8,
    pub reserved1: u8,
// Version of new firmware
    pub version: u32,
// CRC32 of new firmware
    pub crc32: u32,
// Address in chip memory where <data> is written to
    pub addr: u32,
// Size of <data>
    pub size: u32,
// Partial data of new firmware
    pub data: [u8; ],
    pub __ec_align4: },
    pub )0)->cmd)*8)): *mut < BIT(sizeof(((struct ec_params_pchg_update,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_pchg_update {
// Block size
    pub block_size: u32,
    pub __ec_align4: },
//
// Voltage regulator controls
//
// Get basic info of voltage regulator for given index.
//
// Returns the regulator name and supported voltage list in mV.
//
pub const EC_CMD_REGULATOR_GET_INFO: c_uint = 0x012C;
// Maximum length of regulator name
pub const EC_REGULATOR_NAME_MAX_LEN: c_int = 16;
// Maximum length of the supported voltage list.
pub const EC_REGULATOR_VOLTAGE_MAX_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_regulator_get_info {
    pub index: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_regulator_get_info {
    pub name: [c_char; EC_REGULATOR_NAME_MAX_LEN],
    pub num_voltages: u16,
    pub voltages_mv: [u16; EC_REGULATOR_VOLTAGE_MAX_COUNT],
    pub __ec_align2: },
//
// Configure the regulator as enabled / disabled.
//
pub const EC_CMD_REGULATOR_ENABLE: c_uint = 0x012D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_regulator_enable {
    pub index: u32,
    pub enable: u8,
    pub __ec_align4: },
//
// Query if the regulator is enabled.
//
// Returns 1 if the regulator is enabled, 0 if not.
//
pub const EC_CMD_REGULATOR_IS_ENABLED: c_uint = 0x012E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_regulator_is_enabled {
    pub index: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_regulator_is_enabled {
    pub enabled: u8,
    pub __ec_align1: },
//
// Set voltage for the voltage regulator within the range specified.
//
// The driver should select the voltage in range closest to min_mv.
//
// Also note that this might be called before the regulator is enabled, and the
// setting should be in effect after the regulator is enabled.
//
pub const EC_CMD_REGULATOR_SET_VOLTAGE: c_uint = 0x012F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_regulator_set_voltage {
    pub index: u32,
    pub min_mv: u32,
    pub max_mv: u32,
    pub __ec_align4: },
//
// Get the currently configured voltage for the voltage regulator.
//
// Note that this might be called before the regulator is enabled, and this
// should return the configured output voltage if the regulator is enabled.
//
pub const EC_CMD_REGULATOR_GET_VOLTAGE: c_uint = 0x0130;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_regulator_get_voltage {
    pub index: u32,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_regulator_get_voltage {
    pub voltage_mv: u32,
    pub __ec_align4: },
//
// Gather all discovery information for the given port and partner type.
//
// Note that if discovery has not yet completed, only the currently completed
// responses will be filled in.   If the discovery data structures are changed
// in the process of the command running, BUSY will be returned.
//
// VDO field sizes are set to the maximum possible number of VDOs a VDM may
// contain, while the number of SVIDs here is selected to fit within the PROTO2
// maximum parameter size.
//
pub const EC_CMD_TYPEC_DISCOVERY: c_uint = 0x0131;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_partner_type {
    TYPEC_PARTNER_SOP = 0,
    TYPEC_PARTNER_SOP_PRIME = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_typec_discovery {
    pub port: u8,
    pub /: *mut *mut uint8_t partner_type; / enum typec_partner_type,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svid_mode_info {
    pub svid: u16,
    pub /: *mut *mut uint16_t mode_count; / Number of modes partner sent,
    pub /: *mut *mut uint32_t mode_vdo[6]; / Max VDOs allowed after VDM header is 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_typec_discovery {
    pub /: *mut *mut uint8_t identity_count; / Number of identity VDOs partner sent,
    pub /: *mut *mut uint8_t svid_count; / Number of SVIDs partner sent,
    pub reserved: u16,
    pub /: *mut *mut uint32_t discovery_vdo[6]; / Max VDOs allowed after VDM header is 6,
    pub svids: [svid_mode_info; ],
    pub __ec_align1: },
// USB Type-C commands for AP-controlled device policy.
pub const EC_CMD_TYPEC_CONTROL: c_uint = 0x0132;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_control_command {
    TYPEC_CONTROL_COMMAND_EXIT_MODES,
    TYPEC_CONTROL_COMMAND_CLEAR_EVENTS,
    TYPEC_CONTROL_COMMAND_ENTER_MODE,
    TYPEC_CONTROL_COMMAND_TBT_UFP_REPLY,
    TYPEC_CONTROL_COMMAND_USB_MUX_SET,
    TYPEC_CONTROL_COMMAND_BIST_SHARE_MODE,
    TYPEC_CONTROL_COMMAND_SEND_VDM_REQ,
}

// Replies the AP may specify to the TBT EnterMode command as a UFP
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum typec_tbt_ufp_reply {
    TYPEC_TBT_UFP_REPLY_NAK,
    TYPEC_TBT_UFP_REPLY_ACK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_usb_mux_set {
    pub /: *mut *mut uint8_t mux_index; / Index of the mux to set in the chain,
    pub /: *mut *mut *mut uint8_t mux_flags; / USB_PD_MUX_-encoded USB mux state to set,
    pub __ec_align1: },
pub const VDO_MAX_SIZE: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_vdm_req {
// VDM data, including VDM header
    pub vdm_data: [u32; VDO_MAX_SIZE],
// Number of 32-bit fields filled in
    pub vdm_data_objects: u8,
// Partner to address - see enum typec_partner_type
    pub partner_type: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_typec_control {
    pub port: u8,
    pub /: *mut *mut uint8_t command; / enum typec_control_command,
    pub reserved: u16,
//
// This section will be interpreted based on |command|. Define a
// placeholder structure to avoid having to increase the size and bump
// the command version when adding new sub-commands.
//
    pub clear_events_mask: u32,
    pub /: *mut *mut uint8_t mode_to_enter; / enum typec_mode,
    pub /: *mut *mut uint8_t tbt_ufp_reply; / enum typec_tbt_ufp_reply,
    pub mux_params: typec_usb_mux_set,
// Used for VMD_REQ
    pub vdm_req_params: typec_vdm_req,
    pub placeholder: [u8; 128],
}

//
// Gather all status information for a port.
//
// Note: this covers many of the return fields from the deprecated
// EC_CMD_USB_PD_CONTROL command, except those that are redundant with the
// discovery data.  The "enum pd_cc_states" is defined with the deprecated
// EC_CMD_USB_PD_CONTROL command.
//
// This also combines in the EC_CMD_USB_PD_MUX_INFO flags.
//
pub const EC_CMD_TYPEC_STATUS: c_uint = 0x0133;
//
// Power role.
//
// Note this is also used for PD header creation, and values align to those in
// the Power Delivery Specification Revision 3.0 (See
// 6.2.1.1.4 Port Power Role).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_power_role {
    PD_ROLE_SINK = 0,
    PD_ROLE_SOURCE = 1
}

//
// Data role.
//
// Note this is also used for PD header creation, and the first two values
// align to those in the Power Delivery Specification Revision 3.0 (See
// 6.2.1.1.6 Port Data Role).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_data_role {
    PD_ROLE_UFP = 0,
    PD_ROLE_DFP = 1,
    PD_ROLE_DISCONNECTED = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_vconn_role {
    PD_ROLE_VCONN_OFF = 0,
    PD_ROLE_VCONN_SRC = 1,
}

//
// Note: BIT(0) may be used to determine whether the polarity is CC1 or CC2,
// regardless of whether a debug accessory is connected.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcpc_cc_polarity {
//
// _CCx: is used to indicate the polarity while not connected to
// a Debug Accessory.  Only one CC line will assert a resistor and
// the other will be open.
//
    POLARITY_CC1 = 0,
    POLARITY_CC2 = 1,

//
// _CCx_DTS is used to indicate the polarity while connected to a
// SRC Debug Accessory.  Assert resistors on both lines.
//
    POLARITY_CC1_DTS = 2,
    POLARITY_CC2_DTS = 3,

//
// The current TCPC code relies on these specific POLARITY values.
// Adding in a check to verify if the list grows for any reason
// that this will give a hint that other places need to be
// adjusted.
//
    POLARITY_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_typec_status {
    pub port: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_typec_status {
    pub /: *mut *mut uint8_t pd_enabled; / PD communication enabled - bool,
    pub /: *mut *mut uint8_t dev_connected; / Device connected - bool,
    pub /: *mut *mut uint8_t sop_connected; / Device is SOP PD capable - bool,
    pub /: *mut *mut uint8_t source_cap_count; / Number of Source Cap PDOs,
    pub /: *mut *mut uint8_t power_role; / enum pd_power_role,
    pub /: *mut *mut uint8_t data_role; / enum pd_data_role,
    pub /: *mut *mut uint8_t vconn_role; / enum pd_vconn_role,
    pub /: *mut *mut uint8_t sink_cap_count; / Number of Sink Cap PDOs,
    pub /: *mut *mut uint8_t polarity; / enum tcpc_cc_polarity,
    pub /: *mut *mut uint8_t cc_state; / enum pd_cc_states,
    pub /: *mut *mut uint8_t dp_pin; / DP pin mode (MODE_DP_IN_[A-E]),
    pub /: *mut *mut *mut uint8_t mux_state; / USB_PD_MUX - encoded mux state,
    pub /: *mut *mut char tc_state[32]; / TC state name,
    pub /: *mut *mut uint32_t events; / PD_STATUS_EVENT bitmask,
//
// BCD PD revisions for partners
//
// The format has the PD major reversion in the upper nibble, and PD
// minor version in the next nibble.  Following two nibbles are
// currently 0.
// ex. PD 3.2 would map to 0x3200
//
// PD major/minor will be 0 if no PD device is connected.
//
    pub sop_revision: u16,
    pub sop_prime_revision: u16,
    pub /: *mut *mut uint32_t source_cap_pdos[7]; / Max 7 PDOs can be present,
    pub /: *mut *mut uint32_t sink_cap_pdos[7]; / Max 7 PDOs can be present,
    pub __ec_align1: },
//
// Gather the response to the most recent VDM REQ from the AP, as well
// as popping the oldest VDM:Attention from the DPM queue
//
pub const EC_CMD_TYPEC_VDM_RESPONSE: c_uint = 0x013C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_typec_vdm_response {
    pub port: u8,
    pub __ec_align1: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_typec_vdm_response {
// Number of 32-bit fields filled in
    pub vdm_data_objects: u8,
// Partner to address - see enum typec_partner_type
    pub partner_type: u8,
// enum ec_status describing VDM response
    pub vdm_response_err: u16,
// VDM data, including VDM header
    pub vdm_response: [u32; VDO_MAX_SIZE],
// Number of 32-bit Attention fields filled in
    pub vdm_attention_objects: u8,
// Number of remaining messages to consume
    pub vdm_attention_left: u8,
// Reserved
    pub reserved1: u16,
// VDM:Attention contents
    pub vdm_attention: [u32; 2],
    pub __ec_align1: },

//
// UCSI OPM-PPM commands
//
// These commands are used for communication between OPM and PPM.
// Only UCSI3.0 is tested.
//
pub const EC_CMD_UCSI_PPM_SET: c_uint = 0x0140;
// The data size is stored in the host command protocol header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_ucsi_ppm_set {
    pub offset: u16,
    pub data: [u8; ],
    pub __ec_align2: },
pub const EC_CMD_UCSI_PPM_GET: c_uint = 0x0141;
// For 'GET' sub-commands, data will be returned as a raw payload.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_ucsi_ppm_get {
    pub offset: u16,
    pub size: u8,
    pub __ec_align2: },
//
// The command range 0x200-0x2FF is reserved for Rotor.
//
// Reserve a range of host commands for the CR51 firmware.
//
pub const EC_CMD_CR51_BASE: c_uint = 0x0300;
pub const EC_CMD_CR51_LAST: c_uint = 0x03FF;
//
// Fingerprint MCU commands: range 0x0400-0x040x
// Fingerprint SPI sensor passthru command: prototyping ONLY
pub const EC_CMD_FP_PASSTHRU: c_uint = 0x0400;
pub const EC_FP_FLAG_NOT_COMPLETE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_fp_passthru {
    pub /: *mut *mut uint16_t len; / Number of bytes to write then read,
    pub /: *mut *mut uint16_t flags; / EC_FP_FLAG_xxx,
    pub /: *mut *mut uint8_t data[]; / Data to send,
    pub __ec_align2: },
// Configure the Fingerprint MCU behavior
pub const EC_CMD_FP_MODE: c_uint = 0x0402;
// Put the sensor in its lowest power mode

// Wait to see a finger on the sensor

// Poll until the finger has left the sensor

// Capture the current finger image

// Finger enrollment session on-going

// Enroll the current finger image

// Try to match the current finger image

// Reset and re-initialize the sensor.

// special value: don't change anything just read back current mode

// Capture types defined in bits [30..28]
pub const FP_MODE_CAPTURE_TYPE_SHIFT: c_int = 28;

//
// This enum must remain ordered, if you add new values you must ensure that
// FP_CAPTURE_TYPE_MAX is still the last one.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fp_capture_type {
// Full blown vendor-defined capture (produces 'frame_size' bytes)
    FP_CAPTURE_VENDOR_FORMAT = 0,
// Simple raw image capture (produces width x height x bpp bits)
    FP_CAPTURE_SIMPLE_IMAGE = 1,
// Self test pattern (e.g. checkerboard)
    FP_CAPTURE_PATTERN0 = 2,
// Self test pattern (e.g. inverted checkerboard)
    FP_CAPTURE_PATTERN1 = 3,
// Capture for Quality test with fixed contrast
    FP_CAPTURE_QUALITY_TEST = 4,
// Capture for pixel reset value test
    FP_CAPTURE_RESET_TEST = 5,
    FP_CAPTURE_TYPE_MAX,
}

// Extracts the capture type from the sensor 'mode' word

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_fp_mode {
    pub /: *mut *mut uint32_t mode; / as defined by FP_MODE_ constants,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_fp_mode {
    pub /: *mut *mut uint32_t mode; / as defined by FP_MODE_ constants,
    pub __ec_align4: },
// Retrieve Fingerprint sensor information
pub const EC_CMD_FP_INFO: c_uint = 0x0403;
// Number of dead pixels detected on the last maintenance

// Unknown number of dead pixels detected on the last maintenance

// No interrupt from the sensor

// SPI communication error

// Invalid sensor Hardware ID

// Sensor initialization failed

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_fp_info_v0 {
// Sensor identification
    pub vendor_id: u32,
    pub product_id: u32,
    pub model_id: u32,
    pub version: u32,
// Image frame characteristics
    pub frame_size: u32,
    pub /: *mut *mut uint32_t pixel_format; / using V4L2_PIX_FMT_,
    pub width: u16,
    pub height: u16,
    pub bpp: u16,
    pub /: *mut *mut uint16_t errors; / see FP_ERROR_ flags above,
    pub __ec_align4: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_fp_info {
// Sensor identification
    pub vendor_id: u32,
    pub product_id: u32,
    pub model_id: u32,
    pub version: u32,
// Image frame characteristics
    pub frame_size: u32,
    pub /: *mut *mut uint32_t pixel_format; / using V4L2_PIX_FMT_,
    pub width: u16,
    pub height: u16,
    pub bpp: u16,
    pub /: *mut *mut uint16_t errors; / see FP_ERROR_ flags above,
// Template/finger current information
    pub /: *mut *mut uint32_t template_size; / max template size in bytes,
    pub /: *mut *mut uint16_t template_max; / maximum number of fingers/templates,
    pub /: *mut *mut uint16_t template_valid; / number of valid fingers/templates,
    pub /: *mut *mut uint32_t template_dirty; / bitmap of templates with MCU side changes,
    pub /: *mut *mut uint32_t template_version; / version of the template format,
    pub __ec_align4: },
// Get the last captured finger frame or a template content
pub const EC_CMD_FP_FRAME: c_uint = 0x0404;
// constants defining the 'offset' field which also contains the frame index
pub const FP_FRAME_INDEX_SHIFT: c_int = 28;
// Frame buffer where the captured image is stored
pub const FP_FRAME_INDEX_RAW_IMAGE: c_int = 0;
// First frame buffer holding a template
pub const FP_FRAME_INDEX_TEMPLATE: c_int = 1;

pub const FP_FRAME_OFFSET_MASK: c_uint = 0x0FFFFFFF;
// Version of the format of the encrypted templates.
pub const FP_TEMPLATE_FORMAT_VERSION: c_int = 3;
// Constants for encryption parameters
pub const FP_CONTEXT_NONCE_BYTES: c_int = 12;

pub const FP_CONTEXT_TAG_BYTES: c_int = 16;
pub const FP_CONTEXT_SALT_BYTES: c_int = 16;
pub const FP_CONTEXT_TPM_BYTES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_fp_template_encryption_metadata {
//
// Version of the structure format (N=3).
//
    pub struct_version: u16,
// Reserved bytes, set to 0.
    pub reserved: u16,
//
// The salt is *only* ever used for key derivation. The nonce is unique,
// a different one is used for every message.
//
    pub nonce: [u8; FP_CONTEXT_NONCE_BYTES],
    pub salt: [u8; FP_CONTEXT_SALT_BYTES],
    pub tag: [u8; FP_CONTEXT_TAG_BYTES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_fp_frame {
//
// The offset contains the template index or FP_FRAME_INDEX_RAW_IMAGE
// in the high nibble, and the real offset within the frame in
// FP_FRAME_OFFSET_MASK.
//
    pub offset: u32,
    pub size: u32,
    pub __ec_align4: },
// Load a template into the MCU
pub const EC_CMD_FP_TEMPLATE: c_uint = 0x0405;
// Flag in the 'size' field indicating that the full template has been sent
pub const FP_TEMPLATE_COMMIT: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_fp_template {
    pub offset: u32,
    pub size: u32,
    pub data: [u8; ],
    pub __ec_align4: },
// Clear the current fingerprint user context and set a new one
pub const EC_CMD_FP_CONTEXT: c_uint = 0x0406;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_fp_context {
    pub userid: [u32; FP_CONTEXT_USERID_WORDS],
    pub __ec_align4: },
pub const EC_CMD_FP_STATS: c_uint = 0x0407;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_fp_stats {
    pub capture_time_us: u32,
    pub matching_time_us: u32,
    pub overall_time_us: u32,
    pub lo: u32,
    pub hi: u32,
    pub overall_t0: },
    pub timestamps_invalid: u8,
    pub template_matched: i8,
    pub __ec_align2: },
pub const EC_CMD_FP_SEED: c_uint = 0x0408;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_fp_seed {
//
// Version of the structure format (N=3).
//
    pub struct_version: u16,
// Reserved bytes, set to 0.
    pub reserved: u16,
// Seed from the TPM.
    pub seed: [u8; FP_CONTEXT_TPM_BYTES],
    pub __ec_align4: },
pub const EC_CMD_FP_ENC_STATUS: c_uint = 0x0409;
// FP TPM seed has been set or not

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_fp_encryption_status {
// Used bits in encryption engine status
    pub valid_flags: u32,
// Encryption engine status
    pub status: u32,
    pub __ec_align4: },
//
// Touchpad MCU commands: range 0x0500-0x05FF
// Perform touchpad self test
pub const EC_CMD_TP_SELF_TEST: c_uint = 0x0500;
// Get number of frame types, and the size of each type
pub const EC_CMD_TP_FRAME_INFO: c_uint = 0x0501;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_tp_frame_info {
    pub n_frames: u32,
    pub frame_sizes: [u32; ],
    pub __ec_align4: },
// Create a snapshot of current frame readings
pub const EC_CMD_TP_FRAME_SNAPSHOT: c_uint = 0x0502;
// Read the frame
pub const EC_CMD_TP_FRAME_GET: c_uint = 0x0503;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_tp_frame_get {
    pub frame_index: u32,
    pub offset: u32,
    pub size: u32,
    pub __ec_align4: },
//
// EC-EC communication commands: range 0x0600-0x06FF
pub const EC_COMM_TEXT_MAX: c_int = 8;
//
// Get battery static information, i.e. information that never changes, or
// very infrequently.
//
pub const EC_CMD_BATTERY_GET_STATIC: c_uint = 0x0600;
//
// struct ec_params_battery_static_info - Battery static info parameters
// @index: Battery index.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_battery_static_info {
    pub index: u8,
    pub __ec_align_size1: },
//
// struct ec_response_battery_static_info - Battery static info response
// @design_capacity: Battery Design Capacity (mAh)
// @design_voltage: Battery Design Voltage (mV)
// @manufacturer: Battery Manufacturer String
// @model: Battery Model Number String
// @serial: Battery Serial Number String
// @type: Battery Type String
// @cycle_count: Battery Cycle Count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_battery_static_info {
    pub design_capacity: u16,
    pub design_voltage: u16,
    pub manufacturer: [c_char; EC_COMM_TEXT_MAX],
    pub model: [c_char; EC_COMM_TEXT_MAX],
    pub serial: [c_char; EC_COMM_TEXT_MAX],
    pub type: [c_char; EC_COMM_TEXT_MAX],
// TODO(crbug.com/795991): Consider moving to dynamic structure.
    pub cycle_count: u32,
    pub __ec_align4: },
//
// Get battery dynamic information, i.e. information that is likely to change
// every time it is read.
//
pub const EC_CMD_BATTERY_GET_DYNAMIC: c_uint = 0x0601;
//
// struct ec_params_battery_dynamic_info - Battery dynamic info parameters
// @index: Battery index.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_battery_dynamic_info {
    pub index: u8,
    pub __ec_align_size1: },
//
// struct ec_response_battery_dynamic_info - Battery dynamic info response
// @actual_voltage: Battery voltage (mV)
// @actual_current: Battery current (mA); negative=discharging
// @remaining_capacity: Remaining capacity (mAh)
// @full_capacity: Capacity (mAh, might change occasionally)
// @flags: Flags, see EC_BATT_FLAG_
// @desired_voltage: Charging voltage desired by battery (mV)
// @desired_current: Charging current desired by battery (mA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_response_battery_dynamic_info {
    pub actual_voltage: i16,
    pub actual_current: i16,
    pub remaining_capacity: i16,
    pub full_capacity: i16,
    pub flags: i16,
    pub desired_voltage: i16,
    pub desired_current: i16,
    pub __ec_align2: },
//
// Control charger chip. Used to control charger chip on the slave.
//
pub const EC_CMD_CHARGER_CONTROL: c_uint = 0x0602;
//
// struct ec_params_charger_control - Charger control parameters
// @max_current: Charger current (mA). Positive to allow base to draw up to
// max_current and (possibly) charge battery, negative to request current
// from base (OTG).
// @otg_voltage: Voltage (mV) to use in OTG mode, ignored if max_current is
// >= 0.
// @allow_charging: Allow base battery charging (only makes sense if
// max_current > 0).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_charger_control {
    pub max_current: i16,
    pub otg_voltage: u16,
    pub allow_charging: u8,
    pub __ec_align_size1: },
// Get ACK from the USB-C SS muxes
pub const EC_CMD_USB_PD_MUX_ACK: c_uint = 0x0603;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_params_usb_pd_mux_ack {
    pub /: *mut *mut uint8_t port; / USB-C port number,
    pub __ec_align1: },
//
// Reserve a range of host commands for board-specific, experimental, or
// special purpose features. These can be (re)used without updating this file.
//
// CAUTION: Don't go nuts with this. Shipping products should document ALL
// their EC commands for easier development, testing, debugging, and support.
//
// All commands MUST be #defined to be 4-digit UPPER CASE hex values
// (e.g., 0x00AB, not 0xab) for CONFIG_HOSTCMD_SECTION_SORTED to work.
//
// In your experimental code, you may want to do something like this:
//
// #define EC_CMD_MAGIC_FOO 0x0000
// #define EC_CMD_MAGIC_BAR 0x0001
// #define EC_CMD_MAGIC_HEY 0x0002
//
// DECLARE_PRIVATE_HOST_COMMAND(EC_CMD_MAGIC_FOO, magic_foo_handler,
// EC_VER_MASK(0);
//
// DECLARE_PRIVATE_HOST_COMMAND(EC_CMD_MAGIC_BAR, magic_bar_handler,
// EC_VER_MASK(0);
//
// DECLARE_PRIVATE_HOST_COMMAND(EC_CMD_MAGIC_HEY, magic_hey_handler,
// EC_VER_MASK(0);
//
pub const EC_CMD_BOARD_SPECIFIC_BASE: c_uint = 0x3E00;
pub const EC_CMD_BOARD_SPECIFIC_LAST: c_uint = 0x3FFF;
//
// Given the private host command offset, calculate the true private host
// command value.
//

//
// Passthru commands
//
// Some platforms have sub-processors chained to each other.  For example.
//
// AP <--> EC <--> PD MCU
//
// The top 2 bits of the command number are used to indicate which device the
// command is intended for.  Device 0 is always the device receiving the
// command; other device mapping is board-specific.
//
// When a device receives a command to be passed to a sub-processor, it passes
// it on with the device number set back to 0.  This allows the sub-processor
// to remain blissfully unaware of whether the command originated on the next
// device up the chain, or was passed through from the AP.
//
// In the above example, if the AP wants to send command 0x0002 to the PD MCU,
// AP sends command 0x4002 to the EC
// EC sends command 0x0002 to the PD MCU
// EC forwards PD MCU response back to the AP
//
// Offset and max command number for sub-device n

//
// Deprecated constants. These constants have been renamed for clarity. The
// meaning and size has not changed. Programs that use the old names should
// switch to the new names soon, as the old names may not be carried forward
// forever.
//

