
TranslationUnit(
    NonEmpty([
        Declaration(
            InitDeclaratorList(
                InitDeclaratorList { 
                    head: SingleDeclaration { 
                        ty: FullySpecifiedType { 
                            qualifier: None, ty: TypeSpecifier { ty: Float, array_specifier: None } 
                        }, 
                        name: Some(Identifier("e")), 
                        array_specifier: None, 
                        initializer: Some(Simple(FloatConst(3.0))) 
                    }, 
                    tail: [] 
                }
    ))])
)

"vec2 e = vec2(3.0);
float b = 1.0;"
TranslationUnit(
    NonEmpty([
        Declaration(
            InitDeclaratorList(
                InitDeclaratorList { 
                    head: SingleDeclaration { 
                        ty: FullySpecifiedType { 
                            qualifier: None, 
                            ty: TypeSpecifier { 
                                ty: Vec2, array_specifier: None 
                            } 
                        }, 
                        name: Some(Identifier("e")), 
                        array_specifier: None, 
                        initializer: Some(Simple(FunCall(Identifier(Identifier("vec2<f32>")), 
                            [FloatConst(3.0)]))) 
                        }, 
                    tail: [] })), 
        Declaration(
            InitDeclaratorList(
                InitDeclaratorList { 
                    head: SingleDeclaration { 
                        ty: FullySpecifiedType { 
                            qualifier: None, 
                            ty: TypeSpecifier { 
                                ty: Float, 
                                array_specifier: None } 
                        }, 
                        name: Some(Identifier("b")), 
                        array_specifier: None, 
                        initializer: Some(Simple(FloatConst(1.0))) 
                    }, 
                    tail: [] }))]))

"vec2 e = vec2(3.0);"
TranslationUnit(
    NonEmpty([
        Declaration(
            InitDeclaratorList(
                InitDeclaratorList { 
                    head: SingleDeclaration { 
                        ty: FullySpecifiedType { 
                            qualifier: None, ty: TypeSpecifier { ty: Vec2, array_specifier: None } 
                        }, 
                        name: Some(Identifier("e")),
                        array_specifier: None,
                        initializer: Some(Simple(FunCall(Identifier(Identifier("vec2")),
                            [FloatConst(3.0)])))
                    },
                    tail: []
                }))]))


"const vec2 e = vec2(.00035, -.00035);"
TranslationUnit(
    NonEmpty([
        Declaration(
            InitDeclaratorList(
                InitDeclaratorList { 
                    head: SingleDeclaration { 
                        ty: FullySpecifiedType { 
                            qualifier: Some(TypeQualifier { qualifiers: NonEmpty([Storage(Const)]) }), 
                            ty: TypeSpecifier { ty: Vec2, array_specifier: None } }, 
                        name: Some(Identifier("e")), array_specifier: None, 
                        initializer: Some(Simple(FunCall(Identifier(Identifier("vec2<f32>")), [
                            FloatConst(0.00035), Unary(Minus, FloatConst(0.00035))
                        ]))) }, 
                    tail: [] }))]))

// vec3 norm(vec3 po) {}
TranslationUnit(
    NonEmpty(
        [FunctionDefinition(
            FunctionDefinition { 
                prototype: FunctionPrototype { 
                    ty: FullySpecifiedType { 
                        qualifier: None, 
                        ty: TypeSpecifier { 
                            ty: Vec3, array_specifier: None } 
                    }, 
                    name: Identifier("norm"), 
                    parameters: [
                        Named(None, FunctionParameterDeclarator { 
                            ty: TypeSpecifier { 
                                ty: Vec3, array_specifier: None 
                            }, 
                            ident: ArrayedIdentifier { 
                                ident: Identifier("po"), array_spec: None 
                            } 
                        })] 
                }, 
                statement: CompoundStatement { statement_list: [] } })]))

"void norm(vec3 po) {float r, e;  }";
TranslationUnit(NonEmpty(
    [FunctionDefinition(
        FunctionDefinition { 
            prototype: FunctionPrototype { 
                ty: FullySpecifiedType { 
                    qualifier: None, ty: TypeSpecifier { ty: Void, array_specifier: None } }, 
                    name: Identifier("norm"), 
                    parameters: [Named(None, FunctionParameterDeclarator { ty: TypeSpecifier { ty: Vec3, array_specifier: None }, ident: ArrayedIdentifier { ident: Identifier("po"), array_spec: None } })] }, 
            statement: CompoundStatement { statement_list: [
Simple(
    Declaration(
        InitDeclaratorList(
            InitDeclaratorList { 
                head: SingleDeclaration { 
                    ty: FullySpecifiedType { 
                        qualifier: None, 
                        ty: TypeSpecifier { 
                            ty: Float, array_specifier: None } },
                    name: Some(Identifier("r")), 
                    array_specifier: None, 
                    initializer: None }, 
                tail: [
                    SingleDeclarationNoType { 
                        ident: ArrayedIdentifier { 
                            ident: Identifier("e"), 
                            array_spec: None }, 
                        initializer: None }] })))] } })]))

show_function_prototype

"void mainImage( out vec4 fragColor, in vec2 fragCoord ){}"
TranslationUnit(NonEmpty([
    FunctionDefinition(
        FunctionDefinition { 
prototype: FunctionPrototype { 
    ty: FullySpecifiedType { 
        qualifier: None, 
        ty: TypeSpecifier { 
            ty: Void, 
            array_specifier: None } 
        }, 
    name: Identifier("mainImage"), 
    parameters: [
        Named(Some(TypeQualifier { qualifiers: NonEmpty([Storage(Out)]) }), 
            FunctionParameterDeclarator { 
                ty: TypeSpecifier { 
                    ty: Vec4, 
                    array_specifier: None 
                }, 
                ident: ArrayedIdentifier { 
                    ident: Identifier("fragColor"), 
                    array_spec: None 
                } 
            }), 
        Named(Some(TypeQualifier { qualifiers: NonEmpty([Storage(In)]) }), 
            FunctionParameterDeclarator { 
                ty: TypeSpecifier { 
                    ty: Vec2, array_specifier: None }, 
                    ident: ArrayedIdentifier { 
                        ident: Identifier("fragCoord"), array_spec: None 
                    } })] }, 
statement: CompoundStatement { statement_list: [] 
                    } })]))

"struct"
TranslationUnit(NonEmpty(
    [Declaration(
        InitDeclaratorList(
            InitDeclaratorList { 
                head: SingleDeclaration { 
                    ty: FullySpecifiedType { 
                        qualifier: None, 
                        ty: TypeSpecifier { 
                            ty: Struct(StructSpecifier { 
                                name: Some(TypeName("light")), 
                                fields: NonEmpty([
                                    StructFieldSpecifier { 
                                        qualifier: None, 
                                        ty: TypeSpecifier { 
                                            ty: Float, array_specifier: None 
                                        }, 
                                        identifiers: NonEmpty([ArrayedIdentifier { 
                                            ident: Identifier("intensity"), array_spec: None 
                                        }]) 
                                    }, 
                                    StructFieldSpecifier { 
                                        qualifier: None, 
                                        ty: TypeSpecifier { 
                                            ty: Vec3, 
                                            array_specifier: None 
                                        }, 
                                        identifiers: NonEmpty([ArrayedIdentifier { 
                                            ident: Identifier("position"), array_spec: None }]) }]) }), 
                            array_specifier: None } 
                    }, 
                    name: None, 
                    array_specifier: None, 
                    initializer: None }, tail: [] }))]))


TranslationUnit(
    NonEmpty([
        Declaration(
            InitDeclaratorList(
                InitDeclaratorList { 
                    head: SingleDeclaration { 
                        ty: FullySpecifiedType { 
                            qualifier: Some(TypeQualifier { 
                                qualifiers: NonEmpty([Storage(Const)]) }
                            ), 
                            ty: TypeSpecifier { ty: Float, array_specifier: None } 
                        }, 
                        name: Some(Identifier("yaa")), 
                        array_specifier: Some(ArraySpecifier { 
                            dimensions: NonEmpty([ExplicitlySized(IntConst(1))]) }
                        ), 
                            initializer: Some(Simple(FunCall(Expr(Bracket(Variable(Identifier("f32")),
                                ArraySpecifier { dimensions: NonEmpty([ExplicitlySized(IntConst(1))]) })), 
                                    [FloatConst(5.5)]))) 
                        }, 
                    tail: [] }))]))  "const yaa[1]: f32 = f32[1](5.5);" -> "const yaa: array<f32, 1> = array<f32, 1>[5.5]"


ty: FullySpecifiedType { 
    qualifier: Some(TypeQualifier { 
        qualifiers: NonEmpty([Storage(Const)]) }), 
        ty: TypeSpecifier { ty: Float, array_specifier: None } }, 
        name: Some(Identifier("yaa")), 
        array_specifier: Some(ArraySpecifier { 
            dimensions: NonEmpty([ExplicitlySized(IntConst(1))]) }), 
            initializer: Some(Simple(FunCall(Identifier(Identifier("f32")), [FloatConst(5.5)]))) }

TranslationUnit(
    NonEmpty([
        Declaration(
            InitDeclaratorList(
                InitDeclaratorList { 
                    head: SingleDeclaration { 
                        ty: FullySpecifiedType { 
                            qualifier: None, ty: TypeSpecifier { 
                                ty: Vec2, array_specifier: None } 
                            }, 
                        name: Some(Identifier("a")), 
                        array_specifier: None, 
                        initializer: Some(Simple(
                            Binary(
                                Add, 
                                FunCall(
                                    Identifier(Identifier("vec2<f32>")), 
                                    [IntConst(1), IntConst(1)]
                                ), 
                                FunCall(
                                    Identifier(Identifier("vec2<f32>")), 
                                    [IntConst(0), IntConst(0)])))) }, 
                    tail: [] }))]))
)