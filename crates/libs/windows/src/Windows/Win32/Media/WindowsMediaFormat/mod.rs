#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct AM_WMT_EVENT_DATA {
    pub hrStatus: ::windows::core::HRESULT,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for AM_WMT_EVENT_DATA {}
impl ::core::clone::Clone for AM_WMT_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AM_WMT_EVENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AM_WMT_EVENT_DATA").field("hrStatus", &self.hrStatus).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for AM_WMT_EVENT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AM_WMT_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AM_WMT_EVENT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for AM_WMT_EVENT_DATA {}
impl ::core::default::Default for AM_WMT_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CLSID_ClientNetManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd12a3ce_9c42_11d2_beed_0060082f2054);
pub const CLSID_WMBandwidthSharing_Exclusive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf6060aa_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMBandwidthSharing_Partial: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf6060ab_5197_11d2_b6af_00c04fd908e9);
pub const CLSID_WMMUTEX_Bitrate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a01_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Language: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a00_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Presentation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a02_35da_11d1_9034_00a0c90349be);
pub const CLSID_WMMUTEX_Unknown: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e22a03_35da_11d1_9034_00a0c90349be);
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct DRM_COPY_OPL {
    pub wMinimumCopyLevel: u16,
    pub oplIdIncludes: DRM_OPL_OUTPUT_IDS,
    pub oplIdExcludes: DRM_OPL_OUTPUT_IDS,
}
impl ::core::marker::Copy for DRM_COPY_OPL {}
impl ::core::clone::Clone for DRM_COPY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_COPY_OPL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_COPY_OPL").field("wMinimumCopyLevel", &self.wMinimumCopyLevel).field("oplIdIncludes", &self.oplIdIncludes).field("oplIdExcludes", &self.oplIdExcludes).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_COPY_OPL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_COPY_OPL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_COPY_OPL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_COPY_OPL {}
impl ::core::default::Default for DRM_COPY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    pub wCompressedDigitalVideo: u16,
    pub wUncompressedDigitalVideo: u16,
    pub wAnalogVideo: u16,
    pub wCompressedDigitalAudio: u16,
    pub wUncompressedDigitalAudio: u16,
}
impl ::core::marker::Copy for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::clone::Clone for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS").field("wCompressedDigitalVideo", &self.wCompressedDigitalVideo).field("wUncompressedDigitalVideo", &self.wUncompressedDigitalVideo).field("wAnalogVideo", &self.wAnalogVideo).field("wCompressedDigitalAudio", &self.wCompressedDigitalAudio).field("wUncompressedDigitalAudio", &self.wUncompressedDigitalAudio).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::default::Default for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct DRM_OPL_OUTPUT_IDS {
    pub cIds: u16,
    pub rgIds: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for DRM_OPL_OUTPUT_IDS {}
impl ::core::clone::Clone for DRM_OPL_OUTPUT_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_OPL_OUTPUT_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_OPL_OUTPUT_IDS").field("cIds", &self.cIds).field("rgIds", &self.rgIds).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_OPL_OUTPUT_IDS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_OPL_OUTPUT_IDS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_OPL_OUTPUT_IDS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_OPL_OUTPUT_IDS {}
impl ::core::default::Default for DRM_OPL_OUTPUT_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const DRM_OPL_TYPES: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct DRM_OUTPUT_PROTECTION {
    pub guidId: ::windows::core::GUID,
    pub bConfigData: u8,
}
impl ::core::marker::Copy for DRM_OUTPUT_PROTECTION {}
impl ::core::clone::Clone for DRM_OUTPUT_PROTECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_OUTPUT_PROTECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_OUTPUT_PROTECTION").field("guidId", &self.guidId).field("bConfigData", &self.bConfigData).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_OUTPUT_PROTECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_OUTPUT_PROTECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_OUTPUT_PROTECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_OUTPUT_PROTECTION {}
impl ::core::default::Default for DRM_OUTPUT_PROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct DRM_PLAY_OPL {
    pub minOPL: DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS,
    pub oplIdReserved: DRM_OPL_OUTPUT_IDS,
    pub vopi: DRM_VIDEO_OUTPUT_PROTECTION_IDS,
}
impl ::core::marker::Copy for DRM_PLAY_OPL {}
impl ::core::clone::Clone for DRM_PLAY_OPL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_PLAY_OPL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_PLAY_OPL").field("minOPL", &self.minOPL).field("oplIdReserved", &self.oplIdReserved).field("vopi", &self.vopi).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_PLAY_OPL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_PLAY_OPL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_PLAY_OPL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_PLAY_OPL {}
impl ::core::default::Default for DRM_PLAY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct DRM_VAL16 {
    pub val: [u8; 16],
}
impl ::core::marker::Copy for DRM_VAL16 {}
impl ::core::clone::Clone for DRM_VAL16 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_VAL16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_VAL16").field("val", &self.val).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_VAL16 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_VAL16 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_VAL16>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_VAL16 {}
impl ::core::default::Default for DRM_VAL16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    pub cEntries: u16,
    pub rgVop: *mut DRM_OUTPUT_PROTECTION,
}
impl ::core::marker::Copy for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::clone::Clone for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_VIDEO_OUTPUT_PROTECTION_IDS").field("cEntries", &self.cEntries).field("rgVop", &self.rgVop).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_VIDEO_OUTPUT_PROTECTION_IDS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::default::Default for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IAMWMBufferPass(::windows::core::IUnknown);
impl IAMWMBufferPass {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetNotify<'a, Param0: ::windows::core::IntoParam<'a, IAMWMBufferPassCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNotify)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAMWMBufferPass> for ::windows::core::IUnknown {
    fn from(value: IAMWMBufferPass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAMWMBufferPass> for ::windows::core::IUnknown {
    fn from(value: &IAMWMBufferPass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAMWMBufferPass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAMWMBufferPass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAMWMBufferPass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAMWMBufferPass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAMWMBufferPass {}
impl ::core::fmt::Debug for IAMWMBufferPass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAMWMBufferPass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAMWMBufferPass {
    type Vtable = IAMWMBufferPass_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dd816d7_e740_4123_9e24_2444412644d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAMWMBufferPass_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IAMWMBufferPassCallback(::windows::core::IUnknown);
impl IAMWMBufferPassCallback {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Media_DirectShow'*"]
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn Notify<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer3>, Param1: ::windows::core::IntoParam<'a, super::DirectShow::IPin>>(&self, pnssbuffer3: Param0, ppin: Param1, prtstart: *const i64, prtend: *const i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Notify)(::core::mem::transmute_copy(self), pnssbuffer3.into_param().abi(), ppin.into_param().abi(), ::core::mem::transmute(prtstart), ::core::mem::transmute(prtend)).ok()
    }
}
impl ::core::convert::From<IAMWMBufferPassCallback> for ::windows::core::IUnknown {
    fn from(value: IAMWMBufferPassCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAMWMBufferPassCallback> for ::windows::core::IUnknown {
    fn from(value: &IAMWMBufferPassCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAMWMBufferPassCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAMWMBufferPassCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAMWMBufferPassCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAMWMBufferPassCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAMWMBufferPassCallback {}
impl ::core::fmt::Debug for IAMWMBufferPassCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAMWMBufferPassCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAMWMBufferPassCallback {
    type Vtable = IAMWMBufferPassCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb25b8372_d2d2_44b2_8653_1b8dae332489);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAMWMBufferPassCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnssbuffer3: ::windows::core::RawPtr, ppin: ::windows::core::RawPtr, prtstart: *const i64, prtend: *const i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    Notify: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct INSNetSourceCreator(::windows::core::IUnknown);
impl INSNetSourceCreator {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateNetSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pszstreamname: Param0, pmonitor: Param1, pdata: *const u8, pusercontext: Param3, pcallback: Param4, qwcontext: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateNetSource)(::core::mem::transmute_copy(self), pszstreamname.into_param().abi(), pmonitor.into_param().abi(), ::core::mem::transmute(pdata), pusercontext.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(qwcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNetSourceProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstreamname: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetSourceProperties)(::core::mem::transmute_copy(self), pszstreamname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNetSourceSharedNamespace(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetSourceSharedNamespace)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation', 'Win32_System_Com', 'Win32_System_Ole'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetNetSourceAdminInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszstreamname: Param0) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetSourceAdminInterface)(::core::mem::transmute_copy(self), pszstreamname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNumProtocolsSupported)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProtocolName)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprotocolnum), ::core::mem::transmute(pwszprotocolname), ::core::mem::transmute(pcchprotocolname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Shutdown)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<INSNetSourceCreator> for ::windows::core::IUnknown {
    fn from(value: INSNetSourceCreator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSNetSourceCreator> for ::windows::core::IUnknown {
    fn from(value: &INSNetSourceCreator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSNetSourceCreator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INSNetSourceCreator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSNetSourceCreator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSNetSourceCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSNetSourceCreator {}
impl ::core::fmt::Debug for INSNetSourceCreator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSNetSourceCreator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSNetSourceCreator {
    type Vtable = INSNetSourceCreator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c0e4080_9081_11d2_beec_0060082f2054);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSNetSourceCreator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: ::windows::core::PCWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows::core::HRESULT,
    pub GetNetSourceProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: ::windows::core::PCWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNetSourceSharedNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetNetSourceAdminInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstreamname: ::windows::core::PCWSTR, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetNetSourceAdminInterface: usize,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct INSSBuffer(::windows::core::IUnknown);
impl INSSBuffer {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferAndLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
}
impl ::core::convert::From<INSSBuffer> for ::windows::core::IUnknown {
    fn from(value: INSSBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer> for ::windows::core::IUnknown {
    fn from(value: &INSSBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSSBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INSSBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer {}
impl ::core::fmt::Debug for INSSBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSSBuffer {
    type Vtable = INSSBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1cd3524_03d7_11d2_9eed_006097d2d7cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetBufferAndLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct INSSBuffer2(::windows::core::IUnknown);
impl INSSBuffer2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaxLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetBufferAndLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSampleProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSampleProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(pbproperties)).ok()
    }
}
impl ::core::convert::From<INSSBuffer2> for ::windows::core::IUnknown {
    fn from(value: INSSBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer2> for ::windows::core::IUnknown {
    fn from(value: &INSSBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSSBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INSSBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer2> for INSSBuffer {
    fn from(value: INSSBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer2> for INSSBuffer {
    fn from(value: &INSSBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for INSSBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for &'a INSSBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer2 {}
impl ::core::fmt::Debug for INSSBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSSBuffer2 {
    type Vtable = INSSBuffer2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f528693_1035_43fe_b428_757561ad3a68);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer2_Vtbl {
    pub base: INSSBuffer_Vtbl,
    pub GetSampleProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT,
    pub SetSampleProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct INSSBuffer3(::windows::core::IUnknown);
impl INSSBuffer3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMaxLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetBufferAndLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSampleProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSampleProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(pbproperties)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::core::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(dwbufferpropertysize)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(pdwbufferpropertysize)).ok()
    }
}
impl ::core::convert::From<INSSBuffer3> for ::windows::core::IUnknown {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer3> for ::windows::core::IUnknown {
    fn from(value: &INSSBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer3> for INSSBuffer {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer3> for INSSBuffer {
    fn from(value: &INSSBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for &'a INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer3> for INSSBuffer2 {
    fn from(value: INSSBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer3> for INSSBuffer2 {
    fn from(value: &INSSBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer2> for INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer2> for &'a INSSBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer3 {}
impl ::core::fmt::Debug for INSSBuffer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSSBuffer3 {
    type Vtable = INSSBuffer3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc87ceaaf_75be_4bc4_84eb_ac2798507672);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer3_Vtbl {
    pub base: INSSBuffer2_Vtbl,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct INSSBuffer4(::windows::core::IUnknown);
impl INSSBuffer4 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetMaxLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetBufferAndLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdwbuffer), ::core::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetSampleProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetSampleProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbproperties), ::core::mem::transmute(pbproperties)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetProperty)(::core::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(dwbufferpropertysize)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidbufferproperty: Param0, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetProperty)(::core::mem::transmute_copy(self), guidbufferproperty.into_param().abi(), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(pdwbufferpropertysize)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPropertyCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPropertyCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyByIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbufferpropertyindex), ::core::mem::transmute(pguidbufferproperty), ::core::mem::transmute(pvbufferproperty), ::core::mem::transmute(pdwbufferpropertysize)).ok()
    }
}
impl ::core::convert::From<INSSBuffer4> for ::windows::core::IUnknown {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for ::windows::core::IUnknown {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer4> for INSSBuffer {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for INSSBuffer {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer> for &'a INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer4> for INSSBuffer2 {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for INSSBuffer2 {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer2> for INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer2> for &'a INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INSSBuffer4> for INSSBuffer3 {
    fn from(value: INSSBuffer4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INSSBuffer4> for INSSBuffer3 {
    fn from(value: &INSSBuffer4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer3> for INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INSSBuffer3> for &'a INSSBuffer4 {
    fn into_param(self) -> ::windows::core::Param<'a, INSSBuffer3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INSSBuffer4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INSSBuffer4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer4 {}
impl ::core::fmt::Debug for INSSBuffer4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INSSBuffer4 {
    type Vtable = INSSBuffer4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b8fd5a_32e2_49d4_a910_c26cc85465ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct INSSBuffer4_Vtbl {
    pub base: INSSBuffer3_Vtbl,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows::core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMAddressAccess(::windows::core::IUnknown);
impl IWMAddressAccess {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAccessEntryCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__: WM_ADDRESS_ACCESSENTRY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAccessEntry)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(&mut result__)).from_abi::<WM_ADDRESS_ACCESSENTRY>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddAccessEntry)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(paddraccessentry)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAccessEntry)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum)).ok()
    }
}
impl ::core::convert::From<IWMAddressAccess> for ::windows::core::IUnknown {
    fn from(value: IWMAddressAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAddressAccess> for ::windows::core::IUnknown {
    fn from(value: &IWMAddressAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMAddressAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMAddressAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMAddressAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMAddressAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess {}
impl ::core::fmt::Debug for IWMAddressAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAddressAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMAddressAccess {
    type Vtable = IWMAddressAccess_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb3c6389_1633_4e92_af14_9f3173ba39d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetAccessEntryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::core::HRESULT,
    pub GetAccessEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT,
    pub AddAccessEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT,
    pub RemoveAccessEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMAddressAccess2(::windows::core::IUnknown);
impl IWMAddressAccess2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAccessEntryCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__: WM_ADDRESS_ACCESSENTRY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAccessEntry)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(&mut result__)).from_abi::<WM_ADDRESS_ACCESSENTRY>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddAccessEntry)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(paddraccessentry)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveAccessEntry)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAccessEntryEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(pbstraddress), ::core::mem::transmute(pbstrmask)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddAccessEntryEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, aetype: WM_AETYPE, bstraddress: Param1, bstrmask: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddAccessEntryEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(aetype), bstraddress.into_param().abi(), bstrmask.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMAddressAccess2> for ::windows::core::IUnknown {
    fn from(value: IWMAddressAccess2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAddressAccess2> for ::windows::core::IUnknown {
    fn from(value: &IWMAddressAccess2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMAddressAccess2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMAddressAccess2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMAddressAccess2> for IWMAddressAccess {
    fn from(value: IWMAddressAccess2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAddressAccess2> for IWMAddressAccess {
    fn from(value: &IWMAddressAccess2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMAddressAccess> for IWMAddressAccess2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMAddressAccess> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMAddressAccess> for &'a IWMAddressAccess2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMAddressAccess> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMAddressAccess2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMAddressAccess2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess2 {}
impl ::core::fmt::Debug for IWMAddressAccess2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAddressAccess2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMAddressAccess2 {
    type Vtable = IWMAddressAccess2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65a83fc2_3e98_4d4d_81b5_2a742886b33d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAddressAccess2_Vtbl {
    pub base: IWMAddressAccess_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAccessEntryEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAccessEntryEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddAccessEntryEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddAccessEntryEx: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMAuthorizer(::windows::core::IUnknown);
impl IWMAuthorizer {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCertCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCertCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCert)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSharedData)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcertindex), ::core::mem::transmute(pbshareddata), ::core::mem::transmute(pbcert), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
}
impl ::core::convert::From<IWMAuthorizer> for ::windows::core::IUnknown {
    fn from(value: IWMAuthorizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMAuthorizer> for ::windows::core::IUnknown {
    fn from(value: &IWMAuthorizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMAuthorizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMAuthorizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMAuthorizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMAuthorizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAuthorizer {}
impl ::core::fmt::Debug for IWMAuthorizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAuthorizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMAuthorizer {
    type Vtable = IWMAuthorizer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9b67d36_a9ad_4eb4_baef_db284ef5504c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMAuthorizer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCertCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows::core::HRESULT,
    pub GetCert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetSharedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMBackupRestoreProps(::windows::core::IUnknown);
impl IWMBackupRestoreProps {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPropCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPropCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPropByIndex(&self, windex: u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropByIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPropByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropByName)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProp<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszname: Param0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProp)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveProp<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pcwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveProp)(::core::mem::transmute_copy(self), pcwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveAllProps(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllProps)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMBackupRestoreProps> for ::windows::core::IUnknown {
    fn from(value: IWMBackupRestoreProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMBackupRestoreProps> for ::windows::core::IUnknown {
    fn from(value: &IWMBackupRestoreProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMBackupRestoreProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMBackupRestoreProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMBackupRestoreProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMBackupRestoreProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBackupRestoreProps {}
impl ::core::fmt::Debug for IWMBackupRestoreProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMBackupRestoreProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMBackupRestoreProps {
    type Vtable = IWMBackupRestoreProps_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c8e0da6_996f_4ff3_a1af_4838f9377e2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBackupRestoreProps_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetPropCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows::core::HRESULT,
    pub GetPropByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub GetPropByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub RemoveProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RemoveAllProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMBandwidthSharing(::windows::core::IUnknown);
impl IWMBandwidthSharing {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetStreams)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBandwidth(&self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBandwidth)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbitrate), ::core::mem::transmute(pmsbufferwindow)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetBandwidth(&self, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBandwidth)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbitrate), ::core::mem::transmute(msbufferwindow)).ok()
    }
}
impl ::core::convert::From<IWMBandwidthSharing> for ::windows::core::IUnknown {
    fn from(value: IWMBandwidthSharing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMBandwidthSharing> for ::windows::core::IUnknown {
    fn from(value: &IWMBandwidthSharing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMBandwidthSharing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMBandwidthSharing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMBandwidthSharing> for IWMStreamList {
    fn from(value: IWMBandwidthSharing) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMBandwidthSharing> for IWMStreamList {
    fn from(value: &IWMBandwidthSharing) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for IWMBandwidthSharing {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for &'a IWMBandwidthSharing {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMBandwidthSharing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMBandwidthSharing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBandwidthSharing {}
impl ::core::fmt::Debug for IWMBandwidthSharing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMBandwidthSharing").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMBandwidthSharing {
    type Vtable = IWMBandwidthSharing_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad694af1_f8d9_42f8_bc47_70311b0c4f9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMBandwidthSharing_Vtbl {
    pub base: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT,
    pub SetBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMClientConnections(::windows::core::IUnknown);
impl IWMClientConnections {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetClientCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClientCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__: WM_CLIENT_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClientProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclientnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_CLIENT_PROPERTIES>(result__)
    }
}
impl ::core::convert::From<IWMClientConnections> for ::windows::core::IUnknown {
    fn from(value: IWMClientConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMClientConnections> for ::windows::core::IUnknown {
    fn from(value: &IWMClientConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMClientConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMClientConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMClientConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMClientConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections {}
impl ::core::fmt::Debug for IWMClientConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMClientConnections").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMClientConnections {
    type Vtable = IWMClientConnections_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73c66010_a299_41df_b1f0_ccf03b09c1c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetClientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows::core::HRESULT,
    pub GetClientProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMClientConnections2(::windows::core::IUnknown);
impl IWMClientConnections2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetClientCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClientCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__: WM_CLIENT_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClientProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclientnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_CLIENT_PROPERTIES>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetClientInfo(&self, dwclientnum: u32, pwsznetworkaddress: ::windows::core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows::core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows::core::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClientInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclientnum), ::core::mem::transmute(pwsznetworkaddress), ::core::mem::transmute(pcchnetworkaddress), ::core::mem::transmute(pwszport), ::core::mem::transmute(pcchport), ::core::mem::transmute(pwszdnsname), ::core::mem::transmute(pcchdnsname)).ok()
    }
}
impl ::core::convert::From<IWMClientConnections2> for ::windows::core::IUnknown {
    fn from(value: IWMClientConnections2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMClientConnections2> for ::windows::core::IUnknown {
    fn from(value: &IWMClientConnections2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMClientConnections2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMClientConnections2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMClientConnections2> for IWMClientConnections {
    fn from(value: IWMClientConnections2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMClientConnections2> for IWMClientConnections {
    fn from(value: &IWMClientConnections2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMClientConnections> for IWMClientConnections2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMClientConnections> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMClientConnections> for &'a IWMClientConnections2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMClientConnections> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMClientConnections2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMClientConnections2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections2 {}
impl ::core::fmt::Debug for IWMClientConnections2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMClientConnections2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMClientConnections2 {
    type Vtable = IWMClientConnections2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4091571e_4701_4593_bb3d_d5f5f0c74246);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMClientConnections2_Vtbl {
    pub base: IWMClientConnections_Vtbl,
    pub GetClientInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: ::windows::core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows::core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows::core::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMCodecAMVideoAccelerator(::windows::core::IUnknown);
impl IWMCodecAMVideoAccelerator {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Media_DirectShow'*"]
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub unsafe fn SetAcceleratorInterface<'a, Param0: ::windows::core::IntoParam<'a, super::DirectShow::IAMVideoAccelerator>>(&self, piamva: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAcceleratorInterface)(::core::mem::transmute_copy(self), piamva.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation', 'Win32_Media_DirectShow'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub unsafe fn NegotiateConnection(&self, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NegotiateConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmediatype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPlayerNotify<'a, Param0: ::windows::core::IntoParam<'a, IWMPlayerTimestampHook>>(&self, phook: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayerNotify)(::core::mem::transmute_copy(self), phook.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMCodecAMVideoAccelerator> for ::windows::core::IUnknown {
    fn from(value: IWMCodecAMVideoAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecAMVideoAccelerator> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecAMVideoAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecAMVideoAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMCodecAMVideoAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecAMVideoAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecAMVideoAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecAMVideoAccelerator {}
impl ::core::fmt::Debug for IWMCodecAMVideoAccelerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecAMVideoAccelerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCodecAMVideoAccelerator {
    type Vtable = IWMCodecAMVideoAccelerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd98ee251_34e0_4a2d_9312_9b4c788d9fa1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecAMVideoAccelerator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Media_DirectShow")]
    pub SetAcceleratorInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_DirectShow"))]
    SetAcceleratorInterface: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub NegotiateConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow")))]
    NegotiateConnection: usize,
    pub SetPlayerNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMCodecInfo(::windows::core::IUnknown);
impl IWMCodecInfo {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCodecInfoCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCodecFormatCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCodecFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
}
impl ::core::convert::From<IWMCodecInfo> for ::windows::core::IUnknown {
    fn from(value: IWMCodecInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMCodecInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo {}
impl ::core::fmt::Debug for IWMCodecInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCodecInfo {
    type Vtable = IWMCodecInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa970f41e_34de_4a98_b3ba_e4b3ca7528f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCodecInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMCodecInfo2(::windows::core::IUnknown);
impl IWMCodecInfo2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCodecInfoCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCodecFormatCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCodecFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecName)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(wszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: ::windows::core::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecFormatDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(ppistreamconfig), ::core::mem::transmute(wszdesc), ::core::mem::transmute(pcchdesc)).ok()
    }
}
impl ::core::convert::From<IWMCodecInfo2> for ::windows::core::IUnknown {
    fn from(value: IWMCodecInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo2> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMCodecInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMCodecInfo2> for IWMCodecInfo {
    fn from(value: IWMCodecInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo2> for IWMCodecInfo {
    fn from(value: &IWMCodecInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo> for IWMCodecInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo> for &'a IWMCodecInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo2 {}
impl ::core::fmt::Debug for IWMCodecInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCodecInfo2 {
    type Vtable = IWMCodecInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa65e273_b686_4056_91ec_dd768d4df710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo2_Vtbl {
    pub base: IWMCodecInfo_Vtbl,
    pub GetCodecName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecFormatDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr, wszdesc: ::windows::core::PWSTR, pcchdesc: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMCodecInfo3(::windows::core::IUnknown);
impl IWMCodecInfo3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetCodecInfoCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetCodecFormatCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetCodecFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetCodecName)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(wszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: ::windows::core::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetCodecFormatDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), ::core::mem::transmute(ppistreamconfig), ::core::mem::transmute(wszdesc), ::core::mem::transmute(pcchdesc)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecFormatProp<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: Param3, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecFormatProp)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), ::core::mem::transmute(dwformatindex), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecProp<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: Param2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecProp)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetCodecEnumerationSetting<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: Param2, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCodecEnumerationSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecEnumerationSetting<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: Param2, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecEnumerationSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype), ::core::mem::transmute(dwcodecindex), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IWMCodecInfo3> for ::windows::core::IUnknown {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo3> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMCodecInfo3> for IWMCodecInfo {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo3> for IWMCodecInfo {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo> for &'a IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMCodecInfo3> for IWMCodecInfo2 {
    fn from(value: IWMCodecInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecInfo3> for IWMCodecInfo2 {
    fn from(value: &IWMCodecInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo2> for IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMCodecInfo2> for &'a IWMCodecInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMCodecInfo2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo3 {}
impl ::core::fmt::Debug for IWMCodecInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCodecInfo3 {
    type Vtable = IWMCodecInfo3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e51f487_4d93_4f98_8ab4_27d0565adc51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecInfo3_Vtbl {
    pub base: IWMCodecInfo2_Vtbl,
    pub GetCodecFormatProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecProp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetCodecEnumerationSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT,
    pub GetCodecEnumerationSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMCodecVideoAccelerator(::windows::core::IUnknown);
impl IWMCodecVideoAccelerator {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation', 'Win32_Media_DirectShow'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub unsafe fn NegotiateConnection<'a, Param0: ::windows::core::IntoParam<'a, super::DirectShow::IAMVideoAccelerator>>(&self, piamva: Param0, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NegotiateConnection)(::core::mem::transmute_copy(self), piamva.into_param().abi(), ::core::mem::transmute(pmediatype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPlayerNotify<'a, Param0: ::windows::core::IntoParam<'a, IWMPlayerTimestampHook>>(&self, phook: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayerNotify)(::core::mem::transmute_copy(self), phook.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMCodecVideoAccelerator> for ::windows::core::IUnknown {
    fn from(value: IWMCodecVideoAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCodecVideoAccelerator> for ::windows::core::IUnknown {
    fn from(value: &IWMCodecVideoAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCodecVideoAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMCodecVideoAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCodecVideoAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCodecVideoAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecVideoAccelerator {}
impl ::core::fmt::Debug for IWMCodecVideoAccelerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecVideoAccelerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCodecVideoAccelerator {
    type Vtable = IWMCodecVideoAccelerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x990641b0_739f_4e94_a808_9888da8f75af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCodecVideoAccelerator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
    pub NegotiateConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow")))]
    NegotiateConnection: usize,
    pub SetPlayerNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMCredentialCallback(::windows::core::IUnknown);
impl IWMCredentialCallback {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AcquireCredentials<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszrealm: Param0, pwszsite: Param1, pwszuser: &mut [u16], pwszpassword: &mut [u16], hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquireCredentials)(::core::mem::transmute_copy(self), pwszrealm.into_param().abi(), pwszsite.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszuser)), pwszuser.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pwszpassword)), pwszpassword.len() as _, ::core::mem::transmute(hrstatus), ::core::mem::transmute(pdwflags)).ok()
    }
}
impl ::core::convert::From<IWMCredentialCallback> for ::windows::core::IUnknown {
    fn from(value: IWMCredentialCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMCredentialCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMCredentialCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMCredentialCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMCredentialCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMCredentialCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMCredentialCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCredentialCallback {}
impl ::core::fmt::Debug for IWMCredentialCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCredentialCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMCredentialCallback {
    type Vtable = IWMCredentialCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x342e0eb7_e651_450c_975b_2ace2c90c48e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMCredentialCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AcquireCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszrealm: ::windows::core::PCWSTR, pwszsite: ::windows::core::PCWSTR, pwszuser: ::windows::core::PWSTR, cchuser: u32, pwszpassword: ::windows::core::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMEditor(::windows::core::IUnknown);
impl IWMDRMEditor {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDRMProperty)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
}
impl ::core::convert::From<IWMDRMEditor> for ::windows::core::IUnknown {
    fn from(value: IWMDRMEditor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMEditor> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMEditor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMEditor {}
impl ::core::fmt::Debug for IWMDRMEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMEditor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMEditor {
    type Vtable = IWMDRMEditor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff130ebc_a6c3_42a6_b401_c3382c3e08b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMEditor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDRMProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: ::windows::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMMessageParser(::windows::core::IUnknown);
impl IWMDRMMessageParser {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: &[u8], ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ParseRegistrationReqMsg)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbregistrationreqmsg)), pbregistrationreqmsg.len() as _, ::core::mem::transmute(ppdevicecert), ::core::mem::transmute(pdeviceserialnumber)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ParseLicenseRequestMsg)(::core::mem::transmute_copy(self), ::core::mem::transmute(pblicenserequestmsg), ::core::mem::transmute(cblicenserequestmsg), ::core::mem::transmute(ppdevicecert), ::core::mem::transmute(pdeviceserialnumber), ::core::mem::transmute(pbstraction)).ok()
    }
}
impl ::core::convert::From<IWMDRMMessageParser> for ::windows::core::IUnknown {
    fn from(value: IWMDRMMessageParser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMMessageParser> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMMessageParser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMMessageParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMMessageParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMMessageParser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMMessageParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMMessageParser {}
impl ::core::fmt::Debug for IWMDRMMessageParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMMessageParser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMMessageParser {
    type Vtable = IWMDRMMessageParser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa73a0072_25a0_4c99_b4a5_ede8101a6c39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMMessageParser_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ParseRegistrationReqMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ParseLicenseRequestMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ParseLicenseRequestMsg: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMReader(::windows::core::IUnknown);
impl IWMDRMReader {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquireLicense)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelLicenseAcquisition)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Individualize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelIndividualization)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MonitorLicenseAcquisition)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelMonitorLicenseAcquisition)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDRMProperty)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDRMProperty)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
}
impl ::core::convert::From<IWMDRMReader> for ::windows::core::IUnknown {
    fn from(value: IWMDRMReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader {}
impl ::core::fmt::Debug for IWMDRMReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMReader {
    type Vtable = IWMDRMReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2827540_3ee7_432c_b14c_dc17f085d3b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AcquireLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub CancelLicenseAcquisition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Individualize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub CancelIndividualization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MonitorLicenseAcquisition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelMonitorLicenseAcquisition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDRMProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: ::windows::core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub GetDRMProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrname: ::windows::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMReader2(::windows::core::IUnknown);
impl IWMDRMReader2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AcquireLicense)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CancelLicenseAcquisition)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Individualize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CancelIndividualization)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.MonitorLicenseAcquisition)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CancelMonitorLicenseAcquisition)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDRMProperty)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDRMProperty)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEvaluateOutputLevelLicenses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fevaluate: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEvaluateOutputLevelLicenses)(::core::mem::transmute_copy(self), fevaluate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPlayOutputLevels)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplayopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCopyOutputLevels)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcopyopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn TryNextLicense(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TryNextLicense)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMDRMReader2> for ::windows::core::IUnknown {
    fn from(value: IWMDRMReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader2> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMReader2> for IWMDRMReader {
    fn from(value: IWMDRMReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader2> for IWMDRMReader {
    fn from(value: &IWMDRMReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader> for IWMDRMReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader> for &'a IWMDRMReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader2 {}
impl ::core::fmt::Debug for IWMDRMReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMReader2 {
    type Vtable = IWMDRMReader2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbefe7a75_9f1d_4075_b9d9_a3c37bda49a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader2_Vtbl {
    pub base: IWMDRMReader_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEvaluateOutputLevelLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEvaluateOutputLevelLicenses: usize,
    pub GetPlayOutputLevels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT,
    pub GetCopyOutputLevels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT,
    pub TryNextLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMReader3(::windows::core::IUnknown);
impl IWMDRMReader3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AcquireLicense)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.CancelLicenseAcquisition)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Individualize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.CancelIndividualization)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.MonitorLicenseAcquisition)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.CancelMonitorLicenseAcquisition)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrname: Param0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDRMProperty)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDRMProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrname: Param0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDRMProperty)(::core::mem::transmute_copy(self), pwstrname.into_param().abi(), ::core::mem::transmute(pdwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEvaluateOutputLevelLicenses<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fevaluate: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEvaluateOutputLevelLicenses)(::core::mem::transmute_copy(self), fevaluate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetPlayOutputLevels)(::core::mem::transmute_copy(self), ::core::mem::transmute(pplayopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetCopyOutputLevels)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcopyopl), ::core::mem::transmute(pcblength), ::core::mem::transmute(pdwminappcompliancelevel)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn TryNextLicense(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.TryNextLicense)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetInclusionList(&self, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInclusionList)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppguids), ::core::mem::transmute(pcguids)).ok()
    }
}
impl ::core::convert::From<IWMDRMReader3> for ::windows::core::IUnknown {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader3> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMReader3> for IWMDRMReader {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader3> for IWMDRMReader {
    fn from(value: &IWMDRMReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader> for IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader> for &'a IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMReader3> for IWMDRMReader2 {
    fn from(value: IWMDRMReader3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMReader3> for IWMDRMReader2 {
    fn from(value: &IWMDRMReader3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader2> for IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMReader2> for &'a IWMDRMReader3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMReader2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMReader3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader3 {}
impl ::core::fmt::Debug for IWMDRMReader3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMReader3 {
    type Vtable = IWMDRMReader3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe08672de_f1e7_4ff4_a0a3_fc4b08e4caf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMReader3_Vtbl {
    pub base: IWMDRMReader2_Vtbl,
    pub GetInclusionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMTranscryptionManager(::windows::core::IUnknown);
impl IWMDRMTranscryptionManager {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateTranscryptor(&self) -> ::windows::core::Result<IWMDRMTranscryptor> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTranscryptor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMDRMTranscryptor>(result__)
    }
}
impl ::core::convert::From<IWMDRMTranscryptionManager> for ::windows::core::IUnknown {
    fn from(value: IWMDRMTranscryptionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptionManager> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMTranscryptionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMTranscryptionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMTranscryptionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMTranscryptionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptionManager {}
impl ::core::fmt::Debug for IWMDRMTranscryptionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMTranscryptionManager {
    type Vtable = IWMDRMTranscryptionManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1a887b2_a4f0_407a_b02e_efbd23bbecdf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptionManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateTranscryptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptranscryptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMTranscryptor(::windows::core::IUnknown);
impl IWMDRMTranscryptor {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, bstrfilename: Param0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(pblicenserequestmsg), ::core::mem::transmute(cblicenserequestmsg), ::core::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Seek)(::core::mem::transmute_copy(self), ::core::mem::transmute(hnstime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(pcbdata)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMDRMTranscryptor> for ::windows::core::IUnknown {
    fn from(value: IWMDRMTranscryptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptor> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMTranscryptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMTranscryptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMTranscryptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMTranscryptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor {}
impl ::core::fmt::Debug for IWMDRMTranscryptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMTranscryptor {
    type Vtable = IWMDRMTranscryptor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69059850_6e6f_4bb2_806f_71863ddfc471);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMTranscryptor2(::windows::core::IUnknown);
impl IWMDRMTranscryptor2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, bstrfilename: Param0, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Initialize)(::core::mem::transmute_copy(self), bstrfilename.into_param().abi(), ::core::mem::transmute(pblicenserequestmsg), ::core::mem::transmute(cblicenserequestmsg), ::core::mem::transmute(pplicenseresponsemsg), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Seek)(::core::mem::transmute_copy(self), ::core::mem::transmute(hnstime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(pcbdata)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SeekEx<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SeekEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime), ::core::mem::transmute(cnsduration), ::core::mem::transmute(flrate), fincludefileheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ZeroAdjustTimestamps<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ZeroAdjustTimestamps)(::core::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSeekStartTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSeekStartTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWMDRMTranscryptor2> for ::windows::core::IUnknown {
    fn from(value: IWMDRMTranscryptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptor2> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMTranscryptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMTranscryptor2> for IWMDRMTranscryptor {
    fn from(value: IWMDRMTranscryptor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMTranscryptor2> for IWMDRMTranscryptor {
    fn from(value: &IWMDRMTranscryptor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMTranscryptor> for IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMTranscryptor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMTranscryptor> for &'a IWMDRMTranscryptor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMTranscryptor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMTranscryptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor2 {}
impl ::core::fmt::Debug for IWMDRMTranscryptor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptor2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMTranscryptor2 {
    type Vtable = IWMDRMTranscryptor2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0da439f_d331_496a_bece_18e5bac5dd23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMTranscryptor2_Vtbl {
    pub base: IWMDRMTranscryptor_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SeekEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SeekEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ZeroAdjustTimestamps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ZeroAdjustTimestamps: usize,
    pub GetSeekStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows::core::HRESULT,
    pub GetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMWriter(::windows::core::IUnknown);
impl IWMDRMWriter {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateKeySeed)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyseed), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateKeyID)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyid), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GenerateSigningKeyPair)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprivkey), ::core::mem::transmute(pcwchprivkeylength), ::core::mem::transmute(pwszpubkey), ::core::mem::transmute(pcwchpubkeylength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDRMAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
}
impl ::core::convert::From<IWMDRMWriter> for ::windows::core::IUnknown {
    fn from(value: IWMDRMWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter {}
impl ::core::fmt::Debug for IWMDRMWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMWriter {
    type Vtable = IWMDRMWriter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6ea5dd0_12a0_43f4_90ab_a3fd451e6a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GenerateKeySeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    pub GenerateKeyID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT,
    pub GenerateSigningKeyPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT,
    pub SetDRMAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMWriter2(::windows::core::IUnknown);
impl IWMDRMWriter2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GenerateKeySeed)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyseed), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GenerateKeyID)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyid), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GenerateSigningKeyPair)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprivkey), ::core::mem::transmute(pcwchprivkeylength), ::core::mem::transmute(pwszpubkey), ::core::mem::transmute(pcwchpubkeylength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDRMAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWMDRMNetEncryption<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsamplesencrypted: Param0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWMDRMNetEncryption)(::core::mem::transmute_copy(self), fsamplesencrypted.into_param().abi(), ::core::mem::transmute(pbkeyid), ::core::mem::transmute(cbkeyid)).ok()
    }
}
impl ::core::convert::From<IWMDRMWriter2> for ::windows::core::IUnknown {
    fn from(value: IWMDRMWriter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter2> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMWriter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMWriter2> for IWMDRMWriter {
    fn from(value: IWMDRMWriter2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter2> for IWMDRMWriter {
    fn from(value: &IWMDRMWriter2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter> for IWMDRMWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter> for &'a IWMDRMWriter2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMWriter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter2 {}
impl ::core::fmt::Debug for IWMDRMWriter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMWriter2 {
    type Vtable = IWMDRMWriter2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38ee7a94_40e2_4e10_aa3f_33fd3210ed5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter2_Vtbl {
    pub base: IWMDRMWriter_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWMDRMNetEncryption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWMDRMNetEncryption: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDRMWriter3(::windows::core::IUnknown);
impl IWMDRMWriter3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GenerateKeySeed)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyseed), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GenerateKeyID)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszkeyid), ::core::mem::transmute(pcwchlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GenerateSigningKeyPair)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprivkey), ::core::mem::transmute(pcwchprivkeylength), ::core::mem::transmute(pwszpubkey), ::core::mem::transmute(pcwchpubkeylength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetDRMAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDRMAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWMDRMNetEncryption<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fsamplesencrypted: Param0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetWMDRMNetEncryption)(::core::mem::transmute_copy(self), fsamplesencrypted.into_param().abi(), ::core::mem::transmute(pbkeyid), ::core::mem::transmute(cbkeyid)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProtectStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(pimportinitstruct)).ok()
    }
}
impl ::core::convert::From<IWMDRMWriter3> for ::windows::core::IUnknown {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter3> for ::windows::core::IUnknown {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMWriter3> for IWMDRMWriter {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter3> for IWMDRMWriter {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter> for &'a IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMDRMWriter3> for IWMDRMWriter2 {
    fn from(value: IWMDRMWriter3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDRMWriter3> for IWMDRMWriter2 {
    fn from(value: &IWMDRMWriter3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter2> for IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMDRMWriter2> for &'a IWMDRMWriter3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMDRMWriter2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDRMWriter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter3 {}
impl ::core::fmt::Debug for IWMDRMWriter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDRMWriter3 {
    type Vtable = IWMDRMWriter3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7184082_a4aa_4dde_ac9c_e75dbd1117ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDRMWriter3_Vtbl {
    pub base: IWMDRMWriter2_Vtbl,
    pub SetProtectStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMDeviceRegistration(::windows::core::IUnknown);
impl IWMDeviceRegistration {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RegisterDevice<'a, Param3: ::windows::core::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: Param3) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbcertificate)), pbcertificate.len() as _, serialnumber.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMRegisteredDevice>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn UnregisterDevice<'a, Param3: ::windows::core::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbcertificate)), pbcertificate.len() as _, serialnumber.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetRegistrationStats(&self, dwregistertype: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRegistrationStats)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetFirstRegisteredDevice(&self, dwregistertype: u32) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFirstRegisteredDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(&mut result__)).from_abi::<IWMRegisteredDevice>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNextRegisteredDevice(&self) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNextRegisteredDevice)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMRegisteredDevice>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetRegisteredDeviceByID<'a, Param3: ::windows::core::IntoParam<'a, DRM_VAL16>>(&self, dwregistertype: u32, pbcertificate: &[u8], serialnumber: Param3) -> ::windows::core::Result<IWMRegisteredDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRegisteredDeviceByID)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregistertype), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbcertificate)), pbcertificate.len() as _, serialnumber.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMRegisteredDevice>(result__)
    }
}
impl ::core::convert::From<IWMDeviceRegistration> for ::windows::core::IUnknown {
    fn from(value: IWMDeviceRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMDeviceRegistration> for ::windows::core::IUnknown {
    fn from(value: &IWMDeviceRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMDeviceRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMDeviceRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMDeviceRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMDeviceRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDeviceRegistration {}
impl ::core::fmt::Debug for IWMDeviceRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDeviceRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMDeviceRegistration {
    type Vtable = IWMDeviceRegistration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6211f03_8d21_4e94_93e6_8510805f2d99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMDeviceRegistration_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RegisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UnregisterDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::HRESULT,
    pub GetRegistrationStats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows::core::HRESULT,
    pub GetFirstRegisteredDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetNextRegisteredDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetRegisteredDeviceByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMGetSecureChannel(::windows::core::IUnknown);
impl IWMGetSecureChannel {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPeerSecureChannelInterface(&self) -> ::windows::core::Result<IWMSecureChannel> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPeerSecureChannelInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMSecureChannel>(result__)
    }
}
impl ::core::convert::From<IWMGetSecureChannel> for ::windows::core::IUnknown {
    fn from(value: IWMGetSecureChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMGetSecureChannel> for ::windows::core::IUnknown {
    fn from(value: &IWMGetSecureChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMGetSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMGetSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMGetSecureChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMGetSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMGetSecureChannel {}
impl ::core::fmt::Debug for IWMGetSecureChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMGetSecureChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMGetSecureChannel {
    type Vtable = IWMGetSecureChannel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94bc0598_c3d2_11d3_bedf_00c04f612986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMGetSecureChannel_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetPeerSecureChannelInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppeer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMHeaderInfo(::windows::core::IUnknown);
impl IWMHeaderInfo {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttributeCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeByIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwstreamnum), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeByName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMarkerCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszmarkername), ::core::mem::transmute(pcchmarkernamelen), ::core::mem::transmute(pcnsmarkertime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddMarker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddMarker)(::core::mem::transmute_copy(self), pwszmarkername.into_param().abi(), ::core::mem::transmute(cnsmarkertime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetScriptCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScript)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwsztype), ::core::mem::transmute(pcchtypelen), ::core::mem::transmute(pwszcommand), ::core::mem::transmute(pcchcommandlen), ::core::mem::transmute(pcnsscripttime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddScript<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddScript)(::core::mem::transmute_copy(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::core::mem::transmute(cnsscripttime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveScript)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
}
impl ::core::convert::From<IWMHeaderInfo> for ::windows::core::IUnknown {
    fn from(value: IWMHeaderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo> for ::windows::core::IUnknown {
    fn from(value: &IWMHeaderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMHeaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMHeaderInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMHeaderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo {}
impl ::core::fmt::Debug for IWMHeaderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMHeaderInfo {
    type Vtable = IWMHeaderInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bda_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT,
    pub GetAttributeByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub GetAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub GetMarkerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT,
    pub GetMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT,
    pub AddMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmarkername: ::windows::core::PCWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT,
    pub RemoveMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
    pub GetScriptCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT,
    pub GetScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT,
    pub AddScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztype: ::windows::core::PCWSTR, pwszcommand: ::windows::core::PCWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT,
    pub RemoveScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMHeaderInfo2(::windows::core::IUnknown);
impl IWMHeaderInfo2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAttributeCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetAttributeByIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwstreamnum), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetAttributeByName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMarkerCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszmarkername), ::core::mem::transmute(pcchmarkernamelen), ::core::mem::transmute(pcnsmarkertime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddMarker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddMarker)(::core::mem::transmute_copy(self), pwszmarkername.into_param().abi(), ::core::mem::transmute(cnsmarkertime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetScriptCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetScript)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwsztype), ::core::mem::transmute(pcchtypelen), ::core::mem::transmute(pwszcommand), ::core::mem::transmute(pcchcommandlen), ::core::mem::transmute(pcnsscripttime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddScript<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddScript)(::core::mem::transmute_copy(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::core::mem::transmute(cnsscripttime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveScript)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCodecInfoCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pcchname), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchdescription), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcodectype), ::core::mem::transmute(pcbcodecinfo), ::core::mem::transmute(pbcodecinfo)).ok()
    }
}
impl ::core::convert::From<IWMHeaderInfo2> for ::windows::core::IUnknown {
    fn from(value: IWMHeaderInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo2> for ::windows::core::IUnknown {
    fn from(value: &IWMHeaderInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMHeaderInfo2> for IWMHeaderInfo {
    fn from(value: IWMHeaderInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo2> for IWMHeaderInfo {
    fn from(value: &IWMHeaderInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo> for IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo> for &'a IWMHeaderInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMHeaderInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo2 {}
impl ::core::fmt::Debug for IWMHeaderInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMHeaderInfo2 {
    type Vtable = IWMHeaderInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15cf9781_454e_482e_b393_85fae487a810);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo2_Vtbl {
    pub base: IWMHeaderInfo_Vtbl,
    pub GetCodecInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows::core::HRESULT,
    pub GetCodecInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMHeaderInfo3(::windows::core::IUnknown);
impl IWMHeaderInfo3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetAttributeCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetAttributeByIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwstreamnum), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeByName<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstreamnum: *mut u16, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetAttributeByName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMarkerCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszmarkername), ::core::mem::transmute(pcchmarkernamelen), ::core::mem::transmute(pcnsmarkertime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddMarker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszmarkername: Param0, cnsmarkertime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddMarker)(::core::mem::transmute_copy(self), pwszmarkername.into_param().abi(), ::core::mem::transmute(cnsmarkertime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RemoveMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetScriptCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetScript)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwsztype), ::core::mem::transmute(pcchtypelen), ::core::mem::transmute(pwszcommand), ::core::mem::transmute(pcchcommandlen), ::core::mem::transmute(pcnsscripttime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddScript<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwsztype: Param0, pwszcommand: Param1, cnsscripttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddScript)(::core::mem::transmute_copy(self), pwsztype.into_param().abi(), pwszcommand.into_param().abi(), ::core::mem::transmute(cnsscripttime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RemoveScript)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCodecInfoCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetCodecInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pcchname), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchdescription), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcodectype), ::core::mem::transmute(pcbcodecinfo), ::core::mem::transmute(pbcodecinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeCountEx(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttributeCountEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeIndices<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wstreamnum: u16, pwszname: Param1, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeIndices)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pwszname.into_param().abi(), ::core::mem::transmute(pwlangindex), ::core::mem::transmute(pwindices), ::core::mem::transmute(pwcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeByIndexEx(&self, wstreamnum: u16, windex: u16, pwszname: ::windows::core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeByIndexEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(windex), ::core::mem::transmute(pwszname), ::core::mem::transmute(pwnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pwlangindex), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwdatalength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ModifyAttribute(&self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModifyAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(windex), ::core::mem::transmute(r#type), ::core::mem::transmute(wlangindex), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wstreamnum: u16, pszname: Param1, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pszname.into_param().abi(), ::core::mem::transmute(pwindex), ::core::mem::transmute(r#type), ::core::mem::transmute(wlangindex), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn DeleteAttribute(&self, wstreamnum: u16, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(windex)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddCodecInfo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0, pwszdescription: Param1, codectype: WMT_CODEC_INFO_TYPE, pbcodecinfo: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddCodecInfo)(::core::mem::transmute_copy(self), pwszname.into_param().abi(), pwszdescription.into_param().abi(), ::core::mem::transmute(codectype), pbcodecinfo.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbcodecinfo))).ok()
    }
}
impl ::core::convert::From<IWMHeaderInfo3> for ::windows::core::IUnknown {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo3> for ::windows::core::IUnknown {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMHeaderInfo3> for IWMHeaderInfo {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo3> for IWMHeaderInfo {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo> for &'a IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMHeaderInfo3> for IWMHeaderInfo2 {
    fn from(value: IWMHeaderInfo3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMHeaderInfo3> for IWMHeaderInfo2 {
    fn from(value: &IWMHeaderInfo3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo2> for IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMHeaderInfo2> for &'a IWMHeaderInfo3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMHeaderInfo2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMHeaderInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo3 {}
impl ::core::fmt::Debug for IWMHeaderInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMHeaderInfo3 {
    type Vtable = IWMHeaderInfo3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15cc68e3_27cc_4ecd_b222_3f5d02d80bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMHeaderInfo3_Vtbl {
    pub base: IWMHeaderInfo2_Vtbl,
    pub GetAttributeCountEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT,
    pub GetAttributeIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: ::windows::core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::HRESULT,
    pub GetAttributeByIndexEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: ::windows::core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::HRESULT,
    pub ModifyAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT,
    pub AddAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows::core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT,
    pub DeleteAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows::core::HRESULT,
    pub AddCodecInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwszdescription: ::windows::core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMIStreamProps(::windows::core::IUnknown);
impl IWMIStreamProps {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
}
impl ::core::convert::From<IWMIStreamProps> for ::windows::core::IUnknown {
    fn from(value: IWMIStreamProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIStreamProps> for ::windows::core::IUnknown {
    fn from(value: &IWMIStreamProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMIStreamProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMIStreamProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIStreamProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIStreamProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIStreamProps {}
impl ::core::fmt::Debug for IWMIStreamProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIStreamProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMIStreamProps {
    type Vtable = IWMIStreamProps_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6816dad3_2b4b_4c8e_8149_874c3483a753);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIStreamProps_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMImageInfo(::windows::core::IUnknown);
impl IWMImageInfo {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetImageCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetImageCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetImage(&self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetImage)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pcchmimetype), ::core::mem::transmute(pwszmimetype), ::core::mem::transmute(pcchdescription), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pimagetype), ::core::mem::transmute(pcbimagedata), ::core::mem::transmute(pbimagedata)).ok()
    }
}
impl ::core::convert::From<IWMImageInfo> for ::windows::core::IUnknown {
    fn from(value: IWMImageInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMImageInfo> for ::windows::core::IUnknown {
    fn from(value: &IWMImageInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMImageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMImageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMImageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMImageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMImageInfo {}
impl ::core::fmt::Debug for IWMImageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMImageInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMImageInfo {
    type Vtable = IWMImageInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f0aa3b6_7267_4d89_88f2_ba915aa5c4c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMImageInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetImageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows::core::HRESULT,
    pub GetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMIndexer(::windows::core::IUnknown);
impl IWMIndexer {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartIndexing<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartIndexing)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMIndexer> for ::windows::core::IUnknown {
    fn from(value: IWMIndexer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIndexer> for ::windows::core::IUnknown {
    fn from(value: &IWMIndexer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIndexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer {}
impl ::core::fmt::Debug for IWMIndexer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIndexer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMIndexer {
    type Vtable = IWMIndexer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d7cdc71_9888_11d3_8edc_00c04f6109cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub StartIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMIndexer2(::windows::core::IUnknown);
impl IWMIndexer2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartIndexing<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StartIndexing)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Configure(&self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Configure)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(nindexertype), ::core::mem::transmute(pvinterval), ::core::mem::transmute(pvindextype)).ok()
    }
}
impl ::core::convert::From<IWMIndexer2> for ::windows::core::IUnknown {
    fn from(value: IWMIndexer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIndexer2> for ::windows::core::IUnknown {
    fn from(value: &IWMIndexer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMIndexer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMIndexer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMIndexer2> for IWMIndexer {
    fn from(value: IWMIndexer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMIndexer2> for IWMIndexer {
    fn from(value: &IWMIndexer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMIndexer> for IWMIndexer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMIndexer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMIndexer> for &'a IWMIndexer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMIndexer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMIndexer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMIndexer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer2 {}
impl ::core::fmt::Debug for IWMIndexer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIndexer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMIndexer2 {
    type Vtable = IWMIndexer2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb70f1e42_6255_4df0_a6b9_02b212d9e2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMIndexer2_Vtbl {
    pub base: IWMIndexer_Vtbl,
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMInputMediaProps(::windows::core::IUnknown);
impl IWMInputMediaProps {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetMediaType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMediaType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetConnectionName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConnectionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetGroupName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGroupName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
}
impl ::core::convert::From<IWMInputMediaProps> for ::windows::core::IUnknown {
    fn from(value: IWMInputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMInputMediaProps> for ::windows::core::IUnknown {
    fn from(value: &IWMInputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMInputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMInputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMInputMediaProps> for IWMMediaProps {
    fn from(value: IWMInputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMInputMediaProps> for IWMMediaProps {
    fn from(value: &IWMInputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for IWMInputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for &'a IWMInputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMInputMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMInputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMInputMediaProps {}
impl ::core::fmt::Debug for IWMInputMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMInputMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMInputMediaProps {
    type Vtable = IWMInputMediaProps_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd5_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMInputMediaProps_Vtbl {
    pub base: IWMMediaProps_Vtbl,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    pub GetGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMLanguageList(::windows::core::IUnknown);
impl IWMLanguageList {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLanguageCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLanguageCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLanguageDetails(&self, windex: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLanguageDetails)(::core::mem::transmute_copy(self), ::core::mem::transmute(windex), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddLanguageByRFC1766String<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszlanguagestring: Param0) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddLanguageByRFC1766String)(::core::mem::transmute_copy(self), pwszlanguagestring.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
}
impl ::core::convert::From<IWMLanguageList> for ::windows::core::IUnknown {
    fn from(value: IWMLanguageList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLanguageList> for ::windows::core::IUnknown {
    fn from(value: &IWMLanguageList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMLanguageList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMLanguageList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLanguageList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLanguageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLanguageList {}
impl ::core::fmt::Debug for IWMLanguageList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLanguageList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMLanguageList {
    type Vtable = IWMLanguageList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf683f00_2d49_4d8e_92b7_fb19f6a0dc57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLanguageList_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetLanguageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT,
    pub GetLanguageDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    pub AddLanguageByRFC1766String: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows::core::PCWSTR, pwindex: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMLicenseBackup(::windows::core::IUnknown);
impl IWMLicenseBackup {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn BackupLicenses<'a, Param1: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, dwflags: u32, pcallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackupLicenses)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelLicenseBackup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelLicenseBackup)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMLicenseBackup> for ::windows::core::IUnknown {
    fn from(value: IWMLicenseBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLicenseBackup> for ::windows::core::IUnknown {
    fn from(value: &IWMLicenseBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMLicenseBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMLicenseBackup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLicenseBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLicenseBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseBackup {}
impl ::core::fmt::Debug for IWMLicenseBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseBackup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMLicenseBackup {
    type Vtable = IWMLicenseBackup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05e5ac9f_3fb6_4508_bb43_a4067ba1ebe8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseBackup_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub BackupLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CancelLicenseBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMLicenseRestore(::windows::core::IUnknown);
impl IWMLicenseRestore {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RestoreLicenses<'a, Param1: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, dwflags: u32, pcallback: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreLicenses)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelLicenseRestore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelLicenseRestore)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMLicenseRestore> for ::windows::core::IUnknown {
    fn from(value: IWMLicenseRestore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLicenseRestore> for ::windows::core::IUnknown {
    fn from(value: &IWMLicenseRestore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMLicenseRestore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMLicenseRestore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLicenseRestore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLicenseRestore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRestore {}
impl ::core::fmt::Debug for IWMLicenseRestore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseRestore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMLicenseRestore {
    type Vtable = IWMLicenseRestore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc70b6334_a22e_4efb_a245_15e65a004a13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRestore_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RestoreLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CancelLicenseRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMLicenseRevocationAgent(::windows::core::IUnknown);
impl IWMLicenseRevocationAgent {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLRBChallenge(&self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLRBChallenge)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmachineid), ::core::mem::transmute(dwmachineidlength), ::core::mem::transmute(pchallenge), ::core::mem::transmute(dwchallengelength), ::core::mem::transmute(pchallengeoutput), ::core::mem::transmute(pdwchallengeoutputlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ProcessLRB(&self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessLRB)(::core::mem::transmute_copy(self), ::core::mem::transmute(psignedlrb), ::core::mem::transmute(dwsignedlrblength), ::core::mem::transmute(psignedack), ::core::mem::transmute(pdwsignedacklength)).ok()
    }
}
impl ::core::convert::From<IWMLicenseRevocationAgent> for ::windows::core::IUnknown {
    fn from(value: IWMLicenseRevocationAgent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMLicenseRevocationAgent> for ::windows::core::IUnknown {
    fn from(value: &IWMLicenseRevocationAgent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMLicenseRevocationAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMLicenseRevocationAgent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMLicenseRevocationAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMLicenseRevocationAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRevocationAgent {}
impl ::core::fmt::Debug for IWMLicenseRevocationAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseRevocationAgent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMLicenseRevocationAgent {
    type Vtable = IWMLicenseRevocationAgent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6967f2c9_4e26_4b57_8894_799880f7ac7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMLicenseRevocationAgent_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetLRBChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::HRESULT,
    pub ProcessLRB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMMediaProps(::windows::core::IUnknown);
impl IWMMediaProps {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMediaType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMediaType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype)).ok()
    }
}
impl ::core::convert::From<IWMMediaProps> for ::windows::core::IUnknown {
    fn from(value: IWMMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMediaProps> for ::windows::core::IUnknown {
    fn from(value: &IWMMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMediaProps {}
impl ::core::fmt::Debug for IWMMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMediaProps {
    type Vtable = IWMMediaProps_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bce_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMediaProps_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMediaType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMediaType: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMMetadataEditor(::windows::core::IUnknown);
impl IWMMetadataEditor {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMMetadataEditor> for ::windows::core::IUnknown {
    fn from(value: IWMMetadataEditor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMetadataEditor> for ::windows::core::IUnknown {
    fn from(value: &IWMMetadataEditor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMetadataEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMMetadataEditor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMetadataEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMetadataEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor {}
impl ::core::fmt::Debug for IWMMetadataEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMetadataEditor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMetadataEditor {
    type Vtable = IWMMetadataEditor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd9_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMMetadataEditor2(::windows::core::IUnknown);
impl IWMMetadataEditor2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Open)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Flush)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OpenEx<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenEx)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi(), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwsharemode)).ok()
    }
}
impl ::core::convert::From<IWMMetadataEditor2> for ::windows::core::IUnknown {
    fn from(value: IWMMetadataEditor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMetadataEditor2> for ::windows::core::IUnknown {
    fn from(value: &IWMMetadataEditor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMetadataEditor2> for IWMMetadataEditor {
    fn from(value: IWMMetadataEditor2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMetadataEditor2> for IWMMetadataEditor {
    fn from(value: &IWMMetadataEditor2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMetadataEditor> for IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMetadataEditor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMetadataEditor> for &'a IWMMetadataEditor2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMetadataEditor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMetadataEditor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMetadataEditor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor2 {}
impl ::core::fmt::Debug for IWMMetadataEditor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMetadataEditor2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMetadataEditor2 {
    type Vtable = IWMMetadataEditor2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x203cffe3_2e18_4fdf_b59d_6e71530534cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMetadataEditor2_Vtbl {
    pub base: IWMMetadataEditor_Vtbl,
    pub OpenEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMMutualExclusion(::windows::core::IUnknown);
impl IWMMutualExclusion {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetStreams)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype)).ok()
    }
}
impl ::core::convert::From<IWMMutualExclusion> for ::windows::core::IUnknown {
    fn from(value: IWMMutualExclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion> for ::windows::core::IUnknown {
    fn from(value: &IWMMutualExclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMutualExclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMMutualExclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMutualExclusion> for IWMStreamList {
    fn from(value: IWMMutualExclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion> for IWMStreamList {
    fn from(value: &IWMMutualExclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for IWMMutualExclusion {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for &'a IWMMutualExclusion {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMutualExclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMutualExclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion {}
impl ::core::fmt::Debug for IWMMutualExclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMutualExclusion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMutualExclusion {
    type Vtable = IWMMutualExclusion_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bde_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion_Vtbl {
    pub base: IWMStreamList_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMMutualExclusion2(::windows::core::IUnknown);
impl IWMMutualExclusion2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetStreams)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RemoveStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidtype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetRecordCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRecordCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddRecord(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRecord)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveRecord(&self, wrecordnumber: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveRecord)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetRecordName(&self, wrecordnumber: u16, pwszrecordname: ::windows::core::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRecordName)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(pwszrecordname), ::core::mem::transmute(pcchrecordname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetRecordName<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wrecordnumber: u16, pwszrecordname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRecordName)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), pwszrecordname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamsForRecord(&self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamsForRecord)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStreamForRecord)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(wstreamnumber)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStreamForRecord)(::core::mem::transmute_copy(self), ::core::mem::transmute(wrecordnumber), ::core::mem::transmute(wstreamnumber)).ok()
    }
}
impl ::core::convert::From<IWMMutualExclusion2> for ::windows::core::IUnknown {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion2> for ::windows::core::IUnknown {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMutualExclusion2> for IWMStreamList {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion2> for IWMStreamList {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamList> for &'a IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamList> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMMutualExclusion2> for IWMMutualExclusion {
    fn from(value: IWMMutualExclusion2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMMutualExclusion2> for IWMMutualExclusion {
    fn from(value: &IWMMutualExclusion2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMutualExclusion> for IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMutualExclusion> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMutualExclusion> for &'a IWMMutualExclusion2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMutualExclusion> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMMutualExclusion2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMMutualExclusion2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion2 {}
impl ::core::fmt::Debug for IWMMutualExclusion2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMutualExclusion2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMMutualExclusion2 {
    type Vtable = IWMMutualExclusion2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0302b57d_89d1_4ba2_85c9_166f2c53eb91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMMutualExclusion2_Vtbl {
    pub base: IWMMutualExclusion_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetRecordCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows::core::HRESULT,
    pub AddRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows::core::HRESULT,
    pub GetRecordName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows::core::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::HRESULT,
    pub SetRecordName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetStreamsForRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT,
    pub AddStreamForRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT,
    pub RemoveStreamForRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMOutputMediaProps(::windows::core::IUnknown);
impl IWMOutputMediaProps {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetMediaType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMediaType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamGroupName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamGroupName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetConnectionName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConnectionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
}
impl ::core::convert::From<IWMOutputMediaProps> for ::windows::core::IUnknown {
    fn from(value: IWMOutputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMOutputMediaProps> for ::windows::core::IUnknown {
    fn from(value: &IWMOutputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMOutputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMOutputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMOutputMediaProps> for IWMMediaProps {
    fn from(value: IWMOutputMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMOutputMediaProps> for IWMMediaProps {
    fn from(value: &IWMOutputMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for IWMOutputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for &'a IWMOutputMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMOutputMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMOutputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMOutputMediaProps {}
impl ::core::fmt::Debug for IWMOutputMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMOutputMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMOutputMediaProps {
    type Vtable = IWMOutputMediaProps_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd7_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMOutputMediaProps_Vtbl {
    pub base: IWMMediaProps_Vtbl,
    pub GetStreamGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMPacketSize(::windows::core::IUnknown);
impl IWMPacketSize {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxPacketSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxPacketSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxpacketsize)).ok()
    }
}
impl ::core::convert::From<IWMPacketSize> for ::windows::core::IUnknown {
    fn from(value: IWMPacketSize) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPacketSize> for ::windows::core::IUnknown {
    fn from(value: &IWMPacketSize) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPacketSize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMPacketSize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPacketSize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPacketSize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize {}
impl ::core::fmt::Debug for IWMPacketSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPacketSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPacketSize {
    type Vtable = IWMPacketSize_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdfb97ab_188f_40b3_b643_5b7903975c59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetMaxPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMPacketSize2(::windows::core::IUnknown);
impl IWMPacketSize2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaxPacketSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMaxPacketSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxpacketsize)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMinPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMinPacketSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetMinPacketSize(&self, dwminpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinPacketSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwminpacketsize)).ok()
    }
}
impl ::core::convert::From<IWMPacketSize2> for ::windows::core::IUnknown {
    fn from(value: IWMPacketSize2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPacketSize2> for ::windows::core::IUnknown {
    fn from(value: &IWMPacketSize2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPacketSize2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMPacketSize2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMPacketSize2> for IWMPacketSize {
    fn from(value: IWMPacketSize2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPacketSize2> for IWMPacketSize {
    fn from(value: &IWMPacketSize2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMPacketSize> for IWMPacketSize2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMPacketSize> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMPacketSize> for &'a IWMPacketSize2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMPacketSize> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPacketSize2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPacketSize2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize2 {}
impl ::core::fmt::Debug for IWMPacketSize2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPacketSize2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPacketSize2 {
    type Vtable = IWMPacketSize2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bfc2b9e_b646_4233_a877_1c6a079669dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPacketSize2_Vtbl {
    pub base: IWMPacketSize_Vtbl,
    pub GetMinPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetMinPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMPlayerHook(::windows::core::IUnknown);
impl IWMPlayerHook {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn PreDecode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreDecode)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMPlayerHook> for ::windows::core::IUnknown {
    fn from(value: IWMPlayerHook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPlayerHook> for ::windows::core::IUnknown {
    fn from(value: &IWMPlayerHook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPlayerHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMPlayerHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPlayerHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPlayerHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerHook {}
impl ::core::fmt::Debug for IWMPlayerHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPlayerHook").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPlayerHook {
    type Vtable = IWMPlayerHook_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5b7ca9a_0f1c_4f66_9002_74ec50d8b304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerHook_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub PreDecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMPlayerTimestampHook(::windows::core::IUnknown);
impl IWMPlayerTimestampHook {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn MapTimestamp(&self, rtin: i64) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MapTimestamp)(::core::mem::transmute_copy(self), ::core::mem::transmute(rtin), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
}
impl ::core::convert::From<IWMPlayerTimestampHook> for ::windows::core::IUnknown {
    fn from(value: IWMPlayerTimestampHook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPlayerTimestampHook> for ::windows::core::IUnknown {
    fn from(value: &IWMPlayerTimestampHook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPlayerTimestampHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMPlayerTimestampHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPlayerTimestampHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPlayerTimestampHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerTimestampHook {}
impl ::core::fmt::Debug for IWMPlayerTimestampHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPlayerTimestampHook").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPlayerTimestampHook {
    type Vtable = IWMPlayerTimestampHook_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28580dda_d98e_48d0_b7ae_69e473a02825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPlayerTimestampHook_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub MapTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMProfile(::windows::core::IUnknown);
impl IWMProfile {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__: WMT_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_VERSION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcchdescription)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), pwszdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStreamCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStreamByNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStream)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStreamByNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStream)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ReconfigStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReconfigStream)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateNewStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidstreamtype), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMutualExclusionCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMutualExclusion)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmeindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveMutualExclusion)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddMutualExclusion)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateNewMutualExclusion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
}
impl ::core::convert::From<IWMProfile> for ::windows::core::IUnknown {
    fn from(value: IWMProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile> for ::windows::core::IUnknown {
    fn from(value: &IWMProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile {}
impl ::core::fmt::Debug for IWMProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfile {
    type Vtable = IWMProfile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bdb_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetStreamCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT,
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStreamByNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveStreamByNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ReconfigStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateNewStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetMutualExclusionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT,
    pub GetMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateNewMutualExclusion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMProfile2(::windows::core::IUnknown);
impl IWMProfile2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__: WMT_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_VERSION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetName)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcchdescription)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), pwszdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStreamCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStreamByNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveStream)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveStreamByNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddStream)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ReconfigStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ReconfigStream)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateNewStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidstreamtype), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMutualExclusionCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMutualExclusion)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmeindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveMutualExclusion)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddMutualExclusion)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateNewMutualExclusion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProfileID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProfileID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IWMProfile2> for ::windows::core::IUnknown {
    fn from(value: IWMProfile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile2> for ::windows::core::IUnknown {
    fn from(value: &IWMProfile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfile2> for IWMProfile {
    fn from(value: IWMProfile2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile2> for IWMProfile {
    fn from(value: &IWMProfile2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile> for IWMProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile> for &'a IWMProfile2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfile2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile2 {}
impl ::core::fmt::Debug for IWMProfile2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfile2 {
    type Vtable = IWMProfile2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07e72d33_d94e_4be7_8843_60ae5ff7e5f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile2_Vtbl {
    pub base: IWMProfile_Vtbl,
    pub GetProfileID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMProfile3(::windows::core::IUnknown);
impl IWMProfile3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__: WMT_VERSION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_VERSION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszname), ::core::mem::transmute(pcchname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetName)(::core::mem::transmute_copy(self), pwszname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszdescription), ::core::mem::transmute(pcchdescription)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetDescription)(::core::mem::transmute_copy(self), pwszdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStreamCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStreamByNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RemoveStream)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RemoveStreamByNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddStream)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ReconfigStream<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamConfig>>(&self, pconfig: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.ReconfigStream)(::core::mem::transmute_copy(self), pconfig.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateNewStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidstreamtype), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamConfig>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMutualExclusionCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMutualExclusion)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmeindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RemoveMutualExclusion)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddMutualExclusion<'a, Param0: ::windows::core::IntoParam<'a, IWMMutualExclusion>>(&self, pme: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddMutualExclusion)(::core::mem::transmute_copy(self), pme.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CreateNewMutualExclusion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMMutualExclusion>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProfileID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetProfileID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStorageFormat(&self) -> ::windows::core::Result<WMT_STORAGE_FORMAT> {
        let mut result__: WMT_STORAGE_FORMAT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStorageFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STORAGE_FORMAT>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStorageFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(nstorageformat)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBandwidthSharingCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBandwidthSharingCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBandwidthSharing(&self, dwbsindex: u32) -> ::windows::core::Result<IWMBandwidthSharing> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBandwidthSharing)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwbsindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMBandwidthSharing>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveBandwidthSharing<'a, Param0: ::windows::core::IntoParam<'a, IWMBandwidthSharing>>(&self, pbs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveBandwidthSharing)(::core::mem::transmute_copy(self), pbs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddBandwidthSharing<'a, Param0: ::windows::core::IntoParam<'a, IWMBandwidthSharing>>(&self, pbs: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddBandwidthSharing)(::core::mem::transmute_copy(self), pbs.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateNewBandwidthSharing(&self) -> ::windows::core::Result<IWMBandwidthSharing> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateNewBandwidthSharing)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMBandwidthSharing>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamPrioritization(&self) -> ::windows::core::Result<IWMStreamPrioritization> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStreamPrioritization)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamPrioritization>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamPrioritization<'a, Param0: ::windows::core::IntoParam<'a, IWMStreamPrioritization>>(&self, psp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamPrioritization)(::core::mem::transmute_copy(self), psp.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStreamPrioritization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStreamPrioritization)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateNewStreamPrioritization(&self) -> ::windows::core::Result<IWMStreamPrioritization> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateNewStreamPrioritization)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IWMStreamPrioritization>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetExpectedPacketCount(&self, msduration: u64) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetExpectedPacketCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(msduration), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IWMProfile3> for ::windows::core::IUnknown {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile3> for ::windows::core::IUnknown {
    fn from(value: &IWMProfile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfile3> for IWMProfile {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile3> for IWMProfile {
    fn from(value: &IWMProfile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile> for IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile> for &'a IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfile3> for IWMProfile2 {
    fn from(value: IWMProfile3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfile3> for IWMProfile2 {
    fn from(value: &IWMProfile3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile2> for IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfile2> for &'a IWMProfile3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfile2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfile3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfile3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile3 {}
impl ::core::fmt::Debug for IWMProfile3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfile3 {
    type Vtable = IWMProfile3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00ef96cc_a461_4546_8bcd_c9a28f0e06f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfile3_Vtbl {
    pub base: IWMProfile2_Vtbl,
    pub GetStorageFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT,
    pub SetStorageFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT,
    pub GetBandwidthSharingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows::core::HRESULT,
    pub GetBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateNewBandwidthSharing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psp: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateNewStreamPrioritization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetExpectedPacketCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMProfileManager(::windows::core::IUnknown);
impl IWMProfileManager {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateEmptyProfile)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwversion), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoadProfileByID)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprofile), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn LoadProfileByData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprofile: Param0) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoadProfileByData)(::core::mem::transmute_copy(self), pwszprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SaveProfile<'a, Param0: ::windows::core::IntoParam<'a, IWMProfile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, piwmprofile: Param0, pwszprofile: Param1, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveProfile)(::core::mem::transmute_copy(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), ::core::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSystemProfileCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).LoadSystemProfile)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofileindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
}
impl ::core::convert::From<IWMProfileManager> for ::windows::core::IUnknown {
    fn from(value: IWMProfileManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManager> for ::windows::core::IUnknown {
    fn from(value: &IWMProfileManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfileManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMProfileManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfileManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfileManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager {}
impl ::core::fmt::Debug for IWMProfileManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfileManager {
    type Vtable = IWMProfileManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd16679f2_6ca0_472d_8d31_2f5d55aee155);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateEmptyProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LoadProfileByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LoadProfileByData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprofile: ::windows::core::PCWSTR, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SaveProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmprofile: ::windows::core::RawPtr, pwszprofile: ::windows::core::PCWSTR, pdwlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetSystemProfileCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows::core::HRESULT,
    pub LoadSystemProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMProfileManager2(::windows::core::IUnknown);
impl IWMProfileManager2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateEmptyProfile)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwversion), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LoadProfileByID)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprofile), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn LoadProfileByData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprofile: Param0) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LoadProfileByData)(::core::mem::transmute_copy(self), pwszprofile.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SaveProfile<'a, Param0: ::windows::core::IntoParam<'a, IWMProfile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, piwmprofile: Param0, pwszprofile: Param1, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SaveProfile)(::core::mem::transmute_copy(self), piwmprofile.into_param().abi(), pwszprofile.into_param().abi(), ::core::mem::transmute(pdwlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSystemProfileCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.LoadSystemProfile)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprofileindex), ::core::mem::transmute(&mut result__)).from_abi::<IWMProfile>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSystemProfileVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwversion)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSystemProfileVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwversion)).ok()
    }
}
impl ::core::convert::From<IWMProfileManager2> for ::windows::core::IUnknown {
    fn from(value: IWMProfileManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManager2> for ::windows::core::IUnknown {
    fn from(value: &IWMProfileManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfileManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMProfileManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMProfileManager2> for IWMProfileManager {
    fn from(value: IWMProfileManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManager2> for IWMProfileManager {
    fn from(value: &IWMProfileManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfileManager> for IWMProfileManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfileManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMProfileManager> for &'a IWMProfileManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMProfileManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfileManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfileManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager2 {}
impl ::core::fmt::Debug for IWMProfileManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfileManager2 {
    type Vtable = IWMProfileManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a924e51_73c1_494d_8019_23d37ed9b89a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManager2_Vtbl {
    pub base: IWMProfileManager_Vtbl,
    pub GetSystemProfileVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT,
    pub SetSystemProfileVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMProfileManagerLanguage(::windows::core::IUnknown);
impl IWMProfileManagerLanguage {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetUserLanguageID(&self, wlangid: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUserLanguageID)(::core::mem::transmute_copy(self), ::core::mem::transmute(wlangid)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetUserLanguageID(&self, wlangid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUserLanguageID)(::core::mem::transmute_copy(self), ::core::mem::transmute(wlangid)).ok()
    }
}
impl ::core::convert::From<IWMProfileManagerLanguage> for ::windows::core::IUnknown {
    fn from(value: IWMProfileManagerLanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProfileManagerLanguage> for ::windows::core::IUnknown {
    fn from(value: &IWMProfileManagerLanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProfileManagerLanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMProfileManagerLanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProfileManagerLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProfileManagerLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManagerLanguage {}
impl ::core::fmt::Debug for IWMProfileManagerLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManagerLanguage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProfileManagerLanguage {
    type Vtable = IWMProfileManagerLanguage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba4dcc78_7ee0_4ab8_b27a_dbce8bc51454);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProfileManagerLanguage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetUserLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows::core::HRESULT,
    pub SetUserLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMPropertyVault(::windows::core::IUnknown);
impl IWMPropertyVault {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPropertyCount(&self, pdwcount: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwcount)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPropertyByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszname: Param0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyByName)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszname: Param0, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::core::mem::transmute_copy(self), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(dwsize)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPropertyByIndex(&self, dwindex: u32, pszname: ::windows::core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyByIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pszname), ::core::mem::transmute(pdwnamelen), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pdwsize)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CopyPropertiesFrom<'a, Param0: ::windows::core::IntoParam<'a, IWMPropertyVault>>(&self, piwmpropertyvault: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyPropertiesFrom)(::core::mem::transmute_copy(self), piwmpropertyvault.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMPropertyVault> for ::windows::core::IUnknown {
    fn from(value: IWMPropertyVault) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMPropertyVault> for ::windows::core::IUnknown {
    fn from(value: &IWMPropertyVault) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMPropertyVault {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMPropertyVault {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMPropertyVault {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMPropertyVault {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPropertyVault {}
impl ::core::fmt::Debug for IWMPropertyVault {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPropertyVault").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMPropertyVault {
    type Vtable = IWMPropertyVault_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72995a79_5090_42a4_9c8c_d9d0b6d34be5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPropertyVault_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows::core::HRESULT,
    pub GetPropertyByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT,
    pub GetPropertyByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: ::windows::core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT,
    pub CopyPropertiesFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piwmpropertyvault: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMProximityDetection(::windows::core::IUnknown);
impl IWMProximityDetection {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartDetection<'a, Param6: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pbregistrationmsg: &[u8], pblocaladdress: &[u8], dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: Param6, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartDetection)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbregistrationmsg)), pbregistrationmsg.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pblocaladdress)), pblocaladdress.len() as _, ::core::mem::transmute(dwextraportsallowed), ::core::mem::transmute(ppregistrationresponsemsg), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMProximityDetection> for ::windows::core::IUnknown {
    fn from(value: IWMProximityDetection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMProximityDetection> for ::windows::core::IUnknown {
    fn from(value: &IWMProximityDetection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMProximityDetection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMProximityDetection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMProximityDetection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMProximityDetection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProximityDetection {}
impl ::core::fmt::Debug for IWMProximityDetection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProximityDetection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMProximityDetection {
    type Vtable = IWMProximityDetection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a9fd8ee_b651_4bf0_b849_7d4ece79a2b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMProximityDetection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub StartDetection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReader(::windows::core::IUnknown);
impl IWMReader {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pwszurl: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnumber: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputFormatCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnumber), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputFormat(&self, dwoutputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnumber), ::core::mem::transmute(dwformatnumber), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Start(&self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMReader> for ::windows::core::IUnknown {
    fn from(value: IWMReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReader> for ::windows::core::IUnknown {
    fn from(value: &IWMReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReader {}
impl ::core::fmt::Debug for IWMReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReader {
    type Vtable = IWMReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd6_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReader_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderAccelerator(::windows::core::IUnknown);
impl IWMReaderAccelerator {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCodecInterface(&self, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCodecInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(riid), ::core::mem::transmute(ppvcodecinterface)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Notify(&self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Notify)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(psubtype)).ok()
    }
}
impl ::core::convert::From<IWMReaderAccelerator> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAccelerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAccelerator> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAccelerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderAccelerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAccelerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAccelerator {}
impl ::core::fmt::Debug for IWMReaderAccelerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAccelerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAccelerator {
    type Vtable = IWMReaderAccelerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbddc4d08_944d_4d52_a612_46c3fda07dd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAccelerator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCodecInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Notify: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced(::windows::core::IUnknown);
impl IWMReaderAdvanced {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUserProvidedClock)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUserProvidedClock)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeliverTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetManualStreamSelection)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetManualStreamSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamsSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStreamSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClientInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxOutputSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxStreamSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyLateDelivery)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced {}
impl ::core::fmt::Debug for IWMReaderAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced {
    type Vtable = IWMReaderAdvanced_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bea_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserProvidedClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserProvidedClock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserProvidedClock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserProvidedClock: usize,
    pub DeliverTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetManualStreamSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetManualStreamSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetManualStreamSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetManualStreamSelection: usize,
    pub SetStreamsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceiveSelectionCallbacks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceiveSelectionCallbacks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReceiveSelectionCallbacks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReceiveSelectionCallbacks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceiveStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceiveStreamSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReceiveStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReceiveStreamSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateForOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateForOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateForStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateForStream: usize,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientInfo: usize,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub NotifyLateDelivery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced2(::windows::core::IUnknown);
impl IWMReaderAdvanced2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetUserProvidedClock)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetUserProvidedClock)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DeliverTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetManualStreamSelection)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetManualStreamSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetStreamsSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStreamSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClientInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaxOutputSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaxStreamSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.NotifyLateDelivery)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDownloadProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSaveAsProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveFileAs)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProtocolName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartAtMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Preroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogClientID)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLogClientID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopBuffering)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenStream)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced2> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced2> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced2> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced2> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced2 {}
impl ::core::fmt::Debug for IWMReaderAdvanced2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced2 {
    type Vtable = IWMReaderAdvanced2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae14a945_b90c_4d0d_9127_80d665f7d73e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced2_Vtbl {
    pub base: IWMReaderAdvanced_Vtbl,
    pub SetPlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub GetPlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT,
    pub GetBufferProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT,
    pub GetDownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT,
    pub GetSaveAsProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT,
    pub SaveFileAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetProtocolName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT,
    pub StartAtMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub Preroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogClientID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogClientID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLogClientID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLogClientID: usize,
    pub StopBuffering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced3(::windows::core::IUnknown);
impl IWMReaderAdvanced3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetUserProvidedClock)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetUserProvidedClock)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.DeliverTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetManualStreamSelection)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetManualStreamSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetStreamsSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStreamSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetClientInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMaxOutputSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMaxStreamSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.NotifyLateDelivery)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetBufferProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDownloadProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSaveAsProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SaveFileAs)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetProtocolName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StartAtMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Preroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLogClientID)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLogClientID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StopBuffering)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OpenStream)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopNetStreaming)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartAtPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced3> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced3> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced3> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced3> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced3> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced3> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for &'a IWMReaderAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced3 {}
impl ::core::fmt::Debug for IWMReaderAdvanced3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced3 {
    type Vtable = IWMReaderAdvanced3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5dc0674b_f04b_4a4e_9f2a_b1afde2c8100);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced3_Vtbl {
    pub base: IWMReaderAdvanced2_Vtbl,
    pub StopNetStreaming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced4(::windows::core::IUnknown);
impl IWMReaderAdvanced4 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetUserProvidedClock)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetUserProvidedClock)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.DeliverTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetManualStreamSelection)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetManualStreamSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetStreamsSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetStreamSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetClientInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetMaxOutputSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetMaxStreamSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.NotifyLateDelivery)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetBufferProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDownloadProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetSaveAsProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SaveFileAs)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetProtocolName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.StartAtMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Preroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetLogClientID)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetLogClientID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.StopBuffering)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.OpenStream)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StopNetStreaming)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.StartAtPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLanguageCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(wlanguage), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxSpeedFactor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsUsingFastCache)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddLogParam<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddLogParam)(::core::mem::transmute_copy(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendLogParams)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CanSaveFileAs)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelSaveFileAs)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetURL)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for &'a IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced4> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced4> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for &'a IWMReaderAdvanced4 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced4 {}
impl ::core::fmt::Debug for IWMReaderAdvanced4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced4 {
    type Vtable = IWMReaderAdvanced4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x945a76a2_12ae_4d48_bd3c_cd1d90399b85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced4_Vtbl {
    pub base: IWMReaderAdvanced3_Vtbl,
    pub GetLanguageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    pub GetMaxSpeedFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUsingFastCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUsingFastCache: usize,
    pub AddLogParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wsznamespace: ::windows::core::PCWSTR, wszname: ::windows::core::PCWSTR, wszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SendLogParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanSaveFileAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanSaveFileAs: usize,
    pub CancelSaveFileAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced5(::windows::core::IUnknown);
impl IWMReaderAdvanced5 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetUserProvidedClock)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetUserProvidedClock)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.DeliverTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetManualStreamSelection)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetManualStreamSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetStreamsSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetStreamSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetClientInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetMaxOutputSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetMaxStreamSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.NotifyLateDelivery)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetBufferProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetDownloadProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetSaveAsProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SaveFileAs)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetProtocolName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.StartAtMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.GetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.Preroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetLogClientID)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.GetLogClientID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.StopBuffering)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.OpenStream)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.StopNetStreaming)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.StartAtPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLanguageCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(wlanguage), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaxSpeedFactor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsUsingFastCache)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddLogParam<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddLogParam)(::core::mem::transmute_copy(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SendLogParams)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CanSaveFileAs)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CancelSaveFileAs)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetURL)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPlayerHook<'a, Param1: ::windows::core::IntoParam<'a, IWMPlayerHook>>(&self, dwoutputnum: u32, phook: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlayerHook)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), phook.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced5> for IWMReaderAdvanced4 {
    fn from(value: IWMReaderAdvanced5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced5> for IWMReaderAdvanced4 {
    fn from(value: &IWMReaderAdvanced5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced4> for IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced4> for &'a IWMReaderAdvanced5 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced5 {}
impl ::core::fmt::Debug for IWMReaderAdvanced5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced5 {
    type Vtable = IWMReaderAdvanced5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24c44db0_55d1_49ae_a5cc_f13815e36363);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced5_Vtbl {
    pub base: IWMReaderAdvanced4_Vtbl,
    pub SetPlayerHook: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderAdvanced6(::windows::core::IUnknown);
impl IWMReaderAdvanced6 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fuserclock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetUserProvidedClock)(::core::mem::transmute_copy(self), fuserclock.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetUserProvidedClock)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.DeliverTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnstime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fselection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetManualStreamSelection)(::core::mem::transmute_copy(self), fselection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetManualStreamSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetStreamsSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetStreamSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fgetcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), fgetcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetReceiveSelectionCallbacks)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivestreamsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivestreamsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetReceiveStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwoutputnum: u32, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatistics)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.SetClientInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclientinfo)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetMaxOutputSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.base.GetMaxStreamSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.base.NotifyLateDelivery)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnslateness)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__: WMT_PLAY_MODE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetPlayMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PLAY_MODE>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetBufferProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pcnsbuffering)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetDownloadProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwpercent), ::core::mem::transmute(pqwbytesdownloaded), ::core::mem::transmute(pcnsdownload)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetSaveAsProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SaveFileAs<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SaveFileAs)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetProtocolName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszprotocol), ::core::mem::transmute(pcchprotocol)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.StartAtMarker)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmarkerindex), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.GetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.Preroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstart), ::core::mem::transmute(cnsduration), ::core::mem::transmute(frate)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flogclientid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.SetLogClientID)(::core::mem::transmute_copy(self), flogclientid.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.base.GetLogClientID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.StopBuffering)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>, Param1: ::windows::core::IntoParam<'a, IWMReaderCallback>>(&self, pstream: Param0, pcallback: Param1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.base.OpenStream)(::core::mem::transmute_copy(self), pstream.into_param().abi(), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.StopNetStreaming)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.StartAtPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pvoffsetstart), ::core::mem::transmute(pvduration), ::core::mem::transmute(dwoffsetformat), ::core::mem::transmute(frate), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetLanguageCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(wlanguage), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__: f64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMaxSpeedFactor)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.IsUsingFastCache)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddLogParam<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wsznamespace: Param0, wszname: Param1, wszvalue: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddLogParam)(::core::mem::transmute_copy(self), wsznamespace.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SendLogParams)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.CanSaveFileAs)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.CancelSaveFileAs)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetURL)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPlayerHook<'a, Param1: ::windows::core::IntoParam<'a, IWMPlayerHook>>(&self, dwoutputnum: u32, phook: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetPlayerHook)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), phook.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProtectStreamSamples(&self, pbcertificate: &[u8], dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProtectStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbcertificate)), pbcertificate.len() as _, ::core::mem::transmute(dwcertificatetype), ::core::mem::transmute(dwflags), ::core::mem::transmute(pbinitializationvector), ::core::mem::transmute(pcbinitializationvector)).ok()
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced2 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced2 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced2> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced3 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced3 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced3> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced3> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced4 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced4 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced4> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced4> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced4> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced4> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderAdvanced6> for IWMReaderAdvanced5 {
    fn from(value: IWMReaderAdvanced6) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAdvanced6> for IWMReaderAdvanced5 {
    fn from(value: &IWMReaderAdvanced6) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced5> for IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced5> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderAdvanced5> for &'a IWMReaderAdvanced6 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderAdvanced5> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAdvanced6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced6 {}
impl ::core::fmt::Debug for IWMReaderAdvanced6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAdvanced6 {
    type Vtable = IWMReaderAdvanced6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18a2e7f8_428f_4acd_8a00_e64639bc93de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAdvanced6_Vtbl {
    pub base: IWMReaderAdvanced5_Vtbl,
    pub SetProtectStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderAllocatorEx(::windows::core::IUnknown);
impl IWMReaderAllocatorEx {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateForStreamEx(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForStreamEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(dwflags), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateForOutputEx(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForOutputEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(dwflags), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderAllocatorEx> for ::windows::core::IUnknown {
    fn from(value: IWMReaderAllocatorEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderAllocatorEx> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderAllocatorEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderAllocatorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderAllocatorEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderAllocatorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderAllocatorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAllocatorEx {}
impl ::core::fmt::Debug for IWMReaderAllocatorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAllocatorEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderAllocatorEx {
    type Vtable = IWMReaderAllocatorEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f762fa7_a22e_428d_93c9_ac82f3aafe5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderAllocatorEx_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AllocateForStreamEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateForOutputEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderCallback(::windows::core::IUnknown);
impl IWMReaderCallback {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(status), ::core::mem::transmute(hr), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnSample<'a, Param4: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderCallback> for ::windows::core::IUnknown {
    fn from(value: IWMReaderCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderCallback> for IWMStatusCallback {
    fn from(value: IWMReaderCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderCallback> for IWMStatusCallback {
    fn from(value: &IWMReaderCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStatusCallback> for IWMReaderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStatusCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStatusCallback> for &'a IWMReaderCallback {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStatusCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallback {}
impl ::core::fmt::Debug for IWMReaderCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderCallback {
    type Vtable = IWMReaderCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd8_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallback_Vtbl {
    pub base: IWMStatusCallback_Vtbl,
    pub OnSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderCallbackAdvanced(::windows::core::IUnknown);
impl IWMReaderCallbackAdvanced {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnStreamSample<'a, Param4: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStreamSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnTime(&self, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnscurrenttime), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnStreamSelection(&self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStreamSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamcount), ::core::mem::transmute(pstreamnumbers), ::core::mem::transmute(pselections), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnOutputPropsChanged(&self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnOutputPropsChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(pmediatype), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateForStream(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateForOutput(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMReaderCallbackAdvanced> for ::windows::core::IUnknown {
    fn from(value: IWMReaderCallbackAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderCallbackAdvanced> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderCallbackAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderCallbackAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderCallbackAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderCallbackAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderCallbackAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallbackAdvanced {}
impl ::core::fmt::Debug for IWMReaderCallbackAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderCallbackAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderCallbackAdvanced {
    type Vtable = IWMReaderCallbackAdvanced_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406beb_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderCallbackAdvanced_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnStreamSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnStreamSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnOutputPropsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnOutputPropsChanged: usize,
    pub AllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderNetworkConfig(::windows::core::IUnknown);
impl IWMReaderNetworkConfig {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferingTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBufferingTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferingTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsbufferingtime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUDPPortRanges)(::core::mem::transmute_copy(self), ::core::mem::transmute(prangearray), ::core::mem::transmute(pcranges)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetUDPPortRanges(&self, prangearray: &[WM_PORT_NUMBER_RANGE]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUDPPortRanges)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(prangearray)), prangearray.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProxySettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<WMT_PROXY_SETTINGS> {
        let mut result__: WMT_PROXY_SETTINGS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProxySettings)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PROXY_SETTINGS>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProxySettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProxySettings)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(proxysetting)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProxyHostName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, pwszhostname: ::windows::core::PWSTR, pcchhostname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProxyHostName)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszhostname), ::core::mem::transmute(pcchhostname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProxyHostName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, pwszhostname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProxyHostName)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProxyPort<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProxyPort)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProxyPort<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, dwport: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProxyPort)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(dwport)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProxyExceptionList<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: ::windows::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProxyExceptionList)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszexceptionlist), ::core::mem::transmute(pcchexceptionlist)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProxyExceptionList<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProxyExceptionList)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyBypassForLocal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProxyBypassForLocal)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyBypassForLocal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszprotocol: Param0, fbypassforlocal: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProxyBypassForLocal)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetForceRerunAutoProxyDetection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetForceRerunAutoProxyDetection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforcererundetection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetForceRerunAutoProxyDetection)(::core::mem::transmute_copy(self), fforcererundetection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableMulticast(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnableMulticast)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableMulticast<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablemulticast: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableMulticast)(::core::mem::transmute_copy(self), fenablemulticast.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableHTTP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnableHTTP)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableHTTP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablehttp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableHTTP)(::core::mem::transmute_copy(self), fenablehttp.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableUDP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnableUDP)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableUDP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenableudp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableUDP)(::core::mem::transmute_copy(self), fenableudp.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableTCP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnableTCP)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableTCP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenabletcp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableTCP)(::core::mem::transmute_copy(self), fenabletcp.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetProtocolRollover)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectionBandwidth)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConnectionBandwidth)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnectionbandwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNumProtocolsSupported)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSupportedProtocolName)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprotocolnum), ::core::mem::transmute(pwszprotocolname), ::core::mem::transmute(pcchprotocolname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddLoggingUrl<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszurl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddLoggingUrl)(::core::mem::transmute_copy(self), pwszurl.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLoggingUrl)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLoggingUrlCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetLoggingUrlList)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMReaderNetworkConfig> for ::windows::core::IUnknown {
    fn from(value: IWMReaderNetworkConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderNetworkConfig> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderNetworkConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderNetworkConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderNetworkConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderNetworkConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig {}
impl ::core::fmt::Debug for IWMReaderNetworkConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderNetworkConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderNetworkConfig {
    type Vtable = IWMReaderNetworkConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bec_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetBufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows::core::HRESULT,
    pub SetBufferingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows::core::HRESULT,
    pub GetUDPPortRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::HRESULT,
    pub SetUDPPortRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::HRESULT,
    pub GetProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT,
    pub SetProxySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT,
    pub GetProxyHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszhostname: ::windows::core::PWSTR, pcchhostname: *mut u32) -> ::windows::core::HRESULT,
    pub SetProxyHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszhostname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pdwport: *mut u32) -> ::windows::core::HRESULT,
    pub SetProxyPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, dwport: u32) -> ::windows::core::HRESULT,
    pub GetProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszexceptionlist: ::windows::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::HRESULT,
    pub SetProxyExceptionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszexceptionlist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProxyBypassForLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProxyBypassForLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProxyBypassForLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForceRerunAutoProxyDetection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForceRerunAutoProxyDetection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetForceRerunAutoProxyDetection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetForceRerunAutoProxyDetection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableMulticast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableMulticast: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableMulticast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableMulticast: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableHTTP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableHTTP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableHTTP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableHTTP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableUDP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableUDP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableUDP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableUDP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableTCP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableTCP: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableTCP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableTCP: usize,
    pub ResetProtocolRollover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConnectionBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows::core::HRESULT,
    pub SetConnectionBandwidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows::core::HRESULT,
    pub GetNumProtocolsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT,
    pub GetSupportedProtocolName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::HRESULT,
    pub AddLoggingUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetLoggingUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    pub GetLoggingUrlCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows::core::HRESULT,
    pub ResetLoggingUrlList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderNetworkConfig2(::windows::core::IUnknown);
impl IWMReaderNetworkConfig2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferingTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBufferingTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBufferingTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsbufferingtime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetUDPPortRanges)(::core::mem::transmute_copy(self), ::core::mem::transmute(prangearray), ::core::mem::transmute(pcranges)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetUDPPortRanges(&self, prangearray: &[WM_PORT_NUMBER_RANGE]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetUDPPortRanges)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(prangearray)), prangearray.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProxySettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<WMT_PROXY_SETTINGS> {
        let mut result__: WMT_PROXY_SETTINGS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetProxySettings)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<WMT_PROXY_SETTINGS>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProxySettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetProxySettings)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(proxysetting)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProxyHostName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, pwszhostname: ::windows::core::PWSTR, pcchhostname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetProxyHostName)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszhostname), ::core::mem::transmute(pcchhostname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProxyHostName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, pwszhostname: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetProxyHostName)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszhostname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProxyPort<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetProxyPort)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProxyPort<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, dwport: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetProxyPort)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(dwport)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetProxyExceptionList<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: ::windows::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetProxyExceptionList)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(pwszexceptionlist), ::core::mem::transmute(pcchexceptionlist)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProxyExceptionList<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0, pwszexceptionlist: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetProxyExceptionList)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), pwszexceptionlist.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyBypassForLocal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszprotocol: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetProxyBypassForLocal)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyBypassForLocal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszprotocol: Param0, fbypassforlocal: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetProxyBypassForLocal)(::core::mem::transmute_copy(self), pwszprotocol.into_param().abi(), fbypassforlocal.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetForceRerunAutoProxyDetection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetForceRerunAutoProxyDetection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforcererundetection: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetForceRerunAutoProxyDetection)(::core::mem::transmute_copy(self), fforcererundetection.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableMulticast(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetEnableMulticast)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableMulticast<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablemulticast: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEnableMulticast)(::core::mem::transmute_copy(self), fenablemulticast.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableHTTP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetEnableHTTP)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableHTTP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablehttp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEnableHTTP)(::core::mem::transmute_copy(self), fenablehttp.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableUDP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetEnableUDP)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableUDP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenableudp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEnableUDP)(::core::mem::transmute_copy(self), fenableudp.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableTCP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetEnableTCP)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableTCP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenabletcp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEnableTCP)(::core::mem::transmute_copy(self), fenabletcp.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ResetProtocolRollover)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetConnectionBandwidth)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetConnectionBandwidth)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnectionbandwidth)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetNumProtocolsSupported)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetSupportedProtocolName)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprotocolnum), ::core::mem::transmute(pwszprotocolname), ::core::mem::transmute(pcchprotocolname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddLoggingUrl<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszurl: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddLoggingUrl)(::core::mem::transmute_copy(self), pwszurl.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetLoggingUrl)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLoggingUrlCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ResetLoggingUrlList)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableContentCaching(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnableContentCaching)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableContentCaching<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablecontentcaching: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableContentCaching)(::core::mem::transmute_copy(self), fenablecontentcaching.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnableFastCache)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableFastCache<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablefastcache: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableFastCache)(::core::mem::transmute_copy(self), fenablefastcache.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAcceleratedStreamingDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAcceleratedStreamingDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAcceleratedStreamingDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsaccelduration)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAutoReconnectLimit(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAutoReconnectLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAutoReconnectLimit)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwautoreconnectlimit)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableResends(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnableResends)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableResends<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenableresends: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableResends)(::core::mem::transmute_copy(self), fenableresends.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableThinning(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnableThinning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableThinning<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenablethinning: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnableThinning)(::core::mem::transmute_copy(self), fenablethinning.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxNetPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxNetPacketSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWMReaderNetworkConfig2> for ::windows::core::IUnknown {
    fn from(value: IWMReaderNetworkConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderNetworkConfig2> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderNetworkConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMReaderNetworkConfig2> for IWMReaderNetworkConfig {
    fn from(value: IWMReaderNetworkConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderNetworkConfig2> for IWMReaderNetworkConfig {
    fn from(value: &IWMReaderNetworkConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderNetworkConfig> for IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderNetworkConfig> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMReaderNetworkConfig> for &'a IWMReaderNetworkConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMReaderNetworkConfig> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderNetworkConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig2 {}
impl ::core::fmt::Debug for IWMReaderNetworkConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderNetworkConfig2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderNetworkConfig2 {
    type Vtable = IWMReaderNetworkConfig2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd979a853_042b_4050_8387_c939db22013f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderNetworkConfig2_Vtbl {
    pub base: IWMReaderNetworkConfig_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableContentCaching: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableContentCaching: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableContentCaching: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableContentCaching: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableFastCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableFastCache: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableFastCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableFastCache: usize,
    pub GetAcceleratedStreamingDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows::core::HRESULT,
    pub SetAcceleratedStreamingDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows::core::HRESULT,
    pub GetAutoReconnectLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows::core::HRESULT,
    pub SetAutoReconnectLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableResends: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableResends: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableResends: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableResends: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnableThinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnableThinning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableThinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableThinning: usize,
    pub GetMaxNetPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderPlaylistBurn(::windows::core::IUnknown);
impl IWMReaderPlaylistBurn {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn InitPlaylistBurn<'a, Param2: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, cfiles: u32, ppwszfilenames: *const ::windows::core::PWSTR, pcallback: Param2, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitPlaylistBurn)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfiles), ::core::mem::transmute(ppwszfilenames), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetInitResults(&self, cfiles: u32) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInitResults)(::core::mem::transmute_copy(self), ::core::mem::transmute(cfiles), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn EndPlaylistBurn(&self, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndPlaylistBurn)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrburnresult)).ok()
    }
}
impl ::core::convert::From<IWMReaderPlaylistBurn> for ::windows::core::IUnknown {
    fn from(value: IWMReaderPlaylistBurn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderPlaylistBurn> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderPlaylistBurn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderPlaylistBurn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderPlaylistBurn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderPlaylistBurn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderPlaylistBurn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderPlaylistBurn {}
impl ::core::fmt::Debug for IWMReaderPlaylistBurn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderPlaylistBurn").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderPlaylistBurn {
    type Vtable = IWMReaderPlaylistBurn_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf28c0300_9baa_4477_a846_1744d9cbf533);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderPlaylistBurn_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub InitPlaylistBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const ::windows::core::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInitResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndPlaylistBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderStreamClock(::windows::core::IUnknown);
impl IWMReaderStreamClock {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetTime(&self, pcnsnow: *const u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcnsnow)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetTimer(&self, cnswhen: u64, pvparam: *const ::core::ffi::c_void) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetTimer)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnswhen), ::core::mem::transmute(pvparam), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn KillTimer(&self, dwtimerid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KillTimer)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtimerid)).ok()
    }
}
impl ::core::convert::From<IWMReaderStreamClock> for ::windows::core::IUnknown {
    fn from(value: IWMReaderStreamClock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderStreamClock> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderStreamClock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderStreamClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderStreamClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderStreamClock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderStreamClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderStreamClock {}
impl ::core::fmt::Debug for IWMReaderStreamClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderStreamClock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderStreamClock {
    type Vtable = IWMReaderStreamClock_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bed_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderStreamClock_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows::core::HRESULT,
    pub SetTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows::core::HRESULT,
    pub KillTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderTimecode(::windows::core::IUnknown);
impl IWMReaderTimecode {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetTimecodeRangeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTimecodeRangeCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetTimecodeRangeBounds(&self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTimecodeRangeBounds)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(wrangenum), ::core::mem::transmute(pstarttimecode), ::core::mem::transmute(pendtimecode)).ok()
    }
}
impl ::core::convert::From<IWMReaderTimecode> for ::windows::core::IUnknown {
    fn from(value: IWMReaderTimecode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderTimecode> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderTimecode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderTimecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderTimecode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderTimecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderTimecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTimecode {}
impl ::core::fmt::Debug for IWMReaderTimecode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderTimecode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderTimecode {
    type Vtable = IWMReaderTimecode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf369e2f0_e081_4fe6_8450_b810b2f410d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTimecode_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetTimecodeRangeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows::core::HRESULT,
    pub GetTimecodeRangeBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMReaderTypeNegotiation(::windows::core::IUnknown);
impl IWMReaderTypeNegotiation {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn TryOutputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TryOutputProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMReaderTypeNegotiation> for ::windows::core::IUnknown {
    fn from(value: IWMReaderTypeNegotiation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMReaderTypeNegotiation> for ::windows::core::IUnknown {
    fn from(value: &IWMReaderTypeNegotiation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMReaderTypeNegotiation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMReaderTypeNegotiation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMReaderTypeNegotiation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMReaderTypeNegotiation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTypeNegotiation {}
impl ::core::fmt::Debug for IWMReaderTypeNegotiation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderTypeNegotiation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMReaderTypeNegotiation {
    type Vtable = IWMReaderTypeNegotiation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdbe5592_81a1_41ea_93bd_735cad1adc05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMReaderTypeNegotiation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub TryOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMRegisterCallback(::windows::core::IUnknown);
impl IWMRegisterCallback {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pcallback: Param0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Advise)(::core::mem::transmute_copy(self), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Unadvise<'a, Param0: ::windows::core::IntoParam<'a, IWMStatusCallback>>(&self, pcallback: Param0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::core::mem::transmute_copy(self), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMRegisterCallback> for ::windows::core::IUnknown {
    fn from(value: IWMRegisterCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMRegisterCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMRegisterCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMRegisterCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMRegisterCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMRegisterCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMRegisterCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisterCallback {}
impl ::core::fmt::Debug for IWMRegisterCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMRegisterCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMRegisterCallback {
    type Vtable = IWMRegisterCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf4b1f99_4de2_4e49_a363_252740d99bc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisterCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMRegisteredDevice(::windows::core::IUnknown);
impl IWMRegisteredDevice {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDeviceSerialNumber(&self) -> ::windows::core::Result<DRM_VAL16> {
        let mut result__: DRM_VAL16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceSerialNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DRM_VAL16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDeviceCertificate(&self) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceCertificate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDeviceType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAttributeCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttributeCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByIndex(&self, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeByIndex)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttributeByName)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributeByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstrvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributeByName)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), bstrvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Approve<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fapprove: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Approve)(::core::mem::transmute_copy(self), fapprove.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsValid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsApproved(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsApproved)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWmdrmCompliant(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsWmdrmCompliant)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOpened(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsOpened)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMRegisteredDevice> for ::windows::core::IUnknown {
    fn from(value: IWMRegisteredDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMRegisteredDevice> for ::windows::core::IUnknown {
    fn from(value: &IWMRegisteredDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMRegisteredDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMRegisteredDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMRegisteredDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMRegisteredDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisteredDevice {}
impl ::core::fmt::Debug for IWMRegisteredDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMRegisteredDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMRegisteredDevice {
    type Vtable = IWMRegisteredDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4503bec_5508_4148_97ac_bfa75760a70d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMRegisteredDevice_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDeviceSerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT,
    pub GetDeviceCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcertificate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributeByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributeByIndex: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributeByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttributeByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Approve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Approve: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsValid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsApproved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsApproved: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsWmdrmCompliant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsWmdrmCompliant: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOpened: usize,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMSBufferAllocator(::windows::core::IUnknown);
impl IWMSBufferAllocator {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllocateBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxbuffersize), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllocatePageSizeBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxbuffersize), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
}
impl ::core::convert::From<IWMSBufferAllocator> for ::windows::core::IUnknown {
    fn from(value: IWMSBufferAllocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSBufferAllocator> for ::windows::core::IUnknown {
    fn from(value: &IWMSBufferAllocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSBufferAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMSBufferAllocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSBufferAllocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSBufferAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSBufferAllocator {}
impl ::core::fmt::Debug for IWMSBufferAllocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSBufferAllocator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSBufferAllocator {
    type Vtable = IWMSBufferAllocator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61103ca4_2033_11d2_9ef1_006097d2d7cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSBufferAllocator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AllocateBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AllocatePageSizeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource(::windows::core::IUnknown);
impl IWMSInternalAdminNetSource {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, INSNetSourceCreator>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, psharednamespace: Param0, pnamespacenode: Param1, pnetsourcecreator: Param2, fembeddedinserver: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), psharednamespace.into_param().abi(), pnamespacenode.into_param().abi(), pnetsourcecreator.into_param().abi(), fembeddedinserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNetSourceCreator(&self) -> ::windows::core::Result<INSNetSourceCreator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetSourceCreator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<INSNetSourceCreator>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrname: Param1, bstrpassword: Param2, fpersist: Param3, fconfirmedgood: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCredentials)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrealm: Param0, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCredentials)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrealm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteCredentials)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCredentialFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCredentialFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetCredentialFlags(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCredentialFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURL<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindProxyForURL)(::core::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pdwproxycontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RegisterProxyFailure(&self, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterProxyFailure)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrparam), ::core::mem::transmute(dwproxycontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ShutdownProxyContext(&self, dwproxycontext: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShutdownProxyContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproxycontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingIE(&self, dwproxycontext: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsUsingIE)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproxycontext), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource> for ::windows::core::IUnknown {
    fn from(value: IWMSInternalAdminNetSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource> for ::windows::core::IUnknown {
    fn from(value: &IWMSInternalAdminNetSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSInternalAdminNetSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMSInternalAdminNetSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSInternalAdminNetSource {
    type Vtable = IWMSInternalAdminNetSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb23e5f_d127_4afb_8d02_ae5b66d54c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: ::windows::core::RawPtr, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetNetSourceCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentials: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCredentials: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteCredentials: usize,
    pub GetCredentialFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub SetCredentialFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindProxyForURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindProxyForURL: usize,
    pub RegisterProxyFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::HRESULT,
    pub ShutdownProxyContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUsingIE: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUsingIE: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource2(::windows::core::IUnknown);
impl IWMSInternalAdminNetSource2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, bstrname: Param3, bstrpassword: Param4, fpersist: Param5, fconfirmedgood: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCredentialsEx)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCredentialsEx)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), ::core::mem::transmute(pdwurlpolicy), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteCredentialsEx)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, bstrurl: Param2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindProxyForURLEx)(::core::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pdwproxycontext)).ok()
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource2> for ::windows::core::IUnknown {
    fn from(value: IWMSInternalAdminNetSource2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource2> for ::windows::core::IUnknown {
    fn from(value: &IWMSInternalAdminNetSource2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSInternalAdminNetSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMSInternalAdminNetSource2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource2 {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSInternalAdminNetSource2 {
    type Vtable = IWMSInternalAdminNetSource2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe74d58c3_cf77_4b51_af17_744687c43eae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentialsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentialsEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCredentialsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCredentialsEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteCredentialsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteCredentialsEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FindProxyForURLEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindProxyForURLEx: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMSInternalAdminNetSource3(::windows::core::IUnknown);
impl IWMSInternalAdminNetSource3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, bstrname: Param3, bstrpassword: Param4, fpersist: Param5, fconfirmedgood: Param6) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCredentialsEx)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetCredentialsEx)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), ::core::mem::transmute(pdwurlpolicy), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteCredentialsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DeleteCredentialsEx)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, bstrurl: Param2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.FindProxyForURLEx)(::core::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pdwproxycontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNetSourceCreator2(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetSourceCreator2)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprotocol: Param0, bstrhost: Param1, bstrurl: Param2, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindProxyForURLEx2)(::core::mem::transmute_copy(self), bstrprotocol.into_param().abi(), bstrhost.into_param().abi(), bstrurl.into_param().abi(), ::core::mem::transmute(pfproxyenabled), ::core::mem::transmute(pbstrproxyserver), ::core::mem::transmute(pdwproxyport), ::core::mem::transmute(pqwproxycontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RegisterProxyFailure2(&self, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterProxyFailure2)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrparam), ::core::mem::transmute(qwproxycontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn ShutdownProxyContext2(&self, qwproxycontext: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShutdownProxyContext2)(::core::mem::transmute_copy(self), ::core::mem::transmute(qwproxycontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingIE2(&self, qwproxycontext: u64) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsUsingIE2)(::core::mem::transmute_copy(self), ::core::mem::transmute(qwproxycontext), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(
        &self,
        bstrrealm: Param0,
        bstrurl: Param1,
        fproxy: Param2,
        bstrname: Param3,
        bstrpassword: Param4,
        fpersist: Param5,
        fconfirmedgood: Param6,
        fcleartextauthentication: Param7,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCredentialsEx2)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi(), fpersist.into_param().abi(), fconfirmedgood.into_param().abi(), fcleartextauthentication.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bstrrealm: Param0, bstrurl: Param1, fproxy: Param2, fcleartextauthentication: Param3, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCredentialsEx2)(::core::mem::transmute_copy(self), bstrrealm.into_param().abi(), bstrurl.into_param().abi(), fproxy.into_param().abi(), fcleartextauthentication.into_param().abi(), ::core::mem::transmute(pdwurlpolicy), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), ::core::mem::transmute(pfconfirmedgood)).ok()
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource3> for ::windows::core::IUnknown {
    fn from(value: IWMSInternalAdminNetSource3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource3> for ::windows::core::IUnknown {
    fn from(value: &IWMSInternalAdminNetSource3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMSInternalAdminNetSource3> for IWMSInternalAdminNetSource2 {
    fn from(value: IWMSInternalAdminNetSource3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSInternalAdminNetSource3> for IWMSInternalAdminNetSource2 {
    fn from(value: &IWMSInternalAdminNetSource3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMSInternalAdminNetSource2> for IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMSInternalAdminNetSource2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMSInternalAdminNetSource2> for &'a IWMSInternalAdminNetSource3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMSInternalAdminNetSource2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSInternalAdminNetSource3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource3 {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSInternalAdminNetSource3 {
    type Vtable = IWMSInternalAdminNetSource3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b63d08e_4590_44af_9eb3_57ff1e73bf80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSInternalAdminNetSource3_Vtbl {
    pub base: IWMSInternalAdminNetSource2_Vtbl,
    pub GetNetSourceCreator2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindProxyForURLEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindProxyForURLEx2: usize,
    pub RegisterProxyFailure2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::HRESULT,
    pub ShutdownProxyContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUsingIE2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUsingIE2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCredentialsEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCredentialsEx2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCredentialsEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCredentialsEx2: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMSecureChannel(::windows::core::IUnknown);
impl IWMSecureChannel {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCertCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCertCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCert)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSharedData)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcertindex), ::core::mem::transmute(pbshareddata), ::core::mem::transmute(pbcert), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_AddCertificate<'a, Param0: ::windows::core::IntoParam<'a, IWMAuthorizer>>(&self, pcert: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_AddCertificate)(::core::mem::transmute_copy(self), pcert.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_AddSignature(&self, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_AddSignature)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbcertsig), ::core::mem::transmute(cbcertsig)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_Connect<'a, Param0: ::windows::core::IntoParam<'a, IWMSecureChannel>>(&self, potherside: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Connect)(::core::mem::transmute_copy(self), potherside.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WMSC_IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WMSC_IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Disconnect)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_GetValidCertificate(&self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_GetValidCertificate)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbcertificate), ::core::mem::transmute(pdwsignature)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_Encrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Encrypt)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_Decrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Decrypt)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_Lock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Lock)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_Unlock)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WMSC_SetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WMSC_SetSharedData)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcertindex), ::core::mem::transmute(pbshareddata)).ok()
    }
}
impl ::core::convert::From<IWMSecureChannel> for ::windows::core::IUnknown {
    fn from(value: IWMSecureChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSecureChannel> for ::windows::core::IUnknown {
    fn from(value: &IWMSecureChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMSecureChannel> for IWMAuthorizer {
    fn from(value: IWMSecureChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSecureChannel> for IWMAuthorizer {
    fn from(value: &IWMSecureChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMAuthorizer> for IWMSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, IWMAuthorizer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMAuthorizer> for &'a IWMSecureChannel {
    fn into_param(self) -> ::windows::core::Param<'a, IWMAuthorizer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSecureChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSecureChannel {}
impl ::core::fmt::Debug for IWMSecureChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSecureChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSecureChannel {
    type Vtable = IWMSecureChannel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2720598a_d0f2_4189_bd10_91c46ef0936f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSecureChannel_Vtbl {
    pub base: IWMAuthorizer_Vtbl,
    pub WMSC_AddCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcert: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub WMSC_AddSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::HRESULT,
    pub WMSC_Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, potherside: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WMSC_IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WMSC_IsConnected: usize,
    pub WMSC_Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WMSC_GetValidCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::HRESULT,
    pub WMSC_Encrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT,
    pub WMSC_Decrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT,
    pub WMSC_Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WMSC_Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WMSC_SetSharedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMStatusCallback(::windows::core::IUnknown);
impl IWMStatusCallback {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(status), ::core::mem::transmute(hr), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMStatusCallback> for ::windows::core::IUnknown {
    fn from(value: IWMStatusCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStatusCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMStatusCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStatusCallback {}
impl ::core::fmt::Debug for IWMStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStatusCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStatusCallback {
    type Vtable = IWMStatusCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d7cdc70_9888_11d3_8edc_00c04f6109cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStatusCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMStreamConfig(::windows::core::IUnknown);
impl IWMStreamConfig {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStreamType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStreamNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszstreamname), ::core::mem::transmute(pcchstreamname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszstreamname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamName)(::core::mem::transmute_copy(self), pwszstreamname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConnectionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszinputname), ::core::mem::transmute(pcchinputname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetConnectionName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszinputname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConnectionName)(::core::mem::transmute_copy(self), pwszinputname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBitrate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBitrate)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbitrate)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBufferWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(msbufferwindow)).ok()
    }
}
impl ::core::convert::From<IWMStreamConfig> for ::windows::core::IUnknown {
    fn from(value: IWMStreamConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMStreamConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig {}
impl ::core::fmt::Debug for IWMStreamConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamConfig {
    type Vtable = IWMStreamConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bdc_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetStreamType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetStreamNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub SetStreamNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub GetStreamName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT,
    pub SetStreamName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT,
    pub SetConnectionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinputname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT,
    pub SetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT,
    pub GetBufferWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT,
    pub SetBufferWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMStreamConfig2(::windows::core::IUnknown);
impl IWMStreamConfig2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStreamType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStreamNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetStreamNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetStreamName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszstreamname), ::core::mem::transmute(pcchstreamname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszstreamname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetStreamName)(::core::mem::transmute_copy(self), pwszstreamname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetConnectionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszinputname), ::core::mem::transmute(pcchinputname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetConnectionName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszinputname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetConnectionName)(::core::mem::transmute_copy(self), pwszinputname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBitrate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBitrate)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbitrate)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBufferWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBufferWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(msbufferwindow)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetTransportType(&self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__: WMT_TRANSPORT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTransportType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_TRANSPORT_TYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransportType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntransporttype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddDataUnitExtension<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidextensionsystemid: Param0, cbextensiondatasize: u16, pbextensionsysteminfo: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDataUnitExtension)(::core::mem::transmute_copy(self), guidextensionsystemid.into_param().abi(), ::core::mem::transmute(cbextensiondatasize), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbextensionsysteminfo)), pbextensionsysteminfo.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDataUnitExtensionCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataUnitExtension)(::core::mem::transmute_copy(self), ::core::mem::transmute(wdataunitextensionnumber), ::core::mem::transmute(pguidextensionsystemid), ::core::mem::transmute(pcbextensiondatasize), ::core::mem::transmute(pbextensionsysteminfo), ::core::mem::transmute(pcbextensionsysteminfo)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllDataUnitExtensions)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMStreamConfig2> for ::windows::core::IUnknown {
    fn from(value: IWMStreamConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig2> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMStreamConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMStreamConfig2> for IWMStreamConfig {
    fn from(value: IWMStreamConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig2> for IWMStreamConfig {
    fn from(value: &IWMStreamConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig> for IWMStreamConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig> for &'a IWMStreamConfig2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig2 {}
impl ::core::fmt::Debug for IWMStreamConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamConfig2 {
    type Vtable = IWMStreamConfig2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7688d8cb_fc0d_43bd_9459_5a8dec200cfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig2_Vtbl {
    pub base: IWMStreamConfig_Vtbl,
    pub GetTransportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT,
    pub SetTransportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT,
    pub AddDataUnitExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT,
    pub GetDataUnitExtensionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows::core::HRESULT,
    pub GetDataUnitExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveAllDataUnitExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMStreamConfig3(::windows::core::IUnknown);
impl IWMStreamConfig3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStreamType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStreamNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetStreamNumber)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetStreamName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszstreamname), ::core::mem::transmute(pcchstreamname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszstreamname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetStreamName)(::core::mem::transmute_copy(self), pwszstreamname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetConnectionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszinputname), ::core::mem::transmute(pcchinputname)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetConnectionName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszinputname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetConnectionName)(::core::mem::transmute_copy(self), pwszinputname.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetBitrate)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetBitrate)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwbitrate)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetBufferWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetBufferWindow)(::core::mem::transmute_copy(self), ::core::mem::transmute(msbufferwindow)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetTransportType(&self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__: WMT_TRANSPORT_TYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTransportType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_TRANSPORT_TYPE>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransportType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ntransporttype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddDataUnitExtension<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, guidextensionsystemid: Param0, cbextensiondatasize: u16, pbextensionsysteminfo: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddDataUnitExtension)(::core::mem::transmute_copy(self), guidextensionsystemid.into_param().abi(), ::core::mem::transmute(cbextensiondatasize), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbextensionsysteminfo)), pbextensionsysteminfo.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDataUnitExtensionCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDataUnitExtension)(::core::mem::transmute_copy(self), ::core::mem::transmute(wdataunitextensionnumber), ::core::mem::transmute(pguidextensionsystemid), ::core::mem::transmute(pcbextensiondatasize), ::core::mem::transmute(pbextensionsysteminfo), ::core::mem::transmute(pcbextensionsysteminfo)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveAllDataUnitExtensions)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetLanguage(&self, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLanguage)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszlanguagestring), ::core::mem::transmute(pcchlanguagestringlength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszlanguagestring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLanguage)(::core::mem::transmute_copy(self), pwszlanguagestring.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMStreamConfig3> for ::windows::core::IUnknown {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig3> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMStreamConfig3> for IWMStreamConfig {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig3> for IWMStreamConfig {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig> for &'a IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMStreamConfig3> for IWMStreamConfig2 {
    fn from(value: IWMStreamConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamConfig3> for IWMStreamConfig2 {
    fn from(value: &IWMStreamConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig2> for IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStreamConfig2> for &'a IWMStreamConfig3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStreamConfig2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamConfig3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig3 {}
impl ::core::fmt::Debug for IWMStreamConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamConfig3 {
    type Vtable = IWMStreamConfig3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb164104_3aa9_45a7_9ac9_4daee131d6e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamConfig3_Vtbl {
    pub base: IWMStreamConfig2_Vtbl,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMStreamList(::windows::core::IUnknown);
impl IWMStreamList {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreams)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwstreamnumarray), ::core::mem::transmute(pcstreams)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum)).ok()
    }
}
impl ::core::convert::From<IWMStreamList> for ::windows::core::IUnknown {
    fn from(value: IWMStreamList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamList> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMStreamList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamList {}
impl ::core::fmt::Debug for IWMStreamList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamList {
    type Vtable = IWMStreamList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bdd_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamList_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT,
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMStreamPrioritization(::windows::core::IUnknown);
impl IWMStreamPrioritization {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPriorityRecords)(::core::mem::transmute_copy(self), ::core::mem::transmute(precordarray), ::core::mem::transmute(pcrecords)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriorityRecords)(::core::mem::transmute_copy(self), ::core::mem::transmute(precordarray), ::core::mem::transmute(crecords)).ok()
    }
}
impl ::core::convert::From<IWMStreamPrioritization> for ::windows::core::IUnknown {
    fn from(value: IWMStreamPrioritization) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMStreamPrioritization> for ::windows::core::IUnknown {
    fn from(value: &IWMStreamPrioritization) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMStreamPrioritization {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMStreamPrioritization {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMStreamPrioritization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMStreamPrioritization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamPrioritization {}
impl ::core::fmt::Debug for IWMStreamPrioritization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamPrioritization").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMStreamPrioritization {
    type Vtable = IWMStreamPrioritization_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c1c6090_f9a8_4748_8ec3_dd1108ba1e77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMStreamPrioritization_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPriorityRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPriorityRecords: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPriorityRecords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPriorityRecords: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMSyncReader(::windows::core::IUnknown);
impl IWMSyncReader {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime), ::core::mem::transmute(cnsduration)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRangeByFrame)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(qwframenumber), ::core::mem::transmute(cframestoread)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(ppsample), ::core::mem::transmute(pcnssampletime), ::core::mem::transmute(pcnsduration), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pdwoutputnum), ::core::mem::transmute(pwstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStreamsSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStreamSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReadStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fcompressed: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReadStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fcompressed.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetReadStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputFormatCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(dwformatnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputNumberForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStreamNumberForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxOutputSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxStreamSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenStream)(::core::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMSyncReader> for ::windows::core::IUnknown {
    fn from(value: IWMSyncReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSyncReader> for ::windows::core::IUnknown {
    fn from(value: &IWMSyncReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSyncReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMSyncReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSyncReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSyncReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader {}
impl ::core::fmt::Debug for IWMSyncReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSyncReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSyncReader {
    type Vtable = IWMSyncReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9397f121_7705_4dc9_b049_98b698188414);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::HRESULT,
    pub SetRangeByFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::HRESULT,
    pub GetNextSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut ::windows::core::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub SetStreamsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    pub GetStreamSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReadStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReadStreamSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReadStreamSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReadStreamSamples: usize,
    pub GetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetOutputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOutputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetOutputFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetOutputNumberForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::core::HRESULT,
    pub GetStreamNumberForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT,
    pub GetMaxOutputSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaxStreamSampleSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenStream: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMSyncReader2(::windows::core::IUnknown);
impl IWMSyncReader2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Open)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime), ::core::mem::transmute(cnsduration)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetRangeByFrame)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(qwframenumber), ::core::mem::transmute(cframestoread)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetNextSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(ppsample), ::core::mem::transmute(pcnssampletime), ::core::mem::transmute(pcnsduration), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pdwoutputnum), ::core::mem::transmute(pwstreamnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetStreamsSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(cstreamcount), ::core::mem::transmute(pwstreamnumbers), ::core::mem::transmute(pselections)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__: WMT_STREAM_SELECTION = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStreamSelected)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_STREAM_SELECTION>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReadStreamSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, fcompressed: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetReadStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), fcompressed.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetReadStreamSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwoutputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOutputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOutputCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOutputProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMOutputMediaProps>>(&self, dwoutputnum: u32, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOutputProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOutputFormatCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOutputFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(dwformatnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMOutputMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOutputNumberForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStreamNumberForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaxOutputSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutput), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaxStreamSampleSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstream), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_System_Com'*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstream: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OpenStream)(::core::mem::transmute_copy(self), pstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetRangeByTimecode(&self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRangeByTimecode)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(pstart), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetRangeByFrameEx(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetRangeByFrameEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(qwframenumber), ::core::mem::transmute(cframestoread), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetAllocateForOutput<'a, Param1: ::windows::core::IntoParam<'a, IWMReaderAllocatorEx>>(&self, dwoutputnum: u32, pallocator: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), pallocator.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMReaderAllocatorEx> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllocateForOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMReaderAllocatorEx>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetAllocateForStream<'a, Param1: ::windows::core::IntoParam<'a, IWMReaderAllocatorEx>>(&self, wstreamnum: u16, pallocator: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), pallocator.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<IWMReaderAllocatorEx> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllocateForStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsreamnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMReaderAllocatorEx>(result__)
    }
}
impl ::core::convert::From<IWMSyncReader2> for ::windows::core::IUnknown {
    fn from(value: IWMSyncReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSyncReader2> for ::windows::core::IUnknown {
    fn from(value: &IWMSyncReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMSyncReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMSyncReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMSyncReader2> for IWMSyncReader {
    fn from(value: IWMSyncReader2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMSyncReader2> for IWMSyncReader {
    fn from(value: &IWMSyncReader2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMSyncReader> for IWMSyncReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMSyncReader> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMSyncReader> for &'a IWMSyncReader2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMSyncReader> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMSyncReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMSyncReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader2 {}
impl ::core::fmt::Debug for IWMSyncReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSyncReader2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMSyncReader2 {
    type Vtable = IWMSyncReader2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaed3d21_1b6b_4af7_8cb6_3e189bbc187b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMSyncReader2_Vtbl {
    pub base: IWMSyncReader_Vtbl,
    pub SetRangeByTimecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::HRESULT,
    pub SetRangeByFrameEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows::core::HRESULT,
    pub SetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetAllocateForOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetAllocateForStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMVideoMediaProps(::windows::core::IUnknown);
impl IWMVideoMediaProps {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetMediaType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype), ::core::mem::transmute(pcbtype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetMediaType)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxKeyFrameSpacing(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxKeyFrameSpacing)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetMaxKeyFrameSpacing(&self, lltime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxKeyFrameSpacing)(::core::mem::transmute_copy(self), ::core::mem::transmute(lltime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetQuality(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetQuality)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetQuality(&self, dwquality: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuality)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwquality)).ok()
    }
}
impl ::core::convert::From<IWMVideoMediaProps> for ::windows::core::IUnknown {
    fn from(value: IWMVideoMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMVideoMediaProps> for ::windows::core::IUnknown {
    fn from(value: &IWMVideoMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMVideoMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMVideoMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMVideoMediaProps> for IWMMediaProps {
    fn from(value: IWMVideoMediaProps) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMVideoMediaProps> for IWMMediaProps {
    fn from(value: &IWMVideoMediaProps) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for IWMVideoMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMMediaProps> for &'a IWMVideoMediaProps {
    fn into_param(self) -> ::windows::core::Param<'a, IWMMediaProps> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMVideoMediaProps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMVideoMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMVideoMediaProps {}
impl ::core::fmt::Debug for IWMVideoMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMVideoMediaProps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMVideoMediaProps {
    type Vtable = IWMVideoMediaProps_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bcf_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMVideoMediaProps_Vtbl {
    pub base: IWMMediaProps_Vtbl,
    pub GetMaxKeyFrameSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows::core::HRESULT,
    pub SetMaxKeyFrameSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows::core::HRESULT,
    pub GetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows::core::HRESULT,
    pub SetQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWatermarkInfo(::windows::core::IUnknown);
impl IWMWatermarkInfo {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWatermarkEntryCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmettype), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32) -> ::windows::core::Result<WMT_WATERMARK_ENTRY> {
        let mut result__: WMT_WATERMARK_ENTRY = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWatermarkEntry)(::core::mem::transmute_copy(self), ::core::mem::transmute(wmettype), ::core::mem::transmute(dwentrynum), ::core::mem::transmute(&mut result__)).from_abi::<WMT_WATERMARK_ENTRY>(result__)
    }
}
impl ::core::convert::From<IWMWatermarkInfo> for ::windows::core::IUnknown {
    fn from(value: IWMWatermarkInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWatermarkInfo> for ::windows::core::IUnknown {
    fn from(value: &IWMWatermarkInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWatermarkInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWatermarkInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWatermarkInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWatermarkInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWatermarkInfo {}
impl ::core::fmt::Debug for IWMWatermarkInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWatermarkInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWatermarkInfo {
    type Vtable = IWMWatermarkInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f497062_f2e2_4624_8ea7_9dd40d81fc8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWatermarkInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetWatermarkEntryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetWatermarkEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriter(::windows::core::IUnknown);
impl IWMWriter {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProfileByID)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidprofile)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetProfile<'a, Param0: ::windows::core::IntoParam<'a, IWMProfile>>(&self, pprofile: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProfile)(::core::mem::transmute_copy(self), pprofile.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetOutputFilename<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputFilename)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetInputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetInputProps(&self, dwinputnum: u32) -> ::windows::core::Result<IWMInputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(&mut result__)).from_abi::<IWMInputMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetInputProps<'a, Param1: ::windows::core::IntoParam<'a, IWMInputMediaProps>>(&self, dwinputnum: u32, pinput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInputProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pinput.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetInputFormatCount(&self, dwinputnumber: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputFormatCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnumber), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetInputFormat(&self, dwinputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMInputMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnumber), ::core::mem::transmute(dwformatnumber), ::core::mem::transmute(&mut result__)).from_abi::<IWMInputMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn BeginWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginWriting)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn EndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndWriting)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateSample(&self, dwsamplesize: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllocateSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsamplesize), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WriteSample<'a, Param3: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Flush)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriter> for ::windows::core::IUnknown {
    fn from(value: IWMWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriter> for ::windows::core::IUnknown {
    fn from(value: &IWMWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriter {}
impl ::core::fmt::Debug for IWMWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriter {
    type Vtable = IWMWriter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406bd4_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetProfileByID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOutputFilename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetInputCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetInputProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetInputFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub BeginWriting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndWriting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub WriteSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterAdvanced(::windows::core::IUnknown);
impl IWMWriterAdvanced {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSinkCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSink)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsinknum), ::core::mem::transmute(&mut result__)).from_abi::<IWMWriterSink>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSink)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveSink)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteStreamSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(mssamplesendtime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLiveSource)(::core::mem::transmute_copy(self), fislivesource.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsRealTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWriterTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS> {
        let mut result__: WM_WRITER_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSyncTolerance)(::core::mem::transmute_copy(self), ::core::mem::transmute(mswindow)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncTolerance)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IWMWriterAdvanced> for ::windows::core::IUnknown {
    fn from(value: IWMWriterAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterAdvanced {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced {}
impl ::core::fmt::Debug for IWMWriterAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterAdvanced {
    type Vtable = IWMWriterAdvanced_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be3_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSinkCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT,
    pub GetSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub WriteStreamSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLiveSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLiveSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRealTime: usize,
    pub GetWriterTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT,
    pub SetSyncTolerance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT,
    pub GetSyncTolerance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterAdvanced2(::windows::core::IUnknown);
impl IWMWriterAdvanced2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSinkCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSink)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsinknum), ::core::mem::transmute(&mut result__)).from_abi::<IWMWriterSink>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddSink)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveSink)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.WriteStreamSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(mssamplesendtime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLiveSource)(::core::mem::transmute_copy(self), fislivesource.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsRealTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetWriterTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS> {
        let mut result__: WM_WRITER_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSyncTolerance)(::core::mem::transmute_copy(self), ::core::mem::transmute(mswindow)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSyncTolerance)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetInputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwinputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetInputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwinputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
}
impl ::core::convert::From<IWMWriterAdvanced2> for ::windows::core::IUnknown {
    fn from(value: IWMWriterAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced2> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterAdvanced2> for IWMWriterAdvanced {
    fn from(value: IWMWriterAdvanced2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced2> for IWMWriterAdvanced {
    fn from(value: &IWMWriterAdvanced2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced> for IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced> for &'a IWMWriterAdvanced2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterAdvanced2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced2 {}
impl ::core::fmt::Debug for IWMWriterAdvanced2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterAdvanced2 {
    type Vtable = IWMWriterAdvanced2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x962dc1ec_c046_4db8_9cc7_26ceae500817);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced2_Vtbl {
    pub base: IWMWriterAdvanced_Vtbl,
    pub GetInputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT,
    pub SetInputSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterAdvanced3(::windows::core::IUnknown);
impl IWMWriterAdvanced3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetSinkCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetSink)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsinknum), ::core::mem::transmute(&mut result__)).from_abi::<IWMWriterSink>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AddSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddSink)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn RemoveSink<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterSink>>(&self, psink: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RemoveSink)(::core::mem::transmute_copy(self), psink.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn WriteStreamSample<'a, Param5: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.WriteStreamSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(mssamplesendtime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fislivesource: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetLiveSource)(::core::mem::transmute_copy(self), fislivesource.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.IsRealTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetWriterTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatistics(&self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS> {
        let mut result__: WM_WRITER_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_WRITER_STATISTICS>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetSyncTolerance)(::core::mem::transmute_copy(self), ::core::mem::transmute(mswindow)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetSyncTolerance)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetInputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwinputnum: u32, pszname: Param1, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetInputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(ptype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pcblength)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetInputSetting<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, dwinputnum: u32, pszname: Param1, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInputSetting)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), pszname.into_param().abi(), ::core::mem::transmute(r#type), ::core::mem::transmute(::windows::core::as_ptr_or_null(pvalue)), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetStatisticsEx(&self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS_EX> {
        let mut result__: WM_WRITER_STATISTICS_EX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStatisticsEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<WM_WRITER_STATISTICS_EX>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetNonBlocking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNonBlocking)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterAdvanced3> for ::windows::core::IUnknown {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced3> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterAdvanced3> for IWMWriterAdvanced {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced3> for IWMWriterAdvanced {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced> for &'a IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterAdvanced3> for IWMWriterAdvanced2 {
    fn from(value: IWMWriterAdvanced3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterAdvanced3> for IWMWriterAdvanced2 {
    fn from(value: &IWMWriterAdvanced3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced2> for IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterAdvanced2> for &'a IWMWriterAdvanced3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterAdvanced2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterAdvanced3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced3 {}
impl ::core::fmt::Debug for IWMWriterAdvanced3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterAdvanced3 {
    type Vtable = IWMWriterAdvanced3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd6492d_7c37_4e76_9d3b_59261183a22e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterAdvanced3_Vtbl {
    pub base: IWMWriterAdvanced2_Vtbl,
    pub GetStatisticsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::HRESULT,
    pub SetNonBlocking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterFileSink(::windows::core::IUnknown);
impl IWMWriterFileSink {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnHeader)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsRealTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.AllocateDataUnit)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnDataUnit)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnEndWriting)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWMWriterFileSink> for ::windows::core::IUnknown {
    fn from(value: IWMWriterFileSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterFileSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterFileSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterFileSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink> for IWMWriterSink {
    fn from(value: IWMWriterFileSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterFileSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterFileSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink {}
impl ::core::fmt::Debug for IWMWriterFileSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterFileSink {
    type Vtable = IWMWriterFileSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be5_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink_Vtbl {
    pub base: IWMWriterSink_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterFileSink2(::windows::core::IUnknown);
impl IWMWriterFileSink2 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.OnHeader)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.IsRealTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.AllocateDataUnit)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.OnDataUnit)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.OnEndWriting)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Open)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstoptime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStopped(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsStopped)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetFileDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFileDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFileSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsClosed)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWMWriterFileSink2> for ::windows::core::IUnknown {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink2> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink2> for IWMWriterSink {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink2> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink2> for IWMWriterFileSink {
    fn from(value: IWMWriterFileSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink2> for IWMWriterFileSink {
    fn from(value: &IWMWriterFileSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink> for IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink> for &'a IWMWriterFileSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterFileSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink2 {}
impl ::core::fmt::Debug for IWMWriterFileSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterFileSink2 {
    type Vtable = IWMWriterFileSink2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14282ba7_4aef_4205_8ce5_c229035a05bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink2_Vtbl {
    pub base: IWMWriterFileSink_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStopped: usize,
    pub GetFileDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClosed: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterFileSink3(::windows::core::IUnknown);
impl IWMWriterFileSink3 {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.OnHeader)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.IsRealTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.base.AllocateDataUnit)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.OnDataUnit)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.OnEndWriting)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Open)(::core::mem::transmute_copy(self), pwszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Start)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstarttime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Stop)(::core::mem::transmute_copy(self), ::core::mem::transmute(cnsstoptime)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStopped(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsStopped)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetFileDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFileDuration)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFileSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Close)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsClosed)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoIndexing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fdoautoindexing: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAutoIndexing)(::core::mem::transmute_copy(self), fdoautoindexing.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAutoIndexing(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAutoIndexing)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetControlStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnumber: u16, fshouldcontrolstartandstop: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetControlStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), fshouldcontrolstartandstop.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMode(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDataUnitEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfilesinkdataunit)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnbufferedIO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, funbufferedio: Param0, frestrictmemusage: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUnbufferedIO)(::core::mem::transmute_copy(self), funbufferedio.into_param().abi(), frestrictmemusage.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnbufferedIO(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUnbufferedIO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn CompleteOperations(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompleteOperations)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for ::windows::core::IUnknown {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for IWMWriterSink {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for IWMWriterSink {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for IWMWriterFileSink {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for IWMWriterFileSink {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink> for &'a IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterFileSink3> for IWMWriterFileSink2 {
    fn from(value: IWMWriterFileSink3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterFileSink3> for IWMWriterFileSink2 {
    fn from(value: &IWMWriterFileSink3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink2> for IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterFileSink2> for &'a IWMWriterFileSink3 {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterFileSink2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterFileSink3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink3 {}
impl ::core::fmt::Debug for IWMWriterFileSink3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterFileSink3 {
    type Vtable = IWMWriterFileSink3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fea4feb_2945_47a7_a1dd_c53a8fc4c45c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterFileSink3_Vtbl {
    pub base: IWMWriterFileSink2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAutoIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAutoIndexing: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAutoIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAutoIndexing: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetControlStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetControlStream: usize,
    pub GetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows::core::HRESULT,
    pub OnDataUnitEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUnbufferedIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUnbufferedIO: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUnbufferedIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUnbufferedIO: usize,
    pub CompleteOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterNetworkSink(::windows::core::IUnknown);
impl IWMWriterNetworkSink {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnHeader)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsRealTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.AllocateDataUnit)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnDataUnit)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnEndWriting)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetMaximumClients(&self, dwmaxclients: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaximumClients)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwmaxclients)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaximumClients(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaximumClients)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNetworkProtocol)(::core::mem::transmute_copy(self), ::core::mem::transmute(protocol)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetNetworkProtocol(&self) -> ::windows::core::Result<WMT_NET_PROTOCOL> {
        let mut result__: WMT_NET_PROTOCOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNetworkProtocol)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<WMT_NET_PROTOCOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetHostURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHostURL)(::core::mem::transmute_copy(self), ::core::mem::transmute(pwszurl), ::core::mem::transmute(pcchurl)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Open(&self, pdwportnum: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwportnum)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterNetworkSink> for ::windows::core::IUnknown {
    fn from(value: IWMWriterNetworkSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterNetworkSink> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterNetworkSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterNetworkSink> for IWMWriterSink {
    fn from(value: IWMWriterNetworkSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterNetworkSink> for IWMWriterSink {
    fn from(value: &IWMWriterNetworkSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterNetworkSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterNetworkSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterNetworkSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterNetworkSink {}
impl ::core::fmt::Debug for IWMWriterNetworkSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterNetworkSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterNetworkSink {
    type Vtable = IWMWriterNetworkSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be7_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterNetworkSink_Vtbl {
    pub base: IWMWriterSink_Vtbl,
    pub SetMaximumClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows::core::HRESULT,
    pub GetMaximumClients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows::core::HRESULT,
    pub SetNetworkProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows::core::HRESULT,
    pub GetNetworkProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows::core::HRESULT,
    pub GetHostURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterPostView(::windows::core::IUnknown);
impl IWMWriterPostView {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPostViewCallback<'a, Param0: ::windows::core::IntoParam<'a, IWMWriterPostViewCallback>>(&self, pcallback: Param0, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPostViewCallback)(::core::mem::transmute_copy(self), pcallback.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceivePostViewSamples<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnum: u16, freceivepostviewsamples: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetReceivePostViewSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), freceivepostviewsamples.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceivePostViewSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetReceivePostViewSamples)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPostViewProps(&self, wstreamnumber: u16) -> ::windows::core::Result<IWMMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPostViewProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(&mut result__)).from_abi::<IWMMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetPostViewProps<'a, Param1: ::windows::core::IntoParam<'a, IWMMediaProps>>(&self, wstreamnumber: u16, poutput: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPostViewProps)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), poutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPostViewFormatCount(&self, wstreamnumber: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPostViewFormatCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetPostViewFormat(&self, wstreamnumber: u16, dwformatnumber: u32) -> ::windows::core::Result<IWMMediaProps> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPostViewFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(dwformatnumber), ::core::mem::transmute(&mut result__)).from_abi::<IWMMediaProps>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForPostView<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, wstreamnumber: u16, fallocate: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllocateForPostView)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), fallocate.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForPostView(&self, wstreamnumber: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllocateForPostView)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IWMWriterPostView> for ::windows::core::IUnknown {
    fn from(value: IWMWriterPostView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPostView> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterPostView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterPostView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterPostView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPostView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPostView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostView {}
impl ::core::fmt::Debug for IWMWriterPostView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPostView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterPostView {
    type Vtable = IWMWriterPostView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81e20ce4_75ef_491a_8004_fc53c45bdc3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostView_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetPostViewCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReceivePostViewSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReceivePostViewSamples: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetReceivePostViewSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetReceivePostViewSamples: usize,
    pub GetPostViewProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPostViewProps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPostViewFormatCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows::core::HRESULT,
    pub GetPostViewFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllocateForPostView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllocateForPostView: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllocateForPostView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllocateForPostView: usize,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterPostViewCallback(::windows::core::IUnknown);
impl IWMWriterPostViewCallback {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(status), ::core::mem::transmute(hr), ::core::mem::transmute(dwtype), ::core::mem::transmute(pvalue), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnPostViewSample<'a, Param4: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: Param4, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPostViewSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnumber), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(cnssampleduration), ::core::mem::transmute(dwflags), psample.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateForPostView(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllocateForPostView)(::core::mem::transmute_copy(self), ::core::mem::transmute(wstreamnum), ::core::mem::transmute(cbbuffer), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pvcontext)).ok()
    }
}
impl ::core::convert::From<IWMWriterPostViewCallback> for ::windows::core::IUnknown {
    fn from(value: IWMWriterPostViewCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPostViewCallback> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterPostViewCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterPostViewCallback> for IWMStatusCallback {
    fn from(value: IWMWriterPostViewCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPostViewCallback> for IWMStatusCallback {
    fn from(value: &IWMWriterPostViewCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStatusCallback> for IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStatusCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMStatusCallback> for &'a IWMWriterPostViewCallback {
    fn into_param(self) -> ::windows::core::Param<'a, IWMStatusCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPostViewCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPostViewCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostViewCallback {}
impl ::core::fmt::Debug for IWMWriterPostViewCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPostViewCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterPostViewCallback {
    type Vtable = IWMWriterPostViewCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9d6549d_a193_4f24_b308_03123d9b7f8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPostViewCallback_Vtbl {
    pub base: IWMStatusCallback_Vtbl,
    pub OnPostViewSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AllocateForPostView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterPreprocess(::windows::core::IUnknown);
impl IWMWriterPreprocess {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn GetMaxPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxPreprocessingPasses)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn SetNumPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNumPreprocessingPasses)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwnumpasses)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn BeginPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginPreprocessingPass)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn PreprocessSample<'a, Param3: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreprocessSample)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(cnssampletime), ::core::mem::transmute(dwflags), psample.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn EndPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndPreprocessingPass)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputnum), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IWMWriterPreprocess> for ::windows::core::IUnknown {
    fn from(value: IWMWriterPreprocess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPreprocess> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterPreprocess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterPreprocess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterPreprocess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPreprocess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPreprocess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPreprocess {}
impl ::core::fmt::Debug for IWMWriterPreprocess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPreprocess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterPreprocess {
    type Vtable = IWMWriterPreprocess_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc54a285_38c4_45b5_aa23_85b9f7cb424b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPreprocess_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetMaxPreprocessingPasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows::core::HRESULT,
    pub SetNumPreprocessingPasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::HRESULT,
    pub BeginPreprocessingPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub PreprocessSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EndPreprocessingPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterPushSink(::windows::core::IUnknown);
impl IWMWriterPushSink {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnHeader)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsRealTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.AllocateDataUnit)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnDataUnit)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnEndWriting)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pwszurl: Param0, pwsztemplateurl: Param1, fautodestroy: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Connect)(::core::mem::transmute_copy(self), pwszurl.into_param().abi(), pwsztemplateurl.into_param().abi(), fautodestroy.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn EndSession(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndSession)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterPushSink> for ::windows::core::IUnknown {
    fn from(value: IWMWriterPushSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPushSink> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterPushSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterPushSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterPushSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWMWriterPushSink> for IWMWriterSink {
    fn from(value: IWMWriterPushSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterPushSink> for IWMWriterSink {
    fn from(value: &IWMWriterPushSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for IWMWriterPushSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWMWriterSink> for &'a IWMWriterPushSink {
    fn into_param(self) -> ::windows::core::Param<'a, IWMWriterSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterPushSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterPushSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPushSink {}
impl ::core::fmt::Debug for IWMWriterPushSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPushSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterPushSink {
    type Vtable = IWMWriterPushSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc10e6a5_072c_467d_bf57_6330a9dde12a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterPushSink_Vtbl {
    pub base: IWMWriterSink_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pwsztemplateurl: ::windows::core::PCWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
pub struct IWMWriterSink(::windows::core::IUnknown);
impl IWMWriterSink {
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnHeader<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pheader: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnHeader)(::core::mem::transmute_copy(self), pheader.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsRealTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllocateDataUnit)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbdataunit), ::core::mem::transmute(&mut result__)).from_abi::<INSSBuffer>(result__)
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnDataUnit<'a, Param0: ::windows::core::IntoParam<'a, INSSBuffer>>(&self, pdataunit: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDataUnit)(::core::mem::transmute_copy(self), pdataunit.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEndWriting)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IWMWriterSink> for ::windows::core::IUnknown {
    fn from(value: IWMWriterSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWMWriterSink> for ::windows::core::IUnknown {
    fn from(value: &IWMWriterSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWMWriterSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWMWriterSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWMWriterSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWMWriterSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterSink {}
impl ::core::fmt::Debug for IWMWriterSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWMWriterSink {
    type Vtable = IWMWriterSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96406be4_2b2b_11d3_b36b_00c04f6108ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMWriterSink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRealTime: usize,
    pub AllocateDataUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnDataUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub OnEndWriting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NETSOURCE_URLCREDPOLICY_SETTINGS(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const NETSOURCE_URLCREDPOLICY_SETTING_SILENTLOGONOK: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const NETSOURCE_URLCREDPOLICY_SETTING_MUSTPROMPTUSER: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const NETSOURCE_URLCREDPOLICY_SETTING_ANONYMOUSONLY: NETSOURCE_URLCREDPOLICY_SETTINGS = NETSOURCE_URLCREDPOLICY_SETTINGS(2i32);
impl ::core::marker::Copy for NETSOURCE_URLCREDPOLICY_SETTINGS {}
impl ::core::clone::Clone for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NETSOURCE_URLCREDPOLICY_SETTINGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSOURCE_URLCREDPOLICY_SETTINGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WEBSTREAM_SAMPLE_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WEBSTREAM_SAMPLE_TYPE_FILE: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WEBSTREAM_SAMPLE_TYPE_RENDER: WEBSTREAM_SAMPLE_TYPE = WEBSTREAM_SAMPLE_TYPE(2i32);
impl ::core::marker::Copy for WEBSTREAM_SAMPLE_TYPE {}
impl ::core::clone::Clone for WEBSTREAM_SAMPLE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WEBSTREAM_SAMPLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WEBSTREAM_SAMPLE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WEBSTREAM_SAMPLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEBSTREAM_SAMPLE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateBackupRestorer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pcallback: Param0) -> ::windows::core::Result<IWMLicenseBackup> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateBackupRestorer(pcallback: *mut ::core::ffi::c_void, ppbackup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateBackupRestorer(pcallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMLicenseBackup>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateEditor() -> ::windows::core::Result<IWMMetadataEditor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateEditor(ppeditor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateEditor(::core::mem::transmute(&mut result__)).from_abi::<IWMMetadataEditor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateIndexer() -> ::windows::core::Result<IWMIndexer> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateIndexer(ppindexer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateIndexer(::core::mem::transmute(&mut result__)).from_abi::<IWMIndexer>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateProfileManager() -> ::windows::core::Result<IWMProfileManager> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateProfileManager(ppprofilemanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateProfileManager(::core::mem::transmute(&mut result__)).from_abi::<IWMProfileManager>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateReader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkcert: Param0, dwrights: u32) -> ::windows::core::Result<IWMReader> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateReader(punkcert: *mut ::core::ffi::c_void, dwrights: u32, ppreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateReader(punkcert.into_param().abi(), ::core::mem::transmute(dwrights), ::core::mem::transmute(&mut result__)).from_abi::<IWMReader>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateSyncReader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkcert: Param0, dwrights: u32) -> ::windows::core::Result<IWMSyncReader> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateSyncReader(punkcert: *mut ::core::ffi::c_void, dwrights: u32, ppsyncreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateSyncReader(punkcert.into_param().abi(), ::core::mem::transmute(dwrights), ::core::mem::transmute(&mut result__)).from_abi::<IWMSyncReader>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateWriter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkcert: Param0) -> ::windows::core::Result<IWMWriter> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriter(punkcert: *mut ::core::ffi::c_void, ppwriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateWriter(punkcert.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWMWriter>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateWriterFileSink() -> ::windows::core::Result<IWMWriterFileSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterFileSink(ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateWriterFileSink(::core::mem::transmute(&mut result__)).from_abi::<IWMWriterFileSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateWriterNetworkSink() -> ::windows::core::Result<IWMWriterNetworkSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterNetworkSink(ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateWriterNetworkSink(::core::mem::transmute(&mut result__)).from_abi::<IWMWriterNetworkSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[inline]
pub unsafe fn WMCreateWriterPushSink() -> ::windows::core::Result<IWMWriterPushSink> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMCreateWriterPushSink(ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        WMCreateWriterPushSink(::core::mem::transmute(&mut result__)).from_abi::<IWMWriterPushSink>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMDRM_IMPORT_INIT_STRUCT {
    pub dwVersion: u32,
    pub cbEncryptedSessionKeyMessage: u32,
    pub pbEncryptedSessionKeyMessage: *mut u8,
    pub cbEncryptedKeyMessage: u32,
    pub pbEncryptedKeyMessage: *mut u8,
}
impl ::core::marker::Copy for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::clone::Clone for WMDRM_IMPORT_INIT_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMDRM_IMPORT_INIT_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDRM_IMPORT_INIT_STRUCT").field("dwVersion", &self.dwVersion).field("cbEncryptedSessionKeyMessage", &self.cbEncryptedSessionKeyMessage).field("pbEncryptedSessionKeyMessage", &self.pbEncryptedSessionKeyMessage).field("cbEncryptedKeyMessage", &self.cbEncryptedKeyMessage).field("pbEncryptedKeyMessage", &self.pbEncryptedKeyMessage).finish()
    }
}
unsafe impl ::windows::core::Abi for WMDRM_IMPORT_INIT_STRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMDRM_IMPORT_INIT_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMDRM_IMPORT_INIT_STRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::default::Default for WMDRM_IMPORT_INIT_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMDRM_IMPORT_INIT_STRUCT_DEFINED: u32 = 1u32;
pub const WMFORMAT_MPEG2Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d80e3_db46_11cf_b4d1_00805f6cbbea);
pub const WMFORMAT_Script: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c8510f2_debe_4ca7_bba5_f07a104f8dff);
pub const WMFORMAT_VideoInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05589f80_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WaveFormatEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05589f81_c356_11ce_bf01_00aa0055595a);
pub const WMFORMAT_WebStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda1e6b13_8359_4050_b398_388e965bf00c);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WMIsContentProtected<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pwszfilename: Param0, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WMIsContentProtected(pwszfilename: ::windows::core::PCWSTR, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        WMIsContentProtected(pwszfilename.into_param().abi(), ::core::mem::transmute(pfisprotected)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WMMEDIASUBTYPE_ACELPnet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000130_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_Base: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_DRM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000009_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_I420: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30323449_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_IYUV: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56555949_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_M4S2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3253344d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000055_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP43: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MP4S: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5334504d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MPEG2_VIDEO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06d8026_db46_11cf_b4d1_00805f6cbbea);
pub const WMMEDIASUBTYPE_MSS1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3153534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_MSS2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3253534d_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_P422: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32323450_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_PCM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000001_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_RGB1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb78_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB24: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7d_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB32: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7e_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb79_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB555: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7c_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB565: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7b_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_RGB8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe436eb7a_524f_11ce_9f53_0020af0ba770);
pub const WMMEDIASUBTYPE_UYVY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59565955_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_VIDEOIMAGE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d4a45f2_e5f6_4b44_8388_f0ae5c0e0c37);
pub const WMMEDIASUBTYPE_WMAudioV2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV7: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000161_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudioV9: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000162_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMAudio_Lossless: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000163_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000a_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMSP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000b_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMV3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WMVP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50564d57_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVC1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31435657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WVP2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32505657_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_WebStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x776257d4_c627_41cb_8f81_7ac7ff1c40cc);
pub const WMMEDIASUBTYPE_YUY2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32595559_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YV12: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32315659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVU9: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39555659_0000_0010_8000_00aa00389b71);
pub const WMMEDIASUBTYPE_YVYU: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55595659_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Audio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73647561_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_FileTransfer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9e47579_930e_4427_adfc_ad80f290e470);
pub const WMMEDIATYPE_Image: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34a50fd8_8aa5_4386_81fe_a0efe0488e31);
pub const WMMEDIATYPE_Script: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73636d64_0000_0010_8000_00aa00389b71);
pub const WMMEDIATYPE_Text: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bba1ea7_5ab2_4829_ba57_0940209bcf3e);
pub const WMMEDIATYPE_Video: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMMPEG2VIDEOINFO {
    pub hdr: WMVIDEOINFOHEADER2,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub dwProfile: u32,
    pub dwLevel: u32,
    pub dwFlags: u32,
    pub dwSequenceHeader: [u32; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMMPEG2VIDEOINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WMMPEG2VIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMMPEG2VIDEOINFO").field("hdr", &self.hdr).field("dwStartTimeCode", &self.dwStartTimeCode).field("cbSequenceHeader", &self.cbSequenceHeader).field("dwProfile", &self.dwProfile).field("dwLevel", &self.dwLevel).field("dwFlags", &self.dwFlags).field("dwSequenceHeader", &self.dwSequenceHeader).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WMMPEG2VIDEOINFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMMPEG2VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMMPEG2VIDEOINFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMMPEG2VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMSCRIPTFORMAT {
    pub scriptType: ::windows::core::GUID,
}
impl ::core::marker::Copy for WMSCRIPTFORMAT {}
impl ::core::clone::Clone for WMSCRIPTFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMSCRIPTFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMSCRIPTFORMAT").field("scriptType", &self.scriptType).finish()
    }
}
unsafe impl ::windows::core::Abi for WMSCRIPTFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMSCRIPTFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMSCRIPTFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMSCRIPTFORMAT {}
impl ::core::default::Default for WMSCRIPTFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WMSCRIPTTYPE_TwoStrings: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82f38a70_c29f_11d1_97ad_00a0c95ea850);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_ATTR_DATATYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TYPE_DWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TYPE_STRING: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TYPE_BINARY: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TYPE_BOOL: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(3i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TYPE_QWORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(4i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TYPE_WORD: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(5i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TYPE_GUID: WMT_ATTR_DATATYPE = WMT_ATTR_DATATYPE(6i32);
impl ::core::marker::Copy for WMT_ATTR_DATATYPE {}
impl ::core::clone::Clone for WMT_ATTR_DATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_ATTR_DATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_ATTR_DATATYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_ATTR_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_ATTR_DATATYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_ATTR_IMAGETYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IMAGETYPE_BITMAP: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IMAGETYPE_JPEG: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IMAGETYPE_GIF: WMT_ATTR_IMAGETYPE = WMT_ATTR_IMAGETYPE(3i32);
impl ::core::marker::Copy for WMT_ATTR_IMAGETYPE {}
impl ::core::clone::Clone for WMT_ATTR_IMAGETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_ATTR_IMAGETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_ATTR_IMAGETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_ATTR_IMAGETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_ATTR_IMAGETYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMT_BUFFER_SEGMENT {
    pub pBuffer: ::core::option::Option<INSSBuffer>,
    pub cbOffset: u32,
    pub cbLength: u32,
}
impl ::core::clone::Clone for WMT_BUFFER_SEGMENT {
    fn clone(&self) -> Self {
        Self { pBuffer: self.pBuffer.clone(), cbOffset: self.cbOffset, cbLength: self.cbLength }
    }
}
impl ::core::fmt::Debug for WMT_BUFFER_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_BUFFER_SEGMENT").field("pBuffer", &self.pBuffer).field("cbOffset", &self.cbOffset).field("cbLength", &self.cbLength).finish()
    }
}
unsafe impl ::windows::core::Abi for WMT_BUFFER_SEGMENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMT_BUFFER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.cbOffset == other.cbOffset && self.cbLength == other.cbLength
    }
}
impl ::core::cmp::Eq for WMT_BUFFER_SEGMENT {}
impl ::core::default::Default for WMT_BUFFER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_CODEC_INFO_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CODECINFO_AUDIO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CODECINFO_VIDEO: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CODECINFO_UNKNOWN: WMT_CODEC_INFO_TYPE = WMT_CODEC_INFO_TYPE(-1i32);
impl ::core::marker::Copy for WMT_CODEC_INFO_TYPE {}
impl ::core::clone::Clone for WMT_CODEC_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_CODEC_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_CODEC_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_CODEC_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_CODEC_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMT_COLORSPACEINFO_EXTENSION_DATA {
    pub ucColorPrimaries: u8,
    pub ucColorTransferChar: u8,
    pub ucColorMatrixCoef: u8,
}
impl ::core::marker::Copy for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_COLORSPACEINFO_EXTENSION_DATA").field("ucColorPrimaries", &self.ucColorPrimaries).field("ucColorTransferChar", &self.ucColorTransferChar).field("ucColorMatrixCoef", &self.ucColorMatrixCoef).finish()
    }
}
unsafe impl ::windows::core::Abi for WMT_COLORSPACEINFO_EXTENSION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_COLORSPACEINFO_EXTENSION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::default::Default for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_CREDENTIAL_FLAGS(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CREDENTIAL_SAVE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CREDENTIAL_DONT_CACHE: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CREDENTIAL_CLEAR_TEXT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(4i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CREDENTIAL_PROXY: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(8i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CREDENTIAL_ENCRYPT: WMT_CREDENTIAL_FLAGS = WMT_CREDENTIAL_FLAGS(16i32);
impl ::core::marker::Copy for WMT_CREDENTIAL_FLAGS {}
impl ::core::clone::Clone for WMT_CREDENTIAL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_CREDENTIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_CREDENTIAL_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
pub const WMT_DMOCATEGORY_AUDIO_WATERMARK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65221c5a_fa75_4b39_b50c_06c336b6a3ef);
pub const WMT_DMOCATEGORY_VIDEO_WATERMARK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x187cc922_8efc_4404_9daf_63f4830df1bc);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_DRMLA_TRUST(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_DRMLA_UNTRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_DRMLA_TRUSTED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_DRMLA_TAMPERED: WMT_DRMLA_TRUST = WMT_DRMLA_TRUST(2i32);
impl ::core::marker::Copy for WMT_DRMLA_TRUST {}
impl ::core::clone::Clone for WMT_DRMLA_TRUST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_DRMLA_TRUST {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_DRMLA_TRUST {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_DRMLA_TRUST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_DRMLA_TRUST").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMT_FILESINK_DATA_UNIT {
    pub packetHeaderBuffer: WMT_BUFFER_SEGMENT,
    pub cPayloads: u32,
    pub pPayloadHeaderBuffers: *mut WMT_BUFFER_SEGMENT,
    pub cPayloadDataFragments: u32,
    pub pPayloadDataFragments: *mut WMT_PAYLOAD_FRAGMENT,
}
impl ::core::clone::Clone for WMT_FILESINK_DATA_UNIT {
    fn clone(&self) -> Self {
        Self {
            packetHeaderBuffer: self.packetHeaderBuffer.clone(),
            cPayloads: self.cPayloads,
            pPayloadHeaderBuffers: self.pPayloadHeaderBuffers,
            cPayloadDataFragments: self.cPayloadDataFragments,
            pPayloadDataFragments: self.pPayloadDataFragments,
        }
    }
}
impl ::core::fmt::Debug for WMT_FILESINK_DATA_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_FILESINK_DATA_UNIT").field("packetHeaderBuffer", &self.packetHeaderBuffer).field("cPayloads", &self.cPayloads).field("pPayloadHeaderBuffers", &self.pPayloadHeaderBuffers).field("cPayloadDataFragments", &self.cPayloadDataFragments).field("pPayloadDataFragments", &self.pPayloadDataFragments).finish()
    }
}
unsafe impl ::windows::core::Abi for WMT_FILESINK_DATA_UNIT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMT_FILESINK_DATA_UNIT {
    fn eq(&self, other: &Self) -> bool {
        self.packetHeaderBuffer == other.packetHeaderBuffer && self.cPayloads == other.cPayloads && self.pPayloadHeaderBuffers == other.pPayloadHeaderBuffers && self.cPayloadDataFragments == other.cPayloadDataFragments && self.pPayloadDataFragments == other.pPayloadDataFragments
    }
}
impl ::core::cmp::Eq for WMT_FILESINK_DATA_UNIT {}
impl ::core::default::Default for WMT_FILESINK_DATA_UNIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_FILESINK_MODE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_FM_SINGLE_BUFFERS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_FM_FILESINK_DATA_UNITS: WMT_FILESINK_MODE = WMT_FILESINK_MODE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_FM_FILESINK_UNBUFFERED: WMT_FILESINK_MODE = WMT_FILESINK_MODE(4i32);
impl ::core::marker::Copy for WMT_FILESINK_MODE {}
impl ::core::clone::Clone for WMT_FILESINK_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_FILESINK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_FILESINK_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_FILESINK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_FILESINK_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_IMAGE_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_NONE: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_BITMAP: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_JPEG: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_GIF: WMT_IMAGE_TYPE = WMT_IMAGE_TYPE(3i32);
impl ::core::marker::Copy for WMT_IMAGE_TYPE {}
impl ::core::clone::Clone for WMT_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_IMAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_INDEXER_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_PRESENTATION_TIME: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_FRAME_NUMBERS: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_TIMECODE: WMT_INDEXER_TYPE = WMT_INDEXER_TYPE(2i32);
impl ::core::marker::Copy for WMT_INDEXER_TYPE {}
impl ::core::clone::Clone for WMT_INDEXER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_INDEXER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_INDEXER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_INDEXER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_INDEXER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_INDEX_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_NEAREST_DATA_UNIT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_NEAREST_OBJECT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_IT_NEAREST_CLEAN_POINT: WMT_INDEX_TYPE = WMT_INDEX_TYPE(3i32);
impl ::core::marker::Copy for WMT_INDEX_TYPE {}
impl ::core::clone::Clone for WMT_INDEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_INDEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_INDEX_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_INDEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_INDEX_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_MUSICSPEECH_CLASS_MODE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_MS_CLASS_MUSIC: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_MS_CLASS_SPEECH: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_MS_CLASS_MIXED: WMT_MUSICSPEECH_CLASS_MODE = WMT_MUSICSPEECH_CLASS_MODE(2i32);
impl ::core::marker::Copy for WMT_MUSICSPEECH_CLASS_MODE {}
impl ::core::clone::Clone for WMT_MUSICSPEECH_CLASS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_MUSICSPEECH_CLASS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_MUSICSPEECH_CLASS_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_MUSICSPEECH_CLASS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_MUSICSPEECH_CLASS_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_NET_PROTOCOL(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PROTOCOL_HTTP: WMT_NET_PROTOCOL = WMT_NET_PROTOCOL(0i32);
impl ::core::marker::Copy for WMT_NET_PROTOCOL {}
impl ::core::clone::Clone for WMT_NET_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_NET_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_NET_PROTOCOL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_NET_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_NET_PROTOCOL").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_OFFSET_FORMAT(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_OFFSET_FORMAT_100NS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_OFFSET_FORMAT_FRAME_NUMBERS: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_OFFSET_FORMAT_PLAYLIST_OFFSET: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_OFFSET_FORMAT_TIMECODE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(3i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_OFFSET_FORMAT_100NS_APPROXIMATE: WMT_OFFSET_FORMAT = WMT_OFFSET_FORMAT(4i32);
impl ::core::marker::Copy for WMT_OFFSET_FORMAT {}
impl ::core::clone::Clone for WMT_OFFSET_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_OFFSET_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_OFFSET_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_OFFSET_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_OFFSET_FORMAT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMT_PAYLOAD_FRAGMENT {
    pub dwPayloadIndex: u32,
    pub segmentData: WMT_BUFFER_SEGMENT,
}
impl ::core::clone::Clone for WMT_PAYLOAD_FRAGMENT {
    fn clone(&self) -> Self {
        Self { dwPayloadIndex: self.dwPayloadIndex, segmentData: self.segmentData.clone() }
    }
}
impl ::core::fmt::Debug for WMT_PAYLOAD_FRAGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_PAYLOAD_FRAGMENT").field("dwPayloadIndex", &self.dwPayloadIndex).field("segmentData", &self.segmentData).finish()
    }
}
unsafe impl ::windows::core::Abi for WMT_PAYLOAD_FRAGMENT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for WMT_PAYLOAD_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.dwPayloadIndex == other.dwPayloadIndex && self.segmentData == other.segmentData
    }
}
impl ::core::cmp::Eq for WMT_PAYLOAD_FRAGMENT {}
impl ::core::default::Default for WMT_PAYLOAD_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_PLAY_MODE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PLAY_MODE_AUTOSELECT: WMT_PLAY_MODE = WMT_PLAY_MODE(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PLAY_MODE_LOCAL: WMT_PLAY_MODE = WMT_PLAY_MODE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PLAY_MODE_DOWNLOAD: WMT_PLAY_MODE = WMT_PLAY_MODE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PLAY_MODE_STREAMING: WMT_PLAY_MODE = WMT_PLAY_MODE(3i32);
impl ::core::marker::Copy for WMT_PLAY_MODE {}
impl ::core::clone::Clone for WMT_PLAY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_PLAY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_PLAY_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_PLAY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_PLAY_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_PROXY_SETTINGS(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PROXY_SETTING_NONE: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PROXY_SETTING_MANUAL: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PROXY_SETTING_AUTO: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PROXY_SETTING_BROWSER: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(3i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PROXY_SETTING_MAX: WMT_PROXY_SETTINGS = WMT_PROXY_SETTINGS(4i32);
impl ::core::marker::Copy for WMT_PROXY_SETTINGS {}
impl ::core::clone::Clone for WMT_PROXY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_PROXY_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_PROXY_SETTINGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_PROXY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_PROXY_SETTINGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_RIGHTS(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_PLAYBACK: WMT_RIGHTS = WMT_RIGHTS(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_COPY_TO_NON_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_COPY_TO_CD: WMT_RIGHTS = WMT_RIGHTS(8i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_COPY_TO_SDMI_DEVICE: WMT_RIGHTS = WMT_RIGHTS(16i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_ONE_TIME: WMT_RIGHTS = WMT_RIGHTS(32i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_SAVE_STREAM_PROTECTED: WMT_RIGHTS = WMT_RIGHTS(64i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_COPY: WMT_RIGHTS = WMT_RIGHTS(128i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_COLLABORATIVE_PLAY: WMT_RIGHTS = WMT_RIGHTS(256i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_SDMI_TRIGGER: WMT_RIGHTS = WMT_RIGHTS(65536i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RIGHT_SDMI_NOMORECOPIES: WMT_RIGHTS = WMT_RIGHTS(131072i32);
impl ::core::marker::Copy for WMT_RIGHTS {}
impl ::core::clone::Clone for WMT_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_RIGHTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_RIGHTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_STATUS(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_ERROR: WMT_STATUS = WMT_STATUS(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_OPENED: WMT_STATUS = WMT_STATUS(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_BUFFERING_START: WMT_STATUS = WMT_STATUS(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_BUFFERING_STOP: WMT_STATUS = WMT_STATUS(3i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_EOF: WMT_STATUS = WMT_STATUS(4i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_END_OF_FILE: WMT_STATUS = WMT_STATUS(4i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_END_OF_SEGMENT: WMT_STATUS = WMT_STATUS(5i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_END_OF_STREAMING: WMT_STATUS = WMT_STATUS(6i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_LOCATING: WMT_STATUS = WMT_STATUS(7i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CONNECTING: WMT_STATUS = WMT_STATUS(8i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_NO_RIGHTS: WMT_STATUS = WMT_STATUS(9i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_MISSING_CODEC: WMT_STATUS = WMT_STATUS(10i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_STARTED: WMT_STATUS = WMT_STATUS(11i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_STOPPED: WMT_STATUS = WMT_STATUS(12i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CLOSED: WMT_STATUS = WMT_STATUS(13i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_STRIDING: WMT_STATUS = WMT_STATUS(14i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TIMER: WMT_STATUS = WMT_STATUS(15i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_INDEX_PROGRESS: WMT_STATUS = WMT_STATUS(16i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_SAVEAS_START: WMT_STATUS = WMT_STATUS(17i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_SAVEAS_STOP: WMT_STATUS = WMT_STATUS(18i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_NEW_SOURCEFLAGS: WMT_STATUS = WMT_STATUS(19i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_NEW_METADATA: WMT_STATUS = WMT_STATUS(20i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_BACKUPRESTORE_BEGIN: WMT_STATUS = WMT_STATUS(21i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_SOURCE_SWITCH: WMT_STATUS = WMT_STATUS(22i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_ACQUIRE_LICENSE: WMT_STATUS = WMT_STATUS(23i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_INDIVIDUALIZE: WMT_STATUS = WMT_STATUS(24i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_NEEDS_INDIVIDUALIZATION: WMT_STATUS = WMT_STATUS(25i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_NO_RIGHTS_EX: WMT_STATUS = WMT_STATUS(26i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_BACKUPRESTORE_END: WMT_STATUS = WMT_STATUS(27i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_BACKUPRESTORE_CONNECTING: WMT_STATUS = WMT_STATUS(28i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_BACKUPRESTORE_DISCONNECTING: WMT_STATUS = WMT_STATUS(29i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_ERROR_WITHURL: WMT_STATUS = WMT_STATUS(30i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RESTRICTED_LICENSE: WMT_STATUS = WMT_STATUS(31i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CLIENT_CONNECT: WMT_STATUS = WMT_STATUS(32i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CLIENT_DISCONNECT: WMT_STATUS = WMT_STATUS(33i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_NATIVE_OUTPUT_PROPS_CHANGED: WMT_STATUS = WMT_STATUS(34i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RECONNECT_START: WMT_STATUS = WMT_STATUS(35i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_RECONNECT_END: WMT_STATUS = WMT_STATUS(36i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CLIENT_CONNECT_EX: WMT_STATUS = WMT_STATUS(37i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CLIENT_DISCONNECT_EX: WMT_STATUS = WMT_STATUS(38i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_SET_FEC_SPAN: WMT_STATUS = WMT_STATUS(39i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PREROLL_READY: WMT_STATUS = WMT_STATUS(40i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PREROLL_COMPLETE: WMT_STATUS = WMT_STATUS(41i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CLIENT_PROPERTIES: WMT_STATUS = WMT_STATUS(42i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_LICENSEURL_SIGNATURE_STATE: WMT_STATUS = WMT_STATUS(43i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_INIT_PLAYLIST_BURN: WMT_STATUS = WMT_STATUS(44i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TRANSCRYPTOR_INIT: WMT_STATUS = WMT_STATUS(45i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TRANSCRYPTOR_SEEKED: WMT_STATUS = WMT_STATUS(46i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TRANSCRYPTOR_READ: WMT_STATUS = WMT_STATUS(47i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TRANSCRYPTOR_CLOSED: WMT_STATUS = WMT_STATUS(48i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PROXIMITY_RESULT: WMT_STATUS = WMT_STATUS(49i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_PROXIMITY_COMPLETED: WMT_STATUS = WMT_STATUS(50i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CONTENT_ENABLER: WMT_STATUS = WMT_STATUS(51i32);
impl ::core::marker::Copy for WMT_STATUS {}
impl ::core::clone::Clone for WMT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_STORAGE_FORMAT(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_Storage_Format_MP3: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_Storage_Format_V1: WMT_STORAGE_FORMAT = WMT_STORAGE_FORMAT(1i32);
impl ::core::marker::Copy for WMT_STORAGE_FORMAT {}
impl ::core::clone::Clone for WMT_STORAGE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_STORAGE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_STORAGE_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_STORAGE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STORAGE_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_STREAM_SELECTION(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_OFF: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_CLEANPOINT_ONLY: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_ON: WMT_STREAM_SELECTION = WMT_STREAM_SELECTION(2i32);
impl ::core::marker::Copy for WMT_STREAM_SELECTION {}
impl ::core::clone::Clone for WMT_STREAM_SELECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_STREAM_SELECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_STREAM_SELECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_STREAM_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STREAM_SELECTION").field(&self.0).finish()
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMT_TIMECODE_EXTENSION_DATA {
    pub wRange: u16,
    pub dwTimecode: u32,
    pub dwUserbits: u32,
    pub dwAmFlags: u32,
}
impl ::core::marker::Copy for WMT_TIMECODE_EXTENSION_DATA {}
impl ::core::clone::Clone for WMT_TIMECODE_EXTENSION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WMT_TIMECODE_EXTENSION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_TIMECODE_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_TIMECODE_EXTENSION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_TIMECODE_EXTENSION_DATA {}
impl ::core::default::Default for WMT_TIMECODE_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_TIMECODE_FRAMERATE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TIMECODE_FRAMERATE_30: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TIMECODE_FRAMERATE_30DROP: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TIMECODE_FRAMERATE_25: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_TIMECODE_FRAMERATE_24: WMT_TIMECODE_FRAMERATE = WMT_TIMECODE_FRAMERATE(3i32);
impl ::core::marker::Copy for WMT_TIMECODE_FRAMERATE {}
impl ::core::clone::Clone for WMT_TIMECODE_FRAMERATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_TIMECODE_FRAMERATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_TIMECODE_FRAMERATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_TIMECODE_FRAMERATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_TIMECODE_FRAMERATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_TRANSPORT_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_Transport_Type_Unreliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_Transport_Type_Reliable: WMT_TRANSPORT_TYPE = WMT_TRANSPORT_TYPE(1i32);
impl ::core::marker::Copy for WMT_TRANSPORT_TYPE {}
impl ::core::clone::Clone for WMT_TRANSPORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_TRANSPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_TRANSPORT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_TRANSPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_TRANSPORT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_VERSION(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VER_4_0: WMT_VERSION = WMT_VERSION(262144i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VER_7_0: WMT_VERSION = WMT_VERSION(458752i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VER_8_0: WMT_VERSION = WMT_VERSION(524288i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VER_9_0: WMT_VERSION = WMT_VERSION(589824i32);
impl ::core::marker::Copy for WMT_VERSION {}
impl ::core::clone::Clone for WMT_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_INTEGER_DENOMINATOR: i32 = 65536i32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER: u32 = 491406834u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_MAGIC_NUMBER_2: u32 = 491406835u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMT_VIDEOIMAGE_SAMPLE {
    pub dwMagic: u32,
    pub cbStruct: u32,
    pub dwControlFlags: u32,
    pub dwInputFlagsCur: u32,
    pub lCurMotionXtoX: i32,
    pub lCurMotionYtoX: i32,
    pub lCurMotionXoffset: i32,
    pub lCurMotionXtoY: i32,
    pub lCurMotionYtoY: i32,
    pub lCurMotionYoffset: i32,
    pub lCurBlendCoef1: i32,
    pub lCurBlendCoef2: i32,
    pub dwInputFlagsPrev: u32,
    pub lPrevMotionXtoX: i32,
    pub lPrevMotionYtoX: i32,
    pub lPrevMotionXoffset: i32,
    pub lPrevMotionXtoY: i32,
    pub lPrevMotionYtoY: i32,
    pub lPrevMotionYoffset: i32,
    pub lPrevBlendCoef1: i32,
    pub lPrevBlendCoef2: i32,
}
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_VIDEOIMAGE_SAMPLE")
            .field("dwMagic", &self.dwMagic)
            .field("cbStruct", &self.cbStruct)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwInputFlagsCur", &self.dwInputFlagsCur)
            .field("lCurMotionXtoX", &self.lCurMotionXtoX)
            .field("lCurMotionYtoX", &self.lCurMotionYtoX)
            .field("lCurMotionXoffset", &self.lCurMotionXoffset)
            .field("lCurMotionXtoY", &self.lCurMotionXtoY)
            .field("lCurMotionYtoY", &self.lCurMotionYtoY)
            .field("lCurMotionYoffset", &self.lCurMotionYoffset)
            .field("lCurBlendCoef1", &self.lCurBlendCoef1)
            .field("lCurBlendCoef2", &self.lCurBlendCoef2)
            .field("dwInputFlagsPrev", &self.dwInputFlagsPrev)
            .field("lPrevMotionXtoX", &self.lPrevMotionXtoX)
            .field("lPrevMotionYtoX", &self.lPrevMotionYtoX)
            .field("lPrevMotionXoffset", &self.lPrevMotionXoffset)
            .field("lPrevMotionXtoY", &self.lPrevMotionXtoY)
            .field("lPrevMotionYtoY", &self.lPrevMotionYtoY)
            .field("lPrevMotionYoffset", &self.lPrevMotionYoffset)
            .field("lPrevBlendCoef1", &self.lPrevBlendCoef1)
            .field("lPrevBlendCoef2", &self.lPrevBlendCoef2)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WMT_VIDEOIMAGE_SAMPLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_VIDEOIMAGE_SAMPLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WMT_VIDEOIMAGE_SAMPLE2 {
    pub dwMagic: u32,
    pub dwStructSize: u32,
    pub dwControlFlags: u32,
    pub dwViewportWidth: u32,
    pub dwViewportHeight: u32,
    pub dwCurrImageWidth: u32,
    pub dwCurrImageHeight: u32,
    pub fCurrRegionX0: f32,
    pub fCurrRegionY0: f32,
    pub fCurrRegionWidth: f32,
    pub fCurrRegionHeight: f32,
    pub fCurrBlendCoef: f32,
    pub dwPrevImageWidth: u32,
    pub dwPrevImageHeight: u32,
    pub fPrevRegionX0: f32,
    pub fPrevRegionY0: f32,
    pub fPrevRegionWidth: f32,
    pub fPrevRegionHeight: f32,
    pub fPrevBlendCoef: f32,
    pub dwEffectType: u32,
    pub dwNumEffectParas: u32,
    pub fEffectPara0: f32,
    pub fEffectPara1: f32,
    pub fEffectPara2: f32,
    pub fEffectPara3: f32,
    pub fEffectPara4: f32,
    pub bKeepPrevImage: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WMT_VIDEOIMAGE_SAMPLE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_VIDEOIMAGE_SAMPLE2")
            .field("dwMagic", &self.dwMagic)
            .field("dwStructSize", &self.dwStructSize)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwViewportWidth", &self.dwViewportWidth)
            .field("dwViewportHeight", &self.dwViewportHeight)
            .field("dwCurrImageWidth", &self.dwCurrImageWidth)
            .field("dwCurrImageHeight", &self.dwCurrImageHeight)
            .field("fCurrRegionX0", &self.fCurrRegionX0)
            .field("fCurrRegionY0", &self.fCurrRegionY0)
            .field("fCurrRegionWidth", &self.fCurrRegionWidth)
            .field("fCurrRegionHeight", &self.fCurrRegionHeight)
            .field("fCurrBlendCoef", &self.fCurrBlendCoef)
            .field("dwPrevImageWidth", &self.dwPrevImageWidth)
            .field("dwPrevImageHeight", &self.dwPrevImageHeight)
            .field("fPrevRegionX0", &self.fPrevRegionX0)
            .field("fPrevRegionY0", &self.fPrevRegionY0)
            .field("fPrevRegionWidth", &self.fPrevRegionWidth)
            .field("fPrevRegionHeight", &self.fPrevRegionHeight)
            .field("fPrevBlendCoef", &self.fPrevBlendCoef)
            .field("dwEffectType", &self.dwEffectType)
            .field("dwNumEffectParas", &self.dwNumEffectParas)
            .field("fEffectPara0", &self.fEffectPara0)
            .field("fEffectPara1", &self.fEffectPara1)
            .field("fEffectPara2", &self.fEffectPara2)
            .field("fEffectPara3", &self.fEffectPara3)
            .field("fEffectPara4", &self.fEffectPara4)
            .field("bKeepPrevImage", &self.bKeepPrevImage)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WMT_VIDEOIMAGE_SAMPLE2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_VIDEOIMAGE_SAMPLE2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_SAMPLE_ADV_BLENDING: u32 = 8u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_SAMPLE_BLENDING: u32 = 4u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_SAMPLE_INPUT_FRAME: u32 = 1u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_SAMPLE_MOTION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_SAMPLE_OUTPUT_FRAME: u32 = 2u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_SAMPLE_ROTATION: u32 = 2u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_SAMPLE_USES_CURRENT_INPUT_FRAME: u32 = 4u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_SAMPLE_USES_PREVIOUS_INPUT_FRAME: u32 = 8u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_BOW_TIE: u32 = 11u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_CIRCLE: u32 = 12u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_CROSS_FADE: u32 = 13u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_DIAGONAL: u32 = 14u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_DIAMOND: u32 = 15u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FADE_TO_COLOR: u32 = 16u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FILLED_V: u32 = 17u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_FLIP: u32 = 18u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_INSET: u32 = 19u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_IRIS: u32 = 20u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_PAGE_ROLL: u32 = 21u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_RECTANGLE: u32 = 23u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_REVEAL: u32 = 24u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_SLIDE: u32 = 27u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_SPLIT: u32 = 29u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_STAR: u32 = 30u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_VIDEOIMAGE_TRANSITION_WHEEL: u32 = 31u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMT_WATERMARK_ENTRY {
    pub wmetType: WMT_WATERMARK_ENTRY_TYPE,
    pub clsid: ::windows::core::GUID,
    pub cbDisplayName: u32,
    pub pwszDisplayName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WMT_WATERMARK_ENTRY {}
impl ::core::clone::Clone for WMT_WATERMARK_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_WATERMARK_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WATERMARK_ENTRY").field("wmetType", &self.wmetType).field("clsid", &self.clsid).field("cbDisplayName", &self.cbDisplayName).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
unsafe impl ::windows::core::Abi for WMT_WATERMARK_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_WATERMARK_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_WATERMARK_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_WATERMARK_ENTRY {}
impl ::core::default::Default for WMT_WATERMARK_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WMT_WATERMARK_ENTRY_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_WMETYPE_AUDIO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WMT_WMETYPE_VIDEO: WMT_WATERMARK_ENTRY_TYPE = WMT_WATERMARK_ENTRY_TYPE(2i32);
impl ::core::marker::Copy for WMT_WATERMARK_ENTRY_TYPE {}
impl ::core::clone::Clone for WMT_WATERMARK_ENTRY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WMT_WATERMARK_ENTRY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WMT_WATERMARK_ENTRY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WMT_WATERMARK_ENTRY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_WATERMARK_ENTRY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMT_WEBSTREAM_FORMAT {
    pub cbSize: u16,
    pub cbSampleHeaderFixedData: u16,
    pub wVersion: u16,
    pub wReserved: u16,
}
impl ::core::marker::Copy for WMT_WEBSTREAM_FORMAT {}
impl ::core::clone::Clone for WMT_WEBSTREAM_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_WEBSTREAM_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WEBSTREAM_FORMAT").field("cbSize", &self.cbSize).field("cbSampleHeaderFixedData", &self.cbSampleHeaderFixedData).field("wVersion", &self.wVersion).field("wReserved", &self.wReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for WMT_WEBSTREAM_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_WEBSTREAM_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_FORMAT {}
impl ::core::default::Default for WMT_WEBSTREAM_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WMT_WEBSTREAM_SAMPLE_HEADER {
    pub cbLength: u16,
    pub wPart: u16,
    pub cTotalParts: u16,
    pub wSampleType: u16,
    pub wszURL: [u16; 1],
}
impl ::core::marker::Copy for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::clone::Clone for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WEBSTREAM_SAMPLE_HEADER").field("cbLength", &self.cbLength).field("wPart", &self.wPart).field("cTotalParts", &self.cTotalParts).field("wSampleType", &self.wSampleType).field("wszURL", &self.wszURL).finish()
    }
}
unsafe impl ::windows::core::Abi for WMT_WEBSTREAM_SAMPLE_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMT_WEBSTREAM_SAMPLE_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::default::Default for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMVIDEOINFOHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WMVIDEOINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMVIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WMVIDEOINFOHEADER {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMVIDEOINFOHEADER>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMVIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct WMVIDEOINFOHEADER2 {
    pub rcSource: super::super::Foundation::RECT,
    pub rcTarget: super::super::Foundation::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: i64,
    pub dwInterlaceFlags: u32,
    pub dwCopyProtectFlags: u32,
    pub dwPictAspectRatioX: u32,
    pub dwPictAspectRatioY: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub bmiHeader: super::super::Graphics::Gdi::BITMAPINFOHEADER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for WMVIDEOINFOHEADER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WMVIDEOINFOHEADER2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMVIDEOINFOHEADER2")
            .field("rcSource", &self.rcSource)
            .field("rcTarget", &self.rcTarget)
            .field("dwBitRate", &self.dwBitRate)
            .field("dwBitErrorRate", &self.dwBitErrorRate)
            .field("AvgTimePerFrame", &self.AvgTimePerFrame)
            .field("dwInterlaceFlags", &self.dwInterlaceFlags)
            .field("dwCopyProtectFlags", &self.dwCopyProtectFlags)
            .field("dwPictAspectRatioX", &self.dwPictAspectRatioX)
            .field("dwPictAspectRatioY", &self.dwPictAspectRatioY)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("bmiHeader", &self.bmiHeader)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for WMVIDEOINFOHEADER2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WMVIDEOINFOHEADER2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMVIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_ADDRESS_ACCESSENTRY {
    pub dwIPAddress: u32,
    pub dwMask: u32,
}
impl ::core::marker::Copy for WM_ADDRESS_ACCESSENTRY {}
impl ::core::clone::Clone for WM_ADDRESS_ACCESSENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_ADDRESS_ACCESSENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_ADDRESS_ACCESSENTRY").field("dwIPAddress", &self.dwIPAddress).field("dwMask", &self.dwMask).finish()
    }
}
unsafe impl ::windows::core::Abi for WM_ADDRESS_ACCESSENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_ADDRESS_ACCESSENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_ADDRESS_ACCESSENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_ADDRESS_ACCESSENTRY {}
impl ::core::default::Default for WM_ADDRESS_ACCESSENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_AETYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_AETYPE_INCLUDE: WM_AETYPE = WM_AETYPE(105i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_AETYPE_EXCLUDE: WM_AETYPE = WM_AETYPE(101i32);
impl ::core::marker::Copy for WM_AETYPE {}
impl ::core::clone::Clone for WM_AETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_AETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WM_AETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_AETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_AETYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_CLIENT_PROPERTIES {
    pub dwIPAddress: u32,
    pub dwPort: u32,
}
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES {}
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_CLIENT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_CLIENT_PROPERTIES").field("dwIPAddress", &self.dwIPAddress).field("dwPort", &self.dwPort).finish()
    }
}
unsafe impl ::windows::core::Abi for WM_CLIENT_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_CLIENT_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES {}
impl ::core::default::Default for WM_CLIENT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_CLIENT_PROPERTIES_EX {
    pub cbSize: u32,
    pub pwszIPAddress: ::windows::core::PCWSTR,
    pub pwszPort: ::windows::core::PCWSTR,
    pub pwszDNSName: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for WM_CLIENT_PROPERTIES_EX {}
impl ::core::clone::Clone for WM_CLIENT_PROPERTIES_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_CLIENT_PROPERTIES_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_CLIENT_PROPERTIES_EX").field("cbSize", &self.cbSize).field("pwszIPAddress", &self.pwszIPAddress).field("pwszPort", &self.pwszPort).field("pwszDNSName", &self.pwszDNSName).finish()
    }
}
unsafe impl ::windows::core::Abi for WM_CLIENT_PROPERTIES_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_CLIENT_PROPERTIES_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES_EX {}
impl ::core::default::Default for WM_CLIENT_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_CL_INTERLACED420: u32 = 0u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_CL_PROGRESSIVE420: u32 = 1u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_CT_BOTTOM_FIELD_FIRST: u32 = 32u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_CT_INTERLACED: u32 = 128u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_CT_REPEAT_FIRST_FIELD: u32 = 16u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_CT_TOP_FIELD_FIRST: u32 = 64u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_DM_INTERLACED_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_NOTINTERLACED: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_DEINTERLACE_NORMAL: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_DEINTERLACE_HALFSIZE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_DEINTERLACE_HALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(3i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_DEINTERLACE_INVERSETELECINE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(4i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_DEINTERLACE_VERTICALHALFSIZEDOUBLERATE: WM_DM_INTERLACED_TYPE = WM_DM_INTERLACED_TYPE(5i32);
impl ::core::marker::Copy for WM_DM_INTERLACED_TYPE {}
impl ::core::clone::Clone for WM_DM_INTERLACED_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_DM_INTERLACED_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WM_DM_INTERLACED_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_DM_INTERLACED_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_DM_INTERLACED_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_DM_IT_FIRST_FRAME_COHERENCY(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_DISABLE_COHERENT_MODE: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(3i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(4i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_TOP: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(5i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_AA_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(6i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BB_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(7i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_BC_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(8i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_CD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(9i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_DM_IT_FIRST_FRAME_IN_CLIP_IS_DD_BOTTOM: WM_DM_IT_FIRST_FRAME_COHERENCY = WM_DM_IT_FIRST_FRAME_COHERENCY(10i32);
impl ::core::marker::Copy for WM_DM_IT_FIRST_FRAME_COHERENCY {}
impl ::core::clone::Clone for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WM_DM_IT_FIRST_FRAME_COHERENCY {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_DM_IT_FIRST_FRAME_COHERENCY").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
impl ::core::marker::Copy for WM_LEAKY_BUCKET_PAIR {}
impl ::core::clone::Clone for WM_LEAKY_BUCKET_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_LEAKY_BUCKET_PAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_LEAKY_BUCKET_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_LEAKY_BUCKET_PAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_LEAKY_BUCKET_PAIR {}
impl ::core::default::Default for WM_LEAKY_BUCKET_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_MAX_STREAMS: u32 = 63u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_MAX_VIDEO_STREAMS: u32 = 63u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_MEDIA_TYPE {
    pub majortype: ::windows::core::GUID,
    pub subtype: ::windows::core::GUID,
    pub bFixedSizeSamples: super::super::Foundation::BOOL,
    pub bTemporalCompression: super::super::Foundation::BOOL,
    pub lSampleSize: u32,
    pub formattype: ::windows::core::GUID,
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub cbFormat: u32,
    pub pbFormat: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        Self {
            majortype: self.majortype,
            subtype: self.subtype,
            bFixedSizeSamples: self.bFixedSizeSamples,
            bTemporalCompression: self.bTemporalCompression,
            lSampleSize: self.lSampleSize,
            formattype: self.formattype,
            pUnk: self.pUnk.clone(),
            cbFormat: self.cbFormat,
            pbFormat: self.pbFormat,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WM_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_MEDIA_TYPE").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).field("pUnk", &self.pUnk).field("cbFormat", &self.cbFormat).field("pbFormat", &self.pbFormat).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_MEDIA_TYPE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_PICTURE {
    pub pwszMIMEType: ::windows::core::PWSTR,
    pub bPictureType: u8,
    pub pwszDescription: ::windows::core::PWSTR,
    pub dwDataLen: u32,
    pub pbData: *mut u8,
}
impl ::core::marker::Copy for WM_PICTURE {}
impl ::core::clone::Clone for WM_PICTURE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_PICTURE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_PICTURE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_PICTURE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_PICTURE {}
impl ::core::default::Default for WM_PICTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_PLAYBACK_DRC_LEVEL(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_PLAYBACK_DRC_HIGH: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(0i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_PLAYBACK_DRC_MEDIUM: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_PLAYBACK_DRC_LOW: WM_PLAYBACK_DRC_LEVEL = WM_PLAYBACK_DRC_LEVEL(2i32);
impl ::core::marker::Copy for WM_PLAYBACK_DRC_LEVEL {}
impl ::core::clone::Clone for WM_PLAYBACK_DRC_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_PLAYBACK_DRC_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WM_PLAYBACK_DRC_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_PLAYBACK_DRC_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_PLAYBACK_DRC_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_PORT_NUMBER_RANGE {
    pub wPortBegin: u16,
    pub wPortEnd: u16,
}
impl ::core::marker::Copy for WM_PORT_NUMBER_RANGE {}
impl ::core::clone::Clone for WM_PORT_NUMBER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_PORT_NUMBER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_PORT_NUMBER_RANGE").field("wPortBegin", &self.wPortBegin).field("wPortEnd", &self.wPortEnd).finish()
    }
}
unsafe impl ::windows::core::Abi for WM_PORT_NUMBER_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_PORT_NUMBER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_PORT_NUMBER_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_PORT_NUMBER_RANGE {}
impl ::core::default::Default for WM_PORT_NUMBER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_READER_CLIENTINFO {
    pub cbSize: u32,
    pub wszLang: ::windows::core::PWSTR,
    pub wszBrowserUserAgent: ::windows::core::PWSTR,
    pub wszBrowserWebPage: ::windows::core::PWSTR,
    pub qwReserved: u64,
    pub pReserved: *mut super::super::Foundation::LPARAM,
    pub wszHostExe: ::windows::core::PWSTR,
    pub qwHostVersion: u64,
    pub wszPlayerUserAgent: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_READER_CLIENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WM_READER_CLIENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_READER_CLIENTINFO").field("cbSize", &self.cbSize).field("wszLang", &self.wszLang).field("wszBrowserUserAgent", &self.wszBrowserUserAgent).field("wszBrowserWebPage", &self.wszBrowserWebPage).field("qwReserved", &self.qwReserved).field("pReserved", &self.pReserved).field("wszHostExe", &self.wszHostExe).field("qwHostVersion", &self.qwHostVersion).field("wszPlayerUserAgent", &self.wszPlayerUserAgent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_READER_CLIENTINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_READER_CLIENTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_READER_CLIENTINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_READER_CLIENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_READER_STATISTICS {
    pub cbSize: u32,
    pub dwBandwidth: u32,
    pub cPacketsReceived: u32,
    pub cPacketsRecovered: u32,
    pub cPacketsLost: u32,
    pub wQuality: u16,
}
impl ::core::marker::Copy for WM_READER_STATISTICS {}
impl ::core::clone::Clone for WM_READER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_READER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_READER_STATISTICS").field("cbSize", &self.cbSize).field("dwBandwidth", &self.dwBandwidth).field("cPacketsReceived", &self.cPacketsReceived).field("cPacketsRecovered", &self.cPacketsRecovered).field("cPacketsLost", &self.cPacketsLost).field("wQuality", &self.wQuality).finish()
    }
}
unsafe impl ::windows::core::Abi for WM_READER_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_READER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_READER_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_READER_STATISTICS {}
impl ::core::default::Default for WM_READER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_SFEX_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SFEX_NOTASYNCPOINT: WM_SFEX_TYPE = WM_SFEX_TYPE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SFEX_DATALOSS: WM_SFEX_TYPE = WM_SFEX_TYPE(4i32);
impl ::core::marker::Copy for WM_SFEX_TYPE {}
impl ::core::clone::Clone for WM_SFEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_SFEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WM_SFEX_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_SFEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_SFEX_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WM_SF_TYPE(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SF_CLEANPOINT: WM_SF_TYPE = WM_SF_TYPE(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SF_DISCONTINUITY: WM_SF_TYPE = WM_SF_TYPE(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SF_DATALOSS: WM_SF_TYPE = WM_SF_TYPE(4i32);
impl ::core::marker::Copy for WM_SF_TYPE {}
impl ::core::clone::Clone for WM_SF_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WM_SF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WM_SF_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WM_SF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_SF_TYPE").field(&self.0).finish()
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WM_STREAM_PRIORITY_RECORD {
    pub wStreamNumber: u16,
    pub fMandatory: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WM_STREAM_PRIORITY_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WM_STREAM_PRIORITY_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WM_STREAM_PRIORITY_RECORD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_STREAM_PRIORITY_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_STREAM_PRIORITY_RECORD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_STREAM_PRIORITY_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_STREAM_PRIORITY_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_STREAM_TYPE_INFO {
    pub guidMajorType: ::windows::core::GUID,
    pub cbFormat: u32,
}
impl ::core::marker::Copy for WM_STREAM_TYPE_INFO {}
impl ::core::clone::Clone for WM_STREAM_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_STREAM_TYPE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_STREAM_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_STREAM_TYPE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_STREAM_TYPE_INFO {}
impl ::core::default::Default for WM_STREAM_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub pwszContentDescriptor: ::windows::core::PWSTR,
    pub dwLyricsLen: u32,
    pub pbLyrics: *mut u8,
}
impl ::core::marker::Copy for WM_SYNCHRONISED_LYRICS {}
impl ::core::clone::Clone for WM_SYNCHRONISED_LYRICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_SYNCHRONISED_LYRICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_SYNCHRONISED_LYRICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_SYNCHRONISED_LYRICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_SYNCHRONISED_LYRICS {}
impl ::core::default::Default for WM_SYNCHRONISED_LYRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WM_SampleExtensionGUID_ChromaLocation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c5acca0_9276_4b2c_9e4c_a0edefdd217e);
pub const WM_SampleExtensionGUID_ColorSpaceInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf79ada56_30eb_4f2b_9f7a_f24b139a1157);
pub const WM_SampleExtensionGUID_ContentType: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd590dc20_07bc_436c_9cf7_f3bbfbf1a4dc);
pub const WM_SampleExtensionGUID_FileName: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe165ec0e_19ed_45d7_b4a7_25cbd1e28e9b);
pub const WM_SampleExtensionGUID_OutputCleanPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf72a3c6f_6eb4_4ebc_b192_09ad9759e828);
pub const WM_SampleExtensionGUID_PixelAspectRatio: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b1ee554_f9ea_4bc8_821a_376b74e4c4b8);
pub const WM_SampleExtensionGUID_SampleDuration: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6bd9450_867f_4907_83a3_c77921b733ad);
pub const WM_SampleExtensionGUID_SampleProtectionSalt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5403deee_b9ee_438f_aa83_3804997e569d);
pub const WM_SampleExtensionGUID_Timecode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x399595ec_8667_4e2d_8fdb_98814ce76c1e);
pub const WM_SampleExtensionGUID_UserDataInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x732bb4fa_78be_4549_99bd_02db1a55b7a8);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SampleExtension_ChromaLocation_Size: u32 = 1u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SampleExtension_ColorSpaceInfo_Size: u32 = 3u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SampleExtension_ContentType_Size: u32 = 1u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SampleExtension_PixelAspectRatio_Size: u32 = 2u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SampleExtension_SampleDuration_Size: u32 = 2u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const WM_SampleExtension_Timecode_Size: u32 = 14u32;
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_USER_TEXT {
    pub pwszDescription: ::windows::core::PWSTR,
    pub pwszText: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WM_USER_TEXT {}
impl ::core::clone::Clone for WM_USER_TEXT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_USER_TEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_USER_TEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_USER_TEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_USER_TEXT {}
impl ::core::default::Default for WM_USER_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_USER_WEB_URL {
    pub pwszDescription: ::windows::core::PWSTR,
    pub pwszURL: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for WM_USER_WEB_URL {}
impl ::core::clone::Clone for WM_USER_WEB_URL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WM_USER_WEB_URL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_USER_WEB_URL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_USER_WEB_URL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_USER_WEB_URL {}
impl ::core::default::Default for WM_USER_WEB_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_WRITER_STATISTICS {
    pub qwSampleCount: u64,
    pub qwByteCount: u64,
    pub qwDroppedSampleCount: u64,
    pub qwDroppedByteCount: u64,
    pub dwCurrentBitrate: u32,
    pub dwAverageBitrate: u32,
    pub dwExpectedBitrate: u32,
    pub dwCurrentSampleRate: u32,
    pub dwAverageSampleRate: u32,
    pub dwExpectedSampleRate: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_WRITER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_WRITER_STATISTICS")
            .field("qwSampleCount", &self.qwSampleCount)
            .field("qwByteCount", &self.qwByteCount)
            .field("qwDroppedSampleCount", &self.qwDroppedSampleCount)
            .field("qwDroppedByteCount", &self.qwDroppedByteCount)
            .field("dwCurrentBitrate", &self.dwCurrentBitrate)
            .field("dwAverageBitrate", &self.dwAverageBitrate)
            .field("dwExpectedBitrate", &self.dwExpectedBitrate)
            .field("dwCurrentSampleRate", &self.dwCurrentSampleRate)
            .field("dwAverageSampleRate", &self.dwAverageSampleRate)
            .field("dwExpectedSampleRate", &self.dwExpectedSampleRate)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WM_WRITER_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_WRITER_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS {}
impl ::core::default::Default for WM_WRITER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub struct WM_WRITER_STATISTICS_EX {
    pub dwBitratePlusOverhead: u32,
    pub dwCurrentSampleDropRateInQueue: u32,
    pub dwCurrentSampleDropRateInCodec: u32,
    pub dwCurrentSampleDropRateInMultiplexer: u32,
    pub dwTotalSampleDropsInQueue: u32,
    pub dwTotalSampleDropsInCodec: u32,
    pub dwTotalSampleDropsInMultiplexer: u32,
}
impl ::core::marker::Copy for WM_WRITER_STATISTICS_EX {}
impl ::core::clone::Clone for WM_WRITER_STATISTICS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WM_WRITER_STATISTICS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_WRITER_STATISTICS_EX")
            .field("dwBitratePlusOverhead", &self.dwBitratePlusOverhead)
            .field("dwCurrentSampleDropRateInQueue", &self.dwCurrentSampleDropRateInQueue)
            .field("dwCurrentSampleDropRateInCodec", &self.dwCurrentSampleDropRateInCodec)
            .field("dwCurrentSampleDropRateInMultiplexer", &self.dwCurrentSampleDropRateInMultiplexer)
            .field("dwTotalSampleDropsInQueue", &self.dwTotalSampleDropsInQueue)
            .field("dwTotalSampleDropsInCodec", &self.dwTotalSampleDropsInCodec)
            .field("dwTotalSampleDropsInMultiplexer", &self.dwTotalSampleDropsInMultiplexer)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WM_WRITER_STATISTICS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WM_WRITER_STATISTICS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS_EX {}
impl ::core::default::Default for WM_WRITER_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _AM_ASFWRITERCONFIG_PARAM(pub i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const AM_CONFIGASFWRITER_PARAM_AUTOINDEX: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(1i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const AM_CONFIGASFWRITER_PARAM_MULTIPASS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(2i32);
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const AM_CONFIGASFWRITER_PARAM_DONTCOMPRESS: _AM_ASFWRITERCONFIG_PARAM = _AM_ASFWRITERCONFIG_PARAM(3i32);
impl ::core::marker::Copy for _AM_ASFWRITERCONFIG_PARAM {}
impl ::core::clone::Clone for _AM_ASFWRITERCONFIG_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _AM_ASFWRITERCONFIG_PARAM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _AM_ASFWRITERCONFIG_PARAM {
    type Abi = Self;
}
impl ::core::fmt::Debug for _AM_ASFWRITERCONFIG_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AM_ASFWRITERCONFIG_PARAM").field(&self.0).finish()
    }
}
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_dwWMContentAttributes: u32 = 5u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_dwWMNSCAttributes: u32 = 5u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_dwWMSpecialAttributes: u32 = 20u32;
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszASFLeakyBucketPairs: &'static str = "ASFLeakyBucketPairs";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszAllowInterlacedOutput: &'static str = "AllowInterlacedOutput";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszAverageLevel: &'static str = "AverageLevel";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszBufferAverage: &'static str = "Buffer Average";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszComplexity: &'static str = "_COMPLEXITYEX";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszComplexityLive: &'static str = "_COMPLEXITYEXLIVE";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszComplexityMax: &'static str = "_COMPLEXITYEXMAX";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszComplexityOffline: &'static str = "_COMPLEXITYEXOFFLINE";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszDecoderComplexityRequested: &'static str = "_DECODERCOMPLEXITYPROFILE";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszDedicatedDeliveryThread: &'static str = "DedicatedDeliveryThread";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszDeinterlaceMode: &'static str = "DeinterlaceMode";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszDeliverOnReceive: &'static str = "DeliverOnReceive";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszDeviceConformanceTemplate: &'static str = "DeviceConformanceTemplate";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszDynamicRangeControl: &'static str = "DynamicRangeControl";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszEDL: &'static str = "_EDL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszEarlyDataDelivery: &'static str = "EarlyDataDelivery";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszEnableDiscreteOutput: &'static str = "EnableDiscreteOutput";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszEnableFrameInterpolation: &'static str = "EnableFrameInterpolation";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszEnableWMAProSPDIFOutput: &'static str = "EnableWMAProSPDIFOutput";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszFailSeekOnError: &'static str = "FailSeekOnError";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszFixedFrameRate: &'static str = "FixedFrameRate";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszFold6To2Channels3: &'static str = "Fold6To2Channels3";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszFoldToChannelsTemplate: &'static str = "Fold%luTo%luChannels%lu";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszInitialPatternForInverseTelecine: &'static str = "InitialPatternForInverseTelecine";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszInterlacedCoding: &'static str = "InterlacedCoding";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszIsVBRSupported: &'static str = "_ISVBRSUPPORTED";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszJPEGCompressionQuality: &'static str = "JPEGCompressionQuality";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszJustInTimeDecode: &'static str = "JustInTimeDecode";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszMixedClassMode: &'static str = "MixedClassMode";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszMusicClassMode: &'static str = "MusicClassMode";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszMusicSpeechClassMode: &'static str = "MusicSpeechClassMode";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszNeedsPreviousSample: &'static str = "NeedsPreviousSample";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszNumPasses: &'static str = "_PASSESUSED";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszOriginalSourceFormatTag: &'static str = "_SOURCEFORMATTAG";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszOriginalWaveFormat: &'static str = "_ORIGINALWAVEFORMAT";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszPeakValue: &'static str = "PeakValue";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszPermitSeeksBeyondEndOfStream: &'static str = "PermitSeeksBeyondEndOfStream";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszReloadIndexOnSeek: &'static str = "ReloadIndexOnSeek";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszScrambledAudio: &'static str = "ScrambledAudio";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszSingleOutputBuffer: &'static str = "SingleOutputBuffer";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszSoftwareScaling: &'static str = "SoftwareScaling";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszSourceBufferTime: &'static str = "SourceBufferTime";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszSourceMaxBytesAtOnce: &'static str = "SourceMaxBytesAtOnce";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszSpeakerConfig: &'static str = "SpeakerConfig";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszSpeechCaps: &'static str = "SpeechFormatCap";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszSpeechClassMode: &'static str = "SpeechClassMode";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszStreamLanguage: &'static str = "StreamLanguage";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszStreamNumIndexObjects: &'static str = "StreamNumIndexObjects";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszUsePacketAtSeekPoint: &'static str = "UsePacketAtSeekPoint";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszVBRBitrateMax: &'static str = "_RMAX";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszVBRBufferWindowMax: &'static str = "_BMAX";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszVBREnabled: &'static str = "_VBRENABLED";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszVBRPeak: &'static str = "VBR Peak";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszVBRQuality: &'static str = "_VBRQUALITY";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszVideoSampleDurations: &'static str = "VideoSampleDurations";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMADID: &'static str = "WM/ADID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMASFPacketCount: &'static str = "WM/ASFPacketCount";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMASFSecurityObjectsSize: &'static str = "WM/ASFSecurityObjectsSize";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAlbumArtist: &'static str = "WM/AlbumArtist";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAlbumArtistSort: &'static str = "WM/AlbumArtistSort";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAlbumCoverURL: &'static str = "WM/AlbumCoverURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAlbumTitle: &'static str = "WM/AlbumTitle";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAlbumTitleSort: &'static str = "WM/AlbumTitleSort";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAspectRatioX: &'static str = "AspectRatioX";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAspectRatioY: &'static str = "AspectRatioY";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAudioFileURL: &'static str = "WM/AudioFileURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAudioSourceURL: &'static str = "WM/AudioSourceURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAuthor: &'static str = "Author";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAuthorSort: &'static str = "AuthorSort";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMAuthorURL: &'static str = "WM/AuthorURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMBannerImageData: &'static str = "BannerImageData";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMBannerImageType: &'static str = "BannerImageType";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMBannerImageURL: &'static str = "BannerImageURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMBeatsPerMinute: &'static str = "WM/BeatsPerMinute";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMBitrate: &'static str = "Bitrate";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMBroadcast: &'static str = "Broadcast";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMCategory: &'static str = "WM/Category";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMCodec: &'static str = "WM/Codec";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMComposer: &'static str = "WM/Composer";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMComposerSort: &'static str = "WM/ComposerSort";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMConductor: &'static str = "WM/Conductor";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMContainerFormat: &'static str = "WM/ContainerFormat";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMContentDistributor: &'static str = "WM/ContentDistributor";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMContentGroupDescription: &'static str = "WM/ContentGroupDescription";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMCopyright: &'static str = "Copyright";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMCopyrightURL: &'static str = "CopyrightURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMCurrentBitrate: &'static str = "CurrentBitrate";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM: &'static str = "WM/DRM";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_ContentID: &'static str = "DRM_ContentID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_Flags: &'static str = "DRM_Flags";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_HeaderSignPrivKey: &'static str = "DRM_HeaderSignPrivKey";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_IndividualizedVersion: &'static str = "DRM_IndividualizedVersion";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_KeyID: &'static str = "DRM_KeyID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_KeySeed: &'static str = "DRM_KeySeed";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_LASignatureCert: &'static str = "DRM_LASignatureCert";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_LASignatureLicSrvCert: &'static str = "DRM_LASignatureLicSrvCert";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_LASignaturePrivKey: &'static str = "DRM_LASignaturePrivKey";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_LASignatureRootCert: &'static str = "DRM_LASignatureRootCert";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_Level: &'static str = "DRM_Level";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_LicenseAcqURL: &'static str = "DRM_LicenseAcqURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_SourceID: &'static str = "DRM_SourceID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDRM_V1LicenseAcqURL: &'static str = "DRM_V1LicenseAcqURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDVDID: &'static str = "WM/DVDID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDescription: &'static str = "Description";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDirector: &'static str = "WM/Director";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMDuration: &'static str = "Duration";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMEncodedBy: &'static str = "WM/EncodedBy";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMEncodingSettings: &'static str = "WM/EncodingSettings";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMEncodingTime: &'static str = "WM/EncodingTime";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMEpisodeNumber: &'static str = "WM/EpisodeNumber";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMFileSize: &'static str = "FileSize";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMGenre: &'static str = "WM/Genre";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMGenreID: &'static str = "WM/GenreID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMHasArbitraryDataStream: &'static str = "HasArbitraryDataStream";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMHasAttachedImages: &'static str = "HasAttachedImages";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMHasAudio: &'static str = "HasAudio";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMHasFileTransferStream: &'static str = "HasFileTransferStream";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMHasImage: &'static str = "HasImage";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMHasScript: &'static str = "HasScript";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMHasVideo: &'static str = "HasVideo";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMISAN: &'static str = "WM/ISAN";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMISRC: &'static str = "WM/ISRC";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMInitialKey: &'static str = "WM/InitialKey";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMIsCompilation: &'static str = "WM/IsCompilation";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMIsVBR: &'static str = "IsVBR";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMLanguage: &'static str = "WM/Language";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMLyrics: &'static str = "WM/Lyrics";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMLyrics_Synchronised: &'static str = "WM/Lyrics_Synchronised";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMCDI: &'static str = "WM/MCDI";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaClassPrimaryID: &'static str = "WM/MediaClassPrimaryID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaClassSecondaryID: &'static str = "WM/MediaClassSecondaryID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaCredits: &'static str = "WM/MediaCredits";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaIsDelay: &'static str = "WM/MediaIsDelay";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaIsFinale: &'static str = "WM/MediaIsFinale";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaIsLive: &'static str = "WM/MediaIsLive";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaIsPremiere: &'static str = "WM/MediaIsPremiere";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaIsRepeat: &'static str = "WM/MediaIsRepeat";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaIsSAP: &'static str = "WM/MediaIsSAP";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaIsStereo: &'static str = "WM/MediaIsStereo";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaIsSubtitled: &'static str = "WM/MediaIsSubtitled";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaIsTape: &'static str = "WM/MediaIsTape";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaNetworkAffiliation: &'static str = "WM/MediaNetworkAffiliation";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaOriginalBroadcastDateTime: &'static str = "WM/MediaOriginalBroadcastDateTime";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaOriginalChannel: &'static str = "WM/MediaOriginalChannel";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaStationCallSign: &'static str = "WM/MediaStationCallSign";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMediaStationName: &'static str = "WM/MediaStationName";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMModifiedBy: &'static str = "WM/ModifiedBy";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMMood: &'static str = "WM/Mood";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMNSCAddress: &'static str = "NSC_Address";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMNSCDescription: &'static str = "NSC_Description";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMNSCEmail: &'static str = "NSC_Email";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMNSCName: &'static str = "NSC_Name";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMNSCPhone: &'static str = "NSC_Phone";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMNumberOfFrames: &'static str = "NumberOfFrames";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMOptimalBitrate: &'static str = "OptimalBitrate";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMOriginalAlbumTitle: &'static str = "WM/OriginalAlbumTitle";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMOriginalArtist: &'static str = "WM/OriginalArtist";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMOriginalFilename: &'static str = "WM/OriginalFilename";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMOriginalLyricist: &'static str = "WM/OriginalLyricist";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMOriginalReleaseTime: &'static str = "WM/OriginalReleaseTime";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMOriginalReleaseYear: &'static str = "WM/OriginalReleaseYear";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMParentalRating: &'static str = "WM/ParentalRating";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMParentalRatingReason: &'static str = "WM/ParentalRatingReason";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMPartOfSet: &'static str = "WM/PartOfSet";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMPeakBitrate: &'static str = "WM/PeakBitrate";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMPeriod: &'static str = "WM/Period";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMPicture: &'static str = "WM/Picture";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMPlaylistDelay: &'static str = "WM/PlaylistDelay";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMProducer: &'static str = "WM/Producer";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMPromotionURL: &'static str = "WM/PromotionURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMProtected: &'static str = "Is_Protected";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMProtectionType: &'static str = "WM/ProtectionType";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMProvider: &'static str = "WM/Provider";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMProviderCopyright: &'static str = "WM/ProviderCopyright";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMProviderRating: &'static str = "WM/ProviderRating";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMProviderStyle: &'static str = "WM/ProviderStyle";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMPublisher: &'static str = "WM/Publisher";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMRadioStationName: &'static str = "WM/RadioStationName";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMRadioStationOwner: &'static str = "WM/RadioStationOwner";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMRating: &'static str = "Rating";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMSeasonNumber: &'static str = "WM/SeasonNumber";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMSeekable: &'static str = "Seekable";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMSharedUserRating: &'static str = "WM/SharedUserRating";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMSignature_Name: &'static str = "Signature_Name";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMSkipBackward: &'static str = "Can_Skip_Backward";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMSkipForward: &'static str = "Can_Skip_Forward";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMStreamTypeInfo: &'static str = "WM/StreamTypeInfo";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMStridable: &'static str = "Stridable";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMSubTitle: &'static str = "WM/SubTitle";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMSubTitleDescription: &'static str = "WM/SubTitleDescription";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMSubscriptionContentID: &'static str = "WM/SubscriptionContentID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMText: &'static str = "WM/Text";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMTitle: &'static str = "Title";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMTitleSort: &'static str = "TitleSort";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMToolName: &'static str = "WM/ToolName";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMToolVersion: &'static str = "WM/ToolVersion";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMTrack: &'static str = "WM/Track";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMTrackNumber: &'static str = "WM/TrackNumber";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMTrusted: &'static str = "Is_Trusted";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMUniqueFileIdentifier: &'static str = "WM/UniqueFileIdentifier";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMUse_Advanced_DRM: &'static str = "Use_Advanced_DRM";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMUse_DRM: &'static str = "Use_DRM";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMUserWebURL: &'static str = "WM/UserWebURL";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMVideoClosedCaptioning: &'static str = "WM/VideoClosedCaptioning";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMVideoFrameRate: &'static str = "WM/VideoFrameRate";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMVideoHeight: &'static str = "WM/VideoHeight";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMVideoWidth: &'static str = "WM/VideoWidth";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMADRCAverageReference: &'static str = "WM/WMADRCAverageReference";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMADRCAverageTarget: &'static str = "WM/WMADRCAverageTarget";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMADRCPeakReference: &'static str = "WM/WMADRCPeakReference";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMADRCPeakTarget: &'static str = "WM/WMADRCPeakTarget";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMCPDistributor: &'static str = "WM/WMCPDistributor";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMCPDistributorID: &'static str = "WM/WMCPDistributorID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMCollectionGroupID: &'static str = "WM/WMCollectionGroupID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMCollectionID: &'static str = "WM/WMCollectionID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMContentID: &'static str = "WM/WMContentID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMShadowFileSourceDRMType: &'static str = "WM/WMShadowFileSourceDRMType";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWMShadowFileSourceFileType: &'static str = "WM/WMShadowFileSourceFileType";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMWriter: &'static str = "WM/Writer";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWMYear: &'static str = "WM/Year";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWatermarkCLSID: &'static str = "WatermarkCLSID";
#[doc = "*Required features: 'Win32_Media_WindowsMediaFormat'*"]
pub const g_wszWatermarkConfig: &'static str = "WatermarkConfig";
#[cfg(feature = "implement")]
::core::include!("impl.rs");
