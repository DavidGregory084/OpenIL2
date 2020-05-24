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
use std::sync::RwLock;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPCVOID, LPVOID, MAX_PATH};
use winapi::shared::ntdef::{FALSE, TRUE};
use winapi::um::processenv::GetCurrentDirectoryA;
use winapi::um::winbase::{FILE_BEGIN, FILE_CURRENT, FILE_END};
use winapi::um::fileapi::{INVALID_SET_FILE_POINTER};
use winapi::um::winnt::{BOOLEAN, CHAR, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, LONG, LPSTR};

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_fileLength(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jlong {
    unsafe {
        return PHYSFS_fileLength(fd as *mut PHYSFS_File);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_tell(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jlong {
    unsafe {
        return PHYSFS_tell(fd as *mut PHYSFS_File);
    }
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
    unsafe {
        let mut vec = vec![0 as i8; len as usize];
        let c_buf = &mut vec[..];

        let res = PHYSFS_readBytes(
            fd as *mut PHYSFS_File,
            c_buf.as_mut_ptr() as *mut libc::c_void,
            len as PHYSFS_uint64,
        ) as jint;

        env.set_byte_array_region(java_buf, offset, c_buf).unwrap();

        return res;
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_openRead(
    env: JNIEnv,
    obj: JObject,
    file_name: JString,
) -> jlong {
    unsafe {
        let physfs_file = PHYSFS_openRead((**env.get_string(file_name).unwrap()).as_ptr());
        return physfs_file as jlong;
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_seek(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
    pos: jlong,
) -> jint {
    unsafe {
        return PHYSFS_seek(fd as *mut PHYSFS_File, pos as PHYSFS_uint64);
    }
}


#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_eof(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jint {
    unsafe {
        return PHYSFS_eof(fd as *mut PHYSFS_File);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFSInputStream_close(
    env: JNIEnv,
    obj: JObject,
    fd: jlong,
) -> jint {
    unsafe {
        return PHYSFS_close(fd as *mut PHYSFS_File);
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_exists(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
) -> jint {
    unsafe {
        return PHYSFS_exists((**env.get_string(file_name).unwrap()).as_ptr());
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_mount(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
    append: jint,
) -> jint {
    unsafe {
        return PHYSFS_mount(
            (**env.get_string(file_name).unwrap()).as_ptr(),
            std::ptr::null(),
            append,
        );
    }
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
    unsafe {
        return PHYSFS_mount(
            (**env.get_string(file_name).unwrap()).as_ptr(),
            (**env.get_string(mount_point).unwrap()).as_ptr(),
            append,
        );
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_unmount(
    env: JNIEnv,
    class: JClass,
    file_name: JString,
) -> jint {
    unsafe {
        return PHYSFS_unmount((**env.get_string(file_name).unwrap()).as_ptr());
    }
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_PhysFS_getLastErrorCode(
    env: JNIEnv,
    class: JClass,
) -> jint {
    unsafe {
        return PHYSFS_getLastErrorCode();
    }
}

lazy_static! {
    static ref OPEN_FILES: RwLock<Option<FileHandle>> = RwLock::new(None);
}

#[repr(C)]
#[derive(Clone)]
struct FileHandle {
    size: usize,
    next_handle: *mut FileHandle,
    physfs_file: *mut PHYSFS_File
}

unsafe impl Send for FileHandle {}
unsafe impl Sync for FileHandle {}

#[repr(C)]
struct RTSInterface {
    get_current_real_time: unsafe extern fn() -> i64,
    get_current_game_time: unsafe extern fn() -> i64,
    open_file: unsafe fn(LPSTR, u32) -> i32,
    read_file: unsafe fn(*mut FileHandle, LPVOID, DWORD) -> BOOL,
    write_file: unsafe fn(*mut FileHandle, LPCVOID, DWORD) -> BOOL,
    seek_file: unsafe fn(*mut FileHandle, LONG, DWORD) -> DWORD,
    close_file: unsafe fn(*mut FileHandle) -> i32
}

#[link(name = "rts", kind = "dylib")]
extern "C" {
    #[link_name = "_RTS_GetCurrentGameTime@0"]
    fn RTS_GetCurrentGameTime() -> i64;
    #[link_name = "_RTS_GetCurrentRealTime@0"]
    fn RTS_GetCurrentRealTime() -> i64;
}

unsafe fn open_file(file_name: LPSTR, mask: u32) -> i32 {
    // Take the write lock
    let mut file_list = OPEN_FILES.write().unwrap();

    // GENERIC_WRITE
    let handle: *mut PHYSFS_File;

    if mask & 1 != 0 || mask & 2 != 0 {
        // TRUNCATE_EXISTING
        if mask & 512 != 0 {
           handle = PHYSFS_openWrite(file_name);
        // CREATE_ALWAYS
        } else if mask & 256 != 0 {
            handle = PHYSFS_openWrite(file_name);
        // OPEN_EXISTING
        } else {
            handle = PHYSFS_openAppend(file_name);
        }
    // GENERIC_READ
    } else {
        handle = PHYSFS_openRead(file_name);
    };

    if handle.is_null() {
        return -1;
    } else  {
        if PHYSFS_seek(handle, 0) == 0 {
            return -1;
        } else {
            let next_handle = match &mut *file_list {
                Some(next) => next,
                None => std::ptr::null_mut()
            };

            let mut new_file_list = FileHandle {
                size: std::mem::size_of::<FileHandle>(),
                next_handle: next_handle,
                physfs_file: handle
            };

            *file_list = Some(new_file_list.clone());

            return &mut new_file_list as *mut FileHandle as i32;
        }
    }
}

unsafe fn read_file(handle: *mut FileHandle, buf: LPVOID, bytes_to_read: DWORD) -> BOOL {
    return PHYSFS_readBytes((*handle).physfs_file, buf, bytes_to_read.into()) as BOOL;
}

unsafe fn write_file(handle: *mut FileHandle, buf: LPCVOID, bytes_to_write: DWORD) -> BOOL {
    return PHYSFS_writeBytes((*handle).physfs_file, buf, bytes_to_write.into()) as BOOL;
}

unsafe fn seek_file(handle: *mut FileHandle, pos: LONG, move_method: DWORD) -> DWORD {
    if move_method == FILE_BEGIN {
        if PHYSFS_seek((*handle).physfs_file, pos as u64) != 0 {
            return PHYSFS_tell((*handle).physfs_file) as DWORD;
        } else {
            return INVALID_SET_FILE_POINTER;
        }
    } else if move_method == FILE_CURRENT {
        let current_pos = PHYSFS_tell((*handle).physfs_file);
        if current_pos != 0 {
            let desired_pos = current_pos as u64 + pos as u64;
            if PHYSFS_seek((*handle).physfs_file, desired_pos) > 0 {
                return PHYSFS_tell((*handle).physfs_file) as DWORD;
            } else {
                return INVALID_SET_FILE_POINTER
            }
        } else {
            return INVALID_SET_FILE_POINTER;
        }
    } else if move_method == FILE_END {
        let file_length = PHYSFS_fileLength((*handle).physfs_file);
        if file_length > 0 {
            let desired_pos = file_length as u64 + pos as u64;
            if PHYSFS_seek((*handle).physfs_file, desired_pos) > 0 {
                return PHYSFS_tell((*handle).physfs_file) as DWORD;
            } else {
                return INVALID_SET_FILE_POINTER
            }
        } else {
            return INVALID_SET_FILE_POINTER;
        }
    } else {
        return INVALID_SET_FILE_POINTER;
    }
}

unsafe fn close_file(handle: *mut FileHandle) -> BOOL {
    return PHYSFS_close((*handle).physfs_file);
    // if PHYSFS_close((*handle).physfs_file) != 0 {
    //     // Take the write lock
    //     let mut file_list = OPEN_FILES.write().unwrap();

    //     match &mut *file_list {
    //         Some(list) => {
    //             let target_file = (*handle).physfs_file;
    //             let mut current_handle: FileHandle = *list;
                
    //             while !current_handle.physfs_file.is_null() {
    //                 if current_handle.physfs_file == target_file { break; }
    //                 if current_handle.next_handle.is_null() { break; }
    //                 current_handle = *list.next_handle;
    //             }
    //         },
    //         None => {
    //             return 0;
    //         }
    //     };

    // } else {
    //     return 0;
    // };
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "system" fn Java_com_maddox_rts_RTS_interf(
    env: JNIEnv,
    class: JClass,
) -> jint {
    let mut rts_interface = RTSInterface {
        get_current_real_time: RTS_GetCurrentRealTime,
        get_current_game_time: RTS_GetCurrentGameTime,
        open_file: open_file,
        read_file: read_file,
        write_file: write_file,
        seek_file: seek_file,
        close_file: close_file
    };

    return &mut rts_interface as *mut RTSInterface as jint;
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
