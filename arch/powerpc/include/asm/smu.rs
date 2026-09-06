//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/smu.h
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
// Definitions for talking to the SMU chip in newer G5 PowerMacs
//

//
// Known SMU commands
//
// Most of what is below comes from looking at the Open Firmware driver,
// though this is still incomplete and could use better documentation here
// or there...
//
// Partition info commands
//
// These commands are used to retrieve the sdb-partition-XX datas from
// the SMU. The length is always 2. First byte is the subcommand code
// and second byte is the partition ID.
//
// The reply is 6 bytes:
//
// - 0..1 : partition address
// - 2    : a byte containing the partition ID
// - 3    : length (maybe other bits are rest of header ?)
//
// The data must then be obtained with calls to another command:
// SMU_CMD_MISC_ee_GET_DATABLOCK_REC (described below).
//
pub const SMU_CMD_PARTITION_COMMAND: c_uint = 0x3e;
pub const SMU_CMD_PARTITION_LATEST: c_uint = 0x01;
pub const SMU_CMD_PARTITION_BASE: c_uint = 0x02;
pub const SMU_CMD_PARTITION_UPDATE: c_uint = 0x03;
//
// Fan control
//
// This is a "mux" for fan control commands. The command seem to
// act differently based on the number of arguments. With 1 byte
// of argument, this seem to be queries for fans status, setpoint,
// etc..., while with 0xe arguments, we will set the fans speeds.
//
// Queries (1 byte arg):
// ---------------------
//
// arg=0x01: read RPM fans status
// arg=0x02: read RPM fans setpoint
// arg=0x11: read PWM fans status
// arg=0x12: read PWM fans setpoint
//
// the "status" queries return the current speed while the "setpoint" ones
// return the programmed/target speed. It _seems_ that the result is a bit
// mask in the first byte of active/available fans, followed by 6 words (16
// bits) containing the requested speed.
//
// Setpoint (14 bytes arg):
// ------------------------
//
// first arg byte is 0 for RPM fans and 0x10 for PWM. Second arg byte is the
// mask of fans affected by the command. Followed by 6 words containing the
// setpoint value for selected fans in the mask (or 0 if mask value is 0)
//
pub const SMU_CMD_FAN_COMMAND: c_uint = 0x4a;
//
// Battery access
//
// Same command number as the PMU, could it be same syntax ?
//
pub const SMU_CMD_BATTERY_COMMAND: c_uint = 0x6f;
pub const SMU_CMD_GET_BATTERY_INFO: c_uint = 0x00;
//
// Real time clock control
//
// This is a "mux", first data byte contains the "sub" command.
// The "RTC" part of the SMU controls the date, time, powerup
// timer, but also a PRAM
//
// Dates are in BCD format on 7 bytes:
// [sec] [min] [hour] [weekday] [month day] [month] [year]
// with month being 1 based and year minus 100
//
pub const SMU_CMD_RTC_COMMAND: c_uint = 0x8e;
pub const SMU_CMD_RTC_SET_PWRUP_TIMER: c_uint = 0x00 /* i: 7 bytes date */;
pub const SMU_CMD_RTC_GET_PWRUP_TIMER: c_uint = 0x01 /* o: 7 bytes date */;
pub const SMU_CMD_RTC_STOP_PWRUP_TIMER: c_uint = 0x02;
pub const SMU_CMD_RTC_SET_PRAM_BYTE_ACC: c_uint = 0x20 /* i: 1 byte (address?) */;
pub const SMU_CMD_RTC_SET_PRAM_AUTOINC: c_uint = 0x21 /* i: 1 byte (data?) */;
pub const SMU_CMD_RTC_SET_PRAM_LO_BYTES: c_uint = 0x22 /* i: 10 bytes */;
pub const SMU_CMD_RTC_SET_PRAM_HI_BYTES: c_uint = 0x23 /* i: 10 bytes */;
pub const SMU_CMD_RTC_GET_PRAM_BYTE: c_uint = 0x28 /* i: 1 bytes (address?) */;
pub const SMU_CMD_RTC_GET_PRAM_LO_BYTES: c_uint = 0x29 /* o: 10 bytes */;
pub const SMU_CMD_RTC_GET_PRAM_HI_BYTES: c_uint = 0x2a /* o: 10 bytes */;
pub const SMU_CMD_RTC_SET_DATETIME: c_uint = 0x80 /* i: 7 bytes date */;
pub const SMU_CMD_RTC_GET_DATETIME: c_uint = 0x81 /* o: 7 bytes date */;
//
// i2c commands
//
// To issue an i2c command, first is to send a parameter block to
// the SMU. This is a command of type 0x9a with 9 bytes of header
// eventually followed by data for a write:
//
// 0: bus number (from device-tree usually, SMU has lots of busses !)
// 1: transfer type/format (see below)
// 2: device address. For combined and combined4 type transfers, this
// is the "write" version of the address (bit 0x01 cleared)
// 3: subaddress length (0..3)
// 4: subaddress byte 0 (or only byte for subaddress length 1)
// 5: subaddress byte 1
// 6: subaddress byte 2
// 7: combined address (device address for combined mode data phase)
// 8: data length
//
// The transfer types are the same good old Apple ones it seems,
// that is:
// - 0x00: Simple transfer
// - 0x01: Subaddress transfer (addr write + data tx, no restart)
// - 0x02: Combined transfer (addr write + restart + data tx)
//
// This is then followed by actual data for a write.
//
// At this point, the OF driver seems to have a limitation on transfer
// sizes of 0xd bytes on reads and 0x5 bytes on writes. I do not know
// whether this is just an OF limit due to some temporary buffer size
// or if this is an SMU imposed limit. This driver has the same limitation
// for now as I use a 0x10 bytes temporary buffer as well
//
// Once that is completed, a response is expected from the SMU. This is
// obtained via a command of type 0x9a with a length of 1 byte containing
// 0 as the data byte. OF also fills the rest of the data buffer with 0xff's
// though I can't tell yet if this is actually necessary. Once this command
// is complete, at this point, all I can tell is what OF does. OF tests
// byte 0 of the reply:
// - on read, 0xfe or 0xfc : bus is busy, wait (see below) or nak ?
// - on read, 0x00 or 0x01 : reply is in buffer (after the byte 0)
// - on write, < 0 -> failure (immediate exit)
// - else, OF just exists (without error, weird)
//
// So on read, there is this wait-for-busy thing when getting a 0xfc or
// 0xfe result. OF does a loop of up to 64 retries, waiting 20ms and
// doing the above again until either the retries expire or the result
// is no longer 0xfe or 0xfc
//
// The Darwin I2C driver is less subtle though. On any non-success status
// from the response command, it waits 5ms and tries again up to 20 times,
// it doesn't differentiate between fatal errors or "busy" status.
//
// This driver provides an asynchronous paramblock based i2c command
// interface to be used either directly by low level code or by a higher
// level driver interfacing to the linux i2c layer. The current
// implementation of this relies on working timers & timer interrupts
// though, so be careful of calling context for now. This may be "fixed"
// in the future by adding a polling facility.
//
pub const SMU_CMD_I2C_COMMAND: c_uint = 0x9a;
// transfer types
pub const SMU_I2C_TRANSFER_SIMPLE: c_uint = 0x00;
pub const SMU_I2C_TRANSFER_STDSUB: c_uint = 0x01;
pub const SMU_I2C_TRANSFER_COMBINED: c_uint = 0x02;
//
// Power supply control
//
// The "sub" command is an ASCII string in the data, the
// data length is that of the string.
//
// The VSLEW command can be used to get or set the voltage slewing.
// - length 5 (only "VSLEW") : it returns "DONE" and 3 bytes of
// reply at data offset 6, 7 and 8.
// - length 8 ("VSLEWxyz") has 3 additional bytes appended, and is
// used to set the voltage slewing point. The SMU replies with "DONE"
// I yet have to figure out their exact meaning of those 3 bytes in
// both cases. They seem to be:
// x = processor mask
// y = op. point index
// z = processor freq. step index
// I haven't yet deciphered result codes
//
pub const SMU_CMD_POWER_COMMAND: c_uint = 0xaa;

//
// Read ADC sensors
//
// This command takes one byte of parameter: the sensor ID (or "reg"
// value in the device-tree) and returns a 16 bits value
//
pub const SMU_CMD_READ_ADC: c_uint = 0xd8;
// Misc commands
//
// This command seem to be a grab bag of various things
//
// Parameters:
// 1: subcommand
//
pub const SMU_CMD_MISC_df_COMMAND: c_uint = 0xdf;
//
// Sets "system ready" status
//
// I did not yet understand how it exactly works or what it does.
//
// Guessing from OF code, 0x02 activates the display backlight. Apple uses/used
// the same codebase for all OF versions. On PowerBooks, this command would
// enable the backlight. For the G5s, it only activates the front LED. However,
// don't take this for granted.
//
// Parameters:
// 2: status [0x00, 0x01 or 0x02]
//
pub const SMU_CMD_MISC_df_SET_DISPLAY_LIT: c_uint = 0x02;
//
// Sets mode of power switch.
//
// What this actually does is not yet known. Maybe it enables some interrupt.
//
// Parameters:
// 2: enable power switch? [0x00 or 0x01]
// 3 (optional): enable nmi? [0x00 or 0x01]
//
// Returns:
// If parameter 2 is 0x00 and parameter 3 is not specified, returns whether
// NMI is enabled. Otherwise unknown.
//
pub const SMU_CMD_MISC_df_NMI_OPTION: c_uint = 0x04;
// Sets LED dimm offset.
//
// The front LED dimms itself during sleep. Its brightness (or, well, the PWM
// frequency) depends on current time. Therefore, the SMU needs to know the
// timezone.
//
// Parameters:
// 2-8: unknown (BCD coding)
//
pub const SMU_CMD_MISC_df_DIMM_OFFSET: c_uint = 0x99;
//
// Version info commands
//
// Parameters:
// 1 (optional): Specifies version part to retrieve
//
// Returns:
// Version value
//
pub const SMU_CMD_VERSION_COMMAND: c_uint = 0xea;
pub const SMU_VERSION_RUNNING: c_uint = 0x00;
pub const SMU_VERSION_BASE: c_uint = 0x01;
pub const SMU_VERSION_UPDATE: c_uint = 0x02;
//
// Switches
//
// These are switches whose status seems to be known to the SMU.
//
// Parameters:
// none
//
// Result:
// Switch bits (ORed, see below)
//
pub const SMU_CMD_SWITCHES: c_uint = 0xdc;
// Switches bits
pub const SMU_SWITCH_CASE_CLOSED: c_uint = 0x01;
pub const SMU_SWITCH_AC_POWER: c_uint = 0x04;
pub const SMU_SWITCH_POWER_SWITCH: c_uint = 0x08;
//
// Misc commands
//
// This command seem to be a grab bag of various things
//
// SMU_CMD_MISC_ee_GET_DATABLOCK_REC is used, among others, to
// transfer blocks of data from the SMU. So far, I've decrypted it's
// usage to retrieve partition data. In order to do that, you have to
// break your transfer in "chunks" since that command cannot transfer
// more than a chunk at a time. The chunk size used by OF is 0xe bytes,
// but it seems that the darwin driver will let you do 0x1e bytes if
// your "PMU" version is >= 0x30. You can get the "PMU" version apparently
// either in the last 16 bits of property "smu-version-pmu" or as the 16
// bytes at offset 1 of "smu-version-info"
//
// For each chunk, the command takes 7 bytes of arguments:
// byte 0: subcommand code (0x02)
// byte 1: 0x04 (always, I don't know what it means, maybe the address
// space to use or some other nicety. It's hard coded in OF)
// byte 2..5: SMU address of the chunk (big endian 32 bits)
// byte 6: size to transfer (up to max chunk size)
//
// The data is returned directly
//
pub const SMU_CMD_MISC_ee_COMMAND: c_uint = 0xee;
pub const SMU_CMD_MISC_ee_GET_DATABLOCK_REC: c_uint = 0x02;
// Retrieves currently used watts.
//
// Parameters:
// 1: 0x03 (Meaning unknown)
//
pub const SMU_CMD_MISC_ee_GET_WATTS: c_uint = 0x03;
pub const SMU_CMD_MISC_ee_LEDS_CTRL: c_uint = 0x04 /* i: 00 (00,01) [00] */;
pub const SMU_CMD_MISC_ee_GET_DATA: c_uint = 0x05 /* i: 00 , o: ?? */;
//
// Power related commands
//
// Parameters:
// 1: subcommand
//
pub const SMU_CMD_POWER_EVENTS_COMMAND: c_uint = 0x8f;
// SMU_POWER_EVENTS subcommands
//
// Get last shutdown cause
//
// Returns:
// 1 byte (signed char): Last shutdown cause. Exact meaning unknown.
//
// Sets or gets server ID. Meaning or use is unknown.
//
// Parameters:
// 2 (optional): Set server ID (1 byte)
//
// Returns:
// 1 byte (server ID?)
//
// Power events wakeup bits
//
// - Kernel side interface -
//

//
// Asynchronous SMU commands
//
// Fill up this structure and submit it via smu_queue_command(),
// and get notified by the optional done() callback, or because
// status becomes != 1
//
// public
// private
//
// Queues an SMU command, all fields have to be initialized
//
extern "C" {
    pub fn smu_queue_cmd(cmd: *mut smu_cmd) -> c_int;
}
//
// Simple command wrapper. This structure embeds a small buffer
// to ease sending simple SMU commands from the stack
//
// Queues a simple command. All fields will be initialized by that
// function
//
// Completion helper. Pass it to smu_queue_simple or as 'done'
// member to smu_queue_cmd, it will call complete() on the struct
// completion passed in the "misc" argument
//
extern "C" {
    pub fn smu_done_complete(cmd: *mut smu_cmd, misc: *mut c_void);
}
//
// Synchronous helpers. Will spin-wait for completion of a command
//
extern "C" {
    pub fn smu_spinwait_cmd(cmd: *mut smu_cmd);
}
//
// Poll routine to call if blocked with irqs off
//
extern "C" {
    pub fn smu_poll();
}
//
// Init routine, presence check....
//
extern "C" {
    pub fn smu_init() -> int __init;
}
extern "C" {
    pub fn smu_present() -> c_int;
}
//
// Common command wrappers
//
extern "C" {
    pub fn smu_shutdown();
}
extern "C" {
    pub fn smu_restart();
}
extern "C" {
    pub fn smu_get_rtc_time(time: *mut rtc_time, spinwait: c_int) -> c_int;
}
extern "C" {
    pub fn smu_set_rtc_time(time: *mut rtc_time, spinwait: c_int) -> c_int;
}
//
// Kernel asynchronous i2c interface
//
pub const SMU_I2C_READ_MAX: c_uint = 0x1d;
pub const SMU_I2C_WRITE_MAX: c_uint = 0x15;
// SMU i2c header, exactly matches i2c header on wire
// public
// private
//
// Call this to queue an i2c command to the SMU. You must fill info,
// including info.data for a write, done and misc.
// For now, no polling interface is provided so you have to use completion
// callback.
//
extern "C" {
    pub fn smu_queue_i2c(cmd: *mut smu_i2c_cmd) -> c_int;
}

//
// - SMU "sdb" partitions informations -
//
// Partition header format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_sdbp_header {
    pub id: __u8,
    pub len: __u8,
    pub version: __u8,
    pub flags: __u8,
}

//
// demangle 16 and 32 bits integer in some SMU partitions
// (currently, afaik, this concerns only the FVT partition
// (0x12)
//

// This is the definition of the SMU sdb-partition-0x12 table (called
// CPU F/V/T operating points in Darwin). The definition for all those
// SMU tables should be moved to some separate file
//
pub const SMU_SDB_FVT_ID: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_sdbp_fvt {
    pub for: *mut *mut __u32 sysclk; / Base SysClk frequency in Hz,
// this operating point. Value need to
// be unmixed with SMU_U32_MIX()
//
    pub pad: __u8,
    pub this: *mut *mut __u8 maxtemp; / Max temp. supported by,
// operating point
//
    pub 3: *mut *mut __u16 volts[3]; / CPU core voltage for the,
// PowerTune modes, a mode with
// 0V = not supported. Value need
// to be unmixed with SMU_U16_MIX()
//
}

// This partition contains voltage & current sensor calibration
// informations
//
pub const SMU_SDB_CPUVCP_ID: c_uint = 0x21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_sdbp_cpuvcp {
    pub /: *mut *mut __u16 volt_scale; / u4.12 fixed point,
    pub /: *mut *mut __s16 volt_offset; / s4.12 fixed point,
    pub /: *mut *mut __u16 curr_scale; / u4.12 fixed point,
    pub /: *mut *mut __s16 curr_offset; / s4.12 fixed point,
    pub /: *mut *mut __s32 power_quads[3]; / s4.28 fixed point,
}

// This partition contains CPU thermal diode calibration
//
pub const SMU_SDB_CPUDIODE_ID: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_sdbp_cpudiode {
    pub /: *mut *mut __u16 m_value; / u1.15 fixed point,
    pub /: *mut *mut __s16 b_value; / s10.6 fixed point,
}

// This partition contains Slots power calibration
//
pub const SMU_SDB_SLOTSPOW_ID: c_uint = 0x78;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_sdbp_slotspow {
    pub /: *mut *mut __u16 pow_scale; / u4.12 fixed point,
    pub /: *mut *mut __s16 pow_offset; / s4.12 fixed point,
}

// This partition contains machine specific version information about
// the sensor/control layout
//
pub const SMU_SDB_SENSORTREE_ID: c_uint = 0x25;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_sdbp_sensortree {
    pub model_id: __u8,
    pub unknown: [__u8; 3],
}

// This partition contains CPU thermal control PID informations. So far
// only single CPU machines have been seen with an SMU, so we assume this
// carries only informations for those
//
pub const SMU_SDB_CPUPIDDATA_ID: c_uint = 0x17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smu_sdbp_cpupiddata {
    pub unknown1: __u8,
    pub target_temp_delta: __u8,
    pub unknown2: __u8,
    pub history_len: __u8,
    pub power_adj: __s16,
    pub max_power: __u16,
    pub gp,gr,gd: __s32,
}

// Other partitions without known structures
pub const SMU_SDB_DEBUG_SWITCHES_ID: c_uint = 0x05;

//
// This returns the pointer to an SMU "sdb" partition data or NULL
// if not found. The data format is described below
//
// Get "sdb" partition data from an SMU satellite

//
// - Userland interface -
//
// A given instance of the device can be configured for 2 different
// things at the moment:
//
// - sending SMU commands (default at open() time)
// - receiving SMU events (not yet implemented)
//
// Commands are written with write() of a command block. They can be
// "driver" commands (for example to switch to event reception mode)
// or real SMU commands. They are made of a header followed by command
// data if any.
//
// For SMU commands (not for driver commands), you can then read() back
// a reply. The reader will be blocked or not depending on how the device
// file is opened. poll() isn't implemented yet. The reply will consist
// of a header as well, followed by the reply data if any. You should
// always provide a buffer large enough for the maximum reply data, I
// recommand one page.
//
// It is illegal to send SMU commands through a file descriptor configured
// for events reception
//

