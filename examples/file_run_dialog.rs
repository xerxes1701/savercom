extern crate safercom;

use safercom::{
    ole32::{
        co_create_instance,
        co_initialize,
        co_uninitialize,
    },
    types::{
        CLSID,
        HRESULT,
        IID,
    },
    ComInterface,
    ComRef,
    IDispatch
};
use std::{
    ffi::c_void,
    mem::transmute,
};

fn main() -> Result<(), HRESULT> {
    const CLSID_SH_APP: CLSID = CLSID::new(
        0x13709620,
        0xC279,
        0x11CE,
        [0xA4, 0x9E, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
    );

    co_initialize()?;

    let obj: ComRef<IShellDispatch> = co_create_instance(&CLSID_SH_APP)?;

    obj.file_run()?;

    co_uninitialize();

    Ok(())
}

#[repr(C)]
pub struct IShellDispatch {
    pub __vtable: *mut IShellDispatch_vtable,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct IShellDispatch_vtable {
    pub __IDispatch:      <IDispatch as ComInterface>::VTable,
    pub Application: extern "stdcall" fn(*const IShellDispatch, *mut *mut IDispatch) -> HRESULT,
    pub Parent: extern "stdcall" fn(*const IShellDispatch, *mut *mut IDispatch) -> HRESULT,
    pub NameSpace:        *mut c_void, // extern "stdcall" fn(*const IShellDispatch, VARIANT, *mut *mut Folder) -> HRESULT,
    pub BrowseForFolder:  *mut c_void, // extern "stdcall" fn(*const IShellDispatch, u32, BSTR, u32, *mut *mut Folder) -> HRESULT,
    pub Windows: extern "stdcall" fn(*const IShellDispatch, *mut *mut IDispatch) -> HRESULT,
    pub Open:             *mut c_void, // extern "stdcall" fn(*const IShellDispatch, VARIANT) -> HRESULT,
    pub Explore:          *mut c_void, // extern "stdcall" fn(*const IShellDispatch, VARIANT) -> HRESULT,
    pub MinimizeAll:      extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub UndoMinimizeALL:  extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub FileRun:          extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub CascadeWindows:   extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub TileVertically:   extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub TileHorizontally: extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub ShutdownWindows:  extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub Suspend:          extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub EjectPC:          extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub SetTime:          extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub TrayProperties:   extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub Help:             extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub FindFiles:        extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub FindComputer:     extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub RefreshMenu:      extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub ControlPanelItem: *mut c_void, // extern "stdcall" fn(*const IShellDispatch, BSTR) -> HRESULT,
}

impl IShellDispatch {
    pub fn file_run(&self) -> Result<(), HRESULT> {
        unsafe {
            match ((*self.__vtable).FileRun)(transmute(self)) {
                HRESULT::S_OK => Ok(()),
                hr => Err(hr),
            }
        }
    }
}

impl ComInterface for IShellDispatch {
    type VTable = IShellDispatch_vtable;

    const IID: IID = IID::new(
        0xd8f015c0,
        0xc278,
        0x11ce,
        [0xa4, 0x9e, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
    );

    unsafe fn vtable(&self) -> *const Self::VTable {
        self.__vtable
    }
}