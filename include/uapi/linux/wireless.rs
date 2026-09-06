//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/wireless.h
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
// This file define a set of standard wireless extensions
//
// Version :	22	16.3.07
//
// Authors :	Jean Tourrilhes - HPL - <jt@hpl.hp.com>
// Copyright (c) 1997-2007 Jean Tourrilhes, All Rights Reserved.
//
// DOCUMENTATION
//
// Initial APIs (1996 -> onward) :
// -----------------------------
// Basically, the wireless extensions are for now a set of standard ioctl
// call + /proc/net/wireless
//
// The entry /proc/net/wireless give statistics and information on the
// driver.
// This is better than having each driver having its entry because
// its centralised and we may remove the driver module safely.
//
// Ioctl are used to configure the driver and issue commands.  This is
// better than command line options of insmod because we may want to
// change dynamically (while the driver is running) some parameters.
//
// The ioctl mechanimsm are copied from standard devices ioctl.
// We have the list of command plus a structure descibing the
// data exchanged...
// Note that to add these ioctl, I was obliged to modify :
// # net/core/dev.c (two place + add include)
// # net/ipv4/af_inet.c (one place + add include)
//
// /proc/net/wireless is a copy of /proc/net/dev.
// We have a structure for data passed from the driver to /proc/net/wireless
// Too add this, I've modified :
// # net/core/dev.c (two other places)
// # include/linux/netdevice.h (one place)
// # include/linux/proc_fs.h (one place)
//
// New driver API (2002 -> onward) :
// -------------------------------
// This file is only concerned with the user space API and common definitions.
// The new driver API is defined and documented in :
// # include/net/iw_handler.h
//
// Note as well that /proc/net/wireless implementation has now moved in :
// # net/core/wireless.c
//
// Wireless Events (2002 -> onward) :
// --------------------------------
// Events are defined at the end of this file, and implemented in :
// # net/core/wireless.c
//
// Other comments :
// --------------
// Do not add here things that are redundant with other mechanisms
// (drivers init, ifconfig, /proc/net/dev, ...) and with are not
// wireless specific.
//
// These wireless extensions are not magic : each driver has to provide
// support for them...
//
// IMPORTANT NOTE : As everything in the kernel, this is very much a
// work in progress. Contact me if you have ideas of improvements...
//
// INCLUDES

// VERSION
//
// This constant is used to know the availability of the wireless
// extensions and to know which version of wireless extensions it is
// (there is some stuff that will be added in the future...)
// I just plan to increment with each new version.
//
pub const WIRELESS_EXT: c_int = 22;
//
// Changes :
//
// V2 to V3
// --------
// Alan Cox start some incompatibles changes. I've integrated a bit more.
// - Encryption renamed to Encode to avoid US regulation problems
// - Frequency changed from float to struct to avoid problems on old 386
//
// V3 to V4
// --------
// - Add sensitivity
//
// V4 to V5
// --------
// - Missing encoding definitions in range
// - Access points stuff
//
// V5 to V6
// --------
// - 802.11 support (ESSID ioctls)
//
// V6 to V7
// --------
// - define IW_ESSID_MAX_SIZE and IW_MAX_AP
//
// V7 to V8
// --------
// - Changed my e-mail address
// - More 802.11 support (nickname, rate, rts, frag)
// - List index in frequencies
//
// V8 to V9
// --------
// - Support for 'mode of operation' (ad-hoc, managed...)
// - Support for unicast and multicast power saving
// - Change encoding to support larger tokens (>64 bits)
// - Updated iw_params (disable, flags) and use it for NWID
// - Extracted iw_point from iwreq for clarity
//
// V9 to V10
// ---------
// - Add PM capability to range structure
// - Add PM modifier : MAX/MIN/RELATIVE
// - Add encoding option : IW_ENCODE_NOKEY
// - Add TxPower ioctls (work like TxRate)
//
// V10 to V11
// ----------
// - Add WE version in range (help backward/forward compatibility)
// - Add retry ioctls (work like PM)
//
// V11 to V12
// ----------
// - Add SIOCSIWSTATS to get /proc/net/wireless programatically
// - Add DEV PRIVATE IOCTL to avoid collisions in SIOCDEVPRIVATE space
// - Add new statistics (frag, retry, beacon)
// - Add average quality (for user space calibration)
//
// V12 to V13
// ----------
// - Document creation of new driver API.
// - Extract union iwreq_data from struct iwreq (for new driver API).
// - Rename SIOCSIWNAME as SIOCSIWCOMMIT
//
// V13 to V14
// ----------
// - Wireless Events support : define struct iw_event
// - Define additional specific event numbers
// - Add "addr" and "param" fields in union iwreq_data
// - AP scanning stuff (SIOCSIWSCAN and friends)
//
// V14 to V15
// ----------
// - Add IW_PRIV_TYPE_ADDR for struct sockaddr private arg
// - Make struct iw_freq signed (both m & e), add explicit padding
// - Add IWEVCUSTOM for driver specific event/scanning token
// - Add IW_MAX_GET_SPY for driver returning a lot of addresses
// - Add IW_TXPOW_RANGE for range of Tx Powers
// - Add IWEVREGISTERED & IWEVEXPIRED events for Access Points
// - Add IW_MODE_MONITOR for passive monitor
//
// V15 to V16
// ----------
// - Increase the number of bitrates in iw_range to 32 (for 802.11g)
// - Increase the number of frequencies in iw_range to 32 (for 802.11b+a)
// - Reshuffle struct iw_range for increases, add filler
// - Increase IW_MAX_AP to 64 for driver returning a lot of addresses
// - Remove IW_MAX_GET_SPY because conflict with enhanced spy support
// - Add SIOCSIWTHRSPY/SIOCGIWTHRSPY and "struct iw_thrspy"
// - Add IW_ENCODE_TEMP and iw_range->encoding_login_index
//
// V16 to V17
// ----------
// - Add flags to frequency -> auto/fixed
// - Document (struct iw_quality *)->updated, add new flags (INVALID)
// - Wireless Event capability in struct iw_range
// - Add support for relative TxPower (yick !)
//
// V17 to V18 (From Jouni Malinen <j@w1.fi>)
// ----------
// - Add support for WPA/WPA2
// - Add extended encoding configuration (SIOCSIWENCODEEXT and
// SIOCGIWENCODEEXT)
// - Add SIOCSIWGENIE/SIOCGIWGENIE
// - Add SIOCSIWMLME
// - Add SIOCSIWPMKSA
// - Add struct iw_range bit field for supported encoding capabilities
// - Add optional scan request parameters for SIOCSIWSCAN
// - Add SIOCSIWAUTH/SIOCGIWAUTH for setting authentication and WPA
// related parameters (extensible up to 4096 parameter values)
// - Add wireless events: IWEVGENIE, IWEVMICHAELMICFAILURE,
// IWEVASSOCREQIE, IWEVASSOCRESPIE, IWEVPMKIDCAND
//
// V18 to V19
// ----------
// - Remove (struct iw_point *)->pointer from events and streams
// - Remove header includes to help user space
// - Increase IW_ENCODING_TOKEN_MAX from 32 to 64
// - Add IW_QUAL_ALL_UPDATED and IW_QUAL_ALL_INVALID macros
// - Add explicit flag to tell stats are in dBm : IW_QUAL_DBM
// - Add IW_IOCTL_IDX() and IW_EVENT_IDX() macros
//
// V19 to V20
// ----------
// - RtNetlink requests support (SET/GET)
//
// V20 to V21
// ----------
// - Remove (struct net_device *)->get_wireless_stats()
// - Change length in ESSID and NICK to strlen() instead of strlen()+1
// - Add IW_RETRY_SHORT/IW_RETRY_LONG retry modifiers
// - Power/Retry relative values no longer * 100000
// - Add explicit flag to tell stats are in 802.11k RCPI : IW_QUAL_RCPI
//
// V21 to V22
// ----------
// - Prevent leaking of kernel space in stream on 64 bits.
//
// CONSTANTS
// -------------------------- IOCTL LIST --------------------------
// Wireless Identification
pub const SIOCSIWCOMMIT: c_uint = 0x8B00		/* Commit pending changes to driver */;
pub const SIOCGIWNAME: c_uint = 0x8B01		/* get name == wireless protocol */;
// SIOCGIWNAME is used to verify the presence of Wireless Extensions.
// Common values : "IEEE 802.11-DS", "IEEE 802.11-FH", "IEEE 802.11b"...
// Don't put the name of your driver there, it's useless.
// Basic operations
pub const SIOCSIWNWID: c_uint = 0x8B02		/* set network id (pre-802.11) */;
pub const SIOCGIWNWID: c_uint = 0x8B03		/* get network id (the cell) */;
pub const SIOCSIWFREQ: c_uint = 0x8B04		/* set channel/frequency (Hz) */;
pub const SIOCGIWFREQ: c_uint = 0x8B05		/* get channel/frequency (Hz) */;
pub const SIOCSIWMODE: c_uint = 0x8B06		/* set operation mode */;
pub const SIOCGIWMODE: c_uint = 0x8B07		/* get operation mode */;
pub const SIOCSIWSENS: c_uint = 0x8B08		/* set sensitivity (dBm) */;
pub const SIOCGIWSENS: c_uint = 0x8B09		/* get sensitivity (dBm) */;
// Informative stuff
pub const SIOCSIWRANGE: c_uint = 0x8B0A		/* Unused */;
pub const SIOCGIWRANGE: c_uint = 0x8B0B		/* Get range of parameters */;
pub const SIOCSIWPRIV: c_uint = 0x8B0C		/* Unused */;
pub const SIOCGIWPRIV: c_uint = 0x8B0D		/* get private ioctl interface info */;
pub const SIOCSIWSTATS: c_uint = 0x8B0E		/* Unused */;
pub const SIOCGIWSTATS: c_uint = 0x8B0F		/* Get /proc/net/wireless stats */;
// SIOCGIWSTATS is strictly used between user space and the kernel, and
// is never passed to the driver (i.e. the driver will never see it).
// Spy support (statistics per MAC address - used for Mobile IP support)
pub const SIOCSIWSPY: c_uint = 0x8B10		/* set spy addresses */;
pub const SIOCGIWSPY: c_uint = 0x8B11		/* get spy info (quality of link) */;
pub const SIOCSIWTHRSPY: c_uint = 0x8B12		/* set spy threshold (spy event) */;
pub const SIOCGIWTHRSPY: c_uint = 0x8B13		/* get spy threshold */;
// Access Point manipulation
pub const SIOCSIWAP: c_uint = 0x8B14		/* set access point MAC addresses */;
pub const SIOCGIWAP: c_uint = 0x8B15		/* get access point MAC addresses */;
pub const SIOCGIWAPLIST: c_uint = 0x8B17		/* Deprecated in favor of scanning */;
pub const SIOCSIWSCAN: c_uint = 0x8B18		/* trigger scanning (list cells) */;
pub const SIOCGIWSCAN: c_uint = 0x8B19		/* get scanning results */;
// 802.11 specific support
pub const SIOCSIWESSID: c_uint = 0x8B1A		/* set ESSID (network name) */;
pub const SIOCGIWESSID: c_uint = 0x8B1B		/* get ESSID */;
pub const SIOCSIWNICKN: c_uint = 0x8B1C		/* set node name/nickname */;
pub const SIOCGIWNICKN: c_uint = 0x8B1D		/* get node name/nickname */;
// As the ESSID and NICKN are strings up to 32 bytes long, it doesn't fit
// within the 'iwreq' structure, so we need to use the 'data' member to
// point to a string in user space, like it is done for RANGE...
// Other parameters useful in 802.11 and some other devices
pub const SIOCSIWRATE: c_uint = 0x8B20		/* set default bit rate (bps) */;
pub const SIOCGIWRATE: c_uint = 0x8B21		/* get default bit rate (bps) */;
pub const SIOCSIWRTS: c_uint = 0x8B22		/* set RTS/CTS threshold (bytes) */;
pub const SIOCGIWRTS: c_uint = 0x8B23		/* get RTS/CTS threshold (bytes) */;
pub const SIOCSIWFRAG: c_uint = 0x8B24		/* set fragmentation thr (bytes) */;
pub const SIOCGIWFRAG: c_uint = 0x8B25		/* get fragmentation thr (bytes) */;
pub const SIOCSIWTXPOW: c_uint = 0x8B26		/* set transmit power (dBm) */;
pub const SIOCGIWTXPOW: c_uint = 0x8B27		/* get transmit power (dBm) */;
pub const SIOCSIWRETRY: c_uint = 0x8B28		/* set retry limits and lifetime */;
pub const SIOCGIWRETRY: c_uint = 0x8B29		/* get retry limits and lifetime */;
// Encoding stuff (scrambling, hardware security, WEP...)
pub const SIOCSIWENCODE: c_uint = 0x8B2A		/* set encoding token & mode */;
pub const SIOCGIWENCODE: c_uint = 0x8B2B		/* get encoding token & mode */;
// Power saving stuff (power management, unicast and multicast)
pub const SIOCSIWPOWER: c_uint = 0x8B2C		/* set Power Management settings */;
pub const SIOCGIWPOWER: c_uint = 0x8B2D		/* get Power Management settings */;
// WPA : Generic IEEE 802.11 informatiom element (e.g., for WPA/RSN/WMM).
// This ioctl uses struct iw_point and data buffer that includes IE id and len
// fields. More than one IE may be included in the request. Setting the generic
// IE to empty buffer (len=0) removes the generic IE from the driver. Drivers
// are allowed to generate their own WPA/RSN IEs, but in these cases, drivers
// are required to report the used IE as a wireless event, e.g., when
// associating with an AP.
pub const SIOCSIWGENIE: c_uint = 0x8B30		/* set generic IE */;
pub const SIOCGIWGENIE: c_uint = 0x8B31		/* get generic IE */;
// WPA : IEEE 802.11 MLME requests
pub const SIOCSIWMLME: c_uint = 0x8B16		/* request MLME operation; uses;
// struct iw_mlme
// WPA : Authentication mode parameters
pub const SIOCSIWAUTH: c_uint = 0x8B32		/* set authentication mode params */;
pub const SIOCGIWAUTH: c_uint = 0x8B33		/* get authentication mode params */;
// WPA : Extended version of encoding configuration
pub const SIOCSIWENCODEEXT: c_uint = 0x8B34		/* set encoding token & mode */;
pub const SIOCGIWENCODEEXT: c_uint = 0x8B35		/* get encoding token & mode */;
// WPA2 : PMKSA cache management
pub const SIOCSIWPMKSA: c_uint = 0x8B36		/* PMKSA cache operation */;
// -------------------- DEV PRIVATE IOCTL LIST --------------------
// These 32 ioctl are wireless device private, for 16 commands.
// Each driver is free to use them for whatever purpose it chooses,
// however the driver *must* export the description of those ioctls
// with SIOCGIWPRIV and *must* use arguments as defined below.
// If you don't follow those rules, DaveM is going to hate you (reason :
// it make mixed 32/64bit operation impossible).
//
pub const SIOCIWFIRSTPRIV: c_uint = 0x8BE0;
pub const SIOCIWLASTPRIV: c_uint = 0x8BFF;
// Previously, we were using SIOCDEVPRIVATE, but we now have our
// separate range because of collisions with other tools such as
// 'mii-tool'.
// We now have 32 commands, so a bit more space ;-).
// Also, all 'even' commands are only usable by root and don't return the
// content of ifr/iwr to user (but you are not obliged to use the set/get
// convention, just use every other two command). More details in iwpriv.c.
// And I repeat : you are not forced to use them with iwpriv, but you
// must be compliant with it.
//
// ------------------------- IOCTL STUFF -------------------------
// The first and the last (range)
pub const SIOCIWFIRST: c_uint = 0x8B00;

// Odd : get (world access), even : set (root access)

// ----------------------- WIRELESS EVENTS -----------------------
// Those are *NOT* ioctls, do not issue request on them !!!
// Most events use the same identifier as ioctl requests
pub const IWEVTXDROP: c_uint = 0x8C00		/* Packet dropped to excessive retry */;
pub const IWEVQUAL: c_uint = 0x8C01		/* Quality part of statistics (scan) */;
pub const IWEVCUSTOM: c_uint = 0x8C02		/* Driver specific ascii string */;
pub const IWEVREGISTERED: c_uint = 0x8C03		/* Discovered a new node (AP mode) */;
pub const IWEVEXPIRED: c_uint = 0x8C04		/* Expired a node (AP mode) */;
pub const IWEVGENIE: c_uint = 0x8C05		/* Generic IE (WPA, RSN, WMM, ..);
// (scan results); This includes id and
// length fields. One IWEVGENIE may
// contain more than one IE. Scan
// results may contain one or more
// IWEVGENIE events.
pub const IWEVMICHAELMICFAILURE: c_uint = 0x8C06	/* Michael MIC failure;
// (struct iw_michaelmicfailure)
//
pub const IWEVASSOCREQIE: c_uint = 0x8C07		/* IEs used in (Re)Association Request.;
// The data includes id and length
// fields and may contain more than one
// IE. This event is required in
// Managed mode if the driver
// generates its own WPA/RSN IE. This
// should be sent just before
// IWEVREGISTERED event for the
// association.
pub const IWEVASSOCRESPIE: c_uint = 0x8C08		/* IEs used in (Re)Association;
// Response. The data includes id and
// length fields and may contain more
// than one IE. This may be sent
// between IWEVASSOCREQIE and
// IWEVREGISTERED events for the
// association.
pub const IWEVPMKIDCAND: c_uint = 0x8C09		/* PMKID candidate for RSN;
// pre-authentication
// (struct iw_pmkid_cand)
pub const IWEVFIRST: c_uint = 0x8C00;

// ------------------------- PRIVATE INFO -------------------------
//
// The following is used with SIOCGIWPRIV. It allow a driver to define
// the interface (name, type of data) for its private ioctl.
// Privates ioctl are SIOCIWFIRSTPRIV -> SIOCIWLASTPRIV
//
pub const IW_PRIV_TYPE_MASK: c_uint = 0x7000	/* Type of arguments */;
pub const IW_PRIV_TYPE_NONE: c_uint = 0x0000;
pub const IW_PRIV_TYPE_BYTE: c_uint = 0x1000	/* Char as number */;
pub const IW_PRIV_TYPE_CHAR: c_uint = 0x2000	/* Char as character */;
pub const IW_PRIV_TYPE_INT: c_uint = 0x4000	/* 32 bits int */;
pub const IW_PRIV_TYPE_FLOAT: c_uint = 0x5000	/* struct iw_freq */;
pub const IW_PRIV_TYPE_ADDR: c_uint = 0x6000	/* struct sockaddr */;
pub const IW_PRIV_SIZE_FIXED: c_uint = 0x0800	/* Variable or fixed number of args */;
pub const IW_PRIV_SIZE_MASK: c_uint = 0x07FF	/* Max number of those args */;
//
// Note : if the number of args is fixed and the size < 16 octets,
// instead of passing a pointer we will put args in the iwreq struct...
//
// ----------------------- OTHER CONSTANTS -----------------------
// Maximum frequencies in the range struct
pub const IW_MAX_FREQUENCIES: c_int = 32;
// Note : if you have something like 80 frequencies,
// don't increase this constant and don't fill the frequency list.
// The user will be able to set by channel anyway...
// Maximum bit rates in the range struct
pub const IW_MAX_BITRATES: c_int = 32;
// Maximum tx powers in the range struct
pub const IW_MAX_TXPOWER: c_int = 8;
// Note : if you more than 8 TXPowers, just set the max and min or
// a few of them in the struct iw_range.
// Maximum of address that you may set with SPY
pub const IW_MAX_SPY: c_int = 8;
// Maximum of address that you may get in the
pub const IW_MAX_AP: c_int = 64;
// Maximum size of the ESSID and NICKN strings
pub const IW_ESSID_MAX_SIZE: c_int = 32;
// Modes of operation

// Statistics flags (bitmask in updated)
pub const IW_QUAL_QUAL_UPDATED: c_uint = 0x01	/* Value was updated since last read */;
pub const IW_QUAL_LEVEL_UPDATED: c_uint = 0x02;
pub const IW_QUAL_NOISE_UPDATED: c_uint = 0x04;
pub const IW_QUAL_ALL_UPDATED: c_uint = 0x07;
pub const IW_QUAL_DBM: c_uint = 0x08	/* Level + Noise are dBm */;
pub const IW_QUAL_QUAL_INVALID: c_uint = 0x10	/* Driver doesn't provide value */;
pub const IW_QUAL_LEVEL_INVALID: c_uint = 0x20;
pub const IW_QUAL_NOISE_INVALID: c_uint = 0x40;
pub const IW_QUAL_RCPI: c_uint = 0x80	/* Level + Noise are 802.11k RCPI */;
pub const IW_QUAL_ALL_INVALID: c_uint = 0x70;
// Frequency flags
pub const IW_FREQ_AUTO: c_uint = 0x00	/* Let the driver decides */;
pub const IW_FREQ_FIXED: c_uint = 0x01	/* Force a specific value */;
// Maximum number of size of encoding token available
// they are listed in the range structure
pub const IW_MAX_ENCODING_SIZES: c_int = 8;
// Maximum size of the encoding token in bytes

// Flags for encoding (along with the token)
pub const IW_ENCODE_INDEX: c_uint = 0x00FF	/* Token index (if needed) */;
pub const IW_ENCODE_FLAGS: c_uint = 0xFF00	/* Flags defined below */;
pub const IW_ENCODE_MODE: c_uint = 0xF000	/* Modes defined below */;
pub const IW_ENCODE_DISABLED: c_uint = 0x8000	/* Encoding disabled */;
pub const IW_ENCODE_ENABLED: c_uint = 0x0000	/* Encoding enabled */;
pub const IW_ENCODE_RESTRICTED: c_uint = 0x4000	/* Refuse non-encoded packets */;
pub const IW_ENCODE_OPEN: c_uint = 0x2000	/* Accept non-encoded packets */;
pub const IW_ENCODE_NOKEY: c_uint = 0x0800  /* Key is write only, so not present */;
pub const IW_ENCODE_TEMP: c_uint = 0x0400  /* Temporary key */;
// Power management flags available (along with the value, if any)
pub const IW_POWER_ON: c_uint = 0x0000	/* No details... */;
pub const IW_POWER_TYPE: c_uint = 0xF000	/* Type of parameter */;
pub const IW_POWER_PERIOD: c_uint = 0x1000	/* Value is a period/duration of  */;
pub const IW_POWER_TIMEOUT: c_uint = 0x2000	/* Value is a timeout (to go asleep) */;
pub const IW_POWER_MODE: c_uint = 0x0F00	/* Power Management mode */;
pub const IW_POWER_UNICAST_R: c_uint = 0x0100	/* Receive only unicast messages */;
pub const IW_POWER_MULTICAST_R: c_uint = 0x0200	/* Receive only multicast messages */;
pub const IW_POWER_ALL_R: c_uint = 0x0300	/* Receive all messages though PM */;
pub const IW_POWER_FORCE_S: c_uint = 0x0400	/* Force PM procedure for sending unicast */;
pub const IW_POWER_REPEATER: c_uint = 0x0800	/* Repeat broadcast messages in PM period */;
pub const IW_POWER_MODIFIER: c_uint = 0x000F	/* Modify a parameter */;
pub const IW_POWER_MIN: c_uint = 0x0001	/* Value is a minimum  */;
pub const IW_POWER_MAX: c_uint = 0x0002	/* Value is a maximum */;
pub const IW_POWER_RELATIVE: c_uint = 0x0004	/* Value is not in seconds/ms/us */;
// Transmit Power flags available
pub const IW_TXPOW_TYPE: c_uint = 0x00FF	/* Type of value */;
pub const IW_TXPOW_DBM: c_uint = 0x0000	/* Value is in dBm */;
pub const IW_TXPOW_MWATT: c_uint = 0x0001	/* Value is in mW */;
pub const IW_TXPOW_RELATIVE: c_uint = 0x0002	/* Value is in arbitrary units */;
pub const IW_TXPOW_RANGE: c_uint = 0x1000	/* Range of value between min/max */;
// Retry limits and lifetime flags available
pub const IW_RETRY_ON: c_uint = 0x0000	/* No details... */;
pub const IW_RETRY_TYPE: c_uint = 0xF000	/* Type of parameter */;
pub const IW_RETRY_LIMIT: c_uint = 0x1000	/* Maximum number of retries*/;
pub const IW_RETRY_LIFETIME: c_uint = 0x2000	/* Maximum duration of retries in us */;
pub const IW_RETRY_MODIFIER: c_uint = 0x00FF	/* Modify a parameter */;
pub const IW_RETRY_MIN: c_uint = 0x0001	/* Value is a minimum  */;
pub const IW_RETRY_MAX: c_uint = 0x0002	/* Value is a maximum */;
pub const IW_RETRY_RELATIVE: c_uint = 0x0004	/* Value is not in seconds/ms/us */;
pub const IW_RETRY_SHORT: c_uint = 0x0010	/* Value is for short packets  */;
pub const IW_RETRY_LONG: c_uint = 0x0020	/* Value is for long packets */;
// Scanning request flags
pub const IW_SCAN_DEFAULT: c_uint = 0x0000	/* Default scan of the driver */;
pub const IW_SCAN_ALL_ESSID: c_uint = 0x0001	/* Scan all ESSIDs */;
pub const IW_SCAN_THIS_ESSID: c_uint = 0x0002	/* Scan only this ESSID */;
pub const IW_SCAN_ALL_FREQ: c_uint = 0x0004	/* Scan all Frequencies */;
pub const IW_SCAN_THIS_FREQ: c_uint = 0x0008	/* Scan only this Frequency */;
pub const IW_SCAN_ALL_MODE: c_uint = 0x0010	/* Scan all Modes */;
pub const IW_SCAN_THIS_MODE: c_uint = 0x0020	/* Scan only this Mode */;
pub const IW_SCAN_ALL_RATE: c_uint = 0x0040	/* Scan all Bit-Rates */;
pub const IW_SCAN_THIS_RATE: c_uint = 0x0080	/* Scan only this Bit-Rate */;
// struct iw_scan_req scan_type
pub const IW_SCAN_TYPE_ACTIVE: c_int = 0;
pub const IW_SCAN_TYPE_PASSIVE: c_int = 1;
// Maximum size of returned data

// Scan capability flags - in (struct iw_range *)->scan_capa
pub const IW_SCAN_CAPA_NONE: c_uint = 0x00;
pub const IW_SCAN_CAPA_ESSID: c_uint = 0x01;
pub const IW_SCAN_CAPA_BSSID: c_uint = 0x02;
pub const IW_SCAN_CAPA_CHANNEL: c_uint = 0x04;
pub const IW_SCAN_CAPA_MODE: c_uint = 0x08;
pub const IW_SCAN_CAPA_RATE: c_uint = 0x10;
pub const IW_SCAN_CAPA_TYPE: c_uint = 0x20;
pub const IW_SCAN_CAPA_TIME: c_uint = 0x40;
// Max number of char in custom event - use multiple of them if needed

// Generic information element
pub const IW_GENERIC_IE_MAX: c_int = 1024;
// MLME requests (SIOCSIWMLME / struct iw_mlme)
pub const IW_MLME_DEAUTH: c_int = 0;
pub const IW_MLME_DISASSOC: c_int = 1;
pub const IW_MLME_AUTH: c_int = 2;
pub const IW_MLME_ASSOC: c_int = 3;
// SIOCSIWAUTH/SIOCGIWAUTH struct iw_param flags
pub const IW_AUTH_INDEX: c_uint = 0x0FFF;
pub const IW_AUTH_FLAGS: c_uint = 0xF000;
// SIOCSIWAUTH/SIOCGIWAUTH parameters (0 .. 4095)
// (IW_AUTH_INDEX mask in struct iw_param flags; this is the index of the
// parameter that is being set/get to; value will be read/written to
// struct iw_param value field)
pub const IW_AUTH_WPA_VERSION: c_int = 0;
pub const IW_AUTH_CIPHER_PAIRWISE: c_int = 1;
pub const IW_AUTH_CIPHER_GROUP: c_int = 2;
pub const IW_AUTH_KEY_MGMT: c_int = 3;
pub const IW_AUTH_TKIP_COUNTERMEASURES: c_int = 4;
pub const IW_AUTH_DROP_UNENCRYPTED: c_int = 5;
pub const IW_AUTH_80211_AUTH_ALG: c_int = 6;
pub const IW_AUTH_WPA_ENABLED: c_int = 7;
pub const IW_AUTH_RX_UNENCRYPTED_EAPOL: c_int = 8;
pub const IW_AUTH_ROAMING_CONTROL: c_int = 9;
pub const IW_AUTH_PRIVACY_INVOKED: c_int = 10;
pub const IW_AUTH_CIPHER_GROUP_MGMT: c_int = 11;
pub const IW_AUTH_MFP: c_int = 12;
// IW_AUTH_WPA_VERSION values (bit field)
pub const IW_AUTH_WPA_VERSION_DISABLED: c_uint = 0x00000001;
pub const IW_AUTH_WPA_VERSION_WPA: c_uint = 0x00000002;
pub const IW_AUTH_WPA_VERSION_WPA2: c_uint = 0x00000004;
// IW_AUTH_PAIRWISE_CIPHER, IW_AUTH_GROUP_CIPHER, and IW_AUTH_CIPHER_GROUP_MGMT
// values (bit field)
pub const IW_AUTH_CIPHER_NONE: c_uint = 0x00000001;
pub const IW_AUTH_CIPHER_WEP40: c_uint = 0x00000002;
pub const IW_AUTH_CIPHER_TKIP: c_uint = 0x00000004;
pub const IW_AUTH_CIPHER_CCMP: c_uint = 0x00000008;
pub const IW_AUTH_CIPHER_WEP104: c_uint = 0x00000010;
pub const IW_AUTH_CIPHER_AES_CMAC: c_uint = 0x00000020;
// IW_AUTH_KEY_MGMT values (bit field)
pub const IW_AUTH_KEY_MGMT_802_1X: c_int = 1;
pub const IW_AUTH_KEY_MGMT_PSK: c_int = 2;
// IW_AUTH_80211_AUTH_ALG values (bit field)
pub const IW_AUTH_ALG_OPEN_SYSTEM: c_uint = 0x00000001;
pub const IW_AUTH_ALG_SHARED_KEY: c_uint = 0x00000002;
pub const IW_AUTH_ALG_LEAP: c_uint = 0x00000004;
// IW_AUTH_ROAMING_CONTROL values

// control
// IW_AUTH_MFP (management frame protection) values

// SIOCSIWENCODEEXT definitions
pub const IW_ENCODE_SEQ_MAX_SIZE: c_int = 8;
// struct iw_encode_ext ->alg
pub const IW_ENCODE_ALG_NONE: c_int = 0;
pub const IW_ENCODE_ALG_WEP: c_int = 1;
pub const IW_ENCODE_ALG_TKIP: c_int = 2;
pub const IW_ENCODE_ALG_CCMP: c_int = 3;
pub const IW_ENCODE_ALG_PMK: c_int = 4;
pub const IW_ENCODE_ALG_AES_CMAC: c_int = 5;
// struct iw_encode_ext ->ext_flags
pub const IW_ENCODE_EXT_TX_SEQ_VALID: c_uint = 0x00000001;
pub const IW_ENCODE_EXT_RX_SEQ_VALID: c_uint = 0x00000002;
pub const IW_ENCODE_EXT_GROUP_KEY: c_uint = 0x00000004;
pub const IW_ENCODE_EXT_SET_TX_KEY: c_uint = 0x00000008;
// IWEVMICHAELMICFAILURE : struct iw_michaelmicfailure ->flags
pub const IW_MICFAILURE_KEY_ID: c_uint = 0x00000003 /* Key ID 0..3 */;
pub const IW_MICFAILURE_GROUP: c_uint = 0x00000004;
pub const IW_MICFAILURE_PAIRWISE: c_uint = 0x00000008;
pub const IW_MICFAILURE_STAKEY: c_uint = 0x00000010;
pub const IW_MICFAILURE_COUNT: c_uint = 0x00000060 /* 1 or 2 (0 = count not supported);
//
// Bit field values for enc_capa in struct iw_range
pub const IW_ENC_CAPA_WPA: c_uint = 0x00000001;
pub const IW_ENC_CAPA_WPA2: c_uint = 0x00000002;
pub const IW_ENC_CAPA_CIPHER_TKIP: c_uint = 0x00000004;
pub const IW_ENC_CAPA_CIPHER_CCMP: c_uint = 0x00000008;
pub const IW_ENC_CAPA_4WAY_HANDSHAKE: c_uint = 0x00000010;
// Event capability macros - in (struct iw_range *)->event_capa
// Because we have more than 32 possible events, we use an array of
// 32 bit bitmasks. Note : 32 bits = 0x20 = 2^5.

// Event capability constants - event autogenerated by the kernel
// This list is valid for most 802.11 devices, customise as needed...

// "Easy" macro to set events in iw_range (less efficient)

// TYPES
// --------------------------- SUBTYPES ---------------------------
//
// Generic format for most parameters that fit in an int
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_param {
    pub /: *mut *mut __s32 value; / The value of the parameter itself,
    pub /: *mut *mut __u8 fixed; / Hardware should not use auto select,
    pub /: *mut *mut __u8 disabled; / Disable the feature,
    pub /: *mut *mut __u16 flags; / Various specifc flags (if any),
}

//
// For all data larger than 16 octets, we need to use a
// pointer to memory allocated in user space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_point {
    pub /: *mut *mut *mut void __user pointer; / Pointer to the data (in user space),
    pub /: *mut *mut __u16 length; / number of fields or size in bytes,
    pub /: *mut *mut __u16 flags; / Optional params,
}

//
// A frequency
// For numbers lower than 10^9, we encode the number in 'm' and
// set 'e' to 0
// For number greater than 10^9, we divide it by the lowest power
// of 10 to get 'm' lower than 10^9, with 'm'= f / (10^'e')...
// The power of 10 is in 'e', the result of the division is in 'm'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_freq {
    pub /: *mut *mut __s32 m; / Mantissa,
    pub /: *mut *mut __s16 e; / Exponent,
    pub /: *mut *mut __u8 i; / List index (when in range struct),
    pub /: *mut *mut __u8 flags; / Flags (fixed/auto),
}

//
// Quality of the link
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_quality {
    pub SNR,: *mut *mut __u8 qual; / link quality (%retries,,
    pub /: *mut *mut __u8 level; / signal level (dBm),
    pub /: *mut *mut __u8 noise; / noise level (dBm),
    pub /: *mut *mut __u8 updated; / Flags to know if updated,
}

//
// Packet discarded in the wireless adapter due to
// "wireless" specific problems...
// Note : the list of counter and statistics in net_device_stats
// is already pretty exhaustive, and you should use that first.
// This is only additional stats...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_discarded {
    pub /: *mut *mut __u32 nwid; / Rx : Wrong nwid/essid,
    pub /: *mut *mut __u32 code; / Rx : Unable to code/decode (WEP),
    pub /: *mut *mut __u32 fragment; / Rx : Can't perform MAC reassembly,
    pub /: *mut *mut __u32 retries; / Tx : Max MAC retries num reached,
    pub /: *mut *mut __u32 misc; / Others cases,
}

//
// Packet/Time period missed in the wireless adapter due to
// "wireless" specific problems...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_missed {
    pub /: *mut *mut __u32 beacon; / Missed beacons/superframe,
}

//
// Quality range (for spy threshold)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_thrspy {
    pub /: *mut *mut sockaddr addr; / Source address (hw/mac),
    pub /: *mut *mut iw_quality qual; / Quality of the link,
    pub /: *mut *mut iw_quality low; / Low threshold,
    pub /: *mut *mut iw_quality high; / High threshold,
}

//
// Optional data for scan request
//
// Note: these optional parameters are controlling parameters for the
// scanning behavior, these do not apply to getting scan results
// (SIOCGIWSCAN). Drivers are expected to keep a local BSS table and
// provide a merged results with all BSSes even if the previous scan
// request limited scanning to a subset, e.g., by specifying an SSID.
// Especially, scan results are required to include an entry for the
// current BSS if the driver is in Managed mode and associated with an AP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_scan_req {
    pub /: *mut *mut __u8 scan_type; / IW_SCAN_TYPE_{ACTIVE,PASSIVE},
    pub essid_len: __u8,
    pub channel_list: *mut *mut __u8 num_channels; / num entries in,
// 0 = scan all allowed channels
    pub may: *mut *mut __u8 flags; / reserved as padding; use zero, this,
// be used in the future for adding flags
// to request different scan behavior
    pub or: *mut *mut sockaddr bssid; / ff:ff:ff:ff:ff:ff for broadcast BSSID,
// individual address of a specific BSS
//
// Use this ESSID if IW_SCAN_THIS_ESSID flag is used instead of using
// the current ESSID. This allows scan requests for specific ESSID
// without having to change the current ESSID and potentially breaking
// the current association.
//
    pub essid: [__u8; IW_ESSID_MAX_SIZE],
//
// Optional parameters for changing the default scanning behavior.
// These are based on the MLME-SCAN.request from IEEE Std 802.11.
// TU is 1.024 ms. If these are set to 0, driver is expected to use
// reasonable default values. min_channel_time defines the time that
// will be used to wait for the first reply on each channel. If no
// replies are received, next channel will be scanned after this. If
// replies are received, total time waited on the channel is defined by
// max_channel_time.
//
    pub /: *mut *mut __u32 min_channel_time; / in TU,
    pub /: *mut *mut __u32 max_channel_time; / in TU,
    pub channel_list: [iw_freq; IW_MAX_FREQUENCIES],
}

// ------------------------- WPA SUPPORT -------------------------
//
// Extended data structure for get/set encoding (this is used with
// SIOCSIWENCODEEXT/SIOCGIWENCODEEXT. struct iw_point and IW_ENCODE_
// flags are used in the same way as with SIOCSIWENCODE/SIOCGIWENCODE and
// only the data contents changes (key data -> this structure, including
// key data).
//
// If the new key is the first group key, it will be set as the default
// TX key. Otherwise, default TX key index is only changed if
// IW_ENCODE_EXT_SET_TX_KEY flag is set.
//
// Key will be changed with SIOCSIWENCODEEXT in all cases except for
// special "change TX key index" operation which is indicated by setting
// key_len = 0 and ext_flags |= IW_ENCODE_EXT_SET_TX_KEY.
//
// tx_seq/rx_seq are only used when respective
// IW_ENCODE_EXT_{TX,RX}_SEQ_VALID flag is set in ext_flags. Normal
// TKIP/CCMP operation is to set RX seq with SIOCSIWENCODEEXT and start
// TX seq from zero whenever key is changed. SIOCGIWENCODEEXT is normally
// used only by an Authenticator (AP or an IBSS station) to get the
// current TX sequence number. Using TX_SEQ_VALID for SIOCSIWENCODEEXT and
// RX_SEQ_VALID for SIOCGIWENCODEEXT are optional, but can be useful for
// debugging/testing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_encode_ext {
    pub /: *mut *mut *mut __u32 ext_flags; / IW_ENCODE_EXT_,
    pub /: *mut *mut __u8 tx_seq[IW_ENCODE_SEQ_MAX_SIZE]; / LSB first,
    pub /: *mut *mut __u8 rx_seq[IW_ENCODE_SEQ_MAX_SIZE]; / LSB first,
    pub broadcast/multicast: *mut *mut sockaddr addr; / ff:ff:ff:ff:ff:ff for,
// (group) keys or unicast address for
// individual keys
    pub /: *mut *mut *mut __u16 alg; / IW_ENCODE_ALG_,
    pub key_len: __u16,
    pub key: [__u8; ],
}

// SIOCSIWMLME data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_mlme {
    pub /: *mut *mut *mut __u16 cmd; / IW_MLME_,
    pub reason_code: __u16,
    pub addr: sockaddr,
}

// SIOCSIWPMKSA data
pub const IW_PMKSA_ADD: c_int = 1;
pub const IW_PMKSA_REMOVE: c_int = 2;
pub const IW_PMKSA_FLUSH: c_int = 3;
pub const IW_PMKID_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_pmksa {
    pub /: *mut *mut *mut __u32 cmd; / IW_PMKSA_,
    pub bssid: sockaddr,
    pub pmkid: [__u8; IW_PMKID_LEN],
}

// IWEVMICHAELMICFAILURE data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_michaelmicfailure {
    pub flags: __u32,
    pub src_addr: sockaddr,
    pub /: *mut *mut __u8 tsc[IW_ENCODE_SEQ_MAX_SIZE]; / LSB first,
}

// IWEVPMKIDCAND data
pub const IW_PMKID_CAND_PREAUTH: c_uint = 0x00000001 /* RNS pre-authentication enabled */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_pmkid_cand {
    pub /: *mut *mut *mut __u32 flags; / IW_PMKID_CAND_,
    pub the: *mut *mut __u32 index; / the smaller the index, the higher,
// priority
    pub bssid: sockaddr,
}

// ------------------------ WIRELESS STATS ------------------------
//
// Wireless statistics (used for /proc/net/wireless)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_statistics {
    pub Status: *mut *mut __u16 status; /,
// - device dependent for now
    pub link: *mut *mut iw_quality qual; / Quality of the,
// (instant/mean/max)
    pub /: *mut *mut iw_discarded discard; / Packet discarded counts,
    pub /: *mut *mut iw_missed miss; / Packet missed counts,
}

// ------------------------ IOCTL REQUEST ------------------------
//
// This structure defines the payload of an ioctl, and is used
// below.
//
// Note that this structure should fit on the memory footprint
// of iwreq (which is the same as ifreq), which mean a max size of
// 16 octets = 128 bits. Warning, pointers might be 64 bits wide...
// You should check this when increasing the structures defined
// above in this file...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union iwreq_data {
// Config - generic
    pub name: [c_char; IFNAMSIZ],
// Name : used to verify the presence of  wireless extensions.
// Name of the protocol/provider...
    pub /: *mut *mut iw_point essid; / Extended network name,
    pub /: *mut *mut iw_param nwid; / network id (or domain - the cell),
    pub :: *mut *mut iw_freq freq; / frequency or channel,
// 0-1000 = channel
// > 1000 = frequency in Hz
    pub /: *mut *mut iw_param sens; / signal level threshold,
    pub /: *mut *mut iw_param bitrate; / default bit rate,
    pub /: *mut *mut iw_param txpower; / default transmit power,
    pub /: *mut *mut iw_param rts; / RTS threshold,
    pub /: *mut *mut iw_param frag; / Fragmentation threshold,
    pub /: *mut *mut __u32 mode; / Operation mode,
    pub /: *mut *mut iw_param retry; / Retry limits & lifetime,
    pub /: *mut *mut iw_point encoding; / Encoding stuff : tokens,
    pub /: *mut *mut iw_param power; / PM duration/timeout,
    pub /: *mut *mut iw_quality qual; / Quality part of statistics,
    pub /: *mut *mut sockaddr ap_addr; / Access point address,
    pub /: *mut *mut sockaddr addr; / Destination address (hw/mac),
    pub /: *mut *mut iw_param param; / Other small parameters,
    pub /: *mut *mut iw_point data; / Other large parameters,
}

//
// The structure to exchange data for ioctl.
// This structure is the same as 'struct ifreq', but (re)defined for
// convenience...
// Do I need to remind you about structure size (32 octets) ?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwreq {
    pub /: *mut *mut char ifrn_name[IFNAMSIZ]; / if name, e.g. "eth0",
    pub ifr_ifrn: },
// Data part (defined just above)
    pub u: iwreq_data,
}

// -------------------------- IOCTL DATA --------------------------
//
// For those ioctl which want to exchange mode data that what could
// fit in the above structure...
//
// Range of parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_range {
// Informative stuff (to choose between different interface)
    pub /: *mut *mut __u32 throughput; / To give an idea...,
// In theory this value should be the maximum benchmarked
// TCP/IP throughput, because with most of these devices the
// bit rate is meaningless (overhead an co) to estimate how
// fast the connection will go and pick the fastest one.
// I suggest people to play with Netperf or any benchmark...
//
// NWID (or domain id)
    pub /: *mut *mut __u32 min_nwid; / Minimal NWID we are able to set,
    pub /: *mut *mut __u32 max_nwid; / Maximal NWID we are able to set,
// Old Frequency (backward compat - moved lower )
    pub old_num_channels: __u16,
    pub old_num_frequency: __u8,
// Scan capabilities
    pub /: *mut *mut *mut __u8 scan_capa; / IW_SCAN_CAPA_ bit field,
// Wireless event capability bitmasks
    pub event_capa: [__u32; 6],
// signal level threshold range
    pub sensitivity: __s32,
// Quality of link & SNR stuff
// Quality range (link, level, noise)
// If the quality is absolute, it will be in the range [0 ; max_qual],
// if the quality is dBm, it will be in the range [max_qual ; 0].
// Don't forget that we use 8 bit arithmetics...
    pub /: *mut *mut iw_quality max_qual; / Quality of the link,
// This should contain the average/typical values of the quality
// indicator. This should be the threshold between a "good" and
// a "bad" link (example : monitor going from green to orange).
// Currently, user space apps like quality monitors don't have any
// way to calibrate the measurement. With this, they can split
// the range between 0 and max_qual in different quality level
// (using a geometric subdivision centered on the average).
// I expect that people doing the user space apps will feedback
// us on which value we need to put in each driver...
    pub /: *mut *mut iw_quality avg_qual; / Quality of the link,
// Rates
    pub /: *mut *mut __u8 num_bitrates; / Number of entries in the list,
    pub /: *mut *mut __s32 bitrate[IW_MAX_BITRATES]; / list, in bps,
// RTS threshold
    pub /: *mut *mut __s32 min_rts; / Minimal RTS threshold,
    pub /: *mut *mut __s32 max_rts; / Maximal RTS threshold,
// Frag threshold
    pub /: *mut *mut __s32 min_frag; / Minimal frag threshold,
    pub /: *mut *mut __s32 max_frag; / Maximal frag threshold,
// Power Management duration & timeout
    pub /: *mut *mut __s32 min_pmp; / Minimal PM period,
    pub /: *mut *mut __s32 max_pmp; / Maximal PM period,
    pub /: *mut *mut __s32 min_pmt; / Minimal PM timeout,
    pub /: *mut *mut __s32 max_pmt; / Maximal PM timeout,
    pub /: *mut *mut __u16 pmp_flags; / How to decode max/min PM period,
    pub /: *mut *mut __u16 pmt_flags; / How to decode max/min PM timeout,
    pub /: *mut *mut __u16 pm_capa; / What PM options are supported,
// Encoder stuff
    pub /: *mut *mut __u16 encoding_size[IW_MAX_ENCODING_SIZES]; / Different token sizes,
    pub /: *mut *mut __u8 num_encoding_sizes; / Number of entry in the list,
    pub /: *mut *mut __u8 max_encoding_tokens; / Max number of tokens,
// For drivers that need a "login/passwd" form
    pub /: *mut *mut __u8 encoding_login_index; / token index for login token,
// Transmit power
    pub /: *mut *mut __u16 txpower_capa; / What options are supported,
    pub /: *mut *mut __u8 num_txpower; / Number of entries in the list,
    pub /: *mut *mut __s32 txpower[IW_MAX_TXPOWER]; / list, in bps,
// Wireless Extension version info
    pub /: *mut *mut __u8 we_version_compiled; / Must be WIRELESS_EXT,
    pub /: *mut *mut __u8 we_version_source; / Last update of source,
// Retry limits and lifetime
    pub /: *mut *mut __u16 retry_capa; / What retry options are supported,
    pub /: *mut *mut __u16 retry_flags; / How to decode max/min retry limit,
    pub /: *mut *mut __u16 r_time_flags; / How to decode max/min retry life,
    pub /: *mut *mut __s32 min_retry; / Minimal number of retries,
    pub /: *mut *mut __s32 max_retry; / Maximal number of retries,
    pub /: *mut *mut __s32 min_r_time; / Minimal retry lifetime,
    pub /: *mut *mut __s32 max_r_time; / Maximal retry lifetime,
// Frequency
    pub /: *mut *mut __u16 num_channels; / Number of channels [0; num - 1],
    pub /: *mut *mut __u8 num_frequency; / Number of entry in the list,
    pub /: *mut *mut iw_freq freq[IW_MAX_FREQUENCIES]; / list,
// Note : this frequency list doesn't need to fit channel numbers,
// because each entry contain its channel index
    pub /: *mut *mut *mut __u32 enc_capa; / IW_ENC_CAPA_ bit field,
}

//
// Private ioctl interface information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_priv_args {
    pub /: *mut *mut __u32 cmd; / Number of the ioctl to issue,
    pub /: *mut *mut __u16 set_args; / Type and number of args,
    pub /: *mut *mut __u16 get_args; / Type and number of args,
    pub /: *mut *mut char name[IFNAMSIZ]; / Name of the extension,
}

// ----------------------- WIRELESS EVENTS -----------------------
//
// Wireless events are carried through the rtnetlink socket to user
// space. They are encapsulated in the IFLA_WIRELESS field of
// a RTM_NEWLINK message.
//
// A Wireless Event. Contains basically the same data as the ioctl...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iw_event {
    pub /: *mut *mut __u16 len; / Real length of this stuff,
    pub /: *mut *mut __u16 cmd; / Wireless IOCTL,
    pub /: *mut *mut iwreq_data u; / IOCTL fixed payload,
}

// Size of the Event prefix (including padding and alignement junk)

// Size of the various events

// iw_point events are special. First, the payload (extra data) come at
// the end of the event, so they are bigger than IW_EV_POINT_LEN. Second,
// we omit the pointer, so start at an offset.

// Size of the Event prefix when packed in stream

// Size of the various events when packed in stream

