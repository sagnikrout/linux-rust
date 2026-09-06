//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/io_delay.c
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
// I/O delay strategies for inb_p/outb_p
//
// Allow for a DMI based override of port 0x80, needed for certain HP laptops
// and possibly other systems. Also allow for the gradual elimination of
// outb_p/inb_p API uses.
//

pub const IO_DELAY_TYPE_0X80: c_int = 0;
pub const IO_DELAY_TYPE_0XED: c_int = 1;
pub const IO_DELAY_TYPE_UDELAY: c_int = 2;
pub const IO_DELAY_TYPE_NONE: c_int = 3;

    let mut __read_mostly: int io_delay_type = DEFAULT_IO_DELAY_TYPE;
    static int __initdata io_delay_override;
//
// Paravirt wants native_io_delay to be a constant.
//
#[no_mangle]
pub unsafe extern "C" fn native_io_delay() {
    void native_io_delay(void)
    {
    switch (io_delay_type) {
    default:
    case IO_DELAY_TYPE_0X80:
    asm volatile ("outb %al, $0x80");
    break;
    case IO_DELAY_TYPE_0XED:
    asm volatile ("outb %al, $0xed");
    break;
    case IO_DELAY_TYPE_UDELAY:
//
// 2 usecs is an upper-bound for the outb delay but
// note that udelay doesn't have the bus-level
// side-effects that outb does, nor does udelay() have
// precise timings during very early bootup (the delays
// are shorter until calibrated):
//
    udelay(2);
    break;
    case IO_DELAY_TYPE_NONE:
    break;
    }
    }
    EXPORT_SYMBOL(native_io_delay);
#[no_mangle]
unsafe extern "C" fn dmi_io_delay_0xed_port(id: *const dmi_system_id) -> int __init {
    static int __init dmi_io_delay_0xed_port(const struct dmi_system_id *id)
    {
    if (io_delay_type == IO_DELAY_TYPE_0X80) {
    pr_notice("%s: using 0xed I/O delay port\n", id.ident);
    io_delay_type = IO_DELAY_TYPE_0XED;
    }
    return 0;
    }
//
// Quirk table for systems that misbehave (lock up, etc.) if port
// 0x80 is used:
//
    static const struct dmi_system_id io_delay_0xed_port_dmi_table[] __initconst = {
    {
    .callback	= dmi_io_delay_0xed_port,
    .ident		= "Compaq Presario V6000",
    .matches	= {
    DMI_MATCH(DMI_BOARD_VENDOR,	"Quanta"),
    DMI_MATCH(DMI_BOARD_NAME,	"30B7")
    }
    },
    {
    .callback	= dmi_io_delay_0xed_port,
    .ident		= "HP Pavilion dv9000z",
    .matches	= {
    DMI_MATCH(DMI_BOARD_VENDOR,	"Quanta"),
    DMI_MATCH(DMI_BOARD_NAME,	"30B9")
    }
    },
    {
    .callback	= dmi_io_delay_0xed_port,
    .ident		= "HP Pavilion dv6000",
    .matches	= {
    DMI_MATCH(DMI_BOARD_VENDOR,	"Quanta"),
    DMI_MATCH(DMI_BOARD_NAME,	"30B8")
    }
    },
    {
    .callback	= dmi_io_delay_0xed_port,
    .ident		= "HP Pavilion tx1000",
    .matches	= {
    DMI_MATCH(DMI_BOARD_VENDOR,	"Quanta"),
    DMI_MATCH(DMI_BOARD_NAME,	"30BF")
    }
    },
    {
    .callback	= dmi_io_delay_0xed_port,
    .ident		= "Presario F700",
    .matches	= {
    DMI_MATCH(DMI_BOARD_VENDOR,	"Quanta"),
    DMI_MATCH(DMI_BOARD_NAME,	"30D3")
    }
    },
    { }
    };
#[no_mangle]
pub unsafe extern "C" fn io_delay_init() -> void __init {
    void __init io_delay_init(void)
    {
    if (!io_delay_override)
    dmi_check_system(io_delay_0xed_port_dmi_table);
    }
#[no_mangle]
unsafe extern "C" fn io_delay_param(s: *mut c_char) -> int __init {
    static int __init io_delay_param(char *s)
    {
    if (!s)
    return -EINVAL;
    if (!strcmp(s, "0x80"))
    io_delay_type = IO_DELAY_TYPE_0X80;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(s, _arg: "0xed")) -> else {
    else if (!strcmp(s, "0xed"))
    io_delay_type = IO_DELAY_TYPE_0XED;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(s, _arg: "udelay")) -> else {
    else if (!strcmp(s, "udelay"))
    io_delay_type = IO_DELAY_TYPE_UDELAY;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(s, _arg: "none")) -> else {
    else if (!strcmp(s, "none"))
    io_delay_type = IO_DELAY_TYPE_NONE;
    else
    return -EINVAL;
    io_delay_override = 1;
    return 0;
    }
    early_param("io_delay", io_delay_param);
