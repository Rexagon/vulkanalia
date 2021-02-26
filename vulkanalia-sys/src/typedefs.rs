// SPDX-License-Identifier: Apache-2.0

// DO NOT EDIT.
//
// This file has been generated by the Kotlin project in the `generator`
// directory from a Vulkan API registry.

#![allow(
    non_camel_case_types,
    non_snake_case,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::upper_case_acronyms
)]

use std::os::raw::{c_ulong, c_void};

/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBool32.html>
pub type Bool32 = u32;
/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceAddress.html>
pub type DeviceAddress = u64;
/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceSize.html>
pub type DeviceSize = u64;
/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFlags.html>
pub type Flags = u32;
/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFlags64.html>
pub type Flags64 = u64;
/// <https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleMask.html>
pub type SampleMask = u32;

// Android

pub type ANativeWindow = c_void;
pub type AHardwareBuffer = c_void;

// DirectFB

pub type IDirectFB = c_void;
pub type IDirectFBSurface = c_void;

// iOS / macOS

pub type CAMetalLayer = c_void;
pub type GgpFrameToken = u32;
pub type GgpStreamDescriptor = u32;

// QNX

pub type _screen_context = c_void;
pub type _screen_window = c_void;

// Wayland

pub type wl_display = c_void;
pub type wl_surface = c_void;

// Windows

pub type DWORD = c_ulong;
pub type HANDLE = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type HMONITOR = *mut c_void;
pub type HWND = *mut c_void;
pub type LPCWSTR = *const u16;
pub type SECURITY_ATTRIBUTES = c_void;

// X11

pub type Display = *const c_void;
pub type RROutput = c_ulong;
pub type VisualID = c_ulong;
pub type Window = c_ulong;
pub type xcb_connection_t = c_void;
pub type xcb_visualid_t = u32;
pub type xcb_window_t = u32;
pub type zx_handle_t = u32;
