//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/media-entity.h
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
// Media entity
//
// Copyright (C) 2010 Nokia Corporation
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

// Enums used internally at the media controller to represent graphs
//
// enum media_gobj_type - type of a graph object
//
// @MEDIA_GRAPH_ENTITY:		Identify a media entity
// @MEDIA_GRAPH_PAD:		Identify a media pad
// @MEDIA_GRAPH_LINK:		Identify a media link
// @MEDIA_GRAPH_INTF_DEVNODE:	Identify a media Kernel API interface via
// a device node
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum media_gobj_type {
    MEDIA_GRAPH_ENTITY,
    MEDIA_GRAPH_PAD,
    MEDIA_GRAPH_LINK,
    MEDIA_GRAPH_INTF_DEVNODE,
}

pub const MEDIA_BITS_PER_TYPE: c_int = 8;

// Structs to represent the objects that belong to a media graph
//
// struct media_gobj - Define a graph object.
//
// @mdev:	Pointer to the struct &media_device that owns the object
// @id:		Non-zero object ID identifier. The ID should be unique
// inside a media_device, as it is composed by
// %MEDIA_BITS_PER_TYPE to store the type plus
// %MEDIA_BITS_PER_ID to store the ID
// @list:	List entry stored in one of the per-type mdev object lists
//
// All objects on the media graph should have this struct embedded
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_gobj {
    pub mdev: *mut media_device,
    pub id: u32,
    pub list: list_head,
}

pub const MEDIA_ENTITY_ENUM_MAX_DEPTH: c_int = 16;
//
// struct media_entity_enum - An enumeration of media entities.
//
// @bmap:	Bit map in which each bit represents one entity at struct
// media_entity->internal_idx.
// @idx_max:	Number of bits in bmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_entity_enum {
    pub bmap: *mut c_ulong,
    pub idx_max: c_int,
}

//
// struct media_graph - Media graph traversal state
//
// @stack:		Graph traversal stack; the stack contains information
// on the path the media entities to be walked and the
// links through which they were reached.
// @stack.entity:	pointer to &struct media_entity at the graph.
// @stack.link:		pointer to &struct list_head.
// @ent_enum:		Visited entities
// @top:		The top of the stack
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_graph {
    pub entity: *mut media_entity,
    pub link: *mut list_head,
    pub stack: [}; MEDIA_ENTITY_ENUM_MAX_DEPTH],
    pub ent_enum: media_entity_enum,
    pub top: c_int,
}

//
// struct media_pipeline - Media pipeline related information
//
// @allocated:		Media pipeline allocated and freed by the framework
// @mdev:		The media device the pipeline is part of
// @pads:		List of media_pipeline_pad
// @start_count:	Media pipeline start - stop count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_pipeline {
    pub allocated: bool,
    pub mdev: *mut media_device,
    pub pads: list_head,
    pub start_count: c_int,
}

//
// struct media_pipeline_pad - A pad part of a media pipeline
//
// @list:		Entry in the media_pad pads list
// @pipe:		The media_pipeline that the pad is part of
// @pad:		The media pad
//
// This structure associate a pad with a media pipeline. Instances of
// media_pipeline_pad are created by media_pipeline_start() when it builds the
// pipeline, and stored in the &media_pad.pads list. media_pipeline_stop()
// removes the entries from the list and deletes them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_pipeline_pad {
    pub list: list_head,
    pub pipe: *mut media_pipeline,
    pub pad: *mut media_pad,
}

//
// struct media_pipeline_pad_iter - Iterator for media_pipeline_for_each_pad
//
// @cursor: The current element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_pipeline_pad_iter {
    pub cursor: *mut list_head,
}

//
// struct media_pipeline_entity_iter - Iterator for media_pipeline_for_each_entity
//
// @ent_enum: The entity enumeration tracker
// @cursor: The current element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_pipeline_entity_iter {
    pub ent_enum: media_entity_enum,
    pub cursor: *mut list_head,
}

//
// struct media_link - A link object part of a media graph.
//
// @graph_obj:	Embedded structure containing the media object common data
// @list:	Linked list associated with an entity or an interface that
// owns the link.
// @gobj0:	Part of a union. Used to get the pointer for the first
// graph_object of the link.
// @source:	Part of a union. Used only if the first object (gobj0) is
// a pad. In that case, it represents the source pad.
// @intf:	Part of a union. Used only if the first object (gobj0) is
// an interface.
// @gobj1:	Part of a union. Used to get the pointer for the second
// graph_object of the link.
// @sink:	Part of a union. Used only if the second object (gobj1) is
// a pad. In that case, it represents the sink pad.
// @entity:	Part of a union. Used only if the second object (gobj1) is
// an entity.
// @reverse:	Pointer to the link for the reverse direction of a pad to pad
// link.
// @flags:	Link flags, as defined in uapi/media.h (MEDIA_LNK_FL_*)
// @is_backlink: Indicate if the link is a backlink.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_link {
    pub graph_obj: media_gobj,
    pub list: list_head,
    pub gobj0: *mut media_gobj,
    pub source: *mut media_pad,
    pub intf: *mut media_interface,
}

//
// enum media_pad_signal_type - type of the signal inside a media pad
//
// @PAD_SIGNAL_DEFAULT:
// Default signal. Use this when all inputs or all outputs are
// uniquely identified by the pad number.
// @PAD_SIGNAL_ANALOG:
// The pad contains an analog signal. It can be Radio Frequency,
// Intermediate Frequency, a baseband signal or sub-carriers.
// Tuner inputs, IF-PLL demodulators, composite and s-video signals
// should use it.
// @PAD_SIGNAL_DV:
// Contains a digital video signal, with can be a bitstream of samples
// taken from an analog TV video source. On such case, it usually
// contains the VBI data on it.
// @PAD_SIGNAL_AUDIO:
// Contains an Intermediate Frequency analog signal from an audio
// sub-carrier or an audio bitstream. IF signals are provided by tuners
// and consumed by	audio AM/FM decoders. Bitstream audio is provided by
// an audio decoder.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum media_pad_signal_type {
    PAD_SIGNAL_DEFAULT = 0,
    PAD_SIGNAL_ANALOG,
    PAD_SIGNAL_DV,
    PAD_SIGNAL_AUDIO,
}

//
// struct media_pad - A media pad graph object.
//
// @graph_obj:	Embedded structure containing the media object common data
// @entity:	Entity this pad belongs to
// @index:	Pad index in the entity pads array, numbered from 0 to n
// @num_links:	Number of links connected to this pad
// @sig_type:	Type of the signal inside a media pad
// @flags:	Pad flags, as defined in
// :ref:`include/uapi/linux/media.h <media_header>`
// (seek for ``MEDIA_PAD_FL_*``)
// @pipe:	Pipeline this pad belongs to. Use media_entity_pipeline() to
// access this field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_pad {
    pub /: *mut *mut media_gobj graph_obj; / must be first field in struct,
    pub entity: *mut media_entity,
    pub index: u16,
    pub num_links: u16,
    pub sig_type: media_pad_signal_type,
    pub flags: c_ulong,
//
// The fields below are private, and should only be accessed via
// appropriate functions.
//
    pub pipe: *mut media_pipeline,
}

//
// struct media_entity_operations - Media entity operations
// @get_fwnode_pad:	Return the pad number based on a fwnode endpoint or
// a negative value on error. This operation can be used
// to map a fwnode to a media pad number. Optional.
// @link_setup:		Notify the entity of link changes. The operation can
// return an error, in which case link setup will be
// cancelled. Optional.
// @link_validate:	Return whether a link is valid from the entity point of
// view. The media_pipeline_start() function
// validates all links by calling this operation. Optional.
// @has_pad_interdep:	Return whether two pads of the entity are
// interdependent. If two pads are interdependent they are
// part of the same pipeline and enabling one of the pads
// means that the other pad will become "locked" and
// doesn't allow configuration changes. pad0 and pad1 are
// guaranteed to not both be sinks or sources. Never call
// the .has_pad_interdep() operation directly, always use
// media_entity_has_pad_interdep().
// Optional: If the operation isn't implemented all pads
// will be considered as interdependent.
//
// .. note::
//
// Those these callbacks are called with struct &media_device.graph_mutex
// mutex held.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_entity_operations {
    pub endpoint): *mut fwnode_endpoint,
    pub flags): *const *const media_pad remote, u32,
    pub link): *mut *mut int (link_validate)(struct media_link,
    pub pad1): c_uint,
}

//
// enum media_entity_type - Media entity type
//
// @MEDIA_ENTITY_TYPE_BASE:
// The entity isn't embedded in another subsystem structure.
// @MEDIA_ENTITY_TYPE_VIDEO_DEVICE:
// The entity is embedded in a struct video_device instance.
// @MEDIA_ENTITY_TYPE_V4L2_SUBDEV:
// The entity is embedded in a struct v4l2_subdev instance.
//
// Media entity objects are often not instantiated directly, but the media
// entity structure is inherited by (through embedding) other subsystem-specific
// structures. The media entity type identifies the type of the subclass
// structure that implements a media entity instance.
//
// This allows runtime type identification of media entities and safe casting to
// the correct object type. For instance, a media entity structure instance
// embedded in a v4l2_subdev structure instance will have the type
// %MEDIA_ENTITY_TYPE_V4L2_SUBDEV and can safely be cast to a &v4l2_subdev
// structure using the container_of() macro.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum media_entity_type {
    MEDIA_ENTITY_TYPE_BASE,
    MEDIA_ENTITY_TYPE_VIDEO_DEVICE,
    MEDIA_ENTITY_TYPE_V4L2_SUBDEV,
}

//
// struct media_entity - A media entity graph object.
//
// @graph_obj:	Embedded structure containing the media object common data.
// @name:	Entity name.
// @obj_type:	Type of the object that implements the media_entity.
// @function:	Entity main function, as defined in
// :ref:`include/uapi/linux/media.h <media_header>`
// (seek for ``MEDIA_ENT_F_*``)
// @flags:	Entity flags, as defined in
// :ref:`include/uapi/linux/media.h <media_header>`
// (seek for ``MEDIA_ENT_FL_*``)
// @num_pads:	Number of sink and source pads.
// @num_links:	Total number of links, forward and back, enabled and disabled.
// @num_backlinks: Number of backlinks
// @internal_idx: An unique internal entity specific number. The numbers are
// re-used if entities are unregistered or registered again.
// @pads:	Pads array with the size defined by @num_pads.
// @links:	List of data links.
// @ops:	Entity operations.
// @use_count:	Use count for the entity.
// @info:	Union with devnode information.  Kept just for backward
// compatibility.
// @info.dev:	Contains device major and minor info.
// @info.dev.major: device node major, if the device is a devnode.
// @info.dev.minor: device node minor, if the device is a devnode.
//
// .. note::
//
// The @use_count reference count must never be negative, but is a signed
// integer on purpose: a simple ``WARN_ON(<0)`` check can be used to detect
// reference count bugs that would make it negative.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_entity {
    pub /: *mut *mut media_gobj graph_obj; / must be first field in struct,
    pub name: *const c_char,
    pub obj_type: media_entity_type,
    pub function: u32,
    pub flags: c_ulong,
    pub num_pads: u16,
    pub num_links: u16,
    pub num_backlinks: u16,
    pub internal_idx: c_int,
    pub pads: *mut media_pad,
    pub links: list_head,
    pub ops: *const media_entity_operations,
    pub use_count: c_int,
    pub major: u32,
    pub minor: u32,
    pub dev: },
    pub info: },
}

//
// media_entity_for_each_pad - Iterate on all pads in an entity
// @entity: The entity the pads belong to
// @iter: The iterator pad
//
// Iterate on all pads in a media entity.
//

//
// struct media_interface - A media interface graph object.
//
// @graph_obj:		embedded graph object
// @links:		List of links pointing to graph entities
// @type:		Type of the interface as defined in
// :ref:`include/uapi/linux/media.h <media_header>`
// (seek for ``MEDIA_INTF_T_*``)
// @flags:		Interface flags as defined in
// :ref:`include/uapi/linux/media.h <media_header>`
// (seek for ``MEDIA_INTF_FL_*``)
//
// .. note::
//
// Currently, no flags for &media_interface is defined.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_interface {
    pub graph_obj: media_gobj,
    pub links: list_head,
    pub type: u32,
    pub flags: u32,
}

//
// struct media_intf_devnode - A media interface via a device node.
//
// @intf:	embedded interface object
// @major:	Major number of a device node
// @minor:	Minor number of a device node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_intf_devnode {
    pub intf: media_interface,
// Should match the fields at media_v2_intf_devnode
    pub major: u32,
    pub minor: u32,
}

//
// media_entity_id() - return the media entity graph object id
//
// @entity:	pointer to &media_entity
//
// media_type() - return the media object type
//
// @gobj:	Pointer to the struct &media_gobj graph object
//
// media_id() - return the media object ID
//
// @gobj:	Pointer to the struct &media_gobj graph object
//
// media_gobj_gen_id() - encapsulates type and ID on at the object ID
//
// @type:	object type as define at enum &media_gobj_type.
// @local_id:	next ID, from struct &media_device.id.
//
// is_media_entity_v4l2_video_device() - Check if the entity is a video_device
// @entity:	pointer to entity
//
// Return: %true if the entity is an instance of a video_device object and can
// safely be cast to a struct video_device using the container_of() macro, or
// %false otherwise.
//
// is_media_entity_v4l2_subdev() - Check if the entity is a v4l2_subdev
// @entity:	pointer to entity
//
// Return: %true if the entity is an instance of a &v4l2_subdev object and can
// safely be cast to a struct &v4l2_subdev using the container_of() macro, or
// %false otherwise.
//
// media_entity_enum_init - Initialise an entity enumeration
//
// @ent_enum: Entity enumeration to be initialised
// @mdev: The related media device
//
// Return: zero on success or a negative error code.
//
// media_entity_enum_cleanup - Release resources of an entity enumeration
//
// @ent_enum: Entity enumeration to be released
//
extern "C" {
    pub fn media_entity_enum_cleanup(ent_enum: *mut media_entity_enum);
}
//
// media_entity_enum_zero - Clear the entire enum
//
// @ent_enum: Entity enumeration to be cleared
//
// media_entity_enum_set - Mark a single entity in the enum
//
// @ent_enum: Entity enumeration
// @entity: Entity to be marked
//
// media_entity_enum_clear - Unmark a single entity in the enum
//
// @ent_enum: Entity enumeration
// @entity: Entity to be unmarked
//
// media_entity_enum_test - Test whether the entity is marked
//
// @ent_enum: Entity enumeration
// @entity: Entity to be tested
//
// Returns %true if the entity was marked.
//
extern "C" {
    pub fn test_bit(_arg: entity->internal_idx, _arg: ent_enum->bmap) -> return;
}
//
// media_entity_enum_test_and_set - Test whether the entity is marked,
// and mark it
//
// @ent_enum: Entity enumeration
// @entity: Entity to be tested
//
// Returns %true if the entity was marked, and mark it before doing so.
//
extern "C" {
    pub fn __test_and_set_bit(_arg: entity->internal_idx, _arg: ent_enum->bmap) -> return;
}
//
// media_entity_enum_empty - Test whether the entire enum is empty
//
// @ent_enum: Entity enumeration
//
// Return: %true if the entity was empty.
//
extern "C" {
    pub fn bitmap_empty(_arg: ent_enum->bmap, _arg: ent_enum->idx_max) -> return;
}
//
// media_entity_enum_intersects - Test whether two enums intersect
//
// @ent_enum1: First entity enumeration
// @ent_enum2: Second entity enumeration
//
// Return: %true if entity enumerations @ent_enum1 and @ent_enum2 intersect,
// otherwise %false.
//
// gobj_to_entity - returns the struct &media_entity pointer from the
// @gobj contained on it.
//
// @gobj: Pointer to the struct &media_gobj graph object
//

//
// gobj_to_pad - returns the struct &media_pad pointer from the
// @gobj contained on it.
//
// @gobj: Pointer to the struct &media_gobj graph object
//

//
// gobj_to_link - returns the struct &media_link pointer from the
// @gobj contained on it.
//
// @gobj: Pointer to the struct &media_gobj graph object
//

//
// gobj_to_intf - returns the struct &media_interface pointer from the
// @gobj contained on it.
//
// @gobj: Pointer to the struct &media_gobj graph object
//

//
// intf_to_devnode - returns the struct media_intf_devnode pointer from the
// @intf contained on it.
//
// @intf: Pointer to struct &media_intf_devnode
//

//
// media_gobj_create - Initialize a graph object
//
// @mdev:	Pointer to the &media_device that contains the object
// @type:	Type of the object
// @gobj:	Pointer to the struct &media_gobj graph object
//
// This routine initializes the embedded struct &media_gobj inside a
// media graph object. It is called automatically if ``media_*_create``
// function calls are used. However, if the object (entity, link, pad,
// interface) is embedded on some other object, this function should be
// called before registering the object at the media controller.
//
// media_gobj_destroy - Stop using a graph object on a media device
//
// @gobj:	Pointer to the struct &media_gobj graph object
//
// This should be called by all routines like media_device_unregister()
// that remove/destroy media graph objects.
//
extern "C" {
    pub fn media_gobj_destroy(gobj: *mut media_gobj);
}
//
// media_entity_pads_init() - Initialize the entity pads
//
// @entity:	entity where the pads belong
// @num_pads:	total number of sink and source pads
// @pads:	Array of @num_pads pads.
//
// The pads array is managed by the entity driver and passed to
// media_entity_pads_init() where its pointer will be stored in the
// &media_entity structure.
//
// If no pads are needed, drivers could either directly fill
// &media_entity->num_pads with 0 and &media_entity->pads with %NULL or call
// this function that will do the same.
//
// As the number of pads is known in advance, the pads array is not allocated
// dynamically but is managed by the entity driver. Most drivers will embed the
// pads array in a driver-specific structure, avoiding dynamic allocation.
//
// Drivers must set the direction of every pad in the pads array before calling
// media_entity_pads_init(). The function will initialize the other pads fields.
//
// media_entity_cleanup() - free resources associated with an entity
//
// @entity:	entity where the pads belong
//
// This function must be called during the cleanup phase after unregistering
// the entity (currently, it does nothing).
//
// Calling media_entity_cleanup() on a media_entity whose memory has been
// zeroed but that has not been initialized with media_entity_pads_init() is
// valid and is a no-op.
//
// media_get_pad_index() - retrieves a pad index from an entity
//
// @entity:	entity where the pads belong
// @pad_type:	the type of the pad, one of MEDIA_PAD_FL_* pad types
// @sig_type:	type of signal of the pad to be search
//
// This helper function finds the first pad index inside an entity that
// satisfies both @is_sink and @sig_type conditions.
//
// Return:
//
// On success, return the pad number. If the pad was not found or the media
// entity is a NULL pointer, return -EINVAL.
//
// media_create_pad_link() - creates a link between two entities.
//
// @source:	pointer to &media_entity of the source pad.
// @source_pad:	number of the source pad in the pads array
// @sink:	pointer to &media_entity of the sink pad.
// @sink_pad:	number of the sink pad in the pads array.
// @flags:	Link flags, as defined in
// :ref:`include/uapi/linux/media.h <media_header>`
// ( seek for ``MEDIA_LNK_FL_*``)
//
// Valid values for flags:
//
// %MEDIA_LNK_FL_ENABLED
// Indicates that the link is enabled and can be used to transfer media data.
// When two or more links target a sink pad, only one of them can be
// enabled at a time.
//
// %MEDIA_LNK_FL_IMMUTABLE
// Indicates that the link enabled state can't be modified at runtime. If
// %MEDIA_LNK_FL_IMMUTABLE is set, then %MEDIA_LNK_FL_ENABLED must also be
// set, since an immutable link is always enabled.
//
// .. note::
//
// Before calling this function, media_entity_pads_init() and
// media_device_register_entity() should be called previously for both ends.
//
// media_create_pad_links() - creates a link between two entities.
//
// @mdev: Pointer to the media_device that contains the object
// @source_function: Function of the source entities. Used only if @source is
// NULL.
// @source: pointer to &media_entity of the source pad. If NULL, it will use
// all entities that matches the @sink_function.
// @source_pad: number of the source pad in the pads array
// @sink_function: Function of the sink entities. Used only if @sink is NULL.
// @sink: pointer to &media_entity of the sink pad. If NULL, it will use
// all entities that matches the @sink_function.
// @sink_pad: number of the sink pad in the pads array.
// @flags: Link flags, as defined in include/uapi/linux/media.h.
// @allow_both_undefined: if %true, then both @source and @sink can be NULL.
// In such case, it will create a crossbar between all entities that
// matches @source_function to all entities that matches @sink_function.
// If %false, it will return 0 and won't create any link if both @source
// and @sink are NULL.
//
// Valid values for flags:
//
// A %MEDIA_LNK_FL_ENABLED flag indicates that the link is enabled and can be
// used to transfer media data. If multiple links are created and this
// flag is passed as an argument, only the first created link will have
// this flag.
//
// A %MEDIA_LNK_FL_IMMUTABLE flag indicates that the link enabled state can't
// be modified at runtime. If %MEDIA_LNK_FL_IMMUTABLE is set, then
// %MEDIA_LNK_FL_ENABLED must also be set since an immutable link is
// always enabled.
//
// It is common for some devices to have multiple source and/or sink entities
// of the same type that should be linked. While media_create_pad_link()
// creates link by link, this function is meant to allow 1:n, n:1 and even
// cross-bar (n:n) links.
//
// .. note::
//
// Before calling this function, media_entity_pads_init() and
// media_device_register_entity() should be called previously for the
// entities to be linked.
//
extern "C" {
    pub fn __media_entity_remove_links(entity: *mut media_entity);
}
//
// media_entity_remove_links() - remove all links associated with an entity
//
// @entity:	pointer to &media_entity
//
// .. note::
//
// This is called automatically when an entity is unregistered via
// media_device_register_entity().
//
extern "C" {
    pub fn media_entity_remove_links(entity: *mut media_entity);
}
//
// __media_entity_setup_link - Configure a media link without locking
// @link: The link being configured
// @flags: Link configuration flags
//
// The bulk of link setup is handled by the two entities connected through the
// link. This function notifies both entities of the link configuration change.
//
// If the link is immutable or if the current and new configuration are
// identical, return immediately.
//
// The user is expected to hold link->source->parent->mutex. If not,
// media_entity_setup_link() should be used instead.
//
extern "C" {
    pub fn __media_entity_setup_link(link: *mut media_link, flags: u32) -> c_int;
}
//
// media_entity_setup_link() - changes the link flags properties in runtime
//
// @link:	pointer to &media_link
// @flags:	the requested new link flags
//
// The only configurable property is the %MEDIA_LNK_FL_ENABLED link flag
// to enable/disable a link. Links marked with the
// %MEDIA_LNK_FL_IMMUTABLE link flag can not be enabled or disabled.
//
// When a link is enabled or disabled, the media framework calls the
// link_setup operation for the two entities at the source and sink of the
// link, in that order. If the second link_setup call fails, another
// link_setup call is made on the first entity to restore the original link
// flags.
//
// Media device drivers can be notified of link setup operations by setting the
// &media_device.link_notify pointer to a callback function. If provided, the
// notification callback will be called before enabling and after disabling
// links.
//
// Entity drivers must implement the link_setup operation if any of their links
// is non-immutable. The operation must either configure the hardware or store
// the configuration information to be applied later.
//
// Link configuration must not have any side effect on other links. If an
// enabled link at a sink pad prevents another link at the same pad from
// being enabled, the link_setup operation must return %-EBUSY and can't
// implicitly disable the first enabled link.
//
// .. note::
//
// The valid values of the flags for the link is the same as described
// on media_create_pad_link(), for pad to pad links or the same as described
// on media_create_intf_link(), for interface to entity links.
//
extern "C" {
    pub fn media_entity_setup_link(link: *mut media_link, flags: u32) -> c_int;
}
//
// media_entity_find_link - Find a link between two pads
// @source: Source pad
// @sink: Sink pad
//
// Return: returns a pointer to the link between the two entities. If no
// such link exists, return %NULL.
//
// media_pad_remote_pad_first - Find the first pad at the remote end of a link
// @pad: Pad at the local end of the link
//
// Search for a remote pad connected to the given pad by iterating over all
// links originating or terminating at that pad until an enabled link is found.
//
// Return: returns a pointer to the pad at the remote end of the first found
// enabled link, or %NULL if no enabled link has been found.
//
// media_pad_remote_pad_unique - Find a remote pad connected to a pad
// @pad: The pad
//
// Search for and return a remote pad connected to @pad through an enabled
// link. If multiple (or no) remote pads are found, an error is returned.
//
// The uniqueness constraint makes this helper function suitable for entities
// that support a single active source at a time on a given pad.
//
// Return: A pointer to the remote pad, or one of the following error pointers
// if an error occurs:
//
// * -ENOTUNIQ - Multiple links are enabled
// * -ENOLINK - No connected pad found
//
// media_entity_remote_pad_unique - Find a remote pad connected to an entity
// @entity: The entity
// @type: The type of pad to find (MEDIA_PAD_FL_SINK or MEDIA_PAD_FL_SOURCE)
//
// Search for and return a remote pad of @type connected to @entity through an
// enabled link. If multiple (or no) remote pads match these criteria, an error
// is returned.
//
// The uniqueness constraint makes this helper function suitable for entities
// that support a single active source or sink at a time.
//
// Return: A pointer to the remote pad, or one of the following error pointers
// if an error occurs:
//
// * -ENOTUNIQ - Multiple links are enabled
// * -ENOLINK - No connected pad found
//
// media_entity_remote_source_pad_unique - Find a remote source pad connected to
// an entity
// @entity: The entity
//
// Search for and return a remote source pad connected to @entity through an
// enabled link. If multiple (or no) remote pads match these criteria, an error
// is returned.
//
// The uniqueness constraint makes this helper function suitable for entities
// that support a single active source at a time.
//
// Return: A pointer to the remote pad, or one of the following error pointers
// if an error occurs:
//
// * -ENOTUNIQ - Multiple links are enabled
// * -ENOLINK - No connected pad found
//
extern "C" {
    pub fn media_entity_remote_pad_unique(_arg: entity, _arg: MEDIA_PAD_FL_SOURCE) -> return;
}
//
// media_pad_is_streaming - Test if a pad is part of a streaming pipeline
// @pad: The pad
//
// Return: True if the pad is part of a pipeline started with the
// media_pipeline_start() function, false otherwise.
//
// media_entity_is_streaming - Test if an entity is part of a streaming pipeline
// @entity: The entity
//
// Return: True if the entity is part of a pipeline started with the
// media_pipeline_start() function, false otherwise.
//
// media_entity_pipeline - Get the media pipeline an entity is part of
// @entity: The entity
//
// DEPRECATED: use media_pad_pipeline() instead.
//
// This function returns the media pipeline that an entity has been associated
// with when constructing the pipeline with media_pipeline_start(). The pointer
// remains valid until media_pipeline_stop() is called.
//
// In general, entities can be part of multiple pipelines, when carrying
// multiple streams (either on different pads, or on the same pad using
// multiplexed streams). This function is to be used only for entities that
// do not support multiple pipelines.
//
// Return: The media_pipeline the entity is part of, or NULL if the entity is
// not part of any pipeline.
//
// media_pad_pipeline - Get the media pipeline a pad is part of
// @pad: The pad
//
// This function returns the media pipeline that a pad has been associated
// with when constructing the pipeline with media_pipeline_start(). The pointer
// remains valid until media_pipeline_stop() is called.
//
// Return: The media_pipeline the pad is part of, or NULL if the pad is
// not part of any pipeline.
//
// media_entity_get_fwnode_pad - Get pad number from fwnode
//
// @entity: The entity
// @fwnode: Pointer to the fwnode_handle which should be used to find the pad
// @direction_flags: Expected direction of the pad, as defined in
// :ref:`include/uapi/linux/media.h <media_header>`
// (seek for ``MEDIA_PAD_FL_*``)
//
// This function can be used to resolve the media pad number from
// a fwnode. This is useful for devices which use more complex
// mappings of media pads.
//
// If the entity does not implement the get_fwnode_pad() operation
// then this function searches the entity for the first pad that
// matches the @direction_flags.
//
// Return: returns the pad number on success or a negative error code.
//
// media_graph_walk_init - Allocate resources used by graph walk.
//
// @graph: Media graph structure that will be used to walk the graph
// @mdev: Pointer to the &media_device that contains the object
//
// This function is deprecated, use media_pipeline_for_each_pad() instead.
//
// The caller is required to hold the media_device graph_mutex during the graph
// walk until the graph state is released.
//
// Returns zero on success or a negative error code otherwise.
//
// media_graph_walk_cleanup - Release resources used by graph walk.
//
// @graph: Media graph structure that will be used to walk the graph
//
// This function is deprecated, use media_pipeline_for_each_pad() instead.
//
extern "C" {
    pub fn media_graph_walk_cleanup(graph: *mut media_graph);
}
//
// media_graph_walk_start - Start walking the media graph at a
// given entity
//
// @graph: Media graph structure that will be used to walk the graph
// @entity: Starting entity
//
// This function is deprecated, use media_pipeline_for_each_pad() instead.
//
// Before using this function, media_graph_walk_init() must be
// used to allocate resources used for walking the graph. This
// function initializes the graph traversal structure to walk the
// entities graph starting at the given entity. The traversal
// structure must not be modified by the caller during graph
// traversal. After the graph walk, the resources must be released
// using media_graph_walk_cleanup().
//
// media_graph_walk_next - Get the next entity in the graph
// @graph: Media graph structure
//
// This function is deprecated, use media_pipeline_for_each_pad() instead.
//
// Perform a depth-first traversal of the given media entities graph.
//
// The graph structure must have been previously initialized with a call to
// media_graph_walk_start().
//
// Return: returns the next entity in the graph or %NULL if the whole graph
// have been traversed.
//
// media_pipeline_start - Mark a pipeline as streaming
// @origin: Starting pad
// @pipe: Media pipeline to be assigned to all pads in the pipeline.
//
// Mark all pads connected to pad @origin through enabled links, either
// directly or indirectly, as streaming. The given pipeline object is assigned
// to every pad in the pipeline and stored in the media_pad pipe field.
//
// Calls to this function can be nested, in which case the same number of
// media_pipeline_stop() calls will be required to stop streaming. The
// pipeline pointer must be identical for all nested calls to
// media_pipeline_start().
//
// __media_pipeline_start - Mark a pipeline as streaming
//
// @origin: Starting pad
// @pipe: Media pipeline to be assigned to all pads in the pipeline.
//
// ..note:: This is the non-locking version of media_pipeline_start()
//
// media_pipeline_stop - Mark a pipeline as not streaming
// @pad: Starting pad
//
// Mark all pads connected to a given pad through enabled links, either
// directly or indirectly, as not streaming. The media_pad pipe field is
// reset to %NULL.
//
// If multiple calls to media_pipeline_start() have been made, the same
// number of calls to this function are required to mark the pipeline as not
// streaming.
//
extern "C" {
    pub fn media_pipeline_stop(pad: *mut media_pad);
}
//
// __media_pipeline_stop - Mark a pipeline as not streaming
//
// @pad: Starting pad
//
// .. note:: This is the non-locking version of media_pipeline_stop()
//
extern "C" {
    pub fn __media_pipeline_stop(pad: *mut media_pad);
}
//
// media_pipeline_for_each_pad - Iterate on all pads in a media pipeline
// @pipe: The pipeline
// @iter: The iterator (struct media_pipeline_pad_iter)
// @pad: The iterator pad
//
// Iterate on all pads in a media pipeline. This is only valid after the
// pipeline has been built with media_pipeline_start() and before it gets
// destroyed with media_pipeline_stop().
//

//
// media_pipeline_entity_iter_init - Initialize a pipeline entity iterator
// @pipe: The pipeline
// @iter: The iterator
//
// This function must be called to initialize the iterator before using it in a
// media_pipeline_for_each_entity() loop. The iterator must be destroyed by a
// call to media_pipeline_entity_iter_cleanup after the loop (including in code
// paths that break from the loop).
//
// The same iterator can be used in multiple consecutive loops without being
// destroyed and reinitialized.
//
// Return: 0 on success or a negative error code otherwise.
//
// media_pipeline_entity_iter_cleanup - Destroy a pipeline entity iterator
// @iter: The iterator
//
// This function must be called to destroy iterators initialized with
// media_pipeline_entity_iter_init().
//
extern "C" {
    pub fn media_pipeline_entity_iter_cleanup(iter: *mut media_pipeline_entity_iter);
}
//
// media_pipeline_for_each_entity - Iterate on all entities in a media pipeline
// @pipe: The pipeline
// @iter: The iterator (struct media_pipeline_entity_iter)
// @entity: The iterator entity
//
// Iterate on all entities in a media pipeline. This is only valid after the
// pipeline has been built with media_pipeline_start() and before it gets
// destroyed with media_pipeline_stop(). The iterator must be initialized with
// media_pipeline_entity_iter_init() before iteration, and destroyed with
// media_pipeline_entity_iter_cleanup() after (including in code paths that
// break from the loop).
//

//
// media_pipeline_alloc_start - Mark a pipeline as streaming
// @pad: Starting pad
//
// media_pipeline_alloc_start() is similar to media_pipeline_start() but instead
// of working on a given pipeline the function will use an existing pipeline if
// the pad is already part of a pipeline, or allocate a new pipeline.
//
// Calls to media_pipeline_alloc_start() must be matched with
// media_pipeline_stop().
//
extern "C" {
    pub fn media_pipeline_alloc_start(pad: *mut media_pad) -> __must_check int;
}
//
// media_devnode_create() - creates and initializes a device node interface
//
// @mdev:	pointer to struct &media_device
// @type:	type of the interface, as given by
// :ref:`include/uapi/linux/media.h <media_header>`
// ( seek for ``MEDIA_INTF_T_*``) macros.
// @flags:	Interface flags, as defined in
// :ref:`include/uapi/linux/media.h <media_header>`
// ( seek for ``MEDIA_INTF_FL_*``)
// @major:	Device node major number.
// @minor:	Device node minor number.
//
// Return: if succeeded, returns a pointer to the newly allocated
// &media_intf_devnode pointer.
//
// .. note::
//
// Currently, no flags for &media_interface is defined.
//
// media_devnode_remove() - removes a device node interface
//
// @devnode:	pointer to &media_intf_devnode to be freed.
//
// When a device node interface is removed, all links to it are automatically
// removed.
//
extern "C" {
    pub fn media_devnode_remove(devnode: *mut media_intf_devnode);
}
//
// media_create_intf_link() - creates a link between an entity and an interface
//
// @entity:	pointer to %media_entity
// @intf:	pointer to %media_interface
// @flags:	Link flags, as defined in
// :ref:`include/uapi/linux/media.h <media_header>`
// ( seek for ``MEDIA_LNK_FL_*``)
//
// Valid values for flags:
//
// %MEDIA_LNK_FL_ENABLED
// Indicates that the interface is connected to the entity hardware.
// That's the default value for interfaces. An interface may be disabled if
// the hardware is busy due to the usage of some other interface that it is
// currently controlling the hardware.
//
// A typical example is an hybrid TV device that handle only one type of
// stream on a given time. So, when the digital TV is streaming,
// the V4L2 interfaces won't be enabled, as such device is not able to
// also stream analog TV or radio.
//
// .. note::
//
// Before calling this function, media_devnode_create() should be called for
// the interface and media_device_register_entity() should be called for the
// interface that will be part of the link.
//
// __media_remove_intf_link() - remove a single interface link
//
// @link:	pointer to &media_link.
//
// .. note:: This is an unlocked version of media_remove_intf_link()
//
extern "C" {
    pub fn __media_remove_intf_link(link: *mut media_link);
}
//
// media_remove_intf_link() - remove a single interface link
//
// @link:	pointer to &media_link.
//
// .. note:: Prefer to use this one, instead of __media_remove_intf_link()
//
extern "C" {
    pub fn media_remove_intf_link(link: *mut media_link);
}
//
// __media_remove_intf_links() - remove all links associated with an interface
//
// @intf:	pointer to &media_interface
//
// .. note:: This is an unlocked version of media_remove_intf_links().
//
extern "C" {
    pub fn __media_remove_intf_links(intf: *mut media_interface);
}
//
// media_remove_intf_links() - remove all links associated with an interface
//
// @intf:	pointer to &media_interface
//
// .. note::
//
// #) This is called automatically when an entity is unregistered via
// media_device_register_entity() and by media_devnode_remove().
//
// #) Prefer to use this one, instead of __media_remove_intf_links().
//
extern "C" {
    pub fn media_remove_intf_links(intf: *mut media_interface);
}
//
// media_entity_call - Calls a struct media_entity_operations operation on
// an entity
//
// @entity: entity where the @operation will be called
// @operation: type of the operation. Should be the name of a member of
// struct &media_entity_operations.
// @args: arguments for the operation.
//
// This helper function will check if @operation is not %NULL. On such case,
// it will issue a call to @operation\(@entity, @args\).
//

//
// media_create_ancillary_link() - create an ancillary link between two
// instances of &media_entity
//
// @primary:	pointer to the primary &media_entity
// @ancillary:	pointer to the ancillary &media_entity
//
// Create an ancillary link between two entities, indicating that they
// represent two connected pieces of hardware that form a single logical unit.
// A typical example is a camera lens controller being linked to the sensor that
// it is supporting.
//
// The function sets both MEDIA_LNK_FL_ENABLED and MEDIA_LNK_FL_IMMUTABLE for
// the new link.
//
// __media_entity_next_link() - Iterate through a &media_entity's links
//
// @entity:	pointer to the &media_entity
// @link:	pointer to a &media_link to hold the iterated values
// @link_type:	one of the MEDIA_LNK_FL_LINK_TYPE flags
//
// Return the next link against an entity matching a specific link type. This
// allows iteration through an entity's links whilst guaranteeing all of the
// returned links are of the given type.
//
// for_each_media_entity_data_link() - Iterate through an entity's data links
//
// @entity:	pointer to the &media_entity
// @link:	pointer to a &media_link to hold the iterated values
//
// Iterate over a &media_entity's data links
//

