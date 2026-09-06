//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/8xx_immap.h
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
// MPC8xx Internal Memory Map
// Copyright (c) 1997 Dan Malek (dmalek@jlc.net)
//
// The I/O on the MPC860 is comprised of blocks of special registers
// and the dual port ram for the Communication Processor Module.
// Within this space are functional units such as the SIU, memory
// controller, system timers, and other control functions.  It is
// a combination that I found difficult to separate into logical
// functional files.....but anyone else is welcome to try.  -- Dan
//

// System configuration registers.
//
// PCMCIA configuration registers.
//
// Memory controller registers.
//
// -----------------------------------------------------------------------
// BR - Memory Controller: Base Register					16-9
//
pub const BR_BA_MSK: c_uint = 0xffff8000	/* Base Address Mask			*/;
pub const BR_AT_MSK: c_uint = 0x00007000	/* Address Type Mask			*/;
pub const BR_PS_MSK: c_uint = 0x00000c00	/* Port Size Mask			*/;
pub const BR_PS_32: c_uint = 0x00000000	/* 32 bit port size			*/;
pub const BR_PS_16: c_uint = 0x00000800	/* 16 bit port size			*/;
pub const BR_PS_8: c_uint = 0x00000400	/*  8 bit port size			*/;
pub const BR_PARE: c_uint = 0x00000200	/* Parity Enable			*/;
pub const BR_WP: c_uint = 0x00000100	/* Write Protect			*/;
pub const BR_MS_MSK: c_uint = 0x000000c0	/* Machine Select Mask			*/;
pub const BR_MS_GPCM: c_uint = 0x00000000	/* G.P.C.M. Machine Select		*/;
pub const BR_MS_UPMA: c_uint = 0x00000080	/* U.P.M.A Machine Select		*/;
pub const BR_MS_UPMB: c_uint = 0x000000c0	/* U.P.M.B Machine Select		*/;
pub const BR_V: c_uint = 0x00000001	/* Bank Valid				*/;
// -----------------------------------------------------------------------
// OR - Memory Controller: Option Register				16-11
//
pub const OR_AM_MSK: c_uint = 0xffff8000	/* Address Mask Mask			*/;
pub const OR_ATM_MSK: c_uint = 0x00007000	/* Address Type Mask Mask		*/;
pub const OR_CSNT_SAM: c_uint = 0x00000800	/* Chip Select Negation Time/ Start	*/;
// Address Multiplex
pub const OR_ACS_MSK: c_uint = 0x00000600	/* Address to Chip Select Setup mask	*/;
pub const OR_ACS_DIV1: c_uint = 0x00000000	/* CS is output at the same time	*/;
pub const OR_ACS_DIV4: c_uint = 0x00000400	/* CS is output 1/4 a clock later	*/;
pub const OR_ACS_DIV2: c_uint = 0x00000600	/* CS is output 1/2 a clock later	*/;
pub const OR_G5LA: c_uint = 0x00000400	/* Output #GPL5 on #GPL_A5		*/;
pub const OR_G5LS: c_uint = 0x00000200	/* Drive #GPL high on falling edge of...*/;
pub const OR_BI: c_uint = 0x00000100	/* Burst inhibit			*/;
pub const OR_SCY_MSK: c_uint = 0x000000f0	/* Cycle Length in Clocks		*/;
pub const OR_SCY_0_CLK: c_uint = 0x00000000	/* 0 clock cycles wait states		*/;
pub const OR_SCY_1_CLK: c_uint = 0x00000010	/* 1 clock cycles wait states		*/;
pub const OR_SCY_2_CLK: c_uint = 0x00000020	/* 2 clock cycles wait states		*/;
pub const OR_SCY_3_CLK: c_uint = 0x00000030	/* 3 clock cycles wait states		*/;
pub const OR_SCY_4_CLK: c_uint = 0x00000040	/* 4 clock cycles wait states		*/;
pub const OR_SCY_5_CLK: c_uint = 0x00000050	/* 5 clock cycles wait states		*/;
pub const OR_SCY_6_CLK: c_uint = 0x00000060	/* 6 clock cycles wait states		*/;
pub const OR_SCY_7_CLK: c_uint = 0x00000070	/* 7 clock cycles wait states		*/;
pub const OR_SCY_8_CLK: c_uint = 0x00000080	/* 8 clock cycles wait states		*/;
pub const OR_SCY_9_CLK: c_uint = 0x00000090	/* 9 clock cycles wait states		*/;
pub const OR_SCY_10_CLK: c_uint = 0x000000a0	/* 10 clock cycles wait states		*/;
pub const OR_SCY_11_CLK: c_uint = 0x000000b0	/* 11 clock cycles wait states		*/;
pub const OR_SCY_12_CLK: c_uint = 0x000000c0	/* 12 clock cycles wait states		*/;
pub const OR_SCY_13_CLK: c_uint = 0x000000d0	/* 13 clock cycles wait states		*/;
pub const OR_SCY_14_CLK: c_uint = 0x000000e0	/* 14 clock cycles wait states		*/;
pub const OR_SCY_15_CLK: c_uint = 0x000000f0	/* 15 clock cycles wait states		*/;
pub const OR_SETA: c_uint = 0x00000008	/* External Transfer Acknowledge	*/;
pub const OR_TRLX: c_uint = 0x00000004	/* Timing Relaxed			*/;
pub const OR_EHTR: c_uint = 0x00000002	/* Extended Hold Time on Read		*/;
// System Integration Timers.
//

// Clocks and Reset.
//
// System Integration Timers keys.
//
// Clocks and reset keys.
//
// The key to unlock registers maintained by keep-alive power.
//

// Video interface.  MPC823 Only.
//
// LCD interface.  823 Only.
//
// I2C
//
// DMA control/status registers.
//
// Communication Processor Module Interrupt Controller.
//
// Input/Output Port control/status registers.
//
// Communication Processor Module Timers
//
// Finally, the Communication Processor stuff.....
//
// MPC860T Fast Ethernet Controller.  It isn't part of the CPM, but
// it fits within the address space.
//
// The FEC and LCD color map share the same address space....
// I guess we will never see an 823T :-).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fec_lcd {
    pub fl_un_fec: fec_t,
    pub fl_un_cmap: [u_char; 0x200],
}

// General control and status registers.
//
// Baud rate generators.
//
// Serial Communication Channels.
//
// Serial Management Channels.
//
// Serial Peripheral Interface.
//
// Parallel Interface Port.
//
// Port E - MPC87x/88x only.
//
// Communications Processor Timing Register -
//
// Serial Interface and Time Slot Assignment.
//
// 256 bytes of MPC823 video controller RAM array.
//
// The fast ethernet controller is not really part of the CPM,
// but it resides in the address space.
// The LCD color map is also here.
//

// The DUET family has a second FEC here

// Dual Ported RAM follows.
// There are many different formats for this memory area
// depending upon the devices used and options chosen.
// Some processors don't have all of it populated.
//
// Internal memory map.
//

