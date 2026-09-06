//! Automatically rewritten from C to Rust
//! Source: drivers/tty/tty_baudrate.c
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
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//

//
// Routine which returns the baud rate of the tty
//
// Note that the baud_table needs to be kept in sync with the
// include/asm/termbits.h file.
//
    static const speed_t baud_table[] = {
    0, 50, 75, 110, 134, 150, 200, 300, 600, 1200, 1800, 2400,
    4800, 9600, 19200, 38400, 57600, 115200, 230400, 460800,

    76800, 153600, 307200, 614400, 921600, 500000, 576000,
    1000000, 1152000, 1500000, 2000000

    500000, 576000, 921600, 1000000, 1152000, 1500000, 2000000,
    2500000, 3000000, 3500000, 4000000

    };
    static const tcflag_t baud_bits[] = {
    B0, B50, B75, B110, B134, B150, B200, B300, B600, B1200, B1800, B2400,
    B4800, B9600, B19200, B38400, B57600, B115200, B230400, B460800,

    B76800, B153600, B307200, B614400, B921600, B500000, B576000,
    B1000000, B1152000, B1500000, B2000000

    B500000, B576000, B921600, B1000000, B1152000, B1500000, B2000000,
    B2500000, B3000000, B3500000, B4000000

    };
    let mut n_baud_table: static int = ARRAY_SIZE(baud_table);
//
// tty_termios_baud_rate
// @termios: termios structure
//
// Convert termios baud rate data into a speed. This should be called
// with the termios lock held if this termios is a terminal termios
// structure. Device drivers can call this function but should use
// ->c_[io]speed directly as they are updated.
//
// Locking: none
//
#[no_mangle]
pub unsafe extern "C" fn tty_termios_baud_rate(termios: *const ktermios) -> speed_t {
    speed_t tty_termios_baud_rate(const struct ktermios *termios)
    {
    unsigned int cbaud;
    cbaud = termios.c_cflag & CBAUD;
// Magic token for arbitrary speed via c_ispeed/c_ospeed
    if (cbaud == BOTHER)
    return termios.c_ospeed;
    if (cbaud & CBAUDEX) {
    cbaud &= ~CBAUDEX;
    cbaud += 15;
    }
    return cbaud >= n_baud_table ? 0 : baud_table[cbaud];
    }
    EXPORT_SYMBOL(tty_termios_baud_rate);
//
// tty_termios_input_baud_rate
// @termios: termios structure
//
// Convert termios baud rate data into a speed. This should be called
// with the termios lock held if this termios is a terminal termios
// structure. Device drivers can call this function but should use
// ->c_[io]speed directly as they are updated.
//
// Locking: none
//
#[no_mangle]
pub unsafe extern "C" fn tty_termios_input_baud_rate(termios: *const ktermios) -> speed_t {
    speed_t tty_termios_input_baud_rate(const struct ktermios *termios)
    {
    let mut cbaud: c_uint = (termios.c_cflag >> IBSHIFT) & CBAUD;
    if (cbaud == B0)
    return tty_termios_baud_rate(termios);
// Magic token for arbitrary speed via c_ispeed
    if (cbaud == BOTHER)
    return termios.c_ispeed;
    if (cbaud & CBAUDEX) {
    cbaud &= ~CBAUDEX;
    cbaud += 15;
    }
    return cbaud >= n_baud_table ? 0 : baud_table[cbaud];
    }
    EXPORT_SYMBOL(tty_termios_input_baud_rate);
//
// tty_termios_encode_baud_rate
// @termios: ktermios structure holding user requested state
// @ibaud: input speed
// @obaud: output speed
//
// Encode the speeds set into the passed termios structure. This is
// used as a library helper for drivers so that they can report back
// the actual speed selected when it differs from the speed requested
//
// For maximal back compatibility with legacy SYS5/POSIX *nix behaviour
// we need to carefully set the bits when the user does not get the
// desired speed. We allow small margins and preserve as much of possible
// of the input intent to keep compatibility.
//
// Locking: Caller should hold termios lock. This is already held
// when calling this function from the driver termios handler.
//
// The ifdefs deal with platforms whose owners have yet to update them
// and will all go away once this is done.
//
    void tty_termios_encode_baud_rate(struct ktermios *termios,
    speed_t ibaud, speed_t obaud)
    {
    let mut i: c_int = 0;
    let mut ifound: c_int = -1, ofound = -1;
    let mut iclose: c_int = ibaud/50, oclose = obaud/50;
    let mut ibinput: c_int = 0;
    if (obaud == 0)			/* CD dropped */
    ibaud = 0;		/* Clear ibaud to be sure */
    termios.c_ispeed = ibaud;
    termios.c_ospeed = obaud;
    if (((termios.c_cflag >> IBSHIFT) & CBAUD) != B0)
    ibinput = 1;	/* An input speed was specified */
// If the user asked for a precise weird speed give a precise weird
// answer. If they asked for a Bfoo speed they may have problems
// digesting non-exact replies so fuzz a bit.
//
    if ((termios.c_cflag & CBAUD) == BOTHER) {
    oclose = 0;
    if (!ibinput)
    iclose = 0;
    }
    if (((termios.c_cflag >> IBSHIFT) & CBAUD) == BOTHER)
    iclose = 0;
    termios.c_cflag &= ~CBAUD;
    termios.c_cflag &= ~(CBAUD << IBSHIFT);
//
// Our goal is to find a close match to the standard baud rate
// returned. Walk the baud rate table and if we get a very close
// match then report back the speed as a POSIX Bxxxx value by
// preference
//
    do {
    if (obaud - oclose <= baud_table[i] &&
    obaud + oclose >= baud_table[i]) {
    termios.c_cflag |= baud_bits[i];
    ofound = i;
    }
    if (ibaud - iclose <= baud_table[i] &&
    ibaud + iclose >= baud_table[i]) {
// For the case input == output don't set IBAUD bits
// if the user didn't do so.
//
    if (ofound == i && !ibinput) {
    ifound  = i;
    } else {
    ifound = i;
    termios.c_cflag |= (baud_bits[i] << IBSHIFT);
    }
    }
    } while (++i < n_baud_table);
// If we found no match then use BOTHER.
    if (ofound == -1)
    termios.c_cflag |= BOTHER;
// Set exact input bits only if the input and output differ or the
// user already did.
//
    if (ifound == -1 && (ibaud != obaud || ibinput))
    termios.c_cflag |= (BOTHER << IBSHIFT);
    }
    EXPORT_SYMBOL_GPL(tty_termios_encode_baud_rate);
//
// tty_encode_baud_rate		-	set baud rate of the tty
// @tty:   terminal device
// @ibaud: input baud rate
// @obaud: output baud rate
//
// Update the current termios data for the tty with the new speed
// settings. The caller must hold the termios_rwsem for the tty in
// question.
//
#[no_mangle]
pub unsafe extern "C" fn tty_encode_baud_rate(tty: *mut tty_struct, ibaud: speed_t, obaud: speed_t) {
    void tty_encode_baud_rate(struct tty_struct *tty, speed_t ibaud, speed_t obaud)
    {
    tty_termios_encode_baud_rate(&tty.termios, ibaud, obaud);
    }
    EXPORT_SYMBOL_GPL(tty_encode_baud_rate);
