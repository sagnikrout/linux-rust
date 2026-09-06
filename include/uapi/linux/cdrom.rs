//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cdrom.h
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
// -- <linux/cdrom.h>
// General header file for linux CD-ROM drivers
// Copyright (C) 1992         David Giller, rafetmad@oxy.edu
// 1994, 1995   Eberhard Mönkeberg, emoenke@gwdg.de
// 1996         David van Leeuwen, david@tm.tno.nl
// 1997, 1998   Erik Andersen, andersee@debian.org
// 1998-2002    Jens Axboe, axboe@suse.de
//

//
// As of Linux 2.1.x, all Linux CD-ROM application programs will use this
// (and only this) include file.  It is my hope to provide Linux with
// a uniform interface between software accessing CD-ROMs and the various
// device drivers that actually talk to the drives.  There may still be
// 23 different kinds of strange CD-ROM drives, but at least there will
// now be one, and only one, Linux CD-ROM interface.
//
// Additionally, as of Linux 2.1.x, all Linux application programs
// should use the O_NONBLOCK option when opening a CD-ROM device
// for subsequent ioctl commands.  This allows for neat system errors
// like "No medium found" or "Wrong medium type" upon attempting to
// mount or play an empty slot, mount an audio disc, or play a data disc.
// Generally, changing an application program to support O_NONBLOCK
// is as easy as the following:
// -    drive = open("/dev/cdrom", O_RDONLY);
// +    drive = open("/dev/cdrom", O_RDONLY | O_NONBLOCK);
// It is worth the small change.
//
// Patches for many common CD programs (provided by David A. van Leeuwen)
// can be found at:  ftp://ftp.gwdg.de/pub/linux/cdrom/drivers/cm206
//
// When a driver supports a certain function, but the cdrom drive we are
// using doesn't, we will return the error EDRIVE_CANT_DO_THIS.  We will
// borrow the "Operation not supported" error from the network folks to
// accomplish this.  Maybe someday we will get a more targeted error code,
// but this will do for now...

//
// The CD-ROM IOCTL commands  -- these should be supported by
// all the various cdrom drivers.  For the CD-ROM ioctls, we
// will commandeer byte 0x53, or 'S'.
//
pub const CDROMPAUSE: c_uint = 0x5301 /* Pause Audio Operation */;
pub const CDROMRESUME: c_uint = 0x5302 /* Resume paused Audio Operation */;
pub const CDROMPLAYMSF: c_uint = 0x5303 /* Play Audio MSF (struct cdrom_msf) */;
pub const CDROMPLAYTRKIND: c_uint = 0x5304 /* Play Audio Track/index;
pub const CDROMREADTOCHDR: c_uint = 0x5305 /* Read TOC header;
pub const CDROMREADTOCENTRY: c_uint = 0x5306 /* Read TOC entry;
pub const CDROMSTOP: c_uint = 0x5307 /* Stop the cdrom drive */;
pub const CDROMSTART: c_uint = 0x5308 /* Start the cdrom drive */;
pub const CDROMEJECT: c_uint = 0x5309 /* Ejects the cdrom media */;
pub const CDROMVOLCTRL: c_uint = 0x530a /* Control output volume;
pub const CDROMSUBCHNL: c_uint = 0x530b /* Read subchannel data;
pub const CDROMREADMODE2: c_uint = 0x530c /* Read CDROM mode 2 data (2336 Bytes);
pub const CDROMREADMODE1: c_uint = 0x530d /* Read CDROM mode 1 data (2048 Bytes);
pub const CDROMREADAUDIO: c_uint = 0x530e /* (struct cdrom_read_audio) */;
pub const CDROMEJECT_SW: c_uint = 0x530f /* enable(1)/disable(0) auto-ejecting */;
pub const CDROMMULTISESSION: c_uint = 0x5310 /* Obtain the start-of-last-session;
pub const CDROM_GET_MCN: c_uint = 0x5311 /* Obtain the "Universal Product Code";

pub const CDROMRESET: c_uint = 0x5312 /* hard-reset the drive */;
pub const CDROMVOLREAD: c_uint = 0x5313 /* Get the drive's volume setting;
pub const CDROMREADRAW: c_uint = 0x5314	/* read data in raw mode (2352 Bytes);
//
// These ioctls are used only used in aztcd.c and optcd.c
//
pub const CDROMREADCOOKED: c_uint = 0x5315	/* read data in cooked mode */;
pub const CDROMSEEK: c_uint = 0x5316  /* seek msf address */;
//
// This ioctl is only used by the scsi-cd driver.
//
pub const CDROMPLAYBLK: c_uint = 0x5317	/* (struct cdrom_blk) */;
//
// These ioctls are only used in optcd.c
//
pub const CDROMREADALL: c_uint = 0x5318	/* read all 2646 bytes */;
//
// These ioctls were only in (now removed) ide-cd.c for controlling
// drive spindown time.  They should be implemented in the
// Uniform driver, via generic packet commands, GPCMD_MODE_SELECT_10,
// GPCMD_MODE_SENSE_10 and the GPMODE_POWER_PAGE...
// -Erik
//
pub const CDROMGETSPINDOWN: c_uint = 0x531d;
pub const CDROMSETSPINDOWN: c_uint = 0x531e;
//
// These ioctls are implemented through the uniform CD-ROM driver
// They _will_ be adopted by all CD-ROM drivers, when all the CD-ROM
// drivers are eventually ported to the uniform CD-ROM driver interface.
//
pub const CDROMCLOSETRAY: c_uint = 0x5319	/* pendant of CDROMEJECT */;
pub const CDROM_SET_OPTIONS: c_uint = 0x5320  /* Set behavior options */;
pub const CDROM_CLEAR_OPTIONS: c_uint = 0x5321  /* Clear behavior options */;
pub const CDROM_SELECT_SPEED: c_uint = 0x5322  /* Set the CD-ROM speed */;
pub const CDROM_SELECT_DISC: c_uint = 0x5323  /* Select disc (for juke-boxes) */;
pub const CDROM_MEDIA_CHANGED: c_uint = 0x5325  /* Check is media changed  */;
pub const CDROM_DRIVE_STATUS: c_uint = 0x5326  /* Get tray position, etc. */;
pub const CDROM_DISC_STATUS: c_uint = 0x5327  /* Get disc type, etc. */;
pub const CDROM_CHANGER_NSLOTS: c_uint = 0x5328  /* Get number of slots */;
pub const CDROM_LOCKDOOR: c_uint = 0x5329  /* lock or unlock door */;
pub const CDROM_DEBUG: c_uint = 0x5330	/* Turn debug messages on/off */;
pub const CDROM_GET_CAPABILITY: c_uint = 0x5331	/* get capabilities */;
// Note that scsi/scsi_ioctl.h also uses 0x5382 - 0x5386.
// Future CDROM ioctls should be kept below 0x537F
//
// This ioctl is only used by sbpcd at the moment
pub const CDROMAUDIOBUFSIZ: c_uint = 0x5382	/* set the audio buffer size */;
// conflict with SCSI_IOCTL_GET_IDLUN
// DVD-ROM Specific ioctls
pub const DVD_READ_STRUCT: c_uint = 0x5390  /* Read structure */;
pub const DVD_WRITE_STRUCT: c_uint = 0x5391  /* Write structure */;
pub const DVD_AUTH: c_uint = 0x5392  /* Authentication */;
pub const CDROM_SEND_PACKET: c_uint = 0x5393	/* send a packet to the drive */;
pub const CDROM_NEXT_WRITABLE: c_uint = 0x5394	/* get next writable block */;
pub const CDROM_LAST_WRITTEN: c_uint = 0x5395	/* get last block written on disc */;
pub const CDROM_TIMED_MEDIA_CHANGE: c_uint = 0x5396  /* get the timestamp of the last media change */;
//
// CDROM IOCTL structures
//
// Address in MSF format
// Address in either MSF or logical format
// This struct is used by the CDROMPLAYMSF ioctl
// This struct is used by the CDROMPLAYTRKIND ioctl
// This struct is used by the CDROMREADTOCHDR ioctl
// This struct is used by the CDROMVOLCTRL and CDROMVOLREAD ioctls
// This struct is used by the CDROMSUBCHNL ioctl
// This struct is used by the CDROMREADTOCENTRY ioctl
// This struct is used by the CDROMREADMODE1, and CDROMREADMODE2 ioctls
// This struct is used by the CDROMREADAUDIO ioctl
// This struct is used with the CDROMMULTISESSION ioctl
// This struct is used with the CDROM_GET_MCN ioctl.
// Very few audio discs actually have Universal Product Code information,
// which should just be the Medium Catalog Number on the box.  Also note
// that the way the codeis written on CD is _not_ uniform across all discs!
//
// This is used by the CDROMPLAYBLK ioctl
pub const CDROM_PACKET_SIZE: c_int = 12;
pub const CGC_DATA_UNKNOWN: c_int = 0;
pub const CGC_DATA_WRITE: c_int = 1;
pub const CGC_DATA_READ: c_int = 2;
pub const CGC_DATA_NONE: c_int = 3;
// for CDROM_PACKET_COMMAND ioctl
// This struct is used by CDROM_TIMED_MEDIA_CHANGE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdrom_timed_media_change_info {
    pub media: *mut *mut __s64 last_media_change; / Timestamp of the last detected,
// change in ms. May be set by caller,
// updated upon successful return of
// ioctl.
//
    pub indicate: *mut *mut __u64 media_flags; / Flags returned by ioctl to,
// media status.
//
}

pub const MEDIA_CHANGED_FLAG: c_uint = 0x1	/* Last detected media change was more;
// recent than last_media_change set by
// caller.
//
// other bits of media_flags available for future use
//
// A CD-ROM physical sector size is 2048, 2052, 2056, 2324, 2332, 2336,
// 2340, or 2352 bytes long.
// Sector types of the standard CD-ROM data formats:
//
// format   sector type               user data size (bytes)
// -----------------------------------------------------------------------------
// 1     (Red Book)    CD-DA          2352    (CD_FRAMESIZE_RAW)
// 2     (Yellow Book) Mode1 Form1    2048    (CD_FRAMESIZE)
// 3     (Yellow Book) Mode1 Form2    2336    (CD_FRAMESIZE_RAW0)
// 4     (Green Book)  Mode2 Form1    2048    (CD_FRAMESIZE)
// 5     (Green Book)  Mode2 Form2    2328    (2324+4 spare bytes)
//
// The layout of the standard CD-ROM data formats:
// -----------------------------------------------------------------------------
// - audio (red):                  | audio_sample_bytes |
// |        2352        |
//
// - data (yellow, mode1):         | sync - head - data - EDC - zero - ECC |
// |  12  -   4  - 2048 -  4  -   8  - 276 |
//
// - data (yellow, mode2):         | sync - head - data |
// |  12  -   4  - 2336 |
//
// - XA data (green, mode2 form1): | sync - head - sub - data - EDC - ECC |
// |  12  -   4  -  8  - 2048 -  4  - 276 |
//
// - XA data (green, mode2 form2): | sync - head - sub - data - Spare |
// |  12  -   4  -  8  - 2324 -  4    |
//
// Some generally useful CD-ROM information -- mostly based on the above

// most drives don't deliver everything:

// CD-ROM address types (cdrom_tocentry.cdte_format)
pub const CDROM_LBA: c_uint = 0x01 /* "logical block": first frame is #0 */;
pub const CDROM_MSF: c_uint = 0x02 /* "minute-second-frame": binary, not bcd here! */;
// bit to tell whether track is data or audio (cdrom_tocentry.cdte_ctrl)
pub const CDROM_DATA_TRACK: c_uint = 0x04;
// The leadout track is always 0xAA, regardless of # of tracks on disc
pub const CDROM_LEADOUT: c_uint = 0xAA;
// audio states (from SCSI-2, but seen with other drives, too)
pub const CDROM_AUDIO_INVALID: c_uint = 0x00	/* audio status not supported */;
pub const CDROM_AUDIO_PLAY: c_uint = 0x11	/* audio play operation in progress */;
pub const CDROM_AUDIO_PAUSED: c_uint = 0x12	/* audio play operation paused */;
pub const CDROM_AUDIO_COMPLETED: c_uint = 0x13	/* audio play successfully completed */;
pub const CDROM_AUDIO_ERROR: c_uint = 0x14	/* audio play stopped due to error */;
pub const CDROM_AUDIO_NO_STATUS: c_uint = 0x15	/* no current audio status to return */;
// capability flags used with the uniform CD-ROM driver
pub const CDC_CLOSE_TRAY: c_uint = 0x1     /* caddy systems _can't_ close */;
pub const CDC_OPEN_TRAY: c_uint = 0x2     /* but _can_ eject.  */;
pub const CDC_LOCK: c_uint = 0x4     /* disable manual eject */;
pub const CDC_SELECT_SPEED: c_uint = 0x8     /* programmable speed */;
pub const CDC_SELECT_DISC: c_uint = 0x10    /* select disc from juke-box */;
pub const CDC_MULTI_SESSION: c_uint = 0x20    /* read sessions>1 */;
pub const CDC_MCN: c_uint = 0x40    /* Medium Catalog Number */;
pub const CDC_MEDIA_CHANGED: c_uint = 0x80    /* media changed */;
pub const CDC_PLAY_AUDIO: c_uint = 0x100   /* audio functions */;
pub const CDC_RESET: c_uint = 0x200   /* hard reset device */;
pub const CDC_DRIVE_STATUS: c_uint = 0x800   /* driver implements drive status */;
pub const CDC_GENERIC_PACKET: c_uint = 0x1000	/* driver implements generic packets */;
pub const CDC_CD_R: c_uint = 0x2000	/* drive is a CD-R */;
pub const CDC_CD_RW: c_uint = 0x4000	/* drive is a CD-RW */;
pub const CDC_DVD: c_uint = 0x8000	/* drive is a DVD */;
pub const CDC_DVD_R: c_uint = 0x10000	/* drive can write DVD-R */;
pub const CDC_DVD_RAM: c_uint = 0x20000	/* drive can write DVD-RAM */;
pub const CDC_MO_DRIVE: c_uint = 0x40000 /* drive is an MO device */;
pub const CDC_MRW: c_uint = 0x80000 /* drive can read MRW */;
pub const CDC_MRW_W: c_uint = 0x100000 /* drive can write MRW */;
pub const CDC_RAM: c_uint = 0x200000 /* ok to open for WRITE */;
// drive status possibilities returned by CDROM_DRIVE_STATUS ioctl

pub const CDS_NO_DISC: c_int = 1;
pub const CDS_TRAY_OPEN: c_int = 2;
pub const CDS_DRIVE_NOT_READY: c_int = 3;
pub const CDS_DISC_OK: c_int = 4;
// return values for the CDROM_DISC_STATUS ioctl
// can also return CDS_NO_[INFO|DISC], from above
pub const CDS_AUDIO: c_int = 100;
pub const CDS_DATA_1: c_int = 101;
pub const CDS_DATA_2: c_int = 102;
pub const CDS_XA_2_1: c_int = 103;
pub const CDS_XA_2_2: c_int = 104;
pub const CDS_MIXED: c_int = 105;
// User-configurable behavior options for the uniform CD-ROM driver
pub const CDO_AUTO_CLOSE: c_uint = 0x1     /* close tray on first open() */;
pub const CDO_AUTO_EJECT: c_uint = 0x2     /* open tray on last release() */;
pub const CDO_USE_FFLAGS: c_uint = 0x4     /* use O_NONBLOCK information on open */;
pub const CDO_LOCK: c_uint = 0x8     /* lock tray on open files */;
pub const CDO_CHECK_TYPE: c_uint = 0x10    /* check type on open for data */;
// Special codes used when specifying changer slots.

// For partition based multisession access. IDE can handle 64 partitions
// per drive - SCSI CD-ROM's use minors to differentiate between the
// various drives, so we can't do multisessions the same way there.
// Use the -o session=x option to mount on them.
//
pub const CD_PART_MAX: c_int = 64;

//
// Generic Packet commands, MMC commands, and such
//
// The generic packet command opcodes for CD/DVD Logical Units,
// From Table 57 of the SFF8090 Ver. 3 (Mt. Fuji) draft standard.
pub const GPCMD_BLANK: c_uint = 0xa1;
pub const GPCMD_CLOSE_TRACK: c_uint = 0x5b;
pub const GPCMD_FLUSH_CACHE: c_uint = 0x35;
pub const GPCMD_FORMAT_UNIT: c_uint = 0x04;
pub const GPCMD_GET_CONFIGURATION: c_uint = 0x46;
pub const GPCMD_GET_EVENT_STATUS_NOTIFICATION: c_uint = 0x4a;
pub const GPCMD_GET_PERFORMANCE: c_uint = 0xac;
pub const GPCMD_INQUIRY: c_uint = 0x12;
pub const GPCMD_LOAD_UNLOAD: c_uint = 0xa6;
pub const GPCMD_MECHANISM_STATUS: c_uint = 0xbd;
pub const GPCMD_MODE_SELECT_10: c_uint = 0x55;
pub const GPCMD_MODE_SENSE_10: c_uint = 0x5a;
pub const GPCMD_PAUSE_RESUME: c_uint = 0x4b;
pub const GPCMD_PLAY_AUDIO_10: c_uint = 0x45;
pub const GPCMD_PLAY_AUDIO_MSF: c_uint = 0x47;
pub const GPCMD_PLAY_AUDIO_TI: c_uint = 0x48;
pub const GPCMD_PLAY_CD: c_uint = 0xbc;
pub const GPCMD_PREVENT_ALLOW_MEDIUM_REMOVAL: c_uint = 0x1e;
pub const GPCMD_READ_10: c_uint = 0x28;
pub const GPCMD_READ_12: c_uint = 0xa8;
pub const GPCMD_READ_BUFFER: c_uint = 0x3c;
pub const GPCMD_READ_BUFFER_CAPACITY: c_uint = 0x5c;
pub const GPCMD_READ_CDVD_CAPACITY: c_uint = 0x25;
pub const GPCMD_READ_CD: c_uint = 0xbe;
pub const GPCMD_READ_CD_MSF: c_uint = 0xb9;
pub const GPCMD_READ_DISC_INFO: c_uint = 0x51;
pub const GPCMD_READ_DVD_STRUCTURE: c_uint = 0xad;
pub const GPCMD_READ_FORMAT_CAPACITIES: c_uint = 0x23;
pub const GPCMD_READ_HEADER: c_uint = 0x44;
pub const GPCMD_READ_TRACK_RZONE_INFO: c_uint = 0x52;
pub const GPCMD_READ_SUBCHANNEL: c_uint = 0x42;
pub const GPCMD_READ_TOC_PMA_ATIP: c_uint = 0x43;
pub const GPCMD_REPAIR_RZONE_TRACK: c_uint = 0x58;
pub const GPCMD_REPORT_KEY: c_uint = 0xa4;
pub const GPCMD_REQUEST_SENSE: c_uint = 0x03;
pub const GPCMD_RESERVE_RZONE_TRACK: c_uint = 0x53;
pub const GPCMD_SEND_CUE_SHEET: c_uint = 0x5d;
pub const GPCMD_SCAN: c_uint = 0xba;
pub const GPCMD_SEEK: c_uint = 0x2b;
pub const GPCMD_SEND_DVD_STRUCTURE: c_uint = 0xbf;
pub const GPCMD_SEND_EVENT: c_uint = 0xa2;
pub const GPCMD_SEND_KEY: c_uint = 0xa3;
pub const GPCMD_SEND_OPC: c_uint = 0x54;
pub const GPCMD_SET_READ_AHEAD: c_uint = 0xa7;
pub const GPCMD_SET_STREAMING: c_uint = 0xb6;
pub const GPCMD_START_STOP_UNIT: c_uint = 0x1b;
pub const GPCMD_STOP_PLAY_SCAN: c_uint = 0x4e;
pub const GPCMD_TEST_UNIT_READY: c_uint = 0x00;
pub const GPCMD_VERIFY_10: c_uint = 0x2f;
pub const GPCMD_WRITE_10: c_uint = 0x2a;
pub const GPCMD_WRITE_12: c_uint = 0xaa;
pub const GPCMD_WRITE_AND_VERIFY_10: c_uint = 0x2e;
pub const GPCMD_WRITE_BUFFER: c_uint = 0x3b;
// This is listed as optional in ATAPI 2.6, but is (curiously)
// missing from Mt. Fuji, Table 57.  It _is_ mentioned in Mt. Fuji
// Table 377 as an MMC command for SCSi devices though...  Most ATAPI
// drives support it.
pub const GPCMD_SET_SPEED: c_uint = 0xbb;
// This seems to be a SCSI specific CD-ROM opcode
// to play data at track/index
pub const GPCMD_PLAYAUDIO_TI: c_uint = 0x48;
//
// From MS Media Status Notification Support Specification. For
// older drives only.
//
pub const GPCMD_GET_MEDIA_STATUS: c_uint = 0xda;
// Mode page codes for mode sense/set
pub const GPMODE_VENDOR_PAGE: c_uint = 0x00;
pub const GPMODE_R_W_ERROR_PAGE: c_uint = 0x01;
pub const GPMODE_WRITE_PARMS_PAGE: c_uint = 0x05;
pub const GPMODE_WCACHING_PAGE: c_uint = 0x08;
pub const GPMODE_AUDIO_CTL_PAGE: c_uint = 0x0e;
pub const GPMODE_POWER_PAGE: c_uint = 0x1a;
pub const GPMODE_FAULT_FAIL_PAGE: c_uint = 0x1c;
pub const GPMODE_TO_PROTECT_PAGE: c_uint = 0x1d;
pub const GPMODE_CAPABILITIES_PAGE: c_uint = 0x2a;
pub const GPMODE_ALL_PAGES: c_uint = 0x3f;
// Not in Mt. Fuji, but in ATAPI 2.6 -- deprecated now in favor
// of MODE_SENSE_POWER_PAGE
pub const GPMODE_CDROM_PAGE: c_uint = 0x0d;
// DVD struct types
pub const DVD_STRUCT_PHYSICAL: c_uint = 0x00;
pub const DVD_STRUCT_COPYRIGHT: c_uint = 0x01;
pub const DVD_STRUCT_DISCKEY: c_uint = 0x02;
pub const DVD_STRUCT_BCA: c_uint = 0x03;
pub const DVD_STRUCT_MANUFACT: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_layer {
    pub 4: __u8 book_version :,
    pub 4: __u8 book_type :,
    pub 4: __u8 min_rate :,
    pub 4: __u8 disc_size :,
    pub 4: __u8 layer_type :,
    pub 1: __u8 track_path :,
    pub 2: __u8 nlayers :,
    pub 4: __u8 track_density :,
    pub 4: __u8 linear_density :,
    pub 1: __u8 bca :,
    pub start_sector: __u32,
    pub end_sector: __u32,
    pub end_sector_l0: __u32,
}

pub const DVD_LAYERS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_physical {
    pub type: __u8,
    pub layer_num: __u8,
    pub layer: [dvd_layer; DVD_LAYERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_copyright {
    pub type: __u8,
    pub layer_num: __u8,
    pub cpst: __u8,
    pub rmi: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_disckey {
    pub type: __u8,
    pub 2: unsigned agid :,
    pub value: [__u8; 2048],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_bca {
    pub type: __u8,
    pub len: c_int,
    pub value: [__u8; 188],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_manufact {
    pub type: __u8,
    pub layer_num: __u8,
    pub len: c_int,
    pub value: [__u8; 2048],
}

//
// DVD authentication ioctl
//
// Authentication states
pub const DVD_LU_SEND_AGID: c_int = 0;
pub const DVD_HOST_SEND_CHALLENGE: c_int = 1;
pub const DVD_LU_SEND_KEY1: c_int = 2;
pub const DVD_LU_SEND_CHALLENGE: c_int = 3;
pub const DVD_HOST_SEND_KEY2: c_int = 4;
// Termination states
pub const DVD_AUTH_ESTABLISHED: c_int = 5;
pub const DVD_AUTH_FAILURE: c_int = 6;
// Other functions
pub const DVD_LU_SEND_TITLE_KEY: c_int = 7;
pub const DVD_LU_SEND_ASF: c_int = 8;
pub const DVD_INVALIDATE_AGID: c_int = 9;
pub const DVD_LU_SEND_RPC_STATE: c_int = 10;
pub const DVD_HOST_SEND_RPC_STATE: c_int = 11;
// State data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_lu_send_agid {
    pub type: __u8,
    pub 2: unsigned agid :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_host_send_challenge {
    pub type: __u8,
    pub 2: unsigned agid :,
    pub chal: dvd_challenge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_send_key {
    pub type: __u8,
    pub 2: unsigned agid :,
    pub key: dvd_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_lu_send_challenge {
    pub type: __u8,
    pub 2: unsigned agid :,
    pub chal: dvd_challenge,
}

pub const DVD_CPM_NO_COPYRIGHT: c_int = 0;
pub const DVD_CPM_COPYRIGHTED: c_int = 1;
pub const DVD_CP_SEC_NONE: c_int = 0;
pub const DVD_CP_SEC_EXIST: c_int = 1;
pub const DVD_CGMS_UNRESTRICTED: c_int = 0;
pub const DVD_CGMS_SINGLE: c_int = 2;
pub const DVD_CGMS_RESTRICTED: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_lu_send_title_key {
    pub type: __u8,
    pub 2: unsigned agid :,
    pub title_key: dvd_key,
    pub lba: c_int,
    pub 1: unsigned cpm :,
    pub 1: unsigned cp_sec :,
    pub 2: unsigned cgms :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_lu_send_asf {
    pub type: __u8,
    pub 2: unsigned agid :,
    pub 1: unsigned asf :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_host_send_rpcstate {
    pub type: __u8,
    pub pdrc: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvd_lu_send_rpcstate {
    pub 2: __u8 type :,
    pub 3: __u8 vra :,
    pub 3: __u8 ucca :,
    pub region_mask: __u8,
    pub rpc_scheme: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_sense {

    pub 1: __u8 valid :,
    pub 7: __u8 error_code :,

    pub 7: __u8 error_code :,
    pub 1: __u8 valid :,

    pub segment_number: __u8,

    pub 2: __u8 reserved1 :,
    pub 1: __u8 ili :,
    pub 1: __u8 reserved2 :,
    pub 4: __u8 sense_key :,

    pub 4: __u8 sense_key :,
    pub 1: __u8 reserved2 :,
    pub 1: __u8 ili :,
    pub 2: __u8 reserved1 :,
    pub information: [__u8; 4],
    pub add_sense_len: __u8,
    pub command_info: [__u8; 4],
    pub asc: __u8,
    pub ascq: __u8,
    pub fruc: __u8,
    pub sks: [__u8; 3],
    pub asb: [__u8; 46],
}

//
// feature profile
//
pub const CDF_RWRT: c_uint = 0x0020	/* "Random Writable" */;
pub const CDF_HWDM: c_uint = 0x0024	/* "Hardware Defect Management" */;
pub const CDF_MRW: c_uint = 0x0028;
//
// media status bits
//
pub const CDM_MRW_NOTMRW: c_int = 0;
pub const CDM_MRW_BGFORMAT_INACTIVE: c_int = 1;
pub const CDM_MRW_BGFORMAT_ACTIVE: c_int = 2;
pub const CDM_MRW_BGFORMAT_COMPLETE: c_int = 3;
//
// mrw address spaces
//
pub const MRW_LBA_DMA: c_int = 0;
pub const MRW_LBA_GAA: c_int = 1;
//
// mrw mode pages (first is deprecated) -- probed at init time and
// cdi->mrw_mode_page is set
//
pub const MRW_MODE_PC_PRE1: c_uint = 0x2c;
pub const MRW_MODE_PC: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrw_feature_desc {
    pub feature_code: __be16,

    pub 2: __u8 reserved1 :,
    pub 4: __u8 feature_version :,
    pub 1: __u8 persistent :,
    pub 1: __u8 curr :,

    pub 1: __u8 curr :,
    pub 1: __u8 persistent :,
    pub 4: __u8 feature_version :,
    pub 2: __u8 reserved1 :,

    pub add_len: __u8,

    pub 7: __u8 reserved2 :,
    pub 1: __u8 write :,

    pub 1: __u8 write :,
    pub 7: __u8 reserved2 :,

    pub reserved3: __u8,
    pub reserved4: __u8,
    pub reserved5: __u8,
}

// cf. mmc4r02g.pdf 5.3.10 Random Writable Feature (0020h) pg 197 of 635
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rwrt_feature_desc {
    pub feature_code: __be16,

    pub 2: __u8 reserved1 :,
    pub 4: __u8 feature_version :,
    pub 1: __u8 persistent :,
    pub 1: __u8 curr :,

    pub 1: __u8 curr :,
    pub 1: __u8 persistent :,
    pub 4: __u8 feature_version :,
    pub 2: __u8 reserved1 :,

    pub add_len: __u8,
    pub last_lba: __u32,
    pub block_size: __u32,
    pub blocking: __u16,

    pub 7: __u8 reserved2 :,
    pub 1: __u8 page_present :,

    pub 1: __u8 page_present :,
    pub 7: __u8 reserved2 :,

    pub reserved3: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct feature_header {
    pub data_len: __u32,
    pub reserved1: __u8,
    pub reserved2: __u8,
    pub curr_profile: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_page_header {
    pub mode_data_length: __be16,
    pub medium_type: __u8,
    pub reserved1: __u8,
    pub reserved2: __u8,
    pub reserved3: __u8,
    pub desc_length: __be16,
}

// removable medium feature descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rm_feature_desc {
    pub feature_code: __be16,

    pub reserved1:2: __u8,
    pub feature_version:4: __u8,
    pub persistent:1: __u8,
    pub curr:1: __u8,

    pub curr:1: __u8,
    pub persistent:1: __u8,
    pub feature_version:4: __u8,
    pub reserved1:2: __u8,

    pub add_len: __u8,

    pub mech_type:3: __u8,
    pub load:1: __u8,
    pub eject:1: __u8,
    pub pvnt_jmpr:1: __u8,
    pub dbml:1: __u8,
    pub lock:1: __u8,

    pub lock:1: __u8,
    pub dbml:1: __u8,
    pub pvnt_jmpr:1: __u8,
    pub eject:1: __u8,
    pub load:1: __u8,
    pub mech_type:3: __u8,

    pub reserved2: __u8,
    pub reserved3: __u8,
    pub reserved4: __u8,
}
