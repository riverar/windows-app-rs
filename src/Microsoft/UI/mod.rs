#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "UI_Composition")]
pub mod Composition;
#[cfg(feature = "UI_Dispatching")]
pub mod Dispatching;
#[cfg(feature = "UI_Input")]
pub mod Input;
#[cfg(feature = "UI_Text")]
pub mod Text;
#[cfg(feature = "UI_Windowing")]
pub mod Windowing;
#[cfg(feature = "UI_Xaml")]
pub mod Xaml;
#[doc = "*Required features: `UI`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ColorHelper(pub ::windows::runtime::IInspectable);
impl ColorHelper {
    #[doc = "*Required features: `UI`*"]
    pub fn FromArgb(
        a: u8,
        r: u8,
        g: u8,
        b: u8,
    ) -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                a,
                r,
                g,
                b,
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    pub fn IColorHelperStatics<
        R,
        F: FnOnce(&IColorHelperStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ColorHelper, IColorHelperStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ColorHelper {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.ColorHelper;{3adddccd-3949-585b-a566-ccb8350dd221})",
    );
}
unsafe impl ::windows::runtime::Interface for ColorHelper {
    type Vtable = IColorHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        987618509,
        14665,
        22619,
        [165, 102, 204, 184, 53, 13, 210, 33],
    );
}
impl ::windows::runtime::RuntimeName for ColorHelper {
    const NAME: &'static str = "Microsoft.UI.ColorHelper";
}
impl ::std::convert::From<ColorHelper> for ::windows::runtime::IUnknown {
    fn from(value: ColorHelper) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ColorHelper> for ::windows::runtime::IUnknown {
    fn from(value: &ColorHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ColorHelper> for ::windows::runtime::IInspectable {
    fn from(value: ColorHelper) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ColorHelper> for ::windows::runtime::IInspectable {
    fn from(value: &ColorHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ColorHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ColorHelper {}
unsafe impl ::std::marker::Sync for ColorHelper {}
#[doc = "*Required features: `UI`*"]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Colors(pub ::windows::runtime::IInspectable);
impl Colors {
    #[doc = "*Required features: `UI`*"]
    pub fn AliceBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn AntiqueWhite() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Aqua() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Aquamarine() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Azure() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Beige() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Bisque() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Black() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn BlanchedAlmond() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Blue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn BlueViolet() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Brown() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn BurlyWood() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn CadetBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Chartreuse() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Chocolate() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Coral() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn CornflowerBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Cornsilk() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Crimson() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Cyan() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkCyan() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkGoldenrod() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkGray() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkKhaki() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkMagenta() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkOliveGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkOrange() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkOrchid() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkRed() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSalmon() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSeaGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSlateBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkSlateGray() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkTurquoise() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DarkViolet() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DeepPink() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DeepSkyBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DimGray() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn DodgerBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Firebrick() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn FloralWhite() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn ForestGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Fuchsia() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Gainsboro() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn GhostWhite() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Gold() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Goldenrod() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Gray() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Green() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn GreenYellow() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Honeydew() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).59)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn HotPink() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn IndianRed() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).61)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Indigo() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).62)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Ivory() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).63)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Khaki() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).64)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Lavender() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).65)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LavenderBlush() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).66)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LawnGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).67)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LemonChiffon() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).68)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).69)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightCoral() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).70)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightCyan() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).71)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightGoldenrodYellow() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).72)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).73)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightGray() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).74)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightPink() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).75)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSalmon() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).76)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSeaGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).77)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSkyBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).78)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSlateGray() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightSteelBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).80)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LightYellow() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).81)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Lime() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).82)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn LimeGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).83)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Linen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).84)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Magenta() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).85)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Maroon() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).86)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumAquamarine() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).87)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).88)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumOrchid() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).89)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumPurple() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).90)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumSeaGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).91)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumSlateBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).92)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumSpringGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).93)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumTurquoise() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).94)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MediumVioletRed() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).95)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MidnightBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).96)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MintCream() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).97)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn MistyRose() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).98)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Moccasin() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).99)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn NavajoWhite() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).100)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Navy() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).101)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn OldLace() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).102)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Olive() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).103)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn OliveDrab() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).104)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Orange() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).105)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn OrangeRed() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).106)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Orchid() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).107)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleGoldenrod() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).108)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).109)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleTurquoise() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).110)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PaleVioletRed() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).111)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PapayaWhip() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).112)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PeachPuff() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).113)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Peru() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).114)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Pink() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).115)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Plum() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).116)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn PowderBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).117)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Purple() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).118)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Red() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).119)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn RosyBrown() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).120)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn RoyalBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).121)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SaddleBrown() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).122)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Salmon() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).123)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SandyBrown() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).124)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SeaGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).125)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SeaShell() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).126)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Sienna() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).127)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Silver() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).128)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SkyBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).129)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SlateBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).130)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SlateGray() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).131)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Snow() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).132)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SpringGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).133)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn SteelBlue() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).134)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Tan() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).135)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Teal() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).136)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Thistle() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).137)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Tomato() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).138)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Transparent() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).139)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Turquoise() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).140)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Violet() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).141)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Wheat() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).142)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn White() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).143)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn WhiteSmoke() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).144)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn Yellow() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).145)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    #[doc = "*Required features: `UI`*"]
    pub fn YellowGreen() -> ::windows::runtime::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__: ::windows::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).146)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::UI::Color>(result__)
        })
    }
    pub fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Colors, IColorsStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Colors {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Colors;{8cf15863-8411-5afd-946c-328e04da2f2f})",
    );
}
unsafe impl ::windows::runtime::Interface for Colors {
    type Vtable = IColors_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2364627043,
        33809,
        23293,
        [148, 108, 50, 142, 4, 218, 47, 47],
    );
}
impl ::windows::runtime::RuntimeName for Colors {
    const NAME: &'static str = "Microsoft.UI.Colors";
}
impl ::std::convert::From<Colors> for ::windows::runtime::IUnknown {
    fn from(value: Colors) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&Colors> for ::windows::runtime::IUnknown {
    fn from(value: &Colors) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<Colors> for ::windows::runtime::IInspectable {
    fn from(value: Colors) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Colors> for ::windows::runtime::IInspectable {
    fn from(value: &Colors) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Colors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Colors {}
unsafe impl ::std::marker::Sync for Colors {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI`*"]
pub struct DisplayId {
    pub Value: u64,
}
impl DisplayId {}
impl ::std::default::Default for DisplayId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DisplayId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DisplayId")
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DisplayId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::std::cmp::Eq for DisplayId {}
unsafe impl ::windows::runtime::Abi for DisplayId {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayId {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"struct(Microsoft.UI.DisplayId;u8)");
}
impl ::windows::runtime::DefaultType for DisplayId {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IColorHelper(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColorHelper {
    type Vtable = IColorHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        987618509,
        14665,
        22619,
        [165, 102, 204, 184, 53, 13, 210, 33],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelper_abi(
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
pub struct IColorHelperStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColorHelperStatics {
    type Vtable = IColorHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        488474017,
        60259,
        21386,
        [132, 240, 1, 146, 16, 188, 64, 107],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics_abi(
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
        a: u8,
        r: u8,
        g: u8,
        b: u8,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IColors(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColors {
    type Vtable = IColors_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2364627043,
        33809,
        23293,
        [148, 108, 50, 142, 4, 218, 47, 47],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColors_abi(
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
pub struct IColorsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IColorsStatics {
    type Vtable = IColorsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2250286512,
        346,
        22444,
        [163, 243, 137, 93, 11, 18, 105, 174],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsStatics_abi(
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
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI`*"]
pub struct IconId {
    pub Value: u64,
}
impl IconId {}
impl ::std::default::Default for IconId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IconId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IconId")
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IconId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::std::cmp::Eq for IconId {}
unsafe impl ::windows::runtime::Abi for IconId {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IconId {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"struct(Microsoft.UI.IconId;u8)");
}
impl ::windows::runtime::DefaultType for IconId {
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `UI`*"]
pub struct WindowId {
    pub Value: u64,
}
impl WindowId {}
impl ::std::default::Default for WindowId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WindowId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WindowId")
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WindowId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::std::cmp::Eq for WindowId {}
unsafe impl ::windows::runtime::Abi for WindowId {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WindowId {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"struct(Microsoft.UI.WindowId;u8)");
}
impl ::windows::runtime::DefaultType for WindowId {
    type DefaultType = Self;
}
