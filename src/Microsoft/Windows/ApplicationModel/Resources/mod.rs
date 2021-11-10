#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownResourceQualifierNameStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownResourceQualifierNameStatics {
    type Vtable = IKnownResourceQualifierNameStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3714899676,
        21915,
        20680,
        [172, 83, 130, 254, 33, 249, 21, 243],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownResourceQualifierNameStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceCandidate(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceCandidate {
    type Vtable = IResourceCandidate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1817492492,
        61214,
        22456,
        [180, 120, 52, 254, 206, 115, 115, 86],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ResourceCandidateKind,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceCandidateFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceCandidateFactory {
    type Vtable = IResourceCandidateFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3140169976,
        49563,
        24387,
        [136, 217, 105, 173, 114, 138, 50, 244],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidateFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        kind: ResourceCandidateKind,
        data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        data_array_size: u32,
        data: *const u8,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceContext(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceContext {
    type Vtable = IResourceContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2533050588,
        63357,
        22015,
        [175, 18, 52, 134, 30, 61, 73, 57],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoader(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoader {
    type Vtable = IResourceLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3158275775,
        55878,
        21709,
        [135, 21, 139, 138, 175, 22, 234, 172],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoader_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        resourceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        resourceuri: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoaderFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoaderFactory {
    type Vtable = IResourceLoaderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2266989482,
        64308,
        20694,
        [185, 185, 44, 53, 243, 255, 192, 4],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        resourcemap: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceLoaderStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceLoaderStatics {
    type Vtable = IResourceLoaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3969681738,
        5222,
        24367,
        [142, 238, 167, 12, 189, 43, 81, 187],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceLoaderStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceManager {
    type Vtable = IResourceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2887946735,
        33214,
        23705,
        [160, 174, 188, 238, 1, 128, 184, 168],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceManagerFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceManagerFactory {
    type Vtable = IResourceManagerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3601658255,
        17802,
        21339,
        [165, 196, 172, 45, 196, 228, 144, 153],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceMap(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceMap {
    type Vtable = IResourceMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1253824956,
        57166,
        23675,
        [129, 44, 126, 123, 176, 194, 35, 119],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceMap_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reference: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reference: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        resource: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        resource: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        context: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        context: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        resource: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        resource: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        context: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IResourceNotFoundEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1688973451,
        59261,
        23334,
        [131, 15, 21, 148, 30, 14, 130, 0],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceNotFoundEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        candidate: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
pub struct KnownResourceQualifierName {}
impl KnownResourceQualifierName {
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn Contrast() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn Custom() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn DeviceFamily() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn HomeRegion() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn Language() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn LayoutDirection() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn Scale() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn TargetSize() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn Theme() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownResourceQualifierNameStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownResourceQualifierNameStatics<
        R,
        F: FnOnce(&IKnownResourceQualifierNameStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            KnownResourceQualifierName,
            IKnownResourceQualifierNameStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownResourceQualifierName {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.Resources.KnownResourceQualifierName";
}
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct MrtContract(pub u8);
#[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ResourceCandidate(pub ::windows::runtime::IInspectable);
impl ResourceCandidate {
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn ValueAsString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn ValueAsBytes(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                ::windows::runtime::Array::<u8>::set_abi_len(&mut result__),
                &mut result__ as *mut _ as _,
            )
            .and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ResourceCandidateKind> {
        let this = self;
        unsafe {
            let mut result__: ResourceCandidateKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ResourceCandidateKind>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn QualifierValues(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn CreateInstance<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        kind: ResourceCandidateKind,
        data: Param1,
    ) -> ::windows::runtime::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                kind,
                data.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn CreateInstance2(
        data: &[<u8 as ::windows::runtime::DefaultType>::DefaultType],
    ) -> ::windows::runtime::Result<ResourceCandidate> {
        Self::IResourceCandidateFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                data.len() as u32,
                ::std::mem::transmute(data.as_ptr()),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        })
    }
    pub fn IResourceCandidateFactory<
        R,
        F: FnOnce(&IResourceCandidateFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ResourceCandidate,
            IResourceCandidateFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceCandidate {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceCandidate;{6c54bc0c-ef1e-57b8-b478-34fece737356})" ) ;
}
unsafe impl ::windows::runtime::Interface for ResourceCandidate {
    type Vtable = IResourceCandidate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1817492492,
        61214,
        22456,
        [180, 120, 52, 254, 206, 115, 115, 86],
    );
}
impl ::windows::runtime::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceCandidate";
}
impl ::std::convert::From<ResourceCandidate> for ::windows::runtime::IUnknown {
    fn from(value: ResourceCandidate) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ResourceCandidate> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceCandidate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ResourceCandidate> for ::windows::runtime::IInspectable {
    fn from(value: ResourceCandidate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ResourceCandidate> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceCandidate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceCandidate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ResourceCandidate
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ResourceCandidate {}
unsafe impl ::std::marker::Sync for ResourceCandidate {}
#[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const Unknown: ResourceCandidateKind = ResourceCandidateKind(0i32);
    pub const String: ResourceCandidateKind = ResourceCandidateKind(1i32);
    pub const FilePath: ResourceCandidateKind = ResourceCandidateKind(2i32);
    pub const EmbeddedData: ResourceCandidateKind = ResourceCandidateKind(3i32);
}
impl ::std::convert::From<i32> for ResourceCandidateKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ResourceCandidateKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.Windows.ApplicationModel.Resources.ResourceCandidateKind;i4)",
    );
}
impl ::windows::runtime::DefaultType for ResourceCandidateKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ResourceContext(pub ::windows::runtime::IInspectable);
impl ResourceContext {
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn QualifierValues(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceContext {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceContext;{96fb48dc-f77d-55ff-af12-34861e3d4939})" ) ;
}
unsafe impl ::windows::runtime::Interface for ResourceContext {
    type Vtable = IResourceContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2533050588,
        63357,
        22015,
        [175, 18, 52, 134, 30, 61, 73, 57],
    );
}
impl ::windows::runtime::RuntimeName for ResourceContext {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceContext";
}
impl ::std::convert::From<ResourceContext> for ::windows::runtime::IUnknown {
    fn from(value: ResourceContext) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ResourceContext> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceContext) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ResourceContext> for ::windows::runtime::IInspectable {
    fn from(value: ResourceContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ResourceContext> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ResourceContext
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ResourceContext {}
unsafe impl ::std::marker::Sync for ResourceContext {}
#[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ResourceLoader(pub ::windows::runtime::IInspectable);
impl ResourceLoader {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ResourceLoader,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn GetString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        resourceid: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                resourceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn GetStringForUri<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::Uri>,
    >(
        &self,
        resourceuri: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                resourceuri.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        filename: Param0,
    ) -> ::windows::runtime::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                filename.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn CreateInstance2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        filename: Param0,
        resourcemap: Param1,
    ) -> ::windows::runtime::Result<ResourceLoader> {
        Self::IResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                filename.into_param().abi(),
                resourcemap.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceLoader>(result__)
        })
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn GetDefaultResourceFilePath() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IResourceLoaderStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IResourceLoaderFactory<
        R,
        F: FnOnce(&IResourceLoaderFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ResourceLoader,
            IResourceLoaderFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceLoaderStatics<
        R,
        F: FnOnce(&IResourceLoaderStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ResourceLoader,
            IResourceLoaderStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceLoader {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceLoader;{bc3f76bf-da46-54cd-8715-8b8aaf16eaac})" ) ;
}
unsafe impl ::windows::runtime::Interface for ResourceLoader {
    type Vtable = IResourceLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3158275775,
        55878,
        21709,
        [135, 21, 139, 138, 175, 22, 234, 172],
    );
}
impl ::windows::runtime::RuntimeName for ResourceLoader {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceLoader";
}
impl ::std::convert::From<ResourceLoader> for ::windows::runtime::IUnknown {
    fn from(value: ResourceLoader) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ResourceLoader> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceLoader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ResourceLoader> for ::windows::runtime::IInspectable {
    fn from(value: ResourceLoader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ResourceLoader> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceLoader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ResourceLoader
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ResourceLoader {}
unsafe impl ::std::marker::Sync for ResourceLoader {}
#[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ResourceManager(pub ::windows::runtime::IInspectable);
impl ResourceManager {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ResourceManager,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn MainResourceMap(&self) -> ::windows::runtime::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn CreateResourceContext(&self) -> ::windows::runtime::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn ResourceNotFound<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            ::windows::Foundation::TypedEventHandler<ResourceManager, ResourceNotFoundEventArgs>,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: ::windows::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn RemoveResourceNotFound<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        filename: Param0,
    ) -> ::windows::runtime::Result<ResourceManager> {
        Self::IResourceManagerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                filename.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceManager>(result__)
        })
    }
    pub fn IResourceManagerFactory<
        R,
        F: FnOnce(&IResourceManagerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ResourceManager,
            IResourceManagerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceManager {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceManager;{ac2291ef-81be-5c99-a0ae-bcee0180b8a8})" ) ;
}
unsafe impl ::windows::runtime::Interface for ResourceManager {
    type Vtable = IResourceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2887946735,
        33214,
        23705,
        [160, 174, 188, 238, 1, 128, 184, 168],
    );
}
impl ::windows::runtime::RuntimeName for ResourceManager {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceManager";
}
impl ::std::convert::From<ResourceManager> for ::windows::runtime::IUnknown {
    fn from(value: ResourceManager) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ResourceManager> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ResourceManager> for ::windows::runtime::IInspectable {
    fn from(value: ResourceManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ResourceManager> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ResourceManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ResourceManager {}
unsafe impl ::std::marker::Sync for ResourceManager {}
#[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ResourceMap(pub ::windows::runtime::IInspectable);
impl ResourceMap {
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn ResourceCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn GetSubtree<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        reference: Param0,
    ) -> ::windows::runtime::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                reference.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn TryGetSubtree<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        reference: Param0,
    ) -> ::windows::runtime::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                reference.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceMap>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        resource: Param0,
    ) -> ::windows::runtime::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                resource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn GetValueWithContext<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ResourceContext>,
    >(
        &self,
        resource: Param0,
        context: Param1,
    ) -> ::windows::runtime::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                resource.into_param().abi(),
                context.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn GetValueByIndex(
        &self,
        index: u32,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ResourceCandidate,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                index,
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ResourceCandidate,
            >>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn GetValueByIndexWithContext<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ResourceContext>,
    >(
        &self,
        index: u32,
        context: Param1,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IKeyValuePair<
            ::windows::runtime::HSTRING,
            ResourceCandidate,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                index,
                context.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IKeyValuePair<
                ::windows::runtime::HSTRING,
                ResourceCandidate,
            >>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn TryGetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        resource: Param0,
    ) -> ::windows::runtime::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                resource.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn TryGetValueWithContext<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ResourceContext>,
    >(
        &self,
        resource: Param0,
        context: Param1,
    ) -> ::windows::runtime::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                resource.into_param().abi(),
                context.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ResourceCandidate>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceMap {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceMap;{4abbd9bc-df4e-5c7b-812c-7e7bb0c22377})" ) ;
}
unsafe impl ::windows::runtime::Interface for ResourceMap {
    type Vtable = IResourceMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1253824956,
        57166,
        23675,
        [129, 44, 126, 123, 176, 194, 35, 119],
    );
}
impl ::windows::runtime::RuntimeName for ResourceMap {
    const NAME: &'static str = "Microsoft.Windows.ApplicationModel.Resources.ResourceMap";
}
impl ::std::convert::From<ResourceMap> for ::windows::runtime::IUnknown {
    fn from(value: ResourceMap) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ResourceMap> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceMap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ResourceMap> for ::windows::runtime::IInspectable {
    fn from(value: ResourceMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ResourceMap> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResourceMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ResourceMap {}
unsafe impl ::std::marker::Sync for ResourceMap {}
#[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ResourceNotFoundEventArgs(pub ::windows::runtime::IInspectable);
impl ResourceNotFoundEventArgs {
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn Context(&self) -> ::windows::runtime::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ResourceContext>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Windows_ApplicationModel_Resources`*"]
    pub fn SetResolvedCandidate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ResourceCandidate>,
    >(
        &self,
        candidate: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                candidate.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResourceNotFoundEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.Windows.ApplicationModel.Resources.ResourceNotFoundEventArgs;{64abb08b-e77d-5b26-830f-15941e0e8200})" ) ;
}
unsafe impl ::windows::runtime::Interface for ResourceNotFoundEventArgs {
    type Vtable = IResourceNotFoundEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1688973451,
        59261,
        23334,
        [131, 15, 21, 148, 30, 14, 130, 0],
    );
}
impl ::windows::runtime::RuntimeName for ResourceNotFoundEventArgs {
    const NAME: &'static str =
        "Microsoft.Windows.ApplicationModel.Resources.ResourceNotFoundEventArgs";
}
impl ::std::convert::From<ResourceNotFoundEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ResourceNotFoundEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ResourceNotFoundEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ResourceNotFoundEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ResourceNotFoundEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ResourceNotFoundEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ResourceNotFoundEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ResourceNotFoundEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ResourceNotFoundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ResourceNotFoundEventArgs {}
unsafe impl ::std::marker::Sync for ResourceNotFoundEventArgs {}
