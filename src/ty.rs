use {
    crate::special_data::Special,
    syn::{
        parse::{Parse, ParseStream},
        Type,
    },
};

/// Can either be a normal type, or a type definition
pub enum SpecialType {
    /// Our curstom `struct`/`enum` syntax
    Def(Special),
    /// A normal Rust type with custom parsing to support nested special types
    /// (for exemple in generics).
    Augmented(augmented::Type),
    /// A normal Rust type
    Type(Type),
}

impl Parse for SpecialType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Some(ty) = augmented::Type::maybe_parse(input)? {
            Ok(SpecialType::Augmented(ty))
        } else if let Ok(ty) = input.parse::<Type>() {
            Ok(SpecialType::Type(ty))
        } else {
            Ok(SpecialType::Def(input.parse::<Special>()?))
        }
    }
}

/// Re-implementation of syn types to support nested custom syntax.
pub(crate) mod augmented {
    use {
        syn::{
            ext::IdentExt,
            parse::{Parse, ParseStream},
            Result, Token,
            spanned::Spanned,
        },
        crate::{Unpack, UnpackContext, attributes::CompositeAttribute, fish::FishHook, special_data::handle_fish_hook},
        proc_macro2::TokenStream,
    };

    pub enum Type {
        Path(TypePath),
    }

    pub struct TypePath {
        pub qself: Option<syn::QSelf>,
        pub path: Path,
    }

    pub struct Path {
        pub leading_colon: Option<syn::token::PathSep>,
        pub segments: syn::punctuated::Punctuated<PathSegment, syn::token::PathSep>,
    }

    pub struct PathSegment {
        pub ident: syn::Ident,
        pub arguments: PathArguments,
    }

    pub enum PathArguments {
        None,
        AngleBracketed(AngleBracketedGenericArguments),
        // todo: support custom types inside `Fn(A, B) -> C` ?
        // Parenthesized(syn::ParenthesizedGenericArguments),
    }

    pub struct AngleBracketedGenericArguments {
        pub colon2_token: Option<syn::token::PathSep>,
        pub lt_token: syn::token::Lt,
        pub args: syn::punctuated::Punctuated<GenericArgument, syn::token::Comma>,
        pub gt_token: syn::token::Gt,
    }

    pub enum GenericArgument {
        Lifetime(syn::Lifetime),
        // here we replace `syn::Type` by `super::SpecialType` !
        Type(super::SpecialType, Option<FishHook>),
        Const(syn::Expr),
        AssocType(syn::AssocType),
        AssocConst(syn::AssocConst),
        Constraint(syn::Constraint),
    }

    impl Type {
        /// Returns Ok(None) if it doesn't match
        pub fn maybe_parse(input: ParseStream) -> Result<Option<Self>> {
            let lookahead = input.lookahead1();

            // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/ty.rs#L490
            if lookahead.peek(syn::Ident)
                || input.peek(Token![super])
                || input.peek(Token![self])
                || input.peek(Token![Self])
                || input.peek(Token![crate])
                || lookahead.peek(Token![::])
                || lookahead.peek(Token![<])
            {
                let ty: TypePath = input.parse()?;
                if ty.qself.is_some() {
                    return Ok(Some(Type::Path(ty)));
                }

                // we ignore macros
                if input.peek(Token![!]) && !input.peek(Token![!=]) && ty.path.is_mod_style() {
                    return Ok(None);
                }

                return Ok(Some(Type::Path(ty)));
            }

            // not a augmented type, fallback to `syn::Type`
            Ok(None)
        }
    }

    // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/ty.rs#L760
    impl Parse for TypePath {
        fn parse(input: ParseStream) -> Result<Self> {
            let expr_style = false;
            let (qself, path) = parse_qpath(input, expr_style)?;
            Ok(TypePath { qself, path })
        }
    }

    // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/path.rs#L636
    fn parse_qpath(input: ParseStream, expr_style: bool) -> Result<(Option<syn::QSelf>, Path)> {
        if input.peek(Token![<]) {
            let lt_token: Token![<] = input.parse()?;
            let this: syn::Type = input.parse()?;
            let path = if input.peek(Token![as]) {
                let as_token: Token![as] = input.parse()?;
                let path: Path = input.parse()?;
                Some((as_token, path))
            } else {
                None
            };
            let gt_token: Token![>] = input.parse()?;
            let colon2_token: Token![::] = input.parse()?;
            let mut rest = syn::punctuated::Punctuated::new();
            loop {
                let path = PathSegment::parse_helper(input, expr_style)?;
                rest.push_value(path);
                if !input.peek(Token![::]) {
                    break;
                }
                let punct: Token![::] = input.parse()?;
                rest.push_punct(punct);
            }
            let (position, as_token, path) = match path {
                Some((as_token, mut path)) => {
                    let pos = path.segments.len();
                    path.segments.push_punct(colon2_token);
                    path.segments.extend(rest.into_pairs());
                    (pos, Some(as_token), path)
                }
                None => {
                    let path = Path {
                        leading_colon: Some(colon2_token),
                        segments: rest,
                    };
                    (0, None, path)
                }
            };
            let qself = syn::QSelf {
                lt_token,
                ty: Box::new(this),
                position,
                as_token,
                gt_token,
            };
            Ok((Some(qself), path))
        } else {
            let path = Path::parse_helper(input, expr_style)?;
            Ok((None, path))
        }
    }

    // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/path.rs#L629
    impl Path {
        fn is_mod_style(&self) -> bool {
            self.segments
                .iter()
                .all(|segment| segment.arguments.is_none())
        }

        fn parse_helper(input: ParseStream, expr_style: bool) -> Result<Self> {
            let mut path = Path {
                leading_colon: input.parse()?,
                segments: {
                    let mut segments = syn::punctuated::Punctuated::new();
                    let value = PathSegment::parse_helper(input, expr_style)?;
                    segments.push_value(value);
                    segments
                },
            };
            Path::parse_rest(input, &mut path, expr_style)?;
            Ok(path)
        }

        fn parse_rest(
            input: ParseStream,
            path: &mut Self,
            expr_style: bool,
        ) -> Result<()> {
            while input.peek(Token![::]) && !input.peek3(syn::token::Paren) {
                let punct: Token![::] = input.parse()?;
                path.segments.push_punct(punct);
                let value = PathSegment::parse_helper(input, expr_style)?;
                path.segments.push_value(value);
            }
            Ok(())
        }
    }

    impl Parse for Path {
        fn parse(input: ParseStream) -> Result<Self> {
            Self::parse_helper(input, false)
        }
    }

    // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/path.rs#L163
    impl PathArguments {
        pub fn is_none(&self) -> bool {
            match self {
                PathArguments::None => true,
                PathArguments::AngleBracketed(_) => false,
            }
        }
    }

    // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/path.rs#L501
    impl Parse for PathSegment {
        fn parse(input: ParseStream) -> Result<Self> {
            Self::parse_helper(input, false)
        }
    }

    // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/path.rs#L508
    impl PathSegment {
        fn parse_helper(input: ParseStream, expr_style: bool) -> Result<Self> {
            if input.peek(Token![super])
                || input.peek(Token![self])
                || input.peek(Token![crate])
                || cfg!(feature = "full") && input.peek(Token![try])
            {
                let ident = input.call(syn::Ident::parse_any)?;
                return Ok(PathSegment::from(ident));
            }

            let ident = if input.peek(Token![Self]) {
                input.call(syn::Ident::parse_any)?
            } else {
                input.parse()?
            };

            if !expr_style && input.peek(Token![<]) && !input.peek(Token![<=])
                || input.peek(Token![::]) && input.peek3(Token![<])
            {
                Ok(PathSegment {
                    ident,
                    arguments: PathArguments::AngleBracketed(input.parse()?),
                })
            } else {
                Ok(PathSegment::from(ident))
            }
        }
    }

    // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/path.rs#L116
    impl<T> From<T> for PathSegment
    where
        T: Into<syn::Ident>,
    {
        fn from(ident: T) -> Self {
            PathSegment {
                ident: ident.into(),
                arguments: PathArguments::None,
            }
        }
    }

    // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/path.rs#L481
    impl Parse for AngleBracketedGenericArguments {
        fn parse(input: ParseStream) -> Result<Self> {
            let colon2_token: Option<Token![::]> = input.parse()?;
            Self::do_parse(colon2_token, input)
        }
    }

    impl AngleBracketedGenericArguments {
        // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/path.rs#L452
        fn do_parse(colon2_token: Option<Token![::]>, input: ParseStream) -> Result<Self> {
            Ok(AngleBracketedGenericArguments {
                colon2_token,
                lt_token: input.parse()?,
                args: {
                    let mut args = syn::punctuated::Punctuated::new();
                    loop {
                        if input.peek(Token![>]) {
                            break;
                        }
                        let value: GenericArgument = input.parse()?;
                        args.push_value(value);
                        if input.peek(Token![>]) {
                            break;
                        }
                        let punct: Token![,] = input.parse()?;
                        args.push_punct(punct);
                    }
                    args
                },
                gt_token: input.parse()?,
            })
        }
    }

    // Copied from: https://github.com/dtolnay/syn/blob/c7f734d8c1c288ea5fc48391a419c23ade441cac/src/path.rs#L316
    impl Parse for GenericArgument {
        fn parse(mut input: ParseStream) -> Result<Self> {
            if input.peek(syn::Lifetime) && !input.peek2(Token![+]) {
                return Ok(GenericArgument::Lifetime(input.parse()?));
            }

            if input.peek(syn::Lit) || input.peek(syn::token::Brace) {
                // we can't call `const_argument` like in syn's source and it is
                // hard to copy. We instead fall back on `syn::GenericArgument` and convert it.

                match input.parse()? {
                    syn::GenericArgument::Const(c) => return Ok(GenericArgument::Const(c)),
                    _ => unreachable!()
                }
            }

            let mut argument: super::SpecialType = input.parse()?;
            let fish = handle_fish_hook(&mut input, &argument)?;

            match argument {
                super::SpecialType::Type(syn::Type::Path(mut ty))
                    if ty.qself.is_none()
                        && ty.path.leading_colon.is_none()
                        && ty.path.segments.len() == 1
                        && match &ty.path.segments[0].arguments {
                            syn::PathArguments::None | syn::PathArguments::AngleBracketed(_) => {
                                true
                            }
                            syn::PathArguments::Parenthesized(_) => false,
                        } =>
                {
                    if let Some(eq_token) = input.parse::<Option<Token![=]>>()? {
                        let segment = ty.path.segments.pop().unwrap().into_value();
                        let ident = segment.ident;
                        let generics = match segment.arguments {
                            syn::PathArguments::None => None,
                            syn::PathArguments::AngleBracketed(arguments) => Some(arguments),
                            syn::PathArguments::Parenthesized(_) => unreachable!(),
                        };
                        return if input.peek(syn::Lit) || input.peek(syn::token::Brace) {
                            Ok(GenericArgument::AssocConst(syn::AssocConst {
                                ident,
                                generics,
                                eq_token,
                                // value: const_argument(input)?,
                                //
                                // still can't call `const_argument`, so we use
                                // `syn::GenericArgument` and return an error if it doesn't match a
                                // Const
                                value: match input.parse()? {
                                    syn::GenericArgument::Const(c) => c,
                                    arg => return Err(syn::Error::new(arg.span(), "Expected const argument")),
                                }
                            }))
                        } else {
                            // TODO: Add custom syntax here to support
                            // `impl Future<Output = struct ...>` and alike?
                            Ok(GenericArgument::AssocType(syn::AssocType {
                                ident,
                                generics,
                                eq_token,
                                ty: input.parse()?,
                            }))
                        };
                    }

                    if let Some(colon_token) = input.parse::<Option<Token![:]>>()? {
                        let segment = ty.path.segments.pop().unwrap().into_value();
                        return Ok(GenericArgument::Constraint(syn::Constraint {
                            ident: segment.ident,
                            generics: match segment.arguments {
                                syn::PathArguments::None => None,
                                syn::PathArguments::AngleBracketed(arguments) => Some(arguments),
                                syn::PathArguments::Parenthesized(_) => unreachable!(),
                            },
                            colon_token,
                            bounds: {
                                let mut bounds = syn::punctuated::Punctuated::new();
                                loop {
                                    if input.peek(Token![,]) || input.peek(Token![>]) {
                                        break;
                                    }
                                    let value: syn::TypeParamBound = input.parse()?;
                                    bounds.push_value(value);
                                    if !input.peek(Token![+]) {
                                        break;
                                    }
                                    let punct: Token![+] = input.parse()?;
                                    bounds.push_punct(punct);
                                }
                                bounds
                            },
                        }));
                    }

                    argument = super::SpecialType::Type(syn::Type::Path(ty));
                }
                _ => {}
            }

            Ok(GenericArgument::Type(argument, fish))
        }
    }

    // unpack
    impl Unpack for Type {
        type Output = (syn::Type, Vec<TokenStream>);

        fn unpack(
            self,
            unpack_context: UnpackContext,
            from_variant: Vec<CompositeAttribute>,
            override_public: Option<syn::Visibility>,
            enum_context: bool,
        ) -> Self::Output {
            match self {
                Self::Path(ty) => {
                    let (ty, definitions) = ty.unpack(unpack_context, from_variant, override_public, enum_context);
                    (syn::Type::Path(ty), definitions)
                }
            }
        }
    }

    impl Unpack for TypePath {
        type Output = (syn::TypePath, Vec<TokenStream>);

        fn unpack(
            self,
            unpack_context: UnpackContext,
            from_variant: Vec<CompositeAttribute>,
            override_public: Option<syn::Visibility>,
            enum_context: bool,
        ) -> Self::Output {
            let Self { qself, path } = self;
            let (path, definitions) = path.unpack(unpack_context, from_variant, override_public, enum_context);
            (syn::TypePath { qself, path }, definitions )
        }
    }

    impl Unpack for Path {
        type Output = (syn::Path, Vec<TokenStream>);

        fn unpack(
            self,
            unpack_context: UnpackContext,
            from_variant: Vec<CompositeAttribute>,
            override_public: Option<syn::Visibility>,
            enum_context: bool,
        ) -> Self::Output {
            let Self { leading_colon, segments } = self;
            let mut definitions = vec![];
            let segments = segments.into_pairs().map(|seg| {
                let (seg, punct) = seg.into_tuple();
                let (seg, mut defs) = seg.unpack(unpack_context.clone(), from_variant.clone(), override_public.clone(), enum_context);
                definitions.append(&mut defs);
                syn::punctuated::Pair::new(seg, punct)
            }).collect();

            (syn::Path { leading_colon, segments}, definitions)

        }
    }

    impl Unpack for PathSegment {
        type Output = (syn::PathSegment, Vec<TokenStream>);

        fn unpack(
            self,
            unpack_context: UnpackContext,
            from_variant: Vec<CompositeAttribute>,
            override_public: Option<syn::Visibility>,
            enum_context: bool,
        ) -> Self::Output {
            let Self { ident, arguments } = self;
            let (arguments, definitions) = arguments.unpack(unpack_context, from_variant, override_public, enum_context);
            (syn::PathSegment { ident, arguments }, definitions )
        }
    }

    impl Unpack for PathArguments {
        type Output = (syn::PathArguments, Vec<TokenStream>);

        fn unpack(
            self,
            unpack_context: UnpackContext,
            from_variant: Vec<CompositeAttribute>,
            override_public: Option<syn::Visibility>,
            enum_context: bool,
        ) -> Self::Output {
            match self {
                PathArguments::None => (syn::PathArguments::None, vec![]),
                PathArguments::AngleBracketed(a) => {
                    let (a, defs) = a.unpack(unpack_context, from_variant, override_public, enum_context);
                    (syn::PathArguments::AngleBracketed(a), defs)
                }
            }
        }
    }

    impl Unpack for AngleBracketedGenericArguments {
        type Output = (syn::AngleBracketedGenericArguments, Vec<TokenStream>);

        fn unpack(
            self,
            unpack_context: UnpackContext,
            from_variant: Vec<CompositeAttribute>,
            override_public: Option<syn::Visibility>,
            enum_context: bool,
        ) -> Self::Output {
            let Self { colon2_token, lt_token, args, gt_token } = self;
            let mut definitions = vec![];
            let args = args.into_pairs().map(|arg| {
                let (arg, punct) = arg.into_tuple();
                let (arg, mut defs) = arg.unpack(unpack_context.clone(), from_variant.clone(), override_public.clone(), enum_context);
                definitions.append(&mut defs);
                syn::punctuated::Pair::new(arg, punct)
            }).collect();
            (syn::AngleBracketedGenericArguments { colon2_token, lt_token, args, gt_token }, definitions )
        }
    }

    impl Unpack for GenericArgument {
        type Output = (syn::GenericArgument, Vec<TokenStream>);

        fn unpack(
            self,
            unpack_context: UnpackContext,
            from_variant: Vec<CompositeAttribute>,
            override_public: Option<syn::Visibility>,
            enum_context: bool,
        ) -> Self::Output {
            match self {
                GenericArgument::Lifetime(v) => (syn::GenericArgument::Lifetime(v), vec![]),
                GenericArgument::Const(v) => (syn::GenericArgument::Const(v), vec![]),
                GenericArgument::AssocType(v) => (syn::GenericArgument::AssocType(v), vec![]),
                GenericArgument::AssocConst(v) => (syn::GenericArgument::AssocConst(v), vec![]),
                GenericArgument::Constraint(v) => (syn::GenericArgument::Constraint(v), vec![]),
                GenericArgument::Type(super::SpecialType::Type(ty), _fish ) => {
                    (syn::GenericArgument::Type(ty), vec![])
                }
                GenericArgument::Type(super::SpecialType::Augmented(ty), _fish) => {
                    let (ty, defs) = ty.unpack(unpack_context, from_variant, override_public, enum_context);
                    (syn::GenericArgument::Type(ty), defs)
                }
                GenericArgument::Type(super::SpecialType::Def(special), fish) => {
                    let ty = type_from_ident_and_fish(special.ident.clone(), fish);

                    let defs = special.unpack(unpack_context.clone(), from_variant, override_public, enum_context);
                    (syn::GenericArgument::Type(ty), vec![defs])

                }
            }
        }
    }

    fn type_from_ident_and_fish(ident: syn::Ident, fish: Option<FishHook>) -> syn::Type {
        let args = match fish {
            None => syn::PathArguments::None,
            Some(fish) => {
                let syn::AngleBracketedGenericArguments { lt_token, args, gt_token, ..} = fish.generics;
                let args = syn::AngleBracketedGenericArguments { lt_token, args, gt_token, colon2_token: None };
                syn::PathArguments::AngleBracketed(args)
            }
        };
       
        let segment = syn::PathSegment { ident, arguments: args };
        let mut segments = syn::punctuated::Punctuated::new();
        segments.push(segment);
        let path = syn::Path { leading_colon: None, segments };
        let ty = syn::TypePath { qself: None, path };
        syn::Type::Path(ty)
    }
}