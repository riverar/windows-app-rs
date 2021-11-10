#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ElementCompositionPreview(pub ::windows::runtime::IInspectable);
impl ElementCompositionPreview {
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn GetElementVisual<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(
        element: Param0,
    ) -> ::windows::runtime::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn GetElementChildVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::UIElement>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<super::super::Composition::Visual> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Composition::Visual>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn SetElementChildVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::UIElement>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Composition::Visual>,
    >(
        element: Param0,
        visual: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                visual.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`, `UI_Xaml_Controls`*"]
    pub fn GetScrollViewerManipulationPropertySet<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Controls::ScrollViewer>,
    >(
        scrollviewer: Param0,
    ) -> ::windows::runtime::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                scrollviewer.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn SetImplicitShowAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::UIElement>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        element: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn SetImplicitHideAnimation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::UIElement>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Composition::ICompositionAnimationBase>,
    >(
        element: Param0,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                animation.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub fn SetIsTranslationEnabled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::UIElement>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Hosting`, `UI_Composition`*"]
    pub fn GetPointerPositionPropertySet<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::UIElement>,
    >(
        targetelement: Param0,
    ) -> ::windows::runtime::Result<super::super::Composition::CompositionPropertySet> {
        Self::IElementCompositionPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                targetelement.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Composition::CompositionPropertySet>(result__)
        })
    }
    pub fn IElementCompositionPreviewStatics<
        R,
        F: FnOnce(&IElementCompositionPreviewStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ElementCompositionPreview,
            IElementCompositionPreviewStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ElementCompositionPreview {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Hosting.ElementCompositionPreview;{c8ad1ef4-a93f-5a25-85bd-7c498d9856d3})" ) ;
}
unsafe impl ::windows::runtime::Interface for ElementCompositionPreview {
    type Vtable = IElementCompositionPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3366788852,
        43327,
        23077,
        [133, 189, 124, 73, 141, 152, 86, 211],
    );
}
impl ::windows::runtime::RuntimeName for ElementCompositionPreview {
    const NAME: &'static str = "Microsoft.UI.Xaml.Hosting.ElementCompositionPreview";
}
impl ::std::convert::From<ElementCompositionPreview> for ::windows::runtime::IUnknown {
    fn from(value: ElementCompositionPreview) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ElementCompositionPreview> for ::windows::runtime::IUnknown {
    fn from(value: &ElementCompositionPreview) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ElementCompositionPreview
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a ElementCompositionPreview
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ElementCompositionPreview> for ::windows::runtime::IInspectable {
    fn from(value: ElementCompositionPreview) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ElementCompositionPreview> for ::windows::runtime::IInspectable {
    fn from(value: &ElementCompositionPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ElementCompositionPreview
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ElementCompositionPreview
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ElementCompositionPreview {}
unsafe impl ::std::marker::Sync for ElementCompositionPreview {}
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct HostingContract(pub u8);
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDesktopWindowXamlSourceNative(pub ::windows::runtime::IUnknown);
impl IDesktopWindowXamlSourceNative {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn AttachToWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::Win32::Foundation::HWND>,
    >(
        &self,
        parentwnd: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            parentwnd.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn WindowHandle(
        &self,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(hwnd),
        )
        .ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn PreTranslateMessage(
        &self,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(message),
            ::std::mem::transmute(result),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        183119654,
        64207,
        17800,
        [140, 244, 52, 85, 81, 36, 219, 50],
    );
}
impl ::std::convert::From<IDesktopWindowXamlSourceNative> for ::windows::runtime::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDesktopWindowXamlSourceNative> for ::windows::runtime::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDesktopWindowXamlSourceNative
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a IDesktopWindowXamlSourceNative
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        parentwnd: ::windows::Win32::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwnd: *mut ::windows::Win32::Foundation::HWND,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        message: *const ::windows::Win32::UI::WindowsAndMessaging::MSG,
        result: *mut ::windows::Win32::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IElementCompositionPreview(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IElementCompositionPreview {
    type Vtable = IElementCompositionPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3366788852,
        43327,
        23077,
        [133, 189, 124, 73, 141, 152, 86, 211],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreview_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IElementCompositionPreviewStatics {
    type Vtable = IElementCompositionPreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2228902508,
        3322,
        21291,
        [155, 21, 204, 217, 134, 55, 67, 66],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IElementCompositionPreviewStatics_abi(
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
    #[cfg(feature = "UI_Composition")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scrollviewer: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Controls")))] usize,
    #[cfg(feature = "UI_Composition")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        targetelement: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IFindReferenceTargetsCallback(pub ::windows::runtime::IUnknown);
impl IFindReferenceTargetsCallback {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn FoundTrackerTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IReferenceTrackerTarget>,
    >(
        &self,
        target: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            target.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        78858348,
        18055,
        16937,
        [141, 20, 80, 90, 181, 132, 221, 136],
    );
}
impl ::std::convert::From<IFindReferenceTargetsCallback> for ::windows::runtime::IUnknown {
    fn from(value: IFindReferenceTargetsCallback) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFindReferenceTargetsCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IFindReferenceTargetsCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IFindReferenceTargetsCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a IFindReferenceTargetsCallback
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        target: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IReferenceTracker(pub ::windows::runtime::IUnknown);
impl IReferenceTracker {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn FindTrackerTargets<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IFindReferenceTargetsCallback>,
    >(
        &self,
        callback: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            callback.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn GetReferenceTrackerManager(
        &self,
    ) -> ::windows::runtime::Result<IReferenceTrackerManager> {
        let mut result__: <IReferenceTrackerManager as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IReferenceTrackerManager>(result__)
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTracker {
    type Vtable = IReferenceTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        299086138,
        6158,
        18313,
        [168, 190, 119, 18, 136, 40, 147, 230],
    );
}
impl ::std::convert::From<IReferenceTracker> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTracker) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IReferenceTracker> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReferenceTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReferenceTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        callback: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IReferenceTrackerExtension(pub ::windows::runtime::IUnknown);
impl IReferenceTrackerExtension {}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1317633194,
        22997,
        17939,
        [143, 140, 247, 235, 209, 243, 153, 176],
    );
}
impl ::std::convert::From<IReferenceTrackerExtension> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTrackerExtension) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IReferenceTrackerExtension> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTrackerExtension) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IReferenceTrackerExtension
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a IReferenceTrackerExtension
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IReferenceTrackerHost(pub ::windows::runtime::IUnknown);
impl IReferenceTrackerHost {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn DisconnectUnusedReferenceSources(
        &self,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(options),
        )
        .ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn GetTrackerTarget<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        unknown: Param0,
    ) -> ::windows::runtime::Result<IReferenceTrackerTarget> {
        let mut result__: <IReferenceTrackerTarget as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            unknown.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IReferenceTrackerTarget>(result__)
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bytesallocated),
        )
        .ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn RemoveMemoryPressure(
        &self,
        bytesallocated: u64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bytesallocated),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        698817642,
        15426,
        17430,
        [163, 157, 226, 130, 90, 7, 167, 115],
    );
}
impl ::std::convert::From<IReferenceTrackerHost> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTrackerHost) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IReferenceTrackerHost> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTrackerHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReferenceTrackerHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a IReferenceTrackerHost
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        options: __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unknown: ::windows::runtime::RawPtr,
        newreference: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bytesallocated: u64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bytesallocated: u64,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IReferenceTrackerManager(pub ::windows::runtime::IUnknown);
impl IReferenceTrackerManager {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn FindTrackerTargetsCompleted(
        &self,
        findfailed: u8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(findfailed),
        )
        .ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn SetReferenceTrackerHost<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IReferenceTrackerHost>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            value.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1022461108,
        31947,
        19930,
        [132, 85, 126, 108, 233, 154, 50, 152],
    );
}
impl ::std::convert::From<IReferenceTrackerManager> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTrackerManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IReferenceTrackerManager> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTrackerManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IReferenceTrackerManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a IReferenceTrackerManager
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        findfailed: u8,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IReferenceTrackerTarget(pub ::windows::runtime::IUnknown);
impl IReferenceTrackerTarget {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
        ))
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn Peg(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn Unpeg(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1690125304,
        49134,
        20164,
        [183, 235, 41, 53, 21, 141, 174, 33],
    );
}
impl ::std::convert::From<IReferenceTrackerTarget> for ::windows::runtime::IUnknown {
    fn from(value: IReferenceTrackerTarget) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IReferenceTrackerTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IReferenceTrackerTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IReferenceTrackerTarget
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a IReferenceTrackerTarget
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ITrackerOwner(pub ::windows::runtime::IUnknown);
impl ITrackerOwner {
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn CreateTrackerHandle(
        &self,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(returnvalue),
        )
        .ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn DeleteTrackerHandle(
        &self,
        handle: *mut TrackerHandle__,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(handle),
        )
        .ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn SetTrackerValue<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        handle: *mut TrackerHandle__,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(handle),
            value.into_param().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `UI_Xaml_Hosting`*"]
    pub unsafe fn TryGetSafeTrackerValue(
        &self,
        handle: *mut TrackerHandle__,
        returnvalue: *mut ::std::option::Option<::windows::runtime::IUnknown>,
    ) -> u8 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(handle),
            ::std::mem::transmute(returnvalue),
        ))
    }
}
unsafe impl ::windows::runtime::Interface for ITrackerOwner {
    type Vtable = ITrackerOwner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3945054731,
        38934,
        19143,
        [140, 255, 54, 246, 122, 17, 143, 78],
    );
}
impl ::std::convert::From<ITrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: ITrackerOwner) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITrackerOwner> for ::windows::runtime::IUnknown {
    fn from(value: &ITrackerOwner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITrackerOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITrackerOwner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        returnvalue: *mut *mut TrackerHandle__,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handle: *mut TrackerHandle__,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handle: *mut TrackerHandle__,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handle: *mut TrackerHandle__,
        returnvalue: *mut ::windows::runtime::RawPtr,
    ) -> u8,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl TrackerHandle__ {}
impl ::std::default::Default for TrackerHandle__ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TrackerHandle__ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TrackerHandle__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TrackerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::std::cmp::Eq for TrackerHandle__ {}
unsafe impl ::windows::runtime::Abi for TrackerHandle__ {
    type Abi = Self;
}
#[doc = "*Required features: `UI_Xaml_Hosting`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(pub i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(0i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND:
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001 =
    __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001(1i32);
impl ::std::convert::From<i32>
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi
    for __MIDL___MIDL_itf_microsoft2Eui2Examl2Ehosting2Ereferencetracker_0000_0004_0001
{
    type Abi = Self;
}
