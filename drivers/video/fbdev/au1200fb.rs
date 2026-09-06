//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/au1200fb.h
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


//
// BRIEF MODULE DESCRIPTION
// Hardware definitions for the Au1200 LCD controller
//
// Copyright 2004 AMD
// Author:	AMD
//
// This program is free software; you can redistribute	 it and/or modify it
// under  the terms of	 the GNU General  Public License as published by the
// Free Software Foundation;  either version 2 of the	License, or (at your
// option) any later version.
//
// THIS  SOFTWARE  IS PROVIDED	  ``AS	IS'' AND   ANY	EXPRESS OR IMPLIED
// WARRANTIES,	  INCLUDING, BUT NOT  LIMITED  TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN
// NO	EVENT  SHALL   THE AUTHOR  BE	 LIABLE FOR ANY	  DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED	  TO, PROCUREMENT OF  SUBSTITUTE GOODS	OR SERVICES; LOSS OF
// USE, DATA,	OR PROFITS; OR	BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
// ANY THEORY OF LIABILITY, WHETHER IN	 CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// You should have received a copy of the  GNU General Public License along
// with this program; if not, write  to the Free Software Foundation, Inc.,
// 675 Mass Ave, Cambridge, MA 02139, USA.
//
pub const AU1200_LCD_ADDR: c_uint = 0xB5000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au1200_lcd {
    pub reserved0: volatile uint32,
    pub screen: volatile uint32,
    pub backcolor: volatile uint32,
    pub horztiming: volatile uint32,
    pub verttiming: volatile uint32,
    pub clkcontrol: volatile uint32,
    pub pwmdiv: volatile uint32,
    pub pwmhi: volatile uint32,
    pub reserved1: volatile uint32,
    pub winenable: volatile uint32,
    pub colorkey: volatile uint32,
    pub colorkeymsk: volatile uint32,
    pub cursorctrl: volatile uint32,
    pub cursorpos: volatile uint32,
    pub cursorcolor0: volatile uint32,
    pub cursorcolor1: volatile uint32,
    pub cursorcolor2: volatile uint32,
    pub cursorcolor3: uint32,
    pub hwc: },
    pub intstatus: volatile uint32,
    pub intenable: volatile uint32,
    pub outmask: volatile uint32,
    pub fifoctrl: volatile uint32,
    pub reserved2: [uint32; (0x0100-0x0058)/4],
    pub winctrl0: volatile uint32,
    pub winctrl1: volatile uint32,
    pub winctrl2: volatile uint32,
    pub winbuf0: volatile uint32,
    pub winbuf1: volatile uint32,
    pub winbufctrl: volatile uint32,
    pub winreserved0: uint32,
    pub winreserved1: uint32,
    pub window: [}; 4],
    pub reserved3: [uint32; (0x0400-0x0180)/4],
    pub palette: [volatile uint32; (0x0800-0x0400)/4],
    pub cursorpattern: [volatile uint8; 256],
}

// lcd_screen

// lcd_backcolor

// lcd_winenable

// lcd_colorkey

// lcd_colorkeymsk

// lcd windows control 0

// lcd windows control 1

// lcd windows control 2

// lcd windows buffer control

// lcd_intstatus, lcd_intenable

// lcd_horztiming

// Macro flag: #define LCD_HORZTIMING_HND2_N(N)(((N)-1)<<18)
// Macro flag: #define LCD_HORZTIMING_HND1_N(N)(((N)-1)<<9)

// lcd_verttiming

// Macro flag: #define LCD_VERTTIMING_VND2_N(N)(((N)-1)<<18)
// Macro flag: #define LCD_VERTTIMING_VND1_N(N)(((N)-1)<<9)

// lcd_clkcontrol

// lcd_pwmdiv

// lcd_pwmhi

// lcd_hwccon

// lcd_cursorpos

// lcd_cursorcolor

// lcd_fifoctrl

// lcd_outmask

//
