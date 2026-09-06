//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/vmwgfx_drm.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT
//
// Copyright © 2009-2023 VMware, Inc., Palo Alto, CA., USA
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub const DRM_VMW_MAX_SURFACE_FACES: c_int = 6;
pub const DRM_VMW_MAX_MIP_LEVELS: c_int = 24;
pub const DRM_VMW_GET_PARAM: c_int = 0;
pub const DRM_VMW_ALLOC_DMABUF: c_int = 1;
pub const DRM_VMW_ALLOC_BO: c_int = 1;
pub const DRM_VMW_UNREF_DMABUF: c_int = 2;
pub const DRM_VMW_HANDLE_CLOSE: c_int = 2;
pub const DRM_VMW_CURSOR_BYPASS: c_int = 3;
// guarded by DRM_VMW_PARAM_NUM_STREAMS != 0
pub const DRM_VMW_CONTROL_STREAM: c_int = 4;
pub const DRM_VMW_CLAIM_STREAM: c_int = 5;
pub const DRM_VMW_UNREF_STREAM: c_int = 6;
// guarded by DRM_VMW_PARAM_3D == 1
pub const DRM_VMW_CREATE_CONTEXT: c_int = 7;
pub const DRM_VMW_UNREF_CONTEXT: c_int = 8;
pub const DRM_VMW_CREATE_SURFACE: c_int = 9;
pub const DRM_VMW_UNREF_SURFACE: c_int = 10;
pub const DRM_VMW_REF_SURFACE: c_int = 11;
pub const DRM_VMW_EXECBUF: c_int = 12;
pub const DRM_VMW_GET_3D_CAP: c_int = 13;
pub const DRM_VMW_FENCE_WAIT: c_int = 14;
pub const DRM_VMW_FENCE_SIGNALED: c_int = 15;
pub const DRM_VMW_FENCE_UNREF: c_int = 16;
pub const DRM_VMW_FENCE_EVENT: c_int = 17;
pub const DRM_VMW_PRESENT: c_int = 18;
pub const DRM_VMW_PRESENT_READBACK: c_int = 19;
pub const DRM_VMW_UPDATE_LAYOUT: c_int = 20;
pub const DRM_VMW_CREATE_SHADER: c_int = 21;
pub const DRM_VMW_UNREF_SHADER: c_int = 22;
pub const DRM_VMW_GB_SURFACE_CREATE: c_int = 23;
pub const DRM_VMW_GB_SURFACE_REF: c_int = 24;
pub const DRM_VMW_SYNCCPU: c_int = 25;
pub const DRM_VMW_CREATE_EXTENDED_CONTEXT: c_int = 26;
pub const DRM_VMW_GB_SURFACE_CREATE_EXT: c_int = 27;
pub const DRM_VMW_GB_SURFACE_REF_EXT: c_int = 28;
pub const DRM_VMW_MSG: c_int = 29;
pub const DRM_VMW_MKSSTAT_RESET: c_int = 30;
pub const DRM_VMW_MKSSTAT_ADD: c_int = 31;
pub const DRM_VMW_MKSSTAT_REMOVE: c_int = 32;
//
// DRM_VMW_GET_PARAM - get device information.
//
// DRM_VMW_PARAM_FIFO_OFFSET:
// Offset to use to map the first page of the FIFO read-only.
// The fifo is mapped using the mmap() system call on the drm device.
//
// DRM_VMW_PARAM_OVERLAY_IOCTL:
// Does the driver support the overlay ioctl.
//
// DRM_VMW_PARAM_SM4_1
// SM4_1 support is enabled.
//
// DRM_VMW_PARAM_SM5
// SM5 support is enabled.
//
// DRM_VMW_PARAM_GL43
// SM5.1+GL4.3 support is enabled.
//
// DRM_VMW_PARAM_DEVICE_ID
// PCI ID of the underlying SVGA device.
//
pub const DRM_VMW_PARAM_NUM_STREAMS: c_int = 0;
pub const DRM_VMW_PARAM_NUM_FREE_STREAMS: c_int = 1;
pub const DRM_VMW_PARAM_3D: c_int = 2;
pub const DRM_VMW_PARAM_HW_CAPS: c_int = 3;
pub const DRM_VMW_PARAM_FIFO_CAPS: c_int = 4;
pub const DRM_VMW_PARAM_MAX_FB_SIZE: c_int = 5;
pub const DRM_VMW_PARAM_FIFO_HW_VERSION: c_int = 6;
pub const DRM_VMW_PARAM_MAX_SURF_MEMORY: c_int = 7;
pub const DRM_VMW_PARAM_3D_CAPS_SIZE: c_int = 8;
pub const DRM_VMW_PARAM_MAX_MOB_MEMORY: c_int = 9;
pub const DRM_VMW_PARAM_MAX_MOB_SIZE: c_int = 10;
pub const DRM_VMW_PARAM_SCREEN_TARGET: c_int = 11;
pub const DRM_VMW_PARAM_DX: c_int = 12;
pub const DRM_VMW_PARAM_HW_CAPS2: c_int = 13;
pub const DRM_VMW_PARAM_SM4_1: c_int = 14;
pub const DRM_VMW_PARAM_SM5: c_int = 15;
pub const DRM_VMW_PARAM_GL43: c_int = 16;
pub const DRM_VMW_PARAM_DEVICE_ID: c_int = 17;
//
// enum drm_vmw_handle_type - handle type for ref ioctls
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_vmw_handle_type {
    DRM_VMW_HANDLE_LEGACY = 0,
    DRM_VMW_HANDLE_PRIME = 1
}

//
// struct drm_vmw_getparam_arg
//
// @value: Returned value. //Out
// @param: Parameter to query. //In.
//
// Argument to the DRM_VMW_GET_PARAM Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_getparam_arg {
    pub value: __u64,
    pub param: __u32,
    pub pad64: __u32,
}

//
// DRM_VMW_CREATE_CONTEXT - Create a host context.
//
// Allocates a device unique context id, and queues a create context command
// for the host. Does not wait for host completion.
//
// struct drm_vmw_context_arg
//
// @cid: Device unique context ID.
//
// Output argument to the DRM_VMW_CREATE_CONTEXT Ioctl.
// Input argument to the DRM_VMW_UNREF_CONTEXT Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_context_arg {
    pub cid: __s32,
    pub pad64: __u32,
}

//
// DRM_VMW_UNREF_CONTEXT - Create a host context.
//
// Frees a global context id, and queues a destroy host command for the host.
// Does not wait for host completion. The context ID can be used directly
// in the command stream and shows up as the same context ID on the host.
//
// DRM_VMW_CREATE_SURFACE - Create a host suface.
//
// Allocates a device unique surface id, and queues a create surface command
// for the host. Does not wait for host completion. The surface ID can be
// used directly in the command stream and shows up as the same surface
// ID on the host.
//
// struct drm_wmv_surface_create_req
//
// @flags: Surface flags as understood by the host.
// @format: Surface format as understood by the host.
// @mip_levels: Number of mip levels for each face.
// An unused face should have 0 encoded.
// @size_addr: Address of a user-space array of sruct drm_vmw_size
// cast to an __u64 for 32-64 bit compatibility.
// The size of the array should equal the total number of mipmap levels.
// @shareable: Boolean whether other clients (as identified by file descriptors)
// may reference this surface.
// @scanout: Boolean whether the surface is intended to be used as a
// scanout.
//
// Input data to the DRM_VMW_CREATE_SURFACE Ioctl.
// Output data from the DRM_VMW_REF_SURFACE Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_surface_create_req {
    pub flags: __u32,
    pub format: __u32,
    pub mip_levels: [__u32; DRM_VMW_MAX_SURFACE_FACES],
    pub size_addr: __u64,
    pub shareable: __s32,
    pub scanout: __s32,
}

//
// struct drm_wmv_surface_arg
//
// @sid: Surface id of created surface or surface to destroy or reference.
// @handle_type: Handle type for DRM_VMW_REF_SURFACE Ioctl.
//
// Output data from the DRM_VMW_CREATE_SURFACE Ioctl.
// Input argument to the DRM_VMW_UNREF_SURFACE Ioctl.
// Input argument to the DRM_VMW_REF_SURFACE Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_surface_arg {
    pub sid: __s32,
    pub handle_type: drm_vmw_handle_type,
}

//
// struct drm_vmw_size ioctl.
//
// @width - mip level width
// @height - mip level height
// @depth - mip level depth
//
// Description of a mip level.
// Input data to the DRM_WMW_CREATE_SURFACE Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_size {
    pub width: __u32,
    pub height: __u32,
    pub depth: __u32,
    pub pad64: __u32,
}

//
// union drm_vmw_surface_create_arg
//
// @rep: Output data as described above.
// @req: Input data as described above.
//
// Argument to the DRM_VMW_CREATE_SURFACE Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_vmw_surface_create_arg {
    pub rep: drm_vmw_surface_arg,
    pub req: drm_vmw_surface_create_req,
}

//
// DRM_VMW_REF_SURFACE - Reference a host surface.
//
// Puts a reference on a host surface with a give sid, as previously
// returned by the DRM_VMW_CREATE_SURFACE ioctl.
// A reference will make sure the surface isn't destroyed while we hold
// it and will allow the calling client to use the surface ID in the command
// stream.
//
// On successful return, the Ioctl returns the surface information given
// in the DRM_VMW_CREATE_SURFACE ioctl.
//
// union drm_vmw_surface_reference_arg
//
// @rep: Output data as described above.
// @req: Input data as described above.
//
// Argument to the DRM_VMW_REF_SURFACE Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_vmw_surface_reference_arg {
    pub rep: drm_vmw_surface_create_req,
    pub req: drm_vmw_surface_arg,
}

//
// DRM_VMW_UNREF_SURFACE - Unreference a host surface.
//
// Clear a reference previously put on a host surface.
// When all references are gone, including the one implicitly placed
// on creation,
// a destroy surface command will be queued for the host.
// Does not wait for completion.
//
// DRM_VMW_EXECBUF
//
// Submit a command buffer for execution on the host, and return a
// fence seqno that when signaled, indicates that the command buffer has
// executed.
//
// struct drm_vmw_execbuf_arg
//
// @commands: User-space address of a command buffer cast to an __u64.
// @command-size: Size in bytes of the command buffer.
// @throttle-us: Sleep until software is less than @throttle_us
// microseconds ahead of hardware. The driver may round this value
// to the nearest kernel tick.
// @fence_rep: User-space address of a struct drm_vmw_fence_rep cast to an
// __u64.
// @version: Allows expanding the execbuf ioctl parameters without breaking
// backwards compatibility, since user-space will always tell the kernel
// which version it uses.
// @flags: Execbuf flags.
// @imported_fence_fd:  FD for a fence imported from another device
//
// Argument to the DRM_VMW_EXECBUF Ioctl.
//
pub const DRM_VMW_EXECBUF_VERSION: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_execbuf_arg {
    pub commands: __u64,
    pub command_size: __u32,
    pub throttle_us: __u32,
    pub fence_rep: __u64,
    pub version: __u32,
    pub flags: __u32,
    pub context_handle: __u32,
    pub imported_fence_fd: __s32,
}

//
// struct drm_vmw_fence_rep
//
// @handle: Fence object handle for fence associated with a command submission.
// @mask: Fence flags relevant for this fence object.
// @seqno: Fence sequence number in fifo. A fence object with a lower
// seqno will signal the EXEC flag before a fence object with a higher
// seqno. This can be used by user-space to avoid kernel calls to determine
// whether a fence has signaled the EXEC flag. Note that @seqno will
// wrap at 32-bit.
// @passed_seqno: The highest seqno number processed by the hardware
// so far. This can be used to mark user-space fence objects as signaled, and
// to determine whether a fence seqno might be stale.
// @fd: FD associated with the fence, -1 if not exported
// @error: This member should've been set to -EFAULT on submission.
// The following actions should be take on completion:
// error == -EFAULT: Fence communication failed. The host is synchronized.
// Use the last fence id read from the FIFO fence register.
// error != 0 && error != -EFAULT:
// Fence submission failed. The host is synchronized. Use the fence_seq member.
// error == 0: All is OK, The host may not be synchronized.
// Use the fence_seq member.
//
// Input / Output data to the DRM_VMW_EXECBUF Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_fence_rep {
    pub handle: __u32,
    pub mask: __u32,
    pub seqno: __u32,
    pub passed_seqno: __u32,
    pub fd: __s32,
    pub error: __s32,
}

//
// DRM_VMW_ALLOC_BO
//
// Allocate a buffer object that is visible also to the host.
// NOTE: The buffer is
// identified by a handle and an offset, which are private to the guest, but
// useable in the command stream. The guest kernel may translate these
// and patch up the command stream accordingly. In the future, the offset may
// be zero at all times, or it may disappear from the interface before it is
// fixed.
//
// The buffer object may stay user-space mapped in the guest at all times,
// and is thus suitable for sub-allocation.
//
// Buffer objects are mapped using the mmap() syscall on the drm device.
//
// struct drm_vmw_alloc_bo_req
//
// @size: Required minimum size of the buffer.
//
// Input data to the DRM_VMW_ALLOC_BO Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_alloc_bo_req {
    pub size: __u32,
    pub pad64: __u32,
}

//
// struct drm_vmw_bo_rep
//
// @map_handle: Offset to use in the mmap() call used to map the buffer.
// @handle: Handle unique to this buffer. Used for unreferencing.
// @cur_gmr_id: GMR id to use in the command stream when this buffer is
// referenced. See not above.
// @cur_gmr_offset: Offset to use in the command stream when this buffer is
// referenced. See note above.
//
// Output data from the DRM_VMW_ALLOC_BO Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_bo_rep {
    pub map_handle: __u64,
    pub handle: __u32,
    pub cur_gmr_id: __u32,
    pub cur_gmr_offset: __u32,
    pub pad64: __u32,
}

//
// union drm_vmw_alloc_bo_arg
//
// @req: Input data as described above.
// @rep: Output data as described above.
//
// Argument to the DRM_VMW_ALLOC_BO Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_vmw_alloc_bo_arg {
    pub req: drm_vmw_alloc_bo_req,
    pub rep: drm_vmw_bo_rep,
}

//
// DRM_VMW_CONTROL_STREAM - Control overlays, aka streams.
//
// This IOCTL controls the overlay units of the svga device.
// The SVGA overlay units does not work like regular hardware units in
// that they do not automaticaly read back the contents of the given dma
// buffer. But instead only read back for each call to this ioctl, and
// at any point between this call being made and a following call that
// either changes the buffer or disables the stream.
//
// struct drm_vmw_rect
//
// Defines a rectangle. Used in the overlay ioctl to define
// source and destination rectangle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_rect {
    pub x: __s32,
    pub y: __s32,
    pub w: __u32,
    pub h: __u32,
}

//
// struct drm_vmw_control_stream_arg
//
// @stream_id: Stearm to control
// @enabled: If false all following arguments are ignored.
// @handle: Handle to buffer for getting data from.
// @format: Format of the overlay as understood by the host.
// @width: Width of the overlay.
// @height: Height of the overlay.
// @size: Size of the overlay in bytes.
// @pitch: Array of pitches, the two last are only used for YUV12 formats.
// @offset: Offset from start of dma buffer to overlay.
// @src: Source rect, must be within the defined area above.
// @dst: Destination rect, x and y may be negative.
//
// Argument to the DRM_VMW_CONTROL_STREAM Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_control_stream_arg {
    pub stream_id: __u32,
    pub enabled: __u32,
    pub flags: __u32,
    pub color_key: __u32,
    pub handle: __u32,
    pub offset: __u32,
    pub format: __s32,
    pub size: __u32,
    pub width: __u32,
    pub height: __u32,
    pub pitch: [__u32; 3],
    pub pad64: __u32,
    pub src: drm_vmw_rect,
    pub dst: drm_vmw_rect,
}

//
// DRM_VMW_CURSOR_BYPASS - Give extra information about cursor bypass.
//

//
// struct drm_vmw_cursor_bypass_arg
//
// @flags: Flags.
// @crtc_id: Crtc id, only used if DMR_CURSOR_BYPASS_ALL isn't passed.
// @xpos: X position of cursor.
// @ypos: Y position of cursor.
// @xhot: X hotspot.
// @yhot: Y hotspot.
//
// Argument to the DRM_VMW_CURSOR_BYPASS Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_cursor_bypass_arg {
    pub flags: __u32,
    pub crtc_id: __u32,
    pub xpos: __s32,
    pub ypos: __s32,
    pub xhot: __s32,
    pub yhot: __s32,
}

//
// DRM_VMW_CLAIM_STREAM - Claim a single stream.
//
// struct drm_vmw_context_arg
//
// @stream_id: Device unique context ID.
//
// Output argument to the DRM_VMW_CREATE_CONTEXT Ioctl.
// Input argument to the DRM_VMW_UNREF_CONTEXT Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_stream_arg {
    pub stream_id: __u32,
    pub pad64: __u32,
}

//
// DRM_VMW_UNREF_STREAM - Unclaim a stream.
//
// Return a single stream that was claimed by this process. Also makes
// sure that the stream has been stopped.
//
// DRM_VMW_GET_3D_CAP
//
// Read 3D capabilities from the FIFO
//
// struct drm_vmw_get_3d_cap_arg
//
// @buffer: Pointer to a buffer for capability data, cast to an __u64
// @size: Max size to copy
//
// Input argument to the DRM_VMW_GET_3D_CAP_IOCTL
// ioctls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_get_3d_cap_arg {
    pub buffer: __u64,
    pub max_size: __u32,
    pub pad64: __u32,
}

//
// DRM_VMW_FENCE_WAIT
//
// Waits for a fence object to signal. The wait is interruptible, so that
// signals may be delivered during the interrupt. The wait may timeout,
// in which case the calls returns -EBUSY. If the wait is restarted,
// that is restarting without resetting @cookie_valid to zero,
// the timeout is computed from the first call.
//
// The flags argument to the DRM_VMW_FENCE_WAIT ioctl indicates what to wait
// on:
// DRM_VMW_FENCE_FLAG_EXEC: All commands ahead of the fence in the command
// stream
// have executed.
// DRM_VMW_FENCE_FLAG_QUERY: All query results resulting from query finish
// commands
// in the buffer given to the EXECBUF ioctl returning the fence object handle
// are available to user-space.
//
// DRM_VMW_WAIT_OPTION_UNREF: If this wait option is given, and the
// fenc wait ioctl returns 0, the fence object has been unreferenced after
// the wait.
//

//
// struct drm_vmw_fence_wait_arg
//
// @handle: Fence object handle as returned by the DRM_VMW_EXECBUF ioctl.
// @cookie_valid: Must be reset to 0 on first call. Left alone on restart.
// @kernel_cookie: Set to 0 on first call. Left alone on restart.
// @timeout_us: Wait timeout in microseconds. 0 for indefinite timeout.
// @lazy: Set to 1 if timing is not critical. Allow more than a kernel tick
// before returning.
// @flags: Fence flags to wait on.
// @wait_options: Options that control the behaviour of the wait ioctl.
//
// Input argument to the DRM_VMW_FENCE_WAIT ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_fence_wait_arg {
    pub handle: __u32,
    pub cookie_valid: __s32,
    pub kernel_cookie: __u64,
    pub timeout_us: __u64,
    pub lazy: __s32,
    pub flags: __s32,
    pub wait_options: __s32,
    pub pad64: __s32,
}

//
// DRM_VMW_FENCE_SIGNALED
//
// Checks if a fence object is signaled..
//
// struct drm_vmw_fence_signaled_arg
//
// @handle: Fence object handle as returned by the DRM_VMW_EXECBUF ioctl.
// @flags: Fence object flags input to DRM_VMW_FENCE_SIGNALED ioctl
// @signaled: Out: Flags signaled.
// @sequence: Out: Highest sequence passed so far. Can be used to signal the
// EXEC flag of user-space fence objects.
//
// Input/Output argument to the DRM_VMW_FENCE_SIGNALED and DRM_VMW_FENCE_UNREF
// ioctls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_fence_signaled_arg {
    pub handle: __u32,
    pub flags: __u32,
    pub signaled: __s32,
    pub passed_seqno: __u32,
    pub signaled_flags: __u32,
    pub pad64: __u32,
}

//
// DRM_VMW_FENCE_UNREF
//
// Unreferences a fence object, and causes it to be destroyed if there are no
// other references to it.
//
// struct drm_vmw_fence_arg
//
// @handle: Fence object handle as returned by the DRM_VMW_EXECBUF ioctl.
//
// Input/Output argument to the DRM_VMW_FENCE_UNREF ioctl..
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_fence_arg {
    pub handle: __u32,
    pub pad64: __u32,
}

//
// DRM_VMW_FENCE_EVENT
//
// Queues an event on a fence to be delivered on the drm character device
// when the fence has signaled the DRM_VMW_FENCE_FLAG_EXEC flag.
// Optionally the approximate time when the fence signaled is
// given by the event.
//
// The event type
//
pub const DRM_VMW_EVENT_FENCE_SIGNALED: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_event_fence {
    pub base: drm_event,
    pub user_data: __u64,
    pub tv_sec: __u32,
    pub tv_usec: __u32,
}

//
// Flags that may be given to the command.
//
// Request fence signaled time on the event.

//
// struct drm_vmw_fence_event_arg
//
// @fence_rep: Pointer to fence_rep structure cast to __u64 or 0 if
// the fence is not supposed to be referenced by user-space.
// @user_info: Info to be delivered with the event.
// @handle: Attach the event to this fence only.
// @flags: A set of flags as defined above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_fence_event_arg {
    pub fence_rep: __u64,
    pub user_data: __u64,
    pub handle: __u32,
    pub flags: __u32,
}

//
// DRM_VMW_PRESENT
//
// Executes an SVGA present on a given fb for a given surface. The surface
// is placed on the framebuffer. Cliprects are given relative to the given
// point (the point disignated by dest_{x|y}).
//
// struct drm_vmw_present_arg
// @fb_id: framebuffer id to present / read back from.
// @sid: Surface id to present from.
// @dest_x: X placement coordinate for surface.
// @dest_y: Y placement coordinate for surface.
// @clips_ptr: Pointer to an array of clip rects cast to an __u64.
// @num_clips: Number of cliprects given relative to the framebuffer origin,
// in the same coordinate space as the frame buffer.
// @pad64: Unused 64-bit padding.
//
// Input argument to the DRM_VMW_PRESENT ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_present_arg {
    pub fb_id: __u32,
    pub sid: __u32,
    pub dest_x: __s32,
    pub dest_y: __s32,
    pub clips_ptr: __u64,
    pub num_clips: __u32,
    pub pad64: __u32,
}

//
// DRM_VMW_PRESENT_READBACK
//
// Executes an SVGA present readback from a given fb to the dma buffer
// currently bound as the fb. If there is no dma buffer bound to the fb,
// an error will be returned.
//
// struct drm_vmw_present_arg
// @fb_id: fb_id to present / read back from.
// @num_clips: Number of cliprects.
// @clips_ptr: Pointer to an array of clip rects cast to an __u64.
// @fence_rep: Pointer to a struct drm_vmw_fence_rep, cast to an __u64.
// If this member is NULL, then the ioctl should not return a fence.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_present_readback_arg {
    pub fb_id: __u32,
    pub num_clips: __u32,
    pub clips_ptr: __u64,
    pub fence_rep: __u64,
}

//
// DRM_VMW_UPDATE_LAYOUT - Update layout
//
// Updates the preferred modes and connection status for connectors. The
// command consists of one drm_vmw_update_layout_arg pointing to an array
// of num_outputs drm_vmw_rect's.
//
// struct drm_vmw_update_layout_arg
//
// @num_outputs: number of active connectors
// @rects: pointer to array of drm_vmw_rect cast to an __u64
//
// Input argument to the DRM_VMW_UPDATE_LAYOUT Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_update_layout_arg {
    pub num_outputs: __u32,
    pub pad64: __u32,
    pub rects: __u64,
}

//
// DRM_VMW_CREATE_SHADER - Create shader
//
// Creates a shader and optionally binds it to a dma buffer containing
// the shader byte-code.
//
// enum drm_vmw_shader_type - Shader types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_vmw_shader_type {
    drm_vmw_shader_type_vs = 0,
    drm_vmw_shader_type_ps,
}

//
// struct drm_vmw_shader_create_arg
//
// @shader_type: Shader type of the shader to create.
// @size: Size of the byte-code in bytes.
// where the shader byte-code starts
// @buffer_handle: Buffer handle identifying the buffer containing the
// shader byte-code
// @shader_handle: On successful completion contains a handle that
// can be used to subsequently identify the shader.
// @offset: Offset in bytes into the buffer given by @buffer_handle,
//
// Input / Output argument to the DRM_VMW_CREATE_SHADER Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_shader_create_arg {
    pub shader_type: drm_vmw_shader_type,
    pub size: __u32,
    pub buffer_handle: __u32,
    pub shader_handle: __u32,
    pub offset: __u64,
}

//
// DRM_VMW_UNREF_SHADER - Unreferences a shader
//
// Destroys a user-space reference to a shader, optionally destroying
// it.
//
// struct drm_vmw_shader_arg
//
// @handle: Handle identifying the shader to destroy.
//
// Input argument to the DRM_VMW_UNREF_SHADER ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_shader_arg {
    pub handle: __u32,
    pub pad64: __u32,
}

//
// DRM_VMW_GB_SURFACE_CREATE - Create a host guest-backed surface.
//
// Allocates a surface handle and queues a create surface command
// for the host on the first use of the surface. The surface ID can
// be used as the surface ID in commands referencing the surface.
//
// enum drm_vmw_surface_flags
//
// @drm_vmw_surface_flag_shareable:     Deprecated - all userspace surfaces are
// shareable.
// @drm_vmw_surface_flag_scanout:       Whether the surface is a scanout
// surface.
// @drm_vmw_surface_flag_create_buffer: Create a backup buffer if none is
// given.
// @drm_vmw_surface_flag_coherent:      Back surface with coherent memory.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_vmw_surface_flags {
    drm_vmw_surface_flag_shareable = (1 << 0),
    drm_vmw_surface_flag_scanout = (1 << 1),
    drm_vmw_surface_flag_create_buffer = (1 << 2),
    drm_vmw_surface_flag_coherent = (1 << 3),
}

//
// struct drm_vmw_gb_surface_create_req
//
// @svga3d_flags:     SVGA3d surface flags for the device.
// @format:           SVGA3d format.
// @mip_level:        Number of mip levels for all faces.
// @drm_surface_flags Flags as described above.
// @multisample_count Future use. Set to 0.
// @autogen_filter    Future use. Set to 0.
// @buffer_handle     Buffer handle of backup buffer. SVGA3D_INVALID_ID
// if none.
// @base_size         Size of the base mip level for all faces.
// @array_size        Must be zero for non-DX hardware, and if non-zero
// svga3d_flags must have proper bind flags setup.
//
// Input argument to the  DRM_VMW_GB_SURFACE_CREATE Ioctl.
// Part of output argument for the DRM_VMW_GB_SURFACE_REF Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_gb_surface_create_req {
    pub svga3d_flags: __u32,
    pub format: __u32,
    pub mip_levels: __u32,
    pub drm_surface_flags: drm_vmw_surface_flags,
    pub multisample_count: __u32,
    pub autogen_filter: __u32,
    pub buffer_handle: __u32,
    pub array_size: __u32,
    pub base_size: drm_vmw_size,
}

//
// struct drm_vmw_gb_surface_create_rep
//
// @handle:            Surface handle.
// @backup_size:       Size of backup buffers for this surface.
// @buffer_handle:     Handle of backup buffer. SVGA3D_INVALID_ID if none.
// @buffer_size:       Actual size of the buffer identified by
// @buffer_handle
// @buffer_map_handle: Offset into device address space for the buffer
// identified by @buffer_handle.
//
// Part of output argument for the DRM_VMW_GB_SURFACE_REF ioctl.
// Output argument for the DRM_VMW_GB_SURFACE_CREATE ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_gb_surface_create_rep {
    pub handle: __u32,
    pub backup_size: __u32,
    pub buffer_handle: __u32,
    pub buffer_size: __u32,
    pub buffer_map_handle: __u64,
}

//
// union drm_vmw_gb_surface_create_arg
//
// @req: Input argument as described above.
// @rep: Output argument as described above.
//
// Argument to the DRM_VMW_GB_SURFACE_CREATE ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_vmw_gb_surface_create_arg {
    pub rep: drm_vmw_gb_surface_create_rep,
    pub req: drm_vmw_gb_surface_create_req,
}

//
// DRM_VMW_GB_SURFACE_REF - Reference a host surface.
//
// Puts a reference on a host surface with a given handle, as previously
// returned by the DRM_VMW_GB_SURFACE_CREATE ioctl.
// A reference will make sure the surface isn't destroyed while we hold
// it and will allow the calling client to use the surface handle in
// the command stream.
//
// On successful return, the Ioctl returns the surface information given
// to and returned from the DRM_VMW_GB_SURFACE_CREATE ioctl.
//
// struct drm_vmw_gb_surface_reference_arg
//
// @creq: The data used as input when the surface was created, as described
// above at "struct drm_vmw_gb_surface_create_req"
// @crep: Additional data output when the surface was created, as described
// above at "struct drm_vmw_gb_surface_create_rep"
//
// Output Argument to the DRM_VMW_GB_SURFACE_REF ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_gb_surface_ref_rep {
    pub creq: drm_vmw_gb_surface_create_req,
    pub crep: drm_vmw_gb_surface_create_rep,
}

//
// union drm_vmw_gb_surface_reference_arg
//
// @req: Input data as described above at "struct drm_vmw_surface_arg"
// @rep: Output data as described above at "struct drm_vmw_gb_surface_ref_rep"
//
// Argument to the DRM_VMW_GB_SURFACE_REF Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_vmw_gb_surface_reference_arg {
    pub rep: drm_vmw_gb_surface_ref_rep,
    pub req: drm_vmw_surface_arg,
}

//
// DRM_VMW_SYNCCPU - Sync a DMA buffer / MOB for CPU access.
//
// Idles any previously submitted GPU operations on the buffer and
// by default blocks command submissions that reference the buffer.
// If the file descriptor used to grab a blocking CPU sync is closed, the
// cpu sync is released.
// The flags argument indicates how the grab / release operation should be
// performed:
//
// enum drm_vmw_synccpu_flags - Synccpu flags:
//
// @drm_vmw_synccpu_read: Sync for read. If sync is done for read only, it's a
// hint to the kernel to allow command submissions that references the buffer
// for read-only.
// @drm_vmw_synccpu_write: Sync for write. Block all command submissions
// referencing this buffer.
// @drm_vmw_synccpu_dontblock: Dont wait for GPU idle, but rather return
// -EBUSY should the buffer be busy.
// @drm_vmw_synccpu_allow_cs: Allow command submission that touches the buffer
// while the buffer is synced for CPU. This is similar to the GEM bo idle
// behavior.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_vmw_synccpu_flags {
    drm_vmw_synccpu_read = (1 << 0),
    drm_vmw_synccpu_write = (1 << 1),
    drm_vmw_synccpu_dontblock = (1 << 2),
    drm_vmw_synccpu_allow_cs = (1 << 3)
}

//
// enum drm_vmw_synccpu_op - Synccpu operations:
//
// @drm_vmw_synccpu_grab:    Grab the buffer for CPU operations
// @drm_vmw_synccpu_release: Release a previous grab.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_vmw_synccpu_op {
    drm_vmw_synccpu_grab,
    drm_vmw_synccpu_release
}

//
// struct drm_vmw_synccpu_arg
//
// @op:			     The synccpu operation as described above.
// @handle:		     Handle identifying the buffer object.
// @flags:		     Flags as described above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_synccpu_arg {
    pub op: drm_vmw_synccpu_op,
    pub flags: drm_vmw_synccpu_flags,
    pub handle: __u32,
    pub pad64: __u32,
}

//
// DRM_VMW_CREATE_EXTENDED_CONTEXT - Create a host context.
//
// Allocates a device unique context id, and queues a create context command
// for the host. Does not wait for host completion.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_vmw_extended_context {
    drm_vmw_context_legacy,
    drm_vmw_context_dx
}

//
// union drm_vmw_extended_context_arg
//
// @req: Context type.
// @rep: Context identifier.
//
// Argument to the DRM_VMW_CREATE_EXTENDED_CONTEXT Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_vmw_extended_context_arg {
    pub req: drm_vmw_extended_context,
    pub rep: drm_vmw_context_arg,
}

//
// DRM_VMW_HANDLE_CLOSE - Close a user-space handle and release its
// underlying resource.
//
// Note that this ioctl is overlaid on the deprecated DRM_VMW_UNREF_DMABUF
// Ioctl.
//
// struct drm_vmw_handle_close_arg
//
// @handle: Handle to close.
//
// Argument to the DRM_VMW_HANDLE_CLOSE Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_handle_close_arg {
    pub handle: __u32,
    pub pad64: __u32,
}

//
// DRM_VMW_GB_SURFACE_CREATE_EXT - Create a host guest-backed surface.
//
// Allocates a surface handle and queues a create surface command
// for the host on the first use of the surface. The surface ID can
// be used as the surface ID in commands referencing the surface.
//
// This new command extends DRM_VMW_GB_SURFACE_CREATE by adding version
// parameter and 64 bit svga flag.
//
// enum drm_vmw_surface_version
//
// @drm_vmw_surface_gb_v1: Corresponds to current gb surface format with
// svga3d surface flags split into 2, upper half and lower half.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_vmw_surface_version {
    drm_vmw_gb_surface_v1,
}

//
// struct drm_vmw_gb_surface_create_ext_req
//
// @base: Surface create parameters.
// @version: Version of surface create ioctl.
// @svga3d_flags_upper_32_bits: Upper 32 bits of svga3d flags.
// @multisample_pattern: Multisampling pattern when msaa is supported.
// @quality_level: Precision settings for each sample.
// @buffer_byte_stride: Buffer byte stride.
// @must_be_zero: Reserved for future usage.
//
// Input argument to the  DRM_VMW_GB_SURFACE_CREATE_EXT Ioctl.
// Part of output argument for the DRM_VMW_GB_SURFACE_REF_EXT Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_gb_surface_create_ext_req {
    pub base: drm_vmw_gb_surface_create_req,
    pub version: drm_vmw_surface_version,
    pub svga3d_flags_upper_32_bits: __u32,
    pub multisample_pattern: __u32,
    pub quality_level: __u32,
    pub buffer_byte_stride: __u32,
    pub must_be_zero: __u32,
}

//
// union drm_vmw_gb_surface_create_ext_arg
//
// @req: Input argument as described above.
// @rep: Output argument as described above.
//
// Argument to the DRM_VMW_GB_SURFACE_CREATE_EXT ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_vmw_gb_surface_create_ext_arg {
    pub rep: drm_vmw_gb_surface_create_rep,
    pub req: drm_vmw_gb_surface_create_ext_req,
}

//
// DRM_VMW_GB_SURFACE_REF_EXT - Reference a host surface.
//
// Puts a reference on a host surface with a given handle, as previously
// returned by the DRM_VMW_GB_SURFACE_CREATE_EXT ioctl.
// A reference will make sure the surface isn't destroyed while we hold
// it and will allow the calling client to use the surface handle in
// the command stream.
//
// On successful return, the Ioctl returns the surface information given
// to and returned from the DRM_VMW_GB_SURFACE_CREATE_EXT ioctl.
//
// struct drm_vmw_gb_surface_ref_ext_rep
//
// @creq: The data used as input when the surface was created, as described
// above at "struct drm_vmw_gb_surface_create_ext_req"
// @crep: Additional data output when the surface was created, as described
// above at "struct drm_vmw_gb_surface_create_rep"
//
// Output Argument to the DRM_VMW_GB_SURFACE_REF_EXT ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_gb_surface_ref_ext_rep {
    pub creq: drm_vmw_gb_surface_create_ext_req,
    pub crep: drm_vmw_gb_surface_create_rep,
}

//
// union drm_vmw_gb_surface_reference_ext_arg
//
// @req: Input data as described above at "struct drm_vmw_surface_arg"
// @rep: Output data as described above at
// "struct drm_vmw_gb_surface_ref_ext_rep"
//
// Argument to the DRM_VMW_GB_SURFACE_REF Ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_vmw_gb_surface_reference_ext_arg {
    pub rep: drm_vmw_gb_surface_ref_ext_rep,
    pub req: drm_vmw_surface_arg,
}

//
// struct drm_vmw_msg_arg
//
// @send: Pointer to user-space msg string (null terminated).
// @receive: Pointer to user-space receive buffer.
// @send_only: Boolean whether this is only sending or receiving too.
//
// Argument to the DRM_VMW_MSG ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_msg_arg {
    pub send: __u64,
    pub receive: __u64,
    pub send_only: __s32,
    pub receive_len: __u32,
}

//
// struct drm_vmw_mksstat_add_arg
//
// @stat: Pointer to user-space stat-counters array, page-aligned.
// @info: Pointer to user-space counter-infos array, page-aligned.
// @strs: Pointer to user-space stat strings, page-aligned.
// @stat_len: Length in bytes of stat-counters array.
// @info_len: Length in bytes of counter-infos array.
// @strs_len: Length in bytes of the stat strings, terminators included.
// @description: Pointer to instance descriptor string; will be truncated
// to MKS_GUEST_STAT_INSTANCE_DESC_LENGTH chars.
// @id: Output identifier of the produced record; -1 if error.
//
// Argument to the DRM_VMW_MKSSTAT_ADD ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_mksstat_add_arg {
    pub stat: __u64,
    pub info: __u64,
    pub strs: __u64,
    pub stat_len: __u64,
    pub info_len: __u64,
    pub strs_len: __u64,
    pub description: __u64,
    pub id: __u64,
}

//
// struct drm_vmw_mksstat_remove_arg
//
// @id: Identifier of the record being disposed, originally obtained through
// DRM_VMW_MKSSTAT_ADD ioctl.
//
// Argument to the DRM_VMW_MKSSTAT_REMOVE ioctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vmw_mksstat_remove_arg {
    pub id: __u64,
}

