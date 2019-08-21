//! Safe wrappers around functions found in libc "unistd.h" header

use errno;
use {Errno, Error, Result, NixPath};
use fcntl::{fcntl, OFlag, O_CLOEXEC, FD_CLOEXEC};
use fcntl::FcntlArg::F_SETFD;
use libc::{self, c_char, c_void, c_int, c_long, c_uint, size_t, pid_t, off_t,
           uid_t, gid_t, mode_t};
use std::mem;
use std::ffi::{CString, CStr, OsString, OsStr};
use std::os::unix::ffi::{OsStringExt, OsStrExt};
use std::os::unix::io::RawFd;
use std::path::{PathBuf};
use void::Void;
use sys::stat::Mode;
use std::fmt;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::linux::*;

/// User identifier
///
/// Newtype pattern around `uid_t` (which is just alias). It prevents bugs caused by accidentally
/// passing wrong value.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Uid(uid_t);

impl Uid {
    /// Creates `Uid` from raw `uid_t`.
    pub fn from_raw(uid: uid_t) -> Self {
        Uid(uid)
    }

    /// Returns Uid of calling process. This is practically a more Rusty alias for `getuid`.
    pub fn current() -> Self {
        getuid()
    }

    /// Returns effective Uid of calling process. This is practically a more Rusty alias for `geteuid`.
    pub fn effective() -> Self {
        geteuid()
    }

    /// Returns true if the `Uid` represents privileged user - root. (If it equals zero.)
    pub fn is_root(&self) -> bool {
        *self == ROOT
    }
}

impl From<Uid> for uid_t {
    fn from(uid: Uid) -> Self {
        uid.0
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Constant for UID = 0
pub const ROOT: Uid = Uid(0);

/// Group identifier
///
/// Newtype pattern around `gid_t` (which is just alias). It prevents bugs caused by accidentally
/// passing wrong value.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Gid(gid_t);

impl Gid {
    /// Creates `Gid` from raw `gid_t`.
    pub fn from_raw(gid: gid_t) -> Self {
        Gid(gid)
    }

    /// Returns Gid of calling process. This is practically a more Rusty alias for `getgid`.
    pub fn current() -> Self {
        getgid()
    }

    /// Returns effective Gid of calling process. This is practically a more Rusty alias for `getgid`.
    pub fn effective() -> Self {
        getegid()
    }
}

impl From<Gid> for gid_t {
    fn from(gid: Gid) -> Self {
        gid.0
    }
}

impl fmt::Display for Gid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Process identifier
///
/// Newtype pattern around `pid_t` (which is just alias). It prevents bugs caused by accidentally
/// passing wrong value.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pid(pid_t);

impl Pid {
    /// Creates `Pid` from raw `pid_t`.
    pub fn from_raw(pid: pid_t) -> Self {
        Pid(pid)
    }

    /// Returns PID of calling process
    pub fn this() -> Self {
        getpid()
    }

    /// Returns PID of parent of calling process
    pub fn parent() -> Self {
        getppid()
    }
}

impl From<Pid> for pid_t {
    fn from(pid: Pid) -> Self {
        pid.0
    }
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}


/// Represents the successful result of calling `fork`
///
/// When `fork` is called, the process continues execution in the parent process
/// and in the new child.  This return type can be examined to determine whether
/// you are now executing in the parent process or in the child.
#[derive(Clone, Copy)]
pub enum ForkResult {
    Parent { child: Pid },
    Child,
}

impl ForkResult {

    /// Return `true` if this is the child process of the `fork()`
    #[inline]
    pub fn is_child(&self) -> bool {
        match *self {
            ForkResult::Child => true,
            _ => false
        }
    }

    /// Returns `true` if this is the parent process of the `fork()`
    #[inline]
    pub fn is_parent(&self) -> bool {
        !self.is_child()
    }
}

/// Create a new child process duplicating the parent process ([see
/// fork(2)](http://man7.org/linux/man-pages/man2/fork.2.html)).
///
/// After calling the fork system call (successfully) two processes will
/// be created that are identical with the exception of their pid and the
/// return value of this function.  As an example:
///
/// ```no_run
/// use nix::unistd::{fork, ForkResult};
///
/// match fork() {
///    Ok(ForkResult::Parent { child, .. }) => {
///        println!("Continuing execution in parent process, new child has pid: {}", child);
///    }
///    Ok(ForkResult::Child) => println!("I'm a new child process"),
///    Err(_) => println!("Fork failed"),
/// }
/// ```
///
/// This will print something like the following (order indeterministic).  The
/// thing to note is that you end up with two processes continuing execution
/// immediately after the fork call but with different match arms.
///
/// ```text
/// Continuing execution in parent process, new child has pid: 1234
/// I'm a new child process
/// ```
///
/// # Safety
///
/// In a multithreaded program, only [async-signal-safe] functions like `pause`
/// and `_exit` may be called by the child (the parent isn't restricted). Note
/// that memory allocation may **not** be async-signal-safe and thus must be
/// prevented.
///
/// Those functions are only a small subset of your operating system's API, so
/// special care must be taken to only invoke code you can control and audit.
///
/// [async-signal-safe]: http://man7.org/linux/man-pages/man7/signal-safety.7.html
#[inline]
pub fn fork() -> Result<ForkResult> {
    use self::ForkResult::*;
    let res = unsafe { libc::fork() };

    Errno::result(res).map(|res| match res {
        0 => Child,
        res => Parent { child: Pid(res) },
    })
}

/// Get the pid of this process (see
/// [getpid(2)](http://man7.org/linux/man-pages/man2/getpid.2.html)).
///
/// Since you are running code, there is always a pid to return, so there
/// is no error case that needs to be handled.
#[inline]
pub fn getpid() -> Pid {
    Pid(unsafe { libc::getpid() })
}

/// Get the pid of this processes' parent (see
/// [getpid(2)](http://man7.org/linux/man-pages/man2/getpid.2.html)).
///
/// There is always a parent pid to return, so there is no error case that needs
/// to be handled.
#[inline]
pub fn getppid() -> Pid {
    Pid(unsafe { libc::getppid() }) // no error handling, according to man page: "These functions are always successful."
}

/// Set a process group ID (see
/// [setpgid(2)](http://man7.org/linux/man-pages/man2/setpgid.2.html)).
///
/// Set the process group id (PGID) of a particular process.  If a pid of zero
/// is specified, then the pid of the calling process is used.  Process groups
/// may be used to group together a set of processes in order for the OS to
/// apply some operations across the group.
///
/// `setsid()` may be used to create a new process group.
#[inline]
pub fn setpgid(pid: Pid, pgid: Pid) -> Result<()> {
    let res = unsafe { libc::setpgid(pid.into(), pgid.into()) };
    Errno::result(res).map(drop)
}
#[inline]
pub fn getpgid(pid: Option<Pid>) -> Result<Pid> {
    let res = unsafe { libc::getpgid(pid.unwrap_or(Pid(0)).into()) };
    Errno::result(res).map(Pid)
}

/// Create new session and set process group id (see
/// [setsid(2)](http://man7.org/linux/man-pages/man2/setsid.2.html)).
#[inline]
pub fn setsid() -> Result<Pid> {
    Errno::result(unsafe { libc::setsid() }).map(Pid)
}


/// Get the terminal foreground process group (see
/// [tcgetpgrp(3)](http://man7.org/linux/man-pages/man3/tcgetpgrp.3.html)).
///
/// Get the group process id (GPID) of the foreground process group on the
/// terminal associated to file descriptor (FD).
#[inline]
pub fn tcgetpgrp(fd: c_int) -> Result<Pid> {
    let res = unsafe { libc::tcgetpgrp(fd) };
    Errno::result(res).map(Pid)
}
/// Set the terminal foreground process group (see
/// [tcgetpgrp(3)](http://man7.org/linux/man-pages/man3/tcgetpgrp.3.html)).
///
/// Get the group process id (PGID) to the foreground process group on the
/// terminal associated to file descriptor (FD).
#[inline]
pub fn tcsetpgrp(fd: c_int, pgrp: Pid) -> Result<()> {
    let res = unsafe { libc::tcsetpgrp(fd, pgrp.into()) };
    Errno::result(res).map(drop)
}


/// Get the group id of the calling process (see
///[getpgrp(3)](http://man7.org/linux/man-pages/man3/getpgrp.3p.html)).
///
/// Get the process group id (PGID) of the calling process.
/// According to the man page it is always successful.
#[inline]
pub fn getpgrp() -> Pid {
    Pid(unsafe { libc::getpgrp() })
}

/// Get the caller's thread ID (see
/// [gettid(2)](http://man7.org/linux/man-pages/man2/gettid.2.html).
///
/// This function is only available on Linux based systems.  In a single
/// threaded process, the main thread will have the same ID as the process.  In
/// a multithreaded process, each thread will have a unique thread id but the
/// same process ID.
///
/// No error handling is required as a thread id should always exist for any
/// process, even if threads are not being used.
#[cfg(any(target_os = "linux", target_os = "android"))]
#[inline]
pub fn gettid() -> Pid {
    Pid(unsafe { libc::syscall(libc::SYS_gettid) as pid_t })
}

/// Create a copy of the specified file descriptor (see
/// [dup(2)](http://man7.org/linux/man-pages/man2/dup.2.html)).
///
/// The new file descriptor will be have a new index but refer to the same
/// resource as the old file descriptor and the old and new file descriptors may
/// be used interchangeably.  The new and old file descriptor share the same
/// underlying resource, offset, and file status flags.  The actual index used
/// for the file descriptor will be the lowest fd index that is available.
///
/// The two file descriptors do not share file descriptor flags (e.g. `FD_CLOEXEC`).
#[inline]
pub fn dup(oldfd: RawFd) -> Result<RawFd> {
    let res = unsafe { libc::dup(oldfd) };

    Errno::result(res)
}

/// Create a copy of the specified file descriptor using the specified fd (see
/// [dup(2)](http://man7.org/linux/man-pages/man2/dup.2.html)).
///
/// This function behaves similar to `dup()` except that it will try to use the
/// specified fd instead of allocating a new one.  See the man pages for more
/// detail on the exact behavior of this function.
#[inline]
pub fn dup2(oldfd: RawFd, newfd: RawFd) -> Result<RawFd> {
    let res = unsafe { libc::dup2(oldfd, newfd) };

    Errno::result(res)
}

/// Create a new copy of the specified file descriptor using the specified fd
/// and flags (see [dup(2)](http://man7.org/linux/man-pages/man2/dup.2.html)).
///
/// This function behaves similar to `dup2()` but allows for flags to be
/// specified.
pub fn dup3(oldfd: RawFd, newfd: RawFd, flags: OFlag) -> Result<RawFd> {
    dup3_polyfill(oldfd, newfd, flags)
}

#[inline]
fn dup3_polyfill(oldfd: RawFd, newfd: RawFd, flags: OFlag) -> Result<RawFd> {
    if oldfd == newfd {
        return Err(Error::Sys(Errno::EINVAL));
    }

    let fd = try!(dup2(oldfd, newfd));

    if flags.contains(O_CLOEXEC) {
        if let Err(e) = fcntl(fd, F_SETFD(FD_CLOEXEC)) {
            let _ = close(fd);
            return Err(e);
        }
    }

    Ok(fd)
}

/// Change the current working directory of the calling process (see
/// [chdir(2)](http://man7.org/linux/man-pages/man2/chdir.2.html)).
///
/// This function may fail in a number of different scenarios.  See the man
/// pages for additional details on possible failure cases.
#[inline]
pub fn chdir<P: ?Sized + NixPath>(path: &P) -> Result<()> {
    let res = try!(path.with_nix_path(|cstr| {
        unsafe { libc::chdir(cstr.as_ptr()) }
    }));

    Errno::result(res).map(drop)
}

/// Change the current working directory of the process to the one
/// given as an open file descriptor (see
/// [fchdir(2)](http://man7.org/linux/man-pages/man2/fchdir.2.html)).
///
/// This function may fail in a number of different scenarios.  See the man
/// pages for additional details on possible failure cases.
#[inline]
pub fn fchdir(dirfd: RawFd) -> Result<()> {
    let res = unsafe { libc::fchdir(dirfd) };

    Errno::result(res).map(drop)
}

/// Creates new directory `path` with access rights `mode`.
///
/// # Errors
///
/// There are several situations where mkdir might fail:
///
/// - current user has insufficient rights in the parent directory
/// - the path already exists
/// - the path name is too long (longer than `PATH_MAX`, usually 4096 on linux, 1024 on OS X)
///
/// For a full list consult
/// [man mkdir(2)](http://man7.org/linux/man-pages/man2/mkdir.2.html#ERRORS)
///
/// # Example
///
/// ```rust
/// extern crate tempdir;
/// extern crate nix;
///
/// use nix::unistd;
/// use nix::sys::stat;
/// use tempdir::TempDir;
///
/// fn main() {
///     let tmp_dir1 = TempDir::new("test_mkdir").unwrap();
///     let tmp_dir2 = tmp_dir1.path().join("new_dir");
///
///     // create new directory and give read, write and execute rights to the owner
///     match unistd::mkdir(&tmp_dir2, stat::S_IRWXU) {
///        Ok(_) => println!("created {:?}", tmp_dir2),
///        Err(err) => println!("Error creating directory: {}", err),
///     }
/// }
/// ```
#[inline]
pub fn mkdir<P: ?Sized + NixPath>(path: &P, mode: Mode) -> Result<()> {
    let res = try!(path.with_nix_path(|cstr| {
        unsafe { libc::mkdir(cstr.as_ptr(), mode.bits() as mode_t) }
    }));

    Errno::result(res).map(drop)
}

/// Returns the current directory as a PathBuf
///
/// Err is returned if the current user doesn't have the permission to read or search a component
/// of the current path.
///
/// # Example
///
/// ```rust
/// extern crate nix;
///
/// use nix::unistd;
///
/// fn main() {
///     // assume that we are allowed to get current directory
///     let dir = unistd::getcwd().unwrap();
///     println!("The current directory is {:?}", dir);
/// }
/// ```
#[inline]
pub fn getcwd() -> Result<PathBuf> {
    let mut buf = Vec::with_capacity(512);
    loop {
        unsafe {
            let ptr = buf.as_mut_ptr() as *mut libc::c_char;

            // The buffer must be large enough to store the absolute pathname plus
            // a terminating null byte, or else null is returned.
            // To safely handle this we start with a reasonable size (512 bytes)
            // and double the buffer size upon every error
            if !libc::getcwd(ptr, buf.capacity()).is_null() {
                let len = CStr::from_ptr(buf.as_ptr() as *const libc::c_char).to_bytes().len();
                buf.set_len(len);
                buf.shrink_to_fit();
                return Ok(PathBuf::from(OsString::from_vec(buf)));
            } else {
                let error = Errno::last();
                // ERANGE means buffer was too small to store directory name
                if error != Errno::ERANGE {
                    return Err(Error::Sys(error));
                }
            }

            // Trigger the internal buffer resizing logic of `Vec` by requiring
            // more space than the current capacity.
            let cap = buf.capacity();
            buf.set_len(cap);
            buf.reserve(1);
        }
    }
}

/// Change the ownership of the file at `path` to be owned by the specified
/// `owner` (user) and `group` (see
/// [chown(2)](http://man7.org/linux/man-pages/man2/lchown.2.html)).
///
/// The owner/group for the provided path name will not be modified if `None` is
/// provided for that argument.  Ownership change will be attempted for the path
/// only if `Some` owner/group is provided.
///
/// This call may fail under a number of different situations.  See [the man
/// pages](http://man7.org/linux/man-pages/man2/lchown.2.html#ERRORS) for
/// additional details.
#[inline]
pub fn chown<P: ?Sized + NixPath>(path: &P, owner: Option<Uid>, group: Option<Gid>) -> Result<()> {
    let res = try!(path.with_nix_path(|cstr| {
        // According to the POSIX specification, -1 is used to indicate that
        // owner and group, respectively, are not to be changed. Since uid_t and
        // gid_t are unsigned types, we use wrapping_sub to get '-1'.
        unsafe { libc::chown(cstr.as_ptr(),
                             owner.map(Into::into).unwrap_or((0 as uid_t).wrapping_sub(1)),
                             group.map(Into::into).unwrap_or((0 as gid_t).wrapping_sub(1))) }
    }));

    Errno::result(res).map(drop)
}

fn to_exec_array(args: &[CString]) -> Vec<*const c_char> {
    use std::ptr;
    use libc::c_char;

    let mut args_p: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();
    args_p.push(ptr::null());
    args_p
}

/// Replace the current process image with a new one (see
/// [exec(3)](http://man7.org/linux/man-pages/man3/exec.3.html)).
///
/// See the `::nix::unistd::execve` system call for additional details.  `execv`
/// performs the same action but does not allow for customization of the
/// environment for the new process.
#[inline]
pub fn execv(path: &CString, argv: &[CString]) -> Result<Void> {
    let args_p = to_exec_array(argv);

    unsafe {
        libc::execv(path.as_ptr(), args_p.as_ptr())
    };

    Err(Error::Sys(Errno::last()))
}


/// Replace the current process image with a new one (see
/// [execve(2)](http://man7.org/linux/man-pages/man2/execve.2.html)).
///
/// The execve system call allows for another process to be "called" which will
/// replace the current process image.  That is, this process becomes the new
/// command that is run. On success, this function will not return. Instead,
/// the new program will run until it exits.
///
/// If an error occurs, this function will return with an indication of the
/// cause of failure.  See
/// [execve(2)#errors](http://man7.org/linux/man-pages/man2/execve.2.html#ERRORS)
/// for a list of potential problems that maight cause execv to fail.
///
/// `::nix::unistd::execv` and `::nix::unistd::execve` take as arguments a slice
/// of `::std::ffi::CString`s for `args` and `env` (for `execve`). Each element
/// in the `args` list is an argument to the new process. Each element in the
/// `env` list should be a string in the form "key=value".
#[inline]
pub fn execve(path: &CString, args: &[CString], env: &[CString]) -> Result<Void> {
    let args_p = to_exec_array(args);
    let env_p = to_exec_array(env);

    unsafe {
        libc::execve(path.as_ptr(), args_p.as_ptr(), env_p.as_ptr())
    };

    Err(Error::Sys(Errno::last()))
}

/// Replace the current process image with a new one and replicate shell `PATH`
/// searching behavior (see
/// [exec(3)](http://man7.org/linux/man-pages/man3/exec.3.html)).
///
/// See `::nix::unistd::execve` for additoinal details.  `execvp` behaves the
/// same as execv except that it will examine the `PATH` environment variables
/// for file names not specified with a leading slash.  For example, `execv`
/// would not work if "bash" was specified for the path argument, but `execvp`
/// would assuming that a bash executable was on the system `PATH`.
#[inline]
pub fn execvp(filename: &CString, args: &[CString]) -> Result<Void> {
    let args_p = to_exec_array(args);

    unsafe {
        libc::execvp(filename.as_ptr(), args_p.as_ptr())
    };

    Err(Error::Sys(Errno::last()))
}

/// Daemonize this process by detaching from the controlling terminal (see
/// [daemon(3)](http://man7.org/linux/man-pages/man3/daemon.3.html)).
///
/// When a process is launched it is typically associated with a parent and it,
/// in turn, by its controlling terminal/process.  In order for a process to run
/// in the "background" it must daemonize itself by detaching itself.  Under
/// posix, this is done by doing the following:
///
/// 1. Parent process (this one) forks
/// 2. Parent process exits
/// 3. Child process continues to run.
///
/// `nochdir`:
///
/// * `nochdir = true`: The current working directory after daemonizing will
///    be the current working directory.
/// *  `nochdir = false`: The current working directory after daemonizing will
///    be the root direcory, `/`.
///
/// `noclose`:
///
/// * `noclose = true`: The process' current stdin, stdout, and stderr file
///   descriptors will remain identical after daemonizing.
/// * `noclose = false`: The process' stdin, stdout, and stderr will point to
///   `/dev/null` after daemonizing.
///
/// The underlying implementation (in libc) calls both
/// [fork(2)](http://man7.org/linux/man-pages/man2/fork.2.html) and
/// [setsid(2)](http://man7.org/linux/man-pages/man2/setsid.2.html) and, as
/// such, error that could be returned by either of those functions could also
/// show up as errors here.
pub fn daemon(nochdir: bool, noclose: bool) -> Result<()> {
    let res = unsafe { libc::daemon(nochdir as c_int, noclose as c_int) };
    Errno::result(res).map(drop)
}

/// Set the system host name (see
/// [gethostname(2)](http://man7.org/linux/man-pages/man2/gethostname.2.html)).
///
/// Given a name, attempt to update the system host name to the given string.
/// On some systems, the host name is limited to as few as 64 bytes.  An error
/// will be return if the name is not valid or the current process does not have
/// permissions to update the host name.
pub fn sethostname<S: AsRef<OsStr>>(name: S) -> Result<()> {
    // Handle some differences in type of the len arg across platforms.
    cfg_if! {
        if #[cfg(any(target_os = "dragonfly",
                     target_os = "freebsd",
                     target_os = "ios",
                     target_os = "macos", ))] {
            type sethostname_len_t = c_int;
        } else {
            type sethostname_len_t = size_t;
        }
    }
    let ptr = name.as_ref().as_bytes().as_ptr() as *const c_char;
    let len = name.as_ref().len() as sethostname_len_t;

    let res = unsafe { libc::sethostname(ptr, len) };
    Errno::result(res).map(drop)
}

/// Get the host name and store it in the provided buffer, returning a pointer
/// the CStr in that buffer on success (see
/// [gethostname(2)](http://man7.org/linux/man-pages/man2/gethostname.2.html)).
///
/// This function call attempts to get the host name for the running system and
/// store it in a provided buffer.  The buffer will be populated with bytes up
/// to the length of the provided slice including a NUL terminating byte.  If
/// the hostname is longer than the length provided, no error will be provided.
/// The posix specification does not specify whether implementations will
/// null-terminate in this case, but the nix implementation will ensure that the
/// buffer is null terminated in this case.
///
/// ```no_run
/// use nix::unistd;
///
/// let mut buf = [0u8; 64];
/// let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
/// let hostname = hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8");
/// println!("Hostname: {}", hostname);
/// ```
pub fn gethostname<'a>(buffer: &'a mut [u8]) -> Result<&'a CStr> {
    let ptr = buffer.as_mut_ptr() as *mut c_char;
    let len = buffer.len() as size_t;

    let res = unsafe { libc::gethostname(ptr, len) };
    Errno::result(res).map(|_| {
        buffer[len - 1] = 0; // ensure always null-terminated
        unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) }
    })
}

/// Close a raw file descriptor
///
/// Be aware that many Rust types implicitly close-on-drop, including
/// `std::fs::File`.  Explicitly closing them with this method too can result in
/// a double-close condition, which can cause confusing `EBADF` errors in
/// seemingly unrelated code.  Caveat programmer.
///
/// # Examples
///
/// ```no_run
/// extern crate tempfile;
/// extern crate nix;
///
/// use std::os::unix::io::AsRawFd;
/// use nix::unistd::close;
///
/// fn main() {
///     let f = tempfile::tempfile().unwrap();
///     close(f.as_raw_fd()).unwrap();   // Bad!  f will also close on drop!
/// }
/// ```
///
/// ```rust
/// extern crate tempfile;
/// extern crate nix;
///
/// use std::os::unix::io::IntoRawFd;
/// use nix::unistd::close;
///
/// fn main() {
///     let f = tempfile::tempfile().unwrap();
///     close(f.into_raw_fd()).unwrap(); // Good.  into_raw_fd consumes f
/// }
/// ```
pub fn close(fd: RawFd) -> Result<()> {
    let res = unsafe { libc::close(fd) };
    Errno::result(res).map(drop)
}

pub fn read(fd: RawFd, buf: &mut [u8]) -> Result<usize> {
    let res = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut c_void, buf.len() as size_t) };

    Errno::result(res).map(|r| r as usize)
}

pub fn write(fd: RawFd, buf: &[u8]) -> Result<usize> {
    let res = unsafe { libc::write(fd, buf.as_ptr() as *const c_void, buf.len() as size_t) };

    Errno::result(res).map(|r| r as usize)
}

pub enum Whence {
    SeekSet,
    SeekCur,
    SeekEnd,
    SeekData,
    SeekHole
}

impl Whence {
    fn to_libc_type(&self) -> c_int {
        match self {
            &Whence::SeekSet => libc::SEEK_SET,
            &Whence::SeekCur => libc::SEEK_CUR,
            &Whence::SeekEnd => libc::SEEK_END,
            &Whence::SeekData => 3,
            &Whence::SeekHole => 4
        }
    }

}

pub fn lseek(fd: RawFd, offset: libc::off_t, whence: Whence) -> Result<libc::off_t> {
    let res = unsafe { libc::lseek(fd, offset, whence.to_libc_type()) };

    Errno::result(res).map(|r| r as libc::off_t)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn lseek64(fd: RawFd, offset: libc::off64_t, whence: Whence) -> Result<libc::off64_t> {
    let res = unsafe { libc::lseek64(fd, offset, whence.to_libc_type()) };

    Errno::result(res).map(|r| r as libc::off64_t)
}

pub fn pipe() -> Result<(RawFd, RawFd)> {
    unsafe {
        let mut fds: [c_int; 2] = mem::uninitialized();

        let res = libc::pipe(fds.as_mut_ptr());

        try!(Errno::result(res));

        Ok((fds[0], fds[1]))
    }
}

// libc only defines `pipe2` in `libc::notbsd`.
#[cfg(any(target_os = "linux",
          target_os = "android",
          target_os = "emscripten"))]
pub fn pipe2(flags: OFlag) -> Result<(RawFd, RawFd)> {
    let mut fds: [c_int; 2] = unsafe { mem::uninitialized() };

    let res = unsafe { libc::pipe2(fds.as_mut_ptr(), flags.bits()) };

    try!(Errno::result(res));

    Ok((fds[0], fds[1]))
}

#[cfg(not(any(target_os = "linux",
              target_os = "android",
              target_os = "emscripten")))]
pub fn pipe2(flags: OFlag) -> Result<(RawFd, RawFd)> {
    let mut fds: [c_int; 2] = unsafe { mem::uninitialized() };

    let res = unsafe { libc::pipe(fds.as_mut_ptr()) };

    try!(Errno::result(res));

    try!(pipe2_setflags(fds[0], fds[1], flags));

    Ok((fds[0], fds[1]))
}

#[cfg(not(any(target_os = "linux",
              target_os = "android",
              target_os = "emscripten")))]
fn pipe2_setflags(fd1: RawFd, fd2: RawFd, flags: OFlag) -> Result<()> {
    use fcntl::O_NONBLOCK;
    use fcntl::FcntlArg::F_SETFL;

    let mut res = Ok(0);

    if flags.contains(O_CLOEXEC) {
        res = res
            .and_then(|_| fcntl(fd1, F_SETFD(FD_CLOEXEC)))
            .and_then(|_| fcntl(fd2, F_SETFD(FD_CLOEXEC)));
    }

    if flags.contains(O_NONBLOCK) {
        res = res
            .and_then(|_| fcntl(fd1, F_SETFL(O_NONBLOCK)))
            .and_then(|_| fcntl(fd2, F_SETFL(O_NONBLOCK)));
    }

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            let _ = close(fd1);
            let _ = close(fd2);
            Err(e)
        }
    }
}

pub fn ftruncate(fd: RawFd, len: off_t) -> Result<()> {
    Errno::result(unsafe { libc::ftruncate(fd, len) }).map(drop)
}

pub fn isatty(fd: RawFd) -> Result<bool> {
    use libc;

    unsafe {
        // ENOTTY means `fd` is a valid file descriptor, but not a TTY, so
        // we return `Ok(false)`
        if libc::isatty(fd) == 1 {
            Ok(true)
        } else {
            match Errno::last() {
                Errno::ENOTTY => Ok(false),
                err => Err(Error::Sys(err)),
            }
        }
    }
}

pub fn unlink<P: ?Sized + NixPath>(path: &P) -> Result<()> {
    let res = try!(path.with_nix_path(|cstr| {
        unsafe {
            libc::unlink(cstr.as_ptr())
        }
    }));
    Errno::result(res).map(drop)
}

#[inline]
pub fn chroot<P: ?Sized + NixPath>(path: &P) -> Result<()> {
    let res = try!(path.with_nix_path(|cstr| {
        unsafe { libc::chroot(cstr.as_ptr()) }
    }));

    Errno::result(res).map(drop)
}

#[inline]
pub fn fsync(fd: RawFd) -> Result<()> {
    let res = unsafe { libc::fsync(fd) };

    Errno::result(res).map(drop)
}

// `fdatasync(2) is in POSIX, but in libc it is only defined in `libc::notbsd`.
// TODO: exclude only Apple systems after https://github.com/rust-lang/libc/pull/211
#[cfg(any(target_os = "linux",
          target_os = "android",
          target_os = "emscripten"))]
#[inline]
pub fn fdatasync(fd: RawFd) -> Result<()> {
    let res = unsafe { libc::fdatasync(fd) };

    Errno::result(res).map(drop)
}

// POSIX requires that getuid, geteuid, getgid, getegid are always successful,
// so no need to check return value or errno. See:
//   - http://pubs.opengroup.org/onlinepubs/9699919799/functions/getuid.html
//   - http://pubs.opengroup.org/onlinepubs/9699919799/functions/geteuid.html
//   - http://pubs.opengroup.org/onlinepubs/9699919799/functions/getgid.html
//   - http://pubs.opengroup.org/onlinepubs/9699919799/functions/geteuid.html
#[inline]
pub fn getuid() -> Uid {
    Uid(unsafe { libc::getuid() })
}

#[inline]
pub fn geteuid() -> Uid {
    Uid(unsafe { libc::geteuid() })
}

#[inline]
pub fn getgid() -> Gid {
    Gid(unsafe { libc::getgid() })
}

#[inline]
pub fn getegid() -> Gid {
    Gid(unsafe { libc::getegid() })
}

#[inline]
pub fn setuid(uid: Uid) -> Result<()> {
    let res = unsafe { libc::setuid(uid.into()) };

    Errno::result(res).map(drop)
}

#[inline]
pub fn setgid(gid: Gid) -> Result<()> {
    let res = unsafe { libc::setgid(gid.into()) };

    Errno::result(res).map(drop)
}

#[inline]
pub fn pause() -> Result<()> {
    let res = unsafe { libc::pause() };

    Errno::result(res).map(drop)
}

#[inline]
// Per POSIX, does not fail:
//   http://pubs.opengroup.org/onlinepubs/009695399/functions/sleep.html#tag_03_705_05
pub fn sleep(seconds: libc::c_uint) -> c_uint {
    unsafe { libc::sleep(seconds) }
}

/// Creates a regular file which persists even after process termination
///
/// * `template`: a path whose 6 rightmost characters must be X, e.g. /tmp/tmpfile_XXXXXX
/// * returns: tuple of file descriptor and filename
///
/// Err is returned either if no temporary filename could be created or the template doesn't
/// end with XXXXXX
///
/// # Example
///
/// ```rust
/// use nix::unistd;
///
/// let _ = match unistd::mkstemp("/tmp/tempfile_XXXXXX") {
///     Ok((fd, path)) => {
///         unistd::unlink(path.as_path()).unwrap(); // flag file to be deleted at app termination
///         fd
///     }
///     Err(e) => panic!("mkstemp failed: {}", e)
/// };
/// // do something with fd
/// ```
#[inline]
pub fn mkstemp<P: ?Sized + NixPath>(template: &P) -> Result<(RawFd, PathBuf)> {
    let mut path = try!(template.with_nix_path(|path| {path.to_bytes_with_nul().to_owned()}));
    let p = path.as_mut_ptr() as *mut _;
    let fd = unsafe { libc::mkstemp(p) };
    let last = path.pop(); // drop the trailing nul
    debug_assert!(last == Some(b'\0'));
    let pathname = OsString::from_vec(path);
    try!(Errno::result(fd));
    Ok((fd, PathBuf::from(pathname)))
}

/// Variable names for `pathconf`
///
/// Nix uses the same naming convention for these variables as the
/// [getconf(1)](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/getconf.html) utility.
/// That is, `PathconfVar` variables have the same name as the abstract
/// variables  shown in the `pathconf(2)` man page.  Usually, it's the same as
/// the C variable name without the leading `_PC_`.
///
/// POSIX 1003.1-2008 standardizes all of these variables, but some OSes choose
/// not to implement variables that cannot change at runtime.
///
/// # References
///
/// - [pathconf(2)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/pathconf.html)
/// - [limits.h](http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/limits.h.html)
/// - [unistd.h](http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/unistd.h.html)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum PathconfVar {
    #[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "linux",
              target_os = "netbsd", target_os = "openbsd"))]
    /// Minimum number of bits needed to represent, as a signed integer value,
    /// the maximum size of a regular file allowed in the specified directory.
    FILESIZEBITS = libc::_PC_FILESIZEBITS,
    /// Maximum number of links to a single file.
    LINK_MAX = libc::_PC_LINK_MAX,
    /// Maximum number of bytes in a terminal canonical input line.
    MAX_CANON = libc::_PC_MAX_CANON,
    /// Minimum number of bytes for which space is available in a terminal input
    /// queue; therefore, the maximum number of bytes a conforming application
    /// may require to be typed as input before reading them.
    MAX_INPUT = libc::_PC_MAX_INPUT,
    /// Maximum number of bytes in a filename (not including the terminating
    /// null of a filename string).
    NAME_MAX = libc::_PC_NAME_MAX,
    /// Maximum number of bytes the implementation will store as a pathname in a
    /// user-supplied buffer of unspecified size, including the terminating null
    /// character. Minimum number the implementation will accept as the maximum
    /// number of bytes in a pathname.
    PATH_MAX = libc::_PC_PATH_MAX,
    /// Maximum number of bytes that is guaranteed to be atomic when writing to
    /// a pipe.
    PIPE_BUF = libc::_PC_PIPE_BUF,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "linux",
              target_os = "netbsd", target_os = "openbsd"))]
    /// Symbolic links can be created.
    POSIX2_SYMLINKS = libc::_PC_2_SYMLINKS,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd",
              target_os = "linux", target_os = "openbsd"))]
    /// Minimum number of bytes of storage actually allocated for any portion of
    /// a file.
    POSIX_ALLOC_SIZE_MIN = libc::_PC_ALLOC_SIZE_MIN,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd",
              target_os = "linux", target_os = "openbsd"))]
    /// Recommended increment for file transfer sizes between the
    /// `POSIX_REC_MIN_XFER_SIZE` and `POSIX_REC_MAX_XFER_SIZE` values.
    POSIX_REC_INCR_XFER_SIZE = libc::_PC_REC_INCR_XFER_SIZE,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd",
              target_os = "linux", target_os = "openbsd"))]
    /// Maximum recommended file transfer size.
    POSIX_REC_MAX_XFER_SIZE = libc::_PC_REC_MAX_XFER_SIZE,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd",
              target_os = "linux", target_os = "openbsd"))]
    /// Minimum recommended file transfer size.
    POSIX_REC_MIN_XFER_SIZE = libc::_PC_REC_MIN_XFER_SIZE,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd",
              target_os = "linux", target_os = "openbsd"))]
    ///  Recommended file transfer buffer alignment.
    POSIX_REC_XFER_ALIGN = libc::_PC_REC_XFER_ALIGN,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd",
              target_os = "linux", target_os = "netbsd", target_os = "openbsd"))]
    /// Maximum number of bytes in a symbolic link.
    SYMLINK_MAX = libc::_PC_SYMLINK_MAX,
    /// The use of `chown` and `fchown` is restricted to a process with
    /// appropriate privileges, and to changing the group ID of a file only to
    /// the effective group ID of the process or to one of its supplementary
    /// group IDs.
    _POSIX_CHOWN_RESTRICTED = libc::_PC_CHOWN_RESTRICTED,
    /// Pathname components longer than {NAME_MAX} generate an error.
    _POSIX_NO_TRUNC = libc::_PC_NO_TRUNC,
    /// This symbol shall be defined to be the value of a character that shall
    /// disable terminal special character handling.
    _POSIX_VDISABLE = libc::_PC_VDISABLE,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd",
              target_os = "linux", target_os = "openbsd"))]
    /// Asynchronous input or output operations may be performed for the
    /// associated file.
    _POSIX_ASYNC_IO = libc::_PC_ASYNC_IO,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd",
              target_os = "linux", target_os = "openbsd"))]
    /// Prioritized input or output operations may be performed for the
    /// associated file.
    _POSIX_PRIO_IO = libc::_PC_PRIO_IO,
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd",
              target_os = "linux", target_os = "netbsd", target_os = "openbsd"))]
    /// Synchronized input or output operations may be performed for the
    /// associated file.
    _POSIX_SYNC_IO = libc::_PC_SYNC_IO,
    #[cfg(any(target_os = "dragonfly", target_os = "openbsd"))]
    /// The resolution in nanoseconds for all file timestamps.
    _POSIX_TIMESTAMP_RESOLUTION = libc::_PC_TIMESTAMP_RESOLUTION
}

/// Like `pathconf`, but works with file descriptors instead of paths (see
/// [fpathconf(2)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/pathconf.html))
///
/// # Parameters
///
/// - `fd`:   The file descriptor whose variable should be interrogated
/// - `var`:  The pathconf variable to lookup
///
/// # Returns
///
/// - `Ok(Some(x))`: the variable's limit (for limit variables) or its
///     implementation level (for option variables).  Implementation levels are
///     usually a decimal-coded date, such as 200112 for POSIX 2001.12
/// - `Ok(None)`: the variable has no limit (for limit variables) or is
///     unsupported (for option variables)
/// - `Err(x)`: an error occurred
pub fn fpathconf(fd: RawFd, var: PathconfVar) -> Result<Option<c_long>> {
    let raw = unsafe {
        Errno::clear();
        libc::fpathconf(fd, var as c_int)
    };
    if raw == -1 {
        if errno::errno() == 0 {
            Ok(None)
        } else {
            Err(Error::Sys(Errno::last()))
        }
    } else {
        Ok(Some(raw))
    }
}

/// Get path-dependent configurable system variables (see
/// [pathconf(2)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/pathconf.html))
///
/// Returns the value of a path-dependent configurable system variable.  Most
/// supported variables also have associated compile-time constants, but POSIX
/// allows their values to change at runtime.  There are generally two types of
/// `pathconf` variables: options and limits.  See [pathconf(2)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/pathconf.html) for more details.
///
/// # Parameters
///
/// - `path`: Lookup the value of `var` for this file or directory
/// - `var`:  The `pathconf` variable to lookup
///
/// # Returns
///
/// - `Ok(Some(x))`: the variable's limit (for limit variables) or its
///     implementation level (for option variables).  Implementation levels are
///     usually a decimal-coded date, such as 200112 for POSIX 2001.12
/// - `Ok(None)`: the variable has no limit (for limit variables) or is
///     unsupported (for option variables)
/// - `Err(x)`: an error occurred
pub fn pathconf<P: ?Sized + NixPath>(path: &P, var: PathconfVar) -> Result<Option<c_long>> {
    let raw = try!(path.with_nix_path(|cstr| {
        unsafe {
            Errno::clear();
            libc::pathconf(cstr.as_ptr(), var as c_int)
        }
    }));
    if raw == -1 {
        if errno::errno() == 0 {
            Ok(None)
        } else {
            Err(Error::Sys(Errno::last()))
        }
    } else {
        Ok(Some(raw))
    }
}

/// Variable names for `sysconf`
///
/// Nix uses the same naming convention for these variables as the
/// [getconf(1)](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/getconf.html) utility.
/// That is, `SysconfVar` variables have the same name as the abstract variables
/// shown in the `sysconf(3)` man page.  Usually, it's the same as the C
/// variable name without the leading `_SC_`.
///
/// All of these symbols are standardized by POSIX 1003.1-2008, but haven't been
/// implemented by all platforms.
///
/// # References
///
/// - [sysconf(3)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/sysconf.html)
/// - [unistd.h](http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/unistd.h.html)
/// - [limits.h](http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/limits.h.html)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum SysconfVar {
    /// Maximum number of I/O operations in a single list I/O call supported by
    /// the implementation.
    AIO_LISTIO_MAX = libc::_SC_AIO_LISTIO_MAX,
    /// Maximum number of outstanding asynchronous I/O operations supported by
    /// the implementation.
    AIO_MAX = libc::_SC_AIO_MAX,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    /// The maximum amount by which a process can decrease its asynchronous I/O
    /// priority level from its own scheduling priority.
    AIO_PRIO_DELTA_MAX = libc::_SC_AIO_PRIO_DELTA_MAX,
    /// Maximum length of argument to the exec functions including environment data.
    ARG_MAX = libc::_SC_ARG_MAX,
    /// Maximum number of functions that may be registered with `atexit`.
    ATEXIT_MAX = libc::_SC_ATEXIT_MAX,
    /// Maximum obase values allowed by the bc utility.
    BC_BASE_MAX = libc::_SC_BC_BASE_MAX,
    /// Maximum number of elements permitted in an array by the bc utility.
    BC_DIM_MAX = libc::_SC_BC_DIM_MAX,
    /// Maximum scale value allowed by the bc utility.
    BC_SCALE_MAX = libc::_SC_BC_SCALE_MAX,
    /// Maximum length of a string constant accepted by the bc utility.
    BC_STRING_MAX = libc::_SC_BC_STRING_MAX,
    /// Maximum number of simultaneous processes per real user ID.
    CHILD_MAX = libc::_SC_CHILD_MAX,
    // _SC_CLK_TCK is obsolete
    /// Maximum number of weights that can be assigned to an entry of the
    /// LC_COLLATE order keyword in the locale definition file
    COLL_WEIGHTS_MAX = libc::_SC_COLL_WEIGHTS_MAX,
    /// Maximum number of timer expiration overruns.
    DELAYTIMER_MAX = libc::_SC_DELAYTIMER_MAX,
    /// Maximum number of expressions that can be nested within parentheses by
    /// the expr utility.
    EXPR_NEST_MAX = libc::_SC_EXPR_NEST_MAX,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// Maximum length of a host name (not including the terminating null) as
    /// returned from the `gethostname` function
    HOST_NAME_MAX = libc::_SC_HOST_NAME_MAX,
    /// Maximum number of iovec structures that one process has available for
    /// use with `readv` or `writev`.
    IOV_MAX = libc::_SC_IOV_MAX,
    /// Unless otherwise noted, the maximum length, in bytes, of a utility's
    /// input line (either standard input or another file), when the utility is
    /// described as processing text files. The length includes room for the
    /// trailing <newline>.
    LINE_MAX = libc::_SC_LINE_MAX,
    /// Maximum length of a login name.
    LOGIN_NAME_MAX = libc::_SC_LOGIN_NAME_MAX,
    /// Maximum number of simultaneous supplementary group IDs per process.
    NGROUPS_MAX = libc::_SC_NGROUPS_MAX,
    /// Initial size of `getgrgid_r` and `getgrnam_r` data buffers
    GETGR_R_SIZE_MAX = libc::_SC_GETGR_R_SIZE_MAX,
    /// Initial size of `getpwuid_r` and `getpwnam_r` data buffers
    GETPW_R_SIZE_MAX = libc::_SC_GETPW_R_SIZE_MAX,
    /// The maximum number of open message queue descriptors a process may hold.
    MQ_OPEN_MAX = libc::_SC_MQ_OPEN_MAX,
    /// The maximum number of message priorities supported by the implementation.
    MQ_PRIO_MAX = libc::_SC_MQ_PRIO_MAX,
    /// A value one greater than the maximum value that the system may assign to
    /// a newly-created file descriptor.
    OPEN_MAX = libc::_SC_OPEN_MAX,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the Advisory Information option. 
    _POSIX_ADVISORY_INFO = libc::_SC_ADVISORY_INFO,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports barriers.
    _POSIX_BARRIERS = libc::_SC_BARRIERS,
    /// The implementation supports asynchronous input and output.
    _POSIX_ASYNCHRONOUS_IO = libc::_SC_ASYNCHRONOUS_IO,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports clock selection.
    _POSIX_CLOCK_SELECTION = libc::_SC_CLOCK_SELECTION,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Process CPU-Time Clocks option.
    _POSIX_CPUTIME = libc::_SC_CPUTIME,
    /// The implementation supports the File Synchronization option. 
    _POSIX_FSYNC = libc::_SC_FSYNC,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the IPv6 option.
    _POSIX_IPV6 = libc::_SC_IPV6,
    /// The implementation supports job control.
    _POSIX_JOB_CONTROL = libc::_SC_JOB_CONTROL,
    /// The implementation supports memory mapped Files.
    _POSIX_MAPPED_FILES = libc::_SC_MAPPED_FILES,
    /// The implementation supports the Process Memory Locking option.
    _POSIX_MEMLOCK = libc::_SC_MEMLOCK,
    /// The implementation supports the Range Memory Locking option.
    _POSIX_MEMLOCK_RANGE = libc::_SC_MEMLOCK_RANGE,
    /// The implementation supports memory protection.
    _POSIX_MEMORY_PROTECTION = libc::_SC_MEMORY_PROTECTION,
    /// The implementation supports the Message Passing option.
    _POSIX_MESSAGE_PASSING = libc::_SC_MESSAGE_PASSING,
    /// The implementation supports the Monotonic Clock option.
    _POSIX_MONOTONIC_CLOCK = libc::_SC_MONOTONIC_CLOCK,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    /// The implementation supports the Prioritized Input and Output option.
    _POSIX_PRIORITIZED_IO = libc::_SC_PRIORITIZED_IO,
    /// The implementation supports the Process Scheduling option.
    _POSIX_PRIORITY_SCHEDULING = libc::_SC_PRIORITY_SCHEDULING,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the Raw Sockets option.
    _POSIX_RAW_SOCKETS = libc::_SC_RAW_SOCKETS,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports read-write locks.
    _POSIX_READER_WRITER_LOCKS = libc::_SC_READER_WRITER_LOCKS,
    #[cfg(any(target_os = "android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os = "openbsd"))]
    /// The implementation supports realtime signals.
    _POSIX_REALTIME_SIGNALS = libc::_SC_REALTIME_SIGNALS,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Regular Expression Handling option.
    _POSIX_REGEXP = libc::_SC_REGEXP,
    /// Each process has a saved set-user-ID and a saved set-group-ID.
    _POSIX_SAVED_IDS = libc::_SC_SAVED_IDS,
    /// The implementation supports semaphores.
    _POSIX_SEMAPHORES = libc::_SC_SEMAPHORES,
    /// The implementation supports the Shared Memory Objects option.
    _POSIX_SHARED_MEMORY_OBJECTS = libc::_SC_SHARED_MEMORY_OBJECTS,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the POSIX shell.
    _POSIX_SHELL = libc::_SC_SHELL,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Spawn option.
    _POSIX_SPAWN = libc::_SC_SPAWN,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports spin locks.
    _POSIX_SPIN_LOCKS = libc::_SC_SPIN_LOCKS,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the Process Sporadic Server option.
    _POSIX_SPORADIC_SERVER = libc::_SC_SPORADIC_SERVER,
    #[cfg(any(target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    _POSIX_SS_REPL_MAX = libc::_SC_SS_REPL_MAX,
    /// The implementation supports the Synchronized Input and Output option.
    _POSIX_SYNCHRONIZED_IO = libc::_SC_SYNCHRONIZED_IO,
    /// The implementation supports the Thread Stack Address Attribute option.
    _POSIX_THREAD_ATTR_STACKADDR = libc::_SC_THREAD_ATTR_STACKADDR,
    /// The implementation supports the Thread Stack Size Attribute option.
    _POSIX_THREAD_ATTR_STACKSIZE = libc::_SC_THREAD_ATTR_STACKSIZE,
    #[cfg(any(target_os = "ios", target_os="linux", target_os = "macos",
              target_os="netbsd", target_os="openbsd"))]
    /// The implementation supports the Thread CPU-Time Clocks option.
    _POSIX_THREAD_CPUTIME = libc::_SC_THREAD_CPUTIME,
    /// The implementation supports the Non-Robust Mutex Priority Inheritance
    /// option.
    _POSIX_THREAD_PRIO_INHERIT = libc::_SC_THREAD_PRIO_INHERIT,
    /// The implementation supports the Non-Robust Mutex Priority Protection option.
    _POSIX_THREAD_PRIO_PROTECT = libc::_SC_THREAD_PRIO_PROTECT,
    /// The implementation supports the Thread Execution Scheduling option.
    _POSIX_THREAD_PRIORITY_SCHEDULING = libc::_SC_THREAD_PRIORITY_SCHEDULING,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Thread Process-Shared Synchronization
    /// option.
    _POSIX_THREAD_PROCESS_SHARED = libc::_SC_THREAD_PROCESS_SHARED,
    #[cfg(any(target_os="dragonfly", target_os="linux", target_os="openbsd"))]
    /// The implementation supports the Robust Mutex Priority Inheritance option.
    _POSIX_THREAD_ROBUST_PRIO_INHERIT = libc::_SC_THREAD_ROBUST_PRIO_INHERIT,
    #[cfg(any(target_os="dragonfly", target_os="linux", target_os="openbsd"))]
    /// The implementation supports the Robust Mutex Priority Protection option.
    _POSIX_THREAD_ROBUST_PRIO_PROTECT = libc::_SC_THREAD_ROBUST_PRIO_PROTECT,
    /// The implementation supports thread-safe functions.
    _POSIX_THREAD_SAFE_FUNCTIONS = libc::_SC_THREAD_SAFE_FUNCTIONS,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the Thread Sporadic Server option.
    _POSIX_THREAD_SPORADIC_SERVER = libc::_SC_THREAD_SPORADIC_SERVER,
    /// The implementation supports threads.
    _POSIX_THREADS = libc::_SC_THREADS,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports timeouts.
    _POSIX_TIMEOUTS = libc::_SC_TIMEOUTS,
    /// The implementation supports timers. 
    _POSIX_TIMERS = libc::_SC_TIMERS,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the Trace option.
    _POSIX_TRACE = libc::_SC_TRACE,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the Trace Event Filter option.
    _POSIX_TRACE_EVENT_FILTER = libc::_SC_TRACE_EVENT_FILTER,
    #[cfg(any(target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    _POSIX_TRACE_EVENT_NAME_MAX = libc::_SC_TRACE_EVENT_NAME_MAX,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the Trace Inherit option.
    _POSIX_TRACE_INHERIT = libc::_SC_TRACE_INHERIT,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the Trace Log option.
    _POSIX_TRACE_LOG = libc::_SC_TRACE_LOG,
    #[cfg(any(target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    _POSIX_TRACE_NAME_MAX = libc::_SC_TRACE_NAME_MAX,
    #[cfg(any(target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    _POSIX_TRACE_SYS_MAX = libc::_SC_TRACE_SYS_MAX,
    #[cfg(any(target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    _POSIX_TRACE_USER_EVENT_MAX = libc::_SC_TRACE_USER_EVENT_MAX,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the Typed Memory Objects option.
    _POSIX_TYPED_MEMORY_OBJECTS = libc::_SC_TYPED_MEMORY_OBJECTS,
    /// Integer value indicating version of this standard (C-language binding)
    /// to which the implementation conforms. For implementations conforming to
    /// POSIX.1-2008, the value shall be 200809L.
    _POSIX_VERSION = libc::_SC_VERSION,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation provides a C-language compilation environment with
    /// 32-bit `int`, `long`, `pointer`, and `off_t` types.
    _POSIX_V6_ILP32_OFF32 = libc::_SC_V6_ILP32_OFF32,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation provides a C-language compilation environment with
    /// 32-bit `int`, `long`, and pointer types and an `off_t` type using at
    /// least 64 bits.
    _POSIX_V6_ILP32_OFFBIG = libc::_SC_V6_ILP32_OFFBIG,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation provides a C-language compilation environment with
    /// 32-bit `int` and 64-bit `long`, `pointer`, and `off_t` types.
    _POSIX_V6_LP64_OFF64 = libc::_SC_V6_LP64_OFF64,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation provides a C-language compilation environment with an
    /// `int` type using at least 32 bits and `long`, pointer, and `off_t` types
    /// using at least 64 bits.
    _POSIX_V6_LPBIG_OFFBIG = libc::_SC_V6_LPBIG_OFFBIG,
    /// The implementation supports the C-Language Binding option.
    _POSIX2_C_BIND = libc::_SC_2_C_BIND,
    /// The implementation supports the C-Language Development Utilities option.
    _POSIX2_C_DEV = libc::_SC_2_C_DEV,
    /// The implementation supports the Terminal Characteristics option.
    _POSIX2_CHAR_TERM = libc::_SC_2_CHAR_TERM,
    /// The implementation supports the FORTRAN Development Utilities option.
    _POSIX2_FORT_DEV = libc::_SC_2_FORT_DEV,
    /// The implementation supports the FORTRAN Runtime Utilities option.
    _POSIX2_FORT_RUN = libc::_SC_2_FORT_RUN,
    /// The implementation supports the creation of locales by the localedef
    /// utility.
    _POSIX2_LOCALEDEF = libc::_SC_2_LOCALEDEF,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Batch Environment Services and Utilities
    /// option.
    _POSIX2_PBS = libc::_SC_2_PBS,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Batch Accounting option.
    _POSIX2_PBS_ACCOUNTING = libc::_SC_2_PBS_ACCOUNTING,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Batch Checkpoint/Restart option.
    _POSIX2_PBS_CHECKPOINT = libc::_SC_2_PBS_CHECKPOINT,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Locate Batch Job Request option.
    _POSIX2_PBS_LOCATE = libc::_SC_2_PBS_LOCATE,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Batch Job Message Request option.
    _POSIX2_PBS_MESSAGE = libc::_SC_2_PBS_MESSAGE,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    /// The implementation supports the Track Batch Job Request option.
    _POSIX2_PBS_TRACK = libc::_SC_2_PBS_TRACK,
    /// The implementation supports the Software Development Utilities option.
    _POSIX2_SW_DEV = libc::_SC_2_SW_DEV,
    /// The implementation supports the User Portability Utilities option.
    _POSIX2_UPE = libc::_SC_2_UPE,
    /// Integer value indicating version of the Shell and Utilities volume of
    /// POSIX.1 to which the implementation conforms.
    _POSIX2_VERSION = libc::_SC_2_VERSION,
    /// The size of a system page in bytes.
    ///
    /// POSIX also defines an alias named `PAGESIZE`, but Rust does not allow two
    /// enum constants to have the same value, so nix omits `PAGESIZE`.
    PAGE_SIZE = libc::_SC_PAGE_SIZE,
    PTHREAD_DESTRUCTOR_ITERATIONS = libc::_SC_THREAD_DESTRUCTOR_ITERATIONS,
    PTHREAD_KEYS_MAX = libc::_SC_THREAD_KEYS_MAX,
    PTHREAD_STACK_MIN = libc::_SC_THREAD_STACK_MIN,
    PTHREAD_THREADS_MAX = libc::_SC_THREAD_THREADS_MAX,
    RE_DUP_MAX = libc::_SC_RE_DUP_MAX,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    RTSIG_MAX = libc::_SC_RTSIG_MAX,
    SEM_NSEMS_MAX = libc::_SC_SEM_NSEMS_MAX,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    SEM_VALUE_MAX = libc::_SC_SEM_VALUE_MAX,
    #[cfg(any(target_os = "android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os = "openbsd"))]
    SIGQUEUE_MAX = libc::_SC_SIGQUEUE_MAX,
    STREAM_MAX = libc::_SC_STREAM_MAX,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="netbsd",
              target_os="openbsd"))]
    SYMLOOP_MAX = libc::_SC_SYMLOOP_MAX,
    TIMER_MAX = libc::_SC_TIMER_MAX,
    TTY_NAME_MAX = libc::_SC_TTY_NAME_MAX,
    TZNAME_MAX = libc::_SC_TZNAME_MAX,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    /// The implementation supports the X/Open Encryption Option Group.
    _XOPEN_CRYPT = libc::_SC_XOPEN_CRYPT,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    /// The implementation supports the Issue 4, Version 2 Enhanced
    /// Internationalization Option Group.
    _XOPEN_ENH_I18N = libc::_SC_XOPEN_ENH_I18N,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    _XOPEN_LEGACY = libc::_SC_XOPEN_LEGACY,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    /// The implementation supports the X/Open Realtime Option Group.
    _XOPEN_REALTIME = libc::_SC_XOPEN_REALTIME,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    /// The implementation supports the X/Open Realtime Threads Option Group.
    _XOPEN_REALTIME_THREADS = libc::_SC_XOPEN_REALTIME_THREADS,
    /// The implementation supports the Issue 4, Version 2 Shared Memory Option
    /// Group.
    _XOPEN_SHM = libc::_SC_XOPEN_SHM,
    #[cfg(any(target_os="dragonfly", target_os="freebsd", target_os = "ios",
              target_os="linux", target_os = "macos", target_os="openbsd"))]
    /// The implementation supports the XSI STREAMS Option Group.
    _XOPEN_STREAMS = libc::_SC_XOPEN_STREAMS,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    /// The implementation supports the XSI option
    _XOPEN_UNIX = libc::_SC_XOPEN_UNIX,
    #[cfg(any(target_os="android", target_os="dragonfly", target_os="freebsd",
              target_os = "ios", target_os="linux", target_os = "macos",
              target_os="openbsd"))]
    /// Integer value indicating version of the X/Open Portability Guide to
    /// which the implementation conforms.
    _XOPEN_VERSION = libc::_SC_XOPEN_VERSION,
}

/// Get configurable system variables (see
/// [sysconf(3)](http://pubs.opengroup.org/onlinepubs/9699919799/functions/sysconf.html))
///
/// Returns the value of a configurable system variable.  Most supported
/// variables also have associated compile-time constants, but POSIX
/// allows their values to change at runtime.  There are generally two types of
/// sysconf variables: options and limits.  See sysconf(3) for more details.
///
/// # Returns
///
/// - `Ok(Some(x))`: the variable's limit (for limit variables) or its
///     implementation level (for option variables).  Implementation levels are
///     usually a decimal-coded date, such as 200112 for POSIX 2001.12
/// - `Ok(None)`: the variable has no limit (for limit variables) or is
///     unsupported (for option variables)
/// - `Err(x)`: an error occurred
pub fn sysconf(var: SysconfVar) -> Result<Option<c_long>> {
    let raw = unsafe {
        Errno::clear();
        libc::sysconf(var as c_int)
    };
    if raw == -1 {
        if errno::errno() == 0 {
            Ok(None)
        } else {
            Err(Error::Sys(Errno::last()))
        }
    } else {
        Ok(Some(raw))
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux {
    use libc;
    use sys::syscall::{syscall, SYSPIVOTROOT};
    use {Errno, Result, NixPath};
    use super::{Uid, Gid};

    pub fn pivot_root<P1: ?Sized + NixPath, P2: ?Sized + NixPath>(
            new_root: &P1, put_old: &P2) -> Result<()> {
        let res = try!(try!(new_root.with_nix_path(|new_root| {
            put_old.with_nix_path(|put_old| {
                unsafe {
                    syscall(SYSPIVOTROOT, new_root.as_ptr(), put_old.as_ptr())
                }
            })
        })));

        Errno::result(res).map(drop)
    }

    /// Sets the real, effective, and saved uid.
    /// ([see setresuid(2)](http://man7.org/linux/man-pages/man2/setresuid.2.html))
    ///
    /// * `ruid`: real user id
    /// * `euid`: effective user id
    /// * `suid`: saved user id
    /// * returns: Ok or libc error code.
    ///
    /// Err is returned if the user doesn't have permission to set this UID.
    #[inline]
    pub fn setresuid(ruid: Uid, euid: Uid, suid: Uid) -> Result<()> {
        let res = unsafe { libc::setresuid(ruid.into(), euid.into(), suid.into()) };

        Errno::result(res).map(drop)
    }

    /// Sets the real, effective, and saved gid.
    /// ([see setresuid(2)](http://man7.org/linux/man-pages/man2/setresuid.2.html))
    ///
    /// * `rgid`: real user id
    /// * `egid`: effective user id
    /// * `sgid`: saved user id
    /// * returns: Ok or libc error code.
    ///
    /// Err is returned if the user doesn't have permission to set this GID.
    #[inline]
    pub fn setresgid(rgid: Gid, egid: Gid, sgid: Gid) -> Result<()> {
        let res = unsafe { libc::setresgid(rgid.into(), egid.into(), sgid.into()) };

        Errno::result(res).map(drop)
    }
}
