#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct CONVERT_A {
    pub szOldDll: ::windows::core::PSTR,
    pub Anonymous: CONVERT_A_0,
}
impl ::core::marker::Copy for CONVERT_A {}
impl ::core::clone::Clone for CONVERT_A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONVERT_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONVERT_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONVERT_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONVERT_A {}
impl ::core::default::Default for CONVERT_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union CONVERT_A_0 {
    pub fFlags: u32,
    pub Anonymous: CONVERT_A_0_0,
}
impl ::core::marker::Copy for CONVERT_A_0 {}
impl ::core::clone::Clone for CONVERT_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONVERT_A_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONVERT_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONVERT_A_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONVERT_A_0 {}
impl ::core::default::Default for CONVERT_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct CONVERT_A_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for CONVERT_A_0_0 {}
impl ::core::clone::Clone for CONVERT_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONVERT_A_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONVERT_A_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for CONVERT_A_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONVERT_A_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONVERT_A_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONVERT_A_0_0 {}
impl ::core::default::Default for CONVERT_A_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct CONVERT_W {
    pub szOldDll: ::windows::core::PWSTR,
    pub Anonymous: CONVERT_W_0,
}
impl ::core::marker::Copy for CONVERT_W {}
impl ::core::clone::Clone for CONVERT_W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONVERT_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONVERT_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONVERT_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONVERT_W {}
impl ::core::default::Default for CONVERT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union CONVERT_W_0 {
    pub fFlags: u32,
    pub Anonymous: CONVERT_W_0_0,
}
impl ::core::marker::Copy for CONVERT_W_0 {}
impl ::core::clone::Clone for CONVERT_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONVERT_W_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONVERT_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONVERT_W_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONVERT_W_0 {}
impl ::core::default::Default for CONVERT_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct CONVERT_W_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for CONVERT_W_0_0 {}
impl ::core::clone::Clone for CONVERT_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONVERT_W_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONVERT_W_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for CONVERT_W_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONVERT_W_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONVERT_W_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONVERT_W_0_0 {}
impl ::core::default::Default for CONVERT_W_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_BASE_NAME_LENGTH: u32 = 3u32;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_BKINFO {
    pub lgposMark: JET_LGPOS,
    pub Anonymous: JET_BKINFO_0,
    pub genLow: u32,
    pub genHigh: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_BKINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_BKINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_BKINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_BKINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_BKINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_BKINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_BKINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union JET_BKINFO_0 {
    pub logtimeMark: JET_LOGTIME,
    pub bklogtimeMark: JET_BKLOGTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_BKINFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_BKINFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_BKINFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_BKINFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_BKINFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_BKINFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_BKINFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_BKLOGTIME {
    pub bSeconds: super::super::Foundation::CHAR,
    pub bMinutes: super::super::Foundation::CHAR,
    pub bHours: super::super::Foundation::CHAR,
    pub bDay: super::super::Foundation::CHAR,
    pub bMonth: super::super::Foundation::CHAR,
    pub bYear: super::super::Foundation::CHAR,
    pub Anonymous1: JET_BKLOGTIME_0,
    pub Anonymous2: JET_BKLOGTIME_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_BKLOGTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_BKLOGTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_BKLOGTIME {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_BKLOGTIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_BKLOGTIME>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_BKLOGTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_BKLOGTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union JET_BKLOGTIME_0 {
    pub bFiller1: super::super::Foundation::CHAR,
    pub Anonymous: JET_BKLOGTIME_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_BKLOGTIME_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_BKLOGTIME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_BKLOGTIME_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_BKLOGTIME_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_BKLOGTIME_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_BKLOGTIME_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_BKLOGTIME_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_BKLOGTIME_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_BKLOGTIME_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_BKLOGTIME_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JET_BKLOGTIME_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_BKLOGTIME_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_BKLOGTIME_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_BKLOGTIME_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_BKLOGTIME_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_BKLOGTIME_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_BKLOGTIME_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union JET_BKLOGTIME_1 {
    pub bFiller2: super::super::Foundation::CHAR,
    pub Anonymous: JET_BKLOGTIME_1_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_BKLOGTIME_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_BKLOGTIME_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_BKLOGTIME_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_BKLOGTIME_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_BKLOGTIME_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_BKLOGTIME_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_BKLOGTIME_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_BKLOGTIME_1_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_BKLOGTIME_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_BKLOGTIME_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JET_BKLOGTIME_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_BKLOGTIME_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_BKLOGTIME_1_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_BKLOGTIME_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_BKLOGTIME_1_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_BKLOGTIME_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_BKLOGTIME_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub type JET_CALLBACK = ::core::option::Option<unsafe extern "system" fn(sesid: super::StructuredStorage::JET_SESID, dbid: u32, tableid: super::StructuredStorage::JET_TABLEID, cbtyp: u32, pvarg1: *mut ::core::ffi::c_void, pvarg2: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void, ulunused: super::StructuredStorage::JET_API_PTR) -> i32>;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_COLUMNBASE_A {
    pub cbStruct: u32,
    pub columnid: u32,
    pub coltyp: u32,
    pub wCountry: u16,
    pub langid: u16,
    pub cp: u16,
    pub wFiller: u16,
    pub cbMax: u32,
    pub grbit: u32,
    pub szBaseTableName: [super::super::Foundation::CHAR; 256],
    pub szBaseColumnName: [super::super::Foundation::CHAR; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_COLUMNBASE_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_COLUMNBASE_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JET_COLUMNBASE_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNBASE_A").field("cbStruct", &self.cbStruct).field("columnid", &self.columnid).field("coltyp", &self.coltyp).field("wCountry", &self.wCountry).field("langid", &self.langid).field("cp", &self.cp).field("wFiller", &self.wFiller).field("cbMax", &self.cbMax).field("grbit", &self.grbit).field("szBaseTableName", &self.szBaseTableName).field("szBaseColumnName", &self.szBaseColumnName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_COLUMNBASE_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_COLUMNBASE_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_COLUMNBASE_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_COLUMNBASE_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_COLUMNBASE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_COLUMNBASE_W {
    pub cbStruct: u32,
    pub columnid: u32,
    pub coltyp: u32,
    pub wCountry: u16,
    pub langid: u16,
    pub cp: u16,
    pub wFiller: u16,
    pub cbMax: u32,
    pub grbit: u32,
    pub szBaseTableName: [u16; 256],
    pub szBaseColumnName: [u16; 256],
}
impl ::core::marker::Copy for JET_COLUMNBASE_W {}
impl ::core::clone::Clone for JET_COLUMNBASE_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_COLUMNBASE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNBASE_W").field("cbStruct", &self.cbStruct).field("columnid", &self.columnid).field("coltyp", &self.coltyp).field("wCountry", &self.wCountry).field("langid", &self.langid).field("cp", &self.cp).field("wFiller", &self.wFiller).field("cbMax", &self.cbMax).field("grbit", &self.grbit).field("szBaseTableName", &self.szBaseTableName).field("szBaseColumnName", &self.szBaseColumnName).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_COLUMNBASE_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_COLUMNBASE_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_COLUMNBASE_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_COLUMNBASE_W {}
impl ::core::default::Default for JET_COLUMNBASE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_COLUMNCREATE_A {
    pub cbStruct: u32,
    pub szColumnName: ::windows::core::PSTR,
    pub coltyp: u32,
    pub cbMax: u32,
    pub grbit: u32,
    pub pvDefault: *mut ::core::ffi::c_void,
    pub cbDefault: u32,
    pub cp: u32,
    pub columnid: u32,
    pub err: i32,
}
impl ::core::marker::Copy for JET_COLUMNCREATE_A {}
impl ::core::clone::Clone for JET_COLUMNCREATE_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_COLUMNCREATE_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNCREATE_A").field("cbStruct", &self.cbStruct).field("szColumnName", &self.szColumnName).field("coltyp", &self.coltyp).field("cbMax", &self.cbMax).field("grbit", &self.grbit).field("pvDefault", &self.pvDefault).field("cbDefault", &self.cbDefault).field("cp", &self.cp).field("columnid", &self.columnid).field("err", &self.err).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_COLUMNCREATE_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_COLUMNCREATE_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_COLUMNCREATE_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_COLUMNCREATE_A {}
impl ::core::default::Default for JET_COLUMNCREATE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_COLUMNCREATE_W {
    pub cbStruct: u32,
    pub szColumnName: ::windows::core::PWSTR,
    pub coltyp: u32,
    pub cbMax: u32,
    pub grbit: u32,
    pub pvDefault: *mut ::core::ffi::c_void,
    pub cbDefault: u32,
    pub cp: u32,
    pub columnid: u32,
    pub err: i32,
}
impl ::core::marker::Copy for JET_COLUMNCREATE_W {}
impl ::core::clone::Clone for JET_COLUMNCREATE_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_COLUMNCREATE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNCREATE_W").field("cbStruct", &self.cbStruct).field("szColumnName", &self.szColumnName).field("coltyp", &self.coltyp).field("cbMax", &self.cbMax).field("grbit", &self.grbit).field("pvDefault", &self.pvDefault).field("cbDefault", &self.cbDefault).field("cp", &self.cp).field("columnid", &self.columnid).field("err", &self.err).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_COLUMNCREATE_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_COLUMNCREATE_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_COLUMNCREATE_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_COLUMNCREATE_W {}
impl ::core::default::Default for JET_COLUMNCREATE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_COLUMNDEF {
    pub cbStruct: u32,
    pub columnid: u32,
    pub coltyp: u32,
    pub wCountry: u16,
    pub langid: u16,
    pub cp: u16,
    pub wCollate: u16,
    pub cbMax: u32,
    pub grbit: u32,
}
impl ::core::marker::Copy for JET_COLUMNDEF {}
impl ::core::clone::Clone for JET_COLUMNDEF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_COLUMNDEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNDEF").field("cbStruct", &self.cbStruct).field("columnid", &self.columnid).field("coltyp", &self.coltyp).field("wCountry", &self.wCountry).field("langid", &self.langid).field("cp", &self.cp).field("wCollate", &self.wCollate).field("cbMax", &self.cbMax).field("grbit", &self.grbit).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_COLUMNDEF {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_COLUMNDEF {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_COLUMNDEF>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_COLUMNDEF {}
impl ::core::default::Default for JET_COLUMNDEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_COLUMNLIST {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cRecord: u32,
    pub columnidPresentationOrder: u32,
    pub columnidcolumnname: u32,
    pub columnidcolumnid: u32,
    pub columnidcoltyp: u32,
    pub columnidCountry: u32,
    pub columnidLangid: u32,
    pub columnidCp: u32,
    pub columnidCollate: u32,
    pub columnidcbMax: u32,
    pub columnidgrbit: u32,
    pub columnidDefault: u32,
    pub columnidBaseTableName: u32,
    pub columnidBaseColumnName: u32,
    pub columnidDefinitionName: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_COLUMNLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_COLUMNLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_COLUMNLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidPresentationOrder", &self.columnidPresentationOrder)
            .field("columnidcolumnname", &self.columnidcolumnname)
            .field("columnidcolumnid", &self.columnidcolumnid)
            .field("columnidcoltyp", &self.columnidcoltyp)
            .field("columnidCountry", &self.columnidCountry)
            .field("columnidLangid", &self.columnidLangid)
            .field("columnidCp", &self.columnidCp)
            .field("columnidCollate", &self.columnidCollate)
            .field("columnidcbMax", &self.columnidcbMax)
            .field("columnidgrbit", &self.columnidgrbit)
            .field("columnidDefault", &self.columnidDefault)
            .field("columnidBaseTableName", &self.columnidBaseTableName)
            .field("columnidBaseColumnName", &self.columnidBaseColumnName)
            .field("columnidDefinitionName", &self.columnidDefinitionName)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_COLUMNLIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_COLUMNLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_COLUMNLIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_COLUMNLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_COLUMNLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_COMMIT_ID {
    pub signLog: JET_SIGNATURE,
    pub reserved: i32,
    pub commitId: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_COMMIT_ID {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_COMMIT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_COMMIT_ID {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_COMMIT_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_COMMIT_ID>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_COMMIT_ID {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_COMMIT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_COMMIT_ID {
    pub signLog: JET_SIGNATURE,
    pub reserved: i32,
    pub commitId: i64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_COMMIT_ID {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_COMMIT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_COMMIT_ID {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_COMMIT_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_COMMIT_ID>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_COMMIT_ID {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_COMMIT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_CONDITIONALCOLUMN_A {
    pub cbStruct: u32,
    pub szColumnName: ::windows::core::PSTR,
    pub grbit: u32,
}
impl ::core::marker::Copy for JET_CONDITIONALCOLUMN_A {}
impl ::core::clone::Clone for JET_CONDITIONALCOLUMN_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_CONDITIONALCOLUMN_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_CONDITIONALCOLUMN_A").field("cbStruct", &self.cbStruct).field("szColumnName", &self.szColumnName).field("grbit", &self.grbit).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_CONDITIONALCOLUMN_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_CONDITIONALCOLUMN_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_CONDITIONALCOLUMN_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_CONDITIONALCOLUMN_A {}
impl ::core::default::Default for JET_CONDITIONALCOLUMN_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_CONDITIONALCOLUMN_W {
    pub cbStruct: u32,
    pub szColumnName: ::windows::core::PWSTR,
    pub grbit: u32,
}
impl ::core::marker::Copy for JET_CONDITIONALCOLUMN_W {}
impl ::core::clone::Clone for JET_CONDITIONALCOLUMN_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_CONDITIONALCOLUMN_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_CONDITIONALCOLUMN_W").field("cbStruct", &self.cbStruct).field("szColumnName", &self.szColumnName).field("grbit", &self.grbit).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_CONDITIONALCOLUMN_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_CONDITIONALCOLUMN_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_CONDITIONALCOLUMN_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_CONDITIONALCOLUMN_W {}
impl ::core::default::Default for JET_CONDITIONALCOLUMN_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ColInfoGrbitMinimalInfo: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ColInfoGrbitNonDerivedColumnsOnly: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ColInfoGrbitSortByColumnid: u32 = 536870912u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_DBINFOMISC {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_DBINFOMISC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_DBINFOMISC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_DBINFOMISC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_DBINFOMISC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_DBINFOMISC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_DBINFOMISC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_DBINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_DBINFOMISC2 {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
    pub genMinRequired: u32,
    pub genMaxRequired: u32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: u32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: u32,
    pub ulECCFixSuccess: u32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: u32,
    pub ulECCFixFail: u32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: u32,
    pub ulBadChecksum: u32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_DBINFOMISC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_DBINFOMISC2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_DBINFOMISC2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_DBINFOMISC2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_DBINFOMISC2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_DBINFOMISC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_DBINFOMISC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_DBINFOMISC3 {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
    pub genMinRequired: u32,
    pub genMaxRequired: u32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: u32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: u32,
    pub ulECCFixSuccess: u32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: u32,
    pub ulECCFixFail: u32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: u32,
    pub ulBadChecksum: u32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: u32,
    pub genCommitted: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_DBINFOMISC3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_DBINFOMISC3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_DBINFOMISC3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_DBINFOMISC3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_DBINFOMISC3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_DBINFOMISC3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_DBINFOMISC3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_DBINFOMISC4 {
    pub ulVersion: u32,
    pub ulUpdate: u32,
    pub signDb: JET_SIGNATURE,
    pub dbstate: u32,
    pub lgposConsistent: JET_LGPOS,
    pub logtimeConsistent: JET_LOGTIME,
    pub logtimeAttach: JET_LOGTIME,
    pub lgposAttach: JET_LGPOS,
    pub logtimeDetach: JET_LOGTIME,
    pub lgposDetach: JET_LGPOS,
    pub signLog: JET_SIGNATURE,
    pub bkinfoFullPrev: JET_BKINFO,
    pub bkinfoIncPrev: JET_BKINFO,
    pub bkinfoFullCur: JET_BKINFO,
    pub fShadowingDisabled: u32,
    pub fUpgradeDb: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub lSPNumber: i32,
    pub cbPageSize: u32,
    pub genMinRequired: u32,
    pub genMaxRequired: u32,
    pub logtimeGenMaxCreate: JET_LOGTIME,
    pub ulRepairCount: u32,
    pub logtimeRepair: JET_LOGTIME,
    pub ulRepairCountOld: u32,
    pub ulECCFixSuccess: u32,
    pub logtimeECCFixSuccess: JET_LOGTIME,
    pub ulECCFixSuccessOld: u32,
    pub ulECCFixFail: u32,
    pub logtimeECCFixFail: JET_LOGTIME,
    pub ulECCFixFailOld: u32,
    pub ulBadChecksum: u32,
    pub logtimeBadChecksum: JET_LOGTIME,
    pub ulBadChecksumOld: u32,
    pub genCommitted: u32,
    pub bkinfoCopyPrev: JET_BKINFO,
    pub bkinfoDiffPrev: JET_BKINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_DBINFOMISC4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_DBINFOMISC4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_DBINFOMISC4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_DBINFOMISC4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_DBINFOMISC4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_DBINFOMISC4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_DBINFOMISC4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_DBINFOUPGRADE {
    pub cbStruct: u32,
    pub cbFilesizeLow: u32,
    pub cbFilesizeHigh: u32,
    pub cbFreeSpaceRequiredLow: u32,
    pub cbFreeSpaceRequiredHigh: u32,
    pub csecToUpgrade: u32,
    pub Anonymous: JET_DBINFOUPGRADE_0,
}
impl ::core::marker::Copy for JET_DBINFOUPGRADE {}
impl ::core::clone::Clone for JET_DBINFOUPGRADE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_DBINFOUPGRADE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_DBINFOUPGRADE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_DBINFOUPGRADE>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_DBINFOUPGRADE {}
impl ::core::default::Default for JET_DBINFOUPGRADE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_DBINFOUPGRADE_0 {
    pub ulFlags: u32,
    pub Anonymous: JET_DBINFOUPGRADE_0_0,
}
impl ::core::marker::Copy for JET_DBINFOUPGRADE_0 {}
impl ::core::clone::Clone for JET_DBINFOUPGRADE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_DBINFOUPGRADE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_DBINFOUPGRADE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_DBINFOUPGRADE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_DBINFOUPGRADE_0 {}
impl ::core::default::Default for JET_DBINFOUPGRADE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_DBINFOUPGRADE_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for JET_DBINFOUPGRADE_0_0 {}
impl ::core::clone::Clone for JET_DBINFOUPGRADE_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_DBINFOUPGRADE_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_DBINFOUPGRADE_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_DBINFOUPGRADE_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_DBINFOUPGRADE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_DBINFOUPGRADE_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_DBINFOUPGRADE_0_0 {}
impl ::core::default::Default for JET_DBINFOUPGRADE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoCollate: u32 = 5u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoConnect: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoCountry: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoCp: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoDBInUse: u32 = 15u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoFileType: u32 = 19u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoFilename: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoFilesize: u32 = 10u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoFilesizeOnDisk: u32 = 21u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoIsam: u32 = 9u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoLCID: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoLangid: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoMisc: u32 = 14u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoOptions: u32 = 6u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoPageSize: u32 = 17u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoSpaceAvailable: u32 = 12u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoSpaceOwned: u32 = 11u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoTransactions: u32 = 7u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoUpgrade: u32 = 13u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_DbInfoVersion: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_ENUMCOLUMN {
    pub columnid: u32,
    pub err: i32,
    pub Anonymous: JET_ENUMCOLUMN_0,
}
impl ::core::marker::Copy for JET_ENUMCOLUMN {}
impl ::core::clone::Clone for JET_ENUMCOLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_ENUMCOLUMN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_ENUMCOLUMN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_ENUMCOLUMN>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_ENUMCOLUMN {}
impl ::core::default::Default for JET_ENUMCOLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_ENUMCOLUMN_0 {
    pub Anonymous1: JET_ENUMCOLUMN_0_0,
    pub Anonymous2: JET_ENUMCOLUMN_0_1,
}
impl ::core::marker::Copy for JET_ENUMCOLUMN_0 {}
impl ::core::clone::Clone for JET_ENUMCOLUMN_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_ENUMCOLUMN_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_ENUMCOLUMN_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_ENUMCOLUMN_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_ENUMCOLUMN_0 {}
impl ::core::default::Default for JET_ENUMCOLUMN_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_ENUMCOLUMN_0_0 {
    pub cEnumColumnValue: u32,
    pub rgEnumColumnValue: *mut JET_ENUMCOLUMNVALUE,
}
impl ::core::marker::Copy for JET_ENUMCOLUMN_0_0 {}
impl ::core::clone::Clone for JET_ENUMCOLUMN_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_ENUMCOLUMN_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_ENUMCOLUMN_0_0").field("cEnumColumnValue", &self.cEnumColumnValue).field("rgEnumColumnValue", &self.rgEnumColumnValue).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_ENUMCOLUMN_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_ENUMCOLUMN_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_ENUMCOLUMN_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_ENUMCOLUMN_0_0 {}
impl ::core::default::Default for JET_ENUMCOLUMN_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_ENUMCOLUMN_0_1 {
    pub cbData: u32,
    pub pvData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for JET_ENUMCOLUMN_0_1 {}
impl ::core::clone::Clone for JET_ENUMCOLUMN_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_ENUMCOLUMN_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_ENUMCOLUMN_0_1").field("cbData", &self.cbData).field("pvData", &self.pvData).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_ENUMCOLUMN_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_ENUMCOLUMN_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_ENUMCOLUMN_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_ENUMCOLUMN_0_1 {}
impl ::core::default::Default for JET_ENUMCOLUMN_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_ENUMCOLUMNID {
    pub columnid: u32,
    pub ctagSequence: u32,
    pub rgtagSequence: *mut u32,
}
impl ::core::marker::Copy for JET_ENUMCOLUMNID {}
impl ::core::clone::Clone for JET_ENUMCOLUMNID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_ENUMCOLUMNID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_ENUMCOLUMNID").field("columnid", &self.columnid).field("ctagSequence", &self.ctagSequence).field("rgtagSequence", &self.rgtagSequence).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_ENUMCOLUMNID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_ENUMCOLUMNID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_ENUMCOLUMNID>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_ENUMCOLUMNID {}
impl ::core::default::Default for JET_ENUMCOLUMNID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_ENUMCOLUMNVALUE {
    pub itagSequence: u32,
    pub err: i32,
    pub cbData: u32,
    pub pvData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for JET_ENUMCOLUMNVALUE {}
impl ::core::clone::Clone for JET_ENUMCOLUMNVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_ENUMCOLUMNVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_ENUMCOLUMNVALUE").field("itagSequence", &self.itagSequence).field("err", &self.err).field("cbData", &self.cbData).field("pvData", &self.pvData).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_ENUMCOLUMNVALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_ENUMCOLUMNVALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_ENUMCOLUMNVALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_ENUMCOLUMNVALUE {}
impl ::core::default::Default for JET_ENUMCOLUMNVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_ERRCAT(pub i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatUnknown: JET_ERRCAT = JET_ERRCAT(0i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatError: JET_ERRCAT = JET_ERRCAT(1i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatOperation: JET_ERRCAT = JET_ERRCAT(2i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatFatal: JET_ERRCAT = JET_ERRCAT(3i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatIO: JET_ERRCAT = JET_ERRCAT(4i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatResource: JET_ERRCAT = JET_ERRCAT(5i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatMemory: JET_ERRCAT = JET_ERRCAT(6i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatQuota: JET_ERRCAT = JET_ERRCAT(7i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatDisk: JET_ERRCAT = JET_ERRCAT(8i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatData: JET_ERRCAT = JET_ERRCAT(9i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatCorruption: JET_ERRCAT = JET_ERRCAT(10i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatInconsistent: JET_ERRCAT = JET_ERRCAT(11i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatFragmentation: JET_ERRCAT = JET_ERRCAT(12i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatApi: JET_ERRCAT = JET_ERRCAT(13i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatUsage: JET_ERRCAT = JET_ERRCAT(14i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatState: JET_ERRCAT = JET_ERRCAT(15i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatObsolete: JET_ERRCAT = JET_ERRCAT(16i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errcatMax: JET_ERRCAT = JET_ERRCAT(17i32);
impl ::core::marker::Copy for JET_ERRCAT {}
impl ::core::clone::Clone for JET_ERRCAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JET_ERRCAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JET_ERRCAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for JET_ERRCAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_ERRCAT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_ERRINFOBASIC_W {
    pub cbStruct: u32,
    pub errValue: i32,
    pub errcatMostSpecific: JET_ERRCAT,
    pub rgCategoricalHierarchy: [u8; 8],
    pub lSourceLine: u32,
    pub rgszSourceFile: [u16; 64],
}
impl ::core::marker::Copy for JET_ERRINFOBASIC_W {}
impl ::core::clone::Clone for JET_ERRINFOBASIC_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_ERRINFOBASIC_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_ERRINFOBASIC_W").field("cbStruct", &self.cbStruct).field("errValue", &self.errValue).field("errcatMostSpecific", &self.errcatMostSpecific).field("rgCategoricalHierarchy", &self.rgCategoricalHierarchy).field("lSourceLine", &self.lSourceLine).field("rgszSourceFile", &self.rgszSourceFile).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_ERRINFOBASIC_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_ERRINFOBASIC_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_ERRINFOBASIC_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_ERRINFOBASIC_W {}
impl ::core::default::Default for JET_ERRINFOBASIC_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_EventLoggingDisable: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_EventLoggingLevelHigh: u32 = 75u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_EventLoggingLevelLow: u32 = 25u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_EventLoggingLevelMax: u32 = 100u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_EventLoggingLevelMedium: u32 = 50u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_EventLoggingLevelMin: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ExceptionFailFast: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ExceptionMsgBox: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ExceptionNone: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_INDEXCHECKING(pub i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_IndexCheckingOff: JET_INDEXCHECKING = JET_INDEXCHECKING(0i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_IndexCheckingOn: JET_INDEXCHECKING = JET_INDEXCHECKING(1i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_IndexCheckingDeferToOpenTable: JET_INDEXCHECKING = JET_INDEXCHECKING(2i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_IndexCheckingMax: JET_INDEXCHECKING = JET_INDEXCHECKING(3i32);
impl ::core::marker::Copy for JET_INDEXCHECKING {}
impl ::core::clone::Clone for JET_INDEXCHECKING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JET_INDEXCHECKING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCHECKING {
    type Abi = Self;
}
impl ::core::fmt::Debug for JET_INDEXCHECKING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_INDEXCHECKING").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_INDEXCREATE2_A {
    pub cbStruct: u32,
    pub szIndexName: ::windows::core::PSTR,
    pub szKey: ::windows::core::PSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE2_A_0,
    pub Anonymous2: JET_INDEXCREATE2_A_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl ::core::marker::Copy for JET_INDEXCREATE2_A {}
impl ::core::clone::Clone for JET_INDEXCREATE2_A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE2_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE2_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE2_A {}
impl ::core::default::Default for JET_INDEXCREATE2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE2_A_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl ::core::marker::Copy for JET_INDEXCREATE2_A_0 {}
impl ::core::clone::Clone for JET_INDEXCREATE2_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE2_A_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE2_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE2_A_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE2_A_0 {}
impl ::core::default::Default for JET_INDEXCREATE2_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE2_A_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl ::core::marker::Copy for JET_INDEXCREATE2_A_1 {}
impl ::core::clone::Clone for JET_INDEXCREATE2_A_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE2_A_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE2_A_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE2_A_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE2_A_1 {}
impl ::core::default::Default for JET_INDEXCREATE2_A_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_INDEXCREATE2_W {
    pub cbStruct: u32,
    pub szIndexName: ::windows::core::PWSTR,
    pub szKey: ::windows::core::PWSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE2_W_0,
    pub Anonymous2: JET_INDEXCREATE2_W_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl ::core::marker::Copy for JET_INDEXCREATE2_W {}
impl ::core::clone::Clone for JET_INDEXCREATE2_W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE2_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE2_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE2_W {}
impl ::core::default::Default for JET_INDEXCREATE2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE2_W_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl ::core::marker::Copy for JET_INDEXCREATE2_W_0 {}
impl ::core::clone::Clone for JET_INDEXCREATE2_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE2_W_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE2_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE2_W_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE2_W_0 {}
impl ::core::default::Default for JET_INDEXCREATE2_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE2_W_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl ::core::marker::Copy for JET_INDEXCREATE2_W_1 {}
impl ::core::clone::Clone for JET_INDEXCREATE2_W_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE2_W_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE2_W_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE2_W_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE2_W_1 {}
impl ::core::default::Default for JET_INDEXCREATE2_W_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_INDEXCREATE3_A {
    pub cbStruct: u32,
    pub szIndexName: ::windows::core::PSTR,
    pub szKey: ::windows::core::PSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub Anonymous: JET_INDEXCREATE3_A_0,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl ::core::marker::Copy for JET_INDEXCREATE3_A {}
impl ::core::clone::Clone for JET_INDEXCREATE3_A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE3_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE3_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE3_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE3_A {}
impl ::core::default::Default for JET_INDEXCREATE3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE3_A_0 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl ::core::marker::Copy for JET_INDEXCREATE3_A_0 {}
impl ::core::clone::Clone for JET_INDEXCREATE3_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE3_A_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE3_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE3_A_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE3_A_0 {}
impl ::core::default::Default for JET_INDEXCREATE3_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_INDEXCREATE3_W {
    pub cbStruct: u32,
    pub szIndexName: ::windows::core::PWSTR,
    pub szKey: ::windows::core::PWSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub Anonymous: JET_INDEXCREATE3_W_0,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
    pub pSpacehints: *mut JET_SPACEHINTS,
}
impl ::core::marker::Copy for JET_INDEXCREATE3_W {}
impl ::core::clone::Clone for JET_INDEXCREATE3_W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE3_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE3_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE3_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE3_W {}
impl ::core::default::Default for JET_INDEXCREATE3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE3_W_0 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl ::core::marker::Copy for JET_INDEXCREATE3_W_0 {}
impl ::core::clone::Clone for JET_INDEXCREATE3_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE3_W_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE3_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE3_W_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE3_W_0 {}
impl ::core::default::Default for JET_INDEXCREATE3_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_INDEXCREATE_A {
    pub cbStruct: u32,
    pub szIndexName: ::windows::core::PSTR,
    pub szKey: ::windows::core::PSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE_A_0,
    pub Anonymous2: JET_INDEXCREATE_A_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_A,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
}
impl ::core::marker::Copy for JET_INDEXCREATE_A {}
impl ::core::clone::Clone for JET_INDEXCREATE_A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE_A {}
impl ::core::default::Default for JET_INDEXCREATE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE_A_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl ::core::marker::Copy for JET_INDEXCREATE_A_0 {}
impl ::core::clone::Clone for JET_INDEXCREATE_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE_A_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE_A_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE_A_0 {}
impl ::core::default::Default for JET_INDEXCREATE_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE_A_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl ::core::marker::Copy for JET_INDEXCREATE_A_1 {}
impl ::core::clone::Clone for JET_INDEXCREATE_A_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE_A_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE_A_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE_A_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE_A_1 {}
impl ::core::default::Default for JET_INDEXCREATE_A_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_INDEXCREATE_W {
    pub cbStruct: u32,
    pub szIndexName: ::windows::core::PWSTR,
    pub szKey: ::windows::core::PWSTR,
    pub cbKey: u32,
    pub grbit: u32,
    pub ulDensity: u32,
    pub Anonymous1: JET_INDEXCREATE_W_0,
    pub Anonymous2: JET_INDEXCREATE_W_1,
    pub rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    pub cConditionalColumn: u32,
    pub err: i32,
    pub cbKeyMost: u32,
}
impl ::core::marker::Copy for JET_INDEXCREATE_W {}
impl ::core::clone::Clone for JET_INDEXCREATE_W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE_W {}
impl ::core::default::Default for JET_INDEXCREATE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE_W_0 {
    pub lcid: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
}
impl ::core::marker::Copy for JET_INDEXCREATE_W_0 {}
impl ::core::clone::Clone for JET_INDEXCREATE_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE_W_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE_W_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE_W_0 {}
impl ::core::default::Default for JET_INDEXCREATE_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub union JET_INDEXCREATE_W_1 {
    pub cbVarSegMac: u32,
    pub ptuplelimits: *mut JET_TUPLELIMITS,
}
impl ::core::marker::Copy for JET_INDEXCREATE_W_1 {}
impl ::core::clone::Clone for JET_INDEXCREATE_W_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_INDEXCREATE_W_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEXCREATE_W_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXCREATE_W_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEXCREATE_W_1 {}
impl ::core::default::Default for JET_INDEXCREATE_W_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct JET_INDEXID {
    pub cbStruct: u32,
    pub rgbIndexId: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for JET_INDEXID {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for JET_INDEXID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for JET_INDEXID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEXID").field("cbStruct", &self.cbStruct).field("rgbIndexId", &self.rgbIndexId).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for JET_INDEXID {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for JET_INDEXID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXID>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for JET_INDEXID {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_INDEXID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(target_arch = "x86")]
pub struct JET_INDEXID {
    pub cbStruct: u32,
    pub rgbIndexId: [u8; 12],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for JET_INDEXID {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for JET_INDEXID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for JET_INDEXID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEXID").field("cbStruct", &self.cbStruct).field("rgbIndexId", &self.rgbIndexId).finish()
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for JET_INDEXID {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for JET_INDEXID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXID>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for JET_INDEXID {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_INDEXID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_INDEXLIST {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cRecord: u32,
    pub columnidindexname: u32,
    pub columnidgrbitIndex: u32,
    pub columnidcKey: u32,
    pub columnidcEntry: u32,
    pub columnidcPage: u32,
    pub columnidcColumn: u32,
    pub columnidiColumn: u32,
    pub columnidcolumnid: u32,
    pub columnidcoltyp: u32,
    pub columnidCountry: u32,
    pub columnidLangid: u32,
    pub columnidCp: u32,
    pub columnidCollate: u32,
    pub columnidgrbitColumn: u32,
    pub columnidcolumnname: u32,
    pub columnidLCMapFlags: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_INDEXLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_INDEXLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_INDEXLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEXLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidindexname", &self.columnidindexname)
            .field("columnidgrbitIndex", &self.columnidgrbitIndex)
            .field("columnidcKey", &self.columnidcKey)
            .field("columnidcEntry", &self.columnidcEntry)
            .field("columnidcPage", &self.columnidcPage)
            .field("columnidcColumn", &self.columnidcColumn)
            .field("columnidiColumn", &self.columnidiColumn)
            .field("columnidcolumnid", &self.columnidcolumnid)
            .field("columnidcoltyp", &self.columnidcoltyp)
            .field("columnidCountry", &self.columnidCountry)
            .field("columnidLangid", &self.columnidLangid)
            .field("columnidCp", &self.columnidCp)
            .field("columnidCollate", &self.columnidCollate)
            .field("columnidgrbitColumn", &self.columnidgrbitColumn)
            .field("columnidcolumnname", &self.columnidcolumnname)
            .field("columnidLCMapFlags", &self.columnidLCMapFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_INDEXLIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_INDEXLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXLIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_INDEXLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_INDEXLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_INDEXRANGE {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub grbit: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_INDEXRANGE {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_INDEXRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_INDEXRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEXRANGE").field("cbStruct", &self.cbStruct).field("tableid", &self.tableid).field("grbit", &self.grbit).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_INDEXRANGE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_INDEXRANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEXRANGE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_INDEXRANGE {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_INDEXRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_INDEX_COLUMN {
    pub columnid: u32,
    pub relop: JET_RELOP,
    pub pv: *mut ::core::ffi::c_void,
    pub cb: u32,
    pub grbit: u32,
}
impl ::core::marker::Copy for JET_INDEX_COLUMN {}
impl ::core::clone::Clone for JET_INDEX_COLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_INDEX_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEX_COLUMN").field("columnid", &self.columnid).field("relop", &self.relop).field("pv", &self.pv).field("cb", &self.cb).field("grbit", &self.grbit).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_INDEX_COLUMN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEX_COLUMN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEX_COLUMN>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEX_COLUMN {}
impl ::core::default::Default for JET_INDEX_COLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_INDEX_RANGE {
    pub rgStartColumns: *mut JET_INDEX_COLUMN,
    pub cStartColumns: u32,
    pub rgEndColumns: *mut JET_INDEX_COLUMN,
    pub cEndColumns: u32,
}
impl ::core::marker::Copy for JET_INDEX_RANGE {}
impl ::core::clone::Clone for JET_INDEX_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_INDEX_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEX_RANGE").field("rgStartColumns", &self.rgStartColumns).field("cStartColumns", &self.cStartColumns).field("rgEndColumns", &self.rgEndColumns).field("cEndColumns", &self.cEndColumns).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_INDEX_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_INDEX_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INDEX_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_INDEX_RANGE {}
impl ::core::default::Default for JET_INDEX_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_INSTANCE_INFO_A {
    pub hInstanceId: super::StructuredStorage::JET_INSTANCE,
    pub szInstanceName: ::windows::core::PSTR,
    pub cDatabases: super::StructuredStorage::JET_API_PTR,
    pub szDatabaseFileName: *mut *mut i8,
    pub szDatabaseDisplayName: *mut *mut i8,
    pub szDatabaseSLVFileName_Obsolete: *mut *mut i8,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_INSTANCE_INFO_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_INSTANCE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_INSTANCE_INFO_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INSTANCE_INFO_A").field("hInstanceId", &self.hInstanceId).field("szInstanceName", &self.szInstanceName).field("cDatabases", &self.cDatabases).field("szDatabaseFileName", &self.szDatabaseFileName).field("szDatabaseDisplayName", &self.szDatabaseDisplayName).field("szDatabaseSLVFileName_Obsolete", &self.szDatabaseSLVFileName_Obsolete).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_INSTANCE_INFO_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_INSTANCE_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INSTANCE_INFO_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_INSTANCE_INFO_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_INSTANCE_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_INSTANCE_INFO_W {
    pub hInstanceId: super::StructuredStorage::JET_INSTANCE,
    pub szInstanceName: ::windows::core::PWSTR,
    pub cDatabases: super::StructuredStorage::JET_API_PTR,
    pub szDatabaseFileName: *mut *mut u16,
    pub szDatabaseDisplayName: *mut *mut u16,
    pub szDatabaseSLVFileName_Obsolete: *mut *mut u16,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_INSTANCE_INFO_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_INSTANCE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_INSTANCE_INFO_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INSTANCE_INFO_W").field("hInstanceId", &self.hInstanceId).field("szInstanceName", &self.szInstanceName).field("cDatabases", &self.cDatabases).field("szDatabaseFileName", &self.szDatabaseFileName).field("szDatabaseDisplayName", &self.szDatabaseDisplayName).field("szDatabaseSLVFileName_Obsolete", &self.szDatabaseSLVFileName_Obsolete).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_INSTANCE_INFO_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_INSTANCE_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_INSTANCE_INFO_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_INSTANCE_INFO_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_INSTANCE_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_IOPriorityLow: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_IOPriorityNormal: u32 = 0u32;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_LGPOS {
    pub ib: u16,
    pub isec: u16,
    pub lGeneration: i32,
}
impl ::core::marker::Copy for JET_LGPOS {}
impl ::core::clone::Clone for JET_LGPOS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JET_LGPOS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_LGPOS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_LGPOS>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_LGPOS {}
impl ::core::default::Default for JET_LGPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_LOGINFO_A {
    pub cbSize: u32,
    pub ulGenLow: u32,
    pub ulGenHigh: u32,
    pub szBaseName: [super::super::Foundation::CHAR; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_LOGINFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_LOGINFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JET_LOGINFO_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_LOGINFO_A").field("cbSize", &self.cbSize).field("ulGenLow", &self.ulGenLow).field("ulGenHigh", &self.ulGenHigh).field("szBaseName", &self.szBaseName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_LOGINFO_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_LOGINFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_LOGINFO_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_LOGINFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_LOGINFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_LOGINFO_W {
    pub cbSize: u32,
    pub ulGenLow: u32,
    pub ulGenHigh: u32,
    pub szBaseName: [u16; 4],
}
impl ::core::marker::Copy for JET_LOGINFO_W {}
impl ::core::clone::Clone for JET_LOGINFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_LOGINFO_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_LOGINFO_W").field("cbSize", &self.cbSize).field("ulGenLow", &self.ulGenLow).field("ulGenHigh", &self.ulGenHigh).field("szBaseName", &self.szBaseName).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_LOGINFO_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_LOGINFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_LOGINFO_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_LOGINFO_W {}
impl ::core::default::Default for JET_LOGINFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_LOGTIME {
    pub bSeconds: super::super::Foundation::CHAR,
    pub bMinutes: super::super::Foundation::CHAR,
    pub bHours: super::super::Foundation::CHAR,
    pub bDay: super::super::Foundation::CHAR,
    pub bMonth: super::super::Foundation::CHAR,
    pub bYear: super::super::Foundation::CHAR,
    pub Anonymous1: JET_LOGTIME_0,
    pub Anonymous2: JET_LOGTIME_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_LOGTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_LOGTIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_LOGTIME {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_LOGTIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_LOGTIME>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_LOGTIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_LOGTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union JET_LOGTIME_0 {
    pub bFiller1: super::super::Foundation::CHAR,
    pub Anonymous: JET_LOGTIME_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_LOGTIME_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_LOGTIME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_LOGTIME_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_LOGTIME_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_LOGTIME_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_LOGTIME_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_LOGTIME_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_LOGTIME_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_LOGTIME_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_LOGTIME_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JET_LOGTIME_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_LOGTIME_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_LOGTIME_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_LOGTIME_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_LOGTIME_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_LOGTIME_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_LOGTIME_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union JET_LOGTIME_1 {
    pub bFiller2: super::super::Foundation::CHAR,
    pub Anonymous: JET_LOGTIME_1_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_LOGTIME_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_LOGTIME_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_LOGTIME_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_LOGTIME_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_LOGTIME_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_LOGTIME_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_LOGTIME_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_LOGTIME_1_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_LOGTIME_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_LOGTIME_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JET_LOGTIME_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_LOGTIME_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_LOGTIME_1_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_LOGTIME_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_LOGTIME_1_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_LOGTIME_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_LOGTIME_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_LS(pub usize);
impl JET_LS {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for JET_LS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for JET_LS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for JET_LS {}
impl ::core::fmt::Debug for JET_LS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_LS").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_LS {
    type Abi = Self;
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_MoveFirst: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_MoveLast: u32 = 2147483647u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_MovePrevious: i32 = -1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct JET_OBJECTINFO {
    pub cbStruct: u32,
    pub objtyp: u32,
    pub dtCreate: f64,
    pub dtUpdate: f64,
    pub grbit: u32,
    pub flags: u32,
    pub cRecord: u32,
    pub cPage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for JET_OBJECTINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for JET_OBJECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for JET_OBJECTINFO {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for JET_OBJECTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_OBJECTINFO>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for JET_OBJECTINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_OBJECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(target_arch = "x86")]
pub struct JET_OBJECTINFO {
    pub cbStruct: u32,
    pub objtyp: u32,
    pub dtCreate: f64,
    pub dtUpdate: f64,
    pub grbit: u32,
    pub flags: u32,
    pub cRecord: u32,
    pub cPage: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for JET_OBJECTINFO {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for JET_OBJECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for JET_OBJECTINFO {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for JET_OBJECTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_OBJECTINFO>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for JET_OBJECTINFO {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_OBJECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_OBJECTLIST {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cRecord: u32,
    pub columnidcontainername: u32,
    pub columnidobjectname: u32,
    pub columnidobjtyp: u32,
    pub columniddtCreate: u32,
    pub columniddtUpdate: u32,
    pub columnidgrbit: u32,
    pub columnidflags: u32,
    pub columnidcRecord: u32,
    pub columnidcPage: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_OBJECTLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_OBJECTLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_OBJECTLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_OBJECTLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidcontainername", &self.columnidcontainername)
            .field("columnidobjectname", &self.columnidobjectname)
            .field("columnidobjtyp", &self.columnidobjtyp)
            .field("columniddtCreate", &self.columniddtCreate)
            .field("columniddtUpdate", &self.columniddtUpdate)
            .field("columnidgrbit", &self.columnidgrbit)
            .field("columnidflags", &self.columnidflags)
            .field("columnidcRecord", &self.columnidcRecord)
            .field("columnidcPage", &self.columnidcPage)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_OBJECTLIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_OBJECTLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_OBJECTLIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_OBJECTLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_OBJECTLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_OPENTEMPORARYTABLE {
    pub cbStruct: u32,
    pub prgcolumndef: *const JET_COLUMNDEF,
    pub ccolumn: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX,
    pub grbit: u32,
    pub prgcolumnid: *mut u32,
    pub cbKeyMost: u32,
    pub cbVarSegMac: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_OPENTEMPORARYTABLE {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_OPENTEMPORARYTABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_OPENTEMPORARYTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_OPENTEMPORARYTABLE").field("cbStruct", &self.cbStruct).field("prgcolumndef", &self.prgcolumndef).field("ccolumn", &self.ccolumn).field("pidxunicode", &self.pidxunicode).field("grbit", &self.grbit).field("prgcolumnid", &self.prgcolumnid).field("cbKeyMost", &self.cbKeyMost).field("cbVarSegMac", &self.cbVarSegMac).field("tableid", &self.tableid).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_OPENTEMPORARYTABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_OPENTEMPORARYTABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_OPENTEMPORARYTABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_OPENTEMPORARYTABLE {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_OPENTEMPORARYTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_OPENTEMPORARYTABLE2 {
    pub cbStruct: u32,
    pub prgcolumndef: *const JET_COLUMNDEF,
    pub ccolumn: u32,
    pub pidxunicode: *mut JET_UNICODEINDEX2,
    pub grbit: u32,
    pub prgcolumnid: *mut u32,
    pub cbKeyMost: u32,
    pub cbVarSegMac: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_OPENTEMPORARYTABLE2 {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_OPENTEMPORARYTABLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_OPENTEMPORARYTABLE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_OPENTEMPORARYTABLE2").field("cbStruct", &self.cbStruct).field("prgcolumndef", &self.prgcolumndef).field("ccolumn", &self.ccolumn).field("pidxunicode", &self.pidxunicode).field("grbit", &self.grbit).field("prgcolumnid", &self.prgcolumnid).field("cbKeyMost", &self.cbKeyMost).field("cbVarSegMac", &self.cbVarSegMac).field("tableid", &self.tableid).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_OPENTEMPORARYTABLE2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_OPENTEMPORARYTABLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_OPENTEMPORARYTABLE2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_OPENTEMPORARYTABLE2 {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_OPENTEMPORARYTABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_OPERATIONCONTEXT {
    pub ulUserID: u32,
    pub nOperationID: u8,
    pub nOperationType: u8,
    pub nClientType: u8,
    pub fFlags: u8,
}
impl ::core::marker::Copy for JET_OPERATIONCONTEXT {}
impl ::core::clone::Clone for JET_OPERATIONCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_OPERATIONCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_OPERATIONCONTEXT").field("ulUserID", &self.ulUserID).field("nOperationID", &self.nOperationID).field("nOperationType", &self.nOperationType).field("nClientType", &self.nClientType).field("fFlags", &self.fFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_OPERATIONCONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_OPERATIONCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_OPERATIONCONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_OPERATIONCONTEXT {}
impl ::core::default::Default for JET_OPERATIONCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_OSSNAPID(pub usize);
impl JET_OSSNAPID {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for JET_OSSNAPID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for JET_OSSNAPID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for JET_OSSNAPID {}
impl ::core::fmt::Debug for JET_OSSNAPID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_OSSNAPID").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_OSSNAPID {
    type Abi = Self;
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_OnlineDefragAll: u32 = 65535u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_OnlineDefragAllOBSOLETE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_OnlineDefragDatabases: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_OnlineDefragDisable: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_OnlineDefragSpaceTrees: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation', 'Win32_Storage_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
pub type JET_PFNDURABLECOMMITCALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: super::StructuredStorage::JET_INSTANCE, pcommitidseen: *const JET_COMMIT_ID, grbit: u32) -> i32>;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub type JET_PFNREALLOC = ::core::option::Option<unsafe extern "system" fn(pvcontext: *const ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub type JET_PFNSTATUS = ::core::option::Option<unsafe extern "system" fn(sesid: super::StructuredStorage::JET_SESID, snp: u32, snt: u32, pv: *const ::core::ffi::c_void) -> i32>;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_RBSINFOMISC {
    pub lRBSGeneration: i32,
    pub logtimeCreate: JET_LOGTIME,
    pub logtimeCreatePrevRBS: JET_LOGTIME,
    pub ulMajor: u32,
    pub ulMinor: u32,
    pub cbLogicalFileSize: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_RBSINFOMISC {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_RBSINFOMISC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_RBSINFOMISC {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_RBSINFOMISC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RBSINFOMISC>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_RBSINFOMISC {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_RBSINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_RBSINFOMISC {
    pub lRBSGeneration: i32,
    pub logtimeCreate: JET_LOGTIME,
    pub logtimeCreatePrevRBS: JET_LOGTIME,
    pub ulMajor: u32,
    pub ulMinor: u32,
    pub cbLogicalFileSize: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_RBSINFOMISC {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_RBSINFOMISC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_RBSINFOMISC {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_RBSINFOMISC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RBSINFOMISC>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_RBSINFOMISC {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_RBSINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_RBSREVERTINFOMISC {
    pub lGenMinRevertStart: i32,
    pub lGenMaxRevertStart: i32,
    pub lGenMinRevertEnd: i32,
    pub lGenMaxRevertEnd: i32,
    pub logtimeRevertFrom: JET_LOGTIME,
    pub cSecRevert: u64,
    pub cPagesReverted: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_RBSREVERTINFOMISC {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_RBSREVERTINFOMISC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_RBSREVERTINFOMISC {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_RBSREVERTINFOMISC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RBSREVERTINFOMISC>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_RBSREVERTINFOMISC {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_RBSREVERTINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_RBSREVERTINFOMISC {
    pub lGenMinRevertStart: i32,
    pub lGenMaxRevertStart: i32,
    pub lGenMinRevertEnd: i32,
    pub lGenMaxRevertEnd: i32,
    pub logtimeRevertFrom: JET_LOGTIME,
    pub cSecRevert: u64,
    pub cPagesReverted: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_RBSREVERTINFOMISC {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_RBSREVERTINFOMISC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_RBSREVERTINFOMISC {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_RBSREVERTINFOMISC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RBSREVERTINFOMISC>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_RBSREVERTINFOMISC {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_RBSREVERTINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_RECORDLIST {
    pub cbStruct: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cRecord: u32,
    pub columnidBookmark: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_RECORDLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_RECORDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_RECORDLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RECORDLIST").field("cbStruct", &self.cbStruct).field("tableid", &self.tableid).field("cRecord", &self.cRecord).field("columnidBookmark", &self.columnidBookmark).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_RECORDLIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_RECORDLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RECORDLIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_RECORDLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_RECORDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_RECPOS {
    pub cbStruct: u32,
    pub centriesLT: u32,
    pub centriesInRange: u32,
    pub centriesTotal: u32,
}
impl ::core::marker::Copy for JET_RECPOS {}
impl ::core::clone::Clone for JET_RECPOS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_RECPOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RECPOS").field("cbStruct", &self.cbStruct).field("centriesLT", &self.centriesLT).field("centriesInRange", &self.centriesInRange).field("centriesTotal", &self.centriesTotal).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_RECPOS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_RECPOS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RECPOS>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_RECPOS {}
impl ::core::default::Default for JET_RECPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct JET_RECSIZE {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for JET_RECSIZE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for JET_RECSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for JET_RECSIZE {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for JET_RECSIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RECSIZE>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for JET_RECSIZE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_RECSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(target_arch = "x86")]
pub struct JET_RECSIZE {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for JET_RECSIZE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for JET_RECSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for JET_RECSIZE {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for JET_RECSIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RECSIZE>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for JET_RECSIZE {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_RECSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct JET_RECSIZE2 {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
    pub cCompressedColumns: u64,
    pub cbDataCompressed: u64,
    pub cbLongValueDataCompressed: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for JET_RECSIZE2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for JET_RECSIZE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for JET_RECSIZE2 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for JET_RECSIZE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RECSIZE2>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for JET_RECSIZE2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_RECSIZE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(target_arch = "x86")]
pub struct JET_RECSIZE2 {
    pub cbData: u64,
    pub cbLongValueData: u64,
    pub cbOverhead: u64,
    pub cbLongValueOverhead: u64,
    pub cNonTaggedColumns: u64,
    pub cTaggedColumns: u64,
    pub cLongValues: u64,
    pub cMultiValues: u64,
    pub cCompressedColumns: u64,
    pub cbDataCompressed: u64,
    pub cbLongValueDataCompressed: u64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for JET_RECSIZE2 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for JET_RECSIZE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for JET_RECSIZE2 {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for JET_RECSIZE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RECSIZE2>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for JET_RECSIZE2 {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_RECSIZE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JET_RELOP(pub i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_relopEquals: JET_RELOP = JET_RELOP(0i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_relopPrefixEquals: JET_RELOP = JET_RELOP(1i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_relopNotEquals: JET_RELOP = JET_RELOP(2i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_relopLessThanOrEqual: JET_RELOP = JET_RELOP(3i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_relopLessThan: JET_RELOP = JET_RELOP(4i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_relopGreaterThanOrEqual: JET_RELOP = JET_RELOP(5i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_relopGreaterThan: JET_RELOP = JET_RELOP(6i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_relopBitmaskEqualsZero: JET_RELOP = JET_RELOP(7i32);
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_relopBitmaskNotEqualsZero: JET_RELOP = JET_RELOP(8i32);
impl ::core::marker::Copy for JET_RELOP {}
impl ::core::clone::Clone for JET_RELOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JET_RELOP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JET_RELOP {
    type Abi = Self;
}
impl ::core::fmt::Debug for JET_RELOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_RELOP").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_RETINFO {
    pub cbStruct: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
    pub columnidNextTagged: u32,
}
impl ::core::marker::Copy for JET_RETINFO {}
impl ::core::clone::Clone for JET_RETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_RETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RETINFO").field("cbStruct", &self.cbStruct).field("ibLongValue", &self.ibLongValue).field("itagSequence", &self.itagSequence).field("columnidNextTagged", &self.columnidNextTagged).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_RETINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_RETINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RETINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_RETINFO {}
impl ::core::default::Default for JET_RETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_RETRIEVECOLUMN {
    pub columnid: u32,
    pub pvData: *mut ::core::ffi::c_void,
    pub cbData: u32,
    pub cbActual: u32,
    pub grbit: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
    pub columnidNextTagged: u32,
    pub err: i32,
}
impl ::core::marker::Copy for JET_RETRIEVECOLUMN {}
impl ::core::clone::Clone for JET_RETRIEVECOLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_RETRIEVECOLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RETRIEVECOLUMN").field("columnid", &self.columnid).field("pvData", &self.pvData).field("cbData", &self.cbData).field("cbActual", &self.cbActual).field("grbit", &self.grbit).field("ibLongValue", &self.ibLongValue).field("itagSequence", &self.itagSequence).field("columnidNextTagged", &self.columnidNextTagged).field("err", &self.err).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_RETRIEVECOLUMN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_RETRIEVECOLUMN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RETRIEVECOLUMN>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_RETRIEVECOLUMN {}
impl ::core::default::Default for JET_RETRIEVECOLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation', 'Win32_Storage_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
pub struct JET_RSTINFO_A {
    pub cbStruct: u32,
    pub rgrstmap: *mut JET_RSTMAP_A,
    pub crstmap: i32,
    pub lgposStop: JET_LGPOS,
    pub logtimeStop: JET_LOGTIME,
    pub pfnStatus: JET_PFNSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::marker::Copy for JET_RSTINFO_A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::clone::Clone for JET_RSTINFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
unsafe impl ::windows::core::Abi for JET_RSTINFO_A {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::cmp::PartialEq for JET_RSTINFO_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RSTINFO_A>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::cmp::Eq for JET_RSTINFO_A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::default::Default for JET_RSTINFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation', 'Win32_Storage_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
pub struct JET_RSTINFO_W {
    pub cbStruct: u32,
    pub rgrstmap: *mut JET_RSTMAP_W,
    pub crstmap: i32,
    pub lgposStop: JET_LGPOS,
    pub logtimeStop: JET_LOGTIME,
    pub pfnStatus: JET_PFNSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::marker::Copy for JET_RSTINFO_W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::clone::Clone for JET_RSTINFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
unsafe impl ::windows::core::Abi for JET_RSTINFO_W {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::cmp::PartialEq for JET_RSTINFO_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RSTINFO_W>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::cmp::Eq for JET_RSTINFO_W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::default::Default for JET_RSTINFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_RSTMAP_A {
    pub szDatabaseName: ::windows::core::PSTR,
    pub szNewDatabaseName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for JET_RSTMAP_A {}
impl ::core::clone::Clone for JET_RSTMAP_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_RSTMAP_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RSTMAP_A").field("szDatabaseName", &self.szDatabaseName).field("szNewDatabaseName", &self.szNewDatabaseName).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_RSTMAP_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_RSTMAP_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RSTMAP_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_RSTMAP_A {}
impl ::core::default::Default for JET_RSTMAP_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_RSTMAP_W {
    pub szDatabaseName: ::windows::core::PWSTR,
    pub szNewDatabaseName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for JET_RSTMAP_W {}
impl ::core::clone::Clone for JET_RSTMAP_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_RSTMAP_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RSTMAP_W").field("szDatabaseName", &self.szDatabaseName).field("szNewDatabaseName", &self.szNewDatabaseName).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_RSTMAP_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_RSTMAP_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_RSTMAP_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_RSTMAP_W {}
impl ::core::default::Default for JET_RSTMAP_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_SETCOLUMN {
    pub columnid: u32,
    pub pvData: *const ::core::ffi::c_void,
    pub cbData: u32,
    pub grbit: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
    pub err: i32,
}
impl ::core::marker::Copy for JET_SETCOLUMN {}
impl ::core::clone::Clone for JET_SETCOLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_SETCOLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SETCOLUMN").field("columnid", &self.columnid).field("pvData", &self.pvData).field("cbData", &self.cbData).field("grbit", &self.grbit).field("ibLongValue", &self.ibLongValue).field("itagSequence", &self.itagSequence).field("err", &self.err).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_SETCOLUMN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_SETCOLUMN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_SETCOLUMN>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_SETCOLUMN {}
impl ::core::default::Default for JET_SETCOLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_SETINFO {
    pub cbStruct: u32,
    pub ibLongValue: u32,
    pub itagSequence: u32,
}
impl ::core::marker::Copy for JET_SETINFO {}
impl ::core::clone::Clone for JET_SETINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_SETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SETINFO").field("cbStruct", &self.cbStruct).field("ibLongValue", &self.ibLongValue).field("itagSequence", &self.itagSequence).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_SETINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_SETINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_SETINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_SETINFO {}
impl ::core::default::Default for JET_SETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_SETSYSPARAM_A {
    pub paramid: u32,
    pub lParam: super::StructuredStorage::JET_API_PTR,
    pub sz: ::windows::core::PCSTR,
    pub err: i32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_SETSYSPARAM_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_SETSYSPARAM_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_SETSYSPARAM_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SETSYSPARAM_A").field("paramid", &self.paramid).field("lParam", &self.lParam).field("sz", &self.sz).field("err", &self.err).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_SETSYSPARAM_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_SETSYSPARAM_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_SETSYSPARAM_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_SETSYSPARAM_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_SETSYSPARAM_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_SETSYSPARAM_W {
    pub paramid: u32,
    pub lParam: super::StructuredStorage::JET_API_PTR,
    pub sz: ::windows::core::PCWSTR,
    pub err: i32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_SETSYSPARAM_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_SETSYSPARAM_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_SETSYSPARAM_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SETSYSPARAM_W").field("paramid", &self.paramid).field("lParam", &self.lParam).field("sz", &self.sz).field("err", &self.err).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_SETSYSPARAM_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_SETSYSPARAM_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_SETSYSPARAM_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_SETSYSPARAM_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_SETSYSPARAM_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JET_SIGNATURE {
    pub ulRandom: u32,
    pub logtimeCreate: JET_LOGTIME,
    pub szComputerName: [super::super::Foundation::CHAR; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JET_SIGNATURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JET_SIGNATURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JET_SIGNATURE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_SIGNATURE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_SIGNATURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_SNPROG {
    pub cbStruct: u32,
    pub cunitDone: u32,
    pub cunitTotal: u32,
}
impl ::core::marker::Copy for JET_SNPROG {}
impl ::core::clone::Clone for JET_SNPROG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_SNPROG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SNPROG").field("cbStruct", &self.cbStruct).field("cunitDone", &self.cunitDone).field("cunitTotal", &self.cunitTotal).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_SNPROG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_SNPROG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_SNPROG>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_SNPROG {}
impl ::core::default::Default for JET_SNPROG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_SPACEHINTS {
    pub cbStruct: u32,
    pub ulInitialDensity: u32,
    pub cbInitial: u32,
    pub grbit: u32,
    pub ulMaintDensity: u32,
    pub ulGrowth: u32,
    pub cbMinExtent: u32,
    pub cbMaxExtent: u32,
}
impl ::core::marker::Copy for JET_SPACEHINTS {}
impl ::core::clone::Clone for JET_SPACEHINTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_SPACEHINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SPACEHINTS").field("cbStruct", &self.cbStruct).field("ulInitialDensity", &self.ulInitialDensity).field("cbInitial", &self.cbInitial).field("grbit", &self.grbit).field("ulMaintDensity", &self.ulMaintDensity).field("ulGrowth", &self.ulGrowth).field("cbMinExtent", &self.cbMinExtent).field("cbMaxExtent", &self.cbMaxExtent).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_SPACEHINTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_SPACEHINTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_SPACEHINTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_SPACEHINTS {}
impl ::core::default::Default for JET_SPACEHINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_TABLECREATE2_A {
    pub cbStruct: u32,
    pub szTableName: ::windows::core::PSTR,
    pub szTemplateTableName: ::windows::core::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_A,
    pub cIndexes: u32,
    pub szCallback: ::windows::core::PSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_TABLECREATE2_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_TABLECREATE2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE2_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE2_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_TABLECREATE2_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_TABLECREATE2_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE2_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_TABLECREATE2_W {
    pub cbStruct: u32,
    pub szTableName: ::windows::core::PWSTR,
    pub szTemplateTableName: ::windows::core::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_W,
    pub cIndexes: u32,
    pub szCallback: ::windows::core::PWSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_TABLECREATE2_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_TABLECREATE2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE2_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE2_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_TABLECREATE2_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_TABLECREATE2_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE2_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_TABLECREATE3_A {
    pub cbStruct: u32,
    pub szTableName: ::windows::core::PSTR,
    pub szTemplateTableName: ::windows::core::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE2_A,
    pub cIndexes: u32,
    pub szCallback: ::windows::core::PSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_TABLECREATE3_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_TABLECREATE3_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE3_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE3_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_TABLECREATE3_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE3_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_TABLECREATE3_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE3_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_TABLECREATE3_W {
    pub cbStruct: u32,
    pub szTableName: ::windows::core::PWSTR,
    pub szTemplateTableName: ::windows::core::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE2_W,
    pub cIndexes: u32,
    pub szCallback: ::windows::core::PWSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_TABLECREATE3_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_TABLECREATE3_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE3_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE3_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_TABLECREATE3_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE3_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_TABLECREATE3_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE3_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_TABLECREATE4_A {
    pub cbStruct: u32,
    pub szTableName: ::windows::core::PSTR,
    pub szTemplateTableName: ::windows::core::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE3_A,
    pub cIndexes: u32,
    pub szCallback: ::windows::core::PSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_TABLECREATE4_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_TABLECREATE4_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE4_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE4_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_TABLECREATE4_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE4_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_TABLECREATE4_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE4_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE4_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_TABLECREATE4_W {
    pub cbStruct: u32,
    pub szTableName: ::windows::core::PWSTR,
    pub szTemplateTableName: ::windows::core::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE3_W,
    pub cIndexes: u32,
    pub szCallback: ::windows::core::PWSTR,
    pub cbtyp: u32,
    pub grbit: u32,
    pub pSeqSpacehints: *mut JET_SPACEHINTS,
    pub pLVSpacehints: *mut JET_SPACEHINTS,
    pub cbSeparateLV: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_TABLECREATE4_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_TABLECREATE4_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE4_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE4_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_TABLECREATE4_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE4_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_TABLECREATE4_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE4_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE4_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_TABLECREATE_A {
    pub cbStruct: u32,
    pub szTableName: ::windows::core::PSTR,
    pub szTemplateTableName: ::windows::core::PSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_A,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_A,
    pub cIndexes: u32,
    pub grbit: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_TABLECREATE_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_TABLECREATE_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_TABLECREATE_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_TABLECREATE_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub struct JET_TABLECREATE_W {
    pub cbStruct: u32,
    pub szTableName: ::windows::core::PWSTR,
    pub szTemplateTableName: ::windows::core::PWSTR,
    pub ulPages: u32,
    pub ulDensity: u32,
    pub rgcolumncreate: *mut JET_COLUMNCREATE_W,
    pub cColumns: u32,
    pub rgindexcreate: *mut JET_INDEXCREATE_W,
    pub cIndexes: u32,
    pub grbit: u32,
    pub tableid: super::StructuredStorage::JET_TABLEID,
    pub cCreated: u32,
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::marker::Copy for JET_TABLECREATE_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::clone::Clone for JET_TABLECREATE_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
unsafe impl ::windows::core::Abi for JET_TABLECREATE_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_TABLECREATE_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_THREADSTATS {
    pub cbStruct: u32,
    pub cPageReferenced: u32,
    pub cPageRead: u32,
    pub cPagePreread: u32,
    pub cPageDirtied: u32,
    pub cPageRedirtied: u32,
    pub cLogRecord: u32,
    pub cbLogRecord: u32,
}
impl ::core::marker::Copy for JET_THREADSTATS {}
impl ::core::clone::Clone for JET_THREADSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_THREADSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_THREADSTATS").field("cbStruct", &self.cbStruct).field("cPageReferenced", &self.cPageReferenced).field("cPageRead", &self.cPageRead).field("cPagePreread", &self.cPagePreread).field("cPageDirtied", &self.cPageDirtied).field("cPageRedirtied", &self.cPageRedirtied).field("cLogRecord", &self.cLogRecord).field("cbLogRecord", &self.cbLogRecord).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_THREADSTATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_THREADSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_THREADSTATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_THREADSTATS {}
impl ::core::default::Default for JET_THREADSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct JET_THREADSTATS2 {
    pub cbStruct: u32,
    pub cPageReferenced: u32,
    pub cPageRead: u32,
    pub cPagePreread: u32,
    pub cPageDirtied: u32,
    pub cPageRedirtied: u32,
    pub cLogRecord: u32,
    pub cbLogRecord: u32,
    pub cusecPageCacheMiss: u64,
    pub cPageCacheMiss: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for JET_THREADSTATS2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for JET_THREADSTATS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for JET_THREADSTATS2 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for JET_THREADSTATS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_THREADSTATS2>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for JET_THREADSTATS2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_THREADSTATS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[cfg(target_arch = "x86")]
pub struct JET_THREADSTATS2 {
    pub cbStruct: u32,
    pub cPageReferenced: u32,
    pub cPageRead: u32,
    pub cPagePreread: u32,
    pub cPageDirtied: u32,
    pub cPageRedirtied: u32,
    pub cLogRecord: u32,
    pub cbLogRecord: u32,
    pub cusecPageCacheMiss: u64,
    pub cPageCacheMiss: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for JET_THREADSTATS2 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for JET_THREADSTATS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for JET_THREADSTATS2 {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for JET_THREADSTATS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_THREADSTATS2>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for JET_THREADSTATS2 {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_THREADSTATS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_TUPLELIMITS {
    pub chLengthMin: u32,
    pub chLengthMax: u32,
    pub chToIndexMax: u32,
    pub cchIncrement: u32,
    pub ichStart: u32,
}
impl ::core::marker::Copy for JET_TUPLELIMITS {}
impl ::core::clone::Clone for JET_TUPLELIMITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_TUPLELIMITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TUPLELIMITS").field("chLengthMin", &self.chLengthMin).field("chLengthMax", &self.chLengthMax).field("chToIndexMax", &self.chToIndexMax).field("cchIncrement", &self.cchIncrement).field("ichStart", &self.ichStart).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_TUPLELIMITS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_TUPLELIMITS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_TUPLELIMITS>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_TUPLELIMITS {}
impl ::core::default::Default for JET_TUPLELIMITS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_UNICODEINDEX {
    pub lcid: u32,
    pub dwMapFlags: u32,
}
impl ::core::marker::Copy for JET_UNICODEINDEX {}
impl ::core::clone::Clone for JET_UNICODEINDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_UNICODEINDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_UNICODEINDEX").field("lcid", &self.lcid).field("dwMapFlags", &self.dwMapFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_UNICODEINDEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_UNICODEINDEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_UNICODEINDEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_UNICODEINDEX {}
impl ::core::default::Default for JET_UNICODEINDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_UNICODEINDEX2 {
    pub szLocaleName: ::windows::core::PWSTR,
    pub dwMapFlags: u32,
}
impl ::core::marker::Copy for JET_UNICODEINDEX2 {}
impl ::core::clone::Clone for JET_UNICODEINDEX2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_UNICODEINDEX2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_UNICODEINDEX2").field("szLocaleName", &self.szLocaleName).field("dwMapFlags", &self.dwMapFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_UNICODEINDEX2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_UNICODEINDEX2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_UNICODEINDEX2>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_UNICODEINDEX2 {}
impl ::core::default::Default for JET_UNICODEINDEX2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_USERDEFINEDDEFAULT_A {
    pub szCallback: ::windows::core::PSTR,
    pub pbUserData: *mut u8,
    pub cbUserData: u32,
    pub szDependantColumns: ::windows::core::PSTR,
}
impl ::core::marker::Copy for JET_USERDEFINEDDEFAULT_A {}
impl ::core::clone::Clone for JET_USERDEFINEDDEFAULT_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_USERDEFINEDDEFAULT_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_USERDEFINEDDEFAULT_A").field("szCallback", &self.szCallback).field("pbUserData", &self.pbUserData).field("cbUserData", &self.cbUserData).field("szDependantColumns", &self.szDependantColumns).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_USERDEFINEDDEFAULT_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_USERDEFINEDDEFAULT_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_USERDEFINEDDEFAULT_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_USERDEFINEDDEFAULT_A {}
impl ::core::default::Default for JET_USERDEFINEDDEFAULT_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub struct JET_USERDEFINEDDEFAULT_W {
    pub szCallback: ::windows::core::PWSTR,
    pub pbUserData: *mut u8,
    pub cbUserData: u32,
    pub szDependantColumns: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for JET_USERDEFINEDDEFAULT_W {}
impl ::core::clone::Clone for JET_USERDEFINEDDEFAULT_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JET_USERDEFINEDDEFAULT_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_USERDEFINEDDEFAULT_W").field("szCallback", &self.szCallback).field("pbUserData", &self.pbUserData).field("cbUserData", &self.cbUserData).field("szDependantColumns", &self.szDependantColumns).finish()
    }
}
unsafe impl ::windows::core::Abi for JET_USERDEFINEDDEFAULT_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JET_USERDEFINEDDEFAULT_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JET_USERDEFINEDDEFAULT_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for JET_USERDEFINEDDEFAULT_W {}
impl ::core::default::Default for JET_USERDEFINEDDEFAULT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_VERSION: u32 = 1280u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitAbortSnapshot: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitAllDatabasesSnapshot: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitBackupAtomic: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitBackupEndAbort: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitBackupEndNormal: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitBackupIncremental: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitBackupSnapshot: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitBackupTruncateDone: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitBookmarkPermitVirtualCurrency: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitCheckUniqueness: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnAutoincrement: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnCompressed: u32 = 524288u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnDeleteOnZero: u32 = 131072u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnEscrowUpdate: u32 = 2048u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnFinalize: u32 = 16384u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnFixed: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnMaybeNull: u32 = 8192u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnMultiValued: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnNotNULL: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnTTDescending: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnTTKey: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnTagged: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnUnversioned: u32 = 4096u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnUpdatable: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnUserDefinedDefault: u32 = 32768u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitColumnVersion: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitCommitLazyFlush: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitCompactRepair: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitCompactStats: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitConfigStoreReadControlDefault: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitConfigStoreReadControlDisableAll: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitConfigStoreReadControlInhibitRead: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitContinueAfterThaw: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitCopySnapshot: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitCreateHintAppendSequential: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitCreateHintHotpointSequential: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbDeleteCorruptIndexes: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbDeleteUnicodeIndexes: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbEnableBackgroundMaintenance: u32 = 2048u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbExclusive: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbOverwriteExisting: u32 = 512u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbPurgeCacheOnAttach: u32 = 4096u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbReadOnly: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbRecoveryOff: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbShadowingOff: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDbUpgrade: u32 = 512u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDefragmentAvailSpaceTreesOnly: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDefragmentBTree: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDefragmentBatchStart: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDefragmentBatchStop: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDefragmentNoPartialMerges: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDeleteAllExistingLogs: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDeleteColumnIgnoreTemplateColumns: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDeleteHintTableSequential: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDumpCacheIncludeCachedPages: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDumpCacheIncludeCorruptedPages: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDumpCacheIncludeDirtyPages: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDumpCacheMaximum: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDumpCacheMinimum: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDumpCacheNoDecommit: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDumpMaximum: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDumpMinimum: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitDurableCommitCallbackLogUnavailable: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitESE98FileNames: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitEightDotThreeSoftCompat: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitEnumerateCompressOutput: u32 = 524288u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitEnumerateCopy: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitEnumerateIgnoreDefault: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitEnumerateIgnoreUserDefinedDefault: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitEnumerateInRecordOnly: u32 = 2097152u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitEnumeratePresenceOnly: u32 = 131072u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitEnumerateTaggedOnly: u32 = 262144u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitEscrowNoRollback: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitExplicitPrepare: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitForceDetach: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitForceNewLog: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitFullColumnEndLimit: u32 = 512u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitFullColumnStartLimit: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitHungIOEvent: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIdleCompact: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIdleFlushBuffers: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIdleStatus: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIncrementalSnapshot: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexColumnMustBeNonNull: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexColumnMustBeNull: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexCrossProduct: u32 = 16384u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexDisallowNull: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexDisallowTruncation: u32 = 65536u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexDotNetGuid: u32 = 262144u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexEmpty: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexIgnoreAnyNull: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexIgnoreFirstNull: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexIgnoreNull: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexImmutableStructure: u32 = 524288u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexKeyMost: u32 = 32768u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexLazyFlush: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexNestedTable: u32 = 131072u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexPrimary: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexSortNullsHigh: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexTupleLimits: u32 = 8192u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexTuples: u32 = 4096u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexUnicode: u32 = 2048u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexUnique: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitIndexUnversioned: u32 = 512u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitKeepDbAttachedAtEndOfRecovery: u32 = 4096u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitKeyAscending: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitKeyDataZeroLength: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitKeyDescending: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitLSCursor: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitLSReset: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitLSTable: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitLogStreamMustExist: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitMoveFirst: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitMoveKeyNE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitNewKey: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitNoMove: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitNormalizedKey: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitObjectSystem: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitObjectTableDerived: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitObjectTableFixedDDL: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitObjectTableNoFixedVarColumnsInDerivedTables: u32 = 67108864u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitObjectTableTemplate: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitPartialColumnEndLimit: u32 = 2048u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitPartialColumnStartLimit: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitPrereadBackward: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitPrereadFirstPage: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitPrereadForward: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitPrereadNormalizedKey: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRangeInclusive: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRangeInstantDuration: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRangeRemove: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRangeUpperLimit: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitReadLock: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRecordInIndex: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRecordNotInIndex: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRecordSizeInCopyBuffer: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRecordSizeLocal: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRecordSizeRunningTotal: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRecoveryWithoutUndo: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitReplayIgnoreLostLogs: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitReplayIgnoreMissingDB: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitReplayMissingMapEntryDB: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitResizeDatabaseOnlyGrow: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitResizeDatabaseOnlyShrink: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveCopy: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveFromIndex: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveFromPrimaryBookmark: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveHintReserve1: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveHintReserve2: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveHintReserve3: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveHintTableScanBackward: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveHintTableScanForward: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveIgnoreDefault: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveNull: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveTag: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRetrieveTuple: u32 = 2048u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitRollbackAll: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSeekEQ: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSeekGE: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSeekGT: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSeekLE: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSeekLT: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetAppendLV: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetCompressed: u32 = 131072u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetContiguousLV: u32 = 262144u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetIndexRange: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetIntrinsicLV: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetOverwriteLV: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetRevertToDefaultValue: u32 = 512u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetSeparateLV: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetSizeLV: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetUncompressed: u32 = 65536u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetUniqueMultiValues: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetUniqueNormalizedMultiValues: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSetZeroLength: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitShrinkDatabaseOff: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitShrinkDatabaseOn: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitShrinkDatabaseRealtime: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitShrinkDatabaseTrim: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSpaceHintsUtilizeParentSpace: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitStopServiceAll: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitStopServiceBackgroundUserTasks: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitStopServiceQuiesceCaches: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitStopServiceResume: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitStrLimit: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitSubStrLimit: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTDotNetGuid: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTErrorOnDuplicateInsertion: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTForceMaterialization: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTForwardOnly: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTIndexed: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTIntrinsicLVsOnly: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTScrollable: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTSortNullsHigh: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTUnique: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTTUpdatable: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass1: u32 = 65536u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass10: u32 = 655360u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass11: u32 = 720896u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass12: u32 = 786432u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass13: u32 = 851968u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass14: u32 = 917504u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass15: u32 = 983040u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass2: u32 = 131072u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass3: u32 = 196608u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass4: u32 = 262144u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass5: u32 = 327680u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass6: u32 = 393216u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass7: u32 = 458752u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass8: u32 = 524288u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClass9: u32 = 589824u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClassMask: u32 = 2031616u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableClassNone: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableCreateFixedDDL: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableCreateImmutableStructure: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableCreateNoFixedVarColumnsInDerivedTables: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableCreateTemplateTable: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableDenyRead: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableDenyWrite: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableInfoBookmark: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableInfoRollback: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableInfoUpdatable: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableNoCache: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableOpportuneRead: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTablePermitDDL: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTablePreread: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableReadOnly: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableSequential: u32 = 32768u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTableUpdatable: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTermAbrupt: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTermComplete: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTermDirty: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTermStopBackup: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTransactionReadOnly: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitTruncateLogsAfterRecovery: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitUpdateCheckESE97Compatibility: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitWaitAllLevel0Commit: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitWaitLastLevel0Commit: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitWriteLock: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_bitZeroLength: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbBookmarkMost: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbColumnLVPageOverhead: u32 = 82u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbColumnMost: u32 = 255u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbFullNameMost: u32 = 255u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbKeyMost: u32 = 255u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbKeyMost2KBytePage: u32 = 500u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbKeyMost4KBytePage: u32 = 1000u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbKeyMost8KBytePage: u32 = 2000u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbKeyMostMin: u32 = 255u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbLVColumnMost: u32 = 2147483647u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbLVDefaultValueMost: u32 = 255u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbLimitKeyMost: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbNameMost: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbPrimaryKeyMost: u32 = 255u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbSecondaryKeyMost: u32 = 255u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypAfterDelete: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypAfterInsert: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypAfterReplace: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypBeforeDelete: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypBeforeInsert: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypBeforeReplace: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypFinalize: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypFreeCursorLS: u32 = 512u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypFreeTableLS: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypNull: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypOnlineDefragCompleted: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_cbtypUserDefinedDefaultValue: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ccolFixedMost: u32 = 127u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ccolKeyMost: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ccolMost: u32 = 65248u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_ccolVarMost: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypBinary: u32 = 9u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypBit: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypCurrency: u32 = 5u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypDateTime: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypGUID: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypIEEEDouble: u32 = 7u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypIEEESingle: u32 = 6u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypLong: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypLongBinary: u32 = 11u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypLongLong: u32 = 15u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypLongText: u32 = 12u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypMax: u32 = 13u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypNil: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypSLV: u32 = 13u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypShort: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypText: u32 = 10u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypUnsignedByte: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypUnsignedLong: u32 = 14u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypUnsignedLongLong: u32 = 18u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_coltypUnsignedShort: u32 = 17u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configDefault: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configDynamicMediumMemory: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configHighConcurrencyScaling: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configLowDiskFootprint: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configLowMemory: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configLowPower: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configMediumDiskFootprint: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configRemoveQuotas: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configRunSilent: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configSSDProfileIO: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_configUnthrottledMemory: u32 = 512u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_dbstateBeingConverted: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_dbstateCleanShutdown: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_dbstateDirtyShutdown: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_dbstateForceDetach: u32 = 5u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_dbstateJustCreated: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errAccessDenied: i32 = -1907i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errAfterInitialization: i32 = -1850i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errAlreadyInitialized: i32 = -1030i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errAlreadyPrepared: i32 = -1607i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errAttachedDatabaseMismatch: i32 = -1216i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBackupAbortByServer: i32 = -801i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBackupDirectoryNotEmpty: i32 = -504i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBackupInProgress: i32 = -505i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBackupNotAllowedYet: i32 = -523i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadBackupDatabaseSize: i32 = -561i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadBookmark: i32 = -328i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadCheckpointSignature: i32 = -532i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadColumnId: i32 = -1517i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadDbSignature: i32 = -531i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadEmptyPage: i32 = -351i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadItagSequence: i32 = -1518i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadLineCount: i32 = -354i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadLogSignature: i32 = -530i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadLogVersion: i32 = -514i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadPageLink: i32 = -327i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadParentPageLink: i32 = -338i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadPatchPage: i32 = -535i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBadRestoreTargetInstance: i32 = -577i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errBufferTooSmall: i32 = -1038i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCallbackFailed: i32 = -2101i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCallbackNotResolved: i32 = -2102i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotAddFixedVarColumnToDerivedTable: i32 = -1330i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotBeTagged: i32 = -1521i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotDeleteSystemTable: i32 = -1318i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotDeleteTempTable: i32 = -1317i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotDeleteTemplateTable: i32 = -1319i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotDisableVersioning: i32 = -1208i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotIndex: i32 = -1071i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotIndexOnEncryptedColumn: i32 = -1440i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotLogDuringRecoveryRedo: i32 = -512i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotMaterializeForwardOnlySort: i32 = -1113i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotNestDDL: i32 = -1325i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCannotSeparateIntrinsicLV: i32 = -416i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCatalogCorrupted: i32 = -1220i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCheckpointCorrupt: i32 = -533i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCheckpointDepthTooDeep: i32 = -614i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCheckpointFileNotFound: i32 = -542i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errClientRequestToStopJetService: i32 = -1329i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnCannotBeCompressed: i32 = -1538i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnCannotBeEncrypted: i32 = -1439i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnDoesNotFit: i32 = -1503i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnDuplicate: i32 = -1508i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnInRelationship: i32 = -1519i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnInUse: i32 = -1046i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnIndexed: i32 = -1505i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnLong: i32 = -1501i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnNoChunk: i32 = -1502i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnNoEncryptionKey: i32 = -1540i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnNotFound: i32 = -1507i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnNotUpdatable: i32 = -1048i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnRedundant: i32 = -1510i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errColumnTooBig: i32 = -1506i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCommittedLogFileCorrupt: i32 = -586i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errCommittedLogFilesMissing: i32 = -582i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errConsistentTimeMismatch: i32 = -551i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errContainerNotEmpty: i32 = -1043i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDDLNotInheritable: i32 = -1326i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDataHasChanged: i32 = -1611i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabase200Format: i32 = -1210i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabase400Format: i32 = -1211i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabase500Format: i32 = -1212i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseAlreadyRunningMaintenance: i32 = -2004i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseAlreadyUpgraded: i32 = -562i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseAttachedForRecovery: i32 = -1231i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseBufferDependenciesCorrupted: i32 = -255i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseCorrupted: i32 = -1206i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseCorruptedNoRepair: i32 = -1224i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseDirtyShutdown: i32 = -550i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseDuplicate: i32 = -1201i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseFileReadOnly: i32 = -1008i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseIdInUse: i32 = -1218i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseInUse: i32 = -1202i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseIncompleteUpgrade: i32 = -563i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseInconsistent: i32 = -550i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseInvalidName: i32 = -1204i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseInvalidPages: i32 = -1205i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseInvalidPath: i32 = -1217i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseLeakInSpace: i32 = -348i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseLocked: i32 = -1207i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseLogSetMismatch: i32 = -539i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseNotFound: i32 = -1203i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseNotReady: i32 = -1230i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabasePatchFileMismatch: i32 = -552i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseSharingViolation: i32 = -1215i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseSignInUse: i32 = -1222i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseStreamingFileMismatch: i32 = -540i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabaseUnavailable: i32 = -1091i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDatabasesNotFromSameSnapshot: i32 = -580i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDbTimeCorrupted: i32 = -344i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDbTimeTooNew: i32 = -567i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDbTimeTooOld: i32 = -566i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDecompressionFailed: i32 = -1620i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDecryptionFailed: i32 = -1622i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDefaultValueTooBig: i32 = -1524i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDeleteBackupFileFail: i32 = -524i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDensityInvalid: i32 = -1307i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDerivedColumnCorruption: i32 = -1529i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDirtyShutdown: i32 = -1116i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDisabledFunctionality: i32 = -112i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDiskFull: i32 = -1808i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDiskIO: i32 = -1022i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errDiskReadVerificationFailure: i32 = -1021i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errEncryptionBadItag: i32 = -1623i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errEndingRestoreLogTooLow: i32 = -553i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errEngineFormatVersionNoLongerSupportedTooLow: i32 = -619i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errEngineFormatVersionNotYetImplementedTooHigh: i32 = -620i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errEngineFormatVersionParamTooLowForRequestedFeature: i32 = -621i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errEngineFormatVersionSpecifiedTooLowForDatabaseVersion: i32 = -623i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errEngineFormatVersionSpecifiedTooLowForLogVersion: i32 = -622i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errEntryPointNotFound: i32 = -1911i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errExclusiveTableLockRequired: i32 = -1322i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errExistingLogFileHasBadSignature: i32 = -610i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errExistingLogFileIsNotContiguous: i32 = -611i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFeatureNotAvailable: i32 = -1001i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileAccessDenied: i32 = -1032i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileAlreadyExists: i32 = -1814i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileClose: i32 = -102i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileCompressed: i32 = -4005i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileIOAbort: i32 = -4002i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileIOBeyondEOF: i32 = -4001i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileIOFail: i32 = -4004i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileIORetry: i32 = -4003i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileIOSparse: i32 = -4000i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileInvalidType: i32 = -1812i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileNotFound: i32 = -1811i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFileSystemCorruption: i32 = -1121i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFilteredMoveNotSupported: i32 = -1124i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFixedDDL: i32 = -1323i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFixedInheritedDDL: i32 = -1324i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFlushMapDatabaseMismatch: i32 = -1919i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFlushMapUnrecoverable: i32 = -1920i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errFlushMapVersionUnsupported: i32 = -1918i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errForceDetachNotAllowed: i32 = -1219i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errGivenLogFileHasBadSignature: i32 = -555i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errGivenLogFileIsNotContiguous: i32 = -556i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIllegalOperation: i32 = -1312i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInTransaction: i32 = -1108i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexBuildCorrupted: i32 = -1412i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexCantBuild: i32 = -1401i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexDuplicate: i32 = -1403i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexHasPrimary: i32 = -1402i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexInUse: i32 = -1051i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexInvalidDef: i32 = -1406i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexMustStay: i32 = -1405i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexNotFound: i32 = -1404i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesCannotRetrieveFromIndex: i32 = -1436i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesInvalidLimits: i32 = -1435i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesKeyTooSmall: i32 = -1437i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesNonUniqueOnly: i32 = -1432i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesOneColumnOnly: i32 = -1431i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesSecondaryIndexOnly: i32 = -1430i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesTextBinaryColumnsOnly: i32 = -1433i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesTextColumnsOnly: i32 = -1433i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesTooManyColumns: i32 = -1431i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errIndexTuplesVarSegMacNotAllowed: i32 = -1434i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInitInProgress: i32 = -1031i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInstanceNameInUse: i32 = -1086i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInstanceUnavailable: i32 = -1090i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInstanceUnavailableDueToFatalLogDiskFull: i32 = -1092i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInternalError: i32 = -107i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidBackup: i32 = -526i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidBackupSequence: i32 = -521i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidBookmark: i32 = -1045i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidBufferSize: i32 = -1047i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidCodePage: i32 = -1063i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidColumnType: i32 = -1511i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidCountry: i32 = -1061i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidCreateDbVersion: i32 = -1225i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidCreateIndex: i32 = -1409i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidDatabase: i32 = -1028i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidDatabaseId: i32 = -1010i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidDatabaseVersion: i32 = -1209i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidDbparamId: i32 = -1095i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidFilename: i32 = -1044i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidGrbit: i32 = -900i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidIndexId: i32 = -1416i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidInstance: i32 = -1115i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidLCMapStringFlags: i32 = -1064i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidLVChunkSize: i32 = -1438i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidLanguageId: i32 = -1062i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidLogDirectory: i32 = -1025i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidLogSequence: i32 = -515i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidLoggedOperation: i32 = -500i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidName: i32 = -1002i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidObject: i32 = -1316i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidOnSort: i32 = -1702i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidOperation: i32 = -1906i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidParameter: i32 = -1003i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidPath: i32 = -1023i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidPlaceholderColumn: i32 = -1530i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidPreread: i32 = -424i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidSesid: i32 = -1104i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidSesparamId: i32 = -1093i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidSettings: i32 = -1328i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidSystemPath: i32 = -1024i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errInvalidTableId: i32 = -1310i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errKeyBoundary: i32 = -324i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errKeyDuplicate: i32 = -1605i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errKeyIsMade: i32 = -1516i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errKeyNotMade: i32 = -1608i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errKeyTooBig: i32 = -408i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errKeyTruncated: i32 = -346i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLSAlreadySet: i32 = -3001i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLSCallbackNotSpecified: i32 = -3000i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLSNotSet: i32 = -3002i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLVCorrupted: i32 = -1526i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLanguageNotSupported: i32 = -1619i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLinkNotSupported: i32 = -1052i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogBufferTooSmall: i32 = -517i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogCorruptDuringHardRecovery: i32 = -574i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogCorruptDuringHardRestore: i32 = -573i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogCorrupted: i32 = -1852i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogDisabledDueToRecoveryFailure: i32 = -511i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogDiskFull: i32 = -529i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogFileCorrupt: i32 = -501i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogFileNotCopied: i32 = -616i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogFilePathInUse: i32 = -1084i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogFileSizeMismatch: i32 = -541i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogFileSizeMismatchDatabasesConsistent: i32 = -545i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogGenerationMismatch: i32 = -513i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogReadVerifyFailure: i32 = -612i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogSectorSizeMismatch: i32 = -546i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogSectorSizeMismatchDatabasesConsistent: i32 = -547i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogSequenceChecksumMismatch: i32 = -590i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogSequenceEnd: i32 = -519i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogSequenceEndDatabasesConsistent: i32 = -548i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogTornWriteDuringHardRecovery: i32 = -571i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogTornWriteDuringHardRestore: i32 = -570i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLogWriteFail: i32 = -510i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errLoggingDisabled: i32 = -516i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMakeBackupDirectoryFail: i32 = -525i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMissingCurrentLogFiles: i32 = -565i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMissingFileToBackup: i32 = -569i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMissingFullBackup: i32 = -560i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMissingLogFile: i32 = -528i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMissingPatchPage: i32 = -534i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMissingPreviousLogFile: i32 = -509i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMissingRestoreLogFiles: i32 = -557i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMultiValuedColumnMustBeTagged: i32 = -1509i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMultiValuedDuplicate: i32 = -1525i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMultiValuedDuplicateAfterTruncation: i32 = -1528i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMultiValuedIndexViolation: i32 = -1411i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMustBeSeparateLongValue: i32 = -423i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMustDisableLoggingForDbUpgrade: i32 = -575i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errMustRollback: i32 = -1057i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNTSystemCallFailed: i32 = -334i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNoBackup: i32 = -520i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNoBackupDirectory: i32 = -503i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNoCurrentIndex: i32 = -1515i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNoCurrentRecord: i32 = -1603i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNodeCorrupted: i32 = -358i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNotInTransaction: i32 = -1054i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNotInitialized: i32 = -1029i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNullInvalid: i32 = -1504i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errNullKeyDisallowed: i32 = -1053i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOSSnapshotInvalidSequence: i32 = -2401i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOSSnapshotInvalidSnapId: i32 = -2404i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOSSnapshotNotAllowed: i32 = -2403i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOSSnapshotTimeOut: i32 = -2402i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errObjectDuplicate: i32 = -1314i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errObjectNotFound: i32 = -1305i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOneDatabasePerSession: i32 = -1916i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfAutoincrementValues: i32 = -1076i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfBuffers: i32 = -1014i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfCursors: i32 = -1013i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfDatabaseSpace: i32 = -1012i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfDbtimeValues: i32 = -1077i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfFileHandles: i32 = -1020i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfLongValueIDs: i32 = -1075i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfMemory: i32 = -1011i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfObjectIDs: i32 = -1074i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfSequentialIndexValues: i32 = -1078i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfSessions: i32 = -1101i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errOutOfThreads: i32 = -103i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPageBoundary: i32 = -323i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPageInitializedMismatch: i32 = -596i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPageNotInitialized: i32 = -1019i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPageSizeMismatch: i32 = -1213i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPageTagCorrupted: i32 = -357i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPartiallyAttachedDB: i32 = -1221i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPatchFileMissing: i32 = -538i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPermissionDenied: i32 = -1809i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPreviousVersion: i32 = -322i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errPrimaryIndexCorrupted: i32 = -1413i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errReadLostFlushVerifyFailure: i32 = -1119i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errReadPgnoVerifyFailure: i32 = -1118i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errReadVerifyFailure: i32 = -1018i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecordDeleted: i32 = -1017i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecordFormatConversionFailed: i32 = -1915i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecordNoCopy: i32 = -1602i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecordNotDeleted: i32 = -1072i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecordNotFound: i32 = -1601i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecordPrimaryChanged: i32 = -1604i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecordTooBig: i32 = -1026i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecordTooBigForBackwardCompatibility: i32 = -1112i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecoveredWithErrors: i32 = -527i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecoveredWithoutUndo: i32 = -579i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecoveredWithoutUndoDatabasesConsistent: i32 = -584i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRecoveryVerifyFailure: i32 = -1123i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRedoAbruptEnded: i32 = -536i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRequiredLogFilesMissing: i32 = -543i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRestoreInProgress: i32 = -506i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRestoreOfNonBackupDatabase: i32 = -615i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRfsFailure: i32 = -100i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRfsNotArmed: i32 = -101i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRollbackError: i32 = -1917i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRollbackRequired: i32 = -1109i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRunningInMultiInstanceMode: i32 = -1081i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errRunningInOneInstanceMode: i32 = -1080i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSPAvailExtCacheOutOfMemory: i32 = -342i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSPAvailExtCacheOutOfSync: i32 = -340i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSPAvailExtCorrupted: i32 = -341i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSPOwnExtCorrupted: i32 = -343i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSecondaryIndexCorrupted: i32 = -1414i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSectorSizeNotSupported: i32 = -583i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSeparatedLongValue: i32 = -421i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSesidTableIdMismatch: i32 = -1114i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSessionContextAlreadySet: i32 = -1912i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSessionContextNotSetByThisThread: i32 = -1913i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSessionInUse: i32 = -1914i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSessionSharingViolation: i32 = -1910i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSessionWriteConflict: i32 = -1111i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSoftRecoveryOnBackupDatabase: i32 = -544i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSoftRecoveryOnSnapshot: i32 = -581i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSpaceHintsInvalid: i32 = -2103i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errStartingRestoreLogTooHigh: i32 = -554i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errStreamingDataNotLogged: i32 = -549i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSuccess: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSystemParameterConflict: i32 = -1087i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSystemParamsAlreadySet: i32 = -1082i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errSystemPathInUse: i32 = -1083i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTableDuplicate: i32 = -1303i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTableInUse: i32 = -1304i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTableLocked: i32 = -1302i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTableNotEmpty: i32 = -1308i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTaggedNotNULL: i32 = -1514i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTaskDropped: i32 = -106i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTempFileOpenError: i32 = -1803i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTempPathInUse: i32 = -1085i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTermInProgress: i32 = -1000i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyActiveUsers: i32 = -1059i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyAttachedDatabases: i32 = -1805i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyColumns: i32 = -1040i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyIO: i32 = -105i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyIndexes: i32 = -1015i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyInstances: i32 = -1214i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyKeys: i32 = -1016i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyMempoolEntries: i32 = -1073i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyOpenDatabases: i32 = -1027i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyOpenIndexes: i32 = -1410i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyOpenTables: i32 = -1311i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyOpenTablesAndCleanupTimedOut: i32 = -1313i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManyRecords: i32 = -1094i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManySorts: i32 = -1701i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTooManySplits: i32 = -1909i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTransReadOnly: i32 = -1110i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTransTooDeep: i32 = -1103i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTransactionTooLong: i32 = -618i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errTransactionsNotReadyDuringRecovery: i32 = -1232i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errUnicodeLanguageValidationFailure: i32 = -604i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errUnicodeNormalizationNotSupported: i32 = -603i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errUnicodeTranslationBufferTooSmall: i32 = -601i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errUnicodeTranslationFail: i32 = -602i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errUnloadableOSFunctionality: i32 = -113i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errUpdateMustVersion: i32 = -1621i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errUpdateNotPrepared: i32 = -1609i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errVersionStoreEntryTooBig: i32 = -1065i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errVersionStoreOutOfMemory: i32 = -1069i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errVersionStoreOutOfMemoryAndCleanupTimedOut: i32 = -1066i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errWriteConflict: i32 = -1102i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_errWriteConflictPrimaryIndex: i32 = -1105i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_filetypeCheckpoint: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_filetypeDatabase: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_filetypeFlushMap: u32 = 7u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_filetypeLog: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_filetypeTempDatabase: u32 = 5u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_filetypeUnknown: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_objtypNil: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_objtypTable: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramAccessDeniedRetryPeriod: u32 = 53u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramAlternateDatabaseRecoveryPath: u32 = 113u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramBaseName: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramBatchIOBufferMax: u32 = 22u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCachePriority: u32 = 177u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCacheSize: u32 = 41u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCacheSizeMax: u32 = 23u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCacheSizeMin: u32 = 60u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCachedClosedTables: u32 = 125u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCheckFormatWhenOpenFail: u32 = 44u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCheckpointDepthMax: u32 = 24u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCheckpointIOMax: u32 = 135u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCircularLog: u32 = 17u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCleanupMismatchedLogFiles: u32 = 77u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCommitDefault: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramConfigStoreSpec: u32 = 189u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramConfiguration: u32 = 129u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramCreatePathIfNotExist: u32 = 100u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDatabasePageSize: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDbExtensionSize: u32 = 18u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDbScanIntervalMaxSec: u32 = 172u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDbScanIntervalMinSec: u32 = 171u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDbScanThrottle: u32 = 170u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDefragmentSequentialBTrees: u32 = 160u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDefragmentSequentialBTreesDensityCheckFrequency: u32 = 161u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDeleteOldLogs: u32 = 48u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDeleteOutOfRangeLogs: u32 = 52u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDisableCallbacks: u32 = 65u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDisablePerfmon: u32 = 107u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramDurableCommitCallback: u32 = 187u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableAdvanced: u32 = 130u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableDBScanInRecovery: u32 = 169u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableDBScanSerialization: u32 = 180u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableFileCache: u32 = 126u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableIndexChecking: u32 = 45u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableIndexCleanup: u32 = 54u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableOnlineDefrag: u32 = 35u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnablePersistedCallbacks: u32 = 156u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableRBS: u32 = 215u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableShrinkDatabase: u32 = 184u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableSqm: u32 = 188u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableTempTableVersioning: u32 = 46u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEnableViewCache: u32 = 127u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramErrorToString: u32 = 70u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEventLogCache: u32 = 99u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEventLoggingLevel: u32 = 51u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEventSource: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramEventSourceKey: u32 = 49u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramExceptionAction: u32 = 98u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramGlobalMinVerPages: u32 = 81u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramHungIOActions: u32 = 182u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramHungIOThreshold: u32 = 181u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramIOPriority: u32 = 152u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramIOThrottlingTimeQuanta: u32 = 162u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramIgnoreLogVersion: u32 = 47u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramIndexTupleIncrement: u32 = 132u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramIndexTupleStart: u32 = 133u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramIndexTuplesLengthMax: u32 = 111u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramIndexTuplesLengthMin: u32 = 110u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramIndexTuplesToIndexMax: u32 = 112u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramKeyMost: u32 = 134u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLRUKCorrInterval: u32 = 25u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLRUKHistoryMax: u32 = 26u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLRUKPolicy: u32 = 27u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLRUKTimeout: u32 = 28u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLRUKTrxCorrInterval: u32 = 29u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLVChunkSizeMost: u32 = 163u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLegacyFileNames: u32 = 136u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLogBuffers: u32 = 12u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLogCheckpointPeriod: u32 = 14u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLogFileCreateAsynch: u32 = 69u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLogFilePath: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLogFileSize: u32 = 11u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramLogWaitingUserMax: u32 = 15u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxCoalesceReadGapSize: u32 = 166u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxCoalesceReadSize: u32 = 164u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxCoalesceWriteGapSize: u32 = 167u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxCoalesceWriteSize: u32 = 165u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxColtyp: u32 = 131u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxCursors: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxInstances: u32 = 104u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxOpenTables: u32 = 6u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxSessions: u32 = 5u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxTemporaryTables: u32 = 10u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxTransactionSize: u32 = 178u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxValueInvalid: u32 = 217u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMaxVerPages: u32 = 9u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramMinDataForXpress: u32 = 183u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramNoInformationEvent: u32 = 50u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramOSSnapshotTimeout: u32 = 82u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramOneDatabasePerSession: u32 = 102u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramOutstandingIOMax: u32 = 30u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramPageFragment: u32 = 20u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramPageHintCacheSize: u32 = 101u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramPageTempDBMin: u32 = 19u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramPreferredMaxOpenTables: u32 = 7u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramPreferredVerPages: u32 = 63u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramPrereadIOMax: u32 = 179u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramProcessFriendlyName: u32 = 186u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramRBSFilePath: u32 = 216u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramRecordUpgradeDirtyLevel: u32 = 78u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramRecovery: u32 = 34u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramRuntimeCallback: u32 = 73u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramStartFlushThreshold: u32 = 31u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramStopFlushThreshold: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramSystemPath: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass10Name: u32 = 146u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass11Name: u32 = 147u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass12Name: u32 = 148u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass13Name: u32 = 149u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass14Name: u32 = 150u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass15Name: u32 = 151u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass1Name: u32 = 137u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass2Name: u32 = 138u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass3Name: u32 = 139u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass4Name: u32 = 140u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass5Name: u32 = 141u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass6Name: u32 = 142u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass7Name: u32 = 143u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass8Name: u32 = 144u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTableClass9Name: u32 = 145u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramTempPath: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramUnicodeIndexDefault: u32 = 72u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramUseFlushForWriteDurability: u32 = 214u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramVerPageSize: u32 = 128u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramVersionStoreTaskQueueMax: u32 = 105u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramWaitLogFlush: u32 = 13u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramWaypointLatency: u32 = 153u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_paramZeroDatabaseDuringBackup: u32 = 71u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_prepCancel: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_prepInsert: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_prepInsertCopy: u32 = 5u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_prepInsertCopyDeleteOriginal: u32 = 7u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_prepInsertCopyReplaceOriginal: u32 = 9u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_prepReplace: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_prepReplaceNoLock: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_revertstateCompleted: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_revertstateCopingLogs: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_revertstateInProgress: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_revertstateNone: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sesparamCommitDefault: u32 = 4097u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sesparamCorrelationID: u32 = 4101u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sesparamMaxValueInvalid: u32 = 4110u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sesparamOperationContext: u32 = 4100u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sesparamTransactionLevel: u32 = 4099u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_snpBackup: u32 = 9u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_snpCompact: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_snpRepair: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_snpRestore: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_snpScrub: u32 = 11u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_snpUpgrade: u32 = 10u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_snpUpgradeRecordFormat: u32 = 12u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sntBegin: u32 = 5u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sntComplete: u32 = 6u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sntFail: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sntProgress: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sntRequirements: u32 = 7u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sqmDisable: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sqmEnable: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_sqmFromCEIP: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnBufferTruncated: u32 = 1006u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnCallbackNotRegistered: u32 = 2100u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnDefault: u32 = 1537u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnMaxTruncated: u32 = 1512u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnMoreTags: u32 = 1533u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnNotInRecord: u32 = 1539u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnNotLocal: u32 = 1532u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnNull: u32 = 1004u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnPresent: u32 = 1535u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnReference: u32 = 1541u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnSetNull: u32 = 1068u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnSingleValue: u32 = 1536u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnSkipped: u32 = 1531u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnColumnTruncated: u32 = 1534u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnCommittedLogFilesLost: u32 = 585u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnCommittedLogFilesRemoved: u32 = 587u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnCopyLongValue: u32 = 1520u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnCorruptIndexDeleted: u32 = 1415u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnDataHasChanged: u32 = 1610u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnDatabaseAttached: u32 = 1007u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnDatabaseRepaired: u32 = 595u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnDefragAlreadyRunning: u32 = 2000u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnDefragNotRunning: u32 = 2001u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnExistingLogFileHasBadSignature: u32 = 558u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnExistingLogFileIsNotContiguous: u32 = 559u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnFileOpenReadOnly: u32 = 1813u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnFinishWithUndo: u32 = 588u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnIdleFull: u32 = 1908u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnKeyChanged: u32 = 1618u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnNoErrorInfo: u32 = 1055u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnNoIdleActivity: u32 = 1058u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnNoWriteLock: u32 = 1067u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnNyi: i32 = -1i32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnPrimaryIndexOutOfDate: u32 = 1417u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnRemainingVersions: u32 = 321u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnSecondaryIndexOutOfDate: u32 = 1418u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnSeekNotEqual: u32 = 1039u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnSeparateLongValue: u32 = 406u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnShrinkNotPossible: u32 = 1122u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnSkipThisRecord: u32 = 564u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnSortOverflow: u32 = 1009u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnTableEmpty: u32 = 1301u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnTableInUseBySystem: u32 = 1327u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnTargetInstanceRunning: u32 = 578u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wrnUniqueKey: u32 = 345u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wszConfigStoreReadControl: &'static str = "CsReadControl";
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wszConfigStoreRelPathSysParamDefault: &'static str = "SysParamDefault";
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const JET_wszConfigStoreRelPathSysParamOverride: &'static str = "SysParamOverride";
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetAddColumnA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szcolumnname: *const i8, pcolumndef: *const JET_COLUMNDEF, pvdefault: *const ::core::ffi::c_void, cbdefault: u32, pcolumnid: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetAddColumnA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const i8, pcolumndef: *const JET_COLUMNDEF, pvdefault: *const ::core::ffi::c_void, cbdefault: u32, pcolumnid: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetAddColumnA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szcolumnname), ::core::mem::transmute(pcolumndef), ::core::mem::transmute(pvdefault), ::core::mem::transmute(cbdefault), ::core::mem::transmute(pcolumnid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetAddColumnW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szcolumnname: *const u16, pcolumndef: *const JET_COLUMNDEF, pvdefault: *const ::core::ffi::c_void, cbdefault: u32, pcolumnid: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetAddColumnW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const u16, pcolumndef: *const JET_COLUMNDEF, pvdefault: *const ::core::ffi::c_void, cbdefault: u32, pcolumnid: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetAddColumnW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szcolumnname), ::core::mem::transmute(pcolumndef), ::core::mem::transmute(pvdefault), ::core::mem::transmute(cbdefault), ::core::mem::transmute(pcolumnid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetAttachDatabase2A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const i8, cpgdatabasesizemax: u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetAttachDatabase2A(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, cpgdatabasesizemax: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetAttachDatabase2A(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(cpgdatabasesizemax), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetAttachDatabase2W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const u16, cpgdatabasesizemax: u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetAttachDatabase2W(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, cpgdatabasesizemax: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetAttachDatabase2W(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(cpgdatabasesizemax), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetAttachDatabaseA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const i8, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetAttachDatabaseA(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetAttachDatabaseA(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetAttachDatabaseW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const u16, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetAttachDatabaseW(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetAttachDatabaseW(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBackupA(szbackuppath: *const i8, grbit: u32, pfnstatus: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBackupA(szbackuppath: *const i8, grbit: u32, pfnstatus: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetBackupA(::core::mem::transmute(szbackuppath), ::core::mem::transmute(grbit), ::core::mem::transmute(pfnstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBackupInstanceA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, szbackuppath: *const i8, grbit: u32, pfnstatus: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBackupInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szbackuppath: *const i8, grbit: u32, pfnstatus: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetBackupInstanceA(instance.into_param().abi(), ::core::mem::transmute(szbackuppath), ::core::mem::transmute(grbit), ::core::mem::transmute(pfnstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBackupInstanceW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, szbackuppath: *const u16, grbit: u32, pfnstatus: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBackupInstanceW(instance: super::StructuredStorage::JET_INSTANCE, szbackuppath: *const u16, grbit: u32, pfnstatus: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetBackupInstanceW(instance.into_param().abi(), ::core::mem::transmute(szbackuppath), ::core::mem::transmute(grbit), ::core::mem::transmute(pfnstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBackupW(szbackuppath: *const u16, grbit: u32, pfnstatus: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBackupW(szbackuppath: *const u16, grbit: u32, pfnstatus: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetBackupW(::core::mem::transmute(szbackuppath), ::core::mem::transmute(grbit), ::core::mem::transmute(pfnstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetBeginExternalBackup(grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBeginExternalBackup(grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetBeginExternalBackup(::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBeginExternalBackupInstance<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBeginExternalBackupInstance(instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetBeginExternalBackupInstance(instance.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBeginSessionA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, psesid: *mut super::StructuredStorage::JET_SESID, szusername: *const i8, szpassword: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBeginSessionA(instance: super::StructuredStorage::JET_INSTANCE, psesid: *mut super::StructuredStorage::JET_SESID, szusername: *const i8, szpassword: *const i8) -> i32;
        }
        ::core::mem::transmute(JetBeginSessionA(instance.into_param().abi(), ::core::mem::transmute(psesid), ::core::mem::transmute(szusername), ::core::mem::transmute(szpassword)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBeginSessionW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, psesid: *mut super::StructuredStorage::JET_SESID, szusername: *const u16, szpassword: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBeginSessionW(instance: super::StructuredStorage::JET_INSTANCE, psesid: *mut super::StructuredStorage::JET_SESID, szusername: *const u16, szpassword: *const u16) -> i32;
        }
        ::core::mem::transmute(JetBeginSessionW(instance.into_param().abi(), ::core::mem::transmute(psesid), ::core::mem::transmute(szusername), ::core::mem::transmute(szpassword)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBeginTransaction<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBeginTransaction(sesid: super::StructuredStorage::JET_SESID) -> i32;
        }
        ::core::mem::transmute(JetBeginTransaction(sesid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBeginTransaction2<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBeginTransaction2(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetBeginTransaction2(sesid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetBeginTransaction3<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, trxid: i64, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetBeginTransaction3(sesid: super::StructuredStorage::JET_SESID, trxid: i64, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetBeginTransaction3(sesid.into_param().abi(), ::core::mem::transmute(trxid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCloseDatabase<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCloseDatabase(sesid: super::StructuredStorage::JET_SESID, dbid: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCloseDatabase(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCloseFile<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_HANDLE>>(hffile: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCloseFile(hffile: super::StructuredStorage::JET_HANDLE) -> i32;
        }
        ::core::mem::transmute(JetCloseFile(hffile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCloseFileInstance<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_HANDLE>>(instance: Param0, hffile: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCloseFileInstance(instance: super::StructuredStorage::JET_INSTANCE, hffile: super::StructuredStorage::JET_HANDLE) -> i32;
        }
        ::core::mem::transmute(JetCloseFileInstance(instance.into_param().abi(), hffile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCloseTable<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCloseTable(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID) -> i32;
        }
        ::core::mem::transmute(JetCloseTable(sesid.into_param().abi(), tableid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCommitTransaction<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCommitTransaction(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCommitTransaction(sesid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation', 'Win32_Storage_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
#[inline]
pub unsafe fn JetCommitTransaction2<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, grbit: u32, cmsecdurablecommit: u32, pcommitid: *mut JET_COMMIT_ID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCommitTransaction2(sesid: super::StructuredStorage::JET_SESID, grbit: u32, cmsecdurablecommit: u32, pcommitid: *mut JET_COMMIT_ID) -> i32;
        }
        ::core::mem::transmute(JetCommitTransaction2(sesid.into_param().abi(), ::core::mem::transmute(grbit), ::core::mem::transmute(cmsecdurablecommit), ::core::mem::transmute(pcommitid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCompactA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szdatabasesrc: *const i8, szdatabasedest: *const i8, pfnstatus: JET_PFNSTATUS, pconvert: *const CONVERT_A, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCompactA(sesid: super::StructuredStorage::JET_SESID, szdatabasesrc: *const i8, szdatabasedest: *const i8, pfnstatus: ::windows::core::RawPtr, pconvert: *const CONVERT_A, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCompactA(sesid.into_param().abi(), ::core::mem::transmute(szdatabasesrc), ::core::mem::transmute(szdatabasedest), ::core::mem::transmute(pfnstatus), ::core::mem::transmute(pconvert), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCompactW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szdatabasesrc: *const u16, szdatabasedest: *const u16, pfnstatus: JET_PFNSTATUS, pconvert: *const CONVERT_W, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCompactW(sesid: super::StructuredStorage::JET_SESID, szdatabasesrc: *const u16, szdatabasedest: *const u16, pfnstatus: ::windows::core::RawPtr, pconvert: *const CONVERT_W, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCompactW(sesid.into_param().abi(), ::core::mem::transmute(szdatabasesrc), ::core::mem::transmute(szdatabasedest), ::core::mem::transmute(pfnstatus), ::core::mem::transmute(pconvert), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetComputeStats<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetComputeStats(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID) -> i32;
        }
        ::core::mem::transmute(JetComputeStats(sesid.into_param().abi(), tableid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetConfigureProcessForCrashDump(grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetConfigureProcessForCrashDump(grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetConfigureProcessForCrashDump(::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateDatabase2A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const i8, cpgdatabasesizemax: u32, pdbid: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateDatabase2A(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, cpgdatabasesizemax: u32, pdbid: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateDatabase2A(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(cpgdatabasesizemax), ::core::mem::transmute(pdbid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateDatabase2W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const u16, cpgdatabasesizemax: u32, pdbid: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateDatabase2W(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, cpgdatabasesizemax: u32, pdbid: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateDatabase2W(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(cpgdatabasesizemax), ::core::mem::transmute(pdbid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateDatabaseA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const i8, szconnect: *const i8, pdbid: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateDatabaseA(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, szconnect: *const i8, pdbid: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateDatabaseA(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(szconnect), ::core::mem::transmute(pdbid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateDatabaseW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const u16, szconnect: *const u16, pdbid: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateDatabaseW(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, szconnect: *const u16, pdbid: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateDatabaseW(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(szconnect), ::core::mem::transmute(pdbid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex2A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pindexcreate: &[JET_INDEXCREATE_A]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateIndex2A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE_A, cindexcreate: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateIndex2A(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pindexcreate)), pindexcreate.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex2W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pindexcreate: &[JET_INDEXCREATE_W]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateIndex2W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE_W, cindexcreate: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateIndex2W(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pindexcreate)), pindexcreate.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex3A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pindexcreate: &[JET_INDEXCREATE2_A]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateIndex3A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE2_A, cindexcreate: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateIndex3A(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pindexcreate)), pindexcreate.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex3W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pindexcreate: &[JET_INDEXCREATE2_W]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateIndex3W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE2_W, cindexcreate: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateIndex3W(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pindexcreate)), pindexcreate.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex4A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pindexcreate: &[JET_INDEXCREATE3_A]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateIndex4A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE3_A, cindexcreate: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateIndex4A(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pindexcreate)), pindexcreate.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndex4W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pindexcreate: &[JET_INDEXCREATE3_W]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateIndex4W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pindexcreate: *const JET_INDEXCREATE3_W, cindexcreate: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateIndex4W(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pindexcreate)), pindexcreate.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndexA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(sesid: Param0, tableid: Param1, szindexname: *const i8, grbit: u32, szkey: Param4, cbkey: u32, ldensity: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateIndexA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, grbit: u32, szkey: ::windows::core::PCSTR, cbkey: u32, ldensity: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateIndexA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(grbit), szkey.into_param().abi(), ::core::mem::transmute(cbkey), ::core::mem::transmute(ldensity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateIndexW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(sesid: Param0, tableid: Param1, szindexname: *const u16, grbit: u32, szkey: Param4, cbkey: u32, ldensity: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateIndexW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, grbit: u32, szkey: ::windows::core::PCWSTR, cbkey: u32, ldensity: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateIndexW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(grbit), szkey.into_param().abi(), ::core::mem::transmute(cbkey), ::core::mem::transmute(ldensity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateInstance2A(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const i8, szdisplayname: *const i8, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateInstance2A(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const i8, szdisplayname: *const i8, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateInstance2A(::core::mem::transmute(pinstance), ::core::mem::transmute(szinstancename), ::core::mem::transmute(szdisplayname), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateInstance2W(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const u16, szdisplayname: *const u16, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateInstance2W(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const u16, szdisplayname: *const u16, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetCreateInstance2W(::core::mem::transmute(pinstance), ::core::mem::transmute(szinstancename), ::core::mem::transmute(szdisplayname), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateInstanceA(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateInstanceA(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const i8) -> i32;
        }
        ::core::mem::transmute(JetCreateInstanceA(::core::mem::transmute(pinstance), ::core::mem::transmute(szinstancename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateInstanceW(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateInstanceW(pinstance: *mut super::StructuredStorage::JET_INSTANCE, szinstancename: *const u16) -> i32;
        }
        ::core::mem::transmute(JetCreateInstanceW(::core::mem::transmute(pinstance), ::core::mem::transmute(szinstancename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const i8, lpages: u32, ldensity: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, lpages: u32, ldensity: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32;
        }
        ::core::mem::transmute(JetCreateTableA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(lpages), ::core::mem::transmute(ldensity), ::core::mem::transmute(ptableid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex2A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, ptablecreate: *mut JET_TABLECREATE2_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableColumnIndex2A(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE2_A) -> i32;
        }
        ::core::mem::transmute(JetCreateTableColumnIndex2A(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(ptablecreate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex2W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, ptablecreate: *mut JET_TABLECREATE2_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableColumnIndex2W(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE2_W) -> i32;
        }
        ::core::mem::transmute(JetCreateTableColumnIndex2W(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(ptablecreate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex3A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, ptablecreate: *mut JET_TABLECREATE3_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableColumnIndex3A(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE3_A) -> i32;
        }
        ::core::mem::transmute(JetCreateTableColumnIndex3A(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(ptablecreate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex3W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, ptablecreate: *mut JET_TABLECREATE3_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableColumnIndex3W(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE3_W) -> i32;
        }
        ::core::mem::transmute(JetCreateTableColumnIndex3W(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(ptablecreate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex4A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, ptablecreate: *mut JET_TABLECREATE4_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableColumnIndex4A(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE4_A) -> i32;
        }
        ::core::mem::transmute(JetCreateTableColumnIndex4A(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(ptablecreate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndex4W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, ptablecreate: *mut JET_TABLECREATE4_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableColumnIndex4W(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE4_W) -> i32;
        }
        ::core::mem::transmute(JetCreateTableColumnIndex4W(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(ptablecreate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndexA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, ptablecreate: *mut JET_TABLECREATE_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableColumnIndexA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE_A) -> i32;
        }
        ::core::mem::transmute(JetCreateTableColumnIndexA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(ptablecreate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableColumnIndexW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, ptablecreate: *mut JET_TABLECREATE_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableColumnIndexW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, ptablecreate: *mut JET_TABLECREATE_W) -> i32;
        }
        ::core::mem::transmute(JetCreateTableColumnIndexW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(ptablecreate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetCreateTableW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const u16, lpages: u32, ldensity: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetCreateTableW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, lpages: u32, ldensity: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32;
        }
        ::core::mem::transmute(JetCreateTableW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(lpages), ::core::mem::transmute(ldensity), ::core::mem::transmute(ptableid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragment2A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const i8, pcpasses: *mut u32, pcseconds: *mut u32, callback: JET_CALLBACK, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDefragment2A(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, pcpasses: *mut u32, pcseconds: *mut u32, callback: ::windows::core::RawPtr, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDefragment2A(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(pcpasses), ::core::mem::transmute(pcseconds), ::core::mem::transmute(callback), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragment2W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const u16, pcpasses: *mut u32, pcseconds: *mut u32, callback: JET_CALLBACK, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDefragment2W(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, pcpasses: *mut u32, pcseconds: *mut u32, callback: ::windows::core::RawPtr, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDefragment2W(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(pcpasses), ::core::mem::transmute(pcseconds), ::core::mem::transmute(callback), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragment3A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szdatabasename: *const i8, sztablename: *const i8, pcpasses: *mut u32, pcseconds: *mut u32, callback: JET_CALLBACK, pvcontext: *const ::core::ffi::c_void, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDefragment3A(sesid: super::StructuredStorage::JET_SESID, szdatabasename: *const i8, sztablename: *const i8, pcpasses: *mut u32, pcseconds: *mut u32, callback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDefragment3A(sesid.into_param().abi(), ::core::mem::transmute(szdatabasename), ::core::mem::transmute(sztablename), ::core::mem::transmute(pcpasses), ::core::mem::transmute(pcseconds), ::core::mem::transmute(callback), ::core::mem::transmute(pvcontext), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragment3W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szdatabasename: *const u16, sztablename: *const u16, pcpasses: *mut u32, pcseconds: *mut u32, callback: JET_CALLBACK, pvcontext: *const ::core::ffi::c_void, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDefragment3W(sesid: super::StructuredStorage::JET_SESID, szdatabasename: *const u16, sztablename: *const u16, pcpasses: *mut u32, pcseconds: *mut u32, callback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDefragment3W(sesid.into_param().abi(), ::core::mem::transmute(szdatabasename), ::core::mem::transmute(sztablename), ::core::mem::transmute(pcpasses), ::core::mem::transmute(pcseconds), ::core::mem::transmute(callback), ::core::mem::transmute(pvcontext), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragmentA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const i8, pcpasses: *mut u32, pcseconds: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDefragmentA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, pcpasses: *mut u32, pcseconds: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDefragmentA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(pcpasses), ::core::mem::transmute(pcseconds), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDefragmentW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const u16, pcpasses: *mut u32, pcseconds: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDefragmentW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, pcpasses: *mut u32, pcseconds: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDefragmentW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(pcpasses), ::core::mem::transmute(pcseconds), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDelete<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDelete(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID) -> i32;
        }
        ::core::mem::transmute(JetDelete(sesid.into_param().abi(), tableid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteColumn2A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szcolumnname: *const i8, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDeleteColumn2A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const i8, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDeleteColumn2A(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szcolumnname), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteColumn2W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szcolumnname: *const u16, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDeleteColumn2W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const u16, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDeleteColumn2W(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szcolumnname), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteColumnA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szcolumnname: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDeleteColumnA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const i8) -> i32;
        }
        ::core::mem::transmute(JetDeleteColumnA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szcolumnname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteColumnW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szcolumnname: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDeleteColumnW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const u16) -> i32;
        }
        ::core::mem::transmute(JetDeleteColumnW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szcolumnname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteIndexA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDeleteIndexA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8) -> i32;
        }
        ::core::mem::transmute(JetDeleteIndexA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteIndexW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDeleteIndexW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16) -> i32;
        }
        ::core::mem::transmute(JetDeleteIndexW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteTableA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDeleteTableA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8) -> i32;
        }
        ::core::mem::transmute(JetDeleteTableA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDeleteTableW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDeleteTableW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16) -> i32;
        }
        ::core::mem::transmute(JetDeleteTableW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDetachDatabase2A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const i8, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDetachDatabase2A(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDetachDatabase2A(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDetachDatabase2W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const u16, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDetachDatabase2W(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDetachDatabase2W(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDetachDatabaseA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDetachDatabaseA(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8) -> i32;
        }
        ::core::mem::transmute(JetDetachDatabaseA(sesid.into_param().abi(), ::core::mem::transmute(szfilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDetachDatabaseW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDetachDatabaseW(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16) -> i32;
        }
        ::core::mem::transmute(JetDetachDatabaseW(sesid.into_param().abi(), ::core::mem::transmute(szfilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDupCursor<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, ptableid: *mut super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDupCursor(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, ptableid: *mut super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetDupCursor(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(ptableid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetDupSession<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, psesid: *mut super::StructuredStorage::JET_SESID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetDupSession(sesid: super::StructuredStorage::JET_SESID, psesid: *mut super::StructuredStorage::JET_SESID) -> i32;
        }
        ::core::mem::transmute(JetDupSession(sesid.into_param().abi(), ::core::mem::transmute(psesid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEnableMultiInstanceA(psetsysparam: &[JET_SETSYSPARAM_A], pcsetsucceed: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetEnableMultiInstanceA(psetsysparam: *const JET_SETSYSPARAM_A, csetsysparam: u32, pcsetsucceed: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetEnableMultiInstanceA(::core::mem::transmute(::windows::core::as_ptr_or_null(psetsysparam)), psetsysparam.len() as _, ::core::mem::transmute(pcsetsucceed)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEnableMultiInstanceW(psetsysparam: &[JET_SETSYSPARAM_W], pcsetsucceed: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetEnableMultiInstanceW(psetsysparam: *const JET_SETSYSPARAM_W, csetsysparam: u32, pcsetsucceed: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetEnableMultiInstanceW(::core::mem::transmute(::windows::core::as_ptr_or_null(psetsysparam)), psetsysparam.len() as _, ::core::mem::transmute(pcsetsucceed)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetEndExternalBackup() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetEndExternalBackup() -> i32;
        }
        ::core::mem::transmute(JetEndExternalBackup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEndExternalBackupInstance<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetEndExternalBackupInstance(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
        }
        ::core::mem::transmute(JetEndExternalBackupInstance(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEndExternalBackupInstance2<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetEndExternalBackupInstance2(instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetEndExternalBackupInstance2(instance.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEndSession<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetEndSession(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetEndSession(sesid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEnumerateColumns<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, rgenumcolumnid: &[JET_ENUMCOLUMNID], pcenumcolumn: *mut u32, prgenumcolumn: *mut *mut JET_ENUMCOLUMN, pfnrealloc: JET_PFNREALLOC, pvrealloccontext: *const ::core::ffi::c_void, cbdatamost: u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetEnumerateColumns(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, cenumcolumnid: u32, rgenumcolumnid: *const JET_ENUMCOLUMNID, pcenumcolumn: *mut u32, prgenumcolumn: *mut *mut JET_ENUMCOLUMN, pfnrealloc: ::windows::core::RawPtr, pvrealloccontext: *const ::core::ffi::c_void, cbdatamost: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetEnumerateColumns(sesid.into_param().abi(), tableid.into_param().abi(), rgenumcolumnid.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgenumcolumnid)), ::core::mem::transmute(pcenumcolumn), ::core::mem::transmute(prgenumcolumn), ::core::mem::transmute(pfnrealloc), ::core::mem::transmute(pvrealloccontext), ::core::mem::transmute(cbdatamost), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetEscrowUpdate<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, columnid: u32, pv: *const ::core::ffi::c_void, cbmax: u32, pvold: *mut ::core::ffi::c_void, cboldmax: u32, pcboldactual: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetEscrowUpdate(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, columnid: u32, pv: *const ::core::ffi::c_void, cbmax: u32, pvold: *mut ::core::ffi::c_void, cboldmax: u32, pcboldactual: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetEscrowUpdate(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(columnid), ::core::mem::transmute(pv), ::core::mem::transmute(cbmax), ::core::mem::transmute(pvold), ::core::mem::transmute(cboldmax), ::core::mem::transmute(pcboldactual), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation', 'Win32_Storage_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
#[inline]
pub unsafe fn JetExternalRestore2A(szcheckpointfilepath: *const i8, szlogpath: *const i8, rgrstmap: &[JET_RSTMAP_A], szbackuplogpath: *const i8, ploginfo: *mut JET_LOGINFO_A, sztargetinstancename: *const i8, sztargetinstancelogpath: *const i8, sztargetinstancecheckpointpath: *const i8, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetExternalRestore2A(szcheckpointfilepath: *const i8, szlogpath: *const i8, rgrstmap: *const JET_RSTMAP_A, crstfilemap: i32, szbackuplogpath: *const i8, ploginfo: *mut JET_LOGINFO_A, sztargetinstancename: *const i8, sztargetinstancelogpath: *const i8, sztargetinstancecheckpointpath: *const i8, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetExternalRestore2A(::core::mem::transmute(szcheckpointfilepath), ::core::mem::transmute(szlogpath), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgrstmap)), rgrstmap.len() as _, ::core::mem::transmute(szbackuplogpath), ::core::mem::transmute(ploginfo), ::core::mem::transmute(sztargetinstancename), ::core::mem::transmute(sztargetinstancelogpath), ::core::mem::transmute(sztargetinstancecheckpointpath), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetExternalRestore2W(szcheckpointfilepath: *const u16, szlogpath: *const u16, rgrstmap: &[JET_RSTMAP_W], szbackuplogpath: *const u16, ploginfo: *mut JET_LOGINFO_W, sztargetinstancename: *const u16, sztargetinstancelogpath: *const u16, sztargetinstancecheckpointpath: *const u16, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetExternalRestore2W(szcheckpointfilepath: *const u16, szlogpath: *const u16, rgrstmap: *const JET_RSTMAP_W, crstfilemap: i32, szbackuplogpath: *const u16, ploginfo: *mut JET_LOGINFO_W, sztargetinstancename: *const u16, sztargetinstancelogpath: *const u16, sztargetinstancecheckpointpath: *const u16, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetExternalRestore2W(::core::mem::transmute(szcheckpointfilepath), ::core::mem::transmute(szlogpath), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgrstmap)), rgrstmap.len() as _, ::core::mem::transmute(szbackuplogpath), ::core::mem::transmute(ploginfo), ::core::mem::transmute(sztargetinstancename), ::core::mem::transmute(sztargetinstancelogpath), ::core::mem::transmute(sztargetinstancecheckpointpath), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetExternalRestoreA(szcheckpointfilepath: *const i8, szlogpath: *const i8, rgrstmap: &[JET_RSTMAP_A], szbackuplogpath: *const i8, genlow: i32, genhigh: i32, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetExternalRestoreA(szcheckpointfilepath: *const i8, szlogpath: *const i8, rgrstmap: *const JET_RSTMAP_A, crstfilemap: i32, szbackuplogpath: *const i8, genlow: i32, genhigh: i32, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetExternalRestoreA(::core::mem::transmute(szcheckpointfilepath), ::core::mem::transmute(szlogpath), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgrstmap)), rgrstmap.len() as _, ::core::mem::transmute(szbackuplogpath), ::core::mem::transmute(genlow), ::core::mem::transmute(genhigh), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetExternalRestoreW(szcheckpointfilepath: *const u16, szlogpath: *const u16, rgrstmap: &[JET_RSTMAP_W], szbackuplogpath: *const u16, genlow: i32, genhigh: i32, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetExternalRestoreW(szcheckpointfilepath: *const u16, szlogpath: *const u16, rgrstmap: *const JET_RSTMAP_W, crstfilemap: i32, szbackuplogpath: *const u16, genlow: i32, genhigh: i32, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetExternalRestoreW(::core::mem::transmute(szcheckpointfilepath), ::core::mem::transmute(szlogpath), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgrstmap)), rgrstmap.len() as _, ::core::mem::transmute(szbackuplogpath), ::core::mem::transmute(genlow), ::core::mem::transmute(genhigh), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetFreeBuffer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pbbuf: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetFreeBuffer(pbbuf: ::windows::core::PCSTR) -> i32;
        }
        ::core::mem::transmute(JetFreeBuffer(pbbuf.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetGetAttachInfoA(szzdatabases: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetAttachInfoA(szzdatabases: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetAttachInfoA(::core::mem::transmute(szzdatabases), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetAttachInfoInstanceA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, szzdatabases: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetAttachInfoInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szzdatabases: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetAttachInfoInstanceA(instance.into_param().abi(), ::core::mem::transmute(szzdatabases), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetAttachInfoInstanceW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, szzdatabases: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetAttachInfoInstanceW(instance: super::StructuredStorage::JET_INSTANCE, szzdatabases: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetAttachInfoInstanceW(instance.into_param().abi(), ::core::mem::transmute(szzdatabases), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetGetAttachInfoW(wszzdatabases: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetAttachInfoW(wszzdatabases: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetAttachInfoW(::core::mem::transmute(wszzdatabases), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetBookmark<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvbookmark: *mut ::core::ffi::c_void, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetBookmark(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvbookmark: *mut ::core::ffi::c_void, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetBookmark(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvbookmark), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetColumnInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const i8, pcolumnnameorid: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetColumnInfoA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, pcolumnnameorid: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetColumnInfoA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(pcolumnnameorid), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetColumnInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const u16, pwcolumnnameorid: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetColumnInfoW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, pwcolumnnameorid: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetColumnInfoW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(pwcolumnnameorid), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetCurrentIndexA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *mut i8, cbindexname: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetCurrentIndexA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *mut i8, cbindexname: u32) -> i32;
        }
        ::core::mem::transmute(JetGetCurrentIndexA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(cbindexname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetCurrentIndexW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *mut u16, cbindexname: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetCurrentIndexW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *mut u16, cbindexname: u32) -> i32;
        }
        ::core::mem::transmute(JetGetCurrentIndexW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(cbindexname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetCursorInfo<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetCursorInfo(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetCursorInfo(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetGetDatabaseFileInfoA(szdatabasename: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetDatabaseFileInfoA(szdatabasename: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetDatabaseFileInfoA(::core::mem::transmute(szdatabasename), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetGetDatabaseFileInfoW(szdatabasename: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetDatabaseFileInfoW(szdatabasename: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetDatabaseFileInfoW(::core::mem::transmute(szdatabasename), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetDatabaseInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetDatabaseInfoA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetDatabaseInfoA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetDatabaseInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetDatabaseInfoW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetDatabaseInfoW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetGetErrorInfoW(pvcontext: *const ::core::ffi::c_void, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetErrorInfoW(pvcontext: *const ::core::ffi::c_void, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetGetErrorInfoW(::core::mem::transmute(pvcontext), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetIndexInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const i8, szindexname: *const i8, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetIndexInfoA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, szindexname: *const i8, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetIndexInfoA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(szindexname), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbresult), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetIndexInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const u16, szindexname: *const u16, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetIndexInfoW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, szindexname: *const u16, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetIndexInfoW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(szindexname), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbresult), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetInstanceInfoA(pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetInstanceInfoA(pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A) -> i32;
        }
        ::core::mem::transmute(JetGetInstanceInfoA(::core::mem::transmute(pcinstanceinfo), ::core::mem::transmute(painstanceinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetInstanceInfoW(pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetInstanceInfoW(pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W) -> i32;
        }
        ::core::mem::transmute(JetGetInstanceInfoW(::core::mem::transmute(pcinstanceinfo), ::core::mem::transmute(painstanceinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetInstanceMiscInfo<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetInstanceMiscInfo(instance: super::StructuredStorage::JET_INSTANCE, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetInstanceMiscInfo(instance.into_param().abi(), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetLS<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pls: *mut JET_LS, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetLS(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pls: *mut JET_LS, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetGetLS(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pls), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetLock<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetLock(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetGetLock(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetGetLogInfoA(szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetLogInfoA(szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetLogInfoA(::core::mem::transmute(szzlogs), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation', 'Win32_Storage_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
#[inline]
pub unsafe fn JetGetLogInfoInstance2A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32, ploginfo: *mut JET_LOGINFO_A) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetLogInfoInstance2A(instance: super::StructuredStorage::JET_INSTANCE, szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32, ploginfo: *mut JET_LOGINFO_A) -> i32;
        }
        ::core::mem::transmute(JetGetLogInfoInstance2A(instance.into_param().abi(), ::core::mem::transmute(szzlogs), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual), ::core::mem::transmute(ploginfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetLogInfoInstance2W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, wszzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32, ploginfo: *mut JET_LOGINFO_W) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetLogInfoInstance2W(instance: super::StructuredStorage::JET_INSTANCE, wszzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32, ploginfo: *mut JET_LOGINFO_W) -> i32;
        }
        ::core::mem::transmute(JetGetLogInfoInstance2W(instance.into_param().abi(), ::core::mem::transmute(wszzlogs), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual), ::core::mem::transmute(ploginfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetLogInfoInstanceA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetLogInfoInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetLogInfoInstanceA(instance.into_param().abi(), ::core::mem::transmute(szzlogs), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetLogInfoInstanceW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, wszzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetLogInfoInstanceW(instance: super::StructuredStorage::JET_INSTANCE, wszzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetLogInfoInstanceW(instance.into_param().abi(), ::core::mem::transmute(wszzlogs), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetGetLogInfoW(szzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetLogInfoW(szzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetLogInfoW(::core::mem::transmute(szzlogs), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetObjectInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, objtyp: u32, szcontainername: *const i8, szobjectname: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetObjectInfoA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, objtyp: u32, szcontainername: *const i8, szobjectname: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetObjectInfoA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(objtyp), ::core::mem::transmute(szcontainername), ::core::mem::transmute(szobjectname), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetObjectInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, objtyp: u32, szcontainername: *const u16, szobjectname: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetObjectInfoW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, objtyp: u32, szcontainername: *const u16, szobjectname: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetObjectInfoW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(objtyp), ::core::mem::transmute(szcontainername), ::core::mem::transmute(szobjectname), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetRecordPosition<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, precpos: *mut JET_RECPOS, cbrecpos: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetRecordPosition(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, precpos: *mut JET_RECPOS, cbrecpos: u32) -> i32;
        }
        ::core::mem::transmute(JetGetRecordPosition(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(precpos), ::core::mem::transmute(cbrecpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetRecordSize<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, precsize: *mut JET_RECSIZE, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetRecordSize(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, precsize: *mut JET_RECSIZE, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetGetRecordSize(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(precsize), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetRecordSize2<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, precsize: *mut JET_RECSIZE2, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetRecordSize2(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, precsize: *mut JET_RECSIZE2, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetGetRecordSize2(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(precsize), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetSecondaryIndexBookmark<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvsecondarykey: *mut ::core::ffi::c_void, cbsecondarykeymax: u32, pcbsecondarykeyactual: *mut u32, pvprimarybookmark: *mut ::core::ffi::c_void, cbprimarybookmarkmax: u32, pcbprimarybookmarkactual: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetSecondaryIndexBookmark(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvsecondarykey: *mut ::core::ffi::c_void, cbsecondarykeymax: u32, pcbsecondarykeyactual: *mut u32, pvprimarybookmark: *mut ::core::ffi::c_void, cbprimarybookmarkmax: u32, pcbprimarybookmarkactual: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetGetSecondaryIndexBookmark(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvsecondarykey), ::core::mem::transmute(cbsecondarykeymax), ::core::mem::transmute(pcbsecondarykeyactual), ::core::mem::transmute(pvprimarybookmark), ::core::mem::transmute(cbprimarybookmarkmax), ::core::mem::transmute(pcbprimarybookmarkactual), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetSessionParameter<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, sesparamid: u32, pvparam: &mut [u8], pcbparamactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetSessionParameter(sesid: super::StructuredStorage::JET_SESID, sesparamid: u32, pvparam: *mut ::core::ffi::c_void, cbparammax: u32, pcbparamactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetSessionParameter(sesid.into_param().abi(), ::core::mem::transmute(sesparamid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pvparam)), pvparam.len() as _, ::core::mem::transmute(pcbparamactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetSystemParameterA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(instance: Param0, sesid: Param1, paramid: u32, plparam: *mut super::StructuredStorage::JET_API_PTR, szparam: *mut i8, cbmax: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetSystemParameterA(instance: super::StructuredStorage::JET_INSTANCE, sesid: super::StructuredStorage::JET_SESID, paramid: u32, plparam: *mut super::StructuredStorage::JET_API_PTR, szparam: *mut i8, cbmax: u32) -> i32;
        }
        ::core::mem::transmute(JetGetSystemParameterA(instance.into_param().abi(), sesid.into_param().abi(), ::core::mem::transmute(paramid), ::core::mem::transmute(plparam), ::core::mem::transmute(szparam), ::core::mem::transmute(cbmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetSystemParameterW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(instance: Param0, sesid: Param1, paramid: u32, plparam: *mut super::StructuredStorage::JET_API_PTR, szparam: *mut u16, cbmax: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetSystemParameterW(instance: super::StructuredStorage::JET_INSTANCE, sesid: super::StructuredStorage::JET_SESID, paramid: u32, plparam: *mut super::StructuredStorage::JET_API_PTR, szparam: *mut u16, cbmax: u32) -> i32;
        }
        ::core::mem::transmute(JetGetSystemParameterW(instance.into_param().abi(), sesid.into_param().abi(), ::core::mem::transmute(paramid), ::core::mem::transmute(plparam), ::core::mem::transmute(szparam), ::core::mem::transmute(cbmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableColumnInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szcolumnname: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetTableColumnInfoA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const i8, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetTableColumnInfoA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szcolumnname), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableColumnInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szcolumnname: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetTableColumnInfoW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szcolumnname: *const u16, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetTableColumnInfoW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szcolumnname), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableIndexInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const i8, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetTableIndexInfoA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetTableIndexInfoA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbresult), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableIndexInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const u16, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetTableIndexInfoW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, pvresult: *mut ::core::ffi::c_void, cbresult: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetTableIndexInfoW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbresult), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetTableInfoA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetTableInfoA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTableInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetTableInfoW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvresult: *mut ::core::ffi::c_void, cbmax: u32, infolevel: u32) -> i32;
        }
        ::core::mem::transmute(JetGetTableInfoW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax), ::core::mem::transmute(infolevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetGetThreadStats(pvresult: *mut ::core::ffi::c_void, cbmax: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetThreadStats(pvresult: *mut ::core::ffi::c_void, cbmax: u32) -> i32;
        }
        ::core::mem::transmute(JetGetThreadStats(::core::mem::transmute(pvresult), ::core::mem::transmute(cbmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTruncateLogInfoInstanceA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetTruncateLogInfoInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szzlogs: *mut i8, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetTruncateLogInfoInstanceA(instance.into_param().abi(), ::core::mem::transmute(szzlogs), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetTruncateLogInfoInstanceW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, wszzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetTruncateLogInfoInstanceW(instance: super::StructuredStorage::JET_INSTANCE, wszzlogs: *mut u16, cbmax: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetTruncateLogInfoInstanceW(instance.into_param().abi(), ::core::mem::transmute(wszzlogs), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGetVersion<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, pwversion: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGetVersion(sesid: super::StructuredStorage::JET_SESID, pwversion: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetGetVersion(sesid.into_param().abi(), ::core::mem::transmute(pwversion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGotoBookmark<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvbookmark: *const ::core::ffi::c_void, cbbookmark: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGotoBookmark(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvbookmark: *const ::core::ffi::c_void, cbbookmark: u32) -> i32;
        }
        ::core::mem::transmute(JetGotoBookmark(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvbookmark), ::core::mem::transmute(cbbookmark)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGotoPosition<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, precpos: *const JET_RECPOS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGotoPosition(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, precpos: *const JET_RECPOS) -> i32;
        }
        ::core::mem::transmute(JetGotoPosition(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(precpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGotoSecondaryIndexBookmark<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvsecondarykey: *const ::core::ffi::c_void, cbsecondarykey: u32, pvprimarybookmark: *const ::core::ffi::c_void, cbprimarybookmark: u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGotoSecondaryIndexBookmark(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvsecondarykey: *const ::core::ffi::c_void, cbsecondarykey: u32, pvprimarybookmark: *const ::core::ffi::c_void, cbprimarybookmark: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetGotoSecondaryIndexBookmark(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvsecondarykey), ::core::mem::transmute(cbsecondarykey), ::core::mem::transmute(pvprimarybookmark), ::core::mem::transmute(cbprimarybookmark), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetGrowDatabase<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, cpg: u32, pcpgreal: *const u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetGrowDatabase(sesid: super::StructuredStorage::JET_SESID, dbid: u32, cpg: u32, pcpgreal: *const u32) -> i32;
        }
        ::core::mem::transmute(JetGrowDatabase(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(cpg), ::core::mem::transmute(pcpgreal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetIdle<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetIdle(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetIdle(sesid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetIndexRecordCount<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pcrec: *mut u32, crecmax: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetIndexRecordCount(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pcrec: *mut u32, crecmax: u32) -> i32;
        }
        ::core::mem::transmute(JetIndexRecordCount(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pcrec), ::core::mem::transmute(crecmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetInit(pinstance: *mut super::StructuredStorage::JET_INSTANCE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetInit(pinstance: *mut super::StructuredStorage::JET_INSTANCE) -> i32;
        }
        ::core::mem::transmute(JetInit(::core::mem::transmute(pinstance)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetInit2(pinstance: *mut super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetInit2(pinstance: *mut super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetInit2(::core::mem::transmute(pinstance), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation', 'Win32_Storage_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
#[inline]
pub unsafe fn JetInit3A(pinstance: *mut super::StructuredStorage::JET_INSTANCE, prstinfo: *const JET_RSTINFO_A, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetInit3A(pinstance: *mut super::StructuredStorage::JET_INSTANCE, prstinfo: *const JET_RSTINFO_A, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetInit3A(::core::mem::transmute(pinstance), ::core::mem::transmute(prstinfo), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Foundation', 'Win32_Storage_StructuredStorage'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
#[inline]
pub unsafe fn JetInit3W(pinstance: *mut super::StructuredStorage::JET_INSTANCE, prstinfo: *const JET_RSTINFO_W, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetInit3W(pinstance: *mut super::StructuredStorage::JET_INSTANCE, prstinfo: *const JET_RSTINFO_W, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetInit3W(::core::mem::transmute(pinstance), ::core::mem::transmute(prstinfo), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetIntersectIndexes<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, rgindexrange: &[JET_INDEXRANGE], precordlist: *mut JET_RECORDLIST, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetIntersectIndexes(sesid: super::StructuredStorage::JET_SESID, rgindexrange: *const JET_INDEXRANGE, cindexrange: u32, precordlist: *mut JET_RECORDLIST, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetIntersectIndexes(sesid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgindexrange)), rgindexrange.len() as _, ::core::mem::transmute(precordlist), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetMakeKey<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetMakeKey(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetMakeKey(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetMove<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, crow: i32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetMove(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, crow: i32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetMove(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(crow), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetOSSnapshotAbort<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>>(snapid: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotAbort(snapid: JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotAbort(snapid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetOSSnapshotEnd<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>>(snapid: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotEnd(snapid: JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotEnd(snapid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotFreezeA<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>>(snapid: Param0, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotFreezeA(snapid: JET_OSSNAPID, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotFreezeA(snapid.into_param().abi(), ::core::mem::transmute(pcinstanceinfo), ::core::mem::transmute(painstanceinfo), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotFreezeW<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>>(snapid: Param0, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotFreezeW(snapid: JET_OSSNAPID, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotFreezeW(snapid.into_param().abi(), ::core::mem::transmute(pcinstanceinfo), ::core::mem::transmute(painstanceinfo), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotGetFreezeInfoA<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>>(snapid: Param0, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotGetFreezeInfoA(snapid: JET_OSSNAPID, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_A, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotGetFreezeInfoA(snapid.into_param().abi(), ::core::mem::transmute(pcinstanceinfo), ::core::mem::transmute(painstanceinfo), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotGetFreezeInfoW<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>>(snapid: Param0, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotGetFreezeInfoW(snapid: JET_OSSNAPID, pcinstanceinfo: *mut u32, painstanceinfo: *mut *mut JET_INSTANCE_INFO_W, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotGetFreezeInfoW(snapid.into_param().abi(), ::core::mem::transmute(pcinstanceinfo), ::core::mem::transmute(painstanceinfo), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetOSSnapshotPrepare(psnapid: *mut JET_OSSNAPID, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotPrepare(psnapid: *mut JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotPrepare(::core::mem::transmute(psnapid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotPrepareInstance<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(snapid: Param0, instance: Param1, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotPrepareInstance(snapid: JET_OSSNAPID, instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotPrepareInstance(snapid.into_param().abi(), instance.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetOSSnapshotThaw<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>>(snapid: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotThaw(snapid: JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotThaw(snapid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetOSSnapshotTruncateLog<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>>(snapid: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotTruncateLog(snapid: JET_OSSNAPID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotTruncateLog(snapid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOSSnapshotTruncateLogInstance<'a, Param0: ::windows::core::IntoParam<'a, JET_OSSNAPID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(snapid: Param0, instance: Param1, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOSSnapshotTruncateLogInstance(snapid: JET_OSSNAPID, instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOSSnapshotTruncateLogInstance(snapid.into_param().abi(), instance.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenDatabaseA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const i8, szconnect: *const i8, pdbid: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenDatabaseA(sesid: super::StructuredStorage::JET_SESID, szfilename: *const i8, szconnect: *const i8, pdbid: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOpenDatabaseA(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(szconnect), ::core::mem::transmute(pdbid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenDatabaseW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szfilename: *const u16, szconnect: *const u16, pdbid: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenDatabaseW(sesid: super::StructuredStorage::JET_SESID, szfilename: *const u16, szconnect: *const u16, pdbid: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetOpenDatabaseW(sesid.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(szconnect), ::core::mem::transmute(pdbid), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenFileA(szfilename: *const i8, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenFileA(szfilename: *const i8, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetOpenFileA(::core::mem::transmute(szfilename), ::core::mem::transmute(phffile), ::core::mem::transmute(pulfilesizelow), ::core::mem::transmute(pulfilesizehigh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenFileInstanceA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, szfilename: *const i8, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenFileInstanceA(instance: super::StructuredStorage::JET_INSTANCE, szfilename: *const i8, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetOpenFileInstanceA(instance.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(phffile), ::core::mem::transmute(pulfilesizelow), ::core::mem::transmute(pulfilesizehigh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenFileInstanceW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, szfilename: *const u16, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenFileInstanceW(instance: super::StructuredStorage::JET_INSTANCE, szfilename: *const u16, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetOpenFileInstanceW(instance.into_param().abi(), ::core::mem::transmute(szfilename), ::core::mem::transmute(phffile), ::core::mem::transmute(pulfilesizelow), ::core::mem::transmute(pulfilesizehigh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenFileW(szfilename: *const u16, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenFileW(szfilename: *const u16, phffile: *mut super::StructuredStorage::JET_HANDLE, pulfilesizelow: *mut u32, pulfilesizehigh: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetOpenFileW(::core::mem::transmute(szfilename), ::core::mem::transmute(phffile), ::core::mem::transmute(pulfilesizelow), ::core::mem::transmute(pulfilesizehigh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTableA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const i8, pvparameters: *const ::core::ffi::c_void, cbparameters: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenTableA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, pvparameters: *const ::core::ffi::c_void, cbparameters: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32;
        }
        ::core::mem::transmute(JetOpenTableA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(pvparameters), ::core::mem::transmute(cbparameters), ::core::mem::transmute(grbit), ::core::mem::transmute(ptableid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTableW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const u16, pvparameters: *const ::core::ffi::c_void, cbparameters: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenTableW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, pvparameters: *const ::core::ffi::c_void, cbparameters: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID) -> i32;
        }
        ::core::mem::transmute(JetOpenTableW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(pvparameters), ::core::mem::transmute(cbparameters), ::core::mem::transmute(grbit), ::core::mem::transmute(ptableid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTempTable<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, prgcolumndef: &[JET_COLUMNDEF], grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: &mut [u32]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenTempTable(sesid: super::StructuredStorage::JET_SESID, prgcolumndef: *const JET_COLUMNDEF, ccolumn: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetOpenTempTable(sesid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(prgcolumndef)), prgcolumnid.len() as _, ::core::mem::transmute(grbit), ::core::mem::transmute(ptableid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(prgcolumnid))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTempTable2<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, prgcolumndef: &[JET_COLUMNDEF], lcid: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: &mut [u32]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenTempTable2(sesid: super::StructuredStorage::JET_SESID, prgcolumndef: *const JET_COLUMNDEF, ccolumn: u32, lcid: u32, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetOpenTempTable2(sesid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(prgcolumndef)), prgcolumnid.len() as _, ::core::mem::transmute(lcid), ::core::mem::transmute(grbit), ::core::mem::transmute(ptableid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(prgcolumnid))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTempTable3<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, prgcolumndef: &[JET_COLUMNDEF], pidxunicode: *const JET_UNICODEINDEX, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: &mut [u32]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenTempTable3(sesid: super::StructuredStorage::JET_SESID, prgcolumndef: *const JET_COLUMNDEF, ccolumn: u32, pidxunicode: *const JET_UNICODEINDEX, grbit: u32, ptableid: *mut super::StructuredStorage::JET_TABLEID, prgcolumnid: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetOpenTempTable3(sesid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(prgcolumndef)), prgcolumnid.len() as _, ::core::mem::transmute(pidxunicode), ::core::mem::transmute(grbit), ::core::mem::transmute(ptableid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(prgcolumnid))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTemporaryTable<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, popentemporarytable: *const JET_OPENTEMPORARYTABLE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenTemporaryTable(sesid: super::StructuredStorage::JET_SESID, popentemporarytable: *const JET_OPENTEMPORARYTABLE) -> i32;
        }
        ::core::mem::transmute(JetOpenTemporaryTable(sesid.into_param().abi(), ::core::mem::transmute(popentemporarytable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetOpenTemporaryTable2<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, popentemporarytable: *const JET_OPENTEMPORARYTABLE2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetOpenTemporaryTable2(sesid: super::StructuredStorage::JET_SESID, popentemporarytable: *const JET_OPENTEMPORARYTABLE2) -> i32;
        }
        ::core::mem::transmute(JetOpenTemporaryTable2(sesid.into_param().abi(), ::core::mem::transmute(popentemporarytable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetPrepareUpdate<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, prep: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetPrepareUpdate(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, prep: u32) -> i32;
        }
        ::core::mem::transmute(JetPrepareUpdate(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(prep)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetPrereadIndexRanges<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, rgindexranges: &[JET_INDEX_RANGE], pcrangespreread: *mut u32, rgcolumnidpreread: &[u32], grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetPrereadIndexRanges(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, rgindexranges: *const JET_INDEX_RANGE, cindexranges: u32, pcrangespreread: *mut u32, rgcolumnidpreread: *const u32, ccolumnidpreread: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetPrereadIndexRanges(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgindexranges)), rgindexranges.len() as _, ::core::mem::transmute(pcrangespreread), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcolumnidpreread)), rgcolumnidpreread.len() as _, ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetPrereadKeys<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, rgpvkeys: &[*const ::core::ffi::c_void], rgcbkeys: &[u32], pckeyspreread: *mut i32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetPrereadKeys(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, rgpvkeys: *const *const ::core::ffi::c_void, rgcbkeys: *const u32, ckeys: i32, pckeyspreread: *mut i32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetPrereadKeys(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgpvkeys)), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcbkeys)), rgcbkeys.len() as _, ::core::mem::transmute(pckeyspreread), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetReadFile<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_HANDLE>>(hffile: Param0, pv: *mut ::core::ffi::c_void, cb: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetReadFile(hffile: super::StructuredStorage::JET_HANDLE, pv: *mut ::core::ffi::c_void, cb: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetReadFile(hffile.into_param().abi(), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetReadFileInstance<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_HANDLE>>(instance: Param0, hffile: Param1, pv: *mut ::core::ffi::c_void, cb: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetReadFileInstance(instance: super::StructuredStorage::JET_INSTANCE, hffile: super::StructuredStorage::JET_HANDLE, pv: *mut ::core::ffi::c_void, cb: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetReadFileInstance(instance.into_param().abi(), hffile.into_param().abi(), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRegisterCallback<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, cbtyp: u32, pcallback: JET_CALLBACK, pvcontext: *const ::core::ffi::c_void, phcallbackid: *const super::StructuredStorage::JET_HANDLE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRegisterCallback(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, cbtyp: u32, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void, phcallbackid: *const super::StructuredStorage::JET_HANDLE) -> i32;
        }
        ::core::mem::transmute(JetRegisterCallback(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(cbtyp), ::core::mem::transmute(pcallback), ::core::mem::transmute(pvcontext), ::core::mem::transmute(phcallbackid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRenameColumnA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szname: *const i8, sznamenew: *const i8, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRenameColumnA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szname: *const i8, sznamenew: *const i8, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetRenameColumnA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szname), ::core::mem::transmute(sznamenew), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRenameColumnW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szname: *const u16, sznamenew: *const u16, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRenameColumnW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szname: *const u16, sznamenew: *const u16, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetRenameColumnW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szname), ::core::mem::transmute(sznamenew), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRenameTableA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, szname: *const i8, sznamenew: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRenameTableA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, szname: *const i8, sznamenew: *const i8) -> i32;
        }
        ::core::mem::transmute(JetRenameTableA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(szname), ::core::mem::transmute(sznamenew)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRenameTableW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, szname: *const u16, sznamenew: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRenameTableW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, szname: *const u16, sznamenew: *const u16) -> i32;
        }
        ::core::mem::transmute(JetRenameTableW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(szname), ::core::mem::transmute(sznamenew)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetResetSessionContext<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetResetSessionContext(sesid: super::StructuredStorage::JET_SESID) -> i32;
        }
        ::core::mem::transmute(JetResetSessionContext(sesid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetResetTableSequential<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetResetTableSequential(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetResetTableSequential(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetResizeDatabase<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, cpgtarget: u32, pcpgactual: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetResizeDatabase(sesid: super::StructuredStorage::JET_SESID, dbid: u32, cpgtarget: u32, pcpgactual: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetResizeDatabase(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(cpgtarget), ::core::mem::transmute(pcpgactual), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRestore2A(sz: *const i8, szdest: *const i8, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRestore2A(sz: *const i8, szdest: *const i8, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetRestore2A(::core::mem::transmute(sz), ::core::mem::transmute(szdest), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRestore2W(sz: *const u16, szdest: *const u16, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRestore2W(sz: *const u16, szdest: *const u16, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetRestore2W(::core::mem::transmute(sz), ::core::mem::transmute(szdest), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRestoreA(szsource: *const i8, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRestoreA(szsource: *const i8, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetRestoreA(::core::mem::transmute(szsource), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRestoreInstanceA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, sz: *const i8, szdest: *const i8, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRestoreInstanceA(instance: super::StructuredStorage::JET_INSTANCE, sz: *const i8, szdest: *const i8, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetRestoreInstanceA(instance.into_param().abi(), ::core::mem::transmute(sz), ::core::mem::transmute(szdest), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRestoreInstanceW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, sz: *const u16, szdest: *const u16, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRestoreInstanceW(instance: super::StructuredStorage::JET_INSTANCE, sz: *const u16, szdest: *const u16, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetRestoreInstanceW(instance.into_param().abi(), ::core::mem::transmute(sz), ::core::mem::transmute(szdest), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRestoreW(szsource: *const u16, pfn: JET_PFNSTATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRestoreW(szsource: *const u16, pfn: ::windows::core::RawPtr) -> i32;
        }
        ::core::mem::transmute(JetRestoreW(::core::mem::transmute(szsource), ::core::mem::transmute(pfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRetrieveColumn<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, columnid: u32, pvdata: *mut ::core::ffi::c_void, cbdata: u32, pcbactual: *mut u32, grbit: u32, pretinfo: *mut JET_RETINFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRetrieveColumn(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, columnid: u32, pvdata: *mut ::core::ffi::c_void, cbdata: u32, pcbactual: *mut u32, grbit: u32, pretinfo: *mut JET_RETINFO) -> i32;
        }
        ::core::mem::transmute(JetRetrieveColumn(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(columnid), ::core::mem::transmute(pvdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(pcbactual), ::core::mem::transmute(grbit), ::core::mem::transmute(pretinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRetrieveColumns<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pretrievecolumn: &mut [JET_RETRIEVECOLUMN]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRetrieveColumns(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pretrievecolumn: *mut JET_RETRIEVECOLUMN, cretrievecolumn: u32) -> i32;
        }
        ::core::mem::transmute(JetRetrieveColumns(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pretrievecolumn)), pretrievecolumn.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRetrieveKey<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvkey: *mut ::core::ffi::c_void, cbmax: u32, pcbactual: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRetrieveKey(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvkey: *mut ::core::ffi::c_void, cbmax: u32, pcbactual: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetRetrieveKey(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvkey), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbactual), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetRollback<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetRollback(sesid: super::StructuredStorage::JET_SESID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetRollback(sesid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSeek<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSeek(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetSeek(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetColumn<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, columnid: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32, psetinfo: *const JET_SETINFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetColumn(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, columnid: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32, psetinfo: *const JET_SETINFO) -> i32;
        }
        ::core::mem::transmute(JetSetColumn(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(columnid), ::core::mem::transmute(pvdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(grbit), ::core::mem::transmute(psetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetColumnDefaultValueA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const i8, szcolumnname: *const i8, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetColumnDefaultValueA(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const i8, szcolumnname: *const i8, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetSetColumnDefaultValueA(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(szcolumnname), ::core::mem::transmute(pvdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetColumnDefaultValueW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, dbid: u32, sztablename: *const u16, szcolumnname: *const u16, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetColumnDefaultValueW(sesid: super::StructuredStorage::JET_SESID, dbid: u32, sztablename: *const u16, szcolumnname: *const u16, pvdata: *const ::core::ffi::c_void, cbdata: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetSetColumnDefaultValueW(sesid.into_param().abi(), ::core::mem::transmute(dbid), ::core::mem::transmute(sztablename), ::core::mem::transmute(szcolumnname), ::core::mem::transmute(pvdata), ::core::mem::transmute(cbdata), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetColumns<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, psetcolumn: &[JET_SETCOLUMN]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetColumns(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, psetcolumn: *const JET_SETCOLUMN, csetcolumn: u32) -> i32;
        }
        ::core::mem::transmute(JetSetColumns(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(psetcolumn)), psetcolumn.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex2A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const i8, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetCurrentIndex2A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetSetCurrentIndex2A(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex2W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const u16, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetCurrentIndex2W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetSetCurrentIndex2W(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex3A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const i8, grbit: u32, itagsequence: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetCurrentIndex3A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, grbit: u32, itagsequence: u32) -> i32;
        }
        ::core::mem::transmute(JetSetCurrentIndex3A(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(grbit), ::core::mem::transmute(itagsequence)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex3W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const u16, grbit: u32, itagsequence: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetCurrentIndex3W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, grbit: u32, itagsequence: u32) -> i32;
        }
        ::core::mem::transmute(JetSetCurrentIndex3W(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(grbit), ::core::mem::transmute(itagsequence)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex4A<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const i8, pindexid: *const JET_INDEXID, grbit: u32, itagsequence: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetCurrentIndex4A(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8, pindexid: *const JET_INDEXID, grbit: u32, itagsequence: u32) -> i32;
        }
        ::core::mem::transmute(JetSetCurrentIndex4A(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(pindexid), ::core::mem::transmute(grbit), ::core::mem::transmute(itagsequence)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndex4W<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const u16, pindexid: *const JET_INDEXID, grbit: u32, itagsequence: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetCurrentIndex4W(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16, pindexid: *const JET_INDEXID, grbit: u32, itagsequence: u32) -> i32;
        }
        ::core::mem::transmute(JetSetCurrentIndex4W(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname), ::core::mem::transmute(pindexid), ::core::mem::transmute(grbit), ::core::mem::transmute(itagsequence)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndexA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetCurrentIndexA(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const i8) -> i32;
        }
        ::core::mem::transmute(JetSetCurrentIndexA(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCurrentIndexW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, szindexname: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetCurrentIndexW(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, szindexname: *const u16) -> i32;
        }
        ::core::mem::transmute(JetSetCurrentIndexW(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(szindexname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetCursorFilter<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, rgcolumnfilters: &[JET_INDEX_COLUMN], grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetCursorFilter(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, rgcolumnfilters: *const JET_INDEX_COLUMN, ccolumnfilters: u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetSetCursorFilter(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcolumnfilters)), rgcolumnfilters.len() as _, ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetDatabaseSizeA<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szdatabasename: *const i8, cpg: u32, pcpgreal: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetDatabaseSizeA(sesid: super::StructuredStorage::JET_SESID, szdatabasename: *const i8, cpg: u32, pcpgreal: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetSetDatabaseSizeA(sesid.into_param().abi(), ::core::mem::transmute(szdatabasename), ::core::mem::transmute(cpg), ::core::mem::transmute(pcpgreal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetDatabaseSizeW<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, szdatabasename: *const u16, cpg: u32, pcpgreal: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetDatabaseSizeW(sesid: super::StructuredStorage::JET_SESID, szdatabasename: *const u16, cpg: u32, pcpgreal: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetSetDatabaseSizeW(sesid.into_param().abi(), ::core::mem::transmute(szdatabasename), ::core::mem::transmute(cpg), ::core::mem::transmute(pcpgreal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetIndexRange<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableidsrc: Param1, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetIndexRange(sesid: super::StructuredStorage::JET_SESID, tableidsrc: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetSetIndexRange(sesid.into_param().abi(), tableidsrc.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetLS<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>, Param2: ::windows::core::IntoParam<'a, JET_LS>>(sesid: Param0, tableid: Param1, ls: Param2, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetLS(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, ls: JET_LS, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetSetLS(sesid.into_param().abi(), tableid.into_param().abi(), ls.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetSessionContext<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_API_PTR>>(sesid: Param0, ulcontext: Param1) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetSessionContext(sesid: super::StructuredStorage::JET_SESID, ulcontext: super::StructuredStorage::JET_API_PTR) -> i32;
        }
        ::core::mem::transmute(JetSetSessionContext(sesid.into_param().abi(), ulcontext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetSessionParameter<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>>(sesid: Param0, sesparamid: u32, pvparam: *const ::core::ffi::c_void, cbparam: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetSessionParameter(sesid: super::StructuredStorage::JET_SESID, sesparamid: u32, pvparam: *const ::core::ffi::c_void, cbparam: u32) -> i32;
        }
        ::core::mem::transmute(JetSetSessionParameter(sesid.into_param().abi(), ::core::mem::transmute(sesparamid), ::core::mem::transmute(pvparam), ::core::mem::transmute(cbparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetSystemParameterA<'a, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param3: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_API_PTR>>(pinstance: *mut super::StructuredStorage::JET_INSTANCE, sesid: Param1, paramid: u32, lparam: Param3, szparam: *const i8) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetSystemParameterA(pinstance: *mut super::StructuredStorage::JET_INSTANCE, sesid: super::StructuredStorage::JET_SESID, paramid: u32, lparam: super::StructuredStorage::JET_API_PTR, szparam: *const i8) -> i32;
        }
        ::core::mem::transmute(JetSetSystemParameterA(::core::mem::transmute(pinstance), sesid.into_param().abi(), ::core::mem::transmute(paramid), lparam.into_param().abi(), ::core::mem::transmute(szparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetSystemParameterW<'a, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param3: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_API_PTR>>(pinstance: *mut super::StructuredStorage::JET_INSTANCE, sesid: Param1, paramid: u32, lparam: Param3, szparam: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetSystemParameterW(pinstance: *mut super::StructuredStorage::JET_INSTANCE, sesid: super::StructuredStorage::JET_SESID, paramid: u32, lparam: super::StructuredStorage::JET_API_PTR, szparam: *const u16) -> i32;
        }
        ::core::mem::transmute(JetSetSystemParameterW(::core::mem::transmute(pinstance), sesid.into_param().abi(), ::core::mem::transmute(paramid), lparam.into_param().abi(), ::core::mem::transmute(szparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetSetTableSequential<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetSetTableSequential(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetSetTableSequential(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetStopBackup() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetStopBackup() -> i32;
        }
        ::core::mem::transmute(JetStopBackup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetStopBackupInstance<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetStopBackupInstance(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
        }
        ::core::mem::transmute(JetStopBackupInstance(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetStopService() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetStopService() -> i32;
        }
        ::core::mem::transmute(JetStopService())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetStopServiceInstance<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetStopServiceInstance(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
        }
        ::core::mem::transmute(JetStopServiceInstance(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetStopServiceInstance2<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetStopServiceInstance2(instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetStopServiceInstance2(instance.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetTerm<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetTerm(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
        }
        ::core::mem::transmute(JetTerm(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetTerm2<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetTerm2(instance: super::StructuredStorage::JET_INSTANCE, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetTerm2(instance.into_param().abi(), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
#[inline]
pub unsafe fn JetTruncateLog() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetTruncateLog() -> i32;
        }
        ::core::mem::transmute(JetTruncateLog())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetTruncateLogInstance<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_INSTANCE>>(instance: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetTruncateLogInstance(instance: super::StructuredStorage::JET_INSTANCE) -> i32;
        }
        ::core::mem::transmute(JetTruncateLogInstance(instance.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetUnregisterCallback<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>, Param3: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_HANDLE>>(sesid: Param0, tableid: Param1, cbtyp: u32, hcallbackid: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetUnregisterCallback(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, cbtyp: u32, hcallbackid: super::StructuredStorage::JET_HANDLE) -> i32;
        }
        ::core::mem::transmute(JetUnregisterCallback(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(cbtyp), hcallbackid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetUpdate<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvbookmark: *mut ::core::ffi::c_void, cbbookmark: u32, pcbactual: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetUpdate(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvbookmark: *mut ::core::ffi::c_void, cbbookmark: u32, pcbactual: *mut u32) -> i32;
        }
        ::core::mem::transmute(JetUpdate(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvbookmark), ::core::mem::transmute(cbbookmark), ::core::mem::transmute(pcbactual)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet', 'Win32_Storage_StructuredStorage'*"]
#[cfg(feature = "Win32_Storage_StructuredStorage")]
#[inline]
pub unsafe fn JetUpdate2<'a, Param0: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_SESID>, Param1: ::windows::core::IntoParam<'a, super::StructuredStorage::JET_TABLEID>>(sesid: Param0, tableid: Param1, pvbookmark: *mut ::core::ffi::c_void, cbbookmark: u32, pcbactual: *mut u32, grbit: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn JetUpdate2(sesid: super::StructuredStorage::JET_SESID, tableid: super::StructuredStorage::JET_TABLEID, pvbookmark: *mut ::core::ffi::c_void, cbbookmark: u32, pcbactual: *mut u32, grbit: u32) -> i32;
        }
        ::core::mem::transmute(JetUpdate2(sesid.into_param().abi(), tableid.into_param().abi(), ::core::mem::transmute(pvbookmark), ::core::mem::transmute(cbbookmark), ::core::mem::transmute(pcbactual), ::core::mem::transmute(grbit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const cColumnInfoCols: u32 = 14u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const cIndexInfoCols: u32 = 15u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const cObjectInfoCols: u32 = 9u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const wrnBTNotVisibleAccumulated: u32 = 353u32;
#[doc = "*Required features: 'Win32_Storage_Jet'*"]
pub const wrnBTNotVisibleRejected: u32 = 352u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
