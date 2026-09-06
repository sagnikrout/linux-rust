//! Automatically rewritten from C to Rust
//! Source: drivers/net/dsa/mv88e6xxx/port_hidden.c
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
// Marvell 88E6xxx Switch Hidden Registers support
//
// Copyright (c) 2008 Marvell Semiconductor
//
// Copyright (c) 2019 Andrew Lunn <andrew@lunn.ch>
//

// The mv88e6390 and mv88e6341 have some hidden registers used for debug and
// development. The errata also makes use of them.
//
    int mv88e6xxx_port_hidden_write(struct mv88e6xxx_chip *chip, int block,
    int port, int reg, u16 val)
    {
    u16 ctrl;
    int err;
    err = mv88e6xxx_port_write(chip, MV88E6XXX_PORT_RESERVED_1A_DATA_PORT,
    MV88E6XXX_PORT_RESERVED_1A, val);
    if (err)
    return err;
    ctrl = MV88E6XXX_PORT_RESERVED_1A_BUSY |
    MV88E6XXX_PORT_RESERVED_1A_WRITE |
    block << MV88E6XXX_PORT_RESERVED_1A_BLOCK_SHIFT |
    port << MV88E6XXX_PORT_RESERVED_1A_PORT_SHIFT |
    reg;
    return mv88e6xxx_port_write(chip, MV88E6XXX_PORT_RESERVED_1A_CTRL_PORT,
    MV88E6XXX_PORT_RESERVED_1A, ctrl);
    }
#[no_mangle]
pub unsafe extern "C" fn mv88e6xxx_port_hidden_wait(chip: *mut mv88e6xxx_chip) -> c_int {
    int mv88e6xxx_port_hidden_wait(struct mv88e6xxx_chip *chip)
    {
    let mut bit: c_int = __bf_shf(MV88E6XXX_PORT_RESERVED_1A_BUSY);
    return mv88e6xxx_port_wait_bit(chip,
    MV88E6XXX_PORT_RESERVED_1A_CTRL_PORT,
    MV88E6XXX_PORT_RESERVED_1A, bit, 0);
    }
    int mv88e6xxx_port_hidden_read(struct mv88e6xxx_chip *chip, int block, int port,
    int reg, u16 *val)
    {
    u16 ctrl;
    int err;
    ctrl = MV88E6XXX_PORT_RESERVED_1A_BUSY |
    MV88E6XXX_PORT_RESERVED_1A_READ |
    block << MV88E6XXX_PORT_RESERVED_1A_BLOCK_SHIFT |
    port << MV88E6XXX_PORT_RESERVED_1A_PORT_SHIFT |
    reg;
    err = mv88e6xxx_port_write(chip, MV88E6XXX_PORT_RESERVED_1A_CTRL_PORT,
    MV88E6XXX_PORT_RESERVED_1A, ctrl);
    if (err)
    return err;
    err = mv88e6xxx_port_hidden_wait(chip);
    if (err)
    return err;
    return mv88e6xxx_port_read(chip, MV88E6XXX_PORT_RESERVED_1A_DATA_PORT,
    MV88E6XXX_PORT_RESERVED_1A, val);
    }
