#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AnimatedAcceptVisualSource(pub ::windows::runtime::IInspectable);
impl AnimatedAcceptVisualSource {
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
            AnimatedAcceptVisualSource,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`, `UI_Composition`*"]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::std::option::Option<::windows::runtime::IInspectable>,
    ) -> ::windows::runtime::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<::windows::runtime::HSTRING, f64>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: runtime :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnimatedAcceptVisualSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AnimatedAcceptVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        692544963,
        28899,
        21852,
        [150, 87, 1, 252, 64, 81, 22, 157],
    );
}
impl ::windows::runtime::RuntimeName for AnimatedAcceptVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedAcceptVisualSource";
}
impl ::std::convert::From<AnimatedAcceptVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: AnimatedAcceptVisualSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AnimatedAcceptVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: &AnimatedAcceptVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AnimatedAcceptVisualSource> for ::windows::runtime::IInspectable {
    fn from(value: AnimatedAcceptVisualSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnimatedAcceptVisualSource> for ::windows::runtime::IInspectable {
    fn from(value: &AnimatedAcceptVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AnimatedAcceptVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedAcceptVisualSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AnimatedAcceptVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedAcceptVisualSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<AnimatedAcceptVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AnimatedAcceptVisualSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AnimatedAcceptVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AnimatedAcceptVisualSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedAcceptVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::std::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AnimatedAcceptVisualSource {}
unsafe impl ::std::marker::Sync for AnimatedAcceptVisualSource {}
#[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AnimatedBackVisualSource(pub ::windows::runtime::IInspectable);
impl AnimatedBackVisualSource {
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
            AnimatedBackVisualSource,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`, `UI_Composition`*"]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::std::option::Option<::windows::runtime::IInspectable>,
    ) -> ::windows::runtime::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<::windows::runtime::HSTRING, f64>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: runtime :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnimatedBackVisualSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AnimatedBackVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        692544963,
        28899,
        21852,
        [150, 87, 1, 252, 64, 81, 22, 157],
    );
}
impl ::windows::runtime::RuntimeName for AnimatedBackVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedBackVisualSource";
}
impl ::std::convert::From<AnimatedBackVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: AnimatedBackVisualSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AnimatedBackVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: &AnimatedBackVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AnimatedBackVisualSource> for ::windows::runtime::IInspectable {
    fn from(value: AnimatedBackVisualSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnimatedBackVisualSource> for ::windows::runtime::IInspectable {
    fn from(value: &AnimatedBackVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AnimatedBackVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedBackVisualSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AnimatedBackVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedBackVisualSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<AnimatedBackVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AnimatedBackVisualSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AnimatedBackVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AnimatedBackVisualSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedBackVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::std::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AnimatedBackVisualSource {}
unsafe impl ::std::marker::Sync for AnimatedBackVisualSource {}
#[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AnimatedChevronDownSmallVisualSource(pub ::windows::runtime::IInspectable);
impl AnimatedChevronDownSmallVisualSource {
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
            AnimatedChevronDownSmallVisualSource,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`, `UI_Composition`*"]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::std::option::Option<::windows::runtime::IInspectable>,
    ) -> ::windows::runtime::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<::windows::runtime::HSTRING, f64>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: runtime :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnimatedChevronDownSmallVisualSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AnimatedChevronDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        692544963,
        28899,
        21852,
        [150, 87, 1, 252, 64, 81, 22, 157],
    );
}
impl ::windows::runtime::RuntimeName for AnimatedChevronDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronDownSmallVisualSource";
}
impl ::std::convert::From<AnimatedChevronDownSmallVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: AnimatedChevronDownSmallVisualSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AnimatedChevronDownSmallVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: &AnimatedChevronDownSmallVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AnimatedChevronDownSmallVisualSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: AnimatedChevronDownSmallVisualSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnimatedChevronDownSmallVisualSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: &AnimatedChevronDownSmallVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AnimatedChevronDownSmallVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedChevronDownSmallVisualSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AnimatedChevronDownSmallVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedChevronDownSmallVisualSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: AnimatedChevronDownSmallVisualSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AnimatedChevronDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AnimatedChevronDownSmallVisualSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedChevronDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::std::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AnimatedChevronDownSmallVisualSource {}
unsafe impl ::std::marker::Sync for AnimatedChevronDownSmallVisualSource {}
#[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AnimatedChevronRightDownSmallVisualSource(pub ::windows::runtime::IInspectable);
impl AnimatedChevronRightDownSmallVisualSource {
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
            AnimatedChevronRightDownSmallVisualSource,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`, `UI_Composition`*"]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::std::option::Option<::windows::runtime::IInspectable>,
    ) -> ::windows::runtime::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<::windows::runtime::HSTRING, f64>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: runtime :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnimatedChevronRightDownSmallVisualSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AnimatedChevronRightDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        692544963,
        28899,
        21852,
        [150, 87, 1, 252, 64, 81, 22, 157],
    );
}
impl ::windows::runtime::RuntimeName for AnimatedChevronRightDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronRightDownSmallVisualSource";
}
impl ::std::convert::From<AnimatedChevronRightDownSmallVisualSource>
    for ::windows::runtime::IUnknown
{
    fn from(value: AnimatedChevronRightDownSmallVisualSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AnimatedChevronRightDownSmallVisualSource>
    for ::windows::runtime::IUnknown
{
    fn from(value: &AnimatedChevronRightDownSmallVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AnimatedChevronRightDownSmallVisualSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: AnimatedChevronRightDownSmallVisualSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnimatedChevronRightDownSmallVisualSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: &AnimatedChevronRightDownSmallVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: AnimatedChevronRightDownSmallVisualSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: &AnimatedChevronRightDownSmallVisualSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: AnimatedChevronRightDownSmallVisualSource,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AnimatedChevronRightDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &AnimatedChevronRightDownSmallVisualSource,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedChevronRightDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::std::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AnimatedChevronRightDownSmallVisualSource {}
unsafe impl ::std::marker::Sync for AnimatedChevronRightDownSmallVisualSource {}
#[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AnimatedChevronUpDownSmallVisualSource(pub ::windows::runtime::IInspectable);
impl AnimatedChevronUpDownSmallVisualSource {
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
            AnimatedChevronUpDownSmallVisualSource,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`, `UI_Composition`*"]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::std::option::Option<::windows::runtime::IInspectable>,
    ) -> ::windows::runtime::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<::windows::runtime::HSTRING, f64>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: runtime :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnimatedChevronUpDownSmallVisualSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AnimatedChevronUpDownSmallVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        692544963,
        28899,
        21852,
        [150, 87, 1, 252, 64, 81, 22, 157],
    );
}
impl ::windows::runtime::RuntimeName for AnimatedChevronUpDownSmallVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedChevronUpDownSmallVisualSource";
}
impl ::std::convert::From<AnimatedChevronUpDownSmallVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: AnimatedChevronUpDownSmallVisualSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AnimatedChevronUpDownSmallVisualSource>
    for ::windows::runtime::IUnknown
{
    fn from(value: &AnimatedChevronUpDownSmallVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AnimatedChevronUpDownSmallVisualSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: AnimatedChevronUpDownSmallVisualSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnimatedChevronUpDownSmallVisualSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: &AnimatedChevronUpDownSmallVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AnimatedChevronUpDownSmallVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedChevronUpDownSmallVisualSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: &AnimatedChevronUpDownSmallVisualSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: AnimatedChevronUpDownSmallVisualSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AnimatedChevronUpDownSmallVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &AnimatedChevronUpDownSmallVisualSource,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedChevronUpDownSmallVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::std::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AnimatedChevronUpDownSmallVisualSource {}
unsafe impl ::std::marker::Sync for AnimatedChevronUpDownSmallVisualSource {}
#[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AnimatedFindVisualSource(pub ::windows::runtime::IInspectable);
impl AnimatedFindVisualSource {
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
            AnimatedFindVisualSource,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`, `UI_Composition`*"]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::std::option::Option<::windows::runtime::IInspectable>,
    ) -> ::windows::runtime::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<::windows::runtime::HSTRING, f64>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: runtime :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnimatedFindVisualSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AnimatedFindVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        692544963,
        28899,
        21852,
        [150, 87, 1, 252, 64, 81, 22, 157],
    );
}
impl ::windows::runtime::RuntimeName for AnimatedFindVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedFindVisualSource";
}
impl ::std::convert::From<AnimatedFindVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: AnimatedFindVisualSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AnimatedFindVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: &AnimatedFindVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AnimatedFindVisualSource> for ::windows::runtime::IInspectable {
    fn from(value: AnimatedFindVisualSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnimatedFindVisualSource> for ::windows::runtime::IInspectable {
    fn from(value: &AnimatedFindVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AnimatedFindVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedFindVisualSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AnimatedFindVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedFindVisualSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<AnimatedFindVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AnimatedFindVisualSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AnimatedFindVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AnimatedFindVisualSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedFindVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::std::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AnimatedFindVisualSource {}
unsafe impl ::std::marker::Sync for AnimatedFindVisualSource {}
#[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AnimatedGlobalNavigationButtonVisualSource(pub ::windows::runtime::IInspectable);
impl AnimatedGlobalNavigationButtonVisualSource {
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
            AnimatedGlobalNavigationButtonVisualSource,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`, `UI_Composition`*"]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::std::option::Option<::windows::runtime::IInspectable>,
    ) -> ::windows::runtime::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<::windows::runtime::HSTRING, f64>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: runtime :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnimatedGlobalNavigationButtonVisualSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AnimatedGlobalNavigationButtonVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        692544963,
        28899,
        21852,
        [150, 87, 1, 252, 64, 81, 22, 157],
    );
}
impl ::windows::runtime::RuntimeName for AnimatedGlobalNavigationButtonVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedGlobalNavigationButtonVisualSource";
}
impl ::std::convert::From<AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::runtime::IUnknown
{
    fn from(value: AnimatedGlobalNavigationButtonVisualSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::runtime::IUnknown
{
    fn from(value: &AnimatedGlobalNavigationButtonVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: AnimatedGlobalNavigationButtonVisualSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnimatedGlobalNavigationButtonVisualSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: &AnimatedGlobalNavigationButtonVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: AnimatedGlobalNavigationButtonVisualSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource
{
    fn from(value: &AnimatedGlobalNavigationButtonVisualSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AnimatedGlobalNavigationButtonVisualSource>
    for super::IAnimatedVisualSource2
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &AnimatedGlobalNavigationButtonVisualSource,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedGlobalNavigationButtonVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::std::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AnimatedGlobalNavigationButtonVisualSource {}
unsafe impl ::std::marker::Sync for AnimatedGlobalNavigationButtonVisualSource {}
#[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AnimatedSettingsVisualSource(pub ::windows::runtime::IInspectable);
impl AnimatedSettingsVisualSource {
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
            AnimatedSettingsVisualSource,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`, `UI_Composition`*"]
    pub fn TryCreateAnimatedVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Composition::Compositor>,
    >(
        &self,
        compositor: Param0,
        diagnostics: &mut ::std::option::Option<::windows::runtime::IInspectable>,
    ) -> ::windows::runtime::Result<super::IAnimatedVisual> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                compositor.into_param().abi(),
                diagnostics as *mut _ as _,
                &mut result__,
            )
            .from_abi::<super::IAnimatedVisual>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn Markers(
        &self,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IMapView<::windows::runtime::HSTRING, f64>,
    > {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < ::windows::Foundation::Collections:: IMapView :: < :: windows :: runtime :: HSTRING , f64 > > ( result__ )
        }
    }
    #[doc = "*Required features: `UI_Xaml_Controls_AnimatedVisuals`*"]
    pub fn SetColorProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::UI::Color>,
    >(
        &self,
        propertyname: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IAnimatedVisualSource2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                propertyname.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnimatedSettingsVisualSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource;{294765c3-70e3-555c-9657-01fc4051169d})" ) ;
}
unsafe impl ::windows::runtime::Interface for AnimatedSettingsVisualSource {
    type Vtable = super::IAnimatedVisualSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        692544963,
        28899,
        21852,
        [150, 87, 1, 252, 64, 81, 22, 157],
    );
}
impl ::windows::runtime::RuntimeName for AnimatedSettingsVisualSource {
    const NAME: &'static str =
        "Microsoft.UI.Xaml.Controls.AnimatedVisuals.AnimatedSettingsVisualSource";
}
impl ::std::convert::From<AnimatedSettingsVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: AnimatedSettingsVisualSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AnimatedSettingsVisualSource> for ::windows::runtime::IUnknown {
    fn from(value: &AnimatedSettingsVisualSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AnimatedSettingsVisualSource> for ::windows::runtime::IInspectable {
    fn from(value: AnimatedSettingsVisualSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnimatedSettingsVisualSource> for ::windows::runtime::IInspectable {
    fn from(value: &AnimatedSettingsVisualSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AnimatedSettingsVisualSource> for super::IAnimatedVisualSource {
    fn from(value: AnimatedSettingsVisualSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AnimatedSettingsVisualSource> for super::IAnimatedVisualSource {
    fn from(value: &AnimatedSettingsVisualSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource>
    for &AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
impl ::std::convert::TryFrom<AnimatedSettingsVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AnimatedSettingsVisualSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&AnimatedSettingsVisualSource> for super::IAnimatedVisualSource2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AnimatedSettingsVisualSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IAnimatedVisualSource2>
    for &AnimatedSettingsVisualSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IAnimatedVisualSource2> {
        ::std::convert::TryInto::<super::IAnimatedVisualSource2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AnimatedSettingsVisualSource {}
unsafe impl ::std::marker::Sync for AnimatedSettingsVisualSource {}
