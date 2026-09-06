//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/storage/unusual_devs.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Driver for USB Mass Storage compliant devices
// Unusual Devices File
//
// Current development and maintenance by:
// (c) 2000-2002 Matthew Dharm (mdharm-usb@one-eyed-alien.net)
//
// Initial work by:
// (c) 2000 Adam J. Richter (adam@yggdrasil.com), Yggdrasil Computing, Inc.
//
// IMPORTANT NOTE: This file must be included in another file which does
// the following thing for it to work:
// The UNUSUAL_DEV, COMPLIANT_DEV, and USUAL_DEV macros must be defined
// before this file is included.
//
// If you edit this file, please try to keep it sorted first by VendorID,
// then by ProductID.
//
// If you want to add an entry for this file, be sure to include the
// following information:
// - a patch that adds the entry for your device, including your
// email address right above the entry (plus maybe a brief
// explanation of the reason for the entry),
// - a copy of /sys/kernel/debug/usb/devices with your device plugged in
// running with this patch.
// Send your submission to the USB development list <linux-usb@vger.kernel.org>
//
// Note: If you add an entry only in order to set the CAPACITY_OK flag,
// use the COMPLIANT_DEV macro instead of UNUSUAL_DEV.  This is
// because such entries mark devices which actually work correctly,
// as opposed to devices that do something strangely or wrongly.
//
// In-kernel mode switching is deprecated.  Do not add new devices to
// this list for the sole purpose of switching them to a different
// mode.  Existing userspace solutions are superior.
//
// New mode switching devices should instead be added to the database
// maintained at https://www.draisberghof.de/usb_modeswitch
//

// Macro flag: #define NO_SDDR09

// patch submitted by Vivian Bregier <Vivian.Bregier@imag.fr>
// Reported by Rodolfo Quesada <rquesada@roqz.net>
// Reported by Ben Efros <ben@pc-doctor.com>
//
// Reported by Grant Grundler <grundler@parisc-linux.org>
// HP r707 camera in "Disk" mode with 2.00.23 or 2.00.24 firmware.
//
// Reported by Sebastian Kapfer <sebastian_kapfer@gmx.net>
// and Olaf Hering <olh@suse.de> (different bcd's, same vendor/product)
// for USB floppies that need the SINGLE_LUN enforcement.
//
// Patch submitted by Mihnea-Costin Grigore <mihnea@zulu.ro>
//
// Deduced by Jonathan Woithe <jwoithe@just42.net>
// Entry needed for flags: US_FL_FIX_INQUIRY because initial inquiry message
// always fails and confuses drive.
//
// Submitted by Ernestas Vaiciukevicius <ernisv@gmail.com>
// Reported by Orgad Shaneh <orgads@gmail.com>
// Reported by Christian Leber <christian@leber.de>
// Reported by Stefan Werner <dustbln@gmx.de>
// Reported by Pete Zaitcev <zaitcev@redhat.com>, bz#176584
//
// Reported by Andrew Nayenko <relan@bk.ru>
// Updated for new firmware by Phillip Potter <phil@philpotter.co.uk>
//
// Reported by Mario Rettig <mariorettig@web.de>
// Reported by <honkkis@gmail.com>
// Reported by Jon Hart <Jon.Hart@web.de>
//
// Reported by Sumedha Swamy <sumedhaswamy@gmail.com> and
// Einar Th. Einarsson <einarthered@gmail.com>
//
// Reported by Jiri Slaby <jirislaby@gmail.com> and
// Rene C. Castberg <Rene@Castberg.org>
//
// Reported by Matthew Bloch <matthew@bytemark.co.uk>
// Reported by Bardur Arantsson <bardur@scientician.net>
// Reported by Manuel Osdoba <manuel.osdoba@tu-ilmenau.de>
// Reported by Alex Corcoles <alex@corcoles.net>
// Reported by Daniele Forsi <dforsi@gmail.com>
// Patch submitted by Victor A. Santos <victoraur.santos@gmail.com>
// Patch submitted by Mikhail Zolotaryov <lebon@lebon.org.ua>
// Added by Lubomir Rintel <lkundrak@v3.sk>, a very fine chap

//
// Patch submitted by Daniel Drake <dsd@gentoo.org>
// Device reports nonsense bInterfaceProtocol 6 when connected over USB2
//
// Pete Zaitcev <zaitcev@yahoo.com>, from Patrick C. F. Ernzer, bz#162559.
// The key does not actually break, but it returns zero sense which
// makes our SCSI stack to print confusing messages.
//
// Bohdan Linda <bohdan.linda@gmail.com>
// 1GB USB sticks MyFlash High Speed. I have restricted
// the revision to my model only
//
// Reported by Tamas Kerecsen <kerecsen@bigfoot.com>
// Obviously the PROM has not been customized by the VAR;
// the Vendor and Product string descriptors are:
// Generic Mass Storage (PROTOTYPE--Remember to change idVendor)
// Generic Manufacturer (PROTOTYPE--Remember to change idVendor)
//
// This virtual floppy is found in Sun equipment (x4600, x4200m2, etc.)
// Reported by Pete Zaitcev <zaitcev@redhat.com>
// This device chokes on both version of MODE SENSE which we have, so
// use_10_for_ms is not effective, and we use US_FL_NO_WP_DETECT.
//
// Reported by Egbert Eich <eich@suse.com>
// Patch submitted by Philipp Friedrich <philipp@void.at>
// Patch submitted by Stephane Galles <stephane.galles@free.fr>
// Patch submitted by Jens Taprogge <jens.taprogge@taprogge.org>
//
// Reported by Paul Stewart <stewart@wetlogic.net>
// This entry is needed because the device reports Sub=ff
//
// BENQ DC5330
// Reported by Manuel Fombuena <mfombuena@ya.com> and
// Frank Copeland <fjc@thingy.apana.org.au>
//
// Patch for Nikon coolpix 2000
// Submitted by Fabien Cosse <fabien.cosse@wanadoo.fr>
//
// Reported by Doug Maxey (dwm@austin.ibm.com)
// Reported by Ai Chao <aichao@kylinos.cn>
//
// Reported by Simon Levitt <simon@whattf.com>
// This entry needs Sub and Proto fields
//
// Reported by Khalid Aziz <khalid@gonehiking.org>
// This entry is needed because the device reports Sub=ff
//
// Reported by James Buren <braewoods+lkml@braewoods.net>
// Virtual ISOs cannot be remounted if ejected while the device is locked
// Disable locking to mimic Windows behavior that bypasses the issue
//
// Not sure who reported this originally but
// Pavel Machek <pavel@ucw.cz> reported that the extra US_FL_SINGLE_LUN
// flag be added
//
// Reported by Ondrej Zary <linux@zary.sk>
// The device reports one sector more and breaks when that sector is accessed
// Firmwares older than 2.6c (the latest one and the only that claims Linux
// support) have also broken tag handling
//
// Reported by Kriston Fincher <kriston@airmail.net>
// Patch submitted by Sean Millichamp <sean@bruenor.org>
// This is to support the Panasonic PalmCam PV-SD4090
// This entry is needed because the device reports Sub=ff
//
// From Yukihiro Nakai, via zaitcev@yahoo.com.
// This is needed for CB instead of CBI
//
// Reported by Adriaan Penning <a.penning@luon.net>
// Reported by Simeon Simeonov <simeonov_2000@yahoo.com>
//
// Most of the following entries were developed with the help of
// Shuttle/SCM directly.
//

// Reported by Markus Demleitner <msdemlei@cl.uni-heidelberg.de>
// Reported by Daniel Nouri <dpunktnpunkt@web.de>
// Reported by Dmitry Khlystov <adminimus@gmail.com>
// Reported by Vitaly Kuznetsov <vitty@altlinux.ru>
// Added by Dmitry Artamonow <mad_soft@inbox.ru>
//
// Entry and supporting patch by Theodore Kilgore <kilgota@auburn.edu>.
// Device uses standards-violating 32-byte Bulk Command Block Wrappers and
// reports itself as "Proprietary SCSI Bulk." Cf. device entry 0x084d:0x0011.
//
// Reported by Bob Sass <rls@vectordb.com> -- only rev 1.33 tested
//
// Iomega Clik! Drive
// Reported by David Chatenay <dchatenay@hotmail.com>
// The reason this is needed is not fully known.
//
// Added by Alan Stern <stern@rowland.harvard.edu>
//
// Yakumo Mega Image 37
// Submitted by Stephan Fuhrmann <atomenergie@t-online.de>
//
// Another Yakumo camera.
// Reported by Michele Alzetta <michele.alzetta@aliceposta.it>
//
// Reported by Iacopo Spalletti <avvisi@spalletti.it>
//
// Yakumo Mega Image 47
// Reported by Bjoern Paetzel <kolrabi@kolrabi.de>
//
// Reported by Paul Ortyl <ortylp@3miasto.net>
// Note that it's similar to the device above, only different prodID
//
// Submitted by Lars Jacob <jacob.lars@googlemail.com>
// This entry is needed because the device reports Sub=ff
//
// Reported by wim@geeks.nl
// Submitted by Olaf Hering, <olh@suse.de> SuSE Bugzilla #49049
// Submitted by Klaus Mueller <k.mueller@intershop.de>
// Submitted by Rajesh Kumble Nayak <nayak@obs-nice.fr>
// Submitted by Michal Mlotek <mlotek@foobar.pl>
// Submitted by Nathan Babb <nathan@lexi.com>
// Submitted by Frank Engel <frankie@cse.unsw.edu.au>
// Submitted by Mike Alborn <malborn@deandra.homeip.net>
// Submitted by Ren Bigcren <bigcren.ren@sonymobile.com>
// floppy reports multiple luns
// We keep this entry to force the transport; firmware 3.00 and later is ok.
//
// Reported by Johann Cardon <johann.cardon@free.fr>
// This entry is needed only because the device reports
// bInterfaceClass = 0xff (vendor-specific)
//
// Reported by RTE <raszilki@yandex.ru>
// Fabrizio Fellini <fello@libero.it>
//
// Reported by Andre Welter <a.r.welter@gmx.de>
// This antique device predates the release of the Bulk-only Transport
// spec, and if it gets a Get-Max-LUN then it requires the host to do a
// Clear-Halt on the bulk endpoints.  The SINGLE_LUN flag will prevent
// us from sending the request.
//
// Reported by <Hendryk.Pfeiffer@gmx.de>
// Reported by Christian Schaller <cschalle@redhat.com>
//
// Submitted by Joel Bourquard <numlock@freesurf.ch>
// Some versions of this device need the SubClass and Protocol overrides
// while others don't.
//
// Submitted by Sven Anderson <sven-linux@anderson.de>
// There are at least four ProductIDs used for iPods, so I added 0x1202 and
// 0x1204. They just need the US_FL_FIX_CAPACITY. As the bcdDevice appears
// to change with firmware updates, I changed the range to maximum for all
// iPod entries.
//
// Reported by Avi Kivity <avi@argo.co.il>
//
// Reported by Tyson Vinson <lornoss@gmail.com>
// This particular productId is the iPod Nano
//
// Reported by Dan Williams <dcbw@redhat.com>
// Option N.V. mobile broadband modems
// Ignore driver CD mode and force into modem mode by default.
//
// Globetrotter HSDPA; mass storage shows up as Qualcomm for vendor
// Reported by Blake Matheny <bmatheny@purdue.edu>
//
// The following two entries are for a Genesys USB to IDE
// converter chip, but it changes its ProductId depending
// on whether or not a disk or an optical device is enclosed
// They were originally reported by Alexander Oltu
// <alexander@all-2.com> and Peter Marks <peter.marks@turner.com>
// respectively.
//
// US_FL_GO_SLOW and US_FL_MAX_SECTORS_64 added by Phil Dibowitz
// <phil@ipom.com> as these flags were made and hard-coded
// special-cases were pulled from scsiglue.c.
//
// Reported by Ben Efros <ben@pc-doctor.com>
// Added by Maël GUERIN <mael.guerin@murena.io>
//
// Reported by Hanno Boeck <hanno@gmx.de>
// Taken from the Lycoris Kernel
//
// Reported by Darsen Lu <darsen@micro.ee.nthu.edu.tw>
// Reported by Daniel Kukula <daniel.kuku@gmail.com>
// Reported by Rogerio Brito <rbrito@ime.usp.br>
// Reported by Richard -=[]=- <micro_flyer@hotmail.com>
//
// Change to bcdDeviceMin (0x0100 to 0x0001) reported by
// Thomas Bartosik <tbartdev@gmx-topmail.de>
//
// Reported by Alex Butcher <alex.butcher@assursys.co.uk>
// Submitted by Benny Sjostrand <benny@hostmobility.com>
// Reported by Miguel A. Fosas <amn3s1a@ono.com>
// Reported by David Hamilton <niftimusmaximus@lycos.com>
// Reported by Adrian Pilchowiec <adi1981@epf.pl>
//
// Reported by Jean-Baptiste Onofre <jb@nanthrax.net>
// Support the following product :
// "Dane-Elec MediaTouch"
//
// Reported by Massimiliano Ghilardi <massimiliano.ghilardi@gmail.com>
// This USB MP3/AVI player device fails and disconnects if more than 128
// sectors (64kB) are read/written in a single command, and may be present
// at least in the following products:
// "Magnex Digital Video Panel DVP 1800"
// "MP4 AIGO 4GB SLOT SD"
// "Teclast TL-C260 MP3"
// "i.Meizu PMP MP3/MP4"
// "Speed MV8 MP4 Audio Player"
//
// Reported by Olivier Blondeau <zeitoun@gmail.com>
// Submitted by Roman Hodek <roman@hodek.net>
// Reported by Eero Volotinen <eero@ping-viini.org>

//
// Datafab KECF-USB / Sagatek DCS-CF / Simpletech Flashlink UCF-100
// Only revision 1.13 tested (same for all of the above devices,
// based on the Datafab DF-UG-07 chip).  Needed for US_FL_FIX_INQUIRY.
// Submitted by Marek Michalkiewicz <marekm@amelek.gda.pl>.
// See also http://martin.wilck.bei.t-online.de/#kecf .
//
// Reported by Rauch Wolke <rauchwolke@gmx.net>
// and augmented by binbin <binbinsh@gmail.com> (Bugzilla #12882)
//
// Casio QV 2x00/3x00/4000/8000 digital still cameras are not conformant
// to the USB storage specification in two ways:
// - They tell us they are using transport protocol CBI. In reality they
// are using transport protocol CB.
// - They don't like the INQUIRY command. So we must handle this command
// of the SCSI layer ourselves.
// - Some cameras with idProduct=0x1001 and bcdDevice=0x1000 have
// bInterfaceProtocol=0x00 (USB_PR_CBI) while others have 0x01 (USB_PR_CB).
// So don't remove the USB_PR_CB override!
// - Cameras with bcdDevice=0x9009 require the USB_SC_8070 override.
//
// Submitted by Oleksandr Chumachenko <ledest@gmail.com>
// Submitted by Hartmut Wahl <hwahl@hwahl.de>
// Reported by Luciano Rocha <luciano@eurotux.com>
// Reported and patched by Nguyen Anh Quynh <aquynh@gmail.com>
// Reported by Martijn Hijdra <martijn.hijdra@gmail.com>
// Supplied with some Castlewood ORB removable drives
//
// Entry and supporting patch by Theodore Kilgore <kilgota@auburn.edu>.
// Flag will support Bulk devices which use a standards-violating 32-byte
// Command Block Wrapper. Here, the "DC2MEGA" cameras (several brands) with
// Grandtech GT892x chip, which request "Proprietary SCSI Bulk" support.
//
// Reported by <ttkspam@free.fr>
// The device reports a vendor-specific device class, requiring an
// explicit vendor/product match.
//
// Andrew Lunn <andrew@lunn.ch>
// PanDigital Digital Picture Frame. Does not like ALLOW_MEDIUM_REMOVAL
// on LUN 4.
// Note: Vend:Prod clash with "Ltd Maxell WS30 Slim Digital Camera"
//
// Submitted by Jan De Luyck <lkml@kcore.org>
//
// Submitted by Dylan Taft <d13f00l@gmail.com>
// US_FL_IGNORE_RESIDUE Needed
//
// Entry needed for flags. Moreover, all devices with this ID use
// bulk-only transport, but _some_ falsely report Control/Bulk instead.
// One example is "Trumpion Digital Research MYMP3".
// Submitted by Bjoern Brill <brill(at)fs.math.uni-frankfurt.de>
//
// Reported by Filippo Bardelli <filibard@libero.it>
// The device reports a subclass of RBC, which is wrong.
//
// Trumpion Microelectronics MP3 player (felipe_alfaro@linuxmail.org)
// aeb
//
// Reported by Icenowy Zheng <icenowy@aosc.io>
// The SMI SM3350 USB-UFS bridge controller will enter a wrong state
// that do not process read/write command if a long sense is requested,
// so force to use 18-byte sense.
//
// Reported by Paul Hartman <paul.hartman+linux@gmail.com>
// This card reader returns "Illegal Request, Logical Block Address
// Out of Range" for the first READ(10) after a new card is inserted.
//
// Patch by Tasos Sahanidis <tasos@tasossah.com>
// This flash drive always shows up with write protect enabled
// during the first mode sense.
//
// This Pentax still camera is not conformant
// to the USB storage specification: -
// - It does not like the INQUIRY command. So we must handle this command
// of the SCSI layer ourselves.
// Tested on Rev. 10.00 (0x1000)
// Submitted by James Courtier-Dutton <James@superbug.demon.co.uk>
//
// These are virtual windows driver CDs, which the zd1211rw driver
// automatically converts into WLAN devices.
//
// Reported by Dan Williams <dcbw@redhat.com>
// Option N.V. mobile broadband modems
// Ignore driver CD mode and force into modem mode by default.
//
// iCON 225
//
// Reported by F. Aben <f.aben@option.com>
// This device (wrongly) has a vendor-specific device descriptor.
// The entry is needed so usb-storage can bind to it's mass-storage
// interface as an interface driver
//
// Reported by Jan Dumon <j.dumon@option.com>
// These devices (wrongly) have a vendor-specific device descriptor.
// These entries are needed so usb-storage can bind to their mass-storage
// interface as an interface driver
//
// Reported by Namjae Jeon <namjae.jeon@samsung.com>
// Reported by Ben Efros <ben@pc-doctor.com>
// Reported by Kris Lindgren <kris.lindgren@gmail.com>
//
// Reported by Zenm Chen <zenmchen@gmail.com>
// Ignore driver CD mode, otherwise usb_modeswitch may fail to switch
// the device into Wi-Fi mode.
//
// Reported by Zenm Chen <zenmchen@gmail.com>
// Ignore driver CD mode, otherwise usb_modeswitch may fail to switch
// the device into Wi-Fi mode.
//
// Pete Zaitcev <zaitcev@yahoo.com>, bz#164688.
// The device blatantly ignores LUN and returns 1 in GetMaxLUN.
//
// Submitted by Joris Struyve <joris@struyve.be>
//
// Entry for Jenoptik JD 5200z3
//
// email: car.busse@gmx.de
//
// Reported by  Jason Johnston <killean@shaw.ca>
//
// Reported by Lubomir Blaha <tritol@trilogic.cz>
// I _REALLY_ don't know what 3rd, 4th number and all defines mean, but this
// works for me. Can anybody correct these values? (I able to test corrected
// version.)
//
// Reported by Edward Chapman (taken from linux-usb mailing list)
// Netac OnlyDisk Mini U2CV2 512MB USB 2.0 Flash Drive
//
// Patch by Stephan Walter <stephan.walter@epfl.ch>
// I don't know why, but it works...
//
// Reported by Ian McConnell <ian at emit.demon.co.uk>
// Reported by Jim McCloskey <mcclosk@ucsc.edu>
// Submitted by Antoine Mairesse <antoine.mairesse@free.fr>
//
// Submitted by Daniel Drake <dsd@gentoo.org>
// Reported by dayul on the Gentoo Forums
//
// Reported by Rastislav Stanik <rs_kernel@yahoo.com>
//
// Reported by Benjamin Schiller <sbenni@gmx.de>
// It is also sold by Easylite as DJ 20
//
// Patch by Leonid Petrov mail at lpetrov.net
// Reported by Robert Spitzenpfeil <robert@spitzenpfeil.org>
// http://www.qbik.ch/usb/devices/showdev.php?id=1705
// Updated to 103 device by MJ Ray mjr at phonecoop.coop
//
// David Kuehling <dvdkhlng@gmx.de>:
// for MP3-Player AVOX WSX-300ER (bought in Japan).  Reports lots of SCSI
// errors when trying to write.
//
// Submitted by Nick Holloway
// Reported by Moritz Moeller-Herrmann <moritz-kernel@moeller-herrmann.de>
// Reported by Michael Stattmann <michael@stattmann.com>
// Reported by The Solutor <thesolutor@gmail.com>
//
// Reported by Jan Mate <mate@fiit.stuba.sk>
// and by Soeren Sonnenburg <kernel@nn7.de>
//
// Reported by Emmanuel Vasilakis <evas@forthnet.gr>
// Reported by Ricardo Barberis <ricardo@dattatec.com>
//
// Reported by Kevin Cernekee <kpc-usbdev@gelato.uiuc.edu>
// Tested on hardware version 1.10.
// Entry is needed only for the initializer function override.
// Devices with bcd > 110 seem to not need it while those
// with bcd < 110 appear to need it.
//
// Reported by Namjae Jeon <namjae.jeon@samsung.com>
//
// Reported by Fabio Venturi <f.venturi@tdnet.it>
// The device reports a vendor-specific bDeviceClass.
//
// Reported by Pascal Terjan <pterjan@mandriva.com>
// Ignore driver CD mode and force into modem mode by default.
//
// Reported by Kevin Lloyd <linux@sierrawireless.com>
// Entry is needed for the initializer function override,
// which instructs the device to load as a modem
// device.
//
// Reported by Jaco Kroon <jaco@kroon.co.za>
// The usb-storage module found on the Digitech GNX4 (and supposedly other
// devices) misbehaves and causes a bunch of invalid I/O errors.
//
// Reported by fangxiaozhi <huananhu@huawei.com>
// This brings the HUAWEI data card devices into multi-port mode
//
// Reported by Vilius Bilinkevicius <vilisas AT xxx DOT lt)
// Reported by Kotrla Vitezslav <kotrla@ceb.cz>
//
// Reported by Tobias Jakobi <tjakobi@math.uni-bielefeld.de>
// The INIC-3619 bridge is used in the StarTech SLSODDU33B
// SATA-USB enclosure for slimline optical drives.
//
// The quirk enables MakeMKV to properly exchange keys with
// an installed BD drive.
//
// Reported by Qinglin Ye <yestyle@gmail.com>
// Reported by Francesco Foresti <frafore@tiscali.it>
// Reported by Michael Büsch <m@bues.ch>
// Reported by David Kozub <zub@linux.fjfi.cvut.cz>
//
// Reported by Alexandre Oliva <oliva@lsd.ic.unicamp.br>
// JMicron responds to USN and several other SCSI ioctls with a
// residue that causes subsequent I/O requests to fail.
// Reported by Dmitry Nezhevenko <dion@dion.org.ua>
// Reported by Teijo Kinnunen <teijo.kinnunen@code-q.fi>
// Reported-by George Cherian <george.cherian@cavium.com>
//
// Entrega Technologies U1-SC25 (later Xircom PortGear PGSCSI)
// and Mac USB Dock USB-SCSI
//
// Reported by Robert Schedel <r.schedel@yahoo.de>
// Note: this is a 'super top' device like the above 14cd/6600 device
//
// Reported by Oliver Neukum <oneukum@suse.com>
// Reported by Jesse Feddema <jdfeddema@gmail.com>
//
// Reported by Hans de Goede <hdegoede@redhat.com>
// These Appotech controllers are found in Picture Frames, they provide a
// (buggy) emulation of a cdrom drive which contains the windows software
// Uploading of pictures happens over the corresponding /dev/sg device.
//
// Reported by Matthias Schwarzott <zzam@gentoo.org>
// The Amazon Kindle treats SYNCHRONIZE CACHE as an indication that
// the host may be finished with it, and automatically ejects its
// emulated media unless it receives another command within one second.
//
// Reported by Oliver Neukum <oneukum@suse.com>
// This device morphes spontaneously into another device if the access
// pattern of Windows isn't followed. Thus writable media would be dirty
// if the initial instance is used. So the device is limited to its
// virtual CD.
// And yes, the concept that BCD goes up to 9 is not heeded
//
// Reported by Sven Geggus <sven-usbst@geggus.net>
// This encrypted pen drive returns bogus data for the initial READ(10).
//
// Reported by Hans de Goede <hdegoede@redhat.com>
// These are mini projectors using USB for both power and video data transport
// The usb-storage interface is a virtual windows driver CD, which the gm12u320
// driver automatically converts into framebuffer & kms dri device nodes.
//
// Patch by Richard Schütz <r.schtz@t-online.de>
// This external hard drive enclosure uses a JMicron chip which
// needs the US_FL_IGNORE_RESIDUE flag to work properly.
//
// Reported by Jasper Mackenzie <scarletpimpernal@hotmail.com>
// Reported by Witold Lipieta <witold.lipieta@thaumatec.com>
// Supplied with some Castlewood ORB removable drives
//
// Reported by DocMAX <mail@vacharakis.de>,
// Thomas Weißschuh <linux@weissschuh.net>
// and Daniel Brát <danek.brat@gmail.com>
//
// patch submitted by Davide Perini <perini.davide@dpsoftware.org>
// and Renato Perini <rperini@email.it>
//
// Patch by Constantin Baranov <const@tltsu.ru>
// Report by Andreas Koenecke.
// Motorola ROKR Z6.
//
// Reported by Radovan Garabik <garabik@kassiopeia.juls.savba.sk>
// Reported-by: Tim Anderson <tsa@biglakesoftware.com>
//
// Reported by Frederic Marchal <frederic.marchal@wowcompany.com>
// Mio Moov 330
//
// Reported by Cyril Roelandt <tipecaml@gmail.com>
// Reported by Andrey Rahmatullin <wrar@altlinux.org>
// Reported by Sergey Pinaev <dfo@antex.ru>
//
// David Härdeman <david@2gen.com>
// The key makes the SCSI stack print confusing (but harmless) messages
//
// "G-DRIVE" external HDD hangs on write without these.
// Patch submitted by Alexander Kappner <agk@godking.net>
//
// Nick Bowler <nbowler@elliptictech.com>
// SCSI stack spams (otherwise harmless) error messages.
//
// Reported by Icenowy Zheng <uwu@icenowy.me>
// This is an interface for vendor-specific cryptic commands instead
// of real USB storage device.
//
// Reported by Andrew Simmons <andrew.simmons@gmail.com>
// Reported by Alessio Treglia <quadrispro@ubuntu.com>
// Unusual uas devices

// Control/Bulk transport for all SubClass values
// Control/Bulk/Interrupt transport for all SubClass values
// Bulk-only transport for all SubClass values
