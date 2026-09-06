//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/drm_mode.h
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
// Copyright (c) 2007 Dave Airlie <airlied@linux.ie>
// Copyright (c) 2007 Jakob Bornecrantz <wallbraker@gmail.com>
// Copyright (c) 2008 Red Hat Inc.
// Copyright (c) 2007-2008 Tungsten Graphics, Inc., Cedar Park, TX., USA
// Copyright (c) 2007-2008 Intel Corporation
//

//
// DOC: overview
//
// DRM exposes many UAPI and structure definitions to have a consistent
// and standardized interface with users.
// Userspace can refer to these structure definitions and UAPI formats
// to communicate to drivers.
//
pub const DRM_CONNECTOR_NAME_LEN: c_int = 32;
pub const DRM_DISPLAY_MODE_LEN: c_int = 32;
pub const DRM_PROP_NAME_LEN: c_int = 32;

// Video mode flags
// bit compatible with the xrandr RR_ definitions (bits 0-13)
//
// ABI warning: Existing userspace really expects
// the mode flags to match the xrandr definitions. Any
// changes that don't match the xrandr definitions will
// likely need a new client cap or some other mechanism
// to avoid breaking existing userspace. This includes
// allocating new flags in the previously unused bits!
//

//
// When adding a new stereo mode don't forget to adjust DRM_MODE_FLAGS_3D_MAX
// (define not exposed to user space).
//

// Picture aspect ratio options
pub const DRM_MODE_PICTURE_ASPECT_NONE: c_int = 0;
pub const DRM_MODE_PICTURE_ASPECT_4_3: c_int = 1;
pub const DRM_MODE_PICTURE_ASPECT_16_9: c_int = 2;
pub const DRM_MODE_PICTURE_ASPECT_64_27: c_int = 3;
pub const DRM_MODE_PICTURE_ASPECT_256_135: c_int = 4;
// Content type options
pub const DRM_MODE_CONTENT_TYPE_NO_DATA: c_int = 0;
pub const DRM_MODE_CONTENT_TYPE_GRAPHICS: c_int = 1;
pub const DRM_MODE_CONTENT_TYPE_PHOTO: c_int = 2;
pub const DRM_MODE_CONTENT_TYPE_CINEMA: c_int = 3;
pub const DRM_MODE_CONTENT_TYPE_GAME: c_int = 4;
// Aspect ratio flag bitmask (4 bits 22:19)

// DPMS flags
// bit compatible with the xorg definitions.
pub const DRM_MODE_DPMS_ON: c_int = 0;
pub const DRM_MODE_DPMS_STANDBY: c_int = 1;
pub const DRM_MODE_DPMS_SUSPEND: c_int = 2;
pub const DRM_MODE_DPMS_OFF: c_int = 3;
// Scaling mode options

// Dithering mode options
pub const DRM_MODE_DITHERING_OFF: c_int = 0;
pub const DRM_MODE_DITHERING_ON: c_int = 1;
pub const DRM_MODE_DITHERING_AUTO: c_int = 2;
// Dirty info options
pub const DRM_MODE_DIRTY_OFF: c_int = 0;
pub const DRM_MODE_DIRTY_ON: c_int = 1;
pub const DRM_MODE_DIRTY_ANNOTATE: c_int = 2;
// Link Status options
pub const DRM_MODE_LINK_STATUS_GOOD: c_int = 0;
pub const DRM_MODE_LINK_STATUS_BAD: c_int = 1;
// Panel type property
pub const DRM_MODE_PANEL_TYPE_UNKNOWN: c_int = 0;
pub const DRM_MODE_PANEL_TYPE_OLED: c_int = 1;
pub const DRM_MODE_PANEL_TYPE_LCD: c_int = 2;
//
// DRM_MODE_ROTATE_<degrees>
//
// Signals that a drm plane is been rotated <degrees> degrees in counter
// clockwise direction.
//
// This define is provided as a convenience, looking up the property id
// using the name->prop id lookup is the preferred method.
//

//
// DRM_MODE_ROTATE_MASK
//
// Bitmask used to look for drm plane rotations.
//

//
// DRM_MODE_REFLECT_<axis>
//
// Signals that the contents of a drm plane is reflected along the <axis> axis,
// in the same way as mirroring.
// See kerneldoc chapter "Plane Composition Properties" for more details.
//
// This define is provided as a convenience, looking up the property id
// using the name->prop id lookup is the preferred method.
//

//
// DRM_MODE_REFLECT_MASK
//
// Bitmask used to look for drm plane reflections.
//

// Content Protection Flags
pub const DRM_MODE_CONTENT_PROTECTION_UNDESIRED: c_int = 0;
pub const DRM_MODE_CONTENT_PROTECTION_DESIRED: c_int = 1;
pub const DRM_MODE_CONTENT_PROTECTION_ENABLED: c_int = 2;
//
// struct drm_mode_modeinfo - Display mode information.
// @clock: pixel clock in kHz
// @hdisplay: horizontal display size
// @hsync_start: horizontal sync start
// @hsync_end: horizontal sync end
// @htotal: horizontal total size
// @hskew: horizontal skew
// @vdisplay: vertical display size
// @vsync_start: vertical sync start
// @vsync_end: vertical sync end
// @vtotal: vertical total size
// @vscan: vertical scan
// @vrefresh: approximate vertical refresh rate in Hz
// @flags: bitmask of misc. flags, see DRM_MODE_FLAG_* defines
// @type: bitmask of type flags, see DRM_MODE_TYPE_* defines
// @name: string describing the mode resolution
//
// This is the user-space API display mode information structure. For the
// kernel version see struct drm_display_mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_modeinfo {
    pub clock: __u32,
    pub hdisplay: __u16,
    pub hsync_start: __u16,
    pub hsync_end: __u16,
    pub htotal: __u16,
    pub hskew: __u16,
    pub vdisplay: __u16,
    pub vsync_start: __u16,
    pub vsync_end: __u16,
    pub vtotal: __u16,
    pub vscan: __u16,
    pub vrefresh: __u32,
    pub flags: __u32,
    pub type: __u32,
    pub name: [c_char; DRM_DISPLAY_MODE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_card_res {
    pub fb_id_ptr: __u64,
    pub crtc_id_ptr: __u64,
    pub connector_id_ptr: __u64,
    pub encoder_id_ptr: __u64,
    pub count_fbs: __u32,
    pub count_crtcs: __u32,
    pub count_connectors: __u32,
    pub count_encoders: __u32,
    pub min_width: __u32,
    pub max_width: __u32,
    pub min_height: __u32,
    pub max_height: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_crtc {
    pub set_connectors_ptr: __u64,
    pub count_connectors: __u32,
    pub /: *mut *mut *mut __u32 crtc_id; /< Id,
    pub /: *mut *mut *mut __u32 fb_id; /< Id of framebuffer,
    pub /: *mut *mut *mut __u32 x; /< x Position on the framebuffer,
    pub /: *mut *mut *mut __u32 y; /< y Position on the framebuffer,
    pub gamma_size: __u32,
    pub mode_valid: __u32,
    pub mode: drm_mode_modeinfo,
}

// Planes blend with or override other bits on the CRTC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_set_plane {
    pub plane_id: __u32,
    pub crtc_id: __u32,
    pub /: *mut *mut __u32 fb_id; / fb object contains surface format type,
    pub /: *mut *mut __u32 flags; / see above flags,
// Signed dest location allows it to be partially off screen
    pub crtc_x: __s32,
    pub crtc_y: __s32,
    pub crtc_w: __u32,
    pub crtc_h: __u32,
// Source values are 16.16 fixed point
    pub src_x: __u32,
    pub src_y: __u32,
    pub src_h: __u32,
    pub src_w: __u32,
}

//
// struct drm_mode_get_plane - Get plane metadata.
//
// Userspace can perform a GETPLANE ioctl to retrieve information about a
// plane.
//
// To retrieve the number of formats supported, set @count_format_types to zero
// and call the ioctl. @count_format_types will be updated with the value.
//
// To retrieve these formats, allocate an array with the memory needed to store
// @count_format_types formats. Point @format_type_ptr to this array and call
// the ioctl again (with @count_format_types still set to the value returned in
// the first ioctl call).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_plane {
//
// @plane_id: Object ID of the plane whose information should be
// retrieved. Set by caller.
//
    pub plane_id: __u32,
// @crtc_id: Object ID of the current CRTC.
    pub crtc_id: __u32,
// @fb_id: Object ID of the current fb.
    pub fb_id: __u32,
//
// @possible_crtcs: Bitmask of CRTC's compatible with the plane. CRTC's
// are created and they receive an index, which corresponds to their
// position in the bitmask. Bit N corresponds to
// :ref:`CRTC index<crtc_index>` N.
//
    pub possible_crtcs: __u32,
// @gamma_size: Never used.
    pub gamma_size: __u32,
// @count_format_types: Number of formats.
    pub count_format_types: __u32,
//
// @format_type_ptr: Pointer to ``__u32`` array of formats that are
// supported by the plane. These formats do not require modifiers.
//
    pub format_type_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_plane_res {
    pub plane_id_ptr: __u64,
    pub count_planes: __u32,
}

pub const DRM_MODE_ENCODER_NONE: c_int = 0;
pub const DRM_MODE_ENCODER_DAC: c_int = 1;
pub const DRM_MODE_ENCODER_TMDS: c_int = 2;
pub const DRM_MODE_ENCODER_LVDS: c_int = 3;
pub const DRM_MODE_ENCODER_TVDAC: c_int = 4;
pub const DRM_MODE_ENCODER_VIRTUAL: c_int = 5;
pub const DRM_MODE_ENCODER_DSI: c_int = 6;
pub const DRM_MODE_ENCODER_DPMST: c_int = 7;
pub const DRM_MODE_ENCODER_DPI: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_encoder {
    pub encoder_id: __u32,
    pub encoder_type: __u32,
    pub /: *mut *mut *mut __u32 crtc_id; /< Id of crtc,
    pub possible_crtcs: __u32,
    pub possible_clones: __u32,
}

// This is for connectors with multiple signal types.
// Try to match DRM_MODE_CONNECTOR_X as closely as possible.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_mode_subconnector {
    DRM_MODE_SUBCONNECTOR_Automatic   = 0,  /* DVI-I, TV     */
    DRM_MODE_SUBCONNECTOR_Unknown     = 0,  /* DVI-I, TV, DP */
    DRM_MODE_SUBCONNECTOR_VGA	  = 1,  /*            DP */
    DRM_MODE_SUBCONNECTOR_DVID	  = 3,  /* DVI-I      DP */
    DRM_MODE_SUBCONNECTOR_DVIA	  = 4,  /* DVI-I         */
    DRM_MODE_SUBCONNECTOR_Composite   = 5,  /*        TV     */
    DRM_MODE_SUBCONNECTOR_SVIDEO	  = 6,  /*        TV     */
    DRM_MODE_SUBCONNECTOR_Component   = 8,  /*        TV     */
    DRM_MODE_SUBCONNECTOR_SCART	  = 9,  /*        TV     */
    DRM_MODE_SUBCONNECTOR_DisplayPort = 10, /*            DP */
    DRM_MODE_SUBCONNECTOR_HDMIA       = 11, /*            DP */
    DRM_MODE_SUBCONNECTOR_Native      = 15, /*            DP */
    DRM_MODE_SUBCONNECTOR_Wireless    = 18, /*            DP */
}

pub const DRM_MODE_CONNECTOR_Unknown: c_int = 0;
pub const DRM_MODE_CONNECTOR_VGA: c_int = 1;
pub const DRM_MODE_CONNECTOR_DVII: c_int = 2;
pub const DRM_MODE_CONNECTOR_DVID: c_int = 3;
pub const DRM_MODE_CONNECTOR_DVIA: c_int = 4;
pub const DRM_MODE_CONNECTOR_Composite: c_int = 5;
pub const DRM_MODE_CONNECTOR_SVIDEO: c_int = 6;
pub const DRM_MODE_CONNECTOR_LVDS: c_int = 7;
pub const DRM_MODE_CONNECTOR_Component: c_int = 8;
pub const DRM_MODE_CONNECTOR_9PinDIN: c_int = 9;
pub const DRM_MODE_CONNECTOR_DisplayPort: c_int = 10;
pub const DRM_MODE_CONNECTOR_HDMIA: c_int = 11;
pub const DRM_MODE_CONNECTOR_HDMIB: c_int = 12;
pub const DRM_MODE_CONNECTOR_TV: c_int = 13;
pub const DRM_MODE_CONNECTOR_eDP: c_int = 14;
pub const DRM_MODE_CONNECTOR_VIRTUAL: c_int = 15;
pub const DRM_MODE_CONNECTOR_DSI: c_int = 16;
pub const DRM_MODE_CONNECTOR_DPI: c_int = 17;
pub const DRM_MODE_CONNECTOR_WRITEBACK: c_int = 18;
pub const DRM_MODE_CONNECTOR_SPI: c_int = 19;
pub const DRM_MODE_CONNECTOR_USB: c_int = 20;
//
// struct drm_mode_get_connector - Get connector metadata.
//
// User-space can perform a GETCONNECTOR ioctl to retrieve information about a
// connector. User-space is expected to retrieve encoders, modes and properties
// by performing this ioctl at least twice: the first time to retrieve the
// number of elements, the second time to retrieve the elements themselves.
//
// To retrieve the number of elements, set @count_props and @count_encoders to
// zero, set @count_modes to 1, and set @modes_ptr to a temporary struct
// drm_mode_modeinfo element.
//
// To retrieve the elements, allocate arrays for @encoders_ptr, @modes_ptr,
// @props_ptr and @prop_values_ptr, then set @count_modes, @count_props and
// @count_encoders to their capacity.
//
// Performing the ioctl only twice may be racy: the number of elements may have
// changed with a hotplug event in-between the two ioctls. User-space is
// expected to retry the last ioctl until the number of elements stabilizes.
// The kernel won't fill any array which doesn't have the expected length.
//
// **Force-probing a connector
//
// If the @count_modes field is set to zero and the DRM client is the current
// DRM master, the kernel will perform a forced probe on the connector to
// refresh the connector status, modes and EDID. A forced-probe can be slow,
// might cause flickering and the ioctl will block.
//
// User-space needs to force-probe connectors to ensure their metadata is
// up-to-date at startup and after receiving a hot-plug event. User-space
// may perform a forced-probe when the user explicitly requests it. User-space
// shouldn't perform a forced-probe in other situations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_connector {
// @encoders_ptr: Pointer to ``__u32`` array of object IDs.
    pub encoders_ptr: __u64,
// @modes_ptr: Pointer to struct drm_mode_modeinfo array.
    pub modes_ptr: __u64,
// @props_ptr: Pointer to ``__u32`` array of property IDs.
    pub props_ptr: __u64,
// @prop_values_ptr: Pointer to ``__u64`` array of property values.
    pub prop_values_ptr: __u64,
// @count_modes: Number of modes.
    pub count_modes: __u32,
// @count_props: Number of properties.
    pub count_props: __u32,
// @count_encoders: Number of encoders.
    pub count_encoders: __u32,
// @encoder_id: Object ID of the current encoder.
    pub encoder_id: __u32,
// @connector_id: Object ID of the connector.
    pub connector_id: __u32,
//
// @connector_type: Type of the connector.
//
// See DRM_MODE_CONNECTOR_* defines.
//
    pub connector_type: __u32,
//
// @connector_type_id: Type-specific connector number.
//
// This is not an object ID. This is a per-type connector number. Each
// (type, type_id) combination is unique across all connectors of a DRM
// device.
//
// The (type, type_id) combination is not a stable identifier: the
// type_id can change depending on the driver probe order.
//
    pub connector_type_id: __u32,
//
// @connection: Status of the connector.
//
// See enum drm_connector_status.
//
    pub connection: __u32,
// @mm_width: Width of the connected sink in millimeters.
    pub mm_width: __u32,
// @mm_height: Height of the connected sink in millimeters.
    pub mm_height: __u32,
//
// @subpixel: Subpixel order of the connected sink.
//
// See enum subpixel_order.
//
    pub subpixel: __u32,
// @pad: Padding, must be zero.
    pub pad: __u32,
}

// non-extended types: legacy bitmask, one bit per type:

// extended-types: rather than continue to consume a bit per type,
// grab a chunk of the bits to use as integer type id.
//
pub const DRM_MODE_PROP_EXTENDED_TYPE: c_uint = 0x0000ffc0;

// the PROP_ATOMIC flag is used to hide properties from userspace that
// is not aware of atomic properties.  This is mostly to work around
// older userspace (DDX drivers) that read/write each prop they find,
// without being aware that this could be triggering a lengthy modeset.
//
pub const DRM_MODE_PROP_ATOMIC: c_uint = 0x80000000;
//
// struct drm_mode_property_enum - Description for an enum/bitfield entry.
// @value: numeric value for this enum entry.
// @name: symbolic name for this enum entry.
//
// See struct drm_property_enum for details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_property_enum {
    pub value: __u64,
    pub name: [c_char; DRM_PROP_NAME_LEN],
}

//
// struct drm_mode_get_property - Get property metadata.
//
// User-space can perform a GETPROPERTY ioctl to retrieve information about a
// property. The same property may be attached to multiple objects, see
// "Modeset Base Object Abstraction".
//
// The meaning of the @values_ptr field changes depending on the property type.
// See &drm_property.flags for more details.
//
// The @enum_blob_ptr and @count_enum_blobs fields are only meaningful when the
// property has the type &DRM_MODE_PROP_ENUM or &DRM_MODE_PROP_BITMASK. For
// backwards compatibility, the kernel will always set @count_enum_blobs to
// zero when the property has the type &DRM_MODE_PROP_BLOB. User-space must
// ignore these two fields if the property has a different type.
//
// User-space is expected to retrieve values and enums by performing this ioctl
// at least twice: the first time to retrieve the number of elements, the
// second time to retrieve the elements themselves.
//
// To retrieve the number of elements, set @count_values and @count_enum_blobs
// to zero, then call the ioctl. @count_values will be updated with the number
// of elements. If the property has the type &DRM_MODE_PROP_ENUM or
// &DRM_MODE_PROP_BITMASK, @count_enum_blobs will be updated as well.
//
// To retrieve the elements themselves, allocate an array for @values_ptr and
// set @count_values to its capacity. If the property has the type
// &DRM_MODE_PROP_ENUM or &DRM_MODE_PROP_BITMASK, allocate an array for
// @enum_blob_ptr and set @count_enum_blobs to its capacity. Calling the ioctl
// again will fill the arrays.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_property {
// @values_ptr: Pointer to a ``__u64`` array.
    pub values_ptr: __u64,
// @enum_blob_ptr: Pointer to a struct drm_mode_property_enum array.
    pub enum_blob_ptr: __u64,
//
// @prop_id: Object ID of the property which should be retrieved. Set
// by the caller.
//
    pub prop_id: __u32,
//
// @flags: ``DRM_MODE_PROP_*`` bitfield. See &drm_property.flags for
// a definition of the flags.
//
    pub flags: __u32,
//
// @name: Symbolic property name. User-space should use this field to
// recognize properties.
//
    pub name: [c_char; DRM_PROP_NAME_LEN],
// @count_values: Number of elements in @values_ptr.
    pub count_values: __u32,
// @count_enum_blobs: Number of elements in @enum_blob_ptr.
    pub count_enum_blobs: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_connector_set_property {
    pub value: __u64,
    pub prop_id: __u32,
    pub connector_id: __u32,
}

pub const DRM_MODE_OBJECT_CRTC: c_uint = 0xcccccccc;
pub const DRM_MODE_OBJECT_CONNECTOR: c_uint = 0xc0c0c0c0;
pub const DRM_MODE_OBJECT_ENCODER: c_uint = 0xe0e0e0e0;
pub const DRM_MODE_OBJECT_MODE: c_uint = 0xdededede;
pub const DRM_MODE_OBJECT_PROPERTY: c_uint = 0xb0b0b0b0;
pub const DRM_MODE_OBJECT_FB: c_uint = 0xfbfbfbfb;
pub const DRM_MODE_OBJECT_BLOB: c_uint = 0xbbbbbbbb;
pub const DRM_MODE_OBJECT_PLANE: c_uint = 0xeeeeeeee;
pub const DRM_MODE_OBJECT_COLOROP: c_uint = 0xfafafafa;
pub const DRM_MODE_OBJECT_ANY: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_obj_get_properties {
    pub props_ptr: __u64,
    pub prop_values_ptr: __u64,
    pub count_props: __u32,
    pub obj_id: __u32,
    pub obj_type: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_obj_set_property {
    pub value: __u64,
    pub prop_id: __u32,
    pub obj_id: __u32,
    pub obj_type: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_blob {
    pub blob_id: __u32,
    pub length: __u32,
    pub data: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_fb_cmd {
    pub fb_id: __u32,
    pub width: __u32,
    pub height: __u32,
    pub pitch: __u32,
    pub bpp: __u32,
    pub depth: __u32,
// driver specific handle
    pub handle: __u32,
}

//
// struct drm_mode_fb_cmd2 - Frame-buffer metadata.
//
// This struct holds frame-buffer metadata. There are two ways to use it:
//
// - User-space can fill this struct and perform a &DRM_IOCTL_MODE_ADDFB2
// ioctl to register a new frame-buffer. The new frame-buffer object ID will
// be set by the kernel in @fb_id.
// - User-space can set @fb_id and perform a &DRM_IOCTL_MODE_GETFB2 ioctl to
// fetch metadata about an existing frame-buffer.
//
// In case of planar formats, this struct allows up to 4 buffer objects with
// offsets and pitches per plane. The pitch and offset order are dictated by
// the format FourCC as defined by ``drm_fourcc.h``, e.g. NV12 is described as:
//
// YUV 4:2:0 image with a plane of 8-bit Y samples followed by an
// interleaved U/V plane containing 8-bit 2x2 subsampled colour difference
// samples.
//
// So it would consist of a Y plane at ``offsets[0]`` and a UV plane at
// ``offsets[1]``.
//
// To accommodate tiled, compressed, etc formats, a modifier can be specified.
// For more information see the "Format Modifiers" section. Note that even
// though it looks like we have a modifier per-plane, we in fact do not. The
// modifier for each plane must be identical. Thus all combinations of
// different data layouts for multi-plane formats must be enumerated as
// separate modifiers.
//
// All of the entries in @handles, @pitches, @offsets and @modifier must be
// zero when unused. Warning, for @offsets and @modifier zero can't be used to
// figure out whether the entry is used or not since it's a valid value (a zero
// offset is common, and a zero modifier is &DRM_FORMAT_MOD_LINEAR).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_fb_cmd2 {
// @fb_id: Object ID of the frame-buffer.
    pub fb_id: __u32,
// @width: Width of the frame-buffer.
    pub width: __u32,
// @height: Height of the frame-buffer.
    pub height: __u32,
//
// @pixel_format: FourCC format code, see ``DRM_FORMAT_*`` constants in
// ``drm_fourcc.h``.
//
    pub pixel_format: __u32,
//
// @flags: Frame-buffer flags (see &DRM_MODE_FB_INTERLACED and
// &DRM_MODE_FB_MODIFIERS).
//
    pub flags: __u32,
//
// @handles: GEM buffer handle, one per plane. Set to 0 if the plane is
// unused. The same handle can be used for multiple planes.
//
    pub handles: [__u32; 4],
// @pitches: Pitch (aka. stride) in bytes, one per plane.
    pub pitches: [__u32; 4],
// @offsets: Offset into the buffer in bytes, one per plane.
    pub offsets: [__u32; 4],
//
// @modifier: Format modifier, one per plane. See ``DRM_FORMAT_MOD_*``
// constants in ``drm_fourcc.h``. All planes must use the same
// modifier. Ignored unless &DRM_MODE_FB_MODIFIERS is set in @flags.
//
    pub modifier: [__u64; 4],
}

pub const DRM_MODE_FB_DIRTY_ANNOTATE_COPY: c_uint = 0x01;
pub const DRM_MODE_FB_DIRTY_ANNOTATE_FILL: c_uint = 0x02;
pub const DRM_MODE_FB_DIRTY_FLAGS: c_uint = 0x03;
pub const DRM_MODE_FB_DIRTY_MAX_CLIPS: c_int = 256;
//
// Mark a region of a framebuffer as dirty.
//
// Some hardware does not automatically update display contents
// as a hardware or software draw to a framebuffer. This ioctl
// allows userspace to tell the kernel and the hardware what
// regions of the framebuffer have changed.
//
// The kernel or hardware is free to update more then just the
// region specified by the clip rects. The kernel or hardware
// may also delay and/or coalesce several calls to dirty into a
// single update.
//
// Userspace may annotate the updates, the annotates are a
// promise made by the caller that the change is either a copy
// of pixels or a fill of a single color in the region specified.
//
// If the DRM_MODE_FB_DIRTY_ANNOTATE_COPY flag is given then
// the number of updated regions are half of num_clips given,
// where the clip rects are paired in src and dst. The width and
// height of each one of the pairs must match.
//
// If the DRM_MODE_FB_DIRTY_ANNOTATE_FILL flag is given the caller
// promises that the region specified of the clip rects is filled
// completely with a single color as given in the color argument.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_fb_dirty_cmd {
    pub fb_id: __u32,
    pub flags: __u32,
    pub color: __u32,
    pub num_clips: __u32,
    pub clips_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_mode_cmd {
    pub connector_id: __u32,
    pub mode: drm_mode_modeinfo,
}

pub const DRM_MODE_CURSOR_BO: c_uint = 0x01;
pub const DRM_MODE_CURSOR_MOVE: c_uint = 0x02;
pub const DRM_MODE_CURSOR_FLAGS: c_uint = 0x03;
//
// depending on the value in flags different members are used.
//
// CURSOR_BO uses
// crtc_id
// width
// height
// handle - if 0 turns the cursor off
//
// CURSOR_MOVE uses
// crtc_id
// x
// y
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_cursor {
    pub flags: __u32,
    pub crtc_id: __u32,
    pub x: __s32,
    pub y: __s32,
    pub width: __u32,
    pub height: __u32,
// driver specific handle
    pub handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_cursor2 {
    pub flags: __u32,
    pub crtc_id: __u32,
    pub x: __s32,
    pub y: __s32,
    pub width: __u32,
    pub height: __u32,
// driver specific handle
    pub handle: __u32,
    pub hot_x: __s32,
    pub hot_y: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_crtc_lut {
    pub crtc_id: __u32,
    pub gamma_size: __u32,
// pointers to arrays
    pub red: __u64,
    pub green: __u64,
    pub blue: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_color_ctm {
//
// Conversion matrix in S31.32 sign-magnitude
// (not two's complement!) format.
//
// out   matrix    in
// |R|   |0 1 2|   |R|
// |G| = |3 4 5| x |G|
// |B|   |6 7 8|   |B|
//
    pub matrix: [__u64; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_color_ctm_3x4 {
//
// Conversion matrix with 3x4 dimensions in S31.32 sign-magnitude
// (not two's complement!) format.
//
// out   matrix          in
// |R|   |0  1  2  3 |   | R |
// |G| = |4  5  6  7 | x | G |
// |B|   |8  9  10 11|   | B |
// |1.0|
//
    pub matrix: [__u64; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_color_lut {
//
// Values are mapped linearly to 0.0 - 1.0 range, with 0x0 == 0.0 and
// 0xffff == 1.0.
//
    pub red: __u16,
    pub green: __u16,
    pub blue: __u16,
    pub reserved: __u16,
}

//
// struct drm_color_lut32
//
// 32-bit per channel color LUT entry, similar to drm_color_lut.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_color_lut32 {
    pub red: __u32,
    pub green: __u32,
    pub blue: __u32,
    pub reserved: __u32,
}

//
// enum drm_colorop_type - Type of color operation
//
// drm_colorops can be of many different types. Each type behaves differently
// and defines a different set of properties. This enum defines all types and
// gives a high-level description.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_colorop_type {
//
// @DRM_COLOROP_1D_CURVE:
//
// enum string "1D Curve"
//
// A 1D curve that is being applied to all color channels. The
// curve is specified via the CURVE_1D_TYPE colorop property.
//
    DRM_COLOROP_1D_CURVE,

//
// @DRM_COLOROP_1D_LUT:
//
// enum string "1D LUT"
//
// A simple 1D LUT of uniformly spaced &drm_color_lut32 entries,
// packed into a blob via the DATA property. The driver's
// expected LUT size is advertised via the SIZE property.
//
// The DATA blob is an array of struct drm_color_lut32 with size
// of "size".
//
    DRM_COLOROP_1D_LUT,

//
// @DRM_COLOROP_CTM_3X4:
//
// enum string "3x4 Matrix"
//
// A 3x4 matrix. Its values are specified via the
// &drm_color_ctm_3x4 struct provided via the DATA property.
//
// The DATA blob is a float[12]:
// out   matrix          in
// | R |   | 0  1  2  3  |   | R |
// | G | = | 4  5  6  7  | x | G |
// | B |   | 8  9  10 12 |   | B |
//
    DRM_COLOROP_CTM_3X4,

//
// @DRM_COLOROP_MULTIPLIER:
//
// enum string "Multiplier"
//
// A simple multiplier, applied to all color values. The
// multiplier is specified as a S31.32 via the MULTIPLIER
// property.
//
    DRM_COLOROP_MULTIPLIER,

//
// @DRM_COLOROP_3D_LUT:
//
// enum string "3D LUT"
//
// A 3D LUT of &drm_color_lut32 entries,
// packed into a blob via the DATA property. The driver's expected
// LUT size is advertised via the SIZE property, i.e., a 3D LUT with
// 17x17x17 entries will have SIZE set to 17.
//
// The DATA blob is a 3D array of struct drm_color_lut32 with dimension
// length of "size".
// The LUT elements are traversed like so:
//
// for B in range 0..n
// for G in range 0..n
// for R in range 0..n
// index = R + n * (G + n * B)
// color = lut3d[index]
//
    DRM_COLOROP_3D_LUT,
}

//
// enum drm_colorop_lut3d_interpolation_type - type of 3DLUT interpolation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_colorop_lut3d_interpolation_type {
//
// @DRM_COLOROP_LUT3D_INTERPOLATION_TETRAHEDRAL:
//
// Tetrahedral 3DLUT interpolation
//
    DRM_COLOROP_LUT3D_INTERPOLATION_TETRAHEDRAL,
}

//
// enum drm_colorop_lut1d_interpolation_type - type of interpolation for 1D LUTs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_colorop_lut1d_interpolation_type {
//
// @DRM_COLOROP_LUT1D_INTERPOLATION_LINEAR:
//
// Linear interpolation. Values between points of the LUT will be
// linearly interpolated.
//
    DRM_COLOROP_LUT1D_INTERPOLATION_LINEAR,
}

//
// struct drm_plane_size_hint - Plane size hints
// @width: The width of the plane in pixel
// @height: The height of the plane in pixel
//
// The plane SIZE_HINTS property blob contains an
// array of struct drm_plane_size_hint.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_plane_size_hint {
    pub width: __u16,
    pub height: __u16,
}

//
// struct hdr_metadata_infoframe - HDR Metadata Infoframe Data.
//
// HDR Metadata Infoframe as per CTA 861.G spec. This is expected
// to match exactly with the spec.
//
// Userspace is expected to pass the metadata information as per
// the format described in this structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_metadata_infoframe {
//
// @eotf: Electro-Optical Transfer Function (EOTF)
// used in the stream.
//
    pub eotf: __u8,
//
// @metadata_type: Static_Metadata_Descriptor_ID.
//
    pub metadata_type: __u8,
//
// @display_primaries: Color Primaries of the Data.
// These are coded as unsigned 16-bit values in units of
// 0.00002, where 0x0000 represents zero and 0xC350
// represents 1.0000.
// @display_primaries.x: X coordinate of color primary.
// @display_primaries.y: Y coordinate of color primary.
//
    pub y: __u16 x,,
    pub display_primaries: [}; 3],
//
// @white_point: White Point of Colorspace Data.
// These are coded as unsigned 16-bit values in units of
// 0.00002, where 0x0000 represents zero and 0xC350
// represents 1.0000.
// @white_point.x: X coordinate of whitepoint of color primary.
// @white_point.y: Y coordinate of whitepoint of color primary.
//
    pub y: __u16 x,,
    pub white_point: },
//
// @max_display_mastering_luminance: Max Mastering Display Luminance.
// This value is coded as an unsigned 16-bit value in units of 1 cd/m2,
// where 0x0001 represents 1 cd/m2 and 0xFFFF represents 65535 cd/m2.
//
    pub max_display_mastering_luminance: __u16,
//
// @min_display_mastering_luminance: Min Mastering Display Luminance.
// This value is coded as an unsigned 16-bit value in units of
// 0.0001 cd/m2, where 0x0001 represents 0.0001 cd/m2 and 0xFFFF
// represents 6.5535 cd/m2.
//
    pub min_display_mastering_luminance: __u16,
//
// @max_cll: Max Content Light Level.
// This value is coded as an unsigned 16-bit value in units of 1 cd/m2,
// where 0x0001 represents 1 cd/m2 and 0xFFFF represents 65535 cd/m2.
//
    pub max_cll: __u16,
//
// @max_fall: Max Frame Average Light Level.
// This value is coded as an unsigned 16-bit value in units of 1 cd/m2,
// where 0x0001 represents 1 cd/m2 and 0xFFFF represents 65535 cd/m2.
//
    pub max_fall: __u16,
}

//
// struct hdr_output_metadata - HDR output metadata
//
// Metadata Information to be passed from userspace
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_output_metadata {
//
// @metadata_type: Static_Metadata_Descriptor_ID.
//
    pub metadata_type: __u32,
//
// @hdmi_metadata_type1: HDR Metadata Infoframe.
//
    pub hdmi_metadata_type1: hdr_metadata_infoframe,
}

//
// DRM_MODE_PAGE_FLIP_EVENT
//
// Request that the kernel sends back a vblank event (see
// struct drm_event_vblank) with the &DRM_EVENT_FLIP_COMPLETE type when the
// page-flip is done.
//
// When used with atomic uAPI, one event will be delivered per CRTC included in
// the atomic commit. A CRTC is included in an atomic commit if one of its
// properties is set, or if a property is set on a connector or plane linked
// via the CRTC_ID property to the CRTC. At least one CRTC must be included,
// and all pulled in CRTCs must be either previously or newly powered on (in
// other words, a powered off CRTC which stays off cannot be included in the
// atomic commit).
//
pub const DRM_MODE_PAGE_FLIP_EVENT: c_uint = 0x01;
//
// DRM_MODE_PAGE_FLIP_ASYNC
//
// Request that the page-flip is performed as soon as possible, ie. with no
// delay due to waiting for vblank. This may cause tearing to be visible on
// the screen.
//
// When used with atomic uAPI, the driver will return an error if the hardware
// doesn't support performing an asynchronous page-flip for this update.
// User-space should handle this, e.g. by falling back to a regular page-flip.
//
// Note, some hardware might need to perform one last synchronous page-flip
// before being able to switch to asynchronous page-flips. As an exception,
// the driver will return success even though that first page-flip is not
// asynchronous.
//
pub const DRM_MODE_PAGE_FLIP_ASYNC: c_uint = 0x02;
pub const DRM_MODE_PAGE_FLIP_TARGET_ABSOLUTE: c_uint = 0x4;
pub const DRM_MODE_PAGE_FLIP_TARGET_RELATIVE: c_uint = 0x8;

//
// DRM_MODE_PAGE_FLIP_FLAGS
//
// Bitmask of flags suitable for &drm_mode_crtc_page_flip_target.flags.
//

//
// Request a page flip on the specified crtc.
//
// This ioctl will ask KMS to schedule a page flip for the specified
// crtc.  Once any pending rendering targeting the specified fb (as of
// ioctl time) has completed, the crtc will be reprogrammed to display
// that fb after the next vertical refresh.  The ioctl returns
// immediately, but subsequent rendering to the current fb will block
// in the execbuffer ioctl until the page flip happens.  If a page
// flip is already pending as the ioctl is called, EBUSY will be
// returned.
//
// Flag DRM_MODE_PAGE_FLIP_EVENT requests that drm sends back a vblank
// event (see drm.h: struct drm_event_vblank) when the page flip is
// done.  The user_data field passed in with this ioctl will be
// returned as the user_data field in the vblank event struct.
//
// Flag DRM_MODE_PAGE_FLIP_ASYNC requests that the flip happen
// 'as soon as possible', meaning that it not delay waiting for vblank.
// This may cause tearing on the screen.
//
// The reserved field must be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_crtc_page_flip {
    pub crtc_id: __u32,
    pub fb_id: __u32,
    pub flags: __u32,
    pub reserved: __u32,
    pub user_data: __u64,
}

//
// Request a page flip on the specified crtc.
//
// Same as struct drm_mode_crtc_page_flip, but supports new flags and
// re-purposes the reserved field:
//
// The sequence field must be zero unless either of the
// DRM_MODE_PAGE_FLIP_TARGET_ABSOLUTE/RELATIVE flags is specified. When
// the ABSOLUTE flag is specified, the sequence field denotes the absolute
// vblank sequence when the flip should take effect. When the RELATIVE
// flag is specified, the sequence field denotes the relative (to the
// current one when the ioctl is called) vblank sequence when the flip
// should take effect. NOTE: DRM_IOCTL_WAIT_VBLANK must still be used to
// make sure the vblank sequence before the target one has passed before
// calling this ioctl. The purpose of the
// DRM_MODE_PAGE_FLIP_TARGET_ABSOLUTE/RELATIVE flags is merely to clarify
// the target for when code dealing with a page flip runs during a
// vertical blank period.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_crtc_page_flip_target {
    pub crtc_id: __u32,
    pub fb_id: __u32,
    pub flags: __u32,
    pub sequence: __u32,
    pub user_data: __u64,
}

//
// struct drm_mode_create_dumb - Create a KMS dumb buffer for scanout.
// @height: buffer height in pixels
// @width: buffer width in pixels
// @bpp: color mode
// @flags: must be zero
// @handle: buffer object handle
// @pitch: number of bytes between two consecutive lines
// @size: size of the whole buffer in bytes
//
// User-space fills @height, @width, @bpp and @flags. If the IOCTL succeeds,
// the kernel fills @handle, @pitch and @size.
//
// The value of @bpp is a color-mode number describing a specific format
// or a variant thereof. The value often corresponds to the number of bits
// per pixel for most modes, although there are exceptions. Each color mode
// maps to a DRM format plus a number of modes with similar pixel layout.
// Framebuffer layout is always linear.
//
// Support for all modes and formats is optional. Even if dumb-buffer
// creation with a certain color mode succeeds, it is not guaranteed that
// the DRM driver supports any of the related formats. Most drivers support
// a color mode of 32 with a format of DRM_FORMAT_XRGB8888 on their primary
// plane.
//
// +------------+------------------------+------------------------+
// | Color mode | Framebuffer format     | Compatible formats     |
// +============+========================+========================+
// |     32     |  * DRM_FORMAT_XRGB8888 |  * DRM_FORMAT_BGRX8888 |
// |            |                        |  * DRM_FORMAT_RGBX8888 |
// |            |                        |  * DRM_FORMAT_XBGR8888 |
// +------------+------------------------+------------------------+
// |     24     |  * DRM_FORMAT_RGB888   |  * DRM_FORMAT_BGR888   |
// +------------+------------------------+------------------------+
// |     16     |  * DRM_FORMAT_RGB565   |  * DRM_FORMAT_BGR565   |
// +------------+------------------------+------------------------+
// |     15     |  * DRM_FORMAT_XRGB1555 |  * DRM_FORMAT_BGRX1555 |
// |            |                        |  * DRM_FORMAT_RGBX1555 |
// |            |                        |  * DRM_FORMAT_XBGR1555 |
// +------------+------------------------+------------------------+
// |      8     |  * DRM_FORMAT_C8       |  * DRM_FORMAT_D8       |
// |            |                        |  * DRM_FORMAT_R8       |
// +------------+------------------------+------------------------+
// |      4     |  * DRM_FORMAT_C4       |  * DRM_FORMAT_D4       |
// |            |                        |  * DRM_FORMAT_R4       |
// +------------+------------------------+------------------------+
// |      2     |  * DRM_FORMAT_C2       |  * DRM_FORMAT_D2       |
// |            |                        |  * DRM_FORMAT_R2       |
// +------------+------------------------+------------------------+
// |      1     |  * DRM_FORMAT_C1       |  * DRM_FORMAT_D1       |
// |            |                        |  * DRM_FORMAT_R1       |
// +------------+------------------------+------------------------+
//
// Color modes of 10, 12, 15, 30 and 64 are only supported for use by
// legacy user space. Please don't use them in new code. Other modes
// are not support.
//
// Do not attempt to allocate anything but linear framebuffer memory
// with single-plane RGB data. Allocation of other framebuffer
// layouts requires dedicated ioctls in the respective DRM driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_create_dumb {
    pub height: __u32,
    pub width: __u32,
    pub bpp: __u32,
    pub flags: __u32,
    pub handle: __u32,
    pub pitch: __u32,
    pub size: __u64,
}

// set up for mmap of a dumb scanout buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_map_dumb {
// Handle for the object being mapped.
    pub handle: __u32,
    pub pad: __u32,
//
// Fake offset to use for subsequent mmap call
//
// This is a fixed-size type for 32/64 compatibility.
//
    pub offset: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_destroy_dumb {
    pub handle: __u32,
}

//
// DRM_MODE_ATOMIC_TEST_ONLY
//
// Do not apply the atomic commit, instead check whether the hardware supports
// this configuration.
//
// See &drm_mode_config_funcs.atomic_check for more details on test-only
// commits.
//
pub const DRM_MODE_ATOMIC_TEST_ONLY: c_uint = 0x0100;
//
// DRM_MODE_ATOMIC_NONBLOCK
//
// Do not block while applying the atomic commit. The &DRM_IOCTL_MODE_ATOMIC
// IOCTL returns immediately instead of waiting for the changes to be applied
// in hardware. Note, the driver will still check that the update can be
// applied before retuning.
//
pub const DRM_MODE_ATOMIC_NONBLOCK: c_uint = 0x0200;
//
// DRM_MODE_ATOMIC_ALLOW_MODESET
//
// Allow the update to result in temporary or transient visible artifacts while
// the update is being applied. Applying the update may also take significantly
// more time than a page flip. All visual artifacts will disappear by the time
// the update is completed, as signalled through the vblank event's timestamp
// (see struct drm_event_vblank).
//
// This flag must be set when the KMS update might cause visible artifacts.
// Without this flag such KMS update will return a EINVAL error. What kind of
// update may cause visible artifacts depends on the driver and the hardware.
// User-space that needs to know beforehand if an update might cause visible
// artifacts can use &DRM_MODE_ATOMIC_TEST_ONLY without
// &DRM_MODE_ATOMIC_ALLOW_MODESET to see if it fails.
//
// To the best of the driver's knowledge, visual artifacts are guaranteed to
// not appear when this flag is not set. Some sinks might display visual
// artifacts outside of the driver's control.
//
pub const DRM_MODE_ATOMIC_ALLOW_MODESET: c_uint = 0x0400;
//
// DRM_MODE_ATOMIC_FLAGS
//
// Bitfield of flags accepted by the &DRM_IOCTL_MODE_ATOMIC IOCTL in
// &drm_mode_atomic.flags.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_atomic {
    pub flags: __u32,
    pub count_objs: __u32,
    pub objs_ptr: __u64,
    pub count_props_ptr: __u64,
    pub props_ptr: __u64,
    pub prop_values_ptr: __u64,
    pub reserved: __u64,
    pub user_data: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_format_modifier_blob {
pub const FORMAT_BLOB_CURRENT: c_int = 1;
// Version of this blob format
    pub version: __u32,
// Flags
    pub flags: __u32,
// Number of fourcc formats supported
    pub count_formats: __u32,
// Where in this blob the formats exist (in bytes)
    pub formats_offset: __u32,
// Number of drm_format_modifiers
    pub count_modifiers: __u32,
// Where in this blob the modifiers exist (in bytes)
    pub modifiers_offset: __u32,
// __u32 formats[]
// struct drm_format_modifier modifiers[]
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_format_modifier {
// Bitmask of formats in get_plane format list this info applies to. The
// offset allows a sliding window of which 64 formats (bits).
//
// Some examples:
// In today's world with < 65 formats, and formats 0, and 2 are
// supported
// 0x0000000000000005
// ^-offset = 0, formats = 5
//
// If the number formats grew to 128, and formats 98-102 are
// supported with the modifier:
//
// 0x0000007c00000000 0000000000000000
// ^
// |__offset = 64, formats = 0x7c00000000
//
    pub formats: __u64,
    pub offset: __u32,
    pub pad: __u32,
// The modifier that applies to the >get_plane format list bitmask.
    pub modifier: __u64,
}

//
// struct drm_mode_create_blob - Create New blob property
//
// Create a new 'blob' data property, copying length bytes from data pointer,
// and returning new blob ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_create_blob {
// @data: Pointer to data to copy.
    pub data: __u64,
// @length: Length of data to copy.
    pub length: __u32,
// @blob_id: Return: new property ID.
    pub blob_id: __u32,
}

//
// struct drm_mode_destroy_blob - Destroy user blob
// @blob_id: blob_id to destroy
//
// Destroy a user-created blob property.
//
// User-space can release blobs as soon as they do not need to refer to them by
// their blob object ID.  For instance, if you are using a MODE_ID blob in an
// atomic commit and you will not make another commit re-using the same ID, you
// can destroy the blob as soon as the commit has been issued, without waiting
// for it to complete.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_destroy_blob {
    pub blob_id: __u32,
}

//
// struct drm_mode_create_lease - Create lease
//
// Lease mode resources, creating another drm_master.
//
// The @object_ids array must reference at least one CRTC, one connector and
// one plane if &DRM_CLIENT_CAP_UNIVERSAL_PLANES is enabled. Alternatively,
// the lease can be completely empty.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_create_lease {
// @object_ids: Pointer to array of object ids (__u32)
    pub object_ids: __u64,
// @object_count: Number of object ids
    pub object_count: __u32,
// @flags: flags for new FD (O_CLOEXEC, etc)
    pub flags: __u32,
// @lessee_id: Return: unique identifier for lessee.
    pub lessee_id: __u32,
// @fd: Return: file descriptor to new drm_master file
    pub fd: __u32,
}

//
// struct drm_mode_list_lessees - List lessees
//
// List lesses from a drm_master.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_list_lessees {
//
// @count_lessees: Number of lessees.
//
// On input, provides length of the array.
// On output, provides total number. No
// more than the input number will be written
// back, so two calls can be used to get
// the size and then the data.
//
    pub count_lessees: __u32,
// @pad: Padding.
    pub pad: __u32,
//
// @lessees_ptr: Pointer to lessees.
//
// Pointer to __u64 array of lessee ids
//
    pub lessees_ptr: __u64,
}

//
// struct drm_mode_get_lease - Get Lease
//
// Get leased objects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_get_lease {
//
// @count_objects: Number of leased objects.
//
// On input, provides length of the array.
// On output, provides total number. No
// more than the input number will be written
// back, so two calls can be used to get
// the size and then the data.
//
    pub count_objects: __u32,
// @pad: Padding.
    pub pad: __u32,
//
// @objects_ptr: Pointer to objects.
//
// Pointer to __u32 array of object ids.
//
    pub objects_ptr: __u64,
}

//
// struct drm_mode_revoke_lease - Revoke lease
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_revoke_lease {
// @lessee_id: Unique ID of lessee
    pub lessee_id: __u32,
}

//
// struct drm_mode_rect - Two dimensional rectangle.
// @x1: Horizontal starting coordinate (inclusive).
// @y1: Vertical starting coordinate (inclusive).
// @x2: Horizontal ending coordinate (exclusive).
// @y2: Vertical ending coordinate (exclusive).
//
// With drm subsystem using struct drm_rect to manage rectangular area this
// export it to user-space.
//
// Currently used by drm_mode_atomic blob property FB_DAMAGE_CLIPS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_rect {
    pub x1: __s32,
    pub y1: __s32,
    pub x2: __s32,
    pub y2: __s32,
}

//
// struct drm_mode_closefb
// @fb_id: Framebuffer ID.
// @pad: Must be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_mode_closefb {
    pub fb_id: __u32,
    pub pad: __u32,
}

//
// Put 16-bit ARGB values into a standard 64-bit representation that can be
// used for ioctl parameters, inter-driver communication, etc.
//
// If the component values being provided contain less than 16 bits of
// precision, use a conversion ratio to get a better color approximation.
// The ratio is computed as (2^16 - 1) / (2^bpc - 1), where bpc and 16 are
// the input and output precision, respectively.
// Also note bpc must be greater than 0.
//

//
// Extract the specified color component from a standard 64-bit ARGB value.
//
// If the requested precision is less than 16 bits, make use of a conversion
// ratio calculated as (2^bpc - 1) / (2^16 - 1), where bpc and 16 are the
// output and input precision, respectively.
//
// If speed is more important than accuracy, use DRM_ARGB64_GET*_BPCS()
// instead of DRM_ARGB64_GET*_BPC() in order to replace the expensive
// division with a simple bit right-shift operation.
//

