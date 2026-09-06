//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pmu.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Definitions for talking to the PMU.  The PMU is a microcontroller
// which controls battery charging and system power on PowerBook 3400
// and 2400 models as well as the RTC and various other things.
//
// Copyright (C) 1998 Paul Mackerras.
//
pub const PMU_DRIVER_VERSION: c_int = 2;
//
// PMU commands
//
pub const PMU_POWER_CTRL0: c_uint = 0x10	/* control power of some devices */;
pub const PMU_POWER_CTRL: c_uint = 0x11	/* control power of some devices */;
pub const PMU_ADB_CMD: c_uint = 0x20	/* send ADB packet */;
pub const PMU_ADB_POLL_OFF: c_uint = 0x21	/* disable ADB auto-poll */;
pub const PMU_WRITE_XPRAM: c_uint = 0x32	/* write eXtended Parameter RAM */;
pub const PMU_WRITE_NVRAM: c_uint = 0x33	/* write non-volatile RAM */;
pub const PMU_READ_XPRAM: c_uint = 0x3a	/* read eXtended Parameter RAM */;
pub const PMU_READ_NVRAM: c_uint = 0x3b	/* read non-volatile RAM */;
pub const PMU_SET_RTC: c_uint = 0x30	/* set real-time clock */;
pub const PMU_READ_RTC: c_uint = 0x38	/* read real-time clock */;
pub const PMU_SET_VOLBUTTON: c_uint = 0x40	/* set volume up/down position */;
pub const PMU_BACKLIGHT_BRIGHT: c_uint = 0x41	/* set backlight brightness */;
pub const PMU_GET_VOLBUTTON: c_uint = 0x48	/* get volume up/down position */;
pub const PMU_PCEJECT: c_uint = 0x4c	/* eject PC-card from slot */;
pub const PMU_BATTERY_STATE: c_uint = 0x6b	/* report battery state etc. */;
pub const PMU_SMART_BATTERY_STATE: c_uint = 0x6f	/* report battery state (new way) */;
pub const PMU_SET_INTR_MASK: c_uint = 0x70	/* set PMU interrupt mask */;
pub const PMU_INT_ACK: c_uint = 0x78	/* read interrupt bits */;
pub const PMU_SHUTDOWN: c_uint = 0x7e	/* turn power off */;
pub const PMU_CPU_SPEED: c_uint = 0x7d	/* control CPU speed on some models */;
pub const PMU_SLEEP: c_uint = 0x7f	/* put CPU to sleep */;
pub const PMU_POWER_EVENTS: c_uint = 0x8f	/* Send power-event commands to PMU */;
pub const PMU_I2C_CMD: c_uint = 0x9a	/* I2C operations */;
pub const PMU_RESET: c_uint = 0xd0	/* reset CPU */;
pub const PMU_GET_BRIGHTBUTTON: c_uint = 0xd9	/* report brightness up/down pos */;
pub const PMU_GET_COVER: c_uint = 0xdc	/* report cover open/closed */;
pub const PMU_SYSTEM_READY: c_uint = 0xdf	/* tell PMU we are awake */;
pub const PMU_GET_VERSION: c_uint = 0xea	/* read the PMU version */;
// Bits to use with the PMU_POWER_CTRL0 command
pub const PMU_POW0_ON: c_uint = 0x80	/* OR this to power ON the device */;
pub const PMU_POW0_OFF: c_uint = 0x00	/* leave bit 7 to 0 to power it OFF */;
pub const PMU_POW0_HARD_DRIVE: c_uint = 0x04	/* Hard drive power (on wallstreet/lombard ?) */;
// Bits to use with the PMU_POWER_CTRL command
pub const PMU_POW_ON: c_uint = 0x80	/* OR this to power ON the device */;
pub const PMU_POW_OFF: c_uint = 0x00	/* leave bit 7 to 0 to power it OFF */;
pub const PMU_POW_BACKLIGHT: c_uint = 0x01	/* backlight power */;
pub const PMU_POW_CHARGER: c_uint = 0x02	/* battery charger power */;
pub const PMU_POW_IRLED: c_uint = 0x04	/* IR led power (on wallstreet) */;
pub const PMU_POW_MEDIABAY: c_uint = 0x08	/* media bay power (wallstreet/lombard ?) */;
// Bits in PMU interrupt and interrupt mask bytes
pub const PMU_INT_PCEJECT: c_uint = 0x04	/* PC-card eject buttons */;
pub const PMU_INT_SNDBRT: c_uint = 0x08	/* sound/brightness up/down buttons */;
pub const PMU_INT_ADB: c_uint = 0x10	/* ADB autopoll or reply data */;
pub const PMU_INT_BATTERY: c_uint = 0x20	/* Battery state change */;
pub const PMU_INT_ENVIRONMENT: c_uint = 0x40	/* Environment interrupts */;
pub const PMU_INT_TICK: c_uint = 0x80	/* 1-second tick interrupt */;
// Other bits in PMU interrupt valid when PMU_INT_ADB is set
pub const PMU_INT_ADB_AUTO: c_uint = 0x04	/* ADB autopoll, when PMU_INT_ADB */;
pub const PMU_INT_WAITING_CHARGER: c_uint = 0x01	/* ??? */;
pub const PMU_INT_AUTO_SRQ_POLL: c_uint = 0x02	/* ??? */;
// Bits in the environement message (either obtained via PMU_GET_COVER,
// or via PMU_INT_ENVIRONMENT on core99
pub const PMU_ENV_LID_CLOSED: c_uint = 0x01	/* The lid is closed */;
// I2C related definitions
pub const PMU_I2C_MODE_SIMPLE: c_int = 0;
pub const PMU_I2C_MODE_STDSUB: c_int = 1;
pub const PMU_I2C_MODE_COMBINED: c_int = 2;
pub const PMU_I2C_BUS_STATUS: c_int = 0;
pub const PMU_I2C_BUS_SYSCLK: c_int = 1;
pub const PMU_I2C_BUS_POWER: c_int = 2;
pub const PMU_I2C_STATUS_OK: c_int = 0;
pub const PMU_I2C_STATUS_DATAREAD: c_int = 1;
pub const PMU_I2C_STATUS_BUSY: c_uint = 0xfe;
// Kind of PMU (model)
// PMU PMU_POWER_EVENTS commands
// Power events wakeup bits
//
// Ioctl commands for the /dev/pmu device
//

// no param

// out param: u32*	backlight value: 0 to 15

// in param: u32	backlight value: 0 to 15

// out param: u32*	PMU model

// out param: u32*	has_adb: 0 or 1

// out param: u32*	can_sleep: 0 or 1

// no param, but historically was _IOR('B', 6, 0), meaning 4 bytes

