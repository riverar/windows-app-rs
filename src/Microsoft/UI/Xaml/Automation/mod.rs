#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "UI_Xaml_Automation_Peers")]
pub mod Peers;
#[cfg(feature = "UI_Xaml_Automation_Provider")]
pub mod Provider;
#[cfg(feature = "UI_Xaml_Automation_Text")]
pub mod Text;
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AnnotationPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl AnnotationPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AnnotationTypeIdProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AnnotationTypeNameProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AuthorProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DateTimeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn TargetProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IAnnotationPatternIdentifiersStatics<
        R,
        F: FnOnce(&IAnnotationPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AnnotationPatternIdentifiers,
            IAnnotationPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnnotationPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AnnotationPatternIdentifiers;{92d76915-0cd3-59cd-8ae0-c9004628ba1e})" ) ;
}
unsafe impl ::windows::runtime::Interface for AnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2463590677,
        3283,
        22989,
        [138, 224, 201, 0, 70, 40, 186, 30],
    );
}
impl ::windows::runtime::RuntimeName for AnnotationPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AnnotationPatternIdentifiers";
}
impl ::std::convert::From<AnnotationPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AnnotationPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AnnotationPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AnnotationPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AnnotationPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnnotationPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AnnotationPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AnnotationPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AnnotationPatternIdentifiers {}
unsafe impl ::std::marker::Sync for AnnotationPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AnnotationType(pub i32);
impl AnnotationType {
    pub const Unknown: AnnotationType = AnnotationType(60000i32);
    pub const SpellingError: AnnotationType = AnnotationType(60001i32);
    pub const GrammarError: AnnotationType = AnnotationType(60002i32);
    pub const Comment: AnnotationType = AnnotationType(60003i32);
    pub const FormulaError: AnnotationType = AnnotationType(60004i32);
    pub const TrackChanges: AnnotationType = AnnotationType(60005i32);
    pub const Header: AnnotationType = AnnotationType(60006i32);
    pub const Footer: AnnotationType = AnnotationType(60007i32);
    pub const Highlighted: AnnotationType = AnnotationType(60008i32);
    pub const Endnote: AnnotationType = AnnotationType(60009i32);
    pub const Footnote: AnnotationType = AnnotationType(60010i32);
    pub const InsertionChange: AnnotationType = AnnotationType(60011i32);
    pub const DeletionChange: AnnotationType = AnnotationType(60012i32);
    pub const MoveChange: AnnotationType = AnnotationType(60013i32);
    pub const FormatChange: AnnotationType = AnnotationType(60014i32);
    pub const UnsyncedChange: AnnotationType = AnnotationType(60015i32);
    pub const EditingLockedChange: AnnotationType = AnnotationType(60016i32);
    pub const ExternalChange: AnnotationType = AnnotationType(60017i32);
    pub const ConflictingChange: AnnotationType = AnnotationType(60018i32);
    pub const Author: AnnotationType = AnnotationType(60019i32);
    pub const AdvancedProofingIssue: AnnotationType = AnnotationType(60020i32);
    pub const DataValidationError: AnnotationType = AnnotationType(60021i32);
    pub const CircularReferenceError: AnnotationType = AnnotationType(60022i32);
}
impl ::std::convert::From<i32> for AnnotationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AnnotationType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AnnotationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AnnotationType;i4)",
    );
}
impl ::windows::runtime::DefaultType for AnnotationType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationActiveEnd(pub i32);
impl AutomationActiveEnd {
    pub const None: AutomationActiveEnd = AutomationActiveEnd(0i32);
    pub const Start: AutomationActiveEnd = AutomationActiveEnd(1i32);
    pub const End: AutomationActiveEnd = AutomationActiveEnd(2i32);
}
impl ::std::convert::From<i32> for AutomationActiveEnd {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationActiveEnd {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationActiveEnd {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationActiveEnd;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationActiveEnd {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationAnimationStyle(pub i32);
impl AutomationAnimationStyle {
    pub const None: AutomationAnimationStyle = AutomationAnimationStyle(0i32);
    pub const LasVegasLights: AutomationAnimationStyle = AutomationAnimationStyle(1i32);
    pub const BlinkingBackground: AutomationAnimationStyle = AutomationAnimationStyle(2i32);
    pub const SparkleText: AutomationAnimationStyle = AutomationAnimationStyle(3i32);
    pub const MarchingBlackAnts: AutomationAnimationStyle = AutomationAnimationStyle(4i32);
    pub const MarchingRedAnts: AutomationAnimationStyle = AutomationAnimationStyle(5i32);
    pub const Shimmer: AutomationAnimationStyle = AutomationAnimationStyle(6i32);
    pub const Other: AutomationAnimationStyle = AutomationAnimationStyle(7i32);
}
impl ::std::convert::From<i32> for AutomationAnimationStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationAnimationStyle {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationAnimationStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationAnimationStyle;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationAnimationStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AutomationAnnotation(pub ::windows::runtime::IInspectable);
impl AutomationAnnotation {
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
            AutomationAnnotation,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<AnnotationType> {
        let this = self;
        unsafe {
            let mut result__: AnnotationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AnnotationType>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetType(&self, value: AnnotationType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn Element(&self) -> ::windows::runtime::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetElement<'a, Param0: ::windows::runtime::IntoParam<'a, super::UIElement>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CreateInstance(
        r#type: AnnotationType,
    ) -> ::windows::runtime::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                r#type,
                &mut result__,
            )
            .from_abi::<AutomationAnnotation>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CreateWithElementParameter<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::UIElement>,
    >(
        r#type: AnnotationType,
        element: Param1,
    ) -> ::windows::runtime::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                r#type,
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<AutomationAnnotation>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn TypeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ElementProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        dp: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ClearValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>>(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ReadLocalValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetAnimationBaseValue<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RegisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
        Param1: ::windows::runtime::IntoParam<'a, super::DependencyPropertyChangedCallback>,
    >(
        &self,
        dp: Param0,
        callback: Param1,
    ) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                callback.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn UnregisterPropertyChangedCallback<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyProperty>,
    >(
        &self,
        dp: Param0,
        token: i64,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                dp.into_param().abi(),
                token,
            )
            .ok()
        }
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Dispatching`*"]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &::windows::runtime::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Dispatching::DispatcherQueue>(result__)
        }
    }
    pub fn IAutomationAnnotationFactory<
        R,
        F: FnOnce(&IAutomationAnnotationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AutomationAnnotation,
            IAutomationAnnotationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationAnnotationStatics<
        R,
        F: FnOnce(&IAutomationAnnotationStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AutomationAnnotation,
            IAutomationAnnotationStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AutomationAnnotation {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AutomationAnnotation;{c2cc46ad-1414-5f1b-808a-89e5d53d82fe})" ) ;
}
unsafe impl ::windows::runtime::Interface for AutomationAnnotation {
    type Vtable = IAutomationAnnotation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3268167341,
        5140,
        24347,
        [128, 138, 137, 229, 213, 61, 130, 254],
    );
}
impl ::windows::runtime::RuntimeName for AutomationAnnotation {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationAnnotation";
}
impl ::std::convert::From<AutomationAnnotation> for ::windows::runtime::IUnknown {
    fn from(value: AutomationAnnotation) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AutomationAnnotation> for ::windows::runtime::IUnknown {
    fn from(value: &AutomationAnnotation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AutomationAnnotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AutomationAnnotation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AutomationAnnotation> for ::windows::runtime::IInspectable {
    fn from(value: AutomationAnnotation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AutomationAnnotation> for ::windows::runtime::IInspectable {
    fn from(value: &AutomationAnnotation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AutomationAnnotation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AutomationAnnotation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<AutomationAnnotation> for super::DependencyObject {
    fn from(value: AutomationAnnotation) -> Self {
        ::std::convert::Into::<super::DependencyObject>::into(&value)
    }
}
impl ::std::convert::From<&AutomationAnnotation> for super::DependencyObject {
    fn from(value: &AutomationAnnotation) -> Self {
        ::windows::runtime::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for AutomationAnnotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::DependencyObject> for &AutomationAnnotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::DependencyObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::DependencyObject>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
unsafe impl ::std::marker::Send for AutomationAnnotation {}
unsafe impl ::std::marker::Sync for AutomationAnnotation {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationBulletStyle(pub i32);
impl AutomationBulletStyle {
    pub const None: AutomationBulletStyle = AutomationBulletStyle(0i32);
    pub const HollowRoundBullet: AutomationBulletStyle = AutomationBulletStyle(1i32);
    pub const FilledRoundBullet: AutomationBulletStyle = AutomationBulletStyle(2i32);
    pub const HollowSquareBullet: AutomationBulletStyle = AutomationBulletStyle(3i32);
    pub const FilledSquareBullet: AutomationBulletStyle = AutomationBulletStyle(4i32);
    pub const DashBullet: AutomationBulletStyle = AutomationBulletStyle(5i32);
    pub const Other: AutomationBulletStyle = AutomationBulletStyle(6i32);
}
impl ::std::convert::From<i32> for AutomationBulletStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationBulletStyle {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationBulletStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationBulletStyle;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationBulletStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationCaretBidiMode(pub i32);
impl AutomationCaretBidiMode {
    pub const LTR: AutomationCaretBidiMode = AutomationCaretBidiMode(0i32);
    pub const RTL: AutomationCaretBidiMode = AutomationCaretBidiMode(1i32);
}
impl ::std::convert::From<i32> for AutomationCaretBidiMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationCaretBidiMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationCaretBidiMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationCaretBidiMode;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationCaretBidiMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationCaretPosition(pub i32);
impl AutomationCaretPosition {
    pub const Unknown: AutomationCaretPosition = AutomationCaretPosition(0i32);
    pub const EndOfLine: AutomationCaretPosition = AutomationCaretPosition(1i32);
    pub const BeginningOfLine: AutomationCaretPosition = AutomationCaretPosition(2i32);
}
impl ::std::convert::From<i32> for AutomationCaretPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationCaretPosition {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationCaretPosition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationCaretPosition;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationCaretPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AutomationElementIdentifiers(pub ::windows::runtime::IInspectable);
impl AutomationElementIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AcceleratorKeyProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AccessKeyProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AutomationIdProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn BoundingRectangleProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ClassNameProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ClickablePointProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ControlTypeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HasKeyboardFocusProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HelpTextProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsContentElementProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsControlElementProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsEnabledProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsKeyboardFocusableProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsOffscreenProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsPasswordProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsRequiredForFormProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ItemStatusProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ItemTypeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LabeledByProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LocalizedControlTypeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn NameProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn OrientationProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LiveSettingProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ControlledPeersProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn PositionInSetProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SizeOfSetProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LevelProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AnnotationsProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LandmarkTypeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LocalizedLandmarkTypeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsPeripheralProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsDataValidForFormProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FullDescriptionProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DescribedByProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FlowsToProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FlowsFromProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CultureProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HeadingLevelProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsDialogProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IAutomationElementIdentifiersStatics<
        R,
        F: FnOnce(&IAutomationElementIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AutomationElementIdentifiers,
            IAutomationElementIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AutomationElementIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AutomationElementIdentifiers;{2fb51a33-b0cf-5a4c-9ed3-267eca7aeefc})" ) ;
}
unsafe impl ::windows::runtime::Interface for AutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        800397875,
        45263,
        23116,
        [158, 211, 38, 126, 202, 122, 238, 252],
    );
}
impl ::windows::runtime::RuntimeName for AutomationElementIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationElementIdentifiers";
}
impl ::std::convert::From<AutomationElementIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: AutomationElementIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AutomationElementIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for AutomationElementIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AutomationElementIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AutomationElementIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: AutomationElementIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AutomationElementIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AutomationElementIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AutomationElementIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AutomationElementIdentifiers {}
unsafe impl ::std::marker::Sync for AutomationElementIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationFlowDirections(pub i32);
impl AutomationFlowDirections {
    pub const Default: AutomationFlowDirections = AutomationFlowDirections(0i32);
    pub const RightToLeft: AutomationFlowDirections = AutomationFlowDirections(1i32);
    pub const BottomToTop: AutomationFlowDirections = AutomationFlowDirections(2i32);
    pub const Vertical: AutomationFlowDirections = AutomationFlowDirections(3i32);
}
impl ::std::convert::From<i32> for AutomationFlowDirections {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationFlowDirections {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationFlowDirections {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationFlowDirections;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationFlowDirections {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationOutlineStyles(pub i32);
impl AutomationOutlineStyles {
    pub const None: AutomationOutlineStyles = AutomationOutlineStyles(0i32);
    pub const Outline: AutomationOutlineStyles = AutomationOutlineStyles(1i32);
    pub const Shadow: AutomationOutlineStyles = AutomationOutlineStyles(2i32);
    pub const Engraved: AutomationOutlineStyles = AutomationOutlineStyles(3i32);
    pub const Embossed: AutomationOutlineStyles = AutomationOutlineStyles(4i32);
}
impl ::std::convert::From<i32> for AutomationOutlineStyles {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationOutlineStyles {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationOutlineStyles {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationOutlineStyles;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationOutlineStyles {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AutomationProperties(pub ::windows::runtime::IInspectable);
impl AutomationProperties {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AcceleratorKeyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetAcceleratorKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetAcceleratorKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AccessKeyProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetAccessKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetAccessKey<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AutomationIdProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetAutomationId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetAutomationId<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HelpTextProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetHelpText<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetHelpText<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsRequiredForFormProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetIsRequiredForForm<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetIsRequiredForForm<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ItemStatusProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetItemStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetItemStatus<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ItemTypeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetItemType<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetItemType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LabeledByProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetLabeledBy<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<super::UIElement> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::UIElement>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetLabeledBy<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, super::UIElement>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn NameProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LiveSettingProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn GetLiveSetting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<Peers::AutomationLiveSetting> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AutomationLiveSetting = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Peers::AutomationLiveSetting>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn SetLiveSetting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: Peers::AutomationLiveSetting,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AccessibilityViewProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn GetAccessibilityView<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<Peers::AccessibilityView> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AccessibilityView = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Peers::AccessibilityView>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn SetAccessibilityView<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: Peers::AccessibilityView,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ControlledPeersProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetControlledPeers<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IVector<super::UIElement>>
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::UIElement>>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn PositionInSetProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetPositionInSet<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetPositionInSet<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SizeOfSetProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetSizeOfSet<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetSizeOfSet<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LevelProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetLevel<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn AnnotationsProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetAnnotations<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::Foundation::Collections::IVector<AutomationAnnotation>>
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<AutomationAnnotation>>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LandmarkTypeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn GetLandmarkType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<Peers::AutomationLandmarkType> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AutomationLandmarkType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Peers::AutomationLandmarkType>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn SetLandmarkType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: Peers::AutomationLandmarkType,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LocalizedLandmarkTypeProperty() -> ::windows::runtime::Result<super::DependencyProperty>
    {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetLocalizedLandmarkType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetLocalizedLandmarkType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsPeripheralProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetIsPeripheral<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).59)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetIsPeripheral<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).60)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsDataValidForFormProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).61)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetIsDataValidForForm<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).62)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetIsDataValidForForm<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).63)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FullDescriptionProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).64)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetFullDescription<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).65)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetFullDescription<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).66)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LocalizedControlTypeProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).67)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetLocalizedControlType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).68)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetLocalizedControlType<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        element: Param0,
        value: Param1,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).69)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value.into_param().abi(),
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DescribedByProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).70)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetDescribedBy<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IVector<super::DependencyObject>,
    > {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).71)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::DependencyObject>>(
                result__,
            )
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FlowsToProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).72)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetFlowsTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IVector<super::DependencyObject>,
    > {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).73)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::DependencyObject>>(
                result__,
            )
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FlowsFromProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).74)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetFlowsFrom<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<
        ::windows::Foundation::Collections::IVector<super::DependencyObject>,
    > {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).75)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<::windows::Foundation::Collections::IVector<super::DependencyObject>>(
                result__,
            )
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CultureProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).76)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetCulture<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<i32> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).77)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetCulture<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: i32,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).78)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HeadingLevelProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn GetHeadingLevel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
    ) -> ::windows::runtime::Result<Peers::AutomationHeadingLevel> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: Peers::AutomationHeadingLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).80)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<Peers::AutomationHeadingLevel>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    #[doc = "*Required features: `UI_Xaml_Automation`, `UI_Xaml_Automation_Peers`*"]
    pub fn SetHeadingLevel<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>,
    >(
        element: Param0,
        value: Peers::AutomationHeadingLevel,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).81)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsDialogProperty() -> ::windows::runtime::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).82)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GetIsDialog<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
    ) -> ::windows::runtime::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).83)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SetIsDialog<'a, Param0: ::windows::runtime::IntoParam<'a, super::DependencyObject>>(
        element: Param0,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).84)(
                ::std::mem::transmute_copy(this),
                element.into_param().abi(),
                value,
            )
            .ok()
        })
    }
    pub fn IAutomationPropertiesStatics<
        R,
        F: FnOnce(&IAutomationPropertiesStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            AutomationProperties,
            IAutomationPropertiesStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AutomationProperties {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AutomationProperties;{525c6a71-dd8a-52a0-977b-db1b02f8e896})" ) ;
}
unsafe impl ::windows::runtime::Interface for AutomationProperties {
    type Vtable = IAutomationProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1381788273,
        56714,
        21152,
        [151, 123, 219, 27, 2, 248, 232, 150],
    );
}
impl ::windows::runtime::RuntimeName for AutomationProperties {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationProperties";
}
impl ::std::convert::From<AutomationProperties> for ::windows::runtime::IUnknown {
    fn from(value: AutomationProperties) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AutomationProperties> for ::windows::runtime::IUnknown {
    fn from(value: &AutomationProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AutomationProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AutomationProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AutomationProperties> for ::windows::runtime::IInspectable {
    fn from(value: AutomationProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AutomationProperties> for ::windows::runtime::IInspectable {
    fn from(value: &AutomationProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AutomationProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AutomationProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AutomationProperties {}
unsafe impl ::std::marker::Sync for AutomationProperties {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct AutomationProperty(pub ::windows::runtime::IInspectable);
impl AutomationProperty {}
unsafe impl ::windows::runtime::RuntimeType for AutomationProperty {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.AutomationProperty;{5ca6b2c8-ff86-5a41-aa18-6948fae592cf})" ) ;
}
unsafe impl ::windows::runtime::Interface for AutomationProperty {
    type Vtable = IAutomationProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1554428616,
        65414,
        23105,
        [170, 24, 105, 72, 250, 229, 146, 207],
    );
}
impl ::windows::runtime::RuntimeName for AutomationProperty {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationProperty";
}
impl ::std::convert::From<AutomationProperty> for ::windows::runtime::IUnknown {
    fn from(value: AutomationProperty) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AutomationProperty> for ::windows::runtime::IUnknown {
    fn from(value: &AutomationProperty) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AutomationProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a AutomationProperty
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AutomationProperty> for ::windows::runtime::IInspectable {
    fn from(value: AutomationProperty) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AutomationProperty> for ::windows::runtime::IInspectable {
    fn from(value: &AutomationProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for AutomationProperty
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a AutomationProperty
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AutomationProperty {}
unsafe impl ::std::marker::Sync for AutomationProperty {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationStyleId(pub i32);
impl AutomationStyleId {
    pub const Heading1: AutomationStyleId = AutomationStyleId(70001i32);
    pub const Heading2: AutomationStyleId = AutomationStyleId(70002i32);
    pub const Heading3: AutomationStyleId = AutomationStyleId(70003i32);
    pub const Heading4: AutomationStyleId = AutomationStyleId(70004i32);
    pub const Heading5: AutomationStyleId = AutomationStyleId(70005i32);
    pub const Heading6: AutomationStyleId = AutomationStyleId(70006i32);
    pub const Heading7: AutomationStyleId = AutomationStyleId(70007i32);
    pub const Heading8: AutomationStyleId = AutomationStyleId(70008i32);
    pub const Heading9: AutomationStyleId = AutomationStyleId(70009i32);
    pub const Title: AutomationStyleId = AutomationStyleId(70010i32);
    pub const Subtitle: AutomationStyleId = AutomationStyleId(70011i32);
    pub const Normal: AutomationStyleId = AutomationStyleId(70012i32);
    pub const Emphasis: AutomationStyleId = AutomationStyleId(70013i32);
    pub const Quote: AutomationStyleId = AutomationStyleId(70014i32);
    pub const BulletedList: AutomationStyleId = AutomationStyleId(70015i32);
}
impl ::std::convert::From<i32> for AutomationStyleId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationStyleId {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationStyleId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationStyleId;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationStyleId {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationTextDecorationLineStyle(pub i32);
impl AutomationTextDecorationLineStyle {
    pub const None: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(0i32);
    pub const Single: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(1i32);
    pub const WordsOnly: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(2i32);
    pub const Double: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(3i32);
    pub const Dot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(4i32);
    pub const Dash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(5i32);
    pub const DashDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(6i32);
    pub const DashDotDot: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(7i32);
    pub const Wavy: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(8i32);
    pub const ThickSingle: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(9i32);
    pub const DoubleWavy: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(10i32);
    pub const ThickWavy: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(11i32);
    pub const LongDash: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(12i32);
    pub const ThickDash: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(13i32);
    pub const ThickDashDot: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(14i32);
    pub const ThickDashDotDot: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(15i32);
    pub const ThickDot: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(16i32);
    pub const ThickLongDash: AutomationTextDecorationLineStyle =
        AutomationTextDecorationLineStyle(17i32);
    pub const Other: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(18i32);
}
impl ::std::convert::From<i32> for AutomationTextDecorationLineStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationTextDecorationLineStyle {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationTextDecorationLineStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationTextDecorationLineStyle;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationTextDecorationLineStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct AutomationTextEditChangeType(pub i32);
impl AutomationTextEditChangeType {
    pub const None: AutomationTextEditChangeType = AutomationTextEditChangeType(0i32);
    pub const AutoCorrect: AutomationTextEditChangeType = AutomationTextEditChangeType(1i32);
    pub const Composition: AutomationTextEditChangeType = AutomationTextEditChangeType(2i32);
    pub const CompositionFinalized: AutomationTextEditChangeType =
        AutomationTextEditChangeType(3i32);
}
impl ::std::convert::From<i32> for AutomationTextEditChangeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AutomationTextEditChangeType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AutomationTextEditChangeType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.AutomationTextEditChangeType;i4)",
    );
}
impl ::windows::runtime::DefaultType for AutomationTextEditChangeType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DockPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl DockPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DockPositionProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IDockPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDockPatternIdentifiersStatics<
        R,
        F: FnOnce(&IDockPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DockPatternIdentifiers,
            IDockPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DockPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.DockPatternIdentifiers;{75574f99-d145-547e-972b-7d879f93c03e})" ) ;
}
unsafe impl ::windows::runtime::Interface for DockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1968656281,
        53573,
        21630,
        [151, 43, 125, 135, 159, 147, 192, 62],
    );
}
impl ::windows::runtime::RuntimeName for DockPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.DockPatternIdentifiers";
}
impl ::std::convert::From<DockPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: DockPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&DockPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &DockPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DockPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a DockPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<DockPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: DockPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DockPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &DockPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DockPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DockPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DockPatternIdentifiers {}
unsafe impl ::std::marker::Sync for DockPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DockPosition(pub i32);
impl DockPosition {
    pub const Top: DockPosition = DockPosition(0i32);
    pub const Left: DockPosition = DockPosition(1i32);
    pub const Bottom: DockPosition = DockPosition(2i32);
    pub const Right: DockPosition = DockPosition(3i32);
    pub const Fill: DockPosition = DockPosition(4i32);
    pub const None: DockPosition = DockPosition(5i32);
}
impl ::std::convert::From<i32> for DockPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DockPosition {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DockPosition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.DockPosition;i4)",
    );
}
impl ::windows::runtime::DefaultType for DockPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DragPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl DragPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DropEffectProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DropEffectsProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn GrabbedItemsProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsGrabbedProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDragPatternIdentifiersStatics<
        R,
        F: FnOnce(&IDragPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DragPatternIdentifiers,
            IDragPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DragPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.DragPatternIdentifiers;{aa2fdfd5-fb45-5d2b-8d92-a8e7b07061c2})" ) ;
}
unsafe impl ::windows::runtime::Interface for DragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2855264213,
        64325,
        23851,
        [141, 146, 168, 231, 176, 112, 97, 194],
    );
}
impl ::windows::runtime::RuntimeName for DragPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.DragPatternIdentifiers";
}
impl ::std::convert::From<DragPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: DragPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&DragPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &DragPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DragPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a DragPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<DragPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: DragPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DragPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &DragPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DragPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DragPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DragPatternIdentifiers {}
unsafe impl ::std::marker::Sync for DragPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DropTargetPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl DropTargetPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DropTargetEffectProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn DropTargetEffectsProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDropTargetPatternIdentifiersStatics<
        R,
        F: FnOnce(&IDropTargetPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DropTargetPatternIdentifiers,
            IDropTargetPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DropTargetPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.DropTargetPatternIdentifiers;{133e8ff3-1ddd-5cbb-b908-1484d7c04da7})" ) ;
}
unsafe impl ::windows::runtime::Interface for DropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        322867187,
        7645,
        23739,
        [185, 8, 20, 132, 215, 192, 77, 167],
    );
}
impl ::windows::runtime::RuntimeName for DropTargetPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.DropTargetPatternIdentifiers";
}
impl ::std::convert::From<DropTargetPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&DropTargetPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for DropTargetPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a DropTargetPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<DropTargetPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DropTargetPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for DropTargetPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DropTargetPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DropTargetPatternIdentifiers {}
unsafe impl ::std::marker::Sync for DropTargetPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ExpandCollapsePatternIdentifiers(pub ::windows::runtime::IInspectable);
impl ExpandCollapsePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ExpandCollapseStateProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IExpandCollapsePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IExpandCollapsePatternIdentifiersStatics<
        R,
        F: FnOnce(&IExpandCollapsePatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ExpandCollapsePatternIdentifiers,
            IExpandCollapsePatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ExpandCollapsePatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers;{cec15d9f-8630-569a-86a0-524bbea618ff})" ) ;
}
unsafe impl ::windows::runtime::Interface for ExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3468778911,
        34352,
        22170,
        [134, 160, 82, 75, 190, 166, 24, 255],
    );
}
impl ::windows::runtime::RuntimeName for ExpandCollapsePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers";
}
impl ::std::convert::From<ExpandCollapsePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ExpandCollapsePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a ExpandCollapsePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ExpandCollapsePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ExpandCollapsePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ExpandCollapsePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ExpandCollapsePatternIdentifiers {}
unsafe impl ::std::marker::Sync for ExpandCollapsePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ExpandCollapseState(pub i32);
impl ExpandCollapseState {
    pub const Collapsed: ExpandCollapseState = ExpandCollapseState(0i32);
    pub const Expanded: ExpandCollapseState = ExpandCollapseState(1i32);
    pub const PartiallyExpanded: ExpandCollapseState = ExpandCollapseState(2i32);
    pub const LeafNode: ExpandCollapseState = ExpandCollapseState(3i32);
}
impl ::std::convert::From<i32> for ExpandCollapseState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExpandCollapseState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ExpandCollapseState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.ExpandCollapseState;i4)",
    );
}
impl ::windows::runtime::DefaultType for ExpandCollapseState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct GridItemPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl GridItemPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnSpanProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ContainingGridProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowSpanProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IGridItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&IGridItemPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            GridItemPatternIdentifiers,
            IGridItemPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GridItemPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.GridItemPatternIdentifiers;{93609087-1114-557d-b17b-f801e41cebbb})" ) ;
}
unsafe impl ::windows::runtime::Interface for GridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2472579207,
        4372,
        21885,
        [177, 123, 248, 1, 228, 28, 235, 187],
    );
}
impl ::windows::runtime::RuntimeName for GridItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.GridItemPatternIdentifiers";
}
impl ::std::convert::From<GridItemPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&GridItemPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for GridItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a GridItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<GridItemPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GridItemPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for GridItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a GridItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GridItemPatternIdentifiers {}
unsafe impl ::std::marker::Sync for GridItemPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct GridPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl GridPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnCountProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowCountProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IGridPatternIdentifiersStatics<
        R,
        F: FnOnce(&IGridPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            GridPatternIdentifiers,
            IGridPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GridPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.GridPatternIdentifiers;{e5e1e250-c37c-54a2-8c61-1d9ccd3bb39c})" ) ;
}
unsafe impl ::windows::runtime::Interface for GridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3856786000,
        50044,
        21666,
        [140, 97, 29, 156, 205, 59, 179, 156],
    );
}
impl ::windows::runtime::RuntimeName for GridPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.GridPatternIdentifiers";
}
impl ::std::convert::From<GridPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: GridPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&GridPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &GridPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for GridPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a GridPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<GridPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: GridPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GridPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &GridPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for GridPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a GridPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GridPatternIdentifiers {}
unsafe impl ::std::marker::Sync for GridPatternIdentifiers {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2463590677,
        3283,
        22989,
        [138, 224, 201, 0, 70, 40, 186, 30],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiers_abi(
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
pub struct IAnnotationPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAnnotationPatternIdentifiersStatics {
    type Vtable = IAnnotationPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        547436258,
        19015,
        24037,
        [158, 30, 236, 252, 109, 146, 245, 42],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationAnnotation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationAnnotation {
    type Vtable = IAutomationAnnotation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3268167341,
        5140,
        24347,
        [128, 138, 137, 229, 213, 61, 130, 254],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotation_abi(
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
        result__: *mut AnnotationType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: AnnotationType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationAnnotationFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationAnnotationFactory {
    type Vtable = IAutomationAnnotationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2516068211,
        60101,
        22318,
        [135, 222, 36, 217, 81, 75, 154, 137],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationFactory_abi(
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
        r#type: AnnotationType,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        r#type: AnnotationType,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationAnnotationStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationAnnotationStatics {
    type Vtable = IAutomationAnnotationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3316374558,
        64550,
        21572,
        [168, 179, 89, 178, 192, 169, 85, 120],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        800397875,
        45263,
        23116,
        [158, 211, 38, 126, 202, 122, 238, 252],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiers_abi(
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
pub struct IAutomationElementIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationElementIdentifiersStatics {
    type Vtable = IAutomationElementIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1924098956,
        15890,
        24186,
        [162, 236, 38, 220, 25, 63, 157, 249],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationProperties {
    type Vtable = IAutomationProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1381788273,
        56714,
        21152,
        [151, 123, 219, 27, 2, 248, 232, 150],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperties_abi(
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
pub struct IAutomationPropertiesStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationPropertiesStatics {
    type Vtable = IAutomationPropertiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2984501491,
        4399,
        22886,
        [135, 220, 120, 98, 212, 173, 80, 229],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics_abi(
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
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut Peers::AutomationLiveSetting,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: Peers::AutomationLiveSetting,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut Peers::AccessibilityView,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: Peers::AccessibilityView,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut Peers::AutomationLandmarkType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: Peers::AutomationLandmarkType,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut Peers::AutomationHeadingLevel,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: Peers::AutomationHeadingLevel,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        element: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAutomationProperty(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationProperty {
    type Vtable = IAutomationProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1554428616,
        65414,
        23105,
        [170, 24, 105, 72, 250, 229, 146, 207],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperty_abi(
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
pub struct IDockPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1968656281,
        53573,
        21630,
        [151, 43, 125, 135, 159, 147, 192, 62],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiers_abi(
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
pub struct IDockPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDockPatternIdentifiersStatics {
    type Vtable = IDockPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        47556396,
        62621,
        21417,
        [185, 251, 175, 39, 25, 209, 108, 207],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiersStatics_abi(
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
pub struct IDragPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2855264213,
        64325,
        23851,
        [141, 146, 168, 231, 176, 112, 97, 194],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiers_abi(
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
pub struct IDragPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDragPatternIdentifiersStatics {
    type Vtable = IDragPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1211035248,
        3068,
        21842,
        [158, 125, 141, 255, 197, 38, 178, 247],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        322867187,
        7645,
        23739,
        [185, 8, 20, 132, 215, 192, 77, 167],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiers_abi(
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
pub struct IDropTargetPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDropTargetPatternIdentifiersStatics {
    type Vtable = IDropTargetPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1839657149,
        47426,
        21123,
        [190, 53, 80, 26, 232, 127, 136, 199],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3468778911,
        34352,
        22170,
        [134, 160, 82, 75, 190, 166, 24, 255],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiers_abi(
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
pub struct IExpandCollapsePatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IExpandCollapsePatternIdentifiersStatics {
    type Vtable = IExpandCollapsePatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        674300404,
        50188,
        21951,
        [162, 59, 214, 43, 115, 182, 170, 53],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiersStatics_abi(
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
pub struct IGridItemPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2472579207,
        4372,
        21885,
        [177, 123, 248, 1, 228, 28, 235, 187],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiers_abi(
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
pub struct IGridItemPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGridItemPatternIdentifiersStatics {
    type Vtable = IGridItemPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2155002904,
        34768,
        23042,
        [160, 161, 249, 174, 201, 104, 192, 231],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGridPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3856786000,
        50044,
        21666,
        [140, 97, 29, 156, 205, 59, 179, 156],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiers_abi(
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
pub struct IGridPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGridPatternIdentifiersStatics {
    type Vtable = IGridPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3898695756,
        4127,
        23149,
        [163, 8, 55, 20, 245, 16, 247, 68],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1894041671,
        11138,
        24271,
        [184, 8, 233, 212, 83, 193, 254, 83],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiers_abi(
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
pub struct IMultipleViewPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMultipleViewPatternIdentifiersStatics {
    type Vtable = IMultipleViewPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2893142767,
        53396,
        23696,
        [148, 175, 31, 164, 116, 171, 69, 254],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3239369527,
        27253,
        24305,
        [165, 66, 211, 177, 63, 146, 203, 254],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiers_abi(
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
pub struct IRangeValuePatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRangeValuePatternIdentifiersStatics {
    type Vtable = IRangeValuePatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        178952919,
        63928,
        21153,
        [188, 150, 42, 151, 254, 56, 158, 208],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        82945208,
        60871,
        22002,
        [150, 223, 169, 199, 232, 9, 55, 46],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiers_abi(
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
pub struct IScrollPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IScrollPatternIdentifiersStatics {
    type Vtable = IScrollPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        261419760,
        57554,
        23076,
        [180, 21, 141, 21, 6, 206, 71, 170],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f64,
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3459929245,
        41675,
        22861,
        [162, 164, 68, 119, 140, 9, 204, 165],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiers_abi(
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
pub struct ISelectionItemPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISelectionItemPatternIdentifiersStatics {
    type Vtable = ISelectionItemPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        730770812,
        19971,
        23428,
        [158, 52, 139, 115, 132, 203, 216, 98],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1075266514,
        8122,
        23813,
        [184, 159, 99, 22, 118, 69, 50, 55],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiers_abi(
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
pub struct ISelectionPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISelectionPatternIdentifiersStatics {
    type Vtable = ISelectionPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4092399899,
        45578,
        24158,
        [162, 50, 7, 246, 7, 253, 92, 7],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3701664838,
        34148,
        23708,
        [186, 144, 44, 8, 69, 95, 105, 123],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiers_abi(
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
pub struct ISpreadsheetItemPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpreadsheetItemPatternIdentifiersStatics {
    type Vtable = ISpreadsheetItemPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2125533056,
        36154,
        22957,
        [162, 185, 5, 216, 206, 207, 24, 219],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiersStatics_abi(
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
pub struct IStylesPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        330222174,
        46230,
        24053,
        [174, 165, 51, 14, 31, 4, 144, 235],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiers_abi(
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
pub struct IStylesPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStylesPatternIdentifiersStatics {
    type Vtable = IStylesPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2989631610,
        48204,
        22558,
        [163, 60, 61, 106, 238, 16, 208, 75],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3034471683,
        42420,
        23713,
        [135, 21, 22, 200, 198, 161, 15, 204],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiers_abi(
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
pub struct ITableItemPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITableItemPatternIdentifiersStatics {
    type Vtable = ITableItemPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2178042839,
        26363,
        21487,
        [155, 50, 208, 15, 156, 36, 10, 20],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITablePatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1031773195,
        65423,
        20730,
        [188, 1, 44, 195, 194, 224, 110, 44],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiers_abi(
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
pub struct ITablePatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITablePatternIdentifiersStatics {
    type Vtable = ITablePatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        912298846,
        48315,
        22600,
        [142, 154, 38, 72, 84, 247, 161, 154],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2698174284,
        47705,
        20953,
        [156, 1, 3, 77, 121, 65, 194, 128],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiers_abi(
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
pub struct ITogglePatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITogglePatternIdentifiersStatics {
    type Vtable = ITogglePatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2250842293,
        56499,
        22161,
        [164, 86, 194, 241, 92, 71, 109, 251],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiersStatics_abi(
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
pub struct ITransformPattern2Identifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1861704028,
        56204,
        20912,
        [135, 139, 52, 183, 239, 18, 244, 218],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2Identifiers_abi(
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
pub struct ITransformPattern2IdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransformPattern2IdentifiersStatics {
    type Vtable = ITransformPattern2IdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3649531893,
        35309,
        21299,
        [129, 17, 173, 37, 162, 139, 238, 139],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2IdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        591927419,
        50447,
        23054,
        [188, 5, 48, 90, 199, 27, 59, 107],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiers_abi(
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
pub struct ITransformPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITransformPatternIdentifiersStatics {
    type Vtable = ITransformPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3414000868,
        21545,
        20872,
        [138, 160, 95, 150, 85, 138, 135, 144],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IValuePatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4215878549,
        64407,
        22997,
        [147, 35, 70, 81, 206, 150, 75, 85],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiers_abi(
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
pub struct IValuePatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IValuePatternIdentifiersStatics {
    type Vtable = IValuePatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        538573557,
        52836,
        22951,
        [188, 19, 6, 119, 195, 20, 103, 36],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiersStatics_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiers(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3200612833,
        37310,
        23951,
        [170, 202, 106, 216, 131, 152, 114, 210],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiers_abi(
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
pub struct IWindowPatternIdentifiersStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWindowPatternIdentifiersStatics {
    type Vtable = IWindowPatternIdentifiersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        108406596,
        54231,
        21569,
        [184, 121, 55, 54, 129, 212, 127, 100],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiersStatics_abi(
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
        result__: *mut ::windows::runtime::RawPtr,
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
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct MultipleViewPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl MultipleViewPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CurrentViewProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SupportedViewsProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IMultipleViewPatternIdentifiersStatics<
        R,
        F: FnOnce(&IMultipleViewPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            MultipleViewPatternIdentifiers,
            IMultipleViewPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MultipleViewPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.MultipleViewPatternIdentifiers;{70e4c847-2b82-5ecf-b808-e9d453c1fe53})" ) ;
}
unsafe impl ::windows::runtime::Interface for MultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1894041671,
        11138,
        24271,
        [184, 8, 233, 212, 83, 193, 254, 83],
    );
}
impl ::windows::runtime::RuntimeName for MultipleViewPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.MultipleViewPatternIdentifiers";
}
impl ::std::convert::From<MultipleViewPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&MultipleViewPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for MultipleViewPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a MultipleViewPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<MultipleViewPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MultipleViewPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for MultipleViewPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a MultipleViewPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MultipleViewPatternIdentifiers {}
unsafe impl ::std::marker::Sync for MultipleViewPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RangeValuePatternIdentifiers(pub ::windows::runtime::IInspectable);
impl RangeValuePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsReadOnlyProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn LargeChangeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn MaximumProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn MinimumProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SmallChangeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ValueProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IRangeValuePatternIdentifiersStatics<
        R,
        F: FnOnce(&IRangeValuePatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RangeValuePatternIdentifiers,
            IRangeValuePatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RangeValuePatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.RangeValuePatternIdentifiers;{c114db37-6a75-5ef1-a542-d3b13f92cbfe})" ) ;
}
unsafe impl ::windows::runtime::Interface for RangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3239369527,
        27253,
        24305,
        [165, 66, 211, 177, 63, 146, 203, 254],
    );
}
impl ::windows::runtime::RuntimeName for RangeValuePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.RangeValuePatternIdentifiers";
}
impl ::std::convert::From<RangeValuePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&RangeValuePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RangeValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a RangeValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<RangeValuePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RangeValuePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RangeValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RangeValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RangeValuePatternIdentifiers {}
unsafe impl ::std::marker::Sync for RangeValuePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RowOrColumnMajor(pub i32);
impl RowOrColumnMajor {
    pub const RowMajor: RowOrColumnMajor = RowOrColumnMajor(0i32);
    pub const ColumnMajor: RowOrColumnMajor = RowOrColumnMajor(1i32);
    pub const Indeterminate: RowOrColumnMajor = RowOrColumnMajor(2i32);
}
impl ::std::convert::From<i32> for RowOrColumnMajor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RowOrColumnMajor {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RowOrColumnMajor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.RowOrColumnMajor;i4)",
    );
}
impl ::windows::runtime::DefaultType for RowOrColumnMajor {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ScrollAmount(pub i32);
impl ScrollAmount {
    pub const LargeDecrement: ScrollAmount = ScrollAmount(0i32);
    pub const SmallDecrement: ScrollAmount = ScrollAmount(1i32);
    pub const NoAmount: ScrollAmount = ScrollAmount(2i32);
    pub const LargeIncrement: ScrollAmount = ScrollAmount(3i32);
    pub const SmallIncrement: ScrollAmount = ScrollAmount(4i32);
}
impl ::std::convert::From<i32> for ScrollAmount {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ScrollAmount {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ScrollAmount {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.ScrollAmount;i4)",
    );
}
impl ::windows::runtime::DefaultType for ScrollAmount {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ScrollPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl ScrollPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HorizontallyScrollableProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HorizontalScrollPercentProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn HorizontalViewSizeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn NoScroll() -> ::windows::runtime::Result<f64> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f64>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn VerticallyScrollableProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn VerticalScrollPercentProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn VerticalViewSizeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IScrollPatternIdentifiersStatics<
        R,
        F: FnOnce(&IScrollPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ScrollPatternIdentifiers,
            IScrollPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ScrollPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.ScrollPatternIdentifiers;{04f1a4b8-edc7-55f2-96df-a9c7e809372e})" ) ;
}
unsafe impl ::windows::runtime::Interface for ScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        82945208,
        60871,
        22002,
        [150, 223, 169, 199, 232, 9, 55, 46],
    );
}
impl ::windows::runtime::RuntimeName for ScrollPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.ScrollPatternIdentifiers";
}
impl ::std::convert::From<ScrollPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ScrollPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ScrollPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a ScrollPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ScrollPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ScrollPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ScrollPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ScrollPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ScrollPatternIdentifiers {}
unsafe impl ::std::marker::Sync for ScrollPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SelectionItemPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl SelectionItemPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsSelectedProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SelectionContainerProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISelectionItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&ISelectionItemPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SelectionItemPatternIdentifiers,
            ISelectionItemPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SelectionItemPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.SelectionItemPatternIdentifiers;{ce3a549d-a2cb-594d-a2a4-44778c09cca5})" ) ;
}
unsafe impl ::windows::runtime::Interface for SelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3459929245,
        41675,
        22861,
        [162, 164, 68, 119, 140, 9, 204, 165],
    );
}
impl ::windows::runtime::RuntimeName for SelectionItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.SelectionItemPatternIdentifiers";
}
impl ::std::convert::From<SelectionItemPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SelectionItemPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SelectionItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SelectionItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SelectionItemPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SelectionItemPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SelectionItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SelectionItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SelectionItemPatternIdentifiers {}
unsafe impl ::std::marker::Sync for SelectionItemPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SelectionPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl SelectionPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanSelectMultipleProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsSelectionRequiredProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn SelectionProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISelectionPatternIdentifiersStatics<
        R,
        F: FnOnce(&ISelectionPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SelectionPatternIdentifiers,
            ISelectionPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SelectionPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.SelectionPatternIdentifiers;{401743d2-1fba-5d05-b89f-631676453237})" ) ;
}
unsafe impl ::windows::runtime::Interface for SelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1075266514,
        8122,
        23813,
        [184, 159, 99, 22, 118, 69, 50, 55],
    );
}
impl ::windows::runtime::RuntimeName for SelectionPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.SelectionPatternIdentifiers";
}
impl ::std::convert::From<SelectionPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SelectionPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SelectionPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SelectionPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SelectionPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SelectionPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SelectionPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SelectionPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SelectionPatternIdentifiers {}
unsafe impl ::std::marker::Sync for SelectionPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SpreadsheetItemPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl SpreadsheetItemPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FormulaProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ISpreadsheetItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISpreadsheetItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&ISpreadsheetItemPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SpreadsheetItemPatternIdentifiers,
            ISpreadsheetItemPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpreadsheetItemPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers;{dca2ec46-8564-5c9c-ba90-2c08455f697b})" ) ;
}
unsafe impl ::windows::runtime::Interface for SpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3701664838,
        34148,
        23708,
        [186, 144, 44, 8, 69, 95, 105, 123],
    );
}
impl ::windows::runtime::RuntimeName for SpreadsheetItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers";
}
impl ::std::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for SpreadsheetItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a SpreadsheetItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SpreadsheetItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SpreadsheetItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SpreadsheetItemPatternIdentifiers {}
unsafe impl ::std::marker::Sync for SpreadsheetItemPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct StylesPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl StylesPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ExtendedPropertiesProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FillColorProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FillPatternColorProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn FillPatternStyleProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ShapeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn StyleIdProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn StyleNameProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IStylesPatternIdentifiersStatics<
        R,
        F: FnOnce(&IStylesPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            StylesPatternIdentifiers,
            IStylesPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StylesPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.StylesPatternIdentifiers;{13aeca5e-b496-5df5-aea5-330e1f0490eb})" ) ;
}
unsafe impl ::windows::runtime::Interface for StylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        330222174,
        46230,
        24053,
        [174, 165, 51, 14, 31, 4, 144, 235],
    );
}
impl ::windows::runtime::RuntimeName for StylesPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.StylesPatternIdentifiers";
}
impl ::std::convert::From<StylesPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: StylesPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&StylesPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for StylesPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a StylesPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<StylesPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: StylesPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StylesPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for StylesPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a StylesPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for StylesPatternIdentifiers {}
unsafe impl ::std::marker::Sync for StylesPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SupportedTextSelection(pub i32);
impl SupportedTextSelection {
    pub const None: SupportedTextSelection = SupportedTextSelection(0i32);
    pub const Single: SupportedTextSelection = SupportedTextSelection(1i32);
    pub const Multiple: SupportedTextSelection = SupportedTextSelection(2i32);
}
impl ::std::convert::From<i32> for SupportedTextSelection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SupportedTextSelection {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SupportedTextSelection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.SupportedTextSelection;i4)",
    );
}
impl ::windows::runtime::DefaultType for SupportedTextSelection {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SynchronizedInputType(pub i32);
impl SynchronizedInputType {
    pub const KeyUp: SynchronizedInputType = SynchronizedInputType(1i32);
    pub const KeyDown: SynchronizedInputType = SynchronizedInputType(2i32);
    pub const LeftMouseUp: SynchronizedInputType = SynchronizedInputType(4i32);
    pub const LeftMouseDown: SynchronizedInputType = SynchronizedInputType(8i32);
    pub const RightMouseUp: SynchronizedInputType = SynchronizedInputType(16i32);
    pub const RightMouseDown: SynchronizedInputType = SynchronizedInputType(32i32);
}
impl ::std::convert::From<i32> for SynchronizedInputType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SynchronizedInputType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SynchronizedInputType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.SynchronizedInputType;i4)",
    );
}
impl ::windows::runtime::DefaultType for SynchronizedInputType {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TableItemPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl TableItemPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnHeaderItemsProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowHeaderItemsProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITableItemPatternIdentifiersStatics<
        R,
        F: FnOnce(&ITableItemPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TableItemPatternIdentifiers,
            ITableItemPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TableItemPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TableItemPatternIdentifiers;{b4de5d03-a5b4-5ca1-8715-16c8c6a10fcc})" ) ;
}
unsafe impl ::windows::runtime::Interface for TableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3034471683,
        42420,
        23713,
        [135, 21, 22, 200, 198, 161, 15, 204],
    );
}
impl ::windows::runtime::RuntimeName for TableItemPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TableItemPatternIdentifiers";
}
impl ::std::convert::From<TableItemPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TableItemPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for TableItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a TableItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TableItemPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TableItemPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TableItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TableItemPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TableItemPatternIdentifiers {}
unsafe impl ::std::marker::Sync for TableItemPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TablePatternIdentifiers(pub ::windows::runtime::IInspectable);
impl TablePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ColumnHeadersProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowHeadersProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn RowOrColumnMajorProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITablePatternIdentifiersStatics<
        R,
        F: FnOnce(&ITablePatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TablePatternIdentifiers,
            ITablePatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TablePatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TablePatternIdentifiers;{3d7f9c0b-ff8f-50fa-bc01-2cc3c2e06e2c})" ) ;
}
unsafe impl ::windows::runtime::Interface for TablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1031773195,
        65423,
        20730,
        [188, 1, 44, 195, 194, 224, 110, 44],
    );
}
impl ::windows::runtime::RuntimeName for TablePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TablePatternIdentifiers";
}
impl ::std::convert::From<TablePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: TablePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TablePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &TablePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for TablePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a TablePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TablePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: TablePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TablePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &TablePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TablePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TablePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TablePatternIdentifiers {}
unsafe impl ::std::marker::Sync for TablePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TogglePatternIdentifiers(pub ::windows::runtime::IInspectable);
impl TogglePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ToggleStateProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITogglePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITogglePatternIdentifiersStatics<
        R,
        F: FnOnce(&ITogglePatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TogglePatternIdentifiers,
            ITogglePatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TogglePatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TogglePatternIdentifiers;{a0d2df4c-ba59-51d9-9c01-034d7941c280})" ) ;
}
unsafe impl ::windows::runtime::Interface for TogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2698174284,
        47705,
        20953,
        [156, 1, 3, 77, 121, 65, 194, 128],
    );
}
impl ::windows::runtime::RuntimeName for TogglePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TogglePatternIdentifiers";
}
impl ::std::convert::From<TogglePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: TogglePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TogglePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for TogglePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a TogglePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TogglePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: TogglePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TogglePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TogglePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TogglePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TogglePatternIdentifiers {}
unsafe impl ::std::marker::Sync for TogglePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ToggleState(pub i32);
impl ToggleState {
    pub const Off: ToggleState = ToggleState(0i32);
    pub const On: ToggleState = ToggleState(1i32);
    pub const Indeterminate: ToggleState = ToggleState(2i32);
}
impl ::std::convert::From<i32> for ToggleState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ToggleState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ToggleState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.ToggleState;i4)",
    );
}
impl ::windows::runtime::DefaultType for ToggleState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TransformPattern2Identifiers(pub ::windows::runtime::IInspectable);
impl TransformPattern2Identifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanZoomProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ZoomLevelProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn MaxZoomProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn MinZoomProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITransformPattern2IdentifiersStatics<
        R,
        F: FnOnce(&ITransformPattern2IdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TransformPattern2Identifiers,
            ITransformPattern2IdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TransformPattern2Identifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TransformPattern2Identifiers;{6ef7595c-db8c-51b0-878b-34b7ef12f4da})" ) ;
}
unsafe impl ::windows::runtime::Interface for TransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1861704028,
        56204,
        20912,
        [135, 139, 52, 183, 239, 18, 244, 218],
    );
}
impl ::windows::runtime::RuntimeName for TransformPattern2Identifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TransformPattern2Identifiers";
}
impl ::std::convert::From<TransformPattern2Identifiers> for ::windows::runtime::IUnknown {
    fn from(value: TransformPattern2Identifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TransformPattern2Identifiers> for ::windows::runtime::IUnknown {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for TransformPattern2Identifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a TransformPattern2Identifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TransformPattern2Identifiers> for ::windows::runtime::IInspectable {
    fn from(value: TransformPattern2Identifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TransformPattern2Identifiers> for ::windows::runtime::IInspectable {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TransformPattern2Identifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TransformPattern2Identifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TransformPattern2Identifiers {}
unsafe impl ::std::marker::Sync for TransformPattern2Identifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TransformPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl TransformPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanMoveProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanResizeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanRotateProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITransformPatternIdentifiersStatics<
        R,
        F: FnOnce(&ITransformPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            TransformPatternIdentifiers,
            ITransformPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TransformPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.TransformPatternIdentifiers;{2348187b-c50f-5a0e-bc05-305ac71b3b6b})" ) ;
}
unsafe impl ::windows::runtime::Interface for TransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        591927419,
        50447,
        23054,
        [188, 5, 48, 90, 199, 27, 59, 107],
    );
}
impl ::windows::runtime::RuntimeName for TransformPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.TransformPatternIdentifiers";
}
impl ::std::convert::From<TransformPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: TransformPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&TransformPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for TransformPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a TransformPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<TransformPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: TransformPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TransformPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for TransformPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a TransformPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TransformPatternIdentifiers {}
unsafe impl ::std::marker::Sync for TransformPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ValuePatternIdentifiers(pub ::windows::runtime::IInspectable);
impl ValuePatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsReadOnlyProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn ValueProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IValuePatternIdentifiersStatics<
        R,
        F: FnOnce(&IValuePatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ValuePatternIdentifiers,
            IValuePatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ValuePatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.ValuePatternIdentifiers;{fb493395-fb97-59d5-9323-4651ce964b55})" ) ;
}
unsafe impl ::windows::runtime::Interface for ValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4215878549,
        64407,
        22997,
        [147, 35, 70, 81, 206, 150, 75, 85],
    );
}
impl ::windows::runtime::RuntimeName for ValuePatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.ValuePatternIdentifiers";
}
impl ::std::convert::From<ValuePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: ValuePatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ValuePatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for ValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a ValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ValuePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: ValuePatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ValuePatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ValuePatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ValuePatternIdentifiers {}
unsafe impl ::std::marker::Sync for ValuePatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WindowInteractionState(pub i32);
impl WindowInteractionState {
    pub const Running: WindowInteractionState = WindowInteractionState(0i32);
    pub const Closing: WindowInteractionState = WindowInteractionState(1i32);
    pub const ReadyForUserInteraction: WindowInteractionState = WindowInteractionState(2i32);
    pub const BlockedByModalWindow: WindowInteractionState = WindowInteractionState(3i32);
    pub const NotResponding: WindowInteractionState = WindowInteractionState(4i32);
}
impl ::std::convert::From<i32> for WindowInteractionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WindowInteractionState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WindowInteractionState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.WindowInteractionState;i4)",
    );
}
impl ::windows::runtime::DefaultType for WindowInteractionState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WindowPatternIdentifiers(pub ::windows::runtime::IInspectable);
impl WindowPatternIdentifiers {
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanMaximizeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn CanMinimizeProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsModalProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn IsTopmostProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn WindowInteractionStateProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    #[doc = "*Required features: `UI_Xaml_Automation`*"]
    pub fn WindowVisualStateProperty() -> ::windows::runtime::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IWindowPatternIdentifiersStatics<
        R,
        F: FnOnce(&IWindowPatternIdentifiersStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            WindowPatternIdentifiers,
            IWindowPatternIdentifiersStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WindowPatternIdentifiers {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Microsoft.UI.Xaml.Automation.WindowPatternIdentifiers;{bec579e1-91be-5d8f-aaca-6ad8839872d2})" ) ;
}
unsafe impl ::windows::runtime::Interface for WindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3200612833,
        37310,
        23951,
        [170, 202, 106, 216, 131, 152, 114, 210],
    );
}
impl ::windows::runtime::RuntimeName for WindowPatternIdentifiers {
    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.WindowPatternIdentifiers";
}
impl ::std::convert::From<WindowPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: WindowPatternIdentifiers) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&WindowPatternIdentifiers> for ::windows::runtime::IUnknown {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WindowPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &'a WindowPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<WindowPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: WindowPatternIdentifiers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WindowPatternIdentifiers> for ::windows::runtime::IInspectable {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WindowPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WindowPatternIdentifiers
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WindowPatternIdentifiers {}
unsafe impl ::std::marker::Sync for WindowPatternIdentifiers {}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WindowVisualState(pub i32);
impl WindowVisualState {
    pub const Normal: WindowVisualState = WindowVisualState(0i32);
    pub const Maximized: WindowVisualState = WindowVisualState(1i32);
    pub const Minimized: WindowVisualState = WindowVisualState(2i32);
}
impl ::std::convert::From<i32> for WindowVisualState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WindowVisualState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WindowVisualState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.WindowVisualState;i4)",
    );
}
impl ::windows::runtime::DefaultType for WindowVisualState {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Xaml_Automation`*"]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ZoomUnit(pub i32);
impl ZoomUnit {
    pub const NoAmount: ZoomUnit = ZoomUnit(0i32);
    pub const LargeDecrement: ZoomUnit = ZoomUnit(1i32);
    pub const SmallDecrement: ZoomUnit = ZoomUnit(2i32);
    pub const LargeIncrement: ZoomUnit = ZoomUnit(3i32);
    pub const SmallIncrement: ZoomUnit = ZoomUnit(4i32);
}
impl ::std::convert::From<i32> for ZoomUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ZoomUnit {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ZoomUnit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Xaml.Automation.ZoomUnit;i4)",
    );
}
impl ::windows::runtime::DefaultType for ZoomUnit {
    type DefaultType = Self;
}
