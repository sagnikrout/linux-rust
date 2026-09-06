//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/serio/i8042-acpipnpio.h
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
// Names.
//

//
// IRQs.
//

//
// Register numbers.
//

extern "C" {
    pub fn inb(_arg: I8042_DATA_REG) -> return;
}
extern "C" {
    pub fn inb(_arg: I8042_STATUS_REG) -> return;
}

// Quirk table for different mainboards. Options similar or identical to i8042
// module parameters.
// ORDERING IS IMPORTANT! The first match will be apllied and the rest ignored.
// This allows entries to overwrite vendor wide quirks on a per device basis.
// Where this is irrelevant, entries are sorted case sensitive by DMI_SYS_VENDOR
// and/or DMI_BOARD_VENDOR to make it easier to avoid duplicate entries.
//
// Asus X450LCP
//
// ASUS Zenbook UX425QA_UM425QA
// Some Zenbooks report "Zenbook" with a lowercase b.
//
// ASUS ZenBook UX425UA/QA
// ASUS ZenBook UM325UA/QA
//
// On some Asus laptops, just running self tests cause problems.
//
// ASUS P65UP5 - AUX LOOP command does not raise AUX IRQ
// ASUS G1S
// Acer Aspire 5710
// Acer Aspire 7738
// Acer Aspire 5536
//
// Acer Aspire 5738z
// Touchpad stops working in mux mode when dis- + re-enabled
// with the touchpad enable/disable toggle hotkey
//
// Acer Aspire One 150
// Acer Aspire One 532h
//
// Some Wistron based laptops need us to explicitly enable the 'Dritek
// keyboard extension' to make their extra keys start generating scancodes.
// Originally, this was just confined to older laptops, but a few Acer laptops
// have turned up in 2007 that also need this again.
//
// Acer Aspire 5100
// Acer Aspire 5610
// Acer Aspire 5630
// Acer Aspire 5650
// Acer Aspire 5680
// Acer Aspire 5720
// Acer Aspire 9110
// Acer TravelMate 660
// Acer TravelMate 2490
// Acer TravelMate 4280
// Acer TravelMate P459-G2-M
// Amoi M636/A737
// Compal HEL80I
// Advent 4211
// Dell Embedded Box PC 3000
// Dell XPS M1530
// Dell Vostro 1510
// Dell Vostro V13
// Dell Vostro 1320
// Dell Vostro 1520
// Dell Vostro 1720
// Entroware Proteus
//
// Some Fujitsu notebooks are having trouble with touchpads if
// active multiplexing mode is activated. Luckily they don't have
// external PS/2 ports so we can safely disable it.
// ... apparently some Toshibas don't like MUX mode either and
// die horrible death on reboot.
//
// Fujitsu Lifebook P7010/P7010D
// Fujitsu Lifebook P5020D
// Fujitsu Lifebook S2000
// Fujitsu Lifebook S6230
// Fujitsu Lifebook T725 laptop
// Fujitsu Lifebook U745
// Fujitsu T70H
// Fujitsu A544 laptop
// https://bugzilla.redhat.com/show_bug.cgi?id=1111138
// Fujitsu AH544 laptop
// https://bugzilla.kernel.org/show_bug.cgi?id=69731
// Fujitsu U574 laptop
// https://bugzilla.kernel.org/show_bug.cgi?id=69731
// Fujitsu UH554 laptop
// Fujitsu Lifebook P7010
// Fujitsu-Siemens Lifebook T3010
// Fujitsu-Siemens Lifebook E4010
// Fujitsu-Siemens Amilo Pro 2010
// Fujitsu-Siemens Amilo Pro 2030
// Fujitsu Lifebook A574/H
// Fujitsu Lifebook E756
// https://bugzilla.suse.com/show_bug.cgi?id=1229056
// Fujitsu Lifebook E5411
// Fujitsu Lifebook U728
// Gigabyte M912
// Gigabyte Spring Peak - defines wrong chassis type
// Gigabyte T1005 - defines wrong chassis type ("Other")
// Gigabyte T1005M/P - defines wrong chassis type ("Other")
//
// Some laptops need keyboard reset before probing for the trackpad to get
// it detected, initialised & finally work.
//
// Gigabyte P35 v2 - Elantech touchpad
// Aorus branded Gigabyte X3 Plus - Elantech touchpad
// Gigabyte P34 - Elantech touchpad
// Gigabyte P57 - Elantech touchpad
// Gericom Bellagio
// Gigabyte M1022M netbook
//
// HP Pavilion DV4017EA -
// errors on MUX ports are reported without raising AUXDATA
// causing "spurious NAK" messages.
//
// HP Pavilion ZT1000 -
// like DV4017EA does not raise AUXERR for errors on MUX ports.
//
// HP Pavilion DV4270ca -
// like DV4017EA does not raise AUXERR for errors on MUX ports.
//
// Newer HP Pavilion dv4 models
// IBM 2656
// Avatar AVIU-145A6
// Intel MBO Desktop D845PESV
//
// Intel NUC D54250WYK - does not have i8042 controller but
// declares PS/2 devices in DSDT.
//
// Lenovo 3000 n100
// Lenovo XiaoXin Air 12
// Lenovo LaVie Z
// Lenovo Ideapad U455
// Lenovo ThinkPad L460
// Lenovo ThinkPad Twist S230u
// LG Electronics X110
// Medion Akoya Mini E1210
// Medion Akoya E1222
// MSI Wind U-100
//
// No data is coming from the touchscreen unless KBC
// is in legacy mode.
//
// Panasonic CF-29
// Medion Akoya E7225
// Microsoft Virtual Machine
// Medion MAM 2070
// TUXEDO BU1406
// OQO Model 01
// Acer Aspire 5 A515
// ULI EV4873 - AUX LOOP does not work properly
//
// Arima-Rioworks HDAMB -
// AUX LOOP command does not raise AUX IRQ
//
// Sharp Actius MM20
//
// Sony Vaio FZ-240E -
// reset and GET ID commands issued via KBD port are
// sometimes being delivered to AUX3.
//
// Most (all?) VAIOs do not have external PS/2 ports nor
// they implement active multiplexing properly, and
// MUX discovery usually messes up keyboard/touchpad.
//
// Sony Vaio FS-115b
//
// Sony Vaio VGN-CS series require MUX or the touch sensor
// buttons will disturb touchpad operation
//
// A lot of modern Clevo barebones have touchpad and/or keyboard issues
// after suspend fixable with nomux + reset + noloop + nopnp. Luckily,
// none of them have an external PS/2 port so this can safely be set for
// all of them. These two are based on a Clevo design, but have the
// board_name changed.
//
// Mivvy M310
//
// Some laptops need keyboard reset before probing for the trackpad to get
// it detected, initialised & finally work.
//
// Schenker XMG C504 - Elantech touchpad
// Blue FB5601
//
// Some TongFang barebones have touchpad and/or keyboard issues after
// suspend fixable with nomux + reset + noloop + nopnp. Luckily, none of
// them have an external PS/2 port so this can safely be set for all of
// them.
// TongFang barebones come with board_vendor and/or system_vendor set to
// a different value for each individual reseller. The only somewhat
// universal way to identify them is by board_name.
//
// A lot of modern Clevo barebones have touchpad and/or keyboard issues
// after suspend fixable with the forcenorestore quirk.
// Clevo barebones come with board_vendor and/or system_vendor set to
// either the very generic string "Notebook" and/or a different value
// for each individual reseller. The only somewhat universal way to
// identify them is by board_name.
//
// At least one modern Clevo barebone has the touchpad connected both
// via PS/2 and i2c interface. This causes a race condition between the
// psmouse and i2c-hid driver. Since the full capability of the touchpad
// is available via the i2c interface and the device has no external
// PS/2 port, it is safe to just ignore all ps2 mouses here to avoid
// this issue. The known affected device is the
// TUXEDO InfinityBook S17 Gen6 / Clevo NS70MU which comes with one of
// the two different dmi strings below. NS50MU is not a typo!
//
// This is only a partial board_name and might be followed by
// another letter or number. DMI_MATCH however does do partial
// matching.
//
// Clevo P650RS, 650RP6, Sager NP8152-S, and others
//
// This is only a partial board_name and might be followed by
// another letter or number. DMI_MATCH however does do partial
// matching.
//
// This is only a partial board_name and might be followed by
// another letter or number. DMI_MATCH however does do partial
// matching.
//
// This is only a partial board_name and might be followed by
// another letter or number. DMI_MATCH however does do partial
// matching.
//
// This is only a partial board_name and might be followed by
// another letter or number. DMI_MATCH however does do partial
// matching.
//
// See comment on TUXEDO InfinityBook S17 Gen6 / Clevo NS70MU above
//
// The Ayaneo Kun is a handheld device where some the buttons
// are handled by an AT keyboard. The keyboard is usually
// detected as raw, but sometimes, usually after a cold boot,
// it is detected as translated. Make sure that the keyboard
// is always in raw mode.
//

// Keyboard ports are always supposed to be wakeup-enabled

// Honor module parameter when value is not default

// Just return if platform does not have i8042 controller

//
// On ix86 platforms touching the i8042 data register region can do really
// bad things. Because of this the region is always reserved on ix86 boxes.
//
// if (!request_region(I8042_DATA_REG, 16, "i8042"))
// return -EBUSY;
//

//
// A20 was already enabled during early kernel init. But some buggy
// BIOSes (in MSI Laptops) require A20 to be enabled using 8042 to
// resume from S3. So we do it here and hope that nothing breaks.
//

