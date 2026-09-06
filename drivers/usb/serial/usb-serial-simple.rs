//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/usb-serial-simple.c
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
// USB Serial "Simple" driver
//
// Copyright (C) 2001-2006,2008,2013 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (C) 2005 Arthur Huillet (ahuillet@users.sf.net)
// Copyright (C) 2005 Thomas Hergenhahn <thomas.hergenhahn@suse.de>
// Copyright (C) 2009 Outpost Embedded, LLC
// Copyright (C) 2010 Zilogic Systems <code@zilogic.com>
// Copyright (C) 2013 Wei Shuai <cpuwolf@gmail.com>
// Copyright (C) 2013 Linux Foundation
//

    static const struct usb_device_id vendor##_id_table[] = {	\
    IDS(),							\
    { },							\
    };								\
    static struct usb_serial_driver vendor##_device = {		\
    .driver = {						\
    .name =		#vendor,			\
    },							\
    .id_table =		vendor##_id_table,		\
    .num_ports =		nport,				\
    };

// Medtronic CareLink USB driver

    { USB_DEVICE(0x0a21, 0x8001) }	/* MMT-7305WW */
    DEVICE(carelink, CARELINK_IDS);
// Infineon Flashloader driver

    { USB_DEVICE_INTERFACE_CLASS(0x058b, 0x0041, USB_CLASS_CDC_DATA) }, \
    { USB_DEVICE(0x8087, 0x0716) }, \
    { USB_DEVICE(0x8087, 0x0801) }
    DEVICE(flashloader, FLASHLOADER_IDS);
// Funsoft Serial USB driver

    { USB_DEVICE(0x1404, 0xcddc) }
    DEVICE(funsoft, FUNSOFT_IDS);
// Google Serial USB SubClass

    { USB_VENDOR_AND_INTERFACE_INFO(0x18d1,			\
    USB_CLASS_VENDOR_SPEC,	\
    0x50,			\
    0x01) }
    DEVICE(google, GOOGLE_IDS);
// HP4x (48/49) Generic Serial driver

    { USB_DEVICE(0x03f0, 0x0121) }
    DEVICE(hp4x, HP4X_IDS);
// KAUFMANN RKS+CAN VCP

    { USB_DEVICE(0x16d0, 0x0870) }
    DEVICE(kaufmann, KAUFMANN_IDS);
// Libtransistor USB console

    { USB_DEVICE(0x1209, 0x8b00) }
    DEVICE(libtransistor, LIBTRANSISTOR_IDS);
// Motorola USB Phone driver

    { USB_DEVICE(0x05c6, 0x3197) },	/* unknown Motorola phone */	\
    { USB_DEVICE(0x0c44, 0x0022) },	/* unknown Motorola phone */	\
    { USB_DEVICE(0x22b8, 0x2a64) },	/* Motorola KRZR K1m */		\
    { USB_DEVICE(0x22b8, 0x2c84) },	/* Motorola VE240 phone */	\
    { USB_DEVICE(0x22b8, 0x2c64) }	/* Motorola V950 phone */
    DEVICE(moto_modem, MOTO_IDS);
// Motorola Tetra driver

    { USB_DEVICE(0x0cad, 0x9011) },	/* Motorola Solutions TETRA PEI */ \
    { USB_DEVICE(0x0cad, 0x9012) },	/* MTP6550 */ \
    { USB_DEVICE(0x0cad, 0x9013) },	/* MTP3xxx */ \
    { USB_DEVICE(0x0cad, 0x9015) },	/* MTP85xx */ \
    { USB_DEVICE(0x0cad, 0x9016) }	/* TPG2200 */
    DEVICE(motorola_tetra, MOTOROLA_TETRA_IDS);
// Nokia mobile phone driver

    { USB_DEVICE(0x0421, 0x069a) }	/* Nokia 130 (RM-1035) */
    DEVICE(nokia, NOKIA_IDS);
// Novatel Wireless GPS driver

    { USB_DEVICE(0x09d7, 0x0100) }	/* NovAtel FlexPack GPS */
    DEVICE_N(novatel_gps, NOVATEL_IDS, 3);
// OWON electronic test and measurement equipment driver

    { USB_DEVICE(0x5345, 0x1234) } /* HDS200 oscilloscopes and others */
    DEVICE(owon, OWON_IDS);
// Siemens USB/MPI adapter

    { USB_DEVICE(0x908, 0x0004) }
    DEVICE(siemens_mpi, SIEMENS_IDS);
// Suunto ANT+ USB Driver

    { USB_DEVICE(0x0fcf, 0x1008) },	\
    { USB_DEVICE(0x0fcf, 0x1009) } /* Dynastream ANT USB-m Stick */
    DEVICE(suunto, SUUNTO_IDS);
// ViVOpay USB Serial Driver

    { USB_DEVICE(0x1d5f, 0x1004) }	/* ViVOpay 8800 */
    DEVICE(vivopay, VIVOPAY_IDS);
// ZIO Motherboard USB driver

    { USB_DEVICE(0x1CBE, 0x0103) }
    DEVICE(zio, ZIO_IDS);
// All of the above structures mushed into two lists
    static struct usb_serial_driver * const serial_drivers[] = {
    &carelink_device,
    &flashloader_device,
    &funsoft_device,
    &google_device,
    &hp4x_device,
    &kaufmann_device,
    &libtransistor_device,
    &moto_modem_device,
    &motorola_tetra_device,
    &nokia_device,
    &novatel_gps_device,
    &owon_device,
    &siemens_mpi_device,
    &suunto_device,
    &vivopay_device,
    &zio_device,
    core::ptr::null_mut()
    };
    static const struct usb_device_id id_table[] = {
    CARELINK_IDS(),
    FLASHLOADER_IDS(),
    FUNSOFT_IDS(),
    GOOGLE_IDS(),
    HP4X_IDS(),
    KAUFMANN_IDS(),
    LIBTRANSISTOR_IDS(),
    MOTO_IDS(),
    MOTOROLA_TETRA_IDS(),
    NOKIA_IDS(),
    NOVATEL_IDS(),
    OWON_IDS(),
    SIEMENS_IDS(),
    SUUNTO_IDS(),
    VIVOPAY_IDS(),
    ZIO_IDS(),
    { },
    };
    MODULE_DEVICE_TABLE(usb, id_table);
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_DESCRIPTION("USB Serial 'Simple' driver");
    MODULE_LICENSE("GPL v2");
