//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps65010.h
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


// linux/mfd/tps65010.h
//
// Functions to access TPS65010 power management device.
//
// Copyright (C) 2004 Dirk Behme <dirk.behme@de.bosch.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 2 of the License, or (at your
// option) any later version.
//
// THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
// WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN
// NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
// USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
// ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// You should have received a copy of the  GNU General Public License along
// with this program; if not, write  to the Free Software Foundation, Inc.,
// 675 Mass Ave, Cambridge, MA 02139, USA.
//
// ----------------------------------------------------------------------------
// Registers, all 8 bits
// ----------------------------------------------------------------------------
//
pub const TPS_CHGSTATUS: c_uint = 0x01;

pub const TPS_REGSTATUS: c_uint = 0x02;

pub const TPS_MASK1: c_uint = 0x03;
pub const TPS_MASK2: c_uint = 0x04;
pub const TPS_ACKINT1: c_uint = 0x05;
pub const TPS_ACKINT2: c_uint = 0x06;
pub const TPS_CHGCONFIG: c_uint = 0x07;

pub const TPS_LED1_ON: c_uint = 0x08;
pub const TPS_LED1_PER: c_uint = 0x09;
pub const TPS_LED2_ON: c_uint = 0x0a;
pub const TPS_LED2_PER: c_uint = 0x0b;
pub const TPS_VDCDC1: c_uint = 0x0c;

pub const TPS_VDCDC2: c_uint = 0x0d;

pub const TPS_VREGS1: c_uint = 0x0e;

pub const TPS_MASK3: c_uint = 0x0f;
pub const TPS_DEFGPIO: c_uint = 0x10;
//
// ----------------------------------------------------------------------------
// Macros used by exported functions
// ----------------------------------------------------------------------------
//
pub const LED1: c_int = 1;
pub const LED2: c_int = 2;
pub const OFF: c_int = 0;
pub const ON: c_int = 1;
pub const BLINK: c_int = 2;
pub const GPIO1: c_int = 1;
pub const GPIO2: c_int = 2;
pub const GPIO3: c_int = 3;
pub const GPIO4: c_int = 4;
pub const LOW: c_int = 0;
pub const HIGH: c_int = 1;
//
// ----------------------------------------------------------------------------
// Exported functions
// ----------------------------------------------------------------------------
//
// Draw from VBUS:
// 0 mA -- DON'T DRAW (might supply power instead)
// 100 mA -- usb unit load (slowest charge rate)
// 500 mA -- usb high power (fast battery charge)
//
extern "C" {
    pub fn tps65010_set_vbus_draw(mA: unsigned) -> c_int;
}
// tps65010_set_gpio_out_value parameter:
// gpio:  GPIO1, GPIO2, GPIO3 or GPIO4
// value: LOW or HIGH
//
extern "C" {
    pub fn tps65010_set_gpio_out_value(gpio: unsigned, value: unsigned) -> c_int;
}
// tps65010_set_led parameter:
// led:  LED1 or LED2
// mode: ON, OFF or BLINK
//
extern "C" {
    pub fn tps65010_set_led(led: unsigned, mode: unsigned) -> c_int;
}
// tps65010_set_vib parameter:
// value: ON or OFF
//
extern "C" {
    pub fn tps65010_set_vib(value: unsigned) -> c_int;
}
// tps65010_set_low_pwr parameter:
// mode: ON or OFF
//
extern "C" {
    pub fn tps65010_set_low_pwr(mode: unsigned) -> c_int;
}
// tps65010_config_vregs1 parameter:
// value to be written to VREGS1 register
// Note: The complete register is written, set all bits you need
//
extern "C" {
    pub fn tps65010_config_vregs1(value: unsigned) -> c_int;
}
// tps65013_set_low_pwr parameter:
// mode: ON or OFF
//
extern "C" {
    pub fn tps65013_set_low_pwr(mode: unsigned) -> c_int;
}
// tps65010_set_vdcdc2
// value to be written to VDCDC2
//
extern "C" {
    pub fn tps65010_config_vdcdc2(value: unsigned) -> c_int;
}
//
// struct tps65010_board - packages GPIO and LED lines
// @outmask: bit (N-1) is set to allow GPIO-N to be used as an
// (open drain) output
// @setup: optional callback issued once the GPIOs are valid
// @teardown: optional callback issued before the GPIOs are invalidated
//
// Board data may be used to package the GPIO (and LED) lines for use
// in by the generic GPIO and LED frameworks.  The first four GPIOs
// starting at gpio_base are GPIO1..GPIO4.  The next two are LED1/nPG
// and LED2 (with hardware blinking capability, not currently exposed).
//
// The @setup callback may be used with the kind of board-specific glue
// which hands the (now-valid) GPIOs to other drivers, or which puts
// devices in their initial states using these GPIOs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65010_board {
    pub outmask: unsigned,
    pub gc): *mut *mut *mut int (setup)(struct i2c_client client, struct gpio_chip,
    pub gc): *mut *mut *mut void (teardown)(struct i2c_client client, struct gpio_chip,
}
