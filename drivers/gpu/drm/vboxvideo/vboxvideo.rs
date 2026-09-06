//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vboxvideo/vboxvideo.h
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
// Copyright (C) 2006-2016 Oracle Corporation
pub const VBOX_VIDEO_MAX_SCREENS: c_int = 64;
//
// The last 4096 bytes of the guest VRAM contains the generic info for all
// DualView chunks: sizes and offsets of chunks. This is filled by miniport.
//
// Last 4096 bytes of each chunk contain chunk specific data: framebuffer info,
// etc. This is used exclusively by the corresponding instance of a display
// driver.
//
// The VRAM layout:
// Last 4096 bytes - Adapter information area.
// 4096 bytes aligned miniport heap (value specified in the config rouded up).
// Slack - what left after dividing the VRAM.
// 4096 bytes aligned framebuffers:
// last 4096 bytes of each framebuffer is the display information area.
//
// The Virtual Graphics Adapter information in the guest VRAM is stored by the
// guest video driver using structures prepended by VBOXVIDEOINFOHDR.
//
// When the guest driver writes dword 0 to the VBE_DISPI_INDEX_VBOX_VIDEO
// the host starts to process the info. The first element at the start of
// the 4096 bytes region should be normally be a LINK that points to
// actual information chain. That way the guest driver can have some
// fixed layout of the information memory block and just rewrite
// the link to point to relevant memory chain.
//
// The processing stops at the END element.
//
// The host can access the memory only when the port IO is processed.
// All data that will be needed later must be copied from these 4096 bytes.
// But other VRAM can be used by host until the mode is disabled.
//
// The guest driver writes dword 0xffffffff to the VBE_DISPI_INDEX_VBOX_VIDEO
// to disable the mode.
//
// VBE_DISPI_INDEX_VBOX_VIDEO is used to read the configuration information
// from the host and issue commands to the host.
//
// The guest writes the VBE_DISPI_INDEX_VBOX_VIDEO index register, the
// following operations with the VBE data register can be performed:
//
// Operation            Result
// write 16 bit value   NOP
// read 16 bit value    count of monitors
// write 32 bit value   set the vbox cmd value and the cmd processed by the host
// read 32 bit value    result of the last vbox command is returned
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_cmd_hdr {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
    pub __packed: },
//
// The VBVA ring buffer is suitable for transferring large (< 2GB) amount of
// data. For example big bitmaps which do not fit to the buffer.
//
// Guest starts writing to the buffer by initializing a record entry in the
// records queue. VBVA_F_RECORD_PARTIAL indicates that the record is being
// written. As data is written to the ring buffer, the guest increases
// free_offset.
//
// The host reads the records on flushes and processes all completed records.
// When host encounters situation when only a partial record presents and
// len_and_flags & ~VBVA_F_RECORD_PARTIAL >= VBVA_RING_BUFFER_SIZE -
// VBVA_RING_BUFFER_THRESHOLD, the host fetched all record data and updates
// data_offset. After that on each flush the host continues fetching the data
// until the record is completed.
//

pub const VBVA_F_MODE_ENABLED: c_uint = 0x00000001u;
pub const VBVA_F_MODE_VRDP: c_uint = 0x00000002u;
pub const VBVA_F_MODE_VRDP_RESET: c_uint = 0x00000004u;
pub const VBVA_F_MODE_VRDP_ORDER_MASK: c_uint = 0x00000008u;
pub const VBVA_F_STATE_PROCESSING: c_uint = 0x00010000u;
pub const VBVA_F_RECORD_PARTIAL: c_uint = 0x80000000u;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_record {
    pub len_and_flags: u32,
    pub __packed: },
//
// The minimum HGSMI heap size is PAGE_SIZE (4096 bytes) and is a restriction of
// the runtime heapsimple API. Use minimum 2 pages here, because the info area
// also may contain other data (for example hgsmi_host_flags structure).
//
pub const VBVA_ADAPTER_INFORMATION_SIZE: c_int = 65536;
pub const VBVA_MIN_BUFFER_SIZE: c_int = 65536;
// The value for port IO to let the adapter to interpret the adapter memory.
pub const VBOX_VIDEO_DISABLE_ADAPTER_MEMORY: c_uint = 0xFFFFFFFF;
// The value for port IO to let the adapter to interpret the adapter memory.
pub const VBOX_VIDEO_INTERPRET_ADAPTER_MEMORY: c_uint = 0x00000000;
//
// The value for port IO to let the adapter to interpret the display memory.
// The display number is encoded in low 16 bits.
//
pub const VBOX_VIDEO_INTERPRET_DISPLAY_MEMORY_BASE: c_uint = 0x00010000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_host_flags {
    pub host_events: u32,
    pub supported_orders: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_buffer {
    pub host_flags: vbva_host_flags,
// The offset where the data start in the buffer.
    pub data_offset: u32,
// The offset where next data must be placed in the buffer.
    pub free_offset: u32,
// The queue of record descriptions.
    pub records: [vbva_record; VBVA_MAX_RECORDS],
    pub record_first_index: u32,
    pub record_free_index: u32,
// Space to leave free when large partial records are transferred.
    pub partial_write_tresh: u32,
    pub data_len: u32,
// variable size for the rest of the vbva_buffer area in VRAM.
    pub data: [u8; ],
    pub __packed: },

// guest->host commands
pub const VBVA_QUERY_CONF32: c_int = 1;
pub const VBVA_SET_CONF32: c_int = 2;
pub const VBVA_INFO_VIEW: c_int = 3;
pub const VBVA_INFO_HEAP: c_int = 4;
pub const VBVA_FLUSH: c_int = 5;
pub const VBVA_INFO_SCREEN: c_int = 6;
pub const VBVA_ENABLE: c_int = 7;
pub const VBVA_MOUSE_POINTER_SHAPE: c_int = 8;
// informs host about HGSMI caps. see vbva_caps below
pub const VBVA_INFO_CAPS: c_int = 12;
// configures scanline, see VBVASCANLINECFG below
pub const VBVA_SCANLINE_CFG: c_int = 13;
// requests scanline info, see VBVASCANLINEINFO below
pub const VBVA_SCANLINE_INFO: c_int = 14;
// inform host about VBVA Command submission
pub const VBVA_CMDVBVA_SUBMIT: c_int = 16;
// inform host about VBVA Command submission
pub const VBVA_CMDVBVA_FLUSH: c_int = 17;
// G->H DMA command
pub const VBVA_CMDVBVA_CTL: c_int = 18;
// Query most recent mode hints sent
pub const VBVA_QUERY_MODE_HINTS: c_int = 19;
//
// Report the guest virtual desktop position and size for mapping host and
// guest pointer positions.
//
pub const VBVA_REPORT_INPUT_MAPPING: c_int = 20;
// Report the guest cursor position and query the host position.
pub const VBVA_CURSOR_POSITION: c_int = 21;
// host->guest commands
pub const VBVAHG_EVENT: c_int = 1;
pub const VBVAHG_DISPLAY_CUSTOM: c_int = 2;
// vbva_conf32::index
pub const VBOX_VBVA_CONF32_MONITOR_COUNT: c_int = 0;
pub const VBOX_VBVA_CONF32_HOST_HEAP_SIZE: c_int = 1;
//
// Returns VINF_SUCCESS if the host can report mode hints via VBVA.
// Set value to VERR_NOT_SUPPORTED before calling.
//
pub const VBOX_VBVA_CONF32_MODE_HINT_REPORTING: c_int = 2;
//
// Returns VINF_SUCCESS if the host can report guest cursor enabled status via
// VBVA.  Set value to VERR_NOT_SUPPORTED before calling.
//
pub const VBOX_VBVA_CONF32_GUEST_CURSOR_REPORTING: c_int = 3;
//
// Returns the currently available host cursor capabilities.  Available if
// VBOX_VBVA_CONF32_GUEST_CURSOR_REPORTING returns success.
//
pub const VBOX_VBVA_CONF32_CURSOR_CAPABILITIES: c_int = 4;
// Returns the supported flags in vbva_infoscreen.flags.
pub const VBOX_VBVA_CONF32_SCREEN_FLAGS: c_int = 5;
// Returns the max size of VBVA record.
pub const VBOX_VBVA_CONF32_MAX_RECORD_SIZE: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_conf32 {
    pub index: u32,
    pub value: u32,
    pub __packed: },
// Reserved for historical reasons.

//
// Guest cursor capability: can the host show a hardware cursor at the host
// pointer location?
//

// Reserved for historical reasons.

// Reserved for historical reasons.  Must always be unset.

// Reserved for historical reasons.

// Reserved for historical reasons.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_infoview {
// Index of the screen, assigned by the guest.
    pub view_index: u32,
// The screen offset in VRAM, the framebuffer starts here.
    pub view_offset: u32,
// The size of the VRAM memory that can be used for the view.
    pub view_size: u32,
// The recommended maximum size of the VRAM memory for the screen.
    pub max_screen_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_flush {
    pub reserved: u32,
    pub __packed: },
// vbva_infoscreen.flags
pub const VBVA_SCREEN_F_NONE: c_uint = 0x0000;
pub const VBVA_SCREEN_F_ACTIVE: c_uint = 0x0001;
//
// The virtual monitor has been disabled by the guest and should be removed
// by the host and ignored for purposes of pointer position calculation.
//
pub const VBVA_SCREEN_F_DISABLED: c_uint = 0x0002;
//
// The virtual monitor has been blanked by the guest and should be blacked
// out by the host using width, height, etc values from the vbva_infoscreen
// request.
//
pub const VBVA_SCREEN_F_BLANK: c_uint = 0x0004;
//
// The virtual monitor has been blanked by the guest and should be blacked
// out by the host using the previous mode values for width. height, etc.
//
pub const VBVA_SCREEN_F_BLANK2: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_infoscreen {
// Which view contains the screen.
    pub view_index: u32,
// Physical X origin relative to the primary screen.
    pub origin_x: i32,
// Physical Y origin relative to the primary screen.
    pub origin_y: i32,
// Offset of visible framebuffer relative to the framebuffer start.
    pub start_offset: u32,
// The scan line size in bytes.
    pub line_size: u32,
// Width of the screen.
    pub width: u32,
// Height of the screen.
    pub height: u32,
// Color depth.
    pub bits_per_pixel: u16,
// VBVA_SCREEN_F_*
    pub flags: u16,
    pub __packed: },
// vbva_enable.flags
pub const VBVA_F_NONE: c_uint = 0x00000000;
pub const VBVA_F_ENABLE: c_uint = 0x00000001;
pub const VBVA_F_DISABLE: c_uint = 0x00000002;
// extended VBVA to be used with WDDM
pub const VBVA_F_EXTENDED: c_uint = 0x00000004;
// vbva offset is absolute VRAM offset
pub const VBVA_F_ABSOFFSET: c_uint = 0x00000008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_enable {
    pub flags: u32,
    pub offset: u32,
    pub result: i32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_enable_ex {
    pub base: vbva_enable,
    pub screen_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_mouse_pointer_shape {
// The host result.
    pub result: i32,
// VBOX_MOUSE_POINTER_* bit flags.
    pub flags: u32,
// X coordinate of the hot spot.
    pub hot_X: u32,
// Y coordinate of the hot spot.
    pub hot_y: u32,
// Width of the pointer in pixels.
    pub width: u32,
// Height of the pointer in scanlines.
    pub height: u32,
// Pointer data.
//
// The data consists of 1 bpp AND mask followed by 32 bpp XOR (color)
// mask.
//
// For pointers without alpha channel the XOR mask pixels are 32 bit
// values: (lsb)BGR0(msb). For pointers with alpha channel the XOR mask
// consists of (lsb)BGRA(msb) 32 bit values.
//
// Guest driver must create the AND mask for pointers with alpha chan.,
// so if host does not support alpha, the pointer could be displayed as
// a normal color pointer. The AND mask can be constructed from alpha
// values. For example alpha value >= 0xf0 means bit 0 in the AND mask.
//
// The AND mask is 1 bpp bitmap with byte aligned scanlines. Size of AND
// mask, therefore, is and_len = (width + 7) / 8 * height. The padding
// bits at the end of any scanline are undefined.
//
// The XOR mask follows the AND mask on the next 4 bytes aligned offset:
// u8 *xor = and + (and_len + 3) & ~3
// Bytes in the gap between the AND and the XOR mask are undefined.
// XOR mask scanlines have no gap between them and size of XOR mask is:
// xor_len = width * 4 * height.
//
    pub data: [u8; ],
    pub __packed: },
// pointer is visible
pub const VBOX_MOUSE_POINTER_VISIBLE: c_uint = 0x0001;
// pointer has alpha channel
pub const VBOX_MOUSE_POINTER_ALPHA: c_uint = 0x0002;
// pointerData contains new pointer shape
pub const VBOX_MOUSE_POINTER_SHAPE: c_uint = 0x0004;
//
// The guest driver can handle asynch guest cmd completion by reading the
// command offset from io port.
//
pub const VBVACAPS_COMPLETEGCMD_BY_IOREAD: c_uint = 0x00000001;
// the guest driver can handle video adapter IRQs
pub const VBVACAPS_IRQ: c_uint = 0x00000002;
// The guest can read video mode hints sent via VBVA.
pub const VBVACAPS_VIDEO_MODE_HINTS: c_uint = 0x00000004;
// The guest can switch to a software cursor on demand.
pub const VBVACAPS_DISABLE_CURSOR_INTEGRATION: c_uint = 0x00000008;
// The guest does not depend on host handling the VBE registers.
pub const VBVACAPS_USE_VBVA_ONLY: c_uint = 0x00000010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_caps {
    pub rc: i32,
    pub caps: u32,
    pub __packed: },
// Query the most recent mode hints received from the host.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_query_mode_hints {
// The maximum number of screens to return hints for.
    pub hints_queried_count: u16,
// The size of the mode hint structures directly following this one.
    pub hint_structure_guest_size: u16,
// Return code for the operation. Initialise to VERR_NOT_SUPPORTED.
    pub rc: i32,
    pub __packed: },
//
// Structure in which a mode hint is returned. The guest allocates an array
// of these immediately after the vbva_query_mode_hints structure.
// To accommodate future extensions, the vbva_query_mode_hints structure
// specifies the size of the vbva_modehint structures allocated by the guest,
// and the host only fills out structure elements which fit into that size. The
// host should fill any unused members (e.g. dx, dy) or structure space on the
// end with ~0. The whole structure can legally be set to ~0 to skip a screen.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_modehint {
    pub magic: u32,
    pub cx: u32,
    pub cy: u32,
    pub /: *mut *mut u32 bpp; / Which has never been used...,
    pub display: u32,
    pub /: *mut *mut u32 dx; / X offset into the virtual frame-buffer.,
    pub /: *mut *mut u32 dy; / Y offset into the virtual frame-buffer.,
    pub /: *mut *mut u32 enabled; / Not flags. Add new members for new flags.,
    pub __packed: },
pub const VBVAMODEHINT_MAGIC: c_uint = 0x0801add9u;
//
// Report the rectangle relative to which absolute pointer events should be
// expressed. This information remains valid until the next VBVA resize event
// for any screen, at which time it is reset to the bounding rectangle of all
// virtual screens and must be re-set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_report_input_mapping {
    pub /: *mut *mut s32 x; / Upper left X co-ordinate relative to the first screen.,
    pub /: *mut *mut s32 y; / Upper left Y co-ordinate relative to the first screen.,
    pub /: *mut *mut u32 cx; / Rectangle width.,
    pub /: *mut *mut u32 cy; / Rectangle height.,
    pub __packed: },
//
// Report the guest cursor position and query the host one. The host may wish
// to use the guest information to re-position its own cursor (though this is
// currently unlikely).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbva_cursor_position {
    pub /: *mut *mut u32 report_position; / Are we reporting a position?,
    pub /: *mut *mut u32 x; / Guest cursor X position,
    pub /: *mut *mut u32 y; / Guest cursor Y position,
    pub __packed: },
