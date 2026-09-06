//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vkms/vkms_config.h
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
// struct vkms_config - General configuration for VKMS driver
//
// @dev_name: Name of the device
// @planes: List of planes configured for the device
// @crtcs: List of CRTCs configured for the device
// @encoders: List of encoders configured for the device
// @connectors: List of connectors configured for the device
// @dev: Used to store the current VKMS device. Only set when the device is instantiated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_config {
    pub dev_name: *const c_char,
    pub planes: list_head,
    pub crtcs: list_head,
    pub encoders: list_head,
    pub connectors: list_head,
    pub dev: *mut vkms_device,
}

//
// struct vkms_config_plane
//
// @link: Link to the others planes in vkms_config
// @config: The vkms_config this plane belongs to
// @type: Type of the plane. The creator of configuration needs to ensures that
// at least one primary plane is present.
// @possible_crtcs: Array of CRTCs that can be used with this plane
// @plane: Internal usage. This pointer should never be considered as valid.
// It can be used to store a temporary reference to a VKMS plane during
// device creation. This pointer is not managed by the configuration and
// must be managed by other means.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_config_plane {
    pub link: list_head,
    pub config: *mut vkms_config,
    pub type: drm_plane_type,
    pub possible_crtcs: xarray,
    pub default_pipeline: bool,
// Internal usage
    pub plane: *mut vkms_plane,
}

//
// struct vkms_config_crtc
//
// @link: Link to the others CRTCs in vkms_config
// @config: The vkms_config this CRTC belongs to
// @writeback: If true, a writeback buffer can be attached to the CRTC
// @crtc: Internal usage. This pointer should never be considered as valid.
// It can be used to store a temporary reference to a VKMS CRTC during
// device creation. This pointer is not managed by the configuration and
// must be managed by other means.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_config_crtc {
    pub link: list_head,
    pub config: *mut vkms_config,
    pub writeback: bool,
// Internal usage
    pub crtc: *mut vkms_output,
}

//
// struct vkms_config_encoder
//
// @link: Link to the others encoders in vkms_config
// @config: The vkms_config this CRTC belongs to
// @possible_crtcs: Array of CRTCs that can be used with this encoder
// @encoder: Internal usage. This pointer should never be considered as valid.
// It can be used to store a temporary reference to a VKMS encoder
// during device creation. This pointer is not managed by the
// configuration and must be managed by other means.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_config_encoder {
    pub link: list_head,
    pub config: *mut vkms_config,
    pub possible_crtcs: xarray,
// Internal usage
    pub encoder: *mut drm_encoder,
}

//
// struct vkms_config_connector
//
// @link: Link to the others connector in vkms_config
// @config: The vkms_config this connector belongs to
// @status: Status (connected, disconnected...) of the connector
// @possible_encoders: Array of encoders that can be used with this connector
// @connector: Internal usage. This pointer should never be considered as valid.
// It can be used to store a temporary reference to a VKMS connector
// during device creation. This pointer is not managed by the
// configuration and must be managed by other means.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vkms_config_connector {
    pub link: list_head,
    pub config: *mut vkms_config,
    pub status: drm_connector_status,
    pub possible_encoders: xarray,
// Internal usage
    pub connector: *mut vkms_connector,
}

//
// vkms_config_for_each_plane - Iterate over the vkms_config planes
// @config: &struct vkms_config pointer
// @plane_cfg: &struct vkms_config_plane pointer used as cursor
//

//
// vkms_config_for_each_crtc - Iterate over the vkms_config CRTCs
// @config: &struct vkms_config pointer
// @crtc_cfg: &struct vkms_config_crtc pointer used as cursor
//

//
// vkms_config_for_each_encoder - Iterate over the vkms_config encoders
// @config: &struct vkms_config pointer
// @encoder_cfg: &struct vkms_config_encoder pointer used as cursor
//

//
// vkms_config_for_each_connector - Iterate over the vkms_config connectors
// @config: &struct vkms_config pointer
// @connector_cfg: &struct vkms_config_connector pointer used as cursor
//

//
// vkms_config_plane_for_each_possible_crtc - Iterate over the vkms_config_plane
// possible CRTCs
// @plane_cfg: &struct vkms_config_plane pointer
// @idx: Index of the cursor
// @possible_crtc: &struct vkms_config_crtc pointer used as cursor
//

//
// vkms_config_encoder_for_each_possible_crtc - Iterate over the
// vkms_config_encoder possible CRTCs
// @encoder_cfg: &struct vkms_config_encoder pointer
// @idx: Index of the cursor
// @possible_crtc: &struct vkms_config_crtc pointer used as cursor
//

//
// vkms_config_connector_for_each_possible_encoder - Iterate over the
// vkms_config_connector possible encoders
// @connector_cfg: &struct vkms_config_connector pointer
// @idx: Index of the cursor
// @possible_encoder: &struct vkms_config_encoder pointer used as cursor
//

//
// vkms_config_create() - Create a new VKMS configuration
// @dev_name: Name of the device
//
// Returns:
// The new vkms_config or an error. Call vkms_config_destroy() to free the
// returned configuration.
//
// vkms_config_default_create() - Create the configuration for the default device
// @enable_cursor: Create or not a cursor plane
// @enable_writeback: Create or not a writeback connector
// @enable_overlay: Create or not overlay planes
//
// Returns:
// The default vkms_config or an error. Call vkms_config_destroy() to free the
// returned configuration.
//
// vkms_config_destroy() - Free a VKMS configuration
// @config: vkms_config to free
//
extern "C" {
    pub fn vkms_config_destroy(config: *mut vkms_config);
}
//
// vkms_config_get_device_name() - Return the name of the device
// @config: Configuration to get the device name from
//
// Returns:
// The device name. Only valid while @config is valid.
//
// vkms_config_get_num_crtcs() - Return the number of CRTCs in the configuration
// @config: Configuration to get the number of CRTCs from
//
extern "C" {
    pub fn list_count_nodes(_arg: &config->crtcs) -> return;
}
//
// vkms_config_is_valid() - Validate a configuration
// @config: Configuration to validate
//
// Returns:
// Whether the configuration is valid or not.
// For example, a configuration without primary planes is not valid.
//
extern "C" {
    pub fn vkms_config_is_valid(config: *const vkms_config) -> bool;
}
//
// vkms_config_register_debugfs() - Register a debugfs file to show the device's
// configuration
// @vkms_device: Device to register
//
extern "C" {
    pub fn vkms_config_register_debugfs(vkms_device: *mut vkms_device);
}
//
// vkms_config_create_plane() - Add a new plane configuration
// @config: Configuration to add the plane to
//
// Returns:
// The new plane configuration or an error. Call vkms_config_destroy_plane() to
// free the returned plane configuration.
//
// vkms_config_destroy_plane() - Remove and free a plane configuration
// @plane_cfg: Plane configuration to destroy
//
extern "C" {
    pub fn vkms_config_destroy_plane(plane_cfg: *mut vkms_config_plane);
}
//
// vkms_config_plane_type() - Return the plane type
// @plane_cfg: Plane to get the type from
//
// vkms_config_plane_set_type() - Set the plane type
// @plane_cfg: Plane to set the type to
// @type: New plane type
//
// vkms_config_plane_get_default_pipeline() - Return if the plane will
// be created with the default pipeline
// @plane_cfg: Plane to get the information from
//
// vkms_config_plane_set_default_pipeline() - Set if the plane will
// be created with the default pipeline
// @plane_cfg: Plane to configure the pipeline
// @default_pipeline: New default pipeline value
//
// vkms_config_plane_attach_crtc - Attach a plane to a CRTC
// @plane_cfg: Plane to attach
// @crtc_cfg: CRTC to attach @plane_cfg to
//
// vkms_config_plane_detach_crtc - Detach a plane from a CRTC
// @plane_cfg: Plane to detach
// @crtc_cfg: CRTC to detach @plane_cfg from
//
// vkms_config_create_crtc() - Add a new CRTC configuration
// @config: Configuration to add the CRTC to
//
// Returns:
// The new CRTC configuration or an error. Call vkms_config_destroy_crtc() to
// free the returned CRTC configuration.
//
// vkms_config_destroy_crtc() - Remove and free a CRTC configuration
// @config: Configuration to remove the CRTC from
// @crtc_cfg: CRTC configuration to destroy
//
// vkms_config_crtc_get_writeback() - If a writeback connector will be created
// @crtc_cfg: CRTC with or without a writeback connector
//
// vkms_config_crtc_set_writeback() - If a writeback connector will be created
// @crtc_cfg: Target CRTC
// @writeback: Enable or disable the writeback connector
//
// vkms_config_crtc_primary_plane() - Return the primary plane for a CRTC
// @config: Configuration containing the CRTC
// @crtc_config: Target CRTC
//
// Note that, if multiple primary planes are found, the first one is returned.
// In this case, the configuration will be invalid. See vkms_config_is_valid().
//
// Returns:
// The primary plane or NULL if none is assigned yet.
//
// vkms_config_crtc_cursor_plane() - Return the cursor plane for a CRTC
// @config: Configuration containing the CRTC
// @crtc_config: Target CRTC
//
// Note that, if multiple cursor planes are found, the first one is returned.
// In this case, the configuration will be invalid. See vkms_config_is_valid().
//
// Returns:
// The cursor plane or NULL if none is assigned yet.
//
// vkms_config_create_encoder() - Add a new encoder configuration
// @config: Configuration to add the encoder to
//
// Returns:
// The new encoder configuration or an error. Call vkms_config_destroy_encoder()
// to free the returned encoder configuration.
//
// vkms_config_destroy_encoder() - Remove and free a encoder configuration
// @config: Configuration to remove the encoder from
// @encoder_cfg: Encoder configuration to destroy
//
// vkms_config_encoder_attach_crtc - Attach a encoder to a CRTC
// @encoder_cfg: Encoder to attach
// @crtc_cfg: CRTC to attach @encoder_cfg to
//
// vkms_config_encoder_detach_crtc - Detach a encoder from a CRTC
// @encoder_cfg: Encoder to detach
// @crtc_cfg: CRTC to detach @encoder_cfg from
//
// vkms_config_create_connector() - Add a new connector configuration
// @config: Configuration to add the connector to
//
// Returns:
// The new connector configuration or an error. Call
// vkms_config_destroy_connector() to free the returned connector configuration.
//
// vkms_config_destroy_connector() - Remove and free a connector configuration
// @connector_cfg: Connector configuration to destroy
//
extern "C" {
    pub fn vkms_config_destroy_connector(connector_cfg: *mut vkms_config_connector);
}
//
// vkms_config_connector_attach_encoder - Attach a connector to an encoder
// @connector_cfg: Connector to attach
// @encoder_cfg: Encoder to attach @connector_cfg to
//
// vkms_config_connector_detach_encoder - Detach a connector from an encoder
// @connector_cfg: Connector to detach
// @encoder_cfg: Encoder to detach @connector_cfg from
//
// vkms_config_connector_get_status() - Return the status of the connector
// @connector_cfg: Connector to get the status from
//
// vkms_config_connector_set_status() - Set the status of the connector
// @connector_cfg: Connector to set the status to
// @status: New connector status
//
