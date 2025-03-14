use super::*;

pub fn gen(def: &TypeDef, gen: &Gen) -> TokenStream {
    if gen.sys {
        if def.default_interface().is_some() {
            let name = gen_type_ident(def, gen);
            quote! {
                pub type #name = *mut ::core::ffi::c_void;
            }
        } else {
            quote! {}
        }
    } else {
        gen_class(def, gen)
    }
}

fn gen_class(def: &TypeDef, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let has_default = def.default_interface().is_some();
    let interfaces = def.class_interfaces();
    let mut methods = quote! {};
    let mut method_names = MethodNames::new();

    let cfg = def.cfg();
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);

    for (def, kind) in &interfaces {
        if gen.min_xaml && *kind == InterfaceKind::Base && gen.namespace.starts_with("Windows.UI.Xaml") && !def.namespace().starts_with("Windows.Foundation") {
            continue;
        }

        let mut virtual_names = MethodNames::new();

        for method in def.methods() {
            methods.combine(&gen_winrt_method(def, *kind, &method, &mut method_names, &mut virtual_names, gen));
        }
    }

    let factories = interfaces.iter().filter_map(|(def, kind)| match kind {
        InterfaceKind::Static | InterfaceKind::Composable => {
            if def.methods().next().is_some() {
                let interface_name = format_token!("{}", def.name());
                let interface_type = gen_type_name(def, gen);
                let features = gen.cfg(&def.cfg());

                let hidden = if gen.doc {
                    quote! { #[doc(hidden)] }
                } else {
                    quote! {}
                };

                Some(quote! {
                    #hidden
                    #features
                    pub fn #interface_name<R, F: FnOnce(&#interface_type) -> ::windows::core::Result<R>>(
                        callback: F,
                    ) -> ::windows::core::Result<R> {
                        static mut SHARED: ::windows::core::FactoryCache<#name, #interface_type> =
                            ::windows::core::FactoryCache::new();
                        unsafe { SHARED.call(callback) }
                    }
                })
            } else {
                None
            }
        }
        _ => None,
    });

    if has_default {
        let new = if def.has_default_constructor() {
            quote! {
                pub fn new() -> ::windows::core::Result<Self> {
                    Self::IActivationFactory(|f| f.activate_instance::<Self>())
                }
                fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(
                    callback: F,
                ) -> ::windows::core::Result<R> {
                    static mut SHARED: ::windows::core::FactoryCache<#name, ::windows::core::IActivationFactory> =
                        ::windows::core::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
            }
        } else {
            quote! {}
        };

        let mut tokens = quote! {
            #doc
            #features
            #[repr(transparent)]
            pub struct #name(::windows::core::IUnknown);
            #features
            impl #name {
                #new
                #methods
                #(#factories)*
            }
        };

        tokens.combine(&gen_std_traits(def, &cfg, gen));
        tokens.combine(&gen_runtime_trait(def, &cfg, gen));
        tokens.combine(&gen_interface_trait(def, &cfg, gen));
        tokens.combine(&gen_runtime_name(def, &cfg, gen));
        tokens.combine(&gen_async(def, &cfg, gen));
        tokens.combine(&gen_iterator(def, &cfg, gen));
        tokens.combine(&gen_conversions(def, &cfg, gen));
        tokens.combine(&gen_agile(def, &cfg, gen));
        tokens
    } else {
        let mut tokens = quote! {
            #doc
            #features
            pub struct #name {}
            #features
            impl #name {
                #methods
                #(#factories)*
            }
        };

        tokens.combine(&gen_runtime_name(def, &cfg, gen));
        tokens
    }
}

fn gen_agile(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    if def.is_agile() {
        let name = gen_type_ident(def, gen);
        let cfg = gen.cfg(cfg);
        quote! {
            #cfg
            unsafe impl ::core::marker::Send for #name {}
            #cfg
            unsafe impl ::core::marker::Sync for #name {}
        }
    } else {
        TokenStream::new()
    }
}

fn gen_conversions(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let name = gen_type_ident(def, gen);
    let mut tokens = quote! {};

    for def in &[Type::IUnknown, Type::IInspectable] {
        let into = gen_element_name(def, gen);
        let cfg = gen.cfg(cfg);
        tokens.combine(&quote! {
            #cfg
            impl ::core::convert::From<#name> for #into {
                fn from(value: #name) -> Self {
                    unsafe { ::core::mem::transmute(value) }
                }
            }
            #cfg
            impl ::core::convert::From<&#name> for #into {
                fn from(value: &#name) -> Self {
                    ::core::convert::From::from(::core::clone::Clone::clone(value))
                }
            }
            #cfg
            impl<'a> ::windows::core::IntoParam<'a, #into> for #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
                }
            }
            #cfg
            impl<'a> ::windows::core::IntoParam<'a, #into> for &'a #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
                }
            }
        });
    }

    for (def, kind) in def.class_interfaces() {
        if def.is_exclusive() {
            continue;
        }

        if kind != InterfaceKind::Default && kind != InterfaceKind::NonDefault && kind != InterfaceKind::Base {
            continue;
        }

        let into = gen_type_name(&def, gen);
        // TODO: simplify - maybe provide + operator?
        let cfg = gen.cfg(&cfg.union(&def.cfg()));

        tokens.combine(&quote! {
            #cfg
            impl ::core::convert::TryFrom<#name> for #into {
                type Error = ::windows::core::Error;
                fn try_from(value: #name) -> ::windows::core::Result<Self> {
                    ::core::convert::TryFrom::try_from(&value)
                }
            }
            #cfg
            impl ::core::convert::TryFrom<&#name> for #into {
                type Error = ::windows::core::Error;
                fn try_from(value: &#name) -> ::windows::core::Result<Self> {
                    ::windows::core::Interface::cast(value)
                }
            }
            #cfg
            impl<'a> ::windows::core::IntoParam<'a, #into> for #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::IntoParam::into_param(&self)
                }
            }
            #cfg
            impl<'a> ::windows::core::IntoParam<'a, #into> for &#name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::core::convert::TryInto::<#into>::try_into(self)
                        .map(::windows::core::Param::Owned)
                        .unwrap_or(::windows::core::Param::None)
                }
            }
        });
    }

    for def in def.bases() {
        let into = gen_type_name(&def, gen);
        let cfg = gen.cfg(&cfg.union(&def.cfg()));

        tokens.combine(&quote! {
            #cfg
            impl ::core::convert::From<#name> for #into {
                fn from(value: #name) -> Self {
                    ::core::convert::From::from(&value)
                }
            }
            #cfg
            impl ::core::convert::From<&#name> for #into {
                fn from(value: &#name) -> Self {
                    // This unwrap is legitimate because conversion to base can never fail because
                    // the base can never change across versions.
                    ::windows::core::Interface::cast(value).unwrap()
                }
            }
            #cfg
            impl<'a> ::windows::core::IntoParam<'a, #into> for #name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::IntoParam::into_param(&self)
                }
            }
            #cfg
            impl<'a> ::windows::core::IntoParam<'a, #into> for &#name {
                fn into_param(self) -> ::windows::core::Param<'a, #into> {
                    ::windows::core::Param::Owned(::core::convert::Into::<#into>::into(self))
                }
            }
        });
    }

    tokens
}
