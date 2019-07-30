// plugin-system
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate proc_macro;

use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, AttributeArgs, Error, FnArg, ItemFn, Pat, Path, Type};

#[derive(darling::FromMeta)]
struct PluginManifestInput {
    #[darling(rename = "for")]
    core_name: String,
}

#[proc_macro_attribute]
pub fn declare_plugin(
    meta: proc_macro::TokenStream,
    func: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let meta = parse_macro_input!(meta as AttributeArgs);
    let func = parse_macro_input!(func as ItemFn);
    let input = match PluginManifestInput::from_list(&meta) {
        Ok(input) => input,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let output = match plugin_impl(input, func) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error(),
    };
    output.into()
}

fn plugin_impl(input: PluginManifestInput, func: ItemFn) -> syn::Result<TokenStream> {
    let _core_name = &input.core_name; // TODO check correctness of core
    let func_name = &func.ident;
    let maps_args = func
        .decl
        .inputs
        .iter()
        .map(|arg| {
            Ok(match arg_to_dep(&arg)? {
                Some(ident_name) => {
                    quote!(require_plugin!(deps, #ident_name))
                }
                None => quote!(&mut map),
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let output = quote! {
        pub struct PluginManifestImpl;

        impl ::plugin_runtime::PluginManifest for PluginManifestImpl {
            fn init(deps: &mut ::plugin_runtime::PluginList) -> ::plugin_runtime::FeatureMap {
                let mut map = FeatureMap::default();
                #func_name(#(#maps_args),*);
                map
            }
        }

        #func
    };
    Ok(output.into())
}

#[allow(unused)]
fn test_path(path: &Path, expects: &[&str]) -> bool {
    let mut iter = expects.iter();
    for segment in &path.segments {
        dbg!(&segment.ident);
        if let Some(expect) = iter.next() {
            if expect != &segment.ident.to_string() {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}

fn is_feature_map(_path: &Path) -> bool {
    // test_path(path, &["plugin_runtime", "FeatureMap"])
    true
}

fn is_option_feature_map(_path: &Path) -> bool {
    false
    //    if !test_path(path, &["std", "option", "Option"]) {
    //        return false;
    //    }
    //    let types = match &path.segments[2].arguments {
    //        PathArguments::AngleBracketed(types) => types,
    //        _ => { return false; },
    //    };
    //    if types.args.len() != 1 {
    //        return false;
    //    }
    //    let arg = &types.args[0];
    //    let ty = match &arg {
    //        GenericArgument::Type(ty) => ty,
    //        _ => { return false; },
    //    };
    //    let path = match ty {
    //        Type::Path(path) => &path.path,
    //        _ => { return false; },
    //    };
    //    is_feature_map(path)
}

fn arg_to_dep(arg: &FnArg) -> syn::Result<Option<Ident>> {
    match arg {
        FnArg::Captured(arg_captured) => {
            let ty = &arg_captured.ty;
            let pat = &arg_captured.pat;

            let (optional, mutable) = match ty {
                Type::Reference(reference) => {
                    if reference.lifetime.is_some() {
                        return Err(Error::new(
                            ty.span(),
                            "Expected &FeatureMap or &mut FeatureMap, found lifetime",
                        ));
                    }
                    let optional = match reference.elem.as_ref() {
                        Type::Path(path) => {
                            if is_feature_map(&path.path) {
                                false
                            } else if is_option_feature_map(&path.path) {
                                true
                            } else {
                                return Err(Error::new(
                                    ty.span(),
                                    "Expected &[mut] FeatureMap, got unexpected type",
                                ));
                            }
                        }
                        _ => {
                            return Err(Error::new(
                                ty.span(),
                                "Expected &[mut] FeatureMap, got complex type",
                            ));
                        }
                    };
                    let mutable = reference.mutability.is_some();
                    (optional, mutable)
                }
                _ => {
                    return Err(Error::new(
                        ty.span(),
                        "Expected &FeatureMap or &mut FeatureMap, got non-reference",
                    ))
                }
            };

            let ident = match pat {
                Pat::Ident(ident) => &ident.ident,
                _ => {
                    return Err(Error::new(pat.span(), "Expected a single argument name \"this\" or indicating dependency package name"))
                }
            };
            let ident_name = ident.to_string();

            if optional {
                return Err(Error::new(
                    ty.span(),
                    "Option<FeatureMap> is not implemented yet",
                ));
            }
            let arg = if "this" == &ident_name {
                if !mutable {
                    return Err(Error::new(
                        ty.span(),
                        "\"this\" argument should be \"&mut FeatureMap\"",
                    ));
                }
                None
            } else {
                if mutable {
                    return Err(Error::new(
                        ty.span(),
                        "All arguments except \"this\" should be \"&FeatureMap\"",
                    ));
                }
                let ident_name = if ident_name.starts_with('_') {
                    &ident_name[1..]
                } else {
                    &ident_name[..]
                };
                let ident_name = Ident::new(ident_name, Span::call_site());
                Some(ident_name)
            };
            Ok(arg)
        }
        _ => Err(Error::new(arg.span(), "unexpected argument type")),
    }
}
