//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/drm.h
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


// SPDX-License-Identifier: MIT
//
// Header for the Direct Rendering Manager
//
// Author: Rickard E. (Rik) Faith <faith@valinux.com>
//
// Acknowledgments:
// Dec 1999, Richard Henderson <rth@twiddle.net>, move to generic cmpxchg.
//
// Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
// Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
// All rights reserved.
//

pub type drm_handle_t = c_uint;

pub type drm_handle_t = c_uint;

pub type __s8 = i8;
pub type __u8 = u8;
pub type __s16 = i16;
pub type __u16 = u16;
pub type __s32 = i32;
pub type __u32 = u32;
pub type __s64 = i64;
pub type __u64 = u64;
pub type __kernel_size_t = usize;
pub type drm_handle_t = c_ulong;

pub const _DRM_LOCK_HELD: c_uint = 0x80000000U /**< Hardware lock is held */;
pub const _DRM_LOCK_CONT: c_uint = 0x40000000U /**< Hardware lock is contended */;

pub type drm_context_t = c_uint;
pub type drm_drawable_t = c_uint;
pub type drm_magic_t = c_uint;
//
// Cliprect.
//
// \warning: If you change this structure, make sure you change
// XF86DRIClipRectRec in the server as well
//
// \note KW: Actually it's illegal to change either for
// backwards-compatibility reasons.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_clip_rect {
    pub x1: c_ushort,
    pub y1: c_ushort,
    pub x2: c_ushort,
    pub y2: c_ushort,
}

//
// Drawable information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_drawable_info {
    pub num_rects: c_uint,
    pub rects: *mut drm_clip_rect,
}

//
// Texture region,
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tex_region {
    pub next: c_uchar,
    pub prev: c_uchar,
    pub in_use: c_uchar,
    pub padding: c_uchar,
    pub age: c_uint,
}

//
// Hardware lock.
//
// The lock structure is a simple cache-line aligned integer.  To avoid
// processor bus contention on a multiprocessor system, there should not be any
// other data stored in the same cache line.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_hw_lock {
    pub /: *mut *mut *mut __volatile__ unsigned int lock; /< lock variable,
    pub /: *mut *mut *mut char padding[60]; /< Pad to cache line,
}

//
// DRM_IOCTL_VERSION ioctl argument type.
//
// \sa drmGetVersion().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_version {
    pub /: *mut *mut *mut int version_major; /< Major version,
    pub /: *mut *mut *mut int version_minor; /< Minor version,
    pub /: *mut *mut *mut int version_patchlevel; /< Patch level,
    pub /: *mut *mut *mut __kernel_size_t name_len; /< Length of name buffer,
    pub /: *mut *mut *mut *mut char __user name; /< Name of driver,
    pub /: *mut *mut *mut __kernel_size_t date_len; /< Length of date buffer,
    pub /: *mut *mut *mut *mut char __user date; /< User-space buffer to hold date,
    pub /: *mut *mut *mut __kernel_size_t desc_len; /< Length of desc buffer,
    pub /: *mut *mut *mut *mut char __user desc; /< User-space buffer to hold desc,
}

//
// DRM_IOCTL_GET_UNIQUE ioctl argument type.
//
// \sa drmGetBusid() and drmSetBusId().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_unique {
    pub /: *mut *mut *mut __kernel_size_t unique_len; /< Length of unique,
    pub /: *mut *mut *mut *mut char __user unique; /< Unique name for driver instantiation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_list {
    pub /: *mut *mut *mut int count; /< Length of user-space structures,
    pub version: *mut drm_version __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_block {
    pub unused: c_int,
}

//
// DRM_IOCTL_CONTROL ioctl argument type.
//
// \sa drmCtlInstHandler() and drmCtlUninstHandler().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_control {
    pub func: },
    pub irq: c_int,
}

//
// Type of memory to map.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_map_type {
    _DRM_FRAME_BUFFER = 0,	  /**< WC (no caching), no core dump */
    _DRM_REGISTERS = 1,	  /**< no caching, no core dump */
    _DRM_SHM = 2,		  /**< shared, cached */
    _DRM_AGP = 3,		  /**< AGP/GART */
    _DRM_SCATTER_GATHER = 4,  /**< Scatter/gather memory for PCI DMA */
    _DRM_CONSISTENT = 5	  /**< Consistent memory for PCI DMA */
}

//
// Memory mapping flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_map_flags {
    _DRM_RESTRICTED = 0x01,	     /**< Cannot be mapped to user-virtual */
    _DRM_READ_ONLY = 0x02,
    _DRM_LOCKED = 0x04,	     /**< shared, cached, locked */
    _DRM_KERNEL = 0x08,	     /**< kernel requires access */
    _DRM_WRITE_COMBINING = 0x10, /**< use write-combining if available */
    _DRM_CONTAINS_LOCK = 0x20,   /**< SHM page that contains lock */
    _DRM_REMOVABLE = 0x40,	     /**< Removable mapping */
    _DRM_DRIVER = 0x80	     /**< Managed by driver */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ctx_priv_map {
    pub /: *mut *mut *mut unsigned int ctx_id; /< Context requesting private mapping,
    pub /: *mut *mut *mut *mut void handle; /< Handle of map,
}

//
// DRM_IOCTL_GET_MAP, DRM_IOCTL_ADD_MAP and DRM_IOCTL_RM_MAP ioctls
// argument type.
//
// \sa drmAddMap().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_map {
    pub SAREA)*/: *mut *mut *mut unsigned long offset; /< Requested physical address (0 for,
    pub /: *mut *mut *mut unsigned long size; /< Requested physical size (bytes),
    pub /: *mut *mut *mut drm_map_type type; /< Type of memory to map,
    pub /: *mut *mut *mut drm_map_flags flags; /< Flags,
    pub /: *mut *mut *mut *mut void handle; /< User-space: "Handle" to pass to mmap(),
// < Kernel-space: kernel-virtual address
    pub /: *mut *mut *mut int mtrr; /< MTRR slot used,
// Private data
}

//
// DRM_IOCTL_GET_CLIENT ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_client {
    pub /: *mut *mut *mut int idx; /< Which client desired?,
    pub /: *mut *mut *mut int auth; /< Is client authenticated?,
    pub /: *mut *mut *mut unsigned long pid; /< Process ID,
    pub /: *mut *mut *mut unsigned long uid; /< User ID,
    pub /: *mut *mut *mut unsigned long magic; /< Magic,
    pub /: *mut *mut *mut unsigned long iocs; /< Ioctl count,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_stat_type {
    _DRM_STAT_LOCK,
    _DRM_STAT_OPENS,
    _DRM_STAT_CLOSES,
    _DRM_STAT_IOCTLS,
    _DRM_STAT_LOCKS,
    _DRM_STAT_UNLOCKS,
    _DRM_STAT_VALUE,	/**< Generic value */
    _DRM_STAT_BYTE,		/**< Generic byte counter (1024bytes/K) */
    _DRM_STAT_COUNT,	/**< Generic non-byte counter (1000/k) */

    _DRM_STAT_IRQ,		/**< IRQ */
    _DRM_STAT_PRIMARY,	/**< Primary DMA bytes */
    _DRM_STAT_SECONDARY,	/**< Secondary DMA bytes */
    _DRM_STAT_DMA,		/**< DMA */
    _DRM_STAT_SPECIAL,	/**< Special DMA (e.g., priority or polled) */
    _DRM_STAT_MISSED	/**< Missed DMA opportunity */
// Add to the *END* of the list
}

//
// DRM_IOCTL_GET_STATS ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_stats {
    pub count: c_ulong,
    pub value: c_ulong,
    pub type: drm_stat_type,
    pub data: [}; 15],
}

//
// Hardware locking flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_lock_flags {
    _DRM_LOCK_READY = 0x01,	     /**< Wait until hardware is ready for DMA */
    _DRM_LOCK_QUIESCENT = 0x02,  /**< Wait until hardware quiescent */
    _DRM_LOCK_FLUSH = 0x04,	     /**< Flush this context's DMA queue first */
    _DRM_LOCK_FLUSH_ALL = 0x08,  /**< Flush all DMA queues first */
// These *HALT* flags aren't supported yet
    -- they will be used to support the
    full-screen DGA-like mode. */
    _DRM_HALT_ALL_QUEUES = 0x10, /**< Halt all current and future queues */
    _DRM_HALT_CUR_QUEUES = 0x20  /**< Halt all current queues */
}

//
// DRM_IOCTL_LOCK, DRM_IOCTL_UNLOCK and DRM_IOCTL_FINISH ioctl argument type.
//
// \sa drmGetLock() and drmUnlock().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lock {
    pub context: c_int,
    pub flags: drm_lock_flags,
}

//
// DMA flags
//
// \warning
// These values \e must match xf86drm.h.
//
// \sa drm_dma.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_dma_flags {
// Flags for DMA buffer dispatch
    _DRM_DMA_BLOCK = 0x01,	      /**<
// Block until buffer dispatched.
//
// \note The buffer may not yet have
// been processed by the hardware --
// getting a hardware lock with the
// hardware quiescent will ensure
// that the buffer has been
// processed.
//
    _DRM_DMA_WHILE_LOCKED = 0x02, /**< Dispatch while lock held */
    _DRM_DMA_PRIORITY = 0x04,     /**< High priority dispatch */

// Flags for DMA buffer request
    _DRM_DMA_WAIT = 0x10,	      /**< Wait for free buffers */
    _DRM_DMA_SMALLER_OK = 0x20,   /**< Smaller-than-requested buffers OK */
    _DRM_DMA_LARGER_OK = 0x40     /**< Larger-than-requested buffers OK */
}

//
// DRM_IOCTL_ADD_BUFS and DRM_IOCTL_MARK_BUFS ioctl argument type.
//
// \sa drmAddBufs().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_desc {
    pub /: *mut *mut *mut int count; /< Number of buffers of this size,
    pub /: *mut *mut *mut int size; /< Size in bytes,
    pub /: *mut *mut *mut int low_mark; /< Low water mark,
    pub /: *mut *mut *mut int high_mark; /< High water mark,
    pub flags: },
    pub /**<: *mut unsigned long agp_start;,
// Start address of where the AGP buffers are
// in the AGP aperture
//
}

//
// DRM_IOCTL_INFO_BUFS ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_info {
    pub /: *mut *mut *mut int count; /< Entries in list,
    pub list: *mut drm_buf_desc __user,
}

//
// DRM_IOCTL_FREE_BUFS ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_free {
    pub count: c_int,
    pub list: *mut int __user,
}

//
// Buffer information
//
// \sa drm_buf_map.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_pub {
    pub /: *mut *mut *mut int idx; /< Index into the master buffer list,
    pub /: *mut *mut *mut int total; /< Buffer size,
    pub /: *mut *mut *mut int used; /< Amount of buffer in use (for DMA),
    pub /: *mut *mut *mut *mut void __user address; /< Address of buffer,
}

//
// DRM_IOCTL_MAP_BUFS ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_buf_map {
    pub /: *mut *mut *mut int count; /< Length of the buffer list,

    pub virt: *mut void __user,

    pub /: *mut *mut *mut *mut void __user virtual; /< Mmap'd area in user-virtual,

    pub /: *mut *mut *mut *mut drm_buf_pub __user list; /< Buffer information,
}

//
// DRM_IOCTL_DMA ioctl argument type.
//
// Indices here refer to the offset into the buffer list in drm_buf_get.
//
// \sa drmDMA().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dma {
    pub /: *mut *mut *mut int context; /< Context handle,
    pub /: *mut *mut *mut int send_count; /< Number of buffers to send,
    pub /: *mut *mut *mut *mut int __user send_indices; /< List of handles to buffers,
    pub /: *mut *mut *mut *mut int __user send_sizes; /< Lengths of data to send,
    pub /: *mut *mut *mut drm_dma_flags flags; /< Flags,
    pub /: *mut *mut *mut int request_count; /< Number of buffers requested,
    pub /: *mut *mut *mut int request_size; /< Desired size for buffers,
    pub /: *mut *mut *mut *mut int __user request_indices; /< Buffer information,
    pub request_sizes: *mut int __user,
    pub /: *mut *mut *mut int granted_count; /< Number of buffers granted,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_ctx_flags {
    _DRM_CONTEXT_PRESERVED = 0x01,
    _DRM_CONTEXT_2DONLY = 0x02
}

//
// DRM_IOCTL_ADD_CTX ioctl argument type.
//
// \sa drmCreateContext() and drmDestroyContext().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ctx {
    pub handle: drm_context_t,
    pub flags: drm_ctx_flags,
}

//
// DRM_IOCTL_RES_CTX ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ctx_res {
    pub count: c_int,
    pub contexts: *mut drm_ctx __user,
}

//
// DRM_IOCTL_ADD_DRAW and DRM_IOCTL_RM_DRAW ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_draw {
    pub handle: drm_drawable_t,
}

//
// DRM_IOCTL_UPDATE_DRAW ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_update_draw {
    pub handle: drm_drawable_t,
    pub type: c_uint,
    pub num: c_uint,
    pub data: c_ulonglong,
}

//
// DRM_IOCTL_GET_MAGIC and DRM_IOCTL_AUTH_MAGIC ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_auth {
    pub magic: drm_magic_t,
}

//
// DRM_IOCTL_IRQ_BUSID ioctl argument type.
//
// \sa drmGetInterruptFromBusID().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_irq_busid {
    pub /: *mut *mut *mut int irq; /< IRQ number,
    pub /: *mut *mut *mut int busnum; /< bus number,
    pub /: *mut *mut *mut int devnum; /< device number,
    pub /: *mut *mut *mut int funcnum; /< function number,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_vblank_seq_type {
    _DRM_VBLANK_ABSOLUTE = 0x0,	/**< Wait for specific vblank sequence number */
    _DRM_VBLANK_RELATIVE = 0x1,	/**< Wait for given number of vblanks */
// bits 1-6 are reserved for high crtcs
    _DRM_VBLANK_HIGH_CRTC_MASK = 0x0000003e,
    _DRM_VBLANK_EVENT = 0x4000000,   /**< Send event instead of blocking */
    _DRM_VBLANK_FLIP = 0x8000000,   /**< Scheduled buffer swap should flip */
    _DRM_VBLANK_NEXTONMISS = 0x10000000,	/**< If missed, wait for next vblank */
    _DRM_VBLANK_SECONDARY = 0x20000000,	/**< Secondary display controller */
    _DRM_VBLANK_SIGNAL = 0x40000000	/**< Send signal instead of blocking, unsupported */
}

pub const _DRM_VBLANK_HIGH_CRTC_SHIFT: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_wait_vblank_request {
    pub type: drm_vblank_seq_type,
    pub sequence: c_uint,
    pub signal: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_wait_vblank_reply {
    pub type: drm_vblank_seq_type,
    pub sequence: c_uint,
    pub tval_sec: c_long,
    pub tval_usec: c_long,
}

//
// DRM_IOCTL_WAIT_VBLANK ioctl argument type.
//
// \sa drmWaitVBlank().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_wait_vblank {
    pub request: drm_wait_vblank_request,
    pub reply: drm_wait_vblank_reply,
}

pub const _DRM_PRE_MODESET: c_int = 1;
pub const _DRM_POST_MODESET: c_int = 2;
//
// DRM_IOCTL_MODESET_CTL ioctl argument type
//
// \sa drmModesetCtl().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_modeset_ctl {
    pub crtc: __u32,
    pub cmd: __u32,
}

//
// DRM_IOCTL_AGP_ENABLE ioctl argument type.
//
// \sa drmAgpEnable().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_agp_mode {
    pub /: *mut *mut *mut unsigned long mode; /< AGP mode,
}

//
// DRM_IOCTL_AGP_ALLOC and DRM_IOCTL_AGP_FREE ioctls argument type.
//
// \sa drmAgpAlloc() and drmAgpFree().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_agp_buffer {
    pub /: *mut *mut *mut unsigned long size; /< In bytes -- will round to page boundary,
    pub /: *mut *mut *mut unsigned long handle; /< Used for binding / unbinding,
    pub /: *mut *mut *mut unsigned long type; /< Type of memory to allocate,
    pub /: *mut *mut *mut unsigned long physical; /< Physical used by i810,
}

//
// DRM_IOCTL_AGP_BIND and DRM_IOCTL_AGP_UNBIND ioctls argument type.
//
// \sa drmAgpBind() and drmAgpUnbind().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_agp_binding {
    pub /: *mut *mut *mut unsigned long handle; /< From drm_agp_buffer,
    pub /: *mut *mut *mut unsigned long offset; /< In bytes -- will round to page boundary,
}

//
// DRM_IOCTL_AGP_INFO ioctl argument type.
//
// \sa drmAgpVersionMajor(), drmAgpVersionMinor(), drmAgpGetMode(),
// drmAgpBase(), drmAgpSize(), drmAgpMemoryUsed(), drmAgpMemoryAvail(),
// drmAgpVendorId() and drmAgpDeviceId().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_agp_info {
    pub agp_version_major: c_int,
    pub agp_version_minor: c_int,
    pub mode: c_ulong,
    pub /: *mut *mut unsigned long aperture_base; / physical address,
    pub /: *mut *mut unsigned long aperture_size; / bytes,
    pub /: *mut *mut unsigned long memory_allowed; / bytes,
    pub memory_used: c_ulong,
// PCI information
    pub id_vendor: c_ushort,
    pub id_device: c_ushort,
}

//
// DRM_IOCTL_SG_ALLOC ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_scatter_gather {
    pub /: *mut *mut *mut unsigned long size; /< In bytes -- will round to page boundary,
    pub /: *mut *mut *mut unsigned long handle; /< Used for mapping / unmapping,
}

//
// DRM_IOCTL_SET_VERSION ioctl argument type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_set_version {
    pub drm_di_major: c_int,
    pub drm_di_minor: c_int,
    pub drm_dd_major: c_int,
    pub drm_dd_minor: c_int,
}

//
// struct drm_gem_close - Argument for &DRM_IOCTL_GEM_CLOSE ioctl.
// @handle: Handle of the object to be closed.
// @pad: Padding.
//
// Releases the handle to an mm object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_close {
    pub handle: __u32,
    pub pad: __u32,
}

//
// struct drm_gem_flink - Argument for &DRM_IOCTL_GEM_FLINK ioctl.
// @handle: Handle for the object being named.
// @name: Returned global name.
//
// Create a global name for an object, returning the name.
//
// Note that the name does not hold a reference; when the object
// is freed, the name goes away.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_flink {
    pub handle: __u32,
    pub name: __u32,
}

//
// struct drm_gem_open - Argument for &DRM_IOCTL_GEM_OPEN ioctl.
// @name: Name of object being opened.
// @handle: Returned handle for the object.
// @size: Returned size of the object
//
// Open an object using the global name, returning a handle and the size.
//
// This handle (of course) holds a reference to the object, so the object
// will not go away until the handle is deleted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_open {
    pub name: __u32,
    pub handle: __u32,
    pub size: __u64,
}

//
// struct drm_gem_change_handle - Argument for &DRM_IOCTL_GEM_CHANGE_HANDLE ioctl.
// @handle: The handle of a gem object.
// @new_handle: An available gem handle.
//
// This ioctl changes the handle of a GEM object to the specified one.
// The new handle must be unused. On success the old handle is closed
// and all further IOCTL should refer to the new handle only.
// Calls to DRM_IOCTL_PRIME_FD_TO_HANDLE will return the new handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_gem_change_handle {
    pub handle: __u32,
    pub new_handle: __u32,
}

//
// DRM_CAP_DUMB_BUFFER
//
// If set to 1, the driver supports creating dumb buffers via the
// &DRM_IOCTL_MODE_CREATE_DUMB ioctl.
//
pub const DRM_CAP_DUMB_BUFFER: c_uint = 0x1;
//
// DRM_CAP_VBLANK_HIGH_CRTC
//
// If set to 1, the kernel supports specifying a :ref:`CRTC index<crtc_index>`
// in the high bits of &drm_wait_vblank_request.type.
//
// Starting kernel version 2.6.39, this capability is always set to 1.
//
pub const DRM_CAP_VBLANK_HIGH_CRTC: c_uint = 0x2;
//
// DRM_CAP_DUMB_PREFERRED_DEPTH
//
// The preferred bit depth for dumb buffers.
//
// The bit depth is the number of bits used to indicate the color of a single
// pixel excluding any padding. This is different from the number of bits per
// pixel. For instance, XRGB8888 has a bit depth of 24 but has 32 bits per
// pixel.
//
// Note that this preference only applies to dumb buffers, it's irrelevant for
// other types of buffers.
//
pub const DRM_CAP_DUMB_PREFERRED_DEPTH: c_uint = 0x3;
//
// DRM_CAP_DUMB_PREFER_SHADOW
//
// If set to 1, the driver prefers userspace to render to a shadow buffer
// instead of directly rendering to a dumb buffer. For best speed, userspace
// should do streaming ordered memory copies into the dumb buffer and never
// read from it.
//
// Note that this preference only applies to dumb buffers, it's irrelevant for
// other types of buffers.
//
pub const DRM_CAP_DUMB_PREFER_SHADOW: c_uint = 0x4;
//
// DRM_CAP_PRIME
//
// Bitfield of supported PRIME sharing capabilities. See &DRM_PRIME_CAP_IMPORT
// and &DRM_PRIME_CAP_EXPORT.
//
// Starting from kernel version 6.6, both &DRM_PRIME_CAP_IMPORT and
// &DRM_PRIME_CAP_EXPORT are always advertised.
//
// PRIME buffers are exposed as dma-buf file descriptors.
// See :ref:`prime_buffer_sharing`.
//
pub const DRM_CAP_PRIME: c_uint = 0x5;
//
// DRM_PRIME_CAP_IMPORT
//
// If this bit is set in &DRM_CAP_PRIME, the driver supports importing PRIME
// buffers via the &DRM_IOCTL_PRIME_FD_TO_HANDLE ioctl.
//
// Starting from kernel version 6.6, this bit is always set in &DRM_CAP_PRIME.
//
pub const DRM_PRIME_CAP_IMPORT: c_uint = 0x1;
//
// DRM_PRIME_CAP_EXPORT
//
// If this bit is set in &DRM_CAP_PRIME, the driver supports exporting PRIME
// buffers via the &DRM_IOCTL_PRIME_HANDLE_TO_FD ioctl.
//
// Starting from kernel version 6.6, this bit is always set in &DRM_CAP_PRIME.
//
pub const DRM_PRIME_CAP_EXPORT: c_uint = 0x2;
//
// DRM_CAP_TIMESTAMP_MONOTONIC
//
// If set to 0, the kernel will report timestamps with ``CLOCK_REALTIME`` in
// struct drm_event_vblank. If set to 1, the kernel will report timestamps with
// ``CLOCK_MONOTONIC``. See ``clock_gettime(2)`` for the definition of these
// clocks.
//
// Starting from kernel version 2.6.39, the default value for this capability
// is 1. Starting kernel version 4.15, this capability is always set to 1.
//
pub const DRM_CAP_TIMESTAMP_MONOTONIC: c_uint = 0x6;
//
// DRM_CAP_ASYNC_PAGE_FLIP
//
// If set to 1, the driver supports &DRM_MODE_PAGE_FLIP_ASYNC for legacy
// page-flips.
//
pub const DRM_CAP_ASYNC_PAGE_FLIP: c_uint = 0x7;
//
// DRM_CAP_CURSOR_WIDTH
//
// The ``CURSOR_WIDTH`` and ``CURSOR_HEIGHT`` capabilities return a valid
// width x height combination for the hardware cursor. The intention is that a
// hardware agnostic userspace can query a cursor plane size to use.
//
// Note that the cross-driver contract is to merely return a valid size;
// drivers are free to attach another meaning on top, eg. i915 returns the
// maximum plane size.
//
pub const DRM_CAP_CURSOR_WIDTH: c_uint = 0x8;
//
// DRM_CAP_CURSOR_HEIGHT
//
// See &DRM_CAP_CURSOR_WIDTH.
//
pub const DRM_CAP_CURSOR_HEIGHT: c_uint = 0x9;
//
// DRM_CAP_ADDFB2_MODIFIERS
//
// If set to 1, the driver supports supplying modifiers in the
// &DRM_IOCTL_MODE_ADDFB2 ioctl.
//
pub const DRM_CAP_ADDFB2_MODIFIERS: c_uint = 0x10;
//
// DRM_CAP_PAGE_FLIP_TARGET
//
// If set to 1, the driver supports the &DRM_MODE_PAGE_FLIP_TARGET_ABSOLUTE and
// &DRM_MODE_PAGE_FLIP_TARGET_RELATIVE flags in
// &drm_mode_crtc_page_flip_target.flags for the &DRM_IOCTL_MODE_PAGE_FLIP
// ioctl.
//
pub const DRM_CAP_PAGE_FLIP_TARGET: c_uint = 0x11;
//
// DRM_CAP_CRTC_IN_VBLANK_EVENT
//
// If set to 1, the kernel supports reporting the CRTC ID in
// &drm_event_vblank.crtc_id for the &DRM_EVENT_VBLANK and
// &DRM_EVENT_FLIP_COMPLETE events.
//
// Starting kernel version 4.12, this capability is always set to 1.
//
pub const DRM_CAP_CRTC_IN_VBLANK_EVENT: c_uint = 0x12;
//
// DRM_CAP_SYNCOBJ
//
// If set to 1, the driver supports sync objects. See :ref:`drm_sync_objects`.
//
pub const DRM_CAP_SYNCOBJ: c_uint = 0x13;
//
// DRM_CAP_SYNCOBJ_TIMELINE
//
// If set to 1, the driver supports timeline operations on sync objects. See
// :ref:`drm_sync_objects`.
//
pub const DRM_CAP_SYNCOBJ_TIMELINE: c_uint = 0x14;
//
// DRM_CAP_ATOMIC_ASYNC_PAGE_FLIP
//
// If set to 1, the driver supports &DRM_MODE_PAGE_FLIP_ASYNC for atomic
// commits.
//
pub const DRM_CAP_ATOMIC_ASYNC_PAGE_FLIP: c_uint = 0x15;
// DRM_IOCTL_GET_CAP ioctl argument type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_get_cap {
    pub capability: __u64,
    pub value: __u64,
}

//
// DRM_CLIENT_CAP_STEREO_3D
//
// If set to 1, the DRM core will expose the stereo 3D capabilities of the
// monitor by advertising the supported 3D layouts in the flags of struct
// drm_mode_modeinfo. See ``DRM_MODE_FLAG_3D_*``.
//
// This capability is always supported for all drivers starting from kernel
// version 3.13.
//
pub const DRM_CLIENT_CAP_STEREO_3D: c_int = 1;
//
// DRM_CLIENT_CAP_UNIVERSAL_PLANES
//
// If set to 1, the DRM core will expose all planes (overlay, primary, and
// cursor) to userspace.
//
// This capability has been introduced in kernel version 3.15. Starting from
// kernel version 3.17, this capability is always supported for all drivers.
//
pub const DRM_CLIENT_CAP_UNIVERSAL_PLANES: c_int = 2;
//
// DRM_CLIENT_CAP_ATOMIC
//
// If set to 1, the DRM core will expose atomic properties to userspace. This
// implicitly enables &DRM_CLIENT_CAP_UNIVERSAL_PLANES and
// &DRM_CLIENT_CAP_ASPECT_RATIO.
//
// If the driver doesn't support atomic mode-setting, enabling this capability
// will fail with -EOPNOTSUPP.
//
// This capability has been introduced in kernel version 4.0. Starting from
// kernel version 4.2, this capability is always supported for atomic-capable
// drivers.
//
pub const DRM_CLIENT_CAP_ATOMIC: c_int = 3;
//
// DRM_CLIENT_CAP_ASPECT_RATIO
//
// If set to 1, the DRM core will provide aspect ratio information in modes.
// See ``DRM_MODE_FLAG_PIC_AR_*``.
//
// This capability is always supported for all drivers starting from kernel
// version 4.18.
//
pub const DRM_CLIENT_CAP_ASPECT_RATIO: c_int = 4;
//
// DRM_CLIENT_CAP_WRITEBACK_CONNECTORS
//
// If set to 1, the DRM core will expose special connectors to be used for
// writing back to memory the scene setup in the commit. The client must enable
// &DRM_CLIENT_CAP_ATOMIC first.
//
// This capability is always supported for atomic-capable drivers starting from
// kernel version 4.19.
//
pub const DRM_CLIENT_CAP_WRITEBACK_CONNECTORS: c_int = 5;
//
// DRM_CLIENT_CAP_CURSOR_PLANE_HOTSPOT
//
// Drivers for para-virtualized hardware (e.g. vmwgfx, qxl, virtio and
// virtualbox) have additional restrictions for cursor planes (thus
// making cursor planes on those drivers not truly universal,) e.g.
// they need cursor planes to act like one would expect from a mouse
// cursor and have correctly set hotspot properties.
// If this client cap is not set the DRM core will hide cursor plane on
// those virtualized drivers because not setting it implies that the
// client is not capable of dealing with those extra restictions.
// Clients which do set cursor hotspot and treat the cursor plane
// like a mouse cursor should set this property.
// The client must enable &DRM_CLIENT_CAP_ATOMIC first.
//
// Setting this property on drivers which do not special case
// cursor planes (i.e. non-virtualized drivers) will return
// EOPNOTSUPP, which can be used by userspace to gauge
// requirements of the hardware/drivers they're running on.
//
// This capability is always supported for atomic-capable virtualized
// drivers starting from kernel version 6.6.
//
pub const DRM_CLIENT_CAP_CURSOR_PLANE_HOTSPOT: c_int = 6;
//
// DRM_CLIENT_CAP_PLANE_COLOR_PIPELINE
//
// If set to 1 the DRM core will allow setting the COLOR_PIPELINE
// property on a &drm_plane, as well as drm_colorop properties.
//
// Setting of these plane properties will be rejected when this client
// cap is set:
// - COLOR_ENCODING
// - COLOR_RANGE
//
// The client must enable &DRM_CLIENT_CAP_ATOMIC first.
//
pub const DRM_CLIENT_CAP_PLANE_COLOR_PIPELINE: c_int = 7;
// DRM_IOCTL_SET_CLIENT_CAP ioctl argument type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_set_client_cap {
    pub capability: __u64,
    pub value: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_prime_handle {
    pub handle: __u32,
// Flags.. only applicable for handle->fd
    pub flags: __u32,
// Returned dmabuf file descriptor
    pub fd: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_create {
    pub handle: __u32,

    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_destroy {
    pub handle: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_handle {
    pub handle: __u32,
    pub flags: __u32,
    pub fd: __s32,
    pub pad: __u32,
    pub point: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_transfer {
    pub src_handle: __u32,
    pub dst_handle: __u32,
    pub src_point: __u64,
    pub dst_point: __u64,
    pub flags: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_wait {
    pub handles: __u64,
// absolute timeout
    pub timeout_nsec: __s64,
    pub count_handles: __u32,
    pub flags: __u32,
    pub /: *mut *mut __u32 first_signaled; / only valid when not waiting all,
    pub pad: __u32,
//
// @deadline_nsec - fence deadline hint
//
// Deadline hint, in absolute CLOCK_MONOTONIC, to set on backing
// fence(s) if the DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE flag is
// set.
//
    pub deadline_nsec: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_timeline_wait {
    pub handles: __u64,
// wait on specific timeline point for every handles
    pub points: __u64,
// absolute timeout
    pub timeout_nsec: __s64,
    pub count_handles: __u32,
    pub flags: __u32,
    pub /: *mut *mut __u32 first_signaled; / only valid when not waiting all,
    pub pad: __u32,
//
// @deadline_nsec - fence deadline hint
//
// Deadline hint, in absolute CLOCK_MONOTONIC, to set on backing
// fence(s) if the DRM_SYNCOBJ_WAIT_FLAGS_WAIT_DEADLINE flag is
// set.
//
    pub deadline_nsec: __u64,
}

//
// struct drm_syncobj_eventfd
// @handle: syncobj handle.
// @flags: Zero to wait for the point to be signalled, or
// &DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE to wait for a fence to be
// available for the point.
// @point: syncobj timeline point (set to zero for binary syncobjs).
// @fd: Existing eventfd to sent events to.
// @pad: Must be zero.
//
// Register an eventfd to be signalled by a syncobj. The eventfd counter will
// be incremented by one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_eventfd {
    pub handle: __u32,
    pub flags: __u32,
    pub point: __u64,
    pub fd: __s32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_array {
    pub handles: __u64,
    pub count_handles: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_syncobj_timeline_array {
    pub handles: __u64,
    pub points: __u64,
    pub count_handles: __u32,
    pub flags: __u32,
}

// Query current scanout sequence number
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_get_sequence {
    pub /: *mut *mut __u32 crtc_id; / requested crtc_id,
    pub /: *mut *mut __u32 active; / return: crtc output is active,
    pub /: *mut *mut __u64 sequence; / return: most recent vblank sequence,
    pub /: *mut *mut __s64 sequence_ns; / return: most recent time of first pixel out,
}

// Queue event to be delivered at specified sequence. Time stamp marks
// when the first pixel of the refresh cycle leaves the display engine
// for the display
//
pub const DRM_CRTC_SEQUENCE_RELATIVE: c_uint = 0x00000001	/* sequence is relative to current */;
pub const DRM_CRTC_SEQUENCE_NEXT_ON_MISS: c_uint = 0x00000002	/* Use next sequence if we've missed */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_queue_sequence {
    pub crtc_id: __u32,
    pub flags: __u32,
    pub /: *mut *mut __u64 sequence; / on input, target sequence. on output, actual sequence,
    pub /: *mut *mut __u64 user_data; / user data passed to event,
}

pub const DRM_CLIENT_NAME_MAX_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_set_client_name {
    pub name_len: __u64,
    pub name: __u64,
}

//
// DRM_IOCTL_GEM_CLOSE - Close a GEM handle.
//
// GEM handles are not reference-counted by the kernel. User-space is
// responsible for managing their lifetime. For example, if user-space imports
// the same memory object twice on the same DRM file description, the same GEM
// handle is returned by both imports, and user-space needs to ensure
// &DRM_IOCTL_GEM_CLOSE is performed once only. The same situation can happen
// when a memory object is allocated, then exported and imported again on the
// same DRM file description. The &DRM_IOCTL_MODE_GETFB2 IOCTL is an exception
// and always returns fresh new GEM handles even if an existing GEM handle
// already refers to the same memory object before the IOCTL is performed.
//

//
// DRM_IOCTL_PRIME_HANDLE_TO_FD - Convert a GEM handle to a DMA-BUF FD.
//
// User-space sets &drm_prime_handle.handle with the GEM handle to export and
// &drm_prime_handle.flags, and gets back a DMA-BUF file descriptor in
// &drm_prime_handle.fd.
//
// The export can fail for any driver-specific reason, e.g. because export is
// not supported for this specific GEM handle (but might be for others).
//
// Support for exporting DMA-BUFs is advertised via &DRM_PRIME_CAP_EXPORT.
//

//
// DRM_IOCTL_PRIME_FD_TO_HANDLE - Convert a DMA-BUF FD to a GEM handle.
//
// User-space sets &drm_prime_handle.fd with a DMA-BUF file descriptor to
// import, and gets back a GEM handle in &drm_prime_handle.handle.
// &drm_prime_handle.flags is unused.
//
// If an existing GEM handle refers to the memory object backing the DMA-BUF,
// that GEM handle is returned. Therefore user-space which needs to handle
// arbitrary DMA-BUFs must have a user-space lookup data structure to manually
// reference-count duplicated GEM handles. For more information see
// &DRM_IOCTL_GEM_CLOSE.
//
// The import can fail for any driver-specific reason, e.g. because import is
// only supported for DMA-BUFs allocated on this DRM device.
//
// Support for importing DMA-BUFs is advertised via &DRM_PRIME_CAP_IMPORT.
//

//
// DRM_IOCTL_MODE_RMFB - Remove a framebuffer.
//
// This removes a framebuffer previously added via ADDFB/ADDFB2. The IOCTL
// argument is a framebuffer object ID.
//
// Warning: removing a framebuffer currently in-use on an enabled plane will
// disable that plane. The CRTC the plane is linked to may also be disabled
// (depending on driver capabilities).
//

//
// DRM_IOCTL_MODE_CREATE_DUMB - Create a new dumb buffer object.
//
// KMS dumb buffers provide a very primitive way to allocate a buffer object
// suitable for scanout and map it for software rendering. KMS dumb buffers are
// not suitable for hardware-accelerated rendering nor video decoding. KMS dumb
// buffers are not suitable to be displayed on any other device than the KMS
// device where they were allocated from. Also see
// :ref:`kms_dumb_buffer_objects`.
//
// The IOCTL argument is a struct drm_mode_create_dumb.
//
// User-space is expected to create a KMS dumb buffer via this IOCTL, then add
// it as a KMS framebuffer via &DRM_IOCTL_MODE_ADDFB and map it via
// &DRM_IOCTL_MODE_MAP_DUMB.
//
// &DRM_CAP_DUMB_BUFFER indicates whether this IOCTL is supported.
// &DRM_CAP_DUMB_PREFERRED_DEPTH and &DRM_CAP_DUMB_PREFER_SHADOW indicate
// driver preferences for dumb buffers.
//

//
// DRM_IOCTL_MODE_GETFB2 - Get framebuffer metadata.
//
// This queries metadata about a framebuffer. User-space fills
// &drm_mode_fb_cmd2.fb_id as the input, and the kernels fills the rest of the
// struct as the output.
//
// If the client is DRM master or has &CAP_SYS_ADMIN, &drm_mode_fb_cmd2.handles
// will be filled with GEM buffer handles. Fresh new GEM handles are always
// returned, even if another GEM handle referring to the same memory object
// already exists on the DRM file description. The caller is responsible for
// removing the new handles, e.g. via the &DRM_IOCTL_GEM_CLOSE IOCTL. The same
// new handle will be returned for multiple planes in case they use the same
// memory object. Planes are valid until one has a zero handle -- this can be
// used to compute the number of planes.
//
// Otherwise, &drm_mode_fb_cmd2.handles will be zeroed and planes are valid
// until one has a zero &drm_mode_fb_cmd2.pitches.
//
// If the framebuffer has a format modifier, &DRM_MODE_FB_MODIFIERS will be set
// in &drm_mode_fb_cmd2.flags and &drm_mode_fb_cmd2.modifier will contain the
// modifier. Otherwise, user-space must ignore &drm_mode_fb_cmd2.modifier.
//
// To obtain DMA-BUF FDs for each plane without leaking GEM handles, user-space
// can export each handle via &DRM_IOCTL_PRIME_HANDLE_TO_FD, then immediately
// close each unique handle via &DRM_IOCTL_GEM_CLOSE, making sure to not
// double-close handles which are specified multiple times in the array.
//

//
// DRM_IOCTL_SYNCOBJ_EVENTFD - Register an eventfd to be signalled by a syncobj.
//
// This can be used to integrate a syncobj in an event loop.
//
// The IOCTL argument is a struct drm_syncobj_eventfd.
//

//
// DRM_IOCTL_MODE_CLOSEFB - Close a framebuffer.
//
// This closes a framebuffer previously added via ADDFB/ADDFB2. The IOCTL
// argument is a framebuffer object ID.
//
// This IOCTL is similar to &DRM_IOCTL_MODE_RMFB, except it doesn't disable
// planes and CRTCs. As long as the framebuffer is used by a plane, it's kept
// alive. When the plane no longer uses the framebuffer (because the
// framebuffer is replaced with another one, or the plane is disabled), the
// framebuffer is cleaned up.
//
// This is useful to implement flicker-free transitions between two processes.
//
// Depending on the threat model, user-space may want to ensure that the
// framebuffer doesn't expose any sensitive user information: closed
// framebuffers attached to a plane can be read back by the next DRM master.
//

//
// DRM_IOCTL_SET_CLIENT_NAME - Attach a name to a drm_file
//
// Having a name allows for easier tracking and debugging.
// The length of the name (without null ending char) must be
// <= DRM_CLIENT_NAME_MAX_LEN.
// The call will fail if the name contains whitespaces or non-printable chars.
//

//
// DRM_IOCTL_GEM_CHANGE_HANDLE - Move an object to a different handle
//
// Some applications (notably CRIU) need objects to have specific gem handles.
// This ioctl changes the object at one gem handle to use a new gem handle.
//

//
// Device specific ioctls should only be in their respective headers
// The device specific ioctl range is from 0x40 to 0x9f.
// Generic IOCTLS restart at 0xA0.
//
// \sa drmCommandNone(), drmCommandRead(), drmCommandWrite(), and
// drmCommandReadWrite().
//
pub const DRM_COMMAND_BASE: c_uint = 0x40;
pub const DRM_COMMAND_END: c_uint = 0xA0;
//
// struct drm_event - Header for DRM events
// @type: event type.
// @length: total number of payload bytes (including header).
//
// This struct is a header for events written back to user-space on the DRM FD.
// A read on the DRM FD will always only return complete events: e.g. if the
// read buffer is 100 bytes large and there are two 64 byte events pending,
// only one will be returned.
//
// Event types 0 - 0x7fffffff are generic DRM events, 0x80000000 and
// up are chipset specific. Generic DRM events include &DRM_EVENT_VBLANK,
// &DRM_EVENT_FLIP_COMPLETE and &DRM_EVENT_CRTC_SEQUENCE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_event {
    pub type: __u32,
    pub length: __u32,
}

//
// DRM_EVENT_VBLANK - vertical blanking event
//
// This event is sent in response to &DRM_IOCTL_WAIT_VBLANK with the
// &_DRM_VBLANK_EVENT flag set.
//
// The event payload is a struct drm_event_vblank.
//
pub const DRM_EVENT_VBLANK: c_uint = 0x01;
//
// DRM_EVENT_FLIP_COMPLETE - page-flip completion event
//
// This event is sent in response to an atomic commit or legacy page-flip with
// the &DRM_MODE_PAGE_FLIP_EVENT flag set.
//
// The event payload is a struct drm_event_vblank.
//
pub const DRM_EVENT_FLIP_COMPLETE: c_uint = 0x02;
//
// DRM_EVENT_CRTC_SEQUENCE - CRTC sequence event
//
// This event is sent in response to &DRM_IOCTL_CRTC_QUEUE_SEQUENCE.
//
// The event payload is a struct drm_event_crtc_sequence.
//
pub const DRM_EVENT_CRTC_SEQUENCE: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_event_vblank {
    pub base: drm_event,
    pub user_data: __u64,
    pub tv_sec: __u32,
    pub tv_usec: __u32,
    pub sequence: __u32,
    pub /: *mut *mut __u32 crtc_id; / 0 on older kernels that do not support this,
}

// Event delivered at sequence. Time stamp marks when the first pixel
// of the refresh cycle leaves the display engine for the display
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_event_crtc_sequence {
    pub base: drm_event,
    pub user_data: __u64,
    pub time_ns: __s64,
    pub sequence: __u64,
}

// typedef area
pub type drm_clip_rect_t = drm_clip_rect;
pub type drm_drawable_info_t = drm_drawable_info;
pub type drm_tex_region_t = drm_tex_region;
pub type drm_hw_lock_t = drm_hw_lock;
pub type drm_version_t = drm_version;
pub type drm_unique_t = drm_unique;
pub type drm_list_t = drm_list;
pub type drm_block_t = drm_block;
pub type drm_control_t = drm_control;
pub type drm_map_type_t = drm_map_type;
pub type drm_map_flags_t = drm_map_flags;
pub type drm_ctx_priv_map_t = drm_ctx_priv_map;
pub type drm_map_t = drm_map;
pub type drm_client_t = drm_client;
pub type drm_stat_type_t = drm_stat_type;
pub type drm_stats_t = drm_stats;
pub type drm_lock_flags_t = drm_lock_flags;
pub type drm_lock_t = drm_lock;
pub type drm_dma_flags_t = drm_dma_flags;
pub type drm_buf_desc_t = drm_buf_desc;
pub type drm_buf_info_t = drm_buf_info;
pub type drm_buf_free_t = drm_buf_free;
pub type drm_buf_pub_t = drm_buf_pub;
pub type drm_buf_map_t = drm_buf_map;
pub type drm_dma_t = drm_dma;
pub type drm_wait_vblank_t = drm_wait_vblank;
pub type drm_agp_mode_t = drm_agp_mode;
pub type drm_ctx_flags_t = drm_ctx_flags;
pub type drm_ctx_t = drm_ctx;
pub type drm_ctx_res_t = drm_ctx_res;
pub type drm_draw_t = drm_draw;
pub type drm_update_draw_t = drm_update_draw;
pub type drm_auth_t = drm_auth;
pub type drm_irq_busid_t = drm_irq_busid;
pub type drm_vblank_seq_type_t = drm_vblank_seq_type;
pub type drm_agp_buffer_t = drm_agp_buffer;
pub type drm_agp_binding_t = drm_agp_binding;
pub type drm_agp_info_t = drm_agp_info;
pub type drm_scatter_gather_t = drm_scatter_gather;
pub type drm_set_version_t = drm_set_version;

