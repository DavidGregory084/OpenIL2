#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use]
extern crate lazy_static;
extern crate libc;
use jni::objects::*;
use jni::sys::{jbyteArray, jint, jlong};
use jni::*;
use std::ffi::CStr;
use std::fmt;
use std::sync::RwLock;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPCVOID, LPVOID, MAX_PATH};
use winapi::shared::ntdef::{FALSE, TRUE};
use winapi::um::fileapi::INVALID_SET_FILE_POINTER;
use winapi::um::processenv::GetCurrentDirectoryA;
use winapi::um::winbase::{FILE_BEGIN, FILE_CURRENT, FILE_END};
use winapi::um::winnt::{BOOLEAN, CHAR, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, LONG, LPSTR};

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_fileLength(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jlong {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        printErr(
            env,
            format!("PhysFSInputStream#fileLength for file {:?}", file),
        );
    }

    unsafe { PHYSFS_fileLength(file) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_tell(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jlong {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        printErr(env, format!("PhysFSInputStream#tell for file {:?}", file));
    }

    unsafe { PHYSFS_tell(file) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_readBytes(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
    java_buf: jbyteArray,
    offset: jint,
    len: jint,
) -> jint {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        printErr(
            env,
            format!(
                "PhysFSInputStream#readBytes for file {:?} offset {} len {}",
                file, offset, len
            ),
        );
    }

    unsafe {
        let mut vec = vec![0 as i8; len as usize];
        let c_buf = &mut vec[..];

        let res = PHYSFS_readBytes(
            file,
            c_buf.as_mut_ptr() as *mut libc::c_void,
            len as PHYSFS_uint64,
        ) as jint;

        env.set_byte_array_region(java_buf, offset, c_buf).unwrap();

        res
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_openRead(
    env: JNIEnv,
    obj: JObject,
    file_name: JString,
) -> jlong {
    let file_str = env.get_string(file_name).unwrap();

    if cfg!(debug_assertions) {
        printErr(
            env,
            format!(
                "PhysFSInputStream#openRead for file {}",
                file_str.to_str().unwrap()
            ),
        );
    }

    unsafe { PHYSFS_openRead(file_str.as_ptr()) as jlong }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_seek(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
    pos: jlong,
) -> jint {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        printErr(
            env,
            format!("PhysFSInputStream#seek for file {:?} pos {}", file, pos),
        );
    }

    unsafe { PHYSFS_seek(file, pos as PHYSFS_uint64) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_eof(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jint {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        printErr(env, format!("PhysFSInputStream#eof for file {:?}", file));
    }

    unsafe { PHYSFS_eof(file) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_close(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jint {
    let file = fd as *mut PHYSFS_File;

    if cfg!(debug_assertions) {
        printErr(env, format!("PhysFSInputStream#close for file {:?}", file));
    }

    unsafe { PHYSFS_close(file) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_exists(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
) -> jint {
    let file_str = env.get_string(file_name).unwrap();

    if cfg!(debug_assertions) {
        printErr(
            env,
            format!("PhysFS.exists for file {}", file_str.to_str().unwrap()),
        );
    }

    unsafe { PHYSFS_exists(file_str.as_ptr()) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_mount(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
    append: jint,
) -> jint {
    let file_str = env.get_string(file_name).unwrap();

    if cfg!(debug_assertions) {
        printErr(
            env,
            format!(
                "PhysFS.mount for file {} append {}",
                file_str.to_str().unwrap(),
                append
            ),
        );
    }

    unsafe { PHYSFS_mount(file_str.as_ptr(), std::ptr::null(), append) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_mountAt(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
    mount_point: JString,
    append: jint,
) -> jint {
    let file_str = env.get_string(file_name).unwrap();
    let mount_point_str = env.get_string(mount_point).unwrap();

    if cfg!(debug_assertions) {
        printErr(
            env,
            format!(
                "PhysFS.mountAt for file {} mount_point {} append {}",
                file_str.to_str().unwrap(),
                mount_point_str.to_str().unwrap(),
                append
            ),
        );
    }

    unsafe { PHYSFS_mount(file_str.as_ptr(), mount_point_str.as_ptr(), append) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_unmount(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
) -> jint {
    let file_str = env.get_string(file_name).unwrap();

    if cfg!(debug_assertions) {
        printErr(
            env,
            format!("PhysFS.unmount for file {}", file_str.to_str().unwrap()),
        );
    }

    unsafe { PHYSFS_unmount(file_str.as_ptr()) }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_getLastErrorCode(
    env: JNIEnv,
    class: JClass,
) -> jint {
    if cfg!(debug_assertions) {
        printErr(env, "PhysFS.getLastErrorCode".to_string());
    }

    unsafe { PHYSFS_getLastErrorCode() }
}

static RTS_INTERFACE: RTSInterface = RTSInterface {
    get_current_real_time: RTS_GetCurrentRealTime,
    get_current_game_time: RTS_GetCurrentGameTime,
    open_file: open_file,
    read_file: read_file,
    write_file: write_file,
    seek_file: seek_file,
    close_file: close_file,
};

static mut OPEN_FILES: *mut FileHandle = std::ptr::null_mut();

lazy_static! {
    static ref JAVA_VM: RwLock<Option<JavaVM>> = RwLock::new(None);
}

#[repr(C)]
#[derive(Clone, Debug)]
struct FileHandle {
    size: usize,
    next_handle: *mut FileHandle,
    physfs_file: *mut PHYSFS_File,
}

unsafe impl Send for FileHandle {}
unsafe impl Sync for FileHandle {}

#[repr(C)]
struct RTSInterface {
    get_current_real_time: unsafe extern "C" fn() -> i64,
    get_current_game_time: unsafe extern "C" fn() -> i64,
    open_file: unsafe extern "system" fn(LPSTR, u32) -> i32,
    read_file: unsafe extern "system" fn(*mut FileHandle, LPVOID, DWORD) -> BOOL,
    write_file: unsafe extern "system" fn(*mut FileHandle, LPCVOID, DWORD) -> BOOL,
    seek_file: unsafe extern "system" fn(*mut FileHandle, LONG, DWORD) -> DWORD,
    close_file: unsafe extern "system" fn(*mut FileHandle) -> i32,
}

impl std::fmt::Debug for RTSInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RTSInterface")
            .field(
                "get_current_real_time",
                &(&self.get_current_real_time as *const _),
            )
            .field(
                "get_current_game_time",
                &(&self.get_current_game_time as *const _),
            )
            .field("open_file", &(&self.open_file as *const _))
            .field("read_file", &(&self.read_file as *const _))
            .field("write_file", &(&self.write_file as *const _))
            .field("seek_file", &(&self.seek_file as *const _))
            .field("close_file", &(&self.close_file as *const _))
            .finish()
    }
}

#[link(name = "rts", kind = "dylib")]
extern "C" {
    #[link_name = "_RTS_GetCurrentGameTime@0"]
    fn RTS_GetCurrentGameTime() -> i64;
    #[link_name = "_RTS_GetCurrentRealTime@0"]
    fn RTS_GetCurrentRealTime() -> i64;
}

fn printErr(env: JNIEnv, message: String) {
    let newline_string = format!("{}\n", message);
    let jstring = env.new_string(newline_string).unwrap();
    let _ = env.call_static_method(
        "com/maddox/rts/RTS",
        "cppErrPrint",
        "(Ljava/lang/String;)V",
        &[JValue::Object(*jstring)],
    );
}

fn cppErrPrint(message: String) {
    let jvm = JAVA_VM.read().unwrap();
    let env = (*jvm).as_ref().unwrap().get_env().unwrap();
    let newline_string = format!("{}\n", message);
    let jstring = env.new_string(newline_string).unwrap();
    let _ = env.call_static_method(
        "com/maddox/rts/RTS",
        "cppErrPrint",
        "(Ljava/lang/String;)V",
        &[JValue::Object(*jstring)],
    );
}

unsafe extern "system" fn open_file(file_name: LPSTR, mask: u32) -> i32 {
    if cfg!(debug_assertions) {
        let file_str = CStr::from_ptr(file_name).to_str().unwrap();
        cppErrPrint(format!("open_file for file {} mask {}", file_str, mask));
        if OPEN_FILES.is_null() {
            cppErrPrint("initial file list is empty".to_string());
        } else {
            cppErrPrint(format!("initial file list {:?}", *OPEN_FILES));
        }
    }

    let physfs_file: *mut PHYSFS_File;

    // GENERIC_WRITE
    if mask & 1 != 0 || mask & 2 != 0 {
        // TRUNCATE_EXISTING
        if mask & 512 != 0 {
            physfs_file = PHYSFS_openWrite(file_name);
        // CREATE_ALWAYS
        } else if mask & 256 != 0 {
            physfs_file = PHYSFS_openWrite(file_name);
        // OPEN_EXISTING
        } else {
            physfs_file = PHYSFS_openAppend(file_name);
        }
    // GENERIC_READ
    } else {
        physfs_file = PHYSFS_openRead(file_name);
    };

    if cfg!(debug_assertions) {
        let file_str = CStr::from_ptr(file_name).to_str().unwrap();
        cppErrPrint(format!(
            "open_file for file {} returning PhysFS handle {:p}",
            file_str, physfs_file,
        ));
    }

    if physfs_file.is_null() {
        if cfg!(debug_assertions) {
            let file_str = CStr::from_ptr(file_name).to_str().unwrap();
            let error = PHYSFS_getLastErrorCode();
            let msg = CStr::from_ptr(PHYSFS_getErrorByCode(error))
                .to_str()
                .unwrap();
            cppErrPrint(format!(
                "failed to open file {} due to PhysFS error {}: {}",
                file_str, error, msg
            ));
        }
        return -1;
    } else {
        if PHYSFS_seek(physfs_file, 0) == 0 {
            if cfg!(debug_assertions) {
                let file_str = CStr::from_ptr(file_name).to_str().unwrap();
                let error = PHYSFS_getLastErrorCode();
                let msg = CStr::from_ptr(PHYSFS_getErrorByCode(error))
                    .to_str()
                    .unwrap();
                cppErrPrint(format!(
                    "failed to seek to start of file {} due to PhysFS error {}: {}",
                    file_str, error, msg
                ));
            }
            return -1;
        } else {
            if cfg!(debug_assertions) {
                let file_str = CStr::from_ptr(file_name).to_str().unwrap();
                cppErrPrint(format!(
                    "open_file for file {} updating file list",
                    file_str
                ));
            }

            let next_handle: *mut FileHandle = if OPEN_FILES.is_null() {
                std::ptr::null_mut()
            } else {
                std::mem::replace(&mut OPEN_FILES, std::ptr::null_mut())
            };

            if cfg!(debug_assertions) {
                let file_str = CStr::from_ptr(file_name).to_str().unwrap();
                cppErrPrint(format!(
                    "open_file for file {} updated next_handle to {:p}",
                    file_str, next_handle
                ));
            }

            let new_file_list = Box::new(FileHandle {
                size: std::mem::size_of::<FileHandle>(),
                next_handle: next_handle,
                physfs_file: physfs_file,
            });

            if cfg!(debug_assertions) {
                let file_str = CStr::from_ptr(file_name).to_str().unwrap();
                cppErrPrint(format!(
                    "open_file for file {} created new handle {:?}",
                    file_str, new_file_list
                ));
            }

            OPEN_FILES = Box::leak(new_file_list);

            if cfg!(debug_assertions) {
                if OPEN_FILES.is_null() {
                    cppErrPrint("updated file list is empty".to_string());
                } else {
                    cppErrPrint(format!("updated file list {:?}", *OPEN_FILES));
                }
            }

            return OPEN_FILES as jint;
        }
    }
}

unsafe extern "system" fn read_file(
    handle: *mut FileHandle,
    buf: LPVOID,
    bytes_to_read: DWORD,
) -> BOOL {
    if cfg!(debug_assertions) {
        cppErrPrint(format!(
            "read_file called with handle {:?} bytes {}",
            handle, bytes_to_read
        ));
    }
    return PHYSFS_readBytes((*handle).physfs_file, buf, bytes_to_read.into()) as BOOL;
}

unsafe extern "system" fn write_file(
    handle: *mut FileHandle,
    buf: LPCVOID,
    bytes_to_write: DWORD,
) -> BOOL {
    if cfg!(debug_assertions) {
        cppErrPrint(format!(
            "write_file called with handle {:?} bytes {}",
            handle, bytes_to_write
        ));
    }
    return PHYSFS_writeBytes((*handle).physfs_file, buf, bytes_to_write.into()) as BOOL;
}

unsafe extern "system" fn seek_file(
    handle: *mut FileHandle,
    pos: LONG,
    move_method: DWORD,
) -> DWORD {
    if cfg!(debug_assertions) {
        cppErrPrint(format!(
            "seek_file called with handle {:?} pos {} move_method {}",
            handle, pos, move_method
        ));
    }

    if move_method == FILE_BEGIN {
        if PHYSFS_seek((*handle).physfs_file, pos as u64) != 0 {
            return PHYSFS_tell((*handle).physfs_file) as DWORD;
        } else {
            return INVALID_SET_FILE_POINTER;
        }
    } else if move_method == FILE_CURRENT {
        let current_pos = PHYSFS_tell((*handle).physfs_file);
        if current_pos == -1 {
            return INVALID_SET_FILE_POINTER;
        } else if pos != 0 {
            let desired_pos = current_pos as u64 + pos as u64;
            if PHYSFS_seek((*handle).physfs_file, desired_pos) > 0 {
                return PHYSFS_tell((*handle).physfs_file) as DWORD;
            } else {
                return INVALID_SET_FILE_POINTER;
            }
        } else {
            return current_pos as DWORD;
        }
    } else if move_method == FILE_END {
        let file_length = PHYSFS_fileLength((*handle).physfs_file);
        if file_length > 0 {
            let desired_pos = file_length as u64 + pos as u64;
            if PHYSFS_seek((*handle).physfs_file, desired_pos) > 0 {
                return PHYSFS_tell((*handle).physfs_file) as DWORD;
            } else {
                return INVALID_SET_FILE_POINTER;
            }
        } else {
            return INVALID_SET_FILE_POINTER;
        }
    } else {
        return INVALID_SET_FILE_POINTER;
    }
}

unsafe extern "system" fn close_file(handle: *mut FileHandle) -> BOOL {
    if cfg!(debug_assertions) {
        cppErrPrint(format!("close_file called with handle {:?}", *handle));
        if OPEN_FILES.is_null() {
            cppErrPrint("initial file list is empty".to_string());
        } else {
            cppErrPrint(format!("initial file list {:?}", *OPEN_FILES));
        }
    }

    if !handle.is_null() {
        if !OPEN_FILES.is_null() {
            let target_handle: *mut FileHandle = handle;
            let mut last_handle: *mut FileHandle = std::ptr::null_mut();
            let mut current_handle: *mut FileHandle = OPEN_FILES;

            while !current_handle.is_null() {
                if (*current_handle).physfs_file == (*target_handle).physfs_file {
                    if last_handle.is_null() {
                        OPEN_FILES = (*current_handle).next_handle;
                    } else {
                        (*last_handle).next_handle = (*current_handle).next_handle;
                    }
                    Box::from_raw(current_handle);
                    break;
                }
                last_handle = current_handle;
                current_handle = (*current_handle).next_handle;
            }

            if cfg!(debug_assertions) {
                if OPEN_FILES.is_null() {
                    cppErrPrint("updated file list is empty".to_string());
                } else {
                    cppErrPrint(format!("updated file list {:?}", *OPEN_FILES));
                }
            }

            return PHYSFS_close((*handle).physfs_file);
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_RTS_interf(env: JNIEnv, class: JClass) -> jint {
    // Save the Java VM for later
    let mut java_vm = JAVA_VM.write().unwrap();
    *java_vm = Some(env.get_java_vm().unwrap());

    if cfg!(debug_assertions) {
        printErr(env, format!("Returning RTS interface {:?}", &RTS_INTERFACE));
    }

    &RTS_INTERFACE as *const RTSInterface as jint
}

#[allow(unused_variables)]
#[no_mangle]
extern "system" fn DllMain(dllHandle: HINSTANCE, reason: DWORD, reserved: LPVOID) -> BOOLEAN {
    unsafe {
        match reason {
            DLL_PROCESS_ATTACH => {
                if PHYSFS_init(std::ptr::null()) != 0 {
                    let mut vec = vec![0 as CHAR; MAX_PATH as usize];
                    let c_str = &mut vec[..];

                    if GetCurrentDirectoryA(MAX_PATH as u32, c_str.as_mut_ptr() as *mut CHAR) > 0 {
                        if PHYSFS_setWriteDir(c_str.as_mut_ptr() as *mut CHAR) != 0 {
                            if PHYSFS_addToSearchPath(c_str.as_mut_ptr() as *mut CHAR, 0) != 0 {
                                return TRUE;
                            }
                        }
                    }
                }

                return FALSE;
            }
            DLL_PROCESS_DETACH => {
                if PHYSFS_deinit() != 0 {
                    return TRUE;
                }

                return FALSE;
            }
            _ => {
                return TRUE;
            }
        }
    }
}
