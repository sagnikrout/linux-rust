//! Automatically rewritten from C to Rust
//! Source: lib/errname.c
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


// SPDX-License-Identifier: GPL-2.0

//
// Ensure these tables do not accidentally become gigantic if some
// huge errno makes it in. On most architectures, the first table will
// only have about 140 entries, but mips and parisc have more sparsely
// allocated errnos (with EHWPOISON = 257 on parisc, and EDQUOT = 1133
// on mips), so this wastes a bit of space on those - though we
// special case the EDQUOT case.
//

    static const char *names_0[] = {
    E(E2BIG),
    E(EACCES),
    E(EADDRINUSE),
    E(EADDRNOTAVAIL),
    E(EADV),
    E(EAFNOSUPPORT),
    E(EAGAIN), /* EWOULDBLOCK */
    E(EALREADY),
    E(EBADE),
    E(EBADF),
    E(EBADFD),
    E(EBADMSG),
    E(EBADR),
    E(EBADRQC),
    E(EBADSLT),
    E(EBFONT),
    E(EBUSY),
    E(ECANCELED), /* ECANCELLED */
    E(ECHILD),
    E(ECHRNG),
    E(ECOMM),
    E(ECONNABORTED),
    E(ECONNREFUSED), /* EREFUSED */
    E(ECONNRESET),
    E(EDEADLK), /* EDEADLOCK */

    E(EDEADLOCK),

    E(EDESTADDRREQ),
    E(EDOM),
    E(EDOTDOT),

    E(EDQUOT),

    E(EEXIST),
    E(EFAULT),
    E(EFBIG),
    E(EHOSTDOWN),
    E(EHOSTUNREACH),
    E(EHWPOISON),
    E(EIDRM),
    E(EILSEQ),

    E(EINIT),

    E(EINPROGRESS),
    E(EINTR),
    E(EINVAL),
    E(EIO),
    E(EISCONN),
    E(EISDIR),
    E(EISNAM),
    E(EKEYEXPIRED),
    E(EKEYREJECTED),
    E(EKEYREVOKED),
    E(EL2HLT),
    E(EL2NSYNC),
    E(EL3HLT),
    E(EL3RST),
    E(ELIBACC),
    E(ELIBBAD),
    E(ELIBEXEC),
    E(ELIBMAX),
    E(ELIBSCN),
    E(ELNRNG),
    E(ELOOP),
    E(EMEDIUMTYPE),
    E(EMFILE),
    E(EMLINK),
    E(EMSGSIZE),
    E(EMULTIHOP),
    E(ENAMETOOLONG),
    E(ENAVAIL),
    E(ENETDOWN),
    E(ENETRESET),
    E(ENETUNREACH),
    E(ENFILE),
    E(ENOANO),
    E(ENOBUFS),
    E(ENOCSI),
    E(ENODATA),
    E(ENODEV),
    E(ENOENT),
    E(ENOEXEC),
    E(ENOKEY),
    E(ENOLCK),
    E(ENOLINK),
    E(ENOMEDIUM),
    E(ENOMEM),
    E(ENOMSG),
    E(ENONET),
    E(ENOPKG),
    E(ENOPROTOOPT),
    E(ENOSPC),
    E(ENOSR),
    E(ENOSTR),
    E(ENOSYS),
    E(ENOTBLK),
    E(ENOTCONN),
    E(ENOTDIR),
    E(ENOTEMPTY),
    E(ENOTNAM),
    E(ENOTRECOVERABLE),
    E(ENOTSOCK),
    E(ENOTTY),
    E(ENOTUNIQ),
    E(ENXIO),
    E(EOPNOTSUPP),
    E(EOVERFLOW),
    E(EOWNERDEAD),
    E(EPERM),
    E(EPFNOSUPPORT),
    E(EPIPE),

    E(EPROCLIM),

    E(EPROTO),
    E(EPROTONOSUPPORT),
    E(EPROTOTYPE),
    E(ERANGE),
    E(EREMCHG),

    E(EREMDEV),

    E(EREMOTE),
    E(EREMOTEIO),
    E(ERESTART),
    E(ERFKILL),
    E(EROFS),

    E(ERREMOTE),

    E(ESHUTDOWN),
    E(ESOCKTNOSUPPORT),
    E(ESPIPE),
    E(ESRCH),
    E(ESRMNT),
    E(ESTALE),
    E(ESTRPIPE),
    E(ETIME),
    E(ETIMEDOUT),
    E(ETOOMANYREFS),
    E(ETXTBSY),
    E(EUCLEAN),
    E(EUNATCH),
    E(EUSERS),
    E(EXDEV),
    E(EXFULL),
    };

    static_assert(EREFUSED == ECONNREFUSED);

    static_assert(ECANCELLED == ECANCELED);

    static_assert(EAGAIN == EWOULDBLOCK); /* everywhere */

    static const char *names_512[] = {
    E(ERESTARTSYS),
    E(ERESTARTNOINTR),
    E(ERESTARTNOHAND),
    E(ENOIOCTLCMD),
    E(ERESTART_RESTARTBLOCK),
    E(EPROBE_DEFER),
    E(EOPENSTALE),
    E(ENOPARAM),
    E(EBADHANDLE),
    E(ENOTSYNC),
    E(EBADCOOKIE),
    E(ENOTSUPP),
    E(ETOOSMALL),
    E(ESERVERFAULT),
    E(EBADTYPE),
    E(EJUKEBOX),
    E(EIOCBQUEUED),
    E(ERECALLCONFLICT),
    };

    static const char *__errname(unsigned err)
    {
    if (err < ARRAY_SIZE(names_0))
    return names_0[err];
    if (err >= 512 && err - 512 < ARRAY_SIZE(names_512))
    return names_512[err - 512];
// But why?
    if (IS_ENABLED(CONFIG_MIPS) && err == EDQUOT) /* 1133 */
    return "-EDQUOT";
    return core::ptr::null_mut();
    }
//
// errname(EIO) -> "EIO"
// errname(-EIO) -> "-EIO"
//
    const char *errname(int err)
    {
    const char *name = __errname(abs(err));
    if (!name)
    return core::ptr::null_mut();
    return err > 0 ? name + 1 : name;
    }
    EXPORT_SYMBOL(errname);
