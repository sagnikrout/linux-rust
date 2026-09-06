//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fd.h
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

// New file layout: Now the ioctl definitions immediately follow the
// definitions of the structures that they use
//
// Geometry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct floppy_struct {
    pub /: *mut *mut stretch; / bit 0 !=0 means double track steps,
// bit 1 != 0 means swap sides
// bits 2..9 give the first sector
// number (the LSB is flipped)
pub const FD_STRETCH: c_int = 1;
pub const FD_SWAPSIDES: c_int = 2;
pub const FD_ZEROBASED: c_int = 4;
pub const FD_SECTBASEMASK: c_uint = 0x3FC;

pub const FD_2M: c_uint = 0x4;
pub const FD_SIZECODEMASK: c_uint = 0x38;

pub const FD_PERP: c_uint = 0x40;
    pub /: *mut *mut fmt_gap; / gap2 size,
    pub /: *const *const *const char  name; / used only for predefined formats,
}

// commands needing write access have 0x40 set
// commands needing super user access have 0x80 set

// clear user-defined parameters

// set user-defined parameters for current media

// set/get disk parameters

// issue/don't issue kernel messages on media type change
//
// Formatting (obsolete)
//
pub const FD_FILL_BYTE: c_uint = 0xF6 /* format fill byte. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct format_descr {
    pub device,head,track: c_uint,
}

// begin formatting a disk

// format the specified track

// end formatting a disk
//
// Error thresholds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct floppy_max_errors {
// entire track at once
// tried
//
// Threshold for reporting FDC errors to the console.
// Setting this to zero may flood your screen when using
// ultra cheap floppies ;-)
//
}

// set fdc error reporting threshold

// flush buffers for media; either for verifying media, or for
// handling a media change without closing the file descriptor

// set/get abortion and read_track threshold. See also floppy_drive_params
// structure

// get drive type: 5 1/4 or 3 1/2
//
// Drive parameters (user modifiable)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct floppy_drive_params {
    pub /: *mut *mut signed char cmos; / CMOS type,
// Spec2 is (HLD<<1 | ND), where HLD is head load time (1=2ms, 2=4 ms
// etc) and ND is set means no DMA. Hardcoded to 6 (HLD=6ms, use DMA).
//
    pub /: *mut *mut unsigned long max_dtr; / Step rate, usec,
    pub /: *mut *mut unsigned long hlt; / Head load/settle time, msec,
    pub of: *mut *mut unsigned long hut; / Head unload time (remnant,
// 8" drives)
    pub /: *mut *mut unsigned long srt; / Step rate, usec,
    pub (expressed: *mut *mut unsigned long spinup; / time needed for spinup,
// in jiffies)
    pub /: *mut *mut unsigned long spindown; / timeout needed for spindown,
    pub disk: *mut *mut unsigned char spindown_offset; / decides in which position the,
// will stop
    pub /: *mut *mut unsigned char select_delay; / delay to wait after select,
    pub /: *mut *mut unsigned char rps; / rotations per second,
    pub /: *mut *mut unsigned char tracks; / maximum number of tracks,
    pub /: *mut *mut unsigned long timeout; / timeout for interrupt requests,
    pub use: *mut *mut unsigned char interleave_sect; / if there are more sectors,,
// interleave
    pub max_errors: floppy_max_errors,
    pub /: *mut *mut char flags; / various flags, including ftd_msg,
//
// Announce successful media type detection and media information loss after
// disk changes.
// Also used to enable/disable printing of overrun warnings.
//
pub const FTD_MSG: c_uint = 0x10;
pub const FD_BROKEN_DCL: c_uint = 0x20;
pub const FD_DEBUG: c_uint = 0x02;
pub const FD_SILENT_DCL_CLEAR: c_uint = 0x4;
pub const FD_INVERTED_DCL: c_uint = 0x80 /* must be 0x80, because of hardware;
    pub /: *mut *mut char read_track; / use readtrack during probing?,
//
// Auto-detection. Each drive type has eight formats which are
// used in succession to try to read the disk. If the FDC cannot lock onto
// the disk, the next format is tried. This uses the variable 'probing'.
//
pub const FD_AUTODETECT_SIZE: c_int = 8;
    pub /: *mut *mut short autodetect[FD_AUTODETECT_SIZE]; / autodetected formats,
    pub disk: *mut *mut int checkfreq; / how often should the drive be checked for,
// changes
    pub /: *mut *mut int native_format; / native format of this drive,
}

// to clear media change status

// set/get drive parameters
//
// Current drive state (not directly modifiable by user, readonly)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct floppy_drive_struct {
    pub flags: c_ulong,
// values for these flags

    pub spinup_date: c_ulong,
    pub select_date: c_ulong,
    pub first_read_date: c_ulong,
    pub probed_format: c_short,
    pub /: *mut *mut short track; / current track,
    pub /: *mut *mut short maxblock; / id of highest block read,
    pub /: *mut *mut short maxtrack; / id of highest half track read,
    pub /: *mut *mut int generation; / how many diskchanges?,
//
// (User-provided) media information is _not_ discarded after a media change
// if the corresponding keep_data flag is non-zero. Positive values are
// decremented after each probe.
//
    pub keep_data: c_int,
// Prevent "aliased" accesses.
    pub fd_ref: c_int,
    pub fd_device: c_int,
    pub disk: *mut *mut unsigned long last_checked; / when was the drive last checked for a,
// change?
    pub dmabuf: *mut c_char,
    pub bufblocks: c_int,
}

// get drive state: GET returns the cached state, POLL polls for new state
//
// reset FDC
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reset_mode {
    FD_RESET_IF_NEEDED,	/* reset only if the reset flags is set */
    FD_RESET_IF_RAWCMD,	/* obsolete */
    FD_RESET_ALWAYS		/* reset always */
}

//
// FDC state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct floppy_fdc_state {
    pub /: *mut *mut int spec1; / spec1 value last used,
    pub /: *mut *mut int spec2; / spec2 value last used,
    pub dtr: c_int,
    pub /: *mut *mut unsigned char version; / FDC version code,
    pub dor: c_uchar,
    pub /: *mut *mut unsigned long address; / io address,
    pub rawcmd:2: c_uint,
    pub reset:1: c_uint,
    pub need_configure:1: c_uint,
    pub perp_mode:2: c_uint,
    pub has_fifo:1: c_uint,
    pub /: *mut *mut unsigned int driver_version; / version code for floppy driver,
pub const FD_DRIVER_VERSION: c_uint = 0x100;
// user programs using the floppy API should use floppy_fdc_state to
// get the version number of the floppy driver that they are running
// on. If this version number is bigger than the one compiled into the
// user program (the FD_DRIVER_VERSION define), it should be prepared
// to bigger structures
//
    pub track: [c_uchar; 4],
// Position of the heads of the 4 units attached to this FDC,
// as stored on the FDC. In the future, the position as stored
// on the FDC might not agree with the actual physical
// position of these drive heads. By allowing such
// disagreement, it will be possible to reset the FDC without
// incurring the expensive cost of repositioning all heads.
// Right now, these positions are hard wired to 0.
}

//
// Asynchronous Write error tracking
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct floppy_write_errors {
// Write error logging.
//
// These fields can be cleared with the FDWERRORCLR ioctl.
// Only writes that were attempted but failed due to a physical media
// error are logged.  write(2) calls that fail and return an error code
// to the user process are not counted.
//
    pub errors: *mut *mut unsigned int write_errors; / number of physical write,
// encountered
// position of first and last write errors
    pub first_error_sector: c_ulong,
    pub first_error_generation: c_int,
    pub last_error_sector: c_ulong,
    pub last_error_generation: c_int,
    pub write: *mut *mut unsigned int badness; / highest retry count for a read or,
// operation
}

// clear write error and badness information

// get write error and badness information
//
// Raw commands
//
// new interface flag: now we can do them in batches
// Macro flag: #define FDHAVEBATCHEDRAWCMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct floppy_raw_cmd {
    pub flags: c_uint,
pub const FD_RAW_READ: c_int = 1;
pub const FD_RAW_WRITE: c_int = 2;
pub const FD_RAW_NO_MOTOR: c_int = 4;

pub const FD_RAW_SPIN: c_uint = 0x10 /* spin up the disk for this command */;
pub const FD_RAW_NO_MOTOR_AFTER: c_uint = 0x20 /* switch the motor off after command;
// completion
pub const FD_RAW_NEED_DISK: c_uint = 0x40  /* this command needs a disk to be present */;
pub const FD_RAW_NEED_SEEK: c_uint = 0x80  /* this command uses an implied seek (soft) */;
// more "in" flags
pub const FD_RAW_MORE: c_uint = 0x100  /* more records follow */;
pub const FD_RAW_STOP_IF_FAILURE: c_uint = 0x200 /* stop if we encounter a failure */;
pub const FD_RAW_STOP_IF_SUCCESS: c_uint = 0x400 /* stop if command successful */;
pub const FD_RAW_SOFTFAILURE: c_uint = 0x800 /* consider the return value for failure;
// detection too
// more "out" flags
pub const FD_RAW_FAILURE: c_uint = 0x10000 /* command sent to fdc, fdc returned error */;
pub const FD_RAW_HARDFAILURE: c_uint = 0x20000 /* fdc had to be reset, or timed out */;
    pub data: *mut void __user,
    pub /: *mut *mut *mut char kernel_data; / location of data buffer in the kernel,
    pub cmd's: *mut *mut *mut floppy_raw_cmd next; / used for chaining of raw,
// within the kernel
    pub /: *mut *mut long length; / in: length of dma transfer. out: remaining bytes,
    pub /: *mut *mut long phys_length; / physical length, if different from dma length,
    pub /: *mut *mut int buffer_length; / length of allocated buffer,
    pub rate: c_uchar,
pub const FD_RAW_CMD_SIZE: c_int = 16;
pub const FD_RAW_REPLY_SIZE: c_int = 16;

// The command may take up the space initially intended for the reply
// and the reply count. Needed for long 82078 commands such as RESTORE,
// which takes 17 command bytes.
//
    pub cmd_count: c_uchar,
    pub cmd: [c_uchar; FD_RAW_CMD_SIZE],
    pub reply_count: c_uchar,
    pub reply: [c_uchar; FD_RAW_REPLY_SIZE],
}

// send a raw command to the fdc. Structure size not included, because of
// batches

// flicker motor-on bit before reading a sector. Experimental

// eject the disk
