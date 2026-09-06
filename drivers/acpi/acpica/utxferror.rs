//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/utxferror.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utxferror - Various error/warning output functions
//
// Macro flag: #define EXPORT_ACPI_INTERFACES

    ACPI_MODULE_NAME("utxferror")
//
// This module is used for the in-kernel ACPICA as well as the ACPICA
// tools/applications.
//

//
// FUNCTION:    acpi_error
//
// PARAMETERS:  module_name         - Caller's module name (for error output)
// line_number         - Caller's line number (for error output)
// format              - Printf format string + additional args
//
// RETURN:      None
//
// DESCRIPTION: Print "ACPI Error" message with module/line/version info
//
    void ACPI_INTERNAL_VAR_XFACE
    acpi_error(const char *module_name, u32 line_number, const char *format, ...)
    {
    va_list arg_list;
    ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(ACPI_MSG_ERROR);
    va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    ACPI_MSG_SUFFIX;
    va_end(arg_list);
    ACPI_MSG_REDIRECT_END;
    }
    ACPI_EXPORT_SYMBOL(acpi_error)
//
// FUNCTION:    acpi_exception
//
// PARAMETERS:  module_name         - Caller's module name (for error output)
// line_number         - Caller's line number (for error output)
// status              - Status value to be decoded/formatted
// format              - Printf format string + additional args
//
// RETURN:      None
//
// DESCRIPTION: Print an "ACPI Error" message with module/line/version
// info as well as decoded acpi_status.
//
    void ACPI_INTERNAL_VAR_XFACE
    acpi_exception(const char *module_name,
    u32 line_number, acpi_status status, const char *format, ...)
    {
    va_list arg_list;
    ACPI_MSG_REDIRECT_BEGIN;
// For AE_OK, just print the message
    if (ACPI_SUCCESS(status)) {
    acpi_os_printf(ACPI_MSG_ERROR);
    } else {
    acpi_os_printf(ACPI_MSG_ERROR "%s, ",
    acpi_format_exception(status));
    }
    va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    ACPI_MSG_SUFFIX;
    va_end(arg_list);
    ACPI_MSG_REDIRECT_END;
    }
    ACPI_EXPORT_SYMBOL(acpi_exception)
//
// FUNCTION:    acpi_warning
//
// PARAMETERS:  module_name         - Caller's module name (for warning output)
// line_number         - Caller's line number (for warning output)
// format              - Printf format string + additional args
//
// RETURN:      None
//
// DESCRIPTION: Print "ACPI Warning" message with module/line/version info
//
    void ACPI_INTERNAL_VAR_XFACE
    acpi_warning(const char *module_name, u32 line_number, const char *format, ...)
    {
    va_list arg_list;
    ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(ACPI_MSG_WARNING);
    va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    ACPI_MSG_SUFFIX;
    va_end(arg_list);
    ACPI_MSG_REDIRECT_END;
    }
    ACPI_EXPORT_SYMBOL(acpi_warning)
//
// FUNCTION:    acpi_info
//
// PARAMETERS:  format              - Printf format string + additional args
//
// RETURN:      None
//
// DESCRIPTION: Print generic "ACPI:" information message. There is no
// module/line/version info in order to keep the message simple.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_info(format: *const c_char, ...) -> void ACPI_INTERNAL_VAR_XFACE {
    void ACPI_INTERNAL_VAR_XFACE acpi_info(const char *format, ...)
    {
    va_list arg_list;
    ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(ACPI_MSG_INFO);
    va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    acpi_os_printf("\n");
    va_end(arg_list);
    ACPI_MSG_REDIRECT_END;
    }
    ACPI_EXPORT_SYMBOL(acpi_info)
//
// FUNCTION:    acpi_bios_error
//
// PARAMETERS:  module_name         - Caller's module name (for error output)
// line_number         - Caller's line number (for error output)
// format              - Printf format string + additional args
//
// RETURN:      None
//
// DESCRIPTION: Print "ACPI Firmware Error" message with module/line/version
// info
//
    void ACPI_INTERNAL_VAR_XFACE
    acpi_bios_error(const char *module_name,
    u32 line_number, const char *format, ...)
    {
    va_list arg_list;
    ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(ACPI_MSG_BIOS_ERROR);
    va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    ACPI_MSG_SUFFIX;
    va_end(arg_list);
    ACPI_MSG_REDIRECT_END;
    }
    ACPI_EXPORT_SYMBOL(acpi_bios_error)
//
// FUNCTION:    acpi_bios_exception
//
// PARAMETERS:  module_name         - Caller's module name (for error output)
// line_number         - Caller's line number (for error output)
// status              - Status value to be decoded/formatted
// format              - Printf format string + additional args
//
// RETURN:      None
//
// DESCRIPTION: Print an "ACPI Firmware Error" message with module/line/version
// info as well as decoded acpi_status.
//
    void ACPI_INTERNAL_VAR_XFACE
    acpi_bios_exception(const char *module_name,
    u32 line_number,
    acpi_status status, const char *format, ...)
    {
    va_list arg_list;
    ACPI_MSG_REDIRECT_BEGIN;
// For AE_OK, just print the message
    if (ACPI_SUCCESS(status)) {
    acpi_os_printf(ACPI_MSG_BIOS_ERROR);
    } else {
    acpi_os_printf(ACPI_MSG_BIOS_ERROR "%s, ",
    acpi_format_exception(status));
    }
    va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    ACPI_MSG_SUFFIX;
    va_end(arg_list);
    ACPI_MSG_REDIRECT_END;
    }
    ACPI_EXPORT_SYMBOL(acpi_bios_exception)
//
// FUNCTION:    acpi_bios_warning
//
// PARAMETERS:  module_name         - Caller's module name (for warning output)
// line_number         - Caller's line number (for warning output)
// format              - Printf format string + additional args
//
// RETURN:      None
//
// DESCRIPTION: Print "ACPI Firmware Warning" message with module/line/version
// info
//
    void ACPI_INTERNAL_VAR_XFACE
    acpi_bios_warning(const char *module_name,
    u32 line_number, const char *format, ...)
    {
    va_list arg_list;
    ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(ACPI_MSG_BIOS_WARNING);
    va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    ACPI_MSG_SUFFIX;
    va_end(arg_list);
    ACPI_MSG_REDIRECT_END;
    }
    ACPI_EXPORT_SYMBOL(acpi_bios_warning)
