use quote::ToTokens;
use syn::{AttrStyle, DeriveInput, Error, Ident, Lit, LitStr, Meta, MetaList, NestedMeta, Path};

pub struct Repr {
    pub rust: Option<Path>,
    pub transparent: Option<Path>,
    pub packed: Option<Path>,
    pub c: Option<Path>,
    pub int: Option<Path>,
}

impl Default for Repr {
    fn default() -> Self {
        Self {
            rust: None,
            transparent: None,
            packed: None,
            c: None,
            int: None,
        }
    }
}

pub struct Attributes {
    pub copy: Option<Path>,
    pub repr: Repr,
    pub derives: Option<MetaList>,
    pub compares: Option<(Path, Vec<Path>)>,
    pub serialize_bound: Option<LitStr>,
    pub deserialize_bound: Option<LitStr>,
    pub archived: Option<Ident>,
    pub resolver: Option<Ident>,
    pub strict: Option<Path>,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            copy: None,
            repr: Default::default(),
            derives: None,
            compares: None,
            serialize_bound: None,
            deserialize_bound: None,
            archived: None,
            resolver: None,
            strict: None,
        }
    }
}

fn try_set_attribute<T: ToTokens>(
    attribute: &mut Option<T>,
    value: T,
    name: &'static str,
) -> Result<(), Error> {
    if attribute.is_none() {
        *attribute = Some(value);
        Ok(())
    } else {
        Err(Error::new_spanned(
            value,
            &format!("{} already specified", name),
        ))
    }
}

fn parse_archive_attributes(attributes: &mut Attributes, meta: &Meta) -> Result<(), Error> {
    match meta {
        Meta::Path(path) => {
            if path.is_ident("copy") {
                try_set_attribute(&mut attributes.copy, path.clone(), "copy")
            } else if path.is_ident("strict") {
                try_set_attribute(&mut attributes.strict, path.clone(), "strict")
            } else {
                Err(Error::new_spanned(path, "unrecognized archive parameter"))
            }
        }
        Meta::List(list) => {
            if list.path.is_ident("derive") {
                try_set_attribute(&mut attributes.derives, list.clone(), "derive")
            } else if list.path.is_ident("compare") {
                if attributes.compares.is_none() {
                    let mut compares = Vec::new();
                    for compare in list.nested.iter() {
                        if let NestedMeta::Meta(Meta::Path(path)) = compare {
                            compares.push(path.clone());
                        } else {
                            return Err(Error::new_spanned(
                                compare,
                                "compare arguments must be compare traits to derive",
                            ));
                        }
                    }
                    attributes.compares = Some((list.path.clone(), compares));
                    Ok(())
                } else {
                    Err(Error::new_spanned(list, "compares already specified"))
                }
            } else if list.path.is_ident("bound") {
                for bound in list.nested.iter() {
                    if let NestedMeta::Meta(Meta::NameValue(name_value)) = bound {
                        if let Lit::Str(ref lit_str) = name_value.lit {
                            if name_value.path.is_ident("serialize") {
                                if attributes.serialize_bound.is_none() {
                                    attributes.serialize_bound = Some(lit_str.clone());
                                } else {
                                    return Err(Error::new_spanned(
                                        bound,
                                        "serialize bound already specified",
                                    ));
                                }
                            } else if name_value.path.is_ident("deserialize") {
                                if attributes.deserialize_bound.is_none() {
                                    attributes.deserialize_bound = Some(lit_str.clone());
                                } else {
                                    return Err(Error::new_spanned(
                                        bound,
                                        "serialize bound already specified",
                                    ));
                                }
                            } else {
                                return Err(Error::new_spanned(
                                    bound,
                                    "bounds must be either serialize or deserialize",
                                ));
                            }
                        } else {
                            return Err(Error::new_spanned(
                                bound,
                                "bounds arguments must be a string",
                            ));
                        }
                    } else {
                        return Err(Error::new_spanned(
                            bound,
                            "bounds arguments must be serialize or deserialize bounds to apply",
                        ));
                    }
                }
                Ok(())
            } else {
                Err(Error::new_spanned(
                    &list.path,
                    "unrecognized archive parameter",
                ))
            }
        }
        Meta::NameValue(meta) => {
            if meta.path.is_ident("archived") {
                if let Lit::Str(ref lit_str) = meta.lit {
                    try_set_attribute(
                        &mut attributes.archived,
                        Ident::new(&lit_str.value(), lit_str.span()),
                        "archived",
                    )
                } else {
                    Err(Error::new_spanned(meta, "archived must be a string"))
                }
            } else if meta.path.is_ident("resolver") {
                if let Lit::Str(ref lit_str) = meta.lit {
                    try_set_attribute(
                        &mut attributes.resolver,
                        Ident::new(&lit_str.value(), lit_str.span()),
                        "resolver",
                    )
                } else {
                    Err(Error::new_spanned(meta, "resolver must be a string"))
                }
            } else {
                Err(Error::new_spanned(meta, "unrecognized archive parameter"))
            }
        }
    }
}

pub fn parse_attributes(input: &DeriveInput) -> Result<Attributes, Error> {
    let mut result = Attributes::default();
    for attr in input.attrs.iter() {
        if let AttrStyle::Outer = attr.style {
            if let Ok(Meta::List(meta)) = attr.parse_meta() {
                if meta.path.is_ident("archive") {
                    for nested in meta.nested.iter() {
                        if let NestedMeta::Meta(meta) = nested {
                            parse_archive_attributes(&mut result, meta)?;
                        }
                    }
                } else if meta.path.is_ident("repr") {
                    for n in meta.nested.iter() {
                        if let NestedMeta::Meta(Meta::Path(path)) = n {
                            if path.is_ident("rust") {
                                result.repr.rust = Some(path.clone());
                            } else if path.is_ident("transparent") {
                                result.repr.transparent = Some(path.clone());
                            } else if path.is_ident("packed") {
                                result.repr.packed = Some(path.clone());
                            } else if path.is_ident("C") {
                                result.repr.c = Some(path.clone());
                            } else {
                                result.repr.int = Some(path.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(result)
}
